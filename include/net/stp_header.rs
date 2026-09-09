/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/if_ether.h.
pub const ETH_ALEN: usize = 6;

// Opaque types supplied by the surrounding networking implementation.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stp_proto {
    pub group_address: [u8; ETH_ALEN],
    pub rcv: Option<unsafe extern "C" fn(
        proto: *const stp_proto,
        skb: *mut sk_buff,
        dev: *mut net_device,
    )>,
    pub data: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn stp_proto_register(proto: *const stp_proto) -> core::ffi::c_int;
    pub fn stp_proto_unregister(proto: *const stp_proto);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
