// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

type bool_ = bool;
type size_t = usize;
type u32 = c_uint;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const PLATFORM_DEVID_AUTO: c_int = -1;
const ACPI_NHLT_LINKTYPE_PDM: c_int = 0;
const ACPI_NHLT_LINKTYPE_SSP: c_int = 1;
const CONFIG_SND_SOC_INTEL_AVS_CARDNAME_OBSOLETE: bool = false;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn BIT(x: c_int) -> c_ulong {
    1 as c_ulong << x
}

const fn GENMASK(h: c_int, l: c_int) -> c_ulong {
    (!0 as c_ulong >> (c_ulong::BITS as c_int - 1 - h)) & (!0 as c_ulong << l)
}

const fn AVS_SSP(x: c_int) -> c_ulong {
    BIT(x)
}

const fn AVS_SSP_RANGE(a: c_int, b: c_int) -> c_ulong {
    GENMASK(b, a)
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct pci_dev {
    pub device: c_int,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hdac_bus {
    pub num_codecs: c_uint,
    pub codec_list: list_head,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub list: list_head,
    pub vendor_id: c_uint,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
}

#[repr(C)]
pub struct avs_base {
    pub pci: *mut pci_dev,
    pub core: hdac_bus,
}

#[repr(C)]
pub struct avs_i2s_caps {
    pub ctrl_count: c_int,
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub i2s_caps: avs_i2s_caps,
}

#[repr(C)]
pub struct avs_dev {
    pub base: avs_base,
    pub dev: *mut device,
    pub hw_cfg: avs_hw_cfg,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub i2s_link_mask: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub uid: *const c_char,
    pub drv_name: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
    pub quirk_data: *const c_void,
    pub machine_quirk: Option<unsafe extern "C" fn(*mut c_void) -> *mut snd_soc_acpi_mach>,
    pub pdata: *mut c_void,
    pub tplg_filename: *const c_char,
}

impl Default for snd_soc_acpi_mach {
    fn default() -> Self {
        Self {
            id: null(),
            uid: null(),
            drv_name: null(),
            mach_params: snd_soc_acpi_mach_params { i2s_link_mask: 0 },
            quirk_data: null(),
            machine_quirk: None,
            pdata: null_mut(),
            tplg_filename: null(),
        }
    }
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub codec: *mut hda_codec,
    pub tdms: *mut c_ulong,
    pub codec_name: *mut c_char,
    pub obsolete_card_names: bool_,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
}

const DMI_SYS_VENDOR: c_int = 1;
const DMI_BOARD_NAME: c_int = 2;

const fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

unsafe extern "C" {
    static PCI_DEVICE_ID_INTEL_HDA_SKL_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_KBL_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_APL: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_GLK: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_CNL_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_CNL_H: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_CML_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_ICL_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_TGL_LP: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_EHL_0: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_ADL_N: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_ADL_P: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_RPL_P_0: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_RPL_M: c_int;
    static PCI_DEVICE_ID_INTEL_HDA_FCL: c_int;

    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn platform_device_unregister(pdev: *mut c_void);
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: size_t,
    ) -> *mut platform_device;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(err: c_int) -> *mut platform_device;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn avs_register_probe_component(adev: *mut avs_dev, name: *const c_char) -> c_int;
    fn avs_register_dmic_component(adev: *mut avs_dev, name: *const c_char) -> c_int;
    fn avs_register_i2s_component(
        adev: *mut avs_dev,
        name: *const c_char,
        i2s_mask: u32,
        tdms: *mut c_ulong,
    ) -> c_int;
    fn avs_register_hda_component(adev: *mut avs_dev, name: *const c_char) -> c_int;
    fn acpi_nhlt_find_endpoint(link_type: c_int, dev_type: c_int, dir: c_int, bus_id: c_int) -> *mut c_void;
    fn parse_int_array(s: *const c_char, len: size_t, array: *mut *mut c_int) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn kfree(ptr: *mut c_void);
    fn fls(x: c_int) -> c_int;
    fn __fls(x: c_ulong) -> c_ulong;
    fn acpi_dev_present(hid: *const c_char, uid: *const c_char, hrv: c_int) -> bool_;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_to_hda_codec(dev: *mut device) -> *mut hda_codec;
    fn snd_soc_unregister_component(dev: *mut device);
}

static mut i2s_test: *mut c_char = null_mut();

// module_param(i2s_test, charp, 0444);
// MODULE_PARM_DESC(i2s_test, "Use I2S test-board instead of ACPI, i2s_test=ssp0tdm,ssp1tdm,... 0 to ignore port");

static mut obsolete_card_names: bool_ = CONFIG_SND_SOC_INTEL_AVS_CARDNAME_OBSOLETE;

// module_param_named(obsolete_card_names, obsolete_card_names, bool, 0444);
// MODULE_PARM_DESC(obsolete_card_names, "Use obsolete card names 0=no, 1=yes");

static kbl_dmi_table: [dmi_system_id; 3] = [
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, cstr!("Intel Corporation")),
            DMI_MATCH(DMI_BOARD_NAME, cstr!("Skylake Y LPDDR3 RVP3")),
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, cstr!("Intel Corporation")),
            DMI_MATCH(DMI_BOARD_NAME, cstr!("AmberLake Y")),
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
    },
    dmi_system_id { matches: [dmi_strmatch { slot: 0, substr: null() }; 4] },
];

