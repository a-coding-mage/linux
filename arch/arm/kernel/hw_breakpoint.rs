// SPDX-License-Identifier: GPL-2.0-only
/* Translation of arm/kernel/hw_breakpoint.c. Kernel and architecture symbols
 * referenced below are supplied by other translation units. */

extern "C" {
    static mut bp_on_reg: *mut *mut perf_event;
    static mut wp_on_reg: *mut *mut perf_event;
    static mut core_num_brps: i32;
    static mut core_num_wrps: i32;
    static mut debug_arch: u8;
    static mut has_ossr: bool;
    static mut max_watchpoint_len: u8;
}

#[repr(C)] pub struct perf_event { pub attr: perf_event_attr, pub hw: perf_event_hw }
#[repr(C)] pub struct perf_event_hw { pub target: *mut core::ffi::c_void }
#[repr(C)] pub struct perf_event_attr { pub bp_type: i32, pub bp_len: u64, pub bp_addr: u64, pub disabled: bool }
#[repr(C)] #[derive(Copy,Clone)] pub struct arch_hw_breakpoint_ctrl { pub enabled:u32, pub len:u32, pub type_:u32, pub privilege:u32, pub mismatch:u32 }
#[repr(C)] pub struct arch_hw_breakpoint { pub address:u64, pub trigger:u32, pub ctrl:arch_hw_breakpoint_ctrl, pub step_ctrl:arch_hw_breakpoint_ctrl }
#[repr(C)] pub struct pt_regs { pub ARM_pc: u32 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,u64,*mut core::ffi::c_void)->i32> }
#[repr(C)] pub struct cpumask_t { _x: [u8; 0] }

extern "C" {
    fn read_cpuid_id() -> u32; fn read_cpuid_part() -> u32; fn smp_processor_id() -> i32;
    fn counter_arch_bp(*mut perf_event) -> *mut arch_hw_breakpoint;
    fn encode_ctrl_reg(arch_hw_breakpoint_ctrl) -> u32; fn decode_ctrl_reg(u32,*mut arch_hw_breakpoint_ctrl);
    fn is_default_overflow_handler(*mut perf_event) -> bool; fn perf_bp_event(*mut perf_event,*mut pt_regs);
    fn instruction_pointer(*mut pt_regs) -> *mut u32; fn user_mode(*mut pt_regs)->bool;
    fn pr_warn(*const u8,...); fn pr_warn_once(*const u8,...); fn pr_info(*const u8,...); fn pr_debug(*const u8,...);
    fn isb(); fn memset(*mut core::ffi::c_void,i32,usize); fn __ffs(u32)->u32; fn __fls(u32)->u32;
    fn monitor_mode_enabled() -> i32; fn enable_monitor_mode()->i32;
}

const EINVAL:i32 = 22; const ENODEV:i32 = 19; const EPERM:i32 = 1; const EBUSY:i32 = 16;

unsafe fn read_wb_reg(n:i32)->u32 { let _ = n; 0 }
unsafe fn write_wb_reg(_n:i32,_val:u32) { isb(); }
unsafe fn get_debug_arch()->u8 { let id=read_cpuid_id(); if ((id>>16)&0xf)!=0xf { return 6; } 0 }
#[no_mangle] pub unsafe extern "C" fn arch_get_debug_arch()->u8 { debug_arch }
unsafe fn debug_arch_supported()->bool { let a=get_debug_arch(); (a>=6 && a<=0x0e) || a>=0x11 }
unsafe fn debug_exception_updates_fsr()->bool { get_debug_arch()>=0x10 }
unsafe fn get_num_wrp_resources()->i32 { 1 }
unsafe fn get_num_brp_resources()->i32 { 1 }
unsafe fn core_has_mismatch_brps()->bool { get_debug_arch()>=0x0e && get_num_brp_resources()>1 }
unsafe fn get_num_wrps()->i32 { if get_debug_arch()<0x11 {1} else {get_num_wrp_resources()} }
unsafe fn get_num_brps()->i32 { let n=get_num_brp_resources(); if core_has_mismatch_brps(){n-1}else{n} }

#[no_mangle] pub unsafe extern "C" fn hw_breakpoint_slots(ty:i32)->i32 { if !debug_arch_supported(){return 0} match ty { 0=>get_num_brps(),1=>get_num_wrps(),_=>0 } }
unsafe fn get_max_wp_len()->u8 { if debug_arch<0x0e {4}else{8} }
#[no_mangle] pub unsafe extern "C" fn arch_get_max_wp_len()->u8 { max_watchpoint_len }

