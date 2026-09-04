// SPDX-License-Identifier: GPL-2.0

// Dependency: ../usbaudio.h - supplies snd_usb_audio, urb, snd_pcm, snd_rawmidi, input_dev, spinlock_t, wait_queue_head_t, and other C types

pub const USB_VID_NATIVEINSTRUMENTS: u16 = 0x17cc;

pub const USB_PID_RIGKONTROL2: u16 = 0x1969;
pub const USB_PID_RIGKONTROL3: u16 = 0x1940;
pub const USB_PID_KORECONTROLLER: u16 = 0x4711;
pub const USB_PID_KORECONTROLLER2: u16 = 0x4712;
pub const USB_PID_AK1: u16 = 0x0815;
pub const USB_PID_AUDIO2DJ: u16 = 0x041c;
pub const USB_PID_AUDIO4DJ: u16 = 0x0839;
pub const USB_PID_AUDIO8DJ: u16 = 0x1978;
pub const USB_PID_SESSIONIO: u16 = 0x1915;
pub const USB_PID_GUITARRIGMOBILE: u16 = 0x0d8d;
pub const USB_PID_TRAKTORKONTROLX1: u16 = 0x2305;
pub const USB_PID_TRAKTORKONTROLS4: u16 = 0xbaff;
pub const USB_PID_TRAKTORAUDIO2: u16 = 0x041d;
pub const USB_PID_MASCHINECONTROLLER: u16 = 0x0808;

pub const EP1_BUFSIZE: usize = 64;
pub const EP4_BUFSIZE: usize = 512;
pub const CAIAQ_USB_STR_LEN: usize = 0xff;
pub const MAX_STREAMS: usize = 32;

pub const MODNAME: &[u8] = b"snd-usb-caiaq";

pub const EP1_CMD_GET_DEVICE_INFO: u8 = 0x1;
pub const EP1_CMD_READ_ERP: u8 = 0x2;
pub const EP1_CMD_READ_ANALOG: u8 = 0x3;
pub const EP1_CMD_READ_IO: u8 = 0x4;
pub const EP1_CMD_WRITE_IO: u8 = 0x5;
pub const EP1_CMD_MIDI_READ: u8 = 0x6;
pub const EP1_CMD_MIDI_WRITE: u8 = 0x7;
pub const EP1_CMD_AUDIO_PARAMS: u8 = 0x9;
pub const EP1_CMD_AUTO_MSG: u8 = 0xb;
pub const EP1_CMD_DIMM_LEDS: u8 = 0xc;

#[repr(C, packed)]
pub struct caiaq_device_spec {
    pub fw_version: u16,
    pub hw_subtype: u8,
    pub num_erp: u8,
    pub num_analog_in: u8,
    pub num_digital_in: u8,
    pub num_digital_out: u8,
    pub num_analog_audio_out: u8,
    pub num_analog_audio_in: u8,
    pub num_digital_audio_out: u8,
    pub num_digital_audio_in: u8,
    pub num_midi_out: u8,
    pub num_midi_in: u8,
    pub data_alignment: u8,
}

pub struct snd_usb_caiaq_cb_info;

pub struct snd_usb_caiaqdev {
    pub chip: snd_usb_audio,
    pub ep1_in_urb: urb,
    pub midi_out_urb: urb,
    pub data_urbs_in: *mut *mut urb,
    pub data_urbs_out: *mut *mut urb,
    pub data_cb_info: *mut snd_usb_caiaq_cb_info,
    pub ep1_in_buf: [u8; EP1_BUFSIZE],
    pub ep1_out_buf: [u8; EP1_BUFSIZE],
    pub midi_out_buf: [u8; EP1_BUFSIZE],
    pub spec: caiaq_device_spec,
    pub spinlock: spinlock_t,
    pub ep1_wait_queue: wait_queue_head_t,
    pub prepare_wait_queue: wait_queue_head_t,
    pub spec_received: i32,
    pub audio_parm_answer: i32,
    pub midi_out_active: i32,
    pub vendor_name: [i8; CAIAQ_USB_STR_LEN],
    pub product_name: [i8; CAIAQ_USB_STR_LEN],
    pub n_streams: i32,
    pub n_audio_in: i32,
    pub n_audio_out: i32,
    pub streaming: i32,
    pub first_packet: i32,
    pub output_running: i32,
    pub audio_in_buf_pos: [i32; MAX_STREAMS],
    pub audio_out_buf_pos: [i32; MAX_STREAMS],
    pub period_in_count: [i32; MAX_STREAMS],
    pub period_out_count: [i32; MAX_STREAMS],
    pub input_panic: i32,
    pub output_panic: i32,
    pub warned: i32,
    pub audio_in_buf: *mut i8,
    pub audio_out_buf: *mut i8,
    pub samplerates: u32,
    pub bpp: u32,
    pub outurb_active_mask: u64,
    pub sub_playback: [*mut snd_pcm_substream; MAX_STREAMS],
    pub sub_capture: [*mut snd_pcm_substream; MAX_STREAMS],
    pub control_state: [u8; 256],
    pub ep8_out_buf: [u8; 2],
    // CONFIG_SND_USB_CAIAQ_INPUT section (conditionally compiled in C)
    pub input_dev: *mut input_dev,
    pub phys: [i8; 64],
    pub keycode: [u16; 128],
    pub ep4_in_urb: *mut urb,
    pub ep4_in_buf: [u8; EP4_BUFSIZE],
    // ALSA section
    pub pcm: *mut snd_pcm,
    pub pcm_info: snd_pcm_hardware,
    pub rmidi: *mut snd_rawmidi,
    pub midi_receive_substream: *mut snd_rawmidi_substream,
    pub midi_out_substream: *mut snd_rawmidi_substream,
}

pub struct snd_usb_caiaq_cb_info {
    pub cdev: *mut snd_usb_caiaqdev,
    pub index: i32,
}

pub unsafe fn caiaqdev(c: *const snd_usb_audio) -> *mut snd_usb_caiaqdev {
    (*c).private_data as *mut snd_usb_caiaqdev
}

pub fn caiaqdev_to_dev(d: *const snd_usb_caiaqdev) -> *mut device {
    unsafe { (*(*(*d).chip.card).dev) }
}

extern "C" {
    pub fn snd_usb_caiaq_set_audio_params(
        cdev: *mut snd_usb_caiaqdev,
        rate: i32,
        depth: i32,
        bbp: i32,
    ) -> i32;

    pub fn snd_usb_caiaq_set_auto_msg(
        cdev: *mut snd_usb_caiaqdev,
        digital: i32,
        analog: i32,
        erp: i32,
    ) -> i32;

    pub fn snd_usb_caiaq_send_command(
        cdev: *mut snd_usb_caiaqdev,
        command: u8,
        buffer: *const u8,
        len: i32,
    ) -> i32;

    pub fn snd_usb_caiaq_send_command_bank(
        cdev: *mut snd_usb_caiaqdev,
        command: u8,
        bank: u8,
        buffer: *const u8,
        len: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
