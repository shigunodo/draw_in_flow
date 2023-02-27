use wasm_bindgen::prelude::*;
use crate::{Boundary, Cell, Fluid, Index};
use crate::parameters::{NI_M, NJ_M, RAT_PLOT};
use crate::airfoil::AIRFOIL;
use crate::na::HIRAGANA_NA;

impl Boundary {
  fn set_cylinder(&mut self) {
    let r = NJ_M as f64 / 8.0;
    let j_c = NJ_M / 2;
    let i_c = j_c;
    for j in 0..NJ_M {
      for i in 0..NI_M {
        let d2 = (i - i_c) * (i - i_c) + (j - j_c) * (j - j_c);
        if (d2 as f64) < r * r {
          self.cell[j][i] = Cell::Dead;
        } else {
          self.cell[j][i] = Cell::Alive;
        }
      }
    }
  }
  fn set_airfoil(&mut self) {
    for j in 0..NJ_M {
      for i in 0..NI_M {
        if AIRFOIL[j][i] == 0 {
          self.cell[j][i] = Cell::Dead;
        } else {
          self.cell[j][i] = Cell::Alive;
        }
      }
    }
  }
  fn set_na(&mut self) {
    for j in 0..NJ_M {
      for i in 0..NI_M {
        if HIRAGANA_NA[j][i] == 0 {
          self.cell[j][i] = Cell::Dead;
        } else {
          self.cell[j][i] = Cell::Alive;
        }
      }
    }
  }
}

#[wasm_bindgen]
impl Fluid {
  pub fn find_physical_boundary(&mut self) {
    self.bound_up.clear();
    self.bound_down.clear();
    self.bound_left.clear();
    self.bound_right.clear();
    for j in 0..NJ_M {
      for i in 0..NI_M {
        if self.bound.cell[j][i] == Cell::Dead {
          if j != NJ_M-1 {
            if self.bound.cell[j+1][i] == Cell::Alive {
              self.bound_up.push(Index{i, j: j+1});
            }
          }
          if j != 0 {
            if self.bound.cell[j-1][i] == Cell::Alive {
              self.bound_down.push(Index{i, j: j-1});
            }
          }
          if i != NI_M-1 {
            if self.bound.cell[j][i+1] == Cell::Alive {
              self.bound_right.push(Index{i: i+1, j});
            }
          }
          if i != 0 {
            if self.bound.cell[j][i-1] == Cell::Alive {
              self.bound_left.push(Index{i: i-1, j});
            }
          }
        }
      }
    }
  }
}

#[wasm_bindgen]
impl Fluid {
  pub fn set_cylinder(&mut self) {
    self.bound.set_cylinder();
    //self.find_physical_boundary();
  }
  pub fn set_airfoil(&mut self) {
    self.bound.set_airfoil();
    //self.find_physical_boundary();
  }
  pub fn set_na(&mut self) {
    self.bound.set_na();
    //self.find_physical_boundary();
  }
  pub fn get_ni_cells(&self) -> usize {NI_M}
  pub fn get_nj_cells(&self) -> usize {NJ_M}
  pub fn update_cells(&mut self, i_old: f32, j_old: f32, i_new: f32, j_new: f32, line_width: f32, draw_mode: usize) {
    let imin = (i_old.min(i_new) - line_width).max(0.0) as usize;
    let imax = (i_old.max(i_new) + line_width).min(NI_M as f32) as usize;
    let jmin = (j_old.min(j_new) - line_width).max(0.0) as usize;
    let jmax = (j_old.max(j_new) + line_width).min(NJ_M as f32) as usize;
    for j in jmin..jmax {
      for i in imin..imax {
        let dd = calc_doubled_distance(i as f32, j as f32, i_old, j_old, i_new, j_new);
        if dd < line_width * line_width * 0.25 {
          self.update_one_cell(i, j, draw_mode);
        }
      }
    }
  }
  pub fn init_cells(&mut self) {
    for j in 0..NJ_M {
      for i in 0..NI_M {
        self.bound.cell[j][i] = Cell::Alive;
      }
    }
  }
}

#[inline]
fn calc_doubled_distance(i: f32, j: f32, i_old: f32, j_old: f32, i_new: f32, j_new: f32) -> f32 {
  let i_p = i + 0.5;
  let j_p = j + 0.5;
  if (i_old - i_new) * (i_old - i_new) < 1.0e-6 && (j_old - j_new) * (j_old - j_new) < 1.0e-6 {
    return (i_p - i_old) * (i_p - i_old) + (j_p - j_old) * (j_p - j_old)
  }
  if (i_p - i_new) * (i_old - i_new) + (j_p - j_new) * (j_old - j_new) < 0.0 {
    return (i_p - i_new) * (i_p - i_new) + (j_p - j_new) * (j_p - j_new)
  }
  if (i_p - i_old) * (i_new - i_old) + (j_p - j_old) * (j_new - j_old) < 0.0 {
    return (i_p - i_old) * (i_p - i_old) + (j_p - j_old) * (j_p - j_old)
  }
  let ab = (i_new - i_old) * (j_p - j_old) - (i_p - i_old) * (j_new - j_old);
  return ab * ab/((i_new - i_old) * (i_new - i_old) + (j_new - j_old) * (j_new - j_old));
}

impl Fluid {
  #[inline]
  fn update_one_cell(&mut self, i: usize, j: usize, draw_mode: usize) {
    if draw_mode == 0 {
      self.bound.cell[j][i] = Cell::Dead;
      for kj in 0..RAT_PLOT {
        for ki in 0..RAT_PLOT {
          self.plotter.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 71, 181, 255, &mut self.rgb)
        }
      }
    } else {
      self.bound.cell[j][i] = Cell::Alive;
      for kj in 0..RAT_PLOT {
        for ki in 0..RAT_PLOT {
          self.plotter.set_color(i * RAT_PLOT + ki, j * RAT_PLOT + kj, 6, 40, 61, &mut self.rgb)
        }
      }
    }
  }
}