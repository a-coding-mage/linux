/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes linux/types.h and asm/march.h.
// MARCH_HAS_Z15_FEATURES and MARCH_HAS_Z196_FEATURES are build-time
// configuration symbols represented below with cfg! conditions.

#[inline(always)]
pub unsafe fn popcnt_z196(w: usize) -> usize {
    let cnt: usize;
    core::arch::asm!(
        ".insn rrf,0xb9e10000,{cnt},{w},0,0",
        cnt = lateout(reg) cnt,
        w = in(reg) w,
        options(nostack, preserves_flags),
    );
    cnt
}

#[inline(always)]
pub unsafe fn popcnt_z15(w: usize) -> usize {
    let cnt: usize;
    core::arch::asm!(
        ".insn rrf,0xb9e10000,{cnt},{w},8,0",
        cnt = lateout(reg) cnt,
        w = in(reg) w,
        options(nostack, preserves_flags),
    );
    cnt
}

unsafe extern "C" {
    pub fn __sw_hweight64(w: u64) -> usize;
    pub fn __sw_hweight32(w: u32) -> u32;
    pub fn __sw_hweight16(w: u32) -> u32;
    pub fn __sw_hweight8(w: u32) -> u32;
}

#[inline(always)]
pub unsafe fn __arch_hweight64(mut w: u64) -> usize {
    if cfg!(MARCH_HAS_Z15_FEATURES) {
        return popcnt_z15(w as usize);
    }
    if cfg!(MARCH_HAS_Z196_FEATURES) {
        w = popcnt_z196(w as usize) as u64;
        w = w.wrapping_add(w >> 32);
        w = w.wrapping_add(w >> 16);
        w = w.wrapping_add(w >> 8);
        return (w & 0xff) as usize;
    }
    __sw_hweight64(w)
}

#[inline(always)]
pub unsafe fn __arch_hweight32(mut w: u32) -> u32 {
    if cfg!(MARCH_HAS_Z15_FEATURES) {
        return popcnt_z15(w as usize) as u32;
    }
    if cfg!(MARCH_HAS_Z196_FEATURES) {
        w = popcnt_z196(w as usize) as u32;
        w = w.wrapping_add(w >> 16);
        w = w.wrapping_add(w >> 8);
        return w & 0xff;
    }
    __sw_hweight32(w)
}

#[inline(always)]
pub unsafe fn __arch_hweight16(mut w: u32) -> u32 {
    if cfg!(MARCH_HAS_Z15_FEATURES) {
        return popcnt_z15((w as u16) as usize) as u32;
    }
    if cfg!(MARCH_HAS_Z196_FEATURES) {
        w = popcnt_z196(w as usize) as u32;
        w = w.wrapping_add(w >> 8);
        return w & 0xff;
    }
    __sw_hweight16(w)
}

#[inline(always)]
pub unsafe fn __arch_hweight8(w: u32) -> u32 {
    if cfg!(MARCH_HAS_Z196_FEATURES) {
        return popcnt_z196((w as u8) as usize) as u32;
    }
    __sw_hweight8(w)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
