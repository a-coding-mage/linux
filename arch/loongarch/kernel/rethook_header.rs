/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below depend on the externally defined kernel types
// `pt_regs` and `rethook_node`.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rethook_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn arch_rethook_trampoline_callback(regs: *mut pt_regs) -> usize;
    pub fn arch_rethook_prepare(
        rhn: *mut rethook_node,
        regs: *mut pt_regs,
        mcount: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
