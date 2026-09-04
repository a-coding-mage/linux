// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 *                         Emil Myhrman (emil.myhrman@gmail.com)
 */

// Linux kernel includes are translated as external module references:
// use linux::wait::*;
// use linux::usb::*;
// use linux::slab::*;
// use linux::module::*;
// use linux::leds::*;
// use sound::core::*;
// use sound::control::*;

// External module dependencies
// use crate::capture;
// use crate::driver;
// use crate::playback;

use core::mem;
use core::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line6DeviceType {
    Line6Guitarport = 0,
    Line6PodstudioGx = 1,
    Line6PodstudioUx1 = 2,
    Line6PodstudioUx2 = 3,
    Line6ToneportGx = 4,
    Line6ToneportUx1 = 5,
    Line6ToneportUx2 = 6,
}

// Forward declaration
pub struct UsbLine6Toneport {
    pub line6: UsbLine6,
    pub source: i32,
    pub serial_number: u32,
    pub firmware_version: u8,
    pub type_: Line6DeviceType,
    pub leds: [ToneportLed; 2],
}

#[repr(C)]
pub struct ToneportLed {
    pub dev: LedClassdev,
    pub name: [u8; 64],
    pub toneport: *mut UsbLine6Toneport,
    pub registered: bool,
}

pub struct UsbLine6 {
    pub usbdev: *mut UsbDevice,
    pub card: *mut SndCard,
    pub line6pcm: *mut SndLine6Pcm,
    pub startup_work: DelayedWork,
    pub disconnect: Option<unsafe extern "C" fn(*mut UsbLine6)>,
    pub startup: Option<unsafe extern "C" fn(*mut UsbLine6)>,
}

pub struct UsbDevice {
    pub dev: Device,
}

pub struct Device;
pub struct SndCard;

pub struct SndLine6Pcm {
    pub line6: *mut UsbLine6,
    pub volume_monitor: i64,
}

#[repr(C)]
pub struct LedClassdev {
    pub name: *const u8,
    pub brightness: u32,
    pub max_brightness: u32,
    pub brightness_set: Option<unsafe extern "C" fn(*mut LedClassdev, u32)>,
}

pub struct DelayedWork;
pub struct UsbInterface;

#[repr(C)]
pub struct UsbDeviceId {
    pub driver_info: usize,
}

pub struct SndKcontrol;
pub struct SndCtlElemInfo;
pub struct SndCtlElemValue;
pub struct SndKcontrolNew;
pub struct UsbDriver;

fn line6_to_toneport(x: *mut UsbLine6) -> *mut UsbLine6Toneport {
    unsafe {
        let offset = mem::offset_of!(UsbLine6Toneport, line6);
        ((x as *mut u8).offset(-(offset as isize))) as *mut UsbLine6Toneport
    }
}

const TONEPORT_PCM_DELAY: u32 = 1;

#[repr(C)]
pub struct SndRatden {
    pub num_min: u32,
    pub num_max: u32,
    pub num_step: u32,
    pub den: u32,
}

pub static TONEPORT_RATDEN: SndRatden = SndRatden {
    num_min: 44100,
    num_max: 44100,
    num_step: 1,
    den: 1,
};

#[repr(C)]
pub struct SndPcmHwParams {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: u32,
    pub period_bytes_min: u32,
    pub period_bytes_max: u32,
    pub periods_min: u32,
    pub periods_max: u32,
}

#[repr(C)]
pub struct Line6RateConstraint {
    pub nrats: u32,
    pub rats: *const SndRatden,
}

#[repr(C)]
pub struct Line6PcmProperties {
    pub playback_hw: SndPcmHwParams,
    pub capture_hw: SndPcmHwParams,
    pub rates: Line6RateConstraint,
    pub bytes_per_channel: u32,
}

