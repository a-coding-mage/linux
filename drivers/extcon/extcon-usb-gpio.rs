// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/extcon/extcon-usb-gpio.c - USB GPIO extcon driver
 *
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
 * Author: Roger Quadros <rogerq@ti.com>
 */

// Kernel dependencies supplied by the surrounding translation environment.

const USB_GPIO_DEBOUNCE_MS: u32 = 20; // ms

#[repr(C)]
struct UsbExtconInfo {
    dev: *mut device,
    edev: *mut extcon_dev,
    id_gpiod: *mut gpio_desc,
    vbus_gpiod: *mut gpio_desc,
    id_irq: i32,
    vbus_irq: i32,
    debounce_jiffies: usize,
    wq_detcable: delayed_work,
}

static USB_EXTCON_CABLE: [u32; 3] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_NONE];

/*
 * "USB" = VBUS and "USB-HOST" = !ID, so we have:
 * Both "USB" and "USB-HOST" can't be set as active at the
 * same time so if "USB-HOST" is active (i.e. ID is 0)  we keep "USB" inactive
 * even if VBUS is on.
 *
 *  State              |    ID   |   VBUS
 * ----------------------------------------
 *  [1] USB            |    H    |    H
 *  [2] none           |    H    |    L
 *  [3] USB-HOST       |    L    |    H
 *  [4] USB-HOST       |    L    |    L
 *
 * In case we have only one of these signals:
 * - VBUS only - we want to distinguish between [1] and [2], so ID is always 1.
 * - ID only - we want to distinguish between [1] and [4], so VBUS = ID.
 */
unsafe fn usb_extcon_detect_cable(work: *mut work_struct) {
    let info = container_of(
        to_delayed_work(work),
        &mut (*(core::ptr::null_mut::<UsbExtconInfo>())),
        wq_detcable,
    );

    let id: i32 = if !(*info).id_gpiod.is_null() {
        gpiod_get_value_cansleep((*info).id_gpiod)
    } else { 1 };
    let vbus: i32 = if !(*info).vbus_gpiod.is_null() {
        gpiod_get_value_cansleep((*info).vbus_gpiod)
    } else { id };

    if id != 0 {
        extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, false);
    }
    if vbus == 0 {
        extcon_set_state_sync((*info).edev, EXTCON_USB, false);
    }
    if id == 0 {
        extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, true);
    } else if vbus != 0 {
        extcon_set_state_sync((*info).edev, EXTCON_USB, true);
    }
}

unsafe extern "C" fn usb_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let info = dev_id as *mut UsbExtconInfo;
    queue_delayed_work(system_power_efficient_wq, &mut (*info).wq_detcable,
                       (*info).debounce_jiffies);
    IRQ_HANDLED
}

unsafe extern "C" fn usb_extcon_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    if np.is_null() { return -EINVAL; }

    let info = devm_kzalloc(dev, core::mem::size_of::<UsbExtconInfo>(), GFP_KERNEL)
        as *mut UsbExtconInfo;
    if info.is_null() { return -ENOMEM; }
    (*info).dev = dev;
    (*info).id_gpiod = devm_gpiod_get_optional(dev, c"id", GPIOD_IN);
    (*info).vbus_gpiod = devm_gpiod_get_optional(dev, c"vbus", GPIOD_IN);
    if (*info).id_gpiod.is_null() && (*info).vbus_gpiod.is_null() {
        dev_err(dev, c"failed to get gpios\n"); return -ENODEV;
    }
    if IS_ERR((*info).id_gpiod) { return PTR_ERR((*info).id_gpiod); }
    if IS_ERR((*info).vbus_gpiod) { return PTR_ERR((*info).vbus_gpiod); }

    (*info).edev = devm_extcon_dev_allocate(dev, USB_EXTCON_CABLE.as_ptr());
    if IS_ERR((*info).edev) { dev_err(dev, c"failed to allocate extcon device\n"); return -ENOMEM; }
    let mut ret = devm_extcon_dev_register(dev, (*info).edev);
    if ret < 0 { dev_err(dev, c"failed to register extcon device\n"); return ret; }

    if !(*info).id_gpiod.is_null() { ret = gpiod_set_debounce((*info).id_gpiod, (USB_GPIO_DEBOUNCE_MS * 1000) as i32); }
    if ret == 0 && !(*info).vbus_gpiod.is_null() { ret = gpiod_set_debounce((*info).vbus_gpiod, (USB_GPIO_DEBOUNCE_MS * 1000) as i32); }
    if ret < 0 { (*info).debounce_jiffies = msecs_to_jiffies(USB_GPIO_DEBOUNCE_MS); }
    INIT_DELAYED_WORK(&mut (*info).wq_detcable, usb_extcon_detect_cable);

    if !(*info).id_gpiod.is_null() {
        (*info).id_irq = gpiod_to_irq((*info).id_gpiod);
        if (*info).id_irq < 0 { dev_err(dev, c"failed to get ID IRQ\n"); return (*info).id_irq; }
        ret = devm_request_threaded_irq(dev, (*info).id_irq, None, Some(usb_irq_handler), IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT, (*pdev).name, info as *mut _);
        if ret < 0 { dev_err(dev, c"failed to request handler for ID IRQ\n"); return ret; }
    }
    if !(*info).vbus_gpiod.is_null() {
        (*info).vbus_irq = gpiod_to_irq((*info).vbus_gpiod);
        if (*info).vbus_irq < 0 { dev_err(dev, c"failed to get VBUS IRQ\n"); return (*info).vbus_irq; }
        ret = devm_request_threaded_irq(dev, (*info).vbus_irq, None, Some(usb_irq_handler), IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT, (*pdev).name, info as *mut _);
        if ret < 0 { dev_err(dev, c"failed to request handler for VBUS IRQ\n"); return ret; }
    }
    platform_set_drvdata(pdev, info as *mut _);
    device_set_wakeup_capable(dev, true);
    usb_extcon_detect_cable(&mut (*info).wq_detcable.work);
    0
}

unsafe extern "C" fn usb_extcon_remove(pdev: *mut platform_device) {
    let info = platform_get_drvdata(pdev) as *mut UsbExtconInfo;
    cancel_delayed_work_sync(&mut (*info).wq_detcable);
    device_init_wakeup(&mut (*pdev).dev, false);
}

// CONFIG_PM_SLEEP conditional declarations and SIMPLE_DEV_PM_OPS are preserved below.
static USB_EXTCON_PM_OPS: device_pm_ops = SIMPLE_DEV_PM_OPS(usb_extcon_suspend, usb_extcon_resume);

static USB_EXTCON_DT_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"linux,extcon-usb-gpio", },
    of_device_id { sentinel: true },
];

static USB_EXTCON_PLATFORM_IDS: [platform_device_id; 2] = [
    platform_device_id { name: c"extcon-usb-gpio", },
    platform_device_id { sentinel: true },
];

static mut USB_EXTCON_DRIVER: platform_driver = platform_driver {
    probe: Some(usb_extcon_probe), remove: Some(usb_extcon_remove),
    driver: device_driver { name: c"extcon-usb-gpio", pm: &USB_EXTCON_PM_OPS, of_match_table: USB_EXTCON_DT_MATCH.as_ptr() },
    id_table: USB_EXTCON_PLATFORM_IDS.as_ptr(),
};

// module_platform_driver(usb_extcon_driver);
// MODULE_AUTHOR("Roger Quadros <rogerq@ti.com>");
// MODULE_DESCRIPTION("USB GPIO extcon driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
