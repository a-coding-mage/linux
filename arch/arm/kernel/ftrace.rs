/*
 * Dynamic function tracing support.
 *
 * Copyright (C) 2008 Abhishek Sagar <sagar.abhishek@gmail.com>
 * Copyright (C) 2010 Rabin Vincent <rabin@rab.in>
 *
 * For licencing details, see COPYING.
 *
 * Defines low-level handling of mcount calls when the kernel
 * is compiled with the -pg flag. When using dynamic ftrace, the
 * mcount call-sites get patched with NOP till they are enabled.
 * All code mutation routines here are called under stop_machine().
 */

// C header dependencies are supplied by the surrounding kernel translation.

/*
 * The compiler emitted profiling hook consists of
 *
 *   PUSH    {LR}
 *   BL      __gnu_mcount_nc
 *
 * To turn this combined sequence into a NOP, we need to restore the value of
 * SP before the PUSH. Let's use an ADD rather than a POP into LR, as LR is not
 * modified anyway, and reloading LR from memory is highly likely to be less
 * efficient.
 */
#[cfg(feature = "thumb2_kernel")]
const NOP: ::core::ffi::c_ulong = 0xf10d0d04; /* add.w sp, sp, #4 */
#[cfg(not(feature = "thumb2_kernel"))]
const NOP: ::core::ffi::c_ulong = 0xe28dd004; /* add   sp, sp, #4 */

#[cfg(feature = "dynamic_ftrace")]
unsafe fn __ftrace_modify_code(data: *mut ::core::ffi::c_void) -> i32 {
    let command = data as *mut i32;
    ftrace_modify_all_code(*command);
    0
}

#[cfg(feature = "dynamic_ftrace")]
pub unsafe fn arch_ftrace_update_code(command: i32) {
    stop_machine(Some(__ftrace_modify_code), &command as *const _ as *mut _, core::ptr::null_mut());
}

#[cfg(feature = "dynamic_ftrace")]
unsafe fn ftrace_nop_replace(_rec: *mut dyn_ftrace) -> ::core::ffi::c_ulong { NOP }

unsafe extern "C" {
    fn ftrace_caller_from_init();
    fn ftrace_regs_caller_from_init();
}

#[cfg(feature = "dynamic_ftrace")]
unsafe fn adjust_address(rec: *mut dyn_ftrace, addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if !is_enabled_dynamic_ftrace() || system_state >= SYSTEM_FREEING_INITMEM || is_kernel_inittext((*rec).ip) {
        return addr;
    }
    if !is_enabled_dynamic_ftrace_with_regs() || addr == (&ftrace_caller as *const _ as usize as u64) {
        &ftrace_caller_from_init as *const _ as usize as ::core::ffi::c_ulong
    } else {
        &ftrace_regs_caller_from_init as *const _ as usize as ::core::ffi::c_ulong
    }
}

pub unsafe fn ftrace_arch_code_modify_prepare() {}

pub unsafe fn ftrace_arch_code_modify_post_process() {
    /* Make sure any TLB misses during machine stop are cleared. */
    flush_tlb_all();
}

unsafe fn ftrace_call_replace(pc: ::core::ffi::c_ulong, addr: ::core::ffi::c_ulong, warn: bool) -> ::core::ffi::c_ulong {
    arm_gen_branch_link(pc, addr, warn)
}

unsafe fn ftrace_modify_code(pc: ::core::ffi::c_ulong, mut old: ::core::ffi::c_ulong, new: ::core::ffi::c_ulong, validate: bool) -> i32 {
    old = if cfg!(feature = "thumb2_kernel") { __opcode_to_mem_thumb32(old) } else { __opcode_to_mem_arm(old) };
    if validate {
        let mut replaced: ::core::ffi::c_ulong = 0;
        if copy_from_kernel_nofault(&mut replaced as *mut _, pc as *const _, MCOUNT_INSN_SIZE) != 0 { return -EFAULT; }
        if replaced != old { return -EINVAL; }
    }
    __patch_text(pc as *mut _, new);
    0
}

pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let mut pc = &ftrace_call as *const _ as usize as ::core::ffi::c_ulong;
    let mut new = ftrace_call_replace(pc, func as usize as _, true);
    let mut ret = ftrace_modify_code(pc, 0, new, false);
    #[cfg(feature = "dynamic_ftrace_with_regs")]
    if ret == 0 {
        pc = &ftrace_regs_call as *const _ as usize as _;
        new = ftrace_call_replace(pc, func as usize as _, true);
        ret = ftrace_modify_code(pc, 0, new, false);
    }
    ret
}

pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: ::core::ffi::c_ulong) -> i32 {
    let ip = (*rec).ip;
    let aaddr = adjust_address(rec, addr);
    let old = ftrace_nop_replace(rec);
    let new = ftrace_call_replace(ip, aaddr, true);
    ftrace_modify_code((*rec).ip, old, new, true)
}

#[cfg(feature = "dynamic_ftrace_with_regs")]
pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, old_addr: ::core::ffi::c_ulong, addr: ::core::ffi::c_ulong) -> i32 {
    let ip = (*rec).ip;
    let old = ftrace_call_replace(ip, adjust_address(rec, old_addr), true);
    let new = ftrace_call_replace(ip, adjust_address(rec, addr), true);
    ftrace_modify_code((*rec).ip, old, new, true)
}

pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: ::core::ffi::c_ulong) -> i32 {
    let ip = (*rec).ip;
    let old = ftrace_call_replace(ip, adjust_address(rec, addr), true);
    let new = ftrace_nop_replace(rec);
    /* Locations in .init.text may call __gnu_mcount_mc via a linker emitted
     * veneer; omit those from validation. */
    ftrace_modify_code(ip, old, new, !is_kernel_inittext(ip))
}

#[cfg(feature = "function_graph_tracer")]
pub unsafe extern "C" fn prepare_ftrace_return(parent: *mut *mut ::core::ffi::c_ulong, self_addr: ::core::ffi::c_ulong, mut frame_pointer: ::core::ffi::c_ulong, stack_pointer: ::core::ffi::c_ulong) {
    let return_hooker = &return_to_handler as *const _ as usize as ::core::ffi::c_ulong;
    if atomic_read(&current.tracing_graph_pause) != 0 { return; }
    if cfg!(feature = "unwinder_frame_pointer") {
        __get_kernel_nofault(&mut frame_pointer, (frame_pointer - 8) as *mut _, core::mem::size_of::<::core::ffi::c_ulong>());
    } else {
        let mut frame = stackframe { fp: frame_pointer, sp: stack_pointer, lr: self_addr, pc: self_addr };
        if unwind_frame(&mut frame) < 0 { return; }
        if frame.lr != self_addr { parent = frame.lr_addr; }
        frame_pointer = frame.sp;
    }
    let old = *parent;
    *parent = return_hooker as *mut _;
    if function_graph_enter(old as _, self_addr, frame_pointer, core::ptr::null_mut()) != 0 { *parent = old; }
}

#[cfg(all(feature = "function_graph_tracer", feature = "dynamic_ftrace"))]
unsafe fn __ftrace_modify_caller(callsite: *mut ::core::ffi::c_ulong, func: unsafe extern "C" fn(), enable: bool) -> i32 {
    let pc = callsite as usize as ::core::ffi::c_ulong;
    let branch = arm_gen_branch(pc, func as usize as _);
    let nop = arm_gen_nop();
    ftrace_modify_code(pc, if enable { nop } else { branch }, if enable { branch } else { nop }, true)
}

#[cfg(all(feature = "function_graph_tracer", feature = "dynamic_ftrace"))]
unsafe fn ftrace_modify_graph_caller(enable: bool) -> i32 {
    __ftrace_modify_caller(&mut ftrace_graph_call, ftrace_graph_caller, enable)
}

#[cfg(all(feature = "function_graph_tracer", feature = "dynamic_ftrace"))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 { ftrace_modify_graph_caller(true) }

#[cfg(all(feature = "function_graph_tracer", feature = "dynamic_ftrace"))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 { ftrace_modify_graph_caller(false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
