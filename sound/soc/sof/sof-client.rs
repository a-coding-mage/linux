// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation
//
// Authors: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//          Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::{offset_of, size_of};
use core::ptr::{addr_of_mut, null_mut};

type u32 = u32;
type size_t = usize;
type bool_ = bool;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const ENODEV: c_int = 19;

// Constants supplied by included SOF/kernel headers in the original C source.
extern "C" {
    static SOF_GLB_TYPE_MASK: u32;
    static SOF_IPC4_NOTIFICATION_TYPE_MASK: u32;
    static CONFIG_SND_SOC_SOF_DEBUG_IPC_FLOOD_TEST_NUM: c_int;
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub release: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub platform_data: *mut c_void,
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
}

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct auxiliary_device {
    pub name: *const c_char,
    pub dev: device,
    pub id: u32,
}

#[repr(C)]
pub struct auxiliary_driver {
    pub suspend: Option<unsafe extern "C" fn(auxdev: *mut auxiliary_device, state: pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(auxdev: *mut auxiliary_device) -> c_int>,
}

#[repr(C)]
pub struct dentry;

#[repr(C)]
pub struct guid_t;

#[repr(C)]
pub struct sof_ipc4_fw_module;

#[repr(C)]
pub struct snd_sof_widget;

#[repr(C)]
pub struct sof_ipc_fw_version;

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub data_size: size_t,
}

#[repr(C)]
pub struct sof_ipc {
    pub max_payload_size: size_t,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub ipc_type: sof_ipc_type,
}

#[repr(C)]
pub struct snd_sof_fw_ready {
    pub version: sof_ipc_fw_version,
}

#[repr(C)]
pub struct snd_sof_ops {
    pub register_ipc_clients: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub unregister_ipc_clients: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dspless_mode_selected: bool_,
    pub dev: *mut device,
    pub ipc: *mut sof_ipc,
    pub ipc_client_mutex: mutex,
    pub ipc_client_list: list_head,
    pub client_event_handler_mutex: mutex,
    pub ipc_rx_handler_list: list_head,
    pub fw_state_handler_list: list_head,
    pub debugfs_root: *mut dentry,
    pub fw_ready: snd_sof_fw_ready,
    pub fw_state: sof_fw_state,
}

