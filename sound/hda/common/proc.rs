// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Universal Interface for Intel High Definition Audio Codec
 *
 * Generic proc interface
 *
 * Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type hda_nid_t = u16;

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_array {
    pub list: *mut c_void,
    pub used: c_int,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct hda_nid_item {
    pub kctl: *mut snd_kcontrol,
    pub nid: hda_nid_t,
    pub index: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hda_pcm_stream {
    pub nid: hda_nid_t,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
}

#[repr(C)]
pub struct hda_pcm {
    pub list: list_head,
    pub name: *const c_char,
    pub pcm_type: c_int,
    pub pcm: *mut snd_pcm,
    pub stream: [hda_pcm_stream; 2],
}

#[repr(C)]
pub struct hdac_device {
    pub vendor_name: *const c_char,
    pub chip_name: *const c_char,
    pub addr: c_int,
    pub afg: hda_nid_t,
    pub afg_function_id: c_uint,
    pub afg_unsol: c_uint,
    pub mfg: hda_nid_t,
    pub mfg_function_id: c_uint,
    pub mfg_unsol: c_uint,
    pub vendor_id: c_uint,
    pub subsystem_id: c_uint,
    pub revision_id: c_uint,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
    pub pcm_list_head: list_head,
    pub mixers: snd_array,
    pub nids: snd_array,
    pub dump_coef: bool_,
    pub single_adc_amp: bool_,
    pub pin_amp_workaround: bool_,
    pub dp_mst: bool_,
    pub proc_widget_hook:
        Option<unsafe extern "C" fn(*mut snd_info_buffer, *mut hda_codec, hda_nid_t)>,
    pub card: *mut c_void,
}

unsafe extern "C" {
    static snd_hda_pcm_type_name: [*const c_char; 0];

    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_hdac_read_parm_uncached(codec: *mut hdac_device, nid: hda_nid_t, parm: c_uint) -> c_uint;
    fn snd_hda_param_read(codec: *mut hda_codec, nid: hda_nid_t, parm: c_uint) -> c_uint;
    fn snd_hda_get_raw_connections(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        conn: *mut hda_nid_t,
        max_conns: c_int,
    ) -> c_int;
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    );
    fn snd_hda_get_conn_list(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        list: *mut *const hda_nid_t,
    ) -> c_int;
    fn snd_hda_get_num_raw_conns(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_get_dev_select(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_set_dev_select(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int);
    fn snd_hda_get_devices(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_list: *mut u8,
        max_devices: c_uint,
    ) -> c_uint;
    fn snd_hda_get_sub_nodes(codec: *mut hda_codec, nid: hda_nid_t, start_id: *mut hda_nid_t)
        -> c_int;
    fn snd_card_ro_proc_new(
        card: *mut c_void,
        name: *const c_char,
        data: *mut hda_codec,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    ) -> c_int;
    fn snd_print_pcm_bits(pcm: c_uint, buf: *mut c_char, size: usize);
    fn get_amp_channels(kctl: *mut snd_kcontrol) -> c_ulong;
    fn get_amp_direction(kctl: *mut snd_kcontrol) -> c_ulong;
    fn get_amp_index(kctl: *mut snd_kcontrol) -> c_ulong;
    fn get_amp_offset(kctl: *mut snd_kcontrol) -> c_ulong;
    fn get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn get_wcaps_channels(wcaps: c_uint) -> c_uint;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_hda_power_pm(codec: *mut hda_codec);
}

unsafe extern "C" {
    static AC_WID_AUD_OUT: c_uint;
    static AC_WID_AUD_IN: c_uint;
    static AC_WID_AUD_MIX: c_uint;
    static AC_WID_AUD_SEL: c_uint;
    static AC_WID_PIN: c_uint;
    static AC_WID_POWER: c_uint;
    static AC_WID_VOL_KNB: c_uint;
    static AC_WID_BEEP: c_uint;
    static AC_WID_VENDOR: c_uint;
    static HDA_NID_ITEM_AMP: c_uint;
    static HDA_OUTPUT: c_int;
    static HDA_INPUT: c_int;
    static AC_PAR_AMP_OUT_CAP: c_uint;
    static AC_PAR_AMP_IN_CAP: c_uint;
    static AC_AMPCAP_OFFSET: c_uint;
    static AC_AMPCAP_NUM_STEPS: c_uint;
    static AC_AMPCAP_NUM_STEPS_SHIFT: c_uint;
    static AC_AMPCAP_STEP_SIZE: c_uint;
    static AC_AMPCAP_STEP_SIZE_SHIFT: c_uint;
    static AC_AMPCAP_MUTE: c_uint;
    static AC_AMPCAP_MUTE_SHIFT: c_uint;
    static AC_WCAP_STEREO: c_uint;
    static AC_PAR_AUDIO_WIDGET_CAP: c_uint;
    static AC_AMP_GET_OUTPUT: c_int;
    static AC_AMP_GET_INPUT: c_int;
    static AC_VERB_GET_AMP_GAIN_MUTE: c_uint;
    static AC_AMP_GET_LEFT: c_uint;
    static AC_AMP_GET_RIGHT: c_uint;
    static AC_SUPPCM_RATES: c_uint;
    static AC_SUPFMT_PCM: c_uint;
    static AC_SUPFMT_FLOAT32: c_uint;
    static AC_SUPFMT_AC3: c_uint;
    static AC_PAR_PCM: c_uint;
    static AC_PAR_STREAM: c_uint;
    static AC_DEFCFG_CONN_TYPE: u32;
    static AC_DEFCFG_CONN_TYPE_SHIFT: c_uint;
    static AC_DEFCFG_COLOR: u32;
    static AC_DEFCFG_COLOR_SHIFT: c_uint;
    static AC_DEFCFG_LOCATION: u32;
    static AC_DEFCFG_LOCATION_SHIFT: c_uint;
    static AC_DEFCFG_DEVICE: u32;
    static AC_DEFCFG_DEVICE_SHIFT: c_uint;
    static AC_PAR_PIN_CAP: c_uint;
    static AC_PINCAP_IN: c_uint;
    static AC_PINCAP_OUT: c_uint;
    static AC_PINCAP_HP_DRV: c_uint;
    static AC_PINCAP_EAPD: c_uint;
    static AC_PINCAP_PRES_DETECT: c_uint;
    static AC_PINCAP_BALANCE: c_uint;
    static AC_PINCAP_HDMI: c_uint;
    static AC_PINCAP_HBR: c_uint;
    static AC_PINCAP_DP: c_uint;
    static AC_PINCAP_TRIG_REQ: c_uint;
    static AC_PINCAP_IMP_SENSE: c_uint;
    static AC_PINCAP_VREF: c_uint;
    static AC_PINCAP_VREF_SHIFT: c_uint;
    static AC_PINCAP_VREF_HIZ: c_uint;
    static AC_PINCAP_VREF_50: c_uint;
    static AC_PINCAP_VREF_GRD: c_uint;
    static AC_PINCAP_VREF_80: c_uint;
    static AC_PINCAP_VREF_100: c_uint;
    static AC_VERB_GET_EAPD_BTLENABLE: c_uint;
    static AC_EAPDBTL_BALANCED: c_uint;
    static AC_EAPDBTL_EAPD: c_uint;
    static AC_EAPDBTL_LR_SWAP: c_uint;
    static AC_VERB_GET_CONFIG_DEFAULT: c_uint;
    static AC_DEFCFG_PORT_CONN: u32;
    static AC_DEFCFG_PORT_CONN_SHIFT: c_uint;
    static AC_DEFCFG_DEF_ASSOC: u32;
    static AC_DEFCFG_ASSOC_SHIFT: c_uint;
    static AC_DEFCFG_SEQUENCE: u32;
    static AC_DEFCFG_MISC: u32;
    static AC_DEFCFG_MISC_SHIFT: c_uint;
    static AC_DEFCFG_MISC_NO_PRESENCE: u32;
    static AC_VERB_GET_PIN_WIDGET_CONTROL: c_uint;
    static AC_PINCTL_IN_EN: c_uint;
    static AC_PINCTL_OUT_EN: c_uint;
    static AC_PINCTL_HP_EN: c_uint;
    static AC_PINCTL_VREFEN: c_uint;
    static AC_PINCTL_VREF_HIZ: c_int;
    static AC_PINCTL_VREF_50: c_int;
    static AC_PINCTL_VREF_GRD: c_int;
    static AC_PINCTL_VREF_80: c_int;
    static AC_PINCTL_VREF_100: c_int;
    static AC_PAR_VOL_KNB_CAP: c_uint;
    static AC_VERB_GET_VOLUME_KNOB_CONTROL: c_uint;
    static AC_VERB_GET_CONV: c_uint;
    static AC_CONV_STREAM: c_uint;
    static AC_CONV_STREAM_SHIFT: c_uint;
    static AC_CONV_CHANNEL: c_uint;
    static AC_VERB_GET_SDI_SELECT: c_uint;
    static AC_SDI_SELECT: c_uint;
    static AC_VERB_GET_DIGI_CONVERT_1: c_uint;
    static AC_DIG1_ENABLE: c_uint;
    static AC_DIG1_V: c_uint;
    static AC_DIG1_VCFG: c_uint;
    static AC_DIG1_EMPHASIS: c_uint;
    static AC_DIG1_COPYRIGHT: c_uint;
    static AC_DIG1_NONAUDIO: c_uint;
    static AC_DIG1_PROFESSIONAL: c_uint;
    static AC_DIG1_LEVEL: c_uint;
    static AC_DIG3_KAE: u8;
    static AC_DIG2_CC: u8;
    static AC_DIG3_ICT: u8;
    static AC_PAR_POWER_STATE: c_uint;
    static AC_VERB_GET_POWER_STATE: c_uint;
    static AC_PWRST_D0SUP: c_uint;
    static AC_PWRST_D1SUP: c_uint;
    static AC_PWRST_D2SUP: c_uint;
    static AC_PWRST_D3SUP: c_uint;
    static AC_PWRST_D3COLDSUP: c_uint;
    static AC_PWRST_S3D3COLDSUP: c_uint;
    static AC_PWRST_CLKSTOP: c_uint;
    static AC_PWRST_EPSS: c_uint;
    static AC_PWRST_SETTING: c_uint;
    static AC_PWRST_ACTUAL: c_uint;
    static AC_PWRST_ACTUAL_SHIFT: c_uint;
    static AC_PWRST_ERROR: c_uint;
    static AC_PWRST_CLK_STOP_OK: c_uint;
    static AC_PWRST_SETTING_RESET: c_uint;
    static AC_VERB_GET_UNSOLICITED_RESPONSE: c_uint;
    static AC_UNSOL_TAG: c_uint;
    static AC_UNSOL_ENABLED: c_uint;
    static AC_PAR_PROC_CAP: c_uint;
    static AC_PCAP_NUM_COEF: c_uint;
    static AC_PCAP_NUM_COEF_SHIFT: c_uint;
    static AC_PCAP_BENIGN: c_uint;
    static AC_VERB_GET_COEF_INDEX: c_uint;
    static AC_VERB_SET_COEF_INDEX: c_uint;
    static AC_VERB_GET_PROC_COEF: c_uint;
    static AC_VERB_GET_CONNECT_SEL: c_uint;
    static AC_PAR_GPIO_CAP: c_uint;
    static AC_GPIO_IO_COUNT: c_uint;
    static AC_GPIO_O_COUNT: c_uint;
    static AC_GPIO_O_COUNT_SHIFT: c_uint;
    static AC_GPIO_I_COUNT: c_uint;
    static AC_GPIO_I_COUNT_SHIFT: c_uint;
    static AC_GPIO_UNSOLICITED: c_uint;
    static AC_GPIO_WAKE: c_uint;
    static AC_VERB_GET_GPIO_MASK: c_uint;
    static AC_VERB_GET_GPIO_DIRECTION: c_uint;
    static AC_VERB_GET_GPIO_WAKE_MASK: c_uint;
    static AC_VERB_GET_GPIO_UNSOLICITED_RSP_MASK: c_uint;
    static AC_VERB_GET_GPIO_STICKY_MASK: c_uint;
    static AC_VERB_GET_GPIO_DATA: c_uint;
    static AC_VERB_GET_GPO_DATA: c_uint;
    static AC_VERB_GET_GPI_WAKE_MASK: c_uint;
    static AC_VERB_GET_GPI_UNSOLICITED_RSP_MASK: c_uint;
    static AC_VERB_GET_GPI_STICKY_MASK: c_uint;
    static AC_VERB_GET_GPI_DATA: c_uint;
    static AC_MAX_DEV_LIST_LEN: c_uint;
    static AC_VERB_GET_DEVICE_SEL: c_uint;
    static AC_DE_PD: u8;
    static AC_DE_ELDV: u8;
    static AC_DE_IA: u8;
    static AC_WCAP_DIGITAL: c_uint;
    static AC_WCAP_IN_AMP: c_uint;
    static AC_WCAP_OUT_AMP: c_uint;
    static AC_WCAP_STRIPE: c_uint;
    static AC_WCAP_LR_SWAP: c_uint;
    static AC_WCAP_CP_CAPS: c_uint;
    static AC_WCAP_CONN_LIST: c_uint;
    static AC_WCAP_FORMAT_OVRD: c_uint;
    static AC_WCAP_UNSOL_CAP: c_uint;
    static AC_WCAP_POWER: c_uint;
    static AC_WCAP_DELAY: c_uint;
    static AC_WCAP_DELAY_SHIFT: c_uint;
    static AC_WCAP_PROC_WID: c_uint;
    static GFP_KERNEL: c_uint;
}

static mut dump_coef: c_int = -1;

unsafe fn param_read(codec: *mut hda_codec, nid: hda_nid_t, parm: c_uint) -> c_uint {
    unsafe { snd_hdac_read_parm_uncached(&mut (*codec).core, nid, parm) }
}

unsafe fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

unsafe fn get_wid_type_name(mut wid_value: c_uint) -> *const c_char {
    let mut names: [*const c_char; 16] = [ptr::null(); 16];
    names[AC_WID_AUD_OUT as usize] = cstr(b"Audio Output\0");
    names[AC_WID_AUD_IN as usize] = cstr(b"Audio Input\0");
    names[AC_WID_AUD_MIX as usize] = cstr(b"Audio Mixer\0");
    names[AC_WID_AUD_SEL as usize] = cstr(b"Audio Selector\0");
    names[AC_WID_PIN as usize] = cstr(b"Pin Complex\0");
    names[AC_WID_POWER as usize] = cstr(b"Power Widget\0");
    names[AC_WID_VOL_KNB as usize] = cstr(b"Volume Knob Widget\0");
    names[AC_WID_BEEP as usize] = cstr(b"Beep Generator Widget\0");
    names[AC_WID_VENDOR as usize] = cstr(b"Vendor Defined Widget\0");
    if wid_value == !0u32 {
        return cstr(b"UNKNOWN Widget\0");
    }
    wid_value &= 0xf;
    if !names[wid_value as usize].is_null() {
        names[wid_value as usize]
    } else {
        cstr(b"UNKNOWN Widget\0")
    }
}

unsafe fn print_nid_array(
    buffer: *mut snd_info_buffer,
    _codec: *mut hda_codec,
    nid: hda_nid_t,
    array: *mut snd_array,
) {
    let items = (*array).list as *mut hda_nid_item;
    for i in 0..(*array).used {
        let item = items.add(i as usize);
        if (*item).nid == nid {
            let kctl = (*item).kctl;
            snd_iprintf(
                buffer,
                cstr(b"  Control: name=\"%s\", index=%i, device=%i\n\0"),
                (*kctl).id.name.as_ptr(),
                (*kctl).id.index.wrapping_add((*item).index),
                (*kctl).id.device,
            );
            if ((*item).flags & HDA_NID_ITEM_AMP) != 0 {
                snd_iprintf(
                    buffer,
                    cstr(b"    ControlAmp: chs=%lu, dir=%s, idx=%lu, ofs=%lu\n\0"),
                    get_amp_channels(kctl),
                    if get_amp_direction(kctl) != 0 {
                        cstr(b"Out\0")
                    } else {
                        cstr(b"In\0")
                    },
                    get_amp_index(kctl),
                    get_amp_offset(kctl),
                );
            }
        }
    }
}

unsafe fn print_nid_pcms(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let mut pos = (*codec).pcm_list_head.next as *mut hda_pcm;
    let head = &mut (*codec).pcm_list_head as *mut list_head as *mut hda_pcm;
    while pos != head {
        for type_ in 0..2 {
            if (*pos).stream[type_].nid != nid || (*pos).pcm.is_null() {
                continue;
            }
            snd_iprintf(
                buffer,
                cstr(b"  Device: name=\"%s\", type=\"%s\", device=%i\n\0"),
                (*pos).name,
                snd_hda_pcm_type_name[(*pos).pcm_type as usize],
                (*(*pos).pcm).device,
            );
        }
        pos = (*pos).list.next as *mut hda_pcm;
    }
}

unsafe fn print_amp_caps(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t, dir: c_int) {
    let caps = param_read(
        codec,
        nid,
        if dir == HDA_OUTPUT {
            AC_PAR_AMP_OUT_CAP
        } else {
            AC_PAR_AMP_IN_CAP
        },
    );
    if caps == !0u32 || caps == 0 {
        snd_iprintf(buffer, cstr(b"N/A\n\0"));
        return;
    }
    snd_iprintf(
        buffer,
        cstr(b"ofs=0x%02x, nsteps=0x%02x, stepsize=0x%02x, mute=%x\n\0"),
        caps & AC_AMPCAP_OFFSET,
        (caps & AC_AMPCAP_NUM_STEPS) >> AC_AMPCAP_NUM_STEPS_SHIFT,
        (caps & AC_AMPCAP_STEP_SIZE) >> AC_AMPCAP_STEP_SIZE_SHIFT,
        (caps & AC_AMPCAP_MUTE) >> AC_AMPCAP_MUTE_SHIFT,
    );
}

/* is this a stereo widget or a stereo-to-mono mix? */
unsafe fn is_stereo_amps(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dir: c_int,
    mut wcaps: c_uint,
    indices: c_int,
) -> bool {
    let mut conn: hda_nid_t = 0;

    if (wcaps & AC_WCAP_STEREO) != 0 {
        return true;
    }
    /* check for a stereo-to-mono mix; it must be:
     * only a single connection, only for input, and only a mixer widget
     */
    if indices != 1 || dir != HDA_INPUT || get_wcaps_type(wcaps) != AC_WID_AUD_MIX {
        return false;
    }

    if snd_hda_get_raw_connections(codec, nid, &mut conn, 1) < 0 {
        return false;
    }
    /* the connection source is a stereo? */
    wcaps = snd_hda_param_read(codec, conn, AC_PAR_AUDIO_WIDGET_CAP);
    (wcaps & AC_WCAP_STEREO) != 0
}

unsafe fn print_amp_vals(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    mut dir: c_int,
    wcaps: c_uint,
    indices: c_int,
) {
    let stereo = is_stereo_amps(codec, nid, dir, wcaps, indices);

    dir = if dir == HDA_OUTPUT { AC_AMP_GET_OUTPUT } else { AC_AMP_GET_INPUT };
    for i in 0..indices {
        snd_iprintf(buffer, cstr(b" [\0"));
        let mut val = snd_hda_codec_read(
            codec,
            nid,
            0,
            AC_VERB_GET_AMP_GAIN_MUTE,
            AC_AMP_GET_LEFT | dir as c_uint | i as c_uint,
        );
        snd_iprintf(buffer, cstr(b"0x%02x\0"), val);
        if stereo {
            val = snd_hda_codec_read(
                codec,
                nid,
                0,
                AC_VERB_GET_AMP_GAIN_MUTE,
                AC_AMP_GET_RIGHT | dir as c_uint | i as c_uint,
            );
            snd_iprintf(buffer, cstr(b" 0x%02x\0"), val);
        }
        snd_iprintf(buffer, cstr(b"]\0"));
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
}

unsafe fn print_pcm_rates(buffer: *mut snd_info_buffer, mut pcm: c_uint) {
    static rates: [c_uint; 12] = [
        8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000, 384000,
    ];

    pcm &= AC_SUPPCM_RATES;
    snd_iprintf(buffer, cstr(b"    rates [0x%x]:\0"), pcm);
    for i in 0..rates.len() {
        if (pcm & (1u32 << i)) != 0 {
            snd_iprintf(buffer, cstr(b" %d\0"), rates[i]);
        }
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
}

unsafe fn print_pcm_bits(buffer: *mut snd_info_buffer, pcm: c_uint) {
    const SND_PRINT_BITS_ADVISED_BUFSIZE: usize = 80;
    let mut buf = [0 as c_char; SND_PRINT_BITS_ADVISED_BUFSIZE];

    snd_iprintf(buffer, cstr(b"    bits [0x%x]:\0"), (pcm >> 16) & 0xff);
    snd_print_pcm_bits(pcm, buf.as_mut_ptr(), buf.len());
    snd_iprintf(buffer, cstr(b"%s\n\0"), buf.as_ptr());
}

unsafe fn print_pcm_formats(buffer: *mut snd_info_buffer, streams: c_uint) {
    snd_iprintf(buffer, cstr(b"    formats [0x%x]:\0"), streams & 0xf);
    if (streams & AC_SUPFMT_PCM) != 0 {
        snd_iprintf(buffer, cstr(b" PCM\0"));
    }
    if (streams & AC_SUPFMT_FLOAT32) != 0 {
        snd_iprintf(buffer, cstr(b" FLOAT\0"));
    }
    if (streams & AC_SUPFMT_AC3) != 0 {
        snd_iprintf(buffer, cstr(b" AC3\0"));
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
}

unsafe fn print_pcm_caps(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let pcm = param_read(codec, nid, AC_PAR_PCM);
    let stream = param_read(codec, nid, AC_PAR_STREAM);
    if pcm == !0u32 || stream == !0u32 {
        snd_iprintf(buffer, cstr(b"N/A\n\0"));
        return;
    }
    print_pcm_rates(buffer, pcm);
    print_pcm_bits(buffer, pcm);
    print_pcm_formats(buffer, stream);
}

unsafe fn get_jack_connection(mut cfg: u32) -> *const c_char {
    let names = [
        cstr(b"Unknown\0"),
        cstr(b"1/8\0"),
        cstr(b"1/4\0"),
        cstr(b"ATAPI\0"),
        cstr(b"RCA\0"),
        cstr(b"Optical\0"),
        cstr(b"Digital\0"),
        cstr(b"Analog\0"),
        cstr(b"DIN\0"),
        cstr(b"XLR\0"),
        cstr(b"RJ11\0"),
        cstr(b"Comb\0"),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        cstr(b"Other\0"),
    ];
    cfg = (cfg & AC_DEFCFG_CONN_TYPE) >> AC_DEFCFG_CONN_TYPE_SHIFT;
    if !names[cfg as usize].is_null() {
        names[cfg as usize]
    } else {
        cstr(b"UNKNOWN\0")
    }
}

unsafe fn get_jack_color(mut cfg: u32) -> *const c_char {
    let names = [
        cstr(b"Unknown\0"),
        cstr(b"Black\0"),
        cstr(b"Grey\0"),
        cstr(b"Blue\0"),
        cstr(b"Green\0"),
        cstr(b"Red\0"),
        cstr(b"Orange\0"),
        cstr(b"Yellow\0"),
        cstr(b"Purple\0"),
        cstr(b"Pink\0"),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        cstr(b"White\0"),
        cstr(b"Other\0"),
    ];
    cfg = (cfg & AC_DEFCFG_COLOR) >> AC_DEFCFG_COLOR_SHIFT;
    if !names[cfg as usize].is_null() {
        names[cfg as usize]
    } else {
        cstr(b"UNKNOWN\0")
    }
}

/*
 * Parse the pin default config value and returns the string of the
 * jack location, e.g. "Rear", "Front", etc.
 */
unsafe fn get_jack_location(mut cfg: u32) -> *const c_char {
    let bases = [
        cstr(b"N/A\0"),
        cstr(b"Rear\0"),
        cstr(b"Front\0"),
        cstr(b"Left\0"),
        cstr(b"Right\0"),
        cstr(b"Top\0"),
        cstr(b"Bottom\0"),
    ];
    let specials_idx: [u8; 7] = [0x07, 0x08, 0x17, 0x18, 0x19, 0x37, 0x38];
    let specials = [
        cstr(b"Rear Panel\0"),
        cstr(b"Drive Bar\0"),
        cstr(b"Riser\0"),
        cstr(b"HDMI\0"),
        cstr(b"ATAPI\0"),
        cstr(b"Mobile-In\0"),
        cstr(b"Mobile-Out\0"),
    ];

    cfg = (cfg & AC_DEFCFG_LOCATION) >> AC_DEFCFG_LOCATION_SHIFT;
    if (cfg & 0x0f) < 7 {
        return bases[(cfg & 0x0f) as usize];
    }
    for i in 0..specials_idx.len() {
        if cfg == specials_idx[i] as u32 {
            return specials[i];
        }
    }
    cstr(b"UNKNOWN\0")
}

/*
 * Parse the pin default config value and returns the string of the
 * jack connectivity, i.e. external or internal connection.
 */
unsafe fn get_jack_connectivity(cfg: u32) -> *const c_char {
    let jack_locations = [cstr(b"Ext\0"), cstr(b"Int\0"), cstr(b"Sep\0"), cstr(b"Oth\0")];
    jack_locations[((cfg >> (AC_DEFCFG_LOCATION_SHIFT + 4)) & 3) as usize]
}

/*
 * Parse the pin default config value and returns the string of the
 * jack type, i.e. the purpose of the jack, such as Line-Out or CD.
 */
unsafe fn get_jack_type(cfg: u32) -> *const c_char {
    let jack_types = [
        cstr(b"Line Out\0"),
        cstr(b"Speaker\0"),
        cstr(b"HP Out\0"),
        cstr(b"CD\0"),
        cstr(b"SPDIF Out\0"),
        cstr(b"Digital Out\0"),
        cstr(b"Modem Line\0"),
        cstr(b"Modem Hand\0"),
        cstr(b"Line In\0"),
        cstr(b"Aux\0"),
        cstr(b"Mic\0"),
        cstr(b"Telephony\0"),
        cstr(b"SPDIF In\0"),
        cstr(b"Digital In\0"),
        cstr(b"Reserved\0"),
        cstr(b"Other\0"),
    ];
    jack_types[((cfg & AC_DEFCFG_DEVICE) >> AC_DEFCFG_DEVICE_SHIFT) as usize]
}

unsafe fn print_pin_caps(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    supports_vref: *mut c_int,
) {
    let jack_conns = [cstr(b"Jack\0"), cstr(b"N/A\0"), cstr(b"Fixed\0"), cstr(b"Both\0")];
    let mut caps = param_read(codec, nid, AC_PAR_PIN_CAP);
    snd_iprintf(buffer, cstr(b"  Pincap 0x%08x:\0"), caps);
    if (caps & AC_PINCAP_IN) != 0 {
        snd_iprintf(buffer, cstr(b" IN\0"));
    }
    if (caps & AC_PINCAP_OUT) != 0 {
        snd_iprintf(buffer, cstr(b" OUT\0"));
    }
    if (caps & AC_PINCAP_HP_DRV) != 0 {
        snd_iprintf(buffer, cstr(b" HP\0"));
    }
    if (caps & AC_PINCAP_EAPD) != 0 {
        snd_iprintf(buffer, cstr(b" EAPD\0"));
    }
    if (caps & AC_PINCAP_PRES_DETECT) != 0 {
        snd_iprintf(buffer, cstr(b" Detect\0"));
    }
    if (caps & AC_PINCAP_BALANCE) != 0 {
        snd_iprintf(buffer, cstr(b" Balanced\0"));
    }
    if (caps & AC_PINCAP_HDMI) != 0 {
        /* Realtek uses this bit as a different meaning */
        if ((*codec).core.vendor_id >> 16) == 0x10ec {
            snd_iprintf(buffer, cstr(b" R/L\0"));
        } else {
            if (caps & AC_PINCAP_HBR) != 0 {
                snd_iprintf(buffer, cstr(b" HBR\0"));
            }
            snd_iprintf(buffer, cstr(b" HDMI\0"));
        }
    }
    if (caps & AC_PINCAP_DP) != 0 {
        snd_iprintf(buffer, cstr(b" DP\0"));
    }
    if (caps & AC_PINCAP_TRIG_REQ) != 0 {
        snd_iprintf(buffer, cstr(b" Trigger\0"));
    }
    if (caps & AC_PINCAP_IMP_SENSE) != 0 {
        snd_iprintf(buffer, cstr(b" ImpSense\0"));
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
    if (caps & AC_PINCAP_VREF) != 0 {
        let vref = (caps & AC_PINCAP_VREF) >> AC_PINCAP_VREF_SHIFT;
        snd_iprintf(buffer, cstr(b"    Vref caps:\0"));
        if (vref & AC_PINCAP_VREF_HIZ) != 0 {
            snd_iprintf(buffer, cstr(b" HIZ\0"));
        }
        if (vref & AC_PINCAP_VREF_50) != 0 {
            snd_iprintf(buffer, cstr(b" 50\0"));
        }
        if (vref & AC_PINCAP_VREF_GRD) != 0 {
            snd_iprintf(buffer, cstr(b" GRD\0"));
        }
        if (vref & AC_PINCAP_VREF_80) != 0 {
            snd_iprintf(buffer, cstr(b" 80\0"));
        }
        if (vref & AC_PINCAP_VREF_100) != 0 {
            snd_iprintf(buffer, cstr(b" 100\0"));
        }
        snd_iprintf(buffer, cstr(b"\n\0"));
        *supports_vref = 1;
    } else {
        *supports_vref = 0;
    }
    if (caps & AC_PINCAP_EAPD) != 0 {
        let val = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_EAPD_BTLENABLE, 0);
        snd_iprintf(buffer, cstr(b"  EAPD 0x%x:\0"), val);
        if (val & AC_EAPDBTL_BALANCED) != 0 {
            snd_iprintf(buffer, cstr(b" BALANCED\0"));
        }
        if (val & AC_EAPDBTL_EAPD) != 0 {
            snd_iprintf(buffer, cstr(b" EAPD\0"));
        }
        if (val & AC_EAPDBTL_LR_SWAP) != 0 {
            snd_iprintf(buffer, cstr(b" R/L\0"));
        }
        snd_iprintf(buffer, cstr(b"\n\0"));
    }
    caps = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_CONFIG_DEFAULT, 0);
    snd_iprintf(
        buffer,
        cstr(b"  Pin Default 0x%08x: [%s] %s at %s %s\n\0"),
        caps,
        jack_conns[((caps & AC_DEFCFG_PORT_CONN) >> AC_DEFCFG_PORT_CONN_SHIFT) as usize],
        get_jack_type(caps),
        get_jack_connectivity(caps),
        get_jack_location(caps),
    );
    snd_iprintf(
        buffer,
        cstr(b"    Conn = %s, Color = %s\n\0"),
        get_jack_connection(caps),
        get_jack_color(caps),
    );
    /* Default association and sequence values refer to default grouping
     * of pin complexes and their sequence within the group. This is used
     * for priority and resource allocation.
     */
    snd_iprintf(
        buffer,
        cstr(b"    DefAssociation = 0x%x, Sequence = 0x%x\n\0"),
        (caps & AC_DEFCFG_DEF_ASSOC) >> AC_DEFCFG_ASSOC_SHIFT,
        caps & AC_DEFCFG_SEQUENCE,
    );
    if (((caps & AC_DEFCFG_MISC) >> AC_DEFCFG_MISC_SHIFT) & AC_DEFCFG_MISC_NO_PRESENCE) != 0 {
        /* Miscellaneous bit indicates external hardware does not
         * support presence detection even if the pin complex
         * indicates it is supported.
         */
        snd_iprintf(buffer, cstr(b"    Misc = NO_PRESENCE\n\0"));
    }
}

unsafe fn print_pin_ctls(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    supports_vref: c_int,
) {
    let pinctls = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
    snd_iprintf(buffer, cstr(b"  Pin-ctls: 0x%02x:\0"), pinctls);
    if (pinctls & AC_PINCTL_IN_EN) != 0 {
        snd_iprintf(buffer, cstr(b" IN\0"));
    }
    if (pinctls & AC_PINCTL_OUT_EN) != 0 {
        snd_iprintf(buffer, cstr(b" OUT\0"));
    }
    if (pinctls & AC_PINCTL_HP_EN) != 0 {
        snd_iprintf(buffer, cstr(b" HP\0"));
    }
    if supports_vref != 0 {
        let vref = (pinctls & AC_PINCTL_VREFEN) as c_int;
        match vref {
            x if x == AC_PINCTL_VREF_HIZ => snd_iprintf(buffer, cstr(b" VREF_HIZ\0")),
            x if x == AC_PINCTL_VREF_50 => snd_iprintf(buffer, cstr(b" VREF_50\0")),
            x if x == AC_PINCTL_VREF_GRD => snd_iprintf(buffer, cstr(b" VREF_GRD\0")),
            x if x == AC_PINCTL_VREF_80 => snd_iprintf(buffer, cstr(b" VREF_80\0")),
            x if x == AC_PINCTL_VREF_100 => snd_iprintf(buffer, cstr(b" VREF_100\0")),
            _ => {}
        }
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
}

unsafe fn print_vol_knob(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let mut cap = param_read(codec, nid, AC_PAR_VOL_KNB_CAP);
    snd_iprintf(
        buffer,
        cstr(b"  Volume-Knob: delta=%d, steps=%d, \0"),
        (cap >> 7) & 1,
        cap & 0x7f,
    );
    cap = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_VOLUME_KNOB_CONTROL, 0);
    snd_iprintf(
        buffer,
        cstr(b"direct=%d, val=%d\n\0"),
        (cap >> 7) & 1,
        cap & 0x7f,
    );
}

unsafe fn print_audio_io(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    wid_type: c_uint,
) {
    let conv = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_CONV, 0);
    snd_iprintf(
        buffer,
        cstr(b"  Converter: stream=%d, channel=%d\n\0"),
        (conv & AC_CONV_STREAM) >> AC_CONV_STREAM_SHIFT,
        conv & AC_CONV_CHANNEL,
    );

    if wid_type == AC_WID_AUD_IN && (conv & AC_CONV_CHANNEL) == 0 {
        let sdi = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_SDI_SELECT, 0);
        snd_iprintf(buffer, cstr(b"  SDI-Select: %d\n\0"), sdi & AC_SDI_SELECT);
    }
}

unsafe fn print_digital_conv(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let digi1 = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_DIGI_CONVERT_1, 0);
    let digi2 = (digi1 >> 8) as u8;
    let digi3 = (digi1 >> 16) as u8;

    snd_iprintf(buffer, cstr(b"  Digital:\0"));
    if (digi1 & AC_DIG1_ENABLE) != 0 {
        snd_iprintf(buffer, cstr(b" Enabled\0"));
    }
    if (digi1 & AC_DIG1_V) != 0 {
        snd_iprintf(buffer, cstr(b" Validity\0"));
    }
    if (digi1 & AC_DIG1_VCFG) != 0 {
        snd_iprintf(buffer, cstr(b" ValidityCfg\0"));
    }
    if (digi1 & AC_DIG1_EMPHASIS) != 0 {
        snd_iprintf(buffer, cstr(b" Preemphasis\0"));
    }
    if (digi1 & AC_DIG1_COPYRIGHT) != 0 {
        snd_iprintf(buffer, cstr(b" Non-Copyright\0"));
    }
    if (digi1 & AC_DIG1_NONAUDIO) != 0 {
        snd_iprintf(buffer, cstr(b" Non-Audio\0"));
    }
    if (digi1 & AC_DIG1_PROFESSIONAL) != 0 {
        snd_iprintf(buffer, cstr(b" Pro\0"));
    }
    if (digi1 & AC_DIG1_LEVEL) != 0 {
        snd_iprintf(buffer, cstr(b" GenLevel\0"));
    }
    if (digi3 & AC_DIG3_KAE) != 0 {
        snd_iprintf(buffer, cstr(b" KAE\0"));
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
    snd_iprintf(buffer, cstr(b"  Digital category: 0x%x\n\0"), digi2 & AC_DIG2_CC);
    snd_iprintf(buffer, cstr(b"  IEC Coding Type: 0x%x\n\0"), digi3 & AC_DIG3_ICT);
}

unsafe fn get_pwr_state(state: u32) -> *const c_char {
    let buf = [
        cstr(b"D0\0"),
        cstr(b"D1\0"),
        cstr(b"D2\0"),
        cstr(b"D3\0"),
        cstr(b"D3cold\0"),
    ];
    if (state as usize) < buf.len() {
        return buf[state as usize];
    }
    cstr(b"UNKNOWN\0")
}

unsafe fn ilog2(mut v: c_uint) -> usize {
    let mut r = 0usize;
    while v > 1 {
        v >>= 1;
        r += 1;
    }
    r
}

unsafe fn print_power_state(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let mut names: [*const c_char; 32] = [ptr::null(); 32];
    names[ilog2(AC_PWRST_D0SUP)] = cstr(b"D0\0");
    names[ilog2(AC_PWRST_D1SUP)] = cstr(b"D1\0");
    names[ilog2(AC_PWRST_D2SUP)] = cstr(b"D2\0");
    names[ilog2(AC_PWRST_D3SUP)] = cstr(b"D3\0");
    names[ilog2(AC_PWRST_D3COLDSUP)] = cstr(b"D3cold\0");
    names[ilog2(AC_PWRST_S3D3COLDSUP)] = cstr(b"S3D3cold\0");
    names[ilog2(AC_PWRST_CLKSTOP)] = cstr(b"CLKSTOP\0");
    names[ilog2(AC_PWRST_EPSS)] = cstr(b"EPSS\0");

    let sup = param_read(codec, nid, AC_PAR_POWER_STATE) as c_int;
    let pwr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_POWER_STATE, 0);
    if sup != -1 {
        snd_iprintf(buffer, cstr(b"  Power states: \0"));
        for i in 0..names.len() {
            if (sup as c_uint & (1u32 << i)) != 0 {
                snd_iprintf(buffer, cstr(b" %s\0"), names[i]);
            }
        }
        snd_iprintf(buffer, cstr(b"\n\0"));
    }

    snd_iprintf(
        buffer,
        cstr(b"  Power: setting=%s, actual=%s\0"),
        get_pwr_state(pwr & AC_PWRST_SETTING),
        get_pwr_state((pwr & AC_PWRST_ACTUAL) >> AC_PWRST_ACTUAL_SHIFT),
    );
    if (pwr & AC_PWRST_ERROR) != 0 {
        snd_iprintf(buffer, cstr(b", Error\0"));
    }
    if (pwr & AC_PWRST_CLK_STOP_OK) != 0 {
        snd_iprintf(buffer, cstr(b", Clock-stop-OK\0"));
    }
    if (pwr & AC_PWRST_SETTING_RESET) != 0 {
        snd_iprintf(buffer, cstr(b", Setting-reset\0"));
    }
    snd_iprintf(buffer, cstr(b"\n\0"));
}

unsafe fn print_unsol_cap(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let unsol = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_UNSOLICITED_RESPONSE, 0);
    snd_iprintf(
        buffer,
        cstr(b"  Unsolicited: tag=%02x, enabled=%d\n\0"),
        unsol & AC_UNSOL_TAG,
        if (unsol & AC_UNSOL_ENABLED) != 0 { 1 } else { 0 },
    );
}

unsafe fn can_dump_coef(codec: *mut hda_codec) -> bool {
    match dump_coef {
        0 => false,
        1 => true,
        _ => (*codec).dump_coef,
    }
}

unsafe fn print_proc_caps(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let proc_caps = param_read(codec, nid, AC_PAR_PROC_CAP);
    let ncoeff = (proc_caps & AC_PCAP_NUM_COEF) >> AC_PCAP_NUM_COEF_SHIFT;
    snd_iprintf(
        buffer,
        cstr(b"  Processing caps: benign=%d, ncoeff=%d\n\0"),
        proc_caps & AC_PCAP_BENIGN,
        ncoeff,
    );

    if !can_dump_coef(codec) {
        return;
    }

    /* Note: This is racy - another process could run in parallel and change
       the coef index too. */
    let oldindex = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_COEF_INDEX, 0);
    for i in 0..ncoeff {
        snd_hda_codec_write(codec, nid, 0, AC_VERB_SET_COEF_INDEX, i);
        let val = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_PROC_COEF, 0);
        snd_iprintf(buffer, cstr(b"    Coeff 0x%02x: 0x%04x\n\0"), i, val);
    }
    snd_hda_codec_write(codec, nid, 0, AC_VERB_SET_COEF_INDEX, oldindex);
}

