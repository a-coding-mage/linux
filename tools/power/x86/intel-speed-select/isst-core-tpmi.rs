// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Enumerate and control features for TPMI Interface
 * Copyright (c) 2022 Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::MaybeUninit;

// Dependencies from <linux/isst_if.h> and "isst.h" are expected to be supplied
// by the surrounding translated crate.
use crate::*;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

const FEATURE_ENABLE_WAIT_US: c_uint = 1000;
const FEATURE_ENABLE_RETRIES: c_int = 5;

#[inline]
const fn BIT(nr: c_int) -> c_int {
    1_i32 << nr
}

#[no_mangle]
pub unsafe extern "C" fn tpmi_process_ioctl(ioctl_no: c_int, info: *mut c_void) -> c_int {
    let pathname = b"/dev/isst_interface\0".as_ptr() as *const c_char;
    let fd: c_int;

    if is_debug_enabled() != 0 {
        debug_printf(b"Issue IOCTL: \0".as_ptr() as *const c_char);
        match ioctl_no {
            ISST_IF_CORE_POWER_STATE => debug_printf(b"ISST_IF_CORE_POWER_STATE\n\0".as_ptr() as *const c_char),
            ISST_IF_CLOS_PARAM => debug_printf(b"ISST_IF_CLOS_PARAM\n\0".as_ptr() as *const c_char),
            ISST_IF_CLOS_ASSOC => debug_printf(b"ISST_IF_CLOS_ASSOC\n\0".as_ptr() as *const c_char),
            ISST_IF_PERF_LEVELS => debug_printf(b"ISST_IF_PERF_LEVELS\n\0".as_ptr() as *const c_char),
            ISST_IF_PERF_SET_LEVEL => debug_printf(b"ISST_IF_PERF_SET_LEVEL\n\0".as_ptr() as *const c_char),
            ISST_IF_PERF_SET_FEATURE => debug_printf(b"ISST_IF_PERF_SET_FEATURE\n\0".as_ptr() as *const c_char),
            ISST_IF_GET_PERF_LEVEL_INFO => debug_printf(b"ISST_IF_GET_PERF_LEVEL_INFO\n\0".as_ptr() as *const c_char),
            ISST_IF_GET_PERF_LEVEL_CPU_MASK => debug_printf(b"ISST_IF_GET_PERF_LEVEL_CPU_MASK\n\0".as_ptr() as *const c_char),
            ISST_IF_GET_BASE_FREQ_INFO => debug_printf(b"ISST_IF_GET_BASE_FREQ_INFO\n\0".as_ptr() as *const c_char),
            ISST_IF_GET_BASE_FREQ_CPU_MASK => debug_printf(b"ISST_IF_GET_BASE_FREQ_CPU_MASK\n\0".as_ptr() as *const c_char),
            ISST_IF_GET_TURBO_FREQ_INFO => debug_printf(b"ISST_IF_GET_TURBO_FREQ_INFO\n\0".as_ptr() as *const c_char),
            ISST_IF_COUNT_TPMI_INSTANCES => debug_printf(b"ISST_IF_COUNT_TPMI_INSTANCES\n\0".as_ptr() as *const c_char),
            _ => debug_printf(b"%d\n\0".as_ptr() as *const c_char, ioctl_no),
        };
    }

    fd = open(pathname, O_RDWR);
    if fd < 0 {
        return -1;
    }

    if ioctl(fd, ioctl_no, info) == -1 {
        debug_printf(b"IOCTL %d Failed\n\0".as_ptr() as *const c_char, ioctl_no);
        close(fd);
        return -1;
    }

    close(fd);

    0
}

unsafe extern "C" fn tpmi_get_disp_freq_multiplier() -> c_int {
    1
}

unsafe extern "C" fn tpmi_get_trl_max_levels() -> c_int {
    TRL_MAX_LEVELS
}

