// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Enumerate and control features for Mailbox Interface
 * Copyright (c) 2023 Intel Corporation.
 */
// Translated from isst-core-mbox.c. C header dependencies from "isst.h" are
// expected to be supplied by the surrounding translated repository.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn debug_printf(fmt: *const c_char, ...);
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...);
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn get_output_file() -> *mut c_void;
    fn is_emr_platform() -> c_int;
    fn is_skx_based_platform() -> c_int;
    fn is_spr_platform() -> c_int;
    fn is_icx_platform() -> c_int;
    fn isst_read_pm_config(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int;
    fn isst_get_trl_max_levels() -> c_int;
    fn isst_send_msr_command(cpu: c_uint, reg: c_uint, write: c_int, value: *mut u64) -> c_int;
    fn isst_get_config_tdp_lock_status(id: *mut isst_id) -> c_int;
    fn isst_display_error_info_message(error: c_int, msg: *const c_char, arg_valid: c_int, arg: c_int);
    fn get_max_punit_core_id(id: *mut isst_id) -> c_int;
    fn set_cpu_mask_from_punit_coremask(
        id: *mut isst_id,
        mask: u64,
        core_cpumask_size: c_int,
        core_cpumask: *mut c_void,
        cpu_count: *mut c_int,
    );
    fn isst_get_ctdp_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
    fn isst_get_ctdp_control(
        id: *mut isst_id,
        config_index: c_int,
        ctdp_level: *mut isst_pkg_ctdp_level_info,
    ) -> c_int;
    fn find_phy_core_num(cpu: c_int) -> c_int;
}

extern "C" {
    static DISP_FREQ_MULTIPLIER: c_int;
    static ISST_IF_IO_CMD: c_ulong;
    static ISST_IF_MBOX_COMMAND: c_ulong;
    static CONFIG_CLOS: u8;
    static CLOS_PM_QOS_CONFIG: u8;
    static CLOS_PQR_ASSOC: u8;
    static CLOS_PM_CLOS: u8;
    static CLOS_STATUS: u8;
    static MBOX_CMD_WRITE_BIT: c_uint;
    static PQR_ASSOC_OFFSET: c_uint;
    static PM_CLOS_OFFSET: c_uint;
    static READ_PM_CONFIG: u8;
    static WRITE_PM_CONFIG: u8;
    static PM_FEATURE: u8;
    static CONFIG_TDP: u8;
    static CONFIG_TDP_GET_LEVELS_INFO: u8;
    static CONFIG_TDP_GET_TDP_CONTROL: u8;
    static CONFIG_TDP_GET_RATIO_INFO: u8;
    static CONFIG_TDP_GET_UNCORE_P0_P1_INFO: u8;
    static CONFIG_TDP_GET_P1_INFO: u8;
    static CONFIG_TDP_GET_MEM_FREQ: u8;
    static CONFIG_TDP_GET_TDP_INFO: u8;
    static CONFIG_TDP_GET_TJMAX_INFO: u8;
    static CONFIG_TDP_GET_PWR_INFO: u8;
    static CONFIG_TDP_GET_CORE_MASK: u8;
    static CONFIG_TDP_GET_TURBO_LIMIT_RATIOS: u8;
    static CONFIG_TDP_SET_LEVEL: u8;
    static CONFIG_TDP_PBF_GET_CORE_MASK_INFO: u8;
    static CONFIG_TDP_PBF_GET_P1HI_P1LO_INFO: u8;
    static CONFIG_TDP_PBF_GET_TDP_INFO: u8;
    static CONFIG_TDP_PBF_GET_TJ_MAX_INFO: u8;
    static CONFIG_TDP_SET_TDP_CONTROL: u8;
    static CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_NUMCORES: u8;
    static CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_RATIOS: u8;
    static CONFIG_TDP_GET_FACT_LP_CLIPPING_RATIO: u8;
    static ISST_FACT_MAX_BUCKETS: c_int;
}

extern "C" {
    type isst_id;
    type isst_if_io_regs;
    type isst_if_mbox_cmds;
    type isst_pkg_ctdp;
    type isst_pkg_ctdp_level_info;
    type isst_pbf_info;
    type isst_fact_bucket_info;
    type isst_fact_info;
    type isst_clos_config;
    type isst_platform_ops;
    type isst_platform_param;
}

const O_RDWR: c_int = 0o2;
const O_WRONLY: c_int = 0o1;
const ENOTTY: c_int = 25;
const EINVAL: c_int = 22;
const ISST_PARAM_MBOX_DELAY: c_int = 0;
const ISST_PARAM_MBOX_RETRIES: c_int = 1;
const MAX_TRL_LEVELS_EMR: c_int = 5;

#[inline]
unsafe fn BIT(nr: c_uint) -> c_uint {
    1u32.wrapping_shl(nr)
}

#[inline]
unsafe fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32).wrapping_shl(l)) & ((!0u32).wrapping_shr(31 - h))
}

static mut mbox_delay: c_int = 0;
static mut mbox_retries: c_int = 3;

unsafe fn mbox_get_disp_freq_multiplier() -> c_int {
    DISP_FREQ_MULTIPLIER
}

unsafe fn mbox_get_trl_max_levels() -> c_int {
    if is_emr_platform() != 0 {
        return MAX_TRL_LEVELS_EMR;
    }

    3
}

