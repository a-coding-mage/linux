// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

/* ACP machine configuration module */

// Rust translation of dependencies from:
// <linux/acpi.h>, <linux/bits.h>, <linux/dmi.h>, <linux/module.h>,
// <linux/pci.h>, "../sof/amd/acp.h", and "mach-config.h".

use core::ffi::{c_char, c_int, c_void};

const ACP_7_0_REV: u8 = 0x70;

const ACPI_TYPE_INTEGER: c_int = 1;

const DMI_BOARD_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;
const DMI_BOARD_NAME: c_int = 2;
const DMI_SYS_VENDOR: c_int = 3;
const DMI_PRODUCT_VERSION: c_int = 4;

extern "C" {
    static ACP_PCI_DEV_ID: u16;
    static FLAG_AMD_SOF: c_int;
    static FLAG_AMD_LEGACY: c_int;
    static FLAG_AMD_LEGACY_ONLY_DMIC: c_int;

    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn acpi_dev_get_property(
        adev: *mut acpi_device,
        name: *const c_char,
        type_: c_int,
        obj: *mut *const acpi_object,
    ) -> c_int;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn snd_soc_acpi_codec_list(mach: *mut snd_soc_acpi_mach) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub device: u16,
    pub revision: u8,
}

#[repr(C)]
pub union acpi_object_data {
    pub integer: acpi_object_integer,
}

#[repr(C)]
pub struct acpi_object {
    pub type_: c_int,
    pub integer: acpi_object_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_integer {
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
    pub exact_match: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
}

#[repr(C)]
pub struct config_entry {
    pub flags: c_int,
    pub device: u16,
    pub dmi_table: *const dmi_system_id,
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: c_int,
    pub codecs: [*const c_char; 1],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub pdata: *mut c_void,
    pub machine_quirk: Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach) -> c_int>,
    pub quirk_data: *mut c_void,
    pub fw_filename: *const c_char,
    pub sof_tplg_filename: *const c_char,
}

const fn dmi_match(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch {
        slot,
        substr,
        exact_match: false,
    }
}

const fn dmi_exact_match(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch {
        slot,
        substr,
        exact_match: true,
    }
}

const DMI_MATCH_ZERO: dmi_strmatch = dmi_strmatch {
    slot: 0,
    substr: core::ptr::null(),
    exact_match: false,
};

const DMI_SYSTEM_ID_ZERO: dmi_system_id = dmi_system_id {
    matches: [DMI_MATCH_ZERO; 4],
};

static mut acp_quirk_data: c_int = 0;

static acp70_acpi_flag_override_table: [dmi_system_id; 9] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"ASUSTeK COMPUTER INC.".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"HN7306EA".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        /* ASUS Zenbook S16 UM5606GA (Strix Point, ACP 7.0) */
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"ASUSTeK COMPUTER INC.".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"Zenbook S16 UM5606GA".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        /* Lenovo Yoga Pro 7 15ASH11 (Strix Halo, ACP 7.0) */
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"LENOVO".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"83W5".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        /* Lenovo Legion 7 15ASH11 (Strix Halo, ACP 7.0) */
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"LENOVO".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"83V9".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"ASUSTeK COMPUTER INC".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"FA401EA".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"ASUSTeK COMPUTER INC".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"Vivobook 18 M1807GA".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        /* HP OmniBook X Flip 14-kc0xxx (Strix Point, ACP 7.2) */
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"HP".as_ptr()),
            dmi_match(DMI_BOARD_NAME, c"8EA1".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    dmi_system_id {
        /* HP OmniBook X Flip 16-cc0xxx */
        matches: [
            dmi_match(DMI_BOARD_VENDOR, c"HP".as_ptr()),
            dmi_match(DMI_BOARD_NAME, c"8EA2".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_0: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_SYS_VENDOR, c"AMD".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"Majolica-CZN".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_1: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_SYS_VENDOR, c"Google".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_2: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_SYS_VENDOR, c"Valve".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"Jupiter".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_3: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_SYS_VENDOR, c"Valve".as_ptr()),
            dmi_match(DMI_PRODUCT_NAME, c"Galileo".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_4: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"KLVL-WXXW".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_5: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"KLVL-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_6: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"BOM-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_7: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_8: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1020".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_9: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1040".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static CONFIG_DMI_10: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_exact_match(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            dmi_exact_match(DMI_PRODUCT_VERSION, c"M1060".as_ptr()),
            DMI_MATCH_ZERO,
        ],
    },
    DMI_SYSTEM_ID_ZERO,
];

static config_table: [config_entry; 11] = unsafe {
    [
        config_entry {
            flags: FLAG_AMD_SOF,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_0.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_SOF,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_1.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_2.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_SOF,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_3.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_4.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_5.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_6.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_7.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_8.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_9.as_ptr(),
        },
        config_entry {
            flags: FLAG_AMD_LEGACY,
            device: ACP_PCI_DEV_ID,
            dmi_table: CONFIG_DMI_10.as_ptr(),
        },
    ]
};

unsafe fn snd_amd_acp_acpi_find_config(pci: *mut pci_dev) -> c_int {
    let mut obj: *const acpi_object = core::ptr::null();
    let mut acp_flag: c_int = FLAG_AMD_LEGACY_ONLY_DMIC;

    if acpi_dev_get_property(
        ACPI_COMPANION(core::ptr::addr_of_mut!((*pci).dev)),
        c"acp-audio-config-flag".as_ptr(),
        ACPI_TYPE_INTEGER,
        core::ptr::addr_of_mut!(obj),
    ) == 0
    {
        acp_flag = (*obj).integer.value as c_int;
    }

    acp_flag
}

#[no_mangle]
pub unsafe extern "C" fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int {
    let mut table: *const config_entry = config_table.as_ptr();
    let device: u16 = (*pci).device;
    let mut i: usize = 0;

    /* Do not enable FLAGS on older platforms with Rev Id zero
     * For platforms which has ACP 7.0 or higher, read the acp
     * config flag from BIOS ACPI table and for older platforms
     * read it from DMI tables.
     */
    if (*pci).revision == 0 {
        return 0;
    } else if (*pci).revision >= ACP_7_0_REV {
        if dmi_check_system(acp70_acpi_flag_override_table.as_ptr()) != 0 {
            return 0;
        }
        return snd_amd_acp_acpi_find_config(pci);
    }

    while i < config_table.len() {
        if (*table).device != device {
            i += 1;
            table = table.add(1);
            continue;
        }
        if !(*table).dmi_table.is_null() && dmi_check_system((*table).dmi_table) == 0 {
            i += 1;
            table = table.add(1);
            continue;
        }
        acp_quirk_data = (*table).flags;
        return (*table).flags;
    }

    0
}

// EXPORT_SYMBOL(snd_amd_acp_find_config);

static mut amp_rt1019: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"10EC1019".as_ptr()],
};

