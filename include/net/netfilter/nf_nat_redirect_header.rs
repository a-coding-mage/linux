/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies corresponding to <linux/skbuff.h> and
// <uapi/linux/netfilter/nf_nat.h> are supplied externally.

use crate::{nf_nat_range2, sk_buff};

unsafe extern "C" {
    pub fn nf_nat_redirect_ipv4(
        skb: *mut sk_buff,
        range: *const nf_nat_range2,
        hooknum: u32,
    ) -> u32;

    pub fn nf_nat_redirect_ipv6(
        skb: *mut sk_buff,
        range: *const nf_nat_range2,
        hooknum: u32,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
