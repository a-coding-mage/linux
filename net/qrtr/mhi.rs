// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the kernel, MHI, QRTR, and networking subsystems
// are intentionally referenced here rather than reimplemented.

#[repr(C)]
struct QrtrMhiDev {
    ep: qrtr_endpoint,
    mhi_dev: *mut mhi_device,
    dev: *mut device,
}

/* From MHI to QRTR */
unsafe extern "C" fn qcom_mhi_qrtr_dl_callback(
    mhi_dev: *mut mhi_device,
    mhi_res: *mut mhi_result,
) {
    let qdev = dev_get_drvdata(&mut (*mhi_dev).dev);
    let rc: i32;

    if qdev.is_null()
        || ((*mhi_res).transaction_status != 0
            && (*mhi_res).transaction_status != -ENOTCONN)
    {
        return;
    }

    /* Channel got reset. So just free the buffer */
    if (*mhi_res).transaction_status == -ENOTCONN {
        devm_kfree(&mut (*mhi_dev).dev, (*mhi_res).buf_addr);
        return;
    }

    rc = qrtr_endpoint_post(
        &mut (*qdev).ep,
        (*mhi_res).buf_addr,
        (*mhi_res).bytes_xferd,
    );
    if rc == -EINVAL {
        dev_err((*qdev).dev, "invalid ipcrouter packet\n");
    }

    /* Done with the buffer, now recycle it for future use */
    rc = mhi_queue_buf(
        mhi_dev,
        DMA_FROM_DEVICE,
        (*mhi_res).buf_addr,
        (*(*mhi_dev).mhi_cntrl).buffer_len,
        MHI_EOT,
    );
    if rc != 0 {
        dev_err(&mut (*mhi_dev).dev, "Failed to recycle the buffer: %d\n", rc);
    }
}

/* From QRTR to MHI */
unsafe extern "C" fn qcom_mhi_qrtr_ul_callback(
    _mhi_dev: *mut mhi_device,
    mhi_res: *mut mhi_result,
) {
    let skb = (*mhi_res).buf_addr as *mut sk_buff;

    if !(*skb).sk.is_null() {
        sock_put((*skb).sk);
    }
    consume_skb(skb);
}

/* Send data over MHI */
unsafe extern "C" fn qcom_mhi_qrtr_send(
    ep: *mut qrtr_endpoint,
    skb: *mut sk_buff,
) -> i32 {
    let qdev = container_of!(ep, QrtrMhiDev, ep);
    let rc: i32;

    if !(*skb).sk.is_null() {
        sock_hold((*skb).sk);
    }

    rc = skb_linearize(skb);
    if rc != 0 {
        goto_free_skb!(skb, rc);
    }

    rc = mhi_queue_skb(qdev.mhi_dev, DMA_TO_DEVICE, skb, (*skb).len, MHI_EOT);
    if rc != 0 {
        goto_free_skb!(skb, rc);
    }

    return rc;
}

