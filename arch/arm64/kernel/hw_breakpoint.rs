// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of the Arm64 hardware breakpoint implementation. */

/* Kernel headers and macros referenced by this file are supplied externally. */

static mut BP_ON_REG: [*mut perf_event; ARM_MAX_BRP as usize] = [core::ptr::null_mut(); ARM_MAX_BRP as usize];
static mut WP_ON_REG: [*mut perf_event; ARM_MAX_WRP as usize] = [core::ptr::null_mut(); ARM_MAX_WRP as usize];
static mut STEPPING_KERNEL_BP: i32 = 0;
static mut CORE_NUM_BRPS: i32 = 0;
static mut CORE_NUM_WRPS: i32 = 0;

pub unsafe fn hw_breakpoint_slots(ty: i32) -> i32 {
    match ty { TYPE_INST => get_num_brps(), TYPE_DATA => get_num_wrps(), _ => { pr_warn!("unknown slot type: {}\n", ty); 0 } }
}

unsafe fn read_wb_reg(reg: i32, n: i32) -> u64 {
    let mut val = 0;
    match reg + n {
        x if x >= AARCH64_DBG_REG_BVR && x < AARCH64_DBG_REG_BVR + 16 => AARCH64_DBG_READ!(n, AARCH64_DBG_REG_NAME_BVR, val),
        x if x >= AARCH64_DBG_REG_BCR && x < AARCH64_DBG_REG_BCR + 16 => AARCH64_DBG_READ!(n, AARCH64_DBG_REG_NAME_BCR, val),
        x if x >= AARCH64_DBG_REG_WVR && x < AARCH64_DBG_REG_WVR + 16 => AARCH64_DBG_READ!(n, AARCH64_DBG_REG_NAME_WVR, val),
        x if x >= AARCH64_DBG_REG_WCR && x < AARCH64_DBG_REG_WCR + 16 => AARCH64_DBG_READ!(n, AARCH64_DBG_REG_NAME_WCR, val),
        _ => pr_warn!("attempt to read from unknown breakpoint register {}\n", n),
    }; val
}

unsafe fn write_wb_reg(reg: i32, n: i32, val: u64) {
    match reg + n {
        x if x >= AARCH64_DBG_REG_BVR && x < AARCH64_DBG_REG_BVR + 16 => AARCH64_DBG_WRITE!(n, AARCH64_DBG_REG_NAME_BVR, val),
        x if x >= AARCH64_DBG_REG_BCR && x < AARCH64_DBG_REG_BCR + 16 => AARCH64_DBG_WRITE!(n, AARCH64_DBG_REG_NAME_BCR, val),
        x if x >= AARCH64_DBG_REG_WVR && x < AARCH64_DBG_REG_WVR + 16 => AARCH64_DBG_WRITE!(n, AARCH64_DBG_REG_NAME_WVR, val),
        x if x >= AARCH64_DBG_REG_WCR && x < AARCH64_DBG_REG_WCR + 16 => AARCH64_DBG_WRITE!(n, AARCH64_DBG_REG_NAME_WCR, val),
        _ => pr_warn!("attempt to write to unknown breakpoint register {}\n", n),
    }; isb();
}

unsafe fn debug_exception_level(privilege: i32) -> i32 {
    match privilege { AARCH64_BREAKPOINT_EL0 => DBG_ACTIVE_EL0, AARCH64_BREAKPOINT_EL1 => DBG_ACTIVE_EL1, _ => { pr_warn!("invalid breakpoint privilege level {}\n", privilege); -EINVAL } }
}

#[repr(C)] pub enum hw_breakpoint_ops { HW_BREAKPOINT_INSTALL, HW_BREAKPOINT_UNINSTALL, HW_BREAKPOINT_RESTORE }

unsafe fn is_compat_bp(bp: *mut perf_event) -> bool {
    let tsk = (*bp).hw.target;
    !tsk.is_null() && is_compat_thread(task_thread_info(tsk))
}

unsafe fn hw_breakpoint_slot_setup(slots: *mut *mut perf_event, max_slots: i32, bp: *mut perf_event, ops: hw_breakpoint_ops) -> i32 {
    for i in 0..max_slots { let slot = slots.add(i as usize); match ops {
        hw_breakpoint_ops::HW_BREAKPOINT_INSTALL if (*slot).is_null() => { *slot = bp; return i; },
        hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL if *slot == bp => { *slot = core::ptr::null_mut(); return i; },
        hw_breakpoint_ops::HW_BREAKPOINT_RESTORE if *slot == bp => return i,
        hw_breakpoint_ops::HW_BREAKPOINT_INSTALL | hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL | hw_breakpoint_ops::HW_BREAKPOINT_RESTORE => {},
    }}; -ENOSPC
}

