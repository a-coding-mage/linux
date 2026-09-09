/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct datalink_proto {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct packet_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn register_snap_client(
        desc: *const u8,
        rcvfunc: Option<
            unsafe extern "C" fn(
                *mut sk_buff,
                *mut net_device,
                *mut packet_type,
                *mut net_device,
            ) -> i32,
        >,
    ) -> *mut datalink_proto;

    pub fn unregister_snap_client(proto: *mut datalink_proto);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
