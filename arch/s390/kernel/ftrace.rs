// SPDX-License-Identifier: GPL-2.0
/* Dynamic function tracer architecture backend. */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit; their symbols are intentionally left as external dependencies.

#[no_mangle]
pub static mut ftrace_func: *mut core::ffi::c_void = ftrace_stub as *mut core::ffi::c_void;

#[repr(C, packed)]
pub struct ftrace_insn {
    pub opc: u16,
    pub disp: i32,
}

unsafe fn ftrace_shared_hotpatch_trampoline(end: *mut *const i8) -> *const i8 {
    let mut tstart = ftrace_shared_hotpatch_trampoline_br;
    let mut tend = ftrace_shared_hotpatch_trampoline_br_end;
    // CONFIG_EXPOLINE condition is supplied by the kernel build.
    #[cfg(feature = "CONFIG_EXPOLINE")]
    if !nospec_disable {
        tstart = ftrace_shared_hotpatch_trampoline_exrl;
        tend = ftrace_shared_hotpatch_trampoline_exrl_end;
    }
    if !end.is_null() {
        *end = tend;
    }
    tstart
}

#[no_mangle]
pub unsafe fn ftrace_need_init_nop() -> bool {
    !cpu_has_seq_insn()
}

#[no_mangle]
pub unsafe fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> i32 {
    static mut next_vmlinux_trampoline: *mut ftrace_hotpatch_trampoline =
        __ftrace_hotpatch_trampolines_start;
    let orig = ftrace_insn { opc: 0xc004, disp: 0 };
    let mut trampoline: *mut ftrace_hotpatch_trampoline;
    let next_trampoline: *mut *mut ftrace_hotpatch_trampoline;
    let trampolines_end: *mut ftrace_hotpatch_trampoline;
    let mut tmp: ftrace_hotpatch_trampoline = core::mem::zeroed();
    let mut old: ftrace_insn = core::mem::zeroed();
    let shared: *const i8;
    let disp: i32;

    // BUILD_BUG_ON(sizeof(struct ftrace_hotpatch_trampoline) != SIZEOF_FTRACE_HOTPATCH_TRAMPOLINE)
    next_trampoline = &raw mut next_vmlinux_trampoline;
    trampolines_end = __ftrace_hotpatch_trampolines_end;
    shared = ftrace_shared_hotpatch_trampoline(core::ptr::null_mut());
    // CONFIG_MODULES condition is supplied by the kernel build.
    #[cfg(feature = "CONFIG_MODULES")]
    if !mod_.is_null() {
        next_trampoline = &raw mut (*mod_).arch.next_trampoline;
        trampolines_end = (*mod_).arch.trampolines_end;
    }
    if *next_trampoline >= trampolines_end { return -12; }
    trampoline = *next_trampoline;
    *next_trampoline = (*next_trampoline).add(1);
    if copy_from_kernel_nofault(&mut old as *mut _ as *mut core::ffi::c_void,
                                (*rec).ip as *const core::ffi::c_void,
                                core::mem::size_of::<ftrace_insn>()) != 0 { return -14; }
    if core::ptr::read_unaligned(&orig) != old { return -22; }

    (*trampoline).brasl_opc = 0xc015;
    (*trampoline).brasl_disp = ((shared as isize - (&raw const (*trampoline).brasl_opc as *const _ as isize)) / 2) as i32;
    (*trampoline).interceptor = FTRACE_ADDR;
    (*trampoline).rest_of_intercepted_function = (*rec).ip + core::mem::size_of::<ftrace_insn>() as u64;
    s390_kernel_write(trampoline as *mut core::ffi::c_void, &tmp as *const _ as *const core::ffi::c_void, core::mem::size_of_val(&tmp));
    disp = ((trampoline as isize - (*rec).ip as isize) / 2) as i32;
    let insn = (*rec).ip as *mut ftrace_insn;
    s390_kernel_write(&mut (*insn).disp as *mut _ as *mut core::ffi::c_void, &disp as *const _ as *const core::ffi::c_void, 4);
    0
}

unsafe fn ftrace_get_trampoline(rec: *mut dyn_ftrace) -> *mut ftrace_hotpatch_trampoline {
    let mut insn: ftrace_insn = core::mem::zeroed();
    let mut opc: u16 = 0;
    if copy_from_kernel_nofault(&mut insn as *mut _ as *mut _, (*rec).ip as *const _, 6) != 0 { return (-14isize) as *mut _; }
    let trampoline = ((*rec).ip as i64 + insn.disp as i64 * 2) as *mut ftrace_hotpatch_trampoline;
    if get_kernel_nofault(&mut opc, &raw const (*trampoline).brasl_opc) != 0 { return (-14isize) as *mut _; }
    if opc != 0xc015 { return (-22isize) as *mut _; }
    trampoline
}

#[inline]
unsafe fn ftrace_generate_branch_insn(ip: usize, target: usize) -> ftrace_insn {
    ftrace_insn { opc: if target != 0 { 0xc005 } else { 0xc004 }, disp: if target != 0 { ((target - ip) / 2) as i32 } else { 0 } }
}

