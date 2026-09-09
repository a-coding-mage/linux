/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel/vDSO environment:
// asm/barrier.h, asm/unistd.h, asm/csr.h, and uapi/linux/time.h.
// This translation applies to the non-assembler portion of the header.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> i32 {
    let mut ret: isize;
    let nr: isize = __NR_gettimeofday as isize;

    core::arch::asm!(
        "ecall",
        inlateout("a0") _tv as isize => ret,
        in("a1") _tz,
        in("a7") nr,
        options(nostack)
    );

    ret as i32
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> isize {
    let mut ret: isize;
    let nr: isize = __NR_clock_gettime as isize;

    core::arch::asm!(
        "ecall",
        inlateout("a0") _clkid as isize => ret,
        in("a1") _ts,
        in("a7") nr,
        options(nostack)
    );

    ret
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> i32 {
    let mut ret: isize;
    let nr: isize = __NR_clock_getres as isize;

    core::arch::asm!(
        "ecall",
        inlateout("a0") _clkid as isize => ret,
        in("a1") _ts,
        in("a7") nr,
        options(nostack)
    );

    ret as i32
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    _clock_mode: s32,
    _vd: *const vdso_time_data,
) -> u64 {
    /*
     * The purpose of csr_read(CSR_TIME) is to trap the system into
     * M-mode to obtain the value of CSR_TIME. Hence, unlike other
     * architecture, no fence instructions surround the csr_read()
     */
    csr_read(CSR_TIME)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
