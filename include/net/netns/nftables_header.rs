/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct netns_nftables {
    pub base_seq: u32,
    pub gencursor: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
