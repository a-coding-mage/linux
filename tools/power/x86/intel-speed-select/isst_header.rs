/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Intel Speed Select -- Enumerate and control features
 * Copyright (c) 2019 Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

/* C header dependencies removed:
 * stdio.h, unistd.h, sys/types.h, sched.h, sys/stat.h, sys/resource.h,
 * getopt.h, err.h, fcntl.h, signal.h, sys/time.h, limits.h, stdlib.h,
 * string.h, cpuid.h, dirent.h, errno.h, stdarg.h, sys/ioctl.h,
 * linux/isst_if.h.
 *
 * FILE, cpu_set_t, and TRL_MAX_LEVELS are provided by external dependencies.
 */

pub const fn BIT(x: u32) -> c_int {
    1_i32 << x
}

pub const fn BIT_ULL(nr: u32) -> c_ulonglong {
    1_u64 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> usize {
    ((!0_usize) << l) & ((!0_usize) >> (usize::BITS - 1 - h))
}

pub const fn GENMASK_ULL(h: u32, l: u32) -> c_ulonglong {
    ((!0_u64) << l) & ((!0_u64) >> (u64::BITS - 1 - h))
}

pub const CONFIG_TDP: c_int = 0x7f;
pub const CONFIG_TDP_GET_LEVELS_INFO: c_int = 0x00;
pub const CONFIG_TDP_GET_TDP_CONTROL: c_int = 0x01;
pub const CONFIG_TDP_SET_TDP_CONTROL: c_int = 0x02;
pub const CONFIG_TDP_GET_TDP_INFO: c_int = 0x03;
pub const CONFIG_TDP_GET_PWR_INFO: c_int = 0x04;
pub const CONFIG_TDP_GET_TJMAX_INFO: c_int = 0x05;
pub const CONFIG_TDP_GET_CORE_MASK: c_int = 0x06;
pub const CONFIG_TDP_GET_TURBO_LIMIT_RATIOS: c_int = 0x07;
pub const CONFIG_TDP_SET_LEVEL: c_int = 0x08;
pub const CONFIG_TDP_GET_UNCORE_P0_P1_INFO: c_int = 0x09;
pub const CONFIG_TDP_GET_P1_INFO: c_int = 0x0a;
pub const CONFIG_TDP_GET_MEM_FREQ: c_int = 0x0b;
pub const CONFIG_TDP_GET_RATIO_INFO: c_int = 0x0c;

pub const CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_NUMCORES: c_int = 0x10;
pub const CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_RATIOS: c_int = 0x11;
pub const CONFIG_TDP_GET_FACT_LP_CLIPPING_RATIO: c_int = 0x12;

pub const CONFIG_TDP_PBF_GET_CORE_MASK_INFO: c_int = 0x20;
pub const CONFIG_TDP_PBF_GET_P1HI_P1LO_INFO: c_int = 0x21;
pub const CONFIG_TDP_PBF_GET_TJ_MAX_INFO: c_int = 0x22;
pub const CONFIG_TDP_PBF_GET_TDP_INFO: c_int = 0x23;

pub const CONFIG_CLOS: c_int = 0xd0;
pub const CLOS_PQR_ASSOC: c_int = 0x00;
pub const CLOS_PM_CLOS: c_int = 0x01;
pub const CLOS_PM_QOS_CONFIG: c_int = 0x02;
pub const CLOS_STATUS: c_int = 0x03;

pub const MBOX_CMD_WRITE_BIT: c_int = 0x08;

pub const PM_QOS_INFO_OFFSET: c_int = 0x00;
pub const PM_QOS_CONFIG_OFFSET: c_int = 0x04;
pub const PM_CLOS_OFFSET: c_int = 0x08;
pub const PQR_ASSOC_OFFSET: c_int = 0x20;

pub const READ_PM_CONFIG: c_int = 0x94;
pub const WRITE_PM_CONFIG: c_int = 0x95;
pub const PM_FEATURE: c_int = 0x03;

pub const DISP_FREQ_MULTIPLIER: c_int = 100;

pub const MAX_PACKAGE_COUNT: c_int = 32;
pub const MAX_DIE_PER_PACKAGE: c_int = 16;
pub const MAX_PUNIT_PER_DIE: c_int = 8;

/* Unified structure to specific a CPU or a Power Domain */
#[repr(C)]
pub struct isst_id {
    pub cpu: c_int,
    pub pkg: c_int,
    pub die: c_int,
    pub punit: c_int,
}

#[repr(C)]
pub struct isst_clos_config {
    pub clos_min: c_uint,
    pub clos_max: c_uint,
    pub epp: u8,
    pub clos_prop_prio: u8,
    pub clos_desired: u8,
}

#[repr(C)]
pub struct isst_fact_bucket_info {
    pub hp_cores: c_int,
    pub hp_ratios: [c_int; TRL_MAX_LEVELS],
}

#[repr(C)]
pub struct isst_pbf_info {
    pub pbf_acticated: c_int,
    pub pbf_available: c_int,
    pub core_cpumask_size: usize,
    pub core_cpumask: *mut cpu_set_t,
    pub p1_high: c_int,
    pub p1_low: c_int,
    pub t_control: c_int,
    pub t_prochot: c_int,
    pub tdp: c_int,
}

pub const ISST_TRL_MAX_ACTIVE_CORES: usize = 8;
pub const ISST_FACT_MAX_BUCKETS: usize = 8;

#[repr(C)]
pub struct isst_fact_info {
    pub lp_ratios: [c_int; TRL_MAX_LEVELS],
    pub bucket_info: [isst_fact_bucket_info; ISST_FACT_MAX_BUCKETS],
}

#[repr(C)]
pub struct isst_pkg_ctdp_level_info {
    pub processed: c_int,
    pub control_cpu: c_int,
    pub pkg_id: c_int,
    pub die_id: c_int,
    pub level: c_int,
    pub fact_support: c_int,
    pub pbf_support: c_int,
    pub fact_enabled: c_int,
    pub pbf_enabled: c_int,
    pub sst_cp_support: c_int,
    pub sst_cp_enabled: c_int,
    pub tdp_ratio: c_int,
    pub active: c_int,
    pub tdp_control: c_int,
    pub pkg_tdp: c_int,
    pub pkg_min_power: c_int,
    pub pkg_max_power: c_int,
    pub fact: c_int,
    pub t_proc_hot: c_int,
    pub cooling_type: c_int,
    pub uncore_p0: c_int,
    pub uncore_p1: c_int,
    pub uncore_pm: c_int,
    pub uncore1_p0: c_int,
    pub uncore1_p1: c_int,
    pub uncore1_pm: c_int,
    pub sse_p1: c_int,
    pub avx2_p1: c_int,
    pub avx512_p1: c_int,
    pub amx_p1: c_int,
    pub mem_freq: c_int,
    pub core_cpumask_size: usize,
    pub core_cpumask: *mut cpu_set_t,
    pub cpu_count: c_int,
    pub trl_cores: c_ulonglong, /* Buckets info */
    pub trl_ratios: [[c_int; ISST_TRL_MAX_ACTIVE_CORES]; TRL_MAX_LEVELS],
    pub kobj_bucket_index: c_int,
    pub active_bucket: c_int,
    pub fact_max_index: c_int,
    pub fact_max_config: c_int,
    pub pbf_found: c_int,
    pub pbf_active: c_int,
    pub pbf_info: isst_pbf_info,
    pub fact_info: isst_fact_info,
}

pub const ISST_MAX_TDP_LEVELS: usize = 4 + 1; /* +1 for base config */

#[repr(C)]
pub struct isst_pkg_ctdp {
    pub locked: c_int,
    pub version: c_int,
    pub processed: c_int,
    pub levels: c_int,
    pub current_level: c_int,
    pub enabled: c_int,
    pub ctdp_level: [isst_pkg_ctdp_level_info; ISST_MAX_TDP_LEVELS],
}

#[repr(C)]
pub enum isst_platform_param {
    ISST_PARAM_MBOX_DELAY,
    ISST_PARAM_MBOX_RETRIES,
}

#[repr(C)]
pub struct isst_platform_ops {
    pub get_disp_freq_multiplier: Option<unsafe extern "C" fn() -> c_int>,
    pub get_trl_max_levels: Option<unsafe extern "C" fn() -> c_int>,
    pub get_trl_level_name: Option<unsafe extern "C" fn(level: c_int) -> *mut c_char>,
    pub update_platform_param:
        Option<unsafe extern "C" fn(param: isst_platform_param, value: c_int)>,
    pub is_punit_valid: Option<unsafe extern "C" fn(id: *mut isst_id) -> c_int>,
    pub read_pm_config:
        Option<unsafe extern "C" fn(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int>,
    pub get_config_levels:
        Option<unsafe extern "C" fn(id: *mut isst_id, pkg_ctdp: *mut isst_pkg_ctdp) -> c_int>,
    pub get_ctdp_control: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            config_index: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ) -> c_int,
    >,
    pub get_tdp_info: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            config_index: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ) -> c_int,
    >,
    pub get_pwr_info: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            config_index: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ) -> c_int,
    >,
    pub get_coremask_info: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            config_index: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ) -> c_int,
    >,
    pub get_get_trl: Option<
        unsafe extern "C" fn(id: *mut isst_id, level: c_int, avx_level: c_int, trl: *mut c_int) -> c_int,
    >,
    pub get_get_trls: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            level: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ) -> c_int,
    >,
    pub get_trl_bucket_info: Option<
        unsafe extern "C" fn(id: *mut isst_id, level: c_int, buckets_info: *mut c_ulonglong) -> c_int,
    >,
    pub set_tdp_level: Option<unsafe extern "C" fn(id: *mut isst_id, tdp_level: c_int) -> c_int>,
    pub get_pbf_info:
        Option<unsafe extern "C" fn(id: *mut isst_id, level: c_int, pbf_info: *mut isst_pbf_info) -> c_int>,
    pub set_pbf_fact_status:
        Option<unsafe extern "C" fn(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int>,
    pub get_fact_info: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            level: c_int,
            fact_bucket: c_int,
            fact_info: *mut isst_fact_info,
        ) -> c_int,
    >,
    pub adjust_uncore_freq: Option<
        unsafe extern "C" fn(
            id: *mut isst_id,
            config_index: c_int,
            ctdp_level: *mut isst_pkg_ctdp_level_info,
        ),
    >,
    pub get_clos_information:
        Option<unsafe extern "C" fn(id: *mut isst_id, enable: *mut c_int, type_: *mut c_int) -> c_int>,
    pub pm_qos_config:
        Option<unsafe extern "C" fn(id: *mut isst_id, enable_clos: c_int, priority_type: c_int) -> c_int>,
    pub pm_get_clos: Option<
        unsafe extern "C" fn(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int,
    >,
    pub set_clos: Option<
        unsafe extern "C" fn(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int,
    >,
    pub clos_get_assoc_status:
        Option<unsafe extern "C" fn(id: *mut isst_id, clos_id: *mut c_int) -> c_int>,
    pub clos_associate: Option<unsafe extern "C" fn(id: *mut isst_id, clos_id: c_int) -> c_int>,
}

unsafe extern "C" {
    pub fn is_cpu_in_power_domain(cpu: c_int, id: *mut isst_id) -> c_int;
    pub fn get_topo_max_cpus() -> c_int;
    pub fn get_cpu_count(id: *mut isst_id) -> c_int;
    pub fn get_max_punit_core_id(id: *mut isst_id) -> c_int;
    pub fn api_version() -> c_int;

    /* Common interfaces */
    pub fn get_output_file() -> *mut FILE;
    pub fn is_debug_enabled() -> c_int;
    pub fn debug_printf(format: *const c_char, ...);
    pub fn out_format_is_json() -> c_int;
    pub fn set_isst_id(id: *mut isst_id, cpu: c_int);
    pub fn alloc_cpu_set(cpu_set: *mut *mut cpu_set_t) -> usize;
    pub fn free_cpu_set(cpu_set: *mut cpu_set_t);
    pub fn find_phy_core_num(logical_cpu: c_int) -> c_int;
    pub fn set_cpu_mask_from_punit_coremask(
        id: *mut isst_id,
        core_mask: c_ulonglong,
        core_cpumask_size: usize,
        core_cpumask: *mut cpu_set_t,
        cpu_cnt: *mut c_int,
    );
    pub fn isst_send_msr_command(
        cpu: c_uint,
        command: c_uint,
        write: c_int,
        req_resp: *mut c_ulonglong,
    ) -> c_int;

    pub fn isst_set_platform_ops(api_version: c_int) -> c_int;
    pub fn isst_update_platform_param(param: isst_platform_param, vale: c_int);
    pub fn isst_get_disp_freq_multiplier() -> c_int;
    pub fn isst_get_trl_max_levels() -> c_int;
    pub fn isst_get_trl_level_name(level: c_int) -> *mut c_char;
    pub fn isst_is_punit_valid(id: *mut isst_id) -> c_int;

    pub fn isst_get_ctdp_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
    pub fn isst_get_ctdp_control(
        id: *mut isst_id,
        config_index: c_int,
        ctdp_level: *mut isst_pkg_ctdp_level_info,
    ) -> c_int;
    pub fn isst_get_coremask_info(
        id: *mut isst_id,
        config_index: c_int,
        ctdp_level: *mut isst_pkg_ctdp_level_info,
    ) -> c_int;
    pub fn isst_adjust_uncore_freq(
        id: *mut isst_id,
        config_index: c_int,
        ctdp_level: *mut isst_pkg_ctdp_level_info,
    );
    pub fn isst_get_process_ctdp(
        id: *mut isst_id,
        tdp_level: c_int,
        pkg_dev: *mut isst_pkg_ctdp,
    ) -> c_int;
    pub fn isst_get_process_ctdp_complete(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp);
    pub fn isst_ctdp_display_information(
        id: *mut isst_id,
        outf: *mut FILE,
        tdp_level: c_int,
        pkg_dev: *mut isst_pkg_ctdp,
    );
    pub fn isst_ctdp_display_core_info(
        id: *mut isst_id,
        outf: *mut FILE,
        prefix: *mut c_char,
        val: c_uint,
        str0: *mut c_char,
        str1: *mut c_char,
    );
    pub fn isst_ctdp_display_information_start(outf: *mut FILE);
    pub fn isst_ctdp_display_information_end(outf: *mut FILE);
    pub fn isst_pbf_display_information(
        id: *mut isst_id,
        outf: *mut FILE,
        level: c_int,
        info: *mut isst_pbf_info,
    );
    pub fn isst_set_tdp_level(id: *mut isst_id, tdp_level: c_int) -> c_int;
    pub fn isst_set_pbf_fact_status(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int;
    pub fn isst_get_pbf_info(
        id: *mut isst_id,
        level: c_int,
        pbf_info: *mut isst_pbf_info,
    ) -> c_int;
    pub fn isst_get_fact_info(
        id: *mut isst_id,
        level: c_int,
        fact_bucket: c_int,
        fact_info: *mut isst_fact_info,
    ) -> c_int;
    pub fn isst_fact_display_information(
        id: *mut isst_id,
        outf: *mut FILE,
        level: c_int,
        fact_bucket: c_int,
        fact_avx: c_int,
        fact_info: *mut isst_fact_info,
    );
    pub fn isst_set_trl(id: *mut isst_id, trl: c_ulonglong) -> c_int;
    pub fn isst_get_trl(id: *mut isst_id, trl: *mut c_ulonglong) -> c_int;
    pub fn isst_set_trl_from_current_tdp(id: *mut isst_id, trl: c_ulonglong) -> c_int;
    pub fn isst_get_config_tdp_lock_status(id: *mut isst_id) -> c_int;

    pub fn isst_pm_qos_config(id: *mut isst_id, enable_clos: c_int, priority_type: c_int) -> c_int;
    pub fn isst_pm_get_clos(
        id: *mut isst_id,
        clos: c_int,
        clos_config: *mut isst_clos_config,
    ) -> c_int;
    pub fn isst_set_clos(
        id: *mut isst_id,
        clos: c_int,
        clos_config: *mut isst_clos_config,
    ) -> c_int;
    pub fn isst_clos_associate(id: *mut isst_id, clos: c_int) -> c_int;
    pub fn isst_clos_get_assoc_status(id: *mut isst_id, clos_id: *mut c_int) -> c_int;
    pub fn isst_clos_display_information(
        id: *mut isst_id,
        outf: *mut FILE,
        clos: c_int,
        clos_config: *mut isst_clos_config,
    );
    pub fn isst_clos_display_assoc_information(id: *mut isst_id, outf: *mut FILE, clos: c_int);

    pub fn isst_display_result(
        id: *mut isst_id,
        outf: *mut FILE,
        feature: *mut c_char,
        cmd: *mut c_char,
        result: c_int,
    );

    pub fn isst_clos_get_clos_information(
        id: *mut isst_id,
        enable: *mut c_int,
        type_: *mut c_int,
    ) -> c_int;
    pub fn isst_clos_display_clos_information(
        id: *mut isst_id,
        outf: *mut FILE,
        clos_enable: c_int,
        type_: c_int,
        state: c_int,
        cap: c_int,
    );
    pub fn is_clx_n_platform() -> c_int;
    pub fn get_cpufreq_base_freq(cpu: c_int) -> c_int;
    pub fn isst_read_pm_config(
        id: *mut isst_id,
        cp_state: *mut c_int,
        cp_cap: *mut c_int,
    ) -> c_int;
    pub fn isst_display_error_info_message(error: c_int, msg: *mut c_char, arg_valid: c_int, arg: c_int);
    pub fn is_skx_based_platform() -> c_int;
    pub fn is_spr_platform() -> c_int;
    pub fn is_emr_platform() -> c_int;
    pub fn is_icx_platform() -> c_int;
    pub fn isst_trl_display_information(id: *mut isst_id, outf: *mut FILE, trl: c_ulonglong);

    pub fn set_cpu_online_offline(cpu: c_int, state: c_int);
    pub fn for_each_online_power_domain_in_set(
        callback: Option<
            unsafe extern "C" fn(
                id: *mut isst_id,
                arg1: *mut c_void,
                arg2: *mut c_void,
                arg3: *mut c_void,
                arg4: *mut c_void,
            ),
        >,
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
    );
    pub fn isst_daemon(debug_mode: c_int, poll_interval: c_int, no_daemon: c_int) -> c_int;
    pub fn process_level_change(id: *mut isst_id);
    pub fn hfi_main() -> c_int;
    pub fn hfi_exit();

    /* Interface specific callbacks */
    pub fn mbox_get_platform_ops() -> *mut isst_platform_ops;
    pub fn tpmi_get_platform_ops() -> *mut isst_platform_ops;

    /* Cgroup related interface */
    pub fn enable_cpuset_controller() -> c_int;
    pub fn isolate_cpus(
        id: *mut isst_id,
        mask_size: c_int,
        cpu_mask: *mut cpu_set_t,
        level: c_int,
        cpu_0_only: c_int,
    ) -> c_int;
    pub fn use_cgroupv2() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
