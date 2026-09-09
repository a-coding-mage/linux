// SPDX-License-Identifier: GPL-2.0+
//
// extcon-ptn5150.c - PTN5150 CC logic extcon driver to support USB detection
//
// Based on extcon-sm5502.c driver
// Copyright (c) 2018-2019 by Vijai Kumar K
// Author: Vijai Kumar K <vijaikumar.kanagarajan@gmail.com>
// Copyright (c) 2020 Krzysztof Kozlowski <krzk@kernel.org>

// Linux kernel dependencies supplied by other translation units.

const PTN5150_REG_DEVICE_ID: u8 = 0x01;
const PTN5150_REG_CONTROL: u8 = 0x02;
const PTN5150_REG_INT_STATUS: u8 = 0x03;
const PTN5150_REG_CC_STATUS: u8 = 0x04;
const PTN5150_REG_CON_DET: u8 = 0x09;
const PTN5150_REG_VCONN_STATUS: u8 = 0x0a;
const PTN5150_REG_RESET: u8 = 0x0b;
const PTN5150_REG_INT_MASK: u8 = 0x18;
const PTN5150_REG_INT_REG_STATUS: u8 = 0x19;
const PTN5150_REG_END: u8 = PTN5150_REG_INT_REG_STATUS;

const PTN5150_DFP_ATTACHED: u32 = 0x1;
const PTN5150_UFP_ATTACHED: u32 = 0x2;
const PTN5150_REG_DEVICE_ID_VERSION: u32 = 0xf8;
const PTN5150_REG_DEVICE_ID_VENDOR: u32 = 0x07;
const PTN5150_POLARITY_CC1: u32 = 0x1;
const PTN5150_POLARITY_CC2: u32 = 0x2;
const PTN5150_REG_CC_PORT_ATTACHMENT: u32 = 0x1c;
const PTN5150_REG_CC_POLARITY: u32 = 0x03;
const PTN5150_REG_CC_VBUS_DETECTION: u32 = 1 << 7;
const PTN5150_REG_INT_CABLE_ATTACH_MASK: u32 = 1 << 0;
const PTN5150_REG_INT_CABLE_DETACH_MASK: u32 = 1 << 1;

#[repr(C)]
struct ptn5150_info {
    dev: *mut device,
    edev: *mut extcon_dev,
    i2c: *mut i2c_client,
    regmap: *mut regmap,
    int_gpiod: *mut gpio_desc,
    vbus_gpiod: *mut gpio_desc,
    irq: i32,
    irq_work: work_struct,
    mutex: mutex,
    orient_sw: *mut typec_switch,
    role_sw: *mut usb_role_switch,
}

static PTN5150_EXTCON_CABLE: [u32; 3] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_NONE];

#[repr(C)]
static PTN5150_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: PTN5150_REG_END,
};

unsafe fn ptn5150_check_state(info: *mut ptn5150_info) {
    let mut orient: typec_orientation = TYPEC_ORIENTATION_NONE;
    let mut port_status: u32;
    let mut reg_data: u32 = 0;
    let mut vbus: u32;
    let mut usb_role: usb_role = USB_ROLE_NONE;
    let mut ret: i32;

    ret = regmap_read((*info).regmap, PTN5150_REG_CC_STATUS, &mut reg_data);
    if ret != 0 { dev_err((*info).dev, "failed to read CC STATUS %d\n", ret); return; }
    orient = ((reg_data & PTN5150_REG_CC_POLARITY) as typec_orientation);
    orient = match orient as u32 {
        PTN5150_POLARITY_CC1 => TYPEC_ORIENTATION_NORMAL,
        PTN5150_POLARITY_CC2 => TYPEC_ORIENTATION_REVERSE,
        _ => TYPEC_ORIENTATION_NONE,
    };
    ret = typec_switch_set((*info).orient_sw, orient);
    if ret != 0 { dev_err((*info).dev, "failed to set orientation: %d\n", ret); }
    port_status = (reg_data & PTN5150_REG_CC_PORT_ATTACHMENT) >> 2;
    match port_status {
        PTN5150_DFP_ATTACHED => {
            extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, false);
            gpiod_set_value_cansleep((*info).vbus_gpiod, 0);
            extcon_set_state_sync((*info).edev, EXTCON_USB, true);
            usb_role = USB_ROLE_DEVICE;
        },
        PTN5150_UFP_ATTACHED => {
            extcon_set_state_sync((*info).edev, EXTCON_USB, false);
            vbus = (reg_data & PTN5150_REG_CC_VBUS_DETECTION) >> 7;
            gpiod_set_value_cansleep((*info).vbus_gpiod, if vbus != 0 { 0 } else { 1 });
            extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, true);
            usb_role = USB_ROLE_HOST;
        },
        _ => {}
    }
    if usb_role != USB_ROLE_NONE {
        ret = usb_role_switch_set_role((*info).role_sw, usb_role);
        if ret != 0 { dev_err((*info).dev, "failed to set %s role: %d\n", usb_role_string(usb_role), ret); }
    }
}

