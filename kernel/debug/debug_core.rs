// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel Debug Core
 *
 * Direct Rust translation of debug_core.c. Kernel-provided types, constants,
 * functions, macros, and configuration symbols are intentionally referenced
 * as external dependencies.
 */

static mut KGDB_BREAK_ASAP: i32 = 0;
static mut KGDB_INFO: [debuggerinfo_struct; NR_CPUS] = [debuggerinfo_struct::ZERO; NR_CPUS];
pub static mut KGDB_CONNECTED: i32 = 0;
pub static mut KGDB_IO_MODULE_REGISTERED: i32 = 0;
static mut EXCEPTION_LEVEL: i32 = 0;
static mut DBG_IO_OPS: *mut kgdb_io = core::ptr::null_mut();
static mut KGDBREBOOT: i32 = 0;
static mut KGDB_CON_REGISTERED: i32 = 0;
static mut KGDB_USE_CON: i32 = 0;
pub static mut DBG_IS_EARLY: bool = true;
pub static mut DBG_SWITCH_CPU: i32 = 0;
pub static mut DBG_KDB_MODE: i32 = 1;
static mut KGDB_BREAK: [kgdb_bkpt; KGDB_MAX_BREAKPOINTS] = [kgdb_bkpt::UNDEFINED; KGDB_MAX_BREAKPOINTS];
pub static mut KGDB_ACTIVE: atomic_t = ATOMIC_INIT(-1);
static mut MASTERS_IN_KGDB: atomic_t = atomic_t::ZERO;
static mut SLAVES_IN_KGDB: atomic_t = atomic_t::ZERO;
pub static mut KGDB_SETTING_BREAKPOINT: atomic_t = atomic_t::ZERO;
pub static mut KGDB_USETHREAD: *mut task_struct = core::ptr::null_mut();
pub static mut KGDB_CONTHREAD: *mut task_struct = core::ptr::null_mut();
pub static mut KGDB_SINGLE_STEP: i32 = 0;
static mut KGDB_SSTEP_PID: pid_t = 0;
pub static mut KGDB_CPU_DOING_SINGLE_STEP: atomic_t = ATOMIC_INIT(-1);
static mut KGDB_DO_ROUNDUP: i32 = 1;

unsafe fn opt_nokgdbroundup(_str: *mut i8) -> i32 { KGDB_DO_ROUNDUP = 0; 0 }

pub unsafe fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    let mut err = copy_from_kernel_nofault((*bpt).saved_instr.as_mut_ptr(), (*bpt).bpt_addr as *mut i8, BREAK_INSTR_SIZE);
    if err != 0 { return err; }
    err = copy_to_kernel_nofault((*bpt).bpt_addr as *mut i8, arch_kgdb_ops.gdb_bpt_instr, BREAK_INSTR_SIZE);
    err
}

pub unsafe fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    copy_to_kernel_nofault((*bpt).bpt_addr as *mut i8, (*bpt).saved_instr.as_mut_ptr(), BREAK_INSTR_SIZE)
}

pub unsafe fn kgdb_validate_break_address(addr: c_ulong) -> i32 {
    if kgdb_within_blocklist(addr) { return -EINVAL; }
    let mut tmp = kgdb_bkpt::UNDEFINED;
    tmp.bpt_addr = addr;
    let mut err = kgdb_arch_set_breakpoint(&mut tmp);
    if err != 0 { return err; }
    err = kgdb_arch_remove_breakpoint(&mut tmp);
    if err != 0 { pr_err!("Critical breakpoint error, kernel memory destroyed at: %lx\n", addr); }
    err
}

pub unsafe fn kgdb_arch_pc(_exception: i32, regs: *mut pt_regs) -> c_ulong { instruction_pointer(regs) }
pub unsafe fn kgdb_arch_init() -> i32 { 0 }
pub unsafe fn kgdb_skipexception(_exception: i32, _regs: *mut pt_regs) -> i32 { 0 }

pub unsafe fn kgdb_call_nmi_hook(_ignored: *mut core::ffi::c_void) {
    kgdb_nmicallback(raw_smp_processor_id(), get_irq_regs());
}

pub unsafe fn kgdb_roundup_cpus() {
    let this_cpu = raw_smp_processor_id();
    for_each_online_cpu!(cpu, {
        if cpu == this_cpu { continue; }
        let csd = &mut per_cpu!(kgdb_roundup_csd, cpu);
        if KGDB_INFO[cpu as usize].rounding_up { continue; }
        KGDB_INFO[cpu as usize].rounding_up = true;
        if smp_call_function_single_async(cpu, csd) != 0 { KGDB_INFO[cpu as usize].rounding_up = false; }
    });
}

unsafe fn kgdb_flush_swbreak_addr(addr: c_ulong) {
    if !CACHE_FLUSH_IS_SAFE { return; }
    flush_icache_range(addr, addr.wrapping_add(BREAK_INSTR_SIZE as c_ulong));
}

