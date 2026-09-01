// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C include dependencies translated as external Rust dependencies:
// <linux/firmware.h>, <linux/dmi.h>, <linux/module.h>, <linux/pci.h>,
// <linux/platform_data/x86/soc.h>, <linux/pm_runtime.h>,
// <sound/soc-acpi.h>, <sound/soc-acpi-intel-match.h>, <sound/sof.h>,
// "ops.h", "sof-pci-dev.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const SOF_PCI_DISABLE_PM_RUNTIME: c_int = 1 << 0;
const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;
const DMI_PRODUCT_FAMILY: c_int = 2;
const DMI_OEM_STRING: c_int = 3;
const DMI_BIOS_VERSION: c_int = 4;
const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static SND_SOF_SUSPEND_DELAY_MS: c_int;
    static SOF_IPC_TYPE_COUNT: c_int;

    fn soc_intel_is_apl() -> bool;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_allow(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn snd_sof_device_probe(dev: *mut device, pdata: *mut snd_sof_pdata) -> c_int;
    fn snd_sof_device_remove(dev: *mut device);
    fn snd_sof_device_probe_completed(dev: *mut device) -> bool;
    fn snd_sof_device_shutdown(dev: *mut device);
    fn snd_sof_prepare(dev: *mut device) -> c_int;
    fn snd_sof_complete(dev: *mut device);
    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_resume(dev: *mut device) -> c_int;
    fn snd_sof_runtime_suspend(dev: *mut device) -> c_int;
    fn snd_sof_runtime_resume(dev: *mut device) -> c_int;
    fn snd_sof_runtime_idle(dev: *mut device) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
}

#[repr(C)]
pub struct pci_device_id {
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub ops: *const c_void,
    pub ipc_default: c_int,
}

#[repr(C)]
pub struct sof_loadable_file_profile {
    pub ipc_type: c_int,
    pub fw_path: *mut c_char,
    pub fw_name: *mut c_char,
    pub fw_lib_path: *mut c_char,
    pub tplg_path: *mut c_char,
    pub fw_path_postfix: *const c_char,
    pub fw_lib_path_postfix: *const c_char,
    pub tplg_name: *const c_char,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub name: *const c_char,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
    pub subsystem_id_set: bool,
    pub desc: *const sof_dev_desc,
    pub dev: *mut device,
    pub ipc_file_profile_base: sof_loadable_file_profile,
    pub sof_probe_complete: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(id: *const dmi_system_id) -> c_int>,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

const fn dmi_match(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

static mut fw_path: *mut c_char = ptr::null_mut();
// module_param(fw_path, charp, 0444);
// MODULE_PARM_DESC(fw_path, "deprecated - moved to snd-sof module.");

static mut fw_filename: *mut c_char = ptr::null_mut();
// module_param(fw_filename, charp, 0444);
// MODULE_PARM_DESC(fw_filename, "deprecated - moved to snd-sof module.");

static mut lib_path: *mut c_char = ptr::null_mut();
// module_param(lib_path, charp, 0444);
// MODULE_PARM_DESC(lib_path, "deprecated - moved to snd-sof module.");

static mut tplg_path: *mut c_char = ptr::null_mut();
// module_param(tplg_path, charp, 0444);
// MODULE_PARM_DESC(tplg_path, "deprecated - moved to snd-sof module.");

static mut tplg_filename: *mut c_char = ptr::null_mut();
// module_param(tplg_filename, charp, 0444);
// MODULE_PARM_DESC(tplg_filename, "deprecated - moved to snd-sof module.");

static mut sof_pci_debug: c_int = 0;
// module_param_named(sof_pci_debug, sof_pci_debug, int, 0444);
// MODULE_PARM_DESC(sof_pci_debug, "SOF PCI debug options (0x0 all off)");

static mut sof_pci_ipc_type: c_int = -1;
// module_param_named(ipc_type, sof_pci_ipc_type, int, 0444);
// MODULE_PARM_DESC(ipc_type, "deprecated - moved to snd-sof module.");

static mut sof_dmi_override_tplg_name: *const c_char = ptr::null();
static mut sof_dmi_use_community_key: bool = false;

unsafe extern "C" fn sof_tplg_cb(id: *const dmi_system_id) -> c_int {
    sof_dmi_override_tplg_name = (*id).driver_data as *const c_char;
    1
}

static sof_tplg_table: [dmi_system_id; 7] = [
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google_Volteer\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO-MAX98373_ALC5682I_I2S_UP4\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-tgl-rt5682-ssp0-max98373-ssp2.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"Intel Corporation\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_PRODUCT_NAME,
                b"Alder Lake Client Platform\0".as_ptr() as *const c_char,
            ),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO-ADL_MAX98373_ALC5682I_I2S\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-adl-rt5682-ssp0-max98373-ssp2.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO-MAX98390_ALC5682I_I2S\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-adl-max98390-ssp2-rt5682-ssp0.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO_AMP-MAX98360_ALC5682VS_I2S_2WAY\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-adl-max98360a-rt5682-2way.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO-AUDIO_MAX98357_ALC5682I_I2S_2WAY\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-adl-max98357a-rt5682-2way.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: Some(sof_tplg_cb),
        ident: ptr::null(),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr() as *const c_char),
            dmi_match(
                DMI_OEM_STRING,
                b"AUDIO-MAX98360_ALC5682I_I2S_AMP_SSP2\0".as_ptr() as *const c_char,
            ),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: b"sof-adl-max98357a-rt5682.tplg\0".as_ptr() as *const c_void,
    },
    dmi_system_id {
        callback: None,
        ident: ptr::null(),
        matches: [
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: ptr::null(),
    },
];

