/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Freescale Memory Controller kernel module
 *
 * Support Power-based SoCs including MPC85xx, MPC86xx, MPC83xx and
 * ARM-based Layerscape SoCs including LS2xxx and LS1021A. Originally
 * split out from mpc85xx_edac EDAC driver.
 *
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2006-2007 (c) MontaVista Software, Inc.
 */

// The C header guard and include dependencies are intentionally omitted.

// C variadic macro equivalent; the referenced printk routine is supplied externally.
#[macro_export]
macro_rules! fsl_mc_printk {
    ($mci:expr, $level:expr, $fmt:expr $(, $arg:expr)*) => {
        edac_mc_chipset_printk($mci, $level, "FSL_DDR", $fmt $(, $arg)*)
    };
}

/* DRAM error defines */

/* DDR_SDRAM_CFG */
pub const FSL_MC_DDR_SDRAM_CFG: u32 = 0x0110;
pub const FSL_MC_CS_BNDS_0: u32 = 0x0000;
pub const FSL_MC_CS_BNDS_OFS: u32 = 0x0008;

pub const FSL_MC_DATA_ERR_INJECT_HI: u32 = 0x0e00;
pub const FSL_MC_DATA_ERR_INJECT_LO: u32 = 0x0e04;
pub const FSL_MC_ECC_ERR_INJECT: u32 = 0x0e08;
pub const FSL_MC_CAPTURE_DATA_HI: u32 = 0x0e20;
pub const FSL_MC_CAPTURE_DATA_LO: u32 = 0x0e24;
pub const FSL_MC_CAPTURE_ECC: u32 = 0x0e28;
pub const FSL_MC_ERR_DETECT: u32 = 0x0e40;
pub const FSL_MC_ERR_DISABLE: u32 = 0x0e44;
pub const FSL_MC_ERR_INT_EN: u32 = 0x0e48;
pub const FSL_MC_CAPTURE_ATRIBUTES: u32 = 0x0e4c;
pub const FSL_MC_CAPTURE_ADDRESS: u32 = 0x0e50;
pub const FSL_MC_CAPTURE_EXT_ADDRESS: u32 = 0x0e54;
pub const FSL_MC_ERR_SBE: u32 = 0x0e58;

pub const IMX9_MC_ERR_EN: u32 = 0x1000;
pub const IMX9_MC_DATA_ERR_INJECT_OFF: u32 = 0x100;

pub const DSC_MEM_EN: u32 = 0x80000000;
pub const DSC_ECC_EN: u32 = 0x20000000;
pub const DSC_RD_EN: u32 = 0x10000000;
pub const DSC_DBW_MASK: u32 = 0x00180000;
pub const DSC_DBW_32: u32 = 0x00080000;
pub const DSC_DBW_64: u32 = 0x00000000;

pub const ERR_ECC_EN: u32 = 0x80000000;
pub const ERR_INLINE_ECC: u32 = 0x40000000;

pub const DSC_SDTYPE_MASK: u32 = 0x07000000;
pub const DSC_X32_EN: u32 = 0x00000020;

/* Err_Int_En */
pub const DDR_EIE_MSEE: u32 = 0x1; // memory select
pub const DDR_EIE_SBEE: u32 = 0x4; // single-bit ECC error
pub const DDR_EIE_MBEE: u32 = 0x8; // multi-bit ECC error

/* Err_Detect */
pub const DDR_EDE_MSE: u32 = 0x1; // memory select
pub const DDR_EDE_SBE: u32 = 0x4; // single-bit ECC error
pub const DDR_EDE_MBE: u32 = 0x8; // multi-bit ECC error
pub const DDR_EDE_MME: u32 = 0x80000000; // multiple memory errors

/* Err_Disable */
pub const DDR_EDI_MSED: u32 = 0x1; // memory select disable
pub const DDR_EDI_SBED: u32 = 0x4; // single-bit ECC error disable
pub const DDR_EDI_MBED: u32 = 0x8; // multi-bit ECC error disable

pub const TYPE_IMX9: u32 = 0x1; // MC used by iMX9 having registers changed

#[repr(C)]
pub struct fsl_mc_pdata {
    pub name: *mut core::ffi::c_char,
    pub edac_idx: core::ffi::c_int,
    pub mc_vbase: *mut core::ffi::c_void,
    pub inject_vbase: *mut core::ffi::c_void,
    pub irq: core::ffi::c_int,
    pub orig_ddr_err_disable: u32,
    pub orig_ddr_err_sbe: u32,
    pub little_endian: bool,
    pub flag: core::ffi::c_ulong,
}

extern "C" {
    pub fn fsl_mc_err_probe(op: *mut platform_device) -> core::ffi::c_int;
    pub fn fsl_mc_err_remove(op: *mut platform_device);
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
