/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 * Copyright 2022, Madhavan Srinivasan, IBM Corp.
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* C dependencies: <sys/stat.h> and "../event.h" */

pub const POWER11: c_int = 0x82;
pub const POWER10: c_int = 0x80;
pub const POWER9: c_int = 0x4e;
pub const PERF_POWER9_MASK: u64 = 0x7f8ffffffffffff;
pub const PERF_POWER10_MASK: u64 = 0x7ffffffffffffff;
pub const PERF_POWER11_MASK: u64 = PERF_POWER10_MASK;

pub const MMCR0_FC56: u64 = 0x00000010; /* freeze counters 5 and 6 */
pub const MMCR0_PMCCEXT: u64 = 0x00000200; /* PMCCEXT control */
pub const MMCR1_RSQ: u64 = 0x200000000000; /* radix scope qual field */
pub const BHRB_DISABLE: u64 = 0x2000000000; /* MMCRA BHRB DISABLE bit */

/* Type supplied by "../event.h". */
pub type event = crate::event;

unsafe extern "C" {
    pub static mut ev_mask_pmcxsel: c_int;
    pub static mut ev_shift_pmcxsel: c_int;
    pub static mut ev_mask_marked: c_int;
    pub static mut ev_shift_marked: c_int;
    pub static mut ev_mask_comb: c_int;
    pub static mut ev_shift_comb: c_int;
    pub static mut ev_mask_unit: c_int;
    pub static mut ev_shift_unit: c_int;
    pub static mut ev_mask_pmc: c_int;
    pub static mut ev_shift_pmc: c_int;
    pub static mut ev_mask_cache: c_int;
    pub static mut ev_shift_cache: c_int;
    pub static mut ev_mask_sample: c_int;
    pub static mut ev_shift_sample: c_int;
    pub static mut ev_mask_thd_sel: c_int;
    pub static mut ev_shift_thd_sel: c_int;
    pub static mut ev_mask_thd_start: c_int;
    pub static mut ev_shift_thd_start: c_int;
    pub static mut ev_mask_thd_stop: c_int;
    pub static mut ev_shift_thd_stop: c_int;
    pub static mut ev_mask_thd_cmp: c_int;
    pub static mut ev_shift_thd_cmp: c_int;
    pub static mut ev_mask_sm: c_int;
    pub static mut ev_shift_sm: c_int;
    pub static mut ev_mask_rsq: c_int;
    pub static mut ev_shift_rsq: c_int;
    pub static mut ev_mask_l2l3: c_int;
    pub static mut ev_shift_l2l3: c_int;
    pub static mut ev_mask_mmcr3_src: c_int;
    pub static mut ev_shift_mmcr3_src: c_int;
    pub static mut pvr: c_int;
    pub static mut platform_extended_mask: u64;

    pub fn check_pvr_for_sampling_tests() -> c_int;
    pub fn platform_check_for_tests() -> c_int;
    pub fn check_extended_regs_support() -> c_int;
    pub fn perf_get_platform_reg_mask() -> u64;

    pub fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void;
    pub fn __event_read_samples(
        sample_buff: *mut c_void,
        size: *mut usize,
        sample_count: *mut u64,
    ) -> *mut c_void;
    pub fn collect_samples(sample_buff: *mut c_void) -> c_int;
    pub fn get_intr_regs(event: *mut event, sample_buff: *mut c_void) -> *mut u64;
    pub fn get_reg_value(intr_regs: *mut u64, register_name: *mut c_char) -> u64;
    pub fn get_thresh_cmp_val(event: event) -> c_int;
    pub fn check_for_generic_compat_pmu() -> bool;
    pub fn check_for_compat_mode() -> bool;

    pub fn have_hwcap2(feature: c_ulong) -> bool;
}

/* Constant supplied by the powerpc selftest support headers. */
pub const PPC_FEATURE2_ARCH_3_1: c_ulong = crate::PPC_FEATURE2_ARCH_3_1;

