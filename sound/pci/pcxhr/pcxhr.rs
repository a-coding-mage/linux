// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * main file with alsa callbacks
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const DRIVER_NAME: &[u8] = b"pcxhr\0";

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS]; /* Enable this card */
static mut mono: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS]; /* capture  mono only */

/* module parameters and module metadata from the C source are build-system declarations. */

type u32 = u32;
type u_int32_t = u32;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type dma_addr_t = u64;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const IRQF_SHARED: c_uint = 0x80;
const THIS_MODULE: *mut c_void = ptr::null_mut();
const KBUILD_MODNAME: *const c_char = DRIVER_NAME.as_ptr() as *const c_char;

const PCI_ID_VX882HR: usize = 0;
const PCI_ID_PCX882HR: usize = 1;
const PCI_ID_VX881HR: usize = 2;
const PCI_ID_PCX881HR: usize = 3;
const PCI_ID_VX882E: usize = 4;
const PCI_ID_PCX882E: usize = 5;
const PCI_ID_VX881E: usize = 6;
const PCI_ID_PCX881E: usize = 7;
const PCI_ID_VX1222HR: usize = 8;
const PCI_ID_PCX1222HR: usize = 9;
const PCI_ID_VX1221HR: usize = 10;
const PCI_ID_PCX1221HR: usize = 11;
const PCI_ID_VX1222E: usize = 12;
const PCI_ID_PCX1222E: usize = 13;
const PCI_ID_VX1221E: usize = 14;
const PCI_ID_PCX1221E: usize = 15;
const PCI_ID_VX222HR: usize = 16;
const PCI_ID_VX222E: usize = 17;
const PCI_ID_PCX22HR: usize = 18;
const PCI_ID_PCX22E: usize = 19;
const PCI_ID_VX222HRMIC: usize = 20;
const PCI_ID_VX222E_MIC: usize = 21;
const PCI_ID_PCX924HR: usize = 22;
const PCI_ID_PCX924E: usize = 23;
const PCI_ID_PCX924HRMIC: usize = 24;
const PCI_ID_PCX924E_MIC: usize = 25;
const PCI_ID_VX442HR: usize = 26;
const PCI_ID_PCX442HR: usize = 27;
const PCI_ID_VX442E: usize = 28;
const PCI_ID_PCX442E: usize = 29;
const PCI_ID_VX822HR: usize = 30;
const PCI_ID_PCX822HR: usize = 31;
const PCI_ID_VX822E: usize = 32;
const PCI_ID_PCX822E: usize = 33;
const PCI_ID_LAST: usize = 34;

#[repr(C)]
struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: c_ulong,
}

const fn PCI_DEVICE_SUB(vendor: u32, device: u32, subvendor: u32, subdevice: u32, data: usize) -> pci_device_id {
    pci_device_id { vendor, device, subvendor, subdevice, class: 0, class_mask: 0, driver_data: data as c_ulong }
}

static pcxhr_ids: [pci_device_id; 35] = [
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb001, PCI_ID_VX882HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb101, PCI_ID_PCX882HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb201, PCI_ID_VX881HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb301, PCI_ID_PCX881HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb021, PCI_ID_VX882E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb121, PCI_ID_PCX882E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb221, PCI_ID_VX881E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb321, PCI_ID_PCX881E),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb401, PCI_ID_VX1222HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb501, PCI_ID_PCX1222HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb601, PCI_ID_VX1221HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xb701, PCI_ID_PCX1221HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb421, PCI_ID_VX1222E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb521, PCI_ID_PCX1222E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb621, PCI_ID_VX1221E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xb721, PCI_ID_PCX1221E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xba01, PCI_ID_VX222HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xba21, PCI_ID_VX222E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbd01, PCI_ID_PCX22HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbd21, PCI_ID_PCX22E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbc01, PCI_ID_VX222HRMIC),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbc21, PCI_ID_VX222E_MIC),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbb01, PCI_ID_PCX924HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbb21, PCI_ID_PCX924E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbf01, PCI_ID_PCX924HRMIC),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xbf21, PCI_ID_PCX924E_MIC),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xd001, PCI_ID_VX442HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xd101, PCI_ID_PCX442HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xd021, PCI_ID_VX442E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xd121, PCI_ID_PCX442E),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xd201, PCI_ID_VX822HR),
    PCI_DEVICE_SUB(0x10b5, 0x9656, 0x1369, 0xd301, PCI_ID_PCX822HR),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xd221, PCI_ID_VX822E),
    PCI_DEVICE_SUB(0x10b5, 0x9056, 0x1369, 0xd321, PCI_ID_PCX822E),
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

