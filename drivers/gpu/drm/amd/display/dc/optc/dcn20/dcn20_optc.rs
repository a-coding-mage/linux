/* SPDX-License-Identifier: MIT */

// Register helpers, types, and external functions are supplied by the translated dependencies.

pub unsafe fn optc2_enable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE!(optc1, CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2!(optc1, OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 3, OTG_MASTER_EN, 1);
    true
}

pub unsafe fn optc2_set_gsl(optc: *mut timing_generator, params: *const gsl_params) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5!(optc1, OTG_GSL_CONTROL,
        OTG_GSL0_EN, (*params).gsl0_en, OTG_GSL1_EN, (*params).gsl1_en,
        OTG_GSL2_EN, (*params).gsl2_en, OTG_GSL_MASTER_EN, (*params).gsl_master_en,
        OTG_GSL_MASTER_MODE, (*params).gsl_master_mode);
}

pub unsafe fn optc2_set_gsl_source_select(optc: *mut timing_generator, group_idx: i32, gsl_ready_signal: u32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    match group_idx {
        1 => REG_UPDATE!(optc1, GSL_SOURCE_SELECT, GSL0_READY_SOURCE_SEL, gsl_ready_signal),
        2 => REG_UPDATE!(optc1, GSL_SOURCE_SELECT, GSL1_READY_SOURCE_SEL, gsl_ready_signal),
        3 => REG_UPDATE!(optc1, GSL_SOURCE_SELECT, GSL2_READY_SOURCE_SEL, gsl_ready_signal),
        _ => (),
    }
}

pub unsafe fn optc2_set_dsc_config(optc: *mut timing_generator, dsc_mode: optc_dsc_mode, dsc_bytes_per_pixel: u32, dsc_slice_width: u32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OPTC_DATA_FORMAT_CONTROL, OPTC_DSC_MODE, dsc_mode);
    REG_SET!(optc1, OPTC_BYTES_PER_PIXEL, 0, OPTC_DSC_BYTES_PER_PIXEL, dsc_bytes_per_pixel);
    REG_UPDATE!(optc1, OPTC_WIDTH_CONTROL, OPTC_DSC_SLICE_WIDTH, dsc_slice_width);
}

pub unsafe fn optc2_get_dsc_status(optc: *mut timing_generator, dsc_mode: *mut u32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_GET!(optc1, OPTC_DATA_FORMAT_CONTROL, OPTC_DSC_MODE, dsc_mode);
}

pub unsafe fn optc2_set_odm_bypass(optc: *mut timing_generator, timing: *const dc_crtc_timing) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_SET_3!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 0, OPTC_SEG0_SRC_SEL, (*optc).inst, OPTC_SEG1_SRC_SEL, 0xf);
    REG_WRITE!(optc1, OTG_H_TIMING_CNTL, 0);
    let h_div_2 = ((*optc).funcs).is_two_pixels_per_container(timing);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_BY2, h_div_2);
    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0);
    (*optc1).opp_count = 1;
}

pub unsafe fn optc2_set_odm_combine(optc: *mut timing_generator, opp_id: *const i32, opp_cnt: i32, segment_width: i32, _last_segment_width: i32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    ASSERT!(opp_cnt == 2);
    let memory_mask = (0x3u32 << ((*opp_id.add(0)) * 2)) | (0x3u32 << ((*opp_id.add(1)) * 2));
    if REG!(optc1, OPTC_MEMORY_CONFIG) != 0 { REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask); }
    REG_SET_3!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1, OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1));
    REG_UPDATE!(optc1, OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
    REG_SET!(optc1, OTG_H_TIMING_CNTL, 0, OTG_H_TIMING_DIV_BY2, 1);
    (*optc1).opp_count = opp_cnt;
}

pub unsafe fn optc2_get_optc_source(optc: *mut timing_generator, num: *mut u32, id0: *mut u32, id1: *mut u32) {
    let optc1 = DCN10TG_FROM_TG(optc); let mut segments = 0u32;
    REG_GET_3!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_NUM_OF_INPUT_SEGMENT, &mut segments, OPTC_SEG0_SRC_SEL, id0, OPTC_SEG1_SRC_SEL, id1);
    *num = if segments == 1 { 2 } else { 1 }; if *id1 == 0xf { *num = 1; }
}