unsafe fn print_conn_list(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    wid_type: c_uint,
    conn: *mut hda_nid_t,
    conn_len: c_int,
) {
    let mut curr = -1;
    let mut list: *const hda_nid_t = ptr::null();

    if conn_len > 1 && wid_type != AC_WID_AUD_MIX && wid_type != AC_WID_VOL_KNB && wid_type != AC_WID_POWER {
        curr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_CONNECT_SEL, 0) as c_int;
    }
    snd_iprintf(buffer, cstr(b"  Connection: %d\n\0"), conn_len);
    if conn_len > 0 {
        snd_iprintf(buffer, cstr(b"    \0"));
        for c in 0..conn_len {
            snd_iprintf(buffer, cstr(b" 0x%02x\0"), *conn.add(c as usize));
            if c == curr {
                snd_iprintf(buffer, cstr(b"*\0"));
            }
        }
        snd_iprintf(buffer, cstr(b"\n\0"));
    }

    /* Get Cache connections info */
    let cache_len = snd_hda_get_conn_list(codec, nid, &mut list);
    if cache_len >= 0
        && (cache_len != conn_len
            || memcmp(
                list as *const c_void,
                conn as *const c_void,
                conn_len as usize * size_of::<hda_nid_t>(),
            ) != 0)
    {
        snd_iprintf(buffer, cstr(b"  In-driver Connection: %d\n\0"), cache_len);
        if cache_len > 0 {
            snd_iprintf(buffer, cstr(b"    \0"));
            for c in 0..cache_len {
                snd_iprintf(buffer, cstr(b" 0x%02x\0"), *list.add(c as usize));
            }
            snd_iprintf(buffer, cstr(b"\n\0"));
        }
    }
}

