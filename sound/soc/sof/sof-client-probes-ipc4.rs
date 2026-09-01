// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2019-2022 Intel Corporation
//
// Author: Jyri Sarha <jyri.sarha@intel.com>
//

// C dependencies:
// <sound/soc.h>
// <sound/sof/ipc4/header.h>
// <uapi/sound/sof/header.h>
// "sof-audio.h"
// "ipc4-priv.h"
// "sof-client.h"
// "sof-client-probes.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type size_t = usize;

const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const fn genmask(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

#[repr(C)]
pub enum sof_ipc4_dma_type {
    SOF_IPC4_DMA_HDA_HOST_OUTPUT = 0,
    SOF_IPC4_DMA_HDA_HOST_INPUT = 1,
    SOF_IPC4_DMA_HDA_LINK_OUTPUT = 8,
    SOF_IPC4_DMA_HDA_LINK_INPUT = 9,
    SOF_IPC4_DMA_DMIC_LINK_INPUT = 11,
    SOF_IPC4_DMA_I2S_LINK_OUTPUT = 12,
    SOF_IPC4_DMA_I2S_LINK_INPUT = 13,
}

#[repr(C)]
pub enum sof_ipc4_probe_runtime_param {
    SOF_IPC4_PROBE_INJECTION_DMA = 1,
    SOF_IPC4_PROBE_INJECTION_DMA_DETACH,
    SOF_IPC4_PROBE_POINTS,
    SOF_IPC4_PROBE_POINTS_DISCONNECT,
    SOF_IPC4_PROBE_POINTS_AVAILABLE,
}

#[repr(C, packed(4))]
pub struct sof_ipc4_probe_gtw_cfg {
    pub node_id: u32,
    pub dma_buffer_size: u32,
}

#[inline]
const fn SOF_IPC4_PROBE_NODE_ID_INDEX(x: u32) -> u32 {
    x & genmask(7, 0)
}

#[inline]
const fn SOF_IPC4_PROBE_NODE_ID_TYPE(x: u32) -> u32 {
    (x << 8) & genmask(12, 8)
}

#[repr(C, packed(4))]
pub struct sof_ipc4_probe_cfg {
    pub base: sof_ipc4_base_module_cfg,
    pub gtw_cfg: sof_ipc4_probe_gtw_cfg,
}

#[repr(C)]
pub enum sof_ipc4_probe_type {
    SOF_IPC4_PROBE_TYPE_INPUT = 0,
    SOF_IPC4_PROBE_TYPE_OUTPUT,
    SOF_IPC4_PROBE_TYPE_INTERNAL,
}

const SOF_IPC4_PROBE_TYPE_SHIFT: u32 = 24;
const SOF_IPC4_PROBE_TYPE_MASK: u32 = genmask(25, 24);

#[inline]
const fn SOF_IPC4_PROBE_TYPE_GET(x: u32) -> u32 {
    (x & SOF_IPC4_PROBE_TYPE_MASK) >> SOF_IPC4_PROBE_TYPE_SHIFT
}

const SOF_IPC4_PROBE_IDX_SHIFT: u32 = 26;
const SOF_IPC4_PROBE_IDX_MASK: u32 = genmask(31, 26);

#[inline]
const fn SOF_IPC4_PROBE_IDX_GET(x: u32) -> u32 {
    (x & SOF_IPC4_PROBE_IDX_MASK) >> SOF_IPC4_PROBE_IDX_SHIFT
}

#[repr(C, packed(4))]
pub struct sof_ipc4_probe_point {
    pub point_id: u32,
    pub purpose: u32,
    pub stream_tag: u32,
}

#[repr(C, packed)]
pub struct sof_ipc4_probe_info {
    pub num_elems: c_uint,
    pub points: [sof_ipc4_probe_point; 0],
}

const INVALID_PIPELINE_ID: u32 = 0xFF;

unsafe fn sof_probe_ipc4_type_string(type_: u32) -> *const c_char {
    match type_ {
        x if x == sof_ipc4_probe_type::SOF_IPC4_PROBE_TYPE_INPUT as u32 => c"input".as_ptr(),
        x if x == sof_ipc4_probe_type::SOF_IPC4_PROBE_TYPE_OUTPUT as u32 => c"output".as_ptr(),
        x if x == sof_ipc4_probe_type::SOF_IPC4_PROBE_TYPE_INTERNAL as u32 => c"internal".as_ptr(),
        _ => c"UNKNOWN".as_ptr(),
    }
}

/**
 * sof_ipc4_probe_get_module_info - Get IPC4 module info for probe module
 * @cdev:		SOF client device
 * @return:		Pointer to IPC4 probe module info
 *
 * Look up the IPC4 probe module info based on the hard coded uuid and
 * store the value for the future calls.
 */
unsafe fn sof_ipc4_probe_get_module_info(cdev: *mut sof_client_dev) -> *mut sof_man4_module {
    let priv_: *mut sof_probes_priv = (*cdev).data as *mut sof_probes_priv;
    let dev: *mut device = &mut (*cdev).auxdev.dev;
    static PROBE_UUID: guid_t = GUID_INIT(
        0x7CAD0808, 0xAB10, 0xCD23, 0xEF, 0x45, 0x12, 0xAB, 0x34, 0xCD, 0x56, 0xEF,
    );

    if (*priv_).ipc_priv.is_null() {
        let fw_module: *mut sof_ipc4_fw_module = sof_client_ipc4_find_module(cdev, &PROBE_UUID);

        if fw_module.is_null() {
            dev_err(dev, c"%s: no matching uuid found".as_ptr(), c"sof_ipc4_probe_get_module_info".as_ptr());
            return ptr::null_mut();
        }

        (*priv_).ipc_priv = &mut (*fw_module).man4_module_entry as *mut _ as *mut c_void;
    }

    (*priv_).ipc_priv as *mut sof_man4_module
}

/**
 * ipc4_probes_init - initialize data probing
 * @cdev:		SOF client device
 * @stream_tag:		Extractor stream tag
 * @buffer_size:	DMA buffer size to set for extractor
 * @return:		0 on success, negative error code on error
 *
 * Host chooses whether extraction is supported or not by providing
 * valid stream tag to DSP. Once specified, stream described by that
 * tag will be tied to DSP for extraction for the entire lifetime of
 * probe.
 *
 * Probing is initialized only once and each INIT request must be
 * matched by DEINIT call.
 */
unsafe extern "C" fn ipc4_probes_init(
    cdev: *mut sof_client_dev,
    stream_tag: u32,
    buffer_size: size_t,
) -> c_int {
    let mentry: *mut sof_man4_module = sof_ipc4_probe_get_module_info(cdev);
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut cfg: sof_ipc4_probe_cfg = core::mem::zeroed();

    if mentry.is_null() {
        return -ENODEV;
    }

    cfg.gtw_cfg.node_id = SOF_IPC4_PROBE_NODE_ID_INDEX(stream_tag.wrapping_sub(1))
        | SOF_IPC4_PROBE_NODE_ID_TYPE(sof_ipc4_dma_type::SOF_IPC4_DMA_HDA_HOST_INPUT as u32);

    cfg.gtw_cfg.dma_buffer_size = buffer_size as u32;

    msg.primary = (*mentry).id;
    msg.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_INIT_INSTANCE);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.extension = SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE(INVALID_PIPELINE_ID);
    msg.extension |= SOF_IPC4_MOD_EXT_CORE_ID(0);
    msg.extension |= SOF_IPC4_MOD_EXT_PARAM_SIZE((size_of::<sof_ipc4_probe_cfg>() / size_of::<u32>()) as u32);

    msg.data_size = size_of::<sof_ipc4_probe_cfg>();
    msg.data_ptr = &mut cfg as *mut _ as *mut c_void;

    sof_client_ipc_tx_message_no_reply(cdev, &mut msg)
}

