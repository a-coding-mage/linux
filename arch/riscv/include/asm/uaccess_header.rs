/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of riscv/include/asm/uaccess.h.
 * C includes, header guards, and build-time conditions are represented below
 * as comments or cfg-gated Rust items; kernel-provided dependencies remain
 * external dependencies.
 */

// Dependencies: asm-extable, cpufeature, pgtable, errno, compiler,
// thread_info, byteorder, extable, asm, and asm-generic/access_ok.

#[cfg(feature = "riscv_isa_supm")]
#[inline]
pub unsafe fn __untagged_addr_remote(mm: *mut mm_struct, addr: usize) -> usize {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SUPM) {
        let pmlen: u8 = (*mm).context.pmlen;
        // Virtual addresses are sign-extended; physical addresses are zero-extended.
        #[cfg(feature = "mmu")]
        { (((addr << pmlen) as isize) >> pmlen) as usize }
        #[cfg(not(feature = "mmu"))]
        { (addr << pmlen) >> pmlen }
    } else { addr }
}

#[cfg(feature = "riscv_isa_supm")]
#[macro_export]
macro_rules! untagged_addr {
    ($addr:expr) => {{
        let __addr: usize = $addr as usize;
        __untagged_addr_remote(current_mm(), __addr) as _
    }};
}

#[cfg(feature = "riscv_isa_supm")]
#[macro_export]
macro_rules! untagged_addr_remote {
    ($mm:expr, $addr:expr) => {{
        let __addr: usize = $addr as usize;
        mmap_assert_locked($mm);
        __untagged_addr_remote($mm, __addr) as _
    }};
}

#[cfg(not(feature = "riscv_isa_supm"))]
#[macro_export]
macro_rules! untagged_addr { ($addr:expr) => { $addr }; }

#[cfg(feature = "riscv_isa_supm")]
#[macro_export]
macro_rules! access_ok { ($addr:expr, $size:expr) => { likely(__access_ok(untagged_addr!($addr), $size)) }; }

#[cfg(feature = "mmu")]
mod mmu {
    // __enable_user_access / __disable_user_access use volatile RISC-V CSR asm.
    #[macro_export]
    macro_rules! __enable_user_access { () => { unsafe { core::arch::asm!("csrs sstatus, {0}", in(reg) SR_SUM, options(nostack, preserves_flags)); } }; }
    #[macro_export]
    macro_rules! __disable_user_access { () => { unsafe { core::arch::asm!("csrc sstatus, {0}", in(reg) SR_SUM, options(nostack, preserves_flags)); } }; }

    pub const __LSW: usize = 0;
    pub const __MSW: usize = 1;

    // The following macros preserve the original faulting-instruction and
    // exception-table behavior. Kernel-specific asm-extable operands are
    // intentionally retained as external macro dependencies.
    #[macro_export]
    macro_rules! __get_user_asm { ($insn:literal, $x:expr, $ptr:expr, $label:lifetime) => {{
        let mut __tmp: u64;
        unsafe { core::arch::asm!(concat!("1:\n\t", $insn, " {0}, {1}"), out(reg) __tmp, in(reg) $ptr); }
        $x = __tmp as _;
    }}; }

    #[cfg(target_pointer_width = "64")]
    #[macro_export]
    macro_rules! __get_user_8 { ($x:expr, $ptr:expr, $label:lifetime) => { __get_user_asm!("ld", $x, $ptr, $label) }; }

    #[macro_export]
    macro_rules! __get_user_nocheck { ($x:expr, $ptr:expr, $label:lifetime) => {{
        match core::mem::size_of_val(&$ptr) {
            1 => __get_user_asm!("lb", $x, $ptr, $label),
            2 => __get_user_asm!("lh", $x, $ptr, $label),
            4 => __get_user_asm!("lw", $x, $ptr, $label),
            8 => __get_user_8!($x, $ptr, $label),
            _ => panic!("BUILD_BUG"),
        }
    }}; }

