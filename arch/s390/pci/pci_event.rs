// SPDX-License-Identifier: GPL-2.0
/* Direct translation of pci_event.c; declarations supplied by kernel dependencies are external. */

#[repr(C, packed)]
pub struct zpci_ccdf_avail {
    pub reserved1: u32, pub fh: u32, pub fid: u32, pub reserved2: u32,
    pub reserved3: u32, pub reserved4: u32, pub reserved5: u32,
    pub reserved6: u16, pub pec: u16,
}

#[inline]
unsafe fn ers_result_indicates_abort(ers_res: pci_ers_result_t) -> bool {
    match ers_res {
        PCI_ERS_RESULT_CAN_RECOVER | PCI_ERS_RESULT_RECOVERED |
        PCI_ERS_RESULT_NEED_RESET | PCI_ERS_RESULT_NONE => false,
        _ => true,
    }
}

unsafe fn is_driver_supported(driver: *mut pci_driver) -> bool {
    !driver.is_null() && !(*driver).err_handler.is_null() &&
        (*(*driver).err_handler).error_detected.is_some()
}

unsafe fn zpci_store_pci_error(pdev: *mut pci_dev, ccdf: *const zpci_ccdf_err) -> c_int {
    let zdev = to_zpci(pdev);
    let _guard = mutex_guard(&mut (*zdev).pending_errs_lock);
    if !(*zdev).pending_errs.mediated_recovery { return -EINVAL; }
    if (*zdev).pending_errs.count >= ZPCI_ERR_PENDING_MAX {
        dev_warn_ratelimited(&mut (*pdev).dev, "%s: Maximum number (%d) of pending error events queued\n",
                             pci_name(pdev), ZPCI_ERR_PENDING_MAX);
        return -ENOMEM;
    }
    let i = (*zdev).pending_errs.tail % ZPCI_ERR_PENDING_MAX;
    memcpy(&mut (*zdev).pending_errs.err[i] as *mut _, ccdf, core::mem::size_of::<zpci_ccdf_err>());
    (*zdev).pending_errs.tail += 1;
    (*zdev).pending_errs.count += 1;
    0
}

pub unsafe fn zpci_get_pending_error(zdev: *mut zpci_dev, ccdf: *mut zpci_ccdf_err) -> c_int {
    let _guard = mutex_guard(&mut (*zdev).pending_errs_lock);
    if (*zdev).pending_errs.count == 0 { return -ENOMSG; }
    let head = (*zdev).pending_errs.head % ZPCI_ERR_PENDING_MAX;
    memcpy(ccdf, &(*zdev).pending_errs.err[head], core::mem::size_of::<zpci_ccdf_err>());
    (*zdev).pending_errs.head += 1; (*zdev).pending_errs.count -= 1; 0
}

pub unsafe fn zpci_start_mediated_recovery(zdev: *mut zpci_dev) {
    let _guard = mutex_guard(&mut (*zdev).pending_errs_lock);
    (*zdev).pending_errs.mediated_recovery = true;
}

pub unsafe fn zpci_stop_mediated_recovery(zdev: *mut zpci_dev) {
    let _guard = mutex_guard(&mut (*zdev).pending_errs_lock);
    (*zdev).pending_errs.mediated_recovery = false;
    if (*zdev).pending_errs.count != 0 { pr_info("Unhandled PCI error events count=%d for PCI function 0x%x\n", (*zdev).pending_errs.count, (*zdev).fid); }
    memset(&mut (*zdev).pending_errs, 0, core::mem::size_of::<zpci_ccdf_pending>());
}

unsafe fn zpci_event_notify_error_detected(pdev: *mut pci_dev, driver: *mut pci_driver) -> pci_ers_result_t {
    let ers_res = ((*(*driver).err_handler).error_detected.unwrap())(pdev, (*pdev).error_state);
    pci_uevent_ers(pdev, ers_res);
    if ers_result_indicates_abort(ers_res) { pr_info("%s: Automatic recovery failed after initial reporting\n", pci_name(pdev)); }
    else if ers_res == PCI_ERS_RESULT_NEED_RESET { pr_debug("%s: Driver needs reset to recover\n", pci_name(pdev)); }
    ers_res
}

unsafe fn zpci_event_do_error_state_clear(pdev: *mut pci_dev, driver: *mut pci_driver) -> pci_ers_result_t {
    let zdev = to_zpci(pdev); if !zdev_enabled(zdev) { return PCI_ERS_RESULT_NEED_RESET; }
    pr_info("%s: Unblocking device access for examination\n", pci_name(pdev));
    if zpci_reset_load_store_blocked(zdev) != 0 { pr_err("%s: Unblocking device access failed\n", pci_name(pdev)); return PCI_ERS_RESULT_NEED_RESET; }
    let ers_res = if let Some(f) = (*(*driver).err_handler).mmio_enabled { f(pdev) } else { PCI_ERS_RESULT_NONE };
    if ers_result_indicates_abort(ers_res) { pr_info("%s: Automatic recovery failed after MMIO re-enable\n", pci_name(pdev)); return ers_res; }
    if ers_res == PCI_ERS_RESULT_NEED_RESET { pr_debug("%s: Driver needs reset to recover\n", pci_name(pdev)); return ers_res; }
    pr_debug("%s: Unblocking DMA\n", pci_name(pdev));
    if zpci_clear_error_state(zdev) == 0 { (*pdev).error_state = pci_channel_io_normal; } else { pr_err("%s: Unblocking DMA failed\n", pci_name(pdev)); return PCI_ERS_RESULT_NEED_RESET; }
    ers_res
}

