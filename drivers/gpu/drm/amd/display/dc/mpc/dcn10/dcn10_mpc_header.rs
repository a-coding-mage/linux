/* Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency: declarations supplied by mpc.h and related headers.

#[macro_export]
macro_rules! TO_DCN10_MPC {
    ($mpc_base:expr) => { container_of!($mpc_base, dcn10_mpc, base) };
}

#[macro_export]
macro_rules! MPC_COMMON_REG_LIST_DCN1_0 {
    ($inst:expr) => { SRII!(MPCC_TOP_SEL, MPCC, $inst), SRII!(MPCC_BOT_SEL, MPCC, $inst), SRII!(MPCC_CONTROL, MPCC, $inst), SRII!(MPCC_STATUS, MPCC, $inst), SRII!(MPCC_OPP_ID, MPCC, $inst), SRII!(MPCC_BG_G_Y, MPCC, $inst), SRII!(MPCC_BG_R_CR, MPCC, $inst), SRII!(MPCC_BG_B_CB, MPCC, $inst), SRII!(MPCC_SM_CONTROL, MPCC, $inst), SRII!(MPCC_UPDATE_LOCK_SEL, MPCC, $inst) };
}

#[macro_export]
macro_rules! MPC_OUT_MUX_COMMON_REG_LIST_DCN1_0 {
    ($inst:expr) => { SRII!(MUX, MPC_OUT, $inst), VUPDATE_SRII!(CUR, VUPDATE_LOCK_SET, $inst) };
}

#[repr(C)]
pub struct dcn_mpc_registers {
    pub MPCC_TOP_SEL: [u32; MAX_MPCC], pub MPCC_BOT_SEL: [u32; MAX_MPCC],
    pub MPCC_CONTROL: [u32; MAX_MPCC], pub MPCC_STATUS: [u32; MAX_MPCC],
    pub MPCC_OPP_ID: [u32; MAX_MPCC], pub MPCC_BG_G_Y: [u32; MAX_MPCC],
    pub MPCC_BG_R_CR: [u32; MAX_MPCC], pub MPCC_BG_B_CB: [u32; MAX_MPCC],
    pub MPCC_SM_CONTROL: [u32; MAX_MPCC], pub MUX: [u32; MAX_OPP],
    pub MPCC_UPDATE_LOCK_SEL: [u32; MAX_MPCC], pub CUR: [u32; MAX_OPP],
}

#[repr(C)]
pub struct dcn_mpc_shift {
    pub MPCC_TOP_SEL: u8, pub MPCC_BOT_SEL: u8, pub MPCC_MODE: u8,
    pub MPCC_ALPHA_BLND_MODE: u8, pub MPCC_ALPHA_MULTIPLIED_MODE: u8,
    pub MPCC_BLND_ACTIVE_OVERLAP_ONLY: u8, pub MPCC_GLOBAL_ALPHA: u8,
    pub MPCC_GLOBAL_GAIN: u8, pub MPCC_IDLE: u8, pub MPCC_BUSY: u8,
    pub MPCC_OPP_ID: u8, pub MPCC_BG_G_Y: u8, pub MPCC_BG_R_CR: u8,
    pub MPCC_BG_B_CB: u8, pub MPCC_SM_EN: u8, pub MPCC_SM_MODE: u8,
    pub MPCC_SM_FRAME_ALT: u8, pub MPCC_SM_FIELD_ALT: u8,
    pub MPCC_SM_FORCE_NEXT_FRAME_POL: u8, pub MPCC_SM_FORCE_NEXT_TOP_POL: u8,
    pub MPC_OUT_MUX: u8, pub MPCC_UPDATE_LOCK_SEL: u8,
    pub CUR_VUPDATE_LOCK_SET: u8,
}

#[repr(C)]
pub struct dcn_mpc_mask {
    pub MPCC_TOP_SEL: u32, pub MPCC_BOT_SEL: u32, pub MPCC_MODE: u32,
    pub MPCC_ALPHA_BLND_MODE: u32, pub MPCC_ALPHA_MULTIPLIED_MODE: u32,
    pub MPCC_BLND_ACTIVE_OVERLAP_ONLY: u32, pub MPCC_GLOBAL_ALPHA: u32,
    pub MPCC_GLOBAL_GAIN: u32, pub MPCC_IDLE: u32, pub MPCC_BUSY: u32,
    pub MPCC_OPP_ID: u32, pub MPCC_BG_G_Y: u32, pub MPCC_BG_R_CR: u32,
    pub MPCC_BG_B_CB: u32, pub MPCC_SM_EN: u32, pub MPCC_SM_MODE: u32,
    pub MPCC_SM_FRAME_ALT: u32, pub MPCC_SM_FIELD_ALT: u32,
    pub MPCC_SM_FORCE_NEXT_FRAME_POL: u32, pub MPCC_SM_FORCE_NEXT_TOP_POL: u32,
    pub MPC_OUT_MUX: u32, pub MPCC_UPDATE_LOCK_SEL: u32,
    pub CUR_VUPDATE_LOCK_SET: u32,
}

#[repr(C)]
pub struct dcn10_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: ::core::ffi::c_int,
    pub num_mpcc: ::core::ffi::c_int,
    pub mpc_regs: *const dcn_mpc_registers,
    pub mpc_shift: *const dcn_mpc_shift,
    pub mpc_mask: *const dcn_mpc_mask,
}

extern "C" {
    pub fn dcn10_mpc_construct(mpcc10: *mut dcn10_mpc, ctx: *mut dc_context, mpc_regs: *const dcn_mpc_registers, mpc_shift: *const dcn_mpc_shift, mpc_mask: *const dcn_mpc_mask, num_mpcc: ::core::ffi::c_int);
    pub fn mpc1_insert_plane(mpc: *mut mpc, tree: *mut mpc_tree, blnd_cfg: *mut mpcc_blnd_cfg, sm_cfg: *mut mpcc_sm_cfg, insert_above_mpcc: *mut mpcc, dpp_id: ::core::ffi::c_int, mpcc_id: ::core::ffi::c_int) -> *mut mpcc;
    pub fn mpc1_remove_mpcc(mpc: *mut mpc, tree: *mut mpc_tree, mpcc: *mut mpcc);
    pub fn mpc1_mpc_init(mpc: *mut mpc);
    pub fn mpc1_mpc_init_single_inst(mpc: *mut mpc, mpcc_id: u32);
    pub fn mpc1_assert_idle_mpcc(mpc: *mut mpc, id: ::core::ffi::c_int);
    pub fn mpc1_set_bg_color(mpc: *mut mpc, bg_color: *mut tg_color, id: ::core::ffi::c_int);
    pub fn mpc1_update_stereo_mix(mpc: *mut mpc, sm_cfg: *mut mpcc_sm_cfg, mpcc_id: ::core::ffi::c_int);
    pub fn mpc1_assert_mpcc_idle_before_connect(mpc: *mut mpc, mpcc_id: ::core::ffi::c_int);
    pub fn mpc1_init_mpcc_list_from_hw(mpc: *mut mpc, tree: *mut mpc_tree);
    pub fn mpc1_get_mpcc(mpc: *mut mpc, mpcc_id: ::core::ffi::c_int) -> *mut mpcc;
    pub fn mpc1_get_mpcc_for_dpp(tree: *mut mpc_tree, dpp_id: ::core::ffi::c_int) -> *mut mpcc;
    pub fn mpc1_read_mpcc_state(mpc: *mut mpc, mpcc_inst: ::core::ffi::c_int, s: *mut mpcc_state);
    pub fn mpc1_cursor_lock(mpc: *mut mpc, opp_id: ::core::ffi::c_int, lock: bool);
    pub fn mpc1_get_mpc_out_mux(mpc: *mut mpc, opp_id: ::core::ffi::c_int) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
