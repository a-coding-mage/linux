// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-byt-match.c - tables and support for BYT ACPI enumeration.
 *
 * Copyright (c) 2017, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type DmiCallback = unsafe extern "C" fn(*const dmi_system_id) -> c_int;

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
    pub exact_match: bool,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<DmiCallback>,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: c_int,
    pub codecs: [*const c_char; 4],
}

type MachineQuirk = unsafe extern "C" fn(*mut c_void) -> *mut snd_soc_acpi_mach;

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub uid: *const c_char,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub link_mask: c_int,
    pub links: *const c_void,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub board: *const c_char,
    pub machine_quirk: Option<MachineQuirk>,
    pub pdata: *mut c_void,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: c_ulong,
}

unsafe extern "C" {
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
}

const DMI_SYS_VENDOR: c_int = 1;
const DMI_PRODUCT_NAME: c_int = 2;
const DMI_PRODUCT_VERSION: c_int = 6;
const DMI_BOARD_VENDOR: c_int = 7;
const DMI_BOARD_NAME: c_int = 8;

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

const fn dmi_empty_match() -> dmi_strmatch {
    dmi_strmatch {
        slot: 0,
        substr: core::ptr::null(),
        exact_match: false,
    }
}

const fn mach_empty() -> snd_soc_acpi_mach {
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: core::ptr::null(),
        fw_filename: core::ptr::null(),
        board: core::ptr::null(),
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: core::ptr::null(),
        tplg_quirk_mask: 0,
    }
}

static mut byt_machine_id: c_ulong = 0;

const BYT_RT5672: c_ulong = 1;
const BYT_POV_P1006W: c_ulong = 2;

unsafe extern "C" fn byt_rt5672_quirk_cb(_id: *const dmi_system_id) -> c_int {
    unsafe {
        byt_machine_id = BYT_RT5672;
    }
    1
}

unsafe extern "C" fn byt_pov_p1006w_quirk_cb(_id: *const dmi_system_id) -> c_int {
    unsafe {
        byt_machine_id = BYT_POV_P1006W;
    }
    1
}

static mut byt_table: [dmi_system_id; 8] = [
    dmi_system_id {
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"LENOVO\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_VERSION,
                b"ThinkPad 8\0".as_ptr() as *const c_char,
            ),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"LENOVO\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_VERSION,
                b"ThinkPad 10\0".as_ptr() as *const c_char,
            ),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"LENOVO\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_VERSION,
                b"ThinkPad Tablet B\0".as_ptr() as *const c_char,
            ),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"LENOVO\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_VERSION,
                b"Lenovo Miix 2 10\0".as_ptr() as *const c_char,
            ),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        /* Point of View mobii wintab p1006w (v1.0) */
        callback: Some(byt_pov_p1006w_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_exact_match(DMI_SYS_VENDOR, b"Insyde\0".as_ptr() as *const c_char),
            dmi_exact_match(DMI_PRODUCT_NAME, b"BayTrail\0".as_ptr() as *const c_char),
            /* Note 105b is Foxcon's USB/PCI vendor id */
            dmi_exact_match(DMI_BOARD_VENDOR, b"105B\0".as_ptr() as *const c_char),
            dmi_exact_match(DMI_BOARD_NAME, b"0E57\0".as_ptr() as *const c_char),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        /* Aegex 10 tablet (RU2) */
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"AEGEX\0".as_ptr() as *const c_char),
            dmi_match(DMI_PRODUCT_VERSION, b"RU2\0".as_ptr() as *const c_char),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        /* Dell Venue 10 Pro 5055 */
        callback: Some(byt_rt5672_quirk_cb),
        ident: core::ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"Dell Inc.\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_NAME,
                b"Venue 10 Pro 5055\0".as_ptr() as *const c_char,
            ),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
    dmi_system_id {
        callback: None,
        ident: core::ptr::null(),
        matches: [
            dmi_empty_match(),
            dmi_empty_match(),
            dmi_empty_match(),
            dmi_empty_match(),
        ],
        driver_data: core::ptr::null_mut(),
    },
];

/* Various devices use an ACPI id of 10EC5640 while using a rt5672 codec */
static mut byt_rt5672: snd_soc_acpi_mach = snd_soc_acpi_mach {
    id: b"10EC5640\0".as_ptr() as *const c_char,
    uid: core::ptr::null(),
    comp_ids: core::ptr::null(),
    link_mask: 0,
    links: core::ptr::null(),
    drv_name: b"cht-bsw-rt5672\0".as_ptr() as *const c_char,
    fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
    board: b"cht-bsw\0".as_ptr() as *const c_char,
    machine_quirk: None,
    pdata: core::ptr::null_mut(),
    sof_tplg_filename: b"sof-byt-rt5670.tplg\0".as_ptr() as *const c_char,
    tplg_quirk_mask: 0,
};

