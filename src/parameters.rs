/// number of discretized velocity
pub const NC: usize = 9;
/// ratio of grid widths of coarse and fine blocks
pub const RAT_M: usize = 2;
/// number of nodes in i direction of the top block
pub const NI_T: usize = 90;
/// number of nodes in j direction of the top block
pub const NJ_T: usize = NI_T;
/// number of nodes in i direction of the bottom block
pub const NI_B: usize = NI_T;
/// number of nodes in j direction of the bottom block
pub const NJ_B: usize = NJ_T;
/// number of nodes in i direction of the left block
pub const NI_L: usize = NI_T / 2;
/// number of nodes in j direction of the coarse main block
pub const NJ_M_C: usize = NI_T / 2;
/// number of nodes in j direction of the left block
pub const NJ_L: usize = NJ_M_C + NJ_T + NJ_B - 4;
/// number of nodes in i direction of the right block
pub const NI_R: usize = NI_T;
/// number of nodes in j direction of the right block
pub const NJ_R: usize = NJ_L;
/// number of nodes in i direction of the fine main block
pub const NI_M: usize = (NI_T - 1) * RAT_M + 1;
/// number of nodes in j direction of the fine main block
pub const NJ_M: usize = (NJ_M_C - 1) * RAT_M + 1;
/// ratio of grid width of main block and array for plot
pub const RAT_PLOT: usize = 2;
/// length in i direction of the array for plot
pub const NI_PLOT: usize = NI_M * RAT_PLOT;
/// length in j direction of the array for plot
pub const NJ_PLOT: usize = NJ_M * RAT_PLOT;
/// total number of elements of the array for plot
pub const N_RGB: usize = NI_PLOT * NJ_PLOT * 3;
/// length scale in number of nodes
pub const L_SCALE: f64 = NJ_M as f64 / 4.0;
/// number of nodes to be solved at spline interpolation
pub const N_SOLV_TB: usize = NI_T - 2;
/// number of nodes to be solved at spline interpolation
pub const N_SOLV_LR: usize = NJ_M_C - 2;