/* SPDX-License-Identifier: GPL-2.0-only
 *
 * Copyright (C) 2013-15, Intel Corporation
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct snd_soc_acpi_package_context {
    pub name: *mut c_char,           /* package name */
    pub length: c_int,               /* number of elements */
    pub format: *mut acpi_buffer,
    pub state: *mut acpi_buffer,
    pub data_valid: bool,
}

/* codec name is used in DAIs is i2c-<HID>:00 with HID being 8 chars */
pub const SND_ACPI_I2C_ID_LEN: usize = 4 + ACPI_ID_LEN + 3 + 1;

/* acpi match */
#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub fn snd_soc_acpi_find_machine(
        machines: *mut snd_soc_acpi_mach,
    ) -> *mut snd_soc_acpi_mach;

    pub fn snd_soc_acpi_find_package_from_hid(
        hid: *const u8,
        ctx: *mut snd_soc_acpi_package_context,
    ) -> bool;

    /* check all codecs */
    pub fn snd_soc_acpi_codec_list(arg: *mut c_void) -> *mut snd_soc_acpi_mach;
}

/* Build-time CONFIG_ACPI=false fallback. */
#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn snd_soc_acpi_find_machine(
    _machines: *mut snd_soc_acpi_mach,
) -> *mut snd_soc_acpi_mach {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn snd_soc_acpi_find_package_from_hid(
    _hid: *const u8,
    _ctx: *mut snd_soc_acpi_package_context,
) -> bool {
    false
}

/* check all codecs */
#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn snd_soc_acpi_codec_list(_arg: *mut c_void) -> *mut snd_soc_acpi_mach {
    core::ptr::null_mut()
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub acpi_ipc_irq_index: u32,
    pub platform: *const c_char,
    pub codec_mask: u32,
    pub dmic_num: u32,
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub i2s_link_mask: u32,
    pub num_dai_drivers: u32,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub subsystem_rev: u16,
    pub subsystem_id_set: bool,
    pub bt_link_mask: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_endpoint {
    pub num: u8,
    pub aggregated: u8,
    pub group_position: u8,
    pub group_id: u8,
}

#[repr(C)]
pub struct snd_soc_acpi_adr_device {
    pub adr: u64,
    pub num_endpoints: u8,
    pub endpoints: *const snd_soc_acpi_endpoint,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub mask: u32,
    pub num_adr: u32,
    pub adr_d: *const snd_soc_acpi_adr_device,
}

/* when set the topology uses the -ssp<N> suffix, where N is determined based on
 * BIOS or DMI information
 */
pub const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: u32 = 1 << 0;

/* when more than one SSP is reported in the link mask, use the most significant.
 * This choice was found to be valid on platforms with ES8336 codecs.
 */
pub const SND_SOC_ACPI_TPLG_INTEL_SSP_MSB: u32 = 1 << 1;

/* when set the topology uses the -dmic<N>ch suffix, where N is determined based on
 * BIOS or DMI information
 */
pub const SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u32 = 1 << 2;

/* when set the speaker amplifier name suffix (i.e. "-max98360a") will be
 * appended to topology file name
 */
pub const SND_SOC_ACPI_TPLG_INTEL_AMP_NAME: u32 = 1 << 3;

/* when set the headphone codec name suffix (i.e. "-rt5682") will be appended to
 * topology file name
 */
pub const SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME: u32 = 1 << 4;

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: [u8; ACPI_ID_LEN],
    pub uid: *const c_char,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub link_mask: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub tplg_filename: *const c_char,
    pub board: *const c_char,
    pub machine_quirk: Option<unsafe extern "C" fn(*mut c_void) -> *mut snd_soc_acpi_mach>,
    pub quirk_data: *const c_void,
    pub machine_check: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    pub pdata: *mut c_void,
    pub mach_params: snd_soc_acpi_mach_params,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: u32,
    pub get_function_tplg_files: Option<unsafe extern "C" fn(
        *mut snd_soc_card,
        *const snd_soc_acpi_mach,
        *const c_char,
        *mut *const *const c_char,
        bool,
    ) -> c_int>,
}

pub const SND_SOC_ACPI_MAX_CODECS: usize = 3;

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: c_int,
    pub codecs: [[u8; ACPI_ID_LEN]; SND_SOC_ACPI_MAX_CODECS],
}

pub unsafe fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool {
    !(*dev).parent.is_null()
        && !(*(*dev).parent).driver.is_null()
        && !(*(*(*dev).parent).driver).name.is_null()
        && strncmp(
            (*(*(*dev).parent).driver).name,
            b"sof-audio-acpi\0".as_ptr() as *const c_char,
            strlen(b"sof-audio-acpi\0".as_ptr() as *const c_char),
        ) == 0
}

extern "C" {
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    pub fn snd_soc_acpi_sdw_link_slaves_found(
        dev: *mut device,
        link: *const snd_soc_acpi_link_adr,
        peripherals: *mut sdw_peripherals,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