unsafe extern "C" fn tpmi_get_trl_level_name(level: c_int) -> *mut c_char {
    match level {
        0 => b"level-0\0".as_ptr() as *mut c_char,
        1 => b"level-1\0".as_ptr() as *mut c_char,
        2 => b"level-2\0".as_ptr() as *mut c_char,
        3 => b"level-3\0".as_ptr() as *mut c_char,
        4 => b"level-4\0".as_ptr() as *mut c_char,
        5 => b"level-5\0".as_ptr() as *mut c_char,
        6 => b"level-6\0".as_ptr() as *mut c_char,
        7 => b"level-7\0".as_ptr() as *mut c_char,
        _ => core::ptr::null_mut(),
    }
}

unsafe extern "C" fn tpmi_update_platform_param(_param: isst_platform_param, _value: c_int) {
    /* No params need to be updated for now */
}

unsafe extern "C" fn tpmi_is_punit_valid(id: *mut isst_id) -> c_int {
    let mut info = MaybeUninit::<isst_tpmi_instance_count>::uninit();
    let info_p = info.as_mut_ptr();
    let ret: c_int;

    if (*id).punit < 0 {
        return 0;
    }

    (*info_p).socket_id = (*id).pkg;
    ret = tpmi_process_ioctl(ISST_IF_COUNT_TPMI_INSTANCES, info_p as *mut c_void);
    if ret == -1 {
        return 0;
    }

    let info = info.assume_init();
    if (info.valid_mask & BIT((*id).punit)) != 0 {
        return 1;
    }

    0
}

unsafe extern "C" fn tpmi_read_pm_config(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int {
    let mut info = MaybeUninit::<isst_core_power>::uninit();
    let info_p = info.as_mut_ptr();
    let ret: c_int;

    (*info_p).get_set = 0;
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    ret = tpmi_process_ioctl(ISST_IF_CORE_POWER_STATE, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }

    let info = info.assume_init();
    *cp_state = info.enable;
    *cp_cap = info.supported;

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpmi_get_config_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_info>::uninit();
    let info_p = info.as_mut_ptr();
    let ret: c_int;

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;

    ret = tpmi_process_ioctl(ISST_IF_PERF_LEVELS, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }

    let info = info.assume_init();
    (*pkg_dev).version = info.feature_rev;
    (*pkg_dev).levels = info.max_level;
    (*pkg_dev).locked = info.locked;
    (*pkg_dev).current_level = info.current_level;
    (*pkg_dev).locked = info.locked;
    (*pkg_dev).enabled = info.enabled;

    0
}

unsafe extern "C" fn tpmi_get_ctdp_control(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut core_power_info = MaybeUninit::<isst_core_power>::uninit();
    let core_power_info_p = core_power_info.as_mut_ptr();
    let mut info = MaybeUninit::<isst_perf_level_info>::uninit();
    let info_p = info.as_mut_ptr();
    let level_mask: c_int;
    let ret: c_int;

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;

    ret = tpmi_process_ioctl(ISST_IF_PERF_LEVELS, info_p as *mut c_void);
    if ret == -1 {
        return -1;
    }
    let info = info.assume_init();

    if config_index != 0xff {
        level_mask = 1 << config_index;
    } else {
        level_mask = config_index;
    }

    if (info.level_mask & level_mask) == 0 {
        return -1;
    }

    if api_version() > 2 {
        (*ctdp_level).fact_support = info.sst_tf_support & BIT(config_index);
        (*ctdp_level).pbf_support = info.sst_bf_support & BIT(config_index);
    } else {
        (*ctdp_level).fact_support = info.sst_tf_support;
        (*ctdp_level).pbf_support = info.sst_bf_support;
    }

    (*ctdp_level).fact_enabled = ((info.feature_state & BIT(1)) != 0) as c_int;
    (*ctdp_level).pbf_enabled = ((info.feature_state & BIT(0)) != 0) as c_int;

    (*core_power_info_p).get_set = 0;
    (*core_power_info_p).socket_id = (*id).pkg;
    (*core_power_info_p).power_domain_id = (*id).punit;

    let ret = tpmi_process_ioctl(ISST_IF_CORE_POWER_STATE, core_power_info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let core_power_info = core_power_info.assume_init();

    (*ctdp_level).sst_cp_support = core_power_info.supported;
    (*ctdp_level).sst_cp_enabled = core_power_info.enable;

    debug_printf(
        b"cpu:%d CONFIG_TDP_GET_TDP_CONTROL fact_support:%d pbf_support: %d fact_enabled:%d pbf_enabled:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        (*ctdp_level).fact_support,
        (*ctdp_level).pbf_support,
        (*ctdp_level).fact_enabled,
        (*ctdp_level).pbf_enabled,
    );

    0
}

unsafe extern "C" fn tpmi_get_tdp_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut fabric_info = MaybeUninit::<isst_perf_level_fabric_info>::uninit();
    let fabric_info_p = fabric_info.as_mut_ptr();
    let mut info = MaybeUninit::<isst_perf_level_data_info>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_INFO, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    (*ctdp_level).pkg_tdp = info.thermal_design_power_w;
    (*ctdp_level).tdp_ratio = info.tdp_ratio;
    (*ctdp_level).sse_p1 = info.base_freq_mhz;
    (*ctdp_level).avx2_p1 = info.base_freq_avx2_mhz;
    (*ctdp_level).avx512_p1 = info.base_freq_avx512_mhz;
    (*ctdp_level).amx_p1 = info.base_freq_amx_mhz;
    (*ctdp_level).t_proc_hot = info.tjunction_max_c;
    (*ctdp_level).mem_freq = info.max_memory_freq_mhz;
    (*ctdp_level).cooling_type = info.cooling_type;
    (*ctdp_level).uncore_p0 = info.p0_fabric_freq_mhz;
    (*ctdp_level).uncore_p1 = info.p1_fabric_freq_mhz;
    (*ctdp_level).uncore_pm = info.pm_fabric_freq_mhz;

    (*fabric_info_p).socket_id = (*id).pkg;
    (*fabric_info_p).power_domain_id = (*id).punit;
    (*fabric_info_p).level = config_index;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_FABRIC_INFO, fabric_info_p as *mut c_void);
    if ret != -1 {
        let fabric_info = fabric_info.assume_init();
        (*ctdp_level).uncore1_p0 = fabric_info.p0_fabric_freq_mhz[1];
        (*ctdp_level).uncore1_p1 = fabric_info.p1_fabric_freq_mhz[1];
        (*ctdp_level).uncore1_pm = fabric_info.pm_fabric_freq_mhz[1];
    }

    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_TDP_INFO tdp_ratio:%d pkg_tdp:%d ctdp_level->t_proc_hot:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        (*ctdp_level).tdp_ratio,
        (*ctdp_level).pkg_tdp,
        (*ctdp_level).t_proc_hot,
    );

    0
}