/**
 * ipc4_probes_deinit - cleanup after data probing
 * @cdev:		SOF client device
 * @return:		0 on success, negative error code on error
 *
 * Host sends DEINIT request to free previously initialized probe
 * on DSP side once it is no longer needed. DEINIT only when there
 * are no probes connected and with all injectors detached.
 */
unsafe extern "C" fn ipc4_probes_deinit(cdev: *mut sof_client_dev) -> c_int {
    let mentry: *mut sof_man4_module = sof_ipc4_probe_get_module_info(cdev);
    let mut msg: sof_ipc4_msg = core::mem::zeroed();

    if mentry.is_null() {
        return -ENODEV;
    }

    msg.primary = (*mentry).id;
    msg.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_DELETE_INSTANCE);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.extension = SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE(INVALID_PIPELINE_ID);
    msg.extension |= SOF_IPC4_MOD_EXT_CORE_ID(0);

    msg.data_size = 0;
    msg.data_ptr = ptr::null_mut();

    sof_client_ipc_tx_message_no_reply(cdev, &mut msg)
}

/**
 * ipc4_probes_points_info - retrieve list of probe points
 * @cdev:	SOF client device
 * @desc:	Returned list of active probes
 * @num_desc:	Returned count of active probes
 * @type:	Either PROBES_INFO_ACTIVE_PROBES or PROBES_INFO_AVAILABE_PROBES
 * @return:	0 on success, negative error code on error
 *
 * Returns list if active probe points if type is
 * PROBES_INFO_ACTIVE_PROBES, or list of all available probe points if
 * type is PROBES_INFO_AVAILABE_PROBES.
 */
