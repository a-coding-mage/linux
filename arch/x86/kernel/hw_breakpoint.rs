// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HW_breakpoint: a unified kernel/user-space hardware breakpoint facility,
 * using the CPU's debug registers.
 *
 * Copyright (C) 2007 Alan Stern
 * Copyright (C) 2009 IBM Corporation
 * Copyright (C) 2009 Frederic Weisbecker <fweisbec@gmail.com>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/* Per cpu debug control register value */
#[no_mangle]
pub static mut cpu_dr7: unsigned_long = 0;

/* Per cpu debug address registers values */
static mut cpu_debugreg: [unsigned_long; HBP_NUM] = [0; HBP_NUM];

/* Stores the breakpoints currently in use on each breakpoint address register. */
static mut bp_per_reg: [*mut perf_event; HBP_NUM] = [core::ptr::null_mut(); HBP_NUM];

#[inline]
unsafe fn __encode_dr7(drnum: c_int, len: c_uint, r#type: c_uint) -> unsigned_long {
    let mut bp_info = ((len | r#type) & 0xf) as unsigned_long;
    bp_info <<= DR_CONTROL_SHIFT + drnum * DR_CONTROL_SIZE;
    bp_info |= DR_GLOBAL_ENABLE << (drnum * DR_ENABLE_SIZE);
    bp_info
}

/* Encode the length, type, Exact, and Enable bits for a breakpoint in DR7. */
#[no_mangle]
pub unsafe fn encode_dr7(drnum: c_int, len: c_uint, r#type: c_uint) -> unsigned_long {
    __encode_dr7(drnum, len, r#type) | DR_GLOBAL_SLOWDOWN
}

/* Decode the length and type bits for a breakpoint stored in DR7. */
#[no_mangle]
pub unsafe fn decode_dr7(dr7: unsigned_long, bpnum: c_int, len: *mut c_uint,
                         r#type: *mut c_uint) -> c_int {
    let bp_info = (dr7 >> (DR_CONTROL_SHIFT + bpnum * DR_CONTROL_SIZE)) as c_int;
    *len = ((bp_info & 0xc) | 0x40) as c_uint;
    *r#type = ((bp_info & 0x3) | 0x80) as c_uint;
    ((dr7 >> (bpnum * DR_ENABLE_SIZE)) & 0x3) as c_int
}

#[no_mangle]
pub unsafe fn arch_install_hw_breakpoint(bp: *mut perf_event) -> c_int {
    let info = counter_arch_bp(bp);
    let mut i = 0;
    lockdep_assert_irqs_disabled();
    while i < HBP_NUM {
        if bp_per_reg[i].is_null() { bp_per_reg[i] = bp; break; }
        i += 1;
    }
    if WARN_ONCE(i == HBP_NUM, "Can't find any breakpoint slot") { return -EBUSY; }
    set_debugreg((*info).address, i as c_int);
    cpu_debugreg[i] = (*info).address;
    let dr7 = &mut cpu_dr7;
    *dr7 |= encode_dr7(i as c_int, (*info).len, (*info).r#type);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    set_debugreg(*dr7, 7);
    if (*info).mask != 0 { amd_set_dr_addr_mask((*info).mask, i as c_int); }
    0
}

#[no_mangle]
pub unsafe fn arch_uninstall_hw_breakpoint(bp: *mut perf_event) {
    let info = counter_arch_bp(bp);
    lockdep_assert_irqs_disabled();
    let mut i = 0;
    while i < HBP_NUM {
        if bp_per_reg[i] == bp { bp_per_reg[i] = core::ptr::null_mut(); break; }
        i += 1;
    }
    if WARN_ONCE(i == HBP_NUM, "Can't find any breakpoint slot") { return; }
    let mut dr7 = cpu_dr7;
    dr7 &= !__encode_dr7(i as c_int, (*info).len, (*info).r#type);
    set_debugreg(dr7, 7);
    if (*info).mask != 0 { amd_set_dr_addr_mask(0, i as c_int); }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    cpu_dr7 = dr7;
}

unsafe fn arch_bp_generic_len(x86_len: c_int) -> c_int {
    match x86_len {
        X86_BREAKPOINT_LEN_1 => HW_BREAKPOINT_LEN_1,
        X86_BREAKPOINT_LEN_2 => HW_BREAKPOINT_LEN_2,
        X86_BREAKPOINT_LEN_4 => HW_BREAKPOINT_LEN_4,
        X86_BREAKPOINT_LEN_8 => HW_BREAKPOINT_LEN_8,
        _ => -EINVAL,
    }
}

#[no_mangle]
pub unsafe fn arch_bp_generic_fields(x86_len: c_int, x86_type: c_int,
                                      gen_len: *mut c_int, gen_type: *mut c_int) -> c_int {
    match x86_type {
        X86_BREAKPOINT_EXECUTE => {
            if x86_len != X86_BREAKPOINT_LEN_X { return -EINVAL; }
            *gen_type = HW_BREAKPOINT_X; *gen_len = core::mem::size_of::<c_ulong>() as c_int; 0
        }
        X86_BREAKPOINT_WRITE => { *gen_type = HW_BREAKPOINT_W; }
        X86_BREAKPOINT_RW => { *gen_type = HW_BREAKPOINT_W | HW_BREAKPOINT_R; }
        _ => return -EINVAL,
    };
    let len = arch_bp_generic_len(x86_len);
    if len < 0 { return -EINVAL; }
    *gen_len = len; 0
}

#[no_mangle]
pub unsafe fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> bool {
    let va = (*hw).address;
    let len = arch_bp_generic_len((*hw).len);
    WARN_ON_ONCE(len < 0);
    va >= TASK_SIZE_MAX || va.wrapping_add((len - 1) as unsigned_long) >= TASK_SIZE_MAX
}

#[inline] unsafe fn within_area(addr: unsigned_long, end: unsigned_long,
                                base: unsigned_long, size: unsigned_long) -> bool {
    end >= base && addr < base.wrapping_add(size)
}

#[inline] unsafe fn within_cpu_entry(addr: unsigned_long, end: unsigned_long) -> bool {
    if within_area(addr, end, CPU_ENTRY_AREA_BASE, CPU_ENTRY_AREA_MAP_SIZE) { return true; }
    for_each_possible_cpu!(cpu, {
        if within_area(addr, end, get_cpu_gdt_rw(cpu) as unsigned_long, GDT_SIZE) { return true; }
        if within_area(addr, end, (&per_cpu(cpu_tss_rw, cpu)) as *const _ as unsigned_long,
                       core::mem::size_of::<tss_struct>() as unsigned_long) { return true; }
        if within_area(addr, end, (&per_cpu(cpu_tlbstate, cpu)) as *const _ as unsigned_long,
                       core::mem::size_of::<tlb_state>() as unsigned_long) { return true; }
        if within_area(addr, end, (&per_cpu(cpu_dr7, cpu)) as *const _ as unsigned_long,
                       core::mem::size_of::<unsigned_long>() as unsigned_long) { return true; }
    });
    false
}

unsafe fn arch_build_bp_info(_bp: *mut perf_event, attr: *const perf_event_attr,
                             hw: *mut arch_hw_breakpoint) -> c_int {
    let bp_end = (*attr).bp_addr.wrapping_add((*attr).bp_len).wrapping_sub(1);
    if bp_end < (*attr).bp_addr || within_cpu_entry((*attr).bp_addr, bp_end) { return -EINVAL; }
    (*hw).address = (*attr).bp_addr; (*hw).mask = 0;
    match (*attr).bp_type {
        HW_BREAKPOINT_W => (*hw).r#type = X86_BREAKPOINT_WRITE,
        x if x == (HW_BREAKPOINT_W | HW_BREAKPOINT_R) => (*hw).r#type = X86_BREAKPOINT_RW,
        HW_BREAKPOINT_X => {
            if (*attr).bp_addr >= TASK_SIZE_MAX && within_kprobe_blacklist((*attr).bp_addr) { return -EINVAL; }
            (*hw).r#type = X86_BREAKPOINT_EXECUTE;
            if (*attr).bp_len == core::mem::size_of::<c_long>() as unsigned_long {
                (*hw).len = X86_BREAKPOINT_LEN_X; return 0;
            }
            return -EINVAL;
        }
        _ => return -EINVAL,
    }
    match (*attr).bp_len {
        HW_BREAKPOINT_LEN_1 => (*hw).len = X86_BREAKPOINT_LEN_1,
        HW_BREAKPOINT_LEN_2 => (*hw).len = X86_BREAKPOINT_LEN_2,
        HW_BREAKPOINT_LEN_4 => (*hw).len = X86_BREAKPOINT_LEN_4,
        HW_BREAKPOINT_LEN_8 => (*hw).len = X86_BREAKPOINT_LEN_8,
        n => { if !is_power_of_2(n) || ((*attr).bp_addr & (n - 1)) != 0 { return -EINVAL; }
               if !boot_cpu_has(X86_FEATURE_BPEXT) { return -EOPNOTSUPP; }
               (*hw).mask = n - 1; (*hw).len = X86_BREAKPOINT_LEN_1; }
    }; 0
}

#[no_mangle]
pub unsafe fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr,
                                       hw: *mut arch_hw_breakpoint) -> c_int {
    let ret = arch_build_bp_info(bp, attr, hw); if ret != 0 { return ret; }
    let align = match (*hw).len { X86_BREAKPOINT_LEN_1 => (*hw).mask,
        X86_BREAKPOINT_LEN_2 => 1, X86_BREAKPOINT_LEN_4 => 3,
        X86_BREAKPOINT_LEN_8 => 7, _ => { WARN_ON_ONCE(true); return -EINVAL; } };
    if ((*hw).address & align) != 0 { return -EINVAL; } 0
}

#[no_mangle]
pub unsafe fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct) {
    let t = &mut (*tsk).thread;
    for i in 0..HBP_NUM { unregister_hw_breakpoint(t.ptrace_bps[i]); t.ptrace_bps[i] = core::ptr::null_mut(); }
    t.virtual_dr6 = 0; t.ptrace_dr7 = 0;
}

#[no_mangle]
pub unsafe fn hw_breakpoint_restore() {
    for i in 0..4 { set_debugreg(cpu_debugreg[i], i as c_int); }
    set_debugreg(DR6_RESERVED, 6); set_debugreg(cpu_dr7, 7);
}

unsafe fn hw_breakpoint_handler(args: *mut die_args) -> c_int {
    let dr6_p = ERR_PTR((*args).err) as *mut unsigned_long; let dr6 = *dr6_p;
    if dr6 & DR_TRAP_BITS == 0 { return NOTIFY_DONE; }
    for i in 0..HBP_NUM {
        if dr6 & (DR_TRAP0 << i) == 0 { continue; }
        let bp = bp_per_reg[i]; if bp.is_null() { continue; }
        let bpx = (*(*bp).hw.info).r#type == X86_BREAKPOINT_EXECUTE;
        if bpx && dr6 & DR_STEP != 0 { continue; }
        *dr6_p &= !(DR_TRAP0 << i); perf_bp_event(bp, (*args).regs);
        if bpx { (*(*args).regs).flags |= X86_EFLAGS_RF; }
    }
    if ((*current).thread.virtual_dr6 & DR_TRAP_BITS) != 0 || (dr6 & !DR_TRAP_BITS) != 0 { NOTIFY_DONE } else { NOTIFY_STOP }
}

#[no_mangle]
pub unsafe fn hw_breakpoint_exceptions_notify(_unused: *mut notifier_block, val: unsigned_long, data: *mut c_void) -> c_int {
    if val != DIE_DEBUG { return NOTIFY_DONE; } hw_breakpoint_handler(data as *mut die_args)
}

#[no_mangle]
pub unsafe fn hw_breakpoint_pmu_read(_bp: *mut perf_event) { /* TODO */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
