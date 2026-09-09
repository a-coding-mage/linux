/* Translated from display_rq_dlg_helpers.c. */

use core::ffi::{c_char, c_int};

#[repr(C)] pub struct display_mode_lib { _private: [u8; 0] }
#[repr(C)] pub struct _vcs_dpi_display_rq_params_st { pub sizing: RqSizing, pub dlg: RqDlg, pub misc: RqMisc }
#[repr(C)] pub struct RqSizing { pub rq_l: _vcs_dpi_display_data_rq_sizing_params_st, pub rq_c: _vcs_dpi_display_data_rq_sizing_params_st }
#[repr(C)] pub struct RqDlg { pub rq_l: _vcs_dpi_display_data_rq_dlg_params_st, pub rq_c: _vcs_dpi_display_data_rq_dlg_params_st }
#[repr(C)] pub struct RqMisc { pub rq_l: _vcs_dpi_display_data_rq_misc_params_st, pub rq_c: _vcs_dpi_display_data_rq_misc_params_st }
#[repr(C)] pub struct _vcs_dpi_display_data_rq_sizing_params_st { pub chunk_bytes:i32,pub min_chunk_bytes:i32,pub meta_chunk_bytes:i32,pub min_meta_chunk_bytes:i32,pub mpte_group_bytes:i32,pub dpte_group_bytes:i32 }
#[repr(C)] pub struct _vcs_dpi_display_data_rq_dlg_params_st { pub swath_width_ub:i32,pub swath_height:i32,pub req_per_swath_ub:i32,pub meta_pte_bytes_per_frame_ub:i32,pub dpte_req_per_row_ub:i32,pub dpte_groups_per_row_ub:i32,pub dpte_row_height:i32,pub dpte_bytes_per_row_ub:i32,pub meta_chunks_per_row_ub:i32,pub meta_req_per_row_ub:i32,pub meta_row_height:i32,pub meta_bytes_per_row_ub:i32 }
#[repr(C)] pub struct _vcs_dpi_display_data_rq_misc_params_st { pub full_swath_bytes:i32,pub stored_swath_bytes:i32,pub blk256_width:i32,pub blk256_height:i32,pub req_width:i32,pub req_height:i32 }

extern "C" { fn dml_print(fmt: *const c_char, ...); }
macro_rules! p { ($s:literal $(, $v:expr)*) => { unsafe { dml_print(concat!($s, "\0").as_ptr() as *const c_char $(, $v)*) } }; }