#[repr(C)]
pub struct mutex;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_type {
    SOF_IPC_TYPE_3 = 3,
    SOF_IPC_TYPE_4 = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sof_fw_state {
    SOF_FW_STATE_UNKNOWN = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_message_t {
    pub event: c_int,
}

#[repr(C)]
pub struct sof_client_dev {
    pub auxdev: auxiliary_device,
}

pub type sof_client_event_callback =
    Option<unsafe extern "C" fn(cdev: *mut sof_client_dev, msg_buf: *mut c_void)>;
pub type sof_client_fw_state_callback =
    Option<unsafe extern "C" fn(cdev: *mut sof_client_dev, state: sof_fw_state)>;

/**
 * struct sof_ipc_event_entry - IPC client event description
 * @ipc_msg_type: IPC msg type of the event the client is interested
 * @cdev:         sof_client_dev of the requesting client
 * @callback:     Callback function of the client
 * @list:         item in SOF core client event list
 */
#[repr(C)]
pub struct sof_ipc_event_entry {
    pub ipc_msg_type: u32,
    pub cdev: *mut sof_client_dev,
    pub callback: sof_client_event_callback,
    pub list: list_head,
}

/**
 * struct sof_state_event_entry - DSP panic event subscription entry
 * @cdev:         sof_client_dev of the requesting client
 * @callback:     Callback function of the client
 * @list:         item in SOF core client event list
 */
#[repr(C)]
pub struct sof_state_event_entry {
    pub cdev: *mut sof_client_dev,
    pub callback: sof_client_fw_state_callback,
    pub list: list_head,
}

/**
 * struct sof_client_dev_entry - client device entry for internal management use
 * @sdev:       pointer to SOF core device struct
 * @list:       item in SOF core client dev list
 * @client_dev: SOF client device
 */
#[repr(C)]
pub struct sof_client_dev_entry {
    pub sdev: *mut snd_sof_dev,
    pub list: list_head,
    pub client_dev: sof_client_dev,
}

unsafe fn cdev_to_centry(cdev: *mut sof_client_dev) -> *mut sof_client_dev_entry {
    (cdev as *mut u8).sub(offset_of!(sof_client_dev_entry, client_dev)) as *mut sof_client_dev_entry
}

unsafe fn to_auxiliary_dev(dev: *mut device) -> *mut auxiliary_device {
    (dev as *mut u8).sub(offset_of!(auxiliary_device, dev)) as *mut auxiliary_device
}

unsafe fn auxiliary_dev_to_sof_client_dev(auxdev: *mut auxiliary_device) -> *mut sof_client_dev {
    (auxdev as *mut u8).sub(offset_of!(sof_client_dev, auxdev)) as *mut sof_client_dev
}

extern "C" {
    fn kfree(ptr: *mut c_void);
    fn kmemdup(src: *const c_void, len: size_t, flags: c_int) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_add(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_delete(auxdev: *mut auxiliary_device);
    fn auxiliary_device_uninit(auxdev: *mut auxiliary_device);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sof_ipc_tx_message(
        ipc: *mut sof_ipc,
        msg: *mut c_void,
        msg_bytes: size_t,
        reply_data: *mut c_void,
        reply_bytes: size_t,
    ) -> c_int;
    fn sof_ipc_set_get_data(ipc: *mut sof_ipc, msg: *mut c_void, msg_bytes: size_t, set: bool_) -> c_int;
    fn sof_ipc3_do_rx_work(sdev: *mut snd_sof_dev, ipc_msg: *mut c_void, msg_buf: *mut c_void);
    fn sof_ipc4_find_module_by_uuid(sdev: *mut snd_sof_dev, uuid: *const guid_t) -> *mut sof_ipc4_fw_module;
    fn sof_ipc4_find_swidget_by_ids(
        sdev: *mut snd_sof_dev,
        module_id: u32,
        instance_id: c_int,
    ) -> *mut snd_sof_widget;
    fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn try_module_get(module: *mut module) -> bool_;
    fn module_put(module: *mut module);
    fn sof_ops(sdev: *mut snd_sof_dev) -> *mut snd_sof_ops;
    fn SOF_IPC4_NOTIFICATION_TYPE_GET(primary: u32) -> u32;
    fn to_auxiliary_drv(driver: *mut device_driver) -> *const auxiliary_driver;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

macro_rules! dev_err {
    ($($arg:tt)*) => {};
}

macro_rules! dev_warn {
    ($($arg:tt)*) => {};
}

macro_rules! dev_dbg_once {
    ($($arg:tt)*) => {};
}

struct mutex_guard {
    lock: *mut mutex,
}

impl mutex_guard {
    unsafe fn new(lock: *mut mutex) -> Self {
        mutex_lock(lock);
        Self { lock }
    }
}

impl Drop for mutex_guard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

unsafe extern "C" fn sof_client_auxdev_release(dev: *mut device) {
    let auxdev = to_auxiliary_dev(dev);
    let cdev = auxiliary_dev_to_sof_client_dev(auxdev);
    let centry = cdev_to_centry(cdev);

    kfree((*cdev).auxdev.dev.platform_data);
    kfree(centry as *mut c_void);
}

unsafe fn sof_client_dev_add_data(cdev: *mut sof_client_dev, data: *const c_void, size: size_t) -> c_int {
    let mut d: *mut c_void = null_mut();

    if !data.is_null() {
        d = kmemdup(data, size, GFP_KERNEL);
        if d.is_null() {
            return -ENOMEM;
        }
    }

    (*cdev).auxdev.dev.platform_data = d;
    0
}

// Original C condition: IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_IPC_FLOOD_TEST)
unsafe fn sof_register_ipc_flood_test(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int;

    if (*(*sdev).pdata).ipc_type != sof_ipc_type::SOF_IPC_TYPE_3 {
        return 0;
    }

    i = 0;
    while i < CONFIG_SND_SOC_SOF_DEBUG_IPC_FLOOD_TEST_NUM {
        ret = sof_client_dev_register(sdev, c"ipc_flood".as_ptr(), i as u32, null_mut(), 0);
        if ret < 0 {
            break;
        }
        i += 1;
    }

    if ret != 0 {
        while i >= 0 {
            sof_client_dev_unregister(sdev, c"ipc_flood".as_ptr(), i as u32);
            i -= 1;
        }
    }

    ret
}

unsafe fn sof_unregister_ipc_flood_test(sdev: *mut snd_sof_dev) {
    let mut i: c_int = 0;

    while i < CONFIG_SND_SOC_SOF_DEBUG_IPC_FLOOD_TEST_NUM {
        sof_client_dev_unregister(sdev, c"ipc_flood".as_ptr(), i as u32);
        i += 1;
    }
}

// Original C fallback when CONFIG_SND_SOC_SOF_DEBUG_IPC_FLOOD_TEST is disabled:
// sof_register_ipc_flood_test() returns 0 and sof_unregister_ipc_flood_test() is empty.

// Original C condition: IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_IPC_MSG_INJECTOR)
unsafe fn sof_register_ipc_msg_injector(sdev: *mut snd_sof_dev) -> c_int {
    sof_client_dev_register(sdev, c"msg_injector".as_ptr(), 0, null_mut(), 0)
}

unsafe fn sof_unregister_ipc_msg_injector(sdev: *mut snd_sof_dev) {
    sof_client_dev_unregister(sdev, c"msg_injector".as_ptr(), 0);
}

// Original C fallback when CONFIG_SND_SOC_SOF_DEBUG_IPC_MSG_INJECTOR is disabled:
// sof_register_ipc_msg_injector() returns 0 and sof_unregister_ipc_msg_injector() is empty.

// Original C condition: IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_IPC_KERNEL_INJECTOR)
unsafe fn sof_register_ipc_kernel_injector(sdev: *mut snd_sof_dev) -> c_int {
    /* Only IPC3 supported right now */
    if (*(*sdev).pdata).ipc_type != sof_ipc_type::SOF_IPC_TYPE_3 {
        return 0;
    }

    sof_client_dev_register(sdev, c"kernel_injector".as_ptr(), 0, null_mut(), 0)
}

unsafe fn sof_unregister_ipc_kernel_injector(sdev: *mut snd_sof_dev) {
    sof_client_dev_unregister(sdev, c"kernel_injector".as_ptr(), 0);
}

// Original C fallback when CONFIG_SND_SOC_SOF_DEBUG_IPC_KERNEL_INJECTOR is disabled:
// sof_register_ipc_kernel_injector() returns 0 and sof_unregister_ipc_kernel_injector() is empty.

#[no_mangle]
pub unsafe extern "C" fn sof_register_clients(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    if (*sdev).dspless_mode_selected {
        return 0;
    }

    /* Register platform independent client devices */
    ret = sof_register_ipc_flood_test(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "IPC flood test client registration failed\n");
        return ret;
    }

    ret = sof_register_ipc_msg_injector(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "IPC message injector client registration failed\n");
        sof_unregister_ipc_flood_test(sdev);
        return ret;
    }

    ret = sof_register_ipc_kernel_injector(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "IPC kernel injector client registration failed\n");
        sof_unregister_ipc_msg_injector(sdev);
        sof_unregister_ipc_flood_test(sdev);
        return ret;
    }

    /* Platform dependent client device registration */
    if !sof_ops(sdev).is_null() && (*sof_ops(sdev)).register_ipc_clients.is_some() {
        ret = (*sof_ops(sdev)).register_ipc_clients.unwrap()(sdev);
    }

    if ret == 0 {
        return 0;
    }

    sof_unregister_ipc_kernel_injector(sdev);
    sof_unregister_ipc_msg_injector(sdev);
    sof_unregister_ipc_flood_test(sdev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_unregister_clients(sdev: *mut snd_sof_dev) {
    if !sof_ops(sdev).is_null() && (*sof_ops(sdev)).unregister_ipc_clients.is_some() {
        (*sof_ops(sdev)).unregister_ipc_clients.unwrap()(sdev);
    }

    sof_unregister_ipc_kernel_injector(sdev);
    sof_unregister_ipc_msg_injector(sdev);
    sof_unregister_ipc_flood_test(sdev);
}

#[no_mangle]
pub unsafe extern "C" fn sof_client_dev_register(
    sdev: *mut snd_sof_dev,
    name: *const c_char,
    id: u32,
    data: *const c_void,
    size: size_t,
) -> c_int {
    let centry: *mut sof_client_dev_entry;
    let auxdev: *mut auxiliary_device;
    let cdev: *mut sof_client_dev;
    let mut ret: c_int;

    centry = kzalloc(size_of::<sof_client_dev_entry>(), GFP_KERNEL) as *mut sof_client_dev_entry;
    if centry.is_null() {
        return -ENOMEM;
    }

    cdev = addr_of_mut!((*centry).client_dev);

    (*centry).sdev = sdev;
    auxdev = addr_of_mut!((*cdev).auxdev);
    (*auxdev).name = name;
    (*auxdev).dev.parent = (*sdev).dev;
    (*auxdev).dev.release = Some(sof_client_auxdev_release);
    (*auxdev).id = id;

    ret = sof_client_dev_add_data(cdev, data, size);
    if ret < 0 {
        kfree(centry as *mut c_void);
        return ret;
    }

    ret = auxiliary_device_init(auxdev);
    if ret < 0 {
        dev_err!((*sdev).dev, "failed to initialize client dev %s.%d\n", name, id);
        kfree((*cdev).auxdev.dev.platform_data);
        kfree(centry as *mut c_void);
        return ret;
    }

    ret = auxiliary_device_add(addr_of_mut!((*cdev).auxdev));
    if ret < 0 {
        dev_err!((*sdev).dev, "failed to add client dev %s.%d\n", name, id);
        /*
         * sof_client_auxdev_release() will be invoked to free up memory
         * allocations through put_device()
         */
        auxiliary_device_uninit(addr_of_mut!((*cdev).auxdev));
        return ret;
    }

    /* add to list of SOF client devices */
    {
        let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));
        list_add(addr_of_mut!((*centry).list), addr_of_mut!((*sdev).ipc_client_list));
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_client_dev_register, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_dev_unregister(sdev: *mut snd_sof_dev, name: *const c_char, id: u32) {
    let mut pos = (*sdev).ipc_client_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    /*
     * sof_client_auxdev_release() will be invoked to free up memory
     * allocations through put_device()
     */
    while pos != addr_of_mut!((*sdev).ipc_client_list) {
        let centry = (pos as *mut u8).sub(offset_of!(sof_client_dev_entry, list)) as *mut sof_client_dev_entry;
        let cdev = addr_of_mut!((*centry).client_dev);
        pos = (*pos).next;

        if strcmp((*cdev).auxdev.name, name) == 0 && (*cdev).auxdev.id == id {
            list_del(addr_of_mut!((*centry).list));
            auxiliary_device_delete(addr_of_mut!((*cdev).auxdev));
            auxiliary_device_uninit(addr_of_mut!((*cdev).auxdev));
            break;
        }
    }
}
// EXPORT_SYMBOL_NS_GPL(sof_client_dev_unregister, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc_tx_message(
    cdev: *mut sof_client_dev,
    ipc_msg: *mut c_void,
    reply_data: *mut c_void,
    reply_bytes: size_t,
) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_3 {
        let hdr = ipc_msg as *mut sof_ipc_cmd_hdr;

        return sof_ipc_tx_message((*sdev).ipc, ipc_msg, (*hdr).size as size_t, reply_data, reply_bytes);
    } else if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        let msg = ipc_msg as *mut sof_ipc4_msg;

        return sof_ipc_tx_message((*sdev).ipc, ipc_msg, (*msg).data_size, reply_data, reply_bytes);
    }

