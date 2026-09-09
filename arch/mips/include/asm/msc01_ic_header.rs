/*
 * PCI Register definitions for the MIPS System Controller.
 *
 * Copyright (C) 2004 MIPS Technologies, Inc.  All rights reserved.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Register offset addresses */
pub const MSC01_IC_RST_OFS: u32 = 0x00008;
pub const MSC01_IC_ENAL_OFS: u32 = 0x00100;
pub const MSC01_IC_ENAH_OFS: u32 = 0x00108;
pub const MSC01_IC_DISL_OFS: u32 = 0x00120;
pub const MSC01_IC_DISH_OFS: u32 = 0x00128;
pub const MSC01_IC_ISBL_OFS: u32 = 0x00140;
pub const MSC01_IC_ISBH_OFS: u32 = 0x00148;
pub const MSC01_IC_ISAL_OFS: u32 = 0x00160;
pub const MSC01_IC_ISAH_OFS: u32 = 0x00168;
pub const MSC01_IC_LVL_OFS: u32 = 0x00180;
pub const MSC01_IC_RAMW_OFS: u32 = 0x00180;
pub const MSC01_IC_OSB_OFS: u32 = 0x00188;
pub const MSC01_IC_OSA_OFS: u32 = 0x00190;
pub const MSC01_IC_GENA_OFS: u32 = 0x00198;
pub const MSC01_IC_BASE_OFS: u32 = 0x001a0;
pub const MSC01_IC_VEC_OFS: u32 = 0x001b0;
pub const MSC01_IC_EOI_OFS: u32 = 0x001c0;
pub const MSC01_IC_CFG_OFS: u32 = 0x001c8;
pub const MSC01_IC_TRLD_OFS: u32 = 0x001d0;
pub const MSC01_IC_TVAL_OFS: u32 = 0x001e0;
pub const MSC01_IC_TCFG_OFS: u32 = 0x001f0;
pub const MSC01_IC_SUP_OFS: u32 = 0x00200;
pub const MSC01_IC_ENA_OFS: u32 = 0x00800;
pub const MSC01_IC_DIS_OFS: u32 = 0x00820;
pub const MSC01_IC_ISB_OFS: u32 = 0x00840;
pub const MSC01_IC_ISA_OFS: u32 = 0x00860;

/* Register field encodings */
pub const MSC01_IC_RST_RST_SHF: u32 = 0;
pub const MSC01_IC_RST_RST_MSK: u32 = 0x00000001;
pub const MSC01_IC_RST_RST_BIT: u32 = MSC01_IC_RST_RST_MSK;
pub const MSC01_IC_LVL_LVL_SHF: u32 = 0;
pub const MSC01_IC_LVL_LVL_MSK: u32 = 0x000000ff;
pub const MSC01_IC_LVL_SPUR_SHF: u32 = 16;
pub const MSC01_IC_LVL_SPUR_MSK: u32 = 0x00010000;
pub const MSC01_IC_LVL_SPUR_BIT: u32 = MSC01_IC_LVL_SPUR_MSK;
pub const MSC01_IC_RAMW_RIPL_SHF: u32 = 0;
pub const MSC01_IC_RAMW_RIPL_MSK: u32 = 0x0000003f;
pub const MSC01_IC_RAMW_DATA_SHF: u32 = 6;
pub const MSC01_IC_RAMW_DATA_MSK: u32 = 0x00000fc0;
pub const MSC01_IC_RAMW_ADDR_SHF: u32 = 25;
pub const MSC01_IC_RAMW_ADDR_MSK: u32 = 0x7e000000;
pub const MSC01_IC_RAMW_READ_SHF: u32 = 31;
pub const MSC01_IC_RAMW_READ_MSK: u32 = 0x80000000;
pub const MSC01_IC_RAMW_READ_BIT: u32 = MSC01_IC_RAMW_READ_MSK;
pub const MSC01_IC_OSB_OSB_SHF: u32 = 0;
pub const MSC01_IC_OSB_OSB_MSK: u32 = 0x000000ff;
pub const MSC01_IC_OSA_OSA_SHF: u32 = 0;
pub const MSC01_IC_OSA_OSA_MSK: u32 = 0x000000ff;
pub const MSC01_IC_GENA_GENA_SHF: u32 = 0;
pub const MSC01_IC_GENA_GENA_MSK: u32 = 0x00000001;
pub const MSC01_IC_GENA_GENA_BIT: u32 = MSC01_IC_GENA_GENA_MSK;
pub const MSC01_IC_CFG_DIS_SHF: u32 = 0;
pub const MSC01_IC_CFG_DIS_MSK: u32 = 0x00000001;
pub const MSC01_IC_CFG_DIS_BIT: u32 = MSC01_IC_CFG_DIS_MSK;
pub const MSC01_IC_CFG_SHFT_SHF: u32 = 8;
pub const MSC01_IC_CFG_SHFT_MSK: u32 = 0x00000f00;
pub const MSC01_IC_TCFG_ENA_SHF: u32 = 0;
pub const MSC01_IC_TCFG_ENA_MSK: u32 = 0x00000001;
pub const MSC01_IC_TCFG_ENA_BIT: u32 = MSC01_IC_TCFG_ENA_MSK;
pub const MSC01_IC_TCFG_INT_SHF: u32 = 8;
pub const MSC01_IC_TCFG_INT_MSK: u32 = 0x00000100;
pub const MSC01_IC_TCFG_INT_BIT: u32 = MSC01_IC_TCFG_INT_MSK;
pub const MSC01_IC_TCFG_EDGE_SHF: u32 = 16;
pub const MSC01_IC_TCFG_EDGE_MSK: u32 = 0x00010000;
pub const MSC01_IC_TCFG_EDGE_BIT: u32 = MSC01_IC_TCFG_EDGE_MSK;
pub const MSC01_IC_SUP_PRI_SHF: u32 = 0;
pub const MSC01_IC_SUP_PRI_MSK: u32 = 0x00000007;
pub const MSC01_IC_SUP_EDGE_SHF: u32 = 8;
pub const MSC01_IC_SUP_EDGE_MSK: u32 = 0x00000100;
pub const MSC01_IC_SUP_EDGE_BIT: u32 = MSC01_IC_SUP_EDGE_MSK;
pub const MSC01_IC_SUP_STEP: u32 = 8;

