// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2004 James Cleverdon, IBM.
 *
 * Generic APIC sub-arch probe layer.
 *
 * Hacked for x86-64 by James Cleverdon from i386 architecture code by
 * Martin Bligh, Andi Kleen, James Bottomley, John Stultz, and
 * James Cleverdon.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/thread_info.h, asm/apic.h, and local.h.

#[repr(C)]
pub struct apic {
    pub probe: Option<unsafe extern "C" fn() -> bool>,
    pub acpi_madt_oem_check:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_char, *mut ::core::ffi::c_char) -> bool>,
}

unsafe extern "C" {
    static mut __apicdrivers: *mut *mut apic;
    static mut __apicdrivers_end: *mut *mut apic;

    fn enable_IR_x2apic();
    fn apic_install_driver(drv: *mut apic);
}

/* Select the appropriate APIC driver */
pub unsafe extern "C" fn x86_64_probe_apic() {
    let mut drv: *mut *mut apic;

    unsafe {
        enable_IR_x2apic();

        drv = __apicdrivers;
        while drv < __apicdrivers_end {
            let driver = *drv;
            if (*driver).probe.is_some() && ((*driver).probe.unwrap())() {
                apic_install_driver(driver);
                break;
            }
            drv = drv.add(1);
        }
    }
}

pub unsafe extern "C" fn default_acpi_madt_oem_check(
    oem_id: *mut ::core::ffi::c_char,
    oem_table_id: *mut ::core::ffi::c_char,
) -> i32 {
    let mut drv: *mut *mut apic;

    unsafe {
        drv = __apicdrivers;
        while drv < __apicdrivers_end {
            let driver = *drv;
            if ((*driver).acpi_madt_oem_check.unwrap())(oem_id, oem_table_id) {
                apic_install_driver(driver);
                return 1;
            }
            drv = drv.add(1);
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
