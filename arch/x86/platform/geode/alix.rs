// SPDX-License-Identifier: GPL-2.0-only
/*
 * System Specific setup for PCEngines ALIX.
 * At the moment this means setup of GPIO control of LEDs
 * on Alix.2/3/6 boards.
 *
 * Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
 * Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
 *                and Philip Prindeville <philipp@redfish-solutions.com>
 */

const BIOS_SIGNATURE_TINYBIOS: usize = 0xf0000;
const BIOS_SIGNATURE_COREBOOT: usize = 0x500;
const BIOS_REGION_SIZE: usize = 0x10000;

/* This driver is not modular; this preserves the module parameter interface. */
static mut force: bool = false;

#[repr(C)]
struct geode_led {
    gpio: i32,
    active_low: bool,
}

static ALIX_LEDS: [geode_led; 3] = [
    geode_led { gpio: 6, active_low: true },
    geode_led { gpio: 25, active_low: false },
    geode_led { gpio: 27, active_low: false },
];

extern "C" {
    fn geode_create_restart_key(gpio: i32);
    fn geode_create_leds(name: *const u8, leds: *const geode_led, count: usize);
    fn phys_to_virt(address: usize) -> *const u8;
    fn dmi_get_system_info(field: i32) -> *const u8;
    fn is_geode() -> bool;
    fn printk(format: *const u8, ...);
}

const DMI_SYS_VENDOR: i32 = 1;
const DMI_PRODUCT_NAME: i32 = 2;

unsafe fn register_alix() {
    geode_create_restart_key(24);
    geode_create_leds(b"alix\0".as_ptr(), ALIX_LEDS.as_ptr(), ALIX_LEDS.len());
}

unsafe fn alix_present(
    bios_phys: usize,
    alix_sig: *const u8,
    alix_sig_len: usize,
) -> bool {
    let bios_len = BIOS_REGION_SIZE;
    let bios_virt = phys_to_virt(bios_phys);
    let scan_end = bios_virt.add(bios_len - (alix_sig_len + 2));
    let mut p = bios_virt;
    let mut name = [0u8; 64];

    if force {
        printk(b"%s: forced to skip BIOS test, assume system is ALIX.2/ALIX.3\n\0".as_ptr());
        return true;
    }

    while p < scan_end {
        if core::slice::from_raw_parts(p, alix_sig_len)
            != core::slice::from_raw_parts(alix_sig, alix_sig_len)
        {
            p = p.add(1);
            continue;
        }

        core::ptr::copy_nonoverlapping(p, name.as_mut_ptr(), name.len());

        /* remove the first \0 character from string */
        if let Some(index) = name.iter().position(|&c| c == 0) {
            name[index] = b' ';
        }

        /* cut the string at a newline */
        if let Some(index) = name.iter().position(|&c| c == b'\r') {
            name[index] = 0;
        }

        let tail = p.add(alix_sig_len);
        if tail.read() == b'2' || tail.read() == b'3' || tail.read() == b'6' {
            printk(b"%s: system is recognized as \"%s\"\n\0".as_ptr());
            return true;
        }
        p = p.add(1);
    }

    false
}

unsafe fn alix_present_dmi() -> bool {
    let vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    if vendor.is_null() || libc_strcmp(vendor, b"PC Engines\0".as_ptr()) != 0 {
        return false;
    }

    let product = dmi_get_system_info(DMI_PRODUCT_NAME);
    if product.is_null()
        || (libc_strcmp(product, b"ALIX.2D\0".as_ptr()) != 0
            && libc_strcmp(product, b"ALIX.6\0".as_ptr()) != 0)
    {
        return false;
    }

    printk(b"%s: system is recognized as \"%s %s\"\n\0".as_ptr());
    true
}

extern "C" {
    fn libc_strcmp(left: *const u8, right: *const u8) -> i32;
}

unsafe fn alix_init() -> i32 {
    let tinybios_sig = b"PC Engines ALIX.\0";
    let coreboot_sig = b"PC Engines\0ALIX.\0";

    if !is_geode() {
        return 0;
    }

    if alix_present(BIOS_SIGNATURE_TINYBIOS, tinybios_sig.as_ptr(), tinybios_sig.len() - 1)
        || alix_present(BIOS_SIGNATURE_COREBOOT, coreboot_sig.as_ptr(), coreboot_sig.len() - 1)
        || alix_present_dmi()
    {
        register_alix();
    }

    0
}

// module_param(force, bool, 0444);
// MODULE_PARM_DESC(force, "Force detection as ALIX.2/ALIX.3 platform");
// device_initcall(alix_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
