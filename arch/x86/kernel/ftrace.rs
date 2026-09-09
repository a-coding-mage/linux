// SPDX-License-Identifier: GPL-2.0
/* Dynamic function tracing support. */

// Kernel headers and build-time configuration are supplied by the surrounding crate.

#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut ftrace_poke_late: i32 = 0;

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_arch_code_modify_prepare() {
    mutex_lock(&raw mut text_mutex);
    ftrace_poke_late = 1;
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_arch_code_modify_post_process() {
    smp_text_poke_batch_finish();
    ftrace_poke_late = 0;
    mutex_unlock(&raw mut text_mutex);
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_nop_replace() -> *const core::ffi::c_char { x86_nops[5] }

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_call_replace(ip: usize, mut addr: usize) -> *const core::ffi::c_char {
    if ftrace_is_jmp(addr) {
        addr = ftrace_jmp_get(addr);
        text_gen_insn(JMP32_INSN_OPCODE, ip as *mut _, addr as *mut _)
    } else {
        text_gen_insn(CALL_INSN_OPCODE, ip as *mut _, addr as *mut _)
    }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_verify_code(ip: usize, old_code: *const core::ffi::c_char) -> i32 {
    let mut cur_code = [0i8; MCOUNT_INSN_SIZE];
    if copy_from_kernel_nofault(cur_code.as_mut_ptr(), ip as *const _, MCOUNT_INSN_SIZE) != 0 {
        WARN_ON(1); return -EFAULT;
    }
    if memcmp(cur_code.as_ptr(), old_code, MCOUNT_INSN_SIZE) != 0 {
        ftrace_expected = old_code; WARN_ON(1); return -EINVAL;
    }
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_modify_code_direct(ip: usize, old_code: *const i8, new_code: *const i8) -> i32 {
    let ret = ftrace_verify_code(ip, old_code);
    if ret != 0 { return ret; }
    if ftrace_poke_late != 0 {
        smp_text_poke_batch_add(ip as *mut _, new_code, MCOUNT_INSN_SIZE, core::ptr::null_mut());
    } else {
        text_poke_early(ip as *mut _, new_code, MCOUNT_INSN_SIZE);
    }
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let ip = (*rec).ip;
    let old = ftrace_call_replace(ip, addr);
    let new = ftrace_nop_replace();
    if addr == MCOUNT_ADDR { return ftrace_modify_code_direct(ip, old, new); }
    WARN_ONCE(1, "invalid use of ftrace_make_nop"); -EINVAL
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let old = ftrace_nop_replace();
    let new = ftrace_call_replace((*rec).ip, addr);
    ftrace_modify_code_direct((*rec).ip, old, new)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_modify_call(_rec: *mut dyn_ftrace, _old_addr: usize, _addr: usize) -> i32 {
    WARN_ON(1); -EINVAL
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let mut ip = (&raw const ftrace_call) as usize;
    let mut new = ftrace_call_replace(ip, func as usize);
    smp_text_poke_single(ip as *mut _, new, MCOUNT_INSN_SIZE, core::ptr::null_mut());
    ip = (&raw const ftrace_regs_call) as usize;
    new = ftrace_call_replace(ip, func as usize);
    smp_text_poke_single(ip as *mut _, new, MCOUNT_INSN_SIZE, core::ptr::null_mut());
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn ftrace_replace_code(enable: i32) {
    let mut iter: *mut ftrace_rec_iter;
    let mut rec: *mut dyn_ftrace;
    let mut old: *const i8;
    let mut new: *const i8;
    let mut ret: i32;
    for_ftrace_rec_iter!(iter) {
        rec = ftrace_rec_iter_record(iter);
        match ftrace_test_record(rec, enable) {
            FTRACE_UPDATE_MAKE_CALL => old = ftrace_nop_replace(),
            FTRACE_UPDATE_MODIFY_CALL | FTRACE_UPDATE_MAKE_NOP => old = ftrace_call_replace((*rec).ip, ftrace_get_addr_curr(rec)),
            FTRACE_UPDATE_IGNORE => continue,
            _ => continue,
        }
        ret = ftrace_verify_code((*rec).ip, old);
        if ret != 0 { ftrace_expected = old; ftrace_bug(ret, rec); ftrace_expected = core::ptr::null(); return; }
    }
    for_ftrace_rec_iter!(iter) {
        rec = ftrace_rec_iter_record(iter);
        match ftrace_test_record(rec, enable) {
            FTRACE_UPDATE_MAKE_CALL | FTRACE_UPDATE_MODIFY_CALL => new = ftrace_call_replace((*rec).ip, ftrace_get_addr_new(rec)),
            FTRACE_UPDATE_MAKE_NOP => new = ftrace_nop_replace(),
            _ => continue,
        }
        smp_text_poke_batch_add((*rec).ip as *mut _, new, MCOUNT_INSN_SIZE, core::ptr::null_mut());
        ftrace_update_record(rec, enable);
    }
    smp_text_poke_batch_finish();
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn arch_ftrace_update_code(command: i32) { ftrace_modify_all_code(command); }

// The x86_64 dynamic trampoline implementation and function-graph support retain
// the same low-level operations as the C source; dependent kernel symbols are external.
#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
unsafe fn alloc_tramp(size: usize) -> *mut core::ffi::c_void { execmem_alloc_rw(EXECMEM_FTRACE, size) }
#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
unsafe fn tramp_free(tramp: *mut core::ffi::c_void) { execmem_free(tramp); }

#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
#[repr(C)]
union ftrace_op_code_union {
    code: [i8; OP_REF_SIZE],
    fields: ftrace_op_code_fields,
}
#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
#[repr(C, packed)]
struct ftrace_op_code_fields { op: [i8; 3], offset: i32 }

#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
unsafe fn calc_trampoline_call_offset(save_regs: bool) -> usize {
    let start = if save_regs { ftrace_regs_caller as usize } else { ftrace_caller as usize };
    let call = if save_regs { ftrace_regs_call as usize } else { ftrace_call as usize };
    call - start
}

// Remaining external trampoline entry points are declared by the architecture bindings.
#[cfg(all(CONFIG_DYNAMIC_FTRACE, CONFIG_X86_64))]
pub unsafe fn arch_ftrace_trampoline_free(ops: *mut ftrace_ops) {
    if ops.is_null() || (*ops).flags & FTRACE_OPS_FL_ALLOC_TRAMP == 0 { return; }
    tramp_free((*ops).trampoline as *mut _); (*ops).trampoline = 0;
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
unsafe fn skip_ftrace_return() -> bool {
    if (__builtin_frame_address(0) as isize) >= 0 { return true; }
    if ftrace_graph_is_dead() || atomic_read(&raw const (*current).tracing_graph_pause) != 0 { return true; }
    false
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn prepare_ftrace_return(ip: usize, parent: *mut usize, frame_pointer: usize) {
    let return_hooker = return_to_handler as usize;
    if skip_ftrace_return() { return; }
    if !function_graph_enter(*parent, ip, frame_pointer, parent) { *parent = return_hooker; }
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
unsafe fn ftrace_jmp_replace(ip: usize, addr: usize) -> *const i8 {
    text_gen_insn(JMP32_INSN_OPCODE, ip as *mut _, addr as *mut _)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
unsafe fn ftrace_mod_jmp(ip: usize, func: usize) -> i32 {
    let new = ftrace_jmp_replace(ip, func);
    smp_text_poke_single(ip as *mut _, new, MCOUNT_INSN_SIZE, core::ptr::null_mut()); 0
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 {
    ftrace_mod_jmp((&raw const ftrace_graph_call) as usize, ftrace_graph_caller as usize)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 {
    ftrace_mod_jmp((&raw const ftrace_graph_call) as usize, ftrace_stub as usize)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
#[cfg(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)]
pub unsafe fn ftrace_graph_func(ip: usize, _parent_ip: usize, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let regs = &mut arch_ftrace_regs(fregs).regs;
    let stack = kernel_stack_pointer(regs) as *mut usize;
    let parent = stack;
    let return_hooker = return_to_handler as usize;
    if skip_ftrace_return() { return; }
    if !function_graph_enter_regs(*parent, ip, 0, parent, fregs) { *parent = return_hooker; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
