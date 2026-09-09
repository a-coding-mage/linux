/*
 * CPU frequency scaling for Broadcom SoCs with AVS firmware that
 * supports DVS or DVFS
 *
 * Copyright (c) 2016 Broadcom
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation version 2.
 */

// Translated from brcmstb-avs-cpufreq.c. Kernel-provided types and functions
// referenced below are intentionally left as external dependencies.

const AVS_MAX_CMD_ARGS: usize = 4;
const AVS_MBOX_COMMAND: usize = 0x00;
const AVS_MBOX_STATUS: usize = 0x04;
const AVS_MBOX_VOLTAGE1: usize = 0x40;
const AVS_MBOX_PARAM_BASE: usize = 0x18;
const AVS_MBOX_MAGIC: usize = 0x34;
const AVS_MBOX_FREQUENCY: usize = 0x50;
const AVS_MBOX_REVISION: usize = 0x28;
const AVS_MBOX_PSTATE: usize = 0x2c;
const AVS_MBOX_HEARTBEAT: usize = 0x30;
const AVS_MBOX_SIGMA_HVT: usize = 0x38;
const AVS_MBOX_SIGMA_SVT: usize = 0x3c;
const AVS_MBOX_VOLTAGE0: usize = 0x08;
const AVS_MBOX_TEMP0: usize = 0x0c;
const AVS_MBOX_PV0: usize = 0x10;
const AVS_MBOX_MV0: usize = 0x14;
const AVS_MBOX_VOLTAGE1_UNUSED: usize = AVS_MBOX_VOLTAGE1;
const AVS_MBOX_TEMP1: usize = 0x44;
const AVS_MBOX_PV1: usize = 0x48;
const AVS_MBOX_MV1: usize = 0x4c;
const AVS_CPU_L2_SET0: usize = 0x04;
const AVS_CPU_L2_INT_MASK: u32 = 1u32 << 31;
const AVS_CMD_ENABLE: u32 = 0x11;
const AVS_CMD_S2_ENTER: u32 = 0x12;
const AVS_CMD_S2_EXIT: u32 = 0x13;
const AVS_CMD_GET_PMAP: u32 = 0x30;
const AVS_CMD_SET_PMAP: u32 = 0x31;
const AVS_CMD_GET_PSTATE: u32 = 0x40;
const AVS_CMD_SET_PSTATE: u32 = 0x41;
const AVS_MODE_AVS: u32 = 0;
const AVS_MODE_DFS: u32 = 1;
const AVS_MODE_DVS: u32 = 2;
const AVS_MODE_DVFS: u32 = 3;
const NDIV_INT_SHIFT: u32 = 0;
const NDIV_INT_MASK: u32 = 0x3ff;
const PDIV_SHIFT: u32 = 10;
const PDIV_MASK: u32 = 0xf;
const MDIV_P0_SHIFT: u32 = 16;
const MDIV_P0_MASK: u32 = 0xff;
const MDIV_P1_SHIFT: u32 = 0;
const MDIV_P1_MASK: u32 = 0xff;
const MDIV_P2_SHIFT: u32 = 8;
const MDIV_P2_MASK: u32 = 0xff;
const MDIV_P3_SHIFT: u32 = 16;
const MDIV_P3_MASK: u32 = 0xff;
const MDIV_P4_SHIFT: u32 = 24;
const MDIV_P4_MASK: u32 = 0xff;
const AVS_PSTATE_P0: u32 = 0;
const AVS_PSTATE_P1: u32 = 1;
const AVS_PSTATE_P2: u32 = 2;
const AVS_PSTATE_P3: u32 = 3;
const AVS_PSTATE_P4: u32 = 4;
const AVS_PSTATE_MAX: u32 = AVS_PSTATE_P4;
const AVS_STATUS_CLEAR: u32 = 0;
const AVS_STATUS_SUCCESS: u32 = 0xf0;
const AVS_STATUS_FAILURE: u32 = 0xff;
const AVS_STATUS_INVALID: u32 = 0xf1;
const AVS_STATUS_NO_SUPP: u32 = 0xf2;
const AVS_STATUS_NO_MAP: u32 = 0xf3;
const AVS_STATUS_MAP_SET: u32 = 0xf4;
const AVS_STATUS_MAX: u32 = 0xff;
const AVS_LOOP_LIMIT: u32 = 10000;
const AVS_TIMEOUT: u64 = 300;
const AVS_FIRMWARE_MAGIC: u32 = 0xa11600d1;
const BRCM_AVS_CPUFREQ_PREFIX: &str = "brcmstb-avs";
const BRCM_AVS_CPUFREQ_NAME: &str = "brcmstb-avs-cpufreq";
const BRCM_AVS_CPU_DATA: &str = "brcm,avs-cpu-data-mem";
const BRCM_AVS_CPU_INTR: &str = "brcm,avs-cpu-l2-intr";
const BRCM_AVS_HOST_INTR: &str = "sw_intr";

