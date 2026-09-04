// SPDX-License-Identifier: GPL-2.0-or-later
//
// Special handling for implicit feedback mode
//
// Dependencies:
// - linux/init.h, linux/usb.h, linux/usb/audio.h, linux/usb/audio-v2.h
// - sound/core.h, sound/pcm.h, sound/pcm_params.h
// - usbaudio.h, card.h, helper.h, pcm.h, implicit.h

const IMPLICIT_FB_NONE: i32 = 0;
const IMPLICIT_FB_GENERIC: i32 = 1;
const IMPLICIT_FB_FIXED: i32 = 2;
const IMPLICIT_FB_BOTH: i32 = 3; // generic playback + capture (for BOSS)

#[repr(C)]
pub struct snd_usb_implicit_fb_match {
    pub id: u32,
    pub iface_class: u32,
    pub ep_num: u32,
    pub iface: u32,
    pub type_: i32,
}

// Macro translations for device matching
// IMPLICIT_FB_GENERIC_DEV(vend, prod)
// IMPLICIT_FB_FIXED_DEV(vend, prod, ep, ifnum)
// IMPLICIT_FB_BOTH_DEV(vend, prod, ep, ifnum)
// IMPLICIT_FB_SKIP_DEV(vend, prod)

// Note: USB_ID() is a macro that would need to be defined in the dependencies

