/* SPDX-License-Identifier: GPL-2.0 */

// The declarations in this file correspond to the non-assembler portion of
// the original C header.

unsafe extern "C" {
    pub fn mcount();
}

// #define MCOUNT_ADDR ((unsigned long)mcount)
pub static MCOUNT_ADDR: usize = mcount as usize;
pub const MCOUNT_INSN_SIZE: usize = 4;

// #define CC_USING_NOP_MCOUNT
// #define ARCH_SUPPORTS_FTRACE_OPS 1
pub const ARCH_SUPPORTS_FTRACE_OPS: usize = 1;

pub static mut sys_call_table: [usize; 0] = [];

pub unsafe extern "C" fn return_address(n: u32) -> usize;

#[repr(C)]
pub struct ftrace_regs {
    _private: [u8; 0],
}

pub unsafe extern "C" fn ftrace_function_trampoline(
    parent: usize,
    self_addr: usize,
    org_sp_gr3: usize,
    fregs: *mut ftrace_regs,
);

// #ifdef CONFIG_DYNAMIC_FTRACE
#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe extern "C" {
    pub fn ftrace_caller();
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[repr(C)]
pub struct dyn_arch_ftrace {}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe extern "C" fn ftrace_call_adjust(addr: usize) -> usize;
// #endif

// #define ftrace_return_address(n) return_address(n)
#[inline]
pub unsafe fn ftrace_return_address(n: u32) -> usize {
    unsafe { return_address(n) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
