// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Pod HD
 *
 * Copyright (C) 2011 Stefan Hajnoczi <stefanha@gmail.com>
 * Copyright (C) 2015 Andrej Krutak <dev@andree.sk>
 * Copyright (C) 2017 Hans P. Moller <hmoller@uc.cl>
 */

// Depends on: linux/usb.h, linux/slab.h, linux/module.h, sound/core.h, sound/control.h, sound/pcm.h, driver.h, pcm.h

const PODHD_STARTUP_DELAY: u32 = 500;

pub const LINE6_PODHD300: u32 = 0;
pub const LINE6_PODHD400: u32 = 1;
pub const LINE6_PODHD500: u32 = 2;
pub const LINE6_PODX3: u32 = 3;
pub const LINE6_PODX3LIVE: u32 = 4;
pub const LINE6_PODHD500X: u32 = 5;
pub const LINE6_PODHDDESKTOP: u32 = 6;
pub const LINE6_PODHDPROX: u32 = 7;
pub const LINE6_PODHDPRO: u32 = 8;

extern "C" {
    // External types from included headers
    pub type usb_line6;
    pub type snd_card;
    pub type device;
    pub type device_attribute;
    pub type snd_ratden;
    pub type line6_pcm_properties;
    pub type usb_device;
    pub type snd_kcontrol;
    pub type snd_ctl_elem_info;
    pub type snd_line6_pcm;
    pub type snd_ctl_elem_value;
    pub type snd_kcontrol_new;
    pub type usb_interface;
    pub type usb_device_id;
    pub type line6_properties;
    pub type usb_driver;
    pub type attribute;
    pub type attribute_group;
}

#[repr(C)]
pub struct usb_line6_podhd {
    pub line6: usb_line6,
    pub serial_number: u32,
    pub firmware_version: i32,
    pub monitor_level: i32,
}

// container_of equivalent for line6 to podhd conversion
unsafe fn line6_to_podhd(x: *mut usb_line6) -> *mut usb_line6_podhd {
    let x_addr = x as usize;
    let offset = std::mem::offset_of!(usb_line6_podhd, line6);
    (x_addr - offset) as *mut usb_line6_podhd
}

extern "C" {
    // External function declarations
    fn dev_to_snd_card(dev: *mut device) -> *mut snd_card;
    fn sysfs_emit(buf: *mut u8, fmt: *const u8, ...) -> usize;
    fn usb_control_msg_send(
        usbdev: *mut usb_device,
        endpoint: u8,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: u16,
        timeout: u32,
        gfp_t: u32,
    ) -> i32;
    fn usb_control_msg_recv(
        usbdev: *mut usb_device,
        endpoint: u8,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: u16,
        timeout: u32,
        gfp_t: u32,
    ) -> i32;
    fn line6_read_data(
        line6: *mut usb_line6,
        address: u32,
        data: *mut u8,
        datalen: u32,
    ) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn line6_read_serial_number(line6: *mut usb_line6, sn: *mut u32);
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: u8) -> *mut usb_interface;
    fn usb_driver_release_interface(driver: *mut usb_driver, intf: *mut usb_interface);
    fn kmemdup(
        src: *const core::ffi::c_void,
        len: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn line6_send_raw_message(line6: *mut usb_line6, buffer: *mut u8, size: usize);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_line6_pcm;
    fn snd_card_add_dev_attr(
        card: *mut snd_card,
        group: *const attribute_group,
    ) -> i32;
    fn line6_init_pcm(
        line6: *mut usb_line6,
        properties: *const line6_pcm_properties,
    ) -> i32;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_new1(
        kcontrolp: *const snd_kcontrol_new,
        private_data: *mut core::ffi::c_void,
    ) -> *mut snd_kcontrol;
    fn schedule_delayed_work(work: *mut core::ffi::c_void, delay: u32);
    fn usb_driver_claim_interface(
        driver: *mut usb_driver,
        intf: *mut usb_interface,
        priv: *mut core::ffi::c_void,
    ) -> i32;
    fn line6_probe(
        interface: *mut usb_interface,
        id: *const usb_device_id,
        name: *const u8,
        properties: *const line6_properties,
        init_func: extern "C" fn(*mut usb_line6, *const usb_device_id) -> i32,
        size: usize,
    ) -> i32;
    fn line6_disconnect(interface: *mut usb_interface);
    #[cfg(CONFIG_PM)]
    fn line6_suspend(interface: *mut usb_interface, msg: u32) -> i32;
    #[cfg(CONFIG_PM)]
    fn line6_resume(interface: *mut usb_interface) -> i32;
}