    -EINVAL
}
// EXPORT_SYMBOL_NS_GPL(sof_client_ipc_tx_message, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc_rx_message(
    cdev: *mut sof_client_dev,
    ipc_msg: *mut c_void,
    msg_buf: *mut c_void,
) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    // Original C condition also requires IS_ENABLED(CONFIG_SND_SOC_SOF_IPC3).
    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_3 {
        let hdr = ipc_msg as *mut sof_ipc_cmd_hdr;

        if ((*hdr).size as usize) < size_of::<*mut sof_ipc_cmd_hdr>() {
            dev_err!((*sdev).dev, "The received message size is invalid\n");
            return -EINVAL;
        }

        sof_ipc3_do_rx_work(sdev, ipc_msg, msg_buf);
        return 0;
    }

    -EOPNOTSUPP
}
// EXPORT_SYMBOL_NS_GPL(sof_client_ipc_rx_message, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc_set_get_data(
    cdev: *mut sof_client_dev,
    ipc_msg: *mut c_void,
    set: bool_,
) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_3 {
        let hdr = ipc_msg as *mut sof_ipc_cmd_hdr;

        return sof_ipc_set_get_data((*sdev).ipc, ipc_msg, (*hdr).size as size_t, set);
    } else if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        let msg = ipc_msg as *mut sof_ipc4_msg;

        return sof_ipc_set_get_data((*sdev).ipc, ipc_msg, (*msg).data_size, set);
    }

    -EINVAL
}
// EXPORT_SYMBOL_NS_GPL(sof_client_ipc_set_get_data, "SND_SOC_SOF_CLIENT");

