/* SPDX-License-Identifier: GPL-2.0 */
/* AMANDA tracking. */

/* Dependencies supplied by the corresponding Linux networking headers. */

pub type nf_nat_amanda_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: core::ffi::c_uint,
    matchoff: core::ffi::c_uint,
    matchlen: core::ffi::c_uint,
    exp: *mut nf_conntrack_expect,
) -> core::ffi::c_uint;

extern "C" {
    pub static mut nf_nat_amanda_hook: *mut nf_nat_amanda_hook_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
