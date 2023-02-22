use crate::advection::{calc_advection_with_phys_bc, calc_advection};
use crate::bc::{reflect_bc_eq_top, reflect_bc_eq_left, reflect_bc_eq_bottom, convert_c_to_f, convert_f_to_c, reflect_bc_conv_right};
use crate::distribution::calc_basic;
use crate::interpolation::{interpolate_space, interpolate_time};
use crate::Fluid;
use crate::collision::calc_collision;
use crate::parameters::{NC, L_SCALE, RAT_M, NI_T, NJ_T, NJ_B, NI_B, NI_L, NJ_L, NI_R, NJ_R, NI_M, NJ_M, NJ_M_C, N_SOLV_TB, N_SOLV_LR};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Fluid {
  pub fn iterate(&mut self, dt_iter: f64, plot_quantity: u8, stream: u8, vmin: f64, vmax: f64, re: f64, ma: f64) {
    self.param.init(re, ma);
    let step_iter = calc_step_iter(dt_iter, self.param.ma);
    for _ in 0..step_iter {
      self.march();
    }
    self.plot(plot_quantity, stream, vmin, vmax, dt_iter, 1u8);
    (self.drag, self.lift) = self.calc_force();
  }
  pub fn get_drag(&self) -> f64 {self.drag}
  pub fn get_lift(&self) -> f64 {self.lift}
}

#[inline]
pub fn calc_step_iter(dt_iter: f64, ma: f64) -> usize {
  (dt_iter * L_SCALE / (ma / 3.0f64.sqrt()) / RAT_M as f64) as usize
}

impl Fluid {
  #[inline]
  fn calc_force(&self) -> (f64, f64) {
    let u_inf = self.param.ma / 3.0f64.sqrt();
    let mut drag = 0.0;
    for idx in self.bound_left.iter() {
      drag += self.basic.rho_main[idx.j][idx.i];
    }
    for idx in self.bound_right.iter() {
      drag -= self.basic.rho_main[idx.j][idx.i];
    }
    drag *= 2.0 / 3.0 / L_SCALE / u_inf / u_inf;
    let mut lift = 0.0;
    for idx in self.bound_down.iter() {
      lift += self.basic.rho_main[idx.j][idx.i];
    }
    for idx in self.bound_up.iter() {
      lift -= self.basic.rho_main[idx.j][idx.i];
    }
    lift *= 2.0 / 3.0 / L_SCALE / u_inf / u_inf;
    (drag, lift)
  }
}

impl Fluid {
  #[inline]
  fn march(&mut self) {
    self.march_top();
    self.march_bottom();
    self.march_left();
    self.march_right();
    self.set_bc_top();
    self.set_bc_bottom();
    self.set_bc_left();
    self.set_bc_right();
    self.make_interpolation();
    for n in 0..RAT_M {
      self.march_main();
      self.set_bc_main(n);
      calc_basic::<NI_M, NJ_M>(
        &self.dist.cx, &self.dist.cy, 
        &self.dist.f_main, 
        &mut self.basic.rho_main, &mut self.basic.u_main, &mut self.basic.v_main
      );
    }
    self.reflect_result_of_main();
    calc_basic::<NI_T, NJ_T>(
      &self.dist.cx, &self.dist.cy, 
      &self.dist.f_top, 
      &mut self.basic.rho_top, &mut self.basic.u_top, &mut self.basic.v_top
    );
    calc_basic::<NI_B, NJ_B>(
      &self.dist.cx, &self.dist.cy, 
      &self.dist.f_bot, 
      &mut self.basic.rho_bot, &mut self.basic.u_bot, &mut self.basic.v_bot
    );
    calc_basic::<NI_L, NJ_L>(
      &self.dist.cx, &self.dist.cy, 
      &self.dist.f_l, 
      &mut self.basic.rho_l, &mut self.basic.u_l, &mut self.basic.v_l
    );
    calc_basic::<NI_R, NJ_R>(
      &self.dist.cx, &self.dist.cy, 
      &self.dist.f_r, 
      &mut self.basic.rho_r, &mut self.basic.u_r, &mut self.basic.v_r
    );
  }

