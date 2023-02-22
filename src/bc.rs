use crate::parameters::RAT_M;
use crate::{parameters::NC, Param};
use crate::distribution::{calc_feq_cell, calc_basic_cell};

// give the local equilibrium distribution to numerical boundaries: i=0, j=0, NJ
#[inline]
pub fn reflect_bc_eq_top<const NI: usize, const NJ: usize>(
  param: &Param,
  w: &[f64; NC],
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  let u = param.ma/3.0f64.sqrt();
  let v = 0.0;
  for i in 1..(NI-1) {
    let rho = 1.0 / (1.0 + v) * (
      f[NJ-1][i][0] + f[NJ-1][i][1] + f[NJ-1][i][3]
      + 2.0 * (f[NJ-1][i][2] + f[NJ-1][i][5] + f[NJ-1][i][6]));
    calc_feq_cell(rho, u, v, cx, cy, w, &mut f[NJ-1][i]);
  }
  calc_feq_cell(1.0, u, v, cx, cy, w, &mut f[NJ-1][0]);
  calc_feq_cell(1.0, u, v, cx, cy, w, &mut f[NJ-1][NI-1]);
}

#[inline]
pub fn reflect_bc_eq_bottom<const NI: usize, const NJ: usize>(
  param: &Param,
  w: &[f64; NC],
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  let u = param.ma/3.0f64.sqrt();
  let v = 0.0;
  for i in 1..(NI-1) {
    let rho = 1.0 / (1.0 - v) * (
      f[0][i][0] + f[0][i][1] + f[0][i][3]
      + 2.0 * (f[0][i][4] + f[0][i][7] + f[0][i][8]));
    calc_feq_cell(rho, u, v, cx, cy, w, &mut f[0][i]);
  }
  calc_feq_cell(1.0, u, v, cx, cy, w, &mut f[0][0]);
  calc_feq_cell(1.0, u, v, cx, cy, w, &mut f[0][NI-1]);
}

#[inline]
pub fn reflect_bc_eq_left<const NI: usize, const NJ: usize>(
  param: &Param,
  w: &[f64; NC],
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  let u = param.ma/3.0f64.sqrt();
  let v = 0.0;
  for j in 1..(NJ-1) {
    let rho = 1.0 / (1.0 - u) * (
      f[j][0][0] + f[j][0][2] + f[j][0][4]
      + 2.0 * (f[j][0][3] + f[j][0][6] + f[j][0][7]));
    calc_feq_cell(rho, u, v, cx, cy, w, &mut f[j][0]);
  }
}

//#[inline]
//pub fn reflect_bc_eq_right<const NI: usize, const NJ: usize>(
//  param: &Param,
//  w: &[f64; NC],
//  cx: &[f64; NC],
//  cy: &[f64; NC],
//  f: &mut [[[f64; NC]; NI]; NJ]
//) {
//  let u = param.ma/3.0f64.sqrt();
//  let v = 0.0;
//  for j in 1..(NJ-1) {
//    let rho = 1.0 / (1.0 + u) * (
//      f[j][NI-1][0] + f[j][NI-1][2] + f[j][NI-1][4]
//      + 2.0 * (f[j][NI-1][1] + f[j][NI-1][5] + f[j][NI-1][8]));
//    calc_feq_cell(rho, u, v, cx, cy, w, &mut f[j][NI-1]);
//  }
//}

// convective condition (Yang, 2013) for outflow boundary: i = NI
#[inline]
pub fn reflect_bc_conv_right<const NI: usize, const NJ: usize>(
  u: &[[f64; NI]; NJ],
  w: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  let mut u_mean = 0.0;
  for j in 1..(NJ-1) {
    u_mean += u[j][NI-1];
  }
  u_mean /= (NJ - 2) as f64;
  for j in 1..(NJ-1) {
    let du = -0.5 * u_mean * (
      3.0 * u[j][NI-1] - 4.0 * u[j][NI-2] + u[j][NI-3]);
    for k in [3, 6, 7] {
      f[j][NI-1][k] -= 3.0 * w[k] * du
    }
  }
}

//#[inline]
//pub fn reflect_bc_expolat_right<const NI: usize, const NJ: usize>(
//  rho: &[[f64; NI]; NJ],
//  u: &[[f64; NI]; NJ],
//  v: &[[f64; NI]; NJ],
//  f: &mut [[[f64; NC]; NI]; NJ]
//) {
//  for j in 1..(NJ-1) {
//    let cu_1 = u[j][NI-1];
//    let cu_3 = -u[j][NI-1];
//    let uu = u[j][NI-1] * u[j][NI-1] + v[j][NI-1] * v[j][NI-1];
//    let feq_1 = 1.0 / 9.0 * rho[j][NI-1] * (1.0 + 3.0 * cu_1 + 4.5 * cu_1 * cu_1 - 1.5 * uu);
//    let feq_3 = 1.0 / 9.0 * rho[j][NI-1] * (1.0 + 3.0 * cu_3 + 4.5 * cu_3 * cu_3 - 1.5 * uu);
//    f[j][NI-1][3] = feq_3 + f[j][NI-1][3] - feq_1;
//    for k in [6, 7] {
//      f[j][NI-1][k] = 2.0 * f[j][NI-2][k] - f[j][NI-3][k];
//    }
//  }
//}

// bc between multi-blocks
#[inline]
pub fn convert_f_to_c<const NI: usize>(
  cx: &[f64; NC],
  cy: &[f64; NC],
  w: &[f64; NC],
  tau_f: f64, 
  f_f: &[[f64; NC]; NI], 
  tau_c: f64, 
  f_c: &mut [[f64; NC]; NI]
) {
  for i in 0..NI {
    let (rho, u, v) = calc_basic_cell(cx, cy, &f_f[i]);
    let mut feq = [0.0; NC];
    calc_feq_cell(rho, u, v, cx, cy, w, &mut feq);
    for k in 0..NC {
      f_c[i][k] = feq[k] + RAT_M as f64 * (tau_c - 1.0) / (tau_f - 1.0) * (f_f[i][k] - feq[k]);
    }
  }
}
pub fn convert_c_to_f<const NI: usize>(
  cx: &[f64; NC],
  cy: &[f64; NC],
  w: &[f64; NC],
  tau_c: f64, 
  f_c: &[[f64; NC]; NI], 
  tau_f: f64, 
  f_f: &mut [[f64; NC]; NI]
) {
  for i in 0..NI {
    let (rho, u, v) = calc_basic_cell(cx, cy, &f_c[i]);
    let mut feq = [0.0; NC];
    calc_feq_cell(rho, u, v, cx, cy, w, &mut feq);
    for k in 0..NC {
      f_f[i][k] = feq[k] + (tau_f - 1.0) / (tau_c - 1.0) / RAT_M as f64 * (f_c[i][k] - feq[k]);
    }
  }
}