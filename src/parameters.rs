/// number of discretized velocity
pub const NC: usize = 9;

pub const RAT_M: usize = 2;

pub const NI_T: usize = 90;
pub const NJ_T: usize = NI_T;

pub const NI_B: usize = NI_T;
pub const NJ_B: usize = NJ_T;

pub const NI_L: usize = NI_T / 2;
pub const NJ_M_C: usize = NI_T / 2;
pub const NJ_L: usize = NJ_M_C + NJ_T + NJ_B - 4;

pub const NI_R: usize = NI_T;
pub const NJ_R: usize = NJ_L;

pub const NI_M: usize = (NI_T - 1) * RAT_M + 1;
pub const NJ_M: usize = (NJ_M_C - 1) * RAT_M + 1;

pub const RAT_PLOT: usize = 2;
pub const NI_PLOT: usize = NI_M * RAT_PLOT;
pub const NJ_PLOT: usize = NJ_M * RAT_PLOT;
pub const N_RGB: usize = NI_PLOT * NJ_PLOT * 3;

//pub const N_NOISE: usize = 64;
//pub const M_BG: usize = 32;

pub const L_SCALE: f64 = NJ_M as f64 / 4.0;

pub const N_SOLV_TB: usize = NI_T - 2;
pub const N_SOLV_LR: usize = NJ_M_C - 2;