unsafe fn zpci_event_do_reset(pdev: *mut pci_dev, driver: *mut pci_driver) -> pci_ers_result_t {
    pr_info("%s: Initiating reset\n", pci_name(pdev));
    if zpci_hot_reset_device(to_zpci(pdev)) != 0 { pr_err("%s: The reset request failed\n", pci_name(pdev)); return PCI_ERS_RESULT_DISCONNECT; }
    (*pdev).error_state = pci_channel_io_normal;
    let ers_res = if let Some(f) = (*(*driver).err_handler).slot_reset { f(pdev) } else { PCI_ERS_RESULT_NONE };
    if ers_result_indicates_abort(ers_res) { pr_info("%s: Automatic recovery failed after slot reset\n", pci_name(pdev)); }
    ers_res
}

unsafe fn zpci_event_attempt_error_recovery(pdev: *mut pci_dev, ccdf: *mut zpci_ccdf_err) -> pci_ers_result_t {
    let zdev = to_zpci(pdev); let mut ers_res = PCI_ERS_RESULT_DISCONNECT; let mut mediated = false;
    let mut status_str = "success"; device_lock(&mut (*pdev).dev);
    if (*pdev).error_state == pci_channel_io_perm_failure { device_unlock(&mut (*pdev).dev); return ers_res; }
    (*pdev).error_state = pci_channel_io_frozen;
    let driver = to_pci_driver((*pdev).dev.driver);
    if !is_driver_supported(driver) { status_str = if driver.is_null() { "failed (no driver)" } else { "failed (no driver support)" }; device_unlock(&mut (*pdev).dev); zpci_report_status(zdev, "recovery", status_str); return ers_res; }
    let rc = zpci_store_pci_error(pdev, ccdf); if rc == 0 || rc == -ENOMEM { mediated = true; }
    ers_res = zpci_event_notify_error_detected(pdev, driver);
    if ers_result_indicates_abort(ers_res) { status_str = "failed (abort on detection)"; }
    else if mediated { pr_info("%s: Leaving recovery of pass-through device to user-space\n", pci_name(pdev)); ers_res = PCI_ERS_RESULT_RECOVERED; status_str = "in progress"; }
    else { if ers_res != PCI_ERS_RESULT_NEED_RESET { ers_res = zpci_event_do_error_state_clear(pdev, driver); }
        if !ers_result_indicates_abort(ers_res) { if ers_res == PCI_ERS_RESULT_NEED_RESET { ers_res = zpci_event_do_reset(pdev, driver); } if ers_res == PCI_ERS_RESULT_NONE { ers_res = PCI_ERS_RESULT_RECOVERED; } }
        if ers_res != PCI_ERS_RESULT_RECOVERED { pci_uevent_ers(pdev, PCI_ERS_RESULT_DISCONNECT); pr_err("%s: Automatic recovery failed; operator intervention is required\n", pci_name(pdev)); status_str = "failed (driver can't recover)"; }
        else { pr_info("%s: The device is ready to resume operations\n", pci_name(pdev)); if let Some(f) = (*(*driver).err_handler).resume { f(pdev); } pci_uevent_ers(pdev, PCI_ERS_RESULT_RECOVERED); }
    }
    device_unlock(&mut (*pdev).dev); zpci_report_status(zdev, "recovery", status_str); ers_res
}

unsafe fn zpci_event_io_failure(pdev: *mut pci_dev, es: pci_channel_state_t, ccdf: *mut zpci_ccdf_err) {
    pci_dev_lock(pdev); (*pdev).error_state = es; zpci_store_pci_error(pdev, ccdf);
    let driver = to_pci_driver((*pdev).dev.driver); if !driver.is_null() && !(*driver).err_handler.is_null() { if let Some(f) = (*(*driver).err_handler).error_detected { f(pdev, es); } }
    pci_dev_unlock(pdev);
}

unsafe fn __zpci_event_print_error(pdev: *mut pci_dev, ccdf: *mut zpci_ccdf_err) { pr_err("%s: Event 0x%x reports an error for PCI function 0x%x\n", if pdev.is_null() { "n/a" } else { pci_name(pdev) }, (*ccdf).pec, (*ccdf).fid); }

