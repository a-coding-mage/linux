// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Linux kernel dependencies and symbols from the accompanying driver headers
// are intentionally left as external dependencies.

#[repr(C)]
pub struct adf_fatal_error_data {
    pub accel_dev: *mut adf_accel_dev,
    pub work: work_struct,
}

static mut device_reset_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut device_sriov_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn reset_prepare(pdev: *mut pci_dev) -> pci_ers_result_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);

    if accel_dev.is_null() {
        pci_err(pdev, "Can't find acceleration device\n");
        return PCI_ERS_RESULT_DISCONNECT;
    }

    if !adf_dev_started(accel_dev) {
        return PCI_ERS_RESULT_CAN_RECOVER;
    }

    set_bit(ADF_STATUS_RESTARTING, &mut (*accel_dev).status);
    if let Some(exit_arb) = (*(*accel_dev).hw_device).exit_arb {
        dev_dbg(&mut (*pdev).dev, "Disabling arbitration\n");
        exit_arb(accel_dev);
    }
    adf_dev_restarting_notify(accel_dev);
    adf_dev_down(accel_dev);

    PCI_ERS_RESULT_NEED_RESET
}

unsafe fn reset_done(pdev: *mut pci_dev) -> pci_ers_result_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    let res: i32;

    if accel_dev.is_null() {
        pci_err(pdev, "Can't find acceleration device\n");
        return PCI_ERS_RESULT_DISCONNECT;
    }

    if !adf_devmgr_in_reset(accel_dev) {
        return reset_complete(pdev);
    }

    pci_restore_state(pdev);
    res = adf_dev_up(accel_dev, false);
    if res != 0 && res != -EALREADY {
        return PCI_ERS_RESULT_DISCONNECT;
    }

    adf_reenable_sriov(accel_dev);
    adf_pf2vf_notify_restarted(accel_dev);
    adf_dev_restarted_notify(accel_dev);
    clear_bit(ADF_STATUS_RESTARTING, &mut (*accel_dev).status);

    reset_complete(pdev)
}

unsafe fn reset_complete(pdev: *mut pci_dev) -> pci_ers_result_t {
    pci_info(pdev, "Device reset completed successfully\n");
    PCI_ERS_RESULT_RECOVERED
}

unsafe fn adf_error_detected(pdev: *mut pci_dev, state: pci_channel_state_t) -> pci_ers_result_t {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);

    pci_info(pdev, "Acceleration driver hardware error detected.\n");
    if accel_dev.is_null() {
        pci_err(pdev, "Can't find acceleration device\n");
        return PCI_ERS_RESULT_DISCONNECT;
    }
    if state == pci_channel_io_perm_failure {
        pci_err(pdev, "Can't recover from device error\n");
        return PCI_ERS_RESULT_DISCONNECT;
    }
    adf_error_notifier(accel_dev);
    adf_pf2vf_notify_fatal_error(accel_dev);
    reset_prepare(pdev)
}

#[repr(C)]
pub struct adf_reset_dev_data {
    pub mode: i32,
    pub accel_dev: *mut adf_accel_dev,
    pub compl: completion,
    pub reset_work: work_struct,
}

#[repr(C)]
pub struct adf_sriov_dev_data {
    pub accel_dev: *mut adf_accel_dev,
    pub compl: completion,
    pub sriov_work: work_struct,
}

pub unsafe fn adf_reset_sbr(accel_dev: *mut adf_accel_dev) {
    let pdev = accel_to_pci_dev(accel_dev);
    let mut parent = (*(*pdev).bus).self_;
    let mut bridge_ctl: u16 = 0;
    if parent.is_null() { parent = pdev; }
    if !pci_wait_for_pending_transaction(pdev) { pci_info(pdev, "Transaction still in progress. Proceeding\n"); }
    pci_info(pdev, "Secondary bus reset\n");
    pci_read_config_word(parent, PCI_BRIDGE_CONTROL, &mut bridge_ctl);
    bridge_ctl |= PCI_BRIDGE_CTL_BUS_RESET;
    pci_write_config_word(parent, PCI_BRIDGE_CONTROL, bridge_ctl);
    msleep(100);
    bridge_ctl &= !PCI_BRIDGE_CTL_BUS_RESET;
    pci_write_config_word(parent, PCI_BRIDGE_CONTROL, bridge_ctl);
    msleep(100);
}

