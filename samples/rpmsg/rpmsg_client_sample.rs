// SPDX-License-Identifier: GPL-2.0-only
/*
 * Remote processor messaging - sample client driver
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 * Copyright (C) 2011 Google, Inc.
 *
 * Ohad Ben-Cohen <ohad@wizery.com>
 * Brian Swetland <swetland@google.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

const MSG: &[u8] = b"hello world!";

static mut count: i32 = 100;

#[repr(C)]
struct instance_data {
    rx_count: i32,
}

unsafe fn rpmsg_sample_cb(
    rpdev: *mut rpmsg_device,
    data: *mut core::ffi::c_void,
    len: i32,
    _priv: *mut core::ffi::c_void,
    src: u32,
) -> i32 {
    let idata: *mut instance_data = dev_get_drvdata((*rpdev).dev);

    (*idata).rx_count = (*idata).rx_count.wrapping_add(1);
    dev_info(
        (*rpdev).dev,
        "incoming msg %d (src: 0x%x)\n",
        (*idata).rx_count,
        src,
    );

    print_hex_dump_debug(
        "rpmsg_sample_cb",
        DUMP_PREFIX_NONE,
        16,
        1,
        data,
        len,
        true,
    );

    /* samples should not live forever */
    if (*idata).rx_count >= count {
        dev_info((*rpdev).dev, "goodbye!\n");
        return 0;
    }

    /* send a new message now */
    let ret = rpmsg_send((*rpdev).ept, MSG.as_ptr() as *const core::ffi::c_void, MSG.len());
    if ret != 0 {
        dev_err((*rpdev).dev, "rpmsg_send failed: %d\n", ret);
    }

    0
}

unsafe fn rpmsg_sample_probe(rpdev: *mut rpmsg_device) -> i32 {
    let idata: *mut instance_data;

    dev_info(
        (*rpdev).dev,
        "new channel: 0x%x -> 0x%x!\n",
        (*rpdev).src,
        (*rpdev).dst,
    );

    idata = devm_kzalloc(
        (*rpdev).dev,
        core::mem::size_of::<instance_data>(),
        GFP_KERNEL,
    ) as *mut instance_data;
    if idata.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata((*rpdev).dev, idata as *mut core::ffi::c_void);

    /* send a message to our remote processor */
    let ret = rpmsg_send((*rpdev).ept, MSG.as_ptr() as *const core::ffi::c_void, MSG.len());
    if ret != 0 {
        dev_err((*rpdev).dev, "rpmsg_send failed: %d\n", ret);
        return ret;
    }

    0
}

unsafe fn rpmsg_sample_remove(rpdev: *mut rpmsg_device) {
    dev_info((*rpdev).dev, "rpmsg sample client driver is removed\n");
}

static mut rpmsg_driver_sample_id_table: [rpmsg_device_id; 2] = [
    rpmsg_device_id {
        name: *b"rpmsg-client-sample\0",
    },
    rpmsg_device_id { name: [0; 21] },
];

static mut rpmsg_sample_client: rpmsg_driver = rpmsg_driver {
    drv: driver { name: KBUILD_MODNAME },
    id_table: rpmsg_driver_sample_id_table.as_ptr(),
    probe: Some(rpmsg_sample_probe),
    callback: Some(rpmsg_sample_cb),
    remove: Some(rpmsg_sample_remove),
};

// MODULE_DEVICE_TABLE(rpmsg, rpmsg_driver_sample_id_table);
// module_rpmsg_driver(rpmsg_sample_client);
// MODULE_DESCRIPTION("Remote processor messaging sample client driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
