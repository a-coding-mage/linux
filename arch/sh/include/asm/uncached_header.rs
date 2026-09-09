/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/bug.h supplies BUG().

#[cfg(CONFIG_UNCACHED_MAPPING)]
extern "C" {
    pub static mut cached_to_uncached: ::core::ffi::c_ulong;
    pub static mut uncached_size: ::core::ffi::c_ulong;
    pub static mut uncached_start: ::core::ffi::c_ulong;
    pub static mut uncached_end: ::core::ffi::c_ulong;

    pub fn virt_addr_uncached(kaddr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn uncached_init();
    pub fn uncached_resize(size: ::core::ffi::c_ulong);
}

/*
 * Jump to uncached area.
 * When handling TLB or caches, we need to do it from an uncached area.
 */
#[cfg(CONFIG_UNCACHED_MAPPING)]
#[macro_export]
macro_rules! jump_to_uncached {
    () => {{
        let mut __dummy: ::core::ffi::c_ulong;
        unsafe {
            ::core::arch::asm!(
                "mova 1f, {0}",
                "add {1}, {0}",
                "jmp @{0}",
                " nop",
                ".balign 4",
                "1:",
                lateout(reg) __dummy,
                in(reg) $crate::cached_to_uncached,
            );
        }
    }};
}

/*
 * Back to cached area.
 */
#[cfg(CONFIG_UNCACHED_MAPPING)]
#[macro_export]
macro_rules! back_to_cached {
    () => {{
        let mut __dummy: ::core::ffi::c_ulong;
        unsafe {
            ctrl_barrier();
            ::core::arch::asm!(
                "mov.l 1f, {0}",
                "jmp @{0}",
                " nop",
                ".balign 4",
                "1: .long 2f",
                "2:",
                lateout(reg) __dummy,
            );
        }
    }};
}

#[cfg(not(CONFIG_UNCACHED_MAPPING))]
#[macro_export]
macro_rules! virt_addr_uncached {
    ($kaddr:expr) => { 0 };
}

#[cfg(not(CONFIG_UNCACHED_MAPPING))]
#[macro_export]
macro_rules! uncached_init {
    () => {{ }};
}

#[cfg(not(CONFIG_UNCACHED_MAPPING))]
#[macro_export]
macro_rules! uncached_resize {
    ($size:expr) => {{
        BUG();
    }};
}

#[cfg(not(CONFIG_UNCACHED_MAPPING))]
#[macro_export]
macro_rules! jump_to_uncached {
    () => {{ }};
}

#[cfg(not(CONFIG_UNCACHED_MAPPING))]
#[macro_export]
macro_rules! back_to_cached {
    () => {{ }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