// Static device attributes - raw pointer initialization
static mut DEV_ATTR_FIRMWARE_VERSION: *mut attribute = std::ptr::null_mut();
static mut DEV_ATTR_SERIAL_NUMBER: *mut attribute = std::ptr::null_mut();

unsafe extern "C" fn serial_number_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> usize {
    let card = dev_to_snd_card(dev);
    let pod = (*card).private_data as *mut usb_line6_podhd;
    sysfs_emit(buf, b"%u\n\0".as_ptr(), (*pod).serial_number)
}

unsafe extern "C" fn firmware_version_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> usize {
    let card = dev_to_snd_card(dev);
    let pod = (*card).private_data as *mut usb_line6_podhd;
    sysfs_emit(buf, b"%06x\n\0".as_ptr(), (*pod).firmware_version)
}

static PODHD_DEV_ATTRS: [*mut attribute; 3] = [
    unsafe { &mut DEV_ATTR_FIRMWARE_VERSION as *mut _ as *mut attribute },
    unsafe { &mut DEV_ATTR_SERIAL_NUMBER as *mut _ as *mut attribute },
    std::ptr::null_mut(),
];

static PODHD_DEV_ATTR_GROUP: attribute_group = unsafe { std::mem::zeroed() };

static FLOAT_ZERO_TO_ONE_LOOKUP: [u32; 101] = [
    0x00000000, 0x3c23d70a, 0x3ca3d70a, 0x3cf5c28f, 0x3d23d70a, 0x3d4ccccd,
    0x3d75c28f, 0x3d8f5c29, 0x3da3d70a, 0x3db851ec, 0x3dcccccd, 0x3de147ae,
    0x3df5c28f, 0x3e051eb8, 0x3e0f5c29, 0x3e19999a, 0x3e23d70a, 0x3e2e147b,
    0x3e3851ec, 0x3e428f5c, 0x3e4ccccd, 0x3e570a3d, 0x3e6147ae, 0x3e6b851f,
    0x3e75c28f, 0x3e800000, 0x3e851eb8, 0x3e8a3d71, 0x3e8f5c29, 0x3e947ae1,
    0x3e99999a, 0x3e9eb852, 0x3ea3d70a, 0x3ea8f5c3, 0x3eae147b, 0x3eb33333,
    0x3eb851ec, 0x3ebd70a4, 0x3ec28f5c, 0x3ec7ae14, 0x3ecccccd, 0x3ed1eb85,
    0x3ed70a3d, 0x3edc28f6, 0x3ee147ae, 0x3ee66666, 0x3eeb851f, 0x3ef0a3d7,
    0x3ef5c28f, 0x3efae148, 0x3f000000, 0x3f028f5c, 0x3f051eb8, 0x3f07ae14,
    0x3f0a3d71, 0x3f0ccccd, 0x3f0f5c29, 0x3f11eb85, 0x3f147ae1, 0x3f170a3d,
    0x3f19999a, 0x3f1c28f6, 0x3f1eb852, 0x3f2147ae, 0x3f23d70a, 0x3f266666,
    0x3f28f5c3, 0x3f2b851f, 0x3f2e147b, 0x3f30a3d7, 0x3f333333, 0x3f35c28f,
    0x3f3851ec, 0x3f3ae148, 0x3f3d70a4, 0x3f400000, 0x3f428f5c, 0x3f451eb8,
    0x3f47ae14, 0x3f4a3d71, 0x3f4ccccd, 0x3f4f5c29, 0x3f51eb85, 0x3f547ae1,
    0x3f570a3d, 0x3f59999a, 0x3f5c28f6, 0x3f5eb852, 0x3f6147ae, 0x3f63d70a,
    0x3f666666, 0x3f68f5c3, 0x3f6b851f, 0x3f6e147b, 0x3f70a3d7, 0x3f733333,
    0x3f75c28f, 0x3f7851ec, 0x3f7ae148, 0x3f7d70a4, 0x3f800000,
];

