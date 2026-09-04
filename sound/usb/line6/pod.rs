// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Dependencies: linux/slab.h, linux/wait.h, linux/interrupt.h, linux/module.h, linux/usb.h
// Dependencies: sound/core.h, sound/control.h
// Dependencies: capture.h, driver.h, playback.h

use core::ffi::{c_char, c_int, c_uint};
use core::mem;
use core::ptr;

// Locate name in binary program dump
const POD_NAME_OFFSET: usize = 0;
const POD_NAME_LENGTH: usize = 16;

// Other constants
const POD_CONTROL_SIZE: usize = 0x80;
const POD_BUFSIZE_DUMPREQ: usize = 7;
const POD_STARTUP_DELAY: u32 = 1000;

// Stages of POD startup procedure
const POD_STARTUP_VERSIONREQ: i32 = 0;
const POD_STARTUP_SETUP: i32 = 1;
const POD_STARTUP_DONE: i32 = 2;

const LINE6_BASSPODXT: i32 = 0;
const LINE6_BASSPODXTLIVE: i32 = 1;
const LINE6_BASSPODXTPRO: i32 = 2;
const LINE6_POCKETPOD: i32 = 3;
const LINE6_PODXT: i32 = 4;
const LINE6_PODXTLIVE_POD: i32 = 5;
const LINE6_PODXTPRO: i32 = 6;

#[repr(C)]
pub struct usb_line6_pod {
    pub line6: usb_line6,
    pub monitor_level: c_int,
    pub startup_progress: c_int,
    pub serial_number: u32,
    pub firmware_version: c_int,
    pub device_id: c_int,
}

// container_of macro implementation for line6_to_pod
#[inline]
unsafe fn line6_to_pod(x: *mut usb_line6) -> *mut usb_line6_pod {
    let pod_ptr = (x as *mut u8).offset(-(mem::offset_of!(usb_line6_pod, line6) as isize));
    pod_ptr as *mut usb_line6_pod
}

const POD_SYSEX_CODE: i32 = 3;

const POD_SYSEX_SAVE: i32 = 0x24;
const POD_SYSEX_SYSTEM: i32 = 0x56;
const POD_SYSEX_SYSTEMREQ: i32 = 0x57;
const POD_SYSEX_STORE: i32 = 0x71;
const POD_SYSEX_FINISH: i32 = 0x72;
const POD_SYSEX_DUMPMEM: i32 = 0x73;
const POD_SYSEX_DUMP: i32 = 0x74;
const POD_SYSEX_DUMPREQ: i32 = 0x75;

const POD_MONITOR_LEVEL: i32 = 0x04;
const POD_SYSTEM_INVALID: i32 = 0x10000;

const POD_DUMP_MEMORY: i32 = 2;

const POD_BUSY_READ: i32 = 0;
const POD_BUSY_WRITE: i32 = 1;
const POD_CHANNEL_DIRTY: i32 = 2;
const POD_SAVE_PRESSED: i32 = 3;
const POD_BUSY_MIDISEND: i32 = 4;

// External type declarations
#[repr(C)]
pub struct usb_line6 {
    // opaque type from driver.h
}

#[repr(C)]
pub struct snd_ratden {
    pub num_min: i32,
    pub num_max: i32,
    pub num_step: i32,
    pub den: i32,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

#[repr(C)]
pub struct line6_pcm_properties {
    pub playback_hw: snd_pcm_hw_params,
    pub capture_hw: snd_pcm_hw_params,
    pub rates: snd_ratden_reference,
    pub bytes_per_channel: i32,
}

#[repr(C)]
pub struct snd_ratden_reference {
    pub nrats: i32,
    pub rats: *const snd_ratden,
}

#[repr(C)]
pub struct snd_kcontrol {
    // opaque type
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub r#type: u32,
    pub count: u32,
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub min: i64,
    pub max: i64,
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: u32,
    pub name: *const c_char,
    pub index: u32,
    pub access: u32,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct usb_device_id {
    pub idVendor: u16,
    pub idProduct: u16,
    pub driver_info: usize,
}

#[repr(C)]
pub struct line6_properties {
    pub id: *const c_char,
    pub name: *const c_char,
    pub capabilities: u32,
    pub altsetting: u32,
    pub ep_ctrl_r: u8,
    pub ep_ctrl_w: u8,
    pub ep_audio_r: u8,
    pub ep_audio_w: u8,
}

#[repr(C)]
pub struct usb_interface;

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct device_attribute;

#[repr(C)]
pub struct attribute;

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *const *mut attribute,
}

#[repr(C)]
pub struct snd_line6_pcm;

#[repr(C)]
pub struct usb_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut usb_interface, *const usb_device_id) -> c_int>,
    pub disconnect: Option<unsafe extern "C" fn(*mut usb_interface)>,
    pub suspend: Option<unsafe extern "C" fn(*mut usb_interface, *const c_char) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut usb_interface) -> c_int>,
    pub reset_resume: Option<unsafe extern "C" fn(*mut usb_interface) -> c_int>,
    pub id_table: *const usb_device_id,
}