unsafe fn print_gpio(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let gpio = param_read(codec, (*codec).core.afg, AC_PAR_GPIO_CAP);
    let gpio_max = (gpio & AC_GPIO_IO_COUNT) as c_int;
    let gpo_max = ((gpio & AC_GPIO_O_COUNT) >> AC_GPIO_O_COUNT_SHIFT) as c_int;
    let gpi_max = ((gpio & AC_GPIO_I_COUNT) >> AC_GPIO_I_COUNT_SHIFT) as c_int;

    snd_iprintf(
        buffer,
        cstr(b"GPIO: io=%d, o=%d, i=%d, unsolicited=%d, wake=%d\n\0"),
        gpio_max,
        gpo_max,
        gpi_max,
        if (gpio & AC_GPIO_UNSOLICITED) != 0 { 1 } else { 0 },
        if (gpio & AC_GPIO_WAKE) != 0 { 1 } else { 0 },
    );

    if gpio_max != 0 && gpio_max <= 8 {
        let enable = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_MASK, 0);
        let direction = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_DIRECTION, 0);
        let wake = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_WAKE_MASK, 0);
        let unsol = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_UNSOLICITED_RSP_MASK, 0);
        let sticky = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_STICKY_MASK, 0);
        let data = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPIO_DATA, 0);
        for i in 0..gpio_max {
            snd_iprintf(
                buffer,
                cstr(b"  IO[%d]: enable=%d, dir=%d, wake=%d, \0"),
                i,
                if (enable & (1u32 << i)) != 0 { 1 } else { 0 },
                if (direction & (1u32 << i)) != 0 { 1 } else { 0 },
                if (wake & (1u32 << i)) != 0 { 1 } else { 0 },
            );
            snd_iprintf(
                buffer,
                cstr(b"sticky=%d, data=%d, unsol=%d\n\0"),
                if (sticky & (1u32 << i)) != 0 { 1 } else { 0 },
                if (data & (1u32 << i)) != 0 { 1 } else { 0 },
                if (unsol & (1u32 << i)) != 0 { 1 } else { 0 },
            );
        }
    }

    if gpo_max != 0 && gpo_max <= 8 {
        let gpo_data = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPO_DATA, 0);
        for i in 0..gpo_max {
            snd_iprintf(
                buffer,
                cstr(b"  GPO[%d]: data=%d\n\0"),
                i,
                if (gpo_data & (1u32 << i)) != 0 { 1 } else { 0 },
            );
        }
    }

    if gpi_max != 0 && gpi_max <= 8 {
        let wake = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPI_WAKE_MASK, 0);
        let unsol = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPI_UNSOLICITED_RSP_MASK, 0);
        let sticky = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPI_STICKY_MASK, 0);
        let data = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_GPI_DATA, 0);
        for i in 0..gpi_max {
            snd_iprintf(
                buffer,
                cstr(b"  GPI[%d]: wake=%d, sticky=%d, data=%d, unsol=%d\n\0"),
                i,
                if (wake & (1u32 << i)) != 0 { 1 } else { 0 },
                if (sticky & (1u32 << i)) != 0 { 1 } else { 0 },
                if (data & (1u32 << i)) != 0 { 1 } else { 0 },
                if (unsol & (1u32 << i)) != 0 { 1 } else { 0 },
            );
        }
    }

    print_nid_array(buffer, codec, nid, &mut (*codec).mixers);
    print_nid_array(buffer, codec, nid, &mut (*codec).nids);
}