// Original C condition: CONFIG_SND_SOC_SOF_IPC4
#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc4_find_module(
    c: *mut sof_client_dev,
    uuid: *const guid_t,
) -> *mut sof_ipc4_fw_module {
    let sdev = sof_client_dev_to_sof_dev(c);

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        return sof_ipc4_find_module_by_uuid(sdev, uuid);
    }
    dev_err!((*sdev).dev, "Only supported with IPC4\n");

    null_mut()
}
// EXPORT_SYMBOL_NS_GPL(sof_client_ipc4_find_module, "SND_SOC_SOF_CLIENT");

// Original C condition: CONFIG_SND_SOC_SOF_IPC4
#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc4_find_swidget_by_id(
    cdev: *mut sof_client_dev,
    module_id: u32,
    instance_id: c_int,
) -> *mut snd_sof_widget {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        return sof_ipc4_find_swidget_by_ids(sdev, module_id, instance_id);
    }
    dev_err!((*sdev).dev, "Only supported with IPC4\n");

    null_mut()
}
// EXPORT_SYMBOL_NS_GPL(sof_client_ipc4_find_swidget_by_id, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_suspend_clients(sdev: *mut snd_sof_dev, state: pm_message_t) -> c_int {
    let mut pos = (*sdev).ipc_client_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    while pos != addr_of_mut!((*sdev).ipc_client_list) {
        let centry = (pos as *mut u8).sub(offset_of!(sof_client_dev_entry, list)) as *mut sof_client_dev_entry;
        let cdev = addr_of_mut!((*centry).client_dev);
        pos = (*pos).next;

        /* Skip devices without loaded driver */
        if (*cdev).auxdev.dev.driver.is_null() {
            continue;
        }

        let adrv = to_auxiliary_drv((*cdev).auxdev.dev.driver);
        if (*adrv).suspend.is_some() {
            (*adrv).suspend.unwrap()(addr_of_mut!((*cdev).auxdev), state);
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_suspend_clients, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_resume_clients(sdev: *mut snd_sof_dev) -> c_int {
    let mut pos = (*sdev).ipc_client_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    while pos != addr_of_mut!((*sdev).ipc_client_list) {
        let centry = (pos as *mut u8).sub(offset_of!(sof_client_dev_entry, list)) as *mut sof_client_dev_entry;
        let cdev = addr_of_mut!((*centry).client_dev);
        pos = (*pos).next;

        /* Skip devices without loaded driver */
        if (*cdev).auxdev.dev.driver.is_null() {
            continue;
        }

        let adrv = to_auxiliary_drv((*cdev).auxdev.dev.driver);
        if (*adrv).resume.is_some() {
            (*adrv).resume.unwrap()(addr_of_mut!((*cdev).auxdev));
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_resume_clients, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    (*sdev).debugfs_root
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_debugfs_root, "SND_SOC_SOF_CLIENT");

/* DMA buffer allocation in client drivers must use the core SOF device */
#[no_mangle]
pub unsafe extern "C" fn sof_client_get_dma_dev(cdev: *mut sof_client_dev) -> *mut device {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    (*sdev).dev
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_dma_dev, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_get_fw_version(cdev: *mut sof_client_dev) -> *const sof_ipc_fw_version {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    addr_of_mut!((*sdev).fw_ready.version) as *const sof_ipc_fw_version
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_fw_version, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> size_t {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    (*(*sdev).ipc).max_payload_size
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_ipc_max_payload_size, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_get_ipc_type(cdev: *mut sof_client_dev) -> sof_ipc_type {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    (*(*sdev).pdata).ipc_type
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_ipc_type, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int {
    snd_sof_boot_dsp_firmware(sof_client_dev_to_sof_dev(cdev))
}
// EXPORT_SYMBOL_NS_GPL(sof_client_boot_dsp, "SND_SOC_SOF_CLIENT");

/* module refcount management of SOF core */
#[no_mangle]
pub unsafe extern "C" fn sof_client_core_module_get(cdev: *mut sof_client_dev) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    if !try_module_get((*(*(*sdev).dev).driver).owner) {
        return -ENODEV;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_client_core_module_get, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_core_module_put(cdev: *mut sof_client_dev) {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    module_put((*(*(*sdev).dev).driver).owner);
}
// EXPORT_SYMBOL_NS_GPL(sof_client_core_module_put, "SND_SOC_SOF_CLIENT");

/* IPC event handling */
#[no_mangle]
pub unsafe extern "C" fn sof_client_ipc_rx_dispatcher(sdev: *mut snd_sof_dev, msg_buf: *mut c_void) {
    let mut msg_type: u32;

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_3 {
        let hdr = msg_buf as *mut sof_ipc_cmd_hdr;

        msg_type = (*hdr).cmd & SOF_GLB_TYPE_MASK;
    } else if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        let msg = msg_buf as *mut sof_ipc4_msg;

        msg_type = SOF_IPC4_NOTIFICATION_TYPE_GET((*msg).primary);
    } else {
        dev_dbg_once!(
            (*sdev).dev,
            "Not supported IPC version: %d\n",
            (*(*sdev).pdata).ipc_type
        );
        return;
    }

    {
        let mut pos = (*sdev).ipc_rx_handler_list.next;
        let _guard = mutex_guard::new(addr_of_mut!((*sdev).client_event_handler_mutex));
        while pos != addr_of_mut!((*sdev).ipc_rx_handler_list) {
            let event = (pos as *mut u8).sub(offset_of!(sof_ipc_event_entry, list)) as *mut sof_ipc_event_entry;
            pos = (*pos).next;
            if (*event).ipc_msg_type == msg_type {
                if let Some(callback) = (*event).callback {
                    callback((*event).cdev, msg_buf);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_client_register_ipc_rx_handler(
    cdev: *mut sof_client_dev,
    ipc_msg_type: u32,
    callback: sof_client_event_callback,
) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);
    let event: *mut sof_ipc_event_entry;

    if callback.is_none() {
        return -EINVAL;
    }

    if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_3 {
        if (ipc_msg_type & SOF_GLB_TYPE_MASK) == 0 {
            return -EINVAL;
        }
    } else if (*(*sdev).pdata).ipc_type == sof_ipc_type::SOF_IPC_TYPE_4 {
        if (ipc_msg_type & SOF_IPC4_NOTIFICATION_TYPE_MASK) == 0 {
            return -EINVAL;
        }
    } else {
        dev_warn!(
            (*sdev).dev,
            "%s: Not supported IPC version: %d\n",
            c"sof_client_register_ipc_rx_handler".as_ptr(),
            (*(*sdev).pdata).ipc_type
        );
        return -EINVAL;
    }

    event = kmalloc(size_of::<sof_ipc_event_entry>(), GFP_KERNEL) as *mut sof_ipc_event_entry;
    if event.is_null() {
        return -ENOMEM;
    }

    (*event).ipc_msg_type = ipc_msg_type;
    (*event).cdev = cdev;
    (*event).callback = callback;

    /* add to list of SOF client devices */
    {
        let _guard = mutex_guard::new(addr_of_mut!((*sdev).client_event_handler_mutex));
        list_add(addr_of_mut!((*event).list), addr_of_mut!((*sdev).ipc_rx_handler_list));
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_client_register_ipc_rx_handler, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_unregister_ipc_rx_handler(cdev: *mut sof_client_dev, ipc_msg_type: u32) {
    let sdev = sof_client_dev_to_sof_dev(cdev);
    let mut pos = (*sdev).ipc_rx_handler_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    while pos != addr_of_mut!((*sdev).ipc_rx_handler_list) {
        let event = (pos as *mut u8).sub(offset_of!(sof_ipc_event_entry, list)) as *mut sof_ipc_event_entry;
        pos = (*pos).next;
        if (*event).cdev == cdev && (*event).ipc_msg_type == ipc_msg_type {
            list_del(addr_of_mut!((*event).list));
            kfree(event as *mut c_void);
            break;
        }
    }
}
// EXPORT_SYMBOL_NS_GPL(sof_client_unregister_ipc_rx_handler, "SND_SOC_SOF_CLIENT");

/*DSP state notification and query */
#[no_mangle]
pub unsafe extern "C" fn sof_client_fw_state_dispatcher(sdev: *mut snd_sof_dev) {
    let mut pos = (*sdev).fw_state_handler_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    while pos != addr_of_mut!((*sdev).fw_state_handler_list) {
        let event = (pos as *mut u8).sub(offset_of!(sof_state_event_entry, list)) as *mut sof_state_event_entry;
        pos = (*pos).next;
        if let Some(callback) = (*event).callback {
            callback((*event).cdev, (*sdev).fw_state);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_client_register_fw_state_handler(
    cdev: *mut sof_client_dev,
    callback: sof_client_fw_state_callback,
) -> c_int {
    let sdev = sof_client_dev_to_sof_dev(cdev);
    let event: *mut sof_state_event_entry;

    if callback.is_none() {
        return -EINVAL;
    }

    event = kmalloc(size_of::<sof_state_event_entry>(), GFP_KERNEL) as *mut sof_state_event_entry;
    if event.is_null() {
        return -ENOMEM;
    }

    (*event).cdev = cdev;
    (*event).callback = callback;

    /* add to list of SOF client devices */
    {
        let _guard = mutex_guard::new(addr_of_mut!((*sdev).client_event_handler_mutex));
        list_add(addr_of_mut!((*event).list), addr_of_mut!((*sdev).fw_state_handler_list));
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sof_client_register_fw_state_handler, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_unregister_fw_state_handler(cdev: *mut sof_client_dev) {
    let sdev = sof_client_dev_to_sof_dev(cdev);
    let mut pos = (*sdev).fw_state_handler_list.next;
    let _guard = mutex_guard::new(addr_of_mut!((*sdev).ipc_client_mutex));

    while pos != addr_of_mut!((*sdev).fw_state_handler_list) {
        let event = (pos as *mut u8).sub(offset_of!(sof_state_event_entry, list)) as *mut sof_state_event_entry;
        pos = (*pos).next;
        if (*event).cdev == cdev {
            list_del(addr_of_mut!((*event).list));
            kfree(event as *mut c_void);
            break;
        }
    }
}
// EXPORT_SYMBOL_NS_GPL(sof_client_unregister_fw_state_handler, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> sof_fw_state {
    let sdev = sof_client_dev_to_sof_dev(cdev);

    (*sdev).fw_state
}
// EXPORT_SYMBOL_NS_GPL(sof_client_get_fw_state, "SND_SOC_SOF_CLIENT");

#[no_mangle]
pub unsafe extern "C" fn sof_client_dev_to_sof_dev(cdev: *mut sof_client_dev) -> *mut snd_sof_dev {
    let centry = cdev_to_centry(cdev);

    (*centry).sdev
}
// EXPORT_SYMBOL_NS_GPL(sof_client_dev_to_sof_dev, "SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
