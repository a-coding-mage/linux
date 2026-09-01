// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// Translated from C implementation source. External types, constants, and
// functions are provided by the surrounding perf sources.

use core::ffi::{c_char, c_int, c_long, c_void};

type U64 = u64;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const PATH_MAX: usize = 4096;
const _SC_NPROCESSORS_CONF: c_int = 83;

unsafe extern "C" {
    static ARM_SPE_PMU_NAME: [c_char; 0];
    static HISI_PTT_PMU_NAME: [c_char; 0];
    static CORESIGHT_ETM_PMU_NAME: [c_char; 0];

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: c_int) -> c_long;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(dirp: *mut DIR);
    fn closedir(dirp: *mut DIR) -> c_int;

    fn perf_pmu__event_source_devices_scnprintf(buf: *mut c_char, size: usize) -> c_int;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evsel: *mut evsel) -> *mut evsel;
    fn evlist__is_last(evlist: *mut evlist, evsel: *mut evsel) -> bool;

    fn cs_etm_record_init(err: *mut c_int) -> *mut auxtrace_record;
    fn arm_spe_recording_init(err: *mut c_int, found_spe: *mut perf_pmu) -> *mut auxtrace_record;
    fn hisi_ptt_recording_init(err: *mut c_int, found_ptt: *mut perf_pmu) -> *mut auxtrace_record;

    fn pr_err(format: *const c_char, ...);
    fn pr_debug2(format: *const c_char, ...);
    fn smp_mb();
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct perf_pmu {
    pub type_: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
}

#[repr(C)]
pub struct auxtrace_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_mmap {
    pub userpg: *mut perf_event_mmap_page,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub aux_head: U64,
    pub aux_tail: U64,
}

unsafe fn find_all_arm_spe_pmus(nr_spes: *mut c_int, err: *mut c_int) -> *mut *mut perf_pmu {
    let mut arm_spe_pmus: *mut *mut perf_pmu = core::ptr::null_mut();
    let mut ret: c_int;
    let mut i: c_int;
    let nr_cpus: c_int = sysconf(_SC_NPROCESSORS_CONF) as c_int;
    /* arm_spe_xxxxxxxxx\0 */
    let mut arm_spe_pmu_name = [0 as c_char; 64];

    arm_spe_pmus = calloc(nr_cpus as usize, core::mem::size_of::<*mut perf_pmu>())
        as *mut *mut perf_pmu;
    if arm_spe_pmus.is_null() {
        pr_err(c"spes alloc failed\n".as_ptr());
        *err = -ENOMEM;
        return core::ptr::null_mut();
    }

    i = 0;
    while i < nr_cpus {
        ret = sprintf(
            arm_spe_pmu_name.as_mut_ptr(),
            c"%s%d".as_ptr(),
            ARM_SPE_PMU_NAME.as_ptr(),
            i,
        );
        if ret < 0 {
            pr_err(c"sprintf failed\n".as_ptr());
            *err = -ENOMEM;
            return core::ptr::null_mut();
        }

        *arm_spe_pmus.offset(*nr_spes as isize) = perf_pmus__find(arm_spe_pmu_name.as_ptr());
        if !(*arm_spe_pmus.offset(*nr_spes as isize)).is_null() {
            pr_debug2(
                c"%s %d: arm_spe_pmu %d type %d name %s\n".as_ptr(),
                c"find_all_arm_spe_pmus".as_ptr(),
                line!() as c_int,
                *nr_spes,
                (**arm_spe_pmus.offset(*nr_spes as isize)).type_,
                (**arm_spe_pmus.offset(*nr_spes as isize)).name,
            );
            *nr_spes += 1;
        }

        i += 1;
    }

    arm_spe_pmus
}

unsafe fn find_all_hisi_ptt_pmus(nr_ptts: *mut c_int, err: *mut c_int) -> *mut *mut perf_pmu {
    let mut hisi_ptt_pmus: *mut *mut perf_pmu = core::ptr::null_mut();
    let mut dent: *mut dirent;
    let mut path = [0 as c_char; PATH_MAX];
    let mut dir: *mut DIR = core::ptr::null_mut();
    let mut idx: c_int = 0;

    perf_pmu__event_source_devices_scnprintf(path.as_mut_ptr(), path.len());
    dir = opendir(path.as_ptr());
    if dir.is_null() {
        pr_err(c"can't read directory '%s'\n".as_ptr(), path.as_ptr());
        *err = -EINVAL;
        return core::ptr::null_mut();
    }

    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if !strstr((*dent).d_name.as_ptr(), HISI_PTT_PMU_NAME.as_ptr()).is_null() {
            *nr_ptts += 1;
        }
    }

    if *nr_ptts == 0 {
        closedir(dir);
        return hisi_ptt_pmus;
    }

    hisi_ptt_pmus = calloc(*nr_ptts as usize, core::mem::size_of::<*mut perf_pmu>())
        as *mut *mut perf_pmu;
    if hisi_ptt_pmus.is_null() {
        pr_err(c"hisi_ptt alloc failed\n".as_ptr());
        *err = -ENOMEM;
        closedir(dir);
        return hisi_ptt_pmus;
    }

    rewinddir(dir);
    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if !strstr((*dent).d_name.as_ptr(), HISI_PTT_PMU_NAME.as_ptr()).is_null()
            && idx < *nr_ptts
        {
            *hisi_ptt_pmus.offset(idx as isize) = perf_pmus__find((*dent).d_name.as_ptr());
            if !(*hisi_ptt_pmus.offset(idx as isize)).is_null() {
                idx += 1;
            }
        }
    }

    closedir(dir);
    hisi_ptt_pmus
}