#[repr(C)]
struct board_parameters {
    board_name: *const c_char,
    playback_chips: i16,
    capture_chips: i16,
    fw_file_set: i16,
    firmware_num: i16,
}

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static pcxhr_board_params: [board_parameters; PCI_ID_LAST] = [
    board_parameters { board_name: cstr!("VX882HR"), playback_chips: 4, capture_chips: 4, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX882HR"), playback_chips: 4, capture_chips: 4, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX881HR"), playback_chips: 4, capture_chips: 4, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX881HR"), playback_chips: 4, capture_chips: 4, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX882e"), playback_chips: 4, capture_chips: 4, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX882e"), playback_chips: 4, capture_chips: 4, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX881e"), playback_chips: 4, capture_chips: 4, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX881e"), playback_chips: 4, capture_chips: 4, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX1222HR"), playback_chips: 6, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX1222HR"), playback_chips: 6, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("VX1221HR"), playback_chips: 6, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX1221HR"), playback_chips: 6, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("VX1222e"), playback_chips: 6, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX1222e"), playback_chips: 6, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
    board_parameters { board_name: cstr!("VX1221e"), playback_chips: 6, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX1221e"), playback_chips: 6, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
    board_parameters { board_name: cstr!("VX222HR"), playback_chips: 1, capture_chips: 1, fw_file_set: 4, firmware_num: 44 },
    board_parameters { board_name: cstr!("VX222e"), playback_chips: 1, capture_chips: 1, fw_file_set: 4, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX22HR"), playback_chips: 1, capture_chips: 0, fw_file_set: 4, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX22e"), playback_chips: 1, capture_chips: 0, fw_file_set: 4, firmware_num: 44 },
    board_parameters { board_name: cstr!("VX222HR-Mic"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("VX222e-Mic"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX924HR"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX924e"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX924HR-Mic"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("PCX924e-Mic"), playback_chips: 1, capture_chips: 1, fw_file_set: 5, firmware_num: 44 },
    board_parameters { board_name: cstr!("VX442HR"), playback_chips: 2, capture_chips: 2, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX442HR"), playback_chips: 2, capture_chips: 2, fw_file_set: 0, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX442e"), playback_chips: 2, capture_chips: 2, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("PCX442e"), playback_chips: 2, capture_chips: 2, fw_file_set: 1, firmware_num: 41 },
    board_parameters { board_name: cstr!("VX822HR"), playback_chips: 4, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX822HR"), playback_chips: 4, capture_chips: 1, fw_file_set: 2, firmware_num: 42 },
    board_parameters { board_name: cstr!("VX822e"), playback_chips: 4, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
    board_parameters { board_name: cstr!("PCX822e"), playback_chips: 4, capture_chips: 1, fw_file_set: 3, firmware_num: 42 },
];

/* boards without hw AES1 and SRC onboard are all using fw_file_set==4 */
/* VX222HR, VX222e, PCX22HR and PCX22e */
unsafe fn PCXHR_BOARD_HAS_AES1(x: *const pcxhr_mgr) -> bool { (*x).fw_file_set != 4 }
/* some boards do not support 192kHz on digital AES input plugs */
unsafe fn PCXHR_BOARD_AESIN_NO_192K(x: *const pcxhr_mgr) -> bool {
    (*x).capture_chips == 0 || (*x).fw_file_set == 0 || (*x).fw_file_set == 2
}

const PCXHR_FREQ_REG_MASK: c_uint = 0x1f;
const PCXHR_FREQ_QUARTZ_48000: c_uint = 0x00;
const PCXHR_FREQ_QUARTZ_24000: c_uint = 0x01;
const PCXHR_FREQ_QUARTZ_12000: c_uint = 0x09;
const PCXHR_FREQ_QUARTZ_32000: c_uint = 0x08;
const PCXHR_FREQ_QUARTZ_16000: c_uint = 0x04;
const PCXHR_FREQ_QUARTZ_8000: c_uint = 0x0c;
const PCXHR_FREQ_QUARTZ_44100: c_uint = 0x02;
const PCXHR_FREQ_QUARTZ_22050: c_uint = 0x0a;
const PCXHR_FREQ_QUARTZ_11025: c_uint = 0x06;
const PCXHR_FREQ_PLL: c_uint = 0x05;
const PCXHR_FREQ_QUARTZ_192000: c_uint = 0x10;
const PCXHR_FREQ_QUARTZ_96000: c_uint = 0x18;
const PCXHR_FREQ_QUARTZ_176400: c_uint = 0x14;
const PCXHR_FREQ_QUARTZ_88200: c_uint = 0x1c;
const PCXHR_FREQ_QUARTZ_128000: c_uint = 0x12;
const PCXHR_FREQ_QUARTZ_64000: c_uint = 0x1a;
const PCXHR_FREQ_WORD_CLOCK: c_uint = 0x0f;
const PCXHR_FREQ_SYNC_AES: c_uint = 0x0e;
const PCXHR_FREQ_AES_1: c_uint = 0x07;
const PCXHR_FREQ_AES_2: c_uint = 0x0b;
const PCXHR_FREQ_AES_3: c_uint = 0x03;
const PCXHR_FREQ_AES_4: c_uint = 0x0d;
const PCXHR_MODIFY_CLOCK_S_BIT: c_uint = 0x04;
const PCXHR_IRQ_TIMER_FREQ: c_uint = 92000;
const PCXHR_IRQ_TIMER_PERIOD: c_uint = 48;

const HEADER_FMT_BASE_LIN: c_uint = 0xfed00000;
const HEADER_FMT_BASE_FLOAT: c_uint = 0xfad00000;
const HEADER_FMT_INTEL: c_uint = 0x00008000;
const HEADER_FMT_24BITS: c_uint = 0x00004000;
const HEADER_FMT_16BITS: c_uint = 0x00002000;
const HEADER_FMT_UPTO11: c_uint = 0x00000200;
const HEADER_FMT_UPTO32: c_uint = 0x00000100;
const HEADER_FMT_MONO: c_uint = 0x00000080;

const TIME_CODE_VALID_MASK: c_uint = 0x00800000;
const TIME_CODE_NEW_MASK: c_uint = 0x00400000;
const TIME_CODE_BACK_MASK: c_uint = 0x00200000;
const TIME_CODE_WAIT_MASK: c_uint = 0x00100000;
const MANAGE_SIGNAL_TIME_CODE: c_uint = 0x01;
const MANAGE_SIGNAL_MIDI: c_uint = 0x02;

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct snd_info_entry { private_data: *mut c_void }
#[repr(C)] struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_device { device_data: *mut c_void }
#[repr(C)] struct pcxhr_hostport { _private: [u8; 0] }
#[repr(C)] struct snd_dma_buffer { area: *mut c_void }
#[repr(C)] struct pci_dev { dev: device, irq: c_int }
#[repr(C)] struct snd_card {
    dev: *mut device,
    sync_irq: c_int,
    driver: [c_char; 32],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}
#[repr(C)] struct snd_pcm { private_data: *mut c_void, info_flags: c_uint, nonatomic: bool, name: [c_char; 80] }
#[repr(C)] struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: c_uint,
    periods_max: c_uint,
}
#[repr(C)] struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    dma_addr: dma_addr_t,
    dma_bytes: usize,
    period_size: snd_pcm_uframes_t,
    periods: c_uint,
    buffer_size: snd_pcm_uframes_t,
    rate: c_uint,
    private_data: *mut c_void,
}
#[repr(C)] struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
    number: c_int,
}
#[repr(C)] struct pcxhr_pipe { is_capture: c_int, first_audio: c_int }
#[repr(C)] struct pcxhr_stream {
    status: c_int,
    substream: *mut snd_pcm_substream,
    timer_abs_periods: c_uint,
    timer_period_frag: u_int32_t,
    timer_buf_periods: c_int,
    timer_is_synced: c_int,
    pipe: *mut pcxhr_pipe,
    format: c_int,
    channels: c_uint,
}
#[repr(C)] struct pcxhr_rmh {
    cmd: [u32; 16],
    stat: [u32; 32],
    cmd_len: c_int,
    stat_len: c_int,
    dsp_stat: c_int,
    cmd_idx: c_int,
}
#[repr(C)] struct snd_pcxhr {
    card: *mut snd_card,
    chip_idx: c_int,
    mgr: *mut pcxhr_mgr,
    nb_streams_play: c_int,
    nb_streams_capt: c_int,
    playback_stream: [pcxhr_stream; 8],
    capture_stream: [pcxhr_stream; 8],
    pcm: *mut snd_pcm,
}
#[repr(C)] struct pcxhr_mgr {
    use_clock_type: c_int,
    codec_speed: c_uint,
    sample_rate_real: c_int,
    cur_clock_type: c_int,
    is_hr_stereo: bool,
    pci: *mut pci_dev,
    last_reg_stat: u8,
    num_cards: c_uint,
    chip: [*mut snd_pcxhr; 8],
    playback_chips: c_int,
    capture_chips: c_int,
    mono_capture: bool,
    fw_file_set: c_int,
    firmware_num: c_int,
    board_has_aes1: bool,
    board_aes_in_192k: bool,
    granularity: c_uint,
    setup_mutex: mutex,
    lock: mutex,
    msg_lock: mutex,
    dsp_time_last: u32,
    sample_rate: c_uint,
    ref_count_rate: c_int,
    irq: c_int,
    port: [c_ulong; 3],
    name: [c_char; 80],
    prmh: *mut pcxhr_rmh,
    hostport: snd_dma_buffer,
    dsp_loaded: c_uint,
    dsp_version: c_uint,
    board_has_analog: bool,
    dsp_time_err: c_int,
    async_err_pipe_xrun: c_int,
    async_err_stream_xrun: c_int,
    async_err_other_last: c_int,
    capture_ltc: c_int,
}
#[repr(C)] struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)] struct snd_device_ops { dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int> }
#[repr(C)] struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