/* all Up boards use the community key */
unsafe extern "C" fn up_use_community_key(_id: *const dmi_system_id) -> c_int {
    sof_dmi_use_community_key = true;
    1
}

/*
 * For ApolloLake Chromebooks we want to force the use of the Intel production key.
 * All newer platforms use the community key
 */
unsafe extern "C" fn chromebook_use_community_key(_id: *const dmi_system_id) -> c_int {
    if !soc_intel_is_apl() {
        sof_dmi_use_community_key = true;
    }
    1
}

static community_key_platforms: [dmi_system_id; 4] = [
    dmi_system_id {
        ident: b"Up boards\0".as_ptr() as *const c_char,
        callback: Some(up_use_community_key),
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"AAEON\0".as_ptr() as *const c_char),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: ptr::null(),
    },
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr() as *const c_char,
        callback: Some(chromebook_use_community_key),
        matches: [
            dmi_match(DMI_PRODUCT_FAMILY, b"Google\0".as_ptr() as *const c_char),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: ptr::null(),
    },
    dmi_system_id {
        ident: b"Google firmware\0".as_ptr() as *const c_char,
        callback: Some(chromebook_use_community_key),
        matches: [
            dmi_match(DMI_BIOS_VERSION, b"Google\0".as_ptr() as *const c_char),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: ptr::null(),
    },
    dmi_system_id {
        ident: ptr::null(),
        callback: None,
        matches: [
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
            dmi_match(0, ptr::null()),
        ],
        driver_data: ptr::null(),
    },
];

// EXPORT_NS_DEV_PM_OPS(sof_pci_pm, SND_SOC_SOF_PCI_DEV)
#[no_mangle]
pub static sof_pci_pm: dev_pm_ops = dev_pm_ops {
    prepare: Some(snd_sof_prepare),
    complete: Some(snd_sof_complete),
    suspend: Some(snd_sof_suspend),
    resume: Some(snd_sof_resume),
    runtime_suspend: Some(snd_sof_runtime_suspend),
    runtime_resume: Some(snd_sof_runtime_resume),
    runtime_idle: Some(snd_sof_runtime_idle),
};

unsafe extern "C" fn sof_pci_probe_complete(dev: *mut device) {
    dev_dbg(dev, b"Completing SOF PCI probe\0".as_ptr() as *const c_char);

    if sof_pci_debug & SOF_PCI_DISABLE_PM_RUNTIME != 0 {
        return;
    }

    /* allow runtime_pm */
    pm_runtime_set_autosuspend_delay(dev, SND_SOF_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);

    /*
     * runtime pm for pci device is "forbidden" by default.
     * so call pm_runtime_allow() to enable it.
     */
    pm_runtime_allow(dev);

    /* mark last_busy for pm_runtime to make sure not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    /* follow recommendation in pci-driver.c to decrement usage counter */
    pm_runtime_put_noidle(dev);
}

