// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2019-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//
// Code moved to this file by:
//  Jyri Sarha <jyri.sarha@intel.com>
//

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

use crate::*;

#[repr(C, packed)]
struct sof_probe_dma {
    stream_tag: u32,
    dma_buffer_size: u32,
}

#[repr(C, packed)]
struct sof_ipc_probe_dma_add_params {
    hdr: sof_ipc_cmd_hdr,
    num_elems: u32,
    dma: [sof_probe_dma; 0],
}

#[repr(C)]
union sof_ipc_probe_info_params_data {
    dma: [sof_probe_dma; 0],
    desc: [sof_probe_point_desc; 0],
}

#[repr(C, packed)]
struct sof_ipc_probe_info_params {
    rhdr: sof_ipc_reply,
    num_elems: u32,
    data: sof_ipc_probe_info_params_data,
}

#[repr(C, packed)]
struct sof_ipc_probe_point_add_params {
    hdr: sof_ipc_cmd_hdr,
    num_elems: u32,
    desc: [sof_probe_point_desc; 0],
}

#[repr(C, packed)]
struct sof_ipc_probe_point_remove_params {
    hdr: sof_ipc_cmd_hdr,
    num_elems: u32,
    buffer_id: [u32; 0],
}

/**
 * ipc3_probes_init - initialize data probing
 * @cdev:		SOF client device
 * @stream_tag:		Extractor stream tag
 * @buffer_size:	DMA buffer size to set for extractor
 *
 * Host chooses whether extraction is supported or not by providing
 * valid stream tag to DSP. Once specified, stream described by that
 * tag will be tied to DSP for extraction for the entire lifetime of
 * probe.
 *
 * Probing is initialized only once and each INIT request must be
 * matched by DEINIT call.
 */
unsafe extern "C" fn ipc3_probes_init(
    cdev: *mut sof_client_dev,
    stream_tag: u32,
    buffer_size: usize,
) -> i32 {
    let msg: *mut sof_ipc_probe_dma_add_params;
    let size: usize = size_of::<sof_ipc_probe_dma_add_params>() + size_of::<sof_probe_dma>();
    let ret: i32;

    msg = kmalloc(size, GFP_KERNEL) as *mut sof_ipc_probe_dma_add_params;
    if msg.is_null() {
        return -ENOMEM;
    }
    ptr::addr_of_mut!((*msg).hdr.size).write_unaligned(size as _);
    ptr::addr_of_mut!((*msg).hdr.cmd).write_unaligned(SOF_IPC_GLB_PROBE | SOF_IPC_PROBE_INIT);
    ptr::addr_of_mut!((*msg).num_elems).write_unaligned(1);
    let dma = (msg as *mut u8).add(offset_of!(sof_ipc_probe_dma_add_params, dma))
        as *mut sof_probe_dma;
    ptr::addr_of_mut!((*dma).stream_tag).write_unaligned(stream_tag);
    ptr::addr_of_mut!((*dma).dma_buffer_size).write_unaligned(buffer_size as _);

    ret = sof_client_ipc_tx_message_no_reply(cdev, msg as *mut c_void);
    kfree(msg as *const c_void);
    ret
}

/**
 * ipc3_probes_deinit - cleanup after data probing
 * @cdev:		SOF client device
 *
 * Host sends DEINIT request to free previously initialized probe
 * on DSP side once it is no longer needed. DEINIT only when there
 * are no probes connected and with all injectors detached.
 */
unsafe extern "C" fn ipc3_probes_deinit(cdev: *mut sof_client_dev) -> i32 {
    let mut msg: sof_ipc_cmd_hdr = core::mem::zeroed();

    msg.size = size_of::<sof_ipc_cmd_hdr>() as _;
    msg.cmd = SOF_IPC_GLB_PROBE | SOF_IPC_PROBE_DEINIT;

    sof_client_ipc_tx_message_no_reply(cdev, &mut msg as *mut _ as *mut c_void)
}

