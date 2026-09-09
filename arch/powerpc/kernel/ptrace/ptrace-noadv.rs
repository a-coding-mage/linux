// SPDX-License-Identifier: GPL-2.0-or-later

// External kernel declarations supplied by the surrounding translation unit.

pub unsafe fn user_enable_single_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() {
        regs_set_return_msr(regs, ((*regs).msr & !MSR_BE) | MSR_SE);
    }
    set_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn user_enable_block_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() {
        regs_set_return_msr(regs, ((*regs).msr & !MSR_SE) | MSR_BE);
    }
    set_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn user_disable_single_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() {
        regs_set_return_msr(regs, (*regs).msr & !(MSR_SE | MSR_BE));
    }
    clear_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn ppc_gethwdinfo(dbginfo: *mut ppc_debug_info) {
    (*dbginfo).version = 1;
    (*dbginfo).num_instruction_bps = 0;
    (*dbginfo).num_data_bps = if ppc_breakpoint_available() { nr_wp_slots() } else { 0 };
    (*dbginfo).num_condition_regs = 0;
    (*dbginfo).data_bp_alignment = core::mem::size_of::<c_long>();
    (*dbginfo).sizeof_condition = 0;
    // CONFIG_HAVE_HW_BREAKPOINT is a build-time kernel condition.
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    {
        (*dbginfo).features = PPC_DEBUG_FEATURE_DATA_BP_RANGE;
        if dawr_enabled() {
            (*dbginfo).features |= PPC_DEBUG_FEATURE_DATA_BP_DAWR;
        }
    }
    #[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
    {
        (*dbginfo).features = 0;
    }
    if cpu_has_feature(CPU_FTR_ARCH_31) {
        (*dbginfo).features |= PPC_DEBUG_FEATURE_DATA_BP_ARCH_31;
    }
}

pub unsafe fn ptrace_get_debugreg(
    child: *mut task_struct,
    addr: c_ulong,
    datalp: *mut c_ulong,
) -> c_int {
    if addr > 0 {
        return -EINVAL;
    }
    let dabr_fake = ((*child).thread.hw_brk[0].address & !HW_BRK_TYPE_DABR)
        | ((*child).thread.hw_brk[0].type & HW_BRK_TYPE_DABR);
    put_user(dabr_fake, datalp)
}

pub unsafe fn ptrace_set_debugreg(
    task: *mut task_struct,
    addr: c_ulong,
    data: c_ulong,
) -> c_int {
    let mut set_bp = true;
    let mut hw_brk: arch_hw_breakpoint;
    if addr > 0 {
        return -EINVAL;
    }
    if (data & !0x7_u64 as c_ulong) >= TASK_SIZE {
        return -EIO;
    }
    if data != 0 && (data & HW_BRK_TYPE_TRANSLATE) == 0 {
        return -EIO;
    }
    hw_brk.address = data & !HW_BRK_TYPE_DABR;
    hw_brk.type_ = (data & HW_BRK_TYPE_DABR) | HW_BRK_TYPE_PRIV_ALL;
    hw_brk.len = DABR_MAX_LEN;
    hw_brk.hw_len = DABR_MAX_LEN;
    set_bp = data != 0 && (hw_brk.type_ & HW_BRK_TYPE_RDWR) != 0;

    // CONFIG_HAVE_HW_BREAKPOINT is a build-time kernel condition.
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    {
        let thread = &mut (*task).thread;
        let mut bp = thread.ptrace_bps[0];
        if !set_bp {
            if !bp.is_null() {
                unregister_hw_breakpoint(bp);
                thread.ptrace_bps[0] = core::ptr::null_mut();
            }
            return 0;
        }
        if !bp.is_null() {
            let mut attr = (*bp).attr;
            attr.bp_addr = hw_brk.address;
            attr.bp_len = DABR_MAX_LEN;
            arch_bp_generic_fields(hw_brk.type_, &mut attr.bp_type);
            attr.disabled = false;
            let ret = modify_user_hw_breakpoint(bp, &attr);
            if ret != 0 { return ret; }
            thread.ptrace_bps[0] = bp;
            thread.hw_brk[0] = hw_brk;
            return 0;
        }
        let mut attr: perf_event_attr;
        hw_breakpoint_init(&mut attr);
        attr.bp_addr = hw_brk.address;
        attr.bp_len = DABR_MAX_LEN;
        arch_bp_generic_fields(hw_brk.type_, &mut attr.bp_type);
        bp = register_user_hw_breakpoint(&attr, ptrace_triggered, core::ptr::null_mut(), task);
        thread.ptrace_bps[0] = bp;
        if IS_ERR(bp) {
            thread.ptrace_bps[0] = core::ptr::null_mut();
            return PTR_ERR(bp);
        }
    }
    #[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
    if set_bp && !ppc_breakpoint_available() {
        return -ENODEV;
    }
    (*task).thread.hw_brk[0] = hw_brk;
    0
}

