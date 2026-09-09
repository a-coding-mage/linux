// SPDX-License-Identifier: GPL-2.0
/* SiRFstar GNSS receiver driver */

const SIRF_BOOT_DELAY: u64 = 500;
const SIRF_ON_OFF_PULSE_TIME: u64 = 100;
const SIRF_ACTIVATE_TIMEOUT: u64 = 200;
const SIRF_HIBERNATE_TIMEOUT: u64 = 200;
// If no data arrives for this time, we assume that the chip is off.
const SIRF_REPORT_CYCLE: u64 = 2000;

#[repr(C)]
struct SirfData {
    gdev: *mut GnssDevice,
    serdev: *mut SerdevDevice,
    speed: SpeedT,
    vcc: *mut Regulator,
    lna: *mut Regulator,
    on_off: *mut GpioDesc,
    wakeup: *mut GpioDesc,
    irq: i32,
    active: bool,
    gdev_mutex: Mutex,
    open: bool,
    serdev_mutex: Mutex,
    serdev_count: i32,
    power_wait: WaitQueueHead,
}

unsafe fn sirf_serdev_open(data: *mut SirfData) -> i32 {
    let mut ret = 0;
    mutex_lock(&mut (*data).serdev_mutex);
    (*data).serdev_count += 1;
    if (*data).serdev_count == 1 {
        ret = serdev_device_open((*data).serdev);
        if ret != 0 {
            (*data).serdev_count -= 1;
            mutex_unlock(&mut (*data).serdev_mutex);
            return ret;
        }
        serdev_device_set_baudrate((*data).serdev, (*data).speed);
        serdev_device_set_flow_control((*data).serdev, false);
    }
    mutex_unlock(&mut (*data).serdev_mutex);
    ret
}

unsafe fn sirf_serdev_close(data: *mut SirfData) {
    mutex_lock(&mut (*data).serdev_mutex);
    (*data).serdev_count -= 1;
    if (*data).serdev_count == 0 { serdev_device_close((*data).serdev); }
    mutex_unlock(&mut (*data).serdev_mutex);
}

unsafe fn sirf_open(gdev: *mut GnssDevice) -> i32 {
    let data = gnss_get_drvdata(gdev) as *mut SirfData;
    let serdev = (*data).serdev;
    mutex_lock(&mut (*data).gdev_mutex); (*data).open = true; mutex_unlock(&mut (*data).gdev_mutex);
    let ret = sirf_serdev_open(data);
    if ret != 0 { mutex_lock(&mut (*data).gdev_mutex); (*data).open = false; mutex_unlock(&mut (*data).gdev_mutex); return ret; }
    let ret = pm_runtime_get_sync(&mut (*serdev).dev);
    if ret < 0 { dev_err(&(*gdev).dev, "failed to runtime resume: %d\n", ret); pm_runtime_put_noidle(&mut (*serdev).dev); sirf_serdev_close(data); mutex_lock(&mut (*data).gdev_mutex); (*data).open = false; mutex_unlock(&mut (*data).gdev_mutex); }
    ret
}

unsafe fn sirf_close(gdev: *mut GnssDevice) {
    let data = gnss_get_drvdata(gdev) as *mut SirfData;
    sirf_serdev_close(data); pm_runtime_put(&mut (*(*data).serdev).dev);
    mutex_lock(&mut (*data).gdev_mutex); (*data).open = false; mutex_unlock(&mut (*data).gdev_mutex);
}

unsafe fn sirf_write_raw(gdev: *mut GnssDevice, buf: *const u8, count: usize) -> i32 {
    let data = gnss_get_drvdata(gdev) as *mut SirfData;
    let ret = serdev_device_write((*data).serdev, buf, count, MAX_SCHEDULE_TIMEOUT);
    if ret < 0 || ret < count as i32 { return ret; }
    serdev_device_wait_until_sent((*data).serdev, 0); count as i32
}

static SIRF_GNSS_OPS: GnssOperations = GnssOperations { open: Some(sirf_open), close: Some(sirf_close), write_raw: Some(sirf_write_raw) };

unsafe fn sirf_receive_buf(serdev: *mut SerdevDevice, buf: *const u8, count: usize) -> usize {
    let data = serdev_device_get_drvdata(serdev) as *mut SirfData;
    if (*data).wakeup.is_null() && !(*data).active { (*data).active = true; wake_up_interruptible(&mut (*data).power_wait); }
    mutex_lock(&mut (*data).gdev_mutex);
    let ret = if (*data).open { gnss_insert_raw((*data).gdev, buf, count) } else { 0 };
    mutex_unlock(&mut (*data).gdev_mutex); ret as usize
}