unsafe fn mbox_get_trl_level_name(level: c_int) -> *mut c_char {
    if is_emr_platform() != 0 {
        static mut level_str: [c_char; 18] = [0; 18];

        if level >= MAX_TRL_LEVELS_EMR {
            return core::ptr::null_mut();
        }

        snprintf(level_str.as_mut_ptr(), level_str.len(), b"level-%d\0".as_ptr() as *const c_char, level);
        return level_str.as_mut_ptr();
    }

    match level {
        0 => b"sse\0".as_ptr() as *mut c_char,
        1 => b"avx2\0".as_ptr() as *mut c_char,
        2 => b"avx512\0".as_ptr() as *mut c_char,
        _ => core::ptr::null_mut(),
    }
}

unsafe fn mbox_update_platform_param(param: c_int, value: c_int) {
    match param {
        ISST_PARAM_MBOX_DELAY => mbox_delay = value,
        ISST_PARAM_MBOX_RETRIES => mbox_retries = value,
        _ => {}
    }
}

unsafe fn mbox_is_punit_valid(id: *mut isst_id) -> c_int {
    if (*id).cpu < 0 {
        return 0;
    }

    if (*id).pkg < 0 || (*id).die < 0 || (*id).punit != 0 {
        return 0;
    }

    1
}

unsafe fn _send_mmio_command(cpu: c_uint, reg: c_uint, write_arg: c_int, value: *mut c_uint) -> c_int {
    let mut io_regs: isst_if_io_regs = core::mem::zeroed();
    let pathname = b"/dev/isst_interface\0";
    let mut cmd: c_ulong;
    let outf = get_output_file();
    let fd: c_int;

    debug_printf(b"mmio_cmd cpu:%d reg:%d write:%d\n\0".as_ptr() as *const c_char, cpu, reg, write_arg);

    fd = open(pathname.as_ptr() as *const c_char, O_RDWR);
    if fd < 0 {
        err(-1, b"%s open failed\0".as_ptr() as *const c_char, pathname.as_ptr() as *const c_char);
    }

    io_regs.req_count = 1;
    io_regs.io_reg[0].logical_cpu = cpu;
    io_regs.io_reg[0].reg = reg;
    cmd = ISST_IF_IO_CMD;
    if write_arg != 0 {
        io_regs.io_reg[0].read_write = 1;
        io_regs.io_reg[0].value = *value;
    } else {
        io_regs.io_reg[0].read_write = 0;
    }

    if ioctl(fd, cmd, &mut io_regs as *mut isst_if_io_regs) == -1 {
        if errno == ENOTTY {
            perror(b"ISST_IF_IO_COMMAND\n\0".as_ptr() as *const c_char);
            fprintf(stderr, b"Check presence of kernel modules: isst_if_mmio\n\0".as_ptr() as *const c_char);
            exit(0);
        }
        fprintf(
            outf,
            b"Error: mmio_cmd cpu:%d reg:%x read_write:%x\n\0".as_ptr() as *const c_char,
            cpu,
            reg,
            write_arg,
        );
    } else {
        if write_arg == 0 {
            *value = io_regs.io_reg[0].value;
        }

        debug_printf(
            b"mmio_cmd response: cpu:%d reg:%x rd_write:%x resp:%x\n\0".as_ptr() as *const c_char,
            cpu,
            reg,
            write_arg,
            *value,
        );
    }

    close(fd);

    0
}