unsafe fn find_pmu_for_event(
    pmus: *mut *mut perf_pmu,
    pmu_nr: c_int,
    evsel: *mut evsel,
) -> *mut perf_pmu {
    let mut i: c_int;

    if pmus.is_null() {
        return core::ptr::null_mut();
    }

    i = 0;
    while i < pmu_nr {
        if (*evsel).core.attr.type_ == (*(*pmus.offset(i as isize))).type_ as u32 {
            return *pmus.offset(i as isize);
        }
        i += 1;
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn auxtrace_record__init(
    evlist: *mut evlist,
    err: *mut c_int,
) -> *mut auxtrace_record {
    let mut cs_etm_pmu: *mut perf_pmu = core::ptr::null_mut();
    let mut arm_spe_pmus: *mut *mut perf_pmu = core::ptr::null_mut();
    let mut hisi_ptt_pmus: *mut *mut perf_pmu = core::ptr::null_mut();
    let mut evsel: *mut evsel;
    let mut found_etm: *mut perf_pmu = core::ptr::null_mut();
    let mut found_spe: *mut perf_pmu = core::ptr::null_mut();
    let mut found_ptt: *mut perf_pmu = core::ptr::null_mut();
    let mut auxtrace_event_cnt: c_int = 0;
    let mut nr_spes: c_int = 0;
    let mut nr_ptts: c_int = 0;

    if evlist.is_null() {
        return core::ptr::null_mut();
    }

    cs_etm_pmu = perf_pmus__find(CORESIGHT_ETM_PMU_NAME.as_ptr());
    arm_spe_pmus = find_all_arm_spe_pmus(&mut nr_spes, err);
    hisi_ptt_pmus = find_all_hisi_ptt_pmus(&mut nr_ptts, err);

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if !cs_etm_pmu.is_null() && found_etm.is_null() {
            found_etm = find_pmu_for_event(&mut cs_etm_pmu, 1, evsel);
        }

        if !arm_spe_pmus.is_null() && found_spe.is_null() {
            found_spe = find_pmu_for_event(arm_spe_pmus, nr_spes, evsel);
        }

        if !hisi_ptt_pmus.is_null() && found_ptt.is_null() {
            found_ptt = find_pmu_for_event(hisi_ptt_pmus, nr_ptts, evsel);
        }

        if evlist__is_last(evlist, evsel) {
            break;
        }
        evsel = evlist__next(evsel);
    }

    free(arm_spe_pmus as *mut c_void);
    free(hisi_ptt_pmus as *mut c_void);

    if !found_etm.is_null() {
        auxtrace_event_cnt += 1;
    }

    if !found_spe.is_null() {
        auxtrace_event_cnt += 1;
    }

    if !found_ptt.is_null() {
        auxtrace_event_cnt += 1;
    }

    if auxtrace_event_cnt > 1 {
        pr_err(c"Concurrent AUX trace operation not currently supported\n".as_ptr());
        *err = -EOPNOTSUPP;
        return core::ptr::null_mut();
    }

    if !found_etm.is_null() {
        return cs_etm_record_init(err);
    }

    // Original C condition: #if defined(__aarch64__)
    #[cfg(target_arch = "aarch64")]
    {
        if !found_spe.is_null() {
            return arm_spe_recording_init(err, found_spe);
        }

        if !found_ptt.is_null() {
            return hisi_ptt_recording_init(err, found_ptt);
        }
    }

    /*
     * Clear 'err' even if we haven't found an event - that way perf
     * record can still be used even if tracers aren't present.  The NULL
     * return value will take care of telling the infrastructure HW tracing
     * isn't available.
     */
    *err = 0;
    core::ptr::null_mut()
}

// Original C condition: #if defined(__arm__)
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compat_auxtrace_mmap__read_head(mm: *mut auxtrace_mmap) -> U64 {
    let pc: *mut perf_event_mmap_page = (*mm).userpg;
    let result: U64;

    core::arch::asm!(
        "ldrd    {0}, {1}, [{2}]",
        lateout(reg) result,
        lateout(reg) _,
        in(reg) &(*pc).aux_head,
        options(nostack, readonly),
    );

    result
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compat_auxtrace_mmap__write_tail(
    mm: *mut auxtrace_mmap,
    tail: U64,
) -> c_int {
    let pc: *mut perf_event_mmap_page = (*mm).userpg;

    /* Ensure all reads are done before we write the tail out */
    smp_mb();

    core::arch::asm!(
        "strd    {0}, {1}, [{2}]",
        in(reg) tail as u32,
        in(reg) (tail >> 32) as u32,
        in(reg) &mut (*pc).aux_tail,
        options(nostack),
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
