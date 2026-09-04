// SPDX-License-Identifier: GPL-2.0-or-later

// Linux kernel USB audio quirks
// Ported from: include/linux/cleanup.h, include/linux/err.h, include/linux/init.h,
// include/linux/slab.h, include/linux/string.h, include/linux/usb.h,
// include/linux/usb/audio.h, include/linux/usb/midi.h, include/linux/bits.h,
// sound/core.h, sound/info.h, sound/pcm.h

use core::mem;

// External dependencies (to be linked from other modules)
// usbaudio.h, card.h, mixer.h, mixer_quirks.h, midi.h, midi2.h, quirks.h,
// helper.h, endpoint.h, pcm.h, clock.h, stream.h

// Type definitions (stub declarations for external types)
#[repr(C)]
pub struct snd_usb_audio {
    pub dev: *mut usb_device,
    pub usb_id: u32,
    pub setup: u32,
    pub quirk_flags: u64,
    pub card: *mut snd_card,
    pub midi_list: list_head,
    pub num_rawmidis: i32,
}

#[repr(C)]
pub struct usb_interface {
    pub altsetting: *mut usb_host_interface,
    pub num_altsetting: u32,
    pub dev: device,
}

#[repr(C)]
pub struct usb_driver {
    // Driver structure
}

#[repr(C)]
pub struct snd_usb_audio_quirk {
    pub ifnum: i32,
    pub type_: u16,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct usb_device {
    pub dev: device,
    pub actconfig: *mut usb_host_config,
    pub descriptor: usb_device_descriptor,
    pub manufacturer: *const core::ffi::c_char,
    pub product: *const core::ffi::c_char,
}

#[repr(C)]
pub struct usb_host_interface {
    pub altsetting: usb_interface_descriptor,
    pub endpoint: *mut usb_endpoint_descriptor,
    pub extra: *mut u8,
    pub extralen: i32,
}

#[repr(C)]
pub struct usb_interface_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

#[repr(C)]
pub struct usb_endpoint_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}

#[repr(C)]
pub struct usb_device_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

#[repr(C)]
pub struct usb_host_config {
    pub desc: usb_config_descriptor,
}

#[repr(C)]
pub struct usb_config_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub bMaxPower: u8,
}

#[repr(C)]
pub struct audioformat {
    pub list: list_head,
    pub iface: u32,
    pub altsetting: u32,
    pub altset_idx: u32,
    pub endpoint: u32,
    pub ep_attr: u32,
    pub formats: u64,
    pub channels: u32,
    pub fmt_type: u32,
    pub sync_ep: u32,
    pub implicit_fb: u32,
    pub protocol: u32,
    pub datainterval: u32,
    pub maxpacksize: u32,
    pub nr_rates: u32,
    pub rate_table: *mut u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub dsd_dop: bool,
    pub dsd_bitrev: bool,
    pub dsd_raw: bool,
    pub attributes: u32,
}

#[repr(C)]
pub struct snd_usb_substream {
    pub stream: *mut snd_usb_stream,
    pub dev: *mut usb_device,
    pub data_endpoint: *mut snd_usb_endpoint,
    pub direction: u32,
}

#[repr(C)]
pub struct snd_usb_stream {
    pub chip: *mut snd_usb_audio,
    pub substream: [Option<snd_usb_substream>; 2],
}

#[repr(C)]
pub struct snd_usb_endpoint {
    pub chip: *mut snd_usb_audio,
    pub type_: u32,
    pub cur_rate: u32,
    pub syncmaxsize: u32,
    pub skip_packets: u32,
    pub tenor_fb_quirk: u32,
}

#[repr(C)]
pub struct snd_card {
    // Card structure
}