pub unsafe extern "C" fn _send_mbox_command(
    cpu: c_uint,
    command: u8,
    sub_command: u8,
    parameter: c_uint,
    req_data: c_uint,
    resp: *mut c_uint,
) -> c_int {
    let pathname = b"/dev/isst_interface\0";
    let mut fd: c_int;
    let mut retry: c_int;
    let mut mbox_cmds: isst_if_mbox_cmds = core::mem::zeroed();

    debug_printf(
        b"mbox_send: cpu:%d command:%x sub_command:%x parameter:%x req_data:%x\n\0".as_ptr() as *const c_char,
        cpu,
        command as c_int,
        sub_command as c_int,
        parameter,
        req_data,
    );

    if is_skx_based_platform() == 0 && command == CONFIG_CLOS && sub_command != CLOS_PM_QOS_CONFIG {
        let mut value: c_uint = 0;
        let mut write_cmd: c_int = 0;
        let mut ret: c_int = 0;

        debug_printf(b"CPU %d\n\0".as_ptr() as *const c_char, cpu);

        if (parameter & BIT(MBOX_CMD_WRITE_BIT)) != 0 {
            value = req_data;
            write_cmd = 1;
        }

        match sub_command {
            CLOS_PQR_ASSOC => {
                let core_id = parameter & 0xff;
                ret = _send_mmio_command(cpu, PQR_ASSOC_OFFSET + core_id * 4, write_cmd, &mut value);
                if ret == 0 && write_cmd == 0 {
                    *resp = value;
                }
            }
            CLOS_PM_CLOS => {
                let clos_id = parameter & 0x03;
                ret = _send_mmio_command(cpu, PM_CLOS_OFFSET + clos_id * 4, write_cmd, &mut value);
                if ret == 0 && write_cmd == 0 {
                    *resp = value;
                }
            }
            CLOS_STATUS => {}
            _ => {}
        }
        return ret;
    }

    mbox_cmds.cmd_count = 1;
    mbox_cmds.mbox_cmd[0].logical_cpu = cpu;
    mbox_cmds.mbox_cmd[0].command = command;
    mbox_cmds.mbox_cmd[0].sub_command = sub_command;
    mbox_cmds.mbox_cmd[0].parameter = parameter;
    mbox_cmds.mbox_cmd[0].req_data = req_data;

    if mbox_delay != 0 {
        usleep((mbox_delay * 1000) as c_uint);
    }

    fd = open(pathname.as_ptr() as *const c_char, O_RDWR);
    if fd < 0 {
        err(-1, b"%s open failed\0".as_ptr() as *const c_char, pathname.as_ptr() as *const c_char);
    }

    retry = mbox_retries;
    while {
        if ioctl(fd, ISST_IF_MBOX_COMMAND, &mut mbox_cmds as *mut isst_if_mbox_cmds) == -1 {
            if errno == ENOTTY {
                perror(b"ISST_IF_MBOX_COMMAND\n\0".as_ptr() as *const c_char);
                fprintf(stderr, b"Check presence of kernel modules: isst_if_mbox_pci or isst_if_mbox_msr\n\0".as_ptr() as *const c_char);
                exit(0);
            }
            debug_printf(
                b"Error: mbox_cmd cpu:%d command:%x sub_command:%x parameter:%x req_data:%x errorno:%d\n\0".as_ptr() as *const c_char,
                cpu,
                command as c_int,
                sub_command as c_int,
                parameter,
                req_data,
                errno,
            );
            retry -= 1;
        } else {
            *resp = mbox_cmds.mbox_cmd[0].resp_data;
            debug_printf(
                b"mbox_cmd response: cpu:%d command:%x sub_command:%x parameter:%x req_data:%x resp:%x\n\0".as_ptr() as *const c_char,
                cpu,
                command as c_int,
                sub_command as c_int,
                parameter,
                req_data,
                *resp,
            );
            break;
        }
        retry != 0
    } {}

    close(fd);

    if retry == 0 {
        debug_printf(b"Failed mbox command even after retries\n\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe fn mbox_read_pm_config(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command((*id).cpu as c_uint, READ_PM_CONFIG, PM_FEATURE, 0, 0, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d READ_PM_CONFIG resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    *cp_state = (resp & BIT(16)) as c_int;
    *cp_cap = if (resp & BIT(0)) != 0 { 1 } else { 0 };

    0
}

unsafe fn mbox_get_config_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_TDP, CONFIG_TDP_GET_LEVELS_INFO, 0, 0, &mut resp);
    if ret != 0 {
        (*pkg_dev).levels = 0;
        (*pkg_dev).locked = 1;
        (*pkg_dev).current_level = 0;
        (*pkg_dev).version = 0;
        (*pkg_dev).enabled = 0;
        return 0;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_GET_LEVELS_INFO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    (*pkg_dev).version = (resp & 0xff) as c_int;
    (*pkg_dev).levels = ((resp >> 8) & 0xff) as c_int;
    (*pkg_dev).current_level = ((resp >> 16) & 0xff) as c_int;
    (*pkg_dev).locked = ((resp & BIT(24)) != 0) as c_int;
    (*pkg_dev).enabled = ((resp & BIT(31)) != 0) as c_int;

    0
}

unsafe fn mbox_get_ctdp_control(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut cp_state: c_int = 0;
    let mut cp_cap: c_int = 0;
    let mut resp: c_uint = 0;
    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_TDP_CONTROL,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    (*ctdp_level).fact_support = (resp & BIT(0)) as c_int;
    (*ctdp_level).pbf_support = ((resp & BIT(1)) != 0) as c_int;
    (*ctdp_level).fact_enabled = ((resp & BIT(16)) != 0) as c_int;
    (*ctdp_level).pbf_enabled = ((resp & BIT(17)) != 0) as c_int;

    ret = isst_read_pm_config(id, &mut cp_state, &mut cp_cap);
    if ret != 0 {
        debug_printf(b"cpu:%d pm_config is not supported\n\0".as_ptr() as *const c_char, (*id).cpu);
    } else {
        debug_printf(
            b"cpu:%d pm_config SST-CP state:%d cap:%d\n\0".as_ptr() as *const c_char,
            (*id).cpu,
            cp_state,
            cp_cap,
        );
        (*ctdp_level).sst_cp_support = cp_cap;
        (*ctdp_level).sst_cp_enabled = cp_state;
    }

    debug_printf(
        b"cpu:%d CONFIG_TDP_GET_TDP_CONTROL resp:%x fact_support:%d pbf_support: %d fact_enabled:%d pbf_enabled:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        resp,
        (*ctdp_level).fact_support,
        (*ctdp_level).pbf_support,
        (*ctdp_level).fact_enabled,
        (*ctdp_level).pbf_enabled,
    );

    0
}

unsafe fn _get_uncore_p0_p1_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) {
    let mut resp: c_uint = 0;

    (*ctdp_level).uncore_pm = 0;
    (*ctdp_level).uncore_p0 = 0;
    (*ctdp_level).uncore_p1 = 0;

    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_RATIO_INFO,
        0,
        (BIT(16) | config_index as c_uint),
        &mut resp,
    );
    if ret == 0 {
        (*ctdp_level).uncore_p0 = (resp & GENMASK(7, 0)) as c_int;
        (*ctdp_level).uncore_p1 = ((resp & GENMASK(15, 8)) >> 8) as c_int;
        (*ctdp_level).uncore_pm = ((resp & GENMASK(31, 24)) >> 24) as c_int;

        debug_printf(
            b"cpu:%d ctdp:%d CONFIG_TDP_GET_RATIO_INFO resp:%x uncore p0:%d uncore p1:%d uncore pm:%d\n\0".as_ptr() as *const c_char,
            (*id).cpu,
            config_index,
            resp,
            (*ctdp_level).uncore_p0,
            (*ctdp_level).uncore_p1,
            (*ctdp_level).uncore_pm,
        );
        return;
    }

    ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_UNCORE_P0_P1_INFO,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        (*ctdp_level).uncore_p0 = 0;
        (*ctdp_level).uncore_p1 = 0;
        return;
    }

    (*ctdp_level).uncore_p0 = (resp & GENMASK(7, 0)) as c_int;
    (*ctdp_level).uncore_p1 = ((resp & GENMASK(15, 8)) >> 8) as c_int;
    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_UNCORE_P0_P1_INFO resp:%x uncore p0:%d uncore p1:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).uncore_p0,
        (*ctdp_level).uncore_p1,
    );
}