pub static mut TONEPORT_PCM_PROPERTIES: Line6PcmProperties = Line6PcmProperties {
    playback_hw: SndPcmHwParams {
        info: 0x00000063,
        formats: 0x00200000,
        rates: 0x00000010,
        rate_min: 44100,
        rate_max: 44100,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: 60000,
        period_bytes_min: 64,
        period_bytes_max: 8192,
        periods_min: 1,
        periods_max: 1024,
    },
    capture_hw: SndPcmHwParams {
        info: 0x00000043,
        formats: 0x00200000,
        rates: 0x00000010,
        rate_min: 44100,
        rate_max: 44100,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: 60000,
        period_bytes_min: 64,
        period_bytes_max: 8192,
        periods_min: 1,
        periods_max: 1024,
    },
    rates: Line6RateConstraint {
        nrats: 1,
        rats: &TONEPORT_RATDEN,
    },
    bytes_per_channel: 2,
};

pub struct SourceInfo {
    pub name: &'static str,
    pub code: u32,
}

pub static TONEPORT_SOURCE_INFO: &[SourceInfo] = &[
    SourceInfo { name: "Microphone", code: 0x0a01 },
    SourceInfo { name: "Line", code: 0x0801 },
    SourceInfo { name: "Instrument", code: 0x0b01 },
    SourceInfo { name: "Inst & Mic", code: 0x0901 },
];

unsafe fn toneport_send_cmd(usbdev: *mut UsbDevice, cmd1: i32, cmd2: i32) -> i32 {
    let mut ret: i32;

    ret = usb_control_msg_send(
        usbdev,
        0,
        0x67,
        0xc0, /* USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT */
        cmd1,
        cmd2,
        ptr::null_mut(),
        0,
        0, /* LINE6_TIMEOUT */
        0, /* GFP_KERNEL */
    );

    if ret != 0 {
        dev_err(
            &mut (*usbdev).dev,
            c"send failed (error %d)\n",
            ret,
        );
        return ret;
    }

    0
}

unsafe fn snd_toneport_monitor_info(
    kcontrol: *mut SndKcontrol,
    uinfo: *mut SndCtlElemInfo,
) -> i32 {
    (*uinfo).type_ = 0; /* SNDRV_CTL_ELEM_TYPE_INTEGER */
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 256;
    0
}

unsafe fn snd_toneport_monitor_get(
    kcontrol: *mut SndKcontrol,
    ucontrol: *mut SndCtlElemValue,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol) as *mut SndLine6Pcm;

    (*ucontrol).value.integer.value[0] = (*line6pcm).volume_monitor;
    0
}

unsafe fn snd_toneport_monitor_put(
    kcontrol: *mut SndKcontrol,
    ucontrol: *mut SndCtlElemValue,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol) as *mut SndLine6Pcm;
    let err: i32;

    if (*ucontrol).value.integer.value[0] == (*line6pcm).volume_monitor {
        return 0;
    }

    (*line6pcm).volume_monitor = (*ucontrol).value.integer.value[0];

    if (*line6pcm).volume_monitor > 0 {
        err = line6_pcm_acquire(line6pcm, 1, true); /* LINE6_STREAM_MONITOR */
        if err < 0 {
            (*line6pcm).volume_monitor = 0;
            line6_pcm_release(line6pcm, 1);
            return err;
        }
    } else {
        line6_pcm_release(line6pcm, 1);
    }

    1
}

unsafe fn snd_toneport_source_info(
    kcontrol: *mut SndKcontrol,
    uinfo: *mut SndCtlElemInfo,
) -> i32 {
    let size = TONEPORT_SOURCE_INFO.len() as u32;

    (*uinfo).type_ = 3; /* SNDRV_CTL_ELEM_TYPE_ENUMERATED */
    (*uinfo).count = 1;
    (*uinfo).value.enumerated.items = size;

    if (*uinfo).value.enumerated.item >= size {
        (*uinfo).value.enumerated.item = size - 1;
    }

    strscpy(
        (*uinfo).value.enumerated.name.as_mut_ptr(),
        TONEPORT_SOURCE_INFO[(*uinfo).value.enumerated.item as usize]
            .name
            .as_ptr() as *const i8,
        64,
    );

    0
}