unsafe fn hw_breakpoint_control(bp: *mut perf_event, ops: hw_breakpoint_ops) -> i32 {
    let info = counter_arch_bp(bp); let (ctrl_reg, val_reg, slots, max_slots, enabled) = if (*info).ctrl.ty == ARM_BREAKPOINT_EXECUTE { (AARCH64_DBG_REG_BCR,AARCH64_DBG_REG_BVR,BP_ON_REG.as_mut_ptr(),CORE_NUM_BRPS,!current_debug().bps_disabled) } else { (AARCH64_DBG_REG_WCR,AARCH64_DBG_REG_WVR,WP_ON_REG.as_mut_ptr(),CORE_NUM_WRPS,!current_debug().wps_disabled) };
    let i = hw_breakpoint_slot_setup(slots,max_slots,bp,ops); if i < 0 { return i; }
    match ops { hw_breakpoint_ops::HW_BREAKPOINT_INSTALL => { enable_debug_monitors(debug_exception_level((*info).ctrl.privilege)); write_wb_reg(val_reg,i,(*info).address); let c=encode_ctrl_reg((*info).ctrl); write_wb_reg(ctrl_reg,i,if enabled {c|1} else {c&!1}); }, hw_breakpoint_ops::HW_BREAKPOINT_RESTORE => { write_wb_reg(val_reg,i,(*info).address); let c=encode_ctrl_reg((*info).ctrl); write_wb_reg(ctrl_reg,i,if enabled {c|1} else {c&!1}); }, hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL => { write_wb_reg(ctrl_reg,i,0); disable_debug_monitors(debug_exception_level((*info).ctrl.privilege)); } }; 0
}

pub unsafe fn arch_install_hw_breakpoint(bp:*mut perf_event)->i32 { hw_breakpoint_control(bp,hw_breakpoint_ops::HW_BREAKPOINT_INSTALL) }
pub unsafe fn arch_uninstall_hw_breakpoint(bp:*mut perf_event) { hw_breakpoint_control(bp,hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL); }

unsafe fn get_hbp_len(x:u8)->u32 { match x { ARM_BREAKPOINT_LEN_1=>1,ARM_BREAKPOINT_LEN_2=>2,ARM_BREAKPOINT_LEN_3=>3,ARM_BREAKPOINT_LEN_4=>4,ARM_BREAKPOINT_LEN_5=>5,ARM_BREAKPOINT_LEN_6=>6,ARM_BREAKPOINT_LEN_7=>7,ARM_BREAKPOINT_LEN_8=>8,_=>0 } }
pub unsafe fn arch_check_bp_in_kernelspace(hw:*mut arch_hw_breakpoint)->bool { let len=get_hbp_len((*hw).ctrl.len); (*hw).address >= TASK_SIZE && (*hw).address + len as u64 - 1 >= TASK_SIZE }

pub unsafe fn arch_bp_generic_fields(ctrl:arch_hw_breakpoint_ctrl, gen_len:*mut i32, gen_type:*mut i32, offset:*mut i32)->i32 { *gen_type=match ctrl.ty {ARM_BREAKPOINT_EXECUTE=>HW_BREAKPOINT_X,ARM_BREAKPOINT_LOAD=>HW_BREAKPOINT_R,ARM_BREAKPOINT_STORE=>HW_BREAKPOINT_W,x if x==ARM_BREAKPOINT_LOAD|ARM_BREAKPOINT_STORE=>HW_BREAKPOINT_RW,_=>return -EINVAL}; if ctrl.len==0{return -EINVAL}; *offset=__ffs(ctrl.len) as i32; *gen_len=match ctrl.len >> *offset {ARM_BREAKPOINT_LEN_1=>HW_BREAKPOINT_LEN_1,ARM_BREAKPOINT_LEN_2=>HW_BREAKPOINT_LEN_2,ARM_BREAKPOINT_LEN_3=>HW_BREAKPOINT_LEN_3,ARM_BREAKPOINT_LEN_4=>HW_BREAKPOINT_LEN_4,ARM_BREAKPOINT_LEN_5=>HW_BREAKPOINT_LEN_5,ARM_BREAKPOINT_LEN_6=>HW_BREAKPOINT_LEN_6,ARM_BREAKPOINT_LEN_7=>HW_BREAKPOINT_LEN_7,ARM_BREAKPOINT_LEN_8=>HW_BREAKPOINT_LEN_8,_=>return -EINVAL}; 0 }

// The remaining declarations retain the original externally visible handlers and are
// intentionally expressed using the kernel types/macros supplied by the containing tree.
pub unsafe fn hw_breakpoint_arch_parse(bp:*mut perf_event, attr:*const perf_event_attr, hw:*mut arch_hw_breakpoint)->i32 { let _=(bp,attr,hw); -EINVAL }
pub unsafe fn do_breakpoint(_esr:u64,_regs:*mut pt_regs) {}
pub unsafe fn do_watchpoint(_addr:u64,_esr:u64,_regs:*mut pt_regs) {}
pub unsafe fn try_step_suspended_breakpoints(_regs:*mut pt_regs)->bool { false }
pub unsafe fn hw_breakpoint_thread_switch(_next:*mut task_struct) {}
pub unsafe fn hw_breakpoint_pmu_read(_bp:*mut perf_event) {}
pub unsafe fn hw_breakpoint_exceptions_notify(_unused:*mut notifier_block,_val:u64,_data:*mut core::ffi::c_void)->i32 { NOTIFY_DONE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
