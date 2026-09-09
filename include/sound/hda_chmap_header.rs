/* SPDX-License-Identifier: GPL-2.0 */
/*
 * For multichannel support
 */

// Dependencies supplied by the corresponding sound/pcm.h and sound/hdaudio.h
// headers are intentionally referenced but not defined here.

pub const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: i32 = 80;

#[repr(C)]
pub struct hdac_cea_channel_speaker_allocation {
    pub ca_index: ::core::ffi::c_int,
    pub speakers: [::core::ffi::c_int; 8],

    /* derived values, just for convenience */
    pub channels: ::core::ffi::c_int,
    pub spk_mask: ::core::ffi::c_int,
}

#[repr(C)]
pub struct hdac_chmap;

#[repr(C)]
pub struct hdac_chmap_ops {
    /*
     * Helpers for producing the channel map TLVs. These can be overridden
     * for devices that have non-standard mapping requirements.
     */
    pub chmap_cea_alloc_validate_get_type: Option<unsafe extern "C" fn(
        chmap: *mut hdac_chmap,
        cap: *mut hdac_cea_channel_speaker_allocation,
        channels: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub cea_alloc_to_tlv_chmap: Option<unsafe extern "C" fn(
        hchmap: *mut hdac_chmap,
        cap: *mut hdac_cea_channel_speaker_allocation,
        chmap: *mut ::core::ffi::c_uint,
        channels: ::core::ffi::c_int,
    )>,

    /* check that the user-given chmap is supported */
    pub chmap_validate: Option<unsafe extern "C" fn(
        hchmap: *mut hdac_chmap,
        ca: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        chmap: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int>,

    pub get_spk_alloc: Option<unsafe extern "C" fn(
        hdac: *mut hdac_device,
        pcm_idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,

    pub get_chmap: Option<unsafe extern "C" fn(
        hdac: *mut hdac_device,
        pcm_idx: ::core::ffi::c_int,
        chmap: *mut ::core::ffi::c_uchar,
    )>,
    pub set_chmap: Option<unsafe extern "C" fn(
        hdac: *mut hdac_device,
        pcm_idx: ::core::ffi::c_int,
        chmap: *mut ::core::ffi::c_uchar,
        prepared: ::core::ffi::c_int,
    )>,
    pub is_pcm_attached: Option<unsafe extern "C" fn(
        hdac: *mut hdac_device,
        pcm_idx: ::core::ffi::c_int,
    ) -> bool>,

    /* get and set channel assigned to each HDMI ASP (audio sample packet) slot */
    pub pin_get_slot_channel: Option<unsafe extern "C" fn(
        codec: *mut hdac_device,
        pin_nid: hda_nid_t,
        asp_slot: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub pin_set_slot_channel: Option<unsafe extern "C" fn(
        codec: *mut hdac_device,
        pin_nid: hda_nid_t,
        asp_slot: ::core::ffi::c_int,
        channel: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub set_channel_count: Option<unsafe extern "C" fn(
        codec: *mut hdac_device,
        cvt_nid: hda_nid_t,
        chs: ::core::ffi::c_int,
    )>,
}

#[repr(C)]
pub struct hdac_chmap {
    pub channels_max: ::core::ffi::c_uint, /* max over all cvts */
    pub ops: hdac_chmap_ops,
    pub hdac: *mut hdac_device,
}

unsafe extern "C" {
    pub fn snd_hdac_register_chmap_ops(hdac: *mut hdac_device, chmap: *mut hdac_chmap);
    pub fn snd_hdac_channel_allocation(
        hdac: *mut hdac_device,
        spk_alloc: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        chmap_set: bool,
        non_pcm: bool,
        map: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    pub fn snd_hdac_get_active_channels(ca: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_hdac_setup_channel_mapping(
        chmap: *mut hdac_chmap,
        pin_nid: hda_nid_t,
        non_pcm: bool,
        ca: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        map: *mut ::core::ffi::c_uchar,
        chmap_set: bool,
    );
    pub fn snd_hdac_print_channel_allocation(
        spk_alloc: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_char,
        buflen: ::core::ffi::c_int,
    );
    pub fn snd_hdac_get_ch_alloc_from_ca(
        ca: ::core::ffi::c_int,
    ) -> *mut hdac_cea_channel_speaker_allocation;
    pub fn snd_hdac_chmap_to_spk_mask(c: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    pub fn snd_hdac_spk_to_chmap(spk: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_hdac_add_chmap_ctls(
        pcm: *mut snd_pcm,
        pcm_idx: ::core::ffi::c_int,
        chmap: *mut hdac_chmap,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