static mut byt_pov_p1006w: snd_soc_acpi_mach = snd_soc_acpi_mach {
    id: b"10EC5640\0".as_ptr() as *const c_char,
    uid: core::ptr::null(),
    comp_ids: core::ptr::null(),
    link_mask: 0,
    links: core::ptr::null(),
    drv_name: b"bytcr_rt5651\0".as_ptr() as *const c_char,
    fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
    board: b"bytcr_rt5651\0".as_ptr() as *const c_char,
    machine_quirk: None,
    pdata: core::ptr::null_mut(),
    sof_tplg_filename: b"sof-byt-rt5651.tplg\0".as_ptr() as *const c_char,
    tplg_quirk_mask: 0,
};

unsafe extern "C" fn byt_quirk(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
    let mach = arg as *mut snd_soc_acpi_mach;

    unsafe {
        dmi_check_system(core::ptr::addr_of!(byt_table) as *const dmi_system_id);

        match byt_machine_id {
            BYT_RT5672 => core::ptr::addr_of_mut!(byt_rt5672),
            BYT_POV_P1006W => core::ptr::addr_of_mut!(byt_pov_p1006w),
            _ => mach,
        }
    }
}

static mut rt5640_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [
        b"10EC5640\0".as_ptr() as *const c_char,
        b"10EC5642\0".as_ptr() as *const c_char,
        b"INTCCFFD\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ],
};

static mut wm5102_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [
        b"10WM5102\0".as_ptr() as *const c_char,
        b"WM510204\0".as_ptr() as *const c_char,
        b"WM510205\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ],
};

static mut da7213_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [
        b"DGLS7212\0".as_ptr() as *const c_char,
        b"DGLS7213\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

static mut rt5645_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [
        b"10EC5645\0".as_ptr() as *const c_char,
        b"10EC5648\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_baytrail_machines: [snd_soc_acpi_mach; 10] = [
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        uid: core::ptr::null(),
        comp_ids: core::ptr::addr_of!(rt5640_comp_ids),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcr_rt5640\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcr_rt5640\0".as_ptr() as *const c_char,
        machine_quirk: Some(byt_quirk),
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-rt5640.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"10EC5651\0".as_ptr() as *const c_char,
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcr_rt5651\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcr_rt5651\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-rt5651.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        uid: core::ptr::null(),
        comp_ids: core::ptr::addr_of!(wm5102_comp_ids),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcr_wm5102\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcr_wm5102\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-wm5102.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        uid: core::ptr::null(),
        comp_ids: core::ptr::addr_of!(da7213_comp_ids),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcht_da7213\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcht_da7213\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-da7213.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"ESSX8316\0".as_ptr() as *const c_char,
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcht_es8316\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcht_es8316\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-es8316.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"10EC5682\0".as_ptr() as *const c_char,
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"sof_rt5682\0".as_ptr() as *const c_char,
        fw_filename: core::ptr::null(),
        board: core::ptr::null(),
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-rt5682.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    /* some Baytrail platforms rely on RT5645, use CHT machine driver */
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        uid: core::ptr::null(),
        comp_ids: core::ptr::addr_of!(rt5645_comp_ids),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"cht-bsw-rt5645\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"cht-bsw\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-rt5645.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    /* use CHT driver to Baytrail Chromebooks */
    snd_soc_acpi_mach {
        id: b"193C9890\0".as_ptr() as *const c_char,
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"cht-bsw-max98090\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"cht-bsw\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-max98090.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"14F10720\0".as_ptr() as *const c_char,
        uid: core::ptr::null(),
        comp_ids: core::ptr::null(),
        link_mask: 0,
        links: core::ptr::null(),
        drv_name: b"bytcht_cx2072x\0".as_ptr() as *const c_char,
        fw_filename: b"intel/fw_sst_0f28.bin\0".as_ptr() as *const c_char,
        board: b"bytcht_cx2072x\0".as_ptr() as *const c_char,
        machine_quirk: None,
        pdata: core::ptr::null_mut(),
        sof_tplg_filename: b"sof-byt-cx2072x.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    /*
     * IS_ENABLED(CONFIG_SND_SOC_INTEL_BYT_CHT_NOCODEC_MACH):
     *
     * This is always last in the table so that it is selected only when
     * enabled explicitly and there is no codec-related information in SSDT.
     *
     * {
     *     .id = "80860F28",
     *     .drv_name = "bytcht_nocodec",
     *     .fw_filename = "intel/fw_sst_0f28.bin",
     *     .board = "bytcht_nocodec",
     * },
     */
    mach_empty(),
];

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_baytrail_machines); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
