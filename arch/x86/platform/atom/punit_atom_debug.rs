// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel SOC Punit device state debug driver
 * Punit controls power management for North Complex devices (Graphics
 * blocks, Image Signal Processing, video processing, display, DSP etc.)
 *
 * Copyright (c) 2015, Intel Corporation.
 */

// Linux kernel headers provide the types, constants, and functions referenced below.

use core::ffi::{c_char, c_int, c_uint, c_void};

const BT_MBI_UNIT_PMC: c_int = 0;
const MBI_REG_READ: c_int = 0;
const ENODEV: c_int = 19;

const VED_SS_PM0: c_int = 0x32;
const ISP_SS_PM0: c_int = 0x39;
const MIO_SS_PM: c_int = 0x3B;
const SSS_SHIFT: c_int = 24;
const PWRGT_STATUS: c_int = 0x61;
const RENDER_POS: c_int = 0;
const MEDIA_POS: c_int = 2;
const VLV_DISPLAY_POS: c_int = 6;
const CHT_DSP_SSS: c_int = 0x36;
const CHT_DSP_SSS_POS: c_int = 16;

#[repr(C)]
pub struct punit_device {
    pub name: *mut c_char,
    pub reg: c_int,
    pub sss_pos: c_int,
}

static mut punit_device_tng: [punit_device; 5] = [
    punit_device { name: b"DISPLAY\0".as_ptr() as *mut c_char, reg: CHT_DSP_SSS, sss_pos: SSS_SHIFT },
    punit_device { name: b"VED\0".as_ptr() as *mut c_char, reg: VED_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"ISP\0".as_ptr() as *mut c_char, reg: ISP_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"MIO\0".as_ptr() as *mut c_char, reg: MIO_SS_PM, sss_pos: SSS_SHIFT },
    punit_device { name: core::ptr::null_mut(), reg: 0, sss_pos: 0 },
];

static mut punit_device_byt: [punit_device; 7] = [
    punit_device { name: b"GFX RENDER\0".as_ptr() as *mut c_char, reg: PWRGT_STATUS, sss_pos: RENDER_POS },
    punit_device { name: b"GFX MEDIA\0".as_ptr() as *mut c_char, reg: PWRGT_STATUS, sss_pos: MEDIA_POS },
    punit_device { name: b"DISPLAY\0".as_ptr() as *mut c_char, reg: PWRGT_STATUS, sss_pos: VLV_DISPLAY_POS },
    punit_device { name: b"VED\0".as_ptr() as *mut c_char, reg: VED_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"ISP\0".as_ptr() as *mut c_char, reg: ISP_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"MIO\0".as_ptr() as *mut c_char, reg: MIO_SS_PM, sss_pos: SSS_SHIFT },
    punit_device { name: core::ptr::null_mut(), reg: 0, sss_pos: 0 },
];

static mut punit_device_cht: [punit_device; 7] = [
    punit_device { name: b"GFX RENDER\0".as_ptr() as *mut c_char, reg: PWRGT_STATUS, sss_pos: RENDER_POS },
    punit_device { name: b"GFX MEDIA\0".as_ptr() as *mut c_char, reg: PWRGT_STATUS, sss_pos: MEDIA_POS },
    punit_device { name: b"DISPLAY\0".as_ptr() as *mut c_char, reg: CHT_DSP_SSS, sss_pos: CHT_DSP_SSS_POS },
    punit_device { name: b"VED\0".as_ptr() as *mut c_char, reg: VED_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"ISP\0".as_ptr() as *mut c_char, reg: ISP_SS_PM0, sss_pos: SSS_SHIFT },
    punit_device { name: b"MIO\0".as_ptr() as *mut c_char, reg: MIO_SS_PM, sss_pos: SSS_SHIFT },
    punit_device { name: core::ptr::null_mut(), reg: 0, sss_pos: 0 },
];