// External function declarations
extern "C" {
    fn line6_alloc_sysex_buffer(
        line6: *mut usb_line6,
        sysex_code: i32,
        code: i32,
        size: i32,
    ) -> *mut c_char;

    fn line6_send_sysex_message(line6: *mut usb_line6, sysex: *mut c_char, size: i32);

    fn kfree(ptr: *mut c_void);

    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn line6_version_request_async(line6: *mut usb_line6);

    fn line6_read_serial_number(line6: *mut usb_line6, serial_number: *mut u32);

    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn schedule_delayed_work(work: *mut c_void, delay: u32);

    fn snd_card_add_dev_attr(card: *mut snd_card, group: *const attribute_group) -> c_int;

    fn line6_init_pcm(line6: *mut usb_line6, properties: *mut line6_pcm_properties) -> c_int;

    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;

    fn snd_ctl_new1(
        ncontrol: *const snd_kcontrol_new,
        private_data: *mut c_void,
    ) -> *mut snd_kcontrol;

    fn line6_probe(
        interface: *mut usb_interface,
        id: *const usb_device_id,
        name: *const c_char,
        properties: *const line6_properties,
        init: Option<unsafe extern "C" fn(*mut usb_line6, *const usb_device_id) -> c_int>,
        sizeof_pod: usize,
    ) -> c_int;

    fn line6_disconnect(interface: *mut usb_interface);

    fn line6_suspend(interface: *mut usb_interface) -> c_int;

    fn line6_resume(interface: *mut usb_interface) -> c_int;

    fn dev_to_snd_card(dev: *mut device) -> *mut snd_card;

    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn msecs_to_jiffies(msecs: u32) -> u32;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_line6_pcm;

    static LINE6_SYSEX_BEGIN: u8;
    static LINE6_CHANNEL_DEVICE: u8;
    static LINE6_CHANNEL_UNKNOWN: u8;
    static line6_midi_id: [u8; 0];
    static LINE6_CAP_CONTROL: u32;
    static LINE6_CAP_CONTROL_MIDI: u32;
    static LINE6_CAP_PCM: u32;
    static LINE6_CAP_HWMON: u32;
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_SYNC_START: u32;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_RATE_KNOT: u32;
    static SNDRV_CTL_ELEM_IFACE_MIXER: u32;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: u32;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: u32;
    static KBUILD_MODNAME: *const c_char;
}

static POD_RATDEN: snd_ratden = snd_ratden {
    num_min: 78125,
    num_max: 78125,
    num_step: 1,
    den: 2,
};

static mut POD_PCM_PROPERTIES: line6_pcm_properties = line6_pcm_properties {
    playback_hw: snd_pcm_hw_params {
        info: 0,
        formats: 0,
        rates: 0,
        rate_min: 39062,
        rate_max: 39063,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: 60000,
        period_bytes_min: 64,
        period_bytes_max: 8192,
        periods_min: 1,
        periods_max: 1024,
    },
    capture_hw: snd_pcm_hw_params {
        info: 0,
        formats: 0,
        rates: 0,
        rate_min: 39062,
        rate_max: 39063,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: 60000,
        period_bytes_min: 64,
        period_bytes_max: 8192,
        periods_min: 1,
        periods_max: 1024,
    },
    rates: snd_ratden_reference {
        nrats: 1,
        rats: ptr::addr_of!(POD_RATDEN),
    },
    bytes_per_channel: 3,
};