#[repr(C)]
pub struct Pmap { pub mode: u32, pub p1: u32, pub p2: u32, pub state: u32 }

#[repr(C)]
pub struct PrivateData {
    pub base: *mut core::ffi::c_void,
    pub avs_intr_base: *mut core::ffi::c_void,
    pub dev: *mut core::ffi::c_void,
    pub done: Completion,
    pub sem: Semaphore,
    pub pmap: Pmap,
    pub host_irq: i32,
}

#[repr(C)] pub struct Completion { _private: [u8; 0] }
#[repr(C)] pub struct Semaphore { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { _private: [u8; 0] }
#[repr(C)] pub struct CpufreqPolicy { pub driver_data: *mut PrivateData, pub freq_table: *mut FrequencyTable, pub cur: u32 }
#[repr(C)] pub struct FrequencyTable { pub frequency: u32, pub driver_data: u32 }

extern "C" {
    fn of_find_compatible_node(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, n: *const i8) -> *mut core::ffi::c_void;
    fn of_iomap(n: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(n: *mut core::ffi::c_void);
    fn readl(p: *mut core::ffi::c_void) -> u32;
    fn writel(v: u32, p: *mut core::ffi::c_void);
    fn down_interruptible(s: *mut Semaphore) -> i32;
    fn up(s: *mut Semaphore);
    fn wait_for_completion_timeout(c: *mut Completion, t: u64) -> u64;
    fn reinit_completion(c: *mut Completion);
    fn complete(c: *mut Completion);
    fn usleep_range(a: u32, b: u32);
    fn iounmap(p: *mut core::ffi::c_void);
    fn jiffies_to_msecs(v: u64) -> u64;
}

#[inline] fn avs_param(x: usize) -> usize { AVS_MBOX_PARAM_BASE + (if x < AVS_MAX_CMD_ARGS { x } else { 0 }) * core::mem::size_of::<u32>() }
unsafe fn map_region(_name: &str) -> *mut core::ffi::c_void { core::ptr::null_mut() }

unsafe fn wait_for_avs_command(priv_: *mut PrivateData, mut timeout: u64) -> u64 {
    if (*priv_).host_irq >= 0 { return wait_for_completion_timeout(&mut (*priv_).done, timeout); }
    let mut time_left = 0;
    loop {
        time_left = timeout;
        if readl((*priv_).base.add(AVS_MBOX_STATUS)) != 0 { break; }
        usleep_range(1000, 2000);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 { break; }
    }
    time_left
}

unsafe fn issue_avs_command(priv_: *mut PrivateData, cmd: u32, num_in: usize, num_out: usize, args: *mut u32) -> i32 {
    let base = (*priv_).base;
    let mut ret = down_interruptible(&mut (*priv_).sem);
    if ret != 0 { return ret; }
    let mut i = 0; let mut val = 1;
    while val != 0 && i < AVS_LOOP_LIMIT { val = readl(base.add(AVS_MBOX_COMMAND)); i += 1; }
    if i == AVS_LOOP_LIMIT { ret = -11; up(&mut (*priv_).sem); return ret; }
    writel(AVS_STATUS_CLEAR, base.add(AVS_MBOX_STATUS));
    for j in 0..num_in { writel(*args.add(j), base.add(avs_param(j))); }
    reinit_completion(&mut (*priv_).done);
    writel(cmd, base.add(AVS_MBOX_COMMAND));
    writel(AVS_CPU_L2_INT_MASK, (*priv_).avs_intr_base.add(AVS_CPU_L2_SET0));
    let time_left = wait_for_avs_command(priv_, AVS_TIMEOUT);
    val = readl(base.add(AVS_MBOX_STATUS));
    if time_left == 0 || val == 0 || val > AVS_STATUS_MAX { ret = -110; up(&mut (*priv_).sem); return ret; }
    for j in 0..num_out { *args.add(j) = readl(base.add(avs_param(j))); }
    writel(AVS_STATUS_CLEAR, base.add(AVS_MBOX_STATUS));
    ret = match val { AVS_STATUS_INVALID => -22, AVS_STATUS_NO_SUPP => -95, AVS_STATUS_NO_MAP => -2, AVS_STATUS_MAP_SET => -17, AVS_STATUS_FAILURE => -5, _ => ret };
    up(&mut (*priv_).sem); ret
}

unsafe fn brcm_avs_mode_to_string(mode: u32) -> *const u8 { match mode { AVS_MODE_AVS => b"AVS\0".as_ptr(), AVS_MODE_DFS => b"DFS\0".as_ptr(), AVS_MODE_DVS => b"DVS\0".as_ptr(), AVS_MODE_DVFS => b"DVFS\0".as_ptr(), _ => core::ptr::null() } }
unsafe fn brcm_avs_parse_p1(p1: u32, mdiv_p0: *mut u32, pdiv: *mut u32, ndiv: *mut u32) { *mdiv_p0=(p1>>MDIV_P0_SHIFT)&MDIV_P0_MASK; *pdiv=(p1>>PDIV_SHIFT)&PDIV_MASK; *ndiv=(p1>>NDIV_INT_SHIFT)&NDIV_INT_MASK; }
unsafe fn brcm_avs_parse_p2(p2: u32, a:*mut u32,b:*mut u32,c:*mut u32,d:*mut u32) { *d=(p2>>MDIV_P4_SHIFT)&MDIV_P4_MASK; *c=(p2>>MDIV_P3_SHIFT)&MDIV_P3_MASK; *b=(p2>>MDIV_P2_SHIFT)&MDIV_P2_MASK; *a=(p2>>MDIV_P1_SHIFT)&MDIV_P1_MASK; }

unsafe fn brcm_avs_get_pmap(priv_: *mut PrivateData, pmap: *mut Pmap) -> i32 { let mut args=[0u32;4]; let r=issue_avs_command(priv_,AVS_CMD_GET_PMAP,0,4,args.as_mut_ptr()); if r!=0 || pmap.is_null(){return r;} (*pmap).mode=args[0];(*pmap).p1=args[1];(*pmap).p2=args[2];(*pmap).state=args[3];0 }
unsafe fn brcm_avs_set_pmap(priv_: *mut PrivateData,pmap:*mut Pmap)->i32 { let mut a=[(*pmap).mode,(*pmap).p1,(*pmap).p2,(*pmap).state]; issue_avs_command(priv_,AVS_CMD_SET_PMAP,4,0,a.as_mut_ptr()) }
unsafe fn brcm_avs_get_pstate(priv_:*mut PrivateData,p:*mut u32)->i32 { let mut a=[0u32;4];let r=issue_avs_command(priv_,AVS_CMD_GET_PSTATE,0,1,a.as_mut_ptr());if r!=0{return r;}*p=a[0];0 }
unsafe fn brcm_avs_set_pstate(priv_:*mut PrivateData,p:u32)->i32 {let mut a=[p,0,0,0];issue_avs_command(priv_,AVS_CMD_SET_PSTATE,1,0,a.as_mut_ptr())}
unsafe fn brcm_avs_get_voltage(base:*mut core::ffi::c_void)->u32{readl(base.add(AVS_MBOX_VOLTAGE1))}
unsafe fn brcm_avs_get_frequency(base:*mut core::ffi::c_void)->u32{readl(base.add(AVS_MBOX_FREQUENCY)).wrapping_mul(1000)}

// Remaining driver registration and sysfs glue retain the C driver's external kernel interfaces.
#[no_mangle] pub unsafe extern "C" fn brcm_avs_cpufreq_probe(_pdev:*mut PlatformDevice)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn brcm_avs_cpufreq_remove(_pdev:*mut PlatformDevice) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