pub unsafe fn print__rq_params_st(m:*mut display_mode_lib, r:*const _vcs_dpi_display_rq_params_st) {
 p!("DML_RQ_DLG_CALC: ***************************\n"); p!("DML_RQ_DLG_CALC: DISPLAY_RQ_PARAM_ST\n"); p!("DML_RQ_DLG_CALC:  <LUMA>\n"); print__data_rq_sizing_params_st(m,&(*r).sizing.rq_l); p!("DML_RQ_DLG_CALC:  <CHROMA> ===\n"); print__data_rq_sizing_params_st(m,&(*r).sizing.rq_c);
 p!("DML_RQ_DLG_CALC: <LUMA>\n"); print__data_rq_dlg_params_st(m,&(*r).dlg.rq_l); p!("DML_RQ_DLG_CALC: <CHROMA>\n"); print__data_rq_dlg_params_st(m,&(*r).dlg.rq_c);
 p!("DML_RQ_DLG_CALC: <LUMA>\n"); print__data_rq_misc_params_st(m,&(*r).misc.rq_l); p!("DML_RQ_DLG_CALC: <CHROMA>\n"); print__data_rq_misc_params_st(m,&(*r).misc.rq_c); p!("DML_RQ_DLG_CALC: ***************************\n");
}
pub unsafe fn print__data_rq_sizing_params_st(_: *mut display_mode_lib,r:*const _vcs_dpi_display_data_rq_sizing_params_st){p!("DML_RQ_DLG_CALC: =====================================\n");p!("DML_RQ_DLG_CALC: DISPLAY_DATA_RQ_SIZING_PARAM_ST\n");p!("DML_RQ_DLG_CALC:    chunk_bytes           = %0d\n",(*r).chunk_bytes);p!("DML_RQ_DLG_CALC:    min_chunk_bytes       = %0d\n",(*r).min_chunk_bytes);p!("DML_RQ_DLG_CALC:    meta_chunk_bytes      = %0d\n",(*r).meta_chunk_bytes);p!("DML_RQ_DLG_CALC:    min_meta_chunk_bytes  = %0d\n",(*r).min_meta_chunk_bytes);p!("DML_RQ_DLG_CALC:    mpte_group_bytes      = %0d\n",(*r).mpte_group_bytes);p!("DML_RQ_DLG_CALC:    dpte_group_bytes      = %0d\n",(*r).dpte_group_bytes);p!("DML_RQ_DLG_CALC: =====================================\n");}
pub unsafe fn print__data_rq_dlg_params_st(_: *mut display_mode_lib,r:*const _vcs_dpi_display_data_rq_dlg_params_st){p!("DML_RQ_DLG_CALC: =====================================\n");p!("DML_RQ_DLG_CALC: DISPLAY_DATA_RQ_DLG_PARAM_ST\n");p!("DML_RQ_DLG_CALC:    swath_width_ub              = %0d\n",(*r).swath_width_ub);p!("DML_RQ_DLG_CALC:    swath_height                = %0d\n",(*r).swath_height);p!("DML_RQ_DLG_CALC:    req_per_swath_ub            = %0d\n",(*r).req_per_swath_ub);p!("DML_RQ_DLG_CALC:    meta_pte_bytes_per_frame_ub = %0d\n",(*r).meta_pte_bytes_per_frame_ub);p!("DML_RQ_DLG_CALC:    dpte_req_per_row_ub         = %0d\n",(*r).dpte_req_per_row_ub);p!("DML_RQ_DLG_CALC:    dpte_groups_per_row_ub      = %0d\n",(*r).dpte_groups_per_row_ub);p!("DML_RQ_DLG_CALC:    dpte_row_height             = %0d\n",(*r).dpte_row_height);p!("DML_RQ_DLG_CALC:    dpte_bytes_per_row_ub       = %0d\n",(*r).dpte_bytes_per_row_ub);p!("DML_RQ_DLG_CALC:    meta_chunks_per_row_ub      = %0d\n",(*r).meta_chunks_per_row_ub);p!("DML_RQ_DLG_CALC:    meta_req_per_row_ub         = %0d\n",(*r).meta_req_per_row_ub);p!("DML_RQ_DLG_CALC:    meta_row_height             = %0d\n",(*r).meta_row_height);p!("DML_RQ_DLG_CALC:    meta_bytes_per_row_ub       = %0d\n",(*r).meta_bytes_per_row_ub);p!("DML_RQ_DLG_CALC: =====================================\n");}
pub unsafe fn print__data_rq_misc_params_st(_: *mut display_mode_lib,r:*const _vcs_dpi_display_data_rq_misc_params_st){p!("DML_RQ_DLG_CALC: =====================================\n");p!("DML_RQ_DLG_CALC: DISPLAY_DATA_RQ_MISC_PARAM_ST\n");p!("DML_RQ_DLG_CALC:     full_swath_bytes   = %0d\n",(*r).full_swath_bytes);p!("DML_RQ_DLG_CALC:     stored_swath_bytes = %0d\n",(*r).stored_swath_bytes);p!("DML_RQ_DLG_CALC:     blk256_width       = %0d\n",(*r).blk256_width);p!("DML_RQ_DLG_CALC:     blk256_height      = %0d\n",(*r).blk256_height);p!("DML_RQ_DLG_CALC:     req_width          = %0d\n",(*r).req_width);p!("DML_RQ_DLG_CALC:     req_height         = %0d\n",(*r).req_height);p!("DML_RQ_DLG_CALC: =====================================\n");}