unsafe extern "C" fn ptn5150_irq_work(work: *mut work_struct) {
    let info = container_of(work, ptn5150_info, irq_work);
    let mut int_status: u32 = 0;
    let mut ret: i32;
    if (*info).edev.is_null() { return; }
    mutex_lock(&mut (*info).mutex);
    ret = regmap_read((*info).regmap, PTN5150_REG_INT_STATUS, &mut int_status);
    if ret != 0 { dev_err((*info).dev, "failed to read INT STATUS %d\n", ret); mutex_unlock(&mut (*info).mutex); return; }
    if int_status != 0 {
        if int_status & PTN5150_REG_INT_CABLE_ATTACH_MASK != 0 {
            ptn5150_check_state(info);
        } else {
            extcon_set_state_sync((*info).edev, EXTCON_USB_HOST, false);
            extcon_set_state_sync((*info).edev, EXTCON_USB, false);
            gpiod_set_value_cansleep((*info).vbus_gpiod, 0);
            ret = usb_role_switch_set_role((*info).role_sw, USB_ROLE_NONE);
            if ret != 0 { dev_err((*info).dev, "failed to set none role: %d\n", ret); }
            ret = typec_switch_set((*info).orient_sw, TYPEC_ORIENTATION_NONE);
            if ret != 0 { dev_err((*info).dev, "failed to set orientation: %d\n", ret); }
        }
    }
    ret = regmap_read((*info).regmap, PTN5150_REG_INT_REG_STATUS, &mut int_status);
    if ret != 0 { dev_err((*info).dev, "failed to read INT REG STATUS %d\n", ret); mutex_unlock(&mut (*info).mutex); return; }
    mutex_unlock(&mut (*info).mutex);
}

unsafe extern "C" fn ptn5150_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let info = data as *mut ptn5150_info;
    schedule_work(&mut (*info).irq_work);
    IRQ_HANDLED
}

unsafe fn ptn5150_init_dev_type(info: *mut ptn5150_info) -> i32 {
    let mut reg_data = 0u32;
    let mut ret = regmap_read((*info).regmap, PTN5150_REG_DEVICE_ID, &mut reg_data);
    if ret != 0 { dev_err((*info).dev, "failed to read DEVICE_ID %d\n", ret); return -EINVAL; }
    let vendor_id = reg_data & PTN5150_REG_DEVICE_ID_VENDOR;
    let version_id = (reg_data & PTN5150_REG_DEVICE_ID_VERSION) >> 3;
    dev_dbg((*info).dev, "Device type: version: 0x%x, vendor: 0x%x\n", version_id, vendor_id);
    ret = regmap_read((*info).regmap, PTN5150_REG_INT_STATUS, &mut reg_data);
    if ret != 0 { dev_err((*info).dev, "failed to read PTN5150_REG_INT_STATUS %d\n", ret); return -EINVAL; }
    ret = regmap_read((*info).regmap, PTN5150_REG_INT_REG_STATUS, &mut reg_data);
    if ret != 0 { dev_err((*info).dev, "failed to read PTN5150_REG_INT_REG_STATUS %d\n", ret); return -EINVAL; }
    0
}

unsafe fn ptn5150_work_sync_and_put(data: *mut core::ffi::c_void) {
    let info = data as *mut ptn5150_info;
    cancel_work_sync(&mut (*info).irq_work);
    usb_role_switch_put((*info).role_sw);
    typec_switch_put((*info).orient_sw);
}

