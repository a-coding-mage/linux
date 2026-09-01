// SPDX-License-Identifier: GPL-2.0
/*
 * Intel dynamic_speed_select -- Enumerate and control features
 * Copyright (c) 2019 Intel Corporation.
 */

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

// Types and constants are supplied by isst.h in the original source.
const ISST_FACT_MAX_BUCKETS: usize = 8;

#[repr(C)]
pub struct isst_id {
    pub pkg: c_int,
    pub die: c_int,
    pub punit: c_int,
    pub cpu: c_int,
}

#[repr(C)]
pub struct isst_pbf_info {
    pub p1_high: c_int,
    pub p1_low: c_int,
    pub core_cpumask_size: c_int,
    pub core_cpumask: *mut cpu_set_t,
    pub t_prochot: c_int,
    pub tdp: c_int,
}

#[repr(C)]
pub struct isst_fact_bucket_info {
    pub hp_cores: c_int,
    pub hp_ratios: [c_int; 8],
}

#[repr(C)]
pub struct isst_fact_info {
    pub bucket_info: *mut isst_fact_bucket_info,
    pub lp_ratios: [c_int; 8],
}

#[repr(C)]
pub struct isst_pkg_ctdp_level_info {
    pub processed: c_int,
    pub level: c_int,
    pub core_cpumask_size: c_int,
    pub core_cpumask: *mut cpu_set_t,
    pub tdp_ratio: c_int,
    pub sse_p1: c_int,
    pub avx2_p1: c_int,
    pub avx512_p1: c_int,
    pub uncore_pm: c_int,
    pub uncore_p0: c_int,
    pub amx_p1: c_int,
    pub uncore_p1: c_int,
    pub uncore1_p1: c_int,
    pub uncore1_pm: c_int,
    pub uncore1_p0: c_int,
    pub mem_freq: c_int,
    pub cooling_type: c_int,
    pub fact_support: c_int,
    pub fact_enabled: c_int,
    pub pbf_support: c_int,
    pub pbf_enabled: c_int,
    pub sst_cp_support: c_int,
    pub sst_cp_enabled: c_int,
    pub pbf_info: isst_pbf_info,
    pub pkg_tdp: c_int,
    pub t_proc_hot: c_int,
    pub trl_ratios: [[c_int; 8]; 8],
    pub trl_cores: c_ulonglong,
    pub fact_info: isst_fact_info,
}

#[repr(C)]
pub struct isst_pkg_ctdp {
    pub processed: c_int,
    pub levels: c_int,
    pub ctdp_level: *mut isst_pkg_ctdp_level_info,
}

