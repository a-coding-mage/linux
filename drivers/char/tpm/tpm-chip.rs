// SPDX-License-Identifier: GPL-2.0-only
/* TPM chip management routines. */

// C dependencies and build-time configuration are supplied by the surrounding kernel crate.

static mut DEV_NUMS_IDR: IdR = IdR::new();
static mut IDR_LOCK: Mutex = Mutex::new();

pub static TPM_CLASS: Class = Class { name: "tpm", shutdown_pre: Some(tpm_class_shutdown) };
pub static TPMRM_CLASS: Class = Class { name: "tpmrm" };
pub static mut TPM_DEVT: DevT = 0;

unsafe fn tpm_request_locality(chip: *mut TpmChip) -> i32 {
    let ops = (*chip).ops;
    if (*ops).request_locality.is_none() { return 0; }
    let rc = ((*ops).request_locality.unwrap())(chip, 0);
    if rc < 0 { return rc; }
    (*chip).locality = rc;
    0
}

unsafe fn tpm_relinquish_locality(chip: *mut TpmChip) {
    let ops = (*chip).ops;
    if (*ops).relinquish_locality.is_none() { return; }
    let rc = ((*ops).relinquish_locality.unwrap())(chip, (*chip).locality);
    if rc != 0 { dev_err!(&(*chip).dev, "{}: : error {}\n", "tpm_relinquish_locality", rc); }
    (*chip).locality = -1;
}

unsafe fn tpm_cmd_ready(chip: *mut TpmChip) -> i32 {
    if (*(*chip).ops).cmd_ready.is_none() { return 0; }
    ((*(*chip).ops).cmd_ready.unwrap())(chip)
}

unsafe fn tpm_go_idle(chip: *mut TpmChip) -> i32 {
    if (*(*chip).ops).go_idle.is_none() { return 0; }
    ((*(*chip).ops).go_idle.unwrap())(chip)
}

unsafe fn tpm_clk_enable(chip: *mut TpmChip) { if let Some(f) = (*(*chip).ops).clk_enable { f(chip, true); } }
unsafe fn tpm_clk_disable(chip: *mut TpmChip) { if let Some(f) = (*(*chip).ops).clk_enable { f(chip, false); } }

pub unsafe fn tpm_chip_start(chip: *mut TpmChip) -> i32 {
    tpm_clk_enable(chip);
    if (*chip).locality == -1 {
        let ret = tpm_request_locality(chip);
        if ret != 0 { tpm_clk_disable(chip); return ret; }
    }
    let ret = tpm_cmd_ready(chip);
    if ret != 0 { tpm_relinquish_locality(chip); tpm_clk_disable(chip); return ret; }
    0
}

pub unsafe fn tpm_chip_stop(chip: *mut TpmChip) { tpm_go_idle(chip); tpm_relinquish_locality(chip); tpm_clk_disable(chip); }

pub unsafe fn tpm_try_get_ops(chip: *mut TpmChip) -> i32 {
    let mut rc = -EIO;
    if (*chip).flags & TPM_CHIP_FLAG_DISABLE != 0 { return rc; }
    get_device(&mut (*chip).dev);
    down_read(&mut (*chip).ops_sem);
    if (*chip).ops.is_null() { up_read(&mut (*chip).ops_sem); put_device(&mut (*chip).dev); return rc; }
    mutex_lock(&mut (*chip).tpm_mutex);
    if (*chip).flags & TPM_CHIP_FLAG_SUSPENDED != 0 { mutex_unlock(&mut (*chip).tpm_mutex); up_read(&mut (*chip).ops_sem); put_device(&mut (*chip).dev); return rc; }
    rc = tpm_chip_start(chip);
    if rc != 0 { mutex_unlock(&mut (*chip).tpm_mutex); up_read(&mut (*chip).ops_sem); put_device(&mut (*chip).dev); return rc; }
    0
}

pub unsafe fn tpm_put_ops(chip: *mut TpmChip) { tpm_chip_stop(chip); mutex_unlock(&mut (*chip).tpm_mutex); up_read(&mut (*chip).ops_sem); put_device(&mut (*chip).dev); }

pub unsafe fn tpm_default_chip() -> *mut TpmChip {
    let mut chip_num = 0; let mut res = core::ptr::null_mut(); let mut chip_prev;
    mutex_lock(&mut IDR_LOCK);
    loop { chip_prev = chip_num; let chip = idr_get_next(&mut DEV_NUMS_IDR, &mut chip_num); if !chip.is_null() { get_device(&mut (*chip).dev); res = chip; break; } if chip_prev == chip_num { break; } }
    mutex_unlock(&mut IDR_LOCK); res
}