unsafe fn snd_toneport_source_get(
    kcontrol: *mut SndKcontrol,
    ucontrol: *mut SndCtlElemValue,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol) as *mut SndLine6Pcm;
    let toneport = line6_to_toneport((*line6pcm).line6);

    (*ucontrol).value.enumerated.item[0] = (*toneport).source as u32;
    0
}

unsafe fn snd_toneport_source_put(
    kcontrol: *mut SndKcontrol,
    ucontrol: *mut SndCtlElemValue,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol) as *mut SndLine6Pcm;
    let toneport = line6_to_toneport((*line6pcm).line6);
    let source: u32;

    source = (*ucontrol).value.enumerated.item[0];
    if source >= TONEPORT_SOURCE_INFO.len() as u32 {
        return -22; /* EINVAL */
    }
    if source as i32 == (*toneport).source {
        return 0;
    }

    (*toneport).source = source as i32;
    toneport_send_cmd(
        (*toneport).line6.usbdev,
        TONEPORT_SOURCE_INFO[source as usize].code as i32,
        0x0000,
    );
    1
}

unsafe fn toneport_startup(line6: *mut UsbLine6) {
    line6_pcm_acquire((*line6).line6pcm, 1, true); /* LINE6_STREAM_MONITOR */
}

#[repr(C)]
pub struct SndKcontrolNew {
    pub iface: u32,
    pub name: *const u8,
    pub index: u32,
    pub access: u32,
    pub info: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemInfo) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemValue) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemValue) -> i32>,
}

pub static TONEPORT_CONTROL_MONITOR: SndKcontrolNew = SndKcontrolNew {
    iface: 0, /* SNDRV_CTL_ELEM_IFACE_MIXER */
    name: b"Monitor Playback Volume\0" as *const u8,
    index: 0,
    access: 0x00000003, /* SNDRV_CTL_ELEM_ACCESS_READWRITE */
    info: Some(snd_toneport_monitor_info),
    get: Some(snd_toneport_monitor_get),
    put: Some(snd_toneport_monitor_put),
};

pub static TONEPORT_CONTROL_SOURCE: SndKcontrolNew = SndKcontrolNew {
    iface: 0, /* SNDRV_CTL_ELEM_IFACE_MIXER */
    name: b"PCM Capture Source\0" as *const u8,
    index: 0,
    access: 0x00000003, /* SNDRV_CTL_ELEM_ACCESS_READWRITE */
    info: Some(snd_toneport_source_info),
    get: Some(snd_toneport_source_get),
    put: Some(snd_toneport_source_put),
};

unsafe fn toneport_has_led(toneport: *mut UsbLine6Toneport) -> bool {
    match (*toneport).type_ {
        Line6DeviceType::Line6Guitarport | Line6DeviceType::Line6ToneportGx => true,
        _ => false,
    }
}

pub static TONEPORT_LED_COLORS: &[&str] = &["red", "green"];
pub static TONEPORT_LED_INIT_VALS: &[u32] = &[0x00, 0x26];

unsafe fn toneport_update_led(toneport: *mut UsbLine6Toneport) {
    toneport_send_cmd(
        (*toneport).line6.usbdev,
        (((*toneport).leds[0].dev.brightness << 8) | 0x0002) as i32,
        (*toneport).leds[1].dev.brightness as i32,
    );
}

unsafe extern "C" fn toneport_led_brightness_set(
    led_cdev: *mut LedClassdev,
    brightness: u32,
) {
    let leds = (led_cdev as *mut ToneportLed).offset(-(mem::offset_of!(ToneportLed, dev) as isize) as isize)
        as *mut ToneportLed;
    toneport_update_led((*leds).toneport);
}