#[repr(C)]
pub struct device {
    // Device structure
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct usb_string_match {
    pub manufacturer: *const core::ffi::c_char,
    pub product: *const core::ffi::c_char,
}

#[repr(C)]
pub struct usb_audio_quirk_flags_table {
    pub id: u32,
    pub flags: u64,
    pub usb_string_match: *const usb_string_match,
}

// Constants
pub const EXTIGY_FIRMWARE_SIZE_OLD: usize = 794;
pub const EXTIGY_FIRMWARE_SIZE_NEW: usize = 483;

pub const MICROBOOK_BUF_SIZE: usize = 128;

pub const MBOX2_FIRMWARE_SIZE: usize = 646;
pub const MBOX2_BOOT_LOADING: u8 = 0x01;
pub const MBOX2_BOOT_READY: u8 = 0x02;

pub const MBOX3_DESCRIPTOR_SIZE: usize = 464;

pub const MAUDIO_SET: u32 = 0x01;
pub const MAUDIO_SET_COMPATIBLE: u32 = 0x80;
pub const MAUDIO_SET_DTS: u32 = 0x02;
pub const MAUDIO_SET_96K: u32 = 0x04;
pub const MAUDIO_SET_24B: u32 = 0x08;
pub const MAUDIO_SET_DI: u32 = 0x10;
pub const MAUDIO_SET_MASK: u32 = 0x1f;
pub const MAUDIO_SET_24B_48K_DI: u32 = 0x19;
pub const MAUDIO_SET_24B_48K_NOTDI: u32 = 0x09;
pub const MAUDIO_SET_16B_48K_DI: u32 = 0x11;
pub const MAUDIO_SET_16B_48K_NOTDI: u32 = 0x01;

pub const CM6206_REG0_DMA_MASTER: u32 = 1 << 15;
pub const CM6206_REG0_SPDIFO_RATE_48K: u32 = 2 << 12;
pub const CM6206_REG0_SPDIFO_RATE_96K: u32 = 7 << 12;
pub const CM6206_REG0_SPDIFO_CAT_CODE_GENERAL: u32 = 0 << 4;
pub const CM6206_REG0_SPDIFO_EMPHASIS_CD: u32 = 1 << 3;
pub const CM6206_REG0_SPDIFO_COPYRIGHT_NA: u32 = 1 << 2;
pub const CM6206_REG0_SPDIFO_NON_AUDIO: u32 = 1 << 1;
pub const CM6206_REG0_SPDIFO_PRO_FORMAT: u32 = 1 << 0;

pub const CM6206_REG1_TEST_SEL_CLK: u32 = 1 << 14;
pub const CM6206_REG1_PLLBIN_EN: u32 = 1 << 13;
pub const CM6206_REG1_SOFT_MUTE_EN: u32 = 1 << 12;
pub const CM6206_REG1_GPIO4_OUT: u32 = 1 << 11;
pub const CM6206_REG1_GPIO4_OE: u32 = 1 << 10;
pub const CM6206_REG1_GPIO3_OUT: u32 = 1 << 9;
pub const CM6206_REG1_GPIO3_OE: u32 = 1 << 8;
pub const CM6206_REG1_GPIO2_OUT: u32 = 1 << 7;
pub const CM6206_REG1_GPIO2_OE: u32 = 1 << 6;
pub const CM6206_REG1_GPIO1_OUT: u32 = 1 << 5;
pub const CM6206_REG1_GPIO1_OE: u32 = 1 << 4;
pub const CM6206_REG1_SPDIFO_INVALID: u32 = 1 << 3;
pub const CM6206_REG1_SPDIF_LOOP_EN: u32 = 1 << 2;
pub const CM6206_REG1_SPDIFO_DIS: u32 = 1 << 1;
pub const CM6206_REG1_SPDIFI_MIX: u32 = 1 << 0;

pub const CM6206_REG2_DRIVER_ON: u32 = 1 << 15;
pub const CM6206_REG2_HEADP_SEL_SIDE_CHANNELS: u32 = 0 << 13;
pub const CM6206_REG2_HEADP_SEL_SURROUND_CHANNELS: u32 = 1 << 13;
pub const CM6206_REG2_HEADP_SEL_CENTER_SUBW: u32 = 2 << 13;
pub const CM6206_REG2_HEADP_SEL_FRONT_CHANNELS: u32 = 3 << 13;
pub const CM6206_REG2_MUTE_HEADPHONE_RIGHT: u32 = 1 << 12;
pub const CM6206_REG2_MUTE_HEADPHONE_LEFT: u32 = 1 << 11;
pub const CM6206_REG2_MUTE_REAR_SURROUND_RIGHT: u32 = 1 << 10;
pub const CM6206_REG2_MUTE_REAR_SURROUND_LEFT: u32 = 1 << 9;
pub const CM6206_REG2_MUTE_SIDE_SURROUND_RIGHT: u32 = 1 << 8;
pub const CM6206_REG2_MUTE_SIDE_SURROUND_LEFT: u32 = 1 << 7;
pub const CM6206_REG2_MUTE_SUBWOOFER: u32 = 1 << 6;
pub const CM6206_REG2_MUTE_CENTER: u32 = 1 << 5;
pub const CM6206_REG2_MUTE_RIGHT_FRONT: u32 = 1 << 3;
pub const CM6206_REG2_MUTE_LEFT_FRONT: u32 = 1 << 3;
pub const CM6206_REG2_EN_BTL: u32 = 1 << 2;
pub const CM6206_REG2_MCUCLKSEL_1_5_MHZ: u32 = 0;
pub const CM6206_REG2_MCUCLKSEL_3_MHZ: u32 = 1;
pub const CM6206_REG2_MCUCLKSEL_6_MHZ: u32 = 2;
pub const CM6206_REG2_MCUCLKSEL_12_MHZ: u32 = 3;

pub const CM6206_REG3_FLYSPEED_DEFAULT: u32 = 2 << 11;
pub const CM6206_REG3_VRAP25EN: u32 = 1 << 10;
pub const CM6206_REG3_MSEL1: u32 = 1 << 9;
pub const CM6206_REG3_SPDIFI_RATE_44_1K: u32 = 1 << (0 << 7);
pub const CM6206_REG3_SPDIFI_RATE_48K: u32 = 1 << (2 << 7);
pub const CM6206_REG3_SPDIFI_RATE_32K: u32 = 1 << (3 << 7);
pub const CM6206_REG3_PINSEL: u32 = 1 << 6;
pub const CM6206_REG3_FOE: u32 = 1 << 5;
pub const CM6206_REG3_ROE: u32 = 1 << 4;
pub const CM6206_REG3_CBOE: u32 = 1 << 3;
pub const CM6206_REG3_LOSE: u32 = 1 << 2;
pub const CM6206_REG3_HPOE: u32 = 1 << 1;
pub const CM6206_REG3_SPDIFI_CANREC: u32 = 1 << 0;

pub const CM6206_REG5_DA_RSTN: u32 = 1 << 13;
pub const CM6206_REG5_AD_RSTN: u32 = 1 << 12;
pub const CM6206_REG5_SPDIFO_AD2SPDO: u32 = 1 << 12;
pub const CM6206_REG5_SPDIFO_SEL_FRONT: u32 = 0 << 9;
pub const CM6206_REG5_SPDIFO_SEL_SIDE_SUR: u32 = 1 << 9;
pub const CM6206_REG5_SPDIFO_SEL_CEN_LFE: u32 = 2 << 9;
pub const CM6206_REG5_SPDIFO_SEL_REAR_SUR: u32 = 3 << 9;
pub const CM6206_REG5_CODECM: u32 = 1 << 8;
pub const CM6206_REG5_EN_HPF: u32 = 1 << 7;
pub const CM6206_REG5_T_SEL_DSDA4: u32 = 1 << 6;
pub const CM6206_REG5_T_SEL_DSDA3: u32 = 1 << 5;
pub const CM6206_REG5_T_SEL_DSDA2: u32 = 1 << 4;
pub const CM6206_REG5_T_SEL_DSDA1: u32 = 1 << 3;
pub const CM6206_REG5_T_SEL_DSDAD_NORMAL: u32 = 0;
pub const CM6206_REG5_T_SEL_DSDAD_FRONT: u32 = 4;
pub const CM6206_REG5_T_SEL_DSDAD_S_SURROUND: u32 = 5;
pub const CM6206_REG5_T_SEL_DSDAD_CEN_LFE: u32 = 6;
pub const CM6206_REG5_T_SEL_DSDAD_R_SURROUND: u32 = 7;

pub enum EMU_QUIRK_SR {
    EMU_QUIRK_SR_44100HZ = 0,
    EMU_QUIRK_SR_48000HZ = 1,
    EMU_QUIRK_SR_88200HZ = 2,
    EMU_QUIRK_SR_96000HZ = 3,
    EMU_QUIRK_SR_176400HZ = 4,
    EMU_QUIRK_SR_192000HZ = 5,
}

// External functions (declarations)
extern "C" {
    pub fn get_iface_desc(alts: *mut usb_host_interface) -> *mut usb_interface_descriptor;
    pub fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: u32) -> *mut usb_interface;
    pub fn usb_interface_claimed(intf: *mut usb_interface) -> i32;
    pub fn snd_usb_create_quirk(chip: *mut snd_usb_audio, iface: *mut usb_interface,
                                driver: *mut usb_driver, quirk: *const snd_usb_audio_quirk) -> i32;
    pub fn usb_driver_claim_interface(driver: *mut usb_driver, iface: *mut usb_interface,
                                      priv_: *const core::ffi::c_void) -> i32;
    pub fn snd_usb_midi_v2_create(chip: *mut snd_usb_audio, intf: *mut usb_interface,
                                  quirk: *const snd_usb_audio_quirk, flags: u32) -> i32;
    pub fn snd_usb_parse_audio_interface(chip: *mut snd_usb_audio, ifnum: u32) -> i32;
    pub fn usb_set_interface(dev: *mut usb_device, ifnum: u32, altnum: u32) -> i32;
    pub fn snd_usb_audioformat_set_sync_ep(chip: *mut snd_usb_audio, fp: *mut audioformat);
    pub fn snd_usb_add_audio_stream(chip: *mut snd_usb_audio, stream: u32, fp: *mut audioformat,
                                    fmt: *const core::ffi::c_void) -> i32;
    pub fn snd_usb_add_endpoint(chip: *mut snd_usb_audio, ep: u32, type_: u32) -> i32;
    pub fn snd_usb_parse_datainterval(chip: *mut snd_usb_audio, alts: *mut usb_host_interface) -> u32;
    pub fn le16_to_cpu(x: u16) -> u16;
    pub fn le32_to_cpu(x: u32) -> u32;
    pub fn cpu_to_le32(x: u32) -> u32;
    pub fn get_endpoint(alts: *mut usb_host_interface, ep_idx: u32) -> *mut usb_endpoint_descriptor;
    pub fn snd_usb_init_pitch(chip: *mut snd_usb_audio, fp: *mut audioformat);
    pub fn snd_usb_init_sample_rate(chip: *mut snd_usb_audio, fp: *mut audioformat, rate: u32);
    pub fn snd_usb_find_csint_desc(extra: *mut u8, extralen: i32, prev: *mut u8,
                                   subtype: u8) -> *mut u8;
    pub fn usb_endpoint_xfer_isoc(epd: *const usb_endpoint_descriptor) -> i32;
    pub fn usb_endpoint_xfer_bulk(epd: *const usb_endpoint_descriptor) -> i32;
    pub fn usb_endpoint_xfer_int(epd: *const usb_endpoint_descriptor) -> i32;
    pub fn snd_usb_validate_midi_desc(desc: *const core::ffi::c_void) -> i32;
    pub fn snd_usb_ctl_msg(dev: *mut usb_device, pipe: u32, request: u8, requesttype: u8,
                           value: u16, index: u16, data: *mut core::ffi::c_void, size: u16) -> i32;
    pub fn usb_sndctrlpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    pub fn usb_rcvctrlpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    pub fn usb_sndintpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    pub fn usb_rcvintpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    pub fn usb_reset_configuration(dev: *mut usb_device) -> i32;
    pub fn usb_control_msg(dev: *mut usb_device, pipe: u32, request: u8, requesttype: u8,
                           value: u16, index: u16, data: *mut core::ffi::c_void, size: u16,
                           timeout: u32) -> i32;
    pub fn usb_reset_device(dev: *mut usb_device) -> i32;
    pub fn usb_pipe_type_check(dev: *mut usb_device, pipe: u32) -> i32;
    pub fn usb_interrupt_msg(dev: *mut usb_device, pipe: u32, data: *mut core::ffi::c_void,
                             len: i32, actual_length: *mut i32, timeout: u32) -> i32;
    pub fn usb_driver_set_configuration(dev: *mut usb_device, config: i32) -> i32;
    pub fn snd_usb_create_mixer(chip: *mut snd_usb_audio, ifnum: u32) -> i32;
    pub fn __snd_usbmidi_create(card: *mut snd_card, iface: *mut usb_interface,
                                midi_list: *mut list_head, quirk: *const snd_usb_audio_quirk,
                                usb_id: u32, num_rawmidis: *mut i32) -> i32;
    pub fn snd_emuusb_set_samplerate(chip: *mut snd_usb_audio, sr: u32);
    pub fn dev_dbg(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn dev_info(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn dev_warn(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn dev_err(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn usb_audio_dbg(chip: *mut snd_usb_audio, fmt: *const core::ffi::c_char, ...);
    pub fn usb_audio_err(chip: *mut snd_usb_audio, fmt: *const core::ffi::c_char, ...);
    pub fn usb_audio_warn(chip: *mut snd_usb_audio, fmt: *const core::ffi::c_char, ...);
    pub fn msleep(msecs: u32);
    pub fn usleep_range(min: u32, max: u32);
    pub fn print_hex_dump(level: *const core::ffi::c_char, prefix: *const core::ffi::c_char,
                          prefix_type: u32, rowsize: u32, groupsize: u32,
                          buf: *const core::ffi::c_void, len: usize, ascii: bool);
    pub fn dev_get_drvdata(dev: *const device) -> *mut core::ffi::c_void;
    pub fn USB_ID(vendor: u32, product: u32) -> u32;
    pub fn USB_ID_VENDOR(id: u32) -> u16;
    pub fn USB_ID_PRODUCT(id: u32) -> u16;
    pub fn get_cfg_desc(config: *mut usb_host_config) -> *mut usb_config_descriptor;
}

// Macros as functions
#[inline]
pub fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
pub fn list_del(entry: *mut list_head) {
    unsafe {
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
    }
}

#[inline]
pub fn BIT_U64(n: u32) -> u64 {
    1u64 << n
}

// Quirk type constants
pub const QUIRK_IGNORE_INTERFACE: u16 = 0;
pub const QUIRK_COMPOSITE: u16 = 1;
pub const QUIRK_AUTODETECT: u16 = 2;
pub const QUIRK_MIDI_STANDARD_INTERFACE: u16 = 3;
pub const QUIRK_MIDI_FIXED_ENDPOINT: u16 = 4;
pub const QUIRK_MIDI_YAMAHA: u16 = 5;
pub const QUIRK_MIDI_ROLAND: u16 = 6;
pub const QUIRK_MIDI_MIDIMAN: u16 = 7;
pub const QUIRK_MIDI_NOVATION: u16 = 8;
pub const QUIRK_MIDI_RAW_BYTES: u16 = 9;
pub const QUIRK_MIDI_EMAGIC: u16 = 10;
pub const QUIRK_MIDI_CME: u16 = 11;
pub const QUIRK_MIDI_AKAI: u16 = 12;
pub const QUIRK_MIDI_FTDI: u16 = 13;
pub const QUIRK_MIDI_CH345: u16 = 14;
pub const QUIRK_AUDIO_STANDARD_INTERFACE: u16 = 15;
pub const QUIRK_AUDIO_FIXED_ENDPOINT: u16 = 16;
pub const QUIRK_AUDIO_EDIROL_UAXX: u16 = 17;
pub const QUIRK_AUDIO_STANDARD_MIXER: u16 = 18;
pub const QUIRK_TYPE_COUNT: u16 = 19;

pub const USB_AUDIO_IFACE_UNUSED: *const core::ffi::c_void = core::ptr::null();

pub const USB_DIR_IN: u32 = 0x80;
pub const USB_DIR_OUT: u32 = 0x00;
pub const USB_TYPE_CLASS: u32 = 0x20;
pub const USB_TYPE_VENDOR: u32 = 0xC0;
pub const USB_RECIP_INTERFACE: u32 = 0x01;
pub const USB_RECIP_ENDPOINT: u32 = 0x02;
pub const USB_RECIP_DEVICE: u32 = 0x00;
pub const USB_RECIP_OTHER: u32 = 0x03;

pub const USB_REQ_SET_CONFIGURATION: u8 = 9;
pub const USB_REQ_SET_INTERFACE: u8 = 11;

pub const USB_CLASS_VENDOR_SPEC: u8 = 0xFF;

pub const USB_DT_DEVICE: u8 = 1;
pub const USB_DT_CS_INTERFACE: u8 = 0x24;
pub const USB_DT_CS_ENDPOINT: u8 = 0x25;

pub const USB_MS_HEADER: u8 = 1;
pub const USB_MS_MIDI_IN_JACK: u8 = 2;
pub const USB_MS_MIDI_OUT_JACK: u8 = 3;
pub const USB_MS_EMBEDDED: u8 = 1;
pub const USB_MS_EXTERNAL: u8 = 2;
pub const UAC_MS_GENERAL: u8 = 1;

pub const USB_ENDPOINT_SYNCTYPE: u8 = 0x0C;
pub const USB_ENDPOINT_SYNC_ADAPTIVE: u8 = 0x08;
pub const USB_ENDPOINT_SYNC_SYNC: u8 = 0x0C;

pub const UAC_FORMAT_TYPE: u8 = 2;
pub const UAC_FORMAT_TYPE_I: u8 = 1;
pub const UAC_AS_GENERAL: u8 = 1;

pub const UAC_SET_CUR: u8 = 0x01;
pub const UAC_FU_VOLUME: u8 = 0x02;

pub const UAC_EP_CS_ATTR_SAMPLE_RATE: u32 = 1;
pub const UAC_EP_CS_ATTR_FILL_MAX: u32 = 0x80;
pub const UAC_EP_CS_ATTR_PITCH_CONTROL: u32 = 0x02;

pub const SNDRV_PCM_STREAM_PLAYBACK: u32 = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: u32 = 1;

pub const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 8;
pub const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
pub const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 2;
pub const SNDRV_PCM_FMTBIT_DSD_U8: u64 = 1 << 32;
pub const SNDRV_PCM_FMTBIT_DSD_U16_LE: u64 = 1 << 33;
pub const SNDRV_PCM_FMTBIT_DSD_U32_BE: u64 = 1 << 35;
pub const SNDRV_PCM_FMTBIT_DSD_U32_LE: u64 = 1 << 36;

pub const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;

pub const SND_USB_ENDPOINT_TYPE_DATA: u32 = 0;
pub const SND_USB_ENDPOINT_TYPE_SYNC: u32 = 1;

pub const QUIRK_FLAG_GET_SAMPLE_RATE: u64 = 1 << 0;
pub const QUIRK_FLAG_SHARE_MEDIA_DEVICE: u64 = 1 << 1;
pub const QUIRK_FLAG_ALIGN_TRANSFER: u64 = 1 << 2;
pub const QUIRK_FLAG_TX_LENGTH: u64 = 1 << 3;
pub const QUIRK_FLAG_PLAYBACK_FIRST: u64 = 1 << 4;
pub const QUIRK_FLAG_SKIP_CLOCK_SELECTOR: u64 = 1 << 5;
pub const QUIRK_FLAG_IGNORE_CLOCK_SOURCE: u64 = 1 << 6;
pub const QUIRK_FLAG_ITF_USB_DSD_DAC: u64 = 1 << 7;
pub const QUIRK_FLAG_CTL_MSG_DELAY: u64 = 1 << 8;
pub const QUIRK_FLAG_CTL_MSG_DELAY_1M: u64 = 1 << 9;
pub const QUIRK_FLAG_CTL_MSG_DELAY_5M: u64 = 1 << 10;
pub const QUIRK_FLAG_IFACE_DELAY: u64 = 1 << 11;
pub const QUIRK_FLAG_VALIDATE_RATES: u64 = 1 << 12;
pub const QUIRK_FLAG_DISABLE_AUTOSUSPEND: u64 = 1 << 13;
pub const QUIRK_FLAG_IGNORE_CTL_ERROR: u64 = 1 << 14;
pub const QUIRK_FLAG_DSD_RAW: u64 = 1 << 15;
pub const QUIRK_FLAG_SET_IFACE_FIRST: u64 = 1 << 16;
pub const QUIRK_FLAG_GENERIC_IMPLICIT_FB: u64 = 1 << 17;
pub const QUIRK_FLAG_SKIP_IMPLICIT_FB: u64 = 1 << 18;
pub const QUIRK_FLAG_IFACE_SKIP_CLOSE: u64 = 1 << 19;
pub const QUIRK_FLAG_FORCE_IFACE_RESET: u64 = 1 << 20;
pub const QUIRK_FLAG_FIXED_RATE: u64 = 1 << 21;
pub const QUIRK_FLAG_MIC_RES_16: u64 = 1 << 22;
pub const QUIRK_FLAG_MIC_RES_384: u64 = 1 << 23;
pub const QUIRK_FLAG_MIXER_PLAYBACK_MIN_MUTE: u64 = 1 << 24;
pub const QUIRK_FLAG_MIXER_CAPTURE_MIN_MUTE: u64 = 1 << 25;
pub const QUIRK_FLAG_SKIP_IFACE_SETUP: u64 = 1 << 26;
pub const QUIRK_FLAG_MIXER_PLAYBACK_LINEAR_VOL: u64 = 1 << 27;
pub const QUIRK_FLAG_MIXER_CAPTURE_LINEAR_VOL: u64 = 1 << 28;
pub const QUIRK_FLAG_IFB_SILENCE_ON_EMPTY: u64 = 1 << 29;
pub const QUIRK_FLAG_MIXER_GET_CUR_OK: u64 = 1 << 30;
pub const QUIRK_FLAG_PLAYBACK_URB_FIXUP: u64 = 1 << 31;
pub const QUIRK_FLAG_ALWAYS_SET_RATE: u64 = 1 << 32;

pub const GFP_KERNEL: u32 = 0x100000;
pub const ENOMEM: i32 = -12;
pub const EINVAL: i32 = -22;
pub const ENODEV: i32 = -19;
pub const ENXIO: i32 = -6;
pub const EAGAIN: i32 = -11;

pub const KERN_DEBUG: *const core::ffi::c_char = b"KERN_DEBUG\0" as *const u8 as *const core::ffi::c_char;
pub const DUMP_PREFIX_NONE: u32 = 0;

pub const MAX_NR_RATES: u32 = 384;
pub const ARRAY_SIZE_LIMIT: usize = 1024;

pub const BYTES_TO_BITS: fn(usize) -> usize = |x| x * 8;

// Function implementations

pub unsafe fn create_composite_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    quirk_comp: *const snd_usb_audio_quirk,
) -> i32 {
    let probed_ifnum = (*get_iface_desc((*iface).altsetting)).bInterfaceNumber;
    let mut quirk = (*quirk_comp).data as *const snd_usb_audio_quirk;
    let mut err;

    loop {
        if (*quirk).ifnum < 0 {
            break;
        }
        let mut iface_temp = usb_ifnum_to_if((*chip).dev, (*quirk).ifnum as u32);
        if iface_temp.is_null() {
            quirk = (quirk as *const u8).add(core::mem::size_of::<snd_usb_audio_quirk>())
                as *const snd_usb_audio_quirk;
            continue;
        }
        if (*quirk).ifnum != probed_ifnum as i32 && usb_interface_claimed(iface_temp) != 0 {
            quirk = (quirk as *const u8).add(core::mem::size_of::<snd_usb_audio_quirk>())
                as *const snd_usb_audio_quirk;
            continue;
        }
        err = snd_usb_create_quirk(chip, iface_temp, driver, quirk);
        if err < 0 {
            return err;
        }
        quirk = (quirk as *const u8).add(core::mem::size_of::<snd_usb_audio_quirk>())
            as *const snd_usb_audio_quirk;
    }

    quirk = (*quirk_comp).data as *const snd_usb_audio_quirk;
    loop {
        if (*quirk).ifnum < 0 {
            break;
        }
        let mut iface_temp = usb_ifnum_to_if((*chip).dev, (*quirk).ifnum as u32);
        if iface_temp.is_null() {
            quirk = (quirk as *const u8).add(core::mem::size_of::<snd_usb_audio_quirk>())
                as *const snd_usb_audio_quirk;
            continue;
        }
        if (*quirk).ifnum != probed_ifnum as i32 && usb_interface_claimed(iface_temp) == 0 {
            err = usb_driver_claim_interface(driver, iface_temp, USB_AUDIO_IFACE_UNUSED);
            if err < 0 {
                return err;
            }
        }
        quirk = (quirk as *const u8).add(core::mem::size_of::<snd_usb_audio_quirk>())
            as *const snd_usb_audio_quirk;
    }

    0
}

pub unsafe fn ignore_interface_quirk(
    _chip: *mut snd_usb_audio,
    _iface: *mut usb_interface,
    _driver: *mut usb_driver,
    _quirk: *const snd_usb_audio_quirk,
) -> i32 {
    0
}

pub unsafe fn create_any_midi_quirk(
    chip: *mut snd_usb_audio,
    intf: *mut usb_interface,
    _driver: *mut usb_driver,
    quirk: *const snd_usb_audio_quirk,
) -> i32 {
    snd_usb_midi_v2_create(chip, intf, quirk, 0)
}

pub unsafe fn create_standard_audio_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    _driver: *mut usb_driver,
    _quirk: *const snd_usb_audio_quirk,
) -> i32 {
    let alts = (*iface).altsetting;
    let altsd = get_iface_desc(alts);
    let err = snd_usb_parse_audio_interface(chip, (*altsd).bInterfaceNumber as u32);
    if err < 0 {
        usb_audio_err(chip, b"cannot setup if %d: error %d\n\0".as_ptr() as *const i8,
                      (*altsd).bInterfaceNumber, err);
        return err;
    }
    usb_set_interface((*chip).dev, (*altsd).bInterfaceNumber as u32, 0);
    0
}

pub unsafe fn add_audio_stream_from_fixed_fmt(
    chip: *mut snd_usb_audio,
    fp: *mut audioformat,
) -> i32 {
    let stream = if ((*fp).endpoint & USB_DIR_IN) != 0 {
        SNDRV_PCM_STREAM_CAPTURE
    } else {
        SNDRV_PCM_STREAM_PLAYBACK
    };

    snd_usb_audioformat_set_sync_ep(chip, fp);

    let mut err = snd_usb_add_audio_stream(chip, stream, fp, core::ptr::null());
    if err < 0 {
        return err;
    }

    err = snd_usb_add_endpoint(chip, (*fp).endpoint, SND_USB_ENDPOINT_TYPE_DATA);
    if err < 0 {
        return err;
    }

    if (*fp).sync_ep != 0 {
        let endpoint_type = if (*fp).implicit_fb != 0 {
            SND_USB_ENDPOINT_TYPE_DATA
        } else {
            SND_USB_ENDPOINT_TYPE_SYNC
        };
        err = snd_usb_add_endpoint(chip, (*fp).sync_ep, endpoint_type);
        if err < 0 {
            return err;
        }
    }

    0
}

pub unsafe fn create_fixed_stream_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    _driver: *mut usb_driver,
    quirk: *const snd_usb_audio_quirk,
) -> i32 {
    // kmemdup and related functions - need external implementation
    // This is a stub for the unsafe kernel memory operations
    todo!("create_fixed_stream_quirk - requires kernel memory management")
}

pub unsafe fn create_auto_pcm_quirk(
    _chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    _driver: *mut usb_driver,
) -> i32 {
    if (*iface).num_altsetting < 2 {
        return ENODEV;
    }

    let alts = (*iface).altsetting.add(1);
    let altsd = get_iface_desc(alts);

    if (*altsd).bNumEndpoints < 1 {
        return ENODEV;
    }

    let epd = get_endpoint(alts, 0);
    if usb_endpoint_xfer_isoc(epd) == 0 {
        return ENODEV;
    }

    let ashd = snd_usb_find_csint_desc((*alts).extra, (*alts).extralen, core::ptr::null_mut(), UAC_AS_GENERAL);
    let fmtd = snd_usb_find_csint_desc((*alts).extra, (*alts).extralen, core::ptr::null_mut(), UAC_FORMAT_TYPE);

    if ashd.is_null() || (*(ashd as *const u8)) < 7 ||
       fmtd.is_null() || (*(fmtd as *const u8)) < 8 {
        return ENODEV;
    }

    // create_standard_audio_quirk with NULL quirk
    todo!("create_auto_pcm_quirk - needs full audio quirk creation")
}

pub unsafe fn create_yamaha_midi_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    alts: *mut usb_host_interface,
) -> i32 {
    // Static initialized quirk
    todo!("create_yamaha_midi_quirk - midi quirk setup")
}

