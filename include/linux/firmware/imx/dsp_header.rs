/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2019 NXP
 *
 * Header file for the DSP IPC implementation
 */

// C dependencies: <linux/device.h>, <linux/types.h>, and
// <linux/mailbox_client.h> provide the referenced kernel types and errors.

pub const DSP_MU_CHAN_NUM: usize = 4;

#[repr(C)]
pub struct imx_dsp_chan {
    pub ipc: *mut imx_dsp_ipc,
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub name: *mut core::ffi::c_char,
    pub idx: core::ffi::c_int,
}

#[repr(C)]
pub struct imx_dsp_ops {
    pub handle_reply: Option<unsafe extern "C" fn(ipc: *mut imx_dsp_ipc)>,
    pub handle_request: Option<unsafe extern "C" fn(ipc: *mut imx_dsp_ipc)>,
}

#[repr(C)]
pub struct imx_dsp_ipc {
    /* Host <-> DSP communication uses 2 txdb and 2 rxdb channels */
    pub chans: [imx_dsp_chan; DSP_MU_CHAN_NUM],
    pub dev: *mut device,
    pub ops: *mut imx_dsp_ops,
    pub private_data: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn imx_dsp_set_data(ipc: *mut imx_dsp_ipc, data: *mut core::ffi::c_void) {
    (*ipc).private_data = data;
}

#[inline]
pub unsafe fn imx_dsp_get_data(ipc: *mut imx_dsp_ipc) -> *mut core::ffi::c_void {
    (*ipc).private_data
}

/* The following declarations are provided when CONFIG_IMX_DSP is enabled. */
#[cfg(feature = "CONFIG_IMX_DSP")]
extern "C" {
    pub fn imx_dsp_ring_doorbell(
        dsp: *mut imx_dsp_ipc,
        chan_idx: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn imx_dsp_request_channel(
        ipc: *mut imx_dsp_ipc,
        idx: core::ffi::c_int,
    ) -> *mut mbox_chan;
    pub fn imx_dsp_free_channel(ipc: *mut imx_dsp_ipc, idx: core::ffi::c_int);
}

/* CONFIG_IMX_DSP disabled: these are the header's inline fallback definitions. */
#[cfg(not(feature = "CONFIG_IMX_DSP"))]
#[inline]
pub unsafe fn imx_dsp_ring_doorbell(
    _ipc: *mut imx_dsp_ipc,
    _chan_idx: core::ffi::c_uint,
) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_DSP"))]
#[inline]
pub unsafe fn imx_dsp_request_channel(
    _ipc: *mut imx_dsp_ipc,
    _idx: core::ffi::c_int,
) -> *mut mbox_chan {
    ERR_PTR(-EOPNOTSUPP)
}

#[cfg(not(feature = "CONFIG_IMX_DSP"))]
#[inline]
pub unsafe fn imx_dsp_free_channel(_ipc: *mut imx_dsp_ipc, _idx: core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