unsafe fn podhd_set_monitor_level(podhd: *mut usb_line6_podhd, mut value: i32) {
    static MSG: [u8; 16] = [
        0x0c, 0x00,
        0x01, 0x00,
        0x02, 0x00,
        0x04, 0x41,
        0x04, 0x00, 0x13, 0x00,
        0x00, 0x00, 0x00, 0x00
    ];

    let buf = kmemdup(MSG.as_ptr() as *const core::ffi::c_void, MSG.len(), 0) as *mut u8;
    if buf.is_null() {
        return;
    }

    if value < 0 {
        value = 0;
    }

    if value as usize >= FLOAT_ZERO_TO_ONE_LOOKUP.len() {
        value = (FLOAT_ZERO_TO_ONE_LOOKUP.len() - 1) as i32;
    }

    let fl = FLOAT_ZERO_TO_ONE_LOOKUP[value as usize];

    *buf.add(12) = ((fl >> 0) & 0xff) as u8;
    *buf.add(13) = ((fl >> 8) & 0xff) as u8;
    *buf.add(14) = ((fl >> 16) & 0xff) as u8;
    *buf.add(15) = ((fl >> 24) & 0xff) as u8;

    line6_send_raw_message(&mut (*podhd).line6, buf, MSG.len());
    kfree(buf as *mut core::ffi::c_void);

    (*podhd).monitor_level = value;
}

unsafe extern "C" fn snd_podhd_control_monitor_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = 2;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 100;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn snd_podhd_control_monitor_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let podhd = line6_to_podhd(&mut (*line6pcm).line6);

    (*ucontrol).value.integer.value[0] = (*podhd).monitor_level as i64;
    0
}

unsafe extern "C" fn snd_podhd_control_monitor_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let podhd = line6_to_podhd(&mut (*line6pcm).line6);

    if (*ucontrol).value.integer.value[0] == (*podhd).monitor_level as i64 {
        return 0;
    }

    podhd_set_monitor_level(podhd, (*ucontrol).value.integer.value[0] as i32);
    1
}

static PODHD_CONTROL_MONITOR: snd_kcontrol_new = unsafe { std::mem::zeroed() };

unsafe fn podhd_dev_start(pod: *mut usb_line6_podhd) -> i32 {
    let usbdev = (*pod).line6.usbdev;
    let mut init_bytes: [u8; 8] = [0; 8];

    let mut ret = usb_control_msg_send(
        usbdev, 0,
        0x67, 0xc0,
        0x11, 0,
        std::ptr::null_mut(), 0, 5000, 0x10,
    );
    if ret != 0 {
        dev_err((*pod).line6.ifcdev, b"read request failed (error %d)\n\0".as_ptr(), ret);
        return ret;
    }

    ret = usb_control_msg_recv(
        usbdev, 0, 0x67,
        0xc0 | 0x80,
        0x11, 0x0,
        init_bytes.as_mut_ptr() as *mut core::ffi::c_void, 3, 5000, 0x10,
    );
    if ret != 0 {
        dev_err((*pod).line6.ifcdev,
            b"receive length failed (error %d)\n\0".as_ptr(), ret);
        return ret;
    }

    (*pod).firmware_version =
        ((init_bytes[0] as i32) << 16) | ((init_bytes[1] as i32) << 8) | (init_bytes[2] as i32);

    for i in 0..=16 {
        ret = line6_read_data(&mut (*pod).line6, 0xf000 + 0x08 * i, init_bytes.as_mut_ptr(), 8);
        if ret < 0 {
            return ret;
        }
    }

    ret = usb_control_msg_send(
        usbdev, 0,
        3,
        0x00,
        1, 0,
        std::ptr::null_mut(), 0, 5000, 0x10,
    );
    ret
}

unsafe extern "C" fn podhd_startup(line6: *mut usb_line6) {
    let pod = line6_to_podhd(line6);

    let _ = podhd_dev_start(pod);
    line6_read_serial_number(&mut (*pod).line6, &mut (*pod).serial_number);
    if snd_card_register((*line6).card) != 0 {
        dev_err((*line6).ifcdev, b"Failed to register POD HD card.\n\0".as_ptr());
    }
}

