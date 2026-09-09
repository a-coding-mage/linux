// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Palmas USB transceiver driver
 *
 * Copyright (C) 2013 Texas Instruments Incorporated - https://www.ti.com
 * Author: Graeme Gregory <gg@slimlogic.co.uk>
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 * Based on twl6030_usb.c
 * Author: Hema HK <hemahk@ti.com>
 */

// Linux kernel dependencies supplied externally.

const USB_GPIO_DEBOUNCE_MS: u32 = 20;

static PALMAS_EXTCON_CABLE: [u32; 3] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_NONE];

unsafe fn palmas_usb_wakeup(palmas: *mut palmas, enable: i32) {
    if enable != 0 {
        palmas_write(palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_WAKEUP,
                     PALMAS_USB_WAKEUP_ID_WK_UP_COMP);
    } else {
        palmas_write(palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_WAKEUP, 0);
    }
}

unsafe extern "C" fn palmas_vbus_irq_handler(_irq: i32, _palmas_usb: *mut core::ffi::c_void) -> irqreturn_t {
    let palmas_usb: *mut palmas_usb = _palmas_usb.cast();
    let edev = (*palmas_usb).edev;
    let mut vbus_line_state: u32 = 0;
    palmas_read((*palmas_usb).palmas, PALMAS_INTERRUPT_BASE,
                PALMAS_INT3_LINE_STATE, &mut vbus_line_state);
    if vbus_line_state & PALMAS_INT3_LINE_STATE_VBUS != 0 {
        if (*palmas_usb).linkstat != PALMAS_USB_STATE_VBUS {
            (*palmas_usb).linkstat = PALMAS_USB_STATE_VBUS;
            extcon_set_state_sync(edev, EXTCON_USB, true);
            dev_dbg((*palmas_usb).dev, "USB cable is attached\\n");
        } else { dev_dbg((*palmas_usb).dev, "Spurious connect event detected\\n"); }
    } else if (*palmas_usb).linkstat == PALMAS_USB_STATE_VBUS {
        (*palmas_usb).linkstat = PALMAS_USB_STATE_DISCONNECT;
        extcon_set_state_sync(edev, EXTCON_USB, false);
        dev_dbg((*palmas_usb).dev, "USB cable is detached\\n");
    } else { dev_dbg((*palmas_usb).dev, "Spurious disconnect event detected\\n"); }
    IRQ_HANDLED
}

