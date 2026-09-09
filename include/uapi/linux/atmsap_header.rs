/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atmsap.h - ATM Service Access Point addressing definitions */

/* Written 1995-1999 by Werner Almesberger, EPFL LRC/ICA */

/* Dependency: <linux/atmapi.h> supplies __ATM_API_ALIGN. */

/*
 * BEGIN_xx and END_xx markers are used for automatic generation of
 * documentation. Do not change them.
 */

/* Layer 2 protocol identifiers */
pub const ATM_L2_NONE: i32 = 0;
pub const ATM_L2_ISO1745: i32 = 0x01;
pub const ATM_L2_Q291: i32 = 0x02;
pub const ATM_L2_X25_LL: i32 = 0x06;
pub const ATM_L2_X25_ML: i32 = 0x07;
pub const ATM_L2_LAPB: i32 = 0x08;
pub const ATM_L2_HDLC_ARM: i32 = 0x09;
pub const ATM_L2_HDLC_NRM: i32 = 0x0a;
pub const ATM_L2_HDLC_ABM: i32 = 0x0b;
pub const ATM_L2_ISO8802: i32 = 0x0c;
pub const ATM_L2_X75: i32 = 0x0d;
pub const ATM_L2_Q922: i32 = 0x0e;
pub const ATM_L2_USER: i32 = 0x10;
pub const ATM_L2_ISO7776: i32 = 0x11;

/* Layer 3 protocol identifiers */
pub const ATM_L3_NONE: i32 = 0;
pub const ATM_L3_X25: i32 = 0x06;
pub const ATM_L3_ISO8208: i32 = 0x07;
pub const ATM_L3_X223: i32 = 0x08;
pub const ATM_L3_ISO8473: i32 = 0x09;
pub const ATM_L3_T70: i32 = 0x0a;
pub const ATM_L3_TR9577: i32 = 0x0b;
pub const ATM_L3_H310: i32 = 0x0c;
pub const ATM_L3_H321: i32 = 0x0d;
pub const ATM_L3_USER: i32 = 0x10;

/* High layer identifiers */
pub const ATM_HL_NONE: i32 = 0;
pub const ATM_HL_ISO: i32 = 0x01;
pub const ATM_HL_USER: i32 = 0x02;
pub const ATM_HL_HLP: i32 = 0x03;
pub const ATM_HL_VENDOR: i32 = 0x04;

/* ITU-T coded mode of operation */
pub const ATM_IMD_NONE: i32 = 0;
pub const ATM_IMD_NORMAL: i32 = 1;
pub const ATM_IMD_EXTENDED: i32 = 2;

/* H.310 code points */
pub const ATM_TT_NONE: i32 = 0;
pub const ATM_TT_RX: i32 = 1;
pub const ATM_TT_TX: i32 = 2;
pub const ATM_TT_RXTX: i32 = 3;
pub const ATM_MC_NONE: i32 = 0;
pub const ATM_MC_TS: i32 = 1;
pub const ATM_MC_TS_FEC: i32 = 2;
pub const ATM_MC_PS: i32 = 3;
pub const ATM_MC_PS_FEC: i32 = 4;
pub const ATM_MC_H221: i32 = 5;

/* SAP structures */
pub const ATM_MAX_HLI: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli_l2_itu {
    pub mode: u8,
    pub window: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union atm_blli_l2 {
    pub itu: atm_blli_l2_itu,
    pub user: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli_l3_itu {
    pub mode: u8,
    pub def_size: u8,
    pub window: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli_l3_h310 {
    pub term_type: u8,
    pub fw_mpx_cap: u8,
    pub bw_mpx_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli_l3_tr9577 {
    pub ipi: u8,
    pub snap: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union atm_blli_l3 {
    pub itu: atm_blli_l3_itu,
    pub user: u8,
    pub h310: atm_blli_l3_h310,
    pub tr9577: atm_blli_l3_tr9577,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli {
    pub l2_proto: u8,
    pub l2: atm_blli_l2,
    pub l3_proto: u8,
    pub l3: atm_blli_l3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_bhli {
    pub hl_type: u8,
    pub hl_length: u8,
    pub hl_info: [u8; ATM_MAX_HLI],
}

pub const ATM_MAX_BLLI: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_sap {
    pub bhli: atm_bhli,
    pub blli: [atm_blli; ATM_MAX_BLLI],
}

#[inline]
pub fn blli_in_use(blli: atm_blli) -> i32 {
    (blli.l2_proto != 0 || blli.l3_proto != 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
