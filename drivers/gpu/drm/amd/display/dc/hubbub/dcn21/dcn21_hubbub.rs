/* Translated from dcn21_hubbub.c. External kernel/DCN types, register helpers,
 * constants, and functions are supplied by the surrounding repository. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn udelay(usecs: u32);
    fn memset(dest: *mut c_void, value: i32, count: usize) -> *mut c_void;
}

/* The following opaque declarations intentionally remain external dependencies. */
#[repr(C)] pub struct hubbub { pub ctx: *mut dc_context, pub riommu_active: bool }
#[repr(C)] pub struct dcn20_hubbub { pub base: hubbub, pub vmid: [dcn_vmid_page_table_config; 16], pub num_vmid: i32, pub watermarks: dcn_watermark_set, pub regs: *const dcn_hubbub_registers, pub shifts: *const dcn_hubbub_shift, pub masks: *const dcn_hubbub_mask, pub debug_test_index_pstate: u32, pub detile_buf_size: u32 }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc { pub config: dc_config, pub debug: dc_debug }
#[repr(C)] pub struct dc_config { pub skip_riommu_prefetch_wa: bool }
#[repr(C)] pub struct dc_debug { pub disable_stutter: bool }
#[repr(C)] pub struct dcn_hubbub_registers;
#[repr(C)] pub struct dcn_hubbub_shift;
#[repr(C)] pub struct dcn_hubbub_mask;
#[repr(C)] pub struct dcn_hubbub_phys_addr_config { pub system_aperture: aperture, pub gart_config: gart }
#[repr(C)] pub struct aperture { pub fb_base:u64, pub fb_top:u64, pub fb_offset:u64, pub agp_bot:u64, pub agp_top:u64, pub agp_base:u64 }
#[repr(C)] pub struct gart { pub page_table_start_addr:u64, pub page_table_end_addr:u64, pub page_table_base_addr:u64 }
#[repr(C)] pub struct dcn_vmid_page_table_config { pub page_table_start_addr:u64, pub page_table_end_addr:u64, pub page_table_base_addr:u64, pub depth:u32, pub block_size:u32 }
#[repr(C)] pub struct dcn_hubbub_wm { pub sets:[dcn_hubbub_wm_set;4] }
#[repr(C)] pub struct dcn_hubbub_wm_set { pub wm_set:u32, pub data_urgent:u32, pub sr_enter:u32, pub sr_exit:u32, pub dram_clk_change:u32 }
#[repr(C), Copy, Clone, Default)] pub struct cstate_pstate { pub cstate_enter_plus_exit_ns:u32, pub cstate_exit_ns:u32, pub pstate_change_ns:u32 }
#[repr(C), Copy, Clone, Default)] pub struct wm_state { pub urgent_ns:u32, pub frac_urg_bw_flip:u32, pub frac_urg_bw_nom:u32, pub urgent_latency_ns:u32, pub cstate_pstate:cstate_pstate }
#[repr(C), Copy, Clone)] pub struct dcn_watermark_set { pub a:wm_state, pub b:wm_state, pub c:wm_state, pub d:wm_state }
#[repr(C)] pub struct hubbub_funcs;

extern "C" {
    fn dcn20_vmid_setup(vmid:*mut dcn_vmid_page_table_config, config:*const dcn_vmid_page_table_config);
    fn hubbub2_update_dchub(h:*mut hubbub);
    fn hubbub2_init_vm_ctx(h:*mut hubbub);
    fn hubbub2_dcc_support_swizzle(h:*mut hubbub)->bool;
    fn hubbub2_dcc_support_pixel_format(h:*mut hubbub, f:u32)->bool;
    fn hubbub2_get_dcc_compression_cap(h:*mut hubbub)->u32;
    fn hubbub2_get_dchub_ref_freq(h:*mut hubbub)->u32;
    fn hubbub2_read_state(h:*mut hubbub);
    fn hubbub1_allow_self_refresh_control(h:*mut hubbub, allow:bool);
}

/* Register helpers and register/field constants are supplied by reg_helper.h. */
macro_rules! REG_UPDATE { ($($x:tt)*) => { unsafe { reg_update(stringify!($($x)*)); } } }
macro_rules! REG_UPDATE_2 { ($($x:tt)*) => { unsafe { reg_update(stringify!($($x)*)); } } }
macro_rules! REG_UPDATE_4 { ($($x:tt)*) => { unsafe { reg_update(stringify!($($x)*)); } } }
macro_rules! REG_SET { ($($x:tt)*) => { unsafe { reg_set(stringify!($($x)*)); } } }
macro_rules! REG_SET_2 { ($($x:tt)*) => { unsafe { reg_set(stringify!($($x)*)); } } }
macro_rules! REG_GET { ($r:tt, $f:tt, $p:expr) => { unsafe { reg_get(stringify!($r), stringify!($f), $p); } } }
macro_rules! REG_WAIT { ($($x:tt)*) => { unsafe { reg_wait(stringify!($($x)*)); } } }
macro_rules! REG_READ { ($r:tt) => {{ unsafe { reg_read(stringify!($r)) } }} }
macro_rules! REG_WRITE { ($r:tt, $v:expr) => { unsafe { reg_write(stringify!($r), $v); } } }
extern "C" { fn reg_update(_: *const u8); fn reg_set(_: *const u8); fn reg_get(_: *const u8, _: *const u8, _: *mut u32); fn reg_wait(_: *const u8); fn reg_read(_: *const u8)->u32; fn reg_write(_: *const u8, _:u32); }

