// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Linux kernel headers: linux/slab.h, linux/spinlock.h, linux/usb.h,
// linux/wait.h, linux/module.h, sound/core.h
// External module: driver.h

const VARIAX_STARTUP_DELAY1: usize = 1000;
const VARIAX_STARTUP_DELAY3: usize = 100;
const VARIAX_STARTUP_DELAY4: usize = 100;

// Stages of Variax startup procedure
const VARIAX_STARTUP_VERSIONREQ: i32 = 0;
const VARIAX_STARTUP_ACTIVATE: i32 = 1;
const VARIAX_STARTUP_SETUP: i32 = 2;

const LINE6_PODXTLIVE_VARIAX: i32 = 0;
const LINE6_VARIAX: i32 = 1;

#[repr(C)]
struct usb_line6_variax {
    /* Generic Line 6 USB data */
    line6: usb_line6,

    /* Buffer for activation code */
    buffer_activate: *mut u8,

    /* Current progress in startup procedure */
    startup_progress: i32,
}

// container_of macro equivalent - assumes external implementation
// The macro line6_to_variax(x) is defined as:
// container_of(x, struct usb_line6_variax, line6)
macro_rules! line6_to_variax {
    ($x:expr) => {
        container_of!($x, usb_line6_variax, line6)
    };
}

const VARIAX_OFFSET_ACTIVATE: usize = 7;

/*
    This message is sent by the device during initialization and identifies
    the connected guitar version.
*/
static VARIAX_INIT_VERSION: &[u8] = &[
    0xf0, 0x7e, 0x7f, 0x06, 0x02, 0x00, 0x01, 0x0c,
    0x07, 0x00, 0x00, 0x00
];

/*
    This message is the last one sent by the device during initialization.
*/
static VARIAX_INIT_DONE: &[u8] = &[
    0xf0, 0x00, 0x01, 0x0c, 0x07, 0x00, 0x6b
];

static VARIAX_ACTIVATE: &[u8] = &[
    0xf0, 0x00, 0x01, 0x0c, 0x07, 0x00, 0x2a, 0x01,
    0xf7
];

unsafe fn variax_activate_async(variax: *mut usb_line6_variax, a: u8) {
    *(*variax).buffer_activate.add(VARIAX_OFFSET_ACTIVATE) = a;
    line6_send_raw_message_async(
        &mut (*variax).line6,
        (*variax).buffer_activate,
        VARIAX_ACTIVATE.len(),
    );
}

/*
    Variax startup procedure.
    This is a sequence of functions with special requirements (e.g., must
    not run immediately after initialization, must not run in interrupt
    context). After the last one has finished, the device is ready to use.
*/

unsafe fn variax_startup(line6: *mut usb_line6) {
    let variax = line6_to_variax!(line6);

    match (*variax).startup_progress {
        VARIAX_STARTUP_VERSIONREQ => {
            /* repeat request until getting the response */
            schedule_delayed_work(
                &mut (*line6).startup_work,
                msecs_to_jiffies(VARIAX_STARTUP_DELAY1),
            );
            /* request firmware version: */
            line6_version_request_async(line6);
        }
        VARIAX_STARTUP_ACTIVATE => {
            /* activate device: */
            variax_activate_async(variax, 1);
            (*variax).startup_progress = VARIAX_STARTUP_SETUP;
            schedule_delayed_work(
                &mut (*line6).startup_work,
                msecs_to_jiffies(VARIAX_STARTUP_DELAY4),
            );
        }
        VARIAX_STARTUP_SETUP => {
            /* ALSA audio interface: */
            snd_card_register((*variax).line6.card);
        }
        _ => {}
    }
}