unsafe fn _set_uncore_min_max(id: *mut isst_id, max: c_int, freq: c_int) -> c_int {
    let mut buffer: [c_char; 128] = [0; 128];
    let mut freq_str: [c_char; 16] = [0; 16];
    let fd: c_int;
    let ret: isize;
    let len: usize;

    if max != 0 {
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            b"/sys/devices/system/cpu/intel_uncore_frequency/package_%02d_die_%02d/max_freq_khz\0".as_ptr() as *const c_char,
            (*id).pkg,
            (*id).die,
        );
    } else {
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            b"/sys/devices/system/cpu/intel_uncore_frequency/package_%02d_die_%02d/min_freq_khz\0".as_ptr() as *const c_char,
            (*id).pkg,
            (*id).die,
        );
    }

    fd = open(buffer.as_ptr(), O_WRONLY);
    if fd < 0 {
        return fd;
    }

    snprintf(freq_str.as_mut_ptr(), freq_str.len(), b"%d\0".as_ptr() as *const c_char, freq);
    len = strlen(freq_str.as_ptr());
    ret = write(fd, freq_str.as_ptr() as *const c_void, len);
    if ret == -1 {
        close(fd);
        return ret as c_int;
    }
    close(fd);

    0
}

unsafe fn mbox_adjust_uncore_freq(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) {
    _get_uncore_p0_p1_info(id, config_index, ctdp_level);
    if (*ctdp_level).uncore_pm != 0 {
        _set_uncore_min_max(id, 0, (*ctdp_level).uncore_pm * 100000);
    }

    if (*ctdp_level).uncore_p0 != 0 {
        _set_uncore_min_max(id, 1, (*ctdp_level).uncore_p0 * 100000);
    }
}

unsafe fn _get_p1_info(id: *mut isst_id, config_index: c_int, ctdp_level: *mut isst_pkg_ctdp_level_info) {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_P1_INFO,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        (*ctdp_level).sse_p1 = 0;
        (*ctdp_level).avx2_p1 = 0;
        (*ctdp_level).avx512_p1 = 0;
        return;
    }

    (*ctdp_level).sse_p1 = (resp & GENMASK(7, 0)) as c_int;
    (*ctdp_level).avx2_p1 = ((resp & GENMASK(15, 8)) >> 8) as c_int;
    (*ctdp_level).avx512_p1 = ((resp & GENMASK(23, 16)) >> 16) as c_int;
    (*ctdp_level).amx_p1 = ((resp & GENMASK(31, 24)) >> 24) as c_int;
    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_P1_INFO resp:%x sse_p1:%d avx2_p1:%d avx512_p1:%d amx_p1:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).sse_p1,
        (*ctdp_level).avx2_p1,
        (*ctdp_level).avx512_p1,
        (*ctdp_level).amx_p1,
    );
}

unsafe fn _get_uncore_mem_freq(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_MEM_FREQ,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        (*ctdp_level).mem_freq = 0;
        return;
    }

    (*ctdp_level).mem_freq = (resp & GENMASK(7, 0)) as c_int;
    if is_spr_platform() != 0 || is_emr_platform() != 0 {
        (*ctdp_level).mem_freq *= 200;
    } else if is_icx_platform() != 0 {
        if (*ctdp_level).mem_freq < 7 {
            (*ctdp_level).mem_freq = (((12 - (*ctdp_level).mem_freq) as f64 * 133.33 * 2.0 * 10.0) as c_int);
            (*ctdp_level).mem_freq /= 10;
            if (*ctdp_level).mem_freq % 10 > 5 {
                (*ctdp_level).mem_freq += 1;
            }
        } else {
            (*ctdp_level).mem_freq = 0;
        }
    } else {
        (*ctdp_level).mem_freq = 0;
    }
    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_MEM_FREQ resp:%x uncore mem_freq:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).mem_freq,
    );
}