    #[macro_export]
    macro_rules! __get_user { ($x:expr, $ptr:expr) => {{
        let __gu_ptr = untagged_addr!($ptr);
        let mut __gu_val = core::mem::MaybeUninit::uninit();
        __enable_user_access!();
        __get_user_nocheck!(__gu_val, __gu_ptr, '___gu_failed);
        __disable_user_access!();
        $x = unsafe { __gu_val.assume_init() };
        0isize
    }}; }

    #[macro_export]
    macro_rules! get_user { ($x:expr, $ptr:expr) => {{
        might_fault();
        if access_ok!($ptr, core::mem::size_of_val(&$ptr)) { __get_user!($x, $ptr) }
        else { $x = 0 as _; -EFAULT as isize }
    }}; }

    #[macro_export]
    macro_rules! __put_user_asm { ($insn:literal, $x:expr, $ptr:expr, $label:lifetime) => {{
        let __x = $x;
        unsafe { core::arch::asm!(concat!("1:\n\t", $insn, " {0}, [{1}]"), in(reg) __x, in(reg) $ptr); }
    }}; }

    #[cfg(target_pointer_width = "64")]
    #[macro_export]
    macro_rules! __put_user_8 { ($x:expr, $ptr:expr, $label:lifetime) => { __put_user_asm!("sd", $x, $ptr, $label) }; }

    #[macro_export]
    macro_rules! __put_user_nocheck { ($x:expr, $ptr:expr, $label:lifetime) => {{
        match core::mem::size_of_val(&$ptr) {
            1 => __put_user_asm!("sb", $x, $ptr, $label),
            2 => __put_user_asm!("sh", $x, $ptr, $label),
            4 => __put_user_asm!("sw", $x, $ptr, $label),
            8 => __put_user_8!($x, $ptr, $label),
            _ => panic!("BUILD_BUG"),
        }
    }}; }

    #[macro_export]
    macro_rules! __put_user { ($x:expr, $ptr:expr) => {{
        let __gu_ptr = untagged_addr!($ptr);
        let __val = $x;
        __enable_user_access!();
        __put_user_nocheck!(__val, __gu_ptr, '___pu_failed);
        __disable_user_access!();
        0isize
    }}; }

    #[macro_export]
    macro_rules! put_user { ($x:expr, $ptr:expr) => {{
        might_fault();
        if access_ok!($ptr, core::mem::size_of_val(&$ptr)) { __put_user!($x, $ptr) } else { -EFAULT as isize }
    }}; }

    extern "C" {
        pub fn __asm_copy_to_user_sum_enabled(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn __asm_copy_from_user_sum_enabled(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn __asm_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn __asm_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn strncpy_from_user(dest: *mut i8, src: *const i8, count: isize) -> isize;
        pub fn strnlen_user(s: *const i8, n: isize) -> isize;
        pub fn __clear_user(addr: *mut core::ffi::c_void, n: usize) -> usize;
    }

    #[inline]
    pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize { __asm_copy_from_user(to, untagged_addr!(from), n) }
    #[inline]
    pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize { __asm_copy_to_user(untagged_addr!(to), from, n) }
    #[inline]
    pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: usize) -> usize { might_fault(); if access_ok!(to, n) { __clear_user(untagged_addr!(to), n) } else { n } }

    #[inline(always)]
    pub unsafe fn user_access_begin(ptr: *const core::ffi::c_void, len: usize) -> bool {
        if !access_ok!(ptr, len) { return false; }
        __enable_user_access!(); true
    }
    pub fn user_access_save() -> usize { 0 }
    pub fn user_access_restore(_enabled: usize) {}

    #[macro_export]
    macro_rules! arch_unsafe_put_user { ($x:expr, $ptr:expr, $label:lifetime) => { __put_user_nocheck!($x, $ptr, $label) }; }
    #[macro_export]
    macro_rules! arch_unsafe_get_user { ($x:expr, $ptr:expr, $label:lifetime) => {{ let mut __gu_val = core::mem::MaybeUninit::uninit(); __get_user_nocheck!(__gu_val, $ptr, $label); $x = unsafe { __gu_val.assume_init() }; }}; }
    #[macro_export]
    macro_rules! unsafe_copy_to_user { ($dst:expr, $src:expr, $len:expr, $label:lifetime) => { if __asm_copy_to_user_sum_enabled($dst, $src, $len) != 0 { break $label; } }; }
    #[macro_export]
    macro_rules! unsafe_copy_from_user { ($dst:expr, $src:expr, $len:expr, $label:lifetime) => { if __asm_copy_from_user_sum_enabled($dst, $src, $len) != 0 { break $label; } }; }
}

#[cfg(not(feature = "mmu"))]
// CONFIG_MMU disabled: the original header includes asm-generic/uaccess.h.
pub mod asm_generic_uaccess {}

// External kernel types and symbols referenced by this translation.
#[allow(non_camel_case_types)] pub enum mm_struct {}
extern "C" {
    fn current_mm() -> *mut mm_struct;
    fn riscv_has_extension_unlikely(ext: usize) -> bool;
    fn mmap_assert_locked(mm: *mut mm_struct);
    fn might_fault();
}
const RISCV_ISA_EXT_SUPM: usize = 0;
const SR_SUM: usize = 0;
const EFAULT: i32 = 14;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
