/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2006 Andi Kleen, SUSE Labs.
 */

// C header dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_SPARC64)]
#[inline(always)]
unsafe fn vread_tick() -> u64 {
    let ret: u64;
    core::arch::asm!("rd %tick, {0}", out(reg) ret);
    ret
}

#[cfg(CONFIG_SPARC64)]
#[inline(always)]
unsafe fn vread_tick_stick() -> u64 {
    let ret: u64;
    core::arch::asm!("rd %asr24, {0}", out(reg) ret);
    ret
}

#[cfg(not(CONFIG_SPARC64))]
#[inline(always)]
unsafe fn vdso_shift_ns(val: u64, amt: u32) -> u64 {
    let ret: u64;
    core::arch::asm!(
        "sllx %H1, 32, %g1\n\t\
         srl %L1, 0, %L1\n\t\
         or %g1, %L1, %g1\n\t\
         srlx %g1, %2, %L0\n\t\
         srlx %L0, 32, %H0",
        out(reg) ret,
        in(reg) val,
        in(reg) amt,
        lateout("g1") _,
    );
    ret
}

#[cfg(not(CONFIG_SPARC64))]
#[inline(always)]
unsafe fn vread_tick() -> u64 {
    let mut ret: u64;
    core::arch::asm!("rd %tick, %L0\n\t srlx %L0, 32, %H0", inout("o4") ret);
    ret
}

#[cfg(not(CONFIG_SPARC64))]
#[inline(always)]
unsafe fn vread_tick_stick() -> u64 {
    let mut ret: u64;
    core::arch::asm!("rd %asr24, %L0\n\t srlx %L0, 32, %H0", inout("o4") ret);
    ret
}

#[inline(always)]
unsafe fn __arch_get_hw_counter(clock_mode: s32, _vd: *const struct_vdso_time_data) -> u64 {
    if likely(clock_mode == VDSO_CLOCKMODE_STICK) {
        vread_tick_stick()
    } else {
        vread_tick()
    }
}

#[cfg(CONFIG_SPARC64)]
const SYSCALL_STRING: &str = "ta\t0x6d;\nbcs,a\t1f;\n sub\t%g0, %o0, %o0;\n1:";
#[cfg(not(CONFIG_SPARC64))]
const SYSCALL_STRING: &str = "ta\t0x10;\nbcs,a\t1f;\n sub\t%g0, %o0, %o0;\n1:";

// SYSCALL_CLOBBERS: f0..f62 (even registers as listed by the C macro), cc, memory.

#[cfg(CONFIG_SPARC64)]
#[inline(always)]
unsafe fn clock_gettime_fallback(clock: clockid_t, ts: *mut struct___kernel_timespec) -> c_long {
    let num: c_long = __NR_clock_gettime;
    let o0: c_long = clock as c_long;
    let o1: c_long = ts as c_long;
    let mut result = o0;
    core::arch::asm!(SYSCALL_STRING, inout("o0") result, in("g1") num, in("o1") o1, clobber_abi("C"));
    result
}

#[cfg(not(CONFIG_SPARC64))]
#[inline(always)]
unsafe fn clock_gettime_fallback(clock: clockid_t, ts: *mut struct___kernel_timespec) -> c_long {
    let num: c_long = __NR_clock_gettime64;
    let mut result = clock as c_long;
    let o1: c_long = ts as c_long;
    core::arch::asm!(SYSCALL_STRING, inout("o0") result, in("g1") num, in("o1") o1, clobber_abi("C"));
    result
}

#[cfg(not(CONFIG_SPARC64))]
#[inline(always)]
unsafe fn clock_gettime32_fallback(clock: clockid_t, ts: *mut struct_old_timespec32) -> c_long {
    let num: c_long = __NR_clock_gettime;
    let mut result = clock as c_long;
    let o1: c_long = ts as c_long;
    core::arch::asm!(SYSCALL_STRING, inout("o0") result, in("g1") num, in("o1") o1, clobber_abi("C"));
    result
}

#[inline(always)]
unsafe fn gettimeofday_fallback(tv: *mut struct___kernel_old_timeval, tz: *mut struct_timezone) -> c_long {
    let num: c_long = __NR_gettimeofday;
    let mut result = tv as c_long;
    let o1: c_long = tz as c_long;
    core::arch::asm!(SYSCALL_STRING, inout("o0") result, in("g1") num, in("o1") o1, clobber_abi("C"));
    result
}

#[inline(always)]
unsafe fn __arch_get_vdso_u_time_data() -> *const struct_vdso_time_data {
    let ret: usize;
    // SPARC has no native PC-relative relocations; calculate the address manually.
    core::arch::asm!(
        "1:\n\tcall 3f\n\tnop\n\t2:\n\t.word vdso_u_time_data - .\n\t3:\n\tadd %o7, 2b - 1b, %o7\n\tldsw [%o7], {0}\n\tadd {0}, %o7, {0}",
        out(reg) ret,
        lateout("o7") _,
    );
    ret as *const struct_vdso_time_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
