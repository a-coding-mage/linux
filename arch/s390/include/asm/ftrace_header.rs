/* SPDX-License-Identifier: GPL-2.0 */

pub const ARCH_SUPPORTS_FTRACE_OPS: i32 = 1;
pub const MCOUNT_INSN_SIZE: usize = 6;

// The following declarations depend on types and functions supplied by the
// surrounding kernel translation.

#[inline(always)]
pub unsafe fn return_address(n: core::ffi::c_uint) -> c_ulong {
    if n == 0 {
        return __builtin_return_address(0) as c_ulong;
    }

    let mut sf = current_frame_address() as *mut stack_frame;
    let mut remaining = n;
    loop {
        sf = (*sf).back_chain as *mut stack_frame;
        if sf.is_null() {
            return 0;
        }
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
    (*sf).gprs[8]
}

#[inline(always)]
pub unsafe fn ftrace_return_address(n: core::ffi::c_uint) -> c_ulong {
    return_address(n)
}

unsafe extern "C" {
    pub fn ftrace_caller();
    pub static mut ftrace_func: *mut core::ffi::c_void;
    pub fn ftrace_need_init_nop() -> bool;
    pub fn ftrace_init_nop(mod_: *mut module, rec: *mut dyn_ftrace) -> core::ffi::c_int;
    pub fn ftrace_graph_func(
        ip: c_ulong,
        parent_ip: c_ulong,
        op: *mut ftrace_ops,
        fregs: *mut ftrace_regs,
    );
}

#[repr(C)]
pub struct dyn_arch_ftrace {}

pub const MCOUNT_ADDR: usize = 0;
pub const FTRACE_ADDR: c_ulong = ftrace_caller as c_ulong;

pub const KPROBE_ON_FTRACE_NOP: i32 = 0;
pub const KPROBE_ON_FTRACE_CALL: i32 = 1;

#[repr(C)]
pub struct module;
#[repr(C)]
pub struct dyn_ftrace;
#[repr(C)]
pub struct ftrace_ops;
#[repr(C)]
pub struct stack_frame {
    pub back_chain: *mut core::ffi::c_void,
    pub gprs: [c_ulong; 16],
}

#[inline]
pub const fn ftrace_call_adjust(addr: c_ulong) -> c_ulong {
    addr
}

#[inline]
pub const fn ftrace_get_symaddr(fentry_ip: c_ulong) -> c_ulong {
    fentry_ip
}

#[inline(always)]
pub unsafe fn arch_ftrace_get_regs(fregs: *mut ftrace_regs) -> *mut pt_regs {
    let regs = &mut (*arch_ftrace_regs(fregs)).regs as *mut pt_regs;
    if test_pt_regs_flag(regs, PIF_FTRACE_FULL_REGS) {
        regs
    } else {
        core::ptr::null_mut()
    }
}

#[inline(always)]
pub unsafe fn ftrace_regs_set_instruction_pointer(fregs: *mut ftrace_regs, ip: c_ulong) {
    (*arch_ftrace_regs(fregs)).regs.psw.addr = ip;
}

#[inline(always)]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *mut ftrace_regs) -> c_ulong {
    ftrace_regs_get_stack_pointer(fregs)
}

#[inline(always)]
pub unsafe fn ftrace_regs_get_return_address(fregs: *const ftrace_regs) -> c_ulong {
    (*arch_ftrace_regs(fregs as *mut ftrace_regs)).regs.gprs[14]
}

#[inline(always)]
pub unsafe fn arch_ftrace_fill_perf_regs(fregs: *mut ftrace_regs, regs: *mut pt_regs) {
    (*regs).psw.mask = 0;
    (*regs).psw.addr = (*arch_ftrace_regs(fregs)).regs.psw.addr;
    (*regs).gprs[15] = (*arch_ftrace_regs(fregs)).regs.gprs[15];
}

// CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS
// When enabled, arch_ftrace_set_direct_caller stores the direct caller in
// the ORIG_GPR2 part of pt_regs so ftrace_caller can distinguish it.
#[inline]
pub unsafe fn arch_ftrace_set_direct_caller(fregs: *mut ftrace_regs, addr: c_ulong) {
    (*arch_ftrace_regs(fregs)).regs.orig_gpr2 = addr;
}

pub const ARCH_HAS_SYSCALL_MATCH_SYM_NAME: bool = true;

#[inline]
pub unsafe fn arch_syscall_match_sym_name(
    sym: *const core::ffi::c_char,
    name: *const core::ffi::c_char,
) -> bool {
    // Skip the __s390x_ prefix.
    strcmp(sym.add(7), name) == 0 || strcmp(sym.add(8), name) == 0
}

// CONFIG_FUNCTION_TRACER assembly macros:
// FTRACE_NOP_INSN is: .word 0xc004, 0x0000, 0x0000 /* brcl 0,0 */
// FTRACE_GEN_MCOUNT_RECORD(name) emits the __mcount_loc section record
// unless CC_USING_HOTPATCH is enabled.
// FTRACE_GEN_NOP_ASM(name) emits FTRACE_GEN_MCOUNT_RECORD(name) and
// FTRACE_NOP_INSN when CONFIG_FUNCTION_TRACER is enabled.

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" {
    fn __builtin_return_address(level: core::ffi::c_uint) -> *mut core::ffi::c_void;
    fn current_frame_address() -> *mut core::ffi::c_void;
    fn arch_ftrace_regs(fregs: *mut ftrace_regs) -> *mut arch_ftrace_regs_type;
    fn test_pt_regs_flag(regs: *mut pt_regs, flag: core::ffi::c_int) -> bool;
    fn ftrace_regs_get_stack_pointer(fregs: *mut ftrace_regs) -> c_ulong;
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> core::ffi::c_int;
}

#[repr(C)]
pub struct ftrace_regs;
#[repr(C)]
pub struct pt_regs {
    pub psw: psw_t,
    pub gprs: [c_ulong; 16],
    pub orig_gpr2: c_ulong,
}
#[repr(C)]
pub struct arch_ftrace_regs_type {
    pub regs: pt_regs,
}
#[repr(C)]
pub struct psw_t {
    pub mask: c_ulong,
    pub addr: c_ulong,
}

pub const PIF_FTRACE_FULL_REGS: core::ffi::c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
