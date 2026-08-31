// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Enumerate and control features
 * Copyright (c) 2019 Intel Corporation.
 */

// Translated from isst-core.c.  External types, constants, and functions are
// supplied by the Rust translation of "isst.h" and other C dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const O_RDWR: c_int = 2;
const ISST_IF_MSR_COMMAND: c_ulonglong = 0;
const MSR_TRL_FREQ_MULTIPLIER: c_ulonglong = 100;

type FILE = c_void;

#[repr(C)]
pub struct isst_id {
    pub cpu: c_int,
    pub pkg: c_int,
    pub die: c_int,
}

#[repr(C)]
pub struct isst_if_msr_cmd {
    pub logical_cpu: c_uint,
    pub msr: c_uint,
    pub read_write: c_int,
    pub data: c_ulonglong,
}

#[repr(C)]
pub struct isst_if_msr_cmds {
    pub cmd_count: c_int,
    pub msr_cmd: [isst_if_msr_cmd; 1],
}

#[repr(C)]
pub struct isst_pkg_ctdp {
    pub processed: c_int,
    pub levels: c_int,
    pub enabled: c_int,
    pub current_level: c_int,
    pub ctdp_level: [isst_pkg_ctdp_level_info; 8],
}

#[repr(C)]
pub struct isst_pkg_ctdp_level_info {
    pub processed: c_int,
    pub level: c_int,
    pub control_cpu: c_int,
    pub pkg_id: c_int,
    pub die_id: c_int,
    pub pbf_support: c_int,
    pub pbf_found: c_int,
    pub fact_support: c_int,
    pub pbf_info: isst_pbf_info,
    pub fact_info: isst_fact_info,
    pub sse_p1: c_int,
    pub tdp_ratio: c_int,
    pub trl_ratios: [[c_int; 8]; 3],
    pub trl_cores: c_ulonglong,
    pub core_cpumask_size: c_int,
    pub core_cpumask: *mut c_void,
}

#[repr(C)]
pub struct isst_pbf_info {
    pub core_cpumask_size: c_int,
    pub core_cpumask: *mut c_void,
}

#[repr(C)]
pub struct isst_fact_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct isst_clos_config {
    _private: [u8; 0],
}

#[repr(C)]
pub enum isst_platform_param {
    _Dummy = 0,
}

#[repr(C)]
pub struct isst_platform_ops {
    pub update_platform_param: Option<unsafe extern "C" fn(isst_platform_param, c_int)>,
    pub get_disp_freq_multiplier: Option<unsafe extern "C" fn() -> c_int>,
    pub get_trl_max_levels: Option<unsafe extern "C" fn() -> c_int>,
    pub get_trl_level_name: Option<unsafe extern "C" fn(c_int) -> *mut c_char>,
    pub is_punit_valid: Option<unsafe extern "C" fn(*mut isst_id) -> c_int>,
    pub read_pm_config: Option<unsafe extern "C" fn(*mut isst_id, *mut c_int, *mut c_int) -> c_int>,
    pub get_config_levels: Option<unsafe extern "C" fn(*mut isst_id, *mut isst_pkg_ctdp) -> c_int>,
    pub get_ctdp_control:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info) -> c_int>,
    pub get_tdp_info:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info) -> c_int>,
    pub get_pwr_info:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info) -> c_int>,
    pub get_coremask_info:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info) -> c_int>,
    pub get_get_trl: Option<unsafe extern "C" fn(*mut isst_id, c_int, c_int, *mut c_int) -> c_int>,
    pub get_get_trls:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info) -> c_int>,
    pub get_trl_bucket_info:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut c_ulonglong) -> c_int>,
    pub set_tdp_level: Option<unsafe extern "C" fn(*mut isst_id, c_int) -> c_int>,
    pub get_pbf_info: Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pbf_info) -> c_int>,
    pub set_pbf_fact_status: Option<unsafe extern "C" fn(*mut isst_id, c_int, c_int) -> c_int>,
    pub get_fact_info:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, c_int, *mut isst_fact_info) -> c_int>,
    pub adjust_uncore_freq:
        Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_pkg_ctdp_level_info)>,
    pub get_clos_information: Option<unsafe extern "C" fn(*mut isst_id, *mut c_int, *mut c_int) -> c_int>,
    pub pm_qos_config: Option<unsafe extern "C" fn(*mut isst_id, c_int, c_int) -> c_int>,
    pub pm_get_clos: Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_clos_config) -> c_int>,
    pub set_clos: Option<unsafe extern "C" fn(*mut isst_id, c_int, *mut isst_clos_config) -> c_int>,
    pub clos_get_assoc_status: Option<unsafe extern "C" fn(*mut isst_id, *mut c_int) -> c_int>,
    pub clos_associate: Option<unsafe extern "C" fn(*mut isst_id, c_int) -> c_int>,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn mbox_get_platform_ops() -> *mut isst_platform_ops;
    fn tpmi_get_platform_ops() -> *mut isst_platform_ops;
    fn get_output_file() -> *mut FILE;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...);
    fn ioctl(fd: c_int, request: c_ulonglong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn alloc_cpu_set(cpu_set: *mut *mut c_void) -> c_int;
    fn free_cpu_set(cpu_set: *mut c_void);
    fn isst_display_error_info_message(error: c_int, msg: *const c_char, arg_valid: c_int, arg: c_int);
    fn debug_printf(fmt: *const c_char, ...);
    fn is_skx_based_platform() -> c_int;
    fn get_cpufreq_base_freq(cpu: c_int) -> c_int;
}

