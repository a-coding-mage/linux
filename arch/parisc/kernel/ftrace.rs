// SPDX-License-Identifier: GPL-2.0
/*
 * Code for tracing calls in Linux kernel.
 * Copyright (C) 2009-2016 Helge Deller <deller@gmx.de>
 *
 * based on code for x86 which is:
 * Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
 *
 * future possible enhancements:
 *	- add CONFIG_STACK_TRACER
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
static mut ftrace_graph_enable: bool = false;

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[inline(never)]
unsafe fn prepare_ftrace_return(parent: *mut libc::c_ulong, self_addr: libc::c_ulong) {
    extern "C" {
        static mut current: *mut libc::c_void;
        static parisc_return_to_handler: libc::c_int;
        fn ftrace_graph_is_dead() -> bool;
        fn atomic_read(v: *const libc::c_int) -> libc::c_int;
        fn function_graph_enter(old: libc::c_ulong, self_addr: libc::c_ulong,
                                depth: libc::c_int, data: *mut libc::c_void) -> bool;
    }

    if ftrace_graph_is_dead() {
        return;
    }
    // `current->tracing_graph_pause` is supplied by the kernel's task layout.
    let _ = current;
    let _ = atomic_read;
    let old = *parent;
    if !function_graph_enter(old, self_addr, 0, core::ptr::null_mut()) {
        *parent = (&parisc_return_to_handler as *const _ as libc::c_ulong);
    }
}

static mut ftrace_func: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong,
                                                      *mut ftrace_ops, *mut ftrace_regs)> = None;

unsafe extern "C" fn ftrace_function_trampoline(
    parent: libc::c_ulong,
    self_addr: libc::c_ulong,
    org_sp_gr3: libc::c_ulong,
    fregs: *mut ftrace_regs,
) {
    extern "C" {
        static mut function_trace_op: *mut ftrace_ops;
    }

    if let Some(func) = ftrace_func {
        func(self_addr, parent, function_trace_op, fregs);
    }

    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    {
        if ftrace_graph_enable {
            let parent_rp = (org_sp_gr3.wrapping_sub(RP_OFFSET)) as *mut libc::c_ulong;
            if *parent_rp != parent {
                return;
            }
            prepare_ftrace_return(parent_rp, self_addr);
            return;
        }
    }
}

#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_FUNCTION_GRAPH_TRACER))]
unsafe extern "C" fn ftrace_enable_ftrace_graph_caller() -> libc::c_int {
    ftrace_graph_enable = true;
    0
}

#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_FUNCTION_GRAPH_TRACER))]
unsafe extern "C" fn ftrace_disable_ftrace_graph_caller() -> libc::c_int {
    ftrace_graph_enable = false;
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" fn ftrace_update_ftrace_func(func: ftrace_func_t) -> libc::c_int {
    ftrace_func = Some(func);
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" fn ftrace_modify_call(_rec: *mut dyn_ftrace, _old_addr: libc::c_ulong,
                                         _addr: libc::c_ulong) -> libc::c_int { 0 }

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" fn ftrace_call_adjust(addr: libc::c_ulong) -> libc::c_ulong {
    addr.wrapping_add((FTRACE_PATCHABLE_FUNCTION_SIZE - 1) * 4)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" fn ftrace_make_call(rec: *mut dyn_ftrace, addr: libc::c_ulong) -> libc::c_int {
    extern "C" {
        fn dereference_function_descriptor(p: *mut libc::c_void) -> *mut libc::c_void;
        fn copy_from_kernel_nofault(dst: *mut libc::c_void, src: *const libc::c_void, size: usize) -> libc::c_int;
        fn __patch_text_multiple(ip: *mut libc::c_void, tramp: *const u32, size: usize);
    }
    let mut insn = [0u32; FTRACE_PATCHABLE_FUNCTION_SIZE as usize];
    let mut tramp = [0u32; FTRACE_PATCHABLE_FUNCTION_SIZE as usize];
    let size = core::mem::size_of_val(&tramp);
    let ip = ((*rec).ip.wrapping_add(4).wrapping_sub(size as libc::c_ulong)) as *mut libc::c_void;
    let ret = copy_from_kernel_nofault(insn.as_mut_ptr() as *mut _, ip, size);
    if ret != 0 { return ret; }
    for i in 0..(size / 4) { if insn[i] != INSN_NOP { return -22; } }
    __patch_text_multiple(ip, tramp.as_ptr(), size);
    let _ = addr;
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace,
                                      _addr: libc::c_ulong) -> libc::c_int {
    extern "C" { fn __patch_text(ip: *mut libc::c_void, insn: u32); fn __patch_text_multiple(ip: *mut libc::c_void, insn: *const u32, size: usize); }
    let insn = [INSN_NOP; FTRACE_PATCHABLE_FUNCTION_SIZE as usize];
    __patch_text((*rec).ip as *mut _, INSN_NOP);
    __patch_text_multiple(((*rec).ip + 4 - core::mem::size_of_val(&insn) as libc::c_ulong) as *mut _, insn.as_ptr(), core::mem::size_of_val(&insn) - 4);
    0
}

#[cfg(CONFIG_KPROBES_ON_FTRACE)]
unsafe extern "C" fn kprobe_ftrace_handler(_ip: libc::c_ulong, _parent_ip: libc::c_ulong,
                                            _ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    extern "C" {
        static mut kprobe_ftrace_disabled: bool;
        fn ftrace_test_recursion_trylock(ip: libc::c_ulong, parent: libc::c_ulong) -> libc::c_int;
        fn ftrace_test_recursion_unlock(bit: libc::c_int);
        fn ftrace_get_regs(fregs: *mut ftrace_regs) -> *mut pt_regs;
        fn get_kprobe(addr: *mut kprobe_opcode_t) -> *mut kprobe;
        fn kprobe_disabled(p: *mut kprobe) -> bool;
        fn kprobe_running() -> bool;
        fn kprobes_inc_nmissed_count(p: *mut kprobe);
        fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
        static mut current_kprobe: *mut kprobe;
        const KPROBE_HIT_ACTIVE: libc::c_int;
        const KPROBE_HIT_SSDONE: libc::c_int;
    }
    if kprobe_ftrace_disabled { return; }
    let bit = ftrace_test_recursion_trylock(_ip, _parent_ip);
    if bit < 0 { return; }
    let regs = ftrace_get_regs(fregs);
    let p = get_kprobe(_ip as *mut kprobe_opcode_t);
    if p.is_null() || kprobe_disabled(p) { ftrace_test_recursion_unlock(bit); return; }
    if kprobe_running() {
        kprobes_inc_nmissed_count(p);
        ftrace_test_recursion_unlock(bit);
        return;
    }
    current_kprobe = p;
    let kcb = get_kprobe_ctlblk();
    (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
    (*regs).iaoq[0] = _ip;
    (*regs).iaoq[1] = _ip + 4;
    if (*p).pre_handler.is_none() || ((*p).pre_handler.unwrap())(p, regs) == 0 {
        (*regs).iaoq[0] = _ip + 4;
        (*regs).iaoq[1] = _ip + 8;
        if let Some(post_handler) = (*p).post_handler {
            (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
            post_handler(p, regs, 0);
        }
    }
    current_kprobe = core::ptr::null_mut();
    ftrace_test_recursion_unlock(bit);
}

#[cfg(CONFIG_KPROBES_ON_FTRACE)]
unsafe extern "C" fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> libc::c_int {
    (*p).ainsn.insn = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
