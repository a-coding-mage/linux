/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mctp-usb.h - MCTP USB transport binding: common definitions,
 * based on DMTF0283 specification:
 * https://www.dmtf.org/sites/default/files/standards/documents/DSP0283_1.1.0.pdf
 *
 * These are protocol-level definitions, that may be shared between host
 * and gadget drivers.
 *
 * Copyright (C) 2024-2025 Code Construct Pty Ltd
 */

use core::ffi::c_void;

/* Types supplied by the surrounding kernel translation. */
pub type __be16 = u16;
pub type gfp_t = usize;
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

pub type skb_drop_reason = u32;

/* MCTP-over-USB transport header. DSP0283 v1.0 has an 8-bit length field
 * (preceded by 8 reserved bits), v1.1 has a 13-bit length field (preceded by
 * 3 reserved bits). We use a be16 for our length to handle the larger v1.1
 * representation, and mask as appropriate.
 */
#[repr(C, packed)]
pub struct mctp_usb_hdr {
    pub id: __be16,
    pub len: __be16,
}

/* max transfer size for DSP0283 v1.0 */
pub const MCTP_USB_1_0_XFER_SIZE: usize = 512;
pub const MCTP_USB_BTU: usize = 68;
pub const MCTP_USB_MTU_MIN: usize = MCTP_USB_BTU;
pub const MCTP_USB_1_0_PKTLEN_MAX: usize = u8::MAX as usize;
pub const MCTP_USB_1_0_MTU_MAX: usize =
    MCTP_USB_1_0_PKTLEN_MAX - core::mem::size_of::<mctp_usb_hdr>();
pub const MCTP_USB_1_1_PKTLEN_MAX: usize = (1usize << 13) - 1;
pub const MCTP_USB_1_1_MTU_MAX: usize =
    MCTP_USB_1_1_PKTLEN_MAX - core::mem::size_of::<mctp_usb_hdr>();
pub const MCTP_USB_DMTF_ID: u16 = 0x1ab4;

/* mctp-usblib */

/* RX handle: drivers will typically create one on init, which persists for
 * the life of the driver. The same handle is used for progressive
 * prepare -> complete operations (for each incoming USB transfer), which
 * result in netif_rx()-ing the MCTP packets received
 */
#[repr(C)]
pub struct mctp_usblib_rx {
    pub skb: *mut sk_buff,
    pub ep_pktlen: u16,
    pub span: bool,
}

extern "C" {
    pub fn mctp_usblib_rx_init(rx: *mut mctp_usblib_rx, ep_pktlen: u16, span: bool) -> i32;
    pub fn mctp_usblib_rx_fini(rx: *mut mctp_usblib_rx);
    pub fn mctp_usblib_rx_prepare(
        netdev: *mut net_device,
        rx: *mut mctp_usblib_rx,
        bufp: *mut *mut c_void,
        lenp: *mut usize,
        gfp: gfp_t,
    ) -> i32;
    pub fn mctp_usblib_rx_complete(
        netdev: *mut net_device,
        rx: *mut mctp_usblib_rx,
        len: usize,
    ) -> i32;
    pub fn mctp_usblib_rx_cancel(rx: *mut mctp_usblib_rx);
}

/* TX handle: created by mctp_usblib_tx_push() during the tx path, and
 * may persist across multiple packet transmits.
 */
#[repr(C)]
pub struct mctp_usblib_tx_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mctp_usblib_tx_ops {
    /* Start a USB TX for @data. On returning success, the implementation
     * must arrange for mctp_usblib_tx_send_complete() to be called at some
     * later point (eg., on urb completion).
     */
    pub send: Option<unsafe extern "C" fn(*mut mctp_usblib_tx_ctx, *mut c_void, usize) -> i32>,
}

#[repr(C)]
pub struct mctp_usblib_tx {
    pub ops: mctp_usblib_tx_ops,
    pub priv_: *mut c_void,
    pub span: bool,
    /* protects access to cur_ctx */
    pub lock: spinlock_t,
    /* context to which we are adding packets, cleared on send */
    pub cur_ctx: *mut mctp_usblib_tx_ctx,
}

extern "C" {
    pub fn mctp_usblib_tx_init(
        tx: *mut mctp_usblib_tx,
        ops: *const mctp_usblib_tx_ops,
        priv_: *mut c_void,
        span: bool,
    );
    pub fn mctp_usblib_tx_fini(tx: *mut mctp_usblib_tx);
    pub fn mctp_usblib_tx_ctx_priv(tx_ctx: *mut mctp_usblib_tx_ctx) -> *mut c_void;
    pub fn mctp_usblib_tx_push(
        dev: *mut net_device,
        tx: *mut mctp_usblib_tx,
        skb: *mut sk_buff,
        more: bool,
    ) -> i32;
    pub fn mctp_usblib_tx_send_complete(
        tx_ctx: *mut mctp_usblib_tx_ctx,
        dev: *mut net_device,
        ok: bool,
    );
    pub fn mctp_usblib_tx_cancel(
        tx: *mut mctp_usblib_tx,
        dev: *mut net_device,
        reason: skb_drop_reason,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
