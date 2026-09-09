/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2022-2023 Intel Corporation
 */

#[repr(C)]
pub struct hdac_bus;
#[repr(C)]
pub struct hdac_ext_link;
#[repr(C)]
pub struct mutex;
pub type c_ulong = usize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hda_bus_ml_link_type {
    HDA_BUS_ML_LINK_HDA,
    HDA_BUS_ML_LINK_SDW,
    HDA_BUS_ML_LINK_UAOL,
    HDA_BUS_ML_LINK_OTHER,
}

// CONFIG_SND_SOC_SOF_HDA_MLINK: declarations when enabled; inline stubs otherwise.
#[cfg(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK")]
extern "C" {
    pub fn hda_bus_ml_init(bus: *mut hdac_bus) -> i32;
    pub fn hda_bus_ml_free(bus: *mut hdac_bus);
    pub fn hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32;
    pub fn hdac_bus_eml_enable_interrupt_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, enable: bool);
    pub fn hdac_bus_eml_enable_interrupt(bus: *mut hdac_bus, alt: bool, elid: i32, enable: bool);
    pub fn hdac_bus_eml_check_interrupt(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool;
    pub fn hdac_bus_eml_set_syncprd_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, syncprd: u32) -> i32;
    pub fn hdac_bus_eml_sdw_set_syncprd_unlocked(bus: *mut hdac_bus, syncprd: u32) -> i32;
    pub fn hdac_bus_eml_wait_syncpu_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32;
    pub fn hdac_bus_eml_sdw_wait_syncpu_unlocked(bus: *mut hdac_bus) -> i32;
    pub fn hdac_bus_eml_sync_arm_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32);
    pub fn hdac_bus_eml_sdw_sync_arm_unlocked(bus: *mut hdac_bus, sublink: i32);
    pub fn hdac_bus_eml_sync_go_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32;
    pub fn hdac_bus_eml_sdw_sync_go_unlocked(bus: *mut hdac_bus) -> i32;
    pub fn hdac_bus_eml_check_cmdsync_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool;
    pub fn hdac_bus_eml_sdw_check_cmdsync_unlocked(bus: *mut hdac_bus) -> bool;
    pub fn hdac_bus_eml_power_up(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32;
    pub fn hdac_bus_eml_power_up_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32;
    pub fn hdac_bus_eml_power_down(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32;
    pub fn hdac_bus_eml_power_down_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32;
    pub fn hdac_bus_eml_sdw_power_up_unlocked(bus: *mut hdac_bus, sublink: i32) -> i32;
    pub fn hdac_bus_eml_sdw_power_down_unlocked(bus: *mut hdac_bus, sublink: i32) -> i32;
    pub fn hdac_bus_eml_sdw_get_lsdiid_unlocked(bus: *mut hdac_bus, sublink: i32, lsdiid: *mut u16) -> i32;
    pub fn hdac_bus_eml_sdw_set_lsdiid(bus: *mut hdac_bus, sublink: i32, dev_num: i32) -> i32;
    pub fn hdac_bus_eml_sdw_map_stream_ch(bus: *mut hdac_bus, sublink: i32, y: i32, channel_mask: i32, stream_id: i32, dir: i32) -> i32;
    pub fn hda_bus_ml_reset_losidv(bus: *mut hdac_bus);
    pub fn hda_bus_ml_resume(bus: *mut hdac_bus) -> i32;
    pub fn hda_bus_ml_suspend(bus: *mut hdac_bus) -> i32;
    pub fn hda_bus_ml_link_get_type(hlink: *mut hdac_ext_link) -> hda_bus_ml_link_type;
    pub fn hdac_bus_eml_ssp_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link;
    pub fn hdac_bus_eml_dmic_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link;
    pub fn hdac_bus_eml_sdw_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link;
    pub fn hdac_bus_eml_get_mutex(bus: *mut hdac_bus, alt: bool, elid: i32) -> *mut mutex;
    pub fn hdac_bus_eml_enable_offload(bus: *mut hdac_bus, alt: bool, elid: i32, enable: bool);
    pub fn hdac_bus_eml_set_mic_privacy_mask(bus: *mut hdac_bus, alt: bool, elid: i32, mask: c_ulong);
    pub fn hdac_bus_eml_is_mic_privacy_changed(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool;
    pub fn hdac_bus_eml_get_mic_privacy_state(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool;
}

#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hda_bus_ml_init(_: *mut hdac_bus) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hda_bus_ml_free(_: *mut hdac_bus) {}

// The disabled configuration preserves the C inline stubs and their return values.
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
macro_rules! zero_i32 { ($($name:ident($($arg:ident: $ty:ty),*)),* $(,)?) => { $(pub unsafe fn $name($(_: $ty),*) -> i32 { 0 })* }; }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
macro_rules! zero_bool { ($($name:ident($($arg:ident: $ty:ty),*)),* $(,)?) => { $(pub unsafe fn $name($(_: $ty),*) -> bool { false })* }; }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
zero_i32!(
    hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, elid: i32),
    hdac_bus_eml_set_syncprd_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, syncprd: u32),
    hdac_bus_eml_sdw_set_syncprd_unlocked(bus: *mut hdac_bus, syncprd: u32),
    hdac_bus_eml_wait_syncpu_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32),
    hdac_bus_eml_sdw_wait_syncpu_unlocked(bus: *mut hdac_bus),
    hdac_bus_eml_sync_go_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32),
    hdac_bus_eml_sdw_sync_go_unlocked(bus: *mut hdac_bus),
    hdac_bus_eml_power_up(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32),
    hdac_bus_eml_power_up_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32),
    hdac_bus_eml_power_down(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32),
    hdac_bus_eml_power_down_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32),
    hdac_bus_eml_sdw_power_up_unlocked(bus: *mut hdac_bus, sublink: i32),
    hdac_bus_eml_sdw_power_down_unlocked(bus: *mut hdac_bus, sublink: i32),
    hdac_bus_eml_sdw_get_lsdiid_unlocked(bus: *mut hdac_bus, sublink: i32, lsdiid: *mut u16),
    hdac_bus_eml_sdw_set_lsdiid(bus: *mut hdac_bus, sublink: i32, dev_num: i32),
    hdac_bus_eml_sdw_map_stream_ch(bus: *mut hdac_bus, sublink: i32, y: i32, channel_mask: i32, stream_id: i32, dir: i32),
    hda_bus_ml_resume(bus: *mut hdac_bus), hda_bus_ml_suspend(bus: *mut hdac_bus)
);
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_enable_interrupt_unlocked(_: *mut hdac_bus, _: bool, _: i32, _: bool) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_enable_interrupt(_: *mut hdac_bus, _: bool, _: i32, _: bool) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_sync_arm_unlocked(_: *mut hdac_bus, _: bool, _: i32, _: i32) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_sdw_sync_arm_unlocked(_: *mut hdac_bus, _: i32) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hda_bus_ml_reset_losidv(_: *mut hdac_bus) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_enable_offload(_: *mut hdac_bus, _: bool, _: i32, _: bool) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_set_mic_privacy_mask(_: *mut hdac_bus, _: bool, _: i32, _: c_ulong) {}
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hda_bus_ml_link_get_type(_: *mut hdac_ext_link) -> hda_bus_ml_link_type { hda_bus_ml_link_type::HDA_BUS_ML_LINK_HDA }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_ssp_get_hlink(_: *mut hdac_bus) -> *mut hdac_ext_link { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_dmic_get_hlink(_: *mut hdac_bus) -> *mut hdac_ext_link { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_sdw_get_hlink(_: *mut hdac_bus) -> *mut hdac_ext_link { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_get_mutex(_: *mut hdac_bus, _: bool, _: i32) -> *mut mutex { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_check_interrupt(_: *mut hdac_bus, _: bool, _: i32) -> bool { false }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_check_cmdsync_unlocked(_: *mut hdac_bus, _: bool, _: i32) -> bool { false }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_sdw_check_cmdsync_unlocked(_: *mut hdac_bus) -> bool { false }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_is_mic_privacy_changed(_: *mut hdac_bus, _: bool, _: i32) -> bool { false }
#[cfg(not(feature = "CONFIG_SND_SOC_SOF_HDA_MLINK"))]
pub unsafe fn hdac_bus_eml_get_mic_privacy_state(_: *mut hdac_bus, _: bool, _: i32) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