unsafe fn print_dpmst_connections(
    buffer: *mut snd_info_buffer,
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_num: c_int,
) {
    let conn_len = snd_hda_get_num_raw_conns(codec, nid);
    if conn_len <= 0 {
        return;
    }

    let conn = kmalloc(conn_len as usize * size_of::<hda_nid_t>(), GFP_KERNEL) as *mut hda_nid_t;
    if conn.is_null() {
        return;
    }

    let dev_id_saved = snd_hda_get_dev_select(codec, nid);

    snd_hda_set_dev_select(codec, nid, dev_num);
    let curr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_CONNECT_SEL, 0) as c_int;
    if snd_hda_get_raw_connections(codec, nid, conn, conn_len) < 0 {
        kfree(conn as *mut c_void);
        snd_hda_set_dev_select(codec, nid, dev_id_saved);
        return;
    }

    for c in 0..conn_len {
        snd_iprintf(buffer, cstr(b" 0x%02x\0"), *conn.add(c as usize));
        if c == curr {
            snd_iprintf(buffer, cstr(b"*\0"));
        }
    }

    kfree(conn as *mut c_void);
    snd_hda_set_dev_select(codec, nid, dev_id_saved);
}

unsafe fn print_device_list(buffer: *mut snd_info_buffer, codec: *mut hda_codec, nid: hda_nid_t) {
    let mut dev_list = [0u8; AC_MAX_DEV_LIST_LEN as usize];
    let devlist_len = snd_hda_get_devices(codec, nid, dev_list.as_mut_ptr(), AC_MAX_DEV_LIST_LEN);
    snd_iprintf(buffer, cstr(b"  Devices: %u\n\0"), devlist_len);
    if devlist_len == 0 {
        return;
    }

    let curr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_DEVICE_SEL, 0) as c_int;

    for i in 0..devlist_len as c_int {
        if i == curr {
            snd_iprintf(buffer, cstr(b"    *\0"));
        } else {
            snd_iprintf(buffer, cstr(b"     \0"));
        }

        snd_iprintf(
            buffer,
            cstr(b"Dev %02d: PD = %d, ELDV = %d, IA = %d, Connections [\0"),
            i,
            if (dev_list[i as usize] & AC_DE_PD) != 0 { 1 } else { 0 },
            if (dev_list[i as usize] & AC_DE_ELDV) != 0 { 1 } else { 0 },
            if (dev_list[i as usize] & AC_DE_IA) != 0 { 1 } else { 0 },
        );

        print_dpmst_connections(buffer, codec, nid, i);

        snd_iprintf(buffer, cstr(b" ]\n\0"));
    }
}

