/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 440SPe's XOR engines support header file
 *
 * 2006-2009 (C) DENX Software Engineering.
 *
 * Author: Yuri Tikhonov <yur@emcraft.com>
 */

// Dependency intent: Linux types (`u32` and `u8`) are supplied externally.

/* Number of XOR engines available on the contoller */
pub const XOR_ENGINES_NUM: u32 = 1;

/* Number of operands supported in the h/w */
pub const XOR_MAX_OPS: u32 = 16;

/*
 * XOR Command Block Control Register bits
 */
pub const XOR_CBCR_LNK_BIT: u32 = 1u32 << 31; /* link present */
pub const XOR_CBCR_TGT_BIT: u32 = 1u32 << 30; /* target present */
pub const XOR_CBCR_CBCE_BIT: u32 = 1u32 << 29; /* command block compete enable */
pub const XOR_CBCR_RNZE_BIT: u32 = 1u32 << 28; /* result not zero enable */
pub const XOR_CBCR_XNOR_BIT: u32 = 1u32 << 15; /* XOR/XNOR */
pub const XOR_CDCR_OAC_MSK: u32 = 0x7F; /* operand address count */

/*
 * XORCore Status Register bits
 */
pub const XOR_SR_XCP_BIT: u32 = 1u32 << 31; /* core processing */
pub const XOR_SR_ICB_BIT: u32 = 1u32 << 17; /* invalid CB */
pub const XOR_SR_IC_BIT: u32 = 1u32 << 16; /* invalid command */
pub const XOR_SR_IPE_BIT: u32 = 1u32 << 15; /* internal parity error */
pub const XOR_SR_RNZ_BIT: u32 = 1u32 << 2; /* result not Zero */
pub const XOR_SR_CBC_BIT: u32 = 1u32 << 1; /* CB complete */
pub const XOR_SR_CBLC_BIT: u32 = 1u32 << 0; /* CB list complete */

/*
 * XORCore Control Set and Reset Register bits
 */
pub const XOR_CRSR_XASR_BIT: u32 = 1u32 << 31; /* soft reset */
pub const XOR_CRSR_XAE_BIT: u32 = 1u32 << 30; /* enable */
pub const XOR_CRSR_RCBE_BIT: u32 = 1u32 << 29; /* refetch CB enable */
pub const XOR_CRSR_PAUS_BIT: u32 = 1u32 << 28; /* pause */
pub const XOR_CRSR_64BA_BIT: u32 = 1u32 << 27; /* 64/32 CB format */
pub const XOR_CRSR_CLP_BIT: u32 = 1u32 << 25; /* continue list processing */

/*
 * XORCore Interrupt Enable Register
 */
pub const XOR_IE_ICBIE_BIT: u32 = 1u32 << 17; /* Invalid Command Block IRQ Enable */
pub const XOR_IE_ICIE_BIT: u32 = 1u32 << 16; /* Invalid Command IRQ Enable */
pub const XOR_IE_RPTIE_BIT: u32 = 1u32 << 14; /* Read PLB Timeout Error IRQ Enable */
pub const XOR_IE_CBCIE_BIT: u32 = 1u32 << 1; /* CB complete interrupt enable */
pub const XOR_IE_CBLCI_BIT: u32 = 1u32 << 0; /* CB list complete interrupt enable */

/*
 * XOR Accelerator engine Command Block Type
 */
#[repr(C, packed)]
pub struct xor_cb {
    /* Basic 64-bit format XOR CB (Table 19-1, p.463, 440spe_um_1_22.pdf) */
    pub cbc: u32, /* control */
    pub cbbc: u32, /* byte count */
    pub cbs: u32, /* status */
    pub pad0: [u8; 4], /* reserved */
    pub cbtah: u32, /* target address high */
    pub cbtal: u32, /* target address low */
    pub cblah: u32, /* link address high */
    pub cblal: u32, /* link address low */
    pub ops: [xor_cb_ops; 16],
}

#[repr(C, packed)]
pub struct xor_cb_ops {
    pub h: u32,
    pub l: u32,
}

/*
 * XOR hardware registers Table 19-3, UM 1.22
 */
#[repr(C)]
pub struct xor_regs {
    pub op_ar: [[u32; 2]; 16], /* operand address[0]-high,[1]-low registers */
    pub pad0: [u8; 352], /* reserved */
    pub cbcr: u32, /* CB control register */
    pub cbbcr: u32, /* CB byte count register */
    pub cbsr: u32, /* CB status register */
    pub pad1: [u8; 4], /* reserved */
    pub cbtahr: u32, /* operand target address high register */
    pub cbtalr: u32, /* operand target address low register */
    pub cblahr: u32, /* CB link address high register */
    pub cblalr: u32, /* CB link address low register */
    pub crsr: u32, /* control set register */
    pub crrr: u32, /* control reset register */
    pub ccbahr: u32, /* current CB address high register */
    pub ccbalr: u32, /* current CB address low register */
    pub plbr: u32, /* PLB configuration register */
    pub ier: u32, /* interrupt enable register */
    pub pecr: u32, /* parity error count register */
    pub sr: u32, /* status register */
    pub revidr: u32, /* revision ID register */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
