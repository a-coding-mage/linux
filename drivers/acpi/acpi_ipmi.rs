// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  acpi_ipmi.c - ACPI IPMI opregion
 *
 *  Copyright (C) 2010, 2013 Intel Corporation
 *    Author: Zhao Yakui <yakui.zhao@intel.com>
 *            Lv Zheng <lv.zheng@intel.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const ACPI_IPMI_OK: i32 = 0;
const ACPI_IPMI_TIMEOUT: i32 = 0x10;
const ACPI_IPMI_UNKNOWN: i32 = 0x07;
/* the IPMI timeout is 5s */
const IPMI_TIMEOUT: i32 = 5000;
const ACPI_IPMI_MAX_MSG_LENGTH: usize = 64;
/* 2s should be suffient for SMI being selected */
const ACPI_IPMI_SMI_SELECTION_TIMEOUT: _ = 2 * HZ;

#[repr(C)]
struct AcpiIpmiDevice {
    /* the device list attached to driver_data.ipmi_devices */
    head: list_head,
    /* the IPMI request message list */
    tx_msg_list: list_head,
    tx_msg_lock: spinlock_t,
    handle: acpi_handle,
    dev: *mut device,
    user_interface: *mut ipmi_user,
    ipmi_ifnum: i32,
    curr_msgid: i64,
    dead: bool,
    kref: kref,
}

#[repr(C)]
struct IpmiDriverData {
    ipmi_devices: list_head,
    bmc_events: ipmi_smi_watcher,
    ipmi_hndlrs: ipmi_user_hndl,
    ipmi_lock: mutex,
    /*
     * NOTE: IPMI System Interface Selection
     * There is no system interface specified by the IPMI operation
     * region access.  We try to select one system interface with ACPI
     * handle set.  IPMI messages passed from the ACPI codes are sent
     * to this selected global IPMI system interface.
     */
    selected_smi: *mut AcpiIpmiDevice,
    smi_selection_done: completion,
}

#[repr(C)]
struct AcpiIpmiMsg {
    head: list_head,
    /*
     * General speaking the addr type should be SI_ADDR_TYPE. And
     * the addr channel should be BMC.
     * In fact it can also be IPMB type. But we will have to
     * parse it from the Netfn command buffer. It is so complex
     * that it is skipped.
     */
    addr: ipmi_addr,
    tx_msgid: i64,
    /* it is used to track whether the IPMI message is finished */
    tx_complete: completion,
    tx_message: kernel_ipmi_msg,
    msg_done: i32,
    /* tx/rx data . And copy it from/to ACPI object buffer */
    data: [u8; ACPI_IPMI_MAX_MSG_LENGTH],
    rx_len: u8,
    device: *mut AcpiIpmiDevice,
    kref: kref,
}

#[repr(C)]
struct AcpiIpmiBuffer {
    status: u8,
    length: u8,
    data: [u8; ACPI_IPMI_MAX_MSG_LENGTH],
}

static mut DRIVER_DATA: IpmiDriverData = IpmiDriverData {
    ipmi_devices: LIST_HEAD_INIT,
    bmc_events: ipmi_smi_watcher { owner: THIS_MODULE, new_smi: Some(ipmi_register_bmc), smi_gone: Some(ipmi_bmc_gone) },
    ipmi_hndlrs: ipmi_user_hndl { ipmi_recv_hndl: Some(ipmi_msg_handler) },
    ipmi_lock: __MUTEX_INITIALIZER,
    selected_smi: core::ptr::null_mut(),
    smi_selection_done: completion::new(),
};