unsafe extern "C" fn ipc4_probes_points_info(
    cdev: *mut sof_client_dev,
    desc: *mut *mut sof_probe_point_desc,
    num_desc: *mut size_t,
    type_: sof_probe_info_type,
) -> c_int {
    let mentry: *mut sof_man4_module = sof_ipc4_probe_get_module_info(cdev);
    let dev: *mut device = &mut (*cdev).auxdev.dev;
    let mut info: *mut sof_ipc4_probe_info;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let param_id: u32;
    let mut i: c_int;
    let ret: c_int;

    if mentry.is_null() {
        return -ENODEV;
    }

    match type_ {
        sof_probe_info_type::PROBES_INFO_ACTIVE_PROBES => {
            param_id = sof_ipc4_probe_runtime_param::SOF_IPC4_PROBE_POINTS as u32;
        }
        sof_probe_info_type::PROBES_INFO_AVAILABE_PROBES => {
            param_id = sof_ipc4_probe_runtime_param::SOF_IPC4_PROBE_POINTS_AVAILABLE as u32;
        }
        _ => {
            dev_err(dev, c"%s: info type %u not supported".as_ptr(), c"ipc4_probes_points_info".as_ptr(), type_ as c_uint);
            return -EOPNOTSUPP;
        }
    }

    msg.primary = (*mentry).id;
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);

    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(param_id);

    msg.data_size = sof_client_get_ipc_max_payload_size(cdev);
    msg.data_ptr = kzalloc(msg.data_size, GFP_KERNEL);
    if msg.data_ptr.is_null() {
        return -ENOMEM;
    }

    ret = sof_client_ipc_set_get_data(cdev, &mut msg, false);
    if ret != 0 {
        kfree(msg.data_ptr);
        return ret;
    }
    info = msg.data_ptr as *mut sof_ipc4_probe_info;
    if msg.data_size < size_of::<sof_ipc4_probe_info>()
        || (*info).num_elems as usize
            > (msg.data_size - size_of::<sof_ipc4_probe_info>()) / size_of::<sof_ipc4_probe_point>()
    {
        dev_err(
            dev,
            c"%s: invalid probe info element count %u\n".as_ptr(),
            c"ipc4_probes_points_info".as_ptr(),
            (*info).num_elems,
        );
        kfree(msg.data_ptr);
        return -EINVAL;
    }

    *num_desc = (*info).num_elems as size_t;
    dev_dbg(dev, c"%s: got %zu probe points".as_ptr(), c"ipc4_probes_points_info".as_ptr(), *num_desc);

    *desc = kcalloc(*num_desc, size_of::<sof_probe_point_desc>(), GFP_KERNEL) as *mut sof_probe_point_desc;
    if (*desc).is_null() {
        kfree(msg.data_ptr);
        return -ENOMEM;
    }

    i = 0;
    while (i as size_t) < *num_desc {
        (*(*desc).add(i as usize)).buffer_id = (*info).points.as_ptr().add(i as usize).read_unaligned().point_id;
        (*(*desc).add(i as usize)).purpose = (*info).points.as_ptr().add(i as usize).read_unaligned().purpose;
        (*(*desc).add(i as usize)).stream_tag = (*info).points.as_ptr().add(i as usize).read_unaligned().stream_tag;
        i += 1;
    }
    kfree(msg.data_ptr);

    0
}

/**
 * ipc4_probes_point_print - Human readable print of probe point descriptor
 * @cdev:	SOF client device
 * @buf:	Buffer to print to
 * @size:	Available bytes in buffer
 * @desc:	Describes the probe point to print
 * @return:	Number of bytes printed or an error code (snprintf return value)
 */
