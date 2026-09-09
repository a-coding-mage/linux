/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translation unit.

macro_rules! REG { ($mpc10:expr, $reg:expr) => { $mpc10.mpc_regs.$reg }; }
macro_rules! CTX { ($mpc10:expr) => { $mpc10.base.ctx }; }
macro_rules! FN { ($mpc10:expr, $reg_name:ident, $field_name:ident) => {
    ($mpc10.mpc_shift.$field_name, $mpc10.mpc_mask.$field_name)
} }

pub unsafe fn mpc1_set_bg_color(mpc: *mut mpc, bg_color: *mut tg_color, mpcc_id: i32) {
    let mpc10 = TO_DCN10_MPC(mpc);
    let mut bottommost_mpcc = mpc1_get_mpcc(mpc, mpcc_id);
    let (mut bg_r_cr, mut bg_g_y, mut bg_b_cb): (u32, u32, u32);
    (*bottommost_mpcc).blnd_cfg.black_color = *bg_color;
    while !(*bottommost_mpcc).mpcc_bot.is_null() {
        ASSERT(bottommost_mpcc != (*bottommost_mpcc).mpcc_bot);
        if bottommost_mpcc == (*bottommost_mpcc).mpcc_bot { break; }
        bottommost_mpcc = (*bottommost_mpcc).mpcc_bot;
    }
    bg_r_cr = (*bg_color).color_r_cr << 2;
    bg_g_y = (*bg_color).color_g_y << 2;
    bg_b_cb = (*bg_color).color_b_cb << 2;
    REG_SET!(mpc10, MPCC_BG_R_CR[(*bottommost_mpcc).mpcc_id], 0, MPCC_BG_R_CR, bg_r_cr);
    REG_SET!(mpc10, MPCC_BG_G_Y[(*bottommost_mpcc).mpcc_id], 0, MPCC_BG_G_Y, bg_g_y);
    REG_SET!(mpc10, MPCC_BG_B_CB[(*bottommost_mpcc).mpcc_id], 0, MPCC_BG_B_CB, bg_b_cb);
}

unsafe fn mpc1_update_blending(mpc: *mut mpc, blnd_cfg: *mut mpcc_blnd_cfg, mpcc_id: i32) {
    let mpc10 = TO_DCN10_MPC(mpc);
    let mpcc = mpc1_get_mpcc(mpc, mpcc_id);
    REG_UPDATE_5!(mpc10, MPCC_CONTROL[mpcc_id], MPCC_ALPHA_BLND_MODE, (*blnd_cfg).alpha_mode,
        MPCC_ALPHA_MULTIPLIED_MODE, (*blnd_cfg).pre_multiplied_alpha,
        MPCC_BLND_ACTIVE_OVERLAP_ONLY, (*blnd_cfg).overlap_only,
        MPCC_GLOBAL_ALPHA, (*blnd_cfg).global_alpha, MPCC_GLOBAL_GAIN, (*blnd_cfg).global_gain);
    (*mpcc).blnd_cfg = *blnd_cfg;
}

pub unsafe fn mpc1_update_stereo_mix(mpc: *mut mpc, sm_cfg: *mut mpcc_sm_cfg, mpcc_id: i32) {
    let mpc10 = TO_DCN10_MPC(mpc);
    REG_UPDATE_6!(mpc10, MPCC_SM_CONTROL[mpcc_id], MPCC_SM_EN, (*sm_cfg).enable,
        MPCC_SM_MODE, (*sm_cfg).sm_mode, MPCC_SM_FRAME_ALT, (*sm_cfg).frame_alt,
        MPCC_SM_FIELD_ALT, (*sm_cfg).field_alt,
        MPCC_SM_FORCE_NEXT_FRAME_POL, (*sm_cfg).force_next_frame_porlarity,
        MPCC_SM_FORCE_NEXT_TOP_POL, (*sm_cfg).force_next_field_polarity);
}

pub unsafe fn mpc1_assert_idle_mpcc(mpc: *mut mpc, id: i32) {
    let mpc10 = TO_DCN10_MPC(mpc);
    ASSERT(((*mpc10).mpcc_in_use_mask & (1 << id)) == 0);
    REG_WAIT!(mpc10, MPCC_STATUS[id], MPCC_IDLE, 1, 1, 100000);
}

