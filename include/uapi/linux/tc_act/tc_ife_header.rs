/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux UAPI headers:
// linux/types.h, linux/pkt_cls.h, linux/ife.h

/* Flag bits for now just encoding/decoding; mutually exclusive */
pub const IFE_ENCODE: i32 = 1;
pub const IFE_DECODE: i32 = 0;

#[repr(C)]
pub struct tc_ife {
    pub tc_gen: tc_gen,
    pub flags: __u16,
}

/*XXX: We need to encode the total number of bytes consumed */
#[repr(i32)]
pub enum tc_ife_attr {
    TCA_IFE_UNSPEC,
    TCA_IFE_PARMS,
    TCA_IFE_TM,
    TCA_IFE_DMAC,
    TCA_IFE_SMAC,
    TCA_IFE_TYPE,
    TCA_IFE_METALST,
    TCA_IFE_PAD,
    __TCA_IFE_MAX,
}

pub const TCA_IFE_MAX: i32 = __TCA_IFE_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
