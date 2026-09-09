// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the Linux SCMI Generic SystemPower Control driver. */

// Kernel headers and symbols are supplied by the surrounding Rust kernel bindings.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ScmiSyspowerState {
    ScmiSyspowerIdle,
    ScmiSyspowerInProgress,
    ScmiSyspowerRebooting,
}

#[repr(C)]
struct ScmiSyspowerConf {
    dev: *mut device,
    state: ScmiSyspowerState,
    state_mtx: mutex,
    required_transition: scmi_system_events,
    userspace_nb: notifier_block,
    reboot_nb: notifier_block,
    forceful_work: delayed_work,
    suspend_work: work_struct,
}

// These container_of conversions preserve the corresponding C macros.
unsafe fn userspace_nb_to_sconf(x: *mut notifier_block) -> *mut ScmiSyspowerConf {
    container_of!(x, ScmiSyspowerConf, userspace_nb)
}
unsafe fn reboot_nb_to_sconf(x: *mut notifier_block) -> *mut ScmiSyspowerConf {
    container_of!(x, ScmiSyspowerConf, reboot_nb)
}
unsafe fn dwork_to_sconf(x: *mut delayed_work) -> *mut ScmiSyspowerConf {
    container_of!(x, ScmiSyspowerConf, forceful_work)
}

unsafe extern "C" fn scmi_reboot_notifier(
    nb: *mut notifier_block,
    reason: c_ulong,
    _unused: *mut c_void,
) -> c_int {
    let sc = &mut *reboot_nb_to_sconf(nb);
    mutex_lock(&mut sc.state_mtx);
    match reason {
        SYS_HALT | SYS_POWER_OFF => {
            if sc.required_transition == SCMI_SYSTEM_SHUTDOWN {
                sc.state = ScmiSyspowerState::ScmiSyspowerRebooting;
            }
        }
        SYS_RESTART => {
            if sc.required_transition == SCMI_SYSTEM_COLDRESET
                || sc.required_transition == SCMI_SYSTEM_WARMRESET
            {
                sc.state = ScmiSyspowerState::ScmiSyspowerRebooting;
            }
        }
        _ => {}
    }
    if sc.state == ScmiSyspowerState::ScmiSyspowerRebooting {
        dev_dbg!(sc.dev, "Reboot in progress...cancel delayed work.\n");
        cancel_delayed_work_sync(&mut sc.forceful_work);
    }
    mutex_unlock(&mut sc.state_mtx);
    NOTIFY_OK
}

unsafe fn scmi_request_forceful_transition(sc: &mut ScmiSyspowerConf) {
    dev_dbg!(sc.dev, "Serving forceful request:%d\n", sc.required_transition);
    #[cfg(not(module))]
    emergency_sync();
    match sc.required_transition {
        SCMI_SYSTEM_SHUTDOWN => kernel_power_off(),
        SCMI_SYSTEM_COLDRESET | SCMI_SYSTEM_WARMRESET => kernel_restart(core::ptr::null()),
        _ => {}
    }
}

unsafe extern "C" fn scmi_forceful_work_func(work: *mut work_struct) {
    if system_state > SYSTEM_RUNNING { return; }
    let dwork = to_delayed_work(work);
    let sc = &mut *dwork_to_sconf(dwork);
    dev_dbg!(sc.dev, "Graceful request timed out...forcing !\n");
    mutex_lock(&mut sc.state_mtx);
    unregister_reboot_notifier(&mut sc.reboot_nb);
    if sc.state == ScmiSyspowerState::ScmiSyspowerInProgress {
        scmi_request_forceful_transition(sc);
    }
    mutex_unlock(&mut sc.state_mtx);
}

unsafe fn scmi_request_graceful_transition(sc: &mut ScmiSyspowerConf, timeout_ms: c_uint) {
    let mut adj_timeout_ms = 0;
    if timeout_ms != 0 {
        sc.reboot_nb.notifier_call = Some(scmi_reboot_notifier);
        let ret = register_reboot_notifier(&mut sc.reboot_nb);
        if ret == 0 {
            adj_timeout_ms = mult_frac(timeout_ms, 3, 4);
            INIT_DELAYED_WORK!(&mut sc.forceful_work, scmi_forceful_work_func);
            schedule_delayed_work(&mut sc.forceful_work, msecs_to_jiffies(adj_timeout_ms));
        } else {
            dev_warn!(sc.dev, "Cannot register reboot notifier !\n");
        }
    }
    dev_dbg!(sc.dev, "Serving graceful req:%d (timeout_ms:%u  adj_timeout_ms:%u)\n",
             sc.required_transition, timeout_ms, adj_timeout_ms);
    match sc.required_transition {
        SCMI_SYSTEM_SHUTDOWN => orderly_poweroff(true),
        SCMI_SYSTEM_COLDRESET | SCMI_SYSTEM_WARMRESET => orderly_reboot(),
        SCMI_SYSTEM_SUSPEND => schedule_work(&mut sc.suspend_work),
        _ => {}
    }
}

