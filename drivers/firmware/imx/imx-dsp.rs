// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2019 NXP
 *  Author: Daniel Baluta <daniel.baluta@nxp.com>
 *
 * Implementation of the DSP IPC interface (host side)
 */

// Dependencies and types are supplied by the corresponding kernel bindings.

extern "C" {
    fn mbox_send_message(chan: *mut mbox_chan, msg: *mut core::ffi::c_void) -> i32;
    fn mbox_request_channel_byname(cl: *mut mbox_client, name: *mut core::ffi::c_char) -> *mut mbox_chan;
    fn mbox_free_channel(chan: *mut mbox_chan);
    fn kasprintf(gfp: u32, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut core::ffi::c_void;
    fn device_set_of_node_from_dev(dev: *mut device, parent: *mut device);
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
}

// These declarations correspond to types and symbols provided by the kernel headers.
const DSP_MU_CHAN_NUM: usize = 4;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EPROBE_DEFER: i32 = 517;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
pub struct mbox_chan { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct mbox_client {
    pub dev: *mut device,
    pub tx_block: bool,
    pub knows_txdone: bool,
    pub rx_callback: Option<unsafe extern "C" fn(*mut mbox_client, *mut core::ffi::c_void)>,
}
#[repr(C)]
pub struct imx_dsp_ops {
    pub handle_reply: unsafe extern "C" fn(*mut imx_dsp_ipc),
    pub handle_request: unsafe extern "C" fn(*mut imx_dsp_ipc),
}
#[repr(C)]
pub struct imx_dsp_chan {
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub name: *mut core::ffi::c_char,
    pub ipc: *mut imx_dsp_ipc,
    pub idx: usize,
}
#[repr(C)]
pub struct imx_dsp_ipc {
    pub dev: *mut device,
    pub chans: [imx_dsp_chan; DSP_MU_CHAN_NUM],
    pub ops: *mut imx_dsp_ops,
}

/*
 * imx_dsp_ring_doorbell - triggers an interrupt on the other side (DSP)
 *
 * @dsp: DSP IPC handle
 * @chan_idx: index of the channel where to trigger the interrupt
 *
 * Returns non-negative value for success, negative value for error
 */
#[no_mangle]
pub unsafe extern "C" fn imx_dsp_ring_doorbell(ipc: *mut imx_dsp_ipc, idx: u32) -> i32 {
    if idx >= DSP_MU_CHAN_NUM as u32 { return -EINVAL; }
    let dsp_chan = &mut (*ipc).chans[idx as usize];
    let ret = mbox_send_message(dsp_chan.ch, core::ptr::null_mut());
    if ret < 0 { return ret; }
    0
}

/*
 * imx_dsp_handle_rx - rx callback used by imx mailbox
 *
 * @c: mbox client
 * @msg: message received
 *
 * Users of DSP IPC will need to privde handle_reply and handle_request
 * callbacks.
 */
unsafe extern "C" fn imx_dsp_handle_rx(c: *mut mbox_client, _msg: *mut core::ffi::c_void) {
    // container_of(c, struct imx_dsp_chan, cl); the enclosing layout is supplied by the bindings.
    let chan = c as *mut imx_dsp_chan;
    if (*chan).idx == 0 {
        ((*(*chan).ipc).ops).as_ref().unwrap().handle_reply((*chan).ipc);
    } else {
        ((*(*chan).ipc).ops).as_ref().unwrap().handle_request((*chan).ipc);
        imx_dsp_ring_doorbell((*chan).ipc, 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn imx_dsp_request_channel(dsp_ipc: *mut imx_dsp_ipc, idx: i32) -> *mut mbox_chan {
    if idx >= DSP_MU_CHAN_NUM as i32 { return (-EINVAL as isize) as *mut mbox_chan; }
    let dsp_chan = &mut (*dsp_ipc).chans[idx as usize];
    dsp_chan.ch = mbox_request_channel_byname(&mut dsp_chan.cl, dsp_chan.name);
    dsp_chan.ch
}

#[no_mangle]
pub unsafe extern "C" fn imx_dsp_free_channel(dsp_ipc: *mut imx_dsp_ipc, idx: i32) {
    if idx >= DSP_MU_CHAN_NUM as i32 { return; }
    mbox_free_channel((*dsp_ipc).chans[idx as usize].ch);
}

unsafe fn imx_dsp_setup_channels(dsp_ipc: *mut imx_dsp_ipc) -> i32 {
    let dev = (*dsp_ipc).dev;
    let mut i = 0usize;
    while i < DSP_MU_CHAN_NUM {
        let fmt = if i < 2 { b"txdb%d\0" } else { b"rxdb%d\0" };
        let n = if i < 2 { i } else { i - 2 };
        let chan_name = kasprintf(GFP_KERNEL, fmt.as_ptr() as *const _, n as i32);
        if chan_name.is_null() { return -ENOMEM; }
        let dsp_chan = &mut (*dsp_ipc).chans[i];
        dsp_chan.name = chan_name;
        dsp_chan.cl.dev = dev;
        dsp_chan.cl.tx_block = false;
        dsp_chan.cl.knows_txdone = true;
        dsp_chan.cl.rx_callback = Some(imx_dsp_handle_rx);
        dsp_chan.ipc = dsp_ipc;
        dsp_chan.idx = i % 2;
        dsp_chan.ch = mbox_request_channel_byname(&mut dsp_chan.cl, chan_name);
        if (dsp_chan.ch as isize) < 0 {
            let ret = dsp_chan.ch as isize as i32;
            kfree(dsp_chan.name as *mut _);
            if ret != -EPROBE_DEFER { /* dev_err(dev, "Failed to request mbox chan %s ret %d\n", chan_name, ret); */ }
            let mut j = 0usize;
            while j < i {
                mbox_free_channel((*dsp_ipc).chans[j].ch);
                kfree((*dsp_ipc).chans[j].name as *mut _);
                j += 1;
            }
            return ret;
        }
        // dev_dbg(dev, "request mbox chan %s\n", chan_name);
        i += 1;
    }
    0
}

unsafe extern "C" fn imx_dsp_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    device_set_of_node_from_dev(dev, (*pdev).dev.parent);
    let dsp_ipc = devm_kzalloc(dev, core::mem::size_of::<imx_dsp_ipc>(), GFP_KERNEL) as *mut imx_dsp_ipc;
    if dsp_ipc.is_null() { return -ENOMEM; }
    (*dsp_ipc).dev = dev;
    dev_set_drvdata(dev, dsp_ipc as *mut _);
    let ret = imx_dsp_setup_channels(dsp_ipc);
    if ret < 0 { return ret; }
    // dev_info(dev, "NXP i.MX DSP IPC initialized\n");
    0
}

unsafe extern "C" fn imx_dsp_remove(pdev: *mut platform_device) {
    let dsp_ipc = dev_get_drvdata(&mut (*pdev).dev) as *mut imx_dsp_ipc;
    for i in 0..DSP_MU_CHAN_NUM {
        mbox_free_channel((*dsp_ipc).chans[i].ch);
        kfree((*dsp_ipc).chans[i].name as *mut _);
    }
}

// static struct platform_driver imx_dsp_driver = { .driver.name = "imx-dsp", .probe = imx_dsp_probe, .remove = imx_dsp_remove };
// builtin_platform_driver(imx_dsp_driver);
// MODULE_AUTHOR("Daniel Baluta <daniel.baluta@nxp.com");
// MODULE_DESCRIPTION("IMX DSP IPC protocol driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
