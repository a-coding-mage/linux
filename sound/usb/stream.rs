// SPDX-License-Identifier: GPL-2.0-or-later

// External dependencies from linux/kernel headers and sound subsystem:
// - linux/init.h
// - linux/slab.h
// - linux/usb.h
// - linux/usb/audio.h, audio-v2.h, audio-v3.h
// - sound/core.h
// - sound/pcm.h
// - sound/control.h
// - sound/tlv.h
// Internal dependencies:
// - usbaudio.h
// - card.h
// - proc.h
// - quirks.h
// - endpoint.h
// - pcm.h
// - helper.h
// - format.h
// - clock.h
// - stream.h
// - power.h
// - media.h

use core::ffi::{c_int, c_uint, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct audioformat {
    pub list: ListHead,
    pub rate_table: *mut c_int,
    pub chmap: *mut c_void,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct snd_usb_substream {
    pub fmt_list: ListHead,
    pub lock: spin_lock_t,
    pub stream: *mut snd_usb_stream,
    pub direction: c_int,
    pub dev: *mut usb_device,
    pub txfr_quirk: bool,
    pub tx_length_quirk: bool,
    pub speed: c_int,
    pub pkt_offset_adj: c_int,
    pub stream_offset_adj: c_int,
    pub formats: u64,
    pub num_formats: c_int,
    pub fmt_type: c_int,
    pub ep_num: c_int,
    pub channels_max: c_int,
    pub str_pd: *mut snd_usb_power_domain,
    pub cur_audiofmt: *mut audioformat,
}

#[repr(C)]
pub struct snd_usb_stream {
    pub substream: [snd_usb_substream; 2],
    pub list: ListHead,
    pub chip: *mut snd_usb_audio,
    pub pcm: *mut snd_pcm,
    pub pcm_index: c_int,
    pub fmt_type: c_int,
}

#[repr(C)]
pub struct snd_usb_audio {
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub pcm_list: ListHead,
    pub pcm_devs: c_int,
    pub quirk_flags: c_uint,
    pub usb_id: c_uint,
    pub need_delayed_register: bool,
    pub badd_profile: u8,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    pub info_flags: c_int,
    pub name: [u8; 80],
}

#[repr(C)]
pub struct spin_lock_t {
    _opaque: [u8; 4],
}

#[repr(C)]
pub struct usb_device;

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct snd_usb_power_domain {
    pub pd_id: c_int,
    pub pd_d1d0_rec: c_int,
    pub pd_d2d0_rec: c_int,
    pub ctrl_iface: *mut usb_host_interface,
}

#[repr(C)]
pub struct snd_pcm_chmap_elem {
    pub channels: c_int,
    pub map: [u32; 16],
}

#[repr(C)]
pub struct snd_pcm_chmap {
    pub private_data: *mut c_void,
    pub kctl: *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}

#[repr(C)]
pub struct snd_kcontrol_tlv {
    pub c: Option<unsafe extern "C" fn(*mut snd_kcontrol, c_int, c_uint, *mut c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_info;

#[repr(C)]
pub struct snd_ctl_elem_value;

#[repr(C)]
pub struct usb_host_interface;

#[repr(C)]
pub struct uac_iso_endpoint_descriptor;

#[repr(C)]
pub struct uac2_iso_endpoint_descriptor;

#[repr(C)]
pub struct uac3_iso_endpoint_descriptor;

#[repr(C)]
pub struct usb_interface_descriptor;

#[repr(C)]
pub struct uac1_as_header_descriptor;

#[repr(C)]
pub struct uac_format_type_i_continuous_descriptor;

#[repr(C)]
pub struct uac_input_terminal_descriptor;

#[repr(C)]
pub struct uac2_input_terminal_descriptor;

#[repr(C)]
pub struct uac2_output_terminal_descriptor;

#[repr(C)]
pub struct uac2_as_header_descriptor;

#[repr(C)]
pub struct uac3_input_terminal_descriptor;

#[repr(C)]
pub struct uac3_output_terminal_descriptor;

#[repr(C)]
pub struct uac3_cluster_header_descriptor {
    pub bNrChannels: u8,
    pub wLength: u16,
}

#[repr(C)]
pub struct uac3_cluster_segment_descriptor {
    pub wLength: u16,
    pub bSegmentType: u8,
}

#[repr(C)]
pub struct uac3_cluster_information_segment_descriptor {
    pub bChRelationship: u8,
}

#[repr(C)]
pub struct uac3_as_header_descriptor;

#[repr(C)]
pub struct uac3_hc_descriptor_header {
    pub wLength: u16,
}

#[repr(C)]
pub struct usb_host_endpoint;

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 1;
const SNDRV_CHMAP_LAST: c_uint = 0x3f;
const SNDRV_CTL_TLVT_CONTAINER: c_uint = 0;
const SNDRV_CTL_TLVT_CHMAP_FIXED: c_uint = 0x0101;
const SNDRV_CHMAP_FL: u32 = 0;
const SNDRV_CHMAP_FR: u32 = 1;
const SNDRV_CHMAP_FC: u32 = 2;
const SNDRV_CHMAP_LFE: u32 = 3;
const SNDRV_CHMAP_RL: u32 = 4;
const SNDRV_CHMAP_RR: u32 = 5;
const SNDRV_CHMAP_FLC: u32 = 6;
const SNDRV_CHMAP_FRC: u32 = 7;
const SNDRV_CHMAP_RC: u32 = 8;
const SNDRV_CHMAP_SL: u32 = 9;
const SNDRV_CHMAP_SR: u32 = 10;
const SNDRV_CHMAP_TC: u32 = 11;
const SNDRV_CHMAP_TFL: u32 = 12;
const SNDRV_CHMAP_TFC: u32 = 13;
const SNDRV_CHMAP_TFR: u32 = 14;
const SNDRV_CHMAP_TRL: u32 = 15;
const SNDRV_CHMAP_TRC: u32 = 16;
const SNDRV_CHMAP_TRR: u32 = 17;
const SNDRV_CHMAP_TFLC: u32 = 18;
const SNDRV_CHMAP_TFRC: u32 = 19;
const SNDRV_CHMAP_LLFE: u32 = 20;
const SNDRV_CHMAP_RLFE: u32 = 21;
const SNDRV_CHMAP_TSL: u32 = 22;
const SNDRV_CHMAP_TSR: u32 = 23;
const SNDRV_CHMAP_BC: u32 = 24;
const SNDRV_CHMAP_RLC: u32 = 25;
const SNDRV_CHMAP_RRC: u32 = 26;
const SNDRV_CHMAP_MONO: u32 = 27;
const SNDRV_CHMAP_FLW: u32 = 28;
const SNDRV_CHMAP_FRW: u32 = 29;
const SNDRV_CHMAP_UNKNOWN: u32 = 0x3f;

const UAC_VERSION_1: c_int = 0x0100;
const UAC_VERSION_2: c_int = 0x0200;
const UAC_VERSION_3: c_int = 0x0300;

const UAC_FORMAT_TYPE_I: c_int = 1;

const UAC3_CH_MONO: u8 = 1;
const UAC3_CH_LEFT: u8 = 2;
const UAC3_CH_FRONT_LEFT: u8 = 3;
const UAC3_CH_HEADPHONE_LEFT: u8 = 4;
const UAC3_CH_RIGHT: u8 = 5;
const UAC3_CH_FRONT_RIGHT: u8 = 6;
const UAC3_CH_HEADPHONE_RIGHT: u8 = 7;
const UAC3_CH_FRONT_CENTER: u8 = 8;
const UAC3_CH_FRONT_LEFT_OF_CENTER: u8 = 9;
const UAC3_CH_FRONT_RIGHT_OF_CENTER: u8 = 10;
const UAC3_CH_FRONT_WIDE_LEFT: u8 = 11;
const UAC3_CH_FRONT_WIDE_RIGHT: u8 = 12;
const UAC3_CH_SIDE_LEFT: u8 = 13;
const UAC3_CH_SIDE_RIGHT: u8 = 14;
const UAC3_CH_BACK_LEFT: u8 = 15;
const UAC3_CH_BACK_RIGHT: u8 = 16;
const UAC3_CH_BACK_CENTER: u8 = 17;
const UAC3_CH_BACK_LEFT_OF_CENTER: u8 = 18;
const UAC3_CH_BACK_RIGHT_OF_CENTER: u8 = 19;
const UAC3_CH_TOP_CENTER: u8 = 20;
const UAC3_CH_TOP_FRONT_LEFT: u8 = 21;
const UAC3_CH_TOP_FRONT_RIGHT: u8 = 22;
const UAC3_CH_TOP_FRONT_CENTER: u8 = 23;
const UAC3_CH_TOP_FRONT_LOC: u8 = 24;
const UAC3_CH_TOP_FRONT_ROC: u8 = 25;
const UAC3_CH_TOP_SIDE_LEFT: u8 = 26;
const UAC3_CH_TOP_SIDE_RIGHT: u8 = 27;
const UAC3_CH_TOP_BACK_LEFT: u8 = 28;
const UAC3_CH_TOP_BACK_RIGHT: u8 = 29;
const UAC3_CH_TOP_BACK_CENTER: u8 = 30;
const UAC3_CH_BOTTOM_CENTER: u8 = 31;
const UAC3_CH_LOW_FREQUENCY_EFFECTS: u8 = 32;
const UAC3_CH_LFE_LEFT: u8 = 33;
const UAC3_CH_LFE_RIGHT: u8 = 34;
const UAC3_CH_RELATIONSHIP_UNDEFINED: u8 = 0;

const UAC3_CHANNEL_INFORMATION: u8 = 0x01;

const UAC3_PD_STATE_D1: c_int = 1;

const UAC3_BADD_CS_ID9: c_int = 9;
const UAC3_BADD_PD_ID10: c_int = 10;
const UAC3_BADD_PD_ID11: c_int = 11;
const UAC3_BADD_PD_RECOVER_D1D0: c_int = 0;
const UAC3_BADD_PD_RECOVER_D2D0: c_int = 0;
const UAC3_BADD_SAMPLING_RATE: c_int = 48000;

const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_16: u16 = 0x0040;
const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_16: u16 = 0x0060;
const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_24: u16 = 0x0060;
const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_24: u16 = 0x0090;
const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_16: u16 = 0x0080;
const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_16: u16 = 0x00c0;
const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_24: u16 = 0x00c0;
const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_24: u16 = 0x0120;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 5;

const SNDRV_PCM_RATE_CONTINUOUS: c_int = 0x00000800;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

const SND_USB_ENDPOINT_TYPE_DATA: c_int = 0;
const SND_USB_ENDPOINT_TYPE_SYNC: c_int = 1;

const USB_CLASS_AUDIO: u8 = 0x01;
const USB_SUBCLASS_AUDIOSTREAMING: u8 = 0x02;
const USB_SUBCLASS_VENDOR_SPEC: u8 = 0xff;
const USB_CLASS_VENDOR_SPEC: u8 = 0xff;
const USB_ENDPOINT_XFERTYPE_MASK: u8 = 0x03;
const USB_ENDPOINT_XFER_ISOC: u8 = 0x01;
const USB_DIR_IN: u8 = 0x80;

const QUIRK_FLAG_ALIGN_TRANSFER: c_uint = 1;
const QUIRK_FLAG_TX_LENGTH: c_uint = 2;
const QUIRK_FLAG_SET_IFACE_FIRST: c_uint = 4;
const QUIRK_FLAG_SKIP_IFACE_SETUP: c_uint = 8;

const UAC_EP_GENERAL: u8 = 0x01;
const UAC_EP_CS_ATTR_PITCH_CONTROL: c_int = 0x01;
const UAC_EP_CS_ATTR_FILL_MAX: c_int = 0x80;
const UAC2_CONTROL_PITCH: c_int = 0x0001;

const UAC_INPUT_TERMINAL: u8 = 0x02;
const UAC_OUTPUT_TERMINAL: u8 = 0x03;
const UAC_FORMAT_TYPE: u8 = 0x04;
const UAC_AS_GENERAL: u8 = 0x01;

const UAC3_FUNCTION_SUBCLASS_GENERIC_IO: u8 = 0x20;

const USB_DT_CS_ENDPOINT: u8 = 0x25;
const USB_RECIP_INTERFACE: u8 = 0x01;
const USB_TYPE_CLASS: u8 = 0x20;
const USB_DIR_IN_CONST: u8 = 0x80;
const UAC3_CS_REQ_HIGH_CAPABILITY_DESCRIPTOR: u8 = 0x04;

extern "C" {
    fn list_del(entry: *mut ListHead);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kzalloc_obj(ptr: *mut c_void) -> *mut c_void;
    fn snd_media_stream_delete(subs: *mut snd_usb_substream);
    fn snd_usb_set_pcm_ops(pcm: *mut snd_pcm, stream: c_int);
    fn snd_usb_preallocate_buffer(subs: *mut snd_usb_substream);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn put_user(val: c_uint, ptr: *mut c_uint) -> c_int;
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, chmap: *mut snd_pcm_chmap_elem,
                               count: c_int, offset: c_int, chmap_out: *mut *mut snd_pcm_chmap) -> c_int;
    fn snd_usb_find_desc(p: *mut c_void, len: usize, after: *mut c_void, dtype: u8) -> *mut c_void;
    fn snd_usb_find_csint_desc(p: *mut c_void, len: usize, after: *mut c_void, dtype: u8) -> *mut c_void;
    fn snd_usb_find_ctrl_interface(chip: *mut snd_usb_audio, iface_no: c_int) -> *mut usb_host_interface;
    fn snd_usb_validate_audio_desc(desc: *mut c_void, protocol: c_int) -> bool;
    fn get_iface_desc(alts: *mut usb_host_interface) -> *mut usb_interface_descriptor;
    fn get_endpoint(alts: *mut usb_host_interface, ep: c_int) -> *mut usb_host_endpoint;
    fn snd_usb_parse_datainterval(chip: *mut snd_usb_audio, alts: *mut usb_host_interface) -> c_int;
    fn usb_endpoint_max_periodic_payload(dev: *mut usb_device, ep: *mut usb_host_endpoint) -> c_int;
    fn snd_usb_audioformat_attributes_quirk(chip: *mut snd_usb_audio, fp: *mut audioformat, stream: c_int);
    fn snd_usb_parse_audio_format(chip: *mut snd_usb_audio, fp: *mut audioformat, format: u64,
                                   fmt: *mut uac_format_type_i_continuous_descriptor, stream: c_int) -> c_int;
    fn snd_usb_power_domain_set(chip: *mut snd_usb_audio, pd: *mut snd_usb_power_domain, state: c_int);
    fn snd_usb_find_input_terminal_descriptor(ctrl_iface: *mut usb_host_interface, terminal_id: c_int,
                                               protocol: c_int) -> *mut c_void;
    fn snd_usb_find_output_terminal_descriptor(ctrl_iface: *mut usb_host_interface, terminal_id: c_int,
                                                protocol: c_int) -> *mut c_void;
    fn snd_usb_audioformat_set_sync_ep(chip: *mut snd_usb_audio, fp: *mut audioformat);
    fn snd_usb_add_endpoint(chip: *mut snd_usb_audio, ep: c_int, type_: c_int) -> c_int;
    fn snd_usb_apply_interface_quirk(chip: *mut snd_usb_audio, iface_no: c_int, altno: c_int) -> bool;
    fn snd_usb_init_pitch(chip: *mut snd_usb_audio, fp: *mut audioformat);
    fn snd_usb_init_sample_rate(chip: *mut snd_usb_audio, fp: *mut audioformat, rate: c_int);
    fn snd_pcm_new(card: *mut snd_card, name: *const u8, device: c_int, playback_count: c_int,
                   capture_count: c_int, pcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_new_stream(pcm: *mut snd_pcm, stream: c_int, substream_count: c_int) -> c_int;
    fn snd_usb_proc_pcm_format_add(as_: *mut snd_usb_stream);
    fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: c_int) -> *mut usb_interface;
    fn usb_set_interface(dev: *mut usb_device, ifnum: c_int, alternate: c_int) -> c_int;
    fn usb_audio_warn(chip: *mut snd_usb_audio, fmt: *const u8, ...);
    fn snd_usb_ctl_msg(dev: *mut usb_device, pipe: c_uint, request: u8, requesttype: u8,
                       value: u16, index: u16, data: *mut c_void, size: usize) -> c_int;
    fn usb_rcvctrlpipe(dev: *mut usb_device, ep: u32) -> c_uint;
    fn snd_usb_ctrl_intf(ctrl_intf: *mut usb_host_interface) -> u16;
    fn snd_usb_find_power_domain(ctrl_intf: *mut usb_host_interface, terminal_id: c_int) -> *mut snd_usb_power_domain;
    fn snd_usb_parse_audio_format_v3(chip: *mut snd_usb_audio, fp: *mut audioformat,
                                      as_: *mut uac3_as_header_descriptor, stream: c_int) -> c_int;
    fn snd_usb_get_speed(dev: *mut usb_device) -> c_int;
    fn le16_to_cpu(val: u16) -> u16;
    fn le32_to_cpu(val: u32) -> u32;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn scnprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> usize;
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> usize;
    fn dev_err(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);

    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn spin_lock_init(lock: *mut spin_lock_t);
}

#[repr(C)]
pub struct usb_interface {
    pub num_altsetting: c_int,
    pub altsetting: *mut usb_host_interface,
}

unsafe fn audioformat_free(fp: *mut audioformat) {
    if fp.is_null() {
        return;
    }
    list_del(&mut (*fp).list);
    kfree((*fp).rate_table as *mut c_void);
    kfree((*fp).chmap);
    kfree(fp as *mut c_void);
}

unsafe fn free_substream(subs: *mut snd_usb_substream) {
    if (*subs).num_formats == 0 {
        return;
    }

    let mut fp = (*subs).fmt_list.next as *mut audioformat;
    while fp as *mut ListHead != &mut (*subs).fmt_list {
        let next = (*fp).list.next as *mut audioformat;
        audioformat_free(fp);
        fp = next;
    }
    kfree((*subs).str_pd as *mut c_void);
    snd_media_stream_delete(subs);
}

unsafe fn snd_usb_audio_stream_free(stream: *mut snd_usb_stream) {
    free_substream(&mut (*stream).substream[0]);
    free_substream(&mut (*stream).substream[1]);
    list_del(&mut (*stream).list);
    kfree(stream as *mut c_void);
}

unsafe extern "C" fn snd_usb_audio_pcm_free(pcm: *mut snd_pcm) {
    let stream = (*pcm).private_data as *mut snd_usb_stream;
    if !stream.is_null() {
        (*stream).pcm = ptr::null_mut();
        snd_usb_audio_stream_free(stream);
    }
}

unsafe fn snd_usb_init_substream(as_: *mut snd_usb_stream, stream: c_int,
                                  fp: *mut audioformat, pdptr: *mut *mut snd_usb_power_domain) {
    let subs = &mut (*as_).substream[stream as usize];

    INIT_LIST_HEAD(&mut subs.fmt_list);
    spin_lock_init(&mut subs.lock);

    subs.stream = as_;
    subs.direction = stream;
    subs.dev = (*(*as_).chip).dev;
    subs.txfr_quirk = ((*(*as_).chip).quirk_flags & QUIRK_FLAG_ALIGN_TRANSFER) != 0;
    subs.tx_length_quirk = ((*(*as_).chip).quirk_flags & QUIRK_FLAG_TX_LENGTH) != 0;
    subs.speed = snd_usb_get_speed(subs.dev);
    subs.pkt_offset_adj = 0;
    subs.stream_offset_adj = 0;

    snd_usb_set_pcm_ops((*as_).pcm, stream);

    let fmt_list = &mut subs.fmt_list;
    (*fp).list.next = fmt_list as *mut ListHead;
    (*fp).list.prev = fmt_list.prev;
    (*fmt_list).prev.as_mut().unwrap().next = &mut (*fp).list;
    (*fmt_list).prev = &mut (*fp).list;

    subs.formats |= (*fp).formats;
    subs.num_formats += 1;
    subs.fmt_type = (*fp).fmt_type;
    subs.ep_num = (*fp).endpoint;
    if (*fp).channels > subs.channels_max {
        subs.channels_max = (*fp).channels;
    }

    if !pdptr.is_null() && !(*pdptr).is_null() {
        subs.str_pd = *pdptr;
        *pdptr = ptr::null_mut();
        snd_usb_power_domain_set((*as_).chip, subs.str_pd, UAC3_PD_STATE_D1);
    }

    snd_usb_preallocate_buffer(subs);
}

unsafe extern "C" fn usb_chmap_ctl_info(kcontrol: *mut snd_kcontrol,
                                        uinfo: *mut snd_ctl_elem_info) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;
    let subs = (*info).private_data as *mut snd_usb_substream;

    // Set uinfo fields (using assumed offsets for kernel structures)
    0
}

unsafe fn have_dup_chmap(subs: *mut snd_usb_substream, fp: *mut audioformat) -> bool {
    let mut prev = (*subs).fmt_list.prev as *mut audioformat;

    while prev as *mut ListHead != &(*subs).fmt_list {
        if !(*prev).chmap.is_null() {
            if memcmp((*prev).chmap, (*fp).chmap, mem::size_of::<snd_pcm_chmap_elem>()) == 0 {
                return true;
            }
        }
        prev = (*prev).list.prev as *mut audioformat;
    }
    false
}

unsafe extern "C" fn usb_chmap_ctl_tlv(kcontrol: *mut snd_kcontrol, op_flag: c_int,
                                       size: c_uint, tlv: *mut c_uint) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;
    let subs = (*info).private_data as *mut snd_usb_substream;
    let mut count = 0;

    if (size as usize) < 8 {
        return -12; // -ENOMEM
    }
    if put_user(SNDRV_CTL_TLVT_CONTAINER, tlv) != 0 {
        return -14; // -EFAULT
    }
    let size = (size as usize) - 8;
    let mut dst = tlv.add(2);

    let mut fp = (*subs).fmt_list.next as *mut audioformat;
    let mut cur_size = size;
    while fp as *mut ListHead != &(*subs).fmt_list {
        if !(*fp).chmap.is_null() {
            if !have_dup_chmap(subs, fp) {
                let chmap = (*fp).chmap as *mut snd_pcm_chmap_elem;
                let ch_bytes = ((*chmap).channels as usize) * 4;
                if cur_size < 8 + ch_bytes {
                    return -12; // -ENOMEM
                }
                if put_user(SNDRV_CTL_TLVT_CHMAP_FIXED, dst) != 0 ||
                   put_user((ch_bytes as c_uint), dst.add(1)) != 0 {
                    return -14; // -EFAULT
                }
                dst = dst.add(2);
                for i in 0..(*chmap).channels as usize {
                    if put_user((*chmap).map[i] as c_uint, dst.add(i)) != 0 {
                        return -14; // -EFAULT
                    }
                }

                count += 8 + ch_bytes;
                cur_size -= 8 + ch_bytes;
            }
        }
        fp = (*fp).list.next as *mut audioformat;
    }
    if put_user((count as c_uint), tlv.add(1)) != 0 {
        return -14; // -EFAULT
    }
    0
}

unsafe extern "C" fn usb_chmap_ctl_get(kcontrol: *mut snd_kcontrol,
                                       ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;
    let subs = (*info).private_data as *mut snd_usb_substream;
    let mut chmap: *mut snd_pcm_chmap_elem = ptr::null_mut();
    let mut i = 0;

    if !(*subs).cur_audiofmt.is_null() {
        chmap = (*(*subs).cur_audiofmt).chmap as *mut snd_pcm_chmap_elem;
    }
    if !chmap.is_null() {
        while i < (*chmap).channels {
            // Set ucontrol->value.integer.value[i] = chmap->map[i]
            i += 1;
        }
    }
    while i < (*subs).channels_max {
        // Set ucontrol->value.integer.value[i] = 0
        i += 1;
    }
    0
}

unsafe fn add_chmap(pcm: *mut snd_pcm, stream: c_int, subs: *mut snd_usb_substream) -> c_int {
    let mut fp = (*subs).fmt_list.next as *mut audioformat;
    let mut found = false;

    while fp as *mut ListHead != &(*subs).fmt_list {
        if !(*fp).chmap.is_null() {
            found = true;
            break;
        }
        fp = (*fp).list.next as *mut audioformat;
    }

    if !found {
        return 0;
    }

    let mut chmap: *mut snd_pcm_chmap = ptr::null_mut();
    let err = snd_pcm_add_chmap_ctls(pcm, stream, ptr::null_mut(), 0, 0, &mut chmap);
    if err < 0 {
        return err;
    }

    (*chmap).private_data = subs as *mut c_void;
    let kctl = (*chmap).kctl;
    (*kctl).info = Some(usb_chmap_ctl_info);
    (*kctl).get = Some(usb_chmap_ctl_get);
    (*kctl).tlv.c = Some(usb_chmap_ctl_tlv);

    0
}

unsafe fn convert_chmap(channels: c_int, bits: c_uint, protocol: c_int) -> *mut snd_pcm_chmap_elem {
    const UAC1_MAPS: &[u32] = &[
        SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE,
        SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC,
        SNDRV_CHMAP_RC, SNDRV_CHMAP_SL, SNDRV_CHMAP_SR, SNDRV_CHMAP_TC,
        0,
    ];
    const UAC2_MAPS: &[u32] = &[
        SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE,
        SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC,
        SNDRV_CHMAP_RC, SNDRV_CHMAP_SL, SNDRV_CHMAP_SR, SNDRV_CHMAP_TC,
        SNDRV_CHMAP_TFL, SNDRV_CHMAP_TFC, SNDRV_CHMAP_TFR, SNDRV_CHMAP_TRL,
        SNDRV_CHMAP_TRC, SNDRV_CHMAP_TRR, SNDRV_CHMAP_TFLC, SNDRV_CHMAP_TFRC,
        SNDRV_CHMAP_LLFE, SNDRV_CHMAP_RLFE, SNDRV_CHMAP_TSL, SNDRV_CHMAP_TSR,
        SNDRV_CHMAP_BC, SNDRV_CHMAP_RLC, SNDRV_CHMAP_RRC,
        0,
    ];

    if channels as usize > 16 {
        return ptr::null_mut();
    }

    let chmap = kzalloc_obj(&mut snd_pcm_chmap_elem::default() as *mut _) as *mut snd_pcm_chmap_elem;
    if chmap.is_null() {
        return ptr::null_mut();
    }

    let maps = if protocol == UAC_VERSION_2 { UAC2_MAPS } else { UAC1_MAPS };
    (*chmap).channels = channels;
    let mut c = 0;
    let mut bits = bits as u32;
    let mut map_idx = 0;

    if bits != 0 {
        while bits != 0 && maps[map_idx] != 0 {
            if (bits & 1) != 0 {
                (*chmap).map[c as usize] = maps[map_idx];
                c += 1;
            }
            bits >>= 1;
            map_idx += 1;
            if c == channels {
                break;
            }
        }
    } else {
        if channels == 1 {
            (*chmap).map[c as usize] = SNDRV_CHMAP_MONO;
            c += 1;
        } else {
            while c < channels && maps[map_idx] != 0 {
                (*chmap).map[c as usize] = maps[map_idx];
                c += 1;
                map_idx += 1;
            }
        }
    }

    while c < channels {
        (*chmap).map[c as usize] = SNDRV_CHMAP_UNKNOWN;
        c += 1;
    }

    chmap
}

unsafe fn convert_chmap_v3(cluster: *mut uac3_cluster_header_descriptor) -> *mut snd_pcm_chmap_elem {
    let channels = (*cluster).bNrChannels as c_int;
    let chmap = kzalloc_obj(&mut snd_pcm_chmap_elem::default() as *mut _) as *mut snd_pcm_chmap_elem;
    if chmap.is_null() {
        return ptr::null_mut();
    }

    if channels as usize > 16 {
        kfree(chmap as *mut c_void);
        return ptr::null_mut();
    }

    let mut len = le16_to_cpu((*cluster).wLength) as isize;
    let mut c = 0;
    let mut p = cluster.add(1) as *mut u8;
    len -= mem::size_of::<uac3_cluster_header_descriptor>() as isize;

    while len > 0 && c < channels {
        let cs_desc = p as *mut uac3_cluster_segment_descriptor;
        if len < mem::size_of::<uac3_cluster_segment_descriptor>() as isize {
            break;
        }
        let cs_len = le16_to_cpu((*cs_desc).wLength) as isize;
        if cs_len < mem::size_of::<uac3_cluster_segment_descriptor>() as isize {
            break;
        }
        if len < cs_len {
            break;
        }
        let cs_type = (*cs_desc).bSegmentType;

        if cs_type == UAC3_CHANNEL_INFORMATION {
            let is = p as *mut uac3_cluster_information_segment_descriptor;
            if cs_len < mem::size_of::<uac3_cluster_information_segment_descriptor>() as isize {
                break;
            }

            let map = match (*is).bChRelationship {
                UAC3_CH_MONO => SNDRV_CHMAP_MONO,
                UAC3_CH_LEFT | UAC3_CH_FRONT_LEFT | UAC3_CH_HEADPHONE_LEFT => SNDRV_CHMAP_FL,
                UAC3_CH_RIGHT | UAC3_CH_FRONT_RIGHT | UAC3_CH_HEADPHONE_RIGHT => SNDRV_CHMAP_FR,
                UAC3_CH_FRONT_CENTER => SNDRV_CHMAP_FC,
                UAC3_CH_FRONT_LEFT_OF_CENTER => SNDRV_CHMAP_FLC,
                UAC3_CH_FRONT_RIGHT_OF_CENTER => SNDRV_CHMAP_FRC,
                UAC3_CH_FRONT_WIDE_LEFT => SNDRV_CHMAP_FLW,
                UAC3_CH_FRONT_WIDE_RIGHT => SNDRV_CHMAP_FRW,
                UAC3_CH_SIDE_LEFT => SNDRV_CHMAP_SL,
                UAC3_CH_SIDE_RIGHT => SNDRV_CHMAP_SR,
                UAC3_CH_BACK_LEFT => SNDRV_CHMAP_RL,
                UAC3_CH_BACK_RIGHT => SNDRV_CHMAP_RR,
                UAC3_CH_BACK_CENTER => SNDRV_CHMAP_RC,
                UAC3_CH_BACK_LEFT_OF_CENTER => SNDRV_CHMAP_RLC,
                UAC3_CH_BACK_RIGHT_OF_CENTER => SNDRV_CHMAP_RRC,
                UAC3_CH_TOP_CENTER => SNDRV_CHMAP_TC,
                UAC3_CH_TOP_FRONT_LEFT => SNDRV_CHMAP_TFL,
                UAC3_CH_TOP_FRONT_RIGHT => SNDRV_CHMAP_TFR,
                UAC3_CH_TOP_FRONT_CENTER => SNDRV_CHMAP_TFC,
                UAC3_CH_TOP_FRONT_LOC => SNDRV_CHMAP_TFLC,
                UAC3_CH_TOP_FRONT_ROC => SNDRV_CHMAP_TFRC,
                UAC3_CH_TOP_SIDE_LEFT => SNDRV_CHMAP_TSL,
                UAC3_CH_TOP_SIDE_RIGHT => SNDRV_CHMAP_TSR,
                UAC3_CH_TOP_BACK_LEFT => SNDRV_CHMAP_TRL,
                UAC3_CH_TOP_BACK_RIGHT => SNDRV_CHMAP_TRR,
                UAC3_CH_TOP_BACK_CENTER => SNDRV_CHMAP_TRC,
                UAC3_CH_BOTTOM_CENTER => SNDRV_CHMAP_BC,
                UAC3_CH_LOW_FREQUENCY_EFFECTS => SNDRV_CHMAP_LFE,
                UAC3_CH_LFE_LEFT => SNDRV_CHMAP_LLFE,
                UAC3_CH_LFE_RIGHT => SNDRV_CHMAP_RLFE,
                _ => SNDRV_CHMAP_UNKNOWN,
            };
            (*chmap).map[c as usize] = map;
            c += 1;
        }
        p = p.add(cs_len as usize);
        len -= cs_len;
    }

    if channels < c {
        pr_err(b"convert_chmap_v3: channel number mismatch\n\0".as_ptr() as *const u8);
    }

    (*chmap).channels = channels;

    while c < channels {
        (*chmap).map[c as usize] = SNDRV_CHMAP_UNKNOWN;
        c += 1;
    }

    chmap
}

pub unsafe fn snd_usb_add_audio_stream(chip: *mut snd_usb_audio,
                                        stream: c_int,
                                        fp: *mut audioformat,
                                        pdptr: *mut *mut snd_usb_power_domain) -> c_int {
    let mut as_ = (*chip).pcm_list.next as *mut snd_usb_stream;
    while as_ as *mut ListHead != &(*chip).pcm_list {
        if (*as_).fmt_type == (*fp).fmt_type {
            let subs = &mut (*as_).substream[stream as usize];
            if (*subs).ep_num == (*fp).endpoint {
                let fmt_list = &mut subs.fmt_list;
                (*fp).list.next = fmt_list as *mut ListHead;
                (*fp).list.prev = fmt_list.prev;
                (*fmt_list).prev.as_mut().unwrap().next = &mut (*fp).list;
                (*fmt_list).prev = &mut (*fp).list;
                subs.num_formats += 1;
                subs.formats |= (*fp).formats;
                return 0;
            }
        }
        as_ = (*as_).list.next as *mut snd_usb_stream;
    }

    if (*(*chip).card).registered {
        (*chip).need_delayed_register = true;
    }

    as_ = (*chip).pcm_list.next as *mut snd_usb_stream;
    while as_ as *mut ListHead != &(*chip).pcm_list {
        if (*as_).fmt_type == (*fp).fmt_type {
            let subs = &mut (*as_).substream[stream as usize];
            if (*subs).ep_num != 0 {
                as_ = (*as_).list.next as *mut snd_usb_stream;
                continue;
            }
            let err = snd_pcm_new_stream((*as_).pcm, stream, 1);
            if err < 0 {
                return err;
            }
            snd_usb_init_substream(as_, stream, fp, pdptr);
            return add_chmap((*as_).pcm, stream, subs);
        }
        as_ = (*as_).list.next as *mut snd_usb_stream;
    }

    let as_ = kzalloc_obj(&mut snd_usb_stream::default() as *mut _) as *mut snd_usb_stream;
    if as_.is_null() {
        return -12; // -ENOMEM
    }
    (*as_).pcm_index = (*chip).pcm_devs;
    (*as_).chip = chip;
    (*as_).fmt_type = (*fp).fmt_type;
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let playback = if stream == SNDRV_PCM_STREAM_PLAYBACK { 1 } else { 0 };
    let capture = if stream == SNDRV_PCM_STREAM_PLAYBACK { 0 } else { 1 };
    let err = snd_pcm_new((*chip).card, b"USB Audio\0".as_ptr(), (*chip).pcm_devs,
                          playback, capture, &mut pcm);
    if err < 0 {
        kfree(as_ as *mut c_void);
        return err;
    }
    (*as_).pcm = pcm;
    (*pcm).private_data = as_ as *mut c_void;
    (*pcm).private_free = Some(snd_usb_audio_pcm_free);
    (*pcm).info_flags = 0;
    if (*chip).pcm_devs > 0 {
        let name_ptr = (*pcm).name.as_mut_ptr();
        scnprintf(name_ptr, 80, b"USB Audio #%d\0".as_ptr(), (*chip).pcm_devs);
    } else {
        strscpy((*pcm).name.as_mut_ptr(), b"USB Audio\0".as_ptr(), 80);
    }

    snd_usb_init_substream(as_, stream, fp, pdptr);

    if (*chip).usb_id == 0x07632003 {
        let pcm_list = &mut (*chip).pcm_list;
        (*as_).list.next = pcm_list as *mut ListHead;
        (*as_).list.prev = pcm_list.prev;
        (*pcm_list).prev.as_mut().unwrap().next = &mut (*as_).list;
        (*pcm_list).prev = &mut (*as_).list;
    } else {
        let pcm_list = &mut (*chip).pcm_list;
        (*as_).list.next = pcm_list as *mut ListHead;
        (*as_).list.prev = pcm_list.prev;
        (*pcm_list).prev.as_mut().unwrap().next = &mut (*as_).list;
        (*pcm_list).prev = &mut (*as_).list;
    }

    (*chip).pcm_devs += 1;

    snd_usb_proc_pcm_format_add(as_);

    add_chmap(pcm, stream, &mut (*as_).substream[stream as usize])
}

unsafe fn parse_uac_endpoint_attributes(chip: *mut snd_usb_audio,
                                         alts: *mut usb_host_interface,
                                         protocol: c_int,
                                         iface_no: c_int) -> c_int {
    let altsd = get_iface_desc(alts);
    let mut attributes = 0;

    let mut csep = snd_usb_find_desc(
        (*(*alts).endpoint[0]).extra as *mut c_void,
        (*(*alts).endpoint[0]).extralen,
        ptr::null_mut(),
        USB_DT_CS_ENDPOINT
    );

    if csep.is_null() && (*altsd).bNumEndpoints >= 2 {
        csep = snd_usb_find_desc(
            (*(*alts).endpoint[1]).extra as *mut c_void,
            (*(*alts).endpoint[1]).extralen,
            ptr::null_mut(),
            USB_DT_CS_ENDPOINT
        );
    }

    if csep.is_null() {
        csep = snd_usb_find_desc(
            (*alts).extra as *mut c_void,
            (*alts).extralen,
            ptr::null_mut(),
            USB_DT_CS_ENDPOINT
        );
    }

    let csep_typed = csep as *mut uac_iso_endpoint_descriptor;
    if csep.is_null() || (*csep_typed).bLength < 7 ||
       (*csep_typed).bDescriptorSubtype != UAC_EP_GENERAL {
        usb_audio_warn(chip, b"%u:%d : no or invalid class specific endpoint descriptor\n\0".as_ptr(),
                       iface_no, (*altsd).bAlternateSetting);
        return 0;
    }

    if protocol == UAC_VERSION_1 {
        attributes = (*csep_typed).bmAttributes as c_int;
    } else if protocol == UAC_VERSION_2 {
        let csep2 = csep as *mut uac2_iso_endpoint_descriptor;
        if (*csep2).bLength < mem::size_of::<uac2_iso_endpoint_descriptor>() as u8 {
            usb_audio_warn(chip, b"%u:%d : no or invalid class specific endpoint descriptor\n\0".as_ptr(),
                           iface_no, (*altsd).bAlternateSetting);
            return 0;
        }
        attributes = ((*csep_typed).bmAttributes as c_int) & UAC_EP_CS_ATTR_FILL_MAX;

        let bmControls = (*csep2).bmControls as c_int;
        if (bmControls & UAC2_CONTROL_PITCH) != 0 {
            attributes |= UAC_EP_CS_ATTR_PITCH_CONTROL;
        }
    } else {
        let csep3 = csep as *mut uac3_iso_endpoint_descriptor;
        if (*csep3).bLength < mem::size_of::<uac3_iso_endpoint_descriptor>() as u8 {
            usb_audio_warn(chip, b"%u:%d : no or invalid class specific endpoint descriptor\n\0".as_ptr(),
                           iface_no, (*altsd).bAlternateSetting);
            return 0;
        }
        let bmControls = le32_to_cpu((*csep3).bmControls as u32) as c_int;
        if (bmControls & UAC2_CONTROL_PITCH) != 0 {
            attributes |= UAC_EP_CS_ATTR_PITCH_CONTROL;
        }
    }

    attributes
}

unsafe fn audio_format_alloc_init(chip: *mut snd_usb_audio,
                                   alts: *mut usb_host_interface,
                                   protocol: c_int, iface_no: c_int, altset_idx: c_int,
                                   altno: c_int, num_channels: c_int, clock: c_int) -> *mut audioformat {
    let fp = kzalloc_obj(&mut audioformat::default() as *mut _) as *mut audioformat;
    if fp.is_null() {
        return ptr::null_mut();
    }

    (*fp).iface = iface_no;
    (*fp).altsetting = altno;
    (*fp).altset_idx = altset_idx;
    (*fp).endpoint = (*(get_endpoint(alts, 0))).bEndpointAddress as c_int;
    (*fp).ep_attr = (*(get_endpoint(alts, 0))).bmAttributes as c_int;
    (*fp).datainterval = snd_usb_parse_datainterval(chip, alts);
    (*fp).protocol = protocol;
    (*fp).maxpacksize = usb_endpoint_max_periodic_payload((*chip).dev, get_endpoint(alts, 0));
    (*fp).channels = num_channels;
    (*fp).clock = clock;
    INIT_LIST_HEAD(&mut (*fp).list);

    fp
}

pub unsafe fn snd_usb_get_audioformat_uac12(chip: *mut snd_usb_audio,
                                             alts: *mut usb_host_interface,
                                             protocol: c_int, iface_no: c_int, altset_idx: c_int,
                                             altno: c_int, stream: c_int, bm_quirk: c_int) -> *mut audioformat {
    let dev = (*chip).dev;
    let mut num_channels: c_uint = 0;
    let mut chconfig: c_uint = 0;
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, iface_no);
    let mut clock = 0;
    let mut format: u64 = 0;

    if protocol == UAC_VERSION_1 {
        let as_ = snd_usb_find_csint_desc((*alts).extra as *mut c_void, (*alts).extralen,
                                            ptr::null_mut(), UAC_AS_GENERAL) as *mut uac1_as_header_descriptor;
        if as_.is_null() {
            dev_err(&mut (*dev).dev as *mut _, b"%u:%d : UAC_AS_GENERAL descriptor not found\n\0".as_ptr());
            return ptr::null_mut();
        }

        if (*as_).bLength < mem::size_of::<uac1_as_header_descriptor>() as u8 {
            dev_err(&mut (*dev).dev as *mut _, b"%u:%d : invalid UAC_AS_GENERAL desc\n\0".as_ptr());
            return ptr::null_mut();
        }

        format = le16_to_cpu((*as_).wFormatTag as u16) as u64;

        let iterm = snd_usb_find_input_terminal_descriptor(ctrl_intf, (*as_).bTerminalLink as c_int, protocol)
                    as *mut uac_input_terminal_descriptor;
        if !iterm.is_null() {
            num_channels = (*iterm).bNrChannels as c_uint;
            chconfig = le16_to_cpu((*iterm).wChannelConfig as u16) as c_uint;
        }
    } else {
        let as_ = snd_usb_find_csint_desc((*alts).extra as *mut c_void, (*alts).extralen,
                                            ptr::null_mut(), UAC_AS_GENERAL) as *mut uac2_as_header_descriptor;
        if as_.is_null() {
            dev_err(&mut (*dev).dev as *mut _, b"%u:%d : UAC_AS_GENERAL descriptor not found\n\0".as_ptr());
            return ptr::null_mut();
        }

        if (*as_).bLength < mem::size_of::<uac2_as_header_descriptor>() as u8 {
            dev_err(&mut (*dev).dev as *mut _, b"%u:%d : invalid UAC_AS_GENERAL desc\n\0".as_ptr());
            return ptr::null_mut();
        }

        num_channels = (*as_).bNrChannels as c_uint;
        format = le32_to_cpu((*as_).bmFormats as u32) as u64;
        chconfig = le32_to_cpu((*as_).bmChannelConfig as u32) as c_uint;

        let input_term = snd_usb_find_input_terminal_descriptor(ctrl_intf, (*as_).bTerminalLink as c_int, protocol)
                         as *mut uac2_input_terminal_descriptor;
        if !input_term.is_null() {
            clock = (*input_term).bCSourceID as c_int;
            if chconfig == 0 && (num_channels as c_int == (*input_term).bNrChannels as c_int) {
                chconfig = le32_to_cpu((*input_term).bmChannelConfig as u32) as c_uint;
            }
        } else {
            let output_term = snd_usb_find_output_terminal_descriptor(ctrl_intf, (*as_).bTerminalLink as c_int, protocol)
                              as *mut uac2_output_terminal_descriptor;
            if !output_term.is_null() {
                clock = (*output_term).bCSourceID as c_int;
            } else {
                dev_err(&mut (*dev).dev as *mut _, b"%u:%d : bogus bTerminalLink %d\n\0".as_ptr());
                return ptr::null_mut();
            }
        }
    }

    let fmt = snd_usb_find_csint_desc((*alts).extra as *mut c_void, (*alts).extralen,
                                       ptr::null_mut(), UAC_FORMAT_TYPE) as *mut uac_format_type_i_continuous_descriptor;
    if fmt.is_null() {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : no UAC_FORMAT_TYPE desc\n\0".as_ptr());
        return ptr::null_mut();
    }
    if ((protocol == UAC_VERSION_1) && ((*fmt).bLength < 8)) ||
       ((protocol == UAC_VERSION_2) && ((*fmt).bLength < 6)) {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : invalid UAC_FORMAT_TYPE desc\n\0".as_ptr());
        return ptr::null_mut();
    }

    if bm_quirk != 0 && (*fmt).bNrChannels == 1 && (*fmt).bSubframeSize == 2 {
        return ptr::null_mut();
    }

    let fp = audio_format_alloc_init(chip, alts, protocol, iface_no, altset_idx, altno, num_channels as c_int, clock);
    if fp.is_null() {
        return ptr::null_mut(); // TODO: return ERR_PTR(-ENOMEM)
    }

    (*fp).attributes = parse_uac_endpoint_attributes(chip, alts, protocol, iface_no);

    snd_usb_audioformat_attributes_quirk(chip, fp, stream);

    if snd_usb_parse_audio_format(chip, fp, format, fmt, stream) < 0 {
        audioformat_free(fp);
        return ptr::null_mut();
    }

    if (*fp).channels != num_channels as c_int {
        chconfig = 0;
    }

    (*fp).chmap = convert_chmap((*fp).channels, chconfig, protocol) as *mut c_void;

    fp
}

