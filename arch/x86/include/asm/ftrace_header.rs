/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/ptrace.h
// The following items are enabled by CONFIG_FUNCTION_TRACER.
// CC_USING_FENTRY is required by the original header.

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
pub const MCOUNT_ADDR: usize = __fentry__ as usize;
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
pub const MCOUNT_INSN_SIZE: usize = 5; /* sizeof mcount call */

// CONFIG_HAVE_FENTRY: include asm/ibt.h and add ENDBR_INSN_SIZE for IBT.
#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_HAVE_FENTRY"))]
pub const FTRACE_MCOUNT_MAX_OFFSET: usize = ENDBR_INSN_SIZE;

// CONFIG_DYNAMIC_FTRACE
#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE"))]
pub const ARCH_SUPPORTS_FTRACE_OPS: usize = 1;

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
extern "C" {
    pub fn __fentry__();
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[inline]
pub unsafe fn ftrace_call_adjust(addr: usize) -> usize {
    /*
     * addr is the address of the mcount call instruction.
     * recordmcount does the necessary offset calculation.
     */
    addr
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[inline]
pub unsafe fn arch_ftrace_get_symaddr(mut fentry_ip: usize) -> usize {
    if is_endbr((fentry_ip - ENDBR_INSN_SIZE) as *const core::ffi::c_void) {
        fentry_ip -= ENDBR_INSN_SIZE;
    }
    fentry_ip
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[inline]
pub unsafe fn ftrace_get_symaddr(fentry_ip: usize) -> usize {
    arch_ftrace_get_symaddr(fentry_ip)
}

// CONFIG_DYNAMIC_FTRACE_WITH_ARGS: dependency linux/ftrace_regs.h
#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
#[inline(always)]
pub unsafe fn arch_ftrace_get_regs(
    fregs: *mut ftrace_regs,
) -> *mut pt_regs {
    /* Only when FL_SAVE_REGS is set, cs will be non zero */
    if (*arch_ftrace_regs(fregs)).regs.cs == 0 {
        core::ptr::null_mut()
    } else {
        &mut (*arch_ftrace_regs(fregs)).regs
    }
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
#[inline]
pub unsafe fn arch_ftrace_partial_regs(regs: *mut pt_regs) {
    (*regs).flags |= X86_EFLAGS_FIXED;
    (*regs).cs = __KERNEL_CS;
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
#[inline]
pub unsafe fn arch_ftrace_fill_perf_regs(fregs: *mut ftrace_regs, regs: *mut pt_regs) {
    (*regs).ip = (*arch_ftrace_regs(fregs)).regs.ip;
    (*regs).sp = (*arch_ftrace_regs(fregs)).regs.sp;
    (*regs).cs = __KERNEL_CS;
    (*regs).flags = 0;
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
#[inline]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, ip: usize) {
    (*arch_ftrace_regs(fregs)).regs.ip = ip;
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_address(fregs: *mut ftrace_regs) -> usize {
    *(ftrace_regs_get_stack_pointer(fregs) as *const usize)
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS"))]
extern "C" {
    pub fn ftrace_graph_func(
        ip: usize,
        parent_ip: usize,
        op: *mut ftrace_ops,
        fregs: *mut ftrace_regs,
    );
}

// Without CONFIG_DYNAMIC_FTRACE_WITH_ARGS:
// pub const FTRACE_GRAPH_TRAMP_ADDR: usize = FTRACE_GRAPH_ADDR;

// CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS
#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS"))]
#[inline]
pub unsafe fn __arch_ftrace_set_direct_caller(regs: *mut pt_regs, addr: usize) {
    /* Emulate a call */
    (*regs).orig_ax = addr;
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS"))]
#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: usize) {
    __arch_ftrace_set_direct_caller(&mut (*arch_ftrace_regs(fregs)).regs, addr);
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE"))]
#[repr(C)]
pub struct dyn_arch_ftrace {
    /* No extra data needed for x86 */
}

extern "C" {
    pub fn prepare_ftrace_return(ip: usize, parent: *mut usize, frame_pointer: usize);
}

#[cfg(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE"))]
extern "C" {
    pub fn set_ftrace_ops_ro();
}

#[cfg(not(all(feature = "CONFIG_FUNCTION_TRACER", feature = "CONFIG_DYNAMIC_FTRACE")))]
#[inline]
pub fn set_ftrace_ops_ro() {}

// ARCH_HAS_SYSCALL_MATCH_SYM_NAME
#[inline]
pub unsafe fn arch_syscall_match_sym_name(sym: *const u8, name: *const u8) -> bool {
    /*
     * Compare the symbol name with the system call name. Skip the
     * "__x64_sys", "__ia32_sys", "__do_sys" or simple "sys" prefix.
     */
    !strcmp(sym.add(3), name.add(3))
        || (!strncmp(sym, b"__x64_\0".as_ptr(), 6) && !strcmp(sym.add(9), name.add(3)))
        || (!strncmp(sym, b"__ia32_\0".as_ptr(), 7) && !strcmp(sym.add(10), name.add(3)))
        || (!strncmp(sym, b"__do_sys\0".as_ptr(), 8) && !strcmp(sym.add(8), name.add(3)))
}

// !COMPILE_OFFSETS && CONFIG_FTRACE_SYSCALLS && CONFIG_IA32_EMULATION:
// ARCH_TRACE_IGNORE_COMPAT_SYSCALLS 1
#[cfg(all(
    not(feature = "COMPILE_OFFSETS"),
    feature = "CONFIG_FTRACE_SYSCALLS",
    feature = "CONFIG_IA32_EMULATION"
))]
#[inline]
pub unsafe fn arch_trace_is_compat_syscall(_regs: *mut pt_regs) -> bool {
    in_32bit_syscall()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
