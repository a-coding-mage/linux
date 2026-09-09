/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2019 Google LLC.
 */

use core::ffi::{c_int, c_void};

/* Definitions supplied by the included Linux headers. */
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rproc_subdev {
    _private: [u8; 0],
}

pub type IpiHandlerT = unsafe extern "C" fn(data: *mut c_void, len: u32, priv_: *mut c_void);

/*
 * struct mtk_rpmsg_info - IPI functions tied to the rpmsg device.
 * @register_ipi: register IPI handler for an IPI id.
 * @unregister_ipi: unregister IPI handler for a registered IPI id.
 * @send_ipi: send IPI to an IPI id. wait is the timeout (in msecs) to wait
 *            until response, or 0 if there's no timeout.
 * @ns_ipi_id: the IPI id used for name service, or -1 if name service isn't
 *             supported.
 */
#[repr(C)]
pub struct mtk_rpmsg_info {
    pub register_ipi: Option<unsafe extern "C" fn(
        pdev: *mut platform_device,
        id: u32,
        handler: Option<IpiHandlerT>,
        priv_: *mut c_void,
    ) -> c_int>,
    pub unregister_ipi:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, id: u32)>,
    pub send_ipi: Option<unsafe extern "C" fn(
        pdev: *mut platform_device,
        id: u32,
        buf: *const c_void,
        len: u32,
        wait: u32,
    ) -> c_int>,
    pub ns_ipi_id: c_int,
}

extern "C" {
    pub fn mtk_rpmsg_create_rproc_subdev(
        pdev: *mut platform_device,
        info: *mut mtk_rpmsg_info,
    ) -> *mut rproc_subdev;

    pub fn mtk_rpmsg_destroy_rproc_subdev(subdev: *mut rproc_subdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