unsafe fn qcom_mhi_qrtr_queue_dl_buffers(mhi_dev: *mut mhi_device) -> i32 {
    let mut free_desc = mhi_get_free_desc_count(mhi_dev, DMA_FROM_DEVICE);
    let buf: *mut core::ffi::c_void;
    let ret: i32;

    while free_desc != 0 {
        free_desc -= 1;
        buf = devm_kmalloc(
            &mut (*mhi_dev).dev,
            (*(*mhi_dev).mhi_cntrl).buffer_len,
            GFP_KERNEL,
        );
        if buf.is_null() {
            return -ENOMEM;
        }

        ret = mhi_queue_buf(
            mhi_dev,
            DMA_FROM_DEVICE,
            buf,
            (*(*mhi_dev).mhi_cntrl).buffer_len,
            MHI_EOT,
        );
        if ret != 0 {
            dev_err(&mut (*mhi_dev).dev, "Failed to queue buffer: %d\n", ret);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn qcom_mhi_qrtr_probe(
    mhi_dev: *mut mhi_device,
    _id: *const mhi_device_id,
) -> i32 {
    let qdev = devm_kzalloc(&mut (*mhi_dev).dev, core::mem::size_of::<QrtrMhiDev>(), GFP_KERNEL)
        as *mut QrtrMhiDev;
    let rc: i32;

    if qdev.is_null() {
        return -ENOMEM;
    }

    (*qdev).mhi_dev = mhi_dev;
    (*qdev).dev = &mut (*mhi_dev).dev;
    (*qdev).ep.xmit = Some(qcom_mhi_qrtr_send);

    dev_set_drvdata(&mut (*mhi_dev).dev, qdev as *mut core::ffi::c_void);

    /* start channels */
    rc = mhi_prepare_for_transfer(mhi_dev);
    if rc != 0 {
        return rc;
    }

    rc = qrtr_endpoint_register(&mut (*qdev).ep, QRTR_EP_NID_AUTO);
    if rc != 0 {
        mhi_unprepare_from_transfer(mhi_dev);
        return rc;
    }

    rc = qcom_mhi_qrtr_queue_dl_buffers(mhi_dev);
    if rc != 0 {
        qrtr_endpoint_unregister(&mut (*qdev).ep);
        mhi_unprepare_from_transfer(mhi_dev);
        return rc;
    }

    dev_dbg((*qdev).dev, "Qualcomm MHI QRTR driver probed\n");
    0
}

unsafe extern "C" fn qcom_mhi_qrtr_remove(mhi_dev: *mut mhi_device) {
    let qdev = dev_get_drvdata(&mut (*mhi_dev).dev) as *mut QrtrMhiDev;
    qrtr_endpoint_unregister(&mut (*qdev).ep);
    mhi_unprepare_from_transfer(mhi_dev);
    dev_set_drvdata(&mut (*mhi_dev).dev, core::ptr::null_mut());
}

static QCOM_MHI_QRTR_ID_TABLE: &[mhi_device_id] = &[
    mhi_device_id { chan: "IPCR", ..mhi_device_id::DEFAULT },
    mhi_device_id::DEFAULT,
];

unsafe extern "C" fn qcom_mhi_qrtr_pm_suspend_late(dev: *mut device) -> i32 {
    let mhi_dev = container_of!(dev, mhi_device, dev);
    let state = mhi_get_mhi_state((*mhi_dev).mhi_cntrl);
    /* If the device is in suspend state, then no need for the client driver to unprepare the channels. */
    if state == MHI_STATE_M3 { return 0; }
    mhi_unprepare_from_transfer(mhi_dev);
    0
}

unsafe extern "C" fn qcom_mhi_qrtr_pm_resume_early(dev: *mut device) -> i32 {
    let mhi_dev = container_of!(dev, mhi_device, dev);
    let state = mhi_get_mhi_state((*mhi_dev).mhi_cntrl);
    /* If the device is in suspend state, we won't unprepare channels in suspend callback, therefore no need to prepare channels when resume. */
    if state == MHI_STATE_M3 { return 0; }
    let rc = mhi_prepare_for_transfer(mhi_dev);
    if rc != 0 {
        dev_err(dev, "failed to prepare for autoqueue transfer %d\n", rc);
        return rc;
    }
    qcom_mhi_qrtr_queue_dl_buffers(mhi_dev)
}

// SET_LATE_SYSTEM_SLEEP_PM_OPS(qcom_mhi_qrtr_pm_suspend_late, qcom_mhi_qrtr_pm_resume_early)
static QCOM_MHI_QRTR_PM_OPS: dev_pm_ops = dev_pm_ops::new();

static mut QCOM_MHI_QRTR_DRIVER: mhi_driver = mhi_driver {
    probe: Some(qcom_mhi_qrtr_probe),
    remove: Some(qcom_mhi_qrtr_remove),
    dl_xfer_cb: Some(qcom_mhi_qrtr_dl_callback),
    ul_xfer_cb: Some(qcom_mhi_qrtr_ul_callback),
    id_table: QCOM_MHI_QRTR_ID_TABLE.as_ptr(),
    driver: driver {
        name: "qcom_mhi_qrtr",
        pm: &QCOM_MHI_QRTR_PM_OPS,
    },
};

// module_mhi_driver(QCOM_MHI_QRTR_DRIVER);
// MODULE_AUTHOR("Chris Lew <clew@codeaurora.org>");
// MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
// MODULE_DESCRIPTION("Qualcomm IPC-Router MHI interface driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