unsafe fn optc2_set_dwb_source(optc: *mut timing_generator, pipe: u32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    if pipe == 0 { REG_UPDATE!(optc1, DWB_SOURCE_SELECT, OPTC_DWB0_SOURCE_SELECT, (*optc).inst); }
    else if pipe == 1 { REG_UPDATE!(optc1, DWB_SOURCE_SELECT, OPTC_DWB1_SOURCE_SELECT, (*optc).inst); }
}

unsafe fn optc2_align_vblanks(master: *mut timing_generator, slave: *mut timing_generator, master_clock: u32, slave_clock: u32, master_div: u8, _slave_div: u8) {
    let mut optc1 = DCN10TG_FROM_TG(slave); let mut master_v_active=0u32; let mut master_h_total=0u32; let mut slave_h_total=0u32; let mut lock=0u32;
    REG_UPDATE!(optc1, OTG_CONTROL, OTG_MASTER_EN, 0); REG_WAIT!(optc1, OTG_CONTROL, OTG_CURRENT_MASTER_EN_STATE, 0, 10, 5000);
    REG_GET!(optc1, OTG_H_TOTAL, OTG_H_TOTAL, &mut slave_h_total); REG_SET!(optc1, OTG_GLOBAL_CONTROL0, 0, OTG_MASTER_UPDATE_LOCK_SEL, (*master).inst);
    optc1=DCN10TG_FROM_TG(master); REG_GET!(optc1, OTG_MASTER_UPDATE_LOCK, OTG_MASTER_UPDATE_LOCK, &mut lock); REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, 0);
    REG_GET!(optc1, OTG_V_BLANK_START_END, OTG_V_BLANK_START, &mut master_v_active); REG_GET!(optc1, OTG_H_TOTAL, OTG_H_TOTAL, &mut master_h_total);
    let p=10000u64; let mut l=div_u64(p*slave_h_total as u64*master_clock as u64, master_h_total as u64); l=div_u64(l, slave_clock as u64); let xy=div_u64(l,p); let y=(master_v_active as u64-xy-1) as u32; let x=div_u64(((xy+1)*p-l)*master_h_total as u64,p*master_div as u64) as u32;
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_EN, 1); REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_X, x, MASTER_UPDATE_LOCK_DB_Y, y); REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, 1); REG_WAIT!(optc1, OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, 1, 1, 10);
    optc1=DCN10TG_FROM_TG(slave); REG_UPDATE!(optc1, OTG_CONTROL, OTG_MASTER_EN, 1); optc1=DCN10TG_FROM_TG(master); REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, 0); optc1=DCN10TG_FROM_TG(slave); REG_WAIT!(optc1, OTG_CONTROL, OTG_CURRENT_MASTER_EN_STATE, 1, 10, 5000);
    optc1=DCN10TG_FROM_TG(master); REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_EN, 0); REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_X, 0, MASTER_UPDATE_LOCK_DB_Y, 0); REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, lock); optc1=DCN10TG_FROM_TG(slave); REG_SET!(optc1, OTG_GLOBAL_CONTROL0, 0, OTG_MASTER_UPDATE_LOCK_SEL, (*slave).inst);
}

#[inline] unsafe fn div_u64(a:u64,b:u64)->u64 { a / b }

pub unsafe fn optc2_triplebuffer_lock(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);REG_SET!(o,OTG_GLOBAL_CONTROL0,0,OTG_MASTER_UPDATE_LOCK_SEL,(*optc).inst);REG_SET!(o,OTG_VUPDATE_KEEPOUT,0,OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN,1);REG_SET!(o,OTG_MASTER_UPDATE_LOCK,0,OTG_MASTER_UPDATE_LOCK,1);REG_WAIT!(o,OTG_MASTER_UPDATE_LOCK,UPDATE_LOCK_STATUS,1,1,10);}
pub unsafe fn optc2_triplebuffer_unlock(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);REG_SET!(o,OTG_MASTER_UPDATE_LOCK,0,OTG_MASTER_UPDATE_LOCK,0);REG_SET!(o,OTG_VUPDATE_KEEPOUT,0,OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN,0);}

