// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015, Sony Mobile Communications Inc.
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux kernel and qrtr headers are intentionally
// left as external Rust symbols.

#[repr(C)]
struct qrtr_smd_dev {
    ep: qrtr_endpoint,
    channel: *mut rpmsg_endpoint,
    dev: *mut device,
}

/* from smd to qrtr */
unsafe extern "C" fn qcom_smd_qrtr_callback(
    rpdev: *mut rpmsg_device,
    data: *mut core::ffi::c_void,
    len: i32,
    _priv: *mut core::ffi::c_void,
    _addr: u32,
) -> i32 {
    let qdev: *mut qrtr_smd_dev = dev_get_drvdata(unsafe { &mut (*rpdev).dev });
    let mut rc: i32;

    if qdev.is_null() {
        return -EAGAIN;
    }

    rc = qrtr_endpoint_post(unsafe { &mut (*qdev).ep }, data, len);
    if rc == -EINVAL {
        dev_err(unsafe { (*qdev).dev }, "invalid ipcrouter packet\n");
        /* return 0 to let smd drop the packet */
        rc = 0;
    }

    rc
}

/* from qrtr to smd */
unsafe extern "C" fn qcom_smd_qrtr_send(
    ep: *mut qrtr_endpoint,
    skb: *mut sk_buff,
) -> i32 {
    let qdev: *mut qrtr_smd_dev = container_of!(ep, qrtr_smd_dev, ep);
    let mut rc: i32;

    rc = skb_linearize(skb);
    if rc != 0 {
        return qcom_smd_qrtr_send_out(rc, skb);
    }

    rc = rpmsg_send(
        unsafe { (*qdev).channel },
        unsafe { (*skb).data },
        unsafe { (*skb).len },
    );

    qcom_smd_qrtr_send_out(rc, skb)
}

unsafe fn qcom_smd_qrtr_send_out(rc: i32, skb: *mut sk_buff) -> i32 {
    if rc != 0 {
        kfree_skb(skb);
    } else {
        consume_skb(skb);
    }
    rc
}

unsafe extern "C" fn qcom_smd_qrtr_probe(rpdev: *mut rpmsg_device) -> i32 {
    let qdev: *mut qrtr_smd_dev;
    let mut rc: i32;

    qdev = devm_kzalloc(
        unsafe { &mut (*rpdev).dev },
        core::mem::size_of::<qrtr_smd_dev>(),
        GFP_KERNEL,
    ) as *mut qrtr_smd_dev;
    if qdev.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*qdev).channel = (*rpdev).ept;
        (*qdev).dev = &mut (*rpdev).dev;
        (*qdev).ep.xmit = Some(qcom_smd_qrtr_send);
    }

    rc = qrtr_endpoint_register(unsafe { &mut (*qdev).ep }, QRTR_EP_NID_AUTO);
    if rc != 0 {
        return rc;
    }

    dev_set_drvdata(unsafe { &mut (*rpdev).dev }, qdev as *mut core::ffi::c_void);

    dev_dbg(unsafe { &mut (*rpdev).dev }, "Qualcomm SMD QRTR driver probed\n");

    0
}

unsafe extern "C" fn qcom_smd_qrtr_remove(rpdev: *mut rpmsg_device) {
    let qdev: *mut qrtr_smd_dev = dev_get_drvdata(unsafe { &mut (*rpdev).dev });

    qrtr_endpoint_unregister(unsafe { &mut (*qdev).ep });

    dev_set_drvdata(unsafe { &mut (*rpdev).dev }, core::ptr::null_mut());
}

static qcom_smd_qrtr_smd_match: [rpmsg_device_id; 2] = [
    rpmsg_device_id { name: "IPCRTR" },
    rpmsg_device_id { name: "" },
];

static mut qcom_smd_qrtr_driver: rpmsg_driver = rpmsg_driver {
    probe: Some(qcom_smd_qrtr_probe),
    remove: Some(qcom_smd_qrtr_remove),
    callback: Some(qcom_smd_qrtr_callback),
    id_table: qcom_smd_qrtr_smd_match.as_ptr(),
    drv: device_driver {
        name: "qcom_smd_qrtr",
    },
};

module_rpmsg_driver!(qcom_smd_qrtr_driver);

module_alias!("rpmsg:IPCRTR");
module_description!("Qualcomm IPC-Router SMD interface driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