pub unsafe fn mpc1_get_mpcc(mpc: *mut mpc, mpcc_id: i32) -> *mut mpcc {
    ASSERT(mpcc_id < (*TO_DCN10_MPC(mpc)).num_mpcc);
    (*mpc).mpcc_array.as_mut_ptr().add(mpcc_id as usize)
}

pub unsafe fn mpc1_get_mpcc_for_dpp(tree: *mut mpc_tree, dpp_id: i32) -> *mut mpcc {
    let mut tmp_mpcc = (*tree).opp_list;
    while !tmp_mpcc.is_null() {
        if (*tmp_mpcc).dpp_id == dpp_id { return tmp_mpcc; }
        ASSERT(tmp_mpcc != (*tmp_mpcc).mpcc_bot);
        if tmp_mpcc == (*tmp_mpcc).mpcc_bot { break; }
        tmp_mpcc = (*tmp_mpcc).mpcc_bot;
    }
    core::ptr::null_mut()
}

pub unsafe fn mpc1_assert_mpcc_idle_before_connect(mpc: *mut mpc, mpcc_id: i32) {
    let mpc10 = TO_DCN10_MPC(mpc);
    let (mut top_sel, mut mpc_busy, mut mpc_idle) = (0u32, 0u32, 0u32);
    REG_GET!(mpc10, MPCC_TOP_SEL[mpcc_id], MPCC_TOP_SEL, &mut top_sel);
    if top_sel == 0xf {
        REG_GET_2!(mpc10, MPCC_STATUS[mpcc_id], MPCC_BUSY, &mut mpc_busy, MPCC_IDLE, &mut mpc_idle);
        ASSERT(mpc_busy == 0); ASSERT(mpc_idle == 1);
    }
}

