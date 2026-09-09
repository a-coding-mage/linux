/*
 * Copyright (C) 2018 ARM Limited
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

// C dependencies supplied by the surrounding kernel translation:
// asm/vdso/vdso.h, asm/clocksource.h, asm/unistd.h, and asm/vdso.h.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

// For MIPS ISA revisions below 6, the syscall clobbers HI and LO; otherwise
// there are no additional clobbers. This build-time condition is retained in
// the inline-assembly translations below.

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> libc::c_long {
    let mut ret: libc::c_long;
    let mut error: libc::c_long;
    let nr: libc::c_long = __NR_gettimeofday as libc::c_long;
    core::arch::asm!(
        "syscall",
        in("a0") _tv,
        in("a1") _tz,
        in("v0") nr,
        lateout("v0") ret,
        lateout("a3") error,
        lateout("$1") _, lateout("$3") _, lateout("$8") _,
        lateout("$9") _, lateout("$10") _, lateout("$11") _,
        lateout("$12") _, lateout("$13") _, lateout("$14") _,
        lateout("$15") _, lateout("$24") _, lateout("$25") _,
        options(nostack)
    );
    if error != 0 { -ret } else { ret }
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> libc::c_long {
    let mut ret: libc::c_long;
    let mut error: libc::c_long;
    // _MIPS_SIM == _MIPS_SIM_ABI64 selects __NR_clock_gettime;
    // other ABIs select __NR_clock_gettime64.
    let nr: libc::c_long = __NR_clock_gettime as libc::c_long;
    core::arch::asm!(
        "syscall", in("a0") _clkid, in("a1") _ts, in("v0") nr,
        lateout("v0") ret, lateout("a3") error,
        lateout("$1") _, lateout("$3") _, lateout("$8") _,
        lateout("$9") _, lateout("$10") _, lateout("$11") _,
        lateout("$12") _, lateout("$13") _, lateout("$14") _,
        lateout("$15") _, lateout("$24") _, lateout("$25") _,
        options(nostack)
    );
    if error != 0 { -ret } else { ret }
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(_clkid: clockid_t, _ts: *mut __kernel_timespec) -> libc::c_int {
    let mut ret: libc::c_long;
    let mut error: libc::c_long;
    let nr: libc::c_long = __NR_clock_getres as libc::c_long;
    core::arch::asm!(
        "syscall", in("a0") _clkid, in("a1") _ts, in("v0") nr,
        lateout("v0") ret, lateout("a3") error,
        lateout("$1") _, lateout("$3") _, lateout("$8") _,
        lateout("$9") _, lateout("$10") _, lateout("$11") _,
        lateout("$12") _, lateout("$13") _, lateout("$14") _,
        lateout("$15") _, lateout("$24") _, lateout("$25") _,
        options(nostack)
    );
    (if error != 0 { -ret } else { ret }) as libc::c_int
}

// Present only when _MIPS_SIM != _MIPS_SIM_ABI64.
#[inline(always)]
pub unsafe fn clock_gettime32_fallback(_clkid: clockid_t, _ts: *mut old_timespec32) -> libc::c_long {
    let mut ret: libc::c_long;
    let mut error: libc::c_long;
    let nr: libc::c_long = __NR_clock_gettime as libc::c_long;
    core::arch::asm!(
        "syscall", in("a0") _clkid, in("a1") _ts, in("v0") nr,
        lateout("v0") ret, lateout("a3") error,
        lateout("$1") _, lateout("$3") _, lateout("$8") _,
        lateout("$9") _, lateout("$10") _, lateout("$11") _,
        lateout("$12") _, lateout("$13") _, lateout("$14") _,
        lateout("$15") _, lateout("$24") _, lateout("$25") _,
        options(nostack)
    );
    if error != 0 { -ret } else { ret }
}

#[inline(always)]
pub unsafe fn clock_getres32_fallback(_clkid: clockid_t, _ts: *mut old_timespec32) -> libc::c_int {
    let mut ret: libc::c_long;
    let mut error: libc::c_long;
    let nr: libc::c_long = __NR_clock_getres as libc::c_long;
    core::arch::asm!(
        "syscall", in("a0") _clkid, in("a1") _ts, in("v0") nr,
        lateout("v0") ret, lateout("a3") error,
        lateout("$1") _, lateout("$3") _, lateout("$8") _,
        lateout("$9") _, lateout("$10") _, lateout("$11") _,
        lateout("$12") _, lateout("$13") _, lateout("$14") _,
        lateout("$15") _, lateout("$24") _, lateout("$25") _,
        options(nostack)
    );
    (if error != 0 { -ret } else { ret }) as libc::c_int
}

#[inline(always)]
pub unsafe fn read_r4k_count() -> u64 {
    let mut count: u32;
    core::arch::asm!(".set push", ".set mips32r2", "rdhwr {0}, $2", ".set pop", out(reg) count);
    count as u64
}

#[inline(always)]
pub unsafe fn read_gic_count(data: *const vdso_time_data) -> u64 {
    let gic: *mut core::ffi::c_void = get_gic(data);
    let mut hi: u32;
    let mut hi2: u32;
    let mut lo: u32;
    loop {
        hi = __raw_readl(gic.byte_add(core::mem::size_of::<u32>()));
        lo = __raw_readl(gic);
        hi2 = __raw_readl(gic.byte_add(core::mem::size_of::<u32>()));
        if hi2 == hi { break; }
    }
    ((hi as u64) << 32).wrapping_add(lo as u64)
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(clock_mode: i32, vd: *const vdso_time_data) -> u64 {
    // CONFIG_CSRC_R4K: if clock_mode == VDSO_CLOCKMODE_R4K, return read_r4k_count().
    // CONFIG_CLKSRC_MIPS_GIC: if clock_mode == VDSO_CLOCKMODE_GIC, return read_gic_count(vd).
    0
}

#[inline]
pub fn mips_vdso_hres_capable() -> bool {
    // IS_ENABLED(CONFIG_CSRC_R4K) || IS_ENABLED(CONFIG_CLKSRC_MIPS_GIC)
    false
}

pub use mips_vdso_hres_capable as __arch_vdso_hres_capable;

#[inline(always)]
pub unsafe fn __arch_get_vdso_u_time_data() -> *const vdso_time_data {
    get_vdso_time_data()
}

pub use __arch_get_vdso_u_time_data as __arch_get_vdso_u_time_data_alias;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