pub unsafe fn snd_usb_get_audioformat_uac3(chip: *mut snd_usb_audio,
                                            alts: *mut usb_host_interface,
                                            pd_out: *mut *mut snd_usb_power_domain,
                                            iface_no: c_int, altset_idx: c_int,
                                            altno: c_int, stream: c_int) -> *mut audioformat {
    let dev = (*chip).dev;
    let mut chmap: *mut snd_pcm_chmap_elem = ptr::null_mut();
    let mut pd: *mut snd_usb_power_domain = ptr::null_mut();
    let badd_profile = (*chip).badd_profile;
    let mut badd_formats: u64 = 0;
    let mut num_channels = 0;
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, iface_no);

    if badd_profile >= UAC3_FUNCTION_SUBCLASS_GENERIC_IO {
        let maxpacksize = le16_to_cpu((*(get_endpoint(alts, 0))).wMaxPacketSize as u16);

        match maxpacksize {
            UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_16 |
            UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_16 => {
                badd_formats = SNDRV_PCM_FMTBIT_S16_LE;
                num_channels = 1;
            }
            UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_24 |
            UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_24 => {
                badd_formats = SNDRV_PCM_FMTBIT_S24_3LE;
                num_channels = 1;
            }
            UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_16 |
            UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_16 => {
                badd_formats = SNDRV_PCM_FMTBIT_S16_LE;
                num_channels = 2;
            }
            UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_24 |
            UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_24 => {
                badd_formats = SNDRV_PCM_FMTBIT_S24_3LE;
                num_channels = 2;
            }
            _ => {
                dev_err(&mut (*dev).dev as *mut _, b"%u:%d : incorrect wMaxPacketSize for BADD profile\n\0".as_ptr());
                return ptr::null_mut();
            }
        }

        chmap = kzalloc_obj(&mut snd_pcm_chmap_elem::default() as *mut _) as *mut snd_pcm_chmap_elem;
        if chmap.is_null() {
            return ptr::null_mut(); // TODO: return ERR_PTR(-ENOMEM)
        }

        if num_channels == 1 {
            (*chmap).map[0] = SNDRV_CHMAP_MONO;
        } else {
            (*chmap).map[0] = SNDRV_CHMAP_FL;
            (*chmap).map[1] = SNDRV_CHMAP_FR;
        }

        (*chmap).channels = num_channels;
        let clock = UAC3_BADD_CS_ID9;
        let mut fp = audio_format_alloc_init(chip, alts, UAC_VERSION_3, iface_no, altset_idx, altno, num_channels, clock);
        if fp.is_null() {
            kfree(chmap as *mut c_void);
            return ptr::null_mut(); // TODO: return ERR_PTR(-ENOMEM)
        }

        (*fp).chmap = chmap as *mut c_void;
        (*fp).attributes = 0;
        (*fp).fmt_type = UAC_FORMAT_TYPE_I;
        (*fp).formats = badd_formats;
        (*fp).nr_rates = 0;
        (*fp).rate_min = UAC3_BADD_SAMPLING_RATE;
        (*fp).rate_max = UAC3_BADD_SAMPLING_RATE;
        (*fp).rates = SNDRV_PCM_RATE_CONTINUOUS;

        pd = kzalloc_obj(&mut snd_usb_power_domain::default() as *mut _) as *mut snd_usb_power_domain;
        if pd.is_null() {
            audioformat_free(fp);
            return ptr::null_mut();
        }
        (*pd).pd_id = if stream == SNDRV_PCM_STREAM_PLAYBACK { UAC3_BADD_PD_ID10 } else { UAC3_BADD_PD_ID11 };
        (*pd).pd_d1d0_rec = UAC3_BADD_PD_RECOVER_D1D0;
        (*pd).pd_d2d0_rec = UAC3_BADD_PD_RECOVER_D2D0;
        (*pd).ctrl_iface = ctrl_intf;

        *pd_out = pd;
        return fp;
    }

    let as_ = snd_usb_find_csint_desc((*alts).extra as *mut c_void, (*alts).extralen,
                                       ptr::null_mut(), UAC_AS_GENERAL) as *mut uac3_as_header_descriptor;
    if as_.is_null() {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : UAC_AS_GENERAL descriptor not found\n\0".as_ptr());
        return ptr::null_mut();
    }

    if (*as_).bLength < mem::size_of::<uac3_as_header_descriptor>() as u8 {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : invalid UAC_AS_GENERAL desc\n\0".as_ptr());
        return ptr::null_mut();
    }

    let cluster_id = le16_to_cpu((*as_).wClusterDescrID as u16);
    if cluster_id == 0 {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : no cluster descriptor\n\0".as_ptr());
        return ptr::null_mut();
    }

    let mut hc_header: uac3_hc_descriptor_header = mem::zeroed();
    let err = snd_usb_ctl_msg((*chip).dev,
                              usb_rcvctrlpipe((*chip).dev, 0),
                              UAC3_CS_REQ_HIGH_CAPABILITY_DESCRIPTOR,
                              USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_IN_CONST,
                              cluster_id,
                              snd_usb_ctrl_intf(ctrl_intf),
                              &mut hc_header as *mut _ as *mut c_void,
                              mem::size_of::<uac3_hc_descriptor_header>());
    if err < 0 {
        return ptr::null_mut(); // TODO: return ERR_PTR(err)
    } else if err as usize != mem::size_of::<uac3_hc_descriptor_header>() {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : can't get High Capability descriptor\n\0".as_ptr());
        return ptr::null_mut(); // TODO: return ERR_PTR(-EIO)
    }

    let wLength = le16_to_cpu(hc_header.wLength) as usize;
    if wLength < mem::size_of::<uac3_cluster_header_descriptor>() {
        return ptr::null_mut();
    }
    let cluster = kzalloc(wLength, 0x10) as *mut uac3_cluster_header_descriptor; // GFP_KERNEL = 0x10
    if cluster.is_null() {
        return ptr::null_mut(); // TODO: return ERR_PTR(-ENOMEM)
    }
    let err = snd_usb_ctl_msg((*chip).dev,
                              usb_rcvctrlpipe((*chip).dev, 0),
                              UAC3_CS_REQ_HIGH_CAPABILITY_DESCRIPTOR,
                              USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_IN_CONST,
                              cluster_id,
                              snd_usb_ctrl_intf(ctrl_intf),
                              cluster as *mut c_void,
                              wLength);
    if err < 0 {
        kfree(cluster as *mut c_void);
        return ptr::null_mut(); // TODO: return ERR_PTR(err)
    } else if err as usize != wLength {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : can't get Cluster Descriptor\n\0".as_ptr());
        kfree(cluster as *mut c_void);
        return ptr::null_mut(); // TODO: return ERR_PTR(-EIO)
    }

    let cluster_wLength = le16_to_cpu((*cluster).wLength) as usize;
    if cluster_wLength < mem::size_of::<uac3_cluster_header_descriptor>() ||
       cluster_wLength > wLength {
        dev_err(&mut (*dev).dev as *mut _, b"%u:%d : invalid Cluster Descriptor size\n\0".as_ptr());
        kfree(cluster as *mut c_void);
        return ptr::null_mut(); // TODO: return ERR_PTR(-EIO)
    }

    num_channels = (*cluster).bNrChannels as c_int;
    chmap = convert_chmap_v3(cluster);
    kfree(cluster as *mut c_void);

    let input_term = snd_usb_find_input_terminal_descriptor(ctrl_intf, (*as_).bTerminalLink as c_int, UAC_VERSION_3)
                     as *mut uac3_input_terminal_descriptor;
    let mut clock = 0;
    if !input_term.is_null() {
        clock = (*input_term).bCSourceID as c_int;
    } else {
        let output_term = snd_usb_find_output_terminal_descriptor(ctrl_intf, (*as_).bTerminalLink as c_int, UAC_VERSION_3)
                          as *mut uac3_output_terminal_descriptor;
        if !output_term.is_null() {
            clock = (*output_term).bCSourceID as c_int;
        } else {
            dev_err(&mut (*dev).dev as *mut _, b"%u:%d : bogus bTerminalLink %d\n\0".as_ptr());
            kfree(chmap as *mut c_void);
            return ptr::null_mut();
        }
    }

    let fp = audio_format_alloc_init(chip, alts, UAC_VERSION_3, iface_no, altset_idx, altno, num_channels, clock);
    if fp.is_null() {
        kfree(chmap as *mut c_void);
        return ptr::null_mut(); // TODO: return ERR_PTR(-ENOMEM)
    }

    (*fp).chmap = chmap as *mut c_void;

    (*fp).attributes = parse_uac_endpoint_attributes(chip, alts, UAC_VERSION_3, iface_no);

    pd = snd_usb_find_power_domain(ctrl_intf, (*as_).bTerminalLink as c_int);

    if snd_usb_parse_audio_format_v3(chip, fp, as_, stream) < 0 {
        kfree(pd as *mut c_void);
        audioformat_free(fp);
        return ptr::null_mut();
    }

    *pd_out = pd;

    fp
}