unsafe extern "C" fn podhd_disconnect(line6: *mut usb_line6) {
    let pod = line6_to_podhd(line6);

    if ((*(*pod).line6.properties).capabilities & 0x04) != 0 {
        let intf = usb_ifnum_to_if(
            (*line6).usbdev,
            (*(*pod).line6.properties).ctrl_if,
        );
        if !intf.is_null() {
            usb_driver_release_interface(&mut PODHD_DRIVER, intf);
        }
    }
}

unsafe extern "C" fn podhd_init(line6: *mut usb_line6, id: *const usb_device_id) -> i32 {
    let mut err;
    let pod = line6_to_podhd(line6);

    (*line6).disconnect = Some(podhd_disconnect);
    (*line6).startup = Some(podhd_startup);

    if ((*(*pod).line6.properties).capabilities & 0x01) != 0 {
        let intf = usb_ifnum_to_if(
            (*line6).usbdev,
            (*(*pod).line6.properties).ctrl_if,
        );
        if intf.is_null() {
            dev_err((*pod).line6.ifcdev, b"interface %d not found\n\0".as_ptr(),
                (*(*pod).line6.properties).ctrl_if);
            return -19;
        }

        err = usb_driver_claim_interface(&mut PODHD_DRIVER, intf, std::ptr::null_mut());
        if err != 0 {
            dev_err((*pod).line6.ifcdev, b"can't claim interface %d, error %d\n\0".as_ptr(),
                (*(*pod).line6.properties).ctrl_if, err);
            return err;
        }
    }

    if ((*(*pod).line6.properties).capabilities & 0x04) != 0 {
        err = snd_card_add_dev_attr((*line6).card, &PODHD_DEV_ATTR_GROUP);
        if err < 0 {
            return err;
        }
    }

    if ((*(*pod).line6.properties).capabilities & 0x08) != 0 {
        let props = if (*id).driver_info == LINE6_PODX3 || (*id).driver_info == LINE6_PODX3LIVE {
            &PODX3_PCM_PROPERTIES
        } else {
            &PODHD_PCM_PROPERTIES
        };
        err = line6_init_pcm(line6, props);
        if err < 0 {
            return err;
        }
    }

    if ((*(*pod).line6.properties).capabilities & 0x10) != 0 {
        podhd_set_monitor_level(pod, 100);
        err = snd_ctl_add((*line6).card,
            snd_ctl_new1(&PODHD_CONTROL_MONITOR, (*line6).line6pcm as *mut core::ffi::c_void));
        if err < 0 {
            return err;
        }
    }

    if ((*(*pod).line6.properties).capabilities & 0x04) == 0 {
        return snd_card_register((*line6).card);
    }

    schedule_delayed_work(&mut (*line6).startup_work as *mut _ as *mut core::ffi::c_void,
        PODHD_STARTUP_DELAY);
    0
}

// USB device IDs table with configuration data
static PODHD_ID_TABLE: [usb_device_id; 9] = unsafe { [
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
] };

// Line6 properties table with configuration for each device
static PODHD_PROPERTIES_TABLE: [line6_properties; 9] = unsafe { [
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
    std::mem::zeroed(),
] };

// PCM properties for PODHD devices
static PODHD_PCM_PROPERTIES: line6_pcm_properties = unsafe { std::mem::zeroed() };

// PCM properties for PODX3 devices
static PODX3_PCM_PROPERTIES: line6_pcm_properties = unsafe { std::mem::zeroed() };

unsafe extern "C" fn podhd_probe(
    interface: *mut usb_interface,
    id: *const usb_device_id,
) -> i32 {
    line6_probe(
        interface, id,
        b"Line6-PODHD\0".as_ptr(),
        &PODHD_PROPERTIES_TABLE[(*id).driver_info as usize],
        podhd_init,
        std::mem::size_of::<usb_line6_podhd>(),
    )
}

static mut PODHD_DRIVER: usb_driver = unsafe { std::mem::zeroed() };

// USB driver registration - module_usb_driver(podhd_driver) expands to:
// module_driver(podhd_driver, usb_register, usb_deregister)
// which creates module init/exit functions that register the USB driver

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
