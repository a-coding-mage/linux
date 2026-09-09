// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Alibaba Group Holding Limited.
 * Copyright (c) 2024 Samsung Electronics Co., Ltd.
 * Author: Michal Wilczynski <m.wilczynski@samsung.com>
 */

// External kernel and TH1520 protocol types, constants, and functions are
// supplied by the surrounding Rust environment.

const MAX_RX_TIMEOUT: u64 = msecs_to_jiffies(3000);
const MAX_TX_TIMEOUT: u32 = 500;

#[repr(C)]
pub struct th1520_aon_chan {
    pub ch: *mut mbox_chan,
    pub ack_msg: th1520_aon_rpc_ack_common,
    pub cl: mbox_client,
    pub done: completion,
    // make sure only one RPC is performed at a time
    pub transaction_lock: mutex,
}

#[repr(C, packed(1))]
pub struct th1520_aon_msg_req_set_resource_power_mode {
    pub hdr: th1520_aon_rpc_msg_hdr,
    pub resource: u16,
    pub mode: u16,
    pub reserved: [u16; 10],
}

/*
 * This type is used to indicate error response for most functions.
 */
#[repr(C)]
pub enum th1520_aon_error_codes {
    LIGHT_AON_ERR_NONE = 0, // Success
    LIGHT_AON_ERR_VERSION = 1, // Incompatible API version
    LIGHT_AON_ERR_CONFIG = 2, // Configuration error
    LIGHT_AON_ERR_PARM = 3, // Bad parameter
    LIGHT_AON_ERR_NOACCESS = 4, // Permission error (no access)
    LIGHT_AON_ERR_LOCKED = 5, // Permission error (locked)
    LIGHT_AON_ERR_UNAVAILABLE = 6, // Unavailable (out of resources)
    LIGHT_AON_ERR_NOTFOUND = 7, // Not found
    LIGHT_AON_ERR_NOPOWER = 8, // No power
    LIGHT_AON_ERR_IPC = 9, // Generic IPC error
    LIGHT_AON_ERR_BUSY = 10, // Resource is currently busy/active
    LIGHT_AON_ERR_FAIL = 11, // General I/O failure
    LIGHT_AON_ERR_LAST,
}

static mut th1520_aon_linux_errmap: [i32; LIGHT_AON_ERR_LAST as usize] = [
    0, // LIGHT_AON_ERR_NONE
    -EINVAL, // LIGHT_AON_ERR_VERSION
    -EINVAL, // LIGHT_AON_ERR_CONFIG
    -EINVAL, // LIGHT_AON_ERR_PARM
    -EACCES, // LIGHT_AON_ERR_NOACCESS
    -EACCES, // LIGHT_AON_ERR_LOCKED
    -ERANGE, // LIGHT_AON_ERR_UNAVAILABLE
    -EEXIST, // LIGHT_AON_ERR_NOTFOUND
    -EPERM, // LIGHT_AON_ERR_NOPOWER
    -EPIPE, // LIGHT_AON_ERR_IPC
    -EBUSY, // LIGHT_AON_ERR_BUSY
    -EIO, // LIGHT_AON_ERR_FAIL
];

#[inline]
fn th1520_aon_to_linux_errno(errno: i32) -> i32 {
    if errno >= LIGHT_AON_ERR_NONE as i32 && errno < LIGHT_AON_ERR_LAST as i32 {
        unsafe { th1520_aon_linux_errmap[errno as usize] }
    } else {
        -EIO
    }
}

unsafe extern "C" fn th1520_aon_rx_callback(c: *mut mbox_client, rx_msg: *mut core::ffi::c_void) {
    let aon_chan = container_of!(c, th1520_aon_chan, cl);
    let hdr = rx_msg as *mut th1520_aon_rpc_msg_hdr;
    let recv_size = core::mem::size_of::<th1520_aon_rpc_msg_hdr>() + (*hdr).size as usize;

    if recv_size != core::mem::size_of::<th1520_aon_rpc_ack_common>() {
        dev_err!((*c).dev, "Invalid ack size, not completing\n");
        return;
    }

    core::ptr::copy_nonoverlapping(
        rx_msg as *const u8,
        &mut (*aon_chan).ack_msg as *mut _ as *mut u8,
        recv_size,
    );
    complete(&mut (*aon_chan).done);
}

