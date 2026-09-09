// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/hw_breakpoint.c
 *
 * Unified kernel/user-space hardware breakpoint facility for the on-chip UBC.
 *
 * Copyright (C) 2009 - 2010  Paul Mundt
 */

// Kernel and architecture dependencies supplied by other translation units.

static mut BP_PER_REG: [[*mut perf_event; HBP_NUM]; NR_CPUS] = [[core::ptr::null_mut(); HBP_NUM]; NR_CPUS];
static mut UBC_DUMMY: sh_ubc = sh_ubc { num_events: 0, ..unsafe { core::mem::zeroed() } };
static mut SH_UBC: *mut sh_ubc = unsafe { &raw mut UBC_DUMMY };

pub unsafe fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32 {
    let info = counter_arch_bp(bp);
    let mut i = 0;
    while i < (*SH_UBC).num_events {
        let slot = &mut BP_PER_REG[get_cpu()][i];
        if (*slot).is_null() {
            *slot = bp;
            break;
        }
        i += 1;
    }
    if i == (*SH_UBC).num_events {
        if warn_once(true, "Can't find any breakpoint slot") { }
        return -EBUSY;
    }
    clk_enable((*SH_UBC).clk);
    ((*SH_UBC).enable)(info, i);
    0
}

pub unsafe fn arch_uninstall_hw_breakpoint(bp: *mut perf_event) {
    let info = counter_arch_bp(bp);
    let mut i = 0;
    while i < (*SH_UBC).num_events {
        let slot = &mut BP_PER_REG[get_cpu()][i];
        if *slot == bp {
            *slot = core::ptr::null_mut();
            break;
        }
        i += 1;
    }
    if i == (*SH_UBC).num_events {
        if warn_once(true, "Can't find any breakpoint slot") { }
        return;
    }
    ((*SH_UBC).disable)(info, i);
    clk_disable((*SH_UBC).clk);
}

unsafe fn get_hbp_len(hbp_len: u16) -> u32 {
    match hbp_len {
        SH_BREAKPOINT_LEN_1 => 1,
        SH_BREAKPOINT_LEN_2 => 2,
        SH_BREAKPOINT_LEN_4 => 4,
        SH_BREAKPOINT_LEN_8 => 8,
        _ => 0,
    }
}

pub unsafe fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> bool {
    let va = (*hw).address;
    let len = get_hbp_len((*hw).len) as usize;
    va >= TASK_SIZE && va.wrapping_add(len).wrapping_sub(1) >= TASK_SIZE
}

pub unsafe fn arch_bp_generic_fields(sh_len: i32, sh_type: i32, gen_len: *mut i32, gen_type: *mut i32) -> i32 {
    *gen_len = match sh_len {
        SH_BREAKPOINT_LEN_1 => HW_BREAKPOINT_LEN_1,
        SH_BREAKPOINT_LEN_2 => HW_BREAKPOINT_LEN_2,
        SH_BREAKPOINT_LEN_4 => HW_BREAKPOINT_LEN_4,
        SH_BREAKPOINT_LEN_8 => HW_BREAKPOINT_LEN_8,
        _ => return -EINVAL,
    };
    *gen_type = match sh_type {
        SH_BREAKPOINT_READ => HW_BREAKPOINT_R,
        SH_BREAKPOINT_WRITE => HW_BREAKPOINT_W,
        SH_BREAKPOINT_RW => HW_BREAKPOINT_W | HW_BREAKPOINT_R,
        _ => return -EINVAL,
    };
    0
}

unsafe fn arch_build_bp_info(_bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32 {
    (*hw).address = (*attr).bp_addr;
    (*hw).len = match (*attr).bp_len {
        HW_BREAKPOINT_LEN_1 => SH_BREAKPOINT_LEN_1,
        HW_BREAKPOINT_LEN_2 => SH_BREAKPOINT_LEN_2,
        HW_BREAKPOINT_LEN_4 => SH_BREAKPOINT_LEN_4,
        HW_BREAKPOINT_LEN_8 => SH_BREAKPOINT_LEN_8,
        _ => return -EINVAL,
    };
    (*hw).type_ = match (*attr).bp_type {
        HW_BREAKPOINT_R => SH_BREAKPOINT_READ,
        HW_BREAKPOINT_W => SH_BREAKPOINT_WRITE,
        x if x == (HW_BREAKPOINT_W | HW_BREAKPOINT_R) => SH_BREAKPOINT_RW,
        _ => return -EINVAL,
    };
    0
}

pub unsafe fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32 {
    let ret = arch_build_bp_info(bp, attr, hw);
    if ret != 0 { return ret; }
    let align = match (*hw).len {
        SH_BREAKPOINT_LEN_1 => 0,
        SH_BREAKPOINT_LEN_2 => 1,
        SH_BREAKPOINT_LEN_4 => 3,
        SH_BREAKPOINT_LEN_8 => 7,
        _ => return -EINVAL,
    };
    if (*hw).address & align != 0 { return -EINVAL; }
    0
}

pub unsafe fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct) {
    let t = &mut (*tsk).thread;
    for i in 0..(*SH_UBC).num_events {
        unregister_hw_breakpoint(t.ptrace_bps[i]);
        t.ptrace_bps[i] = core::ptr::null_mut();
    }
}

unsafe fn hw_breakpoint_handler(args: *mut die_args) -> i32 {
    let mut rc = NOTIFY_STOP;
    let cmf = ((*SH_UBC).triggered_mask)();
    if cmf == 0 { return NOTIFY_DONE; }
    let mut resume_mask = ((*SH_UBC).active_mask)();
    ((*SH_UBC).disable_all)();
    let cpu = get_cpu();
    for i in 0..(*SH_UBC).num_events {
        let event_mask = 1u32 << i;
        if cmf & event_mask == 0 { continue; }
        rcu_read_lock();
        let bp = BP_PER_REG[cpu][i];
        if !bp.is_null() { rc = NOTIFY_DONE; }
        ((*SH_UBC).clear_triggered_mask)(event_mask);
        if bp.is_null() { rcu_read_unlock(); break; }
        if (*bp).overflow_handler == Some(ptrace_triggered) { resume_mask &= !(1u32 << i); }
        perf_bp_event(bp, (*args).regs);
        if !arch_check_bp_in_kernelspace(&mut (*bp).hw.info) { force_sig_fault(SIGTRAP, TRAP_HWBKPT, core::ptr::null_mut()); }
        rcu_read_unlock();
    }
    ((*SH_UBC).enable_all)(resume_mask);
    put_cpu();
    rc
}

pub unsafe fn breakpoint(regs: *mut pt_regs) {
    let ex = lookup_exception_vector();
    notify_die(DIE_BREAKPOINT, "breakpoint", regs, 0, ex, SIGTRAP);
}

pub unsafe fn hw_breakpoint_exceptions_notify(_unused: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let args = data as *mut die_args;
    if val != DIE_BREAKPOINT || (*args).trapnr != (*SH_UBC).trap_nr { return NOTIFY_DONE; }
    hw_breakpoint_handler(args)
}

pub unsafe fn hw_breakpoint_pmu_read(_bp: *mut perf_event) { /* TODO */ }

pub unsafe fn register_sh_ubc(ubc: *mut sh_ubc) -> i32 {
    if SH_UBC != &raw mut UBC_DUMMY { return -EBUSY; }
    SH_UBC = ubc;
    pr_info!("HW Breakpoints: {} UBC support registered\n", (*ubc).name);
    warn_on((*ubc).num_events > HBP_NUM);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
