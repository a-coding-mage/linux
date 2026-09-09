/*
 * Xtensa hardware breakpoints/watchpoints handling functions
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2016 Cadence Design Systems Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut BP_ON_REG: [*mut perf_event; XCHAL_NUM_IBREAK] = [core::ptr::null_mut(); XCHAL_NUM_IBREAK];
static mut WP_ON_REG: [*mut perf_event; XCHAL_NUM_DBREAK] = [core::ptr::null_mut(); XCHAL_NUM_DBREAK];

pub unsafe fn hw_breakpoint_slots(ty: i32) -> i32 {
    match ty {
        TYPE_INST => XCHAL_NUM_IBREAK as i32,
        TYPE_DATA => XCHAL_NUM_DBREAK as i32,
        _ => {
            pr_warn!("unknown slot type: {}\n", ty);
            0
        }
    }
}

pub unsafe fn arch_check_bp_in_kernelspace(hw: *const arch_hw_breakpoint) -> i32 {
    let va = (*hw).address;
    let len = (*hw).len;
    ((va >= TASK_SIZE) && (va.wrapping_add(len).wrapping_sub(1) >= TASK_SIZE)) as i32
}

/* Construct an arch_hw_breakpoint from a perf_event. */
pub unsafe fn hw_breakpoint_arch_parse(
    _bp: *mut perf_event,
    attr: *const perf_event_attr,
    hw: *mut arch_hw_breakpoint,
) -> i32 {
    (*hw).ty = match (*attr).bp_type {
        HW_BREAKPOINT_X => XTENSA_BREAKPOINT_EXECUTE,
        HW_BREAKPOINT_R => XTENSA_BREAKPOINT_LOAD,
        HW_BREAKPOINT_W => XTENSA_BREAKPOINT_STORE,
        HW_BREAKPOINT_RW => XTENSA_BREAKPOINT_LOAD | XTENSA_BREAKPOINT_STORE,
        _ => return -EINVAL,
    };

    (*hw).len = (*attr).bp_len;
    if (*hw).len < 1 || (*hw).len > 64 || !(*hw).len.is_power_of_two() {
        return -EINVAL;
    }

    (*hw).address = (*attr).bp_addr;
    if (*hw).address & ((*hw).len - 1) != 0 {
        return -EINVAL;
    }
    0
}

pub unsafe fn hw_breakpoint_exceptions_notify(
    _unused: *mut notifier_block,
    _val: usize,
    _data: *mut core::ffi::c_void,
) -> i32 {
    NOTIFY_DONE
}

unsafe fn xtensa_wsr(v: usize, sr: u8) {
    // Indexed wsr is unavailable; keep the explicit switch used by the C source.
    BUILD_BUG_ON!(XCHAL_NUM_IBREAK > 2);
    BUILD_BUG_ON!(XCHAL_NUM_DBREAK > 2);
    match sr {
        SREG_IBREAKA if XCHAL_NUM_IBREAK > 0 => xtensa_set_sr(v, SREG_IBREAKA),
        s if XCHAL_NUM_IBREAK > 1 && s == SREG_IBREAKA + 1 => xtensa_set_sr(v, SREG_IBREAKA + 1),
        SREG_DBREAKA if XCHAL_NUM_DBREAK > 0 => xtensa_set_sr(v, SREG_DBREAKA),
        SREG_DBREAKC if XCHAL_NUM_DBREAK > 0 => xtensa_set_sr(v, SREG_DBREAKC),
        s if XCHAL_NUM_DBREAK > 1 && s == SREG_DBREAKA + 1 => xtensa_set_sr(v, SREG_DBREAKA + 1),
        s if XCHAL_NUM_DBREAK > 1 && s == SREG_DBREAKC + 1 => xtensa_set_sr(v, SREG_DBREAKC + 1),
        _ => {}
    }
}

unsafe fn alloc_slot(slot: *mut *mut perf_event, n: usize, bp: *mut perf_event) -> i32 {
    for i in 0..n {
        if (*slot.add(i)).is_null() {
            *slot.add(i) = bp;
            return i as i32;
        }
    }
    -EBUSY
}

unsafe fn set_ibreak_regs(reg: i32, bp: *mut perf_event) {
    let info = counter_arch_bp(bp);
    xtensa_wsr((*info).address, SREG_IBREAKA + reg as u8);
    let ibreakenable = xtensa_get_sr(SREG_IBREAKENABLE);
    xtensa_set_sr(ibreakenable | (1usize << reg), SREG_IBREAKENABLE);
}