/// th1520_aon_call_rpc() - Send an RPC request to the TH1520 AON subsystem
/// @aon_chan: Pointer to the AON channel structure
/// @msg: Pointer to the message (RPC payload) that will be sent
pub unsafe fn th1520_aon_call_rpc(aon_chan: *mut th1520_aon_chan, msg: *mut core::ffi::c_void) -> i32 {
    let hdr = msg as *mut th1520_aon_rpc_msg_hdr;
    mutex_lock(&mut (*aon_chan).transaction_lock);
    reinit_completion(&mut (*aon_chan).done);

    RPC_SET_VER!(hdr, TH1520_AON_RPC_VERSION);
    RPC_SET_SVC_ID!(hdr, (*hdr).svc);
    RPC_SET_SVC_FLAG_MSG_TYPE!(hdr, RPC_SVC_MSG_TYPE_DATA);
    RPC_SET_SVC_FLAG_ACK_TYPE!(hdr, RPC_SVC_MSG_NEED_ACK);

    let mut ret = mbox_send_message((*aon_chan).ch, msg);
    if ret < 0 {
        dev_err!((*aon_chan).cl.dev, "RPC send msg failed: %d\n", ret);
    } else if !wait_for_completion_timeout(&mut (*aon_chan).done, MAX_RX_TIMEOUT) {
        dev_err!((*aon_chan).cl.dev, "RPC send msg timeout\n");
        mutex_unlock(&mut (*aon_chan).transaction_lock);
        return -ETIMEDOUT;
    } else {
        ret = (*aon_chan).ack_msg.err_code as i32;
    }

    mutex_unlock(&mut (*aon_chan).transaction_lock);
    th1520_aon_to_linux_errno(ret)
}

/// th1520_aon_power_update() - Change power state of a resource via TH1520 AON
pub unsafe fn th1520_aon_power_update(aon_chan: *mut th1520_aon_chan, rsrc: u16, power_on: bool) -> i32 {
    let mut msg: th1520_aon_msg_req_set_resource_power_mode = core::mem::zeroed();
    msg.hdr.svc = TH1520_AON_RPC_SVC_PM;
    msg.hdr.func = TH1520_AON_PM_FUNC_SET_RESOURCE_POWER_MODE;
    msg.hdr.size = TH1520_AON_RPC_MSG_NUM;
    msg.resource = cpu_to_be16(rsrc);
    msg.mode = cpu_to_be16(if power_on { TH1520_AON_PM_PW_MODE_ON } else { TH1520_AON_PM_PW_MODE_OFF });

    let ret = th1520_aon_call_rpc(aon_chan, &mut msg as *mut _ as *mut core::ffi::c_void);
    if ret != 0 {
        dev_err!((*aon_chan).cl.dev, "failed to power %s resource %d ret %d\n", if power_on { "up" } else { "off" }, rsrc, ret);
    }
    ret
}

/// th1520_aon_init() - Initialize TH1520 AON firmware protocol interface
pub unsafe fn th1520_aon_init(dev: *mut device) -> *mut th1520_aon_chan {
    let aon_chan = kzalloc_obj::<th1520_aon_chan>();
    if aon_chan.is_null() { return ERR_PTR(-ENOMEM); }
    let cl = &mut (*aon_chan).cl;
    (*cl).dev = dev;
    (*cl).tx_block = true;
    (*cl).tx_tout = MAX_TX_TIMEOUT;
    (*cl).rx_callback = Some(th1520_aon_rx_callback);
    (*aon_chan).ch = mbox_request_channel_byname(cl, "aon\0".as_ptr() as *const i8);
    if IS_ERR((*aon_chan).ch) {
        dev_err!(dev, "Failed to request aon mbox chan\n");
        let ret = PTR_ERR((*aon_chan).ch);
        kfree(aon_chan);
        return ERR_PTR(ret);
    }
    mutex_init(&mut (*aon_chan).transaction_lock);
    init_completion(&mut (*aon_chan).done);
    aon_chan
}

/// th1520_aon_deinit() - Clean up TH1520 AON firmware protocol interface
pub unsafe fn th1520_aon_deinit(aon_chan: *mut th1520_aon_chan) {
    mbox_free_channel((*aon_chan).ch);
    kfree(aon_chan);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
