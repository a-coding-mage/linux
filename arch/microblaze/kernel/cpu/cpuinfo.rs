/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct cpu_ver_key {
    pub key: *const c_char,
    pub value: u32,
}

#[repr(C)]
pub struct family_string_key {
    pub key: *const c_char,
    pub value: u32,
}

#[repr(C)]
pub struct cpuinfo {
    pub mmu_privins: u32,
    pub cpu_clock_freq: u32,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub static cpu_ver_lookup: &[cpu_ver_key] = &[
    /* These key value are as per MBV field in PVR0 */
    cpu_ver_key { key: c"5.00.a".as_ptr(), value: 0x01 },
    cpu_ver_key { key: c"5.00.b".as_ptr(), value: 0x02 },
    cpu_ver_key { key: c"5.00.c".as_ptr(), value: 0x03 },
    cpu_ver_key { key: c"6.00.a".as_ptr(), value: 0x04 },
    cpu_ver_key { key: c"6.00.b".as_ptr(), value: 0x06 },
    cpu_ver_key { key: c"7.00.a".as_ptr(), value: 0x05 },
    cpu_ver_key { key: c"7.00.b".as_ptr(), value: 0x07 },
    cpu_ver_key { key: c"7.10.a".as_ptr(), value: 0x08 },
    cpu_ver_key { key: c"7.10.b".as_ptr(), value: 0x09 },
    cpu_ver_key { key: c"7.10.c".as_ptr(), value: 0x0a },
    cpu_ver_key { key: c"7.10.d".as_ptr(), value: 0x0b },
    cpu_ver_key { key: c"7.20.a".as_ptr(), value: 0x0c },
    cpu_ver_key { key: c"7.20.b".as_ptr(), value: 0x0d },
    cpu_ver_key { key: c"7.20.c".as_ptr(), value: 0x0e },
    cpu_ver_key { key: c"7.20.d".as_ptr(), value: 0x0f },
    cpu_ver_key { key: c"7.30.a".as_ptr(), value: 0x10 },
    cpu_ver_key { key: c"7.30.b".as_ptr(), value: 0x11 },
    cpu_ver_key { key: c"8.00.a".as_ptr(), value: 0x12 },
    cpu_ver_key { key: c"8.00.b".as_ptr(), value: 0x13 },
    cpu_ver_key { key: c"8.10.a".as_ptr(), value: 0x14 },
    cpu_ver_key { key: c"8.20.a".as_ptr(), value: 0x15 },
    cpu_ver_key { key: c"8.20.b".as_ptr(), value: 0x16 },
    cpu_ver_key { key: c"8.30.a".as_ptr(), value: 0x17 },
    cpu_ver_key { key: c"8.40.a".as_ptr(), value: 0x18 },
    cpu_ver_key { key: c"8.40.b".as_ptr(), value: 0x19 },
    cpu_ver_key { key: c"8.50.a".as_ptr(), value: 0x1a },
    cpu_ver_key { key: c"8.50.b".as_ptr(), value: 0x1c },
    cpu_ver_key { key: c"8.50.c".as_ptr(), value: 0x1e },
    cpu_ver_key { key: c"9.0".as_ptr(), value: 0x1b },
    cpu_ver_key { key: c"9.1".as_ptr(), value: 0x1d },
    cpu_ver_key { key: c"9.2".as_ptr(), value: 0x1f },
    cpu_ver_key { key: c"9.3".as_ptr(), value: 0x20 },
    cpu_ver_key { key: c"9.4".as_ptr(), value: 0x21 },
    cpu_ver_key { key: c"9.5".as_ptr(), value: 0x22 },
    cpu_ver_key { key: c"9.6".as_ptr(), value: 0x23 },
    cpu_ver_key { key: c"10.0".as_ptr(), value: 0x24 },
    cpu_ver_key { key: c"11.0".as_ptr(), value: 0x25 },
    cpu_ver_key { key: core::ptr::null(), value: 0 },
];

/* FIXME Not sure if the actual key is defined by Xilinx in the PVR */
pub static family_string_lookup: &[family_string_key] = &[
    family_string_key { key: c"virtex2".as_ptr(), value: 0x4 },
    family_string_key { key: c"virtex2pro".as_ptr(), value: 0x5 },
    family_string_key { key: c"spartan3".as_ptr(), value: 0x6 },
    family_string_key { key: c"virtex4".as_ptr(), value: 0x7 },
    family_string_key { key: c"virtex5".as_ptr(), value: 0x8 },
    family_string_key { key: c"spartan3e".as_ptr(), value: 0x9 },
    family_string_key { key: c"spartan3a".as_ptr(), value: 0xa },
    family_string_key { key: c"spartan3an".as_ptr(), value: 0xb },
    family_string_key { key: c"spartan3adsp".as_ptr(), value: 0xc },
    family_string_key { key: c"spartan6".as_ptr(), value: 0xd },
    family_string_key { key: c"virtex6".as_ptr(), value: 0xe },
    family_string_key { key: c"virtex7".as_ptr(), value: 0xf },
    /* FIXME There is no key code defined for spartan2 */
    family_string_key { key: c"spartan2".as_ptr(), value: 0xf0 },
    family_string_key { key: c"kintex7".as_ptr(), value: 0x10 },
    family_string_key { key: c"artix7".as_ptr(), value: 0x11 },
    family_string_key { key: c"zynq7000".as_ptr(), value: 0x12 },
    family_string_key { key: c"UltraScale Virtex".as_ptr(), value: 0x13 },
    family_string_key { key: c"UltraScale Kintex".as_ptr(), value: 0x14 },
    family_string_key { key: c"UltraScale+ Zynq".as_ptr(), value: 0x15 },
    family_string_key { key: c"UltraScale+ Virtex".as_ptr(), value: 0x16 },
    family_string_key { key: c"UltraScale+ Kintex".as_ptr(), value: 0x17 },
    family_string_key { key: c"Spartan7".as_ptr(), value: 0x18 },
    family_string_key { key: core::ptr::null(), value: 0 },
];

pub static mut cpuinfo: cpuinfo = cpuinfo { mmu_privins: 0, cpu_clock_freq: 0 };
static mut cpu: *mut device_node = core::ptr::null_mut();

extern "C" {
    fn of_get_cpu_node(cpu: i32, thread: *const core::ffi::c_void) -> *mut device_node;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn cpu_has_pvr() -> i32;
    fn set_cpuinfo_static(info: *mut cpuinfo, node: *mut device_node);
    fn set_cpuinfo_pvr_full(info: *mut cpuinfo, node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn of_clk_get(node: *mut device_node, index: i32) -> *mut clk;
    fn clk_get_rate(clock: *mut clk) -> u32;
    fn fcpu(node: *mut device_node, name: *const c_char) -> u32;
    fn bug();
}

pub unsafe fn setup_cpuinfo() {
    cpu = of_get_cpu_node(0, core::ptr::null());
    if cpu.is_null() {
        pr_err(c"You don't have cpu or are missing cpu reg property!!!\n".as_ptr());
    }

    pr_info(c"setup_cpuinfo: initialising\n".as_ptr());

    match cpu_has_pvr() {
        0 => {
            pr_warn(c"setup_cpuinfo: No PVR support. Using static CPU info from FDT\n".as_ptr());
            set_cpuinfo_static(&raw mut cpuinfo, cpu);
        }
        1 => {
            pr_info(c"setup_cpuinfo: Using full CPU PVR support\n".as_ptr());
            set_cpuinfo_static(&raw mut cpuinfo, cpu);
            set_cpuinfo_pvr_full(&raw mut cpuinfo, cpu);
        }
        _ => {
            pr_warn(c"setup_cpuinfo: Unsupported PVR setting\n".as_ptr());
            set_cpuinfo_static(&raw mut cpuinfo, cpu);
        }
    }

    /* FIXME I found weird behavior with MB 7.00.a/b 7.10.a
     * please do not use FULL PVR with MMU */
    if cpuinfo.mmu_privins != 0 {
        pr_warn(c"setup_cpuinfo: Stream instructions enabled - USERSPACE CAN LOCK THIS KERNEL!\n".as_ptr());
    }

    of_node_put(cpu);
}

pub unsafe fn setup_cpuinfo_clk() {
    let clock = of_clk_get(cpu, 0);
    /* IS_ERR(clock) is supplied by the kernel dependency. */
    if (clock as usize) == usize::MAX {
        pr_err(c"ERROR: CPU CCF input clock not found\n".as_ptr());
        /* take timebase-frequency from DTS */
        cpuinfo.cpu_clock_freq = fcpu(cpu, c"timebase-frequency".as_ptr());
    } else {
        cpuinfo.cpu_clock_freq = clk_get_rate(clock);
    }

    if cpuinfo.cpu_clock_freq == 0 {
        pr_err(c"ERROR: CPU clock frequency not setup\n".as_ptr());
        bug();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
