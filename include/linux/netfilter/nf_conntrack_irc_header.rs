/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux netfilter bindings:
// linux/netfilter.h, linux/skbuff.h, and
// net/netfilter/nf_conntrack_expect.h.

pub const IRC_PORT: u32 = 6667;

pub type nf_nat_irc_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: u32,
    matchoff: u32,
    matchlen: u32,
    exp: *mut nf_conntrack_expect,
) -> u32;

extern "C" {
    // __rcu-qualified pointer to the NAT IRC hook.
    pub static mut nf_nat_irc_hook: *mut nf_nat_irc_hook_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
