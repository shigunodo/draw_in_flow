use std::f64;
use crate::LIC;
use crate::jitter::JITTER;
use crate::parameters::{NI_M, NJ_M, NI_PLOT, NJ_PLOT, RAT_PLOT};

#[inline]
fn advect(i: usize, j: usize, u: f64, v: f64, px: f64, py: f64) -> (i32, i32, f64, f64, f64) {
  let u_is_positive = u >= 0.0;
  let v_is_positive = v >= 0.0;
  if u_is_positive && v_is_positive {
    let (px_new_t, py_new_t, s_t) = advect_top(j, u, v, px, py);
    let (px_new_r, py_new_r, s_r) = advect_right(i, u, v, px, py);
    if s_t < s_r {
      return (i as i32, j as i32 + 1, px_new_t, py_new_t, s_t)
    } else {
      return (i as i32 + 1, j as i32, px_new_r, py_new_r, s_r)
    }
  } else if (!u_is_positive) && v_is_positive {
    let (px_new_t, py_new_t, s_t) = advect_top(j, u, v, px, py);
    let (px_new_l, py_new_l, s_l) = advect_left(i, u, v, px, py);
    if s_t < s_l {
      return (i as i32, j as i32 + 1, px_new_t, py_new_t, s_t)
    } else {
      return (i as i32 - 1, j as i32, px_new_l, py_new_l, s_l)
    }
  } else if u_is_positive && (!v_is_positive) {
    let (px_new_b, py_new_b, s_b) = advect_bottom(j, u, v, px, py);
    let (px_new_r, py_new_r, s_r) = advect_right(i, u, v, px, py);
    if s_b < s_r {
      return (i as i32, j as i32 - 1, px_new_b, py_new_b, s_b)
    } else {
      return (i as i32 + 1, j as i32, px_new_r, py_new_r, s_r)
    }
  } else {
    let (px_new_b, py_new_b, s_b) = advect_bottom(j, u, v, px, py);
    let (px_new_l, py_new_l, s_l) = advect_left(i, u, v, px, py);
    if s_b < s_l {
      return (i as i32, j as i32 - 1, px_new_b, py_new_b, s_b)
    } else {
      return (i as i32 - 1, j as i32, px_new_l, py_new_l, s_l)
    }
  }
}

#[inline]
fn advect_top(j: usize, u: f64, v: f64, px: f64, py: f64) -> (f64, f64, f64) {
  let py_new = j as f64 + 1.0;
  let sy = py_new - py;
  let sx = sy * u / v;
  let px_new = px + sx;
  let s = (sx * sx + sy * sy).sqrt();
  return (px_new, py_new, s)
}
#[inline]
fn advect_right(i: usize, u: f64, v: f64, px: f64, py: f64) -> (f64, f64, f64) {
  let px_new = i as f64 + 1.0;
  let sx = px_new - px;
  let sy = sx * v / u;
  let py_new = py + sy;
  let s = (sx * sx + sy * sy).sqrt();
  (px_new, py_new, s)
}
#[inline]
fn advect_left(i: usize, u: f64, v: f64, px: f64, py: f64) -> (f64, f64, f64) {
  let px_new = i as f64;
  let sx = px_new - px;
  let sy = sx * v / u;
  let py_new = py + sy;
  let s = (sx * sx + sy * sy).sqrt();
  return (px_new, py_new, s)
}
#[inline]
fn advect_bottom(j: usize, u: f64, v: f64, px: f64, py: f64) -> (f64, f64, f64) {
  let py_new = j as f64;
  let sy = py_new - py;
  let sx = sy * u / v;
  let px_new = px + sx;
  let s = (sx * sx + sy * sy).sqrt();
  return (px_new, py_new, s)
}

// Hanning (ripple) function windowed by Hanning function as the kernel
//#[inline]
//fn integral_of_kernel(smin: f64, smax: f64, phase: f64, l: f64) -> f64 {
//  let c = PI / l;
//  let d = 3.0 * PI / l;
//  0.25 * (smax - smin + ((smax * c).sin() - (smin * c).sin()) / c
//          + ((smax * d + phase).sin() - (smin * d + phase).sin()) / d
//          + ((smax * (c - d) - phase).sin() - (smin * (c - d) - phase).sin()) * 0.5 / (c - d)
//          + ((smax * (c + d) + phase).sin() - (smin * (c + d) + phase).sin()) * 0.5 / (c + d))
//}

