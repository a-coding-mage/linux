// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023 Ventana Micro Systems Inc.
 */

// pr_fmt(fmt) = "riscv-pv: " fmt

// Linux and RISC-V header dependencies are supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    static mut sbi_spec_version: c_ulong;
    static mut steal_time: SbiStaStruct;

    fn sbi_mk_version(major: c_ulong, minor: c_ulong) -> c_ulong;
    fn sbi_probe_extension(extension: c_ulong) -> c_long;
    fn sbi_ecall(
        ext: c_ulong,
        fid: c_ulong,
        arg0: c_ulong,
        arg1: c_ulong,
        arg2: c_ulong,
        arg3: c_ulong,
        arg4: c_ulong,
        arg5: c_ulong,
    ) -> SbiRet;
    fn sbi_err_map_linux_errno(error: c_long) -> c_int;
    fn cpuhp_setup_state(
        state: c_int,
        name: *const c_char,
        startup: unsafe extern "C" fn(c_uint) -> c_int,
        teardown: unsafe extern "C" fn(c_uint) -> c_int,
    ) -> c_int;
    fn static_call_update(call: *mut core::ffi::c_void, function: unsafe extern "C" fn(c_int) -> u64);
    fn static_key_slow_inc(key: *mut core::ffi::c_void);
    fn __pa(address: *const SbiStaStruct) -> u64;
    fn per_cpu_ptr(base: *mut SbiStaStruct, cpu: c_int) -> *mut SbiStaStruct;
    fn this_cpu_ptr(base: *mut SbiStaStruct) -> *mut SbiStaStruct;
    fn virt_rmb();
    fn pr_info(format: *const c_char, ...);
    fn pr_warn(format: *const c_char, ...);

    static mut paravirt_steal_enabled: core::ffi::c_void;
    static mut paravirt_steal_rq_enabled: core::ffi::c_void;
}

type c_long = isize;

#[repr(C)]
pub struct SbiStaStruct {
    pub sequence: u32,
    pub steal: u64,
}

#[repr(C)]
pub struct SbiRet {
    pub error: c_long,
    pub value: c_long,
}

static mut steal_acc: bool = true;

unsafe extern "C" fn parse_no_stealacc(_arg: *mut c_char) -> c_int {
    steal_acc = false;
    0
}

// early_param("no-steal-acc", parse_no_stealacc);

// DEFINE_PER_CPU(struct sbi_sta_struct, steal_time) __aligned(64);

unsafe fn has_pv_steal_clock() -> bool {
    if sbi_spec_version >= sbi_mk_version(2, 0)
        && sbi_probe_extension(SBI_EXT_STA) > 0
    {
        pr_info(b"SBI STA extension detected\n\0".as_ptr() as *const c_char);
        return true;
    }

    false
}

unsafe fn sbi_sta_steal_time_set_shmem(
    lo: c_ulong,
    hi: c_ulong,
    flags: c_ulong,
) -> c_int {
    let ret = sbi_ecall(
        SBI_EXT_STA,
        SBI_EXT_STA_STEAL_TIME_SET_SHMEM,
        lo,
        hi,
        flags,
        0,
        0,
        0,
    );
    if ret.error != 0 {
        if lo == SBI_SHMEM_DISABLE && hi == SBI_SHMEM_DISABLE {
            pr_warn(b"Failed to disable steal-time shmem\0".as_ptr() as *const c_char);
        } else {
            pr_warn(b"Failed to set steal-time shmem\0".as_ptr() as *const c_char);
        }
        return sbi_err_map_linux_errno(ret.error);
    }

    0
}

unsafe extern "C" fn pv_time_cpu_online(_cpu: c_uint) -> c_int {
    let st = this_cpu_ptr(&mut steal_time);
    let pa = __pa(st);
    let lo = pa as c_ulong;
    let hi: c_ulong = 0; // IS_ENABLED(CONFIG_32BIT) ? upper_32_bits((u64)pa) : 0

    sbi_sta_steal_time_set_shmem(lo, hi, 0)
}

unsafe extern "C" fn pv_time_cpu_down_prepare(_cpu: c_uint) -> c_int {
    sbi_sta_steal_time_set_shmem(SBI_SHMEM_DISABLE, SBI_SHMEM_DISABLE, 0)
}

unsafe extern "C" fn pv_time_steal_clock(cpu: c_int) -> u64 {
    let st = per_cpu_ptr(&mut steal_time, cpu);
    let mut sequence: u32;
    let mut steal: u64;

    /*
     * Check the sequence field before and after reading the steal
     * field. Repeat the read if it is different or odd.
     */
    loop {
        sequence = core::ptr::read_volatile(&(*st).sequence);
        virt_rmb();
        steal = core::ptr::read_volatile(&(*st).steal);
        virt_rmb();
        if (sequence & 1) == 0
            && sequence == core::ptr::read_volatile(&(*st).sequence)
        {
            break;
        }
    }

    steal
}

pub unsafe extern "C" fn pv_time_init() -> c_int {
    let mut ret: c_int;

    if !has_pv_steal_clock() {
        return 0;
    }

    ret = cpuhp_setup_state(
        CPUHP_AP_ONLINE_DYN,
        b"riscv/pv_time:online\0".as_ptr() as *const c_char,
        pv_time_cpu_online,
        pv_time_cpu_down_prepare,
    );
    if ret < 0 {
        return ret;
    }

    // static_call_update(pv_steal_clock, pv_time_steal_clock);

    static_key_slow_inc(&mut paravirt_steal_enabled);
    if steal_acc {
        static_key_slow_inc(&mut paravirt_steal_rq_enabled);
    }

    pr_info(b"Computing paravirt steal-time\n\0".as_ptr() as *const c_char);

    0
}

const SBI_EXT_STA: c_ulong = 0x535441;
const SBI_EXT_STA_STEAL_TIME_SET_SHMEM: c_ulong = 0;
const SBI_SHMEM_DISABLE: c_ulong = 0;
const CPUHP_AP_ONLINE_DYN: c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
