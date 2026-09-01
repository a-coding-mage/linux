// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017-2020 NXP

// Rust translation of includes:
// <linux/module.h>
// <linux/rpmsg.h>
// "imx-pcm-rpmsg.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = c_uint;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const PLATFORM_DEVID_NONE: c_int = -1;
const PLATFORM_DEVID_AUTO: c_int = -2;

const MSG_TYPE_C: c_int = 0;
const MSG_TYPE_B: c_int = 0;
const MSG_TYPE_A_NUM: c_int = 0;
const TX_PERIOD_DONE: c_int = 0;
const RX_PERIOD_DONE: c_int = 0;
const TX: usize = 0;
const RX: usize = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rpmsg_device_id {
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct rpmsg_device {
    pub dev: device,
    pub src: u32,
    pub dst: u32,
    pub id: rpmsg_device_id,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct rpmsg_driver {
    pub drv: driver,
    pub id_table: *mut rpmsg_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut rpmsg_device) -> c_int>,
    pub callback:
        Option<unsafe extern "C" fn(*mut rpmsg_device, *mut c_void, c_int, *mut c_void, u32) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut rpmsg_device)>,
}

#[repr(C)]
pub struct rpmsg_header {
    pub cmd: c_int,
    pub type_: c_int,
}

#[repr(C)]
pub struct rpmsg_param {
    pub resp: c_int,
    pub buffer_tail: c_int,
}

#[repr(C)]
pub struct rpmsg_r_msg {
    pub header: rpmsg_header,
    pub param: rpmsg_param,
}