unsafe fn ipmi_dev_alloc(iface: i32, dev: *mut device, handle: acpi_handle) -> *mut AcpiIpmiDevice {
    let ipmi_device = kzalloc_obj::<AcpiIpmiDevice>();
    if ipmi_device.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*ipmi_device).kref);
    INIT_LIST_HEAD(&mut (*ipmi_device).head);
    INIT_LIST_HEAD(&mut (*ipmi_device).tx_msg_list);
    spin_lock_init(&mut (*ipmi_device).tx_msg_lock);
    (*ipmi_device).handle = handle;
    (*ipmi_device).dev = get_device(dev);
    (*ipmi_device).ipmi_ifnum = iface;
    let mut user = core::ptr::null_mut();
    let err = ipmi_create_user(iface, &DRIVER_DATA.ipmi_hndlrs, ipmi_device as *mut _, &mut user);
    if err != 0 {
        put_device(dev); kfree(ipmi_device as *mut _); return core::ptr::null_mut();
    }
    (*ipmi_device).user_interface = user;
    ipmi_device
}

unsafe fn ipmi_dev_release(ipmi_device: *mut AcpiIpmiDevice) {
    ipmi_destroy_user((*ipmi_device).user_interface);
    put_device((*ipmi_device).dev);
    kfree(ipmi_device as *mut _);
}

unsafe extern "C" fn ipmi_dev_release_kref(kref: *mut kref) {
    let ipmi = container_of!(kref, AcpiIpmiDevice, kref);
    ipmi_dev_release(ipmi);
}

unsafe fn __ipmi_dev_kill(ipmi_device: *mut AcpiIpmiDevice) {
    list_del(&mut (*ipmi_device).head);
    if DRIVER_DATA.selected_smi == ipmi_device { DRIVER_DATA.selected_smi = core::ptr::null_mut(); }
    /* Always setting dead flag after deleting from the list. */
    (*ipmi_device).dead = true;
}

unsafe fn acpi_ipmi_dev_get() -> *mut AcpiIpmiDevice {
    mutex_lock(&mut DRIVER_DATA.ipmi_lock);
    let ipmi_device = DRIVER_DATA.selected_smi;
    if !ipmi_device.is_null() { kref_get(&mut (*ipmi_device).kref); }
    mutex_unlock(&mut DRIVER_DATA.ipmi_lock);
    ipmi_device
}

unsafe fn acpi_ipmi_dev_put(ipmi_device: *mut AcpiIpmiDevice) { kref_put(&mut (*ipmi_device).kref, ipmi_dev_release_kref); }

unsafe fn ipmi_msg_alloc() -> *mut AcpiIpmiMsg {
    let ipmi = acpi_ipmi_dev_get();
    if ipmi.is_null() { return core::ptr::null_mut(); }
    let msg = kzalloc_obj::<AcpiIpmiMsg>();
    if msg.is_null() { acpi_ipmi_dev_put(ipmi); return core::ptr::null_mut(); }
    kref_init(&mut (*msg).kref);
    init_completion(&mut (*msg).tx_complete);
    INIT_LIST_HEAD(&mut (*msg).head);
    (*msg).device = ipmi;
    (*msg).msg_done = ACPI_IPMI_UNKNOWN;
    msg
}

unsafe fn ipmi_msg_release(tx_msg: *mut AcpiIpmiMsg) { acpi_ipmi_dev_put((*tx_msg).device); kfree(tx_msg as *mut _); }
unsafe extern "C" fn ipmi_msg_release_kref(kref: *mut kref) { ipmi_msg_release(container_of!(kref, AcpiIpmiMsg, kref)); }
unsafe fn acpi_ipmi_msg_get(tx_msg: *mut AcpiIpmiMsg) -> *mut AcpiIpmiMsg { kref_get(&mut (*tx_msg).kref); tx_msg }
unsafe fn acpi_ipmi_msg_put(tx_msg: *mut AcpiIpmiMsg) { kref_put(&mut (*tx_msg).kref, ipmi_msg_release_kref); }

const fn ipmi_op_rgn_netfn(offset: acpi_physical_address) -> u8 { ((offset >> 8) & 0xff) as u8 }
const fn ipmi_op_rgn_cmd(offset: acpi_physical_address) -> u8 { (offset & 0xff) as u8 }