unsafe fn toneport_init_leds(toneport: *mut UsbLine6Toneport) -> i32 {
    let dev = &(*(*toneport).line6.usbdev).dev;
    let mut i: i32;
    let mut err: i32;

    i = 0;
    while i < 2 {
        let led = &mut (*toneport).leds[i as usize];
        let leddev = &mut led.dev;

        led.toneport = toneport;
        snprintf(
            led.name.as_mut_ptr() as *mut i8,
            64,
            c"%s::%s\0".as_ptr() as *const i8,
            dev_name(dev),
            TONEPORT_LED_COLORS[i as usize].as_ptr() as *const i8,
        );
        leddev.name = led.name.as_ptr();
        leddev.brightness = TONEPORT_LED_INIT_VALS[i as usize];
        leddev.max_brightness = 0x26;
        leddev.brightness_set = Some(toneport_led_brightness_set);
        err = led_classdev_register(dev, leddev);
        if err != 0 {
            return err;
        }
        led.registered = true;

        i += 1;
    }

    0
}

unsafe fn toneport_remove_leds(toneport: *mut UsbLine6Toneport) {
    let mut led: *mut ToneportLed;
    let mut i: i32;

    i = 0;
    while i < 2 {
        led = &mut (*toneport).leds[i as usize];
        if !(*led).registered {
            break;
        }
        led_classdev_unregister(&mut (*led).dev);
        (*led).registered = false;

        i += 1;
    }
}

unsafe fn toneport_has_source_select(toneport: *mut UsbLine6Toneport) -> bool {
    match (*toneport).type_ {
        Line6DeviceType::Line6ToneportUx1
        | Line6DeviceType::Line6ToneportUx2
        | Line6DeviceType::Line6PodstudioUx1
        | Line6DeviceType::Line6PodstudioUx2 => true,
        _ => false,
    }
}

unsafe fn toneport_setup(toneport: *mut UsbLine6Toneport) -> i32 {
    let ticks: *mut u32;
    let line6: *mut UsbLine6 = &mut (*toneport).line6;
    let usbdev: *mut UsbDevice = (*line6).usbdev;

    ticks = kmalloc_obj() as *mut u32;
    if ticks.is_null() {
        return -12; /* ENOMEM */
    }

    *ticks = ktime_get_real_seconds() as u32;
    line6_write_data(line6, 0x80c6, ticks as *const u8, 4);
    kfree(ticks as *const u8);

    toneport_send_cmd(usbdev, 0x0301, 0x0000);

    if toneport_has_source_select(toneport) {
        toneport_send_cmd(
            usbdev,
            TONEPORT_SOURCE_INFO[(*toneport).source as usize].code as i32,
            0x0000,
        );
    }

    if toneport_has_led(toneport) {
        toneport_update_led(toneport);
    }

    schedule_delayed_work(
        &mut (*line6).startup_work,
        secs_to_jiffies(TONEPORT_PCM_DELAY),
    );
    0
}

unsafe fn line6_toneport_disconnect(line6: *mut UsbLine6) {
    let toneport = line6_to_toneport(line6);

    if toneport_has_led(toneport) {
        toneport_remove_leds(toneport);
    }
}

unsafe fn toneport_init(line6: *mut UsbLine6, id: *const UsbDeviceId) -> i32 {
    let mut err: i32;
    let toneport = line6_to_toneport(line6);

    (*toneport).type_ = mem::transmute((*id).driver_info as u32);

    (*line6).disconnect = Some(line6_toneport_disconnect);
    (*line6).startup = Some(toneport_startup);

    err = line6_init_pcm(line6, &TONEPORT_PCM_PROPERTIES);
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*line6).card,
        snd_ctl_new1(&TONEPORT_CONTROL_MONITOR, (*line6).line6pcm as *const u8),
    );
    if err < 0 {
        return err;
    }

    if toneport_has_source_select(toneport) {
        err = snd_ctl_add(
            (*line6).card,
            snd_ctl_new1(&TONEPORT_CONTROL_SOURCE, (*line6).line6pcm as *const u8),
        );
        if err < 0 {
            return err;
        }
    }

    line6_read_serial_number(line6, &mut (*toneport).serial_number);
    line6_read_data(line6, 0x80c2, &mut (*toneport).firmware_version, 1);

    if toneport_has_led(toneport) {
        err = toneport_init_leds(toneport);
        if err < 0 {
            return err;
        }
    }

    err = toneport_setup(toneport);
    if err != 0 {
        return err;
    }

    snd_card_register((*line6).card)
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn toneport_reset_resume(interface: *mut UsbInterface) -> i32 {
    let mut err: i32;

    err = toneport_setup(usb_get_intfdata(interface) as *mut UsbLine6Toneport);
    if err != 0 {
        return err;
    }
    line6_resume(interface)
}