/* MIPS System controller interrupt register base. */
/* MSC01_IC_REG_BASE is supplied by another header/dependency. */
macro_rules! msc01_ic_abs {
    ($name:ident, $offset:ident) => {
        pub const $name: usize = MSC01_IC_REG_BASE as usize + $offset as usize;
    };
}

msc01_ic_abs!(MSC01_IC_RST, MSC01_IC_RST_OFS);
msc01_ic_abs!(MSC01_IC_ENAL, MSC01_IC_ENAL_OFS);
msc01_ic_abs!(MSC01_IC_ENAH, MSC01_IC_ENAH_OFS);
msc01_ic_abs!(MSC01_IC_DISL, MSC01_IC_DISL_OFS);
msc01_ic_abs!(MSC01_IC_DISH, MSC01_IC_DISH_OFS);
msc01_ic_abs!(MSC01_IC_ISBL, MSC01_IC_ISBL_OFS);
msc01_ic_abs!(MSC01_IC_ISBH, MSC01_IC_ISBH_OFS);
msc01_ic_abs!(MSC01_IC_ISAL, MSC01_IC_ISAL_OFS);
msc01_ic_abs!(MSC01_IC_ISAH, MSC01_IC_ISAH_OFS);
msc01_ic_abs!(MSC01_IC_LVL, MSC01_IC_LVL_OFS);
msc01_ic_abs!(MSC01_IC_RAMW, MSC01_IC_RAMW_OFS);
msc01_ic_abs!(MSC01_IC_OSB, MSC01_IC_OSB_OFS);
msc01_ic_abs!(MSC01_IC_OSA, MSC01_IC_OSA_OFS);
msc01_ic_abs!(MSC01_IC_GENA, MSC01_IC_GENA_OFS);
msc01_ic_abs!(MSC01_IC_BASE, MSC01_IC_BASE_OFS);
msc01_ic_abs!(MSC01_IC_VEC, MSC01_IC_VEC_OFS);
msc01_ic_abs!(MSC01_IC_EOI, MSC01_IC_EOI_OFS);
msc01_ic_abs!(MSC01_IC_CFG, MSC01_IC_CFG_OFS);
msc01_ic_abs!(MSC01_IC_TRLD, MSC01_IC_TRLD_OFS);
msc01_ic_abs!(MSC01_IC_TVAL, MSC01_IC_TVAL_OFS);
msc01_ic_abs!(MSC01_IC_TCFG, MSC01_IC_TCFG_OFS);
msc01_ic_abs!(MSC01_IC_SUP, MSC01_IC_SUP_OFS);
msc01_ic_abs!(MSC01_IC_ENA, MSC01_IC_ENA_OFS);
msc01_ic_abs!(MSC01_IC_DIS, MSC01_IC_DIS_OFS);
msc01_ic_abs!(MSC01_IC_ISB, MSC01_IC_ISB_OFS);
msc01_ic_abs!(MSC01_IC_ISA, MSC01_IC_ISA_OFS);

/*
 * Soc-it interrupts are configurable.
 * Every board describes its IRQ mapping with this table.
 */
#[repr(C)]
pub struct msc_irqmap {
    pub im_irq: i32,
    pub im_type: i32,
    pub im_lvl: i32,
}
pub type msc_irqmap_t = msc_irqmap;

/* im_type */
pub const MSC01_IRQ_LEVEL: i32 = 0;
pub const MSC01_IRQ_EDGE: i32 = 1;

unsafe extern "C" {
    pub fn init_msc_irqs(icubase: usize, base: u32, imp: *mut msc_irqmap_t, nirq: i32);
    pub fn ll_msc_irq();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
