/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mutex.h, net/netns/bpf.h, and uapi/linux/bpf.h.

#[allow(non_camel_case_types)]
pub enum mutex {}

#[allow(non_camel_case_types)]
pub enum bpf_attr {}

#[allow(non_camel_case_types)]
pub enum bpf_prog {}

#[allow(non_camel_case_types)]
pub enum bpf_attach_type {
    BPF_FLOW_DISSECTOR,
    BPF_SK_LOOKUP,
}

#[allow(non_camel_case_types)]
pub enum netns_bpf_attach_type {
    NETNS_BPF_FLOW_DISSECTOR,
    NETNS_BPF_SK_LOOKUP,
    NETNS_BPF_INVALID,
}

#[allow(non_camel_case_types)]
pub enum bpf_prog_type {}

#[inline]
pub unsafe fn to_netns_bpf_attach_type(
    attach_type: bpf_attach_type,
) -> netns_bpf_attach_type {
    match attach_type {
        bpf_attach_type::BPF_FLOW_DISSECTOR => {
            netns_bpf_attach_type::NETNS_BPF_FLOW_DISSECTOR
        }
        bpf_attach_type::BPF_SK_LOOKUP => netns_bpf_attach_type::NETNS_BPF_SK_LOOKUP,
    }
}

/* Protects updates to netns_bpf */
extern "C" {
    pub static mut netns_bpf_mutex: mutex;
}

// CONFIG_NET controls whether these functions are externally defined by the
// networking subsystem or replaced with -EOPNOTSUPP stubs.
#[cfg(CONFIG_NET)]
extern "C" {
    pub fn netns_bpf_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn netns_bpf_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn netns_bpf_prog_detach(attr: *const bpf_attr, ptype: bpf_prog_type) -> i32;
    pub fn netns_bpf_link_create(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
}

#[cfg(not(CONFIG_NET))]
#[inline]
pub unsafe fn netns_bpf_prog_query(_attr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 {
    -95 // -EOPNOTSUPP
}

#[cfg(not(CONFIG_NET))]
#[inline]
pub unsafe fn netns_bpf_prog_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -95 // -EOPNOTSUPP
}

#[cfg(not(CONFIG_NET))]
#[inline]
pub unsafe fn netns_bpf_prog_detach(_attr: *const bpf_attr, _ptype: bpf_prog_type) -> i32 {
    -95 // -EOPNOTSUPP
}

#[cfg(not(CONFIG_NET))]
#[inline]
pub unsafe fn netns_bpf_link_create(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -95 // -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
