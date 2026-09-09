// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018 NXP
 *  Author: Dong Aisheng <aisheng.dong@nxp.com>
 *
 * Implementation of the SCU IPC functions using MUs (client side).
 */

// Linux kernel dependencies and symbols are supplied by the surrounding tree.

const SCU_MU_CHAN_NUM: usize = 8;
const MAX_RX_TIMEOUT: usize = msecs_to_jiffies(3000);

#[repr(C)]
pub struct imx_sc_chan {
    pub sc_ipc: *mut imx_sc_ipc,
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub idx: i32,
    pub tx_done: completion,
}

#[repr(C)]
pub struct imx_sc_ipc {
    // SCU uses 4 Tx and 4 Rx channels
    pub chans: [imx_sc_chan; SCU_MU_CHAN_NUM],
    pub dev: *mut device,
    pub lock: mutex,
    pub done: completion,
    pub fast_ipc: bool,
    // temporarily store the SCU msg
    pub msg: *mut u32,
    pub rx_size: u8,
    pub count: u8,
}

#[repr(C)]
pub enum imx_sc_error_codes {
    IMX_SC_ERR_NONE = 0,
    IMX_SC_ERR_VERSION = 1,
    IMX_SC_ERR_CONFIG = 2,
    IMX_SC_ERR_PARM = 3,
    IMX_SC_ERR_NOACCESS = 4,
    IMX_SC_ERR_LOCKED = 5,
    IMX_SC_ERR_UNAVAILABLE = 6,
    IMX_SC_ERR_NOTFOUND = 7,
    IMX_SC_ERR_NOPOWER = 8,
    IMX_SC_ERR_IPC = 9,
    IMX_SC_ERR_BUSY = 10,
    IMX_SC_ERR_FAIL = 11,
    IMX_SC_ERR_LAST,
}

static mut imx_sc_linux_errmap: [i32; 12] = [
    0, -EINVAL, -EINVAL, -EINVAL, -EACCES, -EACCES,
    -ERANGE, -ENOENT, -ENODEV, -ECOMM, -EBUSY, -EIO,
];

static mut imx_sc_ipc_handle: *mut imx_sc_ipc = core::ptr::null_mut();

unsafe fn imx_scu_free_mbox_chan(data: *mut core::ffi::c_void) {
    mbox_free_channel(data);
}

unsafe fn imx_scu_clear_handle(data: *mut core::ffi::c_void) {
    if imx_sc_ipc_handle == data as *mut imx_sc_ipc {
        imx_sc_ipc_handle = core::ptr::null_mut();
    }
}

#[inline]
unsafe fn imx_sc_to_linux_errno(errno: i32) -> i32 {
    if errno >= IMX_SC_ERR_NONE as i32 && errno < IMX_SC_ERR_LAST as i32 {
        return imx_sc_linux_errmap[errno as usize];
    }
    -EIO
}

// Get the default handle used by SCU
pub unsafe fn imx_scu_get_handle(ipc: *mut *mut imx_sc_ipc) -> i32 {
    if imx_sc_ipc_handle.is_null() { return -EPROBE_DEFER; }
    *ipc = imx_sc_ipc_handle;
    0
}

// Callback called when the word of a message is ack-ed, eg read by SCU
unsafe fn imx_scu_tx_done(cl: *mut mbox_client, _mssg: *mut core::ffi::c_void, _r: i32) {
    let sc_chan = container_of!(cl, imx_sc_chan, cl);
    complete(&mut (*sc_chan).tx_done);
}

