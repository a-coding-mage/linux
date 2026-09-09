/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated Linux list header.
pub use crate::list_head;

pub struct llc_sap;
pub struct net_device;
pub struct packet_type;
pub struct sk_buff;

#[repr(C)]
pub struct datalink_proto {
    pub type_: [u8; 8],
    pub sap: *mut llc_sap,
    pub header_length: u16,
    pub rcvfunc: Option<unsafe extern "C" fn(
        *mut sk_buff,
        *mut net_device,
        *mut packet_type,
        *mut net_device,
    ) -> i32>,
    pub request: Option<unsafe extern "C" fn(
        *mut datalink_proto,
        *mut sk_buff,
        *const u8,
    ) -> i32>,
    pub node: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