extern "C" {
    fn pcxhr_init_rmh(rmh: *mut pcxhr_rmh, cmd: c_int);
    fn pcxhr_send_msg(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> c_int;
    fn pcxhr_write_io_num_reg_cont(mgr: *mut pcxhr_mgr, mask: c_uint, val: c_uint, changed: *mut c_int) -> c_int;
    fn hr222_sub_set_clock(mgr: *mut pcxhr_mgr, rate: c_uint, changed: *mut c_int) -> c_int;
    fn hr222_get_external_clock(mgr: *mut pcxhr_mgr, clock_type: c_int, sample_rate: *mut c_int) -> c_int;
    fn pcxhr_set_pipe_cmd_params(rmh: *mut pcxhr_rmh, capture: c_int, audio: c_int, stream: c_int, mask: c_int);
    fn pcxhr_set_pipe_state(mgr: *mut pcxhr_mgr, playback_mask: c_int, capture_mask: c_int, start: c_int) -> c_int;
    fn pcxhr_interrupt(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn pcxhr_threaded_irq(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn pcxhr_reset_board(mgr: *mut pcxhr_mgr);
    fn pcxhr_setup_firmware(mgr: *mut pcxhr_mgr) -> c_int;
    fn hr222_read_gpio(mgr: *mut pcxhr_mgr, is_gpi: c_int, value: *mut c_int) -> c_int;
    fn hr222_write_gpo(mgr: *mut pcxhr_mgr, value: c_int) -> c_int;
    fn hr222_manage_timecode(mgr: *mut pcxhr_mgr, enable: c_int);
    fn snd_pcm_substream_chip(subs: *mut snd_pcm_substream) -> *mut snd_pcxhr;
    fn snd_pcm_stream_linked(subs: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_int, step: c_uint) -> c_int;
    fn snd_pcm_set_sync(subs: *mut snd_pcm_substream);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_device_new(card: *mut snd_card, ty: c_int, device_data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_card_rw_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer), write: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn params_channels(hw: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(hw: *mut snd_pcm_hw_params) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_new(parent: *mut device, idx: c_int, xid: *const c_char, module: *mut c_void, extra_size: c_int, card_ret: *mut *mut snd_card) -> c_int;
    fn pci_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_disable_device(pci: *mut pci_dev);
    fn pci_set_master(pci: *mut pci_dev);
    fn dma_set_mask(dev: *mut device, mask: u64) -> c_int;
    fn pci_request_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_release_regions(pci: *mut pci_dev);
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn request_threaded_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn snd_dma_alloc_pages(ty: c_int, dev: *mut device, size: usize, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut pcxhr_mgr;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn udelay(usecs: c_uint);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
}

const PCXHR_CLOCK_TYPE_INTERNAL: c_int = 0;
const PCXHR_CLOCK_TYPE_WORD_CLOCK: c_int = 1;
const PCXHR_CLOCK_TYPE_AES_SYNC: c_int = 2;
const PCXHR_CLOCK_TYPE_AES_1: c_int = 3;
const PCXHR_CLOCK_TYPE_AES_2: c_int = 4;
const PCXHR_CLOCK_TYPE_AES_3: c_int = 5;
const PCXHR_CLOCK_TYPE_AES_4: c_int = 6;
const PCXHR_CLOCK_TYPE_MAX: c_int = 6;
const HR22_CLOCK_TYPE_MAX: c_int = 2;
const CMD_ACCESS_IO_WRITE: c_int = 0;
const CMD_ACCESS_IO_READ: c_int = 1;
const CMD_MODIFY_CLOCK: c_int = 2;
const CMD_START_STREAM: c_int = 3;
const CMD_STOP_STREAM: c_int = 4;
const CMD_FORMAT_STREAM_IN: c_int = 5;
const CMD_FORMAT_STREAM_OUT: c_int = 6;
const CMD_UPDATE_R_BUFFERS: c_int = 7;
const CMD_SET_TIMER_INTERRUPT: c_int = 8;
const CMD_GET_DSP_RESOURCES: c_int = 9;
const CMD_LAST_INDEX: c_int = 10;
const CMD_MANAGE_SIGNAL: c_int = 11;
const CMD_GET_TIME_CODE: c_int = 12;
const IO_NUM_REG_GENCLK: u32 = 0;
const IO_NUM_REG_MUTE_OUT: u32 = 0;
const IO_NUM_SPEED_RATIO: u32 = 0;
const IO_NUM_REG_STATUS: u32 = 0;
const MASK_DSP_WORD: u32 = 0x00ff_ffff;
const REG_STATUS_WORD_CLOCK: u8 = 0;
const REG_STATUS_AES_SYNC: u8 = 1;
const REG_STATUS_AES_1: u8 = 2;
const REG_STATUS_AES_2: u8 = 3;
const REG_STATUS_AES_3: u8 = 4;
const REG_STATUS_AES_4: u8 = 5;
const REG_STATUS_CURRENT: u32 = 0;
const REG_STATUS_SYNC_32000: u32 = 0;
const REG_STATUS_SYNC_44100: u32 = 1;
const REG_STATUS_SYNC_48000: u32 = 2;
const REG_STATUS_SYNC_64000: u32 = 3;
const REG_STATUS_SYNC_88200: u32 = 4;
const REG_STATUS_SYNC_96000: u32 = 5;
const REG_STATUS_SYNC_128000: u32 = 6;
const REG_STATUS_SYNC_176400: u32 = 7;
const REG_STATUS_SYNC_192000: u32 = 8;
const PCXHR_STREAM_STATUS_FREE: c_int = 0;
const PCXHR_STREAM_STATUS_OPEN: c_int = 1;
const PCXHR_STREAM_STATUS_SCHEDULE_RUN: c_int = 2;
const PCXHR_STREAM_STATUS_SCHEDULE_STOP: c_int = 3;
const PCXHR_STREAM_STATUS_STARTED: c_int = 4;
const PCXHR_STREAM_STATUS_STOPPED: c_int = 5;
const PCXHR_STREAM_STATUS_RUNNING: c_int = 6;
const SNDRV_PCM_FORMAT_U8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 1;
const SNDRV_PCM_FORMAT_S16_BE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 3;
const SNDRV_PCM_FORMAT_S24_3BE: c_int = 4;
const SNDRV_PCM_FORMAT_FLOAT_LE: c_int = 5;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_INFO_SYNC_START: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_3BE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_FLOAT_LE: u64 = 1 << 5;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 1 << 1;
const PCXHR_GRANULARITY: c_uint = 96;
const PCXHR_GRANULARITY_HR22: c_uint = 192;
const PCXHR_PLAYBACK_STREAMS: c_int = 4;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const PCXHR_DSP_TIME_INVALID: u32 = 0xffff_ffff;
const PCXHR_FIRMWARE_DSP_MAIN_INDEX: c_int = 0;
const PCXHR_SIZE_MAX_STATUS: c_int = 16;
const PCXHR_SIZE_MAX_LONG_STATUS: c_int = 32;
const REG_CONT_VALSMPTE: c_uint = 0;
const PCXHR_MAX_CARDS: c_uint = 8;

unsafe fn DSP_EXT_CMD_SET(_mgr: *mut pcxhr_mgr) -> bool { false }
unsafe fn snd_BUG_ON(cond: bool) -> bool { cond }
const fn DMA_BIT_MASK(n: u32) -> u64 { if n == 64 { !0 } else { (1u64 << n) - 1 } }
const fn PAGE_ALIGN(size: usize) -> usize { (size + 4095) & !4095 }
fn max(a: c_int, b: c_int) -> c_int { if a > b { a } else { b } }

#[no_mangle]
pub unsafe extern "C" fn pcxhr_pll_freq_register(freq: c_uint, max_freq: c_uint, pllreg: *mut c_uint, realfreq: *mut c_uint) -> c_int {
    let mut reg: c_uint;
    if freq < 6900 || freq > max_freq {
        return -EINVAL;
    }
    reg = (28224000u32.wrapping_mul(2)) / freq;
    reg = (reg - 1) / 2;
    if reg < 0x100 {
        *pllreg = reg + 0xc00;
    } else if reg < 0x200 {
        *pllreg = reg + 0x800;
    } else if reg < 0x400 {
        *pllreg = reg & 0x1ff;
    } else if reg < 0x800 {
        *pllreg = ((reg >> 1) & 0x1ff) + 0x200;
        reg &= !1;
    } else {
        *pllreg = ((reg >> 2) & 0x1ff) + 0x400;
        reg &= !3;
    }
    if !realfreq.is_null() {
        *realfreq = 28224000 / (reg + 1);
    }
    0
}

unsafe extern "C" fn pcxhr_get_clock_reg(mgr: *mut pcxhr_mgr, rate: c_uint, reg: *mut c_uint, freq: *mut c_uint) -> c_int {
    let mut val: c_uint;
    let mut realfreq: c_uint = rate;
    let mut pllreg: c_uint = 0;
    let mut rmh: pcxhr_rmh = zeroed();
    let mut err: c_int;

    match (*mgr).use_clock_type {
        PCXHR_CLOCK_TYPE_INTERNAL => {
            val = match rate {
                48000 => PCXHR_FREQ_QUARTZ_48000,
                24000 => PCXHR_FREQ_QUARTZ_24000,
                12000 => PCXHR_FREQ_QUARTZ_12000,
                32000 => PCXHR_FREQ_QUARTZ_32000,
                16000 => PCXHR_FREQ_QUARTZ_16000,
                8000 => PCXHR_FREQ_QUARTZ_8000,
                44100 => PCXHR_FREQ_QUARTZ_44100,
                22050 => PCXHR_FREQ_QUARTZ_22050,
                11025 => PCXHR_FREQ_QUARTZ_11025,
                192000 => PCXHR_FREQ_QUARTZ_192000,
                96000 => PCXHR_FREQ_QUARTZ_96000,
                176400 => PCXHR_FREQ_QUARTZ_176400,
                88200 => PCXHR_FREQ_QUARTZ_88200,
                128000 => PCXHR_FREQ_QUARTZ_128000,
                64000 => PCXHR_FREQ_QUARTZ_64000,
                _ => {
                    err = pcxhr_pll_freq_register(rate, 110000, &mut pllreg, &mut realfreq);
                    if err != 0 { return err; }
                    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
                    rmh.cmd[0] |= IO_NUM_REG_GENCLK;
                    rmh.cmd[1] = pllreg & MASK_DSP_WORD;
                    rmh.cmd[2] = pllreg >> 24;
                    rmh.cmd_len = 3;
                    err = pcxhr_send_msg(mgr, &mut rmh);
                    if err < 0 {
                        return err;
                    }
                    PCXHR_FREQ_PLL
                }
            };
        }
        PCXHR_CLOCK_TYPE_WORD_CLOCK => val = PCXHR_FREQ_WORD_CLOCK,
        PCXHR_CLOCK_TYPE_AES_SYNC => val = PCXHR_FREQ_SYNC_AES,
        PCXHR_CLOCK_TYPE_AES_1 => val = PCXHR_FREQ_AES_1,
        PCXHR_CLOCK_TYPE_AES_2 => val = PCXHR_FREQ_AES_2,
        PCXHR_CLOCK_TYPE_AES_3 => val = PCXHR_FREQ_AES_3,
        PCXHR_CLOCK_TYPE_AES_4 => val = PCXHR_FREQ_AES_4,
        _ => return -EINVAL,
    }
    *reg = val;
    *freq = realfreq;
    0
}

unsafe extern "C" fn pcxhr_sub_set_clock(mgr: *mut pcxhr_mgr, rate: c_uint, changed: *mut c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut realfreq: c_uint = 0;
    let speed: c_uint;
    let mut rmh: pcxhr_rmh = zeroed();
    let mut err = pcxhr_get_clock_reg(mgr, rate, &mut val, &mut realfreq);
    if err != 0 { return err; }
    if rate < 55000 { speed = 0; } else if rate < 100000 { speed = 1; } else { speed = 2; }
    if (*mgr).codec_speed != speed {
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
        rmh.cmd[0] |= IO_NUM_REG_MUTE_OUT;
        if DSP_EXT_CMD_SET(mgr) {
            rmh.cmd[1] = 1;
            rmh.cmd_len = 2;
        }
        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 { return err; }
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
        rmh.cmd[0] |= IO_NUM_SPEED_RATIO;
        rmh.cmd[1] = speed;
        rmh.cmd_len = 2;
        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 { return err; }
    }
    err = pcxhr_write_io_num_reg_cont(mgr, PCXHR_FREQ_REG_MASK, val, changed);
    if err != 0 { return err; }
    (*mgr).sample_rate_real = realfreq as c_int;
    (*mgr).cur_clock_type = (*mgr).use_clock_type;
    if (*mgr).codec_speed != speed {
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_READ);
        rmh.cmd[0] |= IO_NUM_REG_MUTE_OUT;
        if DSP_EXT_CMD_SET(mgr) {
            rmh.cmd[1] = 1;
            rmh.cmd_len = 2;
        }
        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 { return err; }
        (*mgr).codec_speed = speed;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_set_clock(mgr: *mut pcxhr_mgr, rate: c_uint) -> c_int {
    let mut rmh: pcxhr_rmh = zeroed();
    let mut changed: c_int = 0;
    let mut err: c_int;
    if rate == 0 { return 0; }
    if (*mgr).is_hr_stereo {
        err = hr222_sub_set_clock(mgr, rate, &mut changed);
    } else {
        err = pcxhr_sub_set_clock(mgr, rate, &mut changed);
    }
    if err != 0 { return err; }
    if changed != 0 {
        pcxhr_init_rmh(&mut rmh, CMD_MODIFY_CLOCK);
        rmh.cmd[0] |= PCXHR_MODIFY_CLOCK_S_BIT;
        if rate < PCXHR_IRQ_TIMER_FREQ { rmh.cmd[1] = PCXHR_IRQ_TIMER_PERIOD; } else { rmh.cmd[1] = PCXHR_IRQ_TIMER_PERIOD * 2; }
        rmh.cmd[2] = rate;
        rmh.cmd_len = 3;
        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 { return err; }
    }
    0
}

unsafe extern "C" fn pcxhr_sub_get_external_clock(mgr: *mut pcxhr_mgr, clock_type: c_int, sample_rate: *mut c_int) -> c_int {
    let mut rmh: pcxhr_rmh = zeroed();
    let reg: u8;
    let mut rate: c_int;
    match clock_type {
        PCXHR_CLOCK_TYPE_WORD_CLOCK => reg = REG_STATUS_WORD_CLOCK,
        PCXHR_CLOCK_TYPE_AES_SYNC => reg = REG_STATUS_AES_SYNC,
        PCXHR_CLOCK_TYPE_AES_1 => reg = REG_STATUS_AES_1,
        PCXHR_CLOCK_TYPE_AES_2 => reg = REG_STATUS_AES_2,
        PCXHR_CLOCK_TYPE_AES_3 => reg = REG_STATUS_AES_3,
        PCXHR_CLOCK_TYPE_AES_4 => reg = REG_STATUS_AES_4,
        _ => return -EINVAL,
    }
    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_READ);
    rmh.cmd_len = 2;
    rmh.cmd[0] |= IO_NUM_REG_STATUS;
    if (*mgr).last_reg_stat != reg {
        rmh.cmd[1] = reg as u32;
        let err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 { return err; }
        udelay(100);
        (*mgr).last_reg_stat = reg;
    }
    rmh.cmd[1] = REG_STATUS_CURRENT;
    let err = pcxhr_send_msg(mgr, &mut rmh);
    if err != 0 { return err; }
    rate = match rmh.stat[1] & 0x0f {
        REG_STATUS_SYNC_32000 => 32000,
        REG_STATUS_SYNC_44100 => 44100,
        REG_STATUS_SYNC_48000 => 48000,
        REG_STATUS_SYNC_64000 => 64000,
        REG_STATUS_SYNC_88200 => 88200,
        REG_STATUS_SYNC_96000 => 96000,
        REG_STATUS_SYNC_128000 => 128000,
        REG_STATUS_SYNC_176400 => 176400,
        REG_STATUS_SYNC_192000 => 192000,
        _ => 0,
    };
    *sample_rate = rate;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_get_external_clock(mgr: *mut pcxhr_mgr, clock_type: c_int, sample_rate: *mut c_int) -> c_int {
    if (*mgr).is_hr_stereo {
        hr222_get_external_clock(mgr, clock_type, sample_rate)
    } else {
        pcxhr_sub_get_external_clock(mgr, clock_type, sample_rate)
    }
}

unsafe extern "C" fn pcxhr_set_stream_state(mut chip: *mut snd_pcxhr, stream: *mut pcxhr_stream) -> c_int {
    let mut rmh: pcxhr_rmh = zeroed();
    let start: c_int;
    if (*stream).status == PCXHR_STREAM_STATUS_SCHEDULE_RUN {
        start = 1;
    } else {
        if (*stream).status != PCXHR_STREAM_STATUS_SCHEDULE_STOP {
            return -EINVAL;
        }
        start = 0;
    }
    if (*stream).substream.is_null() { return -EINVAL; }
    (*stream).timer_abs_periods = 0;
    (*stream).timer_period_frag = 0;
    (*stream).timer_buf_periods = 0;
    (*stream).timer_is_synced = 0;
    let stream_mask = if (*(*stream).pipe).is_capture != 0 { 1 } else { 1 << (*(*stream).substream).number };
    pcxhr_init_rmh(&mut rmh, if start != 0 { CMD_START_STREAM } else { CMD_STOP_STREAM });
    pcxhr_set_pipe_cmd_params(&mut rmh, (*(*stream).pipe).is_capture, (*(*stream).pipe).first_audio, 0, stream_mask);
    chip = snd_pcm_substream_chip((*stream).substream);
    let err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    (*stream).status = if start != 0 { PCXHR_STREAM_STATUS_STARTED } else { PCXHR_STREAM_STATUS_STOPPED };
    err
}

unsafe extern "C" fn pcxhr_set_format(stream: *mut pcxhr_stream) -> c_int {
    let chip = snd_pcm_substream_chip((*stream).substream);
    let mut rmh: pcxhr_rmh = zeroed();
    let mut header: c_uint;
    match (*stream).format {
        SNDRV_PCM_FORMAT_U8 => header = HEADER_FMT_BASE_LIN,
        SNDRV_PCM_FORMAT_S16_LE => header = HEADER_FMT_BASE_LIN | HEADER_FMT_16BITS | HEADER_FMT_INTEL,
        SNDRV_PCM_FORMAT_S16_BE => header = HEADER_FMT_BASE_LIN | HEADER_FMT_16BITS,
        SNDRV_PCM_FORMAT_S24_3LE => header = HEADER_FMT_BASE_LIN | HEADER_FMT_24BITS | HEADER_FMT_INTEL,
        SNDRV_PCM_FORMAT_S24_3BE => header = HEADER_FMT_BASE_LIN | HEADER_FMT_24BITS,
        SNDRV_PCM_FORMAT_FLOAT_LE => header = HEADER_FMT_BASE_FLOAT | HEADER_FMT_INTEL,
        _ => return -EINVAL,
    }
    let sample_rate = (*(*chip).mgr).sample_rate as c_int;
    if sample_rate <= 32000 && sample_rate != 0 {
        if sample_rate <= 11025 { header |= HEADER_FMT_UPTO11; } else { header |= HEADER_FMT_UPTO32; }
    }
    if (*stream).channels == 1 { header |= HEADER_FMT_MONO; }
    let is_capture = (*(*stream).pipe).is_capture;
    let stream_num = if is_capture != 0 { 0 } else { (*(*stream).substream).number };
    pcxhr_init_rmh(&mut rmh, if is_capture != 0 { CMD_FORMAT_STREAM_IN } else { CMD_FORMAT_STREAM_OUT });
    pcxhr_set_pipe_cmd_params(&mut rmh, is_capture, (*(*stream).pipe).first_audio, stream_num, 0);
    if is_capture != 0 {
        if DSP_EXT_CMD_SET((*chip).mgr) { rmh.cmd[0] |= 1 << 10; } else { rmh.cmd[0] |= 1 << 12; }
    }
    rmh.cmd[1] = 0;
    rmh.cmd_len = 2;
    if DSP_EXT_CMD_SET((*chip).mgr) {
        rmh.cmd[1] = (*stream).channels;
        if is_capture == 0 {
            rmh.cmd[2] = if (*stream).channels == 1 { 0x01 } else { 0x03 };
            rmh.cmd_len = 3;
        }
    }
    rmh.cmd[rmh.cmd_len as usize] = header >> 8;
    rmh.cmd_len += 1;
    rmh.cmd[rmh.cmd_len as usize] = (header & 0xff) << 16;
    rmh.cmd_len += 1;
    pcxhr_send_msg((*chip).mgr, &mut rmh)
}

unsafe extern "C" fn pcxhr_update_r_buffer(stream: *mut pcxhr_stream) -> c_int {
    let subs = (*stream).substream;
    let chip = snd_pcm_substream_chip(subs);
    let mut rmh: pcxhr_rmh = zeroed();
    let is_capture = ((*subs).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;
    let stream_num = if is_capture != 0 { 0 } else { (*subs).number };
    pcxhr_init_rmh(&mut rmh, CMD_UPDATE_R_BUFFERS);
    pcxhr_set_pipe_cmd_params(&mut rmh, is_capture, (*(*stream).pipe).first_audio, stream_num, 0);
    snd_BUG_ON((*(*subs).runtime).dma_bytes >= 0x200000);
    rmh.cmd[1] = ((*(*subs).runtime).dma_bytes * 8) as u32;
    rmh.cmd[2] = ((*(*subs).runtime).dma_addr >> 24) as u32;
    rmh.cmd[2] |= 1 << 19;
    rmh.cmd[3] = ((*(*subs).runtime).dma_addr as u32) & MASK_DSP_WORD;
    rmh.cmd_len = 4;
    pcxhr_send_msg((*chip).mgr, &mut rmh)
}

/* #if 0 pcxhr_pipe_sample_count() omitted by the original preprocessor condition. */

unsafe fn pcxhr_stream_scheduled_get_pipe(stream: *mut pcxhr_stream, pipe: *mut *mut pcxhr_pipe) -> c_int {
    if (*stream).status == PCXHR_STREAM_STATUS_SCHEDULE_RUN {
        *pipe = (*stream).pipe;
        return 1;
    }
    0
}

unsafe extern "C" fn pcxhr_start_linked_stream(mgr: *mut pcxhr_mgr) {
    let mut pipe: *mut pcxhr_pipe = ptr::null_mut();
    let mut capture_mask: c_int = 0;
    let mut playback_mask: c_int = 0;
    mutex_lock(&mut (*mgr).setup_mutex);
    for i in 0..(*mgr).num_cards as usize {
        let chip = (*mgr).chip[i];
        for j in 0..(*chip).nb_streams_capt as usize {
            if pcxhr_stream_scheduled_get_pipe(&mut (*chip).capture_stream[j], &mut pipe) != 0 {
                capture_mask |= 1 << (*pipe).first_audio;
            }
        }
        for j in 0..(*chip).nb_streams_play as usize {
            if pcxhr_stream_scheduled_get_pipe(&mut (*chip).playback_stream[j], &mut pipe) != 0 {
                playback_mask |= 1 << (*pipe).first_audio;
                break;
            }
        }
    }
    if capture_mask == 0 && playback_mask == 0 {
        mutex_unlock(&mut (*mgr).setup_mutex);
        return;
    }
    let mut err = pcxhr_set_pipe_state(mgr, playback_mask, capture_mask, 0);
    if err != 0 {
        mutex_unlock(&mut (*mgr).setup_mutex);
        return;
    }
    for i in 0..(*mgr).num_cards as usize {
        let chip = (*mgr).chip[i];
        for j in 0..(*chip).nb_streams_capt as usize {
            let stream = &mut (*chip).capture_stream[j] as *mut pcxhr_stream;
            if pcxhr_stream_scheduled_get_pipe(stream, &mut pipe) != 0 {
                err = pcxhr_set_format(stream);
                err = pcxhr_update_r_buffer(stream);
            }
        }
        for j in 0..(*chip).nb_streams_play as usize {
            let stream = &mut (*chip).playback_stream[j] as *mut pcxhr_stream;
            if pcxhr_stream_scheduled_get_pipe(stream, &mut pipe) != 0 {
                err = pcxhr_set_format(stream);
                err = pcxhr_update_r_buffer(stream);
            }
        }
    }
    for i in 0..(*mgr).num_cards as usize {
        let chip = (*mgr).chip[i];
        for j in 0..(*chip).nb_streams_capt as usize {
            let stream = &mut (*chip).capture_stream[j] as *mut pcxhr_stream;
            if pcxhr_stream_scheduled_get_pipe(stream, &mut pipe) != 0 {
                err = pcxhr_set_stream_state(chip, stream);
            }
        }
        for j in 0..(*chip).nb_streams_play as usize {
            let stream = &mut (*chip).playback_stream[j] as *mut pcxhr_stream;
            if pcxhr_stream_scheduled_get_pipe(stream, &mut pipe) != 0 {
                err = pcxhr_set_stream_state(chip, stream);
            }
        }
    }
    err = pcxhr_set_pipe_state(mgr, playback_mask, capture_mask, 1);
    if err != 0 {
        mutex_unlock(&mut (*mgr).setup_mutex);
        return;
    }
    mutex_lock(&mut (*mgr).lock);
    for i in 0..(*mgr).num_cards as usize {
        let chip = (*mgr).chip[i];
        for j in 0..(*chip).nb_streams_capt as usize {
            let stream = &mut (*chip).capture_stream[j];
            if stream.status == PCXHR_STREAM_STATUS_STARTED {
                stream.status = PCXHR_STREAM_STATUS_RUNNING;
            }
        }
        for j in 0..(*chip).nb_streams_play as usize {
            let stream = &mut (*chip).playback_stream[j];
            if stream.status == PCXHR_STREAM_STATUS_STARTED {
                stream.timer_period_frag = stream.timer_period_frag.wrapping_add((*mgr).granularity);
                stream.status = PCXHR_STREAM_STATUS_RUNNING;
            }
        }
    }
    mutex_unlock(&mut (*mgr).lock);
    mutex_unlock(&mut (*mgr).setup_mutex);
}

unsafe extern "C" fn pcxhr_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if snd_pcm_stream_linked(subs) != 0 {
                /* snd_pcm_group_for_each_entry is a kernel macro; the isolated translation marks only the current substream. */
                let stream = (*(*subs).runtime).private_data as *mut pcxhr_stream;
                (*stream).status = PCXHR_STREAM_STATUS_SCHEDULE_RUN;
                snd_pcm_trigger_done(subs, subs);
                pcxhr_start_linked_stream((*chip).mgr);
            } else {
                let stream = (*(*subs).runtime).private_data as *mut pcxhr_stream;
                if pcxhr_set_format(stream) != 0 { return -EINVAL; }
                if pcxhr_update_r_buffer(stream) != 0 { return -EINVAL; }
                (*stream).status = PCXHR_STREAM_STATUS_SCHEDULE_RUN;
                if pcxhr_set_stream_state(chip, stream) != 0 { return -EINVAL; }
                (*stream).status = PCXHR_STREAM_STATUS_RUNNING;
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            /* snd_pcm_group_for_each_entry is a kernel macro; the isolated translation applies the operation to the current substream. */
            let stream = (*(*subs).runtime).private_data as *mut pcxhr_stream;
            (*stream).status = PCXHR_STREAM_STATUS_SCHEDULE_STOP;
            if pcxhr_set_stream_state(chip, stream) != 0 { return -EINVAL; }
            snd_pcm_trigger_done(subs, subs);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => return -EINVAL,
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn pcxhr_hardware_timer(mgr: *mut pcxhr_mgr, start: c_int) -> c_int {
    let mut rmh: pcxhr_rmh = zeroed();
    pcxhr_init_rmh(&mut rmh, CMD_SET_TIMER_INTERRUPT);
    if start != 0 {
        (*mgr).dsp_time_last = PCXHR_DSP_TIME_INVALID;
        rmh.cmd[0] |= (*mgr).granularity;
    }
    pcxhr_send_msg(mgr, &mut rmh)
}

unsafe extern "C" fn pcxhr_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let mut err = 0;
    mutex_lock(&mut (*mgr).setup_mutex);
    if (*mgr).sample_rate != (*(*subs).runtime).rate {
        err = pcxhr_set_clock(mgr, (*(*subs).runtime).rate);
        if err == 0 {
            if (*mgr).sample_rate == 0 {
                err = pcxhr_hardware_timer(mgr, 1);
            }
            (*mgr).sample_rate = (*(*subs).runtime).rate;
        }
    }
    mutex_unlock(&mut (*mgr).setup_mutex);
    err
}

unsafe extern "C" fn pcxhr_hw_params(subs: *mut snd_pcm_substream, hw: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let stream = (*(*subs).runtime).private_data as *mut pcxhr_stream;
    mutex_lock(&mut (*mgr).setup_mutex);
    (*stream).channels = params_channels(hw);
    (*stream).format = params_format(hw);
    mutex_unlock(&mut (*mgr).setup_mutex);
    0
}

static pcxhr_caps: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_3BE | SNDRV_PCM_FMTBIT_FLOAT_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 2 * PCXHR_GRANULARITY as usize,
    period_bytes_max: 16 * 1024,
    periods_min: 2,
    periods_max: 32 * 1024 / PCXHR_GRANULARITY,
};

unsafe extern "C" fn pcxhr_open(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let runtime = (*subs).runtime;
    let stream: *mut pcxhr_stream;
    mutex_lock(&mut (*mgr).setup_mutex);
    (*runtime).hw = pcxhr_caps;
    if (*subs).stream == SNDRV_PCM_STREAM_PLAYBACK {
        stream = &mut (*chip).playback_stream[(*subs).number as usize];
    } else {
        if (*mgr).mono_capture { (*runtime).hw.channels_max = 1; } else { (*runtime).hw.channels_min = 2; }
        stream = &mut (*chip).capture_stream[(*subs).number as usize];
    }
    if (*stream).status != PCXHR_STREAM_STATUS_FREE {
        mutex_unlock(&mut (*mgr).setup_mutex);
        return -EBUSY;
    }
    if (*mgr).is_hr_stereo {
        (*runtime).hw.formats &= !SNDRV_PCM_FMTBIT_FLOAT_LE;
    }
    let mut err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        mutex_unlock(&mut (*mgr).setup_mutex);
        return err;
    }
    if (*mgr).sample_rate != 0 {
        (*runtime).hw.rate_min = (*mgr).sample_rate;
        (*runtime).hw.rate_max = (*mgr).sample_rate;
    } else if (*mgr).use_clock_type != PCXHR_CLOCK_TYPE_INTERNAL {
        let mut external_rate: c_int = 0;
        if pcxhr_get_external_clock(mgr, (*mgr).use_clock_type, &mut external_rate) != 0 || external_rate == 0 {
            mutex_unlock(&mut (*mgr).setup_mutex);
            return -EBUSY;
        }
        (*runtime).hw.rate_min = external_rate as c_uint;
        (*runtime).hw.rate_max = external_rate as c_uint;
    }
    (*stream).status = PCXHR_STREAM_STATUS_OPEN;
    (*stream).substream = subs;
    (*stream).channels = 0;
    (*runtime).private_data = stream as *mut c_void;
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 32);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 32);
    snd_pcm_set_sync(subs);
    (*mgr).ref_count_rate += 1;
    mutex_unlock(&mut (*mgr).setup_mutex);
    0
}

unsafe extern "C" fn pcxhr_close(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    let mgr = (*chip).mgr;
    let stream = (*(*subs).runtime).private_data as *mut pcxhr_stream;
    mutex_lock(&mut (*mgr).setup_mutex);
    (*mgr).ref_count_rate -= 1;
    if (*mgr).ref_count_rate == 0 {
        (*mgr).sample_rate = 0;
        pcxhr_hardware_timer(mgr, 0);
    }
    (*stream).status = PCXHR_STREAM_STATUS_FREE;
    (*stream).substream = ptr::null_mut();
    mutex_unlock(&mut (*mgr).setup_mutex);
    0
}

unsafe extern "C" fn pcxhr_stream_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(subs);
    let runtime = (*subs).runtime;
    let stream = (*runtime).private_data as *mut pcxhr_stream;
    mutex_lock(&mut (*(*chip).mgr).lock);
    let timer_period_frag = (*stream).timer_period_frag;
    let timer_buf_periods = (*stream).timer_buf_periods;
    mutex_unlock(&mut (*(*chip).mgr).lock);
    (timer_buf_periods as snd_pcm_uframes_t).wrapping_mul((*runtime).period_size).wrapping_add(timer_period_frag as snd_pcm_uframes_t)
}

static pcxhr_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pcxhr_open),
    close: Some(pcxhr_close),
    prepare: Some(pcxhr_prepare),
    hw_params: Some(pcxhr_hw_params),
    trigger: Some(pcxhr_trigger),
    pointer: Some(pcxhr_stream_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn pcxhr_create_pcm(chip: *mut snd_pcxhr) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut name = [0 as c_char; 32];
    snprintf(name.as_mut_ptr(), name.len(), cstr!("pcxhr %d"), (*chip).chip_idx);
    let err = snd_pcm_new((*chip).card, name.as_ptr(), 0, (*chip).nb_streams_play, (*chip).nb_streams_capt, &mut pcm);
    if err < 0 { return err; }
    (*pcm).private_data = chip as *mut c_void;
    if (*chip).nb_streams_play != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &pcxhr_ops);
    }
    if (*chip).nb_streams_capt != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &pcxhr_ops);
    }
    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), name.as_ptr());
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*(*chip).mgr).pci).dev, 32 * 1024, 32 * 1024);
    (*chip).pcm = pcm;
    0
}

