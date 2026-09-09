// SPDX-License-Identifier: GPL-2.0

/* Kernel dependencies supplied by the surrounding translation unit. */

/*
 * firmware fallback mechanism
 */

/*
 * use small loading timeout for caching devices' firmware because all these
 * firmware images have been loaded successfully at lease once, also system is
 * ready for completing firmware loading now. The maximum size of firmware in
 * current distributions is about 2M bytes, so 10 secs should be enough.
 */
pub unsafe fn fw_fallback_set_cache_timeout() {
    fw_fallback_config.old_timeout = __firmware_loading_timeout();
    __fw_fallback_set_timeout(10);
}

/* Restores the timeout to the value last configured during normal operation */
pub unsafe fn fw_fallback_set_default_timeout() {
    __fw_fallback_set_timeout(fw_fallback_config.old_timeout);
}

unsafe fn firmware_loading_timeout() -> libc::c_long {
    if __firmware_loading_timeout() > 0 {
        __firmware_loading_timeout() * HZ
    } else {
        MAX_JIFFY_OFFSET
    }
}

#[inline]
unsafe fn fw_sysfs_wait_timeout(fw_priv: *mut fw_priv, timeout: libc::c_long) -> libc::c_int {
    __fw_state_wait_common(fw_priv, timeout)
}

static mut pending_fw_head: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

pub unsafe fn kill_pending_fw_fallback_reqs(kill_all: bool) {
    let mut fw_priv: *mut fw_priv;
    let mut next: *mut fw_priv;

    mutex_lock(&mut fw_lock);
    list_for_each_entry_safe!(fw_priv, next, &mut pending_fw_head, pending_list, {
        if kill_all || (*fw_priv).need_uevent {
            __fw_load_abort(fw_priv);
        }
    });

    if kill_all {
        fw_load_abort_all = true;
    }

    mutex_unlock(&mut fw_lock);
}

/**
 * fw_load_sysfs_fallback() - load a firmware via the sysfs fallback mechanism
 * @fw_sysfs: firmware sysfs information for the firmware to load
 * @timeout: timeout to wait for the load
 *
 * In charge of constructing a sysfs fallback interface for firmware loading.
 **/
unsafe fn fw_load_sysfs_fallback(fw_sysfs: *mut fw_sysfs, mut timeout: libc::c_long) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let f_dev: *mut device = &mut (*fw_sysfs).dev;
    let fw_priv: *mut fw_priv = (*fw_sysfs).fw_priv;

    /* fall back on userspace loading */
    if (*fw_priv).data.is_null() {
        (*fw_priv).is_paged_buf = true;
    }

    dev_set_uevent_suppress(f_dev, true);

    retval = device_add(f_dev);
    if retval != 0 {
        dev_err(f_dev, "%s: device_register failed\n", __func__);
        goto err_put_dev;
    }

    mutex_lock(&mut fw_lock);
    if fw_load_abort_all || fw_state_is_aborted(fw_priv) {
        mutex_unlock(&mut fw_lock);
        retval = -EINTR;
        goto out;
    }

    /*
     * device_add() exposes the loading interface before pending_list is
     * linked into pending_fw_head, so fw_state_done() may run first.
     */
    if fw_state_is_done(fw_priv) {
        mutex_unlock(&mut fw_lock);
        goto out;
    }

    list_add(&mut (*fw_priv).pending_list, &mut pending_fw_head);
    mutex_unlock(&mut fw_lock);

    if (*fw_priv).opt_flags & FW_OPT_UEVENT != 0 {
        (*fw_priv).need_uevent = true;
        dev_set_uevent_suppress(f_dev, false);
        dev_dbg(f_dev, "firmware: requesting %s\n", (*fw_priv).fw_name);
        kobject_uevent(&mut (*fw_sysfs).dev.kobj, KOBJ_ADD);
    } else {
        timeout = MAX_JIFFY_OFFSET;
    }

    retval = fw_sysfs_wait_timeout(fw_priv, timeout);
    if retval < 0 && retval != -ENOENT {
        mutex_lock(&mut fw_lock);
        fw_load_abort(fw_sysfs);
        mutex_unlock(&mut fw_lock);
    }

    if fw_state_is_aborted(fw_priv) {
        if retval == -ERESTARTSYS {
            retval = -EINTR;
        }
    } else if (*fw_priv).is_paged_buf && (*fw_priv).data.is_null() {
        retval = -ENOMEM;
    }

