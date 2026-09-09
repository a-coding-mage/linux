/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_SH_CACHE_INSNS_32_H
// Dependency: linux/types.h (provides reg_size_t).

/* When CONFIG_CPU_SH4A is enabled, invalidate the instruction cache block. */
#[cfg(CONFIG_CPU_SH4A)]
macro_rules! __icbi {
    ($addr:expr) => {{
        unsafe {
            core::arch::asm!("icbi @{}", in(reg) $addr);
        }
    }};
}

/* Otherwise, the C macro expands to mb(). */
#[cfg(not(CONFIG_CPU_SH4A))]
macro_rules! __icbi {
    ($addr:expr) => {{
        let _ = $addr;
        mb();
    }};
}

macro_rules! __ocbp {
    ($addr:expr) => {{
        unsafe {
            core::arch::asm!("ocbp @{}", in(reg) $addr);
        }
    }};
}

macro_rules! __ocbi {
    ($addr:expr) => {{
        unsafe {
            core::arch::asm!("ocbi @{}", in(reg) $addr);
        }
    }};
}

macro_rules! __ocbwb {
    ($addr:expr) => {{
        unsafe {
            core::arch::asm!("ocbwb @{}", in(reg) $addr);
        }
    }};
}

#[inline]
pub(crate) fn register_align(val: *mut core::ffi::c_void) -> reg_size_t {
    val as isize as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
