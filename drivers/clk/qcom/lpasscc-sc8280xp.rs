// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022, Linaro Limited
 */

// Translated from the Linux kernel C implementation. The referenced kernel
// types, constants, and functions are supplied by other dependencies.

static LPASS_AUDIO_SWR_RX_CGCR: usize = 0;
static LPASS_AUDIO_SWR_WSA_CGCR: usize = 1;
static LPASS_AUDIO_SWR_WSA2_CGCR: usize = 2;
static LPASS_AUDIO_SWR_TX_CGCR: usize = 0;

static lpass_audiocc_sc8280xp_resets: [qcom_reset_map; 3] = [
    qcom_reset_map { reg: 0xa0, bit: 1 },
    qcom_reset_map { reg: 0xb0, bit: 1 },
    qcom_reset_map { reg: 0xd8, bit: 1 },
];

static lpass_audiocc_sc8280xp_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    name: "lpass-audio-csr",
    max_register: 0x1000,
};

static lpass_audiocc_sc8280xp_reset_desc: qcom_cc_desc = qcom_cc_desc {
    config: &lpass_audiocc_sc8280xp_regmap_config,
    resets: &lpass_audiocc_sc8280xp_resets,
    num_resets: lpass_audiocc_sc8280xp_resets.len(),
};

static lpasscc_sc8280xp_resets: [qcom_reset_map; 1] = [
    qcom_reset_map { reg: 0xc010, bit: 1 },
];

static lpasscc_sc8280xp_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    name: "lpass-tcsr",
    max_register: 0x12000,
};

static lpasscc_sc8280xp_reset_desc: qcom_cc_desc = qcom_cc_desc {
    config: &lpasscc_sc8280xp_regmap_config,
    resets: &lpasscc_sc8280xp_resets,
    num_resets: lpasscc_sc8280xp_resets.len(),
};

static lpasscc_sc8280xp_match_table: [of_device_id; 3] = [
    of_device_id {
        compatible: "qcom,sc8280xp-lpassaudiocc",
        data: &lpass_audiocc_sc8280xp_reset_desc,
    },
    of_device_id {
        compatible: "qcom,sc8280xp-lpasscc",
        data: &lpasscc_sc8280xp_reset_desc,
    },
    of_device_id { ..Default::default() },
];

unsafe fn lpasscc_sc8280xp_probe(pdev: *mut platform_device) -> i32 {
    let desc: *const qcom_cc_desc =
        of_device_get_match_data(&mut (*pdev).dev);

    qcom_cc_probe_by_index(pdev, 0, desc)
}

static mut lpasscc_sc8280xp_driver: platform_driver = platform_driver {
    probe: Some(lpasscc_sc8280xp_probe),
    driver: driver {
        name: "lpasscc-sc8280xp",
        of_match_table: &lpasscc_sc8280xp_match_table,
    },
};

// module_platform_driver(lpasscc_sc8280xp_driver);

// MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org>");
// MODULE_DESCRIPTION("QTI LPASSCC SC8280XP Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