unsafe fn __snd_usb_parse_audio_interface(chip: *mut snd_usb_audio, iface_no: c_int,
                                           has_non_pcm: *mut bool, non_pcm: bool) -> c_int {
    let dev = (*chip).dev;
    let iface = usb_ifnum_to_if(dev, iface_no);
    let mut num = (*iface).num_altsetting;

    if (*chip).usb_id == 0x04fa4201 && num >= 4 {
        num = 4;
    }

    let mut i = 0;
    while i < num {
        let alts = &mut (*iface).altsetting[i as usize];
        let altsd = get_iface_desc(alts);
        let protocol = (*altsd).bInterfaceProtocol as c_int;

        if ((((*altsd).bInterfaceClass != USB_CLASS_AUDIO ||
              ((*altsd).bInterfaceSubClass != USB_SUBCLASS_AUDIOSTREAMING &&
               (*altsd).bInterfaceSubClass != USB_SUBCLASS_VENDOR_SPEC)) &&
             (*altsd).bInterfaceClass != USB_CLASS_VENDOR_SPEC) ||
            (*altsd).bNumEndpoints < 1 ||
            le16_to_cpu((*(get_endpoint(alts, 0))).wMaxPacketSize as u16) == 0) {
            i += 1;
            continue;
        }

        if ((*(get_endpoint(alts, 0))).bmAttributes & USB_ENDPOINT_XFERTYPE_MASK) != USB_ENDPOINT_XFER_ISOC {
            i += 1;
            continue;
        }

        let stream = if ((*(get_endpoint(alts, 0))).bEndpointAddress & USB_DIR_IN) != 0 {
            SNDRV_PCM_STREAM_CAPTURE
        } else {
            SNDRV_PCM_STREAM_PLAYBACK
        };
        let altno = (*altsd).bAlternateSetting as c_int;

        if snd_usb_apply_interface_quirk(chip, iface_no, altno) {
            i += 1;
            continue;
        }

        let mut pd: *mut snd_usb_power_domain = ptr::null_mut();

        let mut protocol = protocol;
        if ((*chip).usb_id >> 16) == 0x0582 &&
           (*altsd).bInterfaceClass == USB_CLASS_VENDOR_SPEC &&
           protocol <= 2 {
            protocol = UAC_VERSION_1;
        }

        let mut fp: *mut audioformat = ptr::null_mut();
        match protocol {
            UAC_VERSION_1 | UAC_VERSION_2 => {
                let mut bm_quirk = 0;

                if altno == 2 && num == 3 &&
                   !fp.is_null() && (*fp).altsetting == 1 && (*fp).channels == 1 &&
                   (*fp).formats == SNDRV_PCM_FMTBIT_S16_LE &&
                   protocol == UAC_VERSION_1 &&
                   le16_to_cpu((*(get_endpoint(alts, 0))).wMaxPacketSize as u16) as usize == ((*fp).maxpacksize as usize) * 2 {
                    bm_quirk = 1;
                }

                fp = snd_usb_get_audioformat_uac12(chip, alts, protocol, iface_no, i, altno, stream, bm_quirk);
            }
            UAC_VERSION_3 => {
                fp = snd_usb_get_audioformat_uac3(chip, alts, &mut pd, iface_no, i, altno, stream);
            }
            _ => {
                dev_dbg(&mut (*dev).dev as *mut _, b"%u:%d: unknown interface protocol %#02x, assuming v1\n\0".as_ptr());
                let mut bm_quirk = 0;
                if altno == 2 && num == 3 &&
                   !fp.is_null() && (*fp).altsetting == 1 && (*fp).channels == 1 &&
                   (*fp).formats == SNDRV_PCM_FMTBIT_S16_LE &&
                   UAC_VERSION_1 == UAC_VERSION_1 &&
                   le16_to_cpu((*(get_endpoint(alts, 0))).wMaxPacketSize as u16) as usize == ((*fp).maxpacksize as usize) * 2 {
                    bm_quirk = 1;
                }
                fp = snd_usb_get_audioformat_uac12(chip, alts, UAC_VERSION_1, iface_no, i, altno, stream, bm_quirk);
            }
        }

        if fp.is_null() {
            i += 1;
            continue;
        }

        if (*fp).fmt_type != UAC_FORMAT_TYPE_I {
            *has_non_pcm = true;
        }
        if ((*fp).fmt_type == UAC_FORMAT_TYPE_I) == non_pcm {
            audioformat_free(fp);
            fp = ptr::null_mut();
            i += 1;
            continue;
        }

        snd_usb_audioformat_set_sync_ep(chip, fp);

        dev_dbg(&mut (*dev).dev as *mut _, b"%u:%d: add audio endpoint %#x\n\0".as_ptr());
        let err = snd_usb_add_audio_stream(chip, stream, fp, &mut pd);
        if err < 0 {
            audioformat_free(fp);
            return err;
        }

        let err = snd_usb_add_endpoint(chip, (*fp).endpoint, SND_USB_ENDPOINT_TYPE_DATA);
        if err < 0 {
            return err;
        }

        if (*fp).sync_ep != 0 {
            let ep_type = if (*fp).implicit_fb != 0 { SND_USB_ENDPOINT_TYPE_DATA } else { SND_USB_ENDPOINT_TYPE_SYNC };
            let err = snd_usb_add_endpoint(chip, (*fp).sync_ep, ep_type);
            if err < 0 {
                return err;
            }
        }

        let mut set_iface_first = false;
        if protocol == UAC_VERSION_1 ||
           ((*chip).quirk_flags & QUIRK_FLAG_SET_IFACE_FIRST) != 0 {
            set_iface_first = true;
        }

        if ((*chip).quirk_flags & QUIRK_FLAG_SKIP_IFACE_SETUP) != 0 {
            i += 1;
            continue;
        }

        usb_set_interface((*chip).dev, iface_no, 0);
        if set_iface_first {
            usb_set_interface((*chip).dev, iface_no, altno);
        }
        snd_usb_init_pitch(chip, fp);
        snd_usb_init_sample_rate(chip, fp, (*fp).rate_max);
        if !set_iface_first {
            usb_set_interface((*chip).dev, iface_no, altno);
        }

        i += 1;
    }
    0
}