pub unsafe fn create_roland_midi_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    alts: *mut usb_host_interface,
) -> i32 {
    // Static initialized quirk
    todo!("create_roland_midi_quirk - midi quirk setup")
}

pub unsafe fn create_std_midi_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    alts: *mut usb_host_interface,
) -> i32 {
    todo!("create_std_midi_quirk - standard midi setup")
}

pub unsafe fn create_auto_midi_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
) -> i32 {
    let alts = (*iface).altsetting;
    let altsd = get_iface_desc(alts);

    if (*altsd).bNumEndpoints < 1 {
        return ENODEV;
    }

    let epd = get_endpoint(alts, 0);
    if usb_endpoint_xfer_bulk(epd) == 0 && usb_endpoint_xfer_int(epd) == 0 {
        return ENODEV;
    }

    let vendor = USB_ID_VENDOR((*chip).usb_id);
    let mut err = match vendor {
        0x0499 => {
            err = create_yamaha_midi_quirk(chip, iface, driver, alts);
            if err != ENODEV {
                return err;
            }
            err
        }
        0x0582 => {
            err = create_roland_midi_quirk(chip, iface, driver, alts);
            if err != ENODEV {
                return err;
            }
            err
        }
        _ => 0,
    };

    create_std_midi_quirk(chip, iface, driver, alts)
}

