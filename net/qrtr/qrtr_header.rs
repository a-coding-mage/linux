/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_void};

/* Opaque types supplied by the surrounding kernel interfaces. */
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qrtr_node {
    _private: [u8; 0],
}

/* endpoint node id auto assignment */
pub const QRTR_EP_NID_AUTO: c_int = -1;

/**
 * struct qrtr_endpoint - endpoint handle
 * @xmit: Callback for outgoing packets
 *
 * The socket buffer passed to the xmit function becomes owned by the endpoint
 * driver.  As such, when the driver is done with the buffer, it should
 * call kfree_skb() on failure, or consume_skb() on success.
 */
#[repr(C)]
pub struct qrtr_endpoint {
    pub xmit: Option<unsafe extern "C" fn(ep: *mut qrtr_endpoint, skb: *mut sk_buff) -> c_int>,
    /* private: not for endpoint use */
    pub node: *mut qrtr_node,
}

extern "C" {
    pub fn qrtr_endpoint_register(ep: *mut qrtr_endpoint, nid: u32) -> c_int;

    pub fn qrtr_endpoint_unregister(ep: *mut qrtr_endpoint);

    pub fn qrtr_endpoint_post(ep: *mut qrtr_endpoint, data: *const c_void, len: usize) -> c_int;

    pub fn qrtr_ns_init() -> c_int;

    pub fn qrtr_ns_remove();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