#[repr(C)]
pub struct isst_clos_config {
    pub epp: c_int,
    pub clos_prop_prio: c_int,
    pub clos_min: c_int,
    pub clos_max: c_int,
    pub clos_desired: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncat(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn abs(j: c_int) -> c_int;

    fn CPU_COUNT_S(setsize: c_int, set: *mut cpu_set_t) -> c_int;
    fn CPU_ISSET_S(cpu: c_int, setsize: c_int, set: *mut cpu_set_t) -> c_int;

    fn get_topo_max_cpus() -> c_int;
    fn out_format_is_json() -> c_int;
    fn api_version() -> c_int;
    fn isst_get_disp_freq_multiplier() -> c_int;
    fn is_clx_n_platform() -> c_int;
    fn is_emr_platform() -> c_int;
    fn isst_get_trl_max_levels() -> c_int;
    fn isst_get_trl_level_name(level: c_int) -> *const c_char;
    fn get_cpu_count(id: *mut isst_id) -> c_int;
    fn get_output_file() -> *mut FILE;
}

const fn bit(n: c_int) -> c_uint {
    1u32 << (n as u32)
}

unsafe fn printcpulist(str_len: c_int, str_: *mut c_char, mask_size: c_int, cpu_mask: *mut cpu_set_t) {
    let mut i: c_int;
    let mut first: c_int;
    let mut curr_index: c_int;
    let mut index: c_int;

    if CPU_COUNT_S(mask_size, cpu_mask) == 0 {
        snprintf(str_, str_len as usize, c"none".as_ptr());
        return;
    }

    curr_index = 0;
    first = 1;
    i = 0;
    while i < get_topo_max_cpus() {
        if CPU_ISSET_S(i, mask_size, cpu_mask) == 0 {
            i += 1;
            continue;
        }
        if first == 0 {
            index = snprintf(str_.offset(curr_index as isize), (str_len - curr_index) as usize, c",".as_ptr());
            curr_index += index;
            if curr_index >= str_len {
                break;
            }
        }
        index = snprintf(str_.offset(curr_index as isize), (str_len - curr_index) as usize, c"%d".as_ptr(), i);
        curr_index += index;
        if curr_index >= str_len {
            break;
        }
        first = 0;
        i += 1;
    }
}

unsafe fn printcpumask(str_len: c_int, str_: *mut c_char, mask_size: c_int, cpu_mask: *mut cpu_set_t) {
    let mut i: c_int;
    let max_cpus: c_int = get_topo_max_cpus();
    let mask: *mut c_uint;
    let mut size: c_int;
    let mut index: c_int;
    let mut curr_index: c_int;

    size = max_cpus / ((size_of::<c_uint>() * 8) as c_int);
    if max_cpus % ((size_of::<c_uint>() * 8) as c_int) != 0 {
        size += 1;
    }

    mask = calloc(size as usize, size_of::<c_uint>()) as *mut c_uint;
    if mask.is_null() {
        return;
    }

    i = 0;
    while i < max_cpus {
        let mask_index: c_int;
        let bit_index: c_int;

        if CPU_ISSET_S(i, mask_size, cpu_mask) == 0 {
            i += 1;
            continue;
        }

        mask_index = i / ((size_of::<c_uint>() * 8) as c_int);
        bit_index = i % ((size_of::<c_uint>() * 8) as c_int);
        *mask.offset(mask_index as isize) |= bit(bit_index);
        i += 1;
    }

    curr_index = 0;
    i = size - 1;
    while i >= 0 {
        index = snprintf(
            str_.offset(curr_index as isize),
            (str_len - curr_index) as usize,
            c"%08x".as_ptr(),
            *mask.offset(i as isize),
        );
        curr_index += index;
        if curr_index >= str_len {
            break;
        }
        if i != 0 {
            strncat(str_.offset(curr_index as isize), c",".as_ptr(), (str_len - curr_index) as usize);
            curr_index += 1;
        }
        if curr_index >= str_len {
            break;
        }
        i -= 1;
    }

    free(mask as *mut c_void);
}

static mut LAST_LEVEL: c_int = 0;
static mut START: c_int = 0;

unsafe fn format_and_print_txt(outf: *mut FILE, level: c_int, header: *mut c_char, value: *mut c_char) {
    let spaces: *mut c_char = c"  ".as_ptr() as *mut c_char;
    static mut DELIMITERS: [c_char; 256] = [0; 256];
    let mut i: c_int;
    let mut j: c_int = 0;

    if level == 0 {
        return;
    }

    if level == 1 {
        strcpy((&raw mut DELIMITERS).cast::<c_char>(), c" ".as_ptr());
    } else {
        i = 0;
        while i < level - 1 {
            j += snprintf(
                (&raw mut DELIMITERS).cast::<c_char>().offset(j as isize),
                size_of::<[c_char; 256]>() - j as usize,
                c"%s".as_ptr(),
                spaces,
            );
            i += 1;
        }
    }

    if !header.is_null() && !value.is_null() {
        fprintf(outf, c"%s".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>());
        fprintf(outf, c"%s:%s\n".as_ptr(), header, value);
    } else if !header.is_null() {
        fprintf(outf, c"%s".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>());
        fprintf(outf, c"%s\n".as_ptr(), header);
    }
}

unsafe fn format_and_print(outf: *mut FILE, level: c_int, header: *mut c_char, value: *mut c_char) {
    let spaces: *mut c_char = c"  ".as_ptr() as *mut c_char;
    static mut DELIMITERS: [c_char; 256] = [0; 256];
    let mut i: c_int;

    if out_format_is_json() == 0 {
        format_and_print_txt(outf, level, header, value);
        return;
    }

    if level == 0 {
        if !header.is_null() {
            fprintf(outf, c"{".as_ptr());
        } else {
            fprintf(outf, c"\n}\n".as_ptr());
        }
    } else {
        let mut j: c_int = 0;

        i = 0;
        while i < level {
            j += snprintf(
                (&raw mut DELIMITERS).cast::<c_char>().offset(j as isize),
                size_of::<[c_char; 256]>() - j as usize,
                c"%s".as_ptr(),
                spaces,
            );
            i += 1;
        }

        if LAST_LEVEL == level {
            fprintf(outf, c",\n".as_ptr());
        }

        if !value.is_null() {
            if LAST_LEVEL != level {
                fprintf(outf, c"\n".as_ptr());
            }

            fprintf(outf, c"%s\"%s\": ".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>(), header);
            fprintf(outf, c"\"%s\"".as_ptr(), value);
        } else {
            i = LAST_LEVEL - 1;
            while i >= level {
                let mut k: c_int = 0;

                j = i;
                while j > 0 {
                    k += snprintf(
                        (&raw mut DELIMITERS).cast::<c_char>().offset(k as isize),
                        size_of::<[c_char; 256]>() - k as usize,
                        c"%s".as_ptr(),
                        spaces,
                    );
                    j -= 1;
                }
                if i == level && !header.is_null() {
                    fprintf(outf, c"\n%s},".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>());
                } else {
                    fprintf(outf, c"\n%s}".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>());
                }
                i -= 1;
            }
            if abs(LAST_LEVEL - level) < 3 {
                fprintf(outf, c"\n".as_ptr());
            }
            if !header.is_null() {
                fprintf(outf, c"%s\"%s\": {".as_ptr(), (&raw mut DELIMITERS).cast::<c_char>(), header);
            }
        }
    }

    LAST_LEVEL = level;
}

unsafe fn print_package_info(id: *mut isst_id, outf: *mut FILE) -> c_int {
    let mut header: [c_char; 256] = [0; 256];
    let mut level: c_int = 1;

    if out_format_is_json() != 0 {
        if api_version() > 1 {
            if (*id).die < 0 && (*id).cpu < 0 {
                snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"package-%d:die-IO:powerdomain-%d:cpu-None".as_ptr(), (*id).pkg, (*id).punit);
            } else if (*id).cpu < 0 {
                snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"package-%d:die-%d:powerdomain-%d:cpu-None".as_ptr(), (*id).pkg, (*id).die, (*id).punit);
            } else {
                snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"package-%d:die-%d:powerdomain-%d:cpu-%d".as_ptr(), (*id).pkg, (*id).die, (*id).punit, (*id).cpu);
            }
        } else {
            snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"package-%d:die-%d:cpu-%d".as_ptr(), (*id).pkg, (*id).die, (*id).cpu);
        }
        format_and_print(outf, level, header.as_mut_ptr(), std::ptr::null_mut());
        return 1;
    }
    snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"package-%d".as_ptr(), (*id).pkg);
    format_and_print(outf, level, header.as_mut_ptr(), std::ptr::null_mut());
    level += 1;
    if (*id).die < 0 {
        snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"die-IO".as_ptr());
    } else {
        snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"die-%d".as_ptr(), (*id).die);
    }
    format_and_print(outf, level, header.as_mut_ptr(), std::ptr::null_mut());
    level += 1;
    if api_version() > 1 {
        snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"powerdomain-%d".as_ptr(), (*id).punit);
        format_and_print(outf, level, header.as_mut_ptr(), std::ptr::null_mut());
        level += 1;
    }

    if (*id).cpu < 0 {
        snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"cpu-None".as_ptr());
    } else {
        snprintf(header.as_mut_ptr(), size_of::<[c_char; 256]>(), c"cpu-%d".as_ptr(), (*id).cpu);
    }

    format_and_print(outf, level, header.as_mut_ptr(), std::ptr::null_mut());

    level
}