static mut isst_ops: *mut isst_platform_ops = core::ptr::null_mut();

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn check_cb(present: bool) {
    if isst_ops.is_null() || !present {
        fprintf(stderr, cstr!("Invalid ops\n"));
        exit(0);
    }
}

fn bit(nr: c_int) -> c_ulonglong {
    1u64 << nr
}

fn genmask(h: c_int, l: c_int) -> c_ulonglong {
    (!0u64 << l) & (!0u64 >> (63 - h))
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_platform_ops(api_version: c_int) -> c_int {
    match api_version {
        1 => isst_ops = mbox_get_platform_ops(),
        2 | 3 => isst_ops = tpmi_get_platform_ops(),
        _ => isst_ops = core::ptr::null_mut(),
    }

    if isst_ops.is_null() {
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_update_platform_param(param: isst_platform_param, value: c_int) {
    check_cb((*isst_ops).update_platform_param.is_some());
    ((*isst_ops).update_platform_param.unwrap())(param, value);
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_disp_freq_multiplier() -> c_int {
    check_cb((*isst_ops).get_disp_freq_multiplier.is_some());
    ((*isst_ops).get_disp_freq_multiplier.unwrap())()
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_trl_max_levels() -> c_int {
    check_cb((*isst_ops).get_trl_max_levels.is_some());
    ((*isst_ops).get_trl_max_levels.unwrap())()
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_trl_level_name(level: c_int) -> *mut c_char {
    check_cb((*isst_ops).get_trl_level_name.is_some());
    ((*isst_ops).get_trl_level_name.unwrap())(level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_is_punit_valid(id: *mut isst_id) -> c_int {
    check_cb((*isst_ops).is_punit_valid.is_some());
    ((*isst_ops).is_punit_valid.unwrap())(id)
}

#[no_mangle]
pub unsafe extern "C" fn isst_send_msr_command(
    cpu: c_uint,
    msr: c_uint,
    write: c_int,
    req_resp: *mut c_ulonglong,
) -> c_int {
    let mut msr_cmds = core::mem::MaybeUninit::<isst_if_msr_cmds>::zeroed().assume_init();
    let pathname = cstr!("/dev/isst_interface");
    let outf = get_output_file();
    let fd: c_int;

    fd = open(pathname, O_RDWR);
    if fd < 0 {
        err(-1, cstr!("%s open failed"), pathname);
    }

    msr_cmds.cmd_count = 1;
    msr_cmds.msr_cmd[0].logical_cpu = cpu;
    msr_cmds.msr_cmd[0].msr = msr;
    msr_cmds.msr_cmd[0].read_write = write;
    if write != 0 {
        msr_cmds.msr_cmd[0].data = *req_resp;
    }

    if ioctl(fd, ISST_IF_MSR_COMMAND, &mut msr_cmds as *mut isst_if_msr_cmds) == -1 {
        perror(cstr!("ISST_IF_MSR_COMMAND"));
        fprintf(
            outf,
            cstr!("Error: msr_cmd cpu:%d msr:%x read_write:%d\n"),
            cpu,
            msr,
            write,
        );
    } else {
        if write == 0 {
            *req_resp = msr_cmds.msr_cmd[0].data;
        }

        debug_printf(
            cstr!("msr_cmd response: cpu:%d msr:%x rd_write:%x resp:%llx %llx\n"),
            cpu,
            msr,
            write,
            *req_resp,
            msr_cmds.msr_cmd[0].data,
        );
    }

    close(fd);

    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_read_pm_config(
    id: *mut isst_id,
    cp_state: *mut c_int,
    cp_cap: *mut c_int,
) -> c_int {
    check_cb((*isst_ops).read_pm_config.is_some());
    ((*isst_ops).read_pm_config.unwrap())(id, cp_state, cp_cap)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_ctdp_levels(
    id: *mut isst_id,
    pkg_dev: *mut isst_pkg_ctdp,
) -> c_int {
    check_cb((*isst_ops).get_config_levels.is_some());
    ((*isst_ops).get_config_levels.unwrap())(id, pkg_dev)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_ctdp_control(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    check_cb((*isst_ops).get_ctdp_control.is_some());
    ((*isst_ops).get_ctdp_control.unwrap())(id, config_index, ctdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_tdp_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    check_cb((*isst_ops).get_tdp_info.is_some());
    ((*isst_ops).get_tdp_info.unwrap())(id, config_index, ctdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_pwr_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    check_cb((*isst_ops).get_pwr_info.is_some());
    ((*isst_ops).get_pwr_info.unwrap())(id, config_index, ctdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_coremask_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    check_cb((*isst_ops).get_coremask_info.is_some());
    ((*isst_ops).get_coremask_info.unwrap())(id, config_index, ctdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_get_trl_from_msr(id: *mut isst_id, trl: *mut c_int) -> c_int {
    let mut msr_trl: c_ulonglong = 0;
    let mut ret: c_int;

    ret = isst_send_msr_command((*id).cpu as c_uint, 0x1AD, 0, &mut msr_trl);
    if ret != 0 {
        return ret;
    }

    *trl.add(0) = (msr_trl & genmask(7, 0)) as c_int;
    *trl.add(1) = ((msr_trl & genmask(15, 8)) >> 8) as c_int;
    *trl.add(2) = ((msr_trl & genmask(23, 16)) >> 16) as c_int;
    *trl.add(3) = ((msr_trl & genmask(31, 24)) >> 24) as c_int;
    *trl.add(4) = ((msr_trl & genmask(39, 32)) >> 32) as c_int;
    *trl.add(5) = ((msr_trl & genmask(47, 40)) >> 40) as c_int;
    *trl.add(6) = ((msr_trl & genmask(55, 48)) >> 48) as c_int;
    *trl.add(7) = ((msr_trl & genmask(63, 56)) >> 56) as c_int;

    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_get_trl(
    id: *mut isst_id,
    level: c_int,
    avx_level: c_int,
    trl: *mut c_int,
) -> c_int {
    check_cb((*isst_ops).get_get_trl.is_some());
    ((*isst_ops).get_get_trl.unwrap())(id, level, avx_level, trl)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_get_trls(
    id: *mut isst_id,
    level: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    check_cb((*isst_ops).get_get_trls.is_some());
    ((*isst_ops).get_get_trls.unwrap())(id, level, ctdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_trl_bucket_info(
    id: *mut isst_id,
    level: c_int,
    buckets_info: *mut c_ulonglong,
) -> c_int {
    check_cb((*isst_ops).get_trl_bucket_info.is_some());
    ((*isst_ops).get_trl_bucket_info.unwrap())(id, level, buckets_info)
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_tdp_level(id: *mut isst_id, tdp_level: c_int) -> c_int {
    check_cb((*isst_ops).set_tdp_level.is_some());
    ((*isst_ops).set_tdp_level.unwrap())(id, tdp_level)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_pbf_info(
    id: *mut isst_id,
    level: c_int,
    pbf_info: *mut isst_pbf_info,
) -> c_int {
    let mut ctdp_level = core::mem::MaybeUninit::<isst_pkg_ctdp_level_info>::zeroed().assume_init();
    let mut pkg_dev = core::mem::MaybeUninit::<isst_pkg_ctdp>::zeroed().assume_init();
    let mut ret: c_int;

    ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 {
        isst_display_error_info_message(1, cstr!("Failed to get number of levels"), 0, 0);
        return ret;
    }

    if level > pkg_dev.levels {
        isst_display_error_info_message(1, cstr!("Invalid level"), 1, level);
        return -1;
    }

    ret = isst_get_ctdp_control(id, level, &mut ctdp_level);
    if ret != 0 {
        return ret;
    }

    if ctdp_level.pbf_support == 0 {
        isst_display_error_info_message(
            1,
            cstr!("base-freq feature is not present at this level"),
            1,
            level,
        );
        return -1;
    }

    (*pbf_info).core_cpumask_size = alloc_cpu_set(&mut (*pbf_info).core_cpumask);

    check_cb((*isst_ops).get_pbf_info.is_some());
    ((*isst_ops).get_pbf_info.unwrap())(id, level, pbf_info)
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_pbf_fact_status(
    id: *mut isst_id,
    pbf: c_int,
    enable: c_int,
) -> c_int {
    check_cb((*isst_ops).set_pbf_fact_status.is_some());
    ((*isst_ops).set_pbf_fact_status.unwrap())(id, pbf, enable)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_fact_info(
    id: *mut isst_id,
    level: c_int,
    fact_bucket: c_int,
    fact_info: *mut isst_fact_info,
) -> c_int {
    let mut ctdp_level = core::mem::MaybeUninit::<isst_pkg_ctdp_level_info>::zeroed().assume_init();
    let mut pkg_dev = core::mem::MaybeUninit::<isst_pkg_ctdp>::zeroed().assume_init();
    let mut ret: c_int;

    ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 {
        isst_display_error_info_message(1, cstr!("Failed to get number of levels"), 0, 0);
        return ret;
    }

    if level > pkg_dev.levels {
        isst_display_error_info_message(1, cstr!("Invalid level"), 1, level);
        return -1;
    }

    ret = isst_get_ctdp_control(id, level, &mut ctdp_level);
    if ret != 0 {
        return ret;
    }

    if ctdp_level.fact_support == 0 {
        isst_display_error_info_message(
            1,
            cstr!("turbo-freq feature is not present at this level"),
            1,
            level,
        );
        return -1;
    }
    check_cb((*isst_ops).get_fact_info.is_some());
    ((*isst_ops).get_fact_info.unwrap())(id, level, fact_bucket, fact_info)
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_trl(id: *mut isst_id, trl: *mut c_ulonglong) -> c_int {
    let mut ret: c_int;

    ret = isst_send_msr_command((*id).cpu as c_uint, 0x1AD, 0, trl);
    if ret != 0 {
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_trl(id: *mut isst_id, mut trl: c_ulonglong) -> c_int {
    let mut ret: c_int;

    if trl == 0 {
        trl = 0xFFFFFFFFFFFFFFFFu64;
    }

    ret = isst_send_msr_command((*id).cpu as c_uint, 0x1AD, 1, &mut trl);
    if ret != 0 {
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_trl_from_current_tdp(
    id: *mut isst_id,
    trl: c_ulonglong,
) -> c_int {
    let mut msr_trl: c_ulonglong;
    let mut ret: c_int;

    if (*id).cpu < 0 {
        return 0;
    }

    if trl != 0 {
        msr_trl = trl;
    } else {
        let mut pkg_dev = core::mem::MaybeUninit::<isst_pkg_ctdp>::zeroed().assume_init();
        let mut trl = [0 as c_int; 8];
        let mut i: c_int;

        ret = isst_get_ctdp_levels(id, &mut pkg_dev);
        if ret != 0 {
            return ret;
        }

        ret = isst_get_get_trl(id, pkg_dev.current_level, 0, trl.as_mut_ptr());
        if ret != 0 {
            return ret;
        }

        msr_trl = 0;
        i = 0;
        while i < 8 {
            let mut _trl: c_ulonglong = trl[i as usize] as c_ulonglong;

            /* MSR is always in 100 MHz unit */
            if isst_get_disp_freq_multiplier() == 1 {
                _trl /= MSR_TRL_FREQ_MULTIPLIER;
            }

            msr_trl |= _trl << (i * 8);
            i += 1;
        }
    }
    ret = isst_send_msr_command((*id).cpu as c_uint, 0x1AD, 1, &mut msr_trl);
    if ret != 0 {
        return ret;
    }

    0
}

/* Return 1 if locked */
#[no_mangle]
pub unsafe extern "C" fn isst_get_config_tdp_lock_status(id: *mut isst_id) -> c_int {
    let mut tdp_control: c_ulonglong = 0;
    let mut ret: c_int;

    ret = isst_send_msr_command((*id).cpu as c_uint, 0x64b, 0, &mut tdp_control);
    if ret != 0 {
        return ret;
    }

    ret = ((tdp_control & bit(31)) != 0) as c_int;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_process_ctdp_complete(
    _id: *mut isst_id,
    pkg_dev: *mut isst_pkg_ctdp,
) {
    let mut i: c_int;

    if (*pkg_dev).processed == 0 {
        return;
    }

    i = 0;
    while i < (*pkg_dev).levels {
        let ctdp_level: *mut isst_pkg_ctdp_level_info;

        ctdp_level = (*pkg_dev).ctdp_level.as_mut_ptr().add(i as usize);
        if (*ctdp_level).pbf_support != 0 {
            free_cpu_set((*ctdp_level).pbf_info.core_cpumask);
        }
        free_cpu_set((*ctdp_level).core_cpumask);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn isst_adjust_uncore_freq(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) {
    check_cb((*isst_ops).adjust_uncore_freq.is_some());
    ((*isst_ops).adjust_uncore_freq.unwrap())(id, config_index, ctdp_level);
}

#[no_mangle]
pub unsafe extern "C" fn isst_get_process_ctdp(
    id: *mut isst_id,
    tdp_level: c_int,
    pkg_dev: *mut isst_pkg_ctdp,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut valid: c_int = 0;

    if (*pkg_dev).processed != 0 {
        return 0;
    }

    ret = isst_get_ctdp_levels(id, pkg_dev);
    if ret != 0 {
        return ret;
    }

    debug_printf(
        cstr!("cpu: %d ctdp enable:%d current level: %d levels:%d\n"),
        (*id).cpu,
        (*pkg_dev).enabled,
        (*pkg_dev).current_level,
        (*pkg_dev).levels,
    );

    if tdp_level != 0xff && tdp_level > (*pkg_dev).levels {
        isst_display_error_info_message(1, cstr!("Invalid level"), 0, 0);
        return -1;
    }

    if (*pkg_dev).enabled == 0 {
        isst_display_error_info_message(
            0,
            cstr!("perf-profile feature is not supported, just base-config level 0 is valid"),
            0,
            0,
        );
    }

    i = 0;
    while i <= (*pkg_dev).levels {
        let ctdp_level: *mut isst_pkg_ctdp_level_info;

        if tdp_level != 0xff && i != tdp_level {
            i += 1;
            continue;
        }

        debug_printf(
            cstr!("cpu:%d Get Information for TDP level:%d\n"),
            (*id).cpu,
            i,
        );
        ctdp_level = (*pkg_dev).ctdp_level.as_mut_ptr().add(i as usize);

        (*ctdp_level).level = i;
        (*ctdp_level).control_cpu = (*id).cpu;
        (*ctdp_level).pkg_id = (*id).pkg;
        (*ctdp_level).die_id = (*id).die;

        ret = isst_get_ctdp_control(id, i, ctdp_level);
        if ret != 0 {
            i += 1;
            continue;
        }

        valid = 1;
        (*pkg_dev).processed = 1;
        (*ctdp_level).processed = 1;

        if (*ctdp_level).pbf_support != 0 {
            ret = isst_get_pbf_info(id, i, &mut (*ctdp_level).pbf_info);
            if ret == 0 {
                (*ctdp_level).pbf_found = 1;
            }
        }

        if (*ctdp_level).fact_support != 0 {
            ret = isst_get_fact_info(id, i, 0xff, &mut (*ctdp_level).fact_info);
            if ret != 0 {
                return ret;
            }
        }

        if (*pkg_dev).enabled == 0 && is_skx_based_platform() != 0 {
            let mut freq: c_int;

            freq = get_cpufreq_base_freq((*id).cpu);
            if freq > 0 {
                (*ctdp_level).sse_p1 = freq / 100000;
                (*ctdp_level).tdp_ratio = (*ctdp_level).sse_p1;
            }

            isst_get_get_trl_from_msr(id, (*ctdp_level).trl_ratios[0].as_mut_ptr());
            isst_get_trl_bucket_info(id, i, &mut (*ctdp_level).trl_cores);
            i += 1;
            continue;
        }

        ret = isst_get_tdp_info(id, i, ctdp_level);
        if ret != 0 {
            return ret;
        }

        ret = isst_get_pwr_info(id, i, ctdp_level);
        if ret != 0 {
            return ret;
        }

        (*ctdp_level).core_cpumask_size = alloc_cpu_set(&mut (*ctdp_level).core_cpumask);
        ret = isst_get_coremask_info(id, i, ctdp_level);
        if ret != 0 {
            return ret;
        }

        ret = isst_get_trl_bucket_info(id, i, &mut (*ctdp_level).trl_cores);
        if ret != 0 {
            return ret;
        }

        ret = isst_get_get_trls(id, i, ctdp_level);
        if ret != 0 {
            return ret;
        }

        i += 1;
    }

    if valid == 0 {
        isst_display_error_info_message(
            0,
            cstr!("Invalid level, Can't get TDP control information at specified levels on cpu"),
            1,
            (*id).cpu,
        );
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_get_clos_information(
    id: *mut isst_id,
    enable: *mut c_int,
    type_: *mut c_int,
) -> c_int {
    check_cb((*isst_ops).get_clos_information.is_some());
    ((*isst_ops).get_clos_information.unwrap())(id, enable, type_)
}

#[no_mangle]
pub unsafe extern "C" fn isst_pm_qos_config(
    id: *mut isst_id,
    enable_clos: c_int,
    priority_type: c_int,
) -> c_int {
    check_cb((*isst_ops).pm_qos_config.is_some());
    ((*isst_ops).pm_qos_config.unwrap())(id, enable_clos, priority_type)
}

#[no_mangle]
pub unsafe extern "C" fn isst_pm_get_clos(
    id: *mut isst_id,
    clos: c_int,
    clos_config: *mut isst_clos_config,
) -> c_int {
    check_cb((*isst_ops).pm_get_clos.is_some());
    ((*isst_ops).pm_get_clos.unwrap())(id, clos, clos_config)
}

#[no_mangle]
pub unsafe extern "C" fn isst_set_clos(
    id: *mut isst_id,
    clos: c_int,
    clos_config: *mut isst_clos_config,
) -> c_int {
    check_cb((*isst_ops).set_clos.is_some());
    ((*isst_ops).set_clos.unwrap())(id, clos, clos_config)
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_get_assoc_status(id: *mut isst_id, clos_id: *mut c_int) -> c_int {
    check_cb((*isst_ops).clos_get_assoc_status.is_some());
    ((*isst_ops).clos_get_assoc_status.unwrap())(id, clos_id)
}

#[no_mangle]
pub unsafe extern "C" fn isst_clos_associate(id: *mut isst_id, clos_id: c_int) -> c_int {
    check_cb((*isst_ops).clos_associate.is_some());
    ((*isst_ops).clos_associate.unwrap())(id, clos_id)
}