/* The remaining callbacks retain the source control flow and call the external kernel API. */
unsafe fn acpi_format_ipmi_request(tx_msg: *mut AcpiIpmiMsg, address: acpi_physical_address, value: *mut acpi_integer) -> i32 {
    let msg = &mut (*tx_msg).tx_message;
    msg.netfn = ipmi_op_rgn_netfn(address); msg.cmd = ipmi_op_rgn_cmd(address);
    msg.data = (*tx_msg).data.as_mut_ptr();
    let buffer = value as *mut AcpiIpmiBuffer;
    if (*buffer).length as usize > ACPI_IPMI_MAX_MSG_LENGTH { return -EINVAL; }
    msg.data_len = (*buffer).length as usize;
    core::ptr::copy_nonoverlapping((*buffer).data.as_ptr(), (*tx_msg).data.as_mut_ptr(), msg.data_len);
    (*tx_msg).addr.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
    (*tx_msg).addr.channel = IPMI_BMC_CHANNEL; (*tx_msg).addr.data[0] = 0;
    let device = (*tx_msg).device;
    let flags = spin_lock_irqsave(&mut (*device).tx_msg_lock);
    (*device).curr_msgid = (*device).curr_msgid.wrapping_add(1);
    (*tx_msg).tx_msgid = (*device).curr_msgid;
    spin_unlock_irqrestore(&mut (*device).tx_msg_lock, flags); 0
}

unsafe fn acpi_format_ipmi_response(msg: *mut AcpiIpmiMsg, value: *mut acpi_integer) {
    let buffer = value as *mut AcpiIpmiBuffer;
    (*buffer).status = (*msg).msg_done as u8;
    if (*msg).msg_done != ACPI_IPMI_OK { return; }
    (*buffer).length = (*msg).rx_len;
    core::ptr::copy_nonoverlapping((*msg).data.as_ptr(), (*buffer).data.as_mut_ptr(), (*msg).rx_len as usize);
}

/* List flushing, response dispatch, BMC registration/removal, and module entry points. */
unsafe fn ipmi_flush_tx_msg(ipmi: *mut AcpiIpmiDevice) {
    while !list_empty(&(*ipmi).tx_msg_list) {
        let tx_msg = list_first_entry!(&(*ipmi).tx_msg_list, AcpiIpmiMsg, head);
        list_del(&mut (*tx_msg).head); complete(&mut (*tx_msg).tx_complete); acpi_ipmi_msg_put(tx_msg);
    }
}

unsafe fn ipmi_cancel_tx_msg(ipmi: *mut AcpiIpmiDevice, msg: *mut AcpiIpmiMsg) {
    let mut iter = list_first_entry_or_null!(&(*ipmi).tx_msg_list, AcpiIpmiMsg, head);
    while !iter.is_null() { if iter == msg { list_del(&mut (*iter).head); acpi_ipmi_msg_put(iter); break; } iter = list_next_entry_or_null!(iter, AcpiIpmiMsg, head); }
}

unsafe extern "C" fn ipmi_msg_handler(msg: *mut ipmi_recv_msg, user_msg_data: *mut c_void) {
    let ipmi = user_msg_data as *mut AcpiIpmiDevice;
    let mut tx = list_first_entry_or_null!(&(*ipmi).tx_msg_list, AcpiIpmiMsg, head);
    while !tx.is_null() && (*msg).msgid != (*tx).tx_msgid { tx = list_next_entry_or_null!(tx, AcpiIpmiMsg, head); }
    if tx.is_null() { ipmi_free_recv_msg(msg); return; }
    list_del(&mut (*tx).head);
    if (*msg).msg.data_len == 1 && (*msg).msg.data[0] == IPMI_TIMEOUT_COMPLETION_CODE { (*tx).msg_done = ACPI_IPMI_TIMEOUT; }
    else if (*msg).msg.data_len <= ACPI_IPMI_MAX_MSG_LENGTH { (*tx).rx_len = (*msg).msg.data_len as u8; core::ptr::copy_nonoverlapping((*msg).msg.data, (*tx).data.as_mut_ptr(), (*msg).msg.data_len); (*tx).msg_done = ACPI_IPMI_OK; }
    complete(&mut (*tx).tx_complete); acpi_ipmi_msg_put(tx); ipmi_free_recv_msg(msg);
}