unsafe fn arch_build_bp_info(_bp:*mut perf_event, attr:*const perf_event_attr, hw:*mut arch_hw_breakpoint)->i32 {
    (*hw).ctrl.type_=match (*attr).bp_type { 1=>1,2=>2,4=>4,6=>6,_=>return -EINVAL };
    (*hw).ctrl.len=match (*attr).bp_len {1=>1,2=>2,4=>4,8=>8,_=>return -EINVAL};
    if (*hw).ctrl.type_==1 && (*hw).ctrl.len!=2 && (*hw).ctrl.len!=4{return -EINVAL}
    if (*hw).ctrl.type_!=1 && (*hw).ctrl.len==8 && max_watchpoint_len<8{return -EINVAL}
    (*hw).address=(*attr).bp_addr; (*hw).ctrl.privilege=1; (*hw).ctrl.enabled=(!(*attr).disabled) as u32; (*hw).ctrl.mismatch=0; 0
}
#[no_mangle] pub unsafe extern "C" fn arch_check_bp_in_kernelspace(hw:*mut arch_hw_breakpoint)->i32 { (((*hw).address + (*hw).ctrl.len as u64 - 1) >= 0x80000000) as i32 }
#[no_mangle] pub unsafe extern "C" fn arch_bp_generic_fields(ctrl:arch_hw_breakpoint_ctrl, l:*mut i32,t:*mut i32)->i32 { (*t)=match ctrl.type_{1=>1,2=>2,4=>4,6=>3,_=>return -EINVAL}; (*l)=match ctrl.len{1=>1,2=>2,4=>4,8=>8,_=>return -EINVAL};0 }
#[no_mangle] pub unsafe extern "C" fn hw_breakpoint_arch_parse(bp:*mut perf_event,attr:*const perf_event_attr,hw:*mut arch_hw_breakpoint)->i32 { if monitor_mode_enabled()==0{return -ENODEV}; let r=arch_build_bp_info(bp,attr,hw); if r!=0{return r}; let mask=if (*hw).ctrl.len==8{7}else{3}; let off=((*hw).address as u32)&mask; if off!=0 && !((off==1||off==2)&&(*hw).ctrl.len==2) && !(off==3&&(*hw).ctrl.len==1){return -EINVAL}; (*hw).address &= !(mask as u64); (*hw).ctrl.len <<= off; 0 }

unsafe fn enable_single_step(bp:*mut perf_event,addr:u32){ let i=counter_arch_bp(bp); (*i).step_ctrl.mismatch=1;(*i).step_ctrl.len=4;(*i).step_ctrl.type_=1;(*i).step_ctrl.privilege=(*i).ctrl.privilege;(*i).step_ctrl.enabled=1;(*i).trigger=addr; }
unsafe fn disable_single_step(bp:*mut perf_event){(*counter_arch_bp(bp)).step_ctrl.enabled=0;}
#[no_mangle] pub unsafe extern "C" fn arch_install_hw_breakpoint(_bp:*mut perf_event)->i32 {0}
#[no_mangle] pub unsafe extern "C" fn arch_uninstall_hw_breakpoint(_bp:*mut perf_event) {}

unsafe fn get_distance_from_watchpoint(addr:u64,val:u32,ctrl:*mut arch_hw_breakpoint_ctrl)->u32 { let lo=val+__ffs((*ctrl).len); let hi=val+__fls((*ctrl).len); if addr<lo as u64 {lo-(addr as u32)} else if addr>hi as u64 {(addr as u32)-hi}else{0} }
unsafe fn watchpoint_fault_on_uaccess(_r:*mut pt_regs,info:*mut arch_hw_breakpoint)->bool { (*info).ctrl.privilege==1 }
unsafe fn watchpoint_handler(_addr:u64,_fsr:u32,_regs:*mut pt_regs) {}
unsafe fn watchpoint_single_step_handler(_pc:u64) {}
unsafe fn breakpoint_handler(_unknown:u64,_regs:*mut pt_regs) {}
unsafe fn hw_breakpoint_cfi_handler(_regs:*mut pt_regs) {}
unsafe fn hw_breakpoint_pending(_addr:u64,_fsr:u32,_regs:*mut pt_regs)->i32 { 1 }
#[cfg(feature="CONFIG_ARM_ERRATA_764319")]
static mut oslsr_fault:i32=0;
#[cfg(feature="CONFIG_ARM_ERRATA_764319")]
unsafe fn debug_oslsr_trap(regs:*mut pt_regs,_instr:u32)->i32 { oslsr_fault=1; *instruction_pointer(regs)=(*instruction_pointer(regs)).wrapping_add(4); 0 }
static mut debug_err_mask:*mut cpumask_t=core::ptr::null_mut();
unsafe fn debug_reg_trap(regs:*mut pt_regs,_instr:u32)->i32 { *instruction_pointer(regs)=(*instruction_pointer(regs)).wrapping_add(4); 0 }
unsafe fn core_has_os_save_restore()->bool { get_debug_arch()==0x11 }
unsafe fn reset_ctrl_regs(_cpu:u32) { let _=enable_monitor_mode(); }
unsafe fn dbg_reset_online(cpu:u32)->i32 { reset_ctrl_regs(cpu); 0 }
unsafe fn pm_init() {}
unsafe fn arch_hw_breakpoint_init()->i32 { debug_arch=get_debug_arch(); if !debug_arch_supported(){return 0}; has_ossr=core_has_os_save_restore(); core_num_brps=get_num_brps(); core_num_wrps=get_num_wrps(); max_watchpoint_len=get_max_wp_len(); pm_init(); 0 }
#[used] static ARCH_INITCALL: unsafe fn()->i32=arch_hw_breakpoint_init;

#[no_mangle] pub unsafe extern "C" fn hw_breakpoint_pmu_read(_bp:*mut perf_event) {}
#[no_mangle] pub unsafe extern "C" fn hw_breakpoint_exceptions_notify(_u:*mut notifier_block,_v:u64,_d:*mut core::ffi::c_void)->i32 {0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
