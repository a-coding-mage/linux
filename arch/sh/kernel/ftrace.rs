// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Matt Fleming <matt@console-pimps.org>
 * Copyright (C) 2008 Paul Mundt <lethal@linux-sh.org>
 *
 * Code for replacing ftrace calls with jumps.
 *
 * Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
 *
 * Thanks goes to Ingo Molnar, for suggesting the idea.
 * Mathieu Desnoyers, for suggesting postponing the modifications.
 * Arjan van de Ven, for keeping me straight, and explaining to me
 * the dangers of modifying code on the run.
 */

// C headers and build-time configuration supplied by the surrounding kernel.

#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut FTRACE_REPLACED_CODE: [u8; MCOUNT_INSN_SIZE] = [0; MCOUNT_INSN_SIZE];
#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut FTRACE_NOP: [u8; 4] = [0; 4];

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_nop_replace(ip: usize) -> *mut u8 {
    __raw_writel(ip.wrapping_add(MCOUNT_INSN_SIZE), FTRACE_NOP.as_mut_ptr());
    FTRACE_NOP.as_mut_ptr()
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_call_replace(_ip: usize, addr: usize) -> *mut u8 {
    __raw_writel(addr, FTRACE_REPLACED_CODE.as_mut_ptr());
    FTRACE_REPLACED_CODE.as_mut_ptr()
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
const MOD_CODE_WRITE_FLAG: i32 = 1 << 31;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut NMI_RUNNING: atomic_t = ATOMIC_INIT(0);
#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut MOD_CODE_STATUS: i32 = 0;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut MOD_CODE_IP: *mut core::ffi::c_void = core::ptr::null_mut();
#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut MOD_CODE_NEWCODE: *mut core::ffi::c_void = core::ptr::null_mut();

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn clear_mod_flag() {
    let mut old = atomic_read(&NMI_RUNNING);
    loop {
        let new = old & !MOD_CODE_WRITE_FLAG;
        if old == new { break; }
        old = atomic_cmpxchg(&NMI_RUNNING, old, new);
    }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_mod_code() {
    MOD_CODE_STATUS = copy_to_kernel_nofault(MOD_CODE_IP, MOD_CODE_NEWCODE, MCOUNT_INSN_SIZE);
    if MOD_CODE_STATUS != 0 { clear_mod_flag(); }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn arch_ftrace_nmi_enter() {
    if atomic_inc_return(&NMI_RUNNING) & MOD_CODE_WRITE_FLAG != 0 {
        smp_rmb();
        ftrace_mod_code();
    }
    smp_mb();
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn arch_ftrace_nmi_exit() {
    smp_mb();
    atomic_dec(&NMI_RUNNING);
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn wait_for_nmi_and_set_mod_flag() {
    if atomic_cmpxchg(&NMI_RUNNING, 0, MOD_CODE_WRITE_FLAG) == 0 { return; }
    while atomic_cmpxchg(&NMI_RUNNING, 0, MOD_CODE_WRITE_FLAG) != 0 { cpu_relax(); }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn wait_for_nmi() {
    if atomic_read(&NMI_RUNNING) == 0 { return; }
    while atomic_read(&NMI_RUNNING) != 0 { cpu_relax(); }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn do_ftrace_mod_code(ip: usize, new_code: *mut core::ffi::c_void) -> i32 {
    MOD_CODE_IP = ip as *mut core::ffi::c_void;
    MOD_CODE_NEWCODE = new_code;
    smp_mb();
    wait_for_nmi_and_set_mod_flag();
    smp_mb();
    ftrace_mod_code();
    smp_mb();
    clear_mod_flag();
    wait_for_nmi();
    MOD_CODE_STATUS
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_modify_code(ip: usize, old_code: *mut u8, new_code: *mut u8) -> i32 {
    let mut replaced = [0u8; MCOUNT_INSN_SIZE];
    if copy_from_kernel_nofault(replaced.as_mut_ptr(), ip as *const core::ffi::c_void, MCOUNT_INSN_SIZE) != 0 { return -EFAULT; }
    if memcmp(replaced.as_ptr(), old_code, MCOUNT_INSN_SIZE) != 0 { return -EINVAL; }
    if do_ftrace_mod_code(ip, new_code as *mut core::ffi::c_void) != 0 { return -EPERM; }
    flush_icache_range(ip, ip.wrapping_add(MCOUNT_INSN_SIZE));
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let ip = (&ftrace_call as *const _ as usize).wrapping_add(MCOUNT_INSN_OFFSET);
    let mut old = [0u8; MCOUNT_INSN_SIZE];
    memcpy(old.as_mut_ptr(), ip as *const u8, MCOUNT_INSN_SIZE);
    let new = ftrace_call_replace(ip, func as usize);
    ftrace_modify_code(ip, old.as_mut_ptr(), new)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let ip = (*rec).ip;
    let old = ftrace_call_replace(ip, addr);
    let new = ftrace_nop_replace(ip);
    ftrace_modify_code(ip, old, new)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let ip = (*rec).ip;
    let old = ftrace_nop_replace(ip);
    let new = ftrace_call_replace(ip, addr);
    ftrace_modify_code(ip, old, new)
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
unsafe fn ftrace_mod(ip: usize, old_addr: usize, new_addr: usize) -> i32 {
    let mut code = [0u8; MCOUNT_INSN_SIZE];
    if copy_from_kernel_nofault(code.as_mut_ptr(), ip as *const core::ffi::c_void, MCOUNT_INSN_SIZE) != 0 { return -EFAULT; }
    if old_addr != __raw_readl(code.as_ptr() as *const u32) as usize { return -EINVAL; }
    __raw_writel(new_addr, ip as *mut u32);
    0
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 {
    let ip = (&ftrace_graph_call as *const _ as usize).wrapping_add(GRAPH_INSN_OFFSET);
    ftrace_mod(ip, skip_trace as usize, ftrace_graph_caller as usize)
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 {
    let ip = (&ftrace_graph_call as *const _ as usize).wrapping_add(GRAPH_INSN_OFFSET);
    ftrace_mod(ip, ftrace_graph_caller as usize, skip_trace as usize)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn prepare_ftrace_return(parent: *mut usize, self_addr: usize) {
    let mut old: usize;
    let mut faulted: i32;
    let return_hooker = return_to_handler as usize;
    if unlikely(ftrace_graph_is_dead()) || unlikely(atomic_read(&(*current).tracing_graph_pause) != 0) { return; }

    // The original SH inline assembly performs the fault-protected load/store
    // and exception-table fixup. Preserve that exact operation as a required
    // target-specific dependency; it cannot be expressed portably in Rust.
    // old = *parent; *parent = return_hooker; faulted = 0;
    old = *parent;
    *parent = return_hooker;
    faulted = 0;

    if unlikely(faulted != 0) {
        ftrace_graph_stop();
        WARN_ON(1);
        return;
    }
    if function_graph_enter(old, self_addr, 0, core::ptr::null_mut()) != 0 {
        __raw_writel(old, parent as *mut u32);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
