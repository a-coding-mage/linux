// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019-2021, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// Linux kernel and local driver dependencies are supplied by the surrounding build.

const PCI_DEVICE_ID_QCOM_AIC080: u16 = 0xa080;
const PCI_DEVICE_ID_QCOM_AIC100: u16 = 0xa100;
const PCI_DEVICE_ID_QCOM_AIC200: u16 = 0xa110;
const QAIC_NAME: &str = "qaic";
const QAIC_DESC: &str = "Qualcomm Cloud AI Accelerators";
const CNTL_MAJOR: i32 = 5;
const CNTL_MINOR: i32 = 0;
const DBC_NUM: usize = 16;

#[repr(C)]
struct qaic_device_config { family: i32, bar_mask: i32, mhi_bar_idx: u32, dbc_bar_idx: u32 }

static AIC080_CONFIG: qaic_device_config = qaic_device_config { family: FAMILY_AIC100, bar_mask: (1 << 0) | (1 << 2) | (1 << 4), mhi_bar_idx: 0, dbc_bar_idx: 2 };
static AIC100_CONFIG: qaic_device_config = qaic_device_config { family: FAMILY_AIC100, bar_mask: (1 << 0) | (1 << 2) | (1 << 4), mhi_bar_idx: 0, dbc_bar_idx: 2 };
static AIC200_CONFIG: qaic_device_config = qaic_device_config { family: FAMILY_AIC200, bar_mask: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 4), mhi_bar_idx: 1, dbc_bar_idx: 2 };

static mut datapath_polling: bool = false;
static mut link_up: bool = false;

unsafe fn qaicm_wq_release(_dev: *mut drm_device, res: *mut core::ffi::c_void) { destroy_workqueue(res as *mut workqueue_struct); }
unsafe fn qaicm_wq_init(dev: *mut drm_device, name: *const i8) -> *mut workqueue_struct {
    let wq = alloc_workqueue(b"%s\0".as_ptr() as *const i8, WQ_UNBOUND, 0, name);
    if wq.is_null() { return ERR_PTR(-ENOMEM); }
    let ret = drmm_add_action_or_reset(dev, qaicm_wq_release, wq as *mut _);
    if ret != 0 { return ERR_PTR(ret); }
    wq
}
unsafe fn qaicm_srcu_release(_dev: *mut drm_device, res: *mut core::ffi::c_void) { cleanup_srcu_struct(res as *mut srcu_struct); }
unsafe fn qaicm_srcu_init(dev: *mut drm_device, lock: *mut srcu_struct) -> i32 {
    let ret = init_srcu_struct(lock); if ret != 0 { return ret; }
    drmm_add_action_or_reset(dev, qaicm_srcu_release, lock as *mut _)
}
unsafe fn qaicm_pci_release(_dev: *mut drm_device, res: *mut core::ffi::c_void) {
    let qdev = to_qaic_device(_dev); pci_set_drvdata((*qdev).pdev, core::ptr::null_mut()); let _ = res;
}
unsafe fn free_usr(kref: *mut kref) {
    let usr = container_of!(kref, qaic_user, ref_count);
    cleanup_srcu_struct(&mut (*usr).qddev_lock); ida_free(&mut qaic_usrs, (*usr).handle); kfree(usr as *mut _);
}
unsafe fn qaic_open(dev: *mut drm_device, file: *mut drm_file) -> i32 {
    let qddev = to_qaic_drm_device(dev); let qdev = (*qddev).qdev; let rcu_id = srcu_read_lock(&mut (*qdev).dev_lock);
    if (*qdev).dev_state != QAIC_ONLINE { srcu_read_unlock(&mut (*qdev).dev_lock, rcu_id); return -ENODEV; }
    let usr = kmalloc_obj::<qaic_user>(); if usr.is_null() { srcu_read_unlock(&mut (*qdev).dev_lock, rcu_id); return -ENOMEM; }
    (*usr).handle = ida_alloc(&mut qaic_usrs, GFP_KERNEL); if (*usr).handle < 0 { let r = (*usr).handle; kfree(usr as *mut _); srcu_read_unlock(&mut (*qdev).dev_lock, rcu_id); return r; }
    (*usr).qddev = qddev; atomic_set(&mut (*usr).chunk_id, 0); init_srcu_struct(&mut (*usr).qddev_lock); kref_init(&mut (*usr).ref_count);
    let ret = mutex_lock_interruptible(&mut (*qddev).users_mutex); if ret != 0 { cleanup_srcu_struct(&mut (*usr).qddev_lock); ida_free(&mut qaic_usrs, (*usr).handle); kfree(usr as *mut _); srcu_read_unlock(&mut (*qdev).dev_lock, rcu_id); return ret; }
    list_add(&mut (*usr).node, &mut (*qddev).users); mutex_unlock(&mut (*qddev).users_mutex); (*file).driver_priv = usr as *mut _; srcu_read_unlock(&mut (*qdev).dev_lock, rcu_id); 0
}
unsafe fn qaic_postclose(dev: *mut drm_device, file: *mut drm_file) {
    let usr = (*file).driver_priv as *mut qaic_user; let qddev = (*usr).qddev; let usr_id = srcu_read_lock(&mut (*usr).qddev_lock);
    if !qddev.is_null() { let qdev = (*qddev).qdev; let dev_id = srcu_read_lock(&mut (*qdev).dev_lock); if (*qdev).dev_state == QAIC_ONLINE { qaic_release_usr(qdev, usr); for i in 0..(*qdev).num_dbc { if !(*qdev).dbc[i as usize].usr.is_null() && (*(*qdev).dbc[i as usize].usr).handle == (*usr).handle { release_dbc(qdev, i); } } } srcu_read_unlock(&mut (*qdev).dev_lock, dev_id); mutex_lock(&mut (*qddev).users_mutex); if !list_empty(&(*usr).node) { list_del_init(&mut (*usr).node); } mutex_unlock(&mut (*qddev).users_mutex); }
    srcu_read_unlock(&mut (*usr).qddev_lock, usr_id); kref_put(&mut (*usr).ref_count, free_usr); (*file).driver_priv = core::ptr::null_mut(); let _ = dev;
}