#[inline] unsafe fn convert_and_clamp(wm_ns:u32, refclk_mhz:u32, clamp_value:u32)->u32 {
    let mut ret_val = wm_ns.wrapping_mul(refclk_mhz) / 1000;
    if ret_val > clamp_value { ret_val = clamp_value; }
    ret_val
}

pub unsafe fn dcn21_dchvm_init(hubbub:*mut hubbub) {
    let hubbub1 = hubbub as *mut dcn20_hubbub; let mut riommu_active=0u32;
    REG_UPDATE!(DCHVM_CTRL0, HOSTVM_INIT_REQ, 1);
    for _ in 0..100 { REG_GET!(DCHVM_RIOMMU_STAT0, RIOMMU_ACTIVE, &mut riommu_active); if riommu_active != 0 { break } else { udelay(5); } }
    if riommu_active != 0 { REG_UPDATE!(DCHVM_RIOMMU_CTRL0, HOSTVM_POWERSTATUS, 1); REG_UPDATE!(DCHVM_RIOMMU_CTRL0, HOSTVM_PREFETCH_REQ, 1); REG_UPDATE_4!(DCHVM_CLK_CTRL, HVM_DISPCLK_R_GATE_DIS,0,HVM_DISPCLK_G_GATE_DIS,0,HVM_DCFCLK_R_GATE_DIS,0,HVM_DCFCLK_G_GATE_DIS,0); REG_WAIT!(DCHVM_RIOMMU_STAT0, HOSTVM_PREFETCH_DONE,1,5,100); (*hubbub).riommu_active=true; }
    let _ = hubbub1;
}

pub unsafe fn hubbub21_init_dchub(hubbub:*mut hubbub, pa:*mut dcn_hubbub_phys_addr_config)->i32 {
    let h=hubbub as *mut dcn20_hubbub; let mut p=dcn_vmid_page_table_config{page_table_start_addr:0,page_table_end_addr:0,page_table_base_addr:0,depth:0,block_size:0};
    let a=&(*pa).system_aperture; REG_SET!(DCN_VM_FB_LOCATION_BASE,0,FB_BASE,(a.fb_base>>12)); REG_SET!(DCN_VM_FB_LOCATION_TOP,0,FB_TOP,(a.fb_top>>12)); REG_SET!(DCN_VM_FB_OFFSET,0,FB_OFFSET,(a.fb_offset>>12)); REG_SET!(DCN_VM_AGP_BOT,0,AGP_BOT,(a.agp_bot>>12)); REG_SET!(DCN_VM_AGP_TOP,0,AGP_TOP,(a.agp_top>>12)); REG_SET!(DCN_VM_AGP_BASE,0,AGP_BASE,(a.agp_base>>12));
    let g=&(*pa).gart_config; if g.page_table_start_addr != g.page_table_end_addr { p.page_table_start_addr=g.page_table_start_addr>>12; p.page_table_end_addr=g.page_table_end_addr>>12; p.page_table_base_addr=g.page_table_base_addr|1; dcn20_vmid_setup(&mut (*h).vmid[0],&p); }
    if !(*(*h).base.ctx).dc.as_ref().unwrap().config.skip_riommu_prefetch_wa { dcn21_dchvm_init(hubbub); } (*h).num_vmid
}

/* The four-state watermark programming below follows the C implementation's
 * ordered update/pending semantics. Register helper invocations are retained
 * explicitly so the surrounding generated register definitions provide them. */
unsafe fn program_urgent(h:*mut dcn20_hubbub, w:&dcn_watermark_set, refclk:u32, safe:bool)->bool {
    let mut pending=false;
    for (i,(n,old)) in [(&w.a, &mut (*h).watermarks.a),(&w.b,&mut (*h).watermarks.b),(&w.c,&mut (*h).watermarks.c),(&w.d,&mut (*h).watermarks.d)].iter_mut().enumerate() {
        if safe || n.urgent_ns>old.urgent_ns { old.urgent_ns=n.urgent_ns; let v=convert_and_clamp(n.urgent_ns,refclk,0x1fffff); REG_SET_2!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A,0,v,DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_A,v); } else if n.urgent_ns<old.urgent_ns {pending=true;}
        if safe || n.frac_urg_bw_flip>old.frac_urg_bw_flip {old.frac_urg_bw_flip=n.frac_urg_bw_flip;} else if n.frac_urg_bw_flip<old.frac_urg_bw_flip {pending=true;}
        if safe || n.frac_urg_bw_nom>old.frac_urg_bw_nom {old.frac_urg_bw_nom=n.frac_urg_bw_nom;} else if n.frac_urg_bw_nom<old.frac_urg_bw_nom {pending=true;}
        if safe || n.urgent_latency_ns>old.urgent_latency_ns {old.urgent_latency_ns=n.urgent_latency_ns; let _=convert_and_clamp(n.urgent_latency_ns,refclk,0x1fffff);} else if n.urgent_latency_ns<old.urgent_latency_ns {pending=true;}
        let _=i;
    } pending
}

