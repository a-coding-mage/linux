// SPDX-License-Identifier: GPL-2.0-only
/*
 * extcon-qcom-spmi-misc.c - Qualcomm USB extcon driver to support USB ID
 *			and VBUS detection based on extcon-usb-gpio.c.
 *
 * Copyright (C) 2016 Linaro, Ltd.
 * Stephen Boyd <stephen.boyd@linaro.org>
 */

// Dependencies are supplied by the surrounding kernel bindings.

const USB_ID_DEBOUNCE_MS: u32 = 5; // ms

#[repr(C)]
struct QcomUsbExtconInfo {
    edev: *mut ExtconDev,
    id_irq: i32,
    vbus_irq: i32,
    wq_detcable: DelayedWork,
    debounce_jiffies: c_ulong,
}

static QCOM_USB_EXTCON_CABLE: [u32; 3] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_NONE];

unsafe fn qcom_usb_extcon_detect_cable(work: *mut WorkStruct) {
    let mut state = false;
    let mut ret: i32;
    let mut val = ExtconPropertyValue { intval: 0 };
    let info = container_of(
        to_delayed_work(work),
        core::mem::offset_of!(QcomUsbExtconInfo, wq_detcable),
    );

    if (*info).id_irq > 0 {
        // check ID and update cable state
        ret = irq_get_irqchip_state(
            (*info).id_irq,
            IRQCHIP_STATE_LINE_LEVEL,
            &mut state,
        );
        if ret != 0 {
            return;
        }

        if !state {
            val.intval = 1;
            extcon_set_property((*info).edev, EXTCON_USB_HOST, EXTCON_PROP_USB_SS, val);
        }
        extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, !state);
    }

    if (*info).vbus_irq > 0 {
        // check VBUS and update cable state
        ret = irq_get_irqchip_state(
            (*info).vbus_irq,
            IRQCHIP_STATE_LINE_LEVEL,
            &mut state,
        );
        if ret != 0 {
            return;
        }

        if state {
            val.intval = 1;
            extcon_set_property((*info).edev, EXTCON_USB, EXTCON_PROP_USB_SS, val);
        }
        extcon_set_state_sync((*info).edev, EXTCON_USB, state);
    }
}

unsafe extern "C" fn qcom_usb_irq_handler(_irq: i32, dev_id: *mut c_void) -> Irqreturn {
    let info = dev_id as *mut QcomUsbExtconInfo;

    queue_delayed_work(
        system_power_efficient_wq,
        &mut (*info).wq_detcable,
        (*info).debounce_jiffies,
    );

    IRQ_HANDLED
}

unsafe fn qcom_usb_extcon_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let info = devm_kzalloc(dev, core::mem::size_of::<QcomUsbExtconInfo>(), GFP_KERNEL)
        as *mut QcomUsbExtconInfo;
    if info.is_null() {
        return -ENOMEM;
    }

    (*info).edev = devm_extcon_dev_allocate(dev, QCOM_USB_EXTCON_CABLE.as_ptr());
    if is_err((*info).edev) {
        dev_err(dev, "failed to allocate extcon device\n");
        return -ENOMEM;
    }

    let mut ret = devm_extcon_dev_register(dev, (*info).edev);
    if ret < 0 {
        dev_err(dev, "failed to register extcon device\n");
        return ret;
    }

    ret = extcon_set_property_capability((*info).edev, EXTCON_USB, EXTCON_PROP_USB_SS);
    ret |= extcon_set_property_capability((*info).edev, EXTCON_USB_HOST, EXTCON_PROP_USB_SS);
    if ret != 0 {
        dev_err(dev, "failed to register extcon props rc=%d\n", ret);
        return ret;
    }

    (*info).debounce_jiffies = msecs_to_jiffies(USB_ID_DEBOUNCE_MS);

    ret = devm_delayed_work_autocancel(
        dev,
        &mut (*info).wq_detcable,
        qcom_usb_extcon_detect_cable,
    );
    if ret != 0 {
        return ret;
    }

    (*info).id_irq = platform_get_irq_byname_optional(pdev, "usb_id");
    if (*info).id_irq > 0 {
        ret = devm_request_threaded_irq(
            dev,
            (*info).id_irq,
            None,
            Some(qcom_usb_irq_handler),
            IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
            (*pdev).name,
            info as *mut c_void,
        );
        if ret < 0 {
            dev_err(dev, "failed to request handler for ID IRQ\n");
            return ret;
        }
    }

    (*info).vbus_irq = platform_get_irq_byname_optional(pdev, "usb_vbus");
    if (*info).vbus_irq > 0 {
        ret = devm_request_threaded_irq(
            dev,
            (*info).vbus_irq,
            None,
            Some(qcom_usb_irq_handler),
            IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
            (*pdev).name,
            info as *mut c_void,
        );
        if ret < 0 {
            dev_err(dev, "failed to request handler for VBUS IRQ\n");
            return ret;
        }
    }

    if (*info).id_irq < 0 && (*info).vbus_irq < 0 {
        dev_err(dev, "ID and VBUS IRQ not found\n");
        return -EINVAL;
    }

    platform_set_drvdata(pdev, info as *mut c_void);
    devm_device_init_wakeup(dev);

    // Perform initial detection
    qcom_usb_extcon_detect_cable(&mut (*info).wq_detcable.work);

    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn qcom_usb_extcon_suspend(dev: *mut Device) -> i32 {
    let info = dev_get_drvdata(dev) as *mut QcomUsbExtconInfo;
    let mut ret = 0;

    if device_may_wakeup(dev) {
        if (*info).id_irq > 0 {
            ret = enable_irq_wake((*info).id_irq);
        }
        if (*info).vbus_irq > 0 {
            ret = enable_irq_wake((*info).vbus_irq);
        }
    }

    ret
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn qcom_usb_extcon_resume(dev: *mut Device) -> i32 {
    let info = dev_get_drvdata(dev) as *mut QcomUsbExtconInfo;
    let mut ret = 0;

    if device_may_wakeup(dev) {
        if (*info).id_irq > 0 {
            ret = disable_irq_wake((*info).id_irq);
        }
        if (*info).vbus_irq > 0 {
            ret = disable_irq_wake((*info).vbus_irq);
        }
    }

    ret
}

static QCOM_USB_EXTCON_PM_OPS: SimpleDevPmOps = simple_dev_pm_ops(
    qcom_usb_extcon_suspend,
    qcom_usb_extcon_resume,
);

static QCOM_USB_EXTCON_DT_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "qcom,pm8941-misc\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut QCOM_USB_EXTCON_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(qcom_usb_extcon_probe),
    driver: Driver {
        name: "extcon-pm8941-misc\0".as_ptr(),
        pm: &QCOM_USB_EXTCON_PM_OPS,
        of_match_table: QCOM_USB_EXTCON_DT_MATCH.as_ptr(),
    },
};

module_platform_driver!(QCOM_USB_EXTCON_DRIVER);

module_description!("QCOM USB ID extcon driver");
module_author!("Stephen Boyd <stephen.boyd@linaro.org>");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