pub unsafe fn optc2_lock_doublebuffer_enable(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);let(mut v,mut h)=(0,0);REG_UPDATE!(o,OTG_GLOBAL_CONTROL1,MASTER_UPDATE_LOCK_DB_EN,1);REG_UPDATE_2!(o,OTG_GLOBAL_CONTROL2,GLOBAL_UPDATE_LOCK_EN,1,DIG_UPDATE_LOCATION,20);REG_GET!(o,OTG_V_BLANK_START_END,OTG_V_BLANK_START,&mut v);REG_GET!(o,OTG_H_BLANK_START_END,OTG_H_BLANK_START,&mut h);REG_UPDATE_2!(o,OTG_GLOBAL_CONTROL1,MASTER_UPDATE_LOCK_DB_X,(h-200-1)/(*o).opp_count,MASTER_UPDATE_LOCK_DB_Y,v-1);REG_SET_3!(o,OTG_VUPDATE_KEEPOUT,0,MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_START_OFFSET,0,MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_END_OFFSET,100,OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN,1);}
pub unsafe fn optc2_lock_doublebuffer_disable(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);REG_UPDATE_2!(o,OTG_GLOBAL_CONTROL1,MASTER_UPDATE_LOCK_DB_X,0,MASTER_UPDATE_LOCK_DB_Y,0);REG_UPDATE_2!(o,OTG_GLOBAL_CONTROL2,GLOBAL_UPDATE_LOCK_EN,0,DIG_UPDATE_LOCATION,0);REG_UPDATE!(o,OTG_GLOBAL_CONTROL1,MASTER_UPDATE_LOCK_DB_EN,0);}

pub unsafe fn optc2_setup_manual_trigger(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);REG_UPDATE_4!(o,OTG_V_TOTAL_CONTROL,OTG_V_TOTAL_MIN_SEL,1,OTG_V_TOTAL_MAX_SEL,1,OTG_FORCE_LOCK_ON_EVENT,0,OTG_SET_V_TOTAL_MIN_MASK,1<<1);REG_SET_8!(o,OTG_TRIGA_CNTL,0,OTG_TRIGA_SOURCE_SELECT,21,OTG_TRIGA_SOURCE_PIPE_SELECT,(*optc).inst,OTG_TRIGA_RISING_EDGE_DETECT_CNTL,1,OTG_TRIGA_FALLING_EDGE_DETECT_CNTL,0,OTG_TRIGA_POLARITY_SELECT,0,OTG_TRIGA_FREQUENCY_SELECT,0,OTG_TRIGA_DELAY,0,OTG_TRIGA_CLEAR,1);}
pub unsafe fn optc2_program_manual_trigger(optc:*mut timing_generator){let o=DCN10TG_FROM_TG(optc);REG_SET!(o,OTG_TRIGA_MANUAL_TRIG,0,OTG_TRIGA_MANUAL_TRIG,1);}

pub unsafe fn optc2_configure_crc(optc:*mut timing_generator, params:*const crc_params)->bool{let o=DCN10TG_FROM_TG(optc);REG_SET_2!(o,OTG_CRC_CNTL2,0,OTG_CRC_DSC_MODE,(*params).dsc_mode,OTG_CRC_DATA_STREAM_COMBINE_MODE,(*params).odm_mode);optc1_configure_crc(optc,params)}
pub unsafe fn optc2_get_last_used_drr_vtotal(optc:*mut timing_generator, refresh:*mut u32){let o=DCN10TG_FROM_TG(optc);REG_GET!(o,OTG_DRR_CONTROL,OTG_V_TOTAL_LAST_USED_BY_DRR,refresh);}

// The remaining operations are inherited from the DCN1 implementation; this table preserves
// the externally visible dispatch layout of the original implementation.
extern "C" {
    static dcn20_tg_funcs: timing_generator_funcs;
}

pub unsafe fn dcn20_timing_generator_init(optc1:*mut optc){(*optc1).base.funcs=&dcn20_tg_funcs;(*optc1).max_h_total=(*optc1).tg_mask.OTG_H_TOTAL+1;(*optc1).max_v_total=(*optc1).tg_mask.OTG_V_TOTAL+1;(*optc1).min_h_blank=32;(*optc1).min_v_blank=3;(*optc1).min_v_blank_interlace=5;(*optc1).min_h_sync_width=4;(*optc1).min_v_sync_width=1;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
