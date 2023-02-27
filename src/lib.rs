mod parameters; // global parameters
mod distribution; // for distribution function and its conversion
mod collision; // for collision processes
mod plot; // plotter
mod cmap; // parameters for color map
mod advection; // for advection steps
mod bc; // for boundary condition
mod marching; // time-marching and iteration methods
mod cells; // for cells and physical boundary
mod interpolation; // interpolation methods for multi-block scheme
mod lic; // for stream-line visualization
mod jitter; // noisy seed for LIC
mod airfoil; // airfoil data for boundary setting
mod na; // hiragana "na" for boundary setting

use std::f64::consts::PI;

use parameters::{NC, N_RGB, L_SCALE, RAT_M, NI_M, NJ_M, NI_T, NJ_T, NI_B, NJ_B, NI_L, NJ_L, NJ_R, NI_R, N_SOLV_TB, N_SOLV_LR, NI_PLOT, NJ_PLOT, BOUND_VEC_CAP};
use wasm_bindgen::prelude::*;

//--------------------------------------------------------------
// ↓↓↓↓↓ decleration of struct to be created in js script ↓↓↓↓↓
//--------------------------------------------------------------

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Index {
  i: usize,
  j: usize,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Boundary {
  cell: [[Cell; NI_M]; NJ_M],
}

impl Boundary {
  fn new() -> Self {
    Self {
      cell: [[Cell::Alive; NI_M]; NJ_M],
    }
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Basic {
  rho_main: [[f64; NI_M]; NJ_M],
  u_main: [[f64; NI_M]; NJ_M],
  v_main: [[f64; NI_M]; NJ_M],
  rho_top: [[f64; NI_T]; NJ_T],
  u_top: [[f64; NI_T]; NJ_T],
  v_top: [[f64; NI_T]; NJ_T],
  rho_bot: [[f64; NI_B]; NJ_B],
  u_bot: [[f64; NI_B]; NJ_B],
  v_bot: [[f64; NI_B]; NJ_B],
  rho_l: [[f64; NI_L]; NJ_L],
  u_l: [[f64; NI_L]; NJ_L],
  v_l: [[f64; NI_L]; NJ_L],
  rho_r: [[f64; NI_R]; NJ_R],
  u_r: [[f64; NI_R]; NJ_R],
  v_r: [[f64; NI_R]; NJ_R],
}

impl Basic {
  fn new() -> Self {
    Self {
      rho_main: [[1.0; NI_M]; NJ_M],
      u_main: [[1.0; NI_M]; NJ_M],
      v_main: [[0.0; NI_M]; NJ_M],
      rho_top: [[1.0; NI_T]; NJ_T],
      u_top: [[1.0; NI_T]; NJ_T],
      v_top: [[0.0; NI_T]; NJ_T],
      rho_bot: [[1.0; NI_B]; NJ_B],
      u_bot: [[1.0; NI_B]; NJ_B],
      v_bot: [[0.0; NI_B]; NJ_B],
      rho_l: [[1.0; NI_L]; NJ_L],
      u_l: [[1.0; NI_L]; NJ_L],
      v_l: [[0.0; NI_L]; NJ_L],
      rho_r: [[1.0; NI_R]; NJ_R],
      u_r: [[1.0; NI_R]; NJ_R],
      v_r: [[0.0; NI_R]; NJ_R],
    }
  }
  fn init(&mut self, param: &Param) {
    let u0 = param.ma/3.0f64.sqrt();
    for j in 0.. NJ_M {
      for i in 0..NI_M {
        self.rho_main[j][i] = 1.0;
        self.u_main[j][i] = u0;
        self.v_main[j][i] = u0 * 0.1 * (i as f64 / NI_M as f64 * 2.0 * 5.0 * PI).sin();
      }
    }
    for j in 0.. NJ_T {
      for i in 0..NI_T {
        self.rho_top[j][i] = 1.0;
        self.u_top[j][i] = u0;
        self.v_top[j][i] = 0.0;
      }
    }
    for j in 0.. NJ_B {
      for i in 0..NI_B {
        self.rho_bot[j][i] = 1.0;
        self.u_bot[j][i] = u0;
        self.v_bot[j][i] = 0.0;
      }
    }
    for j in 0.. NJ_L {
      for i in 0..NI_L {
        self.rho_l[j][i] = 1.0;
        self.u_l[j][i] = u0;
        self.v_l[j][i] = 0.0;
      }
    }
    for j in 0.. NJ_R {
      for i in 0..NI_R {
        self.rho_r[j][i] = 1.0;
        self.u_r[j][i] = u0;
        self.v_r[j][i] = 0.0;
      }
    }
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Distribution {
  cx: [f64; NC],
  cy: [f64; NC],
  w: [f64; NC],
  f_main: [[[f64; NC]; NI_M]; NJ_M],
  f_top: [[[f64; NC]; NI_T]; NJ_T],
  f_bot: [[[f64; NC]; NI_B]; NJ_B],
  f_l: [[[f64; NC]; NI_L]; NJ_L],
  f_r: [[[f64; NC]; NI_R]; NJ_R],
}

impl Distribution {
  fn new() -> Self {
    Self {
      cx: [0.0, 1.0, 0.0, -1.0, 0.0, 1.0, -1.0, -1.0, 1.0],
      cy: [0.0, 0.0, 1.0, 0.0, -1.0, 1.0, 1.0, -1.0, -1.0],
      w: [4.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/36.0, 1.0/36.0, 1.0/36.0, 1.0/36.0],
      f_main: [[[0.0; NC]; NI_M]; NJ_M],
      f_top: [[[0.0; NC]; NI_T]; NJ_T],
      f_bot: [[[0.0; NC]; NI_B]; NJ_B],
      f_l: [[[0.0; NC]; NI_L]; NJ_L],
      f_r: [[[0.0; NC]; NI_R]; NJ_R],
    }
  }
  fn init(&mut self, basic: &Basic) {
    distribution::calc_feq::<NI_M, NJ_M>(&basic.rho_main, &basic.u_main, &basic.v_main, &self.w, &self.cx, &self.cy, &mut self.f_main);
    distribution::calc_feq::<NI_T, NJ_T>(&basic.rho_top, &basic.u_top, &basic.v_top, &self.w, &self.cx, &self.cy, &mut self.f_top);
    distribution::calc_feq::<NI_B, NJ_B>(&basic.rho_bot, &basic.u_bot, &basic.v_bot, &self.w, &self.cx, &self.cy, &mut self.f_bot);
    distribution::calc_feq::<NI_L, NJ_L>(&basic.rho_l, &basic.u_l, &basic.v_l, &self.w, &self.cx, &self.cy, &mut self.f_l);
    distribution::calc_feq::<NI_R, NJ_R>(&basic.rho_r, &basic.u_r, &basic.v_r, &self.w, &self.cx, &self.cy, &mut self.f_r);
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Param {
  tau_f: f64,
  tau_c: f64,
  ma: f64,
}

impl Param {
  fn new() -> Self {
    Self {
      tau_f: 1.0,
      tau_c: 1.0,
      ma: 1.0,
    }
  }
  fn init(&mut self, re: f64, ma: f64) {
    self.tau_f = 3.0 * L_SCALE * ma / re + 0.5;
    self.tau_c = (self.tau_f - 0.5) / RAT_M as f64 + 0.5;
    self.ma = ma;
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Plot {
  cmap: cmap::CMap,
}

impl Plot {
  fn new() -> Self {
    Self {
      cmap: cmap::CMap::new(),
    }
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Interpolation {
  l_tb: [f64; N_SOLV_TB],
  u_tb: [f64; N_SOLV_TB],
  l_lr: [f64; N_SOLV_LR],
  u_lr: [f64; N_SOLV_LR],
  f_top: [[[f64; NC]; NI_M]; 3],
  f_bot: [[[f64; NC]; NI_M]; 3],
  f_l: [[[f64; NC]; NJ_M]; 3],
  f_r: [[[f64; NC]; NJ_M]; 3],
  ord_top: [usize; 3],
  ord_bot: [usize; 3],
  ord_l: [usize; 3],
  ord_r: [usize; 3],
}

impl Interpolation {
  fn new() -> Self {
    let mut l_tb = [0.0; N_SOLV_TB];
    let mut u_tb = [0.0; N_SOLV_TB];
    interpolation::calc_lu::<N_SOLV_TB>(&mut l_tb, &mut u_tb);
    let mut l_lr = [0.0; N_SOLV_LR];
    let mut u_lr = [0.0; N_SOLV_LR];
    interpolation::calc_lu::<N_SOLV_LR>(&mut l_lr, &mut u_lr);
    Self {
      l_tb,
      u_tb,
      l_lr,
      u_lr,
      f_top: [[[0.0; NC]; NI_M]; 3],
      f_bot: [[[0.0; NC]; NI_M]; 3],
      f_l: [[[0.0; NC]; NJ_M]; 3],
      f_r: [[[0.0; NC]; NJ_M]; 3],
      ord_top: [0, 1, 2],
      ord_bot: [0, 1, 2],
      ord_l: [0, 1, 2],
      ord_r: [0, 1, 2],
    }
  }
  fn init(&mut self, dist: &Distribution) {
    for m in 0..3 {
      for i in 0..NI_M {
        for k in 0..NC {
          self.f_top[m][i][k] = dist.f_main[NJ_M-1][i][k];
          self.f_bot[m][i][k] = dist.f_main[0][i][k];
        }
      }
    }
    for m in 0..3 {
      for j in 0..NJ_M {
        for k in 0..NC {
          self.f_l[m][j][k] = dist.f_main[j][0][k];
          self.f_r[m][j][k] = dist.f_main[j][NI_M-1][k];
        }
      }
    }
    self.ord_top = [0, 1, 2];
    self.ord_bot = [0, 1, 2];
    self.ord_l = [0, 1, 2];
    self.ord_r = [0, 1, 2];
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct LIC {
  phase: f64,
  //noise: [[f64; NI_M]; NJ_M],
  f: [[f64; NI_PLOT]; NJ_PLOT],
}

impl LIC {
  pub fn new() -> Self {
    Self {
      phase: 0.0, 
      f: [[0.0; NI_PLOT]; NJ_PLOT]}
  }
}

#[wasm_bindgen]
pub struct Fluid {
  // This array will be exposed to JavaScript as Unit8Array pointer.
  // When this array is located under the struct Plot,
  // the instance will be copied repeatedly (for some reason), and cause severe memory leaks.
  rgb: [u8; N_RGB],
  //
  pub param: Param,
  pub bound: Boundary,
  pub basic: Basic,
  pub dist: Distribution,
  pub interp: Interpolation,
  pub plotter: Plot,
  pub lic: LIC,
  // These Vecs are more appropriate to be put into the struct Boundary.
  // However, wasm_bindgen didn't allow it, because of absent Copy trait.
  bound_up: Vec<Index>,
  bound_down: Vec<Index>,
  bound_left: Vec<Index>,
  bound_right: Vec<Index>,
  drag: f64,
  lift: f64,
}

#[wasm_bindgen]
impl Fluid {
  pub fn new() -> Self {
    Self {
      rgb: [0u8; N_RGB],
      param: Param::new(), 
      bound: Boundary::new(),
      basic: Basic::new(), 
      dist: Distribution::new(), 
      interp: Interpolation::new(), 
      plotter: Plot::new(), 
      lic: LIC::new(), 
      bound_up: Vec::with_capacity(BOUND_VEC_CAP), 
      bound_down: Vec::with_capacity(BOUND_VEC_CAP), 
      bound_left: Vec::with_capacity(BOUND_VEC_CAP), 
      bound_right: Vec::with_capacity(BOUND_VEC_CAP), 
      drag: 0.0, 
      lift: 0.0,
    }
  }
  pub fn init(&mut self, re: f64, ma: f64) {
    self.param.init(re, ma);
    self.basic.init(&self.param);
    self.dist.init(&self.basic);
    self.interp.init(&self.dist);
    self.drag = 0.0;
    self.lift = 0.0;
  }
}