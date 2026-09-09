// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */

// Dependencies supplied by the surrounding kernel/Rust translation.

const ADF_VF2PF_RATELIMIT_INTERVAL: u32 = 8;
const ADF_VF2PF_RATELIMIT_BURST: u32 = 130;

static mut pf2vf_resp_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)]
struct adf_pf2vf_resp {
    pf2vf_resp_work: work_struct,
    vf_info: *mut adf_accel_vf_info,
}

unsafe fn adf_iov_send_resp(work: *mut work_struct) {
    let pf2vf_resp = container_of!(work, adf_pf2vf_resp, pf2vf_resp_work);
    let vf_info = (*pf2vf_resp).vf_info;
    let accel_dev = (*vf_info).accel_dev;
    let vf_nr: u32 = (*vf_info).vf_nr;
    let mut ret: bool;

    if READ_ONCE!((*accel_dev).pf.vf2pf_disabled) {
        goto_out!(pf2vf_resp);
    }

    mutex_lock(&mut (*vf_info).pfvf_mig_lock);
    ret = adf_recv_and_handle_vf2pf_msg(accel_dev, vf_nr);
    if ret {
        // re-enable interrupt on PF from this VF
        adf_enable_vf2pf_interrupts(accel_dev, 1u32 << vf_nr);
    }
    mutex_unlock(&mut (*vf_info).pfvf_mig_lock);

    kfree(pf2vf_resp as *mut core::ffi::c_void);
}

pub unsafe fn adf_schedule_vf2pf_handler(vf_info: *mut adf_accel_vf_info) {
    let accel_dev = (*vf_info).accel_dev;
    let pf2vf_resp: *mut adf_pf2vf_resp;

    if READ_ONCE!((*accel_dev).pf.vf2pf_disabled) {
        return;
    }

    pf2vf_resp = kzalloc_obj!(adf_pf2vf_resp, GFP_ATOMIC);
    if pf2vf_resp.is_null() {
        return;
    }

    (*pf2vf_resp).vf_info = vf_info;
    INIT_WORK!(&mut (*pf2vf_resp).pf2vf_resp_work, adf_iov_send_resp);
    queue_work(pf2vf_resp_wq, &mut (*pf2vf_resp).pf2vf_resp_work);
}

unsafe fn adf_flush_pf2vf_resp_wq() {
    if !pf2vf_resp_wq.is_null() {
        flush_workqueue(pf2vf_resp_wq);
    }
}

unsafe fn adf_enable_sriov(accel_dev: *mut adf_accel_dev) -> i32 {
    let pdev = accel_to_pci_dev(accel_dev);
    let totalvfs = pci_sriov_get_totalvfs(pdev);
    let hw_data = (*accel_dev).hw_device;
    let mut vf_info = (*accel_dev).pf.vf_info;

    for i in 0..totalvfs {
        // This ptr will be populated when VFs will be created
        (*vf_info).accel_dev = accel_dev;
        (*vf_info).vf_nr = i;
        mutex_init(&mut (*vf_info).pf2vf_lock);
        mutex_init(&mut (*vf_info).pfvf_mig_lock);
        ratelimit_state_init(&mut (*vf_info).vf2pf_ratelimit,
                             ADF_VF2PF_RATELIMIT_INTERVAL,
                             ADF_VF2PF_RATELIMIT_BURST);
        vf_info = vf_info.add(1);
    }

    // Set Valid bits in AE Thread to PCIe Function Mapping
    if let Some(configure) = (*hw_data).configure_iov_threads {
        configure(accel_dev, true);
    }
    // Enable VF to PF interrupts for all VFs
    adf_enable_all_vf2pf_interrupts(accel_dev, totalvfs);
    // Do not enable SR-IOV if already enabled
    if pci_num_vf(pdev) != 0 {
        return 0;
    }
    // Due to the hardware design, all VFs supported in hardware must be enabled.
    pci_enable_sriov(pdev, totalvfs)
}

unsafe fn adf_add_sriov_configuration(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut val: c_ulong = 0;
    let mut ret = adf_cfg_section_add(accel_dev, ADF_KERNEL_SEC);
    if ret != 0 { return ret; }
    ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_CY, &mut val, ADF_DEC);
    if ret != 0 { return ret; }
    ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_DC, &mut val, ADF_DEC);
    if ret != 0 { return ret; }
    set_bit(ADF_STATUS_CONFIGURED, &mut (*accel_dev).status);
    ret
}

unsafe fn adf_do_disable_sriov(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut ret: i32;
    if adf_dev_in_use(accel_dev) { dev_err!(&GET_DEV!(accel_dev), "Cannot disable SR-IOV, device in use\n"); return -EBUSY; }
    if adf_dev_started(accel_dev) {
        if adf_devmgr_in_reset(accel_dev) { dev_err!(&GET_DEV!(accel_dev), "Cannot disable SR-IOV, device in reset\n"); return -EBUSY; }
        ret = adf_dev_down(accel_dev);
        if ret != 0 { adf_cfg_del_all_except(accel_dev, ADF_GENERAL_SEC); return ret; }
    }
    adf_disable_sriov(accel_dev);
    ret = adf_dev_up(accel_dev, true);
    if ret != 0 { adf_cfg_del_all_except(accel_dev, ADF_GENERAL_SEC); }
    ret
}