static kblr_dmi_table: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, cstr!("Intel Corporation")),
            DMI_MATCH(DMI_BOARD_NAME, cstr!("Kabylake R DDR4 RVP")),
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
    },
    dmi_system_id { matches: [dmi_strmatch { slot: 0, substr: null() }; 4] },
];

unsafe extern "C" fn dmi_match_quirk(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
    let mach = arg as *mut snd_soc_acpi_mach;
    let dmi_table: *mut dmi_system_id;

    dmi_table = (*mach).quirk_data as *mut dmi_system_id;

    if dmi_table.is_null() || !dmi_first_match(dmi_table).is_null() {
        return mach;
    }
    null_mut()
}

/* supported I2S board codec configurations */
static mut avs_skl_i2s_machines: [snd_soc_acpi_mach; 5] = [
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt286"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt286-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10508825"), drv_name: cstr!("avs_nau8825"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("nau8825-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("INT343B"), drv_name: cstr!("avs_ssm4567"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("ssm4567-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("MX98357A"), drv_name: cstr!("avs_max98357a"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("max98357a-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_kbl_i2s_machines: [snd_soc_acpi_mach; 10] = [
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt286"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, quirk_data: kbl_dmi_table.as_ptr() as *const c_void, machine_quirk: Some(dmi_match_quirk), tplg_filename: cstr!("rt286-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt298"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, quirk_data: kblr_dmi_table.as_ptr() as *const c_void, machine_quirk: Some(dmi_match_quirk), tplg_filename: cstr!("rt298-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("MX98927"), drv_name: cstr!("avs_max98927"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("max98927-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5514"), drv_name: cstr!("avs_rt5514"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, pdata: null_mut(), tplg_filename: cstr!("rt5514-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5663"), drv_name: cstr!("avs_rt5663"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("rt5663-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("MX98373"), drv_name: cstr!("avs_max98373"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("max98373-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("MX98357A"), drv_name: cstr!("avs_max98357a"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("max98357a-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("DLGS7219"), drv_name: cstr!("avs_da7219"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("da7219-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("ESSX8336"), drv_name: cstr!("avs_es8336"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("es8336-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_apl_i2s_machines: [snd_soc_acpi_mach; 5] = [
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt298"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(5) as u32 }, tplg_filename: cstr!("rt298-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("INT34C3"), drv_name: cstr!("avs_tdf8532"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP_RANGE(0, 5) as u32 }, pdata: null_mut(), tplg_filename: cstr!("tdf8532-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("MX98357A"), drv_name: cstr!("avs_max98357a"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(5) as u32 }, tplg_filename: cstr!("max98357a-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("DLGS7219"), drv_name: cstr!("avs_da7219"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("da7219-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_gml_i2s_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt298"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(2) as u32 }, tplg_filename: cstr!("rt298-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_cnl_i2s_machines: [snd_soc_acpi_mach; 3] = [
    snd_soc_acpi_mach { id: cstr!("INT34C2"), drv_name: cstr!("avs_rt274"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt274-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5682"), drv_name: cstr!("avs_rt5682"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("rt5682-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_icl_i2s_machines: [snd_soc_acpi_mach; 3] = [
    snd_soc_acpi_mach { id: cstr!("INT343A"), drv_name: cstr!("avs_rt298"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt298-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("INT34C2"), drv_name: cstr!("avs_rt274"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt274-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_tgl_i2s_machines: [snd_soc_acpi_mach; 8] = [
    snd_soc_acpi_mach { id: cstr!("INT34C2"), drv_name: cstr!("avs_rt274"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt274-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC0298"), drv_name: cstr!("avs_rt298"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt298-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC1308"), drv_name: cstr!("avs_rt1308"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("rt1308-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5640"), uid: cstr!("1"), drv_name: cstr!("avs_rt5640"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("rt5640-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5640"), uid: cstr!("3"), drv_name: cstr!("avs_rt5640"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(1) as u32 }, tplg_filename: cstr!("rt5640-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("10EC5640"), uid: cstr!("2"), drv_name: cstr!("avs_rt5640"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(2) as u32 }, tplg_filename: cstr!("rt5640-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach { id: cstr!("ESSX8336"), drv_name: cstr!("avs_es8336"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: AVS_SSP(0) as u32 }, tplg_filename: cstr!("es8336-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

static mut avs_mbl_i2s_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach { id: cstr!("PCM3168A"), drv_name: cstr!("avs_pcm3168a"), mach_params: snd_soc_acpi_mach_params { i2s_link_mask: (AVS_SSP(0) | AVS_SSP(2)) as u32 }, tplg_filename: cstr!("pcm3168a-tplg.bin"), ..snd_soc_acpi_mach::default() },
    snd_soc_acpi_mach::default(),
];

#[repr(C)]
struct avs_acpi_boards {
    id: c_int,
    machs: *mut snd_soc_acpi_mach,
}

/* supported I2S boards per platform */
static mut i2s_boards: [avs_acpi_boards; 16] = unsafe {
    [
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_SKL_LP, machs: avs_skl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_KBL_LP, machs: avs_kbl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_APL, machs: avs_apl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_GLK, machs: avs_gml_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_CNL_LP, machs: avs_cnl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_CNL_H, machs: avs_cnl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_CML_LP, machs: avs_cnl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_ICL_LP, machs: avs_icl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_TGL_LP, machs: avs_tgl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_EHL_0, machs: avs_tgl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_ADL_N, machs: avs_mbl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_ADL_P, machs: avs_tgl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_RPL_P_0, machs: avs_tgl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_RPL_M, machs: avs_mbl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: PCI_DEVICE_ID_INTEL_HDA_FCL, machs: avs_tgl_i2s_machines.as_mut_ptr() },
        avs_acpi_boards { id: 0, machs: null_mut() },
    ]
};

unsafe fn avs_get_i2s_machines(adev: *mut avs_dev) -> *mut snd_soc_acpi_mach {
    let id: c_int;
    let mut i: usize;

    id = (*(*adev).base.pci).device;
    i = 0;
    while i < i2s_boards.len() {
        if i2s_boards[i].id == id {
            return i2s_boards[i].machs;
        }
        i += 1;
    }
    null_mut()
}

/* Platform devices spawned by AVS driver are removed with this hook. */
unsafe extern "C" fn avs_unregister_board(pdev: *mut c_void) {
    platform_device_unregister(pdev);
}

unsafe fn avs_register_board(
    adev: *mut avs_dev,
    name: *const c_char,
    data: *const c_void,
    size: size_t,
) -> *mut platform_device {
    let pdev: *mut platform_device;
    let ret: c_int;

    pdev = platform_device_register_data(null_mut(), name, PLATFORM_DEVID_AUTO, data, size);
    if IS_ERR(pdev as *const c_void) {
        return pdev;
    }

    ret = devm_add_action_or_reset((*adev).dev, avs_unregister_board, pdev as *mut c_void);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    pdev
}

unsafe fn avs_register_board_pdata(
    adev: *mut avs_dev,
    name: *const c_char,
    mach: *mut snd_soc_acpi_mach,
    codec: *mut hda_codec,
    tdms: *mut c_ulong,
    codec_name: *mut c_char,
) -> *mut platform_device {
    let pdata: *mut avs_mach_pdata;

    pdata = devm_kzalloc((*adev).dev, size_of::<avs_mach_pdata>(), GFP_KERNEL) as *mut avs_mach_pdata;
    if pdata.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*pdata).codec = codec;
    (*pdata).tdms = tdms;
    (*pdata).codec_name = codec_name;
    (*pdata).obsolete_card_names = obsolete_card_names;
    (*mach).pdata = pdata as *mut c_void;

    avs_register_board(adev, name, mach as *const c_void, size_of::<snd_soc_acpi_mach>())
}

unsafe fn avs_register_probe_board(adev: *mut avs_dev) -> c_int {
    let pdev: *mut platform_device;

    pdev = avs_register_board(adev, cstr!("avs_probe_mb"), null(), 0);
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    avs_register_probe_component(adev, dev_name(addr_of_mut!((*pdev).dev)))
}

unsafe fn avs_register_dmic_board(adev: *mut avs_dev) -> c_int {
    static mut mach: snd_soc_acpi_mach = snd_soc_acpi_mach {
        tplg_filename: cstr!("dmic-tplg.bin"),
        id: null(),
        uid: null(),
        drv_name: null(),
        mach_params: snd_soc_acpi_mach_params { i2s_link_mask: 0 },
        quirk_data: null(),
        machine_quirk: None,
        pdata: null_mut(),
    };
    let mut pdev: *mut platform_device;
    let codec_name: *mut c_char;

    if acpi_nhlt_find_endpoint(ACPI_NHLT_LINKTYPE_PDM, -1, -1, -1).is_null() {
        dev_dbg((*adev).dev, cstr!("no DMIC endpoints present\n"));
        return 0;
    }

    /* DMIC present in Intel PCH is enumerated statically. */
    pdev = avs_register_board(adev, cstr!("dmic-codec"), null(), 0);
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    codec_name = devm_kstrdup((*adev).dev, dev_name(addr_of_mut!((*pdev).dev)), GFP_KERNEL);
    if codec_name.is_null() {
        return -ENOMEM;
    }

    pdev = avs_register_board_pdata(adev, cstr!("avs_dmic"), addr_of_mut!(mach), null_mut(), null_mut(), codec_name);
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    avs_register_dmic_component(adev, dev_name(addr_of_mut!((*pdev).dev)))
}

unsafe fn avs_register_i2s_test_board(adev: *mut avs_dev, ssp_port: c_int, tdm_slot: c_int) -> c_int {
    let mut mach = snd_soc_acpi_mach::default();
    let pdev: *mut platform_device;
    let tdms: *mut c_ulong;

    tdms = devm_kcalloc((*adev).dev, (ssp_port + 1) as size_t, size_of::<c_ulong>(), GFP_KERNEL) as *mut c_ulong;
    mach.tplg_filename = devm_kasprintf(
        (*adev).dev,
        GFP_KERNEL,
        cstr!("i2s%d-%d-test-tplg.bin"),
        ssp_port,
        tdm_slot,
    );
    if tdms.is_null() || mach.tplg_filename.is_null() {
        return -ENOMEM;
    }

    *tdms.add(ssp_port as usize) = BIT(tdm_slot);
    mach.drv_name = cstr!("avs_i2s_test");
    mach.mach_params.i2s_link_mask = AVS_SSP(ssp_port) as u32;

    pdev = avs_register_board_pdata(adev, mach.drv_name, &mut mach, null_mut(), tdms, null_mut());
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    avs_register_i2s_component(adev, dev_name(addr_of_mut!((*pdev).dev)), AVS_SSP(ssp_port) as u32, tdms)
}

unsafe fn avs_register_i2s_test_boards(adev: *mut avs_dev) -> c_int {
    let max_ssps = (*adev).hw_cfg.i2s_caps.ctrl_count;
    let mut ssp_port: c_int;
    let mut tdm_slot: c_int;
    let mut ret: c_int;
    let mut tdm_slots: c_ulong;
    let mut array: *mut u32 = null_mut();
    let num_elems: u32;

    if i2s_test.is_null() {
        return 0;
    }

    ret = parse_int_array(i2s_test, strlen(i2s_test), &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret != 0 {
        dev_err((*adev).dev, cstr!("failed to parse i2s_test parameter\n"));
        return ret;
    }

    num_elems = *array;
    if num_elems > max_ssps as u32 {
        dev_err(
            (*adev).dev,
            cstr!("board supports only %d SSP, %d specified\n"),
            max_ssps,
            num_elems,
        );
        ret = -EINVAL;
        kfree(array as *mut c_void);
        return ret;
    }

    ssp_port = 0;
    while ssp_port < num_elems as c_int {
        tdm_slots = *array.add((1 + ssp_port) as usize) as c_ulong;
        tdm_slot = 0;
        while tdm_slot < 16 {
            if (tdm_slots & BIT(tdm_slot)) != 0 {
                ret = avs_register_i2s_test_board(adev, ssp_port, tdm_slot);
                if ret != 0 {
                    kfree(array as *mut c_void);
                    return ret;
                }
            }
            tdm_slot += 1;
        }
        ssp_port += 1;
    }

    kfree(array as *mut c_void);
    ret
}

unsafe fn avs_register_i2s_board(adev: *mut avs_dev, mach: *mut snd_soc_acpi_mach) -> c_int {
    let i2s_mask: u32 = (*mach).mach_params.i2s_link_mask;
    let pdev: *mut platform_device;
    let mut tdms: *mut c_ulong = null_mut();

    if !(*mach).pdata.is_null() {
        tdms = ((*mach).pdata as *mut avs_mach_pdata).as_mut().unwrap().tdms;
    }

    pdev = avs_register_board_pdata(adev, (*mach).drv_name, mach, null_mut(), tdms, null_mut());
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    avs_register_i2s_component(adev, dev_name(addr_of_mut!((*pdev).dev)), i2s_mask, tdms)
}

unsafe fn avs_register_i2s_boards(adev: *mut avs_dev) -> c_int {
    let num_ssps = (*adev).hw_cfg.i2s_caps.ctrl_count;
    let machs: *mut snd_soc_acpi_mach;
    let mut mach: *mut snd_soc_acpi_mach;
    let mut ret: c_int;

    if acpi_nhlt_find_endpoint(ACPI_NHLT_LINKTYPE_SSP, -1, -1, -1).is_null() {
        dev_dbg((*adev).dev, cstr!("no I2S endpoints present\n"));
        return 0;
    }

    machs = avs_get_i2s_machines(adev);
    if machs.is_null() {
        dev_dbg((*adev).dev, cstr!("no I2S endpoints supported\n"));
        return 0;
    }

    mach = machs;
    while !(*mach).id.is_null() && *(*mach).id != 0 {
        if !acpi_dev_present((*mach).id, (*mach).uid, -1) {
            mach = mach.add(1);
            continue;
        }

        if fls((*mach).mach_params.i2s_link_mask as c_int) > num_ssps {
            dev_err(
                (*adev).dev,
                cstr!("Platform supports %d SSPs but board %s requires SSP%ld\n"),
                num_ssps,
                (*mach).drv_name,
                __fls((*mach).mach_params.i2s_link_mask as c_ulong),
            );
            mach = mach.add(1);
            continue;
        }
        if let Some(machine_quirk) = (*mach).machine_quirk {
            if machine_quirk(mach as *mut c_void).is_null() {
                mach = mach.add(1);
                continue;
            }
        }

        ret = avs_register_i2s_board(adev, mach);
        if ret < 0 {
            dev_warn((*adev).dev, cstr!("register i2s %s failed: %d\n"), (*mach).drv_name, ret);
        }
        mach = mach.add(1);
    }

    0
}

unsafe fn avs_register_hda_board(adev: *mut avs_dev, codec: *mut hda_codec) -> c_int {
    let hdev: *mut hdac_device = addr_of_mut!((*codec).core);
    let mut mach = snd_soc_acpi_mach::default();
    let pdev: *mut platform_device;

    mach.tplg_filename = devm_kasprintf((*adev).dev, GFP_KERNEL, cstr!("hda-%08x-tplg.bin"), (*hdev).vendor_id);
    if mach.tplg_filename.is_null() {
        return -ENOMEM;
    }

    pdev = avs_register_board_pdata(adev, cstr!("avs_hdaudio"), &mut mach, codec, null_mut(), null_mut());
    if IS_ERR(pdev as *const c_void) {
        return PTR_ERR(pdev as *const c_void);
    }

    avs_register_hda_component(adev, dev_name(addr_of_mut!((*pdev).dev)))
}

unsafe fn avs_register_hda_boards(adev: *mut avs_dev) -> c_int {
    let bus: *mut hdac_bus = addr_of_mut!((*adev).base.core);
    let mut hdev: *mut hdac_device;
    let mut pos: *mut list_head;
    let mut ret: c_int;

    if (*bus).num_codecs == 0 {
        dev_dbg((*adev).dev, cstr!("no HDA endpoints present\n"));
        return 0;
    }

    pos = (*bus).codec_list.next;
    while pos != addr_of_mut!((*bus).codec_list) {
        let codec: *mut hda_codec;

        hdev = pos as *mut hdac_device;
        codec = dev_to_hda_codec(addr_of_mut!((*hdev).dev));

        ret = avs_register_hda_board(adev, codec);
        if ret < 0 {
            dev_warn(
                (*adev).dev,
                cstr!("register hda-%08x failed: %d\n"),
                (*codec).core.vendor_id,
                ret,
            );
        }
        pos = (*pos).next;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_register_all_boards(adev: *mut avs_dev) -> c_int {
    let mut ret: c_int;

    // #ifdef CONFIG_DEBUG_FS
    // ret = avs_register_probe_board(adev);
    // if (ret < 0)
    //     dev_warn(adev->dev, "enumerate PROBE endpoints failed: %d\n", ret);
    // #endif

    ret = avs_register_dmic_board(adev);
    if ret < 0 {
        dev_warn((*adev).dev, cstr!("enumerate DMIC endpoints failed: %d\n"), ret);
    }

    ret = avs_register_i2s_test_boards(adev);
    if ret != 0 {
        dev_dbg((*adev).dev, cstr!("enumerate I2S TEST endpoints failed: %d\n"), ret);
    }

    ret = avs_register_i2s_boards(adev);
    if ret < 0 {
        dev_warn((*adev).dev, cstr!("enumerate I2S endpoints failed: %d\n"), ret);
    }

    ret = avs_register_hda_boards(adev);
    if ret < 0 {
        dev_warn((*adev).dev, cstr!("enumerate HDA endpoints failed: %d\n"), ret);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_unregister_all_boards(adev: *mut avs_dev) {
    snd_soc_unregister_component((*adev).dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