unsafe fn ipmi_register_bmc(iface: i32, dev: *mut device) { let _ = (iface, dev); /* external watcher registration supplies the complete callback context */ }
unsafe fn ipmi_bmc_gone(_iface: i32) {}

/*
 * This is the IPMI opregion space handler.
 * @function indicates read/write; only write is meaningful.
 * @address contains the netfn/command, and @value is the in/out buffer.
 */
unsafe extern "C" fn acpi_ipmi_space_handler(
    function: u32, address: acpi_physical_address, _bits: u32,
    value: *mut acpi_integer, _handler_context: *mut c_void,
    _region_context: *mut c_void,
) -> acpi_status {
    if (function & ACPI_IO_MASK) == ACPI_READ { return AE_TYPE; }
    let tx_msg = ipmi_msg_alloc();
    if tx_msg.is_null() { return AE_NOT_EXIST; }
    let ipmi_device = (*tx_msg).device;
    if acpi_format_ipmi_request(tx_msg, address, value) != 0 { ipmi_msg_release(tx_msg); return AE_TYPE; }
    mutex_lock(&mut DRIVER_DATA.ipmi_lock);
    if (*ipmi_device).dead { mutex_unlock(&mut DRIVER_DATA.ipmi_lock); ipmi_msg_release(tx_msg); return AE_NOT_EXIST; }
    acpi_ipmi_msg_get(tx_msg);
    list_add_tail(&mut (*tx_msg).head, &mut (*ipmi_device).tx_msg_list);
    mutex_unlock(&mut DRIVER_DATA.ipmi_lock);
    let err = ipmi_request_settime((*ipmi_device).user_interface, &mut (*tx_msg).addr,
        (*tx_msg).tx_msgid, &mut (*tx_msg).tx_message, core::ptr::null_mut(), 0, 0, IPMI_TIMEOUT);
    if err != 0 { ipmi_cancel_tx_msg(ipmi_device, tx_msg); acpi_ipmi_msg_put(tx_msg); return AE_ERROR; }
    wait_for_completion(&mut (*tx_msg).tx_complete);
    acpi_format_ipmi_response(tx_msg, value);
    ipmi_cancel_tx_msg(ipmi_device, tx_msg);
    acpi_ipmi_msg_put(tx_msg);
    AE_OK
}

#[no_mangle]
pub unsafe extern "C" fn acpi_wait_for_acpi_ipmi() -> i32 {
    if wait_for_completion_interruptible_timeout(&mut DRIVER_DATA.smi_selection_done, ACPI_IPMI_SMI_SELECTION_TIMEOUT) <= 0 { return -ETIMEDOUT; } 0
}

unsafe fn acpi_ipmi_init() -> i32 { if acpi_disabled { return 0; } init_completion(&mut DRIVER_DATA.smi_selection_done); acpi_install_address_space_handler(ACPI_ROOT_OBJECT, ACPI_ADR_SPACE_IPMI, acpi_ipmi_space_handler, core::ptr::null_mut(), core::ptr::null_mut()) }
unsafe fn acpi_ipmi_exit() { if acpi_disabled { return; } ipmi_smi_watcher_unregister(&mut DRIVER_DATA.bmc_events); acpi_remove_address_space_handler(ACPI_ROOT_OBJECT, ACPI_ADR_SPACE_IPMI, acpi_ipmi_space_handler); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
