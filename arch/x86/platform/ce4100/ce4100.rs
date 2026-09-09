// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel CE4100  platform specific setup code
 *
 * (C) Copyright 2010 Intel Corporation
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct X86InitOem {
    pub arch_setup: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct X86InitResources {
    pub probe_roms: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct X86InitMpparse {
    pub find_mptable: Option<unsafe extern "C" fn()>,
    pub early_parse_smp_cfg: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct X86InitPci {
    pub init: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct X86Init {
    pub oem: X86InitOem,
    pub resources: X86InitResources,
    pub mpparse: X86InitMpparse,
    pub pci: X86InitPci,
}

unsafe extern "C" {
    fn outb(value: u8, port: u16);
    fn sdv_serial_fixup();
    fn x86_of_pci_init();
    fn ce4100_pci_init();
    fn x86_init_noop();

    static mut x86_init: X86Init;
    static mut reboot_type: i32;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static BOOT_KBD: i32;
}

/*
 * The CE4100 platform has an internal 8051 Microcontroller which is
 * responsible for signaling to the external Power Management Unit the
 * intention to reset, reboot or power off the system. This 8051 device has
 * its command register mapped at I/O port 0xcf9 and the value 0x4 is used
 * to power off the system.
 */
unsafe extern "C" fn ce4100_power_off() {
    unsafe {
        outb(0x4, 0xcf9);
    }
}

unsafe extern "C" fn sdv_arch_setup() {
    unsafe {
        sdv_serial_fixup();
    }
}

unsafe extern "C" fn sdv_pci_init() {
    unsafe {
        x86_of_pci_init();
    }
}

/*
 * CE4100 specific x86_init function overrides and early setup
 * calls.
 */
pub unsafe extern "C" fn x86_ce4100_early_setup() {
    unsafe {
        x86_init.oem.arch_setup = Some(sdv_arch_setup);
        x86_init.resources.probe_roms = Some(x86_init_noop);
        x86_init.mpparse.find_mptable = Some(x86_init_noop);
        x86_init.mpparse.early_parse_smp_cfg = Some(x86_init_noop);
        x86_init.pci.init = Some(ce4100_pci_init);
        x86_init.pci.init_irq = Some(sdv_pci_init);

        /*
         * By default, the reboot method is ACPI which is supported by the
         * CE4100 bootloader CEFDK using FADT.ResetReg Address and ResetValue
         * the bootloader will however issue a system power off instead of
         * reboot. By using BOOT_KBD we ensure proper system reboot as
         * expected.
         */
        reboot_type = BOOT_KBD;

        pm_power_off = Some(ce4100_power_off);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
