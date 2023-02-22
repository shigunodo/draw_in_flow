use crate::marching::calc_step_iter;
use crate::{Plot, Fluid, Boundary, Cell, Index};
use crate::parameters::{NI_M, NJ_M, RAT_PLOT, NI_PLOT, NJ_PLOT, RAT_M, N_RGB};
use wasm_bindgen::prelude::*;

impl Plot {
  #[inline]
  fn set_cmap(&self, i: usize, j: usize, value: u8, rgb: &mut [u8; N_RGB]) {
    let idx = j * (NI_PLOT * 3) + i * 3;
    rgb[idx    ] = self.cmap.r[value as usize]; // r
    rgb[idx + 1] = self.cmap.g[value as usize]; // g
    rgb[idx + 2] = self.cmap.b[value as usize]; // b
  }
  #[inline]
  pub fn set_color(&self, i: usize, j: usize, r: u8, g: u8, b: u8, rgb: &mut [u8; N_RGB]) {
    let idx = j * (NI_PLOT * 3) + i * 3;
    rgb[idx    ] = r; // r
    rgb[idx + 1] = g; // g
    rgb[idx + 2] = b; // b
  }
  fn multiply_color(&self, i: usize, j: usize, coef: f64, rgb: &mut [u8; N_RGB]) {
    let idx = j * (NI_PLOT * 3) + i * 3;
    rgb[idx] = (rgb[idx] as f64 * coef) as u8;
    rgb[idx+1] = (rgb[idx+1] as f64 * coef) as u8;
    rgb[idx+2] = (rgb[idx+2] as f64 * coef) as u8;
  }
}

impl Plot {
  fn value(&self, value: &[[f64; NI_M]; NJ_M], vmin: f64, vmax: f64, rgb: &mut [u8; N_RGB]) {
    for j in 0..NJ_M {
      for i in 0..NI_M {
        let v_f64 = (value[j][i] - vmin) / (vmax - vmin);
        let v_u8 =
        if v_f64.is_nan() {Option::<u8>::None}
        else {Option::<u8>::Some((v_f64 * 256.0) as u8)};
        for kj in 0..RAT_PLOT {
          for ki in 0..RAT_PLOT {
            match v_u8 {
              Some(value) => self.set_cmap(i * RAT_PLOT + ki, j * RAT_PLOT + kj, value, rgb),
              None => self.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 0, 0, 0, rgb),
            }
          }
        }
      }
    }
  }
  fn boundary(&self, bound: &Boundary, rgb: &mut [u8; N_RGB]) {
    for j in 0..NJ_M {
      for kj in 0..RAT_PLOT {
        for i in 0..NI_M {
          for ki in 0..RAT_PLOT {
            if bound.cell[j][i] == Cell::Dead {
              self.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 71, 181, 255, rgb);
            }
          }
        }
      }
    }
  }
  fn stream(&self, f: &[[f64; NI_PLOT]; NJ_PLOT], rgb: &mut [u8; N_RGB]) {
    for j in 0..NJ_PLOT {
      for i in 0..NI_PLOT {
        self.multiply_color(i, j, f[j][i], rgb);
      }
    }
  }
}

#[wasm_bindgen]
impl Fluid {
  pub fn get_ni(&self) -> usize {NI_PLOT}
  pub fn get_nj(&self) -> usize {NJ_PLOT}
  pub fn get_rate_plot(&self) -> usize {RAT_PLOT}
  pub fn get_ptr(&self) -> *const u8 {
    self.rgb.as_ptr()
  }
}