static dstates: [&'static [u8]; 4] = [b"D0\0", b"D0i1\0", b"D0i2\0", b"D0i3\0"];

#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct x86_cpu_id { pub driver_data: *const c_void }
#[repr(C)] pub struct acpi_s2idle_dev_ops { pub check: Option<unsafe extern "C" fn()> }

extern "C" {
    fn seq_puts(seq: *mut seq_file, text: *const c_char);
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...);
    fn iosf_mbi_read(unit: c_int, reg_type: c_int, reg: c_int, value: *mut c_uint) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut punit_device, fops: *const c_void) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn x86_match_cpu(ids: *const x86_cpu_id) -> *const x86_cpu_id;
    fn acpi_register_lps0_dev(ops: *mut acpi_s2idle_dev_ops);
    fn acpi_unregister_lps0_dev(ops: *mut acpi_s2idle_dev_ops);
}

static mut punit_dbg_file: *mut dentry = core::ptr::null_mut();

unsafe extern "C" fn punit_dev_state_show(seq_file: *mut seq_file, _unused: *mut c_void) -> c_int {
    let mut punit_pwr_status: c_uint = 0;
    let mut punit_devp = (*seq_file).private as *mut punit_device;
    seq_puts(seq_file, b"\n\nPUNIT NORTH COMPLEX DEVICES :\n\0".as_ptr() as *const c_char);
    while !(*punit_devp).name.is_null() {
        let status = iosf_mbi_read(BT_MBI_UNIT_PMC, MBI_REG_READ, (*punit_devp).reg, &mut punit_pwr_status);
        if status != 0 {
            seq_printf(seq_file, b"%9s : Read Failed\n\0".as_ptr() as *const c_char, (*punit_devp).name);
        } else {
            let index = ((punit_pwr_status >> (*punit_devp).sss_pos) & 3) as usize;
            seq_printf(seq_file, b"%9s : %s\n\0".as_ptr() as *const c_char, (*punit_devp).name, dstates[index].as_ptr());
        }
        punit_devp = punit_devp.add(1);
    }
    0
}

// DEFINE_SHOW_ATTRIBUTE(punit_dev_state)
extern "C" { static punit_dev_state_fops: c_void; }

unsafe fn punit_dbgfs_register(punit_device: *mut punit_device) {
    punit_dbg_file = debugfs_create_dir(b"punit_atom\0".as_ptr() as *const c_char, core::ptr::null_mut());
    debugfs_create_file(b"dev_power_state\0".as_ptr() as *const c_char, 0o444, punit_dbg_file, punit_device, &punit_dev_state_fops);
}

unsafe fn punit_dbgfs_unregister() { debugfs_remove_recursive(punit_dbg_file); }

static mut punit_dev: *const punit_device = core::ptr::null();
unsafe extern "C" fn punit_s2idle_check() {
    let mut p = punit_dev;
    while !(*p).name.is_null() {
        if (*p).reg == MIO_SS_PM { p = p.add(1); continue; }
        let mut power: c_uint = 0;
        if iosf_mbi_read(BT_MBI_UNIT_PMC, MBI_REG_READ, (*p).reg, &mut power) != 0 {
            pr_err(b"%s read failed\n\0".as_ptr() as *const c_char, (*p).name);
        } else {
            let dstate = (power >> (*p).sss_pos) & 3;
            if dstate == 0 { pr_err(b"%s is in D0 prior to s2idle\n\0".as_ptr() as *const c_char, (*p).name); }
        }
        p = p.add(1);
    }
}
static mut punit_s2idle_ops: acpi_s2idle_dev_ops = acpi_s2idle_dev_ops { check: Some(punit_s2idle_check) };
unsafe fn punit_s2idle_check_register(device: *mut punit_device) { punit_dev = device; acpi_register_lps0_dev(&mut punit_s2idle_ops); }
unsafe fn punit_s2idle_check_unregister() { acpi_unregister_lps0_dev(&mut punit_s2idle_ops); }

// CPU matching table and module initialization/exit hooks are supplied by the kernel build environment.
unsafe fn punit_atom_debug_init() -> c_int {
    // The X86_MATCH table selects one of punit_device_byt, punit_device_tng, or punit_device_cht.
    // Its exact CPU-ID representation is provided by asm/cpu_device_id.h.
    let id = x86_match_cpu(core::ptr::null());
    if id.is_null() { return -ENODEV; }
    let punit_device = (*id).driver_data as *mut punit_device;
    punit_dbgfs_register(punit_device);
    punit_s2idle_check_register(punit_device);
    0
}
unsafe fn punit_atom_debug_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
