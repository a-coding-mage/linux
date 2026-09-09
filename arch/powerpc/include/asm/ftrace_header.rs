/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the PowerPC ftrace header. Build-time CONFIG_* conditions
 * from the C header are preserved as comments where they cannot be resolved
 * from this file alone. */

#[cfg(CONFIG_FUNCTION_TRACER)]
pub const MCOUNT_ADDR: usize = _mcount as usize;
#[cfg(CONFIG_FUNCTION_TRACER)]
pub const MCOUNT_INSN_SIZE: usize = 4;

/* CONFIG_MPROFILE_KERNEL || CONFIG_ARCH_USING_PATCHABLE_FUNCTION_ENTRY:
 * FTRACE_MCOUNT_MAX_OFFSET = 16.
 * CONFIG_PPC32: FTRACE_MCOUNT_MAX_OFFSET = 8. */

#[cfg(CONFIG_FUNCTION_TRACER)]
extern "C" {
    pub fn _mcount();
    pub fn prepare_ftrace_return(parent: usize, ip: usize, sp: usize) -> usize;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dyn_ftrace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dyn_arch_ftrace {
    #[cfg(CONFIG_PPC_FTRACE_OUT_OF_LINE)]
    pub ool_stub: usize,
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub const fn ftrace_need_init_nop() -> bool { true }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
extern "C" {
    pub fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> i32;
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn arch_ftrace_get_regs(fregs: *mut ftrace_regs) -> *mut pt_regs {
    let regs = arch_ftrace_regs(fregs);
    if (*regs).regs.msr != 0 { &mut (*regs).regs } else { core::ptr::null_mut() }
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn arch_ftrace_fill_perf_regs(fregs: *mut ftrace_regs, regs: *mut perf_regs) {
    (*regs).result = 0;
    (*regs).nip = arch_ftrace_regs(fregs).regs.nip;
    (*regs).gpr[1] = arch_ftrace_regs(fregs).regs.gpr[1];
    core::arch::asm!("mfmsr {0}", out(reg) (*regs).msr);
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_value(fregs: *const ftrace_regs) -> usize {
    arch_ftrace_regs(fregs as *mut ftrace_regs).regs.gpr[3]
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *const ftrace_regs) -> usize {
    arch_ftrace_regs(fregs as *mut ftrace_regs).regs.gpr[1]
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, ip: usize) {
    regs_set_return_ip(&mut arch_ftrace_regs(fregs).regs, ip);
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_address(fregs: *mut ftrace_regs) -> usize {
    arch_ftrace_regs(fregs).regs.link
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
extern "C" {
    pub fn ftrace_graph_func(ip: usize, parent_ip: usize, op: *mut ftrace_ops,
                              fregs: *mut ftrace_regs);
}

#[cfg(CONFIG_FTRACE_SYSCALLS)]
#[inline]
pub unsafe fn arch_syscall_match_sym_name(sym: *const core::ffi::c_char,
                                          name: *const core::ffi::c_char) -> bool {
    strcmp(sym, name) == 0
        || (strncmp(sym, b"__se_sys\0".as_ptr() as _, 8) == 0 && strcmp(sym.add(5), name) == 0)
        || (strncmp(sym, b"ppc_\0".as_ptr() as _, 4) == 0 && strcmp(sym.add(4), name.add(4)) == 0)
        || (strncmp(sym, b"ppc32_\0".as_ptr() as _, 6) == 0 && strcmp(sym.add(6), name.add(4)) == 0)
        || (strncmp(sym, b"ppc64_\0".as_ptr() as _, 6) == 0 && strcmp(sym.add(6), name.add(4)) == 0)
}

#[cfg(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER))]
#[inline]
pub unsafe fn this_cpu_disable_ftrace() { (*get_paca()).ftrace_enabled = 0; }
#[cfg(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER))]
#[inline]
pub unsafe fn this_cpu_enable_ftrace() { (*get_paca()).ftrace_enabled = 1; }
#[cfg(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER))]
#[inline]
pub unsafe fn this_cpu_set_ftrace_enabled(v: u8) { (*get_paca()).ftrace_enabled = v; }
#[cfg(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER))]
#[inline]
pub unsafe fn this_cpu_get_ftrace_enabled() -> u8 { (*get_paca()).ftrace_enabled }
#[cfg(not(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER)))]
#[inline]
pub fn this_cpu_disable_ftrace() {}
#[cfg(not(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER)))]
#[inline]
pub fn this_cpu_enable_ftrace() {}
#[cfg(not(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER)))]
#[inline]
pub fn this_cpu_set_ftrace_enabled(_: u8) {}
#[cfg(not(all(CONFIG_PPC64, CONFIG_FUNCTION_TRACER)))]
#[inline]
pub const fn this_cpu_get_ftrace_enabled() -> u8 { 1 }

#[cfg(CONFIG_FUNCTION_TRACER)]
extern "C" {
    pub static mut ftrace_tramp_text: [u32; 0];
    pub static mut ftrace_tramp_init: [u32; 0];
    pub fn ftrace_free_init_tramp();
    pub fn ftrace_call_adjust(addr: usize) -> usize;
}

#[cfg(all(CONFIG_FUNCTION_TRACER, CONFIG_DYNAMIC_FTRACE_WITH_REGS))]
pub const ARCH_SUPPORTS_FTRACE_OPS: i32 = 1;

#[cfg(all(CONFIG_FUNCTION_TRACER, CONFIG_PPC_FTRACE_OUT_OF_LINE))]
extern "C" {
    pub static mut ftrace_ool_stub_text_end: [ftrace_ool_stub; 0];
    pub static mut ftrace_ool_stub_text: [ftrace_ool_stub; 0];
    pub static mut ftrace_ool_stub_inittext: [ftrace_ool_stub; 0];
    pub static mut ftrace_ool_stub_text_end_count: u32;
    pub static mut ftrace_ool_stub_text_count: u32;
    pub static mut ftrace_ool_stub_inittext_count: u32;
}

#[repr(C, align(8))]
pub struct ftrace_ool_stub {
    #[cfg(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS)]
    pub ftrace_op: *mut ftrace_ops,
    pub insn: [u32; 4],
}

#[cfg(not(CONFIG_FUNCTION_TRACER))]
#[inline]
pub fn ftrace_free_init_tramp() {}
#[cfg(not(CONFIG_FUNCTION_TRACER))]
#[inline]
pub const fn ftrace_call_adjust(addr: usize) -> usize { addr }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS)]
#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: usize) {
    let regs = &mut arch_ftrace_regs(fregs).regs;
    regs.orig_gpr3 = addr;
}

/* External types and functions supplied by the surrounding kernel translation. */
extern "C" {
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn strncmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