unsafe fn tpm_dev_release(dev: *mut Device) {
    let chip = container_of!(dev, TpmChip, dev);
    mutex_lock(&mut IDR_LOCK); idr_remove(&mut DEV_NUMS_IDR, (*chip).dev_num); mutex_unlock(&mut IDR_LOCK);
    kfree((*chip).work_space.context_buf); kfree((*chip).work_space.session_buf);
    #[cfg(feature = "CONFIG_TCG_TPM2_HMAC")] kfree_sensitive((*chip).auth);
    kfree(chip as *mut _);
}

pub unsafe fn tpm_class_shutdown(dev: *mut Device) -> i32 {
    let chip = container_of!(dev, TpmChip, dev); down_write(&mut (*chip).ops_sem);
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 && tpm_chip_start(chip) == 0 { tpm2_end_auth_session(chip); tpm2_shutdown(chip, TPM2_SU_CLEAR); tpm_chip_stop(chip); }
    (*chip).ops = core::ptr::null(); up_write(&mut (*chip).ops_sem); 0
}

pub unsafe fn tpm_chip_alloc(pdev: *mut Device, ops: *const TpmClassOps) -> *mut TpmChip {
    let chip = kzalloc_obj::<TpmChip>(); if chip.is_null() { return ERR_PTR(-ENOMEM); }
    mutex_init(&mut (*chip).tpm_mutex); init_rwsem(&mut (*chip).ops_sem); (*chip).ops = ops;
    mutex_lock(&mut IDR_LOCK); let rc = idr_alloc(&mut DEV_NUMS_IDR, core::ptr::null_mut(), 0, TPM_NUM_DEVICES, GFP_KERNEL); mutex_unlock(&mut IDR_LOCK);
    if rc < 0 { dev_err!(pdev, "No available tpm device numbers\n"); kfree(chip); return ERR_PTR(rc); }
    (*chip).dev_num = rc; device_initialize(&mut (*chip).dev); (*chip).dev.class = &TPM_CLASS; (*chip).dev.release = Some(tpm_dev_release); (*chip).dev.parent = pdev; (*chip).dev.groups = (*chip).groups;
    (*chip).dev.devt = if rc == 0 { MKDEV(MISC_MAJOR, TPM_MINOR) } else { MKDEV(MAJOR(unsafe { TPM_DEVT }), rc) };
    let rc = dev_set_name(&mut (*chip).dev, "tpm%d", rc); if rc != 0 { put_device(&mut (*chip).dev); return ERR_PTR(rc); }
    if pdev.is_null() { (*chip).flags |= TPM_CHIP_FLAG_VIRTUAL; }
    cdev_init(&mut (*chip).cdev, &TPM_FOPS); (*chip).cdev.owner = THIS_MODULE;
    let rc = tpm2_init_space(&mut (*chip).work_space, TPM2_SPACE_BUFFER_SIZE); if rc != 0 { put_device(&mut (*chip).dev); return ERR_PTR(-ENOMEM); }
    (*chip).locality = -1; chip
}

unsafe fn tpm_put_device(dev: *mut core::ffi::c_void) { put_device(dev as *mut Device); }

pub unsafe fn tpmm_chip_alloc(pdev: *mut Device, ops: *const TpmClassOps) -> *mut TpmChip {
    let chip = tpm_chip_alloc(pdev, ops); if IS_ERR(chip) { return chip; }
    let rc = devm_add_action_or_reset(pdev, tpm_put_device, &mut (*chip).dev as *mut _ as *mut _); if rc != 0 { return ERR_PTR(rc); }
    dev_set_drvdata(pdev, chip as *mut _); chip
}