unsafe fn __zpci_event_error(ccdf: *mut zpci_ccdf_err) {
    let zdev = get_zdev_by_fid((*ccdf).fid); if zdev.is_null() { return __zpci_event_print_error(core::ptr::null_mut(), ccdf); }
    let mut fh = 0; let mut pdev = core::ptr::null_mut(); mutex_lock(&mut (*zdev).state_lock);
    if clp_refresh_fh((*zdev).fid, &mut fh) != 0 || fh == 0 || (*ccdf).fh != fh { mutex_unlock(&mut (*zdev).state_lock); zpci_zdev_put(zdev); return; }
    zpci_update_fh(zdev, (*ccdf).fh); if !(*zdev).zbus.is_null() && !(*(*zdev).zbus).bus.is_null() { pdev = pci_get_slot((*(*zdev).zbus).bus, (*zdev).devfn); }
    __zpci_event_print_error(pdev, ccdf); if !pdev.is_null() { match (*ccdf).pec { 0x002a..=0x002c => {}, 0x0040 | 0x003b => zpci_event_io_failure(pdev, pci_channel_io_perm_failure, ccdf), _ => { if zpci_event_attempt_error_recovery(pdev, ccdf) != PCI_ERS_RESULT_RECOVERED { zpci_event_io_failure(pdev, pci_channel_io_perm_failure, ccdf); } } } pci_dev_put(pdev); }
    mutex_unlock(&mut (*zdev).state_lock); zpci_zdev_put(zdev);
}

pub unsafe fn zpci_event_error(data: *mut core::ffi::c_void) { if zpci_is_enabled() { __zpci_event_error(data as *mut zpci_ccdf_err); } }

unsafe fn zpci_event_hard_deconfigured(zdev: *mut zpci_dev, fh: u32) { zpci_update_fh(zdev, fh); zpci_bus_remove_device(zdev, true); if zdev_enabled(zdev) { zpci_disable_device(zdev); } (*zdev).state = ZPCI_FN_STATE_STANDBY; }
unsafe fn zpci_event_reappear(zdev: *mut zpci_dev) { (*zdev).state = ZPCI_FN_STATE_STANDBY; zpci_zdev_get(zdev); zpci_dbg(1, "rea fid:%x, fh:%x\n", (*zdev).fid, (*zdev).fh); }
unsafe fn zpci_event_avail_any_device(ccdf: *mut zpci_ccdf_avail) -> bool { if (*ccdf).pec != 0x0306 { return false; } zpci_remove_reserved_devices(); zpci_scan_devices(); true }

unsafe fn zpci_event_avail_new_device(ccdf: *mut zpci_ccdf_avail) { match (*ccdf).pec { 0x0301 => { let z = zpci_create_device((*ccdf).fid, (*ccdf).fh, ZPCI_FN_STATE_CONFIGURED); if !IS_ERR(z) { if zpci_add_device(z) != 0 { kfree(z); } else { zpci_scan_configured_device(z, (*ccdf).fh); } } }, 0x0302 => { let z = zpci_create_device((*ccdf).fid, (*ccdf).fh, ZPCI_FN_STATE_STANDBY); if !IS_ERR(z) { if zpci_add_device(z) != 0 { kfree(z); } } }, _ => {} } }

pub unsafe fn zpci_event_availability(data: *mut core::ffi::c_void) { let ccdf = data as *mut zpci_ccdf_avail; if !zpci_is_enabled() || zpci_event_avail_any_device(ccdf) { return; } let zdev = get_zdev_by_fid((*ccdf).fid); if zdev.is_null() { return zpci_event_avail_new_device(ccdf); } mutex_lock(&mut (*zdev).state_lock); match (*ccdf).pec { 0x0301 => { if (*zdev).state == ZPCI_FN_STATE_RESERVED { zpci_event_reappear(zdev); } if (*zdev).state == ZPCI_FN_STATE_STANDBY { (*zdev).state = ZPCI_FN_STATE_CONFIGURED; zpci_scan_configured_device(zdev, (*ccdf).fh); } }, 0x0302 => { if (*zdev).state == ZPCI_FN_STATE_RESERVED { zpci_event_reappear(zdev); } zpci_update_fh(zdev, (*ccdf).fh); }, 0x0303 => { if (*zdev).state == ZPCI_FN_STATE_CONFIGURED { zpci_update_fh(zdev, (*ccdf).fh); zpci_deconfigure_device(zdev); } }, 0x0304 => { if (*zdev).state == ZPCI_FN_STATE_CONFIGURED { zpci_event_hard_deconfigured(zdev, (*ccdf).fh); } let mut state = ZPCI_FN_STATE_STANDBY; if clp_get_state((*zdev).fid, &mut state) == 0 && state == ZPCI_FN_STATE_RESERVED { zpci_device_reserved(zdev); } }, 0x0308 => zpci_device_reserved(zdev), _ => {} } mutex_unlock(&mut (*zdev).state_lock); zpci_zdev_put(zdev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