unsafe extern "C" fn ptn5150_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let dev = &mut (*i2c).dev;
    let np = (*i2c).dev.of_node;
    if np.is_null() { return -EINVAL; }
    let info = devm_kzalloc(dev, core::mem::size_of::<ptn5150_info>(), GFP_KERNEL) as *mut ptn5150_info;
    if info.is_null() { return -ENOMEM; }
    i2c_set_clientdata(i2c, info as *mut core::ffi::c_void);
    (*info).dev = dev;
    (*info).i2c = i2c;
    (*info).vbus_gpiod = devm_gpiod_get(dev, "vbus", GPIOD_OUT_LOW);
    if is_err((*info).vbus_gpiod) {
        let ret = ptr_err((*info).vbus_gpiod);
        if ret == -ENOENT { dev_info(dev, "No VBUS GPIO, ignoring VBUS control\n"); (*info).vbus_gpiod = core::ptr::null_mut(); }
        else { return dev_err_probe(dev, ret, "failed to get VBUS GPIO\n"); }
    }
    mutex_init(&mut (*info).mutex);
    init_work(&mut (*info).irq_work, ptn5150_irq_work);
    (*info).regmap = devm_regmap_init_i2c(i2c, &PTN5150_REGMAP_CONFIG);
    if is_err((*info).regmap) { return dev_err_probe(dev, ptr_err((*info).regmap), "failed to allocate register map\n"); }
    (*info).irq = (*i2c).irq;
    if (*info).irq <= 0 {
        (*info).int_gpiod = devm_gpiod_get(dev, "int", GPIOD_IN);
        if is_err((*info).int_gpiod) { return dev_err_probe(dev, ptr_err((*info).int_gpiod), "failed to get INT GPIO\n"); }
        (*info).irq = gpiod_to_irq((*info).int_gpiod);
        if (*info).irq < 0 { dev_err(dev, "failed to get INTB IRQ\n"); return (*info).irq; }
    }
    let ret = devm_request_threaded_irq(dev, (*info).irq, None, Some(ptn5150_irq_handler), IRQF_TRIGGER_FALLING | IRQF_ONESHOT, (*i2c).name, info as *mut core::ffi::c_void);
    if ret < 0 { dev_err(dev, "failed to request handler for INTB IRQ\n"); return ret; }
    (*info).edev = devm_extcon_dev_allocate(dev, PTN5150_EXTCON_CABLE.as_ptr());
    if is_err((*info).edev) { dev_err(dev, "failed to allocate memory for extcon\n"); return -ENOMEM; }
    let ret = devm_extcon_dev_register(dev, (*info).edev);
    if ret != 0 { dev_err(dev, "failed to register extcon device\n"); return ret; }
    extcon_set_property_capability((*info).edev, EXTCON_USB, EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability((*info).edev, EXTCON_USB_HOST, EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability((*info).edev, EXTCON_USB_HOST, EXTCON_PROP_USB_TYPEC_POLARITY);
    if ptn5150_init_dev_type(info) != 0 { return -EINVAL; }
    let connector = device_get_named_child_node(dev, "connector");
    if !connector.is_null() {
        (*info).orient_sw = fwnode_typec_switch_get(connector);
        if is_err((*info).orient_sw) { return dev_err_probe(dev, ptr_err((*info).orient_sw), "failed to get orientation switch\n"); }
    }
    (*info).role_sw = usb_role_switch_get(dev);
    if (*info).role_sw.is_null() && !connector.is_null() { (*info).role_sw = fwnode_usb_role_switch_get(connector); }
    if is_err((*info).role_sw) { return dev_err_probe(dev, ptr_err((*info).role_sw), "failed to get role switch\n"); }
    let ret = devm_add_action_or_reset(dev, ptn5150_work_sync_and_put, info as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    mutex_lock(&mut (*info).mutex);
    ptn5150_check_state(info);
    mutex_unlock(&mut (*info).mutex);
    0
}

unsafe extern "C" fn ptn5150_resume(dev: *mut device) -> i32 {
    let i2c = to_i2c_client(dev);
    let info = i2c_get_clientdata(i2c) as *mut ptn5150_info;
    schedule_work(&mut (*info).irq_work);
    0
}

// Device tables, PM operations, driver registration, and module metadata:
// { compatible = "nxp,ptn5150" }, { "ptn5150" }, driver name "ptn5150",
// description "NXP PTN5150 CC logic Extcon driver", authors Vijai Kumar K and
// Krzysztof Kozlowski, and license GPL v2.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
