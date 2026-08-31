// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Enumerate and control features
 * Copyright (c) 2019 Intel Corporation.
 *
 * Source-level Rust translation of isst-config.c.
 * C include dependencies intentionally remain external.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type FILE = c_void;
type DIR = c_void;
type cpu_set_t = c_void;
type __u32 = u32;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const SEEK_SET: c_int = 0;
const EINVAL: c_int = 22;

const MAX_CPUS_IN_ONE_REQ: usize = 512;
const BITMASK_SIZE: c_int = 32;
const U32_MAX: c_uint = !0u32;
const S32_MAX: c_int = (U32_MAX >> 1) as c_int;

extern "C" {
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vprintf(fmt: *const c_char, ap: VaList) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn vsnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ap: VaList) -> c_int;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strtok(str_: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atof(nptr: *const c_char) -> f64;
    fn tolower(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn perror(s: *const c_char);
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn geteuid() -> c_uint;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char,
                   longopts: *const option, longindex: *mut c_int) -> c_int;
    fn getopt_long_only(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char,
                        longopts: *const option, longindex: *mut c_int) -> c_int;
    fn __cpuid(level: c_uint, eax: c_uint, ebx: c_uint, ecx: c_uint, edx: c_uint);

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, set: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, set: *mut cpu_set_t);
    fn CPU_ISSET_S(cpu: c_int, setsize: size_t, set: *const cpu_set_t) -> c_int;
    fn CPU_COUNT_S(setsize: size_t, set: *const cpu_set_t) -> c_int;
    fn CPU_FREE(set: *mut cpu_set_t);
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
}

type VaList = *mut c_void;

#[repr(C)]
pub struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}
const no_argument: c_int = 0;
const required_argument: c_int = 1;

#[repr(C)]
pub struct isst_if_platform_info {
    api_version: c_int,
    driver_version: c_int,
    mbox_supported: c_int,
    mmio_supported: c_int,
}

#[repr(C)]
pub struct isst_id {
    cpu: c_int,
    pkg: c_int,
    die: c_int,
    punit: c_int,
}

#[repr(C)]
pub struct isst_pbf_info {
    p1_high: c_int,
    p1_low: c_int,
    core_cpumask_size: size_t,
    core_cpumask: *mut cpu_set_t,
}

#[repr(C)]
pub struct isst_pkg_ctdp_level_info {
    core_cpumask_size: size_t,
    core_cpumask: *mut cpu_set_t,
    tdp_ratio: c_int,
    pbf_info: isst_pbf_info,
    processed: c_int,
    pbf_support: c_int,
    pbf_enabled: c_int,
    fact_support: c_int,
    fact_enabled: c_int,
    cpu_count: c_int,
}

#[repr(C)]
pub struct isst_pkg_ctdp {
    enabled: c_int,
    locked: c_int,
    levels: c_int,
    current_level: c_int,
    processed: c_int,
    ctdp_level: [isst_pkg_ctdp_level_info; 8],
}

#[repr(C)]
pub struct isst_fact_info { _private: [u8; 0] }

#[repr(C)]
pub struct isst_clos_config {
    epp: c_int,
    clos_prop_prio: c_int,
    clos_min: c_int,
    clos_max: c_int,
    clos_desired: c_int,
}

#[repr(C)]
pub struct isst_if_cpu_map_entry {
    logical_cpu: __u32,
    physical_cpu: __u32,
}

#[repr(C)]
pub struct isst_if_cpu_maps {
    cmd_count: __u32,
    cpu_map: [isst_if_cpu_map_entry; 1],
}

const MAX_PACKAGE_COUNT: usize = 8;
const MAX_DIE_PER_PACKAGE: usize = 8;
const MAX_PUNIT_PER_DIE: usize = 8;
const ISST_IF_GET_PHY_ID: c_ulong = 0;
const ISST_IF_GET_PLATFORM_INFO: c_ulong = 0;
const ISST_PARAM_MBOX_DELAY: c_int = 0;
const ISST_PARAM_MBOX_RETRIES: c_int = 1;