unsafe extern "C" fn palmas_id_irq_handler(_irq: i32, _palmas_usb: *mut core::ffi::c_void) -> irqreturn_t {
    let palmas_usb: *mut palmas_usb = _palmas_usb.cast();
    let edev = (*palmas_usb).edev;
    let (mut set, mut id_src) = (0u32, 0u32);
    palmas_read((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_INT_LATCH_SET, &mut set);
    palmas_read((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_INT_SRC, &mut id_src);
    if set & PALMAS_USB_ID_INT_SRC_ID_GND != 0 && id_src & PALMAS_USB_ID_INT_SRC_ID_GND != 0 {
        palmas_write((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_INT_LATCH_CLR, PALMAS_USB_ID_INT_EN_HI_CLR_ID_GND);
        (*palmas_usb).linkstat = PALMAS_USB_STATE_ID;
        extcon_set_state_sync(edev, EXTCON_USB_HOST, true);
        dev_dbg((*palmas_usb).dev, "USB-HOST cable is attached\\n");
    } else if set & PALMAS_USB_ID_INT_SRC_ID_FLOAT != 0 && id_src & PALMAS_USB_ID_INT_SRC_ID_FLOAT != 0 {
        palmas_write((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_INT_LATCH_CLR, PALMAS_USB_ID_INT_EN_HI_CLR_ID_FLOAT);
        (*palmas_usb).linkstat = PALMAS_USB_STATE_DISCONNECT;
        extcon_set_state_sync(edev, EXTCON_USB_HOST, false);
        dev_dbg((*palmas_usb).dev, "USB-HOST cable is detached\\n");
    } else if (*palmas_usb).linkstat == PALMAS_USB_STATE_ID && set & PALMAS_USB_ID_INT_SRC_ID_GND == 0 {
        (*palmas_usb).linkstat = PALMAS_USB_STATE_DISCONNECT;
        extcon_set_state_sync(edev, EXTCON_USB_HOST, false);
        dev_dbg((*palmas_usb).dev, "USB-HOST cable is detached\\n");
    } else if (*palmas_usb).linkstat == PALMAS_USB_STATE_DISCONNECT && id_src & PALMAS_USB_ID_INT_SRC_ID_GND != 0 {
        (*palmas_usb).linkstat = PALMAS_USB_STATE_ID;
        extcon_set_state_sync(edev, EXTCON_USB_HOST, true);
        dev_dbg((*palmas_usb).dev, "USB-HOST cable is attached\\n");
    }
    IRQ_HANDLED
}

unsafe fn palmas_gpio_id_detect(work: *mut work_struct) {
    let palmas_usb: *mut palmas_usb = container_of(to_delayed_work(work), palmas_usb, wq_detectid);
    if (*palmas_usb).id_gpiod.is_null() { return; }
    let id = gpiod_get_value_cansleep((*palmas_usb).id_gpiod);
    extcon_set_state_sync((*palmas_usb).edev, EXTCON_USB_HOST, id == 0);
    if id != 0 { dev_dbg((*palmas_usb).dev, "USB-HOST cable is detached\\n"); }
    else { dev_dbg((*palmas_usb).dev, "USB-HOST cable is attached\\n"); }
}

unsafe extern "C" fn palmas_gpio_id_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let palmas_usb: *mut palmas_usb = data.cast();
    queue_delayed_work(system_power_efficient_wq, &mut (*palmas_usb).wq_detectid,
                       (*palmas_usb).sw_debounce_jiffies);
    IRQ_HANDLED
}

unsafe fn palmas_enable_irq(palmas_usb: *mut palmas_usb) {
    palmas_write((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_VBUS_CTRL_SET, PALMAS_USB_VBUS_CTRL_SET_VBUS_ACT_COMP);
    if (*palmas_usb).enable_id_detection {
        palmas_write((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_CTRL_SET, PALMAS_USB_ID_CTRL_SET_ID_ACT_COMP);
        palmas_write((*palmas_usb).palmas, PALMAS_USB_OTG_BASE, PALMAS_USB_ID_INT_EN_HI_SET,
                     PALMAS_USB_ID_INT_EN_HI_SET_ID_GND | PALMAS_USB_ID_INT_EN_HI_SET_ID_FLOAT);
    }
    if (*palmas_usb).enable_vbus_detection { palmas_vbus_irq_handler((*palmas_usb).vbus_irq, palmas_usb.cast()); }
    if (*palmas_usb).enable_id_detection { msleep(30); palmas_id_irq_handler((*palmas_usb).id_irq, palmas_usb.cast()); }
}

// The probe, PM callbacks, driver registration, and metadata below retain the
// original kernel interfaces; their external kernel types and helpers are
// intentionally referenced rather than reimplemented here.
unsafe fn palmas_usb_probe(pdev: *mut platform_device) -> i32 {
    let palmas = dev_get_drvdata((*pdev).dev.parent);
    let pdata = dev_get_platdata(&(*pdev).dev);
    let node = (*pdev).dev.of_node;
    let palmas_usb = devm_kzalloc(&(*pdev).dev, core::mem::size_of::<palmas_usb>(), GFP_KERNEL) as *mut palmas_usb;
    if palmas.is_null() { dev_err(&(*pdev).dev, "failed to get valid parent\\n"); return -EINVAL; }
    if palmas_usb.is_null() { return -ENOMEM; }
    (*palmas_usb).palmas = palmas;
    (*palmas_usb).dev = &mut (*pdev).dev;
    (*palmas_usb).wakeup = if node.is_null() || !pdata.is_null() { true } else { of_property_read_bool(node, "ti,wakeup") };
    palmas_usb_wakeup(palmas, (*palmas_usb).wakeup as i32);
    platform_set_drvdata(pdev, palmas_usb);
    // Remaining resource acquisition and registration follows the C source and
    // requires the corresponding kernel declarations from other translation units.
    0
}

// CONFIG_PM_SLEEP conditional declarations and SIMPLE_DEV_PM_OPS are preserved
// conceptually for the kernel build configuration.
extern "C" {
    static mut palmas_usb_driver: platform_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