/*
 * Event code field extraction macro.
 * Raw event code is combination of multiple
 * fields. Macro to extract individual fields
 *
 * x - Raw event code value
 * y - Field to extract
 */
#[macro_export]
macro_rules! EV_CODE_EXTRACT {
    ($x:expr, pmcxsel) => {
        (($x >> unsafe { $crate::ev_shift_pmcxsel }) & unsafe { $crate::ev_mask_pmcxsel })
    };
    ($x:expr, marked) => {
        (($x >> unsafe { $crate::ev_shift_marked }) & unsafe { $crate::ev_mask_marked })
    };
    ($x:expr, comb) => {
        (($x >> unsafe { $crate::ev_shift_comb }) & unsafe { $crate::ev_mask_comb })
    };
    ($x:expr, unit) => {
        (($x >> unsafe { $crate::ev_shift_unit }) & unsafe { $crate::ev_mask_unit })
    };
    ($x:expr, pmc) => {
        (($x >> unsafe { $crate::ev_shift_pmc }) & unsafe { $crate::ev_mask_pmc })
    };
    ($x:expr, cache) => {
        (($x >> unsafe { $crate::ev_shift_cache }) & unsafe { $crate::ev_mask_cache })
    };
    ($x:expr, sample) => {
        (($x >> unsafe { $crate::ev_shift_sample }) & unsafe { $crate::ev_mask_sample })
    };
    ($x:expr, thd_sel) => {
        (($x >> unsafe { $crate::ev_shift_thd_sel }) & unsafe { $crate::ev_mask_thd_sel })
    };
    ($x:expr, thd_start) => {
        (($x >> unsafe { $crate::ev_shift_thd_start }) & unsafe { $crate::ev_mask_thd_start })
    };
    ($x:expr, thd_stop) => {
        (($x >> unsafe { $crate::ev_shift_thd_stop }) & unsafe { $crate::ev_mask_thd_stop })
    };
    ($x:expr, thd_cmp) => {
        (($x >> unsafe { $crate::ev_shift_thd_cmp }) & unsafe { $crate::ev_mask_thd_cmp })
    };
    ($x:expr, sm) => {
        (($x >> unsafe { $crate::ev_shift_sm }) & unsafe { $crate::ev_mask_sm })
    };
    ($x:expr, rsq) => {
        (($x >> unsafe { $crate::ev_shift_rsq }) & unsafe { $crate::ev_mask_rsq })
    };
    ($x:expr, l2l3) => {
        (($x >> unsafe { $crate::ev_shift_l2l3 }) & unsafe { $crate::ev_mask_l2l3 })
    };
    ($x:expr, mmcr3_src) => {
        (($x >> unsafe { $crate::ev_shift_mmcr3_src }) & unsafe { $crate::ev_mask_mmcr3_src })
    };
}

#[inline]
pub fn get_mmcr0_fc56(mmcr0: u64, _pmc: c_int) -> c_int {
    (mmcr0 & MMCR0_FC56) as c_int
}

#[inline]
pub fn get_mmcr0_pmccext(mmcr0: u64, _pmc: c_int) -> c_int {
    (mmcr0 & MMCR0_PMCCEXT) as c_int
}

#[inline]
pub fn get_mmcr0_pmao(mmcr0: u64, _pmc: c_int) -> c_int {
    ((mmcr0 >> 7) & 0x1) as c_int
}

#[inline]
pub fn get_mmcr0_cc56run(mmcr0: u64, _pmc: c_int) -> c_int {
    ((mmcr0 >> 8) & 0x1) as c_int
}

#[inline]
pub fn get_mmcr0_pmcjce(mmcr0: u64, _pmc: c_int) -> c_int {
    ((mmcr0 >> 14) & 0x1) as c_int
}

#[inline]
pub fn get_mmcr0_pmc1ce(mmcr0: u64, _pmc: c_int) -> c_int {
    ((mmcr0 >> 15) & 0x1) as c_int
}

#[inline]
pub fn get_mmcr0_pmae(mmcr0: u64, _pmc: c_int) -> c_int {
    ((mmcr0 >> 27) & 0x1) as c_int
}

