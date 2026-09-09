// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 MediaTek Corporation. All rights reserved.
 * Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
 */

// Dependencies supplied by the Linux kernel headers are intentionally external.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn mbox_send_message(ch: *mut mbox_chan, msg: *const c_void) -> c_int;
    fn mbox_request_channel_byname(cl: *mut mbox_client, name: *const c_char) -> *mut mbox_chan;
    fn mbox_free_channel(ch: *mut mbox_chan);
    fn device_set_of_node_from_dev(dev: *mut device, parent: *mut device);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const MTK_ADSP_MBOX_REPLY: usize = 0;
const MTK_ADSP_MBOX_REQUEST: usize = 1;
const MTK_ADSP_MBOX_NUM: usize = 2;

#[repr(C)]
pub struct mbox_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct mbox_client {
    pub dev: *mut device,
    pub tx_block: bool,
    pub knows_txdone: bool,
    pub tx_prepare: Option<unsafe extern "C" fn(*mut mbox_client, *mut c_void)>,
    pub rx_callback: Option<unsafe extern "C" fn(*mut mbox_client, *mut c_void)>,
}

#[repr(C)]
pub struct mtk_adsp_ipc_ops {
    pub handle_reply: unsafe extern "C" fn(*mut mtk_adsp_ipc),
    pub handle_request: unsafe extern "C" fn(*mut mtk_adsp_ipc),
}

#[repr(C)]
pub struct mtk_adsp_chan {
    pub cl: mbox_client,
    pub ipc: *mut mtk_adsp_ipc,
    pub idx: usize,
    pub ch: *mut mbox_chan,
}

#[repr(C)]
pub struct mtk_adsp_ipc {
    pub chans: [mtk_adsp_chan; MTK_ADSP_MBOX_NUM],
    pub dev: *mut device,
    pub ops: *mut mtk_adsp_ipc_ops,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

static ADSP_MBOX_CH_NAMES: [&[u8]; MTK_ADSP_MBOX_NUM] = [b"rx\0", b"tx\0"];

/// mtk_adsp_ipc_send - send ipc cmd to MTK ADSP
pub unsafe extern "C" fn mtk_adsp_ipc_send(
    ipc: *mut mtk_adsp_ipc,
    idx: c_uint,
    msg: u32,
) -> c_int {
    if idx as usize >= MTK_ADSP_MBOX_NUM {
        return -EINVAL;
    }

    let adsp_chan = &mut (*ipc).chans[idx as usize];
    let ret = mbox_send_message(adsp_chan.ch, (&msg as *const u32).cast());
    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn mtk_adsp_ipc_recv(c: *mut mbox_client, _msg: *mut c_void) {
    // container_of(c, struct mtk_adsp_chan, cl)
    let chan = (c as *mut u8).sub(core::mem::offset_of!(mtk_adsp_chan, cl))
        as *mut mtk_adsp_chan;
    match (*chan).idx {
        MTK_ADSP_MBOX_REPLY => ((*(*chan).ipc).ops).as_ref().unwrap().handle_reply((*chan).ipc),
        MTK_ADSP_MBOX_REQUEST => ((*(*chan).ipc).ops).as_ref().unwrap().handle_request((*chan).ipc),
        _ => { /* dev_err(dev, "wrong mbox chan %d\n", chan->idx); */ }
    }
}

// The platform-driver registration and kernel logging macros are supplied by
// the surrounding kernel build; preserve their source-level intent here.
unsafe extern "C" fn mtk_adsp_ipc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    device_set_of_node_from_dev(dev, (*dev).parent);
    let adsp_ipc = devm_kzalloc(dev, core::mem::size_of::<mtk_adsp_ipc>(), GFP_KERNEL)
        as *mut mtk_adsp_ipc;
    if adsp_ipc.is_null() {
        return -ENOMEM;
    }

    for i in 0..MTK_ADSP_MBOX_NUM {
        let adsp_chan = &mut (*adsp_ipc).chans[i];
        let cl = &mut adsp_chan.cl;
        cl.dev = (*dev).parent;
        cl.tx_block = false;
        cl.knows_txdone = false;
        cl.tx_prepare = None;
        cl.rx_callback = Some(mtk_adsp_ipc_recv);
        adsp_chan.ipc = adsp_ipc;
        adsp_chan.idx = i;
        adsp_chan.ch = mbox_request_channel_byname(
            cl,
            ADSP_MBOX_CH_NAMES[i].as_ptr() as *const c_char,
        );
        if adsp_chan.ch.is_null() {
            for j in 0..i {
                mbox_free_channel((*adsp_ipc).chans[j].ch);
            }
            return -EINVAL;
        }
    }
    (*adsp_ipc).dev = dev;
    dev_set_drvdata(dev, adsp_ipc.cast());
    0
}

unsafe extern "C" fn mtk_adsp_ipc_remove(pdev: *mut platform_device) {
    let adsp_ipc = dev_get_drvdata(&mut (*pdev).dev) as *mut mtk_adsp_ipc;
    for i in 0..MTK_ADSP_MBOX_NUM {
        mbox_free_channel((*adsp_ipc).chans[i].ch);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