static SIRF_SERDEV_OPS: SerdevDeviceOps = SerdevDeviceOps { receive_buf: Some(sirf_receive_buf), write_wakeup: Some(serdev_device_write_wakeup) };

unsafe fn sirf_wakeup_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqreturnT {
    let data = dev_id as *mut SirfData;
    let ret = gpiod_get_value_cansleep((*data).wakeup);
    dev_dbg(&(*(*data).serdev).dev, "%s - wakeup = %d\n", "sirf_wakeup_handler", ret);
    if ret >= 0 { (*data).active = ret != 0; wake_up_interruptible(&mut (*data).power_wait); }
    IRQ_HANDLED
}

unsafe fn sirf_wait_for_power_state_nowakeup(data: *mut SirfData, active: bool, timeout: u64) -> i32 {
    msleep(timeout); (*data).active = false;
    let ret = wait_event_interruptible_timeout(&mut (*data).power_wait, (*data).active, msecs_to_jiffies(SIRF_REPORT_CYCLE));
    if ret < 0 { return ret; }
    if (ret > 0 && !active) || (ret == 0 && active) { return -ETIMEDOUT; } 0
}

unsafe fn sirf_wait_for_power_state(data: *mut SirfData, active: bool, timeout: u64) -> i32 {
    if (*data).wakeup.is_null() { return sirf_wait_for_power_state_nowakeup(data, active, timeout); }
    let ret = wait_event_interruptible_timeout(&mut (*data).power_wait, (*data).active == active, msecs_to_jiffies(timeout));
    if ret < 0 { return ret; }
    if ret == 0 { dev_warn(&(*(*data).serdev).dev, "timeout waiting for active state = %d\n", active as i32); return -ETIMEDOUT; } 0
}

unsafe fn sirf_pulse_on_off(data: *mut SirfData) { gpiod_set_value_cansleep((*data).on_off, 1); msleep(SIRF_ON_OFF_PULSE_TIME); gpiod_set_value_cansleep((*data).on_off, 0); }

unsafe fn sirf_set_active(data: *mut SirfData, active: bool) -> i32 {
    let timeout = if active { SIRF_ACTIVATE_TIMEOUT } else { SIRF_HIBERNATE_TIMEOUT };
    if (*data).wakeup.is_null() { let ret = sirf_serdev_open(data); if ret != 0 { return ret; } }
    let mut retries = 3; let mut ret;
    loop { sirf_pulse_on_off(data); ret = sirf_wait_for_power_state(data, active, timeout); if ret != -ETIMEDOUT || retries == 0 { break; } retries -= 1; }
    if (*data).wakeup.is_null() { sirf_serdev_close(data); } ret
}

unsafe fn sirf_runtime_suspend(dev: *mut Device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut SirfData;
    let ret = if !(*data).on_off.is_null() { sirf_set_active(data, false) } else { regulator_disable((*data).vcc) }; if ret != 0 { return ret; }
    let ret = regulator_disable((*data).lna); if ret != 0 { let r = if !(*data).on_off.is_null() { sirf_set_active(data, true) } else { regulator_enable((*data).vcc) }; if r != 0 { dev_err(dev, "failed to reenable power on failed suspend: %d\n", r); } } ret
}

unsafe fn sirf_runtime_resume(dev: *mut Device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut SirfData; let ret = regulator_enable((*data).lna); if ret != 0 { return ret; }
    let ret = if !(*data).on_off.is_null() { sirf_set_active(data, true) } else { regulator_enable((*data).vcc) }; if ret != 0 { regulator_disable((*data).lna); } ret
}

unsafe fn sirf_suspend(dev: *mut Device) -> i32 { let data = dev_get_drvdata(dev) as *mut SirfData; let mut ret = 0; if !pm_runtime_suspended(dev) { ret = sirf_runtime_suspend(dev); } if !(*data).wakeup.is_null() { disable_irq((*data).irq); } ret }
unsafe fn sirf_resume(dev: *mut Device) -> i32 { let data = dev_get_drvdata(dev) as *mut SirfData; if !(*data).wakeup.is_null() { enable_irq((*data).irq); } if !pm_runtime_suspended(dev) { sirf_runtime_resume(dev) } else { 0 } }

