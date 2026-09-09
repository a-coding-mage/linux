/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Same as asm-generic/percpu.h, except that we store the per cpu offset
 * in the paca. Based on the x86-64 implementation.
 *
 * The original declarations are conditional on __powerpc64__ and
 * CONFIG_SMP; those build-time conditions are retained here as comments.
 */

#[cfg(all(target_arch = "powerpc64", feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! __my_cpu_offset {
    () => {
        unsafe { (*local_paca).data_offset }
    };
}

/* Supplied by the platform's jump-label implementation when
 * CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK && CONFIG_SMP are enabled. */
#[cfg(all(
    feature = "CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK",
    feature = "CONFIG_SMP"
))]
extern "C" {
    static mut __percpu_first_chunk_is_paged: StaticKeyFalse;
}

#[cfg(all(
    feature = "CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK",
    feature = "CONFIG_SMP"
))]
#[inline]
pub unsafe fn percpu_first_chunk_is_paged() -> bool {
    static_key_enabled(unsafe { &(*core::ptr::addr_of!(__percpu_first_chunk_is_paged)).key })
}

#[cfg(not(all(
    feature = "CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK",
    feature = "CONFIG_SMP"
)))]
pub const percpu_first_chunk_is_paged: bool = false;

/* External types and functions are provided by the corresponding kernel
 * headers (asm-generic/percpu.h, linux/jump_label.h, and asm/paca.h). */
#[allow(non_camel_case_types)]
pub type StaticKeyFalse = __static_key_false;

#[repr(C)]
pub struct __static_key_false {
    pub key: StaticKey,
}

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

extern "C" {
    pub static mut local_paca: *mut Paca;
    fn static_key_enabled(key: *const StaticKey) -> bool;
}

#[repr(C)]
pub struct Paca {
    pub data_offset: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