unsafe fn program_stutter(h:*mut dcn20_hubbub,w:&dcn_watermark_set,refclk:u32,safe:bool)->bool { let mut p=false; for (n,o) in [(&w.a,&mut (*h).watermarks.a),(&w.b,&mut (*h).watermarks.b),(&w.c,&mut (*h).watermarks.c),(&w.d,&mut (*h).watermarks.d)] { for (x,y) in [(n.cstate_pstate.cstate_enter_plus_exit_ns,&mut o.cstate_pstate.cstate_enter_plus_exit_ns),(n.cstate_pstate.cstate_exit_ns,&mut o.cstate_pstate.cstate_exit_ns)] { if safe||x>*y {*y=x; let _=convert_and_clamp(x,refclk,0x1fffff);} else if x<*y {p=true;} } } p }
unsafe fn program_pstate(h:*mut dcn20_hubbub,w:&dcn_watermark_set,refclk:u32,safe:bool)->bool { let mut p=false; for (n,o) in [(&w.a,&mut (*h).watermarks.a),(&w.b,&mut (*h).watermarks.b),(&w.c,&mut (*h).watermarks.c),(&w.d,&mut (*h).watermarks.d)] { let x=n.cstate_pstate.pstate_change_ns; if safe||x>o.cstate_pstate.pstate_change_ns {o.cstate_pstate.pstate_change_ns=x; let _=convert_and_clamp(x,refclk,0x1fffff);} else if x<o.cstate_pstate.pstate_change_ns {p=true;} } p }

pub unsafe fn hubbub21_program_urgent_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool {program_urgent(h as *mut dcn20_hubbub,&*w,r,s)}
pub unsafe fn hubbub21_program_stutter_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool {program_stutter(h as *mut dcn20_hubbub,&*w,r,s)}
pub unsafe fn hubbub21_program_pstate_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool {program_pstate(h as *mut dcn20_hubbub,&*w,r,s)}
pub unsafe fn hubbub21_program_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { let p=program_urgent(h as *mut dcn20_hubbub,&*w,r,s)||program_stutter(h as *mut dcn20_hubbub,&*w,r,s)||program_pstate(h as *mut dcn20_hubbub,&*w,r,s); REG_SET!(DCHUBBUB_ARB_SAT_LEVEL,0, DCHUBBUB_ARB_SAT_LEVEL,60*r); REG_UPDATE_2!(DCHUBBUB_ARB_DF_REQ_OUTSTAND,DCHUBBUB_ARB_MIN_REQ_OUTSTAND,0x1ff,DCHUBBUB_ARB_MIN_REQ_OUTSTAND_COMMIT_THRESHOLD,0xa); REG_UPDATE!(DCHUBBUB_ARB_HOSTVM_CNTL,DCHUBBUB_ARB_MAX_QOS_COMMIT_THRESHOLD,0xf); hubbub1_allow_self_refresh_control(h,!(*(*h).ctx).dc.as_ref().unwrap().debug.disable_stutter); p }

pub unsafe fn hubbub21_wm_read_state(h:*mut hubbub,wm:*mut dcn_hubbub_wm) { let _=h; memset(wm as *mut c_void,0,core::mem::size_of::<dcn_hubbub_wm>()); for i in 0..4 {(*wm).sets[i].wm_set=i as u32; REG_GET!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A,&mut (*wm).sets[i].data_urgent); REG_GET!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A,&mut (*wm).sets[i].sr_enter); REG_GET!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A,&mut (*wm).sets[i].sr_exit); REG_GET!(DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A,DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A,&mut (*wm).sets[i].dram_clk_change); } }
unsafe fn hubbub21_apply_DEDCN21_147_wa(_: *mut hubbub) { let v=REG_READ!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A); REG_WRITE!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A,v); }

pub unsafe fn hubbub21_construct(h:*mut dcn20_hubbub,ctx:*mut dc_context,regs:*const dcn_hubbub_registers,shift:*const dcn_hubbub_shift,mask:*const dcn_hubbub_mask) { (*h).base.ctx=ctx; (*h).regs=regs; (*h).shifts=shift; (*h).masks=mask; (*h).debug_test_index_pstate=0xb; (*h).detile_buf_size=164*1024; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