pub unsafe fn create_autodetect_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    _quirk: *const snd_usb_audio_quirk,
) -> i32 {
    let mut err = create_auto_pcm_quirk(chip, iface, driver);
    if err == ENODEV {
        err = create_auto_midi_quirk(chip, iface, driver);
    }
    err
}

pub unsafe fn create_uaxx_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    _driver: *mut usb_driver,
    _quirk: *const snd_usb_audio_quirk,
) -> i32 {
    todo!("create_uaxx_quirk - UAXX device quirk")
}

pub unsafe fn create_standard_mixer_quirk(
    chip: *mut snd_usb_audio,
    _iface: *mut usb_interface,
    _driver: *mut usb_driver,
    quirk: *const snd_usb_audio_quirk,
) -> i32 {
    if (*quirk).ifnum < 0 {
        return 0;
    }
    snd_usb_create_mixer(chip, (*quirk).ifnum as u32)
}

pub unsafe fn snd_usb_create_quirk(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    driver: *mut usb_driver,
    quirk: *const snd_usb_audio_quirk,
) -> i32 {
    type QuirkFuncT = unsafe extern "C" fn(*mut snd_usb_audio, *mut usb_interface,
                                           *mut usb_driver, *const snd_usb_audio_quirk) -> i32;

    static QUIRK_FUNCS: [Option<QuirkFuncT>; QUIRK_TYPE_COUNT as usize] = [
        Some(ignore_interface_quirk as QuirkFuncT),
        Some(create_composite_quirk as QuirkFuncT),
        Some(create_autodetect_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_any_midi_quirk as QuirkFuncT),
        Some(create_standard_audio_quirk as QuirkFuncT),
        Some(create_fixed_stream_quirk as QuirkFuncT),
        Some(create_uaxx_quirk as QuirkFuncT),
        Some(create_standard_mixer_quirk as QuirkFuncT),
    ];

    if ((*quirk).type_ as usize) < QUIRK_TYPE_COUNT as usize {
        if let Some(func) = QUIRK_FUNCS[(*quirk).type_ as usize] {
            return func(chip, iface, driver, quirk);
        }
    }
    usb_audio_err(chip, b"invalid quirk type %d\n\0".as_ptr() as *const i8, (*quirk).type_);
    ENXIO
}

