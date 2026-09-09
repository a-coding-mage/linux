/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding kernel and KUnit dependencies
// are intentionally left external to this translation unit.

pub const MCTP_DEV_TEST_MTU: ::core::ffi::c_uint = 68;

#[repr(C)]
pub struct mctp_test_dev {
    pub ndev: *mut net_device,
    pub mdev: *mut mctp_dev,

    pub lladdr_len: ::core::ffi::c_ushort,
    pub lladdr: [::core::ffi::c_uchar; MAX_ADDR_LEN],

    pub pkts: sk_buff_head,
}

#[repr(C)]
pub struct mctp_test_route {
    pub rt: mctp_route,
}

#[repr(C)]
pub struct mctp_test_bind_setup {
    pub bind_addr: mctp_eid_t,
    pub bind_net: ::core::ffi::c_int,
    pub bind_type: u8,

    pub have_peer: bool,
    pub peer_addr: mctp_eid_t,
    pub peer_net: ::core::ffi::c_int,

    /* optional name. Used for comparison in "lookup" tests */
    pub name: *const ::core::ffi::c_char,
}

extern "C" {
    pub fn mctp_test_create_dev() -> *mut mctp_test_dev;
    pub fn mctp_test_create_dev_with_addr(eid: mctp_eid_t) -> *mut mctp_test_dev;
    pub fn mctp_test_create_dev_lladdr(
        lladdr_len: ::core::ffi::c_ushort,
        lladdr: *const ::core::ffi::c_uchar,
    ) -> *mut mctp_test_dev;
    pub fn mctp_test_destroy_dev(dev: *mut mctp_test_dev);

    pub fn mctp_test_create_route_direct(
        net: *mut net,
        dev: *mut mctp_dev,
        eid: mctp_eid_t,
        mtu: ::core::ffi::c_uint,
    ) -> *mut mctp_test_route;
    pub fn mctp_test_create_route_gw(
        net: *mut net,
        netid: ::core::ffi::c_uint,
        eid: mctp_eid_t,
        gw: mctp_eid_t,
        mtu: ::core::ffi::c_uint,
    ) -> *mut mctp_test_route;
    pub fn mctp_test_dst_setup(
        test: *mut kunit,
        dst: *mut mctp_dst,
        dev: *mut mctp_test_dev,
        mtu: ::core::ffi::c_uint,
    );
    pub fn mctp_test_route_destroy(test: *mut kunit, rt: *mut mctp_test_route);
    pub fn mctp_test_skb_set_dev(skb: *mut sk_buff, dev: *mut mctp_test_dev);
    pub fn mctp_test_create_skb(
        hdr: *const mctp_hdr,
        data_len: ::core::ffi::c_uint,
    ) -> *mut sk_buff;
    pub fn __mctp_test_create_skb_data(
        hdr: *const mctp_hdr,
        data: *const ::core::ffi::c_void,
        data_len: usize,
    ) -> *mut sk_buff;

    pub fn mctp_test_bind_run(
        test: *mut kunit,
        setup: *const mctp_test_bind_setup,
        ret_bind_errno: *mut ::core::ffi::c_int,
        sock: *mut *mut socket,
    );
}

#[macro_export]
macro_rules! mctp_test_create_skb_data {
    ($h:expr, $d:expr) => {
        $crate::__mctp_test_create_skb_data($h, $d, ::core::mem::size_of_val(&$d))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
