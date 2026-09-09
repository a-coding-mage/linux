/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux networking headers:
// linux/netdevice.h, linux/types.h, and net/rtnetlink.h.

unsafe extern "C" {
    fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> core::ffi::c_int;
}

pub unsafe fn netif_is_bareudp(dev: *const net_device) -> bool {
    !(*dev).rtnl_link_ops.is_null()
        && strcmp(
            (*(*dev).rtnl_link_ops).kind,
            b"bareudp\0".as_ptr() as *const core::ffi::c_char,
        ) == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