pub unsafe fn snd_usb_extigy_boot_quirk(dev: *mut usb_device, _intf: *mut usb_interface) -> i32 {
    todo!("snd_usb_extigy_boot_quirk - Extigy boot sequence")
}

pub unsafe fn snd_usb_audigy2nx_boot_quirk(dev: *mut usb_device) -> i32 {
    let mut buf: u8 = 1;
    snd_usb_ctl_msg(dev, usb_rcvctrlpipe(dev, 0), 0x2a,
                    (USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_OTHER) as u8,
                    0, 0, &mut buf as *mut u8 as *mut core::ffi::c_void, 1);
    if buf == 0 {
        snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0), 0x29,
                        (USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_OTHER) as u8,
                        1, 2000, core::ptr::null_mut(), 0);
        return ENODEV;
    }
    0
}

pub unsafe fn snd_usb_fasttrackpro_boot_quirk(dev: *mut usb_device) -> i32 {
    if (*(*dev).actconfig).desc.bConfigurationValue == 1 {
        dev_info(&(*dev).dev,
                b"Fast Track Pro switching to config #2\n\0".as_ptr() as *const i8);
        let err = usb_driver_set_configuration(dev, 2);
        if err < 0 {
            dev_dbg(&(*dev).dev,
                   b"error usb_driver_set_configuration: %d\n\0".as_ptr() as *const i8, err);
        }
        return ENODEV;
    } else {
        dev_info(&(*dev).dev,
                b"Fast Track Pro config OK\n\0".as_ptr() as *const i8);
    }
    0
}

pub unsafe fn snd_usb_cm106_write_int_reg(dev: *mut usb_device, reg: i32, value: u16) -> i32 {
    let mut buf: [u8; 4] = [0; 4];
    buf[0] = 0x20;
    buf[1] = (value & 0xff) as u8;
    buf[2] = ((value >> 8) & 0xff) as u8;
    buf[3] = reg as u8;
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0), USB_REQ_SET_CONFIGURATION,
                   (USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_ENDPOINT) as u8,
                   0, 0, buf.as_mut_ptr() as *mut core::ffi::c_void, 4)
}

pub unsafe fn snd_usb_cm106_boot_quirk(dev: *mut usb_device) -> i32 {
    snd_usb_cm106_write_int_reg(dev, 2, 0x8004)
}

pub unsafe fn snd_usb_cm6206_boot_quirk(dev: *mut usb_device) -> i32 {
    let mut err = 0;
    let val: [u32; 6] = [
        CM6206_REG0_SPDIFO_RATE_48K | CM6206_REG0_SPDIFO_COPYRIGHT_NA,
        CM6206_REG1_PLLBIN_EN | CM6206_REG1_SOFT_MUTE_EN,
        CM6206_REG2_DRIVER_ON | CM6206_REG2_HEADP_SEL_FRONT_CHANNELS |
            CM6206_REG2_MUTE_HEADPHONE_RIGHT | CM6206_REG2_MUTE_HEADPHONE_LEFT,
        CM6206_REG3_FLYSPEED_DEFAULT | CM6206_REG3_VRAP25EN | CM6206_REG3_FOE |
            CM6206_REG3_ROE | CM6206_REG3_CBOE | CM6206_REG3_LOSE |
            CM6206_REG3_HPOE | CM6206_REG3_SPDIFI_CANREC,
        0x0000,
        CM6206_REG5_DA_RSTN | CM6206_REG5_AD_RSTN,
    ];

    for reg in 0..val.len() {
        err = snd_usb_cm106_write_int_reg(dev, reg as i32, val[reg] as u16);
        if err < 0 {
            return err;
        }
    }
    err
}

pub unsafe fn snd_usb_gamecon780_boot_quirk(dev: *mut usb_device) -> i32 {
    let buf: [u8; 2] = [0x74, 0xe3];
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0), UAC_SET_CUR,
                   (USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_OUT) as u8,
                   (UAC_FU_VOLUME as u16) << 8, 9 << 8,
                   buf.as_ptr() as *const core::ffi::c_void as *mut core::ffi::c_void, 2)
}

pub unsafe fn snd_usb_novation_boot_quirk(dev: *mut usb_device) -> i32 {
    usb_set_interface(dev, 0, 1);
    0
}

pub unsafe fn snd_usb_accessmusic_boot_quirk(dev: *mut usb_device) -> i32 {
    todo!("snd_usb_accessmusic_boot_quirk - Access Music boot")
}

pub unsafe fn snd_usb_nativeinstruments_boot_quirk(dev: *mut usb_device) -> i32 {
    let ret = usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
                             0xaf, USB_TYPE_VENDOR as u8 | USB_RECIP_DEVICE as u8,
                             1, 0, core::ptr::null_mut(), 0, 1000);
    if ret < 0 {
        return ret;
    }
    usb_reset_device(dev);
    EAGAIN
}

pub unsafe fn mbox2_setup_48_24_magic(dev: *mut usb_device) {
    let mut srate: [u8; 3] = [0; 3];
    let mut temp: [u8; 12] = [0; 12];

    srate[0] = 0x80;
    srate[1] = 0xbb;
    srate[2] = 0x00;

    snd_usb_ctl_msg(dev, usb_rcvctrlpipe(dev, 0),
                   0x01, 0x22, 0x0100, 0x0085,
                   temp.as_mut_ptr() as *mut core::ffi::c_void, 0x0003);
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                   0x81, 0xa2, 0x0100, 0x0085,
                   srate.as_mut_ptr() as *mut core::ffi::c_void, 0x0003);
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                   0x81, 0xa2, 0x0100, 0x0086,
                   srate.as_mut_ptr() as *mut core::ffi::c_void, 0x0003);
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                   0x81, 0xa2, 0x0100, 0x0003,
                   srate.as_mut_ptr() as *mut core::ffi::c_void, 0x0003);
}

