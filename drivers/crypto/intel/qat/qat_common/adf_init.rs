// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Kernel and project dependencies are supplied by the surrounding translation unit.

static mut service_table: list_head = LIST_HEAD_INIT(service_table);
static mut service_lock: mutex = DEFINE_MUTEX(service_lock);

unsafe fn adf_service_add(service: *mut service_hndl) {
    mutex_lock(&mut service_lock);
    list_add(&mut (*service).list, &mut service_table);
    mutex_unlock(&mut service_lock);
}

pub unsafe fn adf_service_register(service: *mut service_hndl) -> i32 {
    memset((*service).init_status.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*service).init_status));
    memset((*service).start_status.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*service).start_status));
    adf_service_add(service);
    0
}

unsafe fn adf_service_remove(service: *mut service_hndl) {
    mutex_lock(&mut service_lock);
    list_del(&mut (*service).list);
    mutex_unlock(&mut service_lock);
}

pub unsafe fn adf_service_unregister(service: *mut service_hndl) -> i32 {
    for i in 0..(*service).init_status.len() {
        if (*service).init_status[i] || (*service).start_status[i] {
            pr_err!("QAT: Could not remove active service\n");
            return -EFAULT;
        }
    }
    adf_service_remove(service);
    0
}

/* adf_dev_init() - Init data structures and services for the given accel device. */
unsafe fn adf_dev_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    if hw_data.is_null() {
        dev_err!(GET_DEV(accel_dev), "Failed to init device - hw_data not set\n");
        return -EFAULT;
    }
    adf_set_bme(accel_dev);
    if !test_bit(ADF_STATUS_CONFIGURED, &(*accel_dev).status) && !(*accel_dev).is_vf {
        dev_err!(GET_DEV(accel_dev), "Device not configured\n"); return -EFAULT;
    }
    if adf_init_etr_data(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed initialize etr\n"); return -EFAULT; }
    if let Some(f) = (*hw_data).init_device { if f(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to initialize device\n"); return -EFAULT; } }
    if let Some(f) = (*hw_data).init_admin_comms { if f(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed initialize admin comms\n"); return -EFAULT; } }
    if let Some(f) = (*hw_data).init_arb { if f(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed initialize hw arbiter\n"); return -EFAULT; } }
    if let Some(f) = (*hw_data).get_ring_to_svc_map { (*hw_data).ring_to_svc_map = f(accel_dev); }
    if adf_ae_init(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to initialise Acceleration Engine\n"); return -EFAULT; }
    set_bit(ADF_STATUS_AE_INITIALISED, &mut (*accel_dev).status);
    if adf_ae_fw_load(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to load acceleration FW\n"); return -EFAULT; }
    set_bit(ADF_STATUS_AE_UCODE_LOADED, &mut (*accel_dev).status);
    if (*hw_data).alloc_irq(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to allocate interrupts\n"); return -EFAULT; }
    set_bit(ADF_STATUS_IRQ_ALLOCATED, &mut (*accel_dev).status);
    if let Some(f) = (*hw_data).ras_ops.enable_ras_errors { f(accel_dev); }
    ((*hw_data).enable_ints)(accel_dev); ((*hw_data).enable_error_correction)(accel_dev);
    let ret = ((*hw_data).pfvf_ops.enable_comms)(accel_dev); if ret != 0 { return ret; }
    if !test_bit(ADF_STATUS_CONFIGURED, &(*accel_dev).status) && (*accel_dev).is_vf && qat_crypto_vf_dev_config(accel_dev) != 0 { return -EFAULT; }
    adf_heartbeat_init(accel_dev);
    let ret = adf_rl_init(accel_dev); if ret != 0 && ret != -EOPNOTSUPP { return ret; }
    let ret = adf_tl_init(accel_dev); if ret != 0 && ret != -EOPNOTSUPP { return ret; }
    mutex_lock(&mut service_lock);
    list_for_each_entry!(service, &mut service_table, list, {
        if service.event_hld(accel_dev, ADF_EVENT_INIT) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to initialise service %s\n", service.name); mutex_unlock(&mut service_lock); return -EFAULT; }
        set_bit((*accel_dev).accel_id, service.init_status);
    });
    mutex_unlock(&mut service_lock); 0
}

/* adf_dev_start() - Start acceleration service for the given accel device. */
unsafe fn adf_dev_start(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = (*accel_dev).hw_device; let mut ret: i32; let caps: u32;
    set_bit(ADF_STATUS_STARTING, &mut (*accel_dev).status);
    if adf_ae_start(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "AE Start Failed\n"); return -EFAULT; }
    set_bit(ADF_STATUS_AE_STARTED, &mut (*accel_dev).status);
    if ((*hw_data).send_admin_init)(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to send init message\n"); return -EFAULT; }
    if let Some(f) = (*hw_data).measure_clock { ret = f(accel_dev); if ret != 0 { dev_err!(GET_DEV(accel_dev), "Failed measure device clock\n"); return ret; } }
    if let Some(f) = (*hw_data).set_ssm_wdtimer { f(accel_dev); }
    if let Some(f) = (*hw_data).enable_pm { if f(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to configure Power Management\n"); return -EFAULT; } }
    ret = adf_enable_kpt(accel_dev); if ret != 0 { dev_err!(GET_DEV(accel_dev), "Failed to enable KPT\n"); return ret; }
    if let Some(f) = (*hw_data).start_timer { ret = f(accel_dev); if ret != 0 { dev_err!(GET_DEV(accel_dev), "Failed to start internal sync timer\n"); return ret; } }
    adf_heartbeat_start(accel_dev); ret = adf_rl_start(accel_dev); if ret != 0 && ret != -EOPNOTSUPP { return ret; }
    ret = adf_tl_start(accel_dev); if ret != 0 && ret != -EOPNOTSUPP { return ret; }
    mutex_lock(&mut service_lock);
    list_for_each_entry!(service, &mut service_table, list, { if service.event_hld(accel_dev, ADF_EVENT_START) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to start service %s\n", service.name); mutex_unlock(&mut service_lock); return -EFAULT; } set_bit((*accel_dev).accel_id, service.start_status); });
    mutex_unlock(&mut service_lock); clear_bit(ADF_STATUS_STARTING, &mut (*accel_dev).status); set_bit(ADF_STATUS_STARTED, &mut (*accel_dev).status);
    if !list_empty(&(*accel_dev).crypto_list) && (qat_algs_register() != 0 || qat_asym_algs_register() != 0) { dev_err!(GET_DEV(accel_dev), "Failed to register crypto algs\n"); set_bit(ADF_STATUS_STARTING, &mut (*accel_dev).status); clear_bit(ADF_STATUS_STARTED, &mut (*accel_dev).status); return -EFAULT; }
    set_bit(ADF_STATUS_CRYPTO_ALGS_REGISTERED, &mut (*accel_dev).status); caps = (*hw_data).accel_capabilities_ext_mask;
    if !list_empty(&(*accel_dev).compression_list) && qat_comp_algs_register(caps) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to register compression algs\n"); set_bit(ADF_STATUS_STARTING, &mut (*accel_dev).status); clear_bit(ADF_STATUS_STARTED, &mut (*accel_dev).status); return -EFAULT; }
    set_bit(ADF_STATUS_COMP_ALGS_REGISTERED, &mut (*accel_dev).status); adf_dbgfs_add(accel_dev); adf_sysfs_start_ras(accel_dev); adf_sysfs_start_arb(accel_dev); 0
}

unsafe fn adf_dev_stop(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device; let mut wait = false;
    if !adf_dev_started(accel_dev) && !test_bit(ADF_STATUS_STARTING, &(*accel_dev).status) { return; }
    adf_tl_stop(accel_dev); adf_rl_stop(accel_dev); adf_dbgfs_rm(accel_dev); adf_sysfs_stop_ras(accel_dev); adf_sysfs_stop_arb(accel_dev); clear_bit(ADF_STATUS_STARTING, &mut (*accel_dev).status); clear_bit(ADF_STATUS_STARTED, &mut (*accel_dev).status);
    if !list_empty(&(*accel_dev).crypto_list) && test_bit(ADF_STATUS_CRYPTO_ALGS_REGISTERED, &(*accel_dev).status) { qat_algs_unregister(); qat_asym_algs_unregister(); } clear_bit(ADF_STATUS_CRYPTO_ALGS_REGISTERED, &mut (*accel_dev).status);
    if !list_empty(&(*accel_dev).compression_list) && test_bit(ADF_STATUS_COMP_ALGS_REGISTERED, &(*accel_dev).status) { qat_comp_algs_unregister((*hw_data).accel_capabilities_ext_mask); } clear_bit(ADF_STATUS_COMP_ALGS_REGISTERED, &mut (*accel_dev).status);
    mutex_lock(&mut service_lock); list_for_each_entry!(service, &mut service_table, list, { if !test_bit((*accel_dev).accel_id, service.start_status) { continue; } let ret = service.event_hld(accel_dev, ADF_EVENT_STOP); if ret == 0 || ret == -EAGAIN { clear_bit((*accel_dev).accel_id, service.start_status); if ret == -EAGAIN { wait = true; } } }); mutex_unlock(&mut service_lock);
    if let Some(f) = (*hw_data).stop_timer { f(accel_dev); } ((*hw_data).disable_iov)(accel_dev); if wait { msleep(100); } if test_bit(ADF_STATUS_AE_STARTED, &(*accel_dev).status) { if adf_ae_stop(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "failed to stop AE\n"); } else { clear_bit(ADF_STATUS_AE_STARTED, &mut (*accel_dev).status); } }
}

unsafe fn adf_dev_shutdown(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device; if hw_data.is_null() { dev_err!(GET_DEV(accel_dev), "QAT: Failed to shutdown device - hw_data not set\n"); return; }
    if test_bit(ADF_STATUS_AE_UCODE_LOADED, &(*accel_dev).status) { adf_ae_fw_release(accel_dev); clear_bit(ADF_STATUS_AE_UCODE_LOADED, &mut (*accel_dev).status); }
    if test_bit(ADF_STATUS_AE_INITIALISED, &(*accel_dev).status) { if adf_ae_shutdown(accel_dev) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to shutdown Accel Engine\n"); } else { clear_bit(ADF_STATUS_AE_INITIALISED, &mut (*accel_dev).status); } }
    mutex_lock(&mut service_lock); list_for_each_entry!(service, &mut service_table, list, { if test_bit((*accel_dev).accel_id, service.init_status) { if service.event_hld(accel_dev, ADF_EVENT_SHUTDOWN) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to shutdown service %s\n", service.name); } else { clear_bit((*accel_dev).accel_id, service.init_status); } } }); mutex_unlock(&mut service_lock);
    adf_rl_exit(accel_dev); if let Some(f) = (*hw_data).ras_ops.disable_ras_errors { f(accel_dev); } adf_heartbeat_shutdown(accel_dev); adf_tl_shutdown(accel_dev); if test_bit(ADF_STATUS_IRQ_ALLOCATED, &(*accel_dev).status) { ((*hw_data).free_irq)(accel_dev); clear_bit(ADF_STATUS_IRQ_ALLOCATED, &mut (*accel_dev).status); } if !test_bit(ADF_STATUS_RESTARTING, &(*accel_dev).status) { adf_cfg_del_all_except(accel_dev, ADF_GENERAL_SEC); } if let Some(f) = (*hw_data).exit_arb { f(accel_dev); } if let Some(f) = (*hw_data).exit_admin_comms { f(accel_dev); } adf_cleanup_etr_data(accel_dev); adf_misc_wq_flush(); adf_dev_restore(accel_dev);
}

pub unsafe fn adf_dev_restarting_notify(accel_dev: *mut adf_accel_dev) -> i32 { mutex_lock(&mut service_lock); list_for_each_entry!(service, &mut service_table, list, { if service.event_hld(accel_dev, ADF_EVENT_RESTARTING) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to restart service %s.\n", service.name); } }); mutex_unlock(&mut service_lock); 0 }
pub unsafe fn adf_dev_restarted_notify(accel_dev: *mut adf_accel_dev) -> i32 { mutex_lock(&mut service_lock); list_for_each_entry!(service, &mut service_table, list, { if service.event_hld(accel_dev, ADF_EVENT_RESTARTED) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to restart service %s.\n", service.name); } }); mutex_unlock(&mut service_lock); 0 }
pub unsafe fn adf_error_notifier(accel_dev: *mut adf_accel_dev) { mutex_lock(&mut service_lock); list_for_each_entry!(service, &mut service_table, list, { if service.event_hld(accel_dev, ADF_EVENT_FATAL_ERROR) != 0 { dev_err!(GET_DEV(accel_dev), "Failed to send error event to %s.\n", service.name); } }); mutex_unlock(&mut service_lock); }

pub unsafe fn adf_dev_down(accel_dev: *mut adf_accel_dev) -> i32 { if accel_dev.is_null() { return -EINVAL; } mutex_lock(&mut (*accel_dev).state_lock); adf_dev_stop(accel_dev); adf_dev_shutdown(accel_dev); mutex_unlock(&mut (*accel_dev).state_lock); 0 }

pub unsafe fn adf_dev_up(accel_dev: *mut adf_accel_dev, config: bool) -> i32 {
    if accel_dev.is_null() { return -EINVAL; } mutex_lock(&mut (*accel_dev).state_lock); let mut ret = 0;
    if adf_dev_started(accel_dev) { dev_info!(GET_DEV(accel_dev), "Device qat_dev%d already up\n", (*accel_dev).accel_id); ret = -EALREADY; } else { if config { if let Some(f) = (*GET_HW_DATA(accel_dev)).dev_config { ret = f(accel_dev); } } if ret == 0 { ret = adf_dev_init(accel_dev); } if ret == 0 { ret = adf_dev_start(accel_dev); } }
    mutex_unlock(&mut (*accel_dev).state_lock); ret
}

pub unsafe fn adf_dev_restart(accel_dev: *mut adf_accel_dev) -> i32 { if accel_dev.is_null() { return -EFAULT; } adf_dev_down(accel_dev); let ret = adf_dev_up(accel_dev, false); if ret == -EALREADY { 0 } else { ret } }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
