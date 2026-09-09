// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// C dependencies: linux/clk-provider.h, linux/platform_device.h, clk-mtk.h,
// clk-gate.h, and dt-bindings/clock/mt8192-clk.h.

static imp_iic_wrap_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xe08,
    clr_ofs: 0xe04,
    sta_ofs: 0xe00,
};

macro_rules! gate_imp_iic_wrap {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK_FLAGS!($id, $name, $parent, &imp_iic_wrap_cg_regs, $shift,
                        &mtk_clk_gate_ops_setclr, CLK_OPS_PARENT_ENABLE)
    };
}

static imp_iic_wrap_c_clks: [mtk_gate; 4] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_C_I2C10, "imp_iic_wrap_c_i2c10", "infra_i2c0", 0),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_C_I2C11, "imp_iic_wrap_c_i2c11", "infra_i2c0", 1),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_C_I2C12, "imp_iic_wrap_c_i2c12", "infra_i2c0", 2),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_C_I2C13, "imp_iic_wrap_c_i2c13", "infra_i2c0", 3),
];

static imp_iic_wrap_e_clks: [mtk_gate; 1] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_E_I2C3, "imp_iic_wrap_e_i2c3", "infra_i2c0", 0),
];

static imp_iic_wrap_n_clks: [mtk_gate; 2] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_N_I2C0, "imp_iic_wrap_n_i2c0", "infra_i2c0", 0),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_N_I2C6, "imp_iic_wrap_n_i2c6", "infra_i2c0", 1),
];

static imp_iic_wrap_s_clks: [mtk_gate; 3] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_S_I2C7, "imp_iic_wrap_s_i2c7", "infra_i2c0", 0),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_S_I2C8, "imp_iic_wrap_s_i2c8", "infra_i2c0", 1),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_S_I2C9, "imp_iic_wrap_s_i2c9", "infra_i2c0", 2),
];

static imp_iic_wrap_w_clks: [mtk_gate; 1] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_W_I2C5, "imp_iic_wrap_w_i2c5", "infra_i2c0", 0),
];

static imp_iic_wrap_ws_clks: [mtk_gate; 3] = [
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_WS_I2C1, "imp_iic_wrap_ws_i2c1", "infra_i2c0", 0),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_WS_I2C2, "imp_iic_wrap_ws_i2c2", "infra_i2c0", 1),
    gate_imp_iic_wrap!(CLK_IMP_IIC_WRAP_WS_I2C4, "imp_iic_wrap_ws_i2c4", "infra_i2c0", 2),
];

static imp_iic_wrap_c_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_c_clks, num_clks: imp_iic_wrap_c_clks.len() };
static imp_iic_wrap_e_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_e_clks, num_clks: imp_iic_wrap_e_clks.len() };
static imp_iic_wrap_n_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_n_clks, num_clks: imp_iic_wrap_n_clks.len() };
static imp_iic_wrap_s_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_s_clks, num_clks: imp_iic_wrap_s_clks.len() };
static imp_iic_wrap_w_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_w_clks, num_clks: imp_iic_wrap_w_clks.len() };
static imp_iic_wrap_ws_desc: mtk_clk_desc = mtk_clk_desc { clks: &imp_iic_wrap_ws_clks, num_clks: imp_iic_wrap_ws_clks.len() };

static of_match_clk_mt8192_imp_iic_wrap: [of_device_id; 7] = [
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_c", data: &imp_iic_wrap_c_desc },
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_e", data: &imp_iic_wrap_e_desc },
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_n", data: &imp_iic_wrap_n_desc },
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_s", data: &imp_iic_wrap_s_desc },
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_w", data: &imp_iic_wrap_w_desc },
    of_device_id { compatible: "mediatek,mt8192-imp_iic_wrap_ws", data: &imp_iic_wrap_ws_desc },
    of_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8192_imp_iic_wrap);

static mut clk_mt8192_imp_iic_wrap_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8192-imp_iic_wrap",
        of_match_table: &of_match_clk_mt8192_imp_iic_wrap,
    },
};

module_platform_driver!(clk_mt8192_imp_iic_wrap_drv);
MODULE_DESCRIPTION!("MediaTek MT8192 I2C Wrapper clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