unsafe fn imx_scu_rx_callback(c: *mut mbox_client, msg: *mut core::ffi::c_void) {
    let sc_chan = container_of!(c, imx_sc_chan, cl);
    let sc_ipc = (*sc_chan).sc_ipc;
    let mut data = msg as *mut u32;
    if (*sc_ipc).msg.is_null() {
        dev_warn((*sc_ipc).dev, "unexpected rx idx %d 0x%08x, ignore!\n", (*sc_chan).idx, *data);
        return;
    }
    if (*sc_ipc).fast_ipc {
        let hdr = msg as *mut imx_sc_rpc_msg;
        (*sc_ipc).rx_size = (*hdr).size;
        *(*sc_ipc).msg = *data; data = data.add(1);
        for i in 1..(*sc_ipc).rx_size { *(*sc_ipc).msg.add(i as usize) = *data; data = data.add(1); }
        complete(&mut (*sc_ipc).done); return;
    }
    if (*sc_chan).idx == 0 {
        let hdr = msg as *mut imx_sc_rpc_msg;
        (*sc_ipc).rx_size = (*hdr).size;
        dev_dbg((*sc_ipc).dev, "msg rx size %u\n", (*sc_ipc).rx_size);
        if (*sc_ipc).rx_size > 4 { dev_warn((*sc_ipc).dev, "RPC does not support receiving over 4 words: %u\n", (*sc_ipc).rx_size); }
    }
    *(*sc_ipc).msg.add((*sc_chan).idx as usize) = *data;
    (*sc_ipc).count = (*sc_ipc).count.wrapping_add(1);
    dev_dbg((*sc_ipc).dev, "mu %u msg %u 0x%x\n", (*sc_chan).idx, (*sc_ipc).count, *data);
    if (*sc_ipc).rx_size != 0 && (*sc_ipc).count == (*sc_ipc).rx_size { complete(&mut (*sc_ipc).done); }
}

unsafe fn imx_scu_ipc_write(sc_ipc: *mut imx_sc_ipc, msg: *mut core::ffi::c_void) -> i32 {
    let hdr = *(msg as *mut imx_sc_rpc_msg);
    if hdr.size > IMX_SC_RPC_MAX_MSG { return -EINVAL; }
    dev_dbg((*sc_ipc).dev, "RPC SVC %u FUNC %u SIZE %u\n", hdr.svc, hdr.func, hdr.size);
    let size = if (*sc_ipc).fast_ipc { 1 } else { hdr.size };
    for i in 0..size {
        let sc_chan = &mut (*sc_ipc).chans[i as usize % 4];
        if !(*sc_ipc).fast_ipc { wait_for_completion(&mut sc_chan.tx_done); reinit_completion(&mut sc_chan.tx_done); }
        let ret = mbox_send_message(sc_chan.ch, (msg as *mut u32).add(i as usize) as *mut core::ffi::c_void);
        if ret < 0 { return ret; }
    }
    0
}

// RPC command/response
pub unsafe fn imx_scu_call_rpc(sc_ipc: *mut imx_sc_ipc, msg: *mut core::ffi::c_void, have_resp: bool) -> i32 {
    if WARN_ON(sc_ipc.is_null() || msg.is_null()) { return -EINVAL; }
    mutex_lock(&mut (*sc_ipc).lock); reinit_completion(&mut (*sc_ipc).done);
    let (saved_svc, saved_func) = if have_resp { (*sc_ipc).msg = msg as *mut u32; let h = msg as *mut imx_sc_rpc_msg; ((*h).svc, (*h).func) } else { (0, 0) };
    (*sc_ipc).count = 0;
    let mut ret = imx_scu_ipc_write(sc_ipc, msg);
    if ret < 0 { dev_err((*sc_ipc).dev, "RPC send msg failed: %d\n", ret); } else if have_resp {
        if wait_for_completion_timeout(&mut (*sc_ipc).done, MAX_RX_TIMEOUT) == 0 { dev_err((*sc_ipc).dev, "RPC send msg timeout\n"); mutex_unlock(&mut (*sc_ipc).lock); return -ETIMEDOUT; }
        ret = (*(msg as *mut imx_sc_rpc_msg)).func;
        if saved_svc == IMX_SC_RPC_SVC_MISC && (saved_func == IMX_SC_MISC_FUNC_UNIQUE_ID || saved_func == IMX_SC_MISC_FUNC_GET_BUTTON_STATUS) { ret = 0; }
    }
    (*sc_ipc).msg = core::ptr::null_mut(); mutex_unlock(&mut (*sc_ipc).lock);
    dev_dbg((*sc_ipc).dev, "RPC SVC done\n"); imx_sc_to_linux_errno(ret)
}