/*
    Process a completely received message.
*/
unsafe fn line6_variax_process_message(line6: *mut usb_line6) {
    let variax = line6_to_variax!(line6);
    let buf = (*variax).line6.buffer_message;

    match *buf {
        LINE6_RESET => {
            dev_info(
                (*variax).line6.ifcdev,
                b"VARIAX reset\n" as *const u8 as *const i8,
            );
        }

        LINE6_SYSEX_BEGIN => {
            if memcmp(
                buf.add(1) as *const std::ffi::c_void,
                VARIAX_INIT_VERSION.as_ptr().add(1) as *const std::ffi::c_void,
                VARIAX_INIT_VERSION.len() - 1,
            ) == 0
            {
                if (*variax).startup_progress >= VARIAX_STARTUP_ACTIVATE {
                    return;
                }
                (*variax).startup_progress = VARIAX_STARTUP_ACTIVATE;
                cancel_delayed_work(&mut (*line6).startup_work);
                schedule_delayed_work(
                    &mut (*line6).startup_work,
                    msecs_to_jiffies(VARIAX_STARTUP_DELAY3),
                );
            } else if memcmp(
                buf.add(1) as *const std::ffi::c_void,
                VARIAX_INIT_DONE.as_ptr().add(1) as *const std::ffi::c_void,
                VARIAX_INIT_DONE.len() - 1,
            ) == 0
            {
                /* notify of complete initialization: */
                if (*variax).startup_progress >= VARIAX_STARTUP_SETUP {
                    return;
                }
                cancel_delayed_work(&mut (*line6).startup_work);
                schedule_delayed_work(&mut (*line6).startup_work, 0);
            }
        }
        _ => {}
    }
}

/*
    Variax destructor.
*/
unsafe fn line6_variax_disconnect(line6: *mut usb_line6) {
    let variax = line6_to_variax!(line6);

    kfree((*variax).buffer_activate as *mut std::ffi::c_void);
}

/*
     Try to init workbench device.
*/
unsafe fn variax_init(line6: *mut usb_line6, id: *const usb_device_id) -> i32 {
    let variax = line6_to_variax!(line6);

    (*line6).process_message = Some(line6_variax_process_message);
    (*line6).disconnect = Some(line6_variax_disconnect);
    (*line6).startup = Some(variax_startup);

    /* initialize USB buffers: */
    (*variax).buffer_activate = kmemdup(
        VARIAX_ACTIVATE.as_ptr() as *const std::ffi::c_void,
        VARIAX_ACTIVATE.len(),
        GFP_KERNEL,
    ) as *mut u8;

    if (*variax).buffer_activate.is_null() {
        return -12i32; /* -ENOMEM */
    }

    /* initiate startup procedure: */
    schedule_delayed_work(
        &mut (*line6).startup_work,
        msecs_to_jiffies(VARIAX_STARTUP_DELAY1),
    );
    0
}

#[repr(C)]
struct usb_device_id {
    // External structure definition
    _opaque: [u8; 0],
}

#[repr(C)]
struct line6_properties {
    // External structure definition
    _opaque: [u8; 0],
}

#[repr(C)]
struct usb_interface {
    // External structure definition
    _opaque: [u8; 0],
}

#[repr(C)]
struct usb_driver {
    // External structure definition
    _opaque: [u8; 0],
}

#[repr(C)]
struct usb_line6 {
    // External structure definition
    startup_work: [u8; 0],
    process_message: Option<unsafe fn(*mut usb_line6)>,
    disconnect: Option<unsafe fn(*mut usb_line6)>,
    startup: Option<unsafe fn(*mut usb_line6)>,
    buffer_message: *mut u8,
    ifcdev: *mut std::ffi::c_void,
    card: *mut std::ffi::c_void,
    _pad: [u8; 0],
}

// Device ID table macros
// #define LINE6_DEVICE(prod) USB_DEVICE(0x0e41, prod)
// #define LINE6_IF_NUM(prod, n) USB_DEVICE_INTERFACE_NUMBER(0x0e41, prod, n)