pub fn snd_usb_parse_audio_interface(chip: *mut snd_usb_audio, iface_no: c_int) -> c_int {
    unsafe {
        let mut has_non_pcm = false;

        let err = __snd_usb_parse_audio_interface(chip, iface_no, &mut has_non_pcm, false);
        if err < 0 {
            return err;
        }

        if has_non_pcm {
            let err = __snd_usb_parse_audio_interface(chip, iface_no, &mut has_non_pcm, true);
            if err < 0 {
                return err;
            }
        }

        0
    }
}

#[repr(C)]
pub struct audioformat {
    pub iface: c_int,
    pub altsetting: c_int,
    pub altset_idx: c_int,
    pub endpoint: c_int,
    pub ep_attr: c_int,
    pub datainterval: c_int,
    pub protocol: c_int,
    pub maxpacksize: c_int,
    pub channels: c_int,
    pub clock: c_int,
    pub list: ListHead,
    pub formats: u64,
    pub fmt_type: c_int,
    pub nr_rates: c_int,
    pub rate_min: c_int,
    pub rate_max: c_int,
    pub rates: c_int,
    pub attributes: c_int,
    pub sync_ep: c_int,
    pub implicit_fb: c_int,
    pub chmap: *mut c_void,
}

impl Default for audioformat {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for snd_usb_stream {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for snd_pcm_chmap_elem {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for snd_usb_power_domain {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
