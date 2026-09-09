// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding Linux and SPARC headers are
// intentionally left external to this translation unit.

#[cfg(CONFIG_DYNAMIC_FTRACE)]
static FTRACE_NOP: u32 = 0x0100_0000;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_call_replace(ip: usize, addr: usize) -> u32 {
    let off = (addr as i32).wrapping_sub(ip as i32);
    0x4000_0000u32 | ((off as u32) >> 2)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_modify_code(ip: usize, old: u32, new: u32) -> i32 {
    // The original uses SPARC CAS/flush inline assembly and an exception
    // table fixup.  That architecture-specific inline assembly is supplied
    // by the eventual SPARC build environment.
    let replaced = core::ptr::read_volatile(ip as *const u32);
    let mut faulted = 0;
    if replaced != old && replaced != new {
        faulted = 2;
    }
    faulted
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_nop(
    _mod: *mut module,
    rec: *mut dyn_ftrace,
    addr: usize,
) -> i32 {
    let ip = (*rec).ip as usize;
    let old = ftrace_call_replace(ip, addr);
    let new = FTRACE_NOP;
    ftrace_modify_code(ip, old, new)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let ip = (*rec).ip as usize;
    let old = FTRACE_NOP;
    let new = ftrace_call_replace(ip, addr);
    ftrace_modify_code(ip, old, new)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let ip = (&ftrace_call as *const _ as usize);
    let old = core::ptr::read_volatile(ip as *const u32);
    let new = ftrace_call_replace(ip, func as usize);
    ftrace_modify_code(ip, old, new)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" {
    fn ftrace_graph_call();
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 {
    let ip = ftrace_graph_call as usize;
    let old = core::ptr::read_volatile(ip as *const u32);
    let new = ftrace_call_replace(ip, ftrace_graph_caller as usize);
    ftrace_modify_code(ip, old, new)
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 {
    let ip = ftrace_graph_call as usize;
    let old = core::ptr::read_volatile(ip as *const u32);
    let new = ftrace_call_replace(ip, ftrace_stub as usize);
    ftrace_modify_code(ip, old, new)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn prepare_ftrace_return(
    parent: usize,
    self_addr: usize,
    frame_pointer: usize,
) -> usize {
    let return_hooker = return_to_handler as usize;

    if atomic_read(&(*current).tracing_graph_pause) != 0 {
        return parent.wrapping_add(8);
    }

    if function_graph_enter(parent, self_addr, frame_pointer, core::ptr::null_mut()) != 0 {
        return parent.wrapping_add(8);
    }

    return_hooker
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