unsafe fn set_dbreak_regs(reg: i32, bp: *mut perf_event) {
    let info = counter_arch_bp(bp);
    let mut dbreakc = DBREAKC_MASK_MASK & (0usize.wrapping_sub((*info).len));
    if (*info).ty & XTENSA_BREAKPOINT_LOAD != 0 { dbreakc |= DBREAKC_LOAD_MASK; }
    if (*info).ty & XTENSA_BREAKPOINT_STORE != 0 { dbreakc |= DBREAKC_STOR_MASK; }
    xtensa_wsr((*info).address, SREG_DBREAKA + reg as u8);
    xtensa_wsr(dbreakc, SREG_DBREAKC + reg as u8);
}

pub unsafe fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32 {
    let i;
    if (*counter_arch_bp(bp)).ty == XTENSA_BREAKPOINT_EXECUTE {
        i = alloc_slot(BP_ON_REG.as_mut_ptr(), XCHAL_NUM_IBREAK, bp);
        if i < 0 { return i; }
        set_ibreak_regs(i, bp);
    } else {
        i = alloc_slot(WP_ON_REG.as_mut_ptr(), XCHAL_NUM_DBREAK, bp);
        if i < 0 { return i; }
        set_dbreak_regs(i, bp);
    }
    0
}

unsafe fn free_slot(slot: *mut *mut perf_event, n: usize, bp: *mut perf_event) -> i32 {
    for i in 0..n {
        if *slot.add(i) == bp {
            *slot.add(i) = core::ptr::null_mut();
            return i as i32;
        }
    }
    -EBUSY
}

pub unsafe fn arch_uninstall_hw_breakpoint(bp: *mut perf_event) {
    let info = counter_arch_bp(bp);
    let i;
    if (*info).ty == XTENSA_BREAKPOINT_EXECUTE {
        i = free_slot(BP_ON_REG.as_mut_ptr(), XCHAL_NUM_IBREAK, bp);
        if i >= 0 {
            let e = xtensa_get_sr(SREG_IBREAKENABLE);
            xtensa_set_sr(e & !(1usize << i), SREG_IBREAKENABLE);
        }
    } else {
        i = free_slot(WP_ON_REG.as_mut_ptr(), XCHAL_NUM_DBREAK, bp);
        if i >= 0 { xtensa_wsr(0, SREG_DBREAKC + i as u8); }
    }
}

pub unsafe fn hw_breakpoint_pmu_read(_bp: *mut perf_event) {}

pub unsafe fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct) {
    let t = &mut (*tsk).thread;
    for i in 0..XCHAL_NUM_IBREAK {
        if !t.ptrace_bp[i].is_null() { unregister_hw_breakpoint(t.ptrace_bp[i]); t.ptrace_bp[i] = core::ptr::null_mut(); }
    }
    for i in 0..XCHAL_NUM_DBREAK {
        if !t.ptrace_wp[i].is_null() { unregister_hw_breakpoint(t.ptrace_wp[i]); t.ptrace_wp[i] = core::ptr::null_mut(); }
    }
}

/* Set ptrace breakpoint pointers to zero for this task. */
pub unsafe fn clear_ptrace_hw_breakpoint(tsk: *mut task_struct) {
    core::ptr::write_bytes((*tsk).thread.ptrace_bp.as_mut_ptr(), 0, (*tsk).thread.ptrace_bp.len());
    core::ptr::write_bytes((*tsk).thread.ptrace_wp.as_mut_ptr(), 0, (*tsk).thread.ptrace_wp.len());
}

pub unsafe fn restore_dbreak() {
    for i in 0..XCHAL_NUM_DBREAK {
        let bp = WP_ON_REG[i];
        if !bp.is_null() { set_dbreak_regs(i as i32, bp); }
    }
    clear_thread_flag(TIF_DB_DISABLED);
}

pub unsafe fn check_hw_breakpoint(regs: *mut pt_regs) -> i32 {
    if (*regs).debugcause & BIT(DEBUGCAUSE_IBREAK_BIT) != 0 {
        for i in 0..XCHAL_NUM_IBREAK {
            let bp = BP_ON_REG[i];
            if !bp.is_null() && !(*bp).attr.disabled && (*regs).pc == (*bp).attr.bp_addr { perf_bp_event(bp, regs); }
        }
        return 0;
    } else if (*regs).debugcause & BIT(DEBUGCAUSE_DBREAK_BIT) != 0 {
        let dbnum = ((*regs).debugcause & DEBUGCAUSE_DBNUM_MASK) >> DEBUGCAUSE_DBNUM_SHIFT;
        if dbnum < XCHAL_NUM_DBREAK && !WP_ON_REG[dbnum].is_null() {
            let bp = WP_ON_REG[dbnum];
            if user_mode(regs) { perf_bp_event(bp, regs); } else { set_thread_flag(TIF_DB_DISABLED); xtensa_wsr(0, SREG_DBREAKC + dbnum as u8); }
        } else { WARN_ONCE!(true, "Wrong/unconfigured DBNUM reported in DEBUGCAUSE: {}\n", dbnum); }
        return 0;
    }
    -ENOENT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
