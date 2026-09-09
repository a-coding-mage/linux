// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/misc/xillybus_of.c
 *
 * Copyright 2011 Xillybus Ltd, http://xillybus.com
 *
 * Driver for the Xillybus FPGA/host framework using Open Firmware.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const MODULE_DESCRIPTION: &str = "Xillybus driver for Open Firmware";
const MODULE_AUTHOR: &str = "Eli Billauer, Xillybus Ltd.";
const MODULE_ALIAS: &str = "xillybus_of";
const MODULE_LICENSE: &str = "GPL v2";

static XILLYNAME: &[u8] = b"xillybus_of\0";

/* Match table for of_platform binding */
static XILLYBUS_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: b"xillybus,xillybus-1.00.a\0".as_ptr() },
    of_device_id { compatible: b"xlnx,xillybus-1.00.a\0".as_ptr() }, // Deprecated
    of_device_id { compatible: core::ptr::null() },
];

// Equivalent of MODULE_DEVICE_TABLE(of, xillybus_of_match).

unsafe fn xilly_drv_probe(op: *mut platform_device) -> c_int {
    let dev: *mut device = unsafe { &mut (*op).dev };
    let endpoint: *mut xilly_endpoint;
    let mut rc: c_int;
    let irq: c_int;

    endpoint = unsafe { xillybus_init_endpoint(dev) };

    if endpoint.is_null() {
        return -ENOMEM;
    }

    unsafe { dev_set_drvdata(dev, endpoint as *mut c_void) };

    unsafe { (*endpoint).owner = THIS_MODULE };

    unsafe { (*endpoint).registers = devm_platform_ioremap_resource(op, 0) };
    if unsafe { IS_ERR((*endpoint).registers) } {
        return unsafe { PTR_ERR((*endpoint).registers) };
    }

    irq = unsafe { platform_get_irq(op, 0) };

    rc = unsafe {
        devm_request_irq(
            dev,
            irq,
            xillybus_isr,
            0,
            XILLYNAME.as_ptr() as *const c_char,
            endpoint as *mut c_void,
        )
    };

    if rc != 0 {
        return -ENODEV;
    }

    unsafe { xillybus_endpoint_discovery(endpoint) }
}

unsafe fn xilly_drv_remove(op: *mut platform_device) {
    let dev: *mut device = unsafe { &mut (*op).dev };
    let endpoint: *mut xilly_endpoint =
        unsafe { dev_get_drvdata(dev) as *mut xilly_endpoint };

    unsafe { xillybus_endpoint_remove(endpoint) };
}

static mut XILLYBUS_PLATFORM_DRIVER: platform_driver = platform_driver {
    probe: Some(xilly_drv_probe),
    remove: Some(xilly_drv_remove),
    driver: driver {
        name: XILLYNAME.as_ptr() as *const c_char,
        of_match_table: XILLYBUS_OF_MATCH.as_ptr(),
    },
};

// Equivalent of module_platform_driver(xillybus_platform_driver).


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