unsafe extern "C" fn scmi_userspace_notifier(
    nb: *mut notifier_block, _event: c_ulong, data: *mut c_void,
) -> c_int {
    let er = &*(data as *const scmi_system_power_state_notifier_report);
    let sc = &mut *userspace_nb_to_sconf(nb);
    if er.system_state >= SCMI_SYSTEM_MAX || er.system_state == SCMI_SYSTEM_POWERUP {
        dev_err!(sc.dev, "Ignoring unsupported system_state: 0x%X\n", er.system_state);
        return NOTIFY_DONE;
    }
    if !SCMI_SYSPOWER_IS_REQUEST_GRACEFUL!(er.flags) {
        dev_err!(sc.dev, "Ignoring forceful notification.\n");
        return NOTIFY_DONE;
    }
    if system_state > SYSTEM_RUNNING { return NOTIFY_DONE; }
    mutex_lock(&mut sc.state_mtx);
    if sc.state != ScmiSyspowerState::ScmiSyspowerIdle {
        dev_dbg!(sc.dev, "Transition already in progress...ignore.\n");
        mutex_unlock(&mut sc.state_mtx);
        return NOTIFY_DONE;
    }
    sc.state = ScmiSyspowerState::ScmiSyspowerInProgress;
    mutex_unlock(&mut sc.state_mtx);
    sc.required_transition = er.system_state;
    dev_info!(sc.dev, "Serving shutdown/reboot request: %d\n", sc.required_transition);
    scmi_request_graceful_transition(sc, er.timeout);
    NOTIFY_OK
}

unsafe extern "C" fn scmi_suspend_work_func(_work: *mut work_struct) {
    pm_suspend(PM_SUSPEND_MEM);
}

// Probe, resume, driver registration, and module metadata retain their C interfaces.
// Their kernel types and registration macros are provided by external bindings.
unsafe fn scmi_syspower_probe(sdev: *mut scmi_device) -> c_int {
    let handle = (*sdev).handle;
    if handle.is_null() { return -ENODEV; }
    let ret = ((*handle).devm_protocol_acquire)(sdev, SCMI_PROTOCOL_SYSTEM);
    if ret != 0 { return ret; }
    let sc = devm_kzalloc(&mut (*sdev).dev, core::mem::size_of::<ScmiSyspowerConf>(), GFP_KERNEL)
        as *mut ScmiSyspowerConf;
    if sc.is_null() { return -ENOMEM; }
    (*sc).state = ScmiSyspowerState::ScmiSyspowerIdle;
    mutex_init(&mut (*sc).state_mtx);
    (*sc).required_transition = SCMI_SYSTEM_MAX;
    (*sc).userspace_nb.notifier_call = Some(scmi_userspace_notifier);
    (*sc).dev = &mut (*sdev).dev;
    dev_set_drvdata(&mut (*sdev).dev, sc as *mut c_void);
    INIT_WORK!(&mut (*sc).suspend_work, scmi_suspend_work_func);
    ((*handle).notify_ops.devm_event_notifier_register)(sdev, SCMI_PROTOCOL_SYSTEM,
        SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER, core::ptr::null_mut(), &mut (*sc).userspace_nb)
}

unsafe fn scmi_system_power_resume(dev: *mut device) -> c_int {
    let sc = dev_get_drvdata(dev) as *mut ScmiSyspowerConf;
    (*sc).state = ScmiSyspowerState::ScmiSyspowerIdle;
    0
}

static scmi_system_power_pmops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(NULL, scmi_system_power_resume)
};

static scmi_id_table: [scmi_device_id; 2] = [
    scmi_device_id { protocol_id: SCMI_PROTOCOL_SYSTEM, name: "syspower" },
    scmi_device_id { protocol_id: 0, name: "" },
];

static mut scmi_system_power_driver: scmi_driver = scmi_driver {
    driver: driver { pm: pm_sleep_ptr(&scmi_system_power_pmops) },
    name: "scmi-system-power",
    probe: Some(scmi_syspower_probe),
    id_table: scmi_id_table.as_ptr(),
};

// MODULE_DEVICE_TABLE(scmi, scmi_id_table);
// module_scmi_driver(scmi_system_power_driver);
// MODULE_AUTHOR("Cristian Marussi <cristian.marussi@arm.com>");
// MODULE_DESCRIPTION("ARM SCMI SystemPower Control driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