const LINE6_DEVICE_VENDOR: u16 = 0x0e41;

macro_rules! LINE6_DEVICE {
    ($prod:expr) => {
        UsbDeviceId {
            driver_info: $prod,
        }
    };
}

macro_rules! LINE6_IF_NUM {
    ($prod:expr, $n:expr) => {
        UsbDeviceId {
            driver_info: ($prod | ($n << 16)),
        }
    };
}

pub static TONEPORT_ID_TABLE: &[UsbDeviceId] = &[
    LINE6_DEVICE!(0x4750), /* LINE6_GUITARPORT */
    LINE6_DEVICE!(0x4153), /* LINE6_PODSTUDIO_GX */
    LINE6_DEVICE!(0x4150), /* LINE6_PODSTUDIO_UX1 */
    LINE6_IF_NUM!(0x4151, 0), /* LINE6_PODSTUDIO_UX2 */
    LINE6_DEVICE!(0x4147), /* LINE6_TONEPORT_GX */
    LINE6_DEVICE!(0x4141), /* LINE6_TONEPORT_UX1 */
    LINE6_IF_NUM!(0x4142, 0), /* LINE6_TONEPORT_UX2 */
    UsbDeviceId {
        driver_info: 0,
    }, /* sentinel */
];

#[repr(C)]
pub struct Line6Properties {
    pub id: *const u8,
    pub name: *const u8,
    pub capabilities: u32,
    pub altsetting: u32,
    pub ep_audio_r: u32,
    pub ep_audio_w: u32,
}

