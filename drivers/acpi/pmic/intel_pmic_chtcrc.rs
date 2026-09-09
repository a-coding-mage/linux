// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Cherry Trail Crystal Cove PMIC operation region driver
 *
 * Copyright (C) 2019 Hans de Goede <hdegoede@redhat.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

/*
 * We have no docs for the CHT Crystal Cove PMIC. The Asus Zenfone-2 kernel
 * code has 2 Crystal Cove regulator drivers, one calls the PMIC a "Crystal
 * Cove Plus" PMIC and talks about Cherry Trail, so presumably that one
 * could be used to get register info for the regulators if we need to
 * implement regulator support in the future.
 *
 * For now the sole purpose of this driver is to make
 * intel_soc_pmic_exec_mipi_pmic_seq_element work on devices with a
 * CHT Crystal Cove PMIC.
 */

#[repr(C)]
pub struct IntelPmicOpregionData {
    pub lpat_raw_to_temp: *const (),
    pub pmic_i2c_address: u32,
}

#[repr(C)]
pub struct IntelSocPmic {
    pub regmap: *mut Regmap,
}

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const u8,
}

extern "C" {
    pub static acpi_lpat_raw_to_temp: ();
    pub fn dev_get_drvdata(dev: *mut Device) -> *mut IntelSocPmic;
    pub fn acpi_handle(dev: *mut Device) -> *mut ();
    pub fn intel_pmic_install_opregion_handler(
        dev: *mut Device,
        handle: *mut (),
        regmap: *mut Regmap,
        data: *const IntelPmicOpregionData,
    ) -> i32;
}

/* static const struct intel_pmic_opregion_data intel_chtcrc_pmic_opregion_data = ... */
static INTEL_CHTCRC_PMIC_OPREGION_DATA: IntelPmicOpregionData = IntelPmicOpregionData {
    lpat_raw_to_temp: unsafe { &acpi_lpat_raw_to_temp as *const () },
    pmic_i2c_address: 0x6e,
};

unsafe extern "C" fn intel_chtcrc_pmic_opregion_probe(
    pdev: *mut PlatformDevice,
) -> i32 {
    let pmic: *mut IntelSocPmic = dev_get_drvdata(
        &mut (*(*pdev).dev) as *mut Device,
    );
    intel_pmic_install_opregion_handler(
        &mut (*pdev).dev as *mut Device,
        acpi_handle(&mut (*pdev).dev as *mut Device),
        (*pmic).regmap,
        &INTEL_CHTCRC_PMIC_OPREGION_DATA,
    )
}

static mut INTEL_CHTCRC_PMIC_OPREGION_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(intel_chtcrc_pmic_opregion_probe),
    driver: Driver {
        name: b"cht_crystal_cove_pmic\0".as_ptr(),
    },
};

// Equivalent of builtin_platform_driver(intel_chtcrc_pmic_opregion_driver).
#[allow(non_upper_case_globals)]
pub static intel_chtcrc_pmic_opregion_driver: *mut PlatformDriver = unsafe {
    &mut INTEL_CHTCRC_PMIC_OPREGION_DRIVER
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