static POD_VERSION_HEADER: [u8; 5] = [0xf0, 0x7e, 0x7f, 0x06, 0x02];

// SYSEX_DATA_OFS should come from driver.h, using as external reference
extern "C" {
    static SYSEX_DATA_OFS: usize;
}

unsafe fn pod_alloc_sysex_buffer(pod: *mut usb_line6_pod, code: i32, size: i32) -> *mut c_char {
    line6_alloc_sysex_buffer(&mut (*pod).line6, POD_SYSEX_CODE, code, size)
}

// Process a completely received message.
unsafe fn line6_pod_process_message(line6: *mut usb_line6) {
    let pod = line6_to_pod(line6);
    let buf = (*line6).buffer_message as *const u8;

    if memcmp(
        buf as *const c_void,
        POD_VERSION_HEADER.as_ptr() as *const c_void,
        POD_VERSION_HEADER.len(),
    ) == 0
    {
        (*pod).firmware_version = ((*buf.add(13) as i32) * 100)
            + ((*buf.add(14) as i32) * 10)
            + (*buf.add(15) as i32);
        (*pod).device_id = ((*buf.add(8) as i32) << 16)
            | ((*buf.add(9) as i32) << 8)
            | (*buf.add(10) as i32);
        if (*pod).startup_progress == POD_STARTUP_VERSIONREQ {
            (*pod).startup_progress = POD_STARTUP_SETUP;
            schedule_delayed_work(&(*line6).startup_work as *const _ as *mut c_void, 0);
        }
        return;
    }

    if (*buf) != (LINE6_SYSEX_BEGIN | LINE6_CHANNEL_DEVICE)
        && (*buf) != (LINE6_SYSEX_BEGIN | LINE6_CHANNEL_UNKNOWN)
    {
        return;
    }
    if memcmp(
        buf.add(1) as *const c_void,
        line6_midi_id.as_ptr() as *const c_void,
        line6_midi_id.len(),
    ) != 0
    {
        return;
    }

    if (*buf.add(5)) as i32 == POD_SYSEX_SYSTEM && (*buf.add(6)) as i32 == POD_MONITOR_LEVEL {
        let value = ((*buf.add(7) as i32) << 12)
            | ((*buf.add(8) as i32) << 8)
            | ((*buf.add(9) as i32) << 4)
            | (*buf.add(10) as i32);
        (*pod).monitor_level = value;
    }
}

// Send system parameter (from integer).
unsafe fn pod_set_system_param_int(pod: *mut usb_line6_pod, value: i32, code: i32) -> i32 {
    let size: i32 = 5;

    let sysex = pod_alloc_sysex_buffer(pod, POD_SYSEX_SYSTEM, size);
    if sysex.is_null() {
        return -12; // -ENOMEM
    }
    *sysex.add(SYSEX_DATA_OFS) = (code & 0xff) as c_char;
    *sysex.add(SYSEX_DATA_OFS + 1) = (((value >> 12) & 0x0f) as u8) as c_char;
    *sysex.add(SYSEX_DATA_OFS + 2) = (((value >> 8) & 0x0f) as u8) as c_char;
    *sysex.add(SYSEX_DATA_OFS + 3) = (((value >> 4) & 0x0f) as u8) as c_char;
    *sysex.add(SYSEX_DATA_OFS + 4) = ((value) & 0x0f) as c_char;
    line6_send_sysex_message(&mut (*pod).line6, sysex, size);
    kfree(sysex as *mut c_void);
    0
}

// "read" request on "serial_number" special file.
unsafe extern "C" fn serial_number_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let card = dev_to_snd_card(dev);
    let pod = (*card).private_data as *mut usb_line6_pod;

    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, (*pod).serial_number) as isize
}

// "read" request on "firmware_version" special file.
unsafe extern "C" fn firmware_version_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let card = dev_to_snd_card(dev);
    let pod = (*card).private_data as *mut usb_line6_pod;

    sysfs_emit(
        buf,
        b"%d.%02d\n\0".as_ptr() as *const c_char,
        (*pod).firmware_version / 100,
        (*pod).firmware_version % 100,
    ) as isize
}