unsafe extern "C" fn pcxhr_chip_free(chip: *mut snd_pcxhr) -> c_int {
    kfree(chip as *mut c_void);
    0
}

unsafe extern "C" fn pcxhr_chip_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut snd_pcxhr;
    pcxhr_chip_free(chip)
}

unsafe extern "C" fn pcxhr_create(mgr: *mut pcxhr_mgr, card: *mut snd_card, idx: c_int) -> c_int {
    static ops: snd_device_ops = snd_device_ops { dev_free: Some(pcxhr_chip_dev_free) };
    let chip = kzalloc(size_of::<snd_pcxhr>(), GFP_KERNEL) as *mut snd_pcxhr;
    if chip.is_null() { return -ENOMEM; }
    (*chip).card = card;
    (*chip).chip_idx = idx;
    (*chip).mgr = mgr;
    (*card).sync_irq = (*mgr).irq;
    if idx < (*mgr).playback_chips { (*chip).nb_streams_play = PCXHR_PLAYBACK_STREAMS; }
    if idx < (*mgr).capture_chips {
        if (*mgr).mono_capture { (*chip).nb_streams_capt = 2; } else { (*chip).nb_streams_capt = 1; }
    }
    let err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
    if err < 0 {
        pcxhr_chip_free(chip);
        return err;
    }
    (*mgr).chip[idx as usize] = chip;
    0
}