unsafe extern "C" fn ipc3_probes_info(
    cdev: *mut sof_client_dev,
    cmd: u32,
    params: *mut *mut c_void,
    num_params: *mut usize,
    type_: sof_probe_info_type,
) -> i32 {
    let max_msg_size: usize = sof_client_get_ipc_max_payload_size(cdev);
    let dev: *mut device = ptr::addr_of_mut!((*cdev).auxdev.dev);
    let mut msg: sof_ipc_probe_info_params = core::mem::zeroed();
    let reply: *mut sof_ipc_probe_info_params;
    let bytes: usize;
    let elem_size: usize;
    let mut payload_size: usize;
    let mut ret: i32;

    *params = ptr::null_mut();
    *num_params = 0;

    if type_ != PROBES_INFO_ACTIVE_PROBES {
        dev_err(
            dev,
            c"%s: info type %u not supported".as_ptr(),
            c"ipc3_probes_info".as_ptr(),
            type_,
        );
        return -EOPNOTSUPP;
    }

    reply = kzalloc(max_msg_size, GFP_KERNEL) as *mut sof_ipc_probe_info_params;
    if reply.is_null() {
        return -ENOMEM;
    }
    msg.rhdr.hdr.size = size_of::<sof_ipc_probe_info_params>() as _;
    msg.rhdr.hdr.cmd = SOF_IPC_GLB_PROBE | cmd;

    ret = sof_client_ipc_tx_message(
        cdev,
        &mut msg as *mut _ as *mut c_void,
        reply as *mut c_void,
        max_msg_size,
    );
    if ret < 0 || (*reply).rhdr.error < 0 {
        kfree(reply as *const c_void);
        return ret;
    }

    payload_size = (*reply).rhdr.hdr.size as usize;
    if payload_size < offset_of!(sof_ipc_probe_info_params, data) {
        ret = -EINVAL;
        kfree(reply as *const c_void);
        return ret;
    }

    if (*reply).num_elems == 0 {
        kfree(reply as *const c_void);
        return ret;
    }

    if cmd == SOF_IPC_PROBE_DMA_INFO {
        elem_size = size_of::<sof_probe_dma>();
    } else {
        elem_size = size_of::<sof_probe_point_desc>();
    }

    payload_size -= offset_of!(sof_ipc_probe_info_params, data);
    if ((*reply).num_elems as usize) > payload_size / elem_size {
        dev_err(
            dev,
            c"%s: invalid probe info element count %u\n".as_ptr(),
            c"ipc3_probes_info".as_ptr(),
            (*reply).num_elems,
        );
        ret = -EINVAL;
        kfree(reply as *const c_void);
        return ret;
    }

    bytes = (*reply).num_elems as usize * elem_size;
    *params = kmemdup(
        (reply as *const u8).add(offset_of!(sof_ipc_probe_info_params, data)) as *const c_void,
        bytes,
        GFP_KERNEL,
    ) as *mut c_void;
    if (*params).is_null() {
        ret = -ENOMEM;
        kfree(reply as *const c_void);
        return ret;
    }
    *num_params = (*reply).num_elems as usize;

    kfree(reply as *const c_void);
    ret
}

/**
 * ipc3_probes_points_info - retrieve list of probe points
 * @cdev:		SOF client device
 * @desc:	Returned list of active probes
 * @num_desc:	Returned count of active probes
 * @type:	Either PROBES_INFO_ACTIVE_PROBES or PROBES_INFO_AVAILABE_PROBES
 *
 * If type is PROBES_INFO_ACTIVE_PROBES, host sends PROBE_POINT_INFO
 * request to obtain list of active probe points, valid for
 * disconnection when given probe is no longer required.
 *
 * Type PROBES_INFO_AVAILABE_PROBES is not yet supported.
 */
unsafe extern "C" fn ipc3_probes_points_info(
    cdev: *mut sof_client_dev,
    desc: *mut *mut sof_probe_point_desc,
    num_desc: *mut usize,
    type_: sof_probe_info_type,
) -> i32 {
    ipc3_probes_info(
        cdev,
        SOF_IPC_PROBE_POINT_INFO,
        desc as *mut *mut c_void,
        num_desc,
        type_,
    )
}