unsafe fn print_codec_core_info(codec: *mut hdac_device, buffer: *mut snd_info_buffer) {
    snd_iprintf(buffer, cstr(b"Codec: \0"));
    if !(*codec).vendor_name.is_null() && !(*codec).chip_name.is_null() {
        snd_iprintf(buffer, cstr(b"%s %s\n\0"), (*codec).vendor_name, (*codec).chip_name);
    } else {
        snd_iprintf(buffer, cstr(b"Not Set\n\0"));
    }
    snd_iprintf(buffer, cstr(b"Address: %d\n\0"), (*codec).addr);
    if (*codec).afg != 0 {
        snd_iprintf(
            buffer,
            cstr(b"AFG Function Id: 0x%x (unsol %u)\n\0"),
            (*codec).afg_function_id,
            (*codec).afg_unsol,
        );
    }
    if (*codec).mfg != 0 {
        snd_iprintf(
            buffer,
            cstr(b"MFG Function Id: 0x%x (unsol %u)\n\0"),
            (*codec).mfg_function_id,
            (*codec).mfg_unsol,
        );
    }
    snd_iprintf(buffer, cstr(b"Vendor Id: 0x%08x\n\0"), (*codec).vendor_id);
    snd_iprintf(buffer, cstr(b"Subsystem Id: 0x%08x\n\0"), (*codec).subsystem_id);
    snd_iprintf(buffer, cstr(b"Revision Id: 0x%x\n\0"), (*codec).revision_id);

    if (*codec).mfg != 0 {
        snd_iprintf(buffer, cstr(b"Modem Function Group: 0x%x\n\0"), (*codec).mfg);
    } else {
        snd_iprintf(buffer, cstr(b"No Modem Function Group found\n\0"));
    }
}