unsafe extern "C" fn ipc4_probes_point_print(
    cdev: *mut sof_client_dev,
    buf: *mut c_char,
    size: size_t,
    desc: *mut sof_probe_point_desc,
) -> c_int {
    let dev: *mut device = &mut (*cdev).auxdev.dev;
    let mut swidget: *mut snd_sof_widget;
    let ret: c_int;

    swidget = sof_client_ipc4_find_swidget_by_id(
        cdev,
        SOF_IPC4_MOD_ID_GET((*desc).buffer_id),
        SOF_IPC4_MOD_INSTANCE_GET((*desc).buffer_id),
    );
    if swidget.is_null() {
        dev_err(
            dev,
            c"%s: Failed to find widget for module %lu.%lu\n".as_ptr(),
            c"ipc4_probes_point_print".as_ptr(),
            SOF_IPC4_MOD_ID_GET((*desc).buffer_id),
            SOF_IPC4_MOD_INSTANCE_GET((*desc).buffer_id),
        );
    }

    ret = scnprintf(
        buf,
        size,
        c"%#x,%#x,%#x\t%s %s buf idx %lu %s\n".as_ptr(),
        (*desc).buffer_id,
        (*desc).purpose,
        (*desc).stream_tag,
        if !swidget.is_null() {
            (*(*swidget).widget).name
        } else {
            c"<unknown>".as_ptr()
        },
        sof_probe_ipc4_type_string(SOF_IPC4_PROBE_TYPE_GET((*desc).buffer_id)),
        SOF_IPC4_PROBE_IDX_GET((*desc).buffer_id),
        if (*desc).stream_tag != 0 {
            c"(connected)".as_ptr()
        } else {
            c"".as_ptr()
        },
    );

    ret
}

/**
 * ipc4_probes_points_add - connect specified probes
 * @cdev:	SOF client device
 * @desc:	List of probe points to connect
 * @num_desc:	Number of elements in @desc
 * @return:	0 on success, negative error code on error
 *
 * Translates the generic probe point presentation to an IPC4
 * message to dynamically connect the provided set of endpoints.
 */
unsafe extern "C" fn ipc4_probes_points_add(
    cdev: *mut sof_client_dev,
    desc: *mut sof_probe_point_desc,
    num_desc: size_t,
) -> c_int {
    let mentry: *mut sof_man4_module = sof_ipc4_probe_get_module_info(cdev);
    let mut points: *mut sof_ipc4_probe_point;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut i: c_int;
    let ret: c_int;

    if mentry.is_null() {
        return -EOPNOTSUPP;
    }

    /* The sof_probe_point_desc and sof_ipc4_probe_point structs
     * are of same size and even the integers are the same in the
     * same order, and similar meaning, but since there is no
     * performance issue I wrote the conversion explicitly open for
     * future development.
     */
    points = kzalloc_objs(size_of::<sof_ipc4_probe_point>(), num_desc) as *mut sof_ipc4_probe_point;
    if points.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as size_t) < num_desc {
        (*points.add(i as usize)).point_id = (*desc.add(i as usize)).buffer_id;
        (*points.add(i as usize)).purpose = (*desc.add(i as usize)).purpose;
        (*points.add(i as usize)).stream_tag = (*desc.add(i as usize)).stream_tag;
        i += 1;
    }

    msg.primary = (*mentry).id;
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);

    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(sof_ipc4_probe_runtime_param::SOF_IPC4_PROBE_POINTS as u32);

    msg.data_size = size_of::<sof_ipc4_probe_point>() * num_desc;
    msg.data_ptr = points as *mut c_void;

    ret = sof_client_ipc_set_get_data(cdev, &mut msg, true);

    kfree(points as *mut c_void);

    ret
}

/**
 * ipc4_probes_points_remove - disconnect specified probes
 * @cdev:		SOF client device
 * @buffer_id:		List of probe points to disconnect
 * @num_buffer_id:	Number of elements in @desc
 * @return:		0 on success, negative error code on error
 *
 * Converts the generic buffer_id to IPC4 probe_point_id and remove
 * the probe points with an IPC4 for message.
 */
unsafe extern "C" fn ipc4_probes_points_remove(
    cdev: *mut sof_client_dev,
    buffer_id: *mut c_uint,
    num_buffer_id: size_t,
) -> c_int {
    let mentry: *mut sof_man4_module = sof_ipc4_probe_get_module_info(cdev);
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut probe_point_ids: *mut u32;
    let mut i: c_int;
    let ret: c_int;

    if mentry.is_null() {
        return -ENODEV;
    }

    probe_point_ids = kcalloc(num_buffer_id, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if probe_point_ids.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as size_t) < num_buffer_id {
        *probe_point_ids.add(i as usize) = *buffer_id.add(i as usize);
        i += 1;
    }

    msg.primary = (*mentry).id;
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);

    msg.extension =
        SOF_IPC4_MOD_EXT_MSG_PARAM_ID(sof_ipc4_probe_runtime_param::SOF_IPC4_PROBE_POINTS_DISCONNECT as u32);

    msg.data_size = num_buffer_id * size_of::<u32>();
    msg.data_ptr = probe_point_ids as *mut c_void;

    ret = sof_client_ipc_set_get_data(cdev, &mut msg, true);

    kfree(probe_point_ids as *mut c_void);

    ret
}

