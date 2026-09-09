// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  osi.c - _OSI implementation
 *
 *  Copyright (C) 2016 Intel Corporation
 *    Author: Lv Zheng <lv.zheng@acpi>
 */

// Uncomment next line to get verbose printout
// #define DEBUG
// #define pr_fmt(fmt) "ACPI: " fmt

// Dependencies supplied by the surrounding kernel translation.

const OSI_STRING_LENGTH_MAX: usize = 64;
const OSI_STRING_ENTRIES_MAX: usize = 16;

#[repr(C)]
struct AcpiOsiEntry {
    string: [u8; OSI_STRING_LENGTH_MAX],
    enable: bool,
}

#[repr(C)]
struct AcpiOsiConfig {
    default_disabling: u8,
    linux_enable: u32,
    linux_dmi: u32,
    linux_cmdline: u32,
    darwin_enable: u32,
    darwin_dmi: u32,
    darwin_cmdline: u32,
}

static mut osi_config: AcpiOsiConfig = AcpiOsiConfig {
    default_disabling: 0,
    linux_enable: 0,
    linux_dmi: 0,
    linux_cmdline: 0,
    darwin_enable: 0,
    darwin_dmi: 0,
    darwin_cmdline: 0,
};

static mut osi_setup_entries: [AcpiOsiEntry; OSI_STRING_ENTRIES_MAX] = [
    AcpiOsiEntry { string: *b"Module Device\0", enable: true },
    AcpiOsiEntry { string: *b"Processor Device\0", enable: true },
    AcpiOsiEntry { string: *b"Processor Aggregator Device\0", enable: true },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
    AcpiOsiEntry { string: [0; OSI_STRING_LENGTH_MAX], enable: false },
];

unsafe fn acpi_osi_handler(interface: *const u8, supported: u32) -> u32 {
    if c_str_eq(b"Linux\0".as_ptr(), interface) {
        // pr_notice_once(FW_BUG "BIOS _OSI(Linux) query %s%s\n", ...);
    }
    if c_str_eq(b"Darwin\0".as_ptr(), interface) {
        // pr_notice_once("BIOS _OSI(Darwin) query %s%s\n", ...);
    }
    supported
}

unsafe fn acpi_osi_setup(str_: *mut u8) {
    if !acpi_gbl_create_osi_method {
        return;
    }
    if str_.is_null() || *str_ == 0 {
        // pr_info("_OSI method disabled\n");
        acpi_gbl_create_osi_method = false;
        return;
    }
    let mut enable = true;
    let mut current = str_;
    if *current == b'!' {
        current = current.add(1);
        if *current == 0 {
            if osi_config.default_disabling == 0 {
                osi_config.default_disabling = ACPI_DISABLE_ALL_VENDOR_STRINGS;
            }
            return;
        } else if *current == b'*' {
            osi_config.default_disabling = ACPI_DISABLE_ALL_STRINGS;
            for entry in osi_setup_entries.iter_mut() {
                entry.enable = false;
            }
            return;
        } else if *current == b'!' {
            osi_config.default_disabling = 0;
            return;
        }
        enable = false;
    }
    for entry in osi_setup_entries.iter_mut() {
        if c_str_eq(entry.string.as_ptr(), current) {
            entry.enable = enable;
            break;
        } else if entry.string[0] == 0 {
            entry.enable = enable;
            strscpy(entry.string.as_mut_ptr(), current, OSI_STRING_LENGTH_MAX);
            break;
        }
    }
}

unsafe fn __acpi_osi_setup_darwin(enable: bool) {
    osi_config.darwin_enable = enable as u32;
    if enable {
        acpi_osi_setup(b"!\0".as_ptr() as *mut u8);
        acpi_osi_setup(b"Darwin\0".as_ptr() as *mut u8);
    } else {
        acpi_osi_setup(b"!!\0".as_ptr() as *mut u8);
        acpi_osi_setup(b"!Darwin\0".as_ptr() as *mut u8);
    }
}

unsafe fn acpi_osi_setup_darwin(enable: bool) {
    osi_config.darwin_dmi = 0;
    osi_config.darwin_cmdline = 1;
    __acpi_osi_setup_darwin(enable);
}

unsafe fn __acpi_osi_setup_linux(enable: bool) {
    osi_config.linux_enable = enable as u32;
    if enable {
        acpi_osi_setup(b"Linux\0".as_ptr() as *mut u8);
    } else {
        acpi_osi_setup(b"!Linux\0".as_ptr() as *mut u8);
    }
}

unsafe fn acpi_osi_setup_linux(enable: bool) {
    osi_config.linux_dmi = 0;
    osi_config.linux_cmdline = 1;
    __acpi_osi_setup_linux(enable);
}

unsafe fn acpi_osi_setup_late() {
    if osi_config.default_disabling != 0 {
        let status = acpi_update_interfaces(osi_config.default_disabling);
        if acpi_success(status) {
            // pr_info("Disabled all _OSI OS vendors%s\n", ...);
        }
    }
    for entry in osi_setup_entries.iter() {
        if entry.string[0] == 0 {
            break;
        }
        if entry.enable {
            let status = acpi_install_interface(entry.string.as_ptr());
            if acpi_success(status) {
                // pr_info("Added _OSI(%s)\n", entry.string.as_ptr());
            }
        } else {
            let status = acpi_remove_interface(entry.string.as_ptr());
            if acpi_success(status) {
                // pr_info("Deleted _OSI(%s)\n", entry.string.as_ptr());
            }
        }
    }
}

unsafe fn osi_setup(str_: *mut u8) -> i32 {
    if !str_.is_null() && c_str_eq(b"Linux\0".as_ptr(), str_) {
        acpi_osi_setup_linux(true);
    } else if !str_.is_null() && c_str_eq(b"!Linux\0".as_ptr(), str_) {
        acpi_osi_setup_linux(false);
    } else if !str_.is_null() && c_str_eq(b"Darwin\0".as_ptr(), str_) {
        acpi_osi_setup_darwin(true);
    } else if !str_.is_null() && c_str_eq(b"!Darwin\0".as_ptr(), str_) {
        acpi_osi_setup_darwin(false);
    } else {
        acpi_osi_setup(str_);
    }
    1
}

// __setup("acpi_osi=", osi_setup);

// External symbols supplied by the surrounding kernel translation.
extern "C" {
    static mut acpi_gbl_create_osi_method: bool;
    fn c_str_eq(a: *const u8, b: *const u8) -> bool;
    fn strscpy(dst: *mut u8, src: *const u8, size: usize) -> isize;
    fn acpi_update_interfaces(flags: u8) -> i32;
    fn acpi_install_interface(name: *const u8) -> i32;
    fn acpi_remove_interface(name: *const u8) -> i32;
    fn acpi_success(status: i32) -> bool;
}

const ACPI_DISABLE_ALL_VENDOR_STRINGS: u8 = 1;
const ACPI_DISABLE_ALL_STRINGS: u8 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