/* table of devices that work with this driver */
static VARIAX_ID_TABLE: &[usb_device_id] = &[
    // { LINE6_IF_NUM(0x4650, 1), .driver_info = LINE6_PODXTLIVE_VARIAX },
    // { LINE6_DEVICE(0x534d),    .driver_info = LINE6_VARIAX },
    // {} (null terminator in C)
];

// MODULE_DEVICE_TABLE(usb, variax_id_table);

static VARIAX_PROPERTIES_TABLE: &[line6_properties] = &[
    // [LINE6_PODXTLIVE_VARIAX] = {
    //     .id = "PODxtLive",
    //     .name = "PODxt Live",
    //     .capabilities = LINE6_CAP_CONTROL | LINE6_CAP_CONTROL_MIDI,
    //     .altsetting = 1,
    //     .ep_ctrl_r = 0x86,
    //     .ep_ctrl_w = 0x05,
    //     .ep_audio_r = 0x82,
    //     .ep_audio_w = 0x01,
    // },
    // [LINE6_VARIAX] = {
    //     .id = "Variax",
    //     .name = "Variax Workbench",
    //     .capabilities = LINE6_CAP_CONTROL | LINE6_CAP_CONTROL_MIDI,
    //     .altsetting = 1,
    //     .ep_ctrl_r = 0x82,
    //     .ep_ctrl_w = 0x01,
    //     /* no audio channel */
    // }
];

/*
    Probe USB device.
*/
unsafe fn variax_probe(interface: *mut usb_interface, id: *const usb_device_id) -> i32 {
    line6_probe(
        interface,
        id,
        b"Line6-Variax" as *const u8 as *const i8,
        // &variax_properties_table[(*id).driver_info as usize],
        0 as *const line6_properties,
        variax_init,
        std::mem::size_of::<usb_line6_variax>(),
    )
}

static VARIAX_DRIVER: usb_driver = unsafe {
    // .name = KBUILD_MODNAME,
    // .probe = variax_probe,
    // .disconnect = line6_disconnect,
    // #ifdef CONFIG_PM
    //     .suspend = line6_suspend,
    //     .resume = line6_resume,
    //     .reset_resume = line6_resume,
    // #endif
    // .id_table = variax_id_table,
    std::mem::zeroed()
};

// module_usb_driver(variax_driver);

// MODULE_DESCRIPTION("Variax Workbench USB driver");
// MODULE_LICENSE("GPL");

// External kernel functions
extern "C" {
    fn line6_send_raw_message_async(
        line6: *mut usb_line6,
        buffer: *mut u8,
        size: usize,
    );
    fn schedule_delayed_work(work: *mut std::ffi::c_void, delay: usize);
    fn msecs_to_jiffies(ms: usize) -> usize;
    fn line6_version_request_async(line6: *mut usb_line6);
    fn cancel_delayed_work(work: *mut std::ffi::c_void);
    fn snd_card_register(card: *mut std::ffi::c_void);
    fn dev_info(dev: *mut std::ffi::c_void, fmt: *const i8, ...);
    fn memcmp(s1: *const std::ffi::c_void, s2: *const std::ffi::c_void, n: usize) -> i32;
    fn kmemdup(src: *const std::ffi::c_void, size: usize, flags: i32) -> *mut std::ffi::c_void;
    fn kfree(ptr: *mut std::ffi::c_void);
    fn line6_probe(
        interface: *mut usb_interface,
        id: *const usb_device_id,
        name: *const i8,
        properties: *const line6_properties,
        init: unsafe fn(*mut usb_line6, *const usb_device_id) -> i32,
        size: usize,
    ) -> i32;
    fn line6_disconnect(interface: *mut usb_interface);
    fn line6_suspend(interface: *mut usb_interface, message: i32) -> i32;
    fn line6_resume(interface: *mut usb_interface) -> i32;
}

const GFP_KERNEL: i32 = 0x00d0u32 as i32;
const LINE6_RESET: u8 = 0xff;
const LINE6_SYSEX_BEGIN: u8 = 0xf0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