pub unsafe fn adf_reset_flr(accel_dev: *mut adf_accel_dev) { pcie_flr(accel_to_pci_dev(accel_dev)); }

pub unsafe fn adf_dev_restore(accel_dev: *mut adf_accel_dev) {
    let hw_device = (*accel_dev).hw_device;
    let pdev = accel_to_pci_dev(accel_dev);
    if let Some(reset_device) = (*hw_device).reset_device {
        dev_info(GET_DEV(accel_dev), "Resetting device qat_dev%d\n", (*accel_dev).accel_id);
        reset_device(accel_dev);
        pci_restore_state(pdev);
    }
}

pub unsafe fn adf_set_bme(accel_dev: *mut adf_accel_dev) { pci_set_master(accel_to_pci_dev(accel_dev)); }

unsafe fn adf_device_sriov_worker(work: *mut work_struct) {
    let sriov_data = container_of!(work, adf_sriov_dev_data, sriov_work);
    adf_reenable_sriov((*sriov_data).accel_dev);
    complete(&mut (*sriov_data).compl);
}

unsafe fn adf_device_reset_worker(work: *mut work_struct) {
    let reset_data = container_of!(work, adf_reset_dev_data, reset_work);
    let accel_dev = (*reset_data).accel_dev;
    let wait_jiffies = msecs_to_jiffies(10000);
    let mut sriov_data: adf_sriov_dev_data = core::mem::zeroed();
    adf_dev_restarting_notify(accel_dev);
    if adf_dev_restart(accel_dev) != 0 {
        dev_err(GET_DEV(accel_dev), "Restart device failed\n");
        if (*reset_data).mode == ADF_DEV_RESET_ASYNC { kfree(reset_data); }
        WARN(1, "QAT: device restart failed. Device is unusable\n");
        return;
    }
    sriov_data.accel_dev = accel_dev;
    init_completion(&mut sriov_data.compl);
    INIT_WORK(&mut sriov_data.sriov_work, adf_device_sriov_worker);
    queue_work(device_sriov_wq, &mut sriov_data.sriov_work);
    if wait_for_completion_timeout(&mut sriov_data.compl, wait_jiffies) != 0 { adf_pf2vf_notify_restarted(accel_dev); }
    else { cancel_work_sync(&mut sriov_data.sriov_work); }
    adf_dev_restarted_notify(accel_dev);
    clear_bit(ADF_STATUS_RESTARTING, &mut (*accel_dev).status);
    if (*reset_data).mode == ADF_DEV_RESET_ASYNC { kfree(reset_data); }
    else { complete(&mut (*reset_data).compl); }
}

unsafe fn adf_dev_aer_schedule_reset(accel_dev: *mut adf_accel_dev, mode: i32) -> i32 {
    if !adf_dev_started(accel_dev) || test_and_set_bit(ADF_STATUS_RESTARTING, &mut (*accel_dev).status) != 0 { return 0; }
    let reset_data = kzalloc_obj::<adf_reset_dev_data>();
    if reset_data.is_null() { clear_bit(ADF_STATUS_RESTARTING, &mut (*accel_dev).status); return -ENOMEM; }
    (*reset_data).accel_dev = accel_dev;
    init_completion(&mut (*reset_data).compl);
    (*reset_data).mode = mode;
    INIT_WORK(&mut (*reset_data).reset_work, adf_device_reset_worker);
    queue_work(device_reset_wq, &mut (*reset_data).reset_work);
    if mode == ADF_DEV_RESET_SYNC {
        let timeout = wait_for_completion_timeout(&mut (*reset_data).compl, msecs_to_jiffies(10000));
        let mut ret = 0;
        if timeout == 0 { dev_err(GET_DEV(accel_dev), "Reset device timeout expired\n"); cancel_work_sync(&mut (*reset_data).reset_work); ret = -EFAULT; }
        kfree(reset_data);
        return ret;
    }
    0
}

