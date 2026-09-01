// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Linaro Limited

// Dependencies from the original C includes:
// dt-bindings/sound/qcom,q6dsp-lpass-ports.h
// linux/err.h
// linux/init.h
// linux/clk-provider.h
// linux/module.h
// linux/device.h
// linux/platform_device.h
// q6dsp-lpass-clocks.h
// q6prm.h

use core::ffi::c_char;
use core::ptr;

macro_rules! Q6PRM_CLK {
    ($id:ident) => {
        q6dsp_clk_init {
            clk_id: $id,
            q6dsp_clk_id: concat_idents!(Q6PRM_, $id),
            name: concat!(stringify!($id), "\0").as_ptr() as *const c_char,
            rate: 19200000,
        }
    };
}

static q6prm_clks: [q6dsp_clk_init; 77] = [
    Q6PRM_CLK!(LPASS_CLK_ID_PRI_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_PRI_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SEC_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SEC_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_TER_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_TER_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QUAD_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QUAD_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SPEAKER_I2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SPEAKER_I2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SPEAKER_I2S_OSR),
    Q6PRM_CLK!(LPASS_CLK_ID_QUI_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QUI_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SEN_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_SEN_MI2S_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT0_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT1_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT2_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT3_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT4_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT5_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_INT6_MI2S_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QUI_MI2S_OSR),
    Q6PRM_CLK!(LPASS_CLK_ID_MCLK_1),
    Q6PRM_CLK!(LPASS_CLK_ID_MCLK_2),
    Q6PRM_CLK!(LPASS_CLK_ID_MCLK_3),
    Q6PRM_CLK!(LPASS_CLK_ID_MCLK_4),
    Q6PRM_CLK!(LPASS_CLK_ID_MCLK_5),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA_CORE_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA_CORE_NPL_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_VA_CORE_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_TX_CORE_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_TX_CORE_NPL_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_RX_CORE_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_RX_CORE_NPL_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_VA_CORE_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA2_CORE_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA2_CORE_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_RX_CORE_TX_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_RX_CORE_TX_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA_CORE_TX_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA_CORE_TX_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA2_CORE_TX_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_WSA2_CORE_TX_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_RX_CORE_MCLK2_2X_MCLK),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF0_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF0_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF1_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF1_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF2_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF2_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF3_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF3_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF4_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF4_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF5_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF5_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF6_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF6_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF7_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF7_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF8_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF8_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF9_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF9_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF10_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF10_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF11_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF11_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF12_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_QAIF_IF12_EBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_VA_QAIF_IF0_IBIT),
    Q6PRM_CLK!(LPASS_CLK_ID_VA_QAIF_IF0_EBIT),
    Q6DSP_VOTE_CLK!(
        LPASS_HW_MACRO_VOTE,
        Q6PRM_HW_CORE_ID_LPASS,
        "LPASS_HW_MACRO"
    ),
    Q6DSP_VOTE_CLK!(
        LPASS_HW_DCODEC_VOTE,
        Q6PRM_HW_CORE_ID_DCODEC,
        "LPASS_HW_DCODEC"
    ),
    Q6DSP_VOTE_CLK!(
        LPASS_HW_LPR_VOTE,
        Q6PRM_HW_LPR_VOTE,
        "LPASS_HW_LPR_VOTE"
    ),
];

static q6dsp_clk_q6prm: q6dsp_clk_desc = q6dsp_clk_desc {
    clks: q6prm_clks.as_ptr(),
    num_clks: q6prm_clks.len(),
    lpass_set_clk: Some(q6prm_set_lpass_clock),
    lpass_vote_clk: Some(q6prm_vote_lpass_core_hw),
    lpass_unvote_clk: Some(q6prm_unvote_lpass_core_hw),
};

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static q6prm_clock_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"qcom,q6prm-lpass-clocks\0".as_ptr() as *const c_char,
        data: ptr::addr_of!(q6dsp_clk_q6prm) as *const _,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
module_device_table!(of, q6prm_clock_device_id);

static mut q6prm_clock_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"q6prm-lpass-clock\0".as_ptr() as *const c_char,
        of_match_table: of_match_ptr!(q6prm_clock_device_id),
    },
    probe: Some(q6dsp_clock_dev_probe),
};
module_platform_driver!(q6prm_clock_platform_driver);

module_description!("Q6 Proxy Resource Manager LPASS clock driver");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
