// SPDX-License-Identifier: GPL-2.0
/*
 * Based on arch/arm64/kernel/ftrace.c
 *
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the kernel headers and architecture support.

unsafe fn ftrace_modify_code(pc: usize, old: u32, new: u32, validate: bool) -> i32 {
    let mut replaced: u32 = 0;

    if validate {
        if larch_insn_read(pc as *mut core::ffi::c_void, &mut replaced) != 0 {
            return -EFAULT;
        }
        if replaced != old {
            return -EINVAL;
        }
    }

    if larch_insn_patch_text(pc as *mut core::ffi::c_void, new) != 0 {
        return -EPERM;
    }
    0
}

#[cfg(CONFIG_MODULES)]
unsafe fn reachable_by_bl(addr: usize, pc: usize) -> bool {
    let offset = (addr as isize).wrapping_sub(pc as isize);
    offset >= -(SZ_128M as isize) && offset < SZ_128M as isize
}

#[cfg(CONFIG_MODULES)]
unsafe fn get_ftrace_plt(mod_: *mut module, addr: usize) -> *mut plt_entry {
    let plt = (*mod_).arch.ftrace_trampolines;
    if addr == FTRACE_ADDR {
        return plt.add(FTRACE_PLT_IDX);
    }
    if addr == FTRACE_REGS_ADDR && cfg!(CONFIG_DYNAMIC_FTRACE_WITH_REGS) {
        return plt.add(FTRACE_REGS_PLT_IDX);
    }
    core::ptr::null_mut()
}

#[cfg(CONFIG_MODULES)]
unsafe fn ftrace_find_callable_addr(rec: *mut dyn_ftrace, mut mod_: *mut module, addr: *mut usize) -> bool {
    let pc = (*rec).ip + LOONGARCH_INSN_SIZE;
    let mut plt: *mut plt_entry;

    if *addr != FTRACE_ADDR && *addr != FTRACE_REGS_ADDR && !reachable_by_bl(*addr, pc) {
        *addr = FTRACE_REGS_ADDR;
    }
    if reachable_by_bl(*addr, pc) {
        return true;
    }
    if mod_.is_null() {
        // scoped_guard(rcu): the lookup is protected by the surrounding ftrace lock.
        mod_ = __module_text_address(pc);
    }
    if WARN_ON(mod_.is_null()) {
        return false;
    }
    plt = get_ftrace_plt(mod_, *addr);
    if plt.is_null() {
        pr_err("ftrace: no module PLT for %ps\n", *addr as *const core::ffi::c_void);
        return false;
    }
    *addr = plt as usize;
    true
}

#[cfg(not(CONFIG_MODULES))]
unsafe fn ftrace_find_callable_addr(_rec: *mut dyn_ftrace, _mod_: *mut module, _addr: *mut usize) -> bool { true }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_REGS)]
pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, mut old_addr: usize, mut addr: usize) -> i32 {
    let pc = (*rec).ip + LOONGARCH_INSN_SIZE;
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut addr) { return -EINVAL; }
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut old_addr) { return -EINVAL; }
    let new = larch_insn_gen_bl(pc, addr);
    let old = larch_insn_gen_bl(pc, old_addr);
    ftrace_modify_code(pc, old, new, true)
}

pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let pc = &ftrace_call as *const _ as usize;
    let new = larch_insn_gen_bl(pc, func as usize);
    ftrace_modify_code(pc, 0, new, false)
}

pub unsafe fn ftrace_init_nop(_mod_: *mut module, rec: *mut dyn_ftrace) -> i32 {
    let pc = (*rec).ip;
    let old = larch_insn_gen_nop();
    let new = larch_insn_gen_move(LOONGARCH_GPR_T0, LOONGARCH_GPR_RA);
    ftrace_modify_code(pc, old, new, true)
}

pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, mut addr: usize) -> i32 {
    let pc = (*rec).ip + LOONGARCH_INSN_SIZE;
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut addr) { return -EINVAL; }
    ftrace_modify_code(pc, larch_insn_gen_nop(), larch_insn_gen_bl(pc, addr), true)
}

pub unsafe fn ftrace_make_nop(_mod_: *mut module, rec: *mut dyn_ftrace, mut addr: usize) -> i32 {
    let pc = (*rec).ip + LOONGARCH_INSN_SIZE;
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut addr) { return -EINVAL; }
    ftrace_modify_code(pc, larch_insn_gen_bl(pc, addr), larch_insn_gen_nop(), true)
}

pub unsafe fn arch_ftrace_update_code(mut command: i32) {
    command |= FTRACE_MAY_SLEEP;
    ftrace_modify_all_code(command);
}

pub unsafe fn ftrace_dyn_arch_init() -> i32 { 0 }

// The compiler inserts two NOPs before the regular function prologue. At runtime
// the second NOP is replaced with a branch to enable the ftrace call.

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn prepare_ftrace_return(self_addr: usize, parent: *mut usize) {
    let return_hooker = return_to_handler as usize;
    if unlikely(atomic_read(&(*current).tracing_graph_pause) != 0) { return; }
    let old = *parent;
    if !function_graph_enter(old, self_addr, 0, parent) { *parent = return_hooker; }
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS))]
pub unsafe fn ftrace_graph_func(ip: usize, _parent_ip: usize, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let regs = &mut arch_ftrace_regs(fregs).regs;
    let parent = &mut regs.regs[1] as *mut _ as *mut usize;
    let return_hooker = return_to_handler as usize;
    if unlikely(atomic_read(&(*current).tracing_graph_pause) != 0) { return; }
    let old = *parent;
    if !function_graph_enter_regs(old, ip, 0, parent, fregs) { *parent = return_hooker; }
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
unsafe fn ftrace_modify_graph_caller(enable: bool) -> i32 {
    let pc = &ftrace_graph_call as *const _ as usize;
    let func = ftrace_graph_caller as usize;
    let nop = larch_insn_gen_nop();
    let branch = larch_insn_gen_b(pc, func);
    if enable { ftrace_modify_code(pc, nop, branch, true) } else { ftrace_modify_code(pc, branch, nop, true) }
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 { ftrace_modify_graph_caller(true) }
#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, not(CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS)))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 { ftrace_modify_graph_caller(false) }

#[cfg(CONFIG_KPROBES_ON_FTRACE)]
pub unsafe fn kprobe_ftrace_handler(ip: usize, parent_ip: usize, _ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    if unlikely(kprobe_ftrace_disabled) { return; }
    let bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if bit < 0 { return; }
    let p = get_kprobe(ip as *mut kprobe_opcode_t);
    if p.is_null() || kprobe_disabled(p) { ftrace_test_recursion_unlock(bit); return; }
    let regs = ftrace_get_regs(fregs);
    if regs.is_null() { ftrace_test_recursion_unlock(bit); return; }
    let kcb = get_kprobe_ctlblk();
    if kprobe_running() {
        kprobes_inc_nmissed_count(p);
    } else {
        let orig_ip = instruction_pointer(regs);
        instruction_pointer_set(regs, ip);
        this_cpu_write(current_kprobe, p);
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        if (*p).pre_handler.is_none() || !(*p).pre_handler.unwrap()(p, regs) {
            instruction_pointer_set(regs, (*p).addr as usize + MCOUNT_INSN_SIZE);
            if let Some(post) = (*p).post_handler {
                (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
                post(p, regs, 0);
            }
            instruction_pointer_set(regs, orig_ip);
        }
        this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    ftrace_test_recursion_unlock(bit);
}

#[cfg(CONFIG_KPROBES_ON_FTRACE)]
pub unsafe fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> i32 {
    (*p).ainsn.insn = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