unsafe fn adf_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t { reset_done(pdev) }
unsafe fn adf_resume(pdev: *mut pci_dev) { pci_info(pdev, "Acceleration driver reset completed\n"); pci_info(pdev, "Device is up and running\n"); }
unsafe fn adf_reset_prepare(pdev: *mut pci_dev) { reset_prepare(pdev); }
unsafe fn adf_reset_done(pdev: *mut pci_dev) { reset_done(pdev); }

#[repr(C)]
pub struct pci_error_handlers { pub error_detected: Option<unsafe fn(*mut pci_dev, pci_channel_state_t) -> pci_ers_result_t>, pub slot_reset: Option<unsafe fn(*mut pci_dev) -> pci_ers_result_t>, pub resume: Option<unsafe fn(*mut pci_dev)>, pub reset_prepare: Option<unsafe fn(*mut pci_dev)>, pub reset_done: Option<unsafe fn(*mut pci_dev)> }

pub static adf_err_handler: pci_error_handlers = pci_error_handlers { error_detected: Some(adf_error_detected), slot_reset: Some(adf_slot_reset), resume: Some(adf_resume), reset_prepare: Some(adf_reset_prepare), reset_done: Some(adf_reset_done) };

unsafe fn adf_dev_autoreset(accel_dev: *mut adf_accel_dev) -> i32 { if (*accel_dev).autoreset_on_error { adf_dev_aer_schedule_reset(accel_dev, ADF_DEV_RESET_ASYNC) } else { 0 } }

unsafe fn adf_notify_fatal_error_worker(work: *mut work_struct) {
    let wq_data = container_of!(work, adf_fatal_error_data, work);
    let accel_dev = (*wq_data).accel_dev;
    let hw_device = (*accel_dev).hw_device;
    adf_error_notifier(accel_dev);
    if !(*accel_dev).is_vf {
        if (*accel_dev).autoreset_on_error { if let Some(exit_arb) = (*hw_device).exit_arb { exit_arb(accel_dev); } }
        if !(*accel_dev).pf.vf_info.is_null() { adf_pf2vf_notify_fatal_error(accel_dev); }
        adf_dev_autoreset(accel_dev);
    }
    kfree(wq_data);
}

pub unsafe fn adf_notify_fatal_error(accel_dev: *mut adf_accel_dev) -> i32 {
    let wq_data = kzalloc_obj::<adf_fatal_error_data>();
    if wq_data.is_null() { return -ENOMEM; }
    (*wq_data).accel_dev = accel_dev;
    INIT_WORK(&mut (*wq_data).work, adf_notify_fatal_error_worker);
    adf_misc_wq_queue_work(&mut (*wq_data).work);
    0
}

pub unsafe fn adf_init_aer() -> i32 {
    device_reset_wq = alloc_workqueue("qat_device_reset_wq", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if device_reset_wq.is_null() { return -EFAULT; }
    device_sriov_wq = alloc_workqueue("qat_device_sriov_wq", WQ_PERCPU, 0);
    if device_sriov_wq.is_null() { destroy_workqueue(device_reset_wq); device_reset_wq = core::ptr::null_mut(); return -EFAULT; }
    0
}

pub unsafe fn adf_exit_aer() {
    if !device_reset_wq.is_null() { destroy_workqueue(device_reset_wq); }
    device_reset_wq = core::ptr::null_mut();
    if !device_sriov_wq.is_null() { destroy_workqueue(device_sriov_wq); }
    device_sriov_wq = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
