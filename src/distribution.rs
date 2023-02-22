use crate::parameters::NC;

#[inline]
pub fn calc_feq<const NI: usize, const NJ: usize>(
  rho: &[[f64; NI]; NJ],
  u: &[[f64; NI]; NJ],
  v: &[[f64; NI]; NJ],
  w: &[f64; NC],
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  for j in 0..NJ {
    for i in 0..NI {
      let uu = u[j][i] * u[j][i] + v[j][i] * v[j][i];
      for k in 0..NC {
        let cu = cx[k] * u[j][i] + cy[k] * v[j][i];
        f[j][i][k] = w[k] * rho[j][i]
          * (1.0 + 3.0 * cu + 9.0 / 2.0 * cu * cu - 3.0 / 2.0 * uu);
      }
    }
  }
}

#[inline]
pub fn calc_feq_cell(
  rho: f64, 
  u: f64, 
  v: f64, 
  cx: &[f64; NC], 
  cy: &[f64; NC], 
  w: &[f64; NC], 
  f: &mut [f64; NC]
) {
  let uu = u * u + v * v;
  for k in 0..NC {
    let cu = cx[k] * u + cy[k] * v;
    f[k] = w[k] * rho
      * (1.0 + 3.0 * cu + 9.0 / 2.0 * cu * cu - 3.0 / 2.0 * uu);
  }
}

#[inline]
pub fn calc_basic<const NI: usize, const NJ: usize>(
  cx: &[f64; NC], 
  cy: &[f64; NC], 
  f: &[[[f64; NC]; NI]; NJ],
  rho: &mut [[f64; NI]; NJ],
  u: &mut [[f64; NI]; NJ],
  v: &mut [[f64; NI]; NJ],
) {
  for j in 0..NJ {
    for i in 0..NI {
      rho[j][i] = 0.0;
      u[j][i] = 0.0;
      v[j][i] = 0.0;
      for k in 0..NC {
        rho[j][i] += f[j][i][k];
        u[j][i] += f[j][i][k] * cx[k];
        v[j][i] += f[j][i][k] * cy[k];
      }
      u[j][i] /= rho[j][i];
      v[j][i] /= rho[j][i];
    }
  }
}

#[inline]
pub fn calc_basic_cell(
  cx: &[f64; NC], 
  cy: &[f64; NC], 
  f: &[f64; NC]
) -> (f64, f64, f64) {
  let mut rho = 0.0;
  let mut u = 0.0;
  let mut v = 0.0;
  for k in 0..NC {
    rho += f[k];
    u += f[k] * cx[k];
    v += f[k] * cy[k];
  }
  u /= rho;
  v /= rho;
  (rho, u, v)
}