pub unsafe fn snd_usb_mbox2_boot_quirk(dev: *mut usb_device) -> i32 {
    todo!("snd_usb_mbox2_boot_quirk - Mbox2 boot sequence")
}

pub unsafe fn snd_usb_axefx3_boot_quirk(dev: *mut usb_device) -> i32 {
    dev_dbg(&(*dev).dev,
           b"Waiting for Axe-Fx III to boot up...\n\0".as_ptr() as *const i8);
    let err = usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
                             USB_REQ_SET_INTERFACE,
                             (USB_RECIP_INTERFACE | USB_DIR_OUT) as u8,
                             1, 1, core::ptr::null_mut(), 0, 120000);
    if err < 0 {
        dev_err(&(*dev).dev,
               b"failed waiting for Axe-Fx III to boot: %d\n\0".as_ptr() as *const i8, err);
        return err;
    }
    dev_dbg(&(*dev).dev,
           b"Axe-Fx III is now ready\n\0".as_ptr() as *const i8);
    let err = usb_set_interface(dev, 1, 0);
    if err < 0 {
        dev_dbg(&(*dev).dev,
               b"error stopping Axe-Fx III interface: %d\n\0".as_ptr() as *const i8, err);
    }
    0
}

pub unsafe fn mbox3_setup_defaults(_dev: *mut usb_device) {
    todo!("mbox3_setup_defaults - Mbox3 initialization")
}

pub unsafe fn snd_usb_mbox3_boot_quirk(dev: *mut usb_device) -> i32 {
    todo!("snd_usb_mbox3_boot_quirk - Mbox3 boot sequence")
}

pub unsafe fn snd_usb_motu_microbookii_communicate(
    dev: *mut usb_device,
    buf: *mut u8,
    buf_size: i32,
    length: *mut i32,
) -> i32 {
    todo!("snd_usb_motu_microbookii_communicate - MicroBook II communication")
}

pub unsafe fn snd_usb_motu_microbookii_boot_quirk(dev: *mut usb_device) -> i32 {
    todo!("snd_usb_motu_microbookii_boot_quirk - MicroBook II boot")
}

pub unsafe fn snd_usb_motu_m_series_boot_quirk(dev: *mut usb_device) -> i32 {
    msleep(4000);
    0
}

pub unsafe fn snd_usb_rme_digiface_boot_quirk(dev: *mut usb_device) -> i32 {
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                   16, 0x40, 0x2410, 0x7fff, core::ptr::null_mut(), 0);
    snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                   18, 0x40, 0x0104, 0xffff, core::ptr::null_mut(), 0);

    for ch in 0..32 {
        snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                       22, 0x40, 0x400, ch, core::ptr::null_mut(), 0);
    }

    for ch in 0..34 {
        snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0),
                       21, 0x40, 0x9000, 0x100 + ch, core::ptr::null_mut(), 0);
    }

    0
}

pub unsafe fn quattro_skip_setting_quirk(chip: *mut snd_usb_audio, iface: i32, altno: i32) -> i32 {
    usb_set_interface((*chip).dev, iface as u32, 0);
    if ((*chip).setup & MAUDIO_SET) != 0 {
        if ((*chip).setup & MAUDIO_SET_COMPATIBLE) != 0 {
            if iface != 1 && iface != 2 {
                return 1;
            }
        } else {
            if iface == 1 || iface == 2 {
                return 1;
            }
            if ((*chip).setup & MAUDIO_SET_96K) != 0 && altno != 1 {
                return 1;
            }
            let mask = (*chip).setup & MAUDIO_SET_MASK;
            if mask == MAUDIO_SET_24B_48K_DI && altno != 2 {
                return 1;
            }
            if mask == MAUDIO_SET_24B_48K_NOTDI && altno != 3 {
                return 1;
            }
            if mask == MAUDIO_SET_16B_48K_NOTDI && altno != 4 {
                return 1;
            }
        }
    }
    usb_audio_dbg(chip,
                 b"using altsetting %d for interface %d config %d\n\0".as_ptr() as *const i8,
                 altno, iface, (*chip).setup);
    0
}

pub unsafe fn audiophile_skip_setting_quirk(chip: *mut snd_usb_audio, iface: i32, altno: i32) -> i32 {
    usb_set_interface((*chip).dev, iface as u32, 0);
    if ((*chip).setup & MAUDIO_SET) != 0 {
        if ((*chip).setup & MAUDIO_SET_DTS) != 0 && altno != 6 {
            return 1;
        }
        if ((*chip).setup & MAUDIO_SET_96K) != 0 && altno != 1 {
            return 1;
        }
        let mask = (*chip).setup & MAUDIO_SET_MASK;
        if mask == MAUDIO_SET_24B_48K_DI && altno != 2 {
            return 1;
        }
        if mask == MAUDIO_SET_24B_48K_NOTDI && altno != 3 {
            return 1;
        }
        if mask == MAUDIO_SET_16B_48K_DI && altno != 4 {
            return 1;
        }
        if mask == MAUDIO_SET_16B_48K_NOTDI && altno != 5 {
            return 1;
        }
    }
    0
}

pub unsafe fn fasttrackpro_skip_setting_quirk(chip: *mut snd_usb_audio, iface: i32, altno: i32) -> i32 {
    usb_set_interface((*chip).dev, iface as u32, 0);
    if ((*chip).setup & (MAUDIO_SET | MAUDIO_SET_24B)) != 0 {
        if ((*chip).setup & MAUDIO_SET_96K) != 0 {
            if altno != 3 && altno != 6 {
                return 1;
            }
        } else if ((*chip).setup & MAUDIO_SET_DI) != 0 {
            if iface == 4 {
                return 1;
            }
            if altno != 2 && altno != 5 {
                return 1;
            }
        } else {
            if iface == 5 {
                return 1;
            }
            if altno != 2 && altno != 5 {
                return 1;
            }
        }
    } else {
        if altno != 1 {
            return 1;
        }
    }
    usb_audio_dbg(chip,
                 b"using altsetting %d for interface %d config %d\n\0".as_ptr() as *const i8,
                 altno, iface, (*chip).setup);
    0
}

pub unsafe fn s1810c_skip_setting_quirk(chip: *mut snd_usb_audio, _iface: i32, altno: i32) -> i32 {
    if ((*chip).setup == 0 || (*chip).setup > 2) && altno != 2 {
        return 1;
    } else if (*chip).setup == 1 && altno != 1 {
        return 1;
    } else if (*chip).setup == 2 && altno != 3 {
        return 1;
    }
    0
}

pub unsafe fn snd_usb_apply_interface_quirk(chip: *mut snd_usb_audio, iface: i32, altno: i32) -> i32 {
    if (*chip).usb_id == USB_ID(0x0763, 0x2003) {
        return audiophile_skip_setting_quirk(chip, iface, altno);
    }
    if (*chip).usb_id == USB_ID(0x0763, 0x2001) {
        return quattro_skip_setting_quirk(chip, iface, altno);
    }
    if (*chip).usb_id == USB_ID(0x0763, 0x2012) {
        return fasttrackpro_skip_setting_quirk(chip, iface, altno);
    }
    if (*chip).usb_id == USB_ID(0x194f, 0x010c) {
        return s1810c_skip_setting_quirk(chip, iface, altno);
    }
    0
}

pub unsafe fn snd_usb_apply_boot_quirk(
    dev: *mut usb_device,
    _intf: *mut usb_interface,
    _quirk: *const snd_usb_audio_quirk,
    id: u32,
) -> i32 {
    match id {
        id if id == USB_ID(0x041e, 0x3000) => snd_usb_extigy_boot_quirk(dev, core::ptr::null_mut()),
        id if id == USB_ID(0x041e, 0x3020) => snd_usb_audigy2nx_boot_quirk(dev),
        id if id == USB_ID(0x10f5, 0x0200) => snd_usb_cm106_boot_quirk(dev),
        id if id == USB_ID(0x0d8c, 0x0102) || id == USB_ID(0x0ccd, 0x00b1) => snd_usb_cm6206_boot_quirk(dev),
        id if id == USB_ID(0x0dba, 0x3000) => snd_usb_mbox2_boot_quirk(dev),
        id if id == USB_ID(0x0dba, 0x5000) => snd_usb_mbox3_boot_quirk(dev),
        id if id == USB_ID(0x1235, 0x0010) || id == USB_ID(0x1235, 0x0018) => snd_usb_novation_boot_quirk(dev),
        id if id == USB_ID(0x133e, 0x0815) => snd_usb_accessmusic_boot_quirk(dev),
        id if id == USB_ID(0x17cc, 0x1000) || id == USB_ID(0x17cc, 0x1010) || id == USB_ID(0x17cc, 0x1020) => {
            snd_usb_nativeinstruments_boot_quirk(dev)
        }
        id if id == USB_ID(0x0763, 0x2012) => snd_usb_fasttrackpro_boot_quirk(dev),
        id if id == USB_ID(0x047f, 0xc010) => snd_usb_gamecon780_boot_quirk(dev),
        id if id == USB_ID(0x2466, 0x8010) => snd_usb_axefx3_boot_quirk(dev),
        id if id == USB_ID(0x07fd, 0x0004) => {
            // Needs interface check - stub for now
            -19  // ENODEV
        }
        id if id == USB_ID(0x2a39, 0x3f8c) || id == USB_ID(0x2a39, 0x3fa0) => snd_usb_rme_digiface_boot_quirk(dev),
        _ => 0,
    }
}