pub static TONEPORT_PROPERTIES_TABLE: &[Line6Properties] = &[
    /* LINE6_GUITARPORT */
    Line6Properties {
        id: b"GuitarPort\0" as *const u8,
        name: b"GuitarPort\0" as *const u8,
        capabilities: 1, /* LINE6_CAP_PCM */
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_PODSTUDIO_GX */
    Line6Properties {
        id: b"PODStudioGX\0" as *const u8,
        name: b"POD Studio GX\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_PODSTUDIO_UX1 */
    Line6Properties {
        id: b"PODStudioUX1\0" as *const u8,
        name: b"POD Studio UX1\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_PODSTUDIO_UX2 */
    Line6Properties {
        id: b"PODStudioUX2\0" as *const u8,
        name: b"POD Studio UX2\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_TONEPORT_GX */
    Line6Properties {
        id: b"TonePortGX\0" as *const u8,
        name: b"TonePort GX\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_TONEPORT_UX1 */
    Line6Properties {
        id: b"TonePortUX1\0" as *const u8,
        name: b"TonePort UX1\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
    /* LINE6_TONEPORT_UX2 */
    Line6Properties {
        id: b"TonePortUX2\0" as *const u8,
        name: b"TonePort UX2\0" as *const u8,
        capabilities: 1,
        altsetting: 2,
        ep_audio_r: 0x82,
        ep_audio_w: 0x01,
    },
];

unsafe fn toneport_probe(interface: *mut UsbInterface, id: *const UsbDeviceId) -> i32 {
    line6_probe(
        interface,
        id,
        b"Line6-TonePort\0" as *const u8,
        &TONEPORT_PROPERTIES_TABLE[(*id).driver_info],
        toneport_init,
        mem::size_of::<UsbLine6Toneport>(),
    )
}

#[repr(C)]
pub struct UsbDriver {
    pub name: *const u8,
    pub probe: Option<unsafe extern "C" fn(*mut UsbInterface, *const UsbDeviceId) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut UsbInterface)>,
    pub suspend: Option<unsafe extern "C" fn(*mut UsbInterface, i32) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut UsbInterface) -> i32>,
    pub reset_resume: Option<unsafe extern "C" fn(*mut UsbInterface) -> i32>,
    pub id_table: *const UsbDeviceId,
}

pub static mut TONEPORT_DRIVER: UsbDriver = UsbDriver {
    name: b"toneport\0" as *const u8,
    probe: Some(toneport_probe),
    disconnect: Some(line6_disconnect),
    #[cfg(feature = "CONFIG_PM")]
    suspend: Some(line6_suspend),
    #[cfg(feature = "CONFIG_PM")]
    resume: Some(line6_resume),
    #[cfg(feature = "CONFIG_PM")]
    reset_resume: Some(toneport_reset_resume),
    #[cfg(not(feature = "CONFIG_PM"))]
    suspend: None,
    #[cfg(not(feature = "CONFIG_PM"))]
    resume: None,
    #[cfg(not(feature = "CONFIG_PM"))]
    reset_resume: None,
    id_table: TONEPORT_ID_TABLE.as_ptr(),
};

// module_usb_driver(toneport_driver);
// MODULE_DESCRIPTION("TonePort USB driver");
// MODULE_LICENSE("GPL");

// External kernel function declarations
extern "C" {
    fn usb_control_msg_send(
        usbdev: *mut UsbDevice,
        ep: u32,
        request: u8,
        requesttype: u32,
        value: i32,
        index: i32,
        data: *mut u8,
        size: u32,
        timeout: u32,
        gfp_mask: u32,
    ) -> i32;

    fn dev_err(dev: *mut Device, fmt: *const i8, ...);

    fn snd_kcontrol_chip(kcontrol: *mut SndKcontrol) -> *mut core::ffi::c_void;

    fn line6_pcm_acquire(line6pcm: *mut SndLine6Pcm, stream: i32, acquire: bool) -> i32;

    fn line6_pcm_release(line6pcm: *mut SndLine6Pcm, stream: i32);

    fn strscpy(dest: *mut i8, src: *const i8, size: usize) -> usize;

    fn kmalloc_obj() -> *mut core::ffi::c_void;

    fn kfree(ptr: *const u8);

    fn ktime_get_real_seconds() -> i64;

    fn line6_write_data(line6: *mut UsbLine6, address: u32, data: *const u8, datalen: i32) -> i32;

    fn line6_init_pcm(
        line6: *mut UsbLine6,
        properties: *const Line6PcmProperties,
    ) -> i32;

    fn snd_ctl_add(card: *mut SndCard, kcontrol: *mut core::ffi::c_void) -> i32;

    fn snd_ctl_new1(
        kcontrol_new: *const SndKcontrolNew,
        private_data: *const u8,
    ) -> *mut core::ffi::c_void;

    fn line6_read_serial_number(line6: *mut UsbLine6, serial_number: *mut u32);

    fn line6_read_data(
        line6: *mut UsbLine6,
        address: u32,
        data: *mut u8,
        datalen: i32,
    ) -> i32;

    fn led_classdev_register(dev: *mut Device, led_cdev: *mut LedClassdev) -> i32;

    fn led_classdev_unregister(led_cdev: *mut LedClassdev);

    fn schedule_delayed_work(work: *mut DelayedWork, delay: u32) -> bool;

    fn secs_to_jiffies(secs: u32) -> u32;

    fn snd_card_register(card: *mut SndCard) -> i32;

    fn usb_get_intfdata(intf: *mut UsbInterface) -> *mut UsbLine6Toneport;

    fn line6_resume(interface: *mut UsbInterface) -> i32;

    fn dev_name(dev: *mut Device) -> *const i8;

    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;

    fn line6_probe(
        interface: *mut UsbInterface,
        id: *const UsbDeviceId,
        name: *const u8,
        properties: *const Line6Properties,
        line6_init_fn: unsafe extern "C" fn(*mut UsbLine6, *const UsbDeviceId) -> i32,
        size: usize,
    ) -> i32;

    fn line6_disconnect(interface: *mut UsbInterface);

    #[cfg(feature = "CONFIG_PM")]
    fn line6_suspend(interface: *mut UsbInterface, message: i32) -> i32;

    #[cfg(feature = "CONFIG_PM")]
    fn line6_resume(interface: *mut UsbInterface) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
