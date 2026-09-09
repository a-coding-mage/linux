/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translated from <linux/igmp_internal.h>.

#[repr(C)]
pub struct inet_fill_args {
    pub portid: u32,
    pub seq: u32,
    pub event: i32,
    pub flags: u32,
    pub netnsid: i32,
    pub ifindex: i32,
}

unsafe extern "C" {
    pub fn inet_fill_ifmcaddr(
        skb: *mut sk_buff,
        dev: *mut net_device,
        im: *const ip_mc_list,
        args: *mut inet_fill_args,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