#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn find_empty_ptrace_bp(thread: *mut thread_struct) -> c_int {
    for i in 0..nr_wp_slots() {
        if (*thread).ptrace_bps[i as usize].is_null() { return i; }
    }
    -1
}

unsafe fn find_empty_hw_brk(thread: *mut thread_struct) -> c_int {
    for i in 0..nr_wp_slots() {
        if (*thread).hw_brk[i as usize].address == 0 { return i; }
    }
    -1
}

pub unsafe fn ppc_set_hwdebug(child: *mut task_struct, bp_info: *mut ppc_hw_breakpoint) -> c_long {
    if (*bp_info).version != 1 { return -ENOTSUPP; }
    if ((*bp_info).trigger_type & PPC_BREAKPOINT_TRIGGER_RW) == 0
        || ((*bp_info).trigger_type & !PPC_BREAKPOINT_TRIGGER_RW) != 0
        || (*bp_info).condition_mode != PPC_BREAKPOINT_CONDITION_NONE { return -EINVAL; }
    if (*bp_info).addr as c_ulong >= TASK_SIZE { return -EIO; }
    let mut brk: arch_hw_breakpoint;
    brk.address = ALIGN_DOWN((*bp_info).addr, HW_BREAKPOINT_SIZE);
    brk.type_ = HW_BRK_TYPE_TRANSLATE | HW_BRK_TYPE_PRIV_ALL;
    brk.len = DABR_MAX_LEN;
    brk.hw_len = DABR_MAX_LEN;
    if (*bp_info).trigger_type & PPC_BREAKPOINT_TRIGGER_READ != 0 { brk.type_ |= HW_BRK_TYPE_READ; }
    if (*bp_info).trigger_type & PPC_BREAKPOINT_TRIGGER_WRITE != 0 { brk.type_ |= HW_BRK_TYPE_WRITE; }

    // CONFIG_HAVE_HW_BREAKPOINT is a build-time kernel condition.
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    {
        let thread = &mut (*child).thread;
        let len = if (*bp_info).addr_mode == PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE {
            (*bp_info).addr2 - (*bp_info).addr
        } else if (*bp_info).addr_mode == PPC_BREAKPOINT_MODE_EXACT { 1 } else { return -EINVAL; };
        let i = find_empty_ptrace_bp(thread);
        if i < 0 { return -ENOSPC; }
        let mut attr: perf_event_attr;
        hw_breakpoint_init(&mut attr);
        attr.bp_addr = (*bp_info).addr as c_ulong;
        attr.bp_len = len;
        arch_bp_generic_fields(brk.type_, &mut attr.bp_type);
        let bp = register_user_hw_breakpoint(&attr, ptrace_triggered, core::ptr::null_mut(), child);
        thread.ptrace_bps[i as usize] = bp;
        if IS_ERR(bp) { thread.ptrace_bps[i as usize] = core::ptr::null_mut(); return PTR_ERR(bp); }
        return (i + 1) as c_long;
    }
    #[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
    {
        if (*bp_info).addr_mode != PPC_BREAKPOINT_MODE_EXACT { return -EINVAL; }
        let i = find_empty_hw_brk(&mut (*child).thread);
        if i < 0 { return -ENOSPC; }
        if !ppc_breakpoint_available() { return -ENODEV; }
        (*child).thread.hw_brk[i as usize] = brk;
        return (i + 1) as c_long;
    }
}

pub unsafe fn ppc_del_hwdebug(child: *mut task_struct, data: c_long) -> c_long {
    if data < 1 || data > nr_wp_slots() as c_long { return -EINVAL; }
    // CONFIG_HAVE_HW_BREAKPOINT is a build-time kernel condition.
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    {
        let thread = &mut (*child).thread;
        let i = (data - 1) as usize;
        let bp = thread.ptrace_bps[i];
        if !bp.is_null() { unregister_hw_breakpoint(bp); thread.ptrace_bps[i] = core::ptr::null_mut(); } else { return -ENOENT; }
    }
    #[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
    {
        let brk = &mut (*child).thread.hw_brk[(data - 1) as usize];
        if (brk.flags & HW_BRK_FLAG_DISABLED) == 0 && brk.address == 0 { return -ENOENT; }
        brk.address = 0; brk.type_ = 0; brk.flags = 0;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
