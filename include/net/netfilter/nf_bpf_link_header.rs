/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct bpf_nf_ctx {
    pub state: *const nf_hook_state,
    pub skb: *mut sk_buff,
}

/* Build-time condition preserved from IS_ENABLED(CONFIG_NETFILTER_BPF_LINK). */
#[cfg(feature = "CONFIG_NETFILTER_BPF_LINK")]
extern "C" {
    pub fn bpf_nf_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
}

#[cfg(not(feature = "CONFIG_NETFILTER_BPF_LINK"))]
#[inline]
pub unsafe fn bpf_nf_link_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -95 // -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