pub unsafe fn dbg_activate_sw_breakpoints() -> i32 {
    let mut ret = 0;
    for i in 0..KGDB_MAX_BREAKPOINTS {
        if KGDB_BREAK[i].state != BP_SET { continue; }
        let error = kgdb_arch_set_breakpoint(&mut KGDB_BREAK[i]);
        if error != 0 { ret = error; pr_info!("BP install failed: %lx\n", KGDB_BREAK[i].bpt_addr); continue; }
        kgdb_flush_swbreak_addr(KGDB_BREAK[i].bpt_addr);
        KGDB_BREAK[i].state = BP_ACTIVE;
    }
    ret
}

pub unsafe fn dbg_set_sw_break(addr: c_ulong) -> i32 {
    let err = kgdb_validate_break_address(addr);
    if err != 0 { return err; }
    for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_SET && KGDB_BREAK[i].bpt_addr == addr { return -EEXIST; } }
    let mut breakno: i32 = -1;
    for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_REMOVED && KGDB_BREAK[i].bpt_addr == addr { breakno = i as i32; break; } }
    if breakno == -1 { for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_UNDEFINED { breakno = i as i32; break; } } }
    if breakno == -1 { return -E2BIG; }
    let b = &mut KGDB_BREAK[breakno as usize]; b.state = BP_SET; b.type_ = BP_BREAKPOINT; b.bpt_addr = addr; 0
}

pub unsafe fn dbg_deactivate_sw_breakpoints() -> i32 {
    let mut ret = 0;
    for i in 0..KGDB_MAX_BREAKPOINTS {
        if KGDB_BREAK[i].state != BP_ACTIVE { continue; }
        let error = kgdb_arch_remove_breakpoint(&mut KGDB_BREAK[i]);
        if error != 0 { pr_info!("BP remove failed: %lx\n", KGDB_BREAK[i].bpt_addr); ret = error; }
        kgdb_flush_swbreak_addr(KGDB_BREAK[i].bpt_addr); KGDB_BREAK[i].state = BP_SET;
    } ret
}

pub unsafe fn dbg_remove_sw_break(addr: c_ulong) -> i32 {
    for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_SET && KGDB_BREAK[i].bpt_addr == addr { KGDB_BREAK[i].state = BP_REMOVED; return 0; } }
    -ENOENT
}
pub unsafe fn kgdb_isremovedbreak(addr: c_ulong) -> i32 { for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_REMOVED && KGDB_BREAK[i].bpt_addr == addr { return 1; } } 0 }
pub unsafe fn kgdb_has_hit_break(addr: c_ulong) -> i32 { for i in 0..KGDB_MAX_BREAKPOINTS { if KGDB_BREAK[i].state == BP_ACTIVE && KGDB_BREAK[i].bpt_addr == addr { return 1; } } 0 }

pub unsafe fn dbg_remove_all_break() -> i32 {
    for i in 0..KGDB_MAX_BREAKPOINTS {
        if KGDB_BREAK[i].state == BP_ACTIVE { let error = kgdb_arch_remove_breakpoint(&mut KGDB_BREAK[i]); if error != 0 { pr_err!("breakpoint remove failed: %lx\n", KGDB_BREAK[i].bpt_addr); } }
        KGDB_BREAK[i].state = BP_UNDEFINED;
    }
    if let Some(f) = arch_kgdb_ops.remove_all_hw_break { f(); } 0
}

pub unsafe fn kgdb_free_init_mem() { for i in 0..KGDB_MAX_BREAKPOINTS { if init_section_contains(KGDB_BREAK[i].bpt_addr as *mut core::ffi::c_void, 0) { KGDB_BREAK[i].state = BP_UNDEFINED; } } }

unsafe fn kgdb_io_ready(print_wait: i32) -> i32 {
    if DBG_IO_OPS.is_null() { return 0; }
    if KGDB_CONNECTED != 0 || atomic_read(&KGDB_SETTING_BREAKPOINT) != 0 { return 1; }
    if print_wait != 0 { pr_crit!("Waiting for remote debugger\n"); }
    1
}

unsafe fn dbg_touch_watchdogs() { touch_softlockup_watchdog_sync(); clocksource_touch_watchdog(); rcu_cpu_stall_reset(); }

pub unsafe fn kgdb_handle_exception(evector: i32, signo: i32, ecode: i32, regs: *mut pt_regs) -> i32 {
    if signo != SIGTRAP && panic_timeout != 0 { return 1; }
    let mut ks = kgdb_state::zeroed(); ks.cpu = raw_smp_processor_id(); ks.ex_vector = evector; ks.signo = signo; ks.err_code = ecode; ks.linux_regs = regs;
    if kgdb_info[ks.cpu as usize].enter_kgdb != 0 { return 0; }
    kgdb_cpu_enter(&mut ks, regs, DCPU_WANT_MASTER)
}

unsafe fn module_event(_self_: *mut notifier_block, _val: c_ulong, _data: *mut core::ffi::c_void) -> i32 { 0 }