// Ramp-like kernel function
// Reference: R. Wegenkittl, E. Groller and W. Purgathofer. (1997). Proceedings. Computer Animation '97, 15-21. DOI 10.1109/CA.1997.601035.
#[inline]
fn integral_of_kernel(smin: f64, smax: f64, phase: f64, l: f64) -> f64 {
  let a = smin + phase * l;
  let b = smax + phase * l;
  if a < l && b < l {
    return b - a - (b * b - a * a) * 0.5 / l
  } else if a < l && b >= l {
    return b - a - (2.0 * l * l - a * a + b * b - 2.0 * b * l) * 0.5 / l
  } else {
    return b - a - (b * b - a * a - 2.0 * l * (b - a)) * 0.5 / l
  }
}

#[inline]
fn convolute(
  i: usize, j: usize, 
  u: &[[f64; NI_M]; NJ_M], v: &[[f64; NI_M]; NJ_M], 
  l: f64, phase: f64, noise: &[[f64; NI_PLOT]; NJ_PLOT]
) -> f64 {
  let mut i_now = i;
  let mut j_now = j;
  let mut px = i as f64 + 0.5;
  let mut py = j as f64 + 0.5;
  let mut s_now = 0.0;
  let mut h_sum = 0.0;
  let mut f = 0.0;
  let mut flag = 0usize;
  while s_now < l {
    let i_new;
    let j_new;
    let ds;
    (i_new, j_new, px, py, ds) = advect(i_now, j_now, u[j_now / RAT_PLOT][i_now / RAT_PLOT], v[j_now / RAT_PLOT][i_now / RAT_PLOT], px, py);
    if ds < 1.0e-2 {
      flag += 1;
    }
    let s_new = s_now + ds;
    let h = integral_of_kernel(s_now, s_new, phase,  l);
    f += noise[j_now][i_now] * h;
    h_sum += h;
    let i_is_ib = i_new >= 0 && i_new < NI_PLOT as i32;
    let j_is_ib = j_new >= 0 && j_new < NJ_PLOT as i32;
    if i_is_ib && j_is_ib {
      i_now = i_new as usize;
      j_now = j_new as usize;
      s_now = s_new;
    } else {
      s_now = 10.0 * l;
    }
    if flag > 4 {
      break;
    }
  }
  //let mut i_now = i;
  //let mut j_now = j;
  //let mut px = i as f64 + 0.5;
  //let mut py = j as f64 + 0.5;
  //let mut s_now = 0.0;
  //let mut h_inv_sum = 0.0;
  //let mut flag = 0usize;
  //while s_now < l {
  //  let i_new;
  //  let j_new;
  //  let ds;
  //  (i_new, j_new, px, py, ds) = advect(i_now, j_now, -u[j_now][i_now], -v[j_now][i_now], px, py);
  //  if ds < 1.0e-2 {
  //    flag += 1;
  //  }
  //  let s_new = s_now + ds;
  //  let h = integral_of_kernel(s_now, s_new, phase, l);
  //  f += noise[j_now][i_now] * h;
  //  h_inv_sum += h;
  //  let i_is_ib = i_new >= 0 && i_new < NI_M as i32;
  //  let j_is_ib = j_new >= 0 && j_new < NJ_M as i32;
  //  if i_is_ib && j_is_ib {
  //    i_now = i_new as usize;
  //    j_now = j_new as usize;
  //    s_now = s_new;
  //  } else {
  //    s_now = 10.0 * l;
  //  }
  //  if flag > 4 {
  //    break;
  //  }
  //}
  f / h_sum
}


impl LIC {
  pub fn calc(&mut self, u: &[[f64; NI_M]; NJ_M], v: &[[f64; NI_M]; NJ_M], ma: f64, dt: f64, progress_lic_phase: u8) {
    let u_inf = ma / 3.0f64.sqrt();
    let l_inf = 15.0;
    if progress_lic_phase == 1 {
      self.phase += u_inf * dt * RAT_PLOT as f64 / l_inf;
      if self.phase >= 1.0 {
        self.phase -= 1.0;
      }
    }
    for j in 0..NJ_PLOT {
      for i in 0..NI_PLOT {
        //let lmin = 5.0;
        let mut l = (u[j / RAT_PLOT][i / RAT_PLOT] * u[j / RAT_PLOT][i / RAT_PLOT] 
                  + v[j / RAT_PLOT][i / RAT_PLOT] * v[j / RAT_PLOT][i / RAT_PLOT]).sqrt()
                  / u_inf * l_inf;
        if l > l_inf * 2.0 {
          l = l_inf * 2.0;
        }
        //else if l < lmin {
        //  l = lmin;
        //}
        let mut f = convolute(i, j, u, v, l, self.phase, &JITTER);
        f *= 5.0;
        if f > 1.0 {
          f = 1.0;
        }
        let fm1 = f - 1.0;
        self.f[j][i] = 1.0 - fm1 * fm1;
      }
    }
  }
}