out:
    device_del(f_dev);
err_put_dev:
    put_device(f_dev);
    retval
}

unsafe fn fw_load_from_user_helper(
    firmware: *mut firmware,
    name: *const libc::c_char,
    device: *mut device,
    opt_flags: u32,
) -> libc::c_int {
    let fw_sysfs: *mut fw_sysfs;
    let mut timeout: libc::c_long;
    let mut ret: libc::c_int;

    timeout = firmware_loading_timeout();
    if opt_flags & FW_OPT_NOWAIT != 0 {
        timeout = usermodehelper_read_lock_wait(timeout);
        if timeout == 0 {
            dev_dbg(device, "firmware: %s loading timed out\n", name);
            return -EBUSY;
        }
    } else {
        ret = usermodehelper_read_trylock();
        if WARN_ON(ret != 0) {
            dev_err(device, "firmware: %s will not be loaded\n", name);
            return ret;
        }
    }

    fw_sysfs = fw_create_instance(firmware, name, device, opt_flags);
    if IS_ERR(fw_sysfs) {
        ret = PTR_ERR(fw_sysfs);
        goto out_unlock;
    }

    (*fw_sysfs).fw_priv = (*firmware).priv;
    ret = fw_load_sysfs_fallback(fw_sysfs, timeout);

    if ret == 0 {
        ret = assign_fw(firmware, device);
    }

out_unlock:
    usermodehelper_read_unlock();
    ret
}

unsafe fn fw_force_sysfs_fallback(opt_flags: u32) -> bool {
    if fw_fallback_config.force_sysfs_fallback {
        return true;
    }
    if opt_flags & FW_OPT_USERHELPER == 0 {
        return false;
    }
    true
}

unsafe fn fw_run_sysfs_fallback(opt_flags: u32) -> bool {
    let ret: libc::c_int;

    if fw_fallback_config.ignore_sysfs_fallback {
        pr_info_once!("Ignoring firmware sysfs fallback due to sysctl knob");
        return false;
    }

    if opt_flags & FW_OPT_NOFALLBACK_SYSFS != 0 {
        return false;
    }

    /* Also permit LSMs and IMA to fail firmware sysfs fallback */
    ret = security_kernel_load_data(LOADING_FIRMWARE, true);
    if ret < 0 {
        return false;
    }

    fw_force_sysfs_fallback(opt_flags)
}

/**
 * firmware_fallback_sysfs() - use the fallback mechanism to find firmware
 * @fw: pointer to firmware image
 * @name: name of firmware file to look for
 * @device: device for which firmware is being loaded
 * @opt_flags: options to control firmware loading behaviour, as defined by
 *             &enum fw_opt
 * @ret: return value from direct lookup which triggered the fallback mechanism
 *
 * This function is called if direct lookup for the firmware failed, it enables
 * a fallback mechanism through userspace by exposing a sysfs loading
 * interface. Userspace is in charge of loading the firmware through the sysfs
 * loading interface. This sysfs fallback mechanism may be disabled completely
 * on a system by setting the proc sysctl value ignore_sysfs_fallback to true.
 * If this is false we check if the internal API caller set the
 * @FW_OPT_NOFALLBACK_SYSFS flag, if so it would also disable the fallback
 * mechanism. A system may want to enforce the sysfs fallback mechanism at all
 * times, it can do this by setting ignore_sysfs_fallback to false and
 * force_sysfs_fallback to true.
 * Enabling force_sysfs_fallback is functionally equivalent to build a kernel
 * with CONFIG_FW_LOADER_USER_HELPER_FALLBACK.
 **/
pub unsafe fn firmware_fallback_sysfs(
    fw: *mut firmware,
    name: *const libc::c_char,
    device: *mut device,
    opt_flags: u32,
    ret: libc::c_int,
) -> libc::c_int {
    if !fw_run_sysfs_fallback(opt_flags) {
        return ret;
    }

    if opt_flags & FW_OPT_NO_WARN == 0 {
        dev_warn(device, "Falling back to sysfs fallback for: %s\n", name);
    } else {
        dev_dbg(device, "Falling back to sysfs fallback for: %s\n", name);
    }
    fw_load_from_user_helper(fw, name, device, opt_flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
