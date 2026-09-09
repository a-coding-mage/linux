// Translated from nf_conntrack_bridge.h.
//
// C header guard: NF_CONNTRACK_BRIDGE_
//
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this file.

#[repr(C)]
pub struct nf_hook_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_ct_bridge_info {
    pub ops: *mut nf_hook_ops,
    pub ops_size: ::core::ffi::c_uint,
    pub me: *mut module,
}

extern "C" {
    pub fn nf_ct_bridge_register(info: *mut nf_ct_bridge_info);
    pub fn nf_ct_bridge_unregister(info: *mut nf_ct_bridge_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