pub unsafe fn mpc1_insert_plane(mpc: *mut mpc, tree: *mut mpc_tree, blnd_cfg: *mut mpcc_blnd_cfg,
    sm_cfg: *mut mpcc_sm_cfg, insert_above_mpcc: *mut mpcc, dpp_id: i32, mpcc_id: i32) -> *mut mpcc {
    let mpc10 = TO_DCN10_MPC(mpc);
    let mut new_mpcc: *mut mpcc = core::ptr::null_mut();
    ASSERT(mpcc_id < (*mpc10).num_mpcc);
    ASSERT(((*mpc10).mpcc_in_use_mask & (1 << mpcc_id)) == 0);
    if !insert_above_mpcc.is_null() {
        let mut temp_mpcc = (*tree).opp_list;
        if temp_mpcc != insert_above_mpcc {
            while !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot != insert_above_mpcc { temp_mpcc = (*temp_mpcc).mpcc_bot; }
        }
        if temp_mpcc.is_null() { return core::ptr::null_mut(); }
    }
    new_mpcc = mpc1_get_mpcc(mpc, mpcc_id); (*new_mpcc).dpp_id = dpp_id;
    if !insert_above_mpcc.is_null() {
        (*new_mpcc).mpcc_bot = insert_above_mpcc;
        REG_SET!(mpc10, MPCC_BOT_SEL[mpcc_id], 0, MPCC_BOT_SEL, (*insert_above_mpcc).mpcc_id);
        REG_UPDATE!(mpc10, MPCC_CONTROL[mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_BOT_BLENDING);
    } else {
        (*new_mpcc).mpcc_bot = core::ptr::null_mut();
        REG_SET!(mpc10, MPCC_BOT_SEL[mpcc_id], 0, MPCC_BOT_SEL, 0xf);
        REG_UPDATE!(mpc10, MPCC_CONTROL[mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_LAYER_ONLY);
    }
    REG_SET!(mpc10, MPCC_TOP_SEL[mpcc_id], 0, MPCC_TOP_SEL, dpp_id);
    REG_SET!(mpc10, MPCC_OPP_ID[mpcc_id], 0, MPCC_OPP_ID, (*tree).opp_id);
    REG_SET!(mpc10, MPCC_UPDATE_LOCK_SEL[mpcc_id], 0, MPCC_UPDATE_LOCK_SEL, (*tree).opp_id);
    if (*tree).opp_list == insert_above_mpcc {
        (*tree).opp_list = new_mpcc; REG_UPDATE!(mpc10, MUX[(*tree).opp_id], MPC_OUT_MUX, mpcc_id);
    } else {
        let mut temp_mpcc = (*tree).opp_list;
        while !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot != insert_above_mpcc { temp_mpcc = (*temp_mpcc).mpcc_bot; }
        if !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot == insert_above_mpcc {
            REG_SET!(mpc10, MPCC_BOT_SEL[(*temp_mpcc).mpcc_id], 0, MPCC_BOT_SEL, mpcc_id);
            (*temp_mpcc).mpcc_bot = new_mpcc;
            if insert_above_mpcc.is_null() { REG_UPDATE!(mpc10, MPCC_CONTROL[(*temp_mpcc).mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_BOT_BLENDING); }
        }
    }
    ((*mpc).funcs).update_blending(mpc, blnd_cfg, mpcc_id);
    if !sm_cfg.is_null() { (*new_mpcc).sm_cfg = *sm_cfg; mpc1_update_stereo_mix(mpc, sm_cfg, mpcc_id); }
    (*mpc10).mpcc_in_use_mask |= 1 << mpcc_id; new_mpcc
}

pub unsafe fn mpc1_remove_mpcc(mpc: *mut mpc, tree: *mut mpc_tree, mpcc_to_remove: *mut mpcc) {
    let mpc10 = TO_DCN10_MPC(mpc); let mut found = false; let mpcc_id = (*mpcc_to_remove).mpcc_id;
    if (*tree).opp_list == mpcc_to_remove { found = true; if !(*mpcc_to_remove).mpcc_bot.is_null() { (*tree).opp_list = (*mpcc_to_remove).mpcc_bot; REG_UPDATE!(mpc10, MUX[(*tree).opp_id], MPC_OUT_MUX, (*tree).opp_list).mpcc_id; } else { (*tree).opp_list = core::ptr::null_mut(); REG_UPDATE!(mpc10, MUX[(*tree).opp_id], MPC_OUT_MUX, 0xf); } }
    else { let mut temp_mpcc = (*tree).opp_list; while !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot != mpcc_to_remove { temp_mpcc = (*temp_mpcc).mpcc_bot; } if !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot == mpcc_to_remove { found = true; (*temp_mpcc).mpcc_bot = (*mpcc_to_remove).mpcc_bot; if !(*mpcc_to_remove).mpcc_bot.is_null() { REG_SET!(mpc10, MPCC_BOT_SEL[(*temp_mpcc).mpcc_id], 0, MPCC_BOT_SEL, (*mpcc_to_remove).mpcc_bot.mpcc_id); } else { REG_SET!(mpc10, MPCC_BOT_SEL[(*temp_mpcc).mpcc_id], 0, MPCC_BOT_SEL, 0xf); REG_UPDATE!(mpc10, MPCC_CONTROL[(*temp_mpcc).mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_LAYER_PASSTHROUGH); } } }
    REG_SET!(mpc10, MPCC_TOP_SEL[mpcc_id], 0, MPCC_TOP_SEL, 0xf); REG_SET!(mpc10, MPCC_BOT_SEL[mpcc_id], 0, MPCC_BOT_SEL, 0xf); REG_SET!(mpc10, MPCC_OPP_ID[mpcc_id], 0, MPCC_OPP_ID, 0xf); REG_SET!(mpc10, MPCC_UPDATE_LOCK_SEL[mpcc_id], 0, MPCC_UPDATE_LOCK_SEL, 0xf);
    if found { (*mpc10).mpcc_in_use_mask &= !(1 << mpcc_id); (*mpcc_to_remove).dpp_id = 0xf; (*mpcc_to_remove).mpcc_bot = core::ptr::null_mut(); }
}

unsafe fn mpc1_init_mpcc(mpcc: *mut mpcc, mpcc_inst: i32) { (*mpcc).mpcc_id=mpcc_inst; (*mpcc).dpp_id=0xf; (*mpcc).mpcc_bot=core::ptr::null_mut(); (*mpcc).blnd_cfg.overlap_only=false; (*mpcc).blnd_cfg.global_alpha=0xff; (*mpcc).blnd_cfg.global_gain=0xff; (*mpcc).sm_cfg.enable=false; }

pub unsafe fn mpc1_mpc_init(mpc: *mut mpc) { let mpc10=TO_DCN10_MPC(mpc); (*mpc10).mpcc_in_use_mask=0; let mut mpcc_id=0; while mpcc_id<(*mpc10).num_mpcc { REG_SET!(mpc10, MPCC_TOP_SEL[mpcc_id],0,MPCC_TOP_SEL,0xf); REG_SET!(mpc10,MPCC_BOT_SEL[mpcc_id],0,MPCC_BOT_SEL,0xf); REG_SET!(mpc10,MPCC_OPP_ID[mpcc_id],0,MPCC_OPP_ID,0xf); REG_SET!(mpc10,MPCC_UPDATE_LOCK_SEL[mpcc_id],0,MPCC_UPDATE_LOCK_SEL,0xf); mpc1_init_mpcc((*mpc).mpcc_array.as_mut_ptr().add(mpcc_id as usize),mpcc_id); mpcc_id+=1; } let mut opp_id=0; while opp_id<MAX_OPP { if REG!(mpc10,MUX[opp_id]) { REG_UPDATE!(mpc10,MUX[opp_id],MPC_OUT_MUX,0xf); } opp_id+=1; } }

pub unsafe fn mpc1_mpc_init_single_inst(mpc:*mut mpc,mpcc_id:u32){let mpc10=TO_DCN10_MPC(mpc);let mut opp_id=0;REG_GET!(mpc10,MPCC_OPP_ID[mpcc_id],MPCC_OPP_ID,&mut opp_id);REG_SET!(mpc10,MPCC_TOP_SEL[mpcc_id],0,MPCC_TOP_SEL,0xf);REG_SET!(mpc10,MPCC_BOT_SEL[mpcc_id],0,MPCC_BOT_SEL,0xf);REG_SET!(mpc10,MPCC_OPP_ID[mpcc_id],0,MPCC_OPP_ID,0xf);REG_SET!(mpc10,MPCC_UPDATE_LOCK_SEL[mpcc_id],0,MPCC_UPDATE_LOCK_SEL,0xf);mpc1_init_mpcc((*mpc).mpcc_array.as_mut_ptr().add(mpcc_id as usize),mpcc_id as i32);if opp_id<MAX_OPP&&REG!(mpc10,MUX[opp_id]){REG_UPDATE!(mpc10,MUX[opp_id],MPC_OUT_MUX,0xf);}}

pub unsafe fn mpc1_init_mpcc_list_from_hw(mpc:*mut mpc,tree:*mut mpc_tree){let mpc10=TO_DCN10_MPC(mpc);let(mut opp_id,mut top_sel,mut bot_sel,mut out_mux)=(0u32,0u32,0u32,0u32);REG_GET!(mpc10,MUX[(*tree).opp_id],MPC_OUT_MUX,&mut out_mux);if out_mux!=0xf{let mut mpcc_id=0;while mpcc_id<(*mpc10).num_mpcc{REG_GET!(mpc10,MPCC_OPP_ID[mpcc_id],MPCC_OPP_ID,&mut opp_id);REG_GET!(mpc10,MPCC_TOP_SEL[mpcc_id],MPCC_TOP_SEL,&mut top_sel);REG_GET!(mpc10,MPCC_BOT_SEL[mpcc_id],MPCC_BOT_SEL,&mut bot_sel);if bot_sel==mpcc_id as u32{bot_sel=0xf;}if opp_id==(*tree).opp_id&&top_sel!=0xf{let mpcc=mpc1_get_mpcc(mpc,mpcc_id);(*mpcc).dpp_id=top_sel as i32;(*mpc10).mpcc_in_use_mask|=1<<mpcc_id;if out_mux==mpcc_id as u32{(*tree).opp_list=mpcc;}if bot_sel!=0xf&&bot_sel<(*mpc10).num_mpcc as u32{let bot_mpcc_id=bot_sel as i32;REG_GET!(mpc10,MPCC_OPP_ID[bot_mpcc_id],MPCC_OPP_ID,&mut opp_id);REG_GET!(mpc10,MPCC_TOP_SEL[bot_mpcc_id],MPCC_TOP_SEL,&mut top_sel);if opp_id==(*tree).opp_id&&top_sel!=0xf{(*mpcc).mpcc_bot=mpc1_get_mpcc(mpc,bot_mpcc_id);}}}mpcc_id+=1;}}}

pub unsafe fn mpc1_read_mpcc_state(mpc:*mut mpc,mpcc_inst:i32,s:*mut mpcc_state){let mpc10=TO_DCN10_MPC(mpc);REG_GET!(mpc10,MPCC_OPP_ID[mpcc_inst],MPCC_OPP_ID,&mut (*s).opp_id);REG_GET!(mpc10,MPCC_TOP_SEL[mpcc_inst],MPCC_TOP_SEL,&mut (*s).dpp_id);REG_GET!(mpc10,MPCC_BOT_SEL[mpcc_inst],MPCC_BOT_SEL,&mut (*s).bot_mpcc_id);REG_GET_4!(mpc10,MPCC_CONTROL[mpcc_inst],MPCC_MODE,&mut (*s).mode,MPCC_ALPHA_BLND_MODE,&mut (*s).alpha_mode,MPCC_ALPHA_MULTIPLIED_MODE,&mut (*s).pre_multiplied_alpha,MPCC_BLND_ACTIVE_OVERLAP_ONLY,&mut (*s).overlap_only);REG_GET_2!(mpc10,MPCC_STATUS[mpcc_inst],MPCC_IDLE,&mut (*s).idle,MPCC_BUSY,&mut (*s).busy);}
pub unsafe fn mpc1_cursor_lock(mpc:*mut mpc,opp_id:i32,lock:bool){let mpc10=TO_DCN10_MPC(mpc);REG_SET!(mpc10,CUR[opp_id],0,CUR_VUPDATE_LOCK_SET,if lock{1}else{0});}
pub unsafe fn mpc1_get_mpc_out_mux(mpc:*mut mpc,opp_id:i32)->u32{let mpc10=TO_DCN10_MPC(mpc);let mut val=0xf;if opp_id<MAX_OPP&&REG!(mpc10,MUX[opp_id]){REG_GET!(mpc10,MUX[opp_id],MPC_OUT_MUX,&mut val);}val}

static dcn10_mpc_funcs: mpc_funcs = mpc_funcs {
    read_mpcc_state: Some(mpc1_read_mpcc_state),
    insert_plane: Some(mpc1_insert_plane),
    remove_mpcc: Some(mpc1_remove_mpcc),
    mpc_init: Some(mpc1_mpc_init),
    mpc_init_single_inst: Some(mpc1_mpc_init_single_inst),
    get_mpcc_for_dpp: Some(mpc1_get_mpcc_for_dpp),
    wait_for_idle: Some(mpc1_assert_idle_mpcc),
    assert_mpcc_idle_before_connect: Some(mpc1_assert_mpcc_idle_before_connect),
    init_mpcc_list_from_hw: Some(mpc1_init_mpcc_list_from_hw),
    update_blending: Some(mpc1_update_blending),
    cursor_lock: Some(mpc1_cursor_lock),
    set_denorm: None,
    set_denorm_clamp: None,
    set_output_csc: None,
    set_output_gamma: None,
    get_mpc_out_mux: Some(mpc1_get_mpc_out_mux),
    set_bg_color: Some(mpc1_set_bg_color),
};

pub unsafe fn dcn10_mpc_construct(mpc10:*mut dcn10_mpc,ctx:*mut dc_context,mpc_regs:*const dcn_mpc_registers,mpc_shift:*const dcn_mpc_shift,mpc_mask:*const dcn_mpc_mask,num_mpcc:i32){(*mpc10).base.ctx=ctx;(*mpc10).base.funcs=&dcn10_mpc_funcs;(*mpc10).mpc_regs=mpc_regs;(*mpc10).mpc_shift=mpc_shift;(*mpc10).mpc_mask=mpc_mask;(*mpc10).mpcc_in_use_mask=0;(*mpc10).num_mpcc=num_mpcc;let mut i=0;while i<MAX_MPCC{mpc1_init_mpcc((*mpc10).base.mpcc_array.as_mut_ptr().add(i as usize),i);i+=1;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