unsafe extern "C" fn print_codec_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let codec = (*entry).private_data as *mut hda_codec;
    let mut nid: hda_nid_t = 0;

    print_codec_core_info(&mut (*codec).core, buffer);
    let fg = (*codec).core.afg;
    if fg == 0 {
        return;
    }
    snd_hda_power_pm(codec);
    snd_iprintf(buffer, cstr(b"Default PCM:\n\0"));
    print_pcm_caps(buffer, codec, fg);
    snd_iprintf(buffer, cstr(b"Default Amp-In caps: \0"));
    print_amp_caps(buffer, codec, fg, HDA_INPUT);
    snd_iprintf(buffer, cstr(b"Default Amp-Out caps: \0"));
    print_amp_caps(buffer, codec, fg, HDA_OUTPUT);
    snd_iprintf(buffer, cstr(b"State of AFG node 0x%02x:\n\0"), fg);
    print_power_state(buffer, codec, fg);

    let nodes = snd_hda_get_sub_nodes(codec, fg, &mut nid);
    if nid == 0 || nodes < 0 {
        snd_iprintf(buffer, cstr(b"Invalid AFG subtree\n\0"));
        return;
    }

    print_gpio(buffer, codec, fg);
    if let Some(hook) = (*codec).proc_widget_hook {
        hook(buffer, codec, fg);
    }

    for _i in 0..nodes {
        let mut wid_caps = param_read(codec, nid, AC_PAR_AUDIO_WIDGET_CAP);
        let wid_type = get_wcaps_type(wid_caps);
        let mut conn: *mut hda_nid_t = ptr::null_mut();
        let mut conn_len: c_int = 0;

        snd_iprintf(
            buffer,
            cstr(b"Node 0x%02x [%s] wcaps 0x%x:\0"),
            nid,
            get_wid_type_name(wid_type),
            wid_caps,
        );
        if (wid_caps & AC_WCAP_STEREO) != 0 {
            let chans = get_wcaps_channels(wid_caps);
            if chans == 2 {
                snd_iprintf(buffer, cstr(b" Stereo\0"));
            } else {
                snd_iprintf(buffer, cstr(b" %d-Channels\0"), chans);
            }
        } else {
            snd_iprintf(buffer, cstr(b" Mono\0"));
        }
        if (wid_caps & AC_WCAP_DIGITAL) != 0 {
            snd_iprintf(buffer, cstr(b" Digital\0"));
        }
        if (wid_caps & AC_WCAP_IN_AMP) != 0 {
            snd_iprintf(buffer, cstr(b" Amp-In\0"));
        }
        if (wid_caps & AC_WCAP_OUT_AMP) != 0 {
            snd_iprintf(buffer, cstr(b" Amp-Out\0"));
        }
        if (wid_caps & AC_WCAP_STRIPE) != 0 {
            snd_iprintf(buffer, cstr(b" Stripe\0"));
        }
        if (wid_caps & AC_WCAP_LR_SWAP) != 0 {
            snd_iprintf(buffer, cstr(b" R/L\0"));
        }
        if (wid_caps & AC_WCAP_CP_CAPS) != 0 {
            snd_iprintf(buffer, cstr(b" CP\0"));
        }
        snd_iprintf(buffer, cstr(b"\n\0"));

        print_nid_array(buffer, codec, nid, &mut (*codec).mixers);
        print_nid_array(buffer, codec, nid, &mut (*codec).nids);
        print_nid_pcms(buffer, codec, nid);

        /* volume knob is a special widget that always have connection
         * list
         */
        if wid_type == AC_WID_VOL_KNB {
            wid_caps |= AC_WCAP_CONN_LIST;
        }

        if (wid_caps & AC_WCAP_CONN_LIST) != 0 {
            conn_len = snd_hda_get_num_raw_conns(codec, nid);
            if conn_len > 0 {
                conn = kmalloc(conn_len as usize * size_of::<hda_nid_t>(), GFP_KERNEL) as *mut hda_nid_t;
                if conn.is_null() {
                    return;
                }
                if snd_hda_get_raw_connections(codec, nid, conn, conn_len) < 0 {
                    conn_len = 0;
                }
            }
        }

        if (wid_caps & AC_WCAP_IN_AMP) != 0 {
            snd_iprintf(buffer, cstr(b"  Amp-In caps: \0"));
            print_amp_caps(buffer, codec, nid, HDA_INPUT);
            snd_iprintf(buffer, cstr(b"  Amp-In vals: \0"));
            if wid_type == AC_WID_PIN || ((*codec).single_adc_amp && wid_type == AC_WID_AUD_IN) {
                print_amp_vals(buffer, codec, nid, HDA_INPUT, wid_caps, 1);
            } else {
                print_amp_vals(buffer, codec, nid, HDA_INPUT, wid_caps, conn_len);
            }
        }
        if (wid_caps & AC_WCAP_OUT_AMP) != 0 {
            snd_iprintf(buffer, cstr(b"  Amp-Out caps: \0"));
            print_amp_caps(buffer, codec, nid, HDA_OUTPUT);
            snd_iprintf(buffer, cstr(b"  Amp-Out vals: \0"));
            if wid_type == AC_WID_PIN && (*codec).pin_amp_workaround {
                print_amp_vals(buffer, codec, nid, HDA_OUTPUT, wid_caps, conn_len);
            } else {
                print_amp_vals(buffer, codec, nid, HDA_OUTPUT, wid_caps, 1);
            }
        }

        if wid_type == AC_WID_PIN {
            let mut supports_vref: c_int = 0;
            print_pin_caps(buffer, codec, nid, &mut supports_vref);
            print_pin_ctls(buffer, codec, nid, supports_vref);
        } else if wid_type == AC_WID_VOL_KNB {
            print_vol_knob(buffer, codec, nid);
        } else if wid_type == AC_WID_AUD_OUT || wid_type == AC_WID_AUD_IN {
            print_audio_io(buffer, codec, nid, wid_type);
            if (wid_caps & AC_WCAP_DIGITAL) != 0 {
                print_digital_conv(buffer, codec, nid);
            }
            if (wid_caps & AC_WCAP_FORMAT_OVRD) != 0 {
                snd_iprintf(buffer, cstr(b"  PCM:\n\0"));
                print_pcm_caps(buffer, codec, nid);
            }
        }

        if (wid_caps & AC_WCAP_UNSOL_CAP) != 0 {
            print_unsol_cap(buffer, codec, nid);
        }

        if (wid_caps & AC_WCAP_POWER) != 0 {
            print_power_state(buffer, codec, nid);
        }

        if (wid_caps & AC_WCAP_DELAY) != 0 {
            snd_iprintf(
                buffer,
                cstr(b"  Delay: %d samples\n\0"),
                (wid_caps & AC_WCAP_DELAY) >> AC_WCAP_DELAY_SHIFT,
            );
        }

        if wid_type == AC_WID_PIN && (*codec).dp_mst {
            print_device_list(buffer, codec, nid);
        }

        if (wid_caps & AC_WCAP_CONN_LIST) != 0 {
            print_conn_list(buffer, codec, nid, wid_type, conn, conn_len);
        }

        if (wid_caps & AC_WCAP_PROC_WID) != 0 {
            print_proc_caps(buffer, codec, nid);
        }

        if let Some(hook) = (*codec).proc_widget_hook {
            hook(buffer, codec, nid);
        }

        kfree(conn as *mut c_void);
        nid = nid.wrapping_add(1);
    }
}

/*
 * create a proc read
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hda_codec_proc_new(codec: *mut hda_codec) -> c_int {
    let mut name = [0 as c_char; 32];

    snprintf(
        name.as_mut_ptr(),
        name.len(),
        cstr(b"codec#%d\0"),
        (*codec).core.addr,
    );
    snd_card_ro_proc_new((*codec).card, name.as_ptr(), codec, print_codec_info)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
