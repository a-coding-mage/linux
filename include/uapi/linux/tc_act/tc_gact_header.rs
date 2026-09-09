/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies supplied by the corresponding Linux headers. */

#[repr(C)]
pub struct tc_gact {
    pub tc_gen: tc_gen,
}

pub const PGACT_NONE: i32 = 0;
pub const PGACT_NETRAND: i32 = 1;
pub const PGACT_DETERM: i32 = 2;
pub const MAX_RAND: i32 = PGACT_DETERM + 1;

#[repr(C)]
pub struct tc_gact_p {
    pub ptype: __u16,
    pub pval: __u16,
    pub paction: i32,
}

#[repr(i32)]
pub enum TcaGact {
    TCA_GACT_UNSPEC = 0,
    TCA_GACT_TM = 1,
    TCA_GACT_PARMS = 2,
    TCA_GACT_PROB = 3,
    TCA_GACT_PAD = 4,
    __TCA_GACT_MAX = 5,
}

pub const TCA_GACT_MAX: i32 = TcaGact::__TCA_GACT_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