// Implicit feedback quirk table for playback
static PLAYBACK_IMPLICIT_FB_QUIRKS: &[snd_usb_implicit_fb_match] = &[
    // Fixed EP
    // FIXME: check the availability of generic matching
    snd_usb_implicit_fb_match { id: usb_id(0x0763, 0x2030), iface_class: 0, ep_num: 0x81, iface: 3, type_: IMPLICIT_FB_FIXED }, // M-Audio Fast Track C400
    snd_usb_implicit_fb_match { id: usb_id(0x0763, 0x2031), iface_class: 0, ep_num: 0x81, iface: 3, type_: IMPLICIT_FB_FIXED }, // M-Audio Fast Track C600
    snd_usb_implicit_fb_match { id: usb_id(0x0763, 0x2080), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // M-Audio FastTrack Ultra
    snd_usb_implicit_fb_match { id: usb_id(0x0763, 0x2081), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // M-Audio FastTrack Ultra
    snd_usb_implicit_fb_match { id: usb_id(0x2466, 0x8010), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // Fractal Audio Axe-Fx III
    snd_usb_implicit_fb_match { id: usb_id(0x31e9, 0x0001), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // Solid State Logic SSL2
    snd_usb_implicit_fb_match { id: usb_id(0x31e9, 0x0002), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // Solid State Logic SSL2+
    snd_usb_implicit_fb_match { id: usb_id(0x0499, 0x172f), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // Steinberg UR22C
    snd_usb_implicit_fb_match { id: usb_id(0x0d9a, 0x00df), iface_class: 0, ep_num: 0x81, iface: 2, type_: IMPLICIT_FB_FIXED }, // RTX6001
    snd_usb_implicit_fb_match { id: usb_id(0x19f7, 0x000a), iface_class: 0, ep_num: 0x84, iface: 3, type_: IMPLICIT_FB_FIXED }, // RODE AI-1
    snd_usb_implicit_fb_match { id: usb_id(0x22f0, 0x0006), iface_class: 0, ep_num: 0x81, iface: 3, type_: IMPLICIT_FB_FIXED }, // Allen&Heath Qu-16
    snd_usb_implicit_fb_match { id: usb_id(0x1686, 0xf029), iface_class: 0, ep_num: 0x82, iface: 2, type_: IMPLICIT_FB_FIXED }, // Zoom UAC-2
    snd_usb_implicit_fb_match { id: usb_id(0x2466, 0x8003), iface_class: 0, ep_num: 0x86, iface: 2, type_: IMPLICIT_FB_FIXED }, // Fractal Audio Axe-Fx II
    snd_usb_implicit_fb_match { id: usb_id(0x0499, 0x172a), iface_class: 0, ep_num: 0x86, iface: 2, type_: IMPLICIT_FB_FIXED }, // Yamaha MODX

    // Special matching
    snd_usb_implicit_fb_match { id: usb_id(0x07fd, 0x0004), iface_class: 0x01, ep_num: 0, iface: 0, type_: IMPLICIT_FB_NONE }, // MicroBook IIc
    // ep = 0x84, ifnum = 0
    snd_usb_implicit_fb_match { id: usb_id(0x07fd, 0x0004), iface_class: 0xff, ep_num: 0x84, iface: 0, type_: IMPLICIT_FB_FIXED }, // MOTU MicroBook II
];

// Implicit feedback quirk table for capture: only FIXED type
static CAPTURE_IMPLICIT_FB_QUIRKS: &[snd_usb_implicit_fb_match] = &[];

// Helper to construct USB ID (equivalent to USB_ID macro)
#[inline]
const fn usb_id(vend: u32, prod: u32) -> u32 {
    (vend << 16) | prod
}

// External dependencies
extern "C" {
    pub type snd_usb_audio;
    pub type audioformat;
    pub type usb_host_interface;
    pub type usb_interface;
    pub type usb_endpoint_descriptor;
    pub type snd_usb_stream;
    pub type snd_usb_substream;
    pub type snd_pcm_hw_params;
    pub type snd_pcm_format_t;

    fn usb_ifnum_to_if(dev: *mut core::ffi::c_void, ifnum: u32) -> *mut usb_interface;
    fn usb_audio_dbg(chip: *mut snd_usb_audio, fmt: *const u8, ...);
    fn snd_usb_get_host_interface(
        chip: *mut snd_usb_audio,
        ifnum: u32,
        altsetting: u32,
    ) -> *mut usb_host_interface;
    fn get_endpoint(alts: *mut usb_host_interface, ep_num: i32) -> *mut usb_endpoint_descriptor;
    fn usb_endpoint_is_isoc_in(epd: *const usb_endpoint_descriptor) -> bool;
    fn usb_endpoint_is_isoc_out(epd: *const usb_endpoint_descriptor) -> bool;
    fn pcm_format_to_bits(pcm_format: snd_pcm_format_t) -> u64;
    fn snd_usb_pcm_has_fixed_rate(subs: *mut snd_usb_substream) -> bool;
    fn params_rate(params: *const snd_pcm_hw_params) -> i32;
    fn params_channels(params: *const snd_pcm_hw_params) -> i32;
    fn params_format(params: *const snd_pcm_hw_params) -> snd_pcm_format_t;
}

const USB_CLASS_AUDIO: u32 = 0x01;
const USB_CLASS_VENDOR_SPEC: u32 = 0xff;
const USB_SUBCLASS_AUDIOSTREAMING: u32 = 0x02;
const UAC_VERSION_2: u32 = 0x20;
const USB_ENDPOINT_SYNCTYPE: u8 = 0x0c;
const USB_ENDPOINT_SYNC_ASYNC: u8 = 0x04;
const USB_ENDPOINT_USAGE_MASK: u8 = 0x30;
const USB_ENDPOINT_USAGE_DATA: u8 = 0x00;
const USB_ENDPOINT_USAGE_IMPLICIT_FB: u8 = 0x10;
const USB_DIR_IN: u8 = 0x80;
const QUIRK_FLAG_PLAYBACK_FIRST: u32 = 0x01;
const QUIRK_FLAG_SKIP_IMPLICIT_FB: u32 = 0x02;
const QUIRK_FLAG_GENERIC_IMPLICIT_FB: u32 = 0x04;

// set up sync EP information on the audioformat
unsafe fn add_implicit_fb_sync_ep(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    ep: u8,
    ep_idx: i32,
    ifnum: u32,
    alts: *const usb_host_interface,
) -> i32 {
    let mut iface: *mut usb_interface = core::ptr::null_mut();

    if alts.is_null() {
        iface = usb_ifnum_to_if(chip as *mut core::ffi::c_void, ifnum);
        if iface.is_null() || (*iface).num_altsetting < 2 {
            return 0;
        }
        alts = &(*iface).altsetting[1];
    }

    (*fmt).sync_ep = ep;
    (*fmt).sync_iface = ifnum;
    (*fmt).sync_altsetting = (*(*alts).desc).bAlternateSetting;
    (*fmt).sync_ep_idx = ep_idx;
    (*fmt).implicit_fb = 1;
    usb_audio_dbg(
        chip,
        b"%d:%d: added %s implicit_fb sync_ep %x, iface %d:%d\n\0".as_ptr(),
        (*fmt).iface,
        (*fmt).altsetting,
        if (ep & USB_DIR_IN) != 0 {
            b"playback\0".as_ptr()
        } else {
            b"capture\0".as_ptr()
        },
        (*fmt).sync_ep,
        (*fmt).sync_iface,
        (*fmt).sync_altsetting,
    );
    1
}

// Check whether the given UAC2 iface:altset points to an implicit fb source
unsafe fn add_generic_uac2_implicit_fb(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    ifnum: u32,
    altsetting: u32,
) -> i32 {
    let alts = snd_usb_get_host_interface(chip, ifnum, altsetting);
    if alts.is_null() {
        return 0;
    }

    if (*(*alts).desc).bInterfaceClass != USB_CLASS_AUDIO as u8
        || (*(*alts).desc).bInterfaceSubClass != USB_SUBCLASS_AUDIOSTREAMING as u8
        || (*(*alts).desc).bInterfaceProtocol != UAC_VERSION_2 as u8
        || (*(*alts).desc).bNumEndpoints < 1
    {
        return 0;
    }

    let epd = get_endpoint(alts, 0);
    if !usb_endpoint_is_isoc_in(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_USAGE_MASK) != USB_ENDPOINT_USAGE_IMPLICIT_FB
    {
        return 0;
    }

    add_implicit_fb_sync_ep(chip, fmt, (*epd).bEndpointAddress, 0, ifnum, alts)
}

unsafe fn roland_sanity_check_iface(alts: *const usb_host_interface) -> bool {
    if (*(*alts).desc).bInterfaceClass != USB_CLASS_VENDOR_SPEC as u8
        || ((*(*alts).desc).bInterfaceSubClass != 2
            && (*(*alts).desc).bInterfaceProtocol != 2)
        || (*(*alts).desc).bNumEndpoints < 1
    {
        return false;
    }
    true
}

// Like the UAC2 case above, but specific to Roland with vendor class and hack
unsafe fn add_roland_implicit_fb(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *const usb_host_interface,
) -> i32 {
    if !roland_sanity_check_iface(alts) {
        return 0;
    }

    // only when both streams are with ASYNC type
    let epd = get_endpoint(alts as *mut usb_host_interface, 0);
    if !usb_endpoint_is_isoc_out(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
    {
        return 0;
    }

    // check capture EP
    let alts_capture = snd_usb_get_host_interface(
        chip,
        (*(*alts).desc).bInterfaceNumber + 1,
        (*(*alts).desc).bAlternateSetting,
    );
    if alts_capture.is_null() || !roland_sanity_check_iface(alts_capture) {
        return 0;
    }

    let epd_capture = get_endpoint(alts_capture, 0);
    if !usb_endpoint_is_isoc_in(epd_capture)
        || ((*epd_capture).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
    {
        return 0;
    }

    (*chip).quirk_flags |= QUIRK_FLAG_PLAYBACK_FIRST;
    add_implicit_fb_sync_ep(
        chip,
        fmt,
        (*epd_capture).bEndpointAddress,
        0,
        (*(*alts_capture).desc).bInterfaceNumber,
        alts_capture,
    )
}

// capture quirk for Roland device; always full-duplex
unsafe fn add_roland_capture_quirk(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *const usb_host_interface,
) -> i32 {
    if !roland_sanity_check_iface(alts) {
        return 0;
    }

    let epd = get_endpoint(alts as *mut usb_host_interface, 0);
    if !usb_endpoint_is_isoc_in(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
    {
        return 0;
    }

    let alts_playback = snd_usb_get_host_interface(
        chip,
        (*(*alts).desc).bInterfaceNumber - 1,
        (*(*alts).desc).bAlternateSetting,
    );
    if alts_playback.is_null() || !roland_sanity_check_iface(alts_playback) {
        return 0;
    }

    let epd_playback = get_endpoint(alts_playback, 0);
    if !usb_endpoint_is_isoc_out(epd_playback) {
        return 0;
    }

    add_implicit_fb_sync_ep(
        chip,
        fmt,
        (*epd_playback).bEndpointAddress,
        0,
        (*(*alts_playback).desc).bInterfaceNumber,
        alts_playback,
    )
}

// Playback and capture EPs on Pioneer devices share the same iface/altset
// for the implicit feedback operation
unsafe fn is_pioneer_implicit_fb(chip: *mut snd_usb_audio, alts: *const usb_host_interface) -> bool {
    let vendor = ((*chip).usb_id >> 16) as u32;

    if vendor != 0x2b73 && vendor != 0x08e4 {
        return false;
    }
    if (*(*alts).desc).bInterfaceClass != USB_CLASS_VENDOR_SPEC as u8 {
        return false;
    }
    if (*(*alts).desc).bNumEndpoints != 2 {
        return false;
    }

    let epd = get_endpoint(alts as *mut usb_host_interface, 0);
    if !usb_endpoint_is_isoc_out(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
    {
        return false;
    }

    let epd = get_endpoint(alts as *mut usb_host_interface, 1);
    if !usb_endpoint_is_isoc_in(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
        || (((*epd).bmAttributes & USB_ENDPOINT_USAGE_MASK) != USB_ENDPOINT_USAGE_DATA
            && ((*epd).bmAttributes & USB_ENDPOINT_USAGE_MASK) != USB_ENDPOINT_USAGE_IMPLICIT_FB)
    {
        return false;
    }

    true
}

unsafe fn __add_generic_implicit_fb(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    iface: u32,
    altset: u32,
) -> i32 {
    let alts = snd_usb_get_host_interface(chip, iface, altset);
    if alts.is_null() {
        return 0;
    }

    if ((*(*alts).desc).bInterfaceClass != USB_CLASS_VENDOR_SPEC as u8
        && (*(*alts).desc).bInterfaceClass != USB_CLASS_AUDIO as u8)
        || (*(*alts).desc).bNumEndpoints < 1
    {
        return 0;
    }

    let epd = get_endpoint(alts, 0);
    if !usb_endpoint_is_isoc_in(epd)
        || ((*epd).bmAttributes & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC
    {
        return 0;
    }

    add_implicit_fb_sync_ep(chip, fmt, (*epd).bEndpointAddress, 0, iface, alts)
}

// More generic quirk: look for the sync EP next to the data EP
unsafe fn add_generic_implicit_fb(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *const usb_host_interface,
) -> i32 {
    if ((*fmt).ep_attr & USB_ENDPOINT_SYNCTYPE) != USB_ENDPOINT_SYNC_ASYNC {
        return 0;
    }

    if __add_generic_implicit_fb(
        chip,
        fmt,
        (*(*alts).desc).bInterfaceNumber + 1,
        (*(*alts).desc).bAlternateSetting,
    ) != 0
    {
        return 1;
    }

    __add_generic_implicit_fb(
        chip,
        fmt,
        (*(*alts).desc).bInterfaceNumber - 1,
        (*(*alts).desc).bAlternateSetting,
    )
}

unsafe fn find_implicit_fb_entry(
    chip: *mut snd_usb_audio,
    mut match_: *const snd_usb_implicit_fb_match,
    alts: *const usb_host_interface,
) -> *const snd_usb_implicit_fb_match {
    while (*match_).id != 0 {
        if (*match_).id == (*chip).usb_id
            && ((*match_).iface_class == 0
                || ((*(*alts).desc).bInterfaceClass == (*match_).iface_class as u8))
        {
            return match_;
        }
        match_ = match_.offset(1);
    }

    core::ptr::null()
}

// Setup an implicit feedback endpoint from a quirk. Returns 0 if no quirk
// applies. Returns 1 if a quirk was found.
unsafe fn audioformat_implicit_fb_quirk(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *const usb_host_interface,
) -> i32 {
    let attr = (*fmt).ep_attr & USB_ENDPOINT_SYNCTYPE;

    let p = find_implicit_fb_entry(chip, PLAYBACK_IMPLICIT_FB_QUIRKS.as_ptr(), alts);
    if !p.is_null() {
        match (*p).type_ {
            IMPLICIT_FB_GENERIC => return add_generic_implicit_fb(chip, fmt, alts),
            IMPLICIT_FB_NONE => return 0,
            IMPLICIT_FB_FIXED => {
                return add_implicit_fb_sync_ep(chip, fmt, (*p).ep_num as u8, 0, (*p).iface, core::ptr::null());
            }
            _ => {}
        }
    }

    // Special handling for devices with capture quirks
    let p = find_implicit_fb_entry(chip, CAPTURE_IMPLICIT_FB_QUIRKS.as_ptr(), alts);
    if !p.is_null() {
        match (*p).type_ {
            IMPLICIT_FB_FIXED => return 0,
            IMPLICIT_FB_BOTH => {
                (*chip).quirk_flags |= QUIRK_FLAG_PLAYBACK_FIRST;
                return add_generic_implicit_fb(chip, fmt, alts);
            }
            _ => {}
        }
    }

    // Generic UAC2 implicit feedback
    if attr == USB_ENDPOINT_SYNC_ASYNC
        && (*(*alts).desc).bInterfaceClass == USB_CLASS_AUDIO as u8
        && (*(*alts).desc).bInterfaceProtocol == UAC_VERSION_2 as u8
        && (*(*alts).desc).bNumEndpoints == 1
    {
        if add_generic_uac2_implicit_fb(
            chip,
            fmt,
            (*(*alts).desc).bInterfaceNumber + 1,
            (*(*alts).desc).bAlternateSetting,
        ) != 0
        {
            return 1;
        }
    }

    // Roland/BOSS implicit feedback with vendor spec class
    let vendor = ((*chip).usb_id >> 16) as u32;
    if vendor == 0x0582 {
        if add_roland_implicit_fb(chip, fmt, alts) > 0 {
            return 1;
        }
    }

    // Pioneer devices with vendor spec class
    if is_pioneer_implicit_fb(chip, alts) {
        (*chip).quirk_flags |= QUIRK_FLAG_PLAYBACK_FIRST;
        return add_implicit_fb_sync_ep(
            chip,
            fmt,
            (*get_endpoint(alts as *mut usb_host_interface, 1)).bEndpointAddress,
            1,
            (*(*alts).desc).bInterfaceNumber,
            alts,
        );
    }

    // Try the generic implicit fb if available
    if (*chip).generic_implicit_fb != 0
        || ((*chip).quirk_flags & QUIRK_FLAG_GENERIC_IMPLICIT_FB) != 0
    {
        return add_generic_implicit_fb(chip, fmt, alts);
    }

    // No quirk
    0
}

// same for capture, but only handling FIXED entry
unsafe fn audioformat_capture_quirk(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *const usb_host_interface,
) -> i32 {
    let p = find_implicit_fb_entry(chip, CAPTURE_IMPLICIT_FB_QUIRKS.as_ptr(), alts);
    if !p.is_null()
        && ((*p).type_ == IMPLICIT_FB_FIXED || (*p).type_ == IMPLICIT_FB_BOTH)
    {
        return add_implicit_fb_sync_ep(chip, fmt, (*p).ep_num as u8, 0, (*p).iface, core::ptr::null());
    }

    // Roland/BOSS need full-duplex streams
    let vendor = ((*chip).usb_id >> 16) as u32;
    if vendor == 0x0582 {
        if add_roland_capture_quirk(chip, fmt, alts) > 0 {
            return 1;
        }
    }

    if is_pioneer_implicit_fb(chip, alts) {
        return 1; // skip the quirk, also don't handle generic sync EP
    }

    0
}

// Parse altset and set up implicit feedback endpoint on the audioformat
pub unsafe extern "C" fn snd_usb_parse_implicit_fb_quirk(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
    alts: *mut usb_host_interface,
) -> i32 {
    if ((*chip).quirk_flags & QUIRK_FLAG_SKIP_IMPLICIT_FB) != 0 {
        return 0;
    }
    if ((*fmt).endpoint & USB_DIR_IN) != 0 {
        audioformat_capture_quirk(chip, fmt, alts)
    } else {
        audioformat_implicit_fb_quirk(chip, fmt, alts)
    }
}

// Return the score of matching two audioformats.
// Veto the audioformat if:
// - It has no channels for some reason.
// - Requested PCM format is not supported.
// - Requested sample rate is not supported.
unsafe fn match_endpoint_audioformats(
    _subs: *mut snd_usb_substream,
    fp: *const audioformat,
    rate: i32,
    channels: i32,
    pcm_format: snd_pcm_format_t,
) -> i32 {
    if (*fp).channels < 1 {
        return 0;
    }

    if ((*fp).formats & pcm_format_to_bits(pcm_format)) == 0 {
        return 0;
    }

    if ((*fp).rates & 0x0000000f) != 0 {
        if rate < (*fp).rate_min || rate > (*fp).rate_max {
            return 0;
        }
    } else {
        let mut i = 0;
        while i < (*fp).nr_rates {
            if *(*fp).rate_table.offset(i as isize) == rate {
                break;
            }
            i += 1;
        }
        if i >= (*fp).nr_rates {
            return 0;
        }
    }

    let mut score = 1;
    if (*fp).channels == channels {
        score += 1;
    }

    score
}

unsafe fn find_matching_substream(
    chip: *mut snd_usb_audio,
    stream: i32,
    ep_num: u8,
    fmt_type: i32,
) -> *mut snd_usb_substream {
    let mut as_ = (*chip).pcm_list.next;
    while as_ != &(*chip).pcm_list as *const _ as *mut _ {
        let as_entry = as_ as *mut snd_usb_stream;
        let subs = &mut (*as_entry).substream[stream as usize];
        if (*as_entry).fmt_type == fmt_type && (*subs).ep_num == ep_num {
            return subs;
        }
        as_ = (*as_).next;
    }

    core::ptr::null_mut()
}

// Return the audioformat that is suitable for the implicit fb
pub unsafe extern "C" fn snd_usb_find_implicit_fb_sync_format(
    chip: *mut snd_usb_audio,
    target: *const audioformat,
    params: *const snd_pcm_hw_params,
    stream: i32,
    fixed_rate: *mut bool,
) -> *const audioformat {
    let mut sync_fmt: *const audioformat = core::ptr::null();

    // Use the original audioformat as fallback for the shared altset
    if (*target).iface == (*target).sync_iface
        && (*target).altsetting == (*target).sync_altsetting
    {
        sync_fmt = target;
    }

    let subs = find_matching_substream(chip, stream, (*target).sync_ep, (*target).fmt_type);
    if subs.is_null() {
        if !fixed_rate.is_null() {
            *fixed_rate = false;
        }
        return sync_fmt;
    }

    let mut high_score = 0;
    let mut fp = (*subs).fmt_list.next;
    while fp != &(*subs).fmt_list as *const _ as *mut _ {
        let fp_entry = fp as *mut audioformat;
        let score = match_endpoint_audioformats(
            subs,
            fp_entry,
            params_rate(params),
            params_channels(params),
            params_format(params),
        );
        if score > high_score {
            sync_fmt = fp_entry as *const _;
            high_score = score;
        }
        fp = (*fp).next;
    }

    if !fixed_rate.is_null() {
        *fixed_rate = snd_usb_pcm_has_fixed_rate(subs);
    }
    sync_fmt
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