unsafe fn adf_do_enable_sriov(accel_dev: *mut adf_accel_dev) -> i32 {
    let pdev = accel_to_pci_dev(accel_dev);
    let totalvfs = pci_sriov_get_totalvfs(pdev);
    let mut val: c_ulong;
    let mut ret: i32;
    if !device_iommu_mapped(&GET_DEV!(accel_dev)) { dev_warn!(&GET_DEV!(accel_dev), "IOMMU should be enabled for SR-IOV to work correctly\n"); }
    if adf_dev_started(accel_dev) {
        if adf_devmgr_in_reset(accel_dev) || adf_dev_in_use(accel_dev) { dev_err!(&GET_DEV!(accel_dev), "Device busy\n"); return -EBUSY; }
        ret = adf_dev_down(accel_dev); if ret != 0 { return ret; }
    }
    ret = adf_add_sriov_configuration(accel_dev); if ret != 0 { adf_cfg_del_all_except(accel_dev, ADF_GENERAL_SEC); return ret; }
    (*accel_dev).pf.vf_info = kzalloc_objs!(adf_accel_vf_info, totalvfs);
    if (*accel_dev).pf.vf_info.is_null() { adf_cfg_del_all_except(accel_dev, ADF_GENERAL_SEC); return -ENOMEM; }
    ret = adf_dev_up(accel_dev, false);
    if ret != 0 { dev_err!(&GET_DEV!(accel_dev), "Failed to start qat_dev%d\n", (*accel_dev).accel_id); adf_dev_down(accel_dev); kfree((*accel_dev).pf.vf_info as *mut c_void); (*accel_dev).pf.vf_info = core::ptr::null_mut(); return ret; }
    ret = adf_enable_sriov(accel_dev);
    if ret != 0 { adf_dev_down(accel_dev); kfree((*accel_dev).pf.vf_info as *mut c_void); (*accel_dev).pf.vf_info = core::ptr::null_mut(); return ret; }
    val = 1; ret = adf_cfg_add_key_value_param(accel_dev, ADF_GENERAL_SEC, ADF_SRIOV_ENABLED, &mut val, ADF_DEC);
    if ret != 0 { adf_dev_down(accel_dev); kfree((*accel_dev).pf.vf_info as *mut c_void); (*accel_dev).pf.vf_info = core::ptr::null_mut(); return ret; }
    totalvfs
}

pub unsafe fn adf_reenable_sriov(accel_dev: *mut adf_accel_dev) {
    let pdev = accel_to_pci_dev(accel_dev);
    let mut cfg = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES as usize];
    if adf_cfg_get_param_value(accel_dev, ADF_GENERAL_SEC, ADF_SRIOV_ENABLED, cfg.as_mut_ptr()) != 0 || (*accel_dev).pf.vf_info.is_null() { return; }
    if adf_add_sriov_configuration(accel_dev) != 0 { return; }
    pci_dbg(pdev, "Re-enabling SRIOV\n");
    adf_enable_sriov(accel_dev);
}

pub unsafe fn adf_disable_sriov(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device;
    let totalvfs = pci_sriov_get_totalvfs(accel_to_pci_dev(accel_dev));
    if (*accel_dev).pf.vf_info.is_null() { return; }
    adf_pf2vf_notify_restarting(accel_dev); adf_pf2vf_wait_for_restarting_complete(accel_dev);
    if !test_bit(ADF_STATUS_RESTARTING, &(*accel_dev).status) { pci_disable_sriov(accel_to_pci_dev(accel_dev)); }
    adf_disable_all_vf2pf_interrupts(accel_dev); adf_isr_sync_ae_cluster(accel_dev); adf_flush_pf2vf_resp_wq();
    if let Some(configure) = (*hw_data).configure_iov_threads { configure(accel_dev, false); }
    let mut vf = (*accel_dev).pf.vf_info;
    for _ in 0..totalvfs { mutex_destroy(&mut (*vf).pf2vf_lock); mutex_destroy(&mut (*vf).pfvf_mig_lock); vf = vf.add(1); }
    if !test_bit(ADF_STATUS_RESTARTING, &(*accel_dev).status) { kfree((*accel_dev).pf.vf_info as *mut c_void); (*accel_dev).pf.vf_info = core::ptr::null_mut(); }
}

pub unsafe fn adf_sriov_configure(pdev: *mut pci_dev, numvfs: i32) -> i32 {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() { dev_err!(&mut (*pdev).dev, "Failed to find accel_dev\n"); return -EFAULT; }
    if numvfs != 0 { adf_do_enable_sriov(accel_dev) } else { adf_do_disable_sriov(accel_dev) }
}

pub unsafe fn adf_init_pf_wq() -> i32 {
    pf2vf_resp_wq = alloc_workqueue!("qat_pf2vf_resp_wq", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if pf2vf_resp_wq.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn adf_exit_pf_wq() {
    if !pf2vf_resp_wq.is_null() { destroy_workqueue(pf2vf_resp_wq); pf2vf_resp_wq = core::ptr::null_mut(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
