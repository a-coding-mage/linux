/* SPDX-License-Identifier: GPL-2.0
 *
 * soc-card.h
 *
 * Copyright (C) 2019 Renesas Electronics Corp.
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

// The declarations below depend on types, constants, and functions supplied
// by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum snd_soc_card_subclass {
    SND_SOC_CARD_CLASS_ROOT = 0,
    SND_SOC_CARD_CLASS_RUNTIME = 1,
}

#[inline]
pub unsafe fn snd_soc_card_mutex_lock_root(card: *mut snd_soc_card) {
    mutex_lock_nested(&mut (*card).mutex, snd_soc_card_subclass::SND_SOC_CARD_CLASS_ROOT as _);
}

#[inline]
pub unsafe fn snd_soc_card_mutex_lock(card: *mut snd_soc_card) {
    mutex_lock_nested(&mut (*card).mutex, snd_soc_card_subclass::SND_SOC_CARD_CLASS_RUNTIME as _);
}

#[inline]
pub unsafe fn snd_soc_card_mutex_unlock(card: *mut snd_soc_card) {
    mutex_unlock(&mut (*card).mutex);
}

extern "C" {
    pub fn snd_soc_card_get_kcontrol(
        soc_card: *mut snd_soc_card,
        name: *const ::std::os::raw::c_char,
    ) -> *mut snd_kcontrol;
    pub fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        jack: *mut snd_soc_jack,
    ) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;

    pub fn snd_soc_card_suspend_pre(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_suspend_post(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_resume_pre(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_resume_post(card: *mut snd_soc_card) -> ::std::os::raw::c_int;

    pub fn snd_soc_card_probe(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_late_probe(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_fixup_controls(card: *mut snd_soc_card);
    pub fn snd_soc_card_remove(card: *mut snd_soc_card) -> ::std::os::raw::c_int;

    pub fn snd_soc_card_set_topology_name(
        card: *mut snd_soc_card,
        preifx: *const ::std::os::raw::c_char,
    );
    pub fn snd_soc_card_set_bias_level(
        card: *mut snd_soc_card,
        dapm: *mut snd_soc_dapm_context,
        level: snd_soc_bias_level,
    ) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_set_bias_level_post(
        card: *mut snd_soc_card,
        dapm: *mut snd_soc_dapm_context,
        level: snd_soc_bias_level,
    ) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_add_dai_link(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> ::std::os::raw::c_int;
    pub fn snd_soc_card_remove_dai_link(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    );
}

// CONFIG_PCI controls whether these helpers access PCI subsystem fields.
#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn snd_soc_card_set_pci_ssid(
    card: *mut snd_soc_card,
    vendor: ::std::os::raw::c_ushort,
    device: ::std::os::raw::c_ushort,
) {
    (*card).pci_subsystem_vendor = vendor;
    (*card).pci_subsystem_device = device;
    (*card).pci_subsystem_set = true;
}

#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn snd_soc_card_get_pci_ssid(
    card: *mut snd_soc_card,
    vendor: *mut ::std::os::raw::c_ushort,
    device: *mut ::std::os::raw::c_ushort,
) -> ::std::os::raw::c_int {
    if !(*card).pci_subsystem_set {
        return -ENOENT;
    }
    *vendor = (*card).pci_subsystem_vendor;
    *device = (*card).pci_subsystem_device;
    0
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub unsafe fn snd_soc_card_set_pci_ssid(
    _card: *mut snd_soc_card,
    _vendor: ::std::os::raw::c_ushort,
    _device: ::std::os::raw::c_ushort,
) {}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub unsafe fn snd_soc_card_get_pci_ssid(
    _card: *mut snd_soc_card,
    _vendor: *mut ::std::os::raw::c_ushort,
    _device: *mut ::std::os::raw::c_ushort,
) -> ::std::os::raw::c_int {
    -ENOENT
}

/* device driver data */
#[inline]
pub unsafe fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut ::std::ffi::c_void) {
    (*card).drvdata = data;
}

#[inline]
pub unsafe fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut ::std::ffi::c_void {
    (*card).drvdata
}

#[inline]
pub unsafe fn snd_soc_card_get_codec_dai(
    card: *mut snd_soc_card,
    dai_name: *const ::std::os::raw::c_char,
) -> *mut snd_soc_dai {
    let mut rtd: *mut snd_soc_pcm_runtime;

    // for_each_card_rtds(card, rtd)
    for_each_card_rtds!(card, rtd) {
        if strcmp((*snd_soc_rtd_to_codec(rtd, 0)).name, dai_name) == 0 {
            return snd_soc_rtd_to_codec(rtd, 0);
        }
    }

    ::std::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
