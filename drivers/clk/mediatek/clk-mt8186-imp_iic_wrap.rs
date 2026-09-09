// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// device-tree binding, clk-gate, and clk-mtk headers are referenced below.

static IMP_IIC_WRAP_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xe08,
    clr_ofs: 0xe04,
    sta_ofs: 0xe00,
};

macro_rules! gate_imp_iic_wrap {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &IMP_IIC_WRAP_CG_REGS, $shift,
                  &mtk_clk_gate_ops_setclr)
    };
}

static IMP_IIC_WRAP_CLKS: [mtk_gate; 10] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C0,
        "imp_iic_wrap_ap_clock_i2c0", "infra_ao_i2c_ap", 0),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C1,
        "imp_iic_wrap_ap_clock_i2c1", "infra_ao_i2c_ap", 1),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C2,
        "imp_iic_wrap_ap_clock_i2c2", "infra_ao_i2c_ap", 2),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C3,
        "imp_iic_wrap_ap_clock_i2c3", "infra_ao_i2c_ap", 3),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C4,
        "imp_iic_wrap_ap_clock_i2c4", "infra_ao_i2c_ap", 4),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C5,
        "imp_iic_wrap_ap_clock_i2c5", "infra_ao_i2c_ap", 5),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C6,
        "imp_iic_wrap_ap_clock_i2c6", "infra_ao_i2c_ap", 6),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C7,
        "imp_iic_wrap_ap_clock_i2c7", "infra_ao_i2c_ap", 7),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C8,
        "imp_iic_wrap_ap_clock_i2c8", "infra_ao_i2c_ap", 8),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_AP_CLOCK_I2C9,
        "imp_iic_wrap_ap_clock_i2c9", "infra_ao_i2c_ap", 9),
];

static IMP_IIC_WRAP_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMP_IIC_WRAP_CLKS,
    num_clks: IMP_IIC_WRAP_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_IMP_IIC_WRAP: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8186-imp_iic_wrap",
        data: &IMP_IIC_WRAP_DESC,
    },
    of_device_id {
        // sentinel
    },
];

module_device_table!(of, OF_MATCH_CLK_MT8186_IMP_IIC_WRAP);

static mut CLK_MT8186_IMP_IIC_WRAP_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: device_driver {
        name: "clk-mt8186-imp_iic_wrap",
        of_match_table: &OF_MATCH_CLK_MT8186_IMP_IIC_WRAP,
    },
};

module_platform_driver!(CLK_MT8186_IMP_IIC_WRAP_DRV);

module_description!("MediaTek MT8186 I2C Wrapper clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