  #[inline]
  fn march_top(&mut self) {
    let mut f_str = [[[0.0; NC]; NI_T]; NJ_T];
    calc_collision::<NI_T, NJ_T>(
      self.param.tau_c, &self.dist.cx, 
      &self.dist.cy, &self.dist.f_top, 
      &self.basic.rho_top, &self.basic.u_top, &self.basic.v_top, 
      &mut f_str
    );
    calc_advection::<NI_T, NJ_T>(
      &f_str, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_top
    );
  }

  #[inline]
  fn march_bottom(&mut self) {
    let mut f_str = [[[0.0; NC]; NI_B]; NJ_B];
    calc_collision::<NI_B, NJ_B>(
      self.param.tau_c, &self.dist.cx, 
      &self.dist.cy, &self.dist.f_bot, 
      &self.basic.rho_bot, &self.basic.u_bot, &self.basic.v_bot, 
      &mut f_str
    );
    calc_advection::<NI_B, NJ_B>(
      &f_str, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_bot
    );
  }

  #[inline]
  fn march_left(&mut self) {
    let mut f_str = [[[0.0; NC]; NI_L]; NJ_L];
    calc_collision::<NI_L, NJ_L>(
      self.param.tau_c, &self.dist.cx, 
      &self.dist.cy, &self.dist.f_l, 
      &self.basic.rho_l, &self.basic.u_l, &self.basic.v_l, 
      &mut f_str
    );
    calc_advection::<NI_L, NJ_L>(
      &f_str, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_l
    );
  }

  #[inline]
  fn march_right(&mut self) {
    let mut f_str = [[[0.0; NC]; NI_R]; NJ_R];
    calc_collision::<NI_R, NJ_R>(
      self.param.tau_c, &self.dist.cx, 
      &self.dist.cy, &self.dist.f_r, 
      &self.basic.rho_r, &self.basic.u_r, &self.basic.v_r, 
      &mut f_str
    );
    calc_advection::<NI_R, NJ_R>(
      &f_str, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_r
    );
  }

  #[inline]
  fn march_main(&mut self) {
    let mut f_str = [[[0.0; NC]; NI_M]; NJ_M];
    calc_collision::<NI_M, NJ_M>(
      self.param.tau_f, &self.dist.cx, 
      &self.dist.cy, &self.dist.f_main, 
      &self.basic.rho_main, &self.basic.u_main, &self.basic.v_main, 
      &mut f_str
    );
    calc_advection_with_phys_bc::<NI_M, NJ_M>(
      &self.bound,
      &f_str, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_main
    );
  }