unsafe fn _isst_pbf_display_information(
    _id: *mut isst_id,
    outf: *mut FILE,
    _level: c_int,
    pbf_info: *mut isst_pbf_info,
    disp_level: c_int,
) {
    static mut HEADER: [c_char; 256] = [0; 256];
    static mut VALUE: [c_char; 1024] = [0; 1024];

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"speed-select-base-freq-properties".as_ptr());
    format_and_print(outf, disp_level, (&raw mut HEADER).cast::<c_char>(), std::ptr::null_mut());

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"high-priority-base-frequency(MHz)".as_ptr());
    snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*pbf_info).p1_high * isst_get_disp_freq_multiplier());
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"high-priority-cpu-mask".as_ptr());
    printcpumask(1024, (&raw mut VALUE).cast::<c_char>(), (*pbf_info).core_cpumask_size, (*pbf_info).core_cpumask);
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"high-priority-cpu-list".as_ptr());
    printcpulist(1024, (&raw mut VALUE).cast::<c_char>(), (*pbf_info).core_cpumask_size, (*pbf_info).core_cpumask);
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"low-priority-base-frequency(MHz)".as_ptr());
    snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*pbf_info).p1_low * isst_get_disp_freq_multiplier());
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

    if is_clx_n_platform() != 0 {
        return;
    }

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"tjunction-temperature(C)".as_ptr());
    snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*pbf_info).t_prochot);
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

    snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"thermal-design-power(W)".as_ptr());
    snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*pbf_info).tdp);
    format_and_print(outf, disp_level + 1, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
}

