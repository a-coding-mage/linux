// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
}

#[repr(C)]
pub struct sdca_entity {
    pub id: c_uint,
    pub label: *const c_char,
}

#[repr(C)]
pub struct sdca_control {
    pub sel: c_uint,
}

#[repr(C)]
pub struct sdca_control_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
pub const ENOMEM: c_int = 12;
pub const GFP_KERNEL: c_uint = 0;

unsafe extern "C" {
    static system_dfl_wq: *mut workqueue_struct;

    static SDCA_UMP_OWNER_HOST: c_uint;
    static SDCA_UMP_OWNER_DEVICE: c_uint;
    static SDCA_MESSAGEOFFSET_NCOLS: c_uint;
    static SDCA_MESSAGEOFFSET_BUFFER_START_ADDRESS: c_uint;
    static SDCA_MESSAGEOFFSET_BUFFER_LENGTH: c_uint;
    static SDCA_MESSAGEOFFSET_UMP_MODE: c_uint;
    static SDCA_UMP_MODE_DIRECT: c_uint;

    fn SDW_SDCA_CTL(adr: c_uint, entity_id: c_uint, control_sel: c_uint, offset: c_uint) -> c_uint;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_read(
        map: *mut regmap,
        reg: c_uint,
        val: *mut c_void,
        val_len: c_uint,
    ) -> c_int;
    fn regmap_raw_write(
        map: *mut regmap,
        reg: c_uint,
        val: *mut c_void,
        val_len: c_int,
    ) -> c_int;
    fn sdca_selector_find_range(
        dev: *mut device,
        entity: *mut sdca_entity,
        sel: c_uint,
        ncols: c_uint,
        row: c_uint,
    ) -> *mut sdca_control_range;
    fn sdca_range(range: *mut sdca_control_range, col: c_uint, row: c_uint) -> c_uint;
    fn kmalloc(size: c_uint, flags: c_uint) -> *mut c_void;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn queue_delayed_work(
        wq: *mut workqueue_struct,
        work: *mut delayed_work,
        delay: c_uint,
    ) -> bool;
    fn usecs_to_jiffies(usecs: c_uint) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

/**
 * sdca_ump_get_owner_host - check a UMP buffer is owned by the host
 * @dev: Pointer to the struct device used for error messages.
 * @function_regmap: Pointer to the regmap for the SDCA Function.
 * @function: Pointer to the Function information.
 * @entity: Pointer to the SDCA Entity.
 * @control: Pointer to the SDCA Control for the UMP Owner.
 *
 * Return: Returns zero on success, and a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_ump_get_owner_host(
    dev: *mut device,
    function_regmap: *mut regmap,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    control: *mut sdca_control,
) -> c_int {
    let reg: c_uint;
    let mut owner: c_uint = 0;
    let ret: c_int;

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, 0);
    ret = regmap_read(function_regmap, reg, &mut owner);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: failed to read UMP owner: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
        return ret;
    }

    if owner != SDCA_UMP_OWNER_HOST {
        dev_err(
            dev,
            c"%s: host is not the UMP owner\n".as_ptr(),
            (*entity).label,
        );
        return -EINVAL;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_get_owner_host, "SND_SOC_SDCA");

/**
 * sdca_ump_set_owner_device - set a UMP buffer's ownership back to the device
 * @dev: Pointer to the struct device used for error messages.
 * @function_regmap: Pointer to the regmap for the SDCA Function.
 * @function: Pointer to the Function information.
 * @entity: Pointer to the SDCA Entity.
 * @control: Pointer to the SDCA Control for the UMP Owner.
 *
 * Return: Returns zero on success, and a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_ump_set_owner_device(
    dev: *mut device,
    function_regmap: *mut regmap,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    control: *mut sdca_control,
) -> c_int {
    let reg: c_uint;
    let ret: c_int;

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, 0);
    ret = regmap_write(function_regmap, reg, SDCA_UMP_OWNER_DEVICE);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: failed to write UMP owner: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
    }

    ret
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_set_owner_device, "SND_SOC_SDCA");

/**
 * sdca_ump_read_message - read a UMP message from the device
 * @dev: Pointer to the struct device used for error messages.
 * @device_regmap: Pointer to the Device register map.
 * @function_regmap: Pointer to the regmap for the SDCA Function.
 * @function: Pointer to the Function information.
 * @entity: Pointer to the SDCA Entity.
 * @offset_sel: Control Selector for the UMP Offset Control.
 * @length_sel: Control Selector for the UMP Length Control.
 * @msg: Pointer that will be populated with an dynamically buffer
 * containing the UMP message. Note this needs to be freed by the
 * caller.
 *
 * The caller should first call sdca_ump_get_owner_host() to ensure the host
 * currently owns the UMP buffer, and then this function can be used to
 * retrieve a message. It is the callers responsibility to free the
 * message once it is finished with it. Finally sdca_ump_set_owner_device()
 * should be called to return the buffer to the device.
 *
 * Return: Returns the message length on success, and a negative error
 * code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_ump_read_message(
    dev: *mut device,
    device_regmap: *mut regmap,
    function_regmap: *mut regmap,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    offset_sel: c_uint,
    length_sel: c_uint,
    msg: *mut *mut c_void,
) -> c_int {
    let range: *mut sdca_control_range;
    let mut msg_offset: c_uint = 0;
    let mut msg_len: c_uint = 0;
    let buf_addr: c_uint;
    let buf_len: c_uint;
    let mut reg: c_uint;
    let ret: c_int;

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, offset_sel, 0);
    let read_ret = regmap_read(function_regmap, reg, &mut msg_offset);
    if read_ret < 0 {
        dev_err(
            dev,
            c"%s: failed to read UMP offset: %d\n".as_ptr(),
            (*entity).label,
            read_ret,
        );
        return read_ret;
    }

    range = sdca_selector_find_range(dev, entity, offset_sel, SDCA_MESSAGEOFFSET_NCOLS, 1);
    if range.is_null() {
        return -ENOENT;
    }

    buf_addr = sdca_range(range, SDCA_MESSAGEOFFSET_BUFFER_START_ADDRESS, 0);
    buf_len = sdca_range(range, SDCA_MESSAGEOFFSET_BUFFER_LENGTH, 0);

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, length_sel, 0);
    let read_ret = regmap_read(function_regmap, reg, &mut msg_len);
    if read_ret < 0 {
        dev_err(
            dev,
            c"%s: failed to read UMP length: %d\n".as_ptr(),
            (*entity).label,
            read_ret,
        );
        return read_ret;
    }

    if msg_offset.wrapping_add(msg_len) > buf_len {
        dev_err(
            dev,
            c"%s: message too big for UMP buffer: %d\n".as_ptr(),
            (*entity).label,
            msg_len,
        );
        return -EINVAL;
    }

    *msg = kmalloc(msg_len, GFP_KERNEL);
    if (*msg).is_null() {
        return -ENOMEM;
    }

    ret = regmap_raw_read(device_regmap, buf_addr.wrapping_add(msg_offset), *msg, msg_len);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: failed to read UMP message: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
        return ret;
    }

    msg_len as c_int
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_read_message, "SND_SOC_SDCA");

/**
 * sdca_ump_write_message - write a UMP message to the device
 * @dev: Pointer to the struct device used for error messages.
 * @device_regmap: Pointer to the Device register map.
 * @function_regmap: Pointer to the regmap for the SDCA Function.
 * @function: Pointer to the Function information.
 * @entity: Pointer to the SDCA Entity.
 * @offset_sel: Control Selector for the UMP Offset Control.
 * @msg_offset: Offset within the UMP buffer at which the message should
 * be written.
 * @length_sel: Control Selector for the UMP Length Control.
 * @msg: Pointer to the data that should be written to the UMP buffer.
 * @msg_len: Length of the message data in bytes.
 *
 * The caller should first call sdca_ump_get_owner_host() to ensure the host
 * currently owns the UMP buffer, and then this function can be used to
 * write a message. Finally sdca_ump_set_owner_device() should be called to
 * return the buffer to the device, allowing the device to access the
 * message.
 *
 * Return: Returns zero on success, and a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_ump_write_message(
    dev: *mut device,
    device_regmap: *mut regmap,
    function_regmap: *mut regmap,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    offset_sel: c_uint,
    msg_offset: c_uint,
    length_sel: c_uint,
    msg: *mut c_void,
    msg_len: c_int,
) -> c_int {
    let range: *mut sdca_control_range;
    let buf_addr: c_uint;
    let buf_len: c_uint;
    let ump_mode: c_uint;
    let mut reg: c_uint;
    let mut ret: c_int;

    range = sdca_selector_find_range(dev, entity, offset_sel, SDCA_MESSAGEOFFSET_NCOLS, 1);
    if range.is_null() {
        return -ENOENT;
    }

    buf_addr = sdca_range(range, SDCA_MESSAGEOFFSET_BUFFER_START_ADDRESS, 0);
    buf_len = sdca_range(range, SDCA_MESSAGEOFFSET_BUFFER_LENGTH, 0);
    ump_mode = sdca_range(range, SDCA_MESSAGEOFFSET_UMP_MODE, 0);

    if msg_offset.wrapping_add(msg_len as c_uint) > buf_len {
        dev_err(
            dev,
            c"%s: message too big for UMP buffer: %d\n".as_ptr(),
            (*entity).label,
            msg_len,
        );
        return -EINVAL;
    }

    if ump_mode != SDCA_UMP_MODE_DIRECT {
        dev_err(
            dev,
            c"%s: only direct mode currently supported\n".as_ptr(),
            (*entity).label,
        );
        return -EINVAL;
    }

    ret = regmap_raw_write(device_regmap, buf_addr.wrapping_add(msg_offset), msg, msg_len);
    if ret != 0 {
        dev_err(
            dev,
            c"%s: failed to write UMP message: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
        return ret;
    }

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, offset_sel, 0);
    ret = regmap_write(function_regmap, reg, msg_offset);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: failed to write UMP offset: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
        return ret;
    }

    reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, length_sel, 0);
    ret = regmap_write(function_regmap, reg, msg_len as c_uint);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: failed to write UMP length: %d\n".as_ptr(),
            (*entity).label,
            ret,
        );
        return ret;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_write_message, "SND_SOC_SDCA");

#[no_mangle]
pub unsafe extern "C" fn sdca_ump_cancel_timeout(work: *mut delayed_work) {
    cancel_delayed_work_sync(work);
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_cancel_timeout, "SND_SOC_SDCA");

#[no_mangle]
pub unsafe extern "C" fn sdca_ump_schedule_timeout(
    work: *mut delayed_work,
    timeout_us: c_uint,
) {
    if timeout_us == 0 {
        return;
    }

    queue_delayed_work(system_dfl_wq, work, usecs_to_jiffies(timeout_us));
}
// EXPORT_SYMBOL_NS_GPL(sdca_ump_schedule_timeout, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