// "read" request on "device_id" special file.
unsafe extern "C" fn device_id_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let card = dev_to_snd_card(dev);
    let pod = (*card).private_data as *mut usb_line6_pod;

    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, (*pod).device_id) as isize
}

// POD startup procedure.
// This is a sequence of functions with special requirements (e.g., must
// not run immediately after initialization, must not run in interrupt
// context). After the last one has finished, the device is ready to use.
unsafe fn pod_startup(line6: *mut usb_line6) {
    let pod = line6_to_pod(line6);

    match (*pod).startup_progress {
        POD_STARTUP_VERSIONREQ => {
            line6_version_request_async(line6);
        }
        POD_STARTUP_SETUP => {
            line6_read_serial_number(line6, &mut (*pod).serial_number);
            if snd_card_register((*line6).card) != 0 {
                dev_err(
                    (*line6).ifcdev,
                    b"Failed to register POD card.\n\0".as_ptr() as *const c_char,
                );
            }
            (*pod).startup_progress = POD_STARTUP_DONE;
        }
        _ => {}
    }
}

// POD special files attribute definitions (using constants instead of macros)
// DEVICE_ATTR_RO(device_id) -> creates dev_attr_device_id with read-only access
// DEVICE_ATTR_RO(firmware_version) -> creates dev_attr_firmware_version
// DEVICE_ATTR_RO(serial_number) -> creates dev_attr_serial_number

#[repr(C)]
pub struct device_attr_device_id;
#[repr(C)]
pub struct device_attr_firmware_version;
#[repr(C)]
pub struct device_attr_serial_number;

static POD_DEV_ATTRS: [*mut attribute; 4] = [
    &device_attr_device_id as *const _ as *mut attribute,
    &device_attr_firmware_version as *const _ as *mut attribute,
    &device_attr_serial_number as *const _ as *mut attribute,
    ptr::null_mut(),
];

static POD_DEV_ATTR_GROUP: attribute_group = attribute_group {
    name: b"pod\0".as_ptr() as *const c_char,
    attrs: POD_DEV_ATTRS.as_ptr() as *const *mut attribute,
};

// control info callback
unsafe extern "C" fn snd_pod_control_monitor_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).r#type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 65535;
    0
}

// control get callback
unsafe extern "C" fn snd_pod_control_monitor_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let pod = line6_to_pod(&mut (*line6pcm).line6);

    (*ucontrol).value.integer.value[0] = (*pod).monitor_level as i64;
    0
}

// control put callback
unsafe extern "C" fn snd_pod_control_monitor_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let pod = line6_to_pod(&mut (*line6pcm).line6);

    if (*ucontrol).value.integer.value[0] == (*pod).monitor_level as i64 {
        return 0;
    }

    (*pod).monitor_level = (*ucontrol).value.integer.value[0] as c_int;
    pod_set_system_param_int(pod, (*ucontrol).value.integer.value[0] as c_int, POD_MONITOR_LEVEL);
    1
}

// control definition
static POD_CONTROL_MONITOR: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, // SNDRV_CTL_ELEM_IFACE_MIXER
    name: b"Monitor Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: 0, // SNDRV_CTL_ELEM_ACCESS_READWRITE
    info: Some(snd_pod_control_monitor_info),
    get: Some(snd_pod_control_monitor_get),
    put: Some(snd_pod_control_monitor_put),
};

// Try to init POD device.
unsafe extern "C" fn pod_init(line6: *mut usb_line6, _id: *const usb_device_id) -> c_int {
    let mut err: c_int;
    let pod = line6_to_pod(line6);

    (*line6).process_message = Some(line6_pod_process_message);
    (*line6).startup = Some(pod_startup);

    err = snd_card_add_dev_attr((*line6).card, &POD_DEV_ATTR_GROUP);
    if err < 0 {
        return err;
    }

    err = line6_init_pcm(line6, &mut POD_PCM_PROPERTIES);
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*line6).card,
        snd_ctl_new1(&POD_CONTROL_MONITOR, (*line6).line6pcm as *mut c_void),
    );
    if err < 0 {
        return err;
    }

    if ((*(*pod).line6.properties).capabilities & LINE6_CAP_CONTROL) != 0 {
        (*pod).monitor_level = POD_SYSTEM_INVALID;

        schedule_delayed_work(
            &(*line6).startup_work as *const _ as *mut c_void,
            msecs_to_jiffies(POD_STARTUP_DELAY),
        );
    }

    0
}