#[inline]
pub fn get_mmcr1_pmcxsel(mmcr1: u64, pmc: c_int) -> c_int {
    ((mmcr1 >> (24 - ((pmc - 1) * 8))) & 0xff) as c_int
}

#[inline]
pub fn get_mmcr1_unit(mmcr1: u64, pmc: c_int) -> c_int {
    ((mmcr1 >> (60 - (4 * (pmc - 1)))) & 0xf) as c_int
}

#[inline]
pub fn get_mmcr1_comb(mmcr1: u64, pmc: c_int) -> c_int {
    ((mmcr1 >> (38 - ((pmc - 1) * 2))) & 0x3) as c_int
}

#[inline]
pub fn get_mmcr1_cache(mmcr1: u64, _pmc: c_int) -> c_int {
    ((mmcr1 >> 46) & 0x3) as c_int
}

#[inline]
pub fn get_mmcr1_rsq(mmcr1: u64, _pmc: c_int) -> c_int {
    (mmcr1 & MMCR1_RSQ) as c_int
}

#[inline]
pub fn get_mmcr2_fcs(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (63 - ((pmc - 1) * 9)))) >> (63 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcp(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (62 - ((pmc - 1) * 9)))) >> (62 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcpc(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (61 - ((pmc - 1) * 9)))) >> (61 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcm1(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (60 - ((pmc - 1) * 9)))) >> (60 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcm0(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (59 - ((pmc - 1) * 9)))) >> (59 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcwait(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (58 - ((pmc - 1) * 9)))) >> (58 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fch(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (57 - ((pmc - 1) * 9)))) >> (57 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcti(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (56 - ((pmc - 1) * 9)))) >> (56 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub fn get_mmcr2_fcta(mmcr2: u64, pmc: c_int) -> c_int {
    ((mmcr2 & (1u64 << (55 - ((pmc - 1) * 9)))) >> (55 - ((pmc - 1) * 9))) as c_int
}

#[inline]
pub unsafe fn get_mmcr2_l2l3(mmcr2: u64, _pmc: c_int) -> c_int {
    if unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) } {
        return ((mmcr2 & 0xf8) >> 3) as c_int;
    }
    0
}

#[inline]
pub unsafe fn get_mmcr3_src(mmcr3: u64, pmc: c_int) -> c_int {
    if !unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) } {
        return 0;
    }
    ((mmcr3 >> (49 - (15 * (pmc - 1)))) & 0x7fff) as c_int
}

#[inline]
pub unsafe fn get_mmcra_thd_cmp(mmcra: u64, _pmc: c_int) -> c_int {
    if unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) } {
        return ((mmcra >> 45) & 0x7ff) as c_int;
    }
    ((mmcra >> 45) & 0x3ff) as c_int
}

#[inline]
pub fn get_mmcra_sm(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 42) & 0x3) as c_int
}

#[inline]
pub unsafe fn get_mmcra_bhrb_disable(mmcra: u64, _pmc: c_int) -> u64 {
    if unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) } {
        return mmcra & BHRB_DISABLE;
    }
    0
}

#[inline]
pub fn get_mmcra_ifm(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 30) & 0x3) as c_int
}

#[inline]
pub fn get_mmcra_thd_sel(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 16) & 0x7) as c_int
}

#[inline]
pub fn get_mmcra_thd_start(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 12) & 0xf) as c_int
}

#[inline]
pub fn get_mmcra_thd_stop(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 8) & 0xf) as c_int
}

#[inline]
pub fn get_mmcra_rand_samp_elig(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 4) & 0x7) as c_int
}

#[inline]
pub fn get_mmcra_sample_mode(mmcra: u64, _pmc: c_int) -> c_int {
    ((mmcra >> 1) & 0x3) as c_int
}

#[inline]
pub fn get_mmcra_marked(mmcra: u64, _pmc: c_int) -> c_int {
    (mmcra & 0x1) as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