#[no_mangle]
pub static ipc4_probe_ops: sof_probes_ipc_ops = sof_probes_ipc_ops {
    init: Some(ipc4_probes_init),
    deinit: Some(ipc4_probes_deinit),
    points_info: Some(ipc4_probes_points_info),
    point_print: Some(ipc4_probes_point_print),
    points_add: Some(ipc4_probes_points_add),
    points_remove: Some(ipc4_probes_points_remove),
};

#[repr(C)]
pub struct sof_ipc4_base_module_cfg {
    _data: [u8; 0],
}

#[repr(C)]
pub struct guid_t {
    _data: [u8; 16],
}

const fn GUID_INIT(
    a: u32,
    b: u16,
    c: u16,
    d0: u8,
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8,
    d6: u8,
    d7: u8,
) -> guid_t {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let cb = c.to_le_bytes();
    guid_t {
        _data: [ab[0], ab[1], ab[2], ab[3], bb[0], bb[1], cb[0], cb[1], d0, d1, d2, d3, d4, d5, d6, d7],
    }
}

#[repr(C)]
pub struct device {
    _data: [u8; 0],
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
}

#[repr(C)]
pub struct sof_client_dev {
    pub data: *mut c_void,
    pub auxdev: auxiliary_device,
}

#[repr(C)]
pub struct sof_probes_priv {
    pub ipc_priv: *mut c_void,
}

#[repr(C)]
pub struct sof_man4_module {
    pub id: u32,
}

#[repr(C)]
pub struct sof_ipc4_fw_module {
    pub man4_module_entry: sof_man4_module,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_size: size_t,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct sof_probe_point_desc {
    pub buffer_id: u32,
    pub purpose: u32,
    pub stream_tag: u32,
}

#[repr(C)]
pub enum sof_probe_info_type {
    PROBES_INFO_ACTIVE_PROBES,
    PROBES_INFO_AVAILABE_PROBES,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub widget: *mut snd_soc_dapm_widget,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
}

#[repr(C)]
pub struct sof_probes_ipc_ops {
    pub init: Option<unsafe extern "C" fn(*mut sof_client_dev, u32, size_t) -> c_int>,
    pub deinit: Option<unsafe extern "C" fn(*mut sof_client_dev) -> c_int>,
    pub points_info: Option<
        unsafe extern "C" fn(
            *mut sof_client_dev,
            *mut *mut sof_probe_point_desc,
            *mut size_t,
            sof_probe_info_type,
        ) -> c_int,
    >,
    pub point_print:
        Option<unsafe extern "C" fn(*mut sof_client_dev, *mut c_char, size_t, *mut sof_probe_point_desc) -> c_int>,
    pub points_add: Option<unsafe extern "C" fn(*mut sof_client_dev, *mut sof_probe_point_desc, size_t) -> c_int>,
    pub points_remove: Option<unsafe extern "C" fn(*mut sof_client_dev, *mut c_uint, size_t) -> c_int>,
}

extern "C" {
    fn sof_client_ipc4_find_module(cdev: *mut sof_client_dev, uuid: *const guid_t) -> *mut sof_ipc4_fw_module;
    fn sof_client_ipc_tx_message_no_reply(cdev: *mut sof_client_dev, msg: *mut sof_ipc4_msg) -> c_int;
    fn sof_client_ipc_set_get_data(cdev: *mut sof_client_dev, msg: *mut sof_ipc4_msg, set: bool) -> c_int;
    fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> size_t;
    fn sof_client_ipc4_find_swidget_by_id(
        cdev: *mut sof_client_dev,
        module_id: u32,
        instance_id: u32,
    ) -> *mut snd_sof_widget;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc_objs(size: size_t, n: size_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    fn SOF_IPC4_MSG_TYPE_SET(value: u32) -> u32;
    fn SOF_IPC4_MSG_DIR(value: u32) -> u32;
    fn SOF_IPC4_MSG_TARGET(value: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE(value: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_CORE_ID(value: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_PARAM_SIZE(value: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_MSG_PARAM_ID(value: u32) -> u32;
    fn SOF_IPC4_MOD_ID_GET(value: u32) -> u32;
    fn SOF_IPC4_MOD_INSTANCE_GET(value: u32) -> u32;

    static SOF_IPC4_MOD_INIT_INSTANCE: u32;
    static SOF_IPC4_MOD_DELETE_INSTANCE: u32;
    static SOF_IPC4_MSG_REQUEST: u32;
    static SOF_IPC4_MODULE_MSG: u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