unsafe fn ftrace_patch_branch_insn(ip: usize, old_target: usize, target: usize) -> i32 {
    let orig = ftrace_generate_branch_insn(ip, old_target);
    let new = ftrace_generate_branch_insn(ip, target);
    let mut old: ftrace_insn = core::mem::zeroed();
    if ip & 7 != 0 { return -22; }
    if copy_from_kernel_nofault(&mut old as *mut _ as *mut _, ip as *const _, 6) != 0 { return -14; }
    if old.opc != orig.opc || old.disp != orig.disp { return -22; }
    s390_kernel_write(ip as *mut _, &new as *const _ as *const _, 6); 0
}

unsafe fn ftrace_modify_trampoline_call(rec: *mut dyn_ftrace, old_addr: usize, addr: usize) -> i32 {
    let trampoline = ftrace_get_trampoline(rec);
    if (trampoline as isize) < 0 { return trampoline as isize as i32; }
    let mut old: u64 = 0;
    if get_kernel_nofault(&mut old, &raw const (*trampoline).interceptor) != 0 { return -14; }
    if old != old_addr as u64 { return -22; }
    s390_kernel_write(&mut (*trampoline).interceptor as *mut _ as *mut _, &addr as *const _ as *const _, 8); 0
}

pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, old_addr: usize, addr: usize) -> i32 { if cpu_has_seq_insn() { ftrace_patch_branch_insn((*rec).ip as usize, old_addr, addr) } else { ftrace_modify_trampoline_call(rec, old_addr, addr) } }

unsafe fn ftrace_patch_branch_mask(addr: *mut core::ffi::c_void, expected: u16, enable: bool) -> i32 {
    let mut old = 0u16; let op = if enable { 0xf4u8 } else { 0x04u8 };
    if get_kernel_nofault(&mut old, addr) != 0 { return -14; } if old != expected { return -22; }
    s390_kernel_write((addr as *mut u8).add(1) as *mut _, &op as *const _ as *const _, 1); 0
}

pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: usize) -> i32 { if cpu_has_seq_insn() { ftrace_patch_branch_insn((*rec).ip as usize, addr, 0) } else { ftrace_patch_branch_mask((*rec).ip as *mut _, 0xc0f4, false) } }
unsafe fn ftrace_make_trampoline_call(rec: *mut dyn_ftrace, addr: usize) -> i32 { let t=ftrace_get_trampoline(rec); if (t as isize)<0{return t as isize as i32;} s390_kernel_write(&mut (*t).interceptor as *mut _ as *mut _, &addr as *const _ as *const _, 8); ftrace_patch_branch_mask((*rec).ip as *mut _,0xc004,true) }
pub unsafe fn ftrace_make_call(rec:*mut dyn_ftrace,addr:usize)->i32{if cpu_has_seq_insn(){ftrace_patch_branch_insn((*rec).ip as usize,0,addr)}else{ftrace_make_trampoline_call(rec,addr)}}
pub unsafe fn ftrace_update_ftrace_func(func: *mut core::ffi::c_void)->i32{ftrace_func=func;0}
pub unsafe fn arch_ftrace_update_code(command:i32){ftrace_modify_all_code(command)}
pub unsafe fn ftrace_arch_code_modify_post_process(){text_poke_sync_lock()}

// CONFIG_FUNCTION_GRAPH_TRACER
#[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
pub unsafe fn ftrace_graph_func(ip: usize, _parent_ip: usize, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let regs = arch_ftrace_regs(fregs);
    let parent = &mut (*regs).regs.gprs[14];
    let sp = (*regs).regs.gprs[15];
    if ftrace_graph_is_dead() || atomic_read(&(*current).tracing_graph_pause) != 0 { return; }
    if function_graph_enter_regs(*parent, ip, 0, sp as *mut usize, fregs) == 0 {
        *parent = return_to_handler as usize;
    }
}

// CONFIG_KPROBES_ON_FTRACE
#[cfg(feature = "CONFIG_KPROBES_ON_FTRACE")]
pub unsafe fn kprobe_ftrace_handler(ip: usize, parent_ip: usize, _ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut kcb: *mut kprobe_ctlblk;
    let regs: *mut pt_regs;
    let p: *mut kprobe;
    let bit: i32;
    if kprobe_ftrace_disabled { return; }
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if bit < 0 { return; }
    kmsan_unpoison_memory(fregs as *mut _, ftrace_regs_size());
    regs = ftrace_get_regs(fregs);
    p = get_kprobe(ip as *mut kprobe_opcode_t);
    if regs.is_null() || p.is_null() || kprobe_disabled(p) { ftrace_test_recursion_unlock(bit); return; }
    if kprobe_running() { kprobes_inc_nmissed_count(p); ftrace_test_recursion_unlock(bit); return; }
    __this_cpu_write(current_kprobe, p);
    kcb = get_kprobe_ctlblk();
    (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
    instruction_pointer_set(regs, ip);
    if (*p).pre_handler.is_none() || (*p).pre_handler.unwrap()(p, regs) == 0 {
        instruction_pointer_set(regs, ip + MCOUNT_INSN_SIZE);
        if (*p).post_handler.is_some() {
            (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
            (*p).post_handler.unwrap()(p, regs, 0);
        }
    }
    __this_cpu_write(current_kprobe, core::ptr::null_mut());
    ftrace_test_recursion_unlock(bit);
}

#[cfg(feature = "CONFIG_KPROBES_ON_FTRACE")]
pub unsafe fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> i32 { (*p).ainsn.insn = core::ptr::null_mut(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