unsafe fn qaic_create_drm_device(qdev: *mut qaic_device, partition_id: i32) -> i32 {
    if partition_id != QAIC_NO_PARTITION { return -EINVAL; }
    (*(*qdev).qddev).partition_id = partition_id;
    let drm = to_drm((*qdev).qddev); let ret = drm_dev_register(drm, 0);
    if ret != 0 { pci_dbg((*qdev).pdev, b"drm_dev_register failed %d\n\0".as_ptr() as *const i8, ret); return ret; }
    let ret = qaic_sysfs_init((*qdev).qddev); if ret != 0 { drm_dev_unregister(drm); return ret; }
    qaic_debugfs_init((*qdev).qddev); ret
}
unsafe fn qaic_destroy_drm_device(qdev: *mut qaic_device, _partition_id: i32) {
    let qddev = (*qdev).qddev; drm_dev_unregister(to_drm(qddev)); qaic_sysfs_remove(qddev); (*qddev).partition_id = 0;
    mutex_lock(&mut (*qddev).users_mutex); while !list_empty(&(*qddev).users) { let usr = list_first_entry!(&mut (*qddev).users, qaic_user, node); list_del_init(&mut (*usr).node); kref_get(&mut (*usr).ref_count); (*usr).qddev = core::ptr::null_mut(); mutex_unlock(&mut (*qddev).users_mutex); synchronize_srcu(&(*usr).qddev_lock); kref_put(&mut (*usr).ref_count, free_usr); mutex_lock(&mut (*qddev).users_mutex); } mutex_unlock(&mut (*qddev).users_mutex);
}
unsafe fn qaic_dev_reset_clean_local_state(qdev: *mut qaic_device) { qaic_notify_reset(qdev); qaic_clean_up_ssr(qdev); for i in 0..(*qdev).num_dbc { release_dbc(qdev, i); } }
unsafe fn qaic_notify_reset(qdev: *mut qaic_device) { (*qdev).dev_state = QAIC_OFFLINE; wake_all_cntl(qdev); for i in 0..(*qdev).num_dbc { wakeup_dbc(qdev, i); } synchronize_srcu(&(*qdev).dev_lock); }
unsafe fn qaic_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> i32 { let config = (*id).driver_data as *const qaic_device_config; let qdev = create_qdev(pdev, config); if qdev.is_null() { return -ENOMEM; } let ret = init_pci(qdev, pdev, config); if ret != 0 { return ret; } for i in 0..(*qdev).num_dbc { (*qdev).dbc[i as usize].dbc_base = (*qdev).bar_dbc.add(QAIC_DBC_OFF(i) as usize); } let irq = init_msi(qdev, pdev); if irq < 0 { return irq; } qaic_create_drm_device(qdev, QAIC_NO_PARTITION) }
unsafe fn qaic_pci_remove(pdev: *mut pci_dev) { let qdev = pci_get_drvdata(pdev); if qdev.is_null() { return; } qaic_dev_reset_clean_local_state(qdev); qaic_mhi_free_controller((*qdev).mhi_cntrl, link_up); qaic_destroy_drm_device(qdev, QAIC_NO_PARTITION); }
unsafe fn qaic_init() -> i32 { let ret = pci_register_driver(&mut qaic_pci_driver); if ret != 0 { return ret; } let ret = mhi_driver_register(&mut qaic_mhi_driver); if ret != 0 { pci_unregister_driver(&mut qaic_pci_driver); } ret }
unsafe fn qaic_exit() { link_up = true; qaic_ssr_unregister(); qaic_ras_unregister(); qaic_bootlog_unregister(); qaic_timesync_deinit(); sahara_unregister(); mhi_driver_unregister(&mut qaic_mhi_driver); pci_unregister_driver(&mut qaic_pci_driver); }

// External kernel/driver types, constants, helpers, callbacks, and registration macros
// are intentionally referenced from their supplied dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