  #[inline]
  fn set_bc_top(&mut self) {
    reflect_bc_eq_top::<NI_T, NJ_T>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_top
    );
    // left: between the top and left block
    // right: between the top and right block
    for j in 0..NJ_T {
      for k in 0..NC {
        self.dist.f_top[j][0][k] = self.dist.f_l[j + NJ_B + NJ_M_C - 4][NI_L-2][k];
        self.dist.f_top[j][NI_T-1][k] = self.dist.f_r[j + NJ_B + NJ_M_C - 4][1][k];
      }
    }
  }

  #[inline]
  fn set_bc_bottom(&mut self) {
    reflect_bc_eq_bottom::<NI_B, NJ_B>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_bot
    );
    // left: between the bottom and left block
    // right: between the bottom and right block
    for j in 0..NJ_B {
      for k in 0..NC {
        self.dist.f_bot[j][0][k] = self.dist.f_l[j][NI_L-2][k];
        self.dist.f_bot[j][NI_B-1][k] = self.dist.f_r[j][1][k];
      }
    }
  }

  #[inline]
  fn set_bc_left(&mut self) {
    reflect_bc_eq_top::<NI_L, NJ_L>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_l
    );
    reflect_bc_eq_bottom::<NI_L, NJ_L>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_l
    );
    reflect_bc_eq_left::<NI_L, NJ_L>(
      &self.param, 
      &self.dist.w, &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_l
    );
    // right: between the (left and top) || (left and bottom) block
    for j in 0..(NJ_B-1) {
      for k in 0..NC {
        self.dist.f_l[j][NI_L-1][k] = self.dist.f_bot[j][1][k];
      }
    }
    for j in 1..NJ_T {
      for k in 0..NC {
        self.dist.f_l[j + NJ_B + NJ_M_C - 4][NI_L-1][k] = self.dist.f_top[j][1][k];
      }
    }
  }

  #[inline]
  fn set_bc_right(&mut self) {
    reflect_bc_eq_top::<NI_R, NJ_R>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_r
    );
    reflect_bc_eq_bottom::<NI_R, NJ_R>(
      &self.param, &self.dist.w, 
      &self.dist.cx, &self.dist.cy, 
      &mut self.dist.f_r
    );
    //reflect_bc_eq_right::<NI_R, NJ_R>(
    //  &self.param, 
    //  &self.dist.w, &self.dist.cx, &self.dist.cy, 
    //  &mut self.dist.f_r
    //);
    reflect_bc_conv_right::<NI_R, NJ_R>(
      &self.basic.u_r, &self.dist.w, &mut self.dist.f_r
    );
    // left: between the (right and top) || (right and bottom) block
    for j in 0..(NJ_B-1) {
      for k in 0..NC {
        self.dist.f_r[j][0][k] = self.dist.f_bot[j][NI_B-2][k];
      }
    }
    for j in 1..NJ_T {
      for k in 0..NC {
        self.dist.f_r[j + NJ_B + NJ_M_C - 4][0][k] = self.dist.f_top[j][NI_T-2][k];
      }
    }
  }

  #[inline]
  fn make_interpolation(&mut self) {
    interpolate_space::<NI_T, NI_M, RAT_M, N_SOLV_TB>(
      &self.interp.l_tb, &self.interp.u_tb, 
      &self.dist.f_top[1], 
      &mut self.interp.ord_top, 
      &mut self.interp.f_top
    );
    interpolate_space::<NI_T, NI_M, RAT_M, N_SOLV_TB>(
      &self.interp.l_tb, &self.interp.u_tb, 
      &self.dist.f_bot[NJ_B-2], 
      &mut self.interp.ord_bot, 
      &mut self.interp.f_bot
    );
    let mut f_tmp = [[0.0; NC]; NJ_M_C];
    for j in 0..NJ_M_C {
      for k in 0..NC {
        f_tmp[j][k] = self.dist.f_l[j + NJ_B - 2][NI_L-2][k];
      }
    }
    interpolate_space::<NJ_M_C, NJ_M, RAT_M, N_SOLV_LR>(
      &self.interp.l_lr, &self.interp.u_lr, 
      &f_tmp, 
      &mut self.interp.ord_l, 
      &mut self.interp.f_l
    );
    for j in 0..NJ_M_C {
      for k in 0..NC {
        f_tmp[j][k] = self.dist.f_r[j + NJ_B - 2][1][k];
      }
    }
    interpolate_space::<NJ_M_C, NJ_M, RAT_M, N_SOLV_LR>(
      &self.interp.l_lr, &self.interp.u_lr, 
      &f_tmp, 
      &mut self.interp.ord_r, 
      &mut self.interp.f_r
    );
  }

  #[inline]
  fn set_bc_main(&mut self, n: usize) {
    if n != RAT_M - 1 {
      let mut f_tmp = [[0.0; NC]; NI_M];
      interpolate_time::<NI_M, RAT_M>(
        n as f64 + 1.0, 
        &self.interp.ord_top, 
        &self.interp.f_top, 
        &mut f_tmp
      );
      convert_c_to_f::<NI_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &f_tmp, 
        self.param.tau_f, 
        &mut self.dist.f_main[NJ_M-1]
      );
      interpolate_time::<NI_M, RAT_M>(
        n as f64 + 1.0, 
        &self.interp.ord_bot, 
        &self.interp.f_bot, 
        &mut f_tmp
      );
      convert_c_to_f::<NI_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &f_tmp, 
        self.param.tau_f, 
        &mut self.dist.f_main[0]
      );
      let mut f_tmp = [[0.0; NC]; NJ_M];
      let mut f_converted = [[0.0; NC]; NJ_M];
      interpolate_time::<NJ_M, RAT_M>(
        n as f64 + 1.0, 
        &self.interp.ord_l, 
        &self.interp.f_l, 
        &mut f_tmp
      );
      convert_c_to_f::<NJ_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &f_tmp, 
        self.param.tau_f, 
        &mut f_converted
      );
      for j in 0..NJ_M {
        for k in 0..NC {
          self.dist.f_main[j][0][k] = f_converted[j][k]
        }
      }
      interpolate_time::<NJ_M, RAT_M>(
        n as f64 + 1.0, 
        &self.interp.ord_r, 
        &self.interp.f_r, 
        &mut f_tmp
      );
      convert_c_to_f::<NJ_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &f_tmp, 
        self.param.tau_f, 
        &mut f_converted
      );
      for j in 0..NJ_M {
        for k in 0..NC {
          self.dist.f_main[j][NI_M-1][k] = f_converted[j][k]
        }
      }
    } else {
      convert_c_to_f::<NI_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &self.interp.f_top[self.interp.ord_top[0]], 
        self.param.tau_f, 
        &mut self.dist.f_main[NJ_M-1]
      );
      convert_c_to_f::<NI_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &self.interp.f_bot[self.interp.ord_bot[0]], 
        self.param.tau_f, 
        &mut self.dist.f_main[0]
      );
      let mut f_converted = [[0.0; NC]; NJ_M];
      convert_c_to_f::<NJ_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &self.interp.f_l[self.interp.ord_l[0]], 
        self.param.tau_f, 
        &mut f_converted
      );
      for j in 0..NJ_M {
        for k in 0..NC {
          self.dist.f_main[j][0][k] = f_converted[j][k];
        }
      }
      convert_c_to_f::<NJ_M>(
        &self.dist.cx, &self.dist.cy, &self.dist.w, 
        self.param.tau_c, 
        &self.interp.f_r[self.interp.ord_r[0]], 
        self.param.tau_f, 
        &mut f_converted
      );
      for j in 0..NJ_M {
        for k in 0..NC {
          self.dist.f_main[j][NI_M-1][k] = f_converted[j][k];
        }
      }
    }
  }

  #[inline]
  fn reflect_result_of_main(&mut self) {
    let mut f_tmp = [[0.0; NC]; NI_T];
    // top
    for i in 0..NI_T {
      for k in 0..NC {
        f_tmp[i][k] = self.dist.f_main[NJ_M-1-RAT_M][RAT_M * i][k];
      }
    }
    convert_f_to_c::<NI_T>(
      &self.dist.cx, &self.dist.cy, &self.dist.w, 
      self.param.tau_f, 
      &f_tmp, 
      self.param.tau_c, 
      &mut self.dist.f_top[0]
    );
    // bottom
    for i in 0..NI_T {
      for k in 0..NC {
        f_tmp[i][k] = self.dist.f_main[RAT_M][RAT_M *i][k];
      }
    }
    convert_f_to_c::<NI_T>(
      &self.dist.cx, &self.dist.cy, &self.dist.w, 
      self.param.tau_f, 
      &f_tmp, 
      self.param.tau_c, 
      &mut self.dist.f_bot[NJ_B-1]
    );
    let mut f_tmp = [[0.0; NC]; NJ_M_C];
    let mut f_converted = [[0.0; NC]; NJ_M_C];
    // left
    for j in 0..NJ_M_C {
      for k in 0..NC {
        f_tmp[j][k] = self.dist.f_main[RAT_M * j][RAT_M][k];
      }
    }
    convert_f_to_c::<NJ_M_C>(
      &self.dist.cx, &self.dist.cy, &self.dist.w, 
      self.param.tau_f, 
      &f_tmp, 
      self.param.tau_c, 
      &mut f_converted
    );
    for j in 0..NJ_M_C {
      for k in 0..NC {
        self.dist.f_l[j + NJ_B - 2][NI_L-1][k] = f_converted[j][k];
      }
    }
    // right
    for j in 0..NJ_M_C {
      for k in 0..NC {
        f_tmp[j][k] = self.dist.f_main[RAT_M * j][NI_M - 1 - RAT_M][k];
      }
    }
    convert_f_to_c::<NJ_M_C>(
      &self.dist.cx, &self.dist.cy, &self.dist.w, 
      self.param.tau_f, 
      &f_tmp, 
      self.param.tau_c, 
      &mut f_converted
    );
    for j in 0..NJ_M_C {
      for k in 0..NC {
        self.dist.f_r[j + NJ_B - 2][0][k] = f_converted[j][k];
      }
    }
  }
}