pub unsafe fn snd_usb_apply_boot_quirk_once(
    dev: *mut usb_device,
    _intf: *mut usb_interface,
    _quirk: *const snd_usb_audio_quirk,
    id: u32,
) -> i32 {
    match id {
        id if id == USB_ID(0x07fd, 0x0008) => snd_usb_motu_m_series_boot_quirk(dev),
        _ => 0,
    }
}

pub unsafe fn snd_usb_is_big_endian_format(chip: *mut snd_usb_audio, fp: *const audioformat) -> i32 {
    match (*chip).usb_id {
        id if id == USB_ID(0x0763, 0x2001) => {
            if (*fp).altsetting == 2 || (*fp).altsetting == 3 ||
               (*fp).altsetting == 5 || (*fp).altsetting == 6 {
                return 1;
            }
        }
        id if id == USB_ID(0x0763, 0x2003) => {
            if (*chip).setup == 0x00 ||
               (*fp).altsetting == 1 || (*fp).altsetting == 2 ||
               (*fp).altsetting == 3 {
                return 1;
            }
        }
        id if id == USB_ID(0x0763, 0x2012) => {
            if (*fp).altsetting == 2 || (*fp).altsetting == 3 ||
               (*fp).altsetting == 5 || (*fp).altsetting == 6 {
                return 1;
            }
        }
        _ => {}
    }
    0
}

pub unsafe fn set_format_emu_quirk(subs: *mut snd_usb_substream, fmt: *const audioformat) {
    let mut emu_samplerate_id: u32 = EMU_QUIRK_SR_44100HZ as u32;

    if (*subs).direction == SNDRV_PCM_STREAM_PLAYBACK {
        if !(*(*subs).stream).substream[SNDRV_PCM_STREAM_CAPTURE as usize].is_none() {
            return;
        }
    }

    match (*fmt).rate_min {
        48000 => emu_samplerate_id = EMU_QUIRK_SR_48000HZ as u32,
        88200 => emu_samplerate_id = EMU_QUIRK_SR_88200HZ as u32,
        96000 => emu_samplerate_id = EMU_QUIRK_SR_96000HZ as u32,
        176400 => emu_samplerate_id = EMU_QUIRK_SR_176400HZ as u32,
        192000 => emu_samplerate_id = EMU_QUIRK_SR_192000HZ as u32,
        _ => emu_samplerate_id = EMU_QUIRK_SR_44100HZ as u32,
    }

    snd_emuusb_set_samplerate((*subs).stream, emu_samplerate_id);
    (*subs).data_endpoint.as_mut().map(|ep| {
        if emu_samplerate_id >= EMU_QUIRK_SR_176400HZ as u32 {
            // pkt_offset_adj = 4
        } else {
            // pkt_offset_adj = 0
        }
    });
}

pub unsafe fn pioneer_djm_set_format_quirk(subs: *mut snd_usb_substream, windex: u16) -> i32 {
    let cur_rate = (*(*subs).data_endpoint).cur_rate;
    let mut sr: [u8; 3] = [0; 3];
    sr[0] = (cur_rate & 0xff) as u8;
    sr[1] = ((cur_rate >> 8) & 0xff) as u8;
    sr[2] = ((cur_rate >> 16) & 0xff) as u8;
    usb_set_interface((*subs).dev, 0, 1);
    snd_usb_ctl_msg((*(*subs).stream).chip,
                   usb_sndctrlpipe((*(*subs).stream).chip, 0),
                   0x01, 0x22, 0x0100, windex as u16,
                   sr.as_mut_ptr() as *mut core::ffi::c_void, 0x0003);
    0
}

pub unsafe fn mbox3_set_format_quirk(subs: *mut snd_usb_substream, _fmt: *const audioformat) {
    todo!("mbox3_set_format_quirk - Mbox3 format setup")
}

static RME_DIGIFACE_RATE_TABLE: [i32; 12] = [
    32000, 44100, 48000, 0,
    64000, 88200, 96000, 0,
    128000, 176400, 192000, 0,
];

pub unsafe fn rme_digiface_set_format_quirk(subs: *mut snd_usb_substream) -> i32 {
    let cur_rate = (*(*subs).data_endpoint).cur_rate;
    let mut id = 0;
    for (i, &rate) in RME_DIGIFACE_RATE_TABLE.iter().enumerate() {
        if rate == cur_rate as i32 {
            id = i;
            break;
        }
    }
    if id >= RME_DIGIFACE_RATE_TABLE.len() {
        return EINVAL;
    }
    let speed_mode = ((id >> 2) + 2) as u16;
    let val = ((id as u16) << 3) | (speed_mode << 12);
    snd_usb_ctl_msg((*(*subs).stream).chip,
                   usb_sndctrlpipe((*(*subs).stream).chip, 0),
                   16, 0x40, val, 0x7078, core::ptr::null_mut(), 0);
    0
}

pub unsafe fn snd_usb_set_format_quirk(subs: *mut snd_usb_substream, fmt: *const audioformat) {
    match (*(*subs).stream).chip.as_ref().unwrap().usb_id {
        id if id == USB_ID(0x041e, 0x3f02) || id == USB_ID(0x041e, 0x3f04) ||
             id == USB_ID(0x041e, 0x3f0a) || id == USB_ID(0x041e, 0x3f19) => {
            set_format_emu_quirk(subs, fmt);
        }
        id if id == USB_ID(0x534d, 0x0021) || id == USB_ID(0x534d, 0x2109) => {
            // stream_offset_adj = 2
        }
        id if id == USB_ID(0x2b73, 0x000a) || id == USB_ID(0x2b73, 0x0013) ||
             id == USB_ID(0x2b73, 0x0034) => {
            pioneer_djm_set_format_quirk(subs, 0x0082);
        }
        id if id == USB_ID(0x08e4, 0x017f) || id == USB_ID(0x08e4, 0x0163) => {
            pioneer_djm_set_format_quirk(subs, 0x0086);
        }
        id if id == USB_ID(0x0dba, 0x5000) => {
            mbox3_set_format_quirk(subs, fmt);
        }
        id if id == USB_ID(0x2a39, 0x3f8c) || id == USB_ID(0x2a39, 0x3fa0) => {
            rme_digiface_set_format_quirk(subs);
        }
        _ => {}
    }
}

pub unsafe fn snd_usb_select_mode_quirk(chip: *mut snd_usb_audio, fmt: *const audioformat) -> i32 {
    let dev = (*chip).dev;
    if ((*chip).quirk_flags & QUIRK_FLAG_ITF_USB_DSD_DAC) != 0 {
        let mut err = usb_set_interface(dev, (*fmt).iface, 0);
        if err < 0 {
            return err;
        }
        msleep(20);
        if ((*fmt).formats & SNDRV_PCM_FMTBIT_DSD_U32_BE) != 0 {
            err = snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0), 0,
                                 (USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_INTERFACE) as u8,
                                 1, 1, core::ptr::null_mut(), 0);
            if err < 0 {
                return err;
            }
        } else {
            err = snd_usb_ctl_msg(dev, usb_sndctrlpipe(dev, 0), 0,
                                 (USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_INTERFACE) as u8,
                                 0, 1, core::ptr::null_mut(), 0);
            if err < 0 {
                return err;
            }
        }
        msleep(20);
    }
    0
}

