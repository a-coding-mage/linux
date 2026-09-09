// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

// Kernel headers and architecture declarations correspond to the C includes.

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[repr(C)]
pub union LoongarchInstruction {
    pub reg2i12_format: Reg2i12Format,
    _opaque: [u8; 4],
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[repr(C)]
pub struct Reg2i12Format {
    pub immediate: i32,
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
extern "C" {
    fn is_ra_save_ins(insn: *const LoongarchInstruction) -> bool;
    fn is_stack_alloc_ins(insn: *const LoongarchInstruction) -> bool;
    fn ftrace_graph_is_dead() -> bool;
    fn atomic_read(v: *const AtomicT) -> i32;
    fn function_graph_enter(
        old: usize,
        self_addr: usize,
        depth: i32,
        ret: *mut core::ffi::c_void,
    ) -> bool;
    fn ftrace_graph_stop();
    fn warn_on(condition: bool);
    fn return_to_handler();
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[repr(C)]
pub struct AtomicT {
    pub counter: i32,
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[repr(C)]
pub struct TaskStruct {
    pub tracing_graph_pause: AtomicT,
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
extern "C" {
    static mut current: *mut TaskStruct;
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
unsafe fn ftrace_get_parent_ra_addr(
    insn_addr: usize,
    ra_off: *mut i32,
) -> i32 {
    let mut limit: i32 = 32;
    let mut insn = insn_addr as *mut LoongarchInstruction;

    loop {
        insn = insn.offset(-1);
        limit -= 1;

        if is_ra_save_ins(insn) {
            let immediate = unsafe { (*insn).reg2i12_format.immediate };
            *ra_off = -((1i32 << 12) - immediate);
        }

        if is_stack_alloc_ins(insn) || limit == 0 {
            break;
        }
    }

    if limit == 0 {
        return -22; // -EINVAL
    }

    0
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn prepare_ftrace_return(
    self_addr: usize,
    callsite_sp: usize,
    old: usize,
) {
    let mut ra_off: i32 = 0;
    let return_hooker = return_to_handler as usize;

    if ftrace_graph_is_dead() {
        return;
    }

    if atomic_read(&(*current).tracing_graph_pause) != 0 {
        return;
    }

    if ftrace_get_parent_ra_addr(self_addr, &mut ra_off) != 0 {
        ftrace_graph_stop();
        warn_on(true);
        return;
    }

    if !function_graph_enter(old, self_addr, 0, core::ptr::null_mut()) {
        let return_address = (callsite_sp as *mut u8).offset(ra_off as isize) as *mut usize;
        core::ptr::write(return_address, return_hooker);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
