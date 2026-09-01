// SPDX-License-Identifier: GPL-2.0

// Opaque translation of C's `struct device`, supplied by external headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const LPAIF_MI2S_MCLK: i32 = 1;
pub const LPAIF_MI2S_BCLK: i32 = 2;

/* Clock ID for Primary I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_PRI_MI2S_IBIT: i32 = 0x100;
/* Clock ID for Primary I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_PRI_MI2S_EBIT: i32 = 0x101;
/* Clock ID for Secondary I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_SEC_MI2S_IBIT: i32 = 0x102;
/* Clock ID for Secondary I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_SEC_MI2S_EBIT: i32 = 0x103;
/* Clock ID for Tertiary I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_TER_MI2S_IBIT: i32 = 0x104;
/* Clock ID for Tertiary I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_TER_MI2S_EBIT: i32 = 0x105;
/* Clock ID for Quartnery I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_QUAD_MI2S_IBIT: i32 = 0x106;
/* Clock ID for Quartnery I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_QUAD_MI2S_EBIT: i32 = 0x107;
/* Clock ID for Speaker I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_IBIT: i32 = 0x108;
/* Clock ID for Speaker I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_EBIT: i32 = 0x109;
/* Clock ID for Speaker I2S OSR */
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_OSR: i32 = 0x10A;

/* Clock ID for QUINARY  I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_IBIT: i32 = 0x10B;
/* Clock ID for QUINARY  I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_EBIT: i32 = 0x10C;
/* Clock ID for SENARY  I2S IBIT */
pub const Q6PRM_LPASS_CLK_ID_SEN_MI2S_IBIT: i32 = 0x10D;
/* Clock ID for SENARY  I2S EBIT */
pub const Q6PRM_LPASS_CLK_ID_SEN_MI2S_EBIT: i32 = 0x10E;
/* Clock ID for INT0 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT0_MI2S_IBIT: i32 = 0x10F;
/* Clock ID for INT1 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT1_MI2S_IBIT: i32 = 0x110;
/* Clock ID for INT2 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT2_MI2S_IBIT: i32 = 0x111;
/* Clock ID for INT3 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT3_MI2S_IBIT: i32 = 0x112;
/* Clock ID for INT4 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT4_MI2S_IBIT: i32 = 0x113;
/* Clock ID for INT5 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT5_MI2S_IBIT: i32 = 0x114;
/* Clock ID for INT6 I2S IBIT  */
pub const Q6PRM_LPASS_CLK_ID_INT6_MI2S_IBIT: i32 = 0x115;

/* Clock ID for QUINARY MI2S OSR CLK  */
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_OSR: i32 = 0x116;

/* Clock ID for MCLK1 */
pub const Q6PRM_LPASS_CLK_ID_MCLK_1: i32 = 0x300;
/* Clock ID for MCLK2 */
pub const Q6PRM_LPASS_CLK_ID_MCLK_2: i32 = 0x301;
/* Clock ID for MCLK3 */
pub const Q6PRM_LPASS_CLK_ID_MCLK_3: i32 = 0x302;
/* Clock ID for MCLK4 */
pub const Q6PRM_LPASS_CLK_ID_MCLK_4: i32 = 0x303;
/* Clock ID for MCLK5 */
pub const Q6PRM_LPASS_CLK_ID_MCLK_5: i32 = 0x304;

pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_MCLK: i32 = 0x305;
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_NPL_MCLK: i32 = 0x306;

pub const Q6PRM_LPASS_CLK_ID_VA_CORE_MCLK: i32 = 0x307;
pub const Q6PRM_LPASS_CLK_ID_VA_CORE_2X_MCLK: i32 = 0x308;

pub const Q6PRM_LPASS_CLK_ID_TX_CORE_MCLK: i32 = 0x30c;
pub const Q6PRM_LPASS_CLK_ID_TX_CORE_NPL_MCLK: i32 = 0x30d;

pub const Q6PRM_LPASS_CLK_ID_RX_CORE_MCLK: i32 = 0x30e;
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_NPL_MCLK: i32 = 0x30f;

/* Clock ID for MCLK for WSA2 core */
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_MCLK: i32 = 0x310;
/* Clock ID for NPL MCLK for WSA2 core */
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_2X_MCLK: i32 = 0x311;
/* Clock ID for RX Core TX MCLK */
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_TX_MCLK: i32 = 0x312;
/* Clock ID for RX CORE TX 2X MCLK */
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_TX_2X_MCLK: i32 = 0x313;
/* Clock ID for WSA core TX MCLK */
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_TX_MCLK: i32 = 0x314;
/* Clock ID for WSA core TX 2X MCLK */
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_TX_2X_MCLK: i32 = 0x315;
/* Clock ID for WSA2 core TX MCLK */
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_TX_MCLK: i32 = 0x316;
/* Clock ID for WSA2 core TX 2X MCLK */
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_TX_2X_MCLK: i32 = 0x317;
/* Clock ID for RX CORE MCLK2 2X  MCLK */
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_MCLK2_2X_MCLK: i32 = 0x318;

pub const Q6PRM_LPASS_CLK_ID_QAIF_IF0_IBIT: i32 = 0x500;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF0_EBIT: i32 = 0x501;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF1_IBIT: i32 = 0x502;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF1_EBIT: i32 = 0x503;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF2_IBIT: i32 = 0x504;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF2_EBIT: i32 = 0x505;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF3_IBIT: i32 = 0x506;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF3_EBIT: i32 = 0x507;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF4_IBIT: i32 = 0x508;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF4_EBIT: i32 = 0x509;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF5_IBIT: i32 = 0x50A;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF5_EBIT: i32 = 0x50B;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF6_IBIT: i32 = 0x50C;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF6_EBIT: i32 = 0x50D;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF7_IBIT: i32 = 0x50E;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF7_EBIT: i32 = 0x50F;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF8_IBIT: i32 = 0x510;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF8_EBIT: i32 = 0x511;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF9_IBIT: i32 = 0x512;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF9_EBIT: i32 = 0x513;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF10_IBIT: i32 = 0x514;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF10_EBIT: i32 = 0x515;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF11_IBIT: i32 = 0x516;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF11_EBIT: i32 = 0x517;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF12_IBIT: i32 = 0x518;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF12_EBIT: i32 = 0x519;
pub const Q6PRM_LPASS_CLK_ID_VA_QAIF_IF0_IBIT: i32 = 0x550;
pub const Q6PRM_LPASS_CLK_ID_VA_QAIF_IF0_EBIT: i32 = 0x551;

pub const Q6PRM_LPASS_CLK_SRC_INTERNAL: i32 = 1;
pub const Q6PRM_LPASS_CLK_ROOT_DEFAULT: i32 = 0;
pub const Q6PRM_HW_CORE_ID_LPASS: i32 = 1;
pub const Q6PRM_HW_CORE_ID_DCODEC: i32 = 2;
pub const Q6PRM_HW_LPR_VOTE: i32 = 3;

unsafe extern "C" {
    pub fn q6prm_set_lpass_clock(
        dev: *mut device,
        clk_id: i32,
        clk_attr: i32,
        clk_root: i32,
        freq: u32,
    ) -> i32;

    pub fn q6prm_vote_lpass_core_hw(
        dev: *mut device,
        hw_block_id: u32,
        client_name: *const core::ffi::c_char,
        client_handle: *mut u32,
    ) -> i32;

    pub fn q6prm_unvote_lpass_core_hw(
        dev: *mut device,
        hw_block_id: u32,
        client_handle: u32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
