/* Translation of cvmx-asxx-defs.h.  C bit-fields are represented by their
 * containing 64-bit storage words; field widths and ordering are retained in
 * the declarations below. */

pub unsafe fn __cvmx_interrupt_asxx_enable(block: i32);

#[inline]
pub unsafe fn CVMX_ASXX_GMII_RX_CLK_SET(_block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000180u64) }
#[inline]
pub unsafe fn CVMX_ASXX_GMII_RX_DAT_SET(_block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000188u64) }
#[inline]
pub unsafe fn CVMX_ASXX_INT_EN(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000018u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_INT_REG(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000010u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_MII_RX_DAT_SET(_block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000190u64) }
#[inline]
pub unsafe fn CVMX_ASXX_PRT_LOOP(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000040u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_BYPASS(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000248u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_BYPASS_SETTING(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000250u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_COMP(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000220u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_DATA_DRV(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000218u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_FCRAM_MODE(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000210u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_NCTL_STRONG(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000230u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_NCTL_WEAK(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000240u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_PCTL_STRONG(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000228u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_PCTL_WEAK(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000238u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RLD_SETTING(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000258u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RX_CLK_SETX(offset: u64, block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000020u64) + (((offset & 3) + (block_id & 1) * 0x1000000u64) * 8) }
#[inline]
pub unsafe fn CVMX_ASXX_RX_PRT_EN(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000000u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RX_WOL(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000100u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RX_WOL_MSK(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000108u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RX_WOL_POWOK(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000118u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_RX_WOL_SIG(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000110u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_TX_CLK_SETX(offset: u64, block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000048u64) + (((offset & 3) + (block_id & 1) * 0x1000000u64) * 8) }
#[inline]
pub unsafe fn CVMX_ASXX_TX_COMP_BYP(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000068u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub unsafe fn CVMX_ASXX_TX_HI_WATERX(offset: u64, block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000080u64) + (((offset & 3) + (block_id & 1) * 0x1000000u64) * 8) }
#[inline]
pub unsafe fn CVMX_ASXX_TX_PRT_EN(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800B0000008u64) + (block_id & 1) * 0x8000000u64 }

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_gmii_rx_clk_set_s { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_gmii_rx_dat_set_s { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_mii_rx_dat_set_s { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rx_clk_setx_s { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_clk_setx_s { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_bypass_s { pub bypass: u64, pub reserved_1_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_fcram_mode_s { pub mode: u64, pub reserved_1_63: u64 }

macro_rules! one_layout { ($name:ident, $($field:ident),+ $(,)?) => {
    #[repr(C)] #[derive(Copy, Clone)] pub struct $name { $(pub $field: u64),+ }
}; }
one_layout!(cvmx_asxx_rld_bypass_setting_s, setting, reserved_5_63);
one_layout!(cvmx_asxx_rld_nctl_strong_s, nctl, reserved_5_63);
one_layout!(cvmx_asxx_rld_nctl_weak_s, nctl, reserved_5_63);
one_layout!(cvmx_asxx_rld_pctl_strong_s, pctl, reserved_5_63);
one_layout!(cvmx_asxx_rld_pctl_weak_s, pctl, reserved_5_63);
one_layout!(cvmx_asxx_rld_data_drv_s, nctl, pctl, reserved_8_63);
one_layout!(cvmx_asxx_rx_prt_en_s, prt_en, reserved_4_63);
one_layout!(cvmx_asxx_tx_hi_waterx_s, mark, reserved_4_63);
one_layout!(cvmx_asxx_tx_prt_en_s, prt_en, reserved_4_63);
one_layout!(cvmx_asxx_rx_wol_s, enable, status, reserved_2_63);
one_layout!(cvmx_asxx_rx_wol_msk_s, msk);
one_layout!(cvmx_asxx_rx_wol_powok_s, powerok, reserved_1_63);
one_layout!(cvmx_asxx_rx_wol_sig_s, sig, reserved_32_63);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_int_en_s { pub ovrflw: u64, pub txpop: u64, pub txpsh: u64, pub reserved_12_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_int_en_cn30xx { pub ovrflw: u64, pub reserved_3_3: u64, pub txpop: u64, pub reserved_7_7: u64, pub txpsh: u64, pub reserved_11_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_int_reg_s { pub ovrflw: u64, pub txpop: u64, pub txpsh: u64, pub reserved_12_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_int_reg_cn30xx { pub ovrflw: u64, pub reserved_3_3: u64, pub txpop: u64, pub reserved_7_7: u64, pub txpsh: u64, pub reserved_11_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_prt_loop_s { pub int_loop: u64, pub ext_loop: u64, pub reserved_8_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_prt_loop_cn30xx { pub int_loop: u64, pub reserved_3_3: u64, pub ext_loop: u64, pub reserved_7_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_comp_s { pub nctl: u64, pub pctl: u64, pub reserved_9_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_comp_cn38xx { pub nctl: u64, pub pctl: u64, pub reserved_8_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_setting_s { pub setting: u64, pub dfalock: u64, pub dfalead: u64, pub dfalag: u64, pub dfaset: u64, pub reserved_13_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rld_setting_cn38xx { pub setting: u64, pub reserved_5_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_rx_prt_en_cn30xx { pub prt_en: u64, pub reserved_3_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_hi_waterx_cn30xx { pub mark: u64, pub reserved_3_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_prt_en_cn30xx { pub prt_en: u64, pub reserved_3_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_comp_byp_s { pub reserved_0_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_comp_byp_cn30xx { pub nctl: u64, pub pctl: u64, pub bypass: u64, pub reserved_9_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_comp_byp_cn38xx { pub nctl: u64, pub pctl: u64, pub reserved_8_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_comp_byp_cn50xx { pub nctl: u64, pub reserved_5_7: u64, pub pctl: u64, pub reserved_13_15: u64, pub bypass: u64, pub reserved_17_63: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_asxx_tx_comp_byp_cn58xx { pub nctl: u64, pub reserved_5_7: u64, pub pctl: u64, pub reserved_13_63: u64 }

macro_rules! asxx_union { ($name:ident; $($field:ident : $ty:ty),+ $(,)?) => {
    #[repr(C)] pub union $name { $(pub $field: $ty),+ }
}; }
asxx_union!(cvmx_asxx_gmii_rx_clk_set; u64: u64, s: cvmx_asxx_gmii_rx_clk_set_s);
asxx_union!(cvmx_asxx_gmii_rx_dat_set; u64: u64, s: cvmx_asxx_gmii_rx_dat_set_s);
asxx_union!(cvmx_asxx_int_en; u64: u64, s: cvmx_asxx_int_en_s, cn30xx: cvmx_asxx_int_en_cn30xx);
asxx_union!(cvmx_asxx_int_reg; u64: u64, s: cvmx_asxx_int_reg_s, cn30xx: cvmx_asxx_int_reg_cn30xx);
asxx_union!(cvmx_asxx_mii_rx_dat_set; u64: u64, s: cvmx_asxx_mii_rx_dat_set_s);
asxx_union!(cvmx_asxx_prt_loop; u64: u64, s: cvmx_asxx_prt_loop_s, cn30xx: cvmx_asxx_prt_loop_cn30xx);
asxx_union!(cvmx_asxx_rld_bypass; u64: u64, s: cvmx_asxx_rld_bypass_s);
asxx_union!(cvmx_asxx_rld_bypass_setting; u64: u64, s: cvmx_asxx_rld_bypass_setting_s);
asxx_union!(cvmx_asxx_rld_comp; u64: u64, s: cvmx_asxx_rld_comp_s, cn38xx: cvmx_asxx_rld_comp_cn38xx);
asxx_union!(cvmx_asxx_rld_data_drv; u64: u64, s: cvmx_asxx_rld_data_drv_s);
asxx_union!(cvmx_asxx_rld_fcram_mode; u64: u64, s: cvmx_asxx_rld_fcram_mode_s);
asxx_union!(cvmx_asxx_rld_nctl_strong; u64: u64, s: cvmx_asxx_rld_nctl_strong_s);
asxx_union!(cvmx_asxx_rld_nctl_weak; u64: u64, s: cvmx_asxx_rld_nctl_weak_s);
asxx_union!(cvmx_asxx_rld_pctl_strong; u64: u64, s: cvmx_asxx_rld_pctl_strong_s);
asxx_union!(cvmx_asxx_rld_pctl_weak; u64: u64, s: cvmx_asxx_rld_pctl_weak_s);
asxx_union!(cvmx_asxx_rld_setting; u64: u64, s: cvmx_asxx_rld_setting_s, cn38xx: cvmx_asxx_rld_setting_cn38xx);
asxx_union!(cvmx_asxx_rx_clk_setx; u64: u64, s: cvmx_asxx_rx_clk_setx_s);
asxx_union!(cvmx_asxx_rx_prt_en; u64: u64, s: cvmx_asxx_rx_prt_en_s, cn30xx: cvmx_asxx_rx_prt_en_cn30xx);
asxx_union!(cvmx_asxx_rx_wol; u64: u64, s: cvmx_asxx_rx_wol_s);
asxx_union!(cvmx_asxx_rx_wol_msk; u64: u64, s: cvmx_asxx_rx_wol_msk_s);
asxx_union!(cvmx_asxx_rx_wol_powok; u64: u64, s: cvmx_asxx_rx_wol_powok_s);
asxx_union!(cvmx_asxx_rx_wol_sig; u64: u64, s: cvmx_asxx_rx_wol_sig_s);
asxx_union!(cvmx_asxx_tx_clk_setx; u64: u64, s: cvmx_asxx_tx_clk_setx_s);
asxx_union!(cvmx_asxx_tx_comp_byp; u64: u64, s: cvmx_asxx_tx_comp_byp_s, cn30xx: cvmx_asxx_tx_comp_byp_cn30xx, cn38xx: cvmx_asxx_tx_comp_byp_cn38xx, cn50xx: cvmx_asxx_tx_comp_byp_cn50xx, cn58xx: cvmx_asxx_tx_comp_byp_cn58xx);
asxx_union!(cvmx_asxx_tx_hi_waterx; u64: u64, s: cvmx_asxx_tx_hi_waterx_s, cn30xx: cvmx_asxx_tx_hi_waterx_cn30xx);
asxx_union!(cvmx_asxx_tx_prt_en; u64: u64, s: cvmx_asxx_tx_prt_en_s, cn30xx: cvmx_asxx_tx_prt_en_cn30xx);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
