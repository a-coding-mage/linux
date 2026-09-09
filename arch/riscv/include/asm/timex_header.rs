/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependency supplied by <asm/csr.h>.

pub type cycles_t = core::ffi::c_ulong;

#[cfg(CONFIG_RISCV_M_MODE)]
mod riscv_m_mode {
    // Dependency supplied by <asm/clint.h>.
    unsafe extern "C" {
        static mut clint_time_val: *mut core::ffi::c_void;
        fn readq_relaxed(addr: *const core::ffi::c_void) -> u64;
        fn readl_relaxed(addr: *const u32) -> u32;
        fn random_get_entropy_fallback() -> core::ffi::c_ulong;
    }

    #[cfg(CONFIG_64BIT)]
    #[inline]
    pub unsafe fn get_cycles() -> super::cycles_t {
        readq_relaxed(clint_time_val as *const core::ffi::c_void) as super::cycles_t
    }

    #[cfg(not(CONFIG_64BIT))]
    #[inline]
    pub unsafe fn get_cycles() -> u32 {
        readl_relaxed(clint_time_val as *const u32)
    }

    #[cfg(not(CONFIG_64BIT))]
    #[inline]
    pub unsafe fn get_cycles_hi() -> u32 {
        readl_relaxed((clint_time_val as *const u32).add(1))
    }

    #[inline]
    pub unsafe fn random_get_entropy() -> core::ffi::c_ulong {
        if clint_time_val.is_null() {
            return random_get_entropy_fallback();
        }
        get_cycles() as core::ffi::c_ulong
    }
}

#[cfg(not(CONFIG_RISCV_M_MODE))]
mod riscv_s_mode {
    unsafe extern "C" {
        // Dependency supplied by <asm/csr.h>.
        fn csr_read(csr: u32) -> core::ffi::c_ulong;
    }

    #[inline]
    pub unsafe fn get_cycles() -> super::cycles_t {
        csr_read(CSR_TIME) as super::cycles_t
    }

    #[inline]
    pub unsafe fn get_cycles_hi() -> u32 {
        csr_read(CSR_TIMEH) as u32
    }
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn get_cycles64() -> u64 {
    #[cfg(CONFIG_RISCV_M_MODE)]
    { riscv_m_mode::get_cycles() as u64 }
    #[cfg(not(CONFIG_RISCV_M_MODE))]
    { riscv_s_mode::get_cycles() as u64 }
}

#[cfg(not(CONFIG_64BIT))]
#[inline]
pub unsafe fn get_cycles64() -> u64 {
    let mut hi: u32;
    let lo: u32;
    loop {
        #[cfg(CONFIG_RISCV_M_MODE)]
        { hi = riscv_m_mode::get_cycles_hi(); lo = riscv_m_mode::get_cycles(); }
        #[cfg(not(CONFIG_RISCV_M_MODE))]
        { hi = riscv_s_mode::get_cycles_hi(); lo = riscv_s_mode::get_cycles(); }

        let hi_again = {
            #[cfg(CONFIG_RISCV_M_MODE)]
            { riscv_m_mode::get_cycles_hi() }
            #[cfg(not(CONFIG_RISCV_M_MODE))]
            { riscv_s_mode::get_cycles_hi() }
        };
        if hi == hi_again {
            return ((hi as u64) << 32) | lo as u64;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