pub unsafe fn kgdb_nmicallback(cpu: i32, regs: *mut core::ffi::c_void) -> i32 {
    KGDB_INFO[cpu as usize].rounding_up = false;
    if KGDB_INFO[cpu as usize].enter_kgdb == 0 && raw_spin_is_locked(&dbg_master_lock) { let mut ks = kgdb_state::zeroed(); ks.cpu = cpu; ks.linux_regs = regs; kgdb_cpu_enter(&mut ks, regs as *mut pt_regs, DCPU_IS_SLAVE); return 0; } 1
}

pub unsafe fn kgdb_nmicallin(cpu: i32, trapnr: i32, regs: *mut core::ffi::c_void, err_code: i32, send_ready: *mut atomic_t) -> i32 {
    if !kgdb_io_ready(0) || send_ready.is_null() { return 1; }
    if KGDB_INFO[cpu as usize].enter_kgdb == 0 { let mut ks = kgdb_state::zeroed(); ks.cpu = cpu; ks.ex_vector = trapnr; ks.signo = SIGTRAP; ks.err_code = err_code; ks.linux_regs = regs; ks.send_ready = send_ready; kgdb_cpu_enter(&mut ks, regs as *mut pt_regs, DCPU_WANT_MASTER); return 0; } 1
}

unsafe fn kgdb_console_write(_co: *mut console, s: *const i8, count: c_uint) {
    if KGDB_CONNECTED == 0 || atomic_read(&KGDB_ACTIVE) != -1 || DBG_KDB_MODE != 0 { return; }
    let flags = local_irq_save(); gdbstub_msg_write(s, count); local_irq_restore(flags);
}

unsafe fn opt_kgdb_con(_str: *mut i8) -> i32 { KGDB_USE_CON = 1; 0 }

unsafe fn kgdb_cpu_enter(ks: *mut kgdb_state, regs: *mut pt_regs, exception_state: i32) -> i32 {
    KGDB_INFO[(*ks).cpu as usize].enter_kgdb += 1;
    KGDB_INFO[(*ks).cpu as usize].exception_state |= exception_state;
    if exception_state == DCPU_WANT_MASTER { atomic_inc(&mut MASTERS_IN_KGDB); } else { atomic_inc(&mut SLAVES_IN_KGDB); }
    if let Some(f) = arch_kgdb_ops.disable_hw_break { f(regs); }
    let cpu = (*ks).cpu;
    KGDB_INFO[cpu as usize].debuggerinfo = regs;
    KGDB_INFO[cpu as usize].task = current;
    KGDB_INFO[cpu as usize].ret_state = 0;
    smp_mb();
    if exception_level == 1 { if raw_spin_trylock(&mut dbg_master_lock) { atomic_xchg(&mut KGDB_ACTIVE, cpu); } }
    while atomic_read(&KGDB_ACTIVE) != cpu && exception_state == DCPU_IS_SLAVE { cpu_relax(); }
    if !kgdb_io_ready(1) { KGDB_INFO[cpu as usize].ret_state = 1; }
    else if kgdb_skipexception((*ks).ex_vector, (*ks).linux_regs) == 0 {
        atomic_inc(&mut ignore_console_lock_warning);
        if let Some(f) = (*DBG_IO_OPS).pre_exception { f(); }
        if KGDB_SINGLE_STEP == 0 { raw_spin_lock(&mut dbg_slave_lock); }
        dbg_deactivate_sw_breakpoints(); KGDB_SINGLE_STEP = 0; KGDB_CONTHREAD = current; EXCEPTION_LEVEL = 0;
        loop {
            let error = if DBG_KDB_MODE != 0 { KGDB_CONNECTED = 1; let e = kdb_stub(ks); KGDB_CONNECTED = 0; e } else { gdb_serial_stub(ks) };
            if error == -1 { continue; }
            if error == DBG_PASS_EVENT { DBG_KDB_MODE = !DBG_KDB_MODE; continue; }
            if error == DBG_SWITCH_CPU_EVENT { KGDB_INFO[DBG_SWITCH_CPU as usize].exception_state |= DCPU_NEXT_MASTER; continue; }
            KGDB_INFO[cpu as usize].ret_state = error; break;
        }
        dbg_activate_sw_breakpoints(); if let Some(f) = (*DBG_IO_OPS).post_exception { f(); }
        atomic_dec(&mut ignore_console_lock_warning); if KGDB_SINGLE_STEP == 0 { raw_spin_unlock(&mut dbg_slave_lock); }
    }
    if let Some(f) = arch_kgdb_ops.correct_hw_break { f(); }
    KGDB_INFO[cpu as usize].debuggerinfo = core::ptr::null_mut(); KGDB_INFO[cpu as usize].task = core::ptr::null_mut();
    KGDB_INFO[cpu as usize].exception_state &= !(DCPU_WANT_MASTER | DCPU_IS_SLAVE);
    KGDB_INFO[cpu as usize].enter_kgdb -= 1; atomic_dec(&mut MASTERS_IN_KGDB); atomic_set(&mut KGDB_ACTIVE, -1);
    raw_spin_unlock(&mut dbg_master_lock); dbg_touch_watchdogs(); KGDB_INFO[cpu as usize].ret_state
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
