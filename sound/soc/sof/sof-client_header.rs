/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_void;

/* Dependencies from:
 * <linux/auxiliary_bus.h>
 * <linux/device.h>
 * <linux/list.h>
 * <sound/sof.h>
 */

#[repr(C)]
pub struct sof_ipc_fw_version {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc4_fw_module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxiliary_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct guid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub enum sof_ipc_type {
    /* Defined by <sound/sof.h>. */
}

#[repr(C)]
pub enum sof_fw_state {
    /* Defined by <sound/sof.h>. */
}

/**
 * struct sof_client_dev - SOF client device
 * @auxdev:	auxiliary device
 * @data:	device specific data
 */
#[repr(C)]
pub struct sof_client_dev {
    pub auxdev: auxiliary_device,
    pub data: *mut c_void,
}

/* C macro:
 * #define auxiliary_dev_to_sof_client_dev(auxiliary_dev) \
 *	container_of(auxiliary_dev, struct sof_client_dev, auxdev)
 */
pub unsafe fn auxiliary_dev_to_sof_client_dev(
    auxiliary_dev: *mut auxiliary_device,
) -> *mut sof_client_dev {
    container_of!(auxiliary_dev, sof_client_dev, auxdev)
}

/* C macro:
 * #define dev_to_sof_client_dev(dev) \
 *	container_of(to_auxiliary_dev(dev), struct sof_client_dev, auxdev)
 */
pub unsafe fn dev_to_sof_client_dev(dev: *mut device) -> *mut sof_client_dev {
    container_of!(to_auxiliary_dev(dev), sof_client_dev, auxdev)
}

extern "C" {
    pub fn sof_client_ipc_tx_message(
        cdev: *mut sof_client_dev,
        ipc_msg: *mut c_void,
        reply_data: *mut c_void,
        reply_bytes: usize,
    ) -> i32;
}

pub unsafe fn sof_client_ipc_tx_message_no_reply(
    cdev: *mut sof_client_dev,
    ipc_msg: *mut c_void,
) -> i32 {
    unsafe { sof_client_ipc_tx_message(cdev, ipc_msg, core::ptr::null_mut(), 0) }
}

extern "C" {
    pub fn sof_client_ipc_set_get_data(
        cdev: *mut sof_client_dev,
        ipc_msg: *mut c_void,
        set: bool,
    ) -> i32;

    pub fn sof_client_ipc4_find_module(
        c: *mut sof_client_dev,
        u: *const guid_t,
    ) -> *mut sof_ipc4_fw_module;

    pub fn sof_client_ipc4_find_swidget_by_id(
        cdev: *mut sof_client_dev,
        module_id: u32,
        instance_id: i32,
    ) -> *mut snd_sof_widget;

    pub fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry;
    pub fn sof_client_get_dma_dev(cdev: *mut sof_client_dev) -> *mut device;
    pub fn sof_client_get_fw_version(
        cdev: *mut sof_client_dev,
    ) -> *const sof_ipc_fw_version;
    pub fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> usize;
    pub fn sof_client_get_ipc_type(cdev: *mut sof_client_dev) -> sof_ipc_type;

    /* DSP/firmware boot request */
    pub fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> i32;

    /* module refcount management of SOF core */
    pub fn sof_client_core_module_get(cdev: *mut sof_client_dev) -> i32;
    pub fn sof_client_core_module_put(cdev: *mut sof_client_dev);
}

/* IPC notification */
pub type sof_client_event_callback =
    Option<unsafe extern "C" fn(cdev: *mut sof_client_dev, msg_buf: *mut c_void)>;

extern "C" {
    pub fn sof_client_register_ipc_rx_handler(
        cdev: *mut sof_client_dev,
        ipc_msg_type: u32,
        callback: sof_client_event_callback,
    ) -> i32;

    pub fn sof_client_unregister_ipc_rx_handler(
        cdev: *mut sof_client_dev,
        ipc_msg_type: u32,
    );
}

/* DSP state notification and query */
pub type sof_client_fw_state_callback =
    Option<unsafe extern "C" fn(cdev: *mut sof_client_dev, state: sof_fw_state)>;

extern "C" {
    pub fn sof_client_register_fw_state_handler(
        cdev: *mut sof_client_dev,
        callback: sof_client_fw_state_callback,
    ) -> i32;

    pub fn sof_client_unregister_fw_state_handler(cdev: *mut sof_client_dev);
    pub fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> sof_fw_state;
    pub fn sof_client_ipc_rx_message(
        cdev: *mut sof_client_dev,
        ipc_msg: *mut c_void,
        msg_buf: *mut c_void,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