unsafe extern "C" fn pcxhr_proc_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_pcxhr;
    let mgr = (*chip).mgr;
    snd_iprintf(buffer, cstr!("\n%s\n"), (*mgr).name.as_ptr());
    if (*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX) != 0 {
        let mut rmh: pcxhr_rmh = zeroed();
        let ver_maj: i16 = (((*mgr).dsp_version >> 16) & 0xff) as i16;
        let ver_min: i16 = (((*mgr).dsp_version >> 8) & 0xff) as i16;
        let ver_build: i16 = ((*mgr).dsp_version & 0xff) as i16;
        snd_iprintf(buffer, cstr!("module version %s\n"), cstr!("PCXHR_DRIVER_VERSION_STRING"));
        snd_iprintf(buffer, cstr!("dsp version %d.%d.%d\n"), ver_maj as c_int, ver_min as c_int, ver_build as c_int);
        if (*mgr).board_has_analog { snd_iprintf(buffer, cstr!("analog io available\n")); } else { snd_iprintf(buffer, cstr!("digital only board\n")); }
        pcxhr_init_rmh(&mut rmh, CMD_GET_DSP_RESOURCES);
        if pcxhr_send_msg(mgr, &mut rmh) == 0 {
            let mut cur = rmh.stat[0] as c_int;
            let mut refv = rmh.stat[1] as c_int;
            if refv > 0 {
                if (*mgr).sample_rate_real != 0 && (*mgr).sample_rate_real != 48000 {
                    refv = (refv * 48000) / (*mgr).sample_rate_real;
                    if (*mgr).sample_rate_real >= PCXHR_IRQ_TIMER_FREQ as c_int { refv *= 2; }
                }
                cur = 100 - (100 * cur) / refv;
                snd_iprintf(buffer, cstr!("cpu load    %d%%\n"), cur);
                snd_iprintf(buffer, cstr!("buffer pool %d/%d\n"), rmh.stat[2] as c_int, rmh.stat[3] as c_int);
            }
        }
        snd_iprintf(buffer, cstr!("dma granularity : %d\n"), (*mgr).granularity);
        snd_iprintf(buffer, cstr!("dsp time errors : %d\n"), (*mgr).dsp_time_err);
        snd_iprintf(buffer, cstr!("dsp async pipe xrun errors : %d\n"), (*mgr).async_err_pipe_xrun);
        snd_iprintf(buffer, cstr!("dsp async stream xrun errors : %d\n"), (*mgr).async_err_stream_xrun);
        snd_iprintf(buffer, cstr!("dsp async last other error : %x\n"), (*mgr).async_err_other_last);
        rmh.cmd[0] = 0x4200 + PCXHR_SIZE_MAX_STATUS as u32;
        rmh.cmd_len = 1;
        rmh.stat_len = PCXHR_SIZE_MAX_STATUS;
        rmh.dsp_stat = 0;
        rmh.cmd_idx = CMD_LAST_INDEX;
        if pcxhr_send_msg(mgr, &mut rmh) == 0 {
            if rmh.stat_len > 8 { rmh.stat_len = 8; }
            for i in 0..rmh.stat_len as usize {
                snd_iprintf(buffer, cstr!("debug[%02d] = %06x\n"), i as c_int, rmh.stat[i]);
            }
        }
    } else {
        snd_iprintf(buffer, cstr!("no firmware loaded\n"));
    }
    snd_iprintf(buffer, cstr!("\n"));
}