/**
 * ipc3_probes_points_add - connect specified probes
 * @cdev:		SOF client device
 * @desc:	List of probe points to connect
 * @num_desc:	Number of elements in @desc
 *
 * Dynamically connects to provided set of endpoints. Immediately
 * after connection is established, host must be prepared to
 * transfer data from or to target stream given the probing purpose.
 *
 * Each probe point should be removed using PROBE_POINT_REMOVE
 * request when no longer needed.
 */
unsafe extern "C" fn ipc3_probes_points_add(
    cdev: *mut sof_client_dev,
    desc: *mut sof_probe_point_desc,
    num_desc: usize,
) -> i32 {
    let msg: *mut sof_ipc_probe_point_add_params;
    let size: usize =
        size_of::<sof_ipc_probe_point_add_params>() + size_of::<sof_probe_point_desc>() * num_desc;
    let ret: i32;

    msg = kmalloc(size, GFP_KERNEL) as *mut sof_ipc_probe_point_add_params;
    if msg.is_null() {
        return -ENOMEM;
    }
    ptr::addr_of_mut!((*msg).hdr.size).write_unaligned(size as _);
    ptr::addr_of_mut!((*msg).num_elems).write_unaligned(num_desc as _);
    ptr::addr_of_mut!((*msg).hdr.cmd)
        .write_unaligned(SOF_IPC_GLB_PROBE | SOF_IPC_PROBE_POINT_ADD);
    ptr::copy_nonoverlapping(
        desc as *const u8,
        (msg as *mut u8).add(size_of::<sof_ipc_probe_point_add_params>()),
        size - size_of::<sof_ipc_probe_point_add_params>(),
    );

    ret = sof_client_ipc_tx_message_no_reply(cdev, msg as *mut c_void);
    kfree(msg as *const c_void);
    ret
}

/**
 * ipc3_probes_points_remove - disconnect specified probes
 * @cdev:		SOF client device
 * @buffer_id:		List of probe points to disconnect
 * @num_buffer_id:	Number of elements in @desc
 *
 * Removes previously connected probes from list of active probe
 * points and frees all resources on DSP side.
 */
unsafe extern "C" fn ipc3_probes_points_remove(
    cdev: *mut sof_client_dev,
    buffer_id: *mut u32,
    num_buffer_id: usize,
) -> i32 {
    let msg: *mut sof_ipc_probe_point_remove_params;
    let size: usize =
        size_of::<sof_ipc_probe_point_remove_params>() + size_of::<u32>() * num_buffer_id;
    let ret: i32;

    msg = kmalloc(size, GFP_KERNEL) as *mut sof_ipc_probe_point_remove_params;
    if msg.is_null() {
        return -ENOMEM;
    }
    ptr::addr_of_mut!((*msg).hdr.size).write_unaligned(size as _);
    ptr::addr_of_mut!((*msg).num_elems).write_unaligned(num_buffer_id as _);
    ptr::addr_of_mut!((*msg).hdr.cmd)
        .write_unaligned(SOF_IPC_GLB_PROBE | SOF_IPC_PROBE_POINT_REMOVE);
    ptr::copy_nonoverlapping(
        buffer_id as *const u8,
        (msg as *mut u8).add(size_of::<sof_ipc_probe_point_remove_params>()),
        size - size_of::<sof_ipc_probe_point_remove_params>(),
    );

    ret = sof_client_ipc_tx_message_no_reply(cdev, msg as *mut c_void);
    kfree(msg as *const c_void);
    ret
}

pub static ipc3_probe_ops: sof_probes_ipc_ops = sof_probes_ipc_ops {
    init: Some(ipc3_probes_init),
    deinit: Some(ipc3_probes_deinit),
    points_info: Some(ipc3_probes_points_info),
    points_add: Some(ipc3_probes_points_add),
    points_remove: Some(ipc3_probes_points_remove),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