static mut amp_max: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"MX98360A".as_ptr()],
};

static mut amp_max98388: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"ADS8388".as_ptr()],
};

#[no_mangle]
pub static mut snd_soc_acpi_amd_sof_machines: [snd_soc_acpi_mach; 6] = [
    snd_soc_acpi_mach {
        id: c"10EC5682".as_ptr(),
        drv_name: c"rt5682-rt1019".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_rt1019) as *mut c_void,
        fw_filename: c"sof-rn.ri".as_ptr(),
        sof_tplg_filename: c"sof-rn-rt5682-rt1019.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"10EC5682".as_ptr(),
        drv_name: c"rt5682-max".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_max) as *mut c_void,
        fw_filename: c"sof-rn.ri".as_ptr(),
        sof_tplg_filename: c"sof-rn-rt5682-max98360.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"RTL5682".as_ptr(),
        drv_name: c"rt5682s-max".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_max) as *mut c_void,
        fw_filename: c"sof-rn.ri".as_ptr(),
        sof_tplg_filename: c"sof-rn-rt5682-max98360.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"RTL5682".as_ptr(),
        drv_name: c"rt5682s-rt1019".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_rt1019) as *mut c_void,
        fw_filename: c"sof-rn.ri".as_ptr(),
        sof_tplg_filename: c"sof-rn-rt5682-rt1019.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"AMDI1019".as_ptr(),
        drv_name: c"renoir-dsp".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: c"sof-rn.ri".as_ptr(),
        sof_tplg_filename: c"sof-acp.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_sof_machines);

#[no_mangle]
pub static mut snd_soc_acpi_amd_vangogh_sof_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: c"NVTN2020".as_ptr(),
        drv_name: c"nau8821-max".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_max98388) as *mut c_void,
        fw_filename: c"sof-vangogh.ri".as_ptr(),
        sof_tplg_filename: c"sof-vangogh-nau8821-max.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_vangogh_sof_machines);

#[no_mangle]
pub static mut snd_soc_acpi_amd_rmb_sof_machines: [snd_soc_acpi_mach; 4] = [
    snd_soc_acpi_mach {
        id: c"AMDI1019".as_ptr(),
        drv_name: c"rmb-dsp".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: c"sof-rmb.ri".as_ptr(),
        sof_tplg_filename: c"sof-acp-rmb.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"10508825".as_ptr(),
        drv_name: c"nau8825-max".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_max) as *mut c_void,
        fw_filename: c"sof-rmb.ri".as_ptr(),
        sof_tplg_filename: c"sof-rmb-nau8825-max98360.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: c"RTL5682".as_ptr(),
        drv_name: c"rt5682s-hs-rt1019".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: core::ptr::addr_of_mut!(amp_rt1019) as *mut c_void,
        fw_filename: c"sof-rmb.ri".as_ptr(),
        sof_tplg_filename: c"sof-rmb-rt5682s-rt1019.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_rmb_sof_machines);

#[no_mangle]
pub static mut snd_soc_acpi_amd_acp63_sof_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: c"AMDI1019".as_ptr(),
        drv_name: c"acp63-dsp".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: c"sof-acp_6_3.ri".as_ptr(),
        sof_tplg_filename: c"sof-acp_6_3.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_acp63_sof_machines);

#[no_mangle]
pub static mut snd_soc_acpi_amd_acp70_sof_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: c"AMDI1010".as_ptr(),
        drv_name: c"acp70-dsp".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: c"sof-acp_7_0.ri".as_ptr(),
        sof_tplg_filename: c"sof-acp_7_0.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_acp70_sof_machines);

#[no_mangle]
pub static mut snd_soc_acpi_amd_acp7x_sof_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: c"AMDI1010".as_ptr(),
        drv_name: c"acp7x-dsp".as_ptr(),
        pdata: core::ptr::addr_of_mut!(acp_quirk_data) as *mut c_void,
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: c"sof-acp7x.ri".as_ptr(),
        sof_tplg_filename: c"sof-acp7x.tplg".as_ptr(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        pdata: core::ptr::null_mut(),
        machine_quirk: None,
        quirk_data: core::ptr::null_mut(),
        fw_filename: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

// EXPORT_SYMBOL(snd_soc_acpi_amd_acp7x_sof_machines);

// MODULE_DESCRIPTION("AMD ACP Machine Configuration Module");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
