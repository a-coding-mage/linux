// SPDX-License-Identifier: GPL-2.0-only

// The C header uses size_t supplied by its including environment.

#[repr(C)]
pub union datarec_union {
    pub xdp_pass: size_t,
    pub info: size_t,
}

#[repr(C, align(64))]
pub struct datarec {
    pub processed: size_t,
    pub dropped: size_t,
    pub issue: size_t,
    // C's anonymous union; access the members through `union_`.
    pub union_: datarec_union,
    pub xdp_drop: size_t,
    pub xdp_redirect: size_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