#[no_mangle]
pub unsafe extern "C" fn sof_pci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let mut path_override: *mut sof_loadable_file_profile;
    let dev: *mut device = &mut (*pci).dev;
    let desc: *const sof_dev_desc = (*pci_id).driver_data as *const sof_dev_desc;
    let sof_pdata: *mut snd_sof_pdata;
    let mut ret: c_int;

    dev_dbg(
        &mut (*pci).dev,
        b"PCI DSP detected\0".as_ptr() as *const c_char,
    );

    if desc.is_null() {
        dev_err(dev, b"error: no matching PCI descriptor\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if (*desc).ops.is_null() {
        dev_err(
            dev,
            b"error: no matching PCI descriptor ops\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    sof_pdata = devm_kzalloc(dev, size_of::<snd_sof_pdata>(), GFP_KERNEL) as *mut snd_sof_pdata;
    if sof_pdata.is_null() {
        return -ENOMEM;
    }

    ret = pcim_enable_device(pci);
    if ret < 0 {
        return ret;
    }

    ret = pcim_request_all_regions(pci, b"Audio DSP\0".as_ptr() as *const c_char);
    if ret < 0 {
        return ret;
    }

    (*sof_pdata).name = pci_name(pci);

    /* PCI defines a vendor ID of 0xFFFF as invalid. */
    if (*pci).subsystem_vendor != 0xFFFF {
        (*sof_pdata).subsystem_vendor = (*pci).subsystem_vendor;
        (*sof_pdata).subsystem_device = (*pci).subsystem_device;
        (*sof_pdata).subsystem_id_set = true;
    }

    (*sof_pdata).desc = desc;
    (*sof_pdata).dev = dev;

    path_override = &mut (*sof_pdata).ipc_file_profile_base;

    if sof_pci_ipc_type < 0 {
        (*path_override).ipc_type = (*desc).ipc_default;
    } else if sof_pci_ipc_type < SOF_IPC_TYPE_COUNT {
        (*path_override).ipc_type = sof_pci_ipc_type;
    } else {
        dev_err(
            dev,
            b"Invalid IPC type requested: %d\n\0".as_ptr() as *const c_char,
            sof_pci_ipc_type,
        );
        return -EINVAL;
    }

    (*path_override).fw_path = fw_path;
    (*path_override).fw_name = fw_filename;
    (*path_override).fw_lib_path = lib_path;
    (*path_override).tplg_path = tplg_path;

    if dmi_check_system(community_key_platforms.as_ptr()) != 0 && sof_dmi_use_community_key {
        (*path_override).fw_path_postfix = b"community\0".as_ptr() as *const c_char;
        (*path_override).fw_lib_path_postfix = b"community\0".as_ptr() as *const c_char;
    }

    /*
     * the topology filename will be provided in the machine descriptor, unless
     * it is overridden by a module parameter or DMI quirk.
     */
    if !tplg_filename.is_null() {
        (*path_override).tplg_name = tplg_filename as *const c_char;
    } else {
        dmi_check_system(sof_tplg_table.as_ptr());
        if !sof_dmi_override_tplg_name.is_null() {
            (*path_override).tplg_name = sof_dmi_override_tplg_name;
        }
    }

    /* set callback to be called on successful device probe to enable runtime_pm */
    (*sof_pdata).sof_probe_complete = Some(sof_pci_probe_complete);

    /* call sof helper for DSP hardware probe */
    snd_sof_device_probe(dev, sof_pdata)
}
// EXPORT_SYMBOL_NS(sof_pci_probe, "SND_SOC_SOF_PCI_DEV");

#[no_mangle]
pub unsafe extern "C" fn sof_pci_remove(pci: *mut pci_dev) {
    /* call sof helper for DSP hardware remove */
    snd_sof_device_remove(&mut (*pci).dev);

    /* follow recommendation in pci-driver.c to increment usage counter */
    if snd_sof_device_probe_completed(&mut (*pci).dev)
        && !(sof_pci_debug & SOF_PCI_DISABLE_PM_RUNTIME != 0)
    {
        pm_runtime_get_noresume(&mut (*pci).dev);
    }
}
// EXPORT_SYMBOL_NS(sof_pci_remove, "SND_SOC_SOF_PCI_DEV");

#[no_mangle]
pub unsafe extern "C" fn sof_pci_shutdown(pci: *mut pci_dev) {
    snd_sof_device_shutdown(&mut (*pci).dev);
}
// EXPORT_SYMBOL_NS(sof_pci_shutdown, "SND_SOC_SOF_PCI_DEV");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for PCI platforms");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