unsafe fn sirf_parse_dt(serdev: *mut SerdevDevice) -> i32 {
    let data = serdev_device_get_drvdata(serdev) as *mut SirfData; let mut speed: u32 = 9600;
    of_property_read_u32((*serdev).dev.of_node, "current-speed", &mut speed); (*data).speed = speed as SpeedT; 0
}

unsafe fn sirf_probe(serdev: *mut SerdevDevice) -> i32 {
    let dev = &mut (*serdev).dev; let data = devm_kzalloc(dev, core::mem::size_of::<SirfData>(), GFP_KERNEL) as *mut SirfData; if data.is_null() { return -ENOMEM; }
    let gdev = gnss_allocate_device(dev); if gdev.is_null() { return -ENOMEM; }
    (*gdev).type_ = GNSS_TYPE_SIRF; (*gdev).ops = &SIRF_GNSS_OPS; gnss_set_drvdata(gdev, data); (*data).serdev = serdev; (*data).gdev = gdev;
    mutex_init(&mut (*data).gdev_mutex); mutex_init(&mut (*data).serdev_mutex); init_waitqueue_head(&mut (*data).power_wait);
    serdev_device_set_drvdata(serdev, data); serdev_device_set_client_ops(serdev, &SIRF_SERDEV_OPS);
    let mut ret = sirf_parse_dt(serdev); if ret != 0 { gnss_put_device(gdev); return ret; }
    (*data).vcc = devm_regulator_get(dev, "vcc"); if IS_ERR((*data).vcc) { ret = PTR_ERR((*data).vcc); gnss_put_device(gdev); return ret; }
    (*data).lna = devm_regulator_get(dev, "lna"); if IS_ERR((*data).lna) { ret = PTR_ERR((*data).lna); gnss_put_device(gdev); return ret; }
    (*data).on_off = devm_gpiod_get_optional(dev, "sirf,onoff", GPIOD_OUT_LOW); if IS_ERR((*data).on_off) { ret = PTR_ERR((*data).on_off); gnss_put_device(gdev); return ret; }
    if !(*data).on_off.is_null() { (*data).wakeup = devm_gpiod_get_optional(dev, "sirf,wakeup", GPIOD_IN); if IS_ERR((*data).wakeup) { ret = PTR_ERR((*data).wakeup); gnss_put_device(gdev); return ret; } ret = regulator_enable((*data).vcc); if ret != 0 { gnss_put_device(gdev); return ret; } msleep(SIRF_BOOT_DELAY); }
    if !(*data).wakeup.is_null() { ret = gpiod_get_value_cansleep((*data).wakeup); if ret < 0 { regulator_disable((*data).vcc); gnss_put_device(gdev); return ret; } (*data).active = ret != 0; ret = gpiod_to_irq((*data).wakeup); if ret < 0 { regulator_disable((*data).vcc); gnss_put_device(gdev); return ret; } (*data).irq = ret; ret = request_threaded_irq(ret, None, Some(sirf_wakeup_handler), IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT, "wakeup", data as *mut _); if ret != 0 { regulator_disable((*data).vcc); gnss_put_device(gdev); return ret; } }
    if !(*data).on_off.is_null() && (*data).wakeup.is_null() { (*data).active = false; ret = sirf_serdev_open(data); if ret != 0 { regulator_disable((*data).vcc); gnss_put_device(gdev); return ret; } msleep(SIRF_REPORT_CYCLE); sirf_serdev_close(data); }
    if !(*data).on_off.is_null() && (*data).active { ret = sirf_set_active(data, false); if ret != 0 { free_irq((*data).irq, data as *mut _); regulator_disable((*data).vcc); gnss_put_device(gdev); return ret; } }
    pm_runtime_set_suspended(dev); pm_runtime_enable(dev); ret = gnss_register_device(gdev); if ret != 0 { pm_runtime_disable(dev); if !(*data).wakeup.is_null() { free_irq((*data).irq, data as *mut _); } if !(*data).on_off.is_null() { regulator_disable((*data).vcc); } gnss_put_device(gdev); return ret; } 0
}

unsafe fn sirf_remove(serdev: *mut SerdevDevice) { let data = serdev_device_get_drvdata(serdev) as *mut SirfData; gnss_deregister_device((*data).gdev); pm_runtime_disable(&mut (*serdev).dev); if !(*data).wakeup.is_null() { free_irq((*data).irq, data as *mut _); } if !(*data).on_off.is_null() { regulator_disable((*data).vcc); } gnss_put_device((*data).gdev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