pub unsafe fn snd_usb_endpoint_start_quirk(ep: *mut snd_usb_endpoint) {
    if USB_ID_VENDOR((*(*ep).chip).usb_id) == 0x23ba &&
       (*ep).type_ == SND_USB_ENDPOINT_TYPE_SYNC {
        (*ep).skip_packets = 4;
    }
    if ((*(*ep).chip).usb_id == USB_ID(0x0763, 0x2030) ||
        (*(*ep).chip).usb_id == USB_ID(0x0763, 0x2031)) &&
       (*ep).type_ == SND_USB_ENDPOINT_TYPE_DATA {
        (*ep).skip_packets = 16;
    }
    if ((*(*ep).chip).usb_id == USB_ID(0x0644, 0x8038) ||
        (*(*ep).chip).usb_id == USB_ID(0x1852, 0x5034)) &&
       (*ep).syncmaxsize == 4 {
        (*ep).tenor_fb_quirk = 1;
    }
}

pub unsafe fn snd_usb_ctl_msg_quirk(
    dev: *mut usb_device,
    _pipe: u32,
    _request: u8,
    requesttype: u8,
    _value: u16,
    _index: u16,
    _data: *mut core::ffi::c_void,
    _size: u16,
) {
    let chip = dev_get_drvdata(&(*dev).dev) as *mut snd_usb_audio;
    if chip.is_null() || (requesttype & USB_TYPE_CLASS as u8) != USB_TYPE_CLASS as u8 {
        return;
    }
    if ((*chip).quirk_flags & QUIRK_FLAG_CTL_MSG_DELAY) != 0 {
        msleep(20);
    } else if ((*chip).quirk_flags & QUIRK_FLAG_CTL_MSG_DELAY_1M) != 0 {
        usleep_range(1000, 2000);
    } else if ((*chip).quirk_flags & QUIRK_FLAG_CTL_MSG_DELAY_5M) != 0 {
        usleep_range(5000, 6000);
    }
}

pub unsafe fn snd_usb_interface_dsd_format_quirks(
    chip: *mut snd_usb_audio,
    fp: *mut audioformat,
    _sample_bytes: u32,
) -> u64 {
    let vendor = USB_ID_VENDOR((*chip).usb_id);
    let product = USB_ID_PRODUCT((*chip).usb_id);

    if vendor == 0x23ba && (product as u32) < 0x0110 {
        match (*fp).altsetting {
            1 => {
                (*fp).dsd_dop = true;
                return SNDRV_PCM_FMTBIT_DSD_U16_LE;
            }
            2 => {
                (*fp).dsd_bitrev = true;
                return SNDRV_PCM_FMTBIT_DSD_U8;
            }
            3 => {
                (*fp).dsd_bitrev = true;
                return SNDRV_PCM_FMTBIT_DSD_U16_LE;
            }
            _ => {}
        }
    }

    match (*chip).usb_id {
        id if id == USB_ID(0x139f, 0x5504) || id == USB_ID(0x20b1, 0x3089) ||
             id == USB_ID(0x2522, 0x0007) || id == USB_ID(0x2522, 0x0009) ||
             id == USB_ID(0x2522, 0x0012) || id == USB_ID(0x2772, 0x0230) => {
            if (*fp).altsetting == 2 {
                return SNDRV_PCM_FMTBIT_DSD_U32_BE;
            }
        }
        id if id == USB_ID(0x0d8c, 0x0316) || id == USB_ID(0x10cb, 0x0103) ||
             id == USB_ID(0x16d0, 0x06b2) || id == USB_ID(0x16d0, 0x06b4) ||
             id == USB_ID(0x16d0, 0x0733) || id == USB_ID(0x16d0, 0x09d8) ||
             id == USB_ID(0x16d0, 0x09db) || id == USB_ID(0x16d0, 0x09dd) ||
             id == USB_ID(0x16d0, 0x0ab1) || id == USB_ID(0x16d0, 0xeca1) ||
             id == USB_ID(0x1db5, 0x0003) || id == USB_ID(0x20a0, 0x4143) ||
             id == USB_ID(0x22e1, 0xca01) || id == USB_ID(0x249c, 0x9326) ||
             id == USB_ID(0x2616, 0x0106) || id == USB_ID(0x2622, 0x0041) ||
             id == USB_ID(0x2622, 0x0061) || id == USB_ID(0x278b, 0x5100) ||
             id == USB_ID(0x27f7, 0x3002) || id == USB_ID(0x29a2, 0x0086) ||
             id == USB_ID(0x6b42, 0x0042) => {
            if (*fp).altsetting == 3 {
                return SNDRV_PCM_FMTBIT_DSD_U32_BE;
            }
        }
        id if id == USB_ID(0x16d0, 0x071a) => {
            if (*fp).altsetting == 2 {
                match le16_to_cpu((*(*chip).dev).descriptor.bcdDevice as u16) {
                    0x199 => return SNDRV_PCM_FMTBIT_DSD_U32_LE,
                    0x19b | 0x203 => return SNDRV_PCM_FMTBIT_DSD_U32_BE,
                    _ => {}
                }
            }
        }
        id if id == USB_ID(0x16d0, 0x0a23) => {
            if (*fp).altsetting == 2 {
                return SNDRV_PCM_FMTBIT_DSD_U32_BE;
            }
        }
        _ => {}
    }

    if ((*chip).quirk_flags & QUIRK_FLAG_ITF_USB_DSD_DAC) != 0 {
        let iface = usb_ifnum_to_if((*chip).dev, (*fp).iface);
        if !iface.is_null() && (*fp).altsetting == (*iface).num_altsetting - 1 {
            return SNDRV_PCM_FMTBIT_DSD_U32_BE;
        }
    }

    if ((*chip).quirk_flags & QUIRK_FLAG_DSD_RAW) != 0 && (*fp).dsd_raw {
        return SNDRV_PCM_FMTBIT_DSD_U32_BE;
    }

    0
}

pub unsafe fn snd_usb_audioformat_attributes_quirk(
    chip: *mut snd_usb_audio,
    fp: *mut audioformat,
    stream: i32,
) {
    match (*chip).usb_id {
        id if id == USB_ID(0x0a92, 0x0053) => {
            (*fp).attributes &= !UAC_EP_CS_ATTR_SAMPLE_RATE;
        }
        id if id == USB_ID(0x041e, 0x3020) || id == USB_ID(0x0763, 0x2003) => {
            (*fp).attributes |= UAC_EP_CS_ATTR_SAMPLE_RATE;
        }
        id if id == USB_ID(0x0763, 0x2001) || id == USB_ID(0x0763, 0x2012) ||
             id == USB_ID(0x047f, 0x0ca1) || id == USB_ID(0x077d, 0x07af) => {
            (*fp).ep_attr &= !(USB_ENDPOINT_SYNCTYPE as u32);
            if stream == SNDRV_PCM_STREAM_PLAYBACK as i32 {
                (*fp).ep_attr |= USB_ENDPOINT_SYNC_ADAPTIVE as u32;
            } else {
                (*fp).ep_attr |= USB_ENDPOINT_SYNC_SYNC as u32;
            }
        }
        id if id == USB_ID(0x07fd, 0x0004) => {
            (*fp).attributes &= !UAC_EP_CS_ATTR_FILL_MAX;
        }
        id if id == USB_ID(0x1224, 0x2a25) => {
            (*fp).attributes |= UAC_EP_CS_ATTR_FILL_MAX;
        }
        id if id == USB_ID(0x3511, 0x2b1e) => {
            if stream == SNDRV_PCM_STREAM_CAPTURE as i32 {
                (*fp).attributes &= !UAC_EP_CS_ATTR_PITCH_CONTROL;
            }
        }
        _ => {}
    }
}

static QUIRK_FLAGS_TABLE: [usb_audio_quirk_flags_table; 1] = [
    usb_audio_quirk_flags_table {
        id: 0,
        flags: 0,
        usb_string_match: core::ptr::null(),
    }
];

pub unsafe fn snd_usb_quirk_flags_from_name(_name: *const core::ffi::c_char) -> u64 {
    0
}

pub unsafe fn snd_usb_apply_flag_dbg(
    _reason: *const core::ffi::c_char,
    _chip: *mut snd_usb_audio,
    _flag: u64,
) {}

pub unsafe fn snd_usb_init_quirk_flags_table(chip: *mut snd_usb_audio) {
    let p = QUIRK_FLAGS_TABLE.as_ptr();
    if !p.is_null() && (*p).id != 0 {
        if (*chip).usb_id == (*p).id {
            (*chip).quirk_flags |= (*p).flags;
        }
    }
}

pub unsafe fn snd_usb_init_quirk_flags_parse_string(_chip: *mut snd_usb_audio, _str: *const core::ffi::c_char) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
