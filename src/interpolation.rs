use crate::parameters::NC;

#[inline]
pub fn calc_lu<const N: usize>(
  l: &mut [f64; N], 
  u: &mut [f64; N]
) {
  u[0] = 4.0;
  for k in 1..N  {
    l[k] = 1.0 / u[k-1];
    u[k] = 4.0 - l[k];
  }
}

#[inline]
fn solve_linear<const N: usize>(
  l: &[f64; N], 
  u: &[f64; N],
  x: &mut [f64; N],
) {
  let mut y = [0.0; N];
  y[0] = x[0];
  for k in 1..N {
    y[k] = x[k] - l[k] * y[k-1];
  }
  x[N-1] = y[N-1] / u[N-1];
  for k in (0..(N-1)).rev() {
    x[k] = (y[k] - x[k+1]) / u[k];
  }
}

#[inline]
fn swap_ord(ord: &mut [usize; 3]) {
  let tmp = ord[2];
  ord[2] = ord[1];
  ord[1] = ord[0];
  ord[0] = tmp;
}

pub fn interpolate_space<const NI_C: usize, const NI_F: usize, const RAT_M: usize, const N_SOLV: usize>(
  l: &[f64; N_SOLV], 
  u: &[f64; N_SOLV],  
  f_c: &[[f64; NC]; NI_C], 
  ord: &mut [usize; 3],
  f_int: &mut [[[f64; NC]; NI_F]; 3]
) {
  swap_ord(ord);
  let dx_c = RAT_M as f64;
  for k in 0..NC {
    let mut rhs = [0.0; N_SOLV];
    for i in 1..(NI_C-1) {
      rhs[i-1] = 3.0 / dx_c / dx_c * (f_c[i+1][k] - 2.0 * f_c[i][k] + f_c[i-1][k]);
    }
    solve_linear::<N_SOLV>(l, u, &mut rhs);
    let mut c = [0.0; NI_C];
    for i in 1..(NI_C-1) {
      c[i] = rhs[i-1];
    }
    let mut b = [0.0; NI_C];
    let mut d = [0.0; NI_C];
    for i in 0..(NI_C-1) {
      b[i] = (f_c[i+1][k] - f_c[i][k]) / dx_c - dx_c / 3.0 * (2.0 * c[i] + c[i+1]);
      d[i] = (c[i+1] - c[i]) / 3.0 / dx_c;
    }
    for i_c in 0..(NI_C-1) {
      for i_f in 0..RAT_M {
        let dx = i_f as f64;
        f_int[ord[0]][i_c * RAT_M + i_f][k] = f_c[i_c][k] + b[i_c] * dx
                        + c[i_c] * dx * dx
                        + d[i_c] * dx * dx * dx;
      }
    }
    f_int[ord[0]][NI_F-1][k] = f_c[NI_C-1][k];
  }
}

pub fn interpolate_time<const NI_F: usize, const RAT_M: usize>(
  dt: f64,
  ord: &[usize; 3],
  f: &[[[f64; NC]; NI_F]; 3],
  f_int: &mut [[f64; NC]; NI_F]
) {
  let dt_c = RAT_M as f64;
  for i in 0..NI_F {
    for k in 0..NC {
      f_int[i][k] = (f[ord[0]][i][k] - 2.0 * f[ord[1]][i][k] + f[ord[2]][i][k])
                    * 0.5 / dt_c / dt_c * dt * dt
                    + (f[ord[0]][i][k] - f[ord[2]][i][k]) * 0.5 / dt_c * dt
                    + f[ord[1]][i][k];
    }
  }
}