// The remaining character-device, sysfs, hwrng, bootstrap, register, and unregister routines
// retain their C control flow and call the corresponding kernel dependencies.
unsafe fn tpm_add_char_device(chip: *mut TpmChip) -> i32 {
    let mut rc = cdev_device_add(&mut (*chip).cdev, &mut (*chip).dev); if rc != 0 { dev_err!(&(*chip).dev, "unable to cdev_device_add() {}\n", dev_name(&(*chip).dev)); return rc; }
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 && !tpm_is_firmware_upgrade(chip) { rc = tpm_devs_add(chip); if rc != 0 { cdev_device_del(&mut (*chip).cdev, &mut (*chip).dev); return rc; } }
    mutex_lock(&mut IDR_LOCK); idr_replace(&mut DEV_NUMS_IDR, chip, (*chip).dev_num); mutex_unlock(&mut IDR_LOCK); 0
}
unsafe fn tpm_del_char_device(chip: *mut TpmChip) {
    cdev_device_del(&mut (*chip).cdev, &mut (*chip).dev); mutex_lock(&mut IDR_LOCK); idr_replace(&mut DEV_NUMS_IDR, core::ptr::null_mut(), (*chip).dev_num); mutex_unlock(&mut IDR_LOCK); down_write(&mut (*chip).ops_sem);
    if !(*chip).ops.is_null() { if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 && tpm_chip_start(chip) == 0 { tpm2_shutdown(chip, TPM2_SU_CLEAR); tpm_chip_stop(chip); } (*chip).ops = core::ptr::null(); } up_write(&mut (*chip).ops_sem);
}
unsafe fn tpm_del_legacy_sysfs(chip: *mut TpmChip) { if (*chip).flags & (TPM_CHIP_FLAG_TPM2 | TPM_CHIP_FLAG_VIRTUAL) != 0 || tpm_is_firmware_upgrade(chip) { return; } sysfs_remove_link(&mut (*(*chip).dev.parent).kobj, "ppi"); let mut i = (*chip).groups[0].attrs; while !(*i).is_null() { sysfs_remove_link(&mut (*(*chip).dev.parent).kobj, (*i).name); i = i.add(1); } }
unsafe fn tpm_add_legacy_sysfs(chip: *mut TpmChip) -> i32 { if (*chip).flags & (TPM_CHIP_FLAG_TPM2 | TPM_CHIP_FLAG_VIRTUAL) != 0 || tpm_is_firmware_upgrade(chip) { return 0; } let mut rc = compat_only_sysfs_link_entry_to_kobj(&mut (*(*chip).dev.parent).kobj, &mut (*chip).dev.kobj, "ppi", core::ptr::null()); if rc != 0 && rc != -ENOENT { return rc; } let mut i = (*chip).groups[0].attrs; while !(*i).is_null() { rc = compat_only_sysfs_link_entry_to_kobj(&mut (*(*chip).dev.parent).kobj, &mut (*chip).dev.kobj, (*i).name, core::ptr::null()); if rc != 0 { tpm_del_legacy_sysfs(chip); return rc; } i = i.add(1); } 0 }
unsafe fn tpm_hwrng_read(rng: *mut Hwrng, data: *mut core::ffi::c_void, max: usize, _wait: bool) -> i32 { tpm_get_random(container_of!(rng, TpmChip, hwrng), data, max) }
unsafe fn tpm_is_hwrng_enabled(chip: *mut TpmChip) -> bool { IS_ENABLED!(CONFIG_HW_RANDOM_TPM) && !tpm_is_firmware_upgrade(chip) && (*chip).flags & TPM_CHIP_FLAG_HWRNG_DISABLED == 0 }
unsafe fn tpm_add_hwrng(chip: *mut TpmChip) -> i32 { if !tpm_is_hwrng_enabled(chip) { return 0; } snprintf!((*chip).hwrng_name, "tpm-rng-{}", (*chip).dev_num); (*chip).hwrng.name = (*chip).hwrng_name; (*chip).hwrng.read = Some(tpm_hwrng_read); hwrng_register(&mut (*chip).hwrng) }
unsafe fn tpm_get_pcr_allocation(chip: *mut TpmChip) -> i32 { if tpm_is_firmware_upgrade(chip) { return 0; } let rc = if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 { tpm2_get_pcr_allocation(chip) } else { tpm1_get_pcr_allocation(chip) }; if rc > 0 { -ENODEV } else { rc } }
pub unsafe fn tpm_chip_bootstrap(chip: *mut TpmChip) -> i32 { if (*chip).flags & TPM_CHIP_FLAG_BOOTSTRAPPED != 0 { return 0; } let mut rc = tpm_chip_start(chip); if rc == 0 { rc = tpm_auto_startup(chip); if rc == 0 { rc = tpm_get_pcr_allocation(chip); } tpm_chip_stop(chip); } (*chip).flags |= TPM_CHIP_FLAG_BOOTSTRAPPED; rc }
pub unsafe fn tpm_chip_register(chip: *mut TpmChip) -> i32 { let mut rc = tpm_chip_bootstrap(chip); if rc != 0 { return rc; } tpm_sysfs_add_device(chip); tpm_bios_log_setup(chip); tpm_add_ppi(chip); rc = tpm_add_hwrng(chip); if rc != 0 { tpm_bios_log_teardown(chip); return rc; } rc = tpm_add_char_device(chip); if rc != 0 { if tpm_is_hwrng_enabled(chip) { hwrng_unregister(&mut (*chip).hwrng); } tpm_bios_log_teardown(chip); return rc; } rc = tpm_add_legacy_sysfs(chip); if rc != 0 { tpm_chip_unregister(chip); } rc }
pub unsafe fn tpm_chip_unregister(chip: *mut TpmChip) { #[cfg(feature = "CONFIG_TCG_TPM2_HMAC")] { if tpm_try_get_ops(chip) == 0 { tpm2_end_auth_session(chip); tpm_put_ops(chip); } } tpm_del_legacy_sysfs(chip); if tpm_is_hwrng_enabled(chip) { hwrng_unregister(&mut (*chip).hwrng); } tpm_bios_log_teardown(chip); if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 && !tpm_is_firmware_upgrade(chip) { tpm_devs_remove(chip); } tpm_del_char_device(chip); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