unsafe fn _isst_fact_display_information(
    id: *mut isst_id,
    outf: *mut FILE,
    _level: c_int,
    fact_bucket: c_int,
    fact_avx: c_int,
    fact_info: *mut isst_fact_info,
    base_level: c_int,
) {
    let bucket_info: *mut isst_fact_bucket_info = (*fact_info).bucket_info;
    let trl_max_levels: c_int = isst_get_trl_max_levels();
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let mut print: c_int = 0;
    let mut j: c_int;

    j = 0;
    while j < ISST_FACT_MAX_BUCKETS as c_int {
        if fact_bucket != 0xff && fact_bucket != j {
            j += 1;
            continue;
        }

        /* core count must be valid for CPU power domain */
        if (*bucket_info.offset(j as isize)).hp_cores == 0 && (*id).cpu >= 0 {
            break;
        }

        print = 1;
        j += 1;
    }
    if print == 0 {
        fprintf(stderr, c"Invalid bucket\n".as_ptr());
        return;
    }

    snprintf(header.as_mut_ptr(), 256, c"speed-select-turbo-freq-properties".as_ptr());
    format_and_print(outf, base_level, header.as_mut_ptr(), std::ptr::null_mut());
    j = 0;
    while j < ISST_FACT_MAX_BUCKETS as c_int {
        let mut i: c_int;

        if fact_bucket != 0xff && fact_bucket != j {
            j += 1;
            continue;
        }

        if (*bucket_info.offset(j as isize)).hp_cores == 0 {
            break;
        }

        snprintf(header.as_mut_ptr(), 256, c"bucket-%d".as_ptr(), j);
        format_and_print(outf, base_level + 1, header.as_mut_ptr(), std::ptr::null_mut());

        snprintf(header.as_mut_ptr(), 256, c"high-priority-cores-count".as_ptr());
        snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), (*bucket_info.offset(j as isize)).hp_cores);
        format_and_print(outf, base_level + 2, header.as_mut_ptr(), value.as_mut_ptr());
        i = 0;
        while i < trl_max_levels {
            if (*bucket_info.offset(j as isize)).hp_ratios[i as usize] == 0
                || (fact_avx != 0xFF && (fact_avx & (1 << i)) == 0)
            {
                i += 1;
                continue;
            }
            if i == 0 && api_version() == 1 && is_emr_platform() == 0 {
                snprintf(header.as_mut_ptr(), 256, c"high-priority-max-frequency(MHz)".as_ptr());
            } else {
                snprintf(header.as_mut_ptr(), 256, c"high-priority-max-%s-frequency(MHz)".as_ptr(), isst_get_trl_level_name(i));
            }
            snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), (*bucket_info.offset(j as isize)).hp_ratios[i as usize] * isst_get_disp_freq_multiplier());
            format_and_print(outf, base_level + 2, header.as_mut_ptr(), value.as_mut_ptr());
            i += 1;
        }
        j += 1;
    }
    snprintf(header.as_mut_ptr(), 256, c"speed-select-turbo-freq-clip-frequencies".as_ptr());
    format_and_print(outf, base_level + 1, header.as_mut_ptr(), std::ptr::null_mut());

    j = 0;
    while j < trl_max_levels {
        if (*fact_info).lp_ratios[j as usize] == 0 {
            j += 1;
            continue;
        }

        /* No AVX level name for SSE to be consistent with previous formatting */
        if j == 0 && api_version() == 1 && is_emr_platform() == 0 {
            snprintf(header.as_mut_ptr(), 256, c"low-priority-max-frequency(MHz)".as_ptr());
        } else {
            snprintf(header.as_mut_ptr(), 256, c"low-priority-max-%s-frequency(MHz)".as_ptr(), isst_get_trl_level_name(j));
        }
        snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), (*fact_info).lp_ratios[j as usize] * isst_get_disp_freq_multiplier());
        format_and_print(outf, base_level + 2, header.as_mut_ptr(), value.as_mut_ptr());
        j += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn isst_ctdp_display_core_info(
    id: *mut isst_id,
    outf: *mut FILE,
    prefix: *mut c_char,
    val: c_uint,
    str0: *mut c_char,
    str1: *mut c_char,
) {
    let mut value: [c_char; 256] = [0; 256];
    let mut level: c_int = print_package_info(id, outf);

    level += 1;

    if !str0.is_null() && val == 0 {
        snprintf(value.as_mut_ptr(), 256, c"%s".as_ptr(), str0);
    } else if !str1.is_null() && val != 0 {
        snprintf(value.as_mut_ptr(), 256, c"%s".as_ptr(), str1);
    } else {
        snprintf(value.as_mut_ptr(), 256, c"%u".as_ptr(), val);
    }
    format_and_print(outf, level, prefix, value.as_mut_ptr());

    format_and_print(outf, 1, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_ctdp_display_information(
    id: *mut isst_id,
    outf: *mut FILE,
    tdp_level: c_int,
    pkg_dev: *mut isst_pkg_ctdp,
) {
    static mut HEADER: [c_char; 256] = [0; 256];
    static mut VALUE: [c_char; 1024] = [0; 1024];
    static mut LEVEL: c_int = 0;
    let trl_max_levels: c_int = isst_get_trl_max_levels();
    let mut i: c_int;

    if (*pkg_dev).processed != 0 {
        LEVEL = print_package_info(id, outf);
    }

    i = 0;
    while i <= (*pkg_dev).levels {
        let ctdp_level: *mut isst_pkg_ctdp_level_info;
        let mut j: c_int;
        let mut k: c_int;

        ctdp_level = (*pkg_dev).ctdp_level.offset(i as isize);
        if (*ctdp_level).processed == 0 {
            i += 1;
            continue;
        }

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"perf-profile-level-%d".as_ptr(), (*ctdp_level).level);
        format_and_print(outf, LEVEL + 1, (&raw mut HEADER).cast::<c_char>(), std::ptr::null_mut());

        if (*id).cpu >= 0 {
            snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"cpu-count".as_ptr());
            j = get_cpu_count(id);
            snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), j);
            format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

            j = CPU_COUNT_S((*ctdp_level).core_cpumask_size, (*ctdp_level).core_cpumask);
            if j != 0 {
                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"enable-cpu-count".as_ptr());
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), j);
                format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
            }

            if (*ctdp_level).core_cpumask_size != 0 {
                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"enable-cpu-mask".as_ptr());
                printcpumask(1024, (&raw mut VALUE).cast::<c_char>(), (*ctdp_level).core_cpumask_size, (*ctdp_level).core_cpumask);
                format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"enable-cpu-list".as_ptr());
                printcpulist(1024, (&raw mut VALUE).cast::<c_char>(), (*ctdp_level).core_cpumask_size, (*ctdp_level).core_cpumask);
                format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
            }
        }

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"thermal-design-power-ratio".as_ptr());
        snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*ctdp_level).tdp_ratio);
        format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"base-frequency(MHz)".as_ptr());
        if (*ctdp_level).sse_p1 == 0 {
            (*ctdp_level).sse_p1 = (*ctdp_level).tdp_ratio;
        }
        snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*ctdp_level).sse_p1 * isst_get_disp_freq_multiplier());
        format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

        macro_rules! print_int_if {
            ($field:ident, $name:literal, $mult:expr) => {
                if (*ctdp_level).$field != 0 {
                    snprintf((&raw mut HEADER).cast::<c_char>(), 256, concat!($name, "\0").as_ptr() as *const c_char);
                    snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*ctdp_level).$field * $mult);
                    format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
                }
            };
        }

        print_int_if!(avx2_p1, "base-frequency-avx2(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(avx512_p1, "base-frequency-avx512(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore_pm, "uncore-frequency-min(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore_p0, "uncore-frequency-max(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(amx_p1, "base-frequency-amx(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore_p1, "uncore-frequency-base(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore1_p1, "uncore-1-frequency-base(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore1_pm, "uncore-1-frequency-min(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(uncore1_p0, "uncore-1-frequency-max(MHz)", isst_get_disp_freq_multiplier());
        print_int_if!(mem_freq, "max-mem-frequency(MHz)", 1);

        if api_version() > 1 {
            snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"cooling_type".as_ptr());
            snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*ctdp_level).cooling_type);
            format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
        }

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"speed-select-turbo-freq".as_ptr());
        if (*ctdp_level).fact_support != 0 {
            if (*ctdp_level).fact_enabled != 0 {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"enabled".as_ptr());
            } else {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"disabled".as_ptr());
            }
        } else {
            snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"unsupported".as_ptr());
        }
        format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"speed-select-base-freq".as_ptr());
        if (*ctdp_level).pbf_support != 0 {
            if (*ctdp_level).pbf_enabled != 0 {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"enabled".as_ptr());
            } else {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"disabled".as_ptr());
            }
        } else {
            snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"unsupported".as_ptr());
        }
        format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

        snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"speed-select-core-power".as_ptr());
        if (*ctdp_level).sst_cp_support != 0 {
            if (*ctdp_level).sst_cp_enabled != 0 {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"enabled".as_ptr());
            } else {
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"disabled".as_ptr());
            }
        } else {
            snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"unsupported".as_ptr());
        }
        format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

        if is_clx_n_platform() != 0 {
            if (*ctdp_level).pbf_support != 0 {
                _isst_pbf_display_information(id, outf, tdp_level, &mut (*ctdp_level).pbf_info, LEVEL + 2);
            }
            i += 1;
            continue;
        }

        print_int_if!(pkg_tdp, "thermal-design-power(W)", 1);
        print_int_if!(t_proc_hot, "tjunction-max(C)", 1);

        k = 0;
        while k < trl_max_levels {
            if (*ctdp_level).trl_ratios[k as usize][0] == 0 {
                k += 1;
                continue;
            }

            snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"turbo-ratio-limits-%s".as_ptr(), isst_get_trl_level_name(k));
            format_and_print(outf, LEVEL + 2, (&raw mut HEADER).cast::<c_char>(), std::ptr::null_mut());

            j = 0;
            while j < 8 {
                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"bucket-%d".as_ptr(), j);
                format_and_print(outf, LEVEL + 3, (&raw mut HEADER).cast::<c_char>(), std::ptr::null_mut());

                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"core-count".as_ptr());

                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%llu".as_ptr(), ((*ctdp_level).trl_cores >> (j * 8)) & 0xff);
                format_and_print(outf, LEVEL + 4, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());

                snprintf((&raw mut HEADER).cast::<c_char>(), 256, c"max-turbo-frequency(MHz)".as_ptr());
                snprintf((&raw mut VALUE).cast::<c_char>(), 1024, c"%d".as_ptr(), (*ctdp_level).trl_ratios[k as usize][j as usize] * isst_get_disp_freq_multiplier());
                format_and_print(outf, LEVEL + 4, (&raw mut HEADER).cast::<c_char>(), (&raw mut VALUE).cast::<c_char>());
                j += 1;
            }
            k += 1;
        }

        if (*ctdp_level).pbf_support != 0 {
            _isst_pbf_display_information(id, outf, i, &mut (*ctdp_level).pbf_info, LEVEL + 2);
        }
        if (*ctdp_level).fact_support != 0 {
            _isst_fact_display_information(id, outf, i, 0xff, 0xff, &mut (*ctdp_level).fact_info, LEVEL + 2);
        }
        i += 1;
    }

    format_and_print(outf, 1, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_ctdp_display_information_start(outf: *mut FILE) {
    LAST_LEVEL = 0;
    format_and_print(outf, 0, c"start".as_ptr() as *mut c_char, std::ptr::null_mut());
    START = 1;
}

#[no_mangle]
pub unsafe extern "C" fn isst_ctdp_display_information_end(outf: *mut FILE) {
    format_and_print(outf, 0, std::ptr::null_mut(), std::ptr::null_mut());
    START = 0;
}

#[no_mangle]
pub unsafe extern "C" fn isst_pbf_display_information(
    id: *mut isst_id,
    outf: *mut FILE,
    level: c_int,
    pbf_info: *mut isst_pbf_info,
) {
    let _level: c_int = print_package_info(id, outf);
    _isst_pbf_display_information(id, outf, level, pbf_info, _level + 1);
    format_and_print(outf, 1, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_fact_display_information(
    id: *mut isst_id,
    outf: *mut FILE,
    level: c_int,
    fact_bucket: c_int,
    fact_avx: c_int,
    fact_info: *mut isst_fact_info,
) {
    let _level: c_int = print_package_info(id, outf);
    _isst_fact_display_information(id, outf, level, fact_bucket, fact_avx, fact_info, _level + 1);
    format_and_print(outf, 1, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_display_information(
    id: *mut isst_id,
    outf: *mut FILE,
    clos: c_int,
    clos_config: *mut isst_clos_config,
) {
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let level: c_int = print_package_info(id, outf);

    snprintf(header.as_mut_ptr(), 256, c"core-power".as_ptr());
    format_and_print(outf, level + 1, header.as_mut_ptr(), std::ptr::null_mut());

    snprintf(header.as_mut_ptr(), 256, c"clos".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), clos);
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"epp".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), (*clos_config).epp);
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"clos-proportional-priority".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), (*clos_config).clos_prop_prio);
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"clos-min".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d MHz".as_ptr(), (*clos_config).clos_min * isst_get_disp_freq_multiplier());
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"clos-max".as_ptr());
    if (*clos_config).clos_max * isst_get_disp_freq_multiplier() == 25500 {
        snprintf(value.as_mut_ptr(), 256, c"Max Turbo frequency".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"%d MHz".as_ptr(), (*clos_config).clos_max * isst_get_disp_freq_multiplier());
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"clos-desired".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d MHz".as_ptr(), (*clos_config).clos_desired * isst_get_disp_freq_multiplier());
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    format_and_print(outf, level, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_display_clos_information(
    id: *mut isst_id,
    outf: *mut FILE,
    clos_enable: c_int,
    type_: c_int,
    state: c_int,
    cap: c_int,
) {
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let level: c_int = print_package_info(id, outf);

    snprintf(header.as_mut_ptr(), 256, c"core-power".as_ptr());
    format_and_print(outf, level + 1, header.as_mut_ptr(), std::ptr::null_mut());

    snprintf(header.as_mut_ptr(), 256, c"support-status".as_ptr());
    if cap != 0 {
        snprintf(value.as_mut_ptr(), 256, c"supported".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"unsupported".as_ptr());
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"enable-status".as_ptr());
    if state != 0 {
        snprintf(value.as_mut_ptr(), 256, c"enabled".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"disabled".as_ptr());
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"clos-enable-status".as_ptr());
    if clos_enable != 0 {
        snprintf(value.as_mut_ptr(), 256, c"enabled".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"disabled".as_ptr());
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    snprintf(header.as_mut_ptr(), 256, c"priority-type".as_ptr());
    if type_ != 0 {
        snprintf(value.as_mut_ptr(), 256, c"ordered".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"proportional".as_ptr());
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    format_and_print(outf, level, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_display_assoc_information(id: *mut isst_id, outf: *mut FILE, clos: c_int) {
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let level: c_int = print_package_info(id, outf);

    snprintf(header.as_mut_ptr(), 256, c"get-assoc".as_ptr());
    format_and_print(outf, level + 1, header.as_mut_ptr(), std::ptr::null_mut());

    snprintf(header.as_mut_ptr(), 256, c"clos".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"%d".as_ptr(), clos);
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    format_and_print(outf, level, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_display_result(
    id: *mut isst_id,
    outf: *mut FILE,
    feature: *mut c_char,
    cmd: *mut c_char,
    result: c_int,
) {
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let level: c_int = print_package_info(id, outf);

    snprintf(header.as_mut_ptr(), 256, c"%s".as_ptr(), feature);
    format_and_print(outf, level + 1, header.as_mut_ptr(), std::ptr::null_mut());
    snprintf(header.as_mut_ptr(), 256, c"%s".as_ptr(), cmd);
    if result == 0 {
        snprintf(value.as_mut_ptr(), 256, c"success".as_ptr());
    } else {
        snprintf(value.as_mut_ptr(), 256, c"failed(error %d)".as_ptr(), result);
    }
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    format_and_print(outf, level, std::ptr::null_mut(), std::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn isst_display_error_info_message(error: c_int, msg: *mut c_char, arg_valid: c_int, arg: c_int) {
    let outf: *mut FILE = get_output_file();
    static mut ERROR_INDEX: c_int = 0;
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];

    if out_format_is_json() == 0 {
        if arg_valid != 0 {
            snprintf(value.as_mut_ptr(), 256, c"%s %d".as_ptr(), msg, arg);
        } else {
            snprintf(value.as_mut_ptr(), 256, c"%s".as_ptr(), msg);
        }

        if error != 0 {
            fprintf(outf, c"Error: %s\n".as_ptr(), value.as_mut_ptr());
        } else {
            fprintf(outf, c"Information: %s\n".as_ptr(), value.as_mut_ptr());
        }
        return;
    }

    if START == 0 {
        format_and_print(outf, 0, c"start".as_ptr() as *mut c_char, std::ptr::null_mut());
    }

    if error != 0 {
        snprintf(header.as_mut_ptr(), 256, c"Error%d".as_ptr(), ERROR_INDEX);
        ERROR_INDEX += 1;
    } else {
        snprintf(header.as_mut_ptr(), 256, c"Information:%d".as_ptr(), ERROR_INDEX);
        ERROR_INDEX += 1;
    }
    format_and_print(outf, 1, header.as_mut_ptr(), std::ptr::null_mut());

    snprintf(header.as_mut_ptr(), 256, c"message".as_ptr());
    if arg_valid != 0 {
        snprintf(value.as_mut_ptr(), 256, c"%s %d".as_ptr(), msg, arg);
    } else {
        snprintf(value.as_mut_ptr(), 256, c"%s".as_ptr(), msg);
    }

    format_and_print(outf, 2, header.as_mut_ptr(), value.as_mut_ptr());
    format_and_print(outf, 1, std::ptr::null_mut(), std::ptr::null_mut());
    if START == 0 {
        format_and_print(outf, 0, std::ptr::null_mut(), std::ptr::null_mut());
    }
}

#[no_mangle]
pub unsafe extern "C" fn isst_trl_display_information(id: *mut isst_id, outf: *mut FILE, trl: c_ulonglong) {
    let mut header: [c_char; 256] = [0; 256];
    let mut value: [c_char; 256] = [0; 256];
    let level: c_int = print_package_info(id, outf);

    snprintf(header.as_mut_ptr(), 256, c"get-trl".as_ptr());
    format_and_print(outf, level + 1, header.as_mut_ptr(), std::ptr::null_mut());

    snprintf(header.as_mut_ptr(), 256, c"trl".as_ptr());
    snprintf(value.as_mut_ptr(), 256, c"0x%llx".as_ptr(), trl);
    format_and_print(outf, level + 2, header.as_mut_ptr(), value.as_mut_ptr());

    format_and_print(outf, level, std::ptr::null_mut(), std::ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