#[wasm_bindgen]
impl Fluid {
  pub fn plot(&mut self, plot_quantity: u8, stream: u8, vmin: f64, vmax: f64, dt_iter: f64, progress_lic_phase: u8) {
    let step_iter = calc_step_iter(dt_iter, self.param.ma);
    match plot_quantity {
      0 => self.plot_velocity(vmin, vmax),
      1 => self.plot_vorticity(vmin, vmax),
      2 => self.plot_pressure(vmin, vmax),
      _ => panic!("fn plot: No quantities for plotting specified."),
    }
    if stream == 1 {
      let dt = (step_iter * RAT_M) as f64;
      self.lic.calc(&self.basic.u_main, &self.basic.v_main, self.param.ma, dt, progress_lic_phase);
      self.plotter.stream(&self.lic.f, &mut self.rgb);
    }
    self.plotter.boundary(&self.bound, &mut self.rgb);
  }
  pub fn plot_cells(&mut self) {
    for j in 0..NJ_M {
      for kj in 0..RAT_PLOT {
        for i in 0..NI_M {
          for ki in 0..RAT_PLOT {
            if self.bound.cell[j][i] == Cell::Dead {
              self.plotter.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 71, 181, 255, &mut self.rgb);
            } else {
              self.plotter.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 6, 40, 61, &mut self.rgb);
            }
          }
        }
      }
    }
  }
}


impl Fluid {
  #[inline]
  fn plot_velocity(&mut self, vmin: f64, vmax: f64) {
    let velo = calc_velocity(&self.basic.u_main, &self.basic.v_main);
    self.plotter.value(&velo, vmin, vmax, &mut self.rgb);
  }
  #[inline]
  fn plot_vorticity(&mut self, vmin: f64, vmax: f64) {
    let vor = calc_vorticity(&self.basic.u_main, &self.basic.v_main, &self.bound_up, &self.bound_down, &self.bound_left, &self.bound_right);
    self.plotter.value(&vor, vmin, vmax, &mut self.rgb);
  }
  #[inline]
  fn plot_pressure(&mut self, vmin: f64, vmax: f64) {
    let p = calc_pressure(&self.basic.rho_main);
    self.plotter.value(&p, vmin, vmax, &mut self.rgb);
  }
}

#[inline]
fn calc_vorticity(
  u: &[[f64; NI_M]; NJ_M], 
  v: &[[f64; NI_M]; NJ_M], 
  bound_up: &Vec<Index>,
  bound_down: &Vec<Index>,
  bound_left: &Vec<Index>,
  bound_right: &Vec<Index>,
) -> [[f64; NI_M]; NJ_M] {
  let mut vor = [[std::f64::NAN; NI_M]; NJ_M];
  for j in 1..(NJ_M-1) {
    for i in 1..(NI_M-1) {
      vor[j][i] = 0.5 * (v[j][i+1] - v[j][i-1] - u[j+1][i] + u[j-1][i]);
    }
  }
  for idx in bound_up.iter() {
    vor[idx.j][idx.i] = std::f64::NAN;
  }
  for idx in bound_down.iter() {
    vor[idx.j][idx.i] = std::f64::NAN;
  }
  for idx in bound_left.iter() {
    vor[idx.j][idx.i] = std::f64::NAN;
  }
  for idx in bound_right.iter() {
    vor[idx.j][idx.i] = std::f64::NAN;
  }
  vor
}

#[inline]
fn calc_velocity(u: &[[f64; NI_M]; NJ_M], v: &[[f64; NI_M]; NJ_M]) -> [[f64; NI_M]; NJ_M] {
  let mut velo = [[std::f64::NAN; NI_M]; NJ_M];
  for j in 0..NJ_M {
    for i in 0..NI_M {
      velo[j][i] = (u[j][i] * u[j][i] + v[j][i] * v[j][i]).sqrt();
    }
  }
  velo
}

#[inline]
fn calc_pressure(rho: &[[f64; NI_M]; NJ_M]) -> [[f64; NI_M]; NJ_M] {
  let mut p = [[std::f64::NAN; NI_M]; NJ_M];
  for j in 0..NJ_M {
    for i in 0..NI_M {
      p[j][i] = rho[j][i] - 1.0;
    }
  }
  p
}