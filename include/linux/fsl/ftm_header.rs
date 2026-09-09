// SPDX-License-Identifier: GPL-2.0

pub const FTM_SC: u32 = 0x0; // Status And Control
pub const FTM_CNT: u32 = 0x4; // Counter
pub const FTM_MOD: u32 = 0x8; // Modulo

pub const FTM_CNTIN: u32 = 0x4C; // Counter Initial Value
pub const FTM_STATUS: u32 = 0x50; // Capture And Compare Status
pub const FTM_MODE: u32 = 0x54; // Features Mode Selection
pub const FTM_SYNC: u32 = 0x58; // Synchronization
pub const FTM_OUTINIT: u32 = 0x5C; // Initial State For Channels Output
pub const FTM_OUTMASK: u32 = 0x60; // Output Mask
pub const FTM_COMBINE: u32 = 0x64; // Function For Linked Channels
pub const FTM_DEADTIME: u32 = 0x68; // Deadtime Insertion Control
pub const FTM_EXTTRIG: u32 = 0x6C; // FTM External Trigger
pub const FTM_POL: u32 = 0x70; // Channels Polarity
pub const FTM_FMS: u32 = 0x74; // Fault Mode Status
pub const FTM_FILTER: u32 = 0x78; // Input Capture Filter Control
pub const FTM_FLTCTRL: u32 = 0x7C; // Fault Control
pub const FTM_QDCTRL: u32 = 0x80; // Quadrature Decoder Control And Status
pub const FTM_CONF: u32 = 0x84; // Configuration
pub const FTM_FLTPOL: u32 = 0x88; // FTM Fault Input Polarity
pub const FTM_SYNCONF: u32 = 0x8C; // Synchronization Configuration
pub const FTM_INVCTRL: u32 = 0x90; // FTM Inverting Control
pub const FTM_SWOCTRL: u32 = 0x94; // FTM Software Output Control
pub const FTM_PWMLOAD: u32 = 0x98; // FTM PWM Load

pub const FTM_SC_CLK_MASK_SHIFT: u32 = 3;
pub const FTM_SC_CLK_MASK: u32 = 3 << FTM_SC_CLK_MASK_SHIFT;
pub const FTM_SC_TOF: u32 = 0x80;
pub const FTM_SC_TOIE: u32 = 0x40;
pub const FTM_SC_CPWMS: u32 = 0x20;
pub const FTM_SC_CLKS: u32 = 0x18;
pub const FTM_SC_PS_1: u32 = 0x0;
pub const FTM_SC_PS_2: u32 = 0x1;
pub const FTM_SC_PS_4: u32 = 0x2;
pub const FTM_SC_PS_8: u32 = 0x3;
pub const FTM_SC_PS_16: u32 = 0x4;
pub const FTM_SC_PS_32: u32 = 0x5;
pub const FTM_SC_PS_64: u32 = 0x6;
pub const FTM_SC_PS_128: u32 = 0x7;
pub const FTM_SC_PS_MASK: u32 = 0x7;

pub const FTM_MODE_FAULTIE: u32 = 0x80;
pub const FTM_MODE_FAULTM: u32 = 0x60;
pub const FTM_MODE_CAPTEST: u32 = 0x10;
pub const FTM_MODE_PWMSYNC: u32 = 0x8;
pub const FTM_MODE_WPDIS: u32 = 0x4;
pub const FTM_MODE_INIT: u32 = 0x2;
pub const FTM_MODE_FTMEN: u32 = 0x1;

/* NXP Errata: The PHAFLTREN and PHBFLTREN bits are tide to zero internally
 * and these bits cannot be set. Flextimer cannot use Filter in
 * Quadrature Decoder Mode.
 * https://community.nxp.com/thread/467648#comment-1010319
 */
pub const FTM_QDCTRL_PHAFLTREN: u32 = 0x80;
pub const FTM_QDCTRL_PHBFLTREN: u32 = 0x40;
pub const FTM_QDCTRL_PHAPOL: u32 = 0x20;
pub const FTM_QDCTRL_PHBPOL: u32 = 0x10;
pub const FTM_QDCTRL_QUADMODE: u32 = 0x8;
pub const FTM_QDCTRL_QUADDIR: u32 = 0x4;
pub const FTM_QDCTRL_TOFDIR: u32 = 0x2;
pub const FTM_QDCTRL_QUADEN: u32 = 0x1;

pub const FTM_FMS_FAULTF: u32 = 0x80;
pub const FTM_FMS_WPEN: u32 = 0x40;
pub const FTM_FMS_FAULTIN: u32 = 0x10;
pub const FTM_FMS_FAULTF3: u32 = 0x8;
pub const FTM_FMS_FAULTF2: u32 = 0x4;
pub const FTM_FMS_FAULTF1: u32 = 0x2;
pub const FTM_FMS_FAULTF0: u32 = 0x1;

pub const FTM_CSC_BASE: u32 = 0xC;
pub const FTM_CSC_MSB: u32 = 0x20;
pub const FTM_CSC_MSA: u32 = 0x10;
pub const FTM_CSC_ELSB: u32 = 0x8;
pub const FTM_CSC_ELSA: u32 = 0x4;
pub const fn FTM_CSC(channel: u32) -> u32 {
    FTM_CSC_BASE + channel * 8
}

pub const FTM_CV_BASE: u32 = 0x10;
pub const fn FTM_CV(channel: u32) -> u32 {
    FTM_CV_BASE + channel * 8
}

pub const FTM_PS_MAX: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
