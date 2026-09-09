/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of arch/arm64/include/asm/arch_timer.h
 *
 * C headers and build-time configuration are supplied by other translated
 * units.  Their dependency intent is retained here without implementing them.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// asm/barrier.h, asm/hwcap.h, asm/sysreg.h, linux/*, and
// clocksource/arm_arch_timer.h provide the referenced external symbols.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum arch_timer_erratum_match_type {
    ate_match_dt,
    ate_match_local_cap_id,
    ate_match_acpi_oem_info,
}

#[repr(C)]
pub struct clock_event_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arch_timer_erratum_workaround {
    pub match_type: arch_timer_erratum_match_type,
    pub id: *const c_void,
    pub desc: *const c_char,
    pub read_cntpct_el0: Option<unsafe extern "C" fn() -> u64>,
    pub read_cntvct_el0: Option<unsafe extern "C" fn() -> u64>,
    pub set_next_event_phys:
        Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> c_int>,
    pub set_next_event_virt:
        Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> c_int>,
    pub disable_compat_vdso: bool,
}

extern "C" {
    pub static mut timer_unstable_counter_workaround:
        *const arch_timer_erratum_workaround;

    fn isb();
    fn arch_counter_enforce_ordering(cnt: u64);
    fn read_sysreg(reg: u32) -> u64;
    fn write_sysreg(val: u64, reg: u32);
    fn cpu_set_named_feature(feature: u32);
    fn cpu_have_named_feature(feature: u32) -> bool;
}

// CONFIG_ARM_ARCH_TIMER_OOL_WORKAROUND controls these C preprocessor macros.
// The out-of-line workaround selection is represented by the external pointer.
#[inline]
pub unsafe fn has_erratum_handler(_h: usize) -> bool {
    !timer_unstable_counter_workaround.is_null()
}

// The C macro returns the workaround handler when present, otherwise the
// corresponding arch_timer_* function.  Callers requiring a specific handler
// should perform that selection against the external workaround object.

#[inline(always)]
pub unsafe fn arch_timer_read_cntpct_el0() -> u64 {
    // The C implementation uses ALTERNATIVE inline assembly for CNTVCTSS/ECV.
    // This declaration preserves the externally supplied architectural read.
    unsafe extern "C" {
        fn arch_timer_read_cntpct_el0_asm() -> u64;
    }
    arch_timer_read_cntpct_el0_asm()
}

#[inline(always)]
pub unsafe fn arch_timer_read_cntvct_el0() -> u64 {
    unsafe extern "C" {
        fn arch_timer_read_cntvct_el0_asm() -> u64;
    }
    arch_timer_read_cntvct_el0_asm()
}

// arch_timer_reg_read_stable(reg) invokes erratum_handler(read_##reg)().

pub const ARCH_TIMER_PHYS_ACCESS: c_int = 0;
pub const ARCH_TIMER_VIRT_ACCESS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum arch_timer_reg {
    ARCH_TIMER_REG_CTRL,
    ARCH_TIMER_REG_CVAL,
}

#[inline(always)]
pub unsafe fn arch_timer_reg_write_cp15(access: c_int, reg: arch_timer_reg, val: u64) {
    match (access, reg) {
        (ARCH_TIMER_PHYS_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CTRL) => {
            write_sysreg(val, 0);
            isb();
        }
        (ARCH_TIMER_PHYS_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CVAL) => write_sysreg(val, 1),
        (ARCH_TIMER_VIRT_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CTRL) => {
            write_sysreg(val, 2);
            isb();
        }
        (ARCH_TIMER_VIRT_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CVAL) => write_sysreg(val, 3),
        _ => panic!("BUILD_BUG"),
    }
}

#[inline(always)]
pub unsafe fn arch_timer_reg_read_cp15(access: c_int, reg: arch_timer_reg) -> u64 {
    match (access, reg) {
        (ARCH_TIMER_PHYS_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CTRL) => read_sysreg(0),
        (ARCH_TIMER_VIRT_ACCESS, arch_timer_reg::ARCH_TIMER_REG_CTRL) => read_sysreg(2),
        _ => panic!("BUILD_BUG"),
    }
}

#[inline]
pub unsafe fn arch_timer_get_cntfrq() -> u32 { read_sysreg(4) as u32 }

#[inline]
pub unsafe fn arch_timer_get_cntkctl() -> u32 { read_sysreg(5) as u32 }

#[inline]
pub unsafe fn arch_timer_set_cntkctl(cntkctl: u32) {
    write_sysreg(cntkctl as u64, 6);
    isb();
}

#[inline(always)]
pub unsafe fn __arch_counter_get_cntpct_stable() -> u64 {
    let cnt = arch_timer_read_cntpct_el0();
    arch_counter_enforce_ordering(cnt);
    cnt
}

#[inline(always)]
pub unsafe fn __arch_counter_get_cntpct() -> u64 {
    let cnt = arch_timer_read_cntpct_el0();
    arch_counter_enforce_ordering(cnt);
    cnt
}

#[inline(always)]
pub unsafe fn __arch_counter_get_cntvct_stable() -> u64 {
    let cnt = arch_timer_read_cntvct_el0();
    arch_counter_enforce_ordering(cnt);
    cnt
}

#[inline(always)]
pub unsafe fn __arch_counter_get_cntvct() -> u64 {
    let cnt = arch_timer_read_cntvct_el0();
    arch_counter_enforce_ordering(cnt);
    cnt
}

#[inline]
pub fn arch_timer_arch_init() -> c_int { 0 }

#[inline]
pub unsafe fn arch_timer_set_evtstrm_feature() {
    cpu_set_named_feature(0);
    // CONFIG_COMPAT: compat_elf_hwcap |= COMPAT_HWCAP_EVTSTRM;
}

#[inline]
pub unsafe fn arch_timer_have_evtstrm_feature() -> bool {
    cpu_have_named_feature(0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