#[repr(C)]
pub struct rpmsg_msg {
    pub r_msg: rpmsg_r_msg,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rpmsg_info {
    pub lock: [spinlock_t; 2],
    pub msg: [rpmsg_msg; 0],
    pub num_period: [c_int; 2],
    pub callback: [Option<unsafe extern "C" fn(*mut c_void)>; 2],
    pub callback_param: [*mut c_void; 2],
    pub r_msg: rpmsg_r_msg,
    pub cmd_complete: completion,
}

/*
 * struct imx_audio_rpmsg: private data
 *
 * @rpmsg_pdev: pointer of platform device
 */
#[repr(C)]
pub struct imx_audio_rpmsg {
    pub rpmsg_pdev: *mut platform_device,
    pub card_pdev: *mut platform_device,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn complete(x: *mut completion);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" fn imx_audio_rpmsg_cb(
    rpdev: *mut rpmsg_device,
    data: *mut c_void,
    _len: c_int,
    _priv: *mut c_void,
    src: u32,
) -> c_int {
    let rpmsg = dev_get_drvdata(ptr::addr_of_mut!((*rpdev).dev)) as *mut imx_audio_rpmsg;
    let r_msg = data as *mut rpmsg_r_msg;
    let mut msg: *mut rpmsg_msg;
    let info: *mut rpmsg_info;

    if (*rpmsg).rpmsg_pdev.is_null() {
        return 0;
    }

    info = platform_get_drvdata((*rpmsg).rpmsg_pdev) as *mut rpmsg_info;

    dev_dbg(
        ptr::addr_of_mut!((*rpdev).dev),
        b"get from%d: cmd:%d. %d\n\0".as_ptr() as *const c_char,
        src,
        (*r_msg).header.cmd,
        (*r_msg).param.resp,
    );

    match (*r_msg).header.type_ {
        MSG_TYPE_C => {
            /* TYPE C is notification from M core */
            match (*r_msg).header.cmd {
                TX_PERIOD_DONE => {
                    let mut flags: c_ulong = 0;
                    spin_lock_irqsave(ptr::addr_of_mut!((*info).lock[TX]), &mut flags);
                    msg = ptr::addr_of_mut!(
                        (*info).msg[(TX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize]
                    );
                    (*msg).r_msg.param.buffer_tail = (*r_msg).param.buffer_tail;
                    (*msg).r_msg.param.buffer_tail %= (*info).num_period[TX];
                    spin_unlock_irqrestore(ptr::addr_of_mut!((*info).lock[TX]), flags);
                    if let Some(callback) = (*info).callback[TX] {
                        callback((*info).callback_param[TX]);
                    }
                }
                RX_PERIOD_DONE => {
                    let mut flags: c_ulong = 0;
                    spin_lock_irqsave(ptr::addr_of_mut!((*info).lock[RX]), &mut flags);
                    msg = ptr::addr_of_mut!(
                        (*info).msg[(RX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize]
                    );
                    (*msg).r_msg.param.buffer_tail = (*r_msg).param.buffer_tail;
                    (*msg).r_msg.param.buffer_tail %= (*info).num_period[1];
                    spin_unlock_irqrestore(ptr::addr_of_mut!((*info).lock[RX]), flags);
                    if let Some(callback) = (*info).callback[RX] {
                        callback((*info).callback_param[RX]);
                    }
                }
                _ => {
                    dev_warn(
                        ptr::addr_of_mut!((*rpdev).dev),
                        b"unknown msg command\n\0".as_ptr() as *const c_char,
                    );
                }
            }
        }
        MSG_TYPE_B => {
            /* TYPE B is response msg */
            memcpy(
                ptr::addr_of_mut!((*info).r_msg) as *mut c_void,
                r_msg as *const c_void,
                size_of::<rpmsg_r_msg>(),
            );
            complete(ptr::addr_of_mut!((*info).cmd_complete));
        }
        _ => {
            dev_warn(
                ptr::addr_of_mut!((*rpdev).dev),
                b"unknown msg type\n\0".as_ptr() as *const c_char,
            );
        }
    }

    0
}

unsafe extern "C" fn imx_audio_rpmsg_probe(rpdev: *mut rpmsg_device) -> c_int {
    let data: *mut imx_audio_rpmsg;
    let mut ret: c_int = 0;

    dev_info(
        ptr::addr_of_mut!((*rpdev).dev),
        b"new channel: 0x%x -> 0x%x!\n\0".as_ptr() as *const c_char,
        (*rpdev).src,
        (*rpdev).dst,
    );

    data = devm_kzalloc(
        ptr::addr_of_mut!((*rpdev).dev),
        size_of::<imx_audio_rpmsg>(),
        GFP_KERNEL,
    ) as *mut imx_audio_rpmsg;
    if data.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(ptr::addr_of_mut!((*rpdev).dev), data as *mut c_void);

    /* Register platform driver for rpmsg routine */
    (*data).rpmsg_pdev = platform_device_register_data(
        ptr::addr_of_mut!((*rpdev).dev),
        (*rpdev).id.name.as_ptr(),
        PLATFORM_DEVID_NONE,
        ptr::null(),
        0,
    );
    if IS_ERR((*data).rpmsg_pdev as *const c_void) {
        dev_err(
            ptr::addr_of_mut!((*rpdev).dev),
            b"failed to register rpmsg platform.\n\0".as_ptr() as *const c_char,
        );
        ret = PTR_ERR((*data).rpmsg_pdev as *const c_void);
    }

    (*data).card_pdev = platform_device_register_data(
        ptr::addr_of_mut!((*rpdev).dev),
        b"imx-audio-rpmsg\0".as_ptr() as *const c_char,
        PLATFORM_DEVID_AUTO,
        (*rpdev).id.name.as_ptr() as *const c_void,
        strlen((*rpdev).id.name.as_ptr()) + 1,
    );
    if IS_ERR((*data).card_pdev as *const c_void) {
        dev_err(
            ptr::addr_of_mut!((*rpdev).dev),
            b"failed to register rpmsg card.\n\0".as_ptr() as *const c_char,
        );
        ret = PTR_ERR((*data).card_pdev as *const c_void);
    }

    ret
}

unsafe extern "C" fn imx_audio_rpmsg_remove(rpdev: *mut rpmsg_device) {
    let data = dev_get_drvdata(ptr::addr_of_mut!((*rpdev).dev)) as *mut imx_audio_rpmsg;

    if !(*data).rpmsg_pdev.is_null() {
        platform_device_unregister((*data).rpmsg_pdev);
    }

    if !(*data).card_pdev.is_null() {
        platform_device_unregister((*data).card_pdev);
    }

    dev_info(
        ptr::addr_of_mut!((*rpdev).dev),
        b"audio rpmsg driver is removed\n\0".as_ptr() as *const c_char,
    );
}

static mut IMX_AUDIO_RPMSG_ID_TABLE: [rpmsg_device_id; 3] = [
    rpmsg_device_id {
        name: [
            b'r' as c_char,
            b'p' as c_char,
            b'm' as c_char,
            b's' as c_char,
            b'g' as c_char,
            b'-' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'd' as c_char,
            b'i' as c_char,
            b'o' as c_char,
            b'-' as c_char,
            b'c' as c_char,
            b'h' as c_char,
            b'a' as c_char,
            b'n' as c_char,
            b'n' as c_char,
            b'e' as c_char,
            b'l' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    rpmsg_device_id {
        name: [
            b'r' as c_char,
            b'p' as c_char,
            b'm' as c_char,
            b's' as c_char,
            b'g' as c_char,
            b'-' as c_char,
            b'm' as c_char,
            b'i' as c_char,
            b'c' as c_char,
            b'f' as c_char,
            b'i' as c_char,
            b'l' as c_char,
            b'-' as c_char,
            b'c' as c_char,
            b'h' as c_char,
            b'a' as c_char,
            b'n' as c_char,
            b'n' as c_char,
            b'e' as c_char,
            b'l' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    rpmsg_device_id { name: [0; 32] },
];
// MODULE_DEVICE_TABLE(rpmsg, imx_audio_rpmsg_id_table);

static mut IMX_AUDIO_RPMSG_DRIVER: rpmsg_driver = rpmsg_driver {
    drv: driver {
        name: b"imx_audio_rpmsg\0".as_ptr() as *const c_char,
    },
    id_table: unsafe { IMX_AUDIO_RPMSG_ID_TABLE.as_mut_ptr() },
    probe: Some(imx_audio_rpmsg_probe),
    callback: Some(imx_audio_rpmsg_cb),
    remove: Some(imx_audio_rpmsg_remove),
};

// module_rpmsg_driver(imx_audio_rpmsg_driver);

// MODULE_DESCRIPTION("Freescale SoC Audio RPMSG interface");
// MODULE_AUTHOR("Shengjiu Wang <shengjiu.wang@nxp.com>");
// MODULE_ALIAS("rpmsg:imx_audio_rpmsg");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
