// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2022 Intel Corporation */

// Linux headers and driver headers from the original implementation provide
// the types, constants, macros, and external functions referenced below.

#[repr(C)]
struct adf_gen4_pm_data {
    pm_irq_work: work_struct,
    accel_dev: *mut adf_accel_dev,
    pm_int_sts: u32,
}

unsafe fn send_host_msg(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut pm_idle_support_cfg = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let pmisc: *mut core::ffi::c_void = adf_get_pmisc_base(accel_dev);
    let pm = &mut (*accel_dev).power_management;
    let mut pm_idle_support: bool;
    let mut msg: u32;
    let ret: i32;

    msg = ADF_CSR_RD(pmisc, ADF_GEN4_PM_HOST_MSG);
    if (msg & ADF_GEN4_PM_MSG_PENDING) != 0 {
        return -EBUSY;
    }

    adf_cfg_get_param_value(
        accel_dev,
        ADF_GENERAL_SEC,
        ADF_PM_IDLE_SUPPORT,
        pm_idle_support_cfg.as_mut_ptr(),
    );
    ret = kstrtobool(pm_idle_support_cfg.as_ptr(), &mut pm_idle_support);
    if ret != 0 {
        pm_idle_support = true;
    }

    if pm_idle_support {
        pm.host_ack_counter += 1;
    } else {
        pm.host_nack_counter += 1;
    }

    /* Send HOST_MSG */
    msg = FIELD_PREP(
        ADF_GEN4_PM_MSG_PAYLOAD_BIT_MASK,
        if pm_idle_support { PM_SET_MIN } else { PM_NO_CHANGE },
    );
    msg |= ADF_GEN4_PM_MSG_PENDING;
    ADF_CSR_WR(pmisc, ADF_GEN4_PM_HOST_MSG, msg);

    /* Poll status register to make sure the HOST_MSG has been processed */
    read_poll_timeout(
        ADF_CSR_RD,
        msg,
        (msg & ADF_GEN4_PM_MSG_PENDING) == 0,
        ADF_GEN4_PM_MSG_POLL_DELAY_US,
        ADF_GEN4_PM_POLL_TIMEOUT_US,
        true,
        pmisc,
        ADF_GEN4_PM_HOST_MSG,
    )
}

unsafe fn pm_bh_handler(work: *mut work_struct) {
    let pm_data: *mut adf_gen4_pm_data = container_of!(work, adf_gen4_pm_data, pm_irq_work);
    let accel_dev = (*pm_data).accel_dev;
    let pmisc: *mut core::ffi::c_void = adf_get_pmisc_base(accel_dev);
    let pm = &mut (*accel_dev).power_management;
    let pm_int_sts = (*pm_data).pm_int_sts;
    let mut val: u32;

    /* PM Idle interrupt */
    if (pm_int_sts & ADF_GEN4_PM_IDLE_STS) != 0 {
        pm.idle_irq_counters += 1;
        /* Issue host message to FW */
        if send_host_msg(accel_dev) != 0 {
            dev_warn_ratelimited!(&GET_DEV(accel_dev), "Failed to send host msg to FW\n");
        }
    }

    /* PM throttle interrupt */
    if (pm_int_sts & ADF_GEN4_PM_THR_STS) != 0 {
        pm.throttle_irq_counters += 1;
    }

    /* PM fw interrupt */
    if (pm_int_sts & ADF_GEN4_PM_FW_INT_STS) != 0 {
        pm.fw_irq_counters += 1;
    }

    /* Clear interrupt status */
    ADF_CSR_WR(pmisc, ADF_GEN4_PM_INTERRUPT, pm_int_sts);

    /* Reenable PM interrupt */
    val = ADF_CSR_RD(pmisc, ADF_GEN4_ERRMSK2);
    val &= !ADF_GEN4_PM_SOU;
    ADF_CSR_WR(pmisc, ADF_GEN4_ERRMSK2, val);

    kfree(pm_data);
}

pub unsafe fn adf_gen4_handle_pm_interrupt(accel_dev: *mut adf_accel_dev) -> bool {
    let pmisc: *mut core::ffi::c_void = adf_get_pmisc_base(accel_dev);
    let mut pm_data: *mut adf_gen4_pm_data = core::ptr::null_mut();
    let errsou2: u32;
    let errmsk2: u32;
    let mut val: u32;

    /* Only handle the interrupt triggered by PM */
    errmsk2 = ADF_CSR_RD(pmisc, ADF_GEN4_ERRMSK2);
    if (errmsk2 & ADF_GEN4_PM_SOU) != 0 {
        return false;
    }

    errsou2 = ADF_CSR_RD(pmisc, ADF_GEN4_ERRSOU2);
    if (errsou2 & ADF_GEN4_PM_SOU) == 0 {
        return false;
    }

    /* Disable interrupt */
    val = ADF_CSR_RD(pmisc, ADF_GEN4_ERRMSK2);
    val |= ADF_GEN4_PM_SOU;
    ADF_CSR_WR(pmisc, ADF_GEN4_ERRMSK2, val);

    val = ADF_CSR_RD(pmisc, ADF_GEN4_PM_INTERRUPT);

    pm_data = kzalloc_obj!(*pm_data, GFP_ATOMIC);
    if pm_data.is_null() {
        return false;
    }

    (*pm_data).pm_int_sts = val;
    (*pm_data).accel_dev = accel_dev;

    INIT_WORK!(&mut (*pm_data).pm_irq_work, pm_bh_handler);
    adf_misc_wq_queue_work(&mut (*pm_data).pm_irq_work);

    true
}

EXPORT_SYMBOL_GPL!(adf_gen4_handle_pm_interrupt);

pub unsafe fn adf_gen4_enable_pm(accel_dev: *mut adf_accel_dev) -> i32 {
    let pmisc: *mut core::ffi::c_void = adf_get_pmisc_base(accel_dev);
    let ret: i32;
    let mut val: u32;

    ret = adf_init_admin_pm(accel_dev, ADF_GEN4_PM_DEFAULT_IDLE_FILTER);
    if ret != 0 {
        return ret;
    }

    /* Initialize PM internal data */
    adf_gen4_init_dev_pm_data(accel_dev);

    /* Enable default PM interrupts: IDLE, THROTTLE */
    val = ADF_CSR_RD(pmisc, ADF_GEN4_PM_INTERRUPT);
    val |= ADF_GEN4_PM_INT_EN_DEFAULT;

    /* Clear interrupt status */
    val |= ADF_GEN4_PM_INT_STS_MASK;
    ADF_CSR_WR(pmisc, ADF_GEN4_PM_INTERRUPT, val);

    /* Unmask PM Interrupt */
    val = ADF_CSR_RD(pmisc, ADF_GEN4_ERRMSK2);
    val &= !ADF_GEN4_PM_SOU;
    ADF_CSR_WR(pmisc, ADF_GEN4_ERRMSK2, val);

    0
}

EXPORT_SYMBOL_GPL!(adf_gen4_enable_pm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
