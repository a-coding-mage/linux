/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive for
 * more details.
 *
 * Copyright (C) 2009 DSLab, Lanzhou University, China
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

/* Translated from the C header.  CONFIG_FUNCTION_TRACER controls this block. */
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
pub const MCOUNT_INSN_SIZE: usize = 4; /* sizeof mcount call */

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
unsafe extern "C" {
    pub fn _mcount();
    pub fn prepare_ftrace_return(
        parent_ra_addr: *mut usize,
        self_ra: usize,
        fp: usize,
    );
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
pub use _mcount as mcount;

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[inline(always)]
pub unsafe fn mcount_addr() -> usize {
    _mcount as usize
}

/*
 * The C safe_load/safe_store macros use MIPS inline assembly together with
 * .fixup and __ex_table entries.  These declarations preserve their low-level
 * interface and assembly intent for the target architecture.
 */
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_load {
    ($load:expr, $src:expr, $dst:expr, $error:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("1: ", $load, " %[tmp_dst], 0(%[tmp_src])\n",
                        "   li %[tmp_err], 0\n",
                        "2: .insn\n",
                        ".section .fixup, \"ax\"\n",
                        "3: li %[tmp_err], 1\n",
                        "   j 2b\n",
                        ".previous\n",
                        ".section __ex_table,\"a\"\n",
                        "PTR_WD\t1b, 3b\n",
                        ".previous\n"),
                tmp_dst = lateout(reg) $dst,
                tmp_err = lateout(reg) $error,
                tmp_src = in(reg) $src,
                options(nostack)
            );
        }
    }};
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_store {
    ($store:expr, $src:expr, $dst:expr, $error:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("1: ", $store, " %[tmp_src], 0(%[tmp_dst])\n",
                        "   li %[tmp_err], 0\n",
                        "2: .insn\n",
                        ".section .fixup, \"ax\"\n",
                        "3: li %[tmp_err], 1\n",
                        "   j 2b\n",
                        ".previous\n",
                        ".section __ex_table,\"a\"\n",
                        "PTR_WD\t1b, 3b\n",
                        ".previous\n"),
                tmp_err = lateout(reg) $error,
                tmp_dst = in(reg) $dst,
                tmp_src = in(reg) $src,
                options(nostack)
            );
        }
    }};
}

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_load_code {
    ($dst:expr, $src:expr, $error:expr) => { $crate::safe_load!("lw", $src, $dst, $error) };
}
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_store_code {
    ($src:expr, $dst:expr, $error:expr) => { $crate::safe_store!("sw", $src, $dst, $error) };
}
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_load_stack {
    ($dst:expr, $src:expr, $error:expr) => { $crate::safe_load!("PTR_L", $src, $dst, $error) };
}
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
#[macro_export]
macro_rules! safe_store_stack {
    ($src:expr, $dst:expr, $error:expr) => { $crate::safe_store!("PTR_S", $src, $dst, $error) };
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
#[inline(always)]
pub fn ftrace_call_adjust(addr: usize) -> usize { addr }

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
#[repr(C)]
pub struct DynArchFtrace;

/* CONFIG_FTRACE_SYSCALLS: ARCH_HAS_SYSCALL_MATCH_SYM_NAME */
#[cfg(feature = "CONFIG_FTRACE_SYSCALLS")]
pub unsafe fn arch_syscall_match_sym_name(sym: *const u8, name: *const u8) -> bool {
    unsafe extern "C" {
        fn strcmp(a: *const u8, b: *const u8) -> i32;
        fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32;
    }
    strcmp(sym, name) == 0
        || (strncmp(sym, b"__sys_\0".as_ptr(), 6) == 0
            && strcmp(sym.add(6), name.add(4)) == 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