unsafe extern "C" fn tpmi_get_pwr_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    /* TBD */
    (*ctdp_level).pkg_max_power = 0;
    (*ctdp_level).pkg_min_power = 0;

    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_PWR_INFO pkg_max_power:%d pkg_min_power:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        (*ctdp_level).pkg_max_power,
        (*ctdp_level).pkg_min_power,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpmi_get_coremask_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_cpu_mask>::uninit();
    let info_p = info.as_mut_ptr();
    let mut cpu_count: c_int = 0;

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;
    (*info_p).punit_cpu_map = 1;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_CPU_MASK, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    set_cpu_mask_from_punit_coremask(
        id,
        info.mask,
        (*ctdp_level).core_cpumask_size,
        (*ctdp_level).core_cpumask,
        &mut cpu_count,
    );
    (*ctdp_level).cpu_count = cpu_count;

    debug_printf(
        b"cpu:%d ctdp:%d core_mask ino cpu count:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        (*ctdp_level).cpu_count,
    );

    0
}

unsafe extern "C" fn tpmi_get_get_trls(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_data_info>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_INFO, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let mut info = info.assume_init();

    if info.max_buckets > TRL_MAX_BUCKETS {
        info.max_buckets = TRL_MAX_BUCKETS;
    }

    if info.max_trl_levels > TRL_MAX_LEVELS {
        info.max_trl_levels = TRL_MAX_LEVELS;
    }

    let mut i = 0;
    while i < info.max_trl_levels {
        let mut j = 0;
        while j < info.max_buckets {
            (*ctdp_level).trl_ratios[i as usize][j as usize] = info.trl_freq_mhz[i as usize][j as usize];
            j += 1;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn tpmi_get_get_trl(
    id: *mut isst_id,
    config_index: c_int,
    _level: c_int,
    trl: *mut c_int,
) -> c_int {
    let mut ctdp_level = MaybeUninit::<isst_pkg_ctdp_level_info>::uninit();

    let ret = tpmi_get_get_trls(id, config_index, ctdp_level.as_mut_ptr());
    if ret != 0 {
        return ret;
    }
    let ctdp_level = ctdp_level.assume_init();

    /* FIX ME: Just return for level 0 */
    let mut i = 0;
    while i < 8 {
        *trl.add(i as usize) = ctdp_level.trl_ratios[0][i as usize];
        i += 1;
    }

    0
}

unsafe extern "C" fn tpmi_get_trl_bucket_info(
    id: *mut isst_id,
    config_index: c_int,
    buckets_info: *mut c_ulonglong,
) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_data_info>::uninit();
    let info_p = info.as_mut_ptr();
    let mask = buckets_info as *mut u8;

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_INFO, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let mut info = info.assume_init();

    if info.max_buckets > TRL_MAX_BUCKETS {
        info.max_buckets = TRL_MAX_BUCKETS;
    }

    let mut i = 0;
    while i < info.max_buckets {
        *mask.add(i as usize) = info.bucket_core_counts[i as usize];
        i += 1;
    }

    debug_printf(
        b"cpu:%d TRL bucket info: 0x%llx\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        *buckets_info,
    );

    0
}

unsafe extern "C" fn tpmi_set_tdp_level(id: *mut isst_id, tdp_level: c_int) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_control>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = tdp_level;

    let ret = tpmi_process_ioctl(ISST_IF_PERF_SET_LEVEL, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }

    0
}

unsafe extern "C" fn _pbf_get_coremask_info(
    id: *mut isst_id,
    config_index: c_int,
    pbf_info: *mut isst_pbf_info,
) -> c_int {
    let mut info = MaybeUninit::<isst_perf_level_cpu_mask>::uninit();
    let info_p = info.as_mut_ptr();
    let mut cpu_count: c_int = 0;

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;
    (*info_p).punit_cpu_map = 1;

    let ret = tpmi_process_ioctl(ISST_IF_GET_BASE_FREQ_CPU_MASK, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    set_cpu_mask_from_punit_coremask(
        id,
        info.mask,
        (*pbf_info).core_cpumask_size,
        (*pbf_info).core_cpumask,
        &mut cpu_count,
    );

    debug_printf(
        b"cpu:%d ctdp:%d pbf core_mask info cpu count:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        cpu_count,
    );

    0
}

unsafe extern "C" fn tpmi_get_pbf_info(id: *mut isst_id, level: c_int, pbf_info: *mut isst_pbf_info) -> c_int {
    let mut info = MaybeUninit::<isst_base_freq_info>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = level;

    let ret = tpmi_process_ioctl(ISST_IF_GET_BASE_FREQ_INFO, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    (*pbf_info).p1_low = info.low_base_freq_mhz;
    (*pbf_info).p1_high = info.high_base_freq_mhz;
    (*pbf_info).tdp = info.thermal_design_power_w;
    (*pbf_info).t_prochot = info.tjunction_max_c;

    debug_printf(
        b"cpu:%d ctdp:%d pbf info:%d:%d:%d:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        level,
        (*pbf_info).p1_low,
        (*pbf_info).p1_high,
        (*pbf_info).tdp,
        (*pbf_info).t_prochot,
    );

    _pbf_get_coremask_info(id, level, pbf_info)
}

unsafe extern "C" fn tpmi_set_pbf_fact_status(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int {
    let mut pkg_dev = MaybeUninit::<isst_pkg_ctdp>::uninit();
    let mut ctdp_level = MaybeUninit::<isst_pkg_ctdp_level_info>::uninit();
    let current_level: c_int;
    let mut info = MaybeUninit::<isst_perf_feature_control>::uninit();
    let info_p = info.as_mut_ptr();
    let mut ret: c_int;
    let mut i: c_int;

    ret = isst_get_ctdp_levels(id, pkg_dev.as_mut_ptr());
    if ret != 0 {
        debug_printf(b"cpu:%d No support for dynamic ISST\n\0".as_ptr() as *const c_char, (*id).cpu);
    }

    current_level = pkg_dev.assume_init().current_level;

    ret = isst_get_ctdp_control(id, current_level, ctdp_level.as_mut_ptr());
    if ret != 0 {
        return ret;
    }
    let mut ctdp_level_v = ctdp_level.assume_init();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).feature = 0;

    if pbf != 0 {
        if ctdp_level_v.fact_enabled != 0 {
            (*info_p).feature |= BIT(1);
        }

        if enable != 0 {
            (*info_p).feature |= BIT(0);
        } else {
            (*info_p).feature &= !BIT(0);
        }
    } else {
        if enable != 0 && ctdp_level_v.sst_cp_enabled == 0 {
            isst_display_error_info_message(
                0,
                b"Make sure to execute before: core-power enable\0".as_ptr() as *const c_char,
                0,
                0,
            );
        }

        if ctdp_level_v.pbf_enabled != 0 {
            (*info_p).feature |= BIT(0);
        }

        if enable != 0 {
            (*info_p).feature |= BIT(1);
        } else {
            (*info_p).feature &= !BIT(1);
        }
    }

    ret = tpmi_process_ioctl(ISST_IF_PERF_SET_FEATURE, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }

    i = 0;
    while i < FEATURE_ENABLE_RETRIES {
        usleep(FEATURE_ENABLE_WAIT_US);

        /* Check status */
        let mut next_ctdp_level = MaybeUninit::<isst_pkg_ctdp_level_info>::uninit();
        ret = isst_get_ctdp_control(id, current_level, next_ctdp_level.as_mut_ptr());
        if ret != 0 {
            return ret;
        }
        ctdp_level_v = next_ctdp_level.assume_init();

        debug_printf(
            b"pbf_enabled:%d fact_enabled:%d\n\0".as_ptr() as *const c_char,
            ctdp_level_v.pbf_enabled,
            ctdp_level_v.fact_enabled,
        );

        if pbf != 0 {
            if ctdp_level_v.pbf_enabled == enable {
                break;
            }
        } else if ctdp_level_v.fact_enabled == enable {
            break;
        }
        i += 1;
    }

    if i == FEATURE_ENABLE_RETRIES {
        return -1;
    }

    0
}

unsafe extern "C" fn tpmi_get_fact_info(
    id: *mut isst_id,
    level: c_int,
    _fact_bucket: c_int,
    fact_info: *mut isst_fact_info,
) -> c_int {
    let mut info = MaybeUninit::<isst_turbo_freq_info>::uninit();
    let info_p = info.as_mut_ptr();

    memset(info_p as *mut c_void, 0, core::mem::size_of::<isst_turbo_freq_info>());
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = level;

    let ret = tpmi_process_ioctl(ISST_IF_GET_TURBO_FREQ_INFO, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let mut info = info.assume_init();

    let mut i = 0;
    while i < info.max_clip_freqs {
        (*fact_info).lp_ratios[i as usize] = info.lp_clip_freq_mhz[i as usize];
        i += 1;
    }

    if info.max_buckets > TRL_MAX_BUCKETS {
        info.max_buckets = TRL_MAX_BUCKETS;
    }

    if info.max_trl_levels > TRL_MAX_LEVELS {
        info.max_trl_levels = TRL_MAX_LEVELS;
    }

    i = 0;
    while i < info.max_trl_levels {
        let mut j = 0;
        while j < info.max_buckets {
            (*fact_info).bucket_info[j as usize].hp_ratios[i as usize] =
                info.trl_freq_mhz[i as usize][j as usize];
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < info.max_buckets {
        (*fact_info).bucket_info[i as usize].hp_cores = info.bucket_core_counts[i as usize];
        i += 1;
    }

    0
}

unsafe extern "C" fn _set_uncore_min_max(id: *mut isst_id, max: c_int, freq: c_int) {
    let mut buffer = [0 as c_char; 512];
    let mut tmp_id: c_uint = 0;
    let dir = opendir(b"/sys/devices/system/cpu/intel_uncore_frequency/\0".as_ptr() as *const c_char);
    if dir.is_null() {
        return;
    }

    loop {
        let entry = readdir(dir);
        if entry.is_null() {
            break;
        }

        /* Check domain_id */
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            b"/sys/devices/system/cpu/intel_uncore_frequency/%s/domain_id\0".as_ptr() as *const c_char,
            (*entry).d_name.as_ptr(),
        );

        let mut filep = fopen(buffer.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if filep.is_null() {
            break;
        }

        let mut ret = fscanf(filep, b"%u\0".as_ptr() as *const c_char, &mut tmp_id);
        fclose(filep);
        if ret != 1 {
            break;
        }

        if tmp_id != (*id).punit as c_uint {
            continue;
        }

        /* Check package_id */
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            b"/sys/devices/system/cpu/intel_uncore_frequency/%s/package_id\0".as_ptr() as *const c_char,
            (*entry).d_name.as_ptr(),
        );

        filep = fopen(buffer.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if filep.is_null() {
            break;
        }

        ret = fscanf(filep, b"%u\0".as_ptr() as *const c_char, &mut tmp_id);
        fclose(filep);

        if ret != 1 {
            break;
        }

        if tmp_id != (*id).pkg as c_uint {
            continue;
        }

        /* Found the right sysfs path, adjust and quit */
        if max != 0 {
            snprintf(
                buffer.as_mut_ptr(),
                buffer.len(),
                b"/sys/devices/system/cpu/intel_uncore_frequency/%s/max_freq_khz\0".as_ptr() as *const c_char,
                (*entry).d_name.as_ptr(),
            );
        } else {
            snprintf(
                buffer.as_mut_ptr(),
                buffer.len(),
                b"/sys/devices/system/cpu/intel_uncore_frequency/%s/min_freq_khz\0".as_ptr() as *const c_char,
                (*entry).d_name.as_ptr(),
            );
        }

        filep = fopen(buffer.as_ptr(), b"w\0".as_ptr() as *const c_char);
        if filep.is_null() {
            break;
        }

        fprintf(filep, b"%d\n\0".as_ptr() as *const c_char, freq);
        fclose(filep);
        break;
    }

    closedir(dir);
}

unsafe extern "C" fn tpmi_adjust_uncore_freq(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) {
    let mut info = MaybeUninit::<isst_perf_level_data_info>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).level = config_index;

    let ret = tpmi_process_ioctl(ISST_IF_GET_PERF_LEVEL_INFO, info_p as *mut c_void);
    if ret == -1 {
        return;
    }
    let info = info.assume_init();

    (*ctdp_level).uncore_p0 = info.p0_fabric_freq_mhz;
    (*ctdp_level).uncore_p1 = info.p1_fabric_freq_mhz;
    (*ctdp_level).uncore_pm = info.pm_fabric_freq_mhz;

    if (*ctdp_level).uncore_pm != 0 {
        _set_uncore_min_max(id, 0, (*ctdp_level).uncore_pm * 100000);
    }

    if (*ctdp_level).uncore_p0 != 0 {
        _set_uncore_min_max(id, 1, (*ctdp_level).uncore_p0 * 100000);
    }
}

unsafe extern "C" fn tpmi_get_clos_information(id: *mut isst_id, enable: *mut c_int, type_: *mut c_int) -> c_int {
    let mut info = MaybeUninit::<isst_core_power>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).get_set = 0;
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    let ret = tpmi_process_ioctl(ISST_IF_CORE_POWER_STATE, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    *enable = info.enable;
    *type_ = info.priority_type;

    0
}

unsafe extern "C" fn tpmi_pm_qos_config(id: *mut isst_id, enable_clos: c_int, priority_type: c_int) -> c_int {
    let mut info = MaybeUninit::<isst_core_power>::uninit();
    let info_p = info.as_mut_ptr();
    let mut cp_state: c_int = 0;
    let mut cp_cap: c_int = 0;
    let mut ret: c_int;

    (*info_p).get_set = 1;
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).enable = enable_clos;
    (*info_p).priority_type = priority_type;

    let saved_punit = (*id).punit;

    /* Set for all other dies also. This is per package setting */
    let mut i = 0;
    while i < MAX_PUNIT_PER_DIE {
        (*id).punit = i;
        if isst_is_punit_valid(id) != 0 {
            (*info_p).power_domain_id = i;
            ret = tpmi_process_ioctl(ISST_IF_CORE_POWER_STATE, info_p as *mut c_void);
            if ret == -1 {
                (*id).punit = saved_punit;
                return ret;
            }
            /* Get status */
            let mut j = 0;
            while j < FEATURE_ENABLE_RETRIES {
                usleep(FEATURE_ENABLE_WAIT_US);
                ret = tpmi_read_pm_config(id, &mut cp_state, &mut cp_cap);
                debug_printf(
                    b"ret:%d cp_state:%d enable_clos:%d\n\0".as_ptr() as *const c_char,
                    ret,
                    cp_state,
                    enable_clos,
                );
                if ret != 0 || cp_state == enable_clos {
                    break;
                }
                j += 1;
            }
            if j == FEATURE_ENABLE_RETRIES {
                (*id).punit = saved_punit;
                return -1;
            }
        }
        i += 1;
    }

    (*id).punit = saved_punit;

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpmi_pm_get_clos(
    id: *mut isst_id,
    clos: c_int,
    clos_config: *mut isst_clos_config,
) -> c_int {
    let mut info = MaybeUninit::<isst_clos_param>::uninit();
    let info_p = info.as_mut_ptr();

    (*info_p).get_set = 0;
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).clos = clos;

    let ret = tpmi_process_ioctl(ISST_IF_CLOS_PARAM, info_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let info = info.assume_init();

    (*clos_config).epp = 0;
    (*clos_config).clos_prop_prio = info.prop_prio;
    (*clos_config).clos_min = info.min_freq_mhz;
    (*clos_config).clos_max = info.max_freq_mhz;
    (*clos_config).clos_desired = 0;

    debug_printf(
        b"cpu:%d clos:%d min:%d max:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        clos,
        (*clos_config).clos_min,
        (*clos_config).clos_max,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpmi_set_clos(
    id: *mut isst_id,
    clos: c_int,
    clos_config: *mut isst_clos_config,
) -> c_int {
    let mut info = MaybeUninit::<isst_clos_param>::uninit();
    let info_p = info.as_mut_ptr();
    let mut ret: c_int;

    (*info_p).get_set = 1;
    (*info_p).socket_id = (*id).pkg;
    (*info_p).power_domain_id = (*id).punit;
    (*info_p).clos = clos;
    (*info_p).prop_prio = (*clos_config).clos_prop_prio;
    (*info_p).min_freq_mhz = (*clos_config).clos_min;
    (*info_p).max_freq_mhz = (*clos_config).clos_max;

    if (*info_p).min_freq_mhz <= 0xff {
        (*info_p).min_freq_mhz *= 100;
    }
    if (*info_p).max_freq_mhz <= 0xff {
        (*info_p).max_freq_mhz *= 100;
    }

    let saved_punit = (*id).punit;

    /* Set for all other dies also. This is per package setting */
    let mut i = 0;
    while i < MAX_PUNIT_PER_DIE {
        (*id).punit = i;
        if isst_is_punit_valid(id) != 0 {
            (*info_p).power_domain_id = i;
            ret = tpmi_process_ioctl(ISST_IF_CLOS_PARAM, info_p as *mut c_void);
            if ret == -1 {
                (*id).punit = saved_punit;
                return ret;
            }
        }
        i += 1;
    }

    (*id).punit = saved_punit;

    debug_printf(
        b"set cpu:%d clos:%d min:%d max:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        clos,
        (*clos_config).clos_min,
        (*clos_config).clos_max,
    );

    0
}

unsafe extern "C" fn tpmi_clos_get_assoc_status(id: *mut isst_id, clos_id: *mut c_int) -> c_int {
    let mut assoc_cmds = MaybeUninit::<isst_if_clos_assoc_cmds>::uninit();
    let assoc_cmds_p = assoc_cmds.as_mut_ptr();

    (*assoc_cmds_p).cmd_count = 1;
    (*assoc_cmds_p).get_set = 0;
    (*assoc_cmds_p).punit_cpu_map = 1;
    (*assoc_cmds_p).assoc_info[0].logical_cpu = find_phy_core_num((*id).cpu);
    (*assoc_cmds_p).assoc_info[0].socket_id = (*id).pkg;
    (*assoc_cmds_p).assoc_info[0].power_domain_id = (*id).punit;

    let ret = tpmi_process_ioctl(ISST_IF_CLOS_ASSOC, assoc_cmds_p as *mut c_void);
    if ret == -1 {
        return ret;
    }
    let assoc_cmds = assoc_cmds.assume_init();

    *clos_id = assoc_cmds.assoc_info[0].clos;

    0
}

unsafe extern "C" fn tpmi_clos_associate(id: *mut isst_id, clos_id: c_int) -> c_int {
    let mut assoc_cmds = MaybeUninit::<isst_if_clos_assoc_cmds>::uninit();
    let assoc_cmds_p = assoc_cmds.as_mut_ptr();

    (*assoc_cmds_p).cmd_count = 1;
    (*assoc_cmds_p).get_set = 1;
    (*assoc_cmds_p).punit_cpu_map = 1;
    (*assoc_cmds_p).assoc_info[0].logical_cpu = find_phy_core_num((*id).cpu);
    (*assoc_cmds_p).assoc_info[0].clos = clos_id;
    (*assoc_cmds_p).assoc_info[0].socket_id = (*id).pkg;
    (*assoc_cmds_p).assoc_info[0].power_domain_id = (*id).punit;

    let ret = tpmi_process_ioctl(ISST_IF_CLOS_ASSOC, assoc_cmds_p as *mut c_void);
    if ret == -1 {
        return ret;
    }

    0
}

static mut tpmi_ops: isst_platform_ops = isst_platform_ops {
    get_disp_freq_multiplier: Some(tpmi_get_disp_freq_multiplier),
    get_trl_max_levels: Some(tpmi_get_trl_max_levels),
    get_trl_level_name: Some(tpmi_get_trl_level_name),
    update_platform_param: Some(tpmi_update_platform_param),
    is_punit_valid: Some(tpmi_is_punit_valid),
    read_pm_config: Some(tpmi_read_pm_config),
    get_config_levels: Some(tpmi_get_config_levels),
    get_ctdp_control: Some(tpmi_get_ctdp_control),
    get_tdp_info: Some(tpmi_get_tdp_info),
    get_pwr_info: Some(tpmi_get_pwr_info),
    get_coremask_info: Some(tpmi_get_coremask_info),
    get_get_trl: Some(tpmi_get_get_trl),
    get_get_trls: Some(tpmi_get_get_trls),
    get_trl_bucket_info: Some(tpmi_get_trl_bucket_info),
    set_tdp_level: Some(tpmi_set_tdp_level),
    get_pbf_info: Some(tpmi_get_pbf_info),
    set_pbf_fact_status: Some(tpmi_set_pbf_fact_status),
    get_fact_info: Some(tpmi_get_fact_info),
    adjust_uncore_freq: Some(tpmi_adjust_uncore_freq),
    get_clos_information: Some(tpmi_get_clos_information),
    pm_qos_config: Some(tpmi_pm_qos_config),
    pm_get_clos: Some(tpmi_pm_get_clos),
    set_clos: Some(tpmi_set_clos),
    clos_get_assoc_status: Some(tpmi_clos_get_assoc_status),
    clos_associate: Some(tpmi_clos_associate),
};

#[no_mangle]
pub unsafe extern "C" fn tpmi_get_platform_ops() -> *mut isst_platform_ops {
    &mut tpmi_ops
}
