/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux/Rust bindings:
// RPMSG_NAME_SIZE, __rpmsg32, and rpmsg_device.

/**
 * struct rpmsg_ns_msg - dynamic name service announcement message
 * @name: name of remote service that is published
 * @addr: address of remote service that is published
 * @flags: indicates whether service is created or destroyed
 *
 * This message is sent across to publish a new service, or announce
 * about its removal. When we receive these messages, an appropriate
 * rpmsg channel (i.e device) is created/destroyed. In turn, the ->probe()
 * or ->remove() handler of the appropriate rpmsg driver will be invoked
 * (if/as-soon-as one is registered).
 */
#[repr(C, packed)]
pub struct rpmsg_ns_msg {
    pub name: [core::ffi::c_char; RPMSG_NAME_SIZE],
    pub addr: __rpmsg32,
    pub flags: __rpmsg32,
}

/**
 * enum rpmsg_ns_flags - dynamic name service announcement flags
 *
 * @RPMSG_NS_CREATE: a new remote service was just created
 * @RPMSG_NS_DESTROY: a known remote service was just destroyed
 */
#[repr(C)]
pub enum rpmsg_ns_flags {
    RPMSG_NS_CREATE = 0,
    RPMSG_NS_DESTROY = 1,
}

/* Address 53 is reserved for advertising remote services */
pub const RPMSG_NS_ADDR: u32 = 53;

unsafe extern "C" {
    pub fn rpmsg_ns_register_device(rpdev: *mut rpmsg_device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
