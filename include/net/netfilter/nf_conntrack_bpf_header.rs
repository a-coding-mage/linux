/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/kconfig.h> and <net/netfilter/nf_conntrack.h>
// is supplied by the surrounding translation unit.

#[repr(C)]
pub struct nf_conn___init {
    pub ct: nf_conn,
}

// The original condition is:
// (IS_BUILTIN(CONFIG_NF_CONNTRACK) && IS_ENABLED(CONFIG_DEBUG_INFO_BTF)) ||
// (IS_MODULE(CONFIG_NF_CONNTRACK) && IS_ENABLED(CONFIG_DEBUG_INFO_BTF_MODULES))
// The feature below is a local Rust representation of that build-time choice.
#[cfg(feature = "nf_conntrack_bpf_external")]
extern "C" {
    pub fn register_nf_conntrack_bpf() -> ::core::ffi::c_int;
    pub fn cleanup_nf_conntrack_bpf();
}

#[cfg(not(feature = "nf_conntrack_bpf_external"))]
#[inline]
pub fn register_nf_conntrack_bpf() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "nf_conntrack_bpf_external"))]
#[inline]
pub fn cleanup_nf_conntrack_bpf() {}

// The original condition is:
// (IS_BUILTIN(CONFIG_NF_NAT) && IS_ENABLED(CONFIG_DEBUG_INFO_BTF)) ||
// (IS_MODULE(CONFIG_NF_NAT) && IS_ENABLED(CONFIG_DEBUG_INFO_BTF_MODULES))
// The feature below is a local Rust representation of that build-time choice.
#[cfg(feature = "nf_nat_bpf_external")]
extern "C" {
    pub fn register_nf_nat_bpf() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "nf_nat_bpf_external"))]
#[inline]
pub fn register_nf_nat_bpf() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