unsafe extern "C" fn pcxhr_proc_sync(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_pcxhr;
    let mgr = (*chip).mgr;
    static textsHR22: [*const c_char; 3] = [cstr!("Internal"), cstr!("AES Sync"), cstr!("AES 1")];
    static textsPCXHR: [*const c_char; 7] = [cstr!("Internal"), cstr!("Word"), cstr!("AES Sync"), cstr!("AES 1"), cstr!("AES 2"), cstr!("AES 3"), cstr!("AES 4")];
    let (texts, max_clock) = if (*mgr).is_hr_stereo { (textsHR22.as_ptr(), HR22_CLOCK_TYPE_MAX) } else { (textsPCXHR.as_ptr(), PCXHR_CLOCK_TYPE_MAX) };
    snd_iprintf(buffer, cstr!("\n%s\n"), (*mgr).name.as_ptr());
    snd_iprintf(buffer, cstr!("Current Sample Clock\t: %s\n"), *texts.add((*mgr).cur_clock_type as usize));
    snd_iprintf(buffer, cstr!("Current Sample Rate\t= %d\n"), (*mgr).sample_rate_real);
    if (*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX) != 0 {
        for i in 1..=max_clock {
            let mut sample_rate: c_int = 0;
            let err = pcxhr_get_external_clock(mgr, i, &mut sample_rate);
            if err != 0 { break; }
            snd_iprintf(buffer, cstr!("%s Clock\t\t= %d\n"), *texts.add(i as usize), sample_rate);
        }
    } else {
        snd_iprintf(buffer, cstr!("no firmware loaded\n"));
    }
    snd_iprintf(buffer, cstr!("\n"));
}

