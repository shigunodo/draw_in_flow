use crate::{Boundary, Cell};
use crate::parameters::NC;

#[inline]
fn inv(k: usize) -> usize {
  match k {
    0 => 0,
    1 => 3,
    2 => 4,
    3 => 1,
    4 => 2,
    5 => 7,
    6 => 8,
    7 => 5,
    8 => 6,
    _ => panic!("fn inv(k): given k is out of bounds."),
  }
}

#[inline]
pub fn calc_advection_with_phys_bc<const NI: usize, const NJ: usize>(
  bound: &Boundary,
  f_str: &[[[f64; NC]; NI]; NJ], 
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  // calculation of advection
  for j in 0..NJ {
    for i in 0..NI {
      if bound.cell[j][i] == Cell::Alive {
        for k in 0..NC {
          let i_new = i as i32 + cx[k] as i32;
          let j_new = j as i32 + cy[k] as i32;
          let i_new_is_ob = i_new < 0 || i_new >= NI as i32; // ob = out of bounds
          let j_new_is_ob = j_new < 0 || j_new >= NJ as i32;
          // the mapping of k=1 is skipped at right outflow boundary: only for extrapolation bc-scheme
          //let outflow_bc_normal = i_new == NI as i32 - 1 && k == 1;
          if i_new_is_ob || j_new_is_ob {}
          else if bound.cell[j_new as usize][i_new as usize] == Cell::Dead {
            // half way bounce back scheme for physical boundaries
            f[j][i][inv(k)] = f_str[j][i][k];
          }
          //else if outflow_bc_normal { // only for extrapolation bc-scheme
          //  f[j_new as usize][i_new as usize][inv(k)] = f[j_new as usize][i_new as usize][k];
          //  f[j_new as usize][i_new as usize][k] = f_str[j][i][k];
          //}
          else {
            f[j_new as usize][i_new as usize][k] = f_str[j][i][k];
          }
        }
      }
    }
  }
}

#[inline]
pub fn calc_advection<const NI: usize, const NJ: usize>(
  f_str: &[[[f64; NC]; NI]; NJ], 
  cx: &[f64; NC],
  cy: &[f64; NC],
  f: &mut [[[f64; NC]; NI]; NJ]
) {
  // calculation of advection
  for j in 0..NJ {
    for i in 0..NI {
      for k in 0..NC {
        let i_new = i as i32 + cx[k] as i32;
        let j_new = j as i32 + cy[k] as i32;
        let i_new_is_ob = i_new < 0 || i_new >= NI as i32; // ob = out of bounds
        let j_new_is_ob = j_new < 0 || j_new >= NJ as i32;
        // the mapping of k=1 is skipped at right outflow boundary: only for extrapolation bc-scheme
        //let outflow_bc_normal = i_new == NI as i32 - 1 && k == 1;
        if i_new_is_ob || j_new_is_ob {}
        //else if outflow_bc_normal { // only for extrapolation bc-scheme
        //  f[j_new as usize][i_new as usize][inv(k)] = f[j_new as usize][i_new as usize][k];
        //  f[j_new as usize][i_new as usize][k] = f_str[j][i][k];
        //}
        else {
          f[j_new as usize][i_new as usize][k] = f_str[j][i][k];
        }
      }
    }
  }
}