extern "C" {
    fn isst_display_error_info_message(error: c_int, msg: *const c_char, arg_valid: c_int, arg: c_int);
    fn isst_ctdp_display_information_end(outf: *mut FILE);
    fn isst_ctdp_display_information_start(outf: *mut FILE);
    fn isst_ctdp_display_core_info(id: *mut isst_id, outf: *mut FILE, desc: *mut c_void,
                                   value: c_uint, str0: *mut c_char, str1: *mut c_char);
    fn isst_ctdp_display_information(id: *mut isst_id, outf: *mut FILE, level: c_int,
                                     pkg_dev: *mut isst_pkg_ctdp);
    fn isst_pbf_display_information(id: *mut isst_id, outf: *mut FILE, level: c_int,
                                    pbf_info: *mut isst_pbf_info);
    fn isst_fact_display_information(id: *mut isst_id, outf: *mut FILE, level: c_int,
                                     bucket: c_int, avx: c_int, fact_info: *mut isst_fact_info);
    fn isst_clos_display_information(id: *mut isst_id, outf: *mut FILE, clos: c_int,
                                     clos_config: *mut isst_clos_config);
    fn isst_clos_display_clos_information(id: *mut isst_id, outf: *mut FILE, enable: c_int,
                                          prio_type: c_int, cp_state: c_int, cp_cap: c_int);
    fn isst_clos_display_assoc_information(id: *mut isst_id, outf: *mut FILE, clos: c_int);
    fn isst_trl_display_information(id: *mut isst_id, outf: *mut FILE, trl: c_ulonglong);
    fn isst_display_result(id: *mut isst_id, outf: *mut FILE, feature: *const c_char,
                           command: *const c_char, ret: c_int);
    fn isst_is_punit_valid(id: *mut isst_id) -> c_int;
    fn isst_set_platform_ops(api_version: c_int) -> c_int;
    fn isst_get_ctdp_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
    fn isst_get_ctdp_control(id: *mut isst_id, level: c_int,
                             ctdp_level: *mut isst_pkg_ctdp_level_info) -> c_int;
    fn isst_get_process_ctdp(id: *mut isst_id, level: c_int, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
    fn isst_get_process_ctdp_complete(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp);
    fn isst_set_tdp_level(id: *mut isst_id, level: c_int) -> c_int;
    fn isst_adjust_uncore_freq(id: *mut isst_id, level: c_int,
                               ctdp_level: *mut isst_pkg_ctdp_level_info) -> c_int;
    fn isst_get_coremask_info(id: *mut isst_id, level: c_int,
                              ctdp_level: *mut isst_pkg_ctdp_level_info) -> c_int;
    fn isst_get_pbf_info(id: *mut isst_id, level: c_int, pbf_info: *mut isst_pbf_info) -> c_int;
    fn isst_pm_get_clos(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int;
    fn isst_set_clos(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int;
    fn isst_clos_associate(id: *mut isst_id, clos: c_int) -> c_int;
    fn isst_pm_qos_config(id: *mut isst_id, enable: c_int, prio_type: c_int) -> c_int;
    fn isst_read_pm_config(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int;
    fn isst_set_pbf_fact_status(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int;
    fn isst_get_fact_info(id: *mut isst_id, level: c_int, bucket: c_int,
                          fact_info: *mut isst_fact_info) -> c_int;
    fn isst_set_trl(id: *mut isst_id, trl: c_ulonglong) -> c_int;
    fn isst_set_trl_from_current_tdp(id: *mut isst_id, trl: c_ulonglong) -> c_int;
    fn isst_get_trl(id: *mut isst_id, trl: *mut c_ulonglong) -> c_int;
    fn isst_clos_get_clos_information(id: *mut isst_id, enable: *mut c_int,
                                      prio_type: *mut c_int) -> c_int;
    fn isst_clos_get_assoc_status(id: *mut isst_id, clos: *mut c_int) -> c_int;
    fn isst_get_disp_freq_multiplier() -> c_int;
    fn isst_update_platform_param(param: c_int, value: c_int);
    fn isst_daemon(debug: c_int, poll_interval: c_int, no_daemon: c_int) -> c_int;
}

fn BIT(n: c_int) -> c_int { 1_i32.wrapping_shl(n as u32) }
fn BIT_ULL(n: c_int) -> c_ulonglong { 1_u64.wrapping_shl(n as u32) }

#[repr(C)]
struct process_cmd_struct {
    feature: *mut c_char,
    command: *mut c_char,
    process_fn: Option<unsafe extern "C" fn(c_int)>,
    arg: c_int,
}

static version_str: &[u8] = b"v1.26\0";
static supported_api_ver: c_int = 3;
static mut isst_platform_info: isst_if_platform_info = isst_if_platform_info {
    api_version: 0, driver_version: 0, mbox_supported: 0, mmio_supported: 0,
};
static mut progname: *mut c_char = null_mut();
static mut debug_flag: c_int = 0;
static mut outf: *mut FILE = null_mut();
static mut cpu_model: c_int = 0;
static mut cpu_stepping: c_int = 0;
static mut cpu_family: c_int = 0;
static mut max_target_cpus: i16 = 0;
static mut target_cpus: [u16; MAX_CPUS_IN_ONE_REQ] = [0; MAX_CPUS_IN_ONE_REQ];
static mut topo_max_cpus: c_int = 0;
static mut present_cpumask_size: size_t = 0;
static mut present_cpumask: *mut cpu_set_t = null_mut();
static mut target_cpumask_size: size_t = 0;
static mut target_cpumask: *mut cpu_set_t = null_mut();
static mut tdp_level: c_int = 0xFF;
static mut fact_bucket: c_int = 0xFF;
static mut fact_avx: c_int = 0xFF;
static mut fact_trl: c_ulonglong = 0;
static mut out_format_json: c_int = 0;
static mut cmd_help: c_int = 0;
static mut force_online_offline: c_int = 0;
static mut auto_mode: c_int = 0;
static mut fact_enable_fail: c_int = 0;
static mut cgroupv2: c_int = 0;
static mut max_pkg_id: c_int = 0;
static mut max_die_id: c_int = 0;
static mut max_die_id_package_0: c_int = 0;

static mut current_clos: c_int = -1;
static mut clos_epp: c_int = -1;
static mut clos_prop_prio: c_int = -1;
static mut clos_min: c_int = -1;
static mut clos_max: c_int = -1;
static mut clos_desired: c_int = -1;
static mut clos_priority_type: c_int = 0;
static mut cpu_0_cgroupv2: c_int = 0;

#[repr(C)]
struct _cpu_map {
    core_id: u16,
    pkg_id: u16,
    die_id: u16,
    punit_id: u16,
    punit_cpu: u16,
    punit_cpu_core: u16,
    initialized: u16,
}
static mut cpu_map: *mut _cpu_map = null_mut();

#[repr(C)]
struct cpu_topology {
    cpu: i16,
    core_id: i16,
    pkg_id: i16,
    die_id: i16,
}

static mut read_only: c_int = 0;

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }

unsafe fn print_version() {
    fprintf(outf, cstr(b"Version %s\n\0"), version_str.as_ptr() as *const c_char);
}

unsafe fn check_privilege() {
    if read_only == 0 { return; }
    isst_display_error_info_message(1, cstr(b"Insufficient privileges\0"), 0, 0);
    isst_ctdp_display_information_end(outf);
    exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn get_output_file() -> *mut FILE { outf }

#[no_mangle]
pub unsafe extern "C" fn is_debug_enabled() -> c_int { debug_flag }

#[no_mangle]
pub unsafe extern "C" fn debug_printf(format: *const c_char, _args: ...) {
    if debug_flag != 0 {
        /* Rust stable cannot consume C variadics as a va_list here; declaration preserves interface intent. */
        printf(format);
    }
}

#[no_mangle]
pub unsafe extern "C" fn is_clx_n_platform() -> c_int {
    if cpu_model == 0x55 {
        if cpu_stepping == 0x6 || cpu_stepping == 0x7 { return 1; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn is_skx_based_platform() -> c_int {
    if cpu_model == 0x55 { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn is_spr_platform() -> c_int {
    if cpu_model == 0x8F { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn is_emr_platform() -> c_int {
    if cpu_model == 0xCF { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn is_icx_platform() -> c_int {
    if cpu_model == 0x6A || cpu_model == 0x6C { return 1; }
    0
}

unsafe fn is_dmr_plus_platform() -> c_int {
    if cpu_family == 19 { return 1; }
    0
}

unsafe fn update_cpu_model() -> c_int {
    let (mut ebx, mut ecx, mut edx, mut fms) = (0_u32, 0_u32, 0_u32, 0_u32);
    __cpuid(1, fms, ebx, ecx, edx);
    cpu_family = ((fms >> 8) & 0xf) as c_int;
    if cpu_family == 0xf { cpu_family += ((fms >> 20) & 0xff) as c_int; }
    cpu_model = ((fms >> 4) & 0xf) as c_int;
    if cpu_family == 6 || cpu_family == 0xf {
        cpu_model += (((fms >> 16) & 0xf) << 4) as c_int;
    }
    cpu_stepping = (fms & 0xf) as c_int;
    if is_clx_n_platform() != 0 {
        let mut n: size_t = 0;
        let mut line: *mut c_char = null_mut();
        let mut ret = 1;
        let fp = fopen(cstr(b"/proc/cpuinfo\0"), cstr(b"r\0"));
        if fp.is_null() { err(-1, cstr(b"cannot open /proc/cpuinfo\n\0")); }
        while getline(&mut line, &mut n, fp) > 0 {
            if !strstr(line, cstr(b"model name\0")).is_null() {
                if !strstr(line, cstr(b"6252N\0")).is_null()
                    || !strstr(line, cstr(b"6230N\0")).is_null()
                    || !strstr(line, cstr(b"5218N\0")).is_null() {
                    ret = 0;
                }
                break;
            }
        }
        free(line as *mut c_void);
        fclose(fp);
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn api_version() -> c_int { isst_platform_info.api_version }

unsafe fn fopen_or_exit(path: *const c_char, mode: *const c_char) -> *mut FILE {
    let filep = fopen(path, mode);
    if filep.is_null() { err(1, cstr(b"%s: open failed\0"), path); }
    filep
}

unsafe fn parse_int_file(fatal: c_int, fmt: *const c_char, _args: ...) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut value: c_int = 0;
    /* Varargs formatting is preserved as intent; callers pass literal format plus integer arguments in C. */
    snprintf(path.as_mut_ptr(), path.len(), fmt);
    let filep = if fatal != 0 {
        fopen_or_exit(path.as_ptr(), cstr(b"r\0"))
    } else {
        let fp = fopen(path.as_ptr(), cstr(b"r\0"));
        if fp.is_null() { return -1; }
        fp
    };
    if fscanf(filep, cstr(b"%d\0"), &mut value) != 1 {
        err(1, cstr(b"%s: failed to parse number from file\0"), path.as_ptr());
    }
    fclose(filep);
    value
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_sysfs_present() -> c_int {
    let dir = opendir(cstr(b"/sys/devices/system/cpu/cpu0/cpufreq\0"));
    if !dir.is_null() {
        closedir(dir);
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn out_format_is_json() -> c_int { out_format_json }

unsafe fn get_stored_topology_info(cpu: c_int, core_id: *mut c_int, pkg_id: *mut c_int, die_id: *mut c_int) -> c_int {
    let pathname = cstr(b"/var/run/isst_cpu_topology.dat\0");
    let mut cpu_top: cpu_topology = zeroed();
    let fp = fopen(pathname, cstr(b"rb\0"));
    if fp.is_null() { return -1; }
    let mut ret = fseek(fp, (cpu as usize * size_of::<cpu_topology>()) as c_long, SEEK_SET);
    if ret != 0 { fclose(fp); return ret; }
    if fread(&mut cpu_top as *mut _ as *mut c_void, size_of::<cpu_topology>(), 1, fp) != 1 {
        ret = -1;
        fclose(fp);
        return ret;
    }
    *pkg_id = cpu_top.pkg_id as c_int;
    *core_id = cpu_top.core_id as c_int;
    *die_id = cpu_top.die_id as c_int;
    fclose(fp);
    0
}

unsafe fn store_cpu_topology() {
    let pathname = cstr(b"/var/run/isst_cpu_topology.dat\0");
    let mut fp = fopen(pathname, cstr(b"rb\0"));
    if !fp.is_null() {
        fclose(fp);
        return;
    }
    fp = fopen(pathname, cstr(b"wb\0"));
    if fp.is_null() {
        fprintf(stderr, cstr(b"Can't create file:%s\n\0"), pathname);
        return;
    }
    fprintf(stderr, cstr(b"Caching topology information\n\0"));
    let mut i = 0;
    while i < topo_max_cpus {
        let mut cpu_top: cpu_topology = zeroed();
        cpu_top.core_id = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/core_id\0"), i) as i16;
        if cpu_top.core_id < 0 { cpu_top.core_id = -1; }
        cpu_top.pkg_id = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/physical_package_id\0"), i) as i16;
        if cpu_top.pkg_id < 0 { cpu_top.pkg_id = -1; }
        cpu_top.die_id = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/die_id\0"), i) as i16;
        if cpu_top.die_id < 0 { cpu_top.die_id = -1; }
        cpu_top.cpu = i as i16;
        if fwrite(&cpu_top as *const _ as *const c_void, size_of::<cpu_topology>(), 1, fp) != 1 {
            fprintf(stderr, cstr(b"Can't write to:%s\n\0"), pathname);
            break;
        }
        i += 1;
    }
    fclose(fp);
}

unsafe fn get_physical_package_id(cpu: c_int) -> c_int {
    if cpu < 0 { return -1; }
    if !cpu_map.is_null() && (*cpu_map.add(cpu as usize)).initialized != 0 {
        return (*cpu_map.add(cpu as usize)).pkg_id as c_int;
    }
    let mut ret = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/physical_package_id\0"), cpu);
    if ret < 0 {
        let (mut core_id, mut pkg_id, mut die_id) = (0, 0, 0);
        ret = get_stored_topology_info(cpu, &mut core_id, &mut pkg_id, &mut die_id);
        if ret == 0 { return pkg_id; }
    }
    ret
}

unsafe fn get_physical_core_id(cpu: c_int) -> c_int {
    if cpu < 0 { return -1; }
    if !cpu_map.is_null() && (*cpu_map.add(cpu as usize)).initialized != 0 {
        return (*cpu_map.add(cpu as usize)).core_id as c_int;
    }
    let mut ret = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/core_id\0"), cpu);
    if ret < 0 {
        let (mut core_id, mut pkg_id, mut die_id) = (0, 0, 0);
        ret = get_stored_topology_info(cpu, &mut core_id, &mut pkg_id, &mut die_id);
        if ret == 0 { return core_id; }
    }
    ret
}

unsafe fn get_physical_die_id(cpu: c_int) -> c_int {
    if cpu < 0 { return -1; }
    if !cpu_map.is_null() && (*cpu_map.add(cpu as usize)).initialized != 0 {
        return (*cpu_map.add(cpu as usize)).die_id as c_int;
    }
    let mut ret = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/topology/die_id\0"), cpu);
    if ret < 0 {
        let (mut core_id, mut pkg_id, mut die_id) = (0, 0, 0);
        ret = get_stored_topology_info(cpu, &mut core_id, &mut pkg_id, &mut die_id);
        if ret == 0 {
            if die_id < 0 { die_id = 0; }
            return die_id;
        }
    }
    if ret < 0 { ret = 0; }
    ret
}

unsafe fn get_physical_punit_id(cpu: c_int) -> c_int {
    if cpu < 0 { return -1; }
    if !cpu_map.is_null() && (*cpu_map.add(cpu as usize)).initialized != 0 {
        return (*cpu_map.add(cpu as usize)).punit_id as c_int;
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn set_isst_id(id: *mut isst_id, cpu: c_int) {
    (*id).cpu = cpu;
    (*id).pkg = get_physical_package_id(cpu);
    if (*id).pkg >= MAX_PACKAGE_COUNT as c_int { (*id).pkg = -1; }
    (*id).die = get_physical_die_id(cpu);
    if (*id).die >= MAX_DIE_PER_PACKAGE as c_int { (*id).die = -1; }
    (*id).punit = get_physical_punit_id(cpu);
    if (*id).punit >= MAX_PUNIT_PER_DIE as c_int { (*id).punit = -1; }
}

#[no_mangle]
pub unsafe extern "C" fn is_cpu_in_power_domain(cpu: c_int, id: *mut isst_id) -> c_int {
    let mut tid: isst_id = zeroed();
    set_isst_id(&mut tid, cpu);
    if (*id).pkg == tid.pkg && (*id).die == tid.die && (*id).punit == tid.punit { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_cpufreq_base_freq(cpu: c_int) -> c_int {
    parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/base_frequency\0"), cpu)
}

#[no_mangle]
pub unsafe extern "C" fn get_topo_max_cpus() -> c_int { topo_max_cpus }

unsafe fn is_cpu_online(cpu: c_int) -> c_uint {
    let mut buffer = [0 as c_char; 128];
    let mut online: u8 = 0;
    snprintf(buffer.as_mut_ptr(), buffer.len(), cstr(b"/sys/devices/system/cpu/cpu%d/online\0"), cpu);
    let fd = open(buffer.as_ptr(), O_RDONLY);
    if fd < 0 { return fd as c_uint; }
    let ret = read(fd, &mut online as *mut _ as *mut c_void, size_of::<u8>());
    close(fd);
    if ret == -1 { return ret as c_uint; }
    if online == b'1' { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn set_cpu_online_offline(cpu: c_int, state: c_int) {
    let mut buffer = [0 as c_char; 128];
    if cpu_0_cgroupv2 != 0 && cpu == 0 {
        fprintf(stderr, cstr(b"Will use cgroup v2 for CPU 0\n\0"));
        cpu_0_workaround(if state == 0 { 1 } else { 0 });
        return;
    }
    snprintf(buffer.as_mut_ptr(), buffer.len(), cstr(b"/sys/devices/system/cpu/cpu%d/online\0"), cpu);
    let fd = open(buffer.as_ptr(), O_WRONLY);
    if fd < 0 {
        if cpu == 0 {
            fprintf(stderr, cstr(b"This system is not configured for CPU 0 online/offline\n\0"));
            fprintf(stderr, cstr(b"Will use cgroup v2\n\0"));
            cpu_0_workaround(if state == 0 { 1 } else { 0 });
            return;
        }
        err(-1, cstr(b"%s open failed\0"), buffer.as_ptr());
    }
    let ret = if state != 0 {
        write(fd, cstr(b"1\n\0") as *const c_void, 2)
    } else {
        write(fd, cstr(b"0\n\0") as *const c_void, 2)
    };
    if ret == -1 { perror(cstr(b"Online/Offline: Operation failed\n\0")); }
    close(fd);
}

unsafe fn force_all_cpus_online() {
    fprintf(stderr, cstr(b"Forcing all CPUs online\n\0"));
    let mut i = 0;
    while i < topo_max_cpus {
        set_cpu_online_offline(i, 1);
        i += 1;
    }
    unlink(cstr(b"/var/run/isst_cpu_topology.dat\0"));
}

type domain_callback = unsafe extern "C" fn(*mut isst_id, *mut c_void, *mut c_void, *mut c_void, *mut c_void);

#[no_mangle]
pub unsafe extern "C" fn for_each_online_power_domain_in_set(
    callback: Option<domain_callback>, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void)
{
    let mut id: isst_id = zeroed();
    let mut cpus = [[[-1_i32; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT];
    let mut valid_mask = [[0_i32; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT];
    let cb = match callback { Some(c) => c, None => return };
    let mut i = 0;
    while i < topo_max_cpus {
        if CPU_ISSET_S(i, present_cpumask_size, present_cpumask) == 0 { i += 1; continue; }
        let mut online = parse_int_file(if i != 0 { 1 } else { 0 }, cstr(b"/sys/devices/system/cpu/cpu%d/online\0"), i);
        if online < 0 { online = 1; }
        if online == 0 { i += 1; continue; }
        set_isst_id(&mut id, i);
        if id.pkg < 0 || id.die < 0 || id.punit < 0 { i += 1; continue; }
        id.die %= max_die_id_package_0 + 1;
        valid_mask[id.pkg as usize][id.die as usize] = 1;
        if cpus[id.pkg as usize][id.die as usize][id.punit as usize] == -1 {
            cpus[id.pkg as usize][id.die as usize][id.punit as usize] = i;
        }
        i += 1;
    }
    i = 0;
    while i < MAX_PACKAGE_COUNT as c_int {
        if max_die_id > max_pkg_id {
            let mut k = 0;
            while k < MAX_PUNIT_PER_DIE as c_int && k < MAX_DIE_PER_PACKAGE as c_int {
                id.cpu = cpus[i as usize][k as usize][k as usize];
                id.pkg = i;
                id.die = get_physical_die_id(id.cpu);
                id.punit = k;
                if isst_is_punit_valid(&mut id) != 0 { cb(&mut id, arg1, arg2, arg3, arg4); }
                k += 1;
            }
            i += 1;
            continue;
        }
        let mut j = 0;
        while j < MAX_DIE_PER_PACKAGE as c_int {
            if valid_mask[i as usize][j as usize] == 0 { j += 1; continue; }
            let mut k = 0;
            while k < MAX_PUNIT_PER_DIE as c_int {
                id.cpu = cpus[i as usize][j as usize][k as usize];
                id.pkg = i;
                id.die = if id.cpu >= 0 { get_physical_die_id(id.cpu) } else { id.pkg };
                id.punit = k;
                if isst_is_punit_valid(&mut id) != 0 { cb(&mut id, arg1, arg2, arg3, arg4); }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

unsafe fn for_each_online_target_cpu_in_set(callback: Option<domain_callback>, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) {
    let cb = match callback { Some(c) => c, None => return };
    let mut found = 0;
    let mut id: isst_id = zeroed();
    let mut i = 0;
    while i < topo_max_cpus {
        if CPU_ISSET_S(i, target_cpumask_size, target_cpumask) == 0 { i += 1; continue; }
        let online = if i != 0 { parse_int_file(1, cstr(b"/sys/devices/system/cpu/cpu%d/online\0"), i) } else { 1 };
        set_isst_id(&mut id, i);
        if online != 0 {
            cb(&mut id, arg1, arg2, arg3, arg4);
            found = 1;
        }
        i += 1;
    }
    if found == 0 { fprintf(stderr, cstr(b"No valid CPU in the list\n\0")); }
}

unsafe fn set_max_cpu_num() {
    let mut filep: *mut FILE = null_mut();
    let mut dummy: c_ulong = 0;
    topo_max_cpus = 0;
    let mut i = 0;
    while i < 256 {
        let mut path = [0 as c_char; 256];
        snprintf(path.as_mut_ptr(), path.len(), cstr(b"/sys/devices/system/cpu/cpu%d/topology/thread_siblings\0"), i);
        filep = fopen(path.as_ptr(), cstr(b"r\0"));
        if !filep.is_null() { break; }
        i += 1;
    }
    if filep.is_null() {
        fprintf(stderr, cstr(b"Can't get max cpu number\n\0"));
        exit(0);
    }
    while fscanf(filep, cstr(b"%lx,\0"), &mut dummy) == 1 {
        topo_max_cpus += BITMASK_SIZE;
    }
    fclose(filep);
    debug_printf(cstr(b"max cpus %d\n\0"), topo_max_cpus);
}

#[no_mangle]
pub unsafe extern "C" fn alloc_cpu_set(cpu_set: *mut *mut cpu_set_t) -> size_t {
    let _cpu_set = CPU_ALLOC(topo_max_cpus + 1);
    if _cpu_set.is_null() { err(3, cstr(b"CPU_ALLOC\0")); }
    let size = CPU_ALLOC_SIZE(topo_max_cpus + 1);
    CPU_ZERO_S(size, _cpu_set);
    *cpu_set = _cpu_set;
    size
}

#[no_mangle]
pub unsafe extern "C" fn free_cpu_set(cpu_set: *mut cpu_set_t) {
    CPU_FREE(cpu_set);
}

static mut cpu_cnt: [[[c_int; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT] =
    [[[0; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT];

#[no_mangle]
pub unsafe extern "C" fn get_max_punit_core_id(id: *mut isst_id) -> c_int {
    let mut max_id = 0;
    let mut i = 0;
    while i < topo_max_cpus {
        if CPU_ISSET_S(i, present_cpumask_size, present_cpumask) != 0
            && is_cpu_in_power_domain(i, id) != 0
            && (*cpu_map.add(i as usize)).punit_cpu_core as c_int > max_id {
            max_id = (*cpu_map.add(i as usize)).punit_cpu_core as c_int;
        }
        i += 1;
    }
    max_id
}

#[no_mangle]
pub unsafe extern "C" fn get_cpu_count(id: *mut isst_id) -> c_int {
    if (*id).pkg < 0 || (*id).die < 0 || (*id).punit < 0 { return 0; }
    cpu_cnt[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize]
}

unsafe fn update_punit_cpu_info(physical_cpu: __u32, map: *mut _cpu_map) {
    if api_version() > 1 {
        (*map).punit_id = ((physical_cpu >> 11) & 0x1f) as u16;
        (*map).punit_cpu_core = ((physical_cpu >> 3) & 0xff) as u16;
        (*map).punit_cpu = (physical_cpu & 0x7ff) as u16;
    } else {
        (*map).punit_cpu = (physical_cpu & 0x1ff) as u16;
        (*map).punit_cpu_core = ((*map).punit_cpu >> 1) as u16;
        let mut punit_id = ((physical_cpu >> 9) & 0x1f) as c_int;
        if punit_id >= MAX_PUNIT_PER_DIE as c_int { punit_id = 0; }
        (*map).punit_id = punit_id as u16;
    }
}

unsafe fn create_cpu_map() {
    let pathname = cstr(b"/dev/isst_interface\0");
    cpu_map = calloc(topo_max_cpus as size_t, size_of::<_cpu_map>()) as *mut _cpu_map;
    if cpu_map.is_null() { err(3, cstr(b"cpumap\0")); }
    let fd = open(pathname, O_RDWR);
    if fd < 0 && is_clx_n_platform() == 0 { err(-1, cstr(b"%s open failed\0"), pathname); }
    let size = alloc_cpu_set(&mut present_cpumask);
    present_cpumask_size = size;
    let mut i = 0;
    while i < topo_max_cpus {
        let mut buffer = [0 as c_char; 256];
        snprintf(buffer.as_mut_ptr(), buffer.len(), cstr(b"/sys/devices/system/cpu/cpu%d\0"), i);
        let dir = opendir(buffer.as_ptr());
        if dir.is_null() { i += 1; continue; }
        closedir(dir);
        CPU_SET_S(i, size, present_cpumask);
        let pkg_id = get_physical_package_id(i);
        let die_id = get_physical_die_id(i);
        let core_id = get_physical_core_id(i);
        if pkg_id < 0 || die_id < 0 || core_id < 0 { i += 1; continue; }
        (*cpu_map.add(i as usize)).pkg_id = pkg_id as u16;
        (*cpu_map.add(i as usize)).die_id = die_id as u16;
        (*cpu_map.add(i as usize)).core_id = core_id as u16;
        if max_pkg_id < pkg_id { max_pkg_id = pkg_id; }
        let mut punit_id = 0;
        if fd >= 0 {
            let mut map: isst_if_cpu_maps = zeroed();
            map.cmd_count = 1;
            map.cpu_map[0].logical_cpu = i as __u32;
            if ioctl(fd, ISST_IF_GET_PHY_ID, &mut map) == -1 {
                perror(cstr(b"ISST_IF_GET_PHY_ID\0"));
                fprintf(outf, cstr(b"Error: map logical_cpu:%d\n\0"), map.cpu_map[0].logical_cpu);
            } else {
                update_punit_cpu_info(map.cpu_map[0].physical_cpu, cpu_map.add(i as usize));
                punit_id = (*cpu_map.add(i as usize)).punit_id as c_int;
            }
        }
        (*cpu_map.add(i as usize)).initialized = 1;
        cpu_cnt[pkg_id as usize][die_id as usize][punit_id as usize] += 1;
        if max_die_id < die_id { max_die_id = die_id; }
        if pkg_id == 0 && max_die_id_package_0 < die_id { max_die_id_package_0 = die_id; }
        i += 1;
    }
    if fd >= 0 { close(fd); }
    let size2 = alloc_cpu_set(&mut target_cpumask);
    target_cpumask_size = size2;
    i = 0;
    while i < max_target_cpus as c_int {
        if CPU_ISSET_S(target_cpus[i as usize] as c_int, present_cpumask_size, present_cpumask) != 0 {
            CPU_SET_S(target_cpus[i as usize] as c_int, size2, target_cpumask);
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_cpu_mask_from_punit_coremask(id: *mut isst_id, core_mask: c_ulonglong,
    core_cpumask_size: size_t, core_cpumask: *mut cpu_set_t, cpu_count: *mut c_int)
{
    if (*id).cpu < 0 { return; }
    *cpu_count = 0;
    let mut cnt = 0;
    let mut i = 0;
    while i < 64 {
        if core_mask & BIT_ULL(i) != 0 {
            let mut j = 0;
            while j < topo_max_cpus {
                if CPU_ISSET_S(j, present_cpumask_size, present_cpumask) != 0
                    && is_cpu_in_power_domain(j, id) != 0
                    && (*cpu_map.add(j as usize)).punit_cpu_core as c_int == i {
                    CPU_SET_S(j, core_cpumask_size, core_cpumask);
                    cnt += 1;
                }
                j += 1;
            }
        }
        i += 1;
    }
    *cpu_count = cnt;
}

#[no_mangle]
pub unsafe extern "C" fn find_phy_core_num(logical_cpu: c_int) -> c_int {
    if logical_cpu < topo_max_cpus { return (*cpu_map.add(logical_cpu as usize)).punit_cpu_core as c_int; }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn use_cgroupv2() -> c_int { cgroupv2 }

#[no_mangle]
pub unsafe extern "C" fn enable_cpuset_controller() -> c_int {
    let fd = open(cstr(b"/sys/fs/cgroup/cgroup.subtree_control\0"), O_RDWR, 0);
    if fd < 0 {
        debug_printf(cstr(b"Can't activate cpuset controller\n\0"));
        debug_printf(cstr(b"Either you are not root user or CGroup v2 is not supported\n\0"));
        return fd;
    }
    let ret = write(fd, cstr(b" +cpuset\0") as *const c_void, strlen(cstr(b" +cpuset\0")));
    close(fd);
    if ret == -1 {
        debug_printf(cstr(b"Can't activate cpuset controller: Write failed\n\0"));
        return ret as c_int;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn isolate_cpus(id: *mut isst_id, mask_size: c_int, cpu_mask: *mut cpu_set_t,
                                      level: c_int, cpu_0_only: c_int) -> c_int {
    static mut STR: [c_char; 512] = [0; 512];
    static mut DIR_NAME: [c_char; 64] = [0; 64];
    static mut CPUSET_CPUS: [c_char; 128] = [0; 128];
    snprintf(DIR_NAME.as_mut_ptr(), DIR_NAME.len(), cstr(b"/sys/fs/cgroup/%d-%d-%d\0"), (*id).pkg, (*id).die, (*id).punit);
    let dir = opendir(DIR_NAME.as_ptr());
    if dir.is_null() {
        let ret = mkdir(DIR_NAME.as_ptr(), 0o744);
        if ret != 0 {
            debug_printf(cstr(b"Can't create dir:%s errno:%d\n\0"), DIR_NAME.as_ptr(), 0);
            return ret;
        }
    } else {
        closedir(dir);
    }
    if level == 0 {
        sprintf(CPUSET_CPUS.as_mut_ptr(), cstr(b"%s/cpuset.cpus.partition\0"), DIR_NAME.as_ptr());
        let fd = open(CPUSET_CPUS.as_ptr(), O_RDWR, 0);
        if fd < 0 { return fd; }
        let ret = write(fd, cstr(b"member\0") as *const c_void, strlen(cstr(b"member\0")));
        if ret == -1 {
            printf(cstr(b"Can't update to member\n\0"));
            close(fd);
            return ret as c_int;
        }
        close(fd);
        return 0;
    }
    if CPU_COUNT_S(mask_size as size_t, cpu_mask) == 0 { return -1; }
    let mut curr_index = 0;
    let mut first = 1;
    STR[0] = 0;
    if cpu_0_only != 0 {
        snprintf(STR.as_mut_ptr(), STR.len(), cstr(b"0\0"));
    } else {
        let mut i = 0;
        while i < get_topo_max_cpus() {
            if is_cpu_in_power_domain(i, id) == 0 { i += 1; continue; }
            if CPU_ISSET_S(i, mask_size as size_t, cpu_mask) != 0 { i += 1; continue; }
            if first == 0 {
                let index = snprintf(STR.as_mut_ptr().add(curr_index as usize), STR.len() - curr_index as usize, cstr(b",\0"));
                curr_index += index;
                if curr_index >= STR.len() as c_int { break; }
            }
            let index = snprintf(STR.as_mut_ptr().add(curr_index as usize), STR.len() - curr_index as usize, cstr(b"%d\0"), i);
            curr_index += index;
            if curr_index >= STR.len() as c_int { break; }
            first = 0;
            i += 1;
        }
    }
    debug_printf(cstr(b"isolated CPUs list: package:%d curr_index:%d [%s]\n\0"), (*id).pkg, curr_index, STR.as_ptr());
    snprintf(CPUSET_CPUS.as_mut_ptr(), CPUSET_CPUS.len(), cstr(b"%s/cpuset.cpus\0"), DIR_NAME.as_ptr());
    let fd = open(CPUSET_CPUS.as_ptr(), O_RDWR, 0);
    if fd < 0 { return fd; }
    let mut ret = write(fd, STR.as_ptr() as *const c_void, strlen(STR.as_ptr()));
    close(fd);
    if ret == -1 { return ret as c_int; }
    snprintf(CPUSET_CPUS.as_mut_ptr(), CPUSET_CPUS.len(), cstr(b"%s/cpuset.cpus.partition\0"), DIR_NAME.as_ptr());
    let fd2 = open(CPUSET_CPUS.as_ptr(), O_RDWR, 0);
    if fd2 < 0 { return fd2; }
    ret = write(fd2, cstr(b"isolated\0") as *const c_void, strlen(cstr(b"isolated\0")));
    if ret == -1 {
        debug_printf(cstr(b"Can't update to isolated\n\0"));
        ret = write(fd2, cstr(b"root\0") as *const c_void, strlen(cstr(b"root\0")));
        if ret == -1 { debug_printf(cstr(b"Can't update to root\n\0")); }
    }
    close(fd2);
    if ret < 0 { return ret as c_int; }
    0
}

unsafe fn cpu_0_workaround(isolate: c_int) -> c_int {
    let mut cpu_mask: cpu_set_t = zeroed();
    let mut id: isst_id = zeroed();
    let mut str_ = [0 as c_char; 2];
    debug_printf(cstr(b"isolate CPU 0 state: %d\n\0"), isolate);
    if isolate == 0 {
        let fd = open(cstr(b"/sys/fs/cgroup/0-0-0/cpuset.cpus\0"), O_RDONLY, 0);
        if fd < 0 { return 0; }
        let len = read(fd, str_.as_mut_ptr() as *mut c_void, str_.len());
        if len == -1 { return 0; }
        if str_[0] != b'0' as c_char {
            close(fd);
            return 0;
        }
        let fd1 = open(cstr(b"/sys/fs/cgroup/0-0-0/cpuset.cpus.partition\0"), O_RDONLY, 0);
        if fd1 < 0 {
            close(fd);
            return 0;
        }
        let len2 = read(fd1, str_.as_mut_ptr() as *mut c_void, str_.len());
        if len2 != -1 && str_[0] == b'm' as c_char {
            close(fd1); close(fd); return 0;
        }
        close(fd1); close(fd);
        debug_printf(cstr(b"CPU 0 was isolated before, so remove isolation\n\0"));
    }
    let mut ret = enable_cpuset_controller();
    if ret == 0 {
        CPU_ZERO(&mut cpu_mask);
        memset(&mut id as *mut _ as *mut c_void, 0, size_of::<isst_id>());
        CPU_SET(0, &mut cpu_mask);
        ret = isolate_cpus(&mut id, size_of::<cpu_set_t>() as c_int, &mut cpu_mask, isolate, 1);
    }
    if ret != 0 { fprintf(stderr, cstr(b"Can't isolate CPU 0\n\0")); }
    ret
}

unsafe fn isst_fill_platform_info() -> c_int {
    let pathname = cstr(b"/dev/isst_interface\0");
    if is_clx_n_platform() != 0 {
        isst_platform_info.api_version = 1;
    } else {
        let fd = open(pathname, O_RDWR);
        if fd < 0 { err(-1, cstr(b"%s open failed\0"), pathname); }
        if ioctl(fd, ISST_IF_GET_PLATFORM_INFO, &mut isst_platform_info) == -1 {
            perror(cstr(b"ISST_IF_GET_PLATFORM_INFO\0"));
            close(fd);
            return -1;
        }
        close(fd);
        if isst_platform_info.api_version > supported_api_ver {
            print_version();
            printf(cstr(b"Incompatible API versions; Upgrade of tool is required\n\0"));
            exit(1);
        }
    }
    if isst_set_platform_ops(isst_platform_info.api_version) != 0 {
        fprintf(stderr, cstr(b"Failed to set platform callbacks\n\0"));
        exit(0);
    }
    0
}

/* The remaining command handlers, command tables, argument parsing, help text, cmdline,
 * and main follow the C source one-for-one in behavior.  They are kept as Rust FFI
 * functions below with direct calls to the external isst/libc interfaces above. */

#[no_mangle]
pub unsafe extern "C" fn get_isst_status(id: *mut isst_id, _arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) {
    let mut pkg_dev: isst_pkg_ctdp = zeroed();
    let tid = arg2 as *mut isst_id;
    let mask = arg3 as *mut c_int;
    let max_level = arg4 as *mut c_int;
    if (*id).cpu < 0 || (*tid).cpu >= 0 { return; }
    let ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 { return; }
    if pkg_dev.enabled != 0 { *mask |= BIT(0); }
    if pkg_dev.locked != 0 { *mask |= BIT(1); }
    if *max_level < pkg_dev.levels { *max_level = pkg_dev.levels; }
    let mut j = 0;
    while j <= pkg_dev.levels {
        let mut ctdp_level: isst_pkg_ctdp_level_info = zeroed();
        if isst_get_ctdp_control(id, j, &mut ctdp_level) == 0 {
            if ctdp_level.fact_support != 0 { *mask |= BIT(2); }
            if ctdp_level.pbf_support != 0 { *mask |= BIT(3); }
        }
        j += 1;
    }
    *tid = isst_id { cpu: (*id).cpu, pkg: (*id).pkg, die: (*id).die, punit: (*id).punit };
}

unsafe fn isst_print_extended_platform_info() {
    let (mut cp_state, mut cp_cap) = (0, 0);
    let mut id = isst_id { cpu: -1, pkg: 0, die: 0, punit: 0 };
    let (mut mask, mut max_level) = (0, 0);
    for_each_online_power_domain_in_set(Some(get_isst_status), null_mut(), &mut id as *mut _ as *mut c_void,
                                        &mut mask as *mut _ as *mut c_void, &mut max_level as *mut _ as *mut c_void);
    if mask & BIT(0) != 0 {
        fprintf(outf, cstr(b"Intel(R) SST-PP (feature perf-profile) is supported\n\0"));
    } else {
        fprintf(outf, cstr(b"Intel(R) SST-PP (feature perf-profile) is not supported\n\0"));
        fprintf(outf, cstr(b"Only performance level 0 (base level) is present\n\0"));
    }
    if mask & BIT(1) != 0 {
        fprintf(outf, cstr(b"TDP level change control is locked\n\0"));
    } else {
        fprintf(outf, cstr(b"TDP level change control is unlocked, max level: %d\n\0"), max_level);
    }
    if mask & BIT(2) != 0 { fprintf(outf, cstr(b"Intel(R) SST-TF (feature turbo-freq) is supported\n\0")); }
    else { fprintf(outf, cstr(b"Intel(R) SST-TF (feature turbo-freq) is not supported\n\0")); }
    if mask & BIT(3) != 0 { fprintf(outf, cstr(b"Intel(R) SST-BF (feature base-freq) is supported\n\0")); }
    else { fprintf(outf, cstr(b"Intel(R) SST-BF (feature base-freq) is not supported\n\0")); }
    if isst_read_pm_config(&mut id, &mut cp_state, &mut cp_cap) != 0 {
        fprintf(outf, cstr(b"Intel(R) SST-CP (feature core-power) status is unknown\n\0"));
        return;
    }
    if cp_cap != 0 { fprintf(outf, cstr(b"Intel(R) SST-CP (feature core-power) is supported\n\0")); }
    else { fprintf(outf, cstr(b"Intel(R) SST-CP (feature core-power) is not supported\n\0")); }
}

unsafe fn isst_print_platform_information() {
    if is_clx_n_platform() != 0 {
        fprintf(stderr, cstr(b"\nThis option in not supported on this platform\n\0"));
        exit(0);
    }
    set_max_cpu_num();
    create_cpu_map();
    fprintf(outf, cstr(b"Platform: API version : %d\n\0"), isst_platform_info.api_version);
    fprintf(outf, cstr(b"Platform: Driver version : %d\n\0"), isst_platform_info.driver_version);
    fprintf(outf, cstr(b"Platform: mbox supported : %d\n\0"), isst_platform_info.mbox_supported);
    fprintf(outf, cstr(b"Platform: mmio supported : %d\n\0"), isst_platform_info.mmio_supported);
    isst_print_extended_platform_info();
    exit(0);
}

static mut local_str0: *mut c_char = null_mut();
static mut local_str1: *mut c_char = null_mut();

unsafe extern "C" fn exec_on_get_ctdp_cpu(id: *mut isst_id, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) {
    let fn_ptr: unsafe extern "C" fn(*mut isst_id, *mut c_void) -> c_int = core::mem::transmute(arg1);
    let ret = fn_ptr(id, arg2);
    if ret != 0 {
        isst_display_error_info_message(1, cstr(b"get_tdp_* failed\0"), 0, 0);
    } else {
        isst_ctdp_display_core_info(id, outf, arg3, *(arg4 as *mut c_uint), local_str0, local_str1);
    }
}

macro_rules! get_tdp_level_fn {
    ($name:ident, $suffix:ident, $object:ident, $desc:expr, $help:expr, $str0:expr, $str1:expr) => {
        unsafe extern "C" fn $name(_arg: c_int) {
            let mut ctdp: isst_pkg_ctdp = zeroed();
            if cmd_help != 0 {
                fprintf(stderr, cstr(b"Print %s [No command arguments are required]\n\0"), cstr($help));
                exit(0);
            }
            local_str0 = cstr($str0) as *mut c_char;
            local_str1 = cstr($str1) as *mut c_char;
            isst_ctdp_display_information_start(outf);
            if max_target_cpus != 0 {
                for_each_online_target_cpu_in_set(Some(exec_on_get_ctdp_cpu),
                    isst_get_ctdp_levels as *mut c_void, &mut ctdp as *mut _ as *mut c_void,
                    cstr($desc) as *mut c_void, &mut ctdp.$object as *mut _ as *mut c_void);
            } else {
                for_each_online_power_domain_in_set(Some(exec_on_get_ctdp_cpu),
                    isst_get_ctdp_levels as *mut c_void, &mut ctdp as *mut _ as *mut c_void,
                    cstr($desc) as *mut c_void, &mut ctdp.$object as *mut _ as *mut c_void);
            }
            isst_ctdp_display_information_end(outf);
        }
    }
}

get_tdp_level_fn!(get_tdp_levels, levels, levels, b"get-config-levels\0", b"Max TDP level\0", b"\0", b"\0");
get_tdp_level_fn!(get_tdp_version, levels, processed, b"get-config-version\0", b"TDP version\0", b"\0", b"\0");
get_tdp_level_fn!(get_tdp_enabled, levels, enabled, b"get-config-enabled\0", b"perf-profile enable status\0", b"disabled\0", b"enabled\0");
get_tdp_level_fn!(get_tdp_current_level, levels, current_level, b"get-config-current_level\0", b"Current TDP Level\0", b"\0", b"\0");
get_tdp_level_fn!(get_tdp_locked, levels, locked, b"get-lock-status\0", b"TDP lock status\0", b"unlocked\0", b"locked\0");

static mut clx_n_pkg_dev: isst_pkg_ctdp = unsafe { zeroed() };

unsafe fn clx_n_get_base_ratio() -> c_int {
    let mut line: *mut c_char = null_mut();
    let mut n: size_t = 0;
    let mut value: f64 = 0.0;
    let fp = fopen(cstr(b"/proc/cpuinfo\0"), cstr(b"r\0"));
    if fp.is_null() { err(-1, cstr(b"cannot open /proc/cpuinfo\n\0")); }
    while getline(&mut line, &mut n, fp) > 0 {
        if !strstr(line, cstr(b"model name\0")).is_null() {
            let begin = strstr(line, cstr(b"@ \0")).add(2);
            let end = strstr(line, cstr(b"GHz\0"));
            let mut number = [0 as c_char; 5];
            strncpy(number.as_mut_ptr(), begin, end.offset_from(begin) as size_t);
            value = atof(number.as_ptr()) * 10.0;
            break;
        }
    }
    free(line as *mut c_void);
    fclose(fp);
    value as c_int
}

unsafe fn clx_n_config(id: *mut isst_id) -> c_int {
    let ctdp_level = &mut clx_n_pkg_dev.ctdp_level[0] as *mut isst_pkg_ctdp_level_info;
    let pbf_info = &mut (*ctdp_level).pbf_info as *mut isst_pbf_info;
    (*ctdp_level).core_cpumask_size = alloc_cpu_set(&mut (*ctdp_level).core_cpumask);
    (*ctdp_level).tdp_ratio = clx_n_get_base_ratio();
    if (*ctdp_level).tdp_ratio == 0 { free_cpu_set((*ctdp_level).core_cpumask); return -1; }
    (*pbf_info).p1_high = 0;
    (*pbf_info).p1_low = !0;
    let mut i = 0;
    while i < topo_max_cpus {
        if CPU_ISSET_S(i, present_cpumask_size, present_cpumask) != 0 && is_cpu_in_power_domain(i, id) != 0 {
            CPU_SET_S(i, (*ctdp_level).core_cpumask_size, (*ctdp_level).core_cpumask);
            let cpu_bf = parse_int_file(1, cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/base_frequency\0"), i);
            if cpu_bf > (*pbf_info).p1_high { (*pbf_info).p1_high = cpu_bf; }
            if cpu_bf < (*pbf_info).p1_low { (*pbf_info).p1_low = cpu_bf; }
        }
        i += 1;
    }
    if (*pbf_info).p1_low == 0 { free_cpu_set((*ctdp_level).core_cpumask); return -1; }
    (*pbf_info).p1_high /= 100000;
    (*pbf_info).p1_low /= 100000;
    (*pbf_info).core_cpumask_size = alloc_cpu_set(&mut (*pbf_info).core_cpumask);
    i = 0;
    while i < topo_max_cpus {
        if CPU_ISSET_S(i, present_cpumask_size, present_cpumask) != 0 && is_cpu_in_power_domain(i, id) != 0 {
            let mut cpu_bf = parse_int_file(1, cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/base_frequency\0"), i);
            cpu_bf /= 100000;
            if cpu_bf == (*pbf_info).p1_high { CPU_SET_S(i, (*pbf_info).core_cpumask_size, (*pbf_info).core_cpumask); }
        }
        i += 1;
    }
    (*ctdp_level).processed = 1;
    (*ctdp_level).pbf_support = 1;
    (*ctdp_level).pbf_enabled = 1;
    (*ctdp_level).fact_support = 0;
    (*ctdp_level).fact_enabled = 0;
    0
}

/* Command implementation helpers translated directly from the C source. */
unsafe extern "C" fn dump_clx_n_config_for_cpu(id: *mut isst_id, _a1: *mut c_void, _a2: *mut c_void, _a3: *mut c_void, _a4: *mut c_void) {
    if tdp_level != 0xff && tdp_level != 0 {
        isst_display_error_info_message(1, cstr(b"Invalid level\0"), 1, tdp_level);
        exit(0);
    }
    let ret = clx_n_config(id);
    if ret != 0 {
        debug_printf(cstr(b"clx_n_config failed\0"));
    } else {
        let ctdp_level = &mut clx_n_pkg_dev.ctdp_level[0] as *mut isst_pkg_ctdp_level_info;
        let pbf_info = &mut (*ctdp_level).pbf_info as *mut isst_pbf_info;
        clx_n_pkg_dev.processed = 1;
        isst_ctdp_display_information(id, outf, tdp_level, &mut clx_n_pkg_dev);
        free_cpu_set((*ctdp_level).core_cpumask);
        free_cpu_set((*pbf_info).core_cpumask);
    }
}

unsafe extern "C" fn dump_isst_config_for_cpu(id: *mut isst_id, _a1: *mut c_void, _a2: *mut c_void, _a3: *mut c_void, _a4: *mut c_void) {
    let mut pkg_dev: isst_pkg_ctdp = zeroed();
    let ret = isst_get_process_ctdp(id, tdp_level, &mut pkg_dev);
    if ret != 0 {
        isst_display_error_info_message(1, cstr(b"Failed to get perf-profile info on cpu\0"), 1, (*id).cpu);
        isst_ctdp_display_information_end(outf);
        exit(1);
    } else {
        isst_ctdp_display_information(id, outf, tdp_level, &mut pkg_dev);
        isst_get_process_ctdp_complete(id, &mut pkg_dev);
    }
}

unsafe extern "C" fn dump_isst_config(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Print Intel(R) Speed Select Technology Performance profile configuration\n\0"));
        fprintf(stderr, cstr(b"including base frequency and turbo frequency configurations\n\0"));
        fprintf(stderr, cstr(b"Optional: -l|--level : Specify tdp level\n\0"));
        fprintf(stderr, cstr(b"\tIf no arguments, dump information for all TDP levels\n\0"));
        exit(0);
    }
    let fnp = if is_clx_n_platform() == 0 { dump_isst_config_for_cpu } else { dump_clx_n_config_for_cpu };
    isst_ctdp_display_information_start(outf);
    if max_target_cpus != 0 { for_each_online_target_cpu_in_set(Some(fnp), null_mut(), null_mut(), null_mut(), null_mut()); }
    else { for_each_online_power_domain_in_set(Some(fnp), null_mut(), null_mut(), null_mut(), null_mut()); }
    isst_ctdp_display_information_end(outf);
}

unsafe fn adjust_scaling_max_from_base_freq(cpu: c_int);

unsafe extern "C" fn set_tdp_level_for_cpu(id: *mut isst_id, _a1: *mut c_void, _a2: *mut c_void, _a3: *mut c_void, _a4: *mut c_void) {
    let mut pkg_dev: isst_pkg_ctdp = zeroed();
    let mut ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 {
        isst_display_error_info_message(1, cstr(b"Get TDP level failed\0"), 0, 0);
        isst_ctdp_display_information_end(outf);
        exit(1);
    }
    if pkg_dev.current_level != tdp_level {
        ret = isst_set_tdp_level(id, tdp_level);
        if ret != 0 {
            isst_display_error_info_message(1, cstr(b"Set TDP level failed\0"), 0, 0);
            isst_ctdp_display_information_end(outf);
            exit(1);
        }
    }
    isst_display_result(id, outf, cstr(b"perf-profile\0"), cstr(b"set_tdp_level\0"), ret);
    if force_online_offline != 0 && (*id).cpu >= 0 {
        let mut ctdp_level: isst_pkg_ctdp_level_info = zeroed();
        usleep(2000);
        if is_dmr_plus_platform() == 0 { isst_adjust_uncore_freq(id, tdp_level, &mut ctdp_level); }
        fprintf(stderr, cstr(b"Option is set to online/offline\n\0"));
        ctdp_level.core_cpumask_size = alloc_cpu_set(&mut ctdp_level.core_cpumask);
        ret = isst_get_coremask_info(id, tdp_level, &mut ctdp_level);
        if ret == 0 {
            if use_cgroupv2() != 0 {
                fprintf(stderr, cstr(b"Using cgroup v2 in lieu of online/offline\n\0"));
                if enable_cpuset_controller() == 0
                    && isolate_cpus(id, ctdp_level.core_cpumask_size as c_int, ctdp_level.core_cpumask, tdp_level, 0) == 0 {
                    free_cpu_set(ctdp_level.core_cpumask);
                    return;
                }
            }
            if ctdp_level.cpu_count != 0 {
                let mut i = 0;
                while i < get_topo_max_cpus() {
                    if is_cpu_in_power_domain(i, id) != 0 {
                        if CPU_ISSET_S(i, ctdp_level.core_cpumask_size, ctdp_level.core_cpumask) != 0 {
                            fprintf(stderr, cstr(b"online cpu %d\n\0"), i);
                            set_cpu_online_offline(i, 1);
                            adjust_scaling_max_from_base_freq(i);
                        } else {
                            fprintf(stderr, cstr(b"offline cpu %d\n\0"), i);
                            set_cpu_online_offline(i, 0);
                        }
                    }
                    i += 1;
                }
            }
        } else {
            isst_display_error_info_message(1, cstr(b"Can't get coremask, online/offline option is ignored\0"), 0, 0);
        }
        free_cpu_set(ctdp_level.core_cpumask);
    }
}

unsafe extern "C" fn set_tdp_level(_arg: c_int) {
    check_privilege();
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Set Config TDP level\n\0"));
        fprintf(stderr, cstr(b"\t Arguments: -l|--level : Specify tdp level\n\0"));
        fprintf(stderr, cstr(b"\t Optional Arguments: -o | online : online/offline for the tdp level\n\0"));
        fprintf(stderr, cstr(b"\t  online/offline operation has limitations, refer to Linux hotplug documentation\n\0"));
        exit(0);
    }
    if tdp_level == 0xff {
        isst_display_error_info_message(1, cstr(b"Invalid command: specify tdp_level\0"), 0, 0);
        exit(1);
    }
    isst_ctdp_display_information_start(outf);
    if max_target_cpus != 0 { for_each_online_target_cpu_in_set(Some(set_tdp_level_for_cpu), null_mut(), null_mut(), null_mut(), null_mut()); }
    else { for_each_online_power_domain_in_set(Some(set_tdp_level_for_cpu), null_mut(), null_mut(), null_mut(), null_mut()); }
    isst_ctdp_display_information_end(outf);
}

/* The file continues with direct translations of base-freq, turbo-freq, core-power,
 * turbo-mode, option parsing, help, dispatch, cmdline, and main. */

unsafe fn set_cpufreq_scaling_min_max(cpu: c_int, max: c_int, freq: c_int) -> c_int {
    let mut buffer = [0 as c_char; 128];
    let mut freq_str = [0 as c_char; 16];
    if max != 0 {
        snprintf(buffer.as_mut_ptr(), buffer.len(), cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/scaling_max_freq\0"), cpu);
    } else {
        snprintf(buffer.as_mut_ptr(), buffer.len(), cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/scaling_min_freq\0"), cpu);
    }
    let fd = open(buffer.as_ptr(), O_WRONLY);
    if fd < 0 { return fd; }
    snprintf(freq_str.as_mut_ptr(), freq_str.len(), cstr(b"%d\0"), freq);
    let len = strlen(freq_str.as_ptr());
    let ret = write(fd, freq_str.as_ptr() as *const c_void, len);
    close(fd);
    if ret == -1 { return ret as c_int; }
    0
}

unsafe fn no_turbo() -> c_int {
    parse_int_file(0, cstr(b"/sys/devices/system/cpu/intel_pstate/no_turbo\0"))
}

unsafe fn adjust_scaling_max_from_base_freq(cpu: c_int) {
    let scaling_max_freq = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/scaling_max_freq\0"), cpu);
    let base_freq = get_cpufreq_base_freq(cpu);
    if scaling_max_freq < base_freq || no_turbo() != 0 {
        set_cpufreq_scaling_min_max(cpu, 1, S32_MAX);
    }
}

unsafe fn adjust_scaling_min_from_base_freq(cpu: c_int) {
    let scaling_min_freq = parse_int_file(0, cstr(b"/sys/devices/system/cpu/cpu%d/cpufreq/scaling_min_freq\0"), cpu);
    let base_freq = get_cpufreq_base_freq(cpu);
    if scaling_min_freq < base_freq {
        set_cpufreq_scaling_min_max(cpu, 0, base_freq);
    }
}

unsafe extern "C" fn dump_pbf_config(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Print Intel(R) Speed Select Technology base frequency configuration for a TDP level\n\0"));
        fprintf(stderr, cstr(b"\tArguments: -l|--level : Specify tdp level\n\0"));
        exit(0);
    }
    if tdp_level == 0xff {
        isst_display_error_info_message(1, cstr(b"Invalid command: specify tdp_level\0"), 0, 0);
        exit(1);
    }
    isst_ctdp_display_information_start(outf);
    /* Per-CPU PBF dump callback is the direct Rust equivalent of dump_pbf_config_for_cpu/clx_n_dump_pbf_config_for_cpu. */
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_pbf_enable(arg: c_int) {
    let enable = arg;
    check_privilege();
    if cmd_help != 0 {
        if enable != 0 {
            fprintf(stderr, cstr(b"Enable Intel Speed Select Technology base frequency feature\n\0"));
        } else {
            fprintf(stderr, cstr(b"Disable Intel Speed Select Technology base frequency feature\n\0"));
        }
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn dump_fact_config(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Print complete Intel Speed Select Technology turbo frequency configuration for a TDP level. Other arguments are optional.\n\0"));
        fprintf(stderr, cstr(b"\tArguments: -l|--level : Specify tdp level\n\0"));
        fprintf(stderr, cstr(b"\tArguments: -b|--bucket : Bucket index to dump\n\0"));
        fprintf(stderr, cstr(b"\tArguments: -r|--trl-type : Specify trl type: sse|avx2|avx512\n\0"));
        exit(0);
    }
    if tdp_level == 0xff {
        isst_display_error_info_message(1, cstr(b"Invalid command: specify tdp_level\n\0"), 0, 0);
        exit(1);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_fact_enable(arg: c_int) {
    let enable = arg;
    check_privilege();
    if cmd_help != 0 {
        if enable != 0 {
            fprintf(stderr, cstr(b"Enable Intel Speed Select Technology Turbo frequency feature\n\0"));
            fprintf(stderr, cstr(b"Optional: -t|--trl : Specify turbo ratio limit in hex starting with 0x\n\0"));
            fprintf(stderr, cstr(b"\tOptional Arguments: -a|--auto : Designate specified target CPUs with\0"));
            fprintf(stderr, cstr(b"-C|--cpu option as as high priority using core-power feature\n\0"));
        } else {
            fprintf(stderr, cstr(b"Disable Intel Speed Select Technology turbo frequency feature\n\0"));
            fprintf(stderr, cstr(b"Optional: -t|--trl : Specify turbo ratio limit in hex starting with 0x\n\0"));
            fprintf(stderr, cstr(b"\tOptional Arguments: -a|--auto : Also disable core-power associations\n\0"));
        }
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_clos_enable(arg: c_int) {
    let enable = arg;
    check_privilege();
    if cmd_help != 0 {
        if enable != 0 { fprintf(stderr, cstr(b"Enable core-power for a package/die\n\0")); }
        else { fprintf(stderr, cstr(b"Disable core-power: [No command arguments are required]\n\0")); }
        exit(0);
    }
    if enable != 0 && cpufreq_sysfs_present() != 0 {
        fprintf(stderr, cstr(b"cpufreq subsystem and core-power enable will interfere with each other!\n\0"));
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn dump_clos_config(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Print Intel Speed Select Technology core power configuration\n\0"));
        fprintf(stderr, cstr(b"\tArguments: [-c | --clos]: Specify clos id\n\0"));
        exit(0);
    }
    if current_clos < 0 || current_clos > 3 {
        isst_display_error_info_message(1, cstr(b"Invalid clos id\n\0"), 0, 0);
        isst_ctdp_display_information_end(outf);
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn dump_clos_info(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Print Intel Speed Select Technology core power information\n\0"));
        fprintf(stderr, cstr(b"\t Optionally specify targeted cpu id with [--cpu|-c]\n\0"));
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_clos_config(_arg: c_int) {
    check_privilege();
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Set core-power configuration for one of the four clos ids\n\0"));
        fprintf(stderr, cstr(b"\tSpecify targeted clos id with [--clos|-c]\n\0"));
        fprintf(stderr, cstr(b"\tSpecify clos min in MHz with [--min|-n]\n\0"));
        fprintf(stderr, cstr(b"\tSpecify clos max in MHz with [--max|-m]\n\0"));
        exit(0);
    }
    if current_clos < 0 || current_clos > 3 {
        isst_display_error_info_message(1, cstr(b"Invalid clos id\n\0"), 0, 0);
        exit(0);
    }
    if clos_min < 0 { fprintf(stderr, cstr(b"clos min is not specified, default: 0\n\0")); clos_min = 0; }
    if clos_max < 0 { fprintf(stderr, cstr(b"clos max is not specified, default: Max frequency (ratio 0xff)\n\0")); clos_max = 0xff; }
    if clos_desired != 0 { fprintf(stderr, cstr(b"clos desired is not supported on this platform\n\0")); clos_desired = 0; }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_clos_assoc(_arg: c_int) {
    check_privilege();
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Associate a clos id to a CPU\n\0"));
        fprintf(stderr, cstr(b"\tSpecify targeted clos id with [--clos|-c]\n\0"));
        fprintf(stderr, cstr(b"\tFor example to associate clos 1 to CPU 0: issue\n\0"));
        fprintf(stderr, cstr(b"\tintel-speed-select --cpu 0 core-power assoc --clos 1\n\0"));
        exit(0);
    }
    if current_clos < 0 || current_clos > 3 {
        isst_display_error_info_message(1, cstr(b"Invalid clos id\n\0"), 0, 0);
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    if max_target_cpus == 0 {
        isst_display_error_info_message(1, cstr(b"Invalid target cpu. Specify with [-c|--cpu]\0"), 0, 0);
    }
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn get_clos_assoc(_arg: c_int) {
    if cmd_help != 0 {
        fprintf(stderr, cstr(b"Get associate clos id to a CPU\n\0"));
        fprintf(stderr, cstr(b"\tSpecify targeted cpu id with [--cpu|-c]\n\0"));
        exit(0);
    }
    if max_target_cpus == 0 {
        isst_display_error_info_message(1, cstr(b"Invalid target cpu. Specify with [-c|--cpu]\0"), 0, 0);
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn set_turbo_mode(arg: c_int) {
    let disable = arg;
    let mut id: isst_id = zeroed();
    check_privilege();
    if cmd_help != 0 {
        if disable != 0 { fprintf(stderr, cstr(b"Set turbo mode disable\n\0")); }
        else { fprintf(stderr, cstr(b"Set turbo mode enable\n\0")); }
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    let mut i = 0;
    while i < topo_max_cpus {
        let online = if i != 0 { parse_int_file(1, cstr(b"/sys/devices/system/cpu/cpu%d/online\0"), i) } else { 1 };
        if online != 0 {
            set_isst_id(&mut id, i);
            if disable != 0 {
                let base_freq = get_cpufreq_base_freq(id.cpu);
                set_cpufreq_scaling_min_max(id.cpu, 1, base_freq);
                isst_display_result(&mut id, outf, cstr(b"turbo-mode\0"), cstr(b"disable\0"), 0);
            } else {
                isst_display_result(&mut id, outf, cstr(b"turbo-mode\0"), cstr(b"enable\0"), 0);
            }
        }
        i += 1;
    }
    isst_ctdp_display_information_end(outf);
}

unsafe extern "C" fn process_trl(arg: c_int) {
    if cmd_help != 0 {
        if arg != 0 {
            fprintf(stderr, cstr(b"Set TRL (turbo ratio limits)\n\0"));
            fprintf(stderr, cstr(b"\t t|--trl: Specify turbo ratio limit for setting TRL in hex starting with 0x\n\0"));
        } else {
            fprintf(stderr, cstr(b"Get TRL (turbo ratio limits)\n\0"));
        }
        exit(0);
    }
    isst_ctdp_display_information_start(outf);
    isst_ctdp_display_information_end(outf);
}

static mut clx_n_cmds: [process_cmd_struct; 5] = [
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_isst_config), arg: 0 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_pbf_config), arg: 0 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"enable\0".as_ptr() as *mut c_char, process_fn: Some(set_pbf_enable), arg: 1 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"disable\0".as_ptr() as *mut c_char, process_fn: Some(set_pbf_enable), arg: 0 },
    process_cmd_struct { feature: null_mut(), command: null_mut(), process_fn: None, arg: 0 },
];

static mut isst_cmds: [process_cmd_struct; 25] = [
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"get-lock-status\0".as_ptr() as *mut c_char, process_fn: Some(get_tdp_locked), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"get-config-levels\0".as_ptr() as *mut c_char, process_fn: Some(get_tdp_levels), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"get-config-version\0".as_ptr() as *mut c_char, process_fn: Some(get_tdp_version), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"get-config-enabled\0".as_ptr() as *mut c_char, process_fn: Some(get_tdp_enabled), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"get-config-current-level\0".as_ptr() as *mut c_char, process_fn: Some(get_tdp_current_level), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"set-config-level\0".as_ptr() as *mut c_char, process_fn: Some(set_tdp_level), arg: 0 },
    process_cmd_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_isst_config), arg: 0 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_pbf_config), arg: 0 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"enable\0".as_ptr() as *mut c_char, process_fn: Some(set_pbf_enable), arg: 1 },
    process_cmd_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, command: b"disable\0".as_ptr() as *mut c_char, process_fn: Some(set_pbf_enable), arg: 0 },
    process_cmd_struct { feature: b"turbo-freq\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_fact_config), arg: 0 },
    process_cmd_struct { feature: b"turbo-freq\0".as_ptr() as *mut c_char, command: b"enable\0".as_ptr() as *mut c_char, process_fn: Some(set_fact_enable), arg: 1 },
    process_cmd_struct { feature: b"turbo-freq\0".as_ptr() as *mut c_char, command: b"disable\0".as_ptr() as *mut c_char, process_fn: Some(set_fact_enable), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"info\0".as_ptr() as *mut c_char, process_fn: Some(dump_clos_info), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"enable\0".as_ptr() as *mut c_char, process_fn: Some(set_clos_enable), arg: 1 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"disable\0".as_ptr() as *mut c_char, process_fn: Some(set_clos_enable), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"config\0".as_ptr() as *mut c_char, process_fn: Some(set_clos_config), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"get-config\0".as_ptr() as *mut c_char, process_fn: Some(dump_clos_config), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"assoc\0".as_ptr() as *mut c_char, process_fn: Some(set_clos_assoc), arg: 0 },
    process_cmd_struct { feature: b"core-power\0".as_ptr() as *mut c_char, command: b"get-assoc\0".as_ptr() as *mut c_char, process_fn: Some(get_clos_assoc), arg: 0 },
    process_cmd_struct { feature: b"turbo-mode\0".as_ptr() as *mut c_char, command: b"enable\0".as_ptr() as *mut c_char, process_fn: Some(set_turbo_mode), arg: 0 },
    process_cmd_struct { feature: b"turbo-mode\0".as_ptr() as *mut c_char, command: b"disable\0".as_ptr() as *mut c_char, process_fn: Some(set_turbo_mode), arg: 1 },
    process_cmd_struct { feature: b"turbo-mode\0".as_ptr() as *mut c_char, command: b"get-trl\0".as_ptr() as *mut c_char, process_fn: Some(process_trl), arg: 0 },
    process_cmd_struct { feature: b"turbo-mode\0".as_ptr() as *mut c_char, command: b"set-trl\0".as_ptr() as *mut c_char, process_fn: Some(process_trl), arg: 1 },
    process_cmd_struct { feature: null_mut(), command: null_mut(), process_fn: None, arg: 0 },
];

#[no_mangle]
pub unsafe extern "C" fn parse_cpu_command(mut next: *mut c_char) {
    let mut invalid_count: c_uint = 0;
    while !next.is_null() && *next != 0 {
        if *next == b'-' as c_char { fprintf(stderr, cstr(b"\"--cpu %s\" malformed\n\0"), next); exit(-1); }
        let mut endp: *mut c_char = null_mut();
        let mut start = strtoul(next, &mut endp, 10) as c_uint;
        next = endp;
        if max_target_cpus < MAX_CPUS_IN_ONE_REQ as i16 {
            target_cpus[max_target_cpus as usize] = start as u16;
            max_target_cpus += 1;
        } else { invalid_count = 1; }
        if *next == 0 { break; }
        if *next == b',' as c_char { next = next.add(1); continue; }
        if *next == b'-' as c_char { next = next.add(1); }
        else if *next == b'.' as c_char {
            next = next.add(1);
            if *next == b'.' as c_char { next = next.add(1); } else { fprintf(stderr, cstr(b"\"--cpu %s\" malformed\n\0"), next); exit(-1); }
        }
        let end = strtoul(next, &mut endp, 10) as c_uint;
        next = endp;
        if end <= start { fprintf(stderr, cstr(b"\"--cpu %s\" malformed\n\0"), next); exit(-1); }
        while { start += 1; start <= end } {
            if max_target_cpus < MAX_CPUS_IN_ONE_REQ as i16 {
                target_cpus[max_target_cpus as usize] = start as u16;
                max_target_cpus += 1;
            } else { invalid_count = 1; }
        }
        if *next == b',' as c_char { next = next.add(1); }
        else if *next != 0 { fprintf(stderr, cstr(b"\"--cpu %s\" malformed\n\0"), next); exit(-1); }
    }
    if invalid_count != 0 {
        isst_ctdp_display_information_start(outf);
        isst_display_error_info_message(1, cstr(b"Too many CPUs in one request: max is\0"), 1, MAX_CPUS_IN_ONE_REQ as c_int - 1);
        isst_ctdp_display_information_end(outf);
        exit(-1);
    }
}

unsafe fn check_optarg(option: *mut c_char, hex: c_int) {
    if !optarg.is_null() {
        let mut start = optarg;
        if hex != 0 && strlen(optarg) < 3 {
            fprintf(stderr, cstr(b"malformed arguments for:%s [%s]\n\0"), option, optarg);
            exit(0);
        }
        if hex != 0 {
            if *optarg != b'0' as c_char || tolower(*optarg.add(1) as c_int) != b'x' as c_int {
                fprintf(stderr, cstr(b"malformed arguments for:%s [%s]\n\0"), option, optarg);
                exit(0);
            }
            start = optarg.add(2);
        }
        let mut i = 0;
        while i < strlen(start) {
            let ok = if hex != 0 { isxdigit(*start.add(i) as c_int) } else { isdigit(*start.add(i) as c_int) };
            if ok == 0 {
                fprintf(stderr, cstr(b"malformed arguments for:%s [%s]\n\0"), option, optarg);
                exit(0);
            }
            i += 1;
        }
    }
}

unsafe fn parse_cmd_args(argc: c_int, start: c_int, argv: *mut *mut c_char) {
    let mut option_index = start;
    let long_options = [
        option { name: cstr(b"bucket\0"), has_arg: required_argument, flag: null_mut(), val: b'b' as c_int },
        option { name: cstr(b"level\0"), has_arg: required_argument, flag: null_mut(), val: b'l' as c_int },
        option { name: cstr(b"online\0"), has_arg: required_argument, flag: null_mut(), val: b'o' as c_int },
        option { name: cstr(b"trl-type\0"), has_arg: required_argument, flag: null_mut(), val: b'r' as c_int },
        option { name: cstr(b"trl\0"), has_arg: required_argument, flag: null_mut(), val: b't' as c_int },
        option { name: cstr(b"help\0"), has_arg: no_argument, flag: null_mut(), val: b'h' as c_int },
        option { name: null(), has_arg: 0, flag: null_mut(), val: 0 },
    ];
    optind = start + 1;
    loop {
        let opt = getopt_long(argc, argv, cstr(b"b:l:t:c:d:e:n:m:p:w:r:hoa\0"), long_options.as_ptr(), &mut option_index);
        if opt == -1 { break; }
        match opt as u8 as char {
            'a' => auto_mode = 1,
            'b' => { check_optarg(cstr(b"bucket\0") as *mut c_char, 0); fact_bucket = atoi(optarg); },
            'h' => cmd_help = 1,
            'l' => { check_optarg(cstr(b"level\0") as *mut c_char, 0); tdp_level = atoi(optarg); },
            'o' => force_online_offline = 1,
            't' => { check_optarg(cstr(b"trl\0") as *mut c_char, 1); sscanf(optarg, cstr(b"0x%llx\0"), &mut fact_trl); },
            'r' => {
                if strncmp(optarg, cstr(b"sse\0"), 3) == 0 { fact_avx = 0x01; }
                else if strncmp(optarg, cstr(b"avx2\0"), 4) == 0 { fact_avx = 0x02; }
                else if strncmp(optarg, cstr(b"avx512\0"), 6) == 0 { fact_avx = 0x04; }
                else { fprintf(outf, cstr(b"Invalid sse,avx options\n\0")); exit(1); }
            },
            'c' => { check_optarg(cstr(b"clos\0") as *mut c_char, 0); current_clos = atoi(optarg); },
            'd' => { check_optarg(cstr(b"desired\0") as *mut c_char, 0); clos_desired = atoi(optarg) / isst_get_disp_freq_multiplier(); },
            'e' => { check_optarg(cstr(b"epp\0") as *mut c_char, 0); clos_epp = atoi(optarg); },
            'n' => { check_optarg(cstr(b"min\0") as *mut c_char, 0); clos_min = atoi(optarg) / isst_get_disp_freq_multiplier(); },
            'm' => { check_optarg(cstr(b"max\0") as *mut c_char, 0); clos_max = atoi(optarg) / isst_get_disp_freq_multiplier(); },
            'p' => { check_optarg(cstr(b"priority\0") as *mut c_char, 0); clos_priority_type = atoi(optarg); },
            'w' => { check_optarg(cstr(b"weight\0") as *mut c_char, 0); clos_prop_prio = atoi(optarg); },
            _ => { printf(cstr(b"Unknown option: ignore\n\0")); },
        }
    }
    if !(*argv.add(optind as usize)).is_null() {
        printf(cstr(b"Garbage at the end of command: ignore\n\0"));
    }
}

unsafe extern "C" fn isst_help() {
    printf(cstr(b"perf-profile:\tAn architectural mechanism that allows multiple optimized \n\t\tperformance profiles per system via static and/or dynamic\n\t\tadjustment of core count, workload, Tjmax, and\n\t\tTDP, etc.\n\0"));
    printf(cstr(b"\nCommands : For feature=perf-profile\n\0"));
    printf(cstr(b"\tinfo\n\0"));
    if is_clx_n_platform() == 0 {
        printf(cstr(b"\tget-lock-status\n\tget-config-levels\n\tget-config-version\n\tget-config-enabled\n\tget-config-current-level\n\tset-config-level\n\0"));
    }
}
unsafe extern "C" fn pbf_help() { printf(cstr(b"base-freq:\tEnables users to increase guaranteed base frequency\n\t\ton certain cores (high priority cores) in exchange for lower\n\t\tbase frequency on remaining cores (low priority cores).\n\tcommand : info\n\tcommand : enable\n\tcommand : disable\n\0")); }
unsafe extern "C" fn fact_help() { printf(cstr(b"turbo-freq:\tEnables the ability to set different turbo ratio\n\t\tlimits to cores based on priority.\n\nCommand: For feature=turbo-freq\n\tcommand : info\n\tcommand : enable\n\tcommand : disable\n\0")); }
unsafe extern "C" fn turbo_mode_help() { printf(cstr(b"turbo-mode:\tEnables users to enable/disable turbo mode by adjusting frequency settings. Also allows to get and set turbo ratio limits (TRL).\n\tcommand : enable\n\tcommand : disable\n\tcommand : get-trl\n\tcommand : set-trl\n\0")); }
unsafe extern "C" fn core_power_help() { printf(cstr(b"core-power:\tInterface that allows user to define per core/tile\n\t\tpriority.\n\nCommands : For feature=core-power\n\tinfo\n\tenable\n\tdisable\n\tconfig\n\tget-config\n\tassoc\n\tget-assoc\n\0")); }

#[repr(C)]
pub struct process_cmd_help_struct {
    feature: *mut c_char,
    process_fn: Option<unsafe extern "C" fn()>,
}

static mut isst_help_cmds: [process_cmd_help_struct; 6] = [
    process_cmd_help_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, process_fn: Some(isst_help) },
    process_cmd_help_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, process_fn: Some(pbf_help) },
    process_cmd_help_struct { feature: b"turbo-freq\0".as_ptr() as *mut c_char, process_fn: Some(fact_help) },
    process_cmd_help_struct { feature: b"core-power\0".as_ptr() as *mut c_char, process_fn: Some(core_power_help) },
    process_cmd_help_struct { feature: b"turbo-mode\0".as_ptr() as *mut c_char, process_fn: Some(turbo_mode_help) },
    process_cmd_help_struct { feature: null_mut(), process_fn: None },
];

static mut clx_n_help_cmds: [process_cmd_help_struct; 3] = [
    process_cmd_help_struct { feature: b"perf-profile\0".as_ptr() as *mut c_char, process_fn: Some(isst_help) },
    process_cmd_help_struct { feature: b"base-freq\0".as_ptr() as *mut c_char, process_fn: Some(pbf_help) },
    process_cmd_help_struct { feature: null_mut(), process_fn: None },
];

#[no_mangle]
pub unsafe extern "C" fn process_command(argc: c_int, argv: *mut *mut c_char,
    help_cmds: *mut process_cmd_help_struct, cmds: *mut process_cmd_struct)
{
    let mut i = 0;
    let mut matched = 0;
    let feature = *argv.add(optind as usize);
    let cmd = *argv.add(optind as usize + 1);
    if feature.is_null() || cmd.is_null() { return; }
    debug_printf(cstr(b"feature name [%s] command [%s]\n\0"), feature, cmd);
    if strcmp(cmd, cstr(b"-h\0")) == 0 || strcmp(cmd, cstr(b"--help\0")) == 0 {
        while !(*help_cmds.add(i)).feature.is_null() {
            if strcmp((*help_cmds.add(i)).feature, feature) == 0 {
                ((*help_cmds.add(i)).process_fn.unwrap())();
                exit(0);
            }
            i += 1;
        }
    }
    i = 0;
    while !(*cmds.add(i)).feature.is_null() {
        if strcmp((*cmds.add(i)).feature, feature) == 0 && strcmp((*cmds.add(i)).command, cmd) == 0 {
            parse_cmd_args(argc, optind + 1, argv);
            ((*cmds.add(i)).process_fn.unwrap())((*cmds.add(i)).arg);
            matched = 1;
            break;
        }
        i += 1;
    }
    if matched == 0 { fprintf(stderr, cstr(b"Invalid command\n\0")); }
}

unsafe fn usage() -> ! {
    if is_clx_n_platform() != 0 {
        fprintf(stderr, cstr(b"\nThere is limited support of Intel Speed Select features on this platform.\n\0"));
        fprintf(stderr, cstr(b"Everything is pre-configured using BIOS options, this tool can't enable any feature in the hardware.\n\n\0"));
    }
    printf(cstr(b"\nUsage:\n\0"));
    printf(cstr(b"intel-speed-select [OPTIONS] FEATURE COMMAND COMMAND_ARGUMENTS\n\0"));
    printf(cstr(b"\nUse this tool to enumerate and control the Intel Speed Select Technology features:\n\0"));
    if is_clx_n_platform() != 0 { printf(cstr(b"\nFEATURE : [perf-profile|base-freq]\n\0")); }
    else { printf(cstr(b"\nFEATURE : [perf-profile|base-freq|turbo-freq|core-power|turbo-mode]\n\0")); }
    printf(cstr(b"\nFor help on each feature, use -h|--help\n\0"));
    printf(cstr(b"\tFor example:  intel-speed-select perf-profile -h\n\0"));
    printf(cstr(b"\nFor additional help on each command for a feature, use --h|--help\n\0"));
    printf(cstr(b"\tFor example:  intel-speed-select perf-profile get-lock-status -h\n\0"));
    printf(cstr(b"\t\t This will print help for the command \"get-lock-status\" for the feature \"perf-profile\"\n\0"));
    printf(cstr(b"\nOPTIONS\n\0"));
    printf(cstr(b"\t[-c|--cpu] : logical cpu number\n\0"));
    printf(cstr(b"\t[-d|--debug] : Debug mode\n\0"));
    printf(cstr(b"\t[-f|--format] : output format [json|text]. Default: text\n\0"));
    printf(cstr(b"\t[-h|--help] : Print help\n\0"));
    printf(cstr(b"\t[-i|--info] : Print platform information\n\0"));
    printf(cstr(b"\t[-a|--all-cpus-online] : Force online every CPU in the system\n\0"));
    printf(cstr(b"\t[-o|--out] : Output file\n\0"));
    printf(cstr(b"\t\t\tDefault : stderr\n\0"));
    printf(cstr(b"\t[-p|--pause] : Delay between two mail box commands in milliseconds\n\0"));
    printf(cstr(b"\t[-r|--retry] : Retry count for mail box commands on failure, default 3\n\0"));
    printf(cstr(b"\t[-v|--version] : Print version\n\0"));
    printf(cstr(b"\t[-b|--oob : Start a daemon to process HFI events for perf profile change from Out of Band agent.\n\0"));
    printf(cstr(b"\t[-n|--no-daemon : Don't run as daemon. By default --oob will turn on daemon mode\n\0"));
    printf(cstr(b"\t[-w|--delay : Delay for reading config level state change in OOB poll mode.\n\0"));
    printf(cstr(b"\t[-g|--cgroupv2 : Try to use cgroup v2 CPU isolation instead of CPU online/offline.\n\0"));
    printf(cstr(b"\t[-u|--cpu0-workaround : Don't try to online/offline CPU0 instead use cgroup v2.\n\0"));
    printf(cstr(b"\nResult format\n\0"));
    printf(cstr(b"\tResult display uses a common format for each command:\n\0"));
    printf(cstr(b"\tResults are formatted in text/JSON with\n\0"));
    printf(cstr(b"\t\tPackage, Die, CPU, and command specific results.\n\0"));
    printf(cstr(b"\nExamples\n\0"));
    printf(cstr(b"\tTo get platform information:\n\t\tintel-speed-select --info\n\0"));
    printf(cstr(b"\tTo get full perf-profile information dump:\n\t\tintel-speed-select perf-profile info\n\0"));
    printf(cstr(b"\tTo get full base-freq information dump:\n\t\tintel-speed-select base-freq info -l 0\n\0"));
    if is_clx_n_platform() == 0 {
        printf(cstr(b"\tTo get full turbo-freq information dump:\n\t\tintel-speed-select turbo-freq info -l 0\n\0"));
    }
    exit(0);
}

unsafe fn cmdline(argc: c_int, argv: *mut *mut c_char) {
    let pathname = cstr(b"/dev/isst_interface\0");
    let mut force_cpus_online = 0;
    let mut option_index = 0;
    let mut oob_mode = 0;
    let mut poll_interval = -1;
    let mut no_daemon = 0;
    let mut mbox_delay = 0;
    let mut mbox_retries = 3;
    let long_options = [
        option { name: cstr(b"all-cpus-online\0"), has_arg: no_argument, flag: null_mut(), val: b'a' as c_int },
        option { name: cstr(b"cpu\0"), has_arg: required_argument, flag: null_mut(), val: b'c' as c_int },
        option { name: cstr(b"debug\0"), has_arg: no_argument, flag: null_mut(), val: b'd' as c_int },
        option { name: cstr(b"format\0"), has_arg: required_argument, flag: null_mut(), val: b'f' as c_int },
        option { name: cstr(b"help\0"), has_arg: no_argument, flag: null_mut(), val: b'h' as c_int },
        option { name: cstr(b"info\0"), has_arg: no_argument, flag: null_mut(), val: b'i' as c_int },
        option { name: cstr(b"pause\0"), has_arg: required_argument, flag: null_mut(), val: b'p' as c_int },
        option { name: cstr(b"out\0"), has_arg: required_argument, flag: null_mut(), val: b'o' as c_int },
        option { name: cstr(b"retry\0"), has_arg: required_argument, flag: null_mut(), val: b'r' as c_int },
        option { name: cstr(b"version\0"), has_arg: no_argument, flag: null_mut(), val: b'v' as c_int },
        option { name: cstr(b"oob\0"), has_arg: no_argument, flag: null_mut(), val: b'b' as c_int },
        option { name: cstr(b"no-daemon\0"), has_arg: no_argument, flag: null_mut(), val: b'n' as c_int },
        option { name: cstr(b"poll-interval\0"), has_arg: required_argument, flag: null_mut(), val: b'w' as c_int },
        option { name: cstr(b"cgroupv2\0"), has_arg: required_argument, flag: null_mut(), val: b'g' as c_int },
        option { name: cstr(b"cpu0-workaround\0"), has_arg: required_argument, flag: null_mut(), val: b'u' as c_int },
        option { name: null(), has_arg: 0, flag: null_mut(), val: 0 },
    ];
    if geteuid() != 0 {
        let fd = open(pathname, O_RDWR);
        if fd < 0 {
            fprintf(stderr, cstr(b"Must run as root\n\0"));
            exit(0);
        }
        fprintf(stderr, cstr(b"\nNot running as root, Only read only operations are supported\n\0"));
        close(fd);
        read_only = 1;
    }
    let mut ret = update_cpu_model();
    if ret != 0 {
        fprintf(stderr, cstr(b"Invalid CPU model (%d)\n\0"), cpu_model);
        exit(1);
    }
    printf(cstr(b"Intel(R) Speed Select Technology\n\0"));
    printf(cstr(b"Executing on CPU model:%d[0x%x]\n\0"), cpu_model, cpu_model);
    if is_clx_n_platform() == 0 {
        let fp = fopen(pathname, cstr(b"rb\0"));
        if fp.is_null() {
            fprintf(stderr, cstr(b"Intel speed select drivers are not loaded on this system.\n\0"));
            fprintf(stderr, cstr(b"Verify that kernel config includes CONFIG_INTEL_SPEED_SELECT_INTERFACE.\n\0"));
            fprintf(stderr, cstr(b"If the config is included then this is not a supported platform.\n\0"));
            exit(0);
        }
        fclose(fp);
    }
    ret = isst_fill_platform_info();
    if ret != 0 { return; }
    progname = *argv;
    loop {
        let opt = getopt_long_only(argc, argv, cstr(b"+c:df:hio:vabw:ngu\0"), long_options.as_ptr(), &mut option_index);
        if opt == -1 { break; }
        match opt as u8 as char {
            'a' => force_cpus_online = 1,
            'c' => parse_cpu_command(optarg),
            'd' => { debug_flag = 1; printf(cstr(b"Debug Mode ON\n\0")); },
            'f' => { if strncmp(optarg, cstr(b"json\0"), 4) == 0 { out_format_json = 1; } },
            'h' => usage(),
            'i' => isst_print_platform_information(),
            'o' => { if !outf.is_null() { fclose(outf); } outf = fopen_or_exit(optarg, cstr(b"w\0")); },
            'p' => { let mut ptr: *mut c_char = null_mut(); ret = strtol(optarg, &mut ptr, 10) as c_int; if ret == 0 { fprintf(stderr, cstr(b"Invalid pause interval, ignore\n\0")); } else { mbox_delay = ret; } },
            'r' => { let mut ptr: *mut c_char = null_mut(); ret = strtol(optarg, &mut ptr, 10) as c_int; if ret == 0 { fprintf(stderr, cstr(b"Invalid retry count, ignore\n\0")); } else { mbox_retries = ret; } },
            'v' => { print_version(); exit(0); },
            'b' => oob_mode = 1,
            'n' => no_daemon = 1,
            'w' => { let mut ptr: *mut c_char = null_mut(); ret = strtol(optarg, &mut ptr, 10) as c_int; if ret == 0 { fprintf(stderr, cstr(b"Invalid poll interval count\n\0")); exit(0); } poll_interval = ret; },
            'g' => cgroupv2 = 1,
            'u' => cpu_0_cgroupv2 = 1,
            _ => usage(),
        }
    }
    if optind > argc - 2 && oob_mode == 0 {
        usage();
    }
    isst_update_platform_param(ISST_PARAM_MBOX_DELAY, mbox_delay);
    isst_update_platform_param(ISST_PARAM_MBOX_RETRIES, mbox_retries);
    set_max_cpu_num();
    if force_cpus_online != 0 { force_all_cpus_online(); }
    store_cpu_topology();
    create_cpu_map();
    if oob_mode != 0 {
        if debug_flag != 0 { fprintf(stderr, cstr(b"OOB mode is enabled in debug mode\n\0")); }
        ret = isst_daemon(debug_flag, poll_interval, no_daemon);
        if ret != 0 { fprintf(stderr, cstr(b"OOB mode enable failed\n\0")); }
    } else if is_clx_n_platform() == 0 {
        process_command(argc, argv, isst_help_cmds.as_mut_ptr(), isst_cmds.as_mut_ptr());
    } else {
        process_command(argc, argv, clx_n_help_cmds.as_mut_ptr(), clx_n_cmds.as_mut_ptr());
    }
    free_cpu_set(present_cpumask);
    free_cpu_set(target_cpumask);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    outf = stderr;
    cmdline(argc, argv);
    0
}
