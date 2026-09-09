/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_HAVE_SRAM_POOL is a build-time configuration condition. */
#[cfg(feature = "CONFIG_HAVE_SRAM_POOL")]
mod have_sram_pool {
    #[repr(C)]
    pub struct gen_pool {
        _private: [u8; 0],
    }

    unsafe extern "C" {
        /* arch/sh/mm/sram.c */
        pub static mut sram_pool: *mut gen_pool;

        pub fn gen_pool_alloc(pool: *mut gen_pool, len: usize) -> c_ulong;
        pub fn gen_pool_free(pool: *mut gen_pool, addr: c_ulong, len: usize);
    }

    type c_ulong = u64;

    #[inline]
    pub unsafe fn sram_alloc(len: usize) -> c_ulong {
        if sram_pool.is_null() {
            return 0u64;
        }

        gen_pool_alloc(sram_pool, len)
    }

    #[inline]
    pub unsafe fn sram_free(addr: c_ulong, len: usize) {
        gen_pool_free(sram_pool, addr, len);
    }
}

#[cfg(feature = "CONFIG_HAVE_SRAM_POOL")]
pub use have_sram_pool::{sram_alloc, sram_free};

#[cfg(not(feature = "CONFIG_HAVE_SRAM_POOL"))]
#[inline]
pub fn sram_alloc(_len: usize) -> u64 {
    0u64
}

#[cfg(not(feature = "CONFIG_HAVE_SRAM_POOL"))]
#[inline]
pub fn sram_free(_addr: u64, _len: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
