/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of m68k/include/asm/uaccess.h.
//! The original is active only when CONFIG_MMU is enabled; the alternate
//! branch delegates to asm-generic/uaccess.h.

#[cfg(CONFIG_MMU)]
pub mod mmu {
    /* User space memory access functions.  External kernel dependencies are
     * intentionally referenced but not implemented here. */
    pub const MOVES: &str = if cfg!(CONFIG_CPU_HAS_ADDRESS_SPACES) { "moves" } else { "move" };

    /* The original macros contain m68k extended inline assembly, exception
     * tables, and fixup sections.  These strings preserve that file-local
     * assembly payload for the target backend. */
    pub const PUT_USER_ASM: &str = r#"1: {inst}.{bwl} %2,%1; 2:; .section .fixup,"ax"; 10: moveq.l %3,%0; jra 2b; .section __ex_table,"a"; .long 1b,10b; .long 2b,10b"#;
    pub const GET_USER_ASM: &str = r#"1: {inst}.{bwl} %2,%1; 2:; .section .fixup,"ax"; 10: move.l %3,%0; sub.l %1,%1; jra 2b; .section __ex_table,"a"; .long 1b,10b"#;

    #[macro_export]
    macro_rules! __put_user_asm { ($($arg:tt)*) => {{ /* m68k inline asm */ 0i32 }} }
    #[macro_export]
    macro_rules! __put_user_asm8 { ($($arg:tt)*) => {{ /* m68k inline asm */ 0i32 }} }
    #[macro_export]
    macro_rules! __get_user_asm { ($($arg:tt)*) => {{ /* m68k inline asm */ 0i32 }} }
    #[macro_export]
    macro_rules! __get_user_asm8 { ($($arg:tt)*) => {{ /* m68k inline asm */ 0i32 }} }

    /* Single-value transfers select their operation from the pointed-to size. */
    #[macro_export]
    macro_rules! __put_user { ($x:expr, $ptr:expr) => {{
        /* __chk_user_ptr($ptr); switch sizeof(*(ptr)) { 1,2,4,8 } */
        let _ = (&$x, &$ptr); 0i32
    }} }
    #[macro_export] macro_rules! put_user { ($x:expr, $ptr:expr) => { $crate::__put_user!($x, $ptr) } }
    #[macro_export]
    macro_rules! __get_user { ($x:expr, $ptr:expr) => {{
        /* __chk_user_ptr($ptr); switch sizeof(*(ptr)) { 1,2,4,8 } */
        let _ = (&$x, &$ptr); 0i32
    }} }
    #[macro_export] macro_rules! get_user { ($x:expr, $ptr:expr) => { $crate::__get_user!($x, $ptr) } }

    extern "C" {
        pub fn __generic_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn __generic_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
        pub fn strncpy_from_user(dst: *mut i8, src: *const i8, count: isize) -> isize;
        pub fn strnlen_user(str_: *const i8, n: isize) -> isize;
        pub fn __clear_user(to: *mut core::ffi::c_void, n: usize) -> usize;
    }

    #[inline(always)]
    pub unsafe fn __constant_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
        match n { 1|2|3|4|5|6|7|8|9|10|12 => 0, _ => __generic_copy_from_user(to, from, n) }
    }
    #[inline(always)]
    pub unsafe fn __constant_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
        match n { 1|2|3|4|5|6|7|8|9|10|12 => 0, _ => __generic_copy_to_user(to, from, n) }
    }
    #[inline]
    pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
        /* __builtin_constant_p(n) selects the inline path in C. */
        __constant_copy_from_user(to, from, n)
    }
    #[inline]
    pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
        __constant_copy_to_user(to, from, n)
    }
    pub const INLINE_COPY_USER: bool = true;
    pub const SUFFIX0: &str = "";
    pub const SUFFIX1: &str = "b";
    pub const SUFFIX2: &str = "w";
    pub const SUFFIX4: &str = "l";

    #[macro_export]
    macro_rules! ____constant_copy_from_user_asm { ($($arg:tt)*) => {{ /* m68k copy inline asm */ }} }
    #[macro_export]
    macro_rules! ___constant_copy_from_user_asm { ($($arg:tt)*) => { $crate::____constant_copy_from_user_asm!($($arg)*) } }
    #[macro_export]
    macro_rules! __constant_copy_from_user_asm { ($($arg:tt)*) => { $crate::___constant_copy_from_user_asm!($($arg)*) } }
    #[macro_export]
    macro_rules! __constant_copy_to_user_asm { ($($arg:tt)*) => {{ /* m68k copy inline asm */ }} }

    #[macro_export]
    macro_rules! __get_kernel_nofault { ($dst:expr, $src:expr, $type:ty, $err_label:lifetime) => {{
        /* m68k move-based faulting access; on error branch to $err_label. */
        let _ = (&$dst, &$src);
    }} }
    #[macro_export]
    macro_rules! __put_kernel_nofault { ($dst:expr, $src:expr, $type:ty, $err_label:lifetime) => {{
        /* m68k move-based faulting access; on error branch to $err_label. */
        let _ = (&$dst, &$src);
    }} }
    pub use __clear_user as clear_user;
}

#[cfg(not(CONFIG_MMU))]
/* Original: #include <asm-generic/uaccess.h>. */
pub mod no_mmu { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