// Macros for device table
// #define LINE6_DEVICE(prod) USB_DEVICE(0x0e41, prod)
// #define LINE6_IF_NUM(prod, n) USB_DEVICE_INTERFACE_NUMBER(0x0e41, prod, n)

// table of devices that work with this driver
static POD_ID_TABLE: [usb_device_id; 8] = [
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x4250,
        driver_info: LINE6_BASSPODXT as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x4642,
        driver_info: LINE6_BASSPODXTLIVE as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x4252,
        driver_info: LINE6_BASSPODXTPRO as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x5051,
        driver_info: LINE6_POCKETPOD as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x5044,
        driver_info: LINE6_PODXT as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x4650,
        driver_info: LINE6_PODXTLIVE_POD as usize,
    },
    usb_device_id {
        idVendor: 0x0e41,
        idProduct: 0x5050,
        driver_info: LINE6_PODXTPRO as usize,
    },
    usb_device_id {
        idVendor: 0,
        idProduct: 0,
        driver_info: 0,
    },
];

static POD_PROPERTIES_TABLE: [line6_properties; 7] = [
    line6_properties {
        id: b"BassPODxt\0".as_ptr() as *const c_char,
        name: b"BassPODxt\0".as_ptr() as *const c_char,
        capabilities: 0, // LINE6_CAP_CONTROL | LINE6_CAP_CONTROL_MIDI | LINE6_CAP_PCM | LINE6_CAP_HWMON
        altsetting: 5,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    line6_properties {
        id: b"BassPODxtLive\0".as_ptr() as *const c_char,
        name: b"BassPODxt Live\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 1,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    line6_properties {
        id: b"BassPODxtPro\0".as_ptr() as *const c_char,
        name: b"BassPODxt Pro\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 5,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    line6_properties {
        id: b"PocketPOD\0".as_ptr() as *const c_char,
        name: b"Pocket POD\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 0,
        ep_ctrl_r: 0x82,
        ep_ctrl_w: 0x02,
        ep_audio_r: 0,
        ep_audio_w: 0,
    },
    line6_properties {
        id: b"PODxt\0".as_ptr() as *const c_char,
        name: b"PODxt\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 5,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    line6_properties {
        id: b"PODxtLive\0".as_ptr() as *const c_char,
        name: b"PODxt Live\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 1,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    line6_properties {
        id: b"PODxtPro\0".as_ptr() as *const c_char,
        name: b"PODxt Pro\0".as_ptr() as *const c_char,
        capabilities: 0,
        altsetting: 5,
        ep_ctrl_r: 0x84,
        ep_ctrl_w: 0x03,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
];

// Probe USB device.
unsafe extern "C" fn pod_probe(interface: *mut usb_interface, id: *const usb_device_id) -> c_int {
    line6_probe(
        interface,
        id,
        b"Line6-POD\0".as_ptr() as *const c_char,
        &POD_PROPERTIES_TABLE[(*id).driver_info],
        Some(pod_init),
        mem::size_of::<usb_line6_pod>(),
    )
}

static POD_DRIVER: usb_driver = usb_driver {
    name: KBUILD_MODNAME,
    probe: Some(pod_probe),
    disconnect: Some(line6_disconnect),
    suspend: Some(line6_suspend),
    resume: Some(line6_resume),
    reset_resume: Some(line6_resume),
    id_table: POD_ID_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(usb, pod_id_table) is a macro for kernel module metadata
// MODULE-related macros are module metadata:
// MODULE_DESCRIPTION("Line 6 POD USB driver");
// MODULE_LICENSE("GPL");
// module_usb_driver(pod_driver) - registers the driver
// These are handled by kernel build system and module infrastructure

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