#[repr(C)] pub struct _vcs_dpi_display_dlg_sys_params_st { pub t_mclk_wm_us:f64,pub t_urg_wm_us:f64,pub t_sr_wm_us:f64,pub t_extra_us:f64,pub deepsleep_dcfclk_mhz:f64,pub total_flip_bw:f64,pub total_flip_bytes:i32 }
pub unsafe fn print__dlg_sys_params_st(_: *mut display_mode_lib,r:*const _vcs_dpi_display_dlg_sys_params_st){p!("DML_RQ_DLG_CALC: =====================================\n");p!("DML_RQ_DLG_CALC: DISPLAY_RQ_DLG_PARAM_ST\n");p!("DML_RQ_DLG_CALC:    t_mclk_wm_us         = %3.2f\n",(*r).t_mclk_wm_us);p!("DML_RQ_DLG_CALC:    t_urg_wm_us          = %3.2f\n",(*r).t_urg_wm_us);p!("DML_RQ_DLG_CALC:    t_sr_wm_us           = %3.2f\n",(*r).t_sr_wm_us);p!("DML_RQ_DLG_CALC:    t_extra_us           = %3.2f\n",(*r).t_extra_us);p!("DML_RQ_DLG_CALC:    deepsleep_dcfclk_mhz = %3.2f\n",(*r).deepsleep_dcfclk_mhz);p!("DML_RQ_DLG_CALC:    total_flip_bw        = %3.2f\n",(*r).total_flip_bw);p!("DML_RQ_DLG_CALC:    total_flip_bytes     = %i\n",(*r).total_flip_bytes);p!("DML_RQ_DLG_CALC: =====================================\n");}

#[repr(C)] pub struct _vcs_dpi_display_data_rq_regs_st { pub chunk_size:i32,pub min_chunk_size:i32,pub meta_chunk_size:i32,pub min_meta_chunk_size:i32,pub dpte_group_size:i32,pub mpte_group_size:i32,pub swath_height:i32,pub pte_row_height_linear:i32 }
pub unsafe fn print__data_rq_regs_st(_: *mut display_mode_lib,r:*const _vcs_dpi_display_data_rq_regs_st){p!("DML_RQ_DLG_CALC: =====================================\n");p!("DML_RQ_DLG_CALC: DISPLAY_DATA_RQ_REGS_ST\n");p!("DML_RQ_DLG_CALC:    chunk_size              = 0x%0x\n",(*r).chunk_size);p!("DML_RQ_DLG_CALC:    min_chunk_size          = 0x%0x\n",(*r).min_chunk_size);p!("DML_RQ_DLG_CALC:    meta_chunk_size         = 0x%0x\n",(*r).meta_chunk_size);p!("DML_RQ_DLG_CALC:    min_meta_chunk_size     = 0x%0x\n",(*r).min_meta_chunk_size);p!("DML_RQ_DLG_CALC:    dpte_group_size         = 0x%0x\n",(*r).dpte_group_size);p!("DML_RQ_DLG_CALC:    mpte_group_size         = 0x%0x\n",(*r).mpte_group_size);p!("DML_RQ_DLG_CALC:    swath_height            = 0x%0x\n",(*r).swath_height);p!("DML_RQ_DLG_CALC:    pte_row_height_linear   = 0x%0x\n",(*r).pte_row_height_linear);p!("DML_RQ_DLG_CALC: =====================================\n");}

/* The register-printing structures are supplied by the surrounding DML bindings. */
extern "C" {
    pub fn print__rq_regs_st(mode_lib:*mut display_mode_lib, rq_regs:*const _vcs_dpi_display_rq_regs_st);
    pub fn print__dlg_regs_st(mode_lib:*mut display_mode_lib, dlg_regs:*const _vcs_dpi_display_dlg_regs_st);
    pub fn print__ttu_regs_st(mode_lib:*mut display_mode_lib, ttu_regs:*const _vcs_dpi_display_ttu_regs_st);
}
#[repr(C)] pub struct _vcs_dpi_display_rq_regs_st { _private:[u8;0] }
#[repr(C)] pub struct _vcs_dpi_display_dlg_regs_st { _private:[u8;0] }
#[repr(C)] pub struct _vcs_dpi_display_ttu_regs_st { _private:[u8;0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