unsafe extern "C" fn pcxhr_proc_gpio_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_pcxhr;
    let mgr = (*chip).mgr;
    if (*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX) != 0 {
        let mut value: c_int = 0;
        hr222_read_gpio(mgr, 1, &mut value);
        snd_iprintf(buffer, cstr!("GPI: 0x%x\n"), value);
        hr222_read_gpio(mgr, 0, &mut value);
        snd_iprintf(buffer, cstr!("GPO: 0x%x\n"), value);
    } else {
        snd_iprintf(buffer, cstr!("no firmware loaded\n"));
    }
    snd_iprintf(buffer, cstr!("\n"));
}

unsafe extern "C" fn pcxhr_proc_gpo_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_pcxhr;
    let mgr = (*chip).mgr;
    let mut line = [0 as c_char; 64];
    let mut value: c_int = 0;
    if (*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX) == 0 { return; }
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(line.as_ptr(), cstr!("GPO: 0x%x"), &mut value) != 1 { continue; }
        hr222_write_gpo(mgr, value);
    }
}

unsafe extern "C" fn pcxhr_proc_ltc(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_pcxhr;
    let mgr = (*chip).mgr;
    let mut rmh: pcxhr_rmh = zeroed();
    if (*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX) == 0 {
        snd_iprintf(buffer, cstr!("no firmware loaded\n"));
        return;
    }
    if (*mgr).capture_ltc == 0 {
        pcxhr_init_rmh(&mut rmh, CMD_MANAGE_SIGNAL);
        rmh.cmd[0] |= MANAGE_SIGNAL_TIME_CODE;
        let err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 {
            snd_iprintf(buffer, cstr!("ltc not activated (%d)\n"), err);
            return;
        }
        if (*mgr).is_hr_stereo {
            hr222_manage_timecode(mgr, 1);
        } else {
            pcxhr_write_io_num_reg_cont(mgr, REG_CONT_VALSMPTE, REG_CONT_VALSMPTE, ptr::null_mut());
        }
        (*mgr).capture_ltc = 1;
    }
    pcxhr_init_rmh(&mut rmh, CMD_GET_TIME_CODE);
    let err = pcxhr_send_msg(mgr, &mut rmh);
    if err != 0 {
        snd_iprintf(buffer, cstr!("ltc read error (err=%d)\n"), err);
        return;
    }
    let ltcHrs = 10 * ((rmh.stat[0] >> 8) & 0x3) + (rmh.stat[0] & 0xf);
    let ltcMin = 10 * ((rmh.stat[1] >> 16) & 0x7) + ((rmh.stat[1] >> 8) & 0xf);
    let ltcSec = 10 * (rmh.stat[1] & 0x7) + ((rmh.stat[2] >> 16) & 0xf);
    let ltcFrm = 10 * ((rmh.stat[2] >> 8) & 0x3) + (rmh.stat[2] & 0xf);
    snd_iprintf(buffer, cstr!("timecode: %02u:%02u:%02u-%02u\n"), ltcHrs, ltcMin, ltcSec, ltcFrm);
    snd_iprintf(buffer, cstr!("raw: 0x%04x%06x%06x\n"), rmh.stat[0] & 0x00ffff, rmh.stat[1] & 0xffffff, rmh.stat[2] & 0xffffff);
    if rmh.stat[0] & TIME_CODE_VALID_MASK == 0 {
        snd_iprintf(buffer, cstr!("warning: linear timecode not valid\n"));
    }
}

