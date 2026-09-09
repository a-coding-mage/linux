// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 * Copyright (C) 2017 Andes Technology Corporation
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_arch_code_modify_prepare() {
    mutex_lock(&raw mut text_mutex);
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_arch_code_modify_post_process() {
    mutex_unlock(&raw mut text_mutex);
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_call_adjust(mut addr: usize) -> usize {
    if is_enabled(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS) {
        return addr + 8 + MCOUNT_AUIPC_SIZE;
    }

    addr + MCOUNT_AUIPC_SIZE
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn arch_ftrace_get_symaddr(fentry_ip: usize) -> usize {
    fentry_ip - MCOUNT_AUIPC_SIZE
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn arch_ftrace_update_code(mut command: i32) {
    command |= FTRACE_MAY_SLEEP;
    ftrace_modify_all_code(command);
    flush_icache_all();
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn __ftrace_modify_call(source: usize, target: usize, validate: bool) -> i32 {
    let mut call = [0u32; 2];
    let offset: u32 = (target - source) as u32;
    let mut replaced = [0u32; 2];

    call[1] = to_jalr_t0(offset);

    if validate {
        call[0] = to_auipc_t0(offset);
        /*
         * Read the text we want to modify;
         * return must be -EFAULT on read error
         */
        if copy_from_kernel_nofault(
            replaced.as_mut_ptr() as *mut core::ffi::c_void,
            source as *const core::ffi::c_void,
            2 * MCOUNT_INSN_SIZE,
        ) != 0 {
            return -EFAULT;
        }

        /* Bypass the check if the auipc insn is a kprobe breakpoint */
        if replaced[0] != call[0]
            && !(riscv_insn_is_ebreak(replaced[0]) || riscv_insn_is_c_ebreak(replaced[0]))
        {
            pr_err("%p: expected (%08x) but got (%08x)\n", source, call[0], replaced[0]);
            return -EINVAL;
        }
    }

    /* Replace the jalr at once. Return -EPERM on write error. */
    if patch_insn_write(
        (source + MCOUNT_AUIPC_SIZE) as *mut core::ffi::c_void,
        call[1..].as_ptr() as *const core::ffi::c_void,
        MCOUNT_JALR_SIZE,
    ) != 0 {
        return -EPERM;
    }

    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS)]
unsafe fn riscv64_rec_get_ops(rec: *mut dyn_ftrace) -> *const ftrace_ops {
    let mut ops: *const ftrace_ops = core::ptr::null();

    if (*rec).flags & FTRACE_FL_CALL_OPS_EN != 0 {
        ops = ftrace_find_unique_ops(rec);
        warn_on_once(ops.is_null());
    }

    if ops.is_null() {
        ops = &raw const ftrace_list_ops;
    }

    ops
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS)]
unsafe fn ftrace_rec_set_ops(rec: *const dyn_ftrace, ops: *const ftrace_ops) -> i32 {
    let literal = align_down((*rec).ip - 12, 8);
    patch_text_nosync(
        literal as *mut core::ffi::c_void,
        &ops as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<*const ftrace_ops>(),
    )
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS)]
unsafe fn ftrace_rec_set_nop_ops(rec: *mut dyn_ftrace) -> i32 {
    ftrace_rec_set_ops(rec, &raw const ftrace_nop_ops)
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS)]
unsafe fn ftrace_rec_update_ops(rec: *mut dyn_ftrace) -> i32 {
    ftrace_rec_set_ops(rec, riscv64_rec_get_ops(rec))
}

#[cfg(not(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS))]
unsafe fn ftrace_rec_set_nop_ops(_rec: *mut dyn_ftrace) -> i32 { 0 }

#[cfg(not(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS))]
unsafe fn ftrace_rec_update_ops(_rec: *mut dyn_ftrace) -> i32 { 0 }

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, mut addr: usize) -> i32 {
    let mut distance: usize;
    let orig_addr: usize;
    let pc = (*rec).ip - MCOUNT_AUIPC_SIZE;
    let ret = ftrace_rec_update_ops(rec);
    if ret != 0 { return ret; }

    orig_addr = ftrace_caller as usize;
    distance = if addr > orig_addr { addr - orig_addr } else { orig_addr - addr };
    if distance > JALR_RANGE { addr = FTRACE_ADDR; }

    __ftrace_modify_call(pc, addr, false)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, _addr: usize) -> i32 {
    let nop4: u32 = RISCV_INSN_NOP4;
    let ret = ftrace_rec_set_nop_ops(rec);
    if ret != 0 { return ret; }

    if patch_insn_write((*rec).ip as *mut core::ffi::c_void, &nop4 as *const _ as *const _, MCOUNT_NOP4_SIZE) != 0 {
        return -EPERM;
    }
    0
}

/*
 * This is called early on, and isn't wrapped by
 * ftrace_arch_code_modify_{prepare,post_process}() and therefore doesn't hold
 * text_mutex, which triggers a lockdep failure.  SMP isn't running so we could
 * just directly poke the text, but it's simpler to just take the lock
 * ourselves.
 */
#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_init_nop(_mod: *mut module, rec: *mut dyn_ftrace) -> i32 {
    let pc = (*rec).ip - MCOUNT_AUIPC_SIZE;
    let mut nops = [0u32; 2];
    let offset = ftrace_caller as usize - pc;

    let _guard = mutex_guard(&raw mut text_mutex);
    let ret = ftrace_rec_set_nop_ops(rec);
    if ret != 0 { return ret; }

    nops[0] = to_auipc_t0(offset as u32);
    nops[1] = RISCV_INSN_NOP4;
    patch_insn_write(pc as *mut core::ffi::c_void, nops.as_ptr() as *const _, 2 * MCOUNT_INSN_SIZE)
}

pub static mut ftrace_call_dest: ftrace_func_t = ftrace_stub;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    if is_enabled(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS) { return 0; }

    write_once(&raw mut ftrace_call_dest, func);
    /* The data fence ensure that the update to ftrace_call_dest happens before
     * the write to function_trace_op later in the generic ftrace. */
    smp_wmb();
    warn_on(irqs_disabled());
    smp_call_function(ftrace_sync_ipi, core::ptr::null_mut(), 1);
    0
}

#[cfg(not(CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_call_adjust(addr: usize) -> usize { addr }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS)]
pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, _old_addr: usize, _addr: usize) -> i32 {
    let caller = (*rec).ip - MCOUNT_AUIPC_SIZE;
    let ret = ftrace_rec_update_ops(rec);
    if ret != 0 { return ret; }
    __ftrace_modify_call(caller, FTRACE_ADDR, true)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
/* Most of this function is copied from arm64. */
pub unsafe fn prepare_ftrace_return(parent: *mut usize, self_addr: usize, frame_pointer: usize) {
    let return_hooker = return_to_handler as usize;
    if atomic_read(&(*current).tracing_graph_pause) != 0 { return; }
    let old = *parent;
    if !function_graph_enter(old, self_addr, frame_pointer, parent) { *parent = return_hooker; }
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe fn ftrace_graph_func(_ip: usize, ip: usize, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let return_hooker = return_to_handler as usize;
    let frame_pointer = arch_ftrace_regs(fregs).s0;
    let parent = &mut arch_ftrace_regs(fregs).ra as *mut usize;
    if atomic_read(&(*current).tracing_graph_pause) != 0 { return; }
    let old = *parent;
    if !function_graph_enter_regs(old, ip, frame_pointer, parent, fregs) { *parent = return_hooker; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