unsafe fn mbox_get_tdp_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut resp: c_uint = 0;
    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_TDP_INFO,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        isst_display_error_info_message(1, b"Invalid level, Can't get TDP information at level\0".as_ptr() as *const c_char, 1, config_index);
        return ret;
    }

    (*ctdp_level).pkg_tdp = (resp & GENMASK(14, 0)) as c_int;
    (*ctdp_level).tdp_ratio = ((resp & GENMASK(23, 16)) >> 16) as c_int;

    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_TDP_INFO resp:%x tdp_ratio:%d pkg_tdp:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).tdp_ratio,
        (*ctdp_level).pkg_tdp,
    );

    ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_TJMAX_INFO,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    (*ctdp_level).t_proc_hot = (resp & GENMASK(7, 0)) as c_int;

    _get_uncore_p0_p1_info(id, config_index, ctdp_level);
    _get_p1_info(id, config_index, ctdp_level);
    _get_uncore_mem_freq(id, config_index, ctdp_level);

    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_TJMAX_INFO resp:%x t_proc_hot:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).t_proc_hot,
    );

    0
}

unsafe fn mbox_get_pwr_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_PWR_INFO,
        0,
        config_index as c_uint,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    (*ctdp_level).pkg_max_power = (resp & GENMASK(14, 0)) as c_int;
    (*ctdp_level).pkg_min_power = ((resp & GENMASK(30, 16)) >> 16) as c_int;

    debug_printf(
        b"cpu:%d ctdp:%d CONFIG_TDP_GET_PWR_INFO resp:%x pkg_max_power:%d pkg_min_power:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        config_index,
        resp,
        (*ctdp_level).pkg_max_power,
        (*ctdp_level).pkg_min_power,
    );

    0
}

unsafe fn mbox_get_coremask_info(
    id: *mut isst_id,
    config_index: c_int,
    ctdp_level: *mut isst_pkg_ctdp_level_info,
) -> c_int {
    let mut resp: c_uint = 0;
    let mut i: c_int;

    (*ctdp_level).cpu_count = 0;
    i = 0;
    while i < 2 {
        let mask: u64;
        let mut cpu_count: c_int = 0;

        let ret = _send_mbox_command(
            (*id).cpu as c_uint,
            CONFIG_TDP,
            CONFIG_TDP_GET_CORE_MASK,
            0,
            ((i << 8) | config_index) as c_uint,
            &mut resp,
        );
        if ret != 0 {
            return ret;
        }

        debug_printf(
            b"cpu:%d ctdp:%d mask:%d CONFIG_TDP_GET_CORE_MASK resp:%x\n\0".as_ptr() as *const c_char,
            (*id).cpu,
            config_index,
            i,
            resp,
        );

        mask = (resp as u64) << (32 * i);
        set_cpu_mask_from_punit_coremask(
            id,
            mask,
            (*ctdp_level).core_cpumask_size,
            (*ctdp_level).core_cpumask,
            &mut cpu_count,
        );
        (*ctdp_level).cpu_count += cpu_count;
        debug_printf(
            b"cpu:%d ctdp:%d mask:%d cpu count:%d\n\0".as_ptr() as *const c_char,
            (*id).cpu,
            config_index,
            i,
            (*ctdp_level).cpu_count,
        );
        i += 1;
    }

    0
}

unsafe fn mbox_get_get_trl(id: *mut isst_id, level: c_int, avx_level: c_int, trl: *mut c_int) -> c_int {
    let mut resp: c_uint = 0;
    let mut req: c_uint = (level | (avx_level << 16)) as c_uint;
    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_TURBO_LIMIT_RATIOS,
        0,
        req,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_GET_TURBO_LIMIT_RATIOS req:%x resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, req, resp);

    *trl.add(0) = (resp & GENMASK(7, 0)) as c_int;
    *trl.add(1) = ((resp & GENMASK(15, 8)) >> 8) as c_int;
    *trl.add(2) = ((resp & GENMASK(23, 16)) >> 16) as c_int;
    *trl.add(3) = ((resp & GENMASK(31, 24)) >> 24) as c_int;

    req = (level as c_uint) | BIT(8) | ((avx_level as c_uint) << 16);
    ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_TURBO_LIMIT_RATIOS,
        0,
        req,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_GET_TURBO_LIMIT req:%x resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, req, resp);

    *trl.add(4) = (resp & GENMASK(7, 0)) as c_int;
    *trl.add(5) = ((resp & GENMASK(15, 8)) >> 8) as c_int;
    *trl.add(6) = ((resp & GENMASK(23, 16)) >> 16) as c_int;
    *trl.add(7) = ((resp & GENMASK(31, 24)) >> 24) as c_int;

    0
}

