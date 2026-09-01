/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soc-intel-quirks.h - prototypes for quirk autodetection
 *
 * Copyright (c) 2019, Intel Corporation.
 *
 */

/* Dependencies from <linux/platform_data/x86/soc.h>. */
extern "C" {
    pub fn soc_intel_is_byt() -> bool;
}

/*
 * Original C condition:
 * #if IS_REACHABLE(CONFIG_IOSF_MBI)
 */

/* Dependencies from <linux/dmi.h>, <asm/iosf_mbi.h>, and platform headers. */
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dmi_strmatch {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
}

extern "C" {
    pub static DMI_SYS_VENDOR: core::ffi::c_int;
    pub static DMI_PRODUCT_FAMILY: core::ffi::c_int;
    pub static BT_MBI_UNIT_PMC: core::ffi::c_int;
    pub static MBI_REG_READ: core::ffi::c_int;
    pub static IORESOURCE_IRQ: core::ffi::c_uint;

    pub fn DMI_MATCH(
        slot: core::ffi::c_int,
        substr: *const core::ffi::c_char,
    ) -> dmi_strmatch;
    pub fn dmi_check_system(list: *const dmi_system_id) -> bool;
    pub fn iosf_mbi_available() -> bool;
    pub fn iosf_mbi_read(
        port: core::ffi::c_int,
        opcode: core::ffi::c_int,
        offset: core::ffi::c_uint,
        mdr: *mut u32,
    ) -> core::ffi::c_int;
    pub fn platform_get_resource(
        dev: *mut platform_device,
        type_: core::ffi::c_uint,
        num: core::ffi::c_uint,
    ) -> *mut resource;

    pub fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    pub fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

pub unsafe fn soc_intel_is_byt_cr(pdev: *mut platform_device) -> bool {
    /*
     * List of systems which:
     * 1. Use a non CR version of the Bay Trail SoC
     * 2. Contain at least 6 interrupt resources so that the
     *    platform_get_resource(pdev, IORESOURCE_IRQ, 5) check below
     *    succeeds
     * 3. Despite 1. and 2. still have their IPC IRQ at index 0 rather then 5
     *
     * This needs to be here so that it can be shared between the SST and
     * SOF drivers. We rely on the compiler to optimize this out in files
     * where soc_intel_is_byt_cr is not used.
     */
    let force_bytcr_table: [dmi_system_id; 2] = [
        dmi_system_id {
            /* Lenovo Yoga Tablet 2 series */
            matches: [
                DMI_MATCH(DMI_SYS_VENDOR, b"LENOVO\0".as_ptr() as *const core::ffi::c_char),
                DMI_MATCH(
                    DMI_PRODUCT_FAMILY,
                    b"YOGATablet2\0".as_ptr() as *const core::ffi::c_char,
                ),
                core::mem::zeroed(),
                core::mem::zeroed(),
            ],
        },
        core::mem::zeroed(),
    ];
    let dev: *mut device = &mut (*pdev).dev;
    let mut status: core::ffi::c_int = 0;

    if !soc_intel_is_byt() {
        return false;
    }

    if dmi_check_system(force_bytcr_table.as_ptr()) {
        return true;
    }

    if iosf_mbi_available() {
        let mut bios_status: u32;

        status = iosf_mbi_read(
            BT_MBI_UNIT_PMC, /* 0x04 PUNIT */
            MBI_REG_READ,   /* 0x10 */
            0x006,          /* BIOS_CONFIG */
            &mut bios_status,
        );

        if status != 0 {
            dev_err(dev, b"could not read PUNIT BIOS_CONFIG\n\0".as_ptr() as *const core::ffi::c_char);
        } else {
            /* bits 26:27 mirror PMIC options */
            bios_status = (bios_status >> 26) & 3;

            if bios_status == 1 || bios_status == 3 {
                dev_info(dev, b"Detected Baytrail-CR platform\n\0".as_ptr() as *const core::ffi::c_char);
                return true;
            }

            dev_info(dev, b"BYT-CR not detected\n\0".as_ptr() as *const core::ffi::c_char);
        }
    } else {
        dev_info(
            dev,
            b"IOSF_MBI not available, no BYT-CR detection\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    if platform_get_resource(pdev, IORESOURCE_IRQ, 5).is_null() {
        /*
         * Some devices detected as BYT-T have only a single IRQ listed,
         * causing platform_get_irq with index 5 to return -ENXIO.
         * The correct IRQ in this case is at index 0, as on BYT-CR.
         */
        dev_info(dev, b"Falling back to Baytrail-CR platform\n\0".as_ptr() as *const core::ffi::c_char);
        return true;
    }

    false
}

/*
 * Original C #else branch when !IS_REACHABLE(CONFIG_IOSF_MBI):
 *
 * static inline bool soc_intel_is_byt_cr(struct platform_device *pdev)
 * {
 *     return false;
 * }
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
