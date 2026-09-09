/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct psample_group {
    pub list: list_head,
    pub net: *mut net,
    pub group_num: u32,
    pub refcount: u32,
    pub seq: u32,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct psample_metadata {
    pub trunc_size: u32,
    pub in_ifindex: i32,
    pub out_ifindex: i32,
    pub out_tc: u16,
    pub out_tc_occ: u64, /* bytes */
    pub latency: u64,    /* nanoseconds */
    /* C bit-fields: out_tc_valid:1, out_tc_occ_valid:1, latency_valid:1,
     * rate_as_probability:1, unused:4. */
    pub flags: u8,
    pub user_cookie: *const u8,
    pub user_cookie_len: u32,
}

extern "C" {
    pub fn psample_group_get(net: *mut net, group_num: u32) -> *mut psample_group;
    pub fn psample_group_take(group: *mut psample_group);
    pub fn psample_group_put(group: *mut psample_group);

    /* CONFIG_PSAMPLE-enabled declaration. */
    pub fn psample_sample_packet(
        group: *mut psample_group,
        skb: *const sk_buff,
        sample_rate: u32,
        md: *const psample_metadata,
    );
}

/* When CONFIG_PSAMPLE is disabled, the C header provides an empty inline
 * implementation of psample_sample_packet with the same signature. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