unsafe fn mbox_get_get_trls(id: *mut isst_id, level: c_int, ctdp_level: *mut isst_pkg_ctdp_level_info) -> c_int {
    let trl_max_levels = isst_get_trl_max_levels();
    let mut i: c_int = 0;

    while i < trl_max_levels {
        let ret = mbox_get_get_trl(id, level, i, (*ctdp_level).trl_ratios[i as usize].as_mut_ptr());
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn mbox_get_trl_bucket_info(id: *mut isst_id, _level: c_int, buckets_info: *mut u64) -> c_int {
    debug_printf(b"cpu:%d bucket info via MSR\n\0".as_ptr() as *const c_char, (*id).cpu);

    *buckets_info = 0;

    let ret = isst_send_msr_command((*id).cpu as c_uint, 0x1ae, 0, buckets_info);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d bucket info via MSR successful 0x%llx\n\0".as_ptr() as *const c_char, (*id).cpu, *buckets_info);

    0
}

unsafe fn mbox_set_tdp_level(id: *mut isst_id, tdp_level: c_int) -> c_int {
    let mut resp: c_uint = 0;

    if isst_get_config_tdp_lock_status(id) != 0 {
        isst_display_error_info_message(1, b"TDP is locked\0".as_ptr() as *const c_char, 0, 0);
        return -1;
    }

    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_TDP, CONFIG_TDP_SET_LEVEL, 0, tdp_level as c_uint, &mut resp);
    if ret != 0 {
        isst_display_error_info_message(1, b"Set TDP level failed for level\0".as_ptr() as *const c_char, 1, tdp_level);
        return ret;
    }

    0
}

unsafe fn mbox_get_pbf_info(id: *mut isst_id, level: c_int, pbf_info: *mut isst_pbf_info) -> c_int {
    let max_punit_core = get_max_punit_core_id(id);
    let max_mask_index = if max_punit_core > 32 { 2 } else { 1 };
    let mut resp: c_uint = 0;
    let mut i: c_int = 0;

    while i < max_mask_index {
        let mask: u64;
        let mut count: c_int = 0;

        let ret = _send_mbox_command(
            (*id).cpu as c_uint,
            CONFIG_TDP,
            CONFIG_TDP_PBF_GET_CORE_MASK_INFO,
            0,
            ((i << 8) | level) as c_uint,
            &mut resp,
        );
        if ret != 0 {
            break;
        }

        debug_printf(b"cpu:%d CONFIG_TDP_PBF_GET_CORE_MASK_INFO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

        mask = (resp as u64) << (32 * i);
        set_cpu_mask_from_punit_coremask(
            id,
            mask,
            (*pbf_info).core_cpumask_size,
            (*pbf_info).core_cpumask,
            &mut count,
        );
        i += 1;
    }

    let mut req: c_uint = level as c_uint;
    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_PBF_GET_P1HI_P1LO_INFO,
        0,
        req,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_PBF_GET_P1HI_P1LO_INFO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    (*pbf_info).p1_low = (resp & 0xff) as c_int;
    (*pbf_info).p1_high = ((resp & GENMASK(15, 8)) >> 8) as c_int;

    req = level as c_uint;
    ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_TDP, CONFIG_TDP_PBF_GET_TDP_INFO, 0, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_PBF_GET_TDP_INFO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    (*pbf_info).tdp = (resp & 0xffff) as c_int;

    req = level as c_uint;
    ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_TDP, CONFIG_TDP_PBF_GET_TJ_MAX_INFO, 0, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_PBF_GET_TJ_MAX_INFO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);
    (*pbf_info).t_control = ((resp >> 8) & 0xff) as c_int;
    (*pbf_info).t_prochot = (resp & 0xff) as c_int;

    0
}

unsafe fn mbox_set_pbf_fact_status(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int {
    let mut pkg_dev: isst_pkg_ctdp = core::mem::zeroed();
    let mut ctdp_level: isst_pkg_ctdp_level_info = core::mem::zeroed();
    let mut resp: c_uint = 0;
    let mut req: c_uint = 0;

    let mut ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 {
        debug_printf(b"cpu:%d No support for dynamic ISST\n\0".as_ptr() as *const c_char, (*id).cpu);
    }

    let current_level = pkg_dev.current_level;

    ret = isst_get_ctdp_control(id, current_level, &mut ctdp_level);
    if ret != 0 {
        return ret;
    }

    if pbf != 0 {
        if ctdp_level.fact_enabled != 0 {
            req = BIT(16);
        }

        if enable != 0 {
            req |= BIT(17);
        } else {
            req &= !BIT(17);
        }
    } else {
        if enable != 0 && ctdp_level.sst_cp_enabled == 0 {
            isst_display_error_info_message(0, b"Make sure to execute before: core-power enable\0".as_ptr() as *const c_char, 0, 0);
        }

        if ctdp_level.pbf_enabled != 0 {
            req = BIT(17);
        }

        if enable != 0 {
            req |= BIT(16);
        } else {
            req &= !BIT(16);
        }
    }

    ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_TDP, CONFIG_TDP_SET_TDP_CONTROL, 0, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_SET_TDP_CONTROL pbf/fact:%d req:%x\n\0".as_ptr() as *const c_char, (*id).cpu, pbf, req);

    0
}

unsafe fn _get_fact_bucket_info(
    id: *mut isst_id,
    level: c_int,
    bucket_info: *mut isst_fact_bucket_info,
) -> c_int {
    let trl_max_levels = isst_get_trl_max_levels();
    let mut resp: c_uint = 0;
    let mut i: c_int = 0;

    while i < 2 {
        let mut j: c_int = 0;

        let ret = _send_mbox_command(
            (*id).cpu as c_uint,
            CONFIG_TDP,
            CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_NUMCORES,
            0,
            ((i << 8) | level) as c_uint,
            &mut resp,
        );
        if ret != 0 {
            return ret;
        }

        debug_printf(
            b"cpu:%d CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_NUMCORES index:%d level:%d resp:%x\n\0".as_ptr() as *const c_char,
            (*id).cpu,
            i,
            level,
            resp,
        );

        while j < 4 {
            (*bucket_info.add((j + (i * 4)) as usize)).hp_cores = ((resp >> (j * 8)) & 0xff) as c_int;
            j += 1;
        }
        i += 1;
    }

    let mut k: c_int = 0;
    while k < trl_max_levels {
        i = 0;
        while i < 2 {
            let mut j: c_int = 0;

            let ret = _send_mbox_command(
                (*id).cpu as c_uint,
                CONFIG_TDP,
                CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_RATIOS,
                0,
                ((k << 16) | (i << 8) | level) as c_uint,
                &mut resp,
            );
            if ret != 0 {
                return ret;
            }

            debug_printf(
                b"cpu:%d CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_RATIOS index:%d level:%d avx:%d resp:%x\n\0".as_ptr() as *const c_char,
                (*id).cpu,
                i,
                level,
                k,
                resp,
            );

            while j < 4 {
                (*bucket_info.add((j + (i * 4)) as usize)).hp_ratios[k as usize] =
                    ((resp >> (j * 8)) & 0xff) as c_int;
                j += 1;
            }
            i += 1;
        }
        k += 1;
    }

    0
}

unsafe fn mbox_get_fact_info(
    id: *mut isst_id,
    level: c_int,
    fact_bucket: c_int,
    fact_info: *mut isst_fact_info,
) -> c_int {
    let mut resp: c_uint = 0;

    let mut ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_TDP,
        CONFIG_TDP_GET_FACT_LP_CLIPPING_RATIO,
        0,
        level as c_uint,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CONFIG_TDP_GET_FACT_LP_CLIPPING_RATIO resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    (*fact_info).lp_ratios[0] = (resp & 0xff) as c_int;
    (*fact_info).lp_ratios[1] = ((resp >> 8) & 0xff) as c_int;
    (*fact_info).lp_ratios[2] = ((resp >> 16) & 0xff) as c_int;

    ret = _get_fact_bucket_info(id, level, (*fact_info).bucket_info.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    let mut print: c_int = 0;
    let mut j: c_int = 0;
    while j < ISST_FACT_MAX_BUCKETS {
        if fact_bucket != 0xff && fact_bucket != j {
            j += 1;
            continue;
        }

        if (*fact_info).bucket_info[j as usize].hp_cores == 0 {
            break;
        }

        print = 1;
        j += 1;
    }
    if print == 0 {
        isst_display_error_info_message(1, b"Invalid bucket\0".as_ptr() as *const c_char, 0, 0);
        return -1;
    }

    0
}

unsafe fn mbox_get_clos_information(id: *mut isst_id, enable: *mut c_int, type_arg: *mut c_int) -> c_int {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PM_QOS_CONFIG, 0, 0, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PM_QOS_CONFIG resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    if (resp & BIT(1)) != 0 {
        *enable = 1;
    } else {
        *enable = 0;
    }

    if (resp & BIT(2)) != 0 {
        *type_arg = 1;
    } else {
        *type_arg = 0;
    }

    0
}

unsafe fn _write_pm_config(id: *mut isst_id, cp_state: c_int) -> c_int {
    let mut resp: c_uint = 0;
    let req: c_uint = if cp_state != 0 { BIT(16) } else { 0 };

    let ret = _send_mbox_command((*id).cpu as c_uint, WRITE_PM_CONFIG, PM_FEATURE, 0, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d WRITE_PM_CONFIG resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    0
}

unsafe fn mbox_pm_qos_config(id: *mut isst_id, enable_clos: c_int, priority_type: c_int) -> c_int {
    let mut resp: c_uint = 0;
    let mut req: c_uint;
    let mut ret: c_int;

    if enable_clos == 0 {
        let mut pkg_dev: isst_pkg_ctdp = core::mem::zeroed();
        let mut ctdp_level: isst_pkg_ctdp_level_info = core::mem::zeroed();

        ret = isst_get_ctdp_levels(id, &mut pkg_dev);
        if ret != 0 {
            debug_printf(b"isst_get_ctdp_levels\n\0".as_ptr() as *const c_char);
            return ret;
        }

        ret = isst_get_ctdp_control(id, pkg_dev.current_level, &mut ctdp_level);
        if ret != 0 {
            return ret;
        }

        if ctdp_level.fact_enabled != 0 {
            isst_display_error_info_message(1, b"Ignoring request, turbo-freq feature is still enabled\0".as_ptr() as *const c_char, 0, 0);
            return -EINVAL;
        }
        ret = _write_pm_config(id, 0);
        if ret != 0 {
            isst_display_error_info_message(0, b"WRITE_PM_CONFIG command failed, ignoring error\0".as_ptr() as *const c_char, 0, 0);
        }
    } else {
        ret = _write_pm_config(id, 1);
        if ret != 0 {
            isst_display_error_info_message(0, b"WRITE_PM_CONFIG command failed, ignoring error\0".as_ptr() as *const c_char, 0, 0);
        }
    }

    ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PM_QOS_CONFIG, 0, 0, &mut resp);
    if ret != 0 {
        isst_display_error_info_message(1, b"CLOS_PM_QOS_CONFIG command failed\0".as_ptr() as *const c_char, 0, 0);
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PM_QOS_CONFIG resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, resp);

    req = resp;

    if enable_clos != 0 {
        req = req | BIT(1);
    } else {
        req = req & !BIT(1);
    }

    if priority_type > 1 {
        isst_display_error_info_message(1, b"Invalid priority type: Changing type to ordered\0".as_ptr() as *const c_char, 0, 0);
    }

    if priority_type != 0 {
        req = req | BIT(2);
    } else {
        req = req & !BIT(2);
    }

    ret = _send_mbox_command(
        (*id).cpu as c_uint,
        CONFIG_CLOS,
        CLOS_PM_QOS_CONFIG,
        BIT(MBOX_CMD_WRITE_BIT),
        req,
        &mut resp,
    );
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PM_QOS_CONFIG priority type:%d req:%x\n\0".as_ptr() as *const c_char, (*id).cpu, priority_type, req);

    0
}

unsafe fn mbox_pm_get_clos(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int {
    let mut resp: c_uint = 0;
    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PM_CLOS, clos as c_uint, 0, &mut resp);
    if ret != 0 {
        return ret;
    }

    (*clos_config).epp = (resp & 0x0f) as c_int;
    (*clos_config).clos_prop_prio = ((resp >> 4) & 0x0f) as c_int;
    (*clos_config).clos_min = ((resp >> 8) & 0xff) as c_int;
    (*clos_config).clos_max = ((resp >> 16) & 0xff) as c_int;
    (*clos_config).clos_desired = ((resp >> 24) & 0xff) as c_int;

    0
}

unsafe fn mbox_set_clos(id: *mut isst_id, clos: c_int, clos_config: *mut isst_clos_config) -> c_int {
    let mut resp: c_uint = 0;
    let mut req: c_uint;

    req = ((*clos_config).epp & 0x0f) as c_uint;
    req |= (((*clos_config).clos_prop_prio & 0x0f) as c_uint) << 4;
    req |= (((*clos_config).clos_min & 0xff) as c_uint) << 8;
    req |= (((*clos_config).clos_max & 0xff) as c_uint) << 16;
    req |= (((*clos_config).clos_desired & 0xff) as c_uint) << 24;

    let param = BIT(MBOX_CMD_WRITE_BIT) | clos as c_uint;

    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PM_CLOS, param, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PM_CLOS param:%x req:%x\n\0".as_ptr() as *const c_char, (*id).cpu, param, req);

    0
}

unsafe fn mbox_clos_get_assoc_status(id: *mut isst_id, clos_id: *mut c_int) -> c_int {
    let mut resp: c_uint = 0;
    let core_id = find_phy_core_num((*id).cpu);
    let param: c_uint = core_id as c_uint;

    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PQR_ASSOC, param, 0, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PQR_ASSOC param:%x resp:%x\n\0".as_ptr() as *const c_char, (*id).cpu, param, resp);
    *clos_id = ((resp >> 16) & 0x03) as c_int;

    0
}

unsafe fn mbox_clos_associate(id: *mut isst_id, clos_id: c_int) -> c_int {
    let mut resp: c_uint = 0;
    let req: c_uint = ((clos_id & 0x03) as c_uint) << 16;
    let core_id = find_phy_core_num((*id).cpu);
    let param = BIT(MBOX_CMD_WRITE_BIT) | core_id as c_uint;

    let ret = _send_mbox_command((*id).cpu as c_uint, CONFIG_CLOS, CLOS_PQR_ASSOC, param, req, &mut resp);
    if ret != 0 {
        return ret;
    }

    debug_printf(b"cpu:%d CLOS_PQR_ASSOC param:%x req:%x\n\0".as_ptr() as *const c_char, (*id).cpu, param, req);

    0
}

static mut mbox_ops: isst_platform_ops = isst_platform_ops {
    get_disp_freq_multiplier: Some(mbox_get_disp_freq_multiplier),
    get_trl_max_levels: Some(mbox_get_trl_max_levels),
    get_trl_level_name: Some(mbox_get_trl_level_name),
    update_platform_param: Some(mbox_update_platform_param),
    is_punit_valid: Some(mbox_is_punit_valid),
    read_pm_config: Some(mbox_read_pm_config),
    get_config_levels: Some(mbox_get_config_levels),
    get_ctdp_control: Some(mbox_get_ctdp_control),
    get_tdp_info: Some(mbox_get_tdp_info),
    get_pwr_info: Some(mbox_get_pwr_info),
    get_coremask_info: Some(mbox_get_coremask_info),
    get_get_trl: Some(mbox_get_get_trl),
    get_get_trls: Some(mbox_get_get_trls),
    get_trl_bucket_info: Some(mbox_get_trl_bucket_info),
    set_tdp_level: Some(mbox_set_tdp_level),
    get_pbf_info: Some(mbox_get_pbf_info),
    set_pbf_fact_status: Some(mbox_set_pbf_fact_status),
    get_fact_info: Some(mbox_get_fact_info),
    adjust_uncore_freq: Some(mbox_adjust_uncore_freq),
    get_clos_information: Some(mbox_get_clos_information),
    pm_qos_config: Some(mbox_pm_qos_config),
    pm_get_clos: Some(mbox_pm_get_clos),
    set_clos: Some(mbox_set_clos),
    clos_get_assoc_status: Some(mbox_clos_get_assoc_status),
    clos_associate: Some(mbox_clos_associate),
};

#[no_mangle]
pub unsafe extern "C" fn mbox_get_platform_ops() -> *mut isst_platform_ops {
    &mut mbox_ops
}