unsafe fn imx_scu_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let sc_ipc = devm_kzalloc(dev, core::mem::size_of::<imx_sc_ipc>(), GFP_KERNEL) as *mut imx_sc_ipc;
    if sc_ipc.is_null() { return -ENOMEM; }
    let mut args = core::mem::zeroed::<of_phandle_args>();
    let mut ret = of_parse_phandle_with_args((*pdev).dev.of_node, "mboxes", "#mbox-cells", 0, &mut args);
    if ret != 0 { return ret; }
    (*sc_ipc).fast_ipc = of_device_is_compatible(args.np, "fsl,imx8-mu-scu");
    of_node_put(args.np);
    let num_channel = if (*sc_ipc).fast_ipc { 2 } else { SCU_MU_CHAN_NUM };
    for i in 0..num_channel {
        let name = if i < num_channel / 2 { kasprintf(GFP_KERNEL, "tx%d", i) } else { kasprintf(GFP_KERNEL, "rx%d", i - num_channel / 2) };
        if name.is_null() { return -ENOMEM; }
        let sc_chan = &mut (*sc_ipc).chans[i];
        let cl = &mut sc_chan.cl;
        cl.dev = dev; cl.tx_block = false; cl.knows_txdone = true; cl.rx_callback = Some(imx_scu_rx_callback);
        if !(*sc_ipc).fast_ipc { cl.tx_done = Some(imx_scu_tx_done); init_completion(&mut sc_chan.tx_done); complete(&mut sc_chan.tx_done); }
        sc_chan.sc_ipc = sc_ipc; sc_chan.idx = (i % (num_channel / 2)) as i32;
        sc_chan.ch = mbox_request_channel_byname(cl, name);
        if IS_ERR(sc_chan.ch) { ret = PTR_ERR(sc_chan.ch); dev_err_probe(dev, ret, "Failed to request mbox chan %s\n", name); kfree(name); return ret; }
        dev_dbg(dev, "request mbox chan %s\n", name); kfree(name);
        ret = devm_add_action_or_reset(dev, Some(imx_scu_free_mbox_chan), sc_chan.ch as *mut core::ffi::c_void);
        if ret != 0 { return ret; }
    }
    (*sc_ipc).dev = dev; ret = devm_mutex_init(dev, &mut (*sc_ipc).lock); if ret != 0 { return ret; }
    init_completion(&mut (*sc_ipc).done); imx_sc_ipc_handle = sc_ipc;
    ret = devm_add_action_or_reset(dev, Some(imx_scu_clear_handle), sc_ipc as *mut core::ffi::c_void); if ret != 0 { return ret; }
    ret = imx_scu_soc_init(dev); if ret != 0 { dev_warn(dev, "failed to initialize SoC info: %d\n", ret); }
    ret = imx_scu_enable_general_irq_channel(dev); if ret != 0 { dev_warn(dev, "failed to enable general irq channel: %d\n", ret); }
    dev_info(dev, "NXP i.MX SCU Initialized\n");
    ret = devm_of_platform_populate(dev); if ret != 0 { of_platform_depopulate(dev); } ret
}

static mut imx_scu_driver: platform_driver = platform_driver { driver: driver { name: "imx-scu", of_match_table: imx_scu_match, suppress_bind_attrs: true }, probe: Some(imx_scu_probe) };
static imx_scu_match: [of_device_id; 2] = [of_device_id { compatible: "fsl,imx-scu" }, of_device_id { sentinel: true }];
unsafe fn imx_scu_driver_init() -> i32 { platform_driver_register(&mut imx_scu_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
