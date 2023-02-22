use crate::parameters::NC;

#[inline]
fn convert_f_to_cm(f: &[f64; NC], cx: &[f64; NC], cy: &[f64; NC], u: f64, v: f64, cm: &mut [f64; NC]) {
  //*cm = [0.0; NC];
  for k in 0..f.len() {
    let mx = cx[k] - u;
    let my = cy[k] - v;
    //cm[0] += f[k];
    //cm[1] += f[k] * mx;
    //cm[2] += f[k] * my;
    cm[3] += f[k] * (mx * mx + my * my);
    cm[4] += f[k] * (mx * mx - my * my);
    cm[5] += f[k] * mx * my;
    cm[6] += f[k] * mx * mx * my;
    cm[7] += f[k] * mx * my * my;
    cm[8] += f[k] * mx * mx * my * my;
  }
}

#[inline]
fn calc_cmeq(rho: f64, u: f64, v: f64, cmeq: &mut [f64; NC]) {
  //cmeq[0] = rho;
  //cmeq[1] = 0.0;
  //cmeq[2] = 0.0;
  cmeq[3] = 2.0 / 3.0 * rho;
  cmeq[4] = 0.0;
  cmeq[5] = 0.0;
  cmeq[6] = -rho * u * u * v;
  cmeq[7] = -rho * u * v * v;
  cmeq[8] = rho * (1.0 / 9.0 + 3.0 * u * u * v * v);
}

#[inline]
fn convert_cm_to_f(cm: &[f64; NC], rho: f64, u: f64, v: f64, f: &mut [f64; NC]) {
  f[0] =   cm[3] * 0.5 * (-2.0 + u * u + v * v)
         + cm[4] * 0.5 * (-u * u + v * v)
         + cm[5] * 4.0 * u * v
         + cm[6] * 2.0 * v
         + cm[7] * 2.0 * u
         + cm[8]
         + rho * (1.0 - u * u - v * v + u * u * v * v);
  f[1] =   cm[3] * 0.25 * (1.0 - u - u * u - v * v)
         + cm[4] * 0.25 * (1.0 + u + u * u - v * v)
         + cm[5] * (-v - 2.0 * u * v)
         - cm[6] * v
         + cm[7] * 0.5 * (-1.0 - 2.0 * u)
         - cm[8] * 0.5
         + 0.5 * rho * (u + u * u - u * v * v - u * u * v * v);
  f[2] =   cm[3] * 0.25 * (1.0 - v - u * u - v * v)
         + cm[4] * 0.25 * (-1.0 - v + u * u - v * v)
         + cm[5] * (-u - 2.0 * u * v)
         + cm[6] * 0.5 * (-1.0 - 2.0 * v)
         - cm[7] * u
         - cm[8] * 0.5
         + 0.5 * rho * (v + v * v - u * u * v - u * u * v * v);
  f[3] =   cm[3] * 0.25 * (1.0 + u - u * u - v * v)
         + cm[4] * 0.25 * (1.0 - u + u * u + v * v)
         + cm[5] * (v - 2.0 * u * v)
         - cm[6] * v
         + cm[7] * 0.5 * (1.0 - 2.0 * u)
         - cm[8] * 0.5
         + 0.5 * rho * (-u + u * u + u * v * v - u * u * v * v);
  f[4] =   cm[3] * 0.25 * (1.0 + v - u * u - v * v)
         + cm[4] * 0.25 * (-1.0 + v + u * u - v * v)
         + cm[5] * (u - 2.0 * u * v)
         + cm[6] * 0.5 * (1.0 - 2.0 * v)
         - cm[7] * u
         - cm[8] * 0.5
         + 0.5 * rho * (-v + v * v + u * u * v - u * u * v * v);
  f[5] =   cm[3] * 0.125 * (u + v + u * u + v * v)
         + cm[4] * 0.125 * (-u + v - u * u + v * v)
         + cm[5] * 0.25 * (1.0 + 2.0 * u + 2.0 * v + 4.0 * u * v)
         + cm[6] * 0.25 * (1.0 + 2.0 * v)
         + cm[7] * 0.25 * (1.0 + 2.0 * u)
         + cm[8] * 0.25
         + 0.25 * rho * (u * v + u * u * v + u * v * v + u * u * v * v);
  f[6] =   cm[3] * 0.125 * (-u + v + u * u + v * v)
         + cm[4] * 0.125 * (u + v - u * u + v * v)
         + cm[5] * 0.25 * (-1.0 + 2.0 * u - 2.0 * v + 4.0 * u * v)
         + cm[6] * 0.25 * (1.0 + 2.0 * v)
         + cm[7] * 0.25 * (-1.0 + 2.0 * u)
         + cm[8] * 0.25
         + 0.25 * rho * (-u * v + u * u * v - u * v * v + u * u * v * v);
  f[7] =   cm[3] * 0.125 * (-u - v + u * u + v * v)
         + cm[4] * 0.125 * (u - v - u * u + v * v)
         + cm[5] * 0.25 * (1.0 - 2.0 * u - 2.0 * v + 4.0 * u * v)
         + cm[6] * 0.25 * (-1.0 + 2.0 * v)
         + cm[7] * 0.25 * (-1.0 + 2.0 * u)
         + cm[8] * 0.25
         + 0.25 * rho * (u * v - u * u * v - u * v * v + u * u * v * v);
  f[8] =   cm[3] * 0.125 * (u - v + u * u + v * v)
         + cm[4] * 0.125 * (-u - v - u * u + v * v)
         + cm[5] * 0.25 * (-1.0 - 2.0 * u + 2.0 * v + 4.0 * u * v)
         + cm[6] * 0.25 * (-1.0 + 2.0 * v)
         + cm[7] * 0.25 * (1.0 + 2.0 * u)
         + cm[8] * 0.25
         + 0.25 * rho * (-u * v - u * u * v + u * v * v + u * u * v * v);
}

pub fn calc_collision<const NI: usize, const NJ: usize>(
  tau: f64,
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &[[[f64; NC]; NI]; NJ],
  rho: &[[f64; NI]; NJ],
  u: &[[f64; NI]; NJ],
  v: &[[f64; NI]; NJ],
  f_str: &mut [[[f64; NC]; NI]; NJ],
) {
  for j in 0..NJ {
    for i in 0.. NI {
      let mut cm = [0.0; NC];
      convert_f_to_cm(&f[j][i], cx, cy, u[j][i], v[j][i], &mut cm);
      let mut cmeq = [0.0; NC];
      calc_cmeq(rho[j][i], u[j][i], v[j][i], &mut cmeq);
      let s = [1.0, 1.0, 1.0, 1.0, 1.0/tau, 1.0/tau, 1.0, 1.0, 1.0];
      for k in 3..9 {
        cm[k] -= s[k] * (cm[k] - cmeq[k]);
      }
      convert_cm_to_f(&cm, rho[j][i], u[j][i], v[j][i], &mut f_str[j][i]);
    }
  }
}