unsafe extern "C" fn pcxhr_proc_init(chip: *mut snd_pcxhr) {
    snd_card_ro_proc_new((*chip).card, cstr!("info"), chip as *mut c_void, pcxhr_proc_info);
    snd_card_ro_proc_new((*chip).card, cstr!("sync"), chip as *mut c_void, pcxhr_proc_sync);
    if (*(*chip).mgr).is_hr_stereo {
        snd_card_rw_proc_new((*chip).card, cstr!("gpio"), chip as *mut c_void, pcxhr_proc_gpio_read, pcxhr_proc_gpo_write);
    }
    snd_card_ro_proc_new((*chip).card, cstr!("ltc"), chip as *mut c_void, pcxhr_proc_ltc);
}

unsafe extern "C" fn pcxhr_free(mgr: *mut pcxhr_mgr) -> c_int {
    for i in 0..(*mgr).num_cards as usize {
        if !(*mgr).chip[i].is_null() {
            snd_card_free((*(*mgr).chip[i]).card);
        }
    }
    if (*mgr).dsp_loaded != 0 {
        pcxhr_reset_board(mgr);
    }
    if (*mgr).irq >= 0 {
        free_irq((*mgr).irq, mgr as *mut c_void);
    }
    pci_release_regions((*mgr).pci);
    if !(*mgr).hostport.area.is_null() {
        snd_dma_free_pages(&mut (*mgr).hostport);
        (*mgr).hostport.area = ptr::null_mut();
    }
    kfree((*mgr).prmh as *mut c_void);
    pci_disable_device((*mgr).pci);
    kfree(mgr as *mut c_void);
    0
}

unsafe extern "C" fn pcxhr_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut err: c_int;
    let mut size: usize;
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = pci_enable_device(pci);
    if err < 0 { return err; }
    pci_set_master(pci);
    if dma_set_mask(&mut (*pci).dev, DMA_BIT_MASK(32)) < 0 {
        pci_disable_device(pci);
        return -ENXIO;
    }
    let mgr = kzalloc(size_of::<pcxhr_mgr>(), GFP_KERNEL) as *mut pcxhr_mgr;
    if mgr.is_null() {
        pci_disable_device(pci);
        return -ENOMEM;
    }
    if snd_BUG_ON((*pci_id).driver_data as usize >= PCI_ID_LAST) {
        kfree(mgr as *mut c_void);
        pci_disable_device(pci);
        return -ENODEV;
    }
    let bp = &pcxhr_board_params[(*pci_id).driver_data as usize];
    let card_name = bp.board_name;
    (*mgr).playback_chips = bp.playback_chips as c_int;
    (*mgr).capture_chips = bp.capture_chips as c_int;
    (*mgr).fw_file_set = bp.fw_file_set as c_int;
    (*mgr).firmware_num = bp.firmware_num as c_int;
    (*mgr).mono_capture = mono[dev as usize];
    (*mgr).is_hr_stereo = (*mgr).playback_chips == 1;
    (*mgr).board_has_aes1 = PCXHR_BOARD_HAS_AES1(mgr);
    (*mgr).board_aes_in_192k = !PCXHR_BOARD_AESIN_NO_192K(mgr);
    if (*mgr).is_hr_stereo { (*mgr).granularity = PCXHR_GRANULARITY_HR22; } else { (*mgr).granularity = PCXHR_GRANULARITY; }
    err = pci_request_regions(pci, card_name);
    if err < 0 {
        kfree(mgr as *mut c_void);
        pci_disable_device(pci);
        return err;
    }
    for i in 0..3 {
        (*mgr).port[i] = pci_resource_start(pci, i as c_int);
    }
    (*mgr).pci = pci;
    (*mgr).irq = -1;
    mutex_init(&mut (*mgr).lock);
    mutex_init(&mut (*mgr).msg_lock);
    mutex_init(&mut (*mgr).setup_mutex);
    if request_threaded_irq((*pci).irq, pcxhr_interrupt, pcxhr_threaded_irq, IRQF_SHARED, KBUILD_MODNAME, mgr as *mut c_void) != 0 {
        pcxhr_free(mgr);
        return -EBUSY;
    }
    (*mgr).irq = (*pci).irq;
    snprintf((*mgr).name.as_mut_ptr(), (*mgr).name.len(), cstr!("Digigram at 0x%lx & 0x%lx, 0x%lx irq %i"), (*mgr).port[0], (*mgr).port[1], (*mgr).port[2], (*mgr).irq);
    (*mgr).prmh = kmalloc(size_of::<pcxhr_rmh>() + size_of::<u32>() * (PCXHR_SIZE_MAX_LONG_STATUS as usize - PCXHR_SIZE_MAX_STATUS as usize), GFP_KERNEL) as *mut pcxhr_rmh;
    if (*mgr).prmh.is_null() {
        pcxhr_free(mgr);
        return -ENOMEM;
    }
    for i in 0..PCXHR_MAX_CARDS {
        let mut card: *mut snd_card = ptr::null_mut();
        let mut tmpid = [0 as c_char; 16];
        let idx: c_int;
        if i as c_int >= max((*mgr).playback_chips, (*mgr).capture_chips) { break; }
        (*mgr).num_cards += 1;
        if index[dev as usize] < 0 { idx = index[dev as usize]; } else { idx = index[dev as usize] + i as c_int; }
        snprintf(tmpid.as_mut_ptr(), tmpid.len(), cstr!("%s-%d"), if !id[dev as usize].is_null() { id[dev as usize] as *const c_char } else { card_name }, i as c_int);
        err = snd_card_new(&mut (*pci).dev, idx, tmpid.as_ptr(), THIS_MODULE, 0, &mut card);
        if err < 0 {
            pcxhr_free(mgr);
            return err;
        }
        strscpy((*card).driver.as_mut_ptr(), DRIVER_NAME.as_ptr() as *const c_char);
        snprintf((*card).shortname.as_mut_ptr(), (*card).shortname.len(), cstr!("Digigram [PCM #%d]"), i as c_int);
        snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), cstr!("%s [PCM #%d]"), (*mgr).name.as_ptr(), i as c_int);
        err = pcxhr_create(mgr, card, i as c_int);
        if err < 0 {
            snd_card_free(card);
            pcxhr_free(mgr);
            return err;
        }
        if i == 0 {
            pcxhr_proc_init((*mgr).chip[i as usize]);
        }
        err = snd_card_register(card);
        if err < 0 {
            pcxhr_free(mgr);
            return err;
        }
    }
    size = PAGE_ALIGN(size_of::<pcxhr_hostport>());
    if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, size, &mut (*mgr).hostport) < 0 {
        pcxhr_free(mgr);
        return -ENOMEM;
    }
    memset((*mgr).hostport.area, 0, size);
    err = pcxhr_setup_firmware(mgr);
    if err < 0 {
        pcxhr_free(mgr);
        return err;
    }
    pci_set_drvdata(pci, mgr as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn pcxhr_remove(pci: *mut pci_dev) {
    pcxhr_free(pci_get_drvdata(pci));
}

extern "C" {
    fn snd_card_register(card: *mut snd_card) -> c_int;
}

static mut pcxhr_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: pcxhr_ids.as_ptr(),
    probe: Some(pcxhr_probe),
    remove: Some(pcxhr_remove),
};

/* module_pci_driver(pcxhr_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
