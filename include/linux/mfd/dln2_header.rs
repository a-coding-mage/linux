/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted; Rust modules provide equivalent inclusion
// protection.

#[inline]
pub const fn DLN2_CMD(cmd: u16, id: u16) -> u16 {
    cmd | id.wrapping_shl(8)
}

#[repr(C)]
pub struct dln2_platform_data {
    pub handle: u16, // sub-driver handle (internally used only)
    pub port: u8,   // I2C/SPI port
}

// dln2_event_cb_t - event callback function signature
//
// @pdev - the sub-device that registered this callback
// @echo - the echo header field received in the message
// @data - the data payload
// @len  - the data payload length
//
// The callback function is called in interrupt context and the data payload is
// only valid during the call. If the user needs later access of the data, it
// must copy it.
pub type dln2_event_cb_t = unsafe extern "C" fn(
    pdev: *mut platform_device,
    echo: u16,
    data: *const core::ffi::c_void,
    len: core::ffi::c_int,
);

// Opaque external type supplied by the platform dependency.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    // dl2n_register_event_cb - register a callback function for an event
    //
    // @pdev - the sub-device that registers the callback
    // @event - the event for which to register a callback
    // @event_cb - the callback function
    //
    // @return 0 in case of success, negative value in case of error
    pub fn dln2_register_event_cb(
        pdev: *mut platform_device,
        event: u16,
        event_cb: dln2_event_cb_t,
    ) -> core::ffi::c_int;

    // dln2_unregister_event_cb - unregister the callback function for an event
    //
    // @pdev - the sub-device that registered the callback
    // @event - the event for which to register a callback
    pub fn dln2_unregister_event_cb(pdev: *mut platform_device, event: u16);

    // dln2_transfer - issue a DLN2 command and wait for a response and the
    // associated data
    //
    // @pdev - the sub-device which is issuing this transfer
    // @cmd - the command to be sent to the device
    // @obuf - the buffer to be sent to the device; it can be NULL if the user
    // doesn't need to transmit data with this command
    // @obuf_len - the size of the buffer to be sent to the device
    // @ibuf - any data associated with the response will be copied here; it can be
    // NULL if the user doesn't need the response data
    // @ibuf_len - must be initialized to the input buffer size; it will be modified
    // to indicate the actual data transferred;
    //
    // @return 0 for success, negative value for errors
    pub fn dln2_transfer(
        pdev: *mut platform_device,
        cmd: u16,
        obuf: *const core::ffi::c_void,
        obuf_len: usize,
        ibuf: *mut core::ffi::c_void,
        ibuf_len: *mut usize,
    ) -> core::ffi::c_int;
}

// dln2_transfer_rx - variant of dln2_transfer() where TX buffer is not needed
#[inline]
pub unsafe fn dln2_transfer_rx(
    pdev: *mut platform_device,
    cmd: u16,
    ibuf: *mut core::ffi::c_void,
    ibuf_len: *mut usize,
) -> core::ffi::c_int {
    dln2_transfer(pdev, cmd, core::ptr::null(), 0, ibuf, ibuf_len)
}

// dln2_transfer_tx - variant of dln2_transfer() where RX buffer is not needed
#[inline]
pub unsafe fn dln2_transfer_tx(
    pdev: *mut platform_device,
    cmd: u16,
    obuf: *const core::ffi::c_void,
    obuf_len: usize,
) -> core::ffi::c_int {
    dln2_transfer(pdev, cmd, obuf, obuf_len, core::ptr::null_mut(), core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
