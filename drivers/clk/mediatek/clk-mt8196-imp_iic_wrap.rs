// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// Dependencies supplied by the kernel clock, module, device-tree, platform,
// and local MediaTek clock headers are intentionally left external.

static IMP_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0xe08,
    clr_ofs: 0xe04,
    sta_ofs: 0xe00,
};

macro_rules! gate_imp {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &IMP_CG_REGS,
            shift: $shift,
            flags: CLK_OPS_PARENT_ENABLE,
            ops: &mtk_clk_gate_ops_setclr,
        }
    };
}

static IMPC_CLKS: [MtkGate; 4] = [
    gate_imp!(CLK_IMPC_I2C11, "impc_i2c11", "i2c_p", 0),
    gate_imp!(CLK_IMPC_I2C12, "impc_i2c12", "i2c_p", 1),
    gate_imp!(CLK_IMPC_I2C13, "impc_i2c13", "i2c_p", 2),
    gate_imp!(CLK_IMPC_I2C14, "impc_i2c14", "i2c_p", 3),
];

static IMPC_MCD: MtkClkDesc = MtkClkDesc {
    clks: IMPC_CLKS.as_ptr(),
    num_clks: IMPC_CLKS.len(),
};

static IMPE_CLKS: [MtkGate; 1] = [
    gate_imp!(CLK_IMPE_I2C5, "impe_i2c5", "i2c_east", 0),
];

static IMPE_MCD: MtkClkDesc = MtkClkDesc {
    clks: IMPE_CLKS.as_ptr(),
    num_clks: IMPE_CLKS.len(),
};

static IMPN_HWV_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0000,
    clr_ofs: 0x0004,
    sta_ofs: 0x2c00,
};

macro_rules! gate_hwv_impn {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &IMP_CG_REGS,
            hwv_regs: &IMPN_HWV_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_hwv_ops_setclr,
            flags: CLK_OPS_PARENT_ENABLE,
        }
    };
}

static IMPN_CLKS: [MtkGate; 6] = [
    gate_imp!(CLK_IMPN_I2C1, "impn_i2c1", "i2c_north", 0),
    gate_imp!(CLK_IMPN_I2C2, "impn_i2c2", "i2c_north", 1),
    gate_imp!(CLK_IMPN_I2C4, "impn_i2c4", "i2c_north", 2),
    gate_hwv_impn!(CLK_IMPN_I2C7, "impn_i2c7", "i2c_north", 3),
    gate_imp!(CLK_IMPN_I2C8, "impn_i2c8", "i2c_north", 4),
    gate_imp!(CLK_IMPN_I2C9, "impn_i2c9", "i2c_north", 5),
];

static IMPN_MCD: MtkClkDesc = MtkClkDesc {
    clks: IMPN_CLKS.as_ptr(),
    num_clks: IMPN_CLKS.len(),
};

static IMPW_CLKS: [MtkGate; 4] = [
    gate_imp!(CLK_IMPW_I2C0, "impw_i2c0", "i2c_west", 0),
    gate_imp!(CLK_IMPW_I2C3, "impw_i2c3", "i2c_west", 1),
    gate_imp!(CLK_IMPW_I2C6, "impw_i2c6", "i2c_west", 2),
    gate_imp!(CLK_IMPW_I2C10, "impw_i2c10", "i2c_west", 3),
];

static IMPW_MCD: MtkClkDesc = MtkClkDesc {
    clks: IMPW_CLKS.as_ptr(),
    num_clks: IMPW_CLKS.len(),
};

static OF_MATCH_CLK_MT8196_IMP_IIC_WRAP: [OfDeviceId; 5] = [
    OfDeviceId { compatible: "mediatek,mt8196-imp-iic-wrap-c", data: &IMPC_MCD },
    OfDeviceId { compatible: "mediatek,mt8196-imp-iic-wrap-e", data: &IMPE_MCD },
    OfDeviceId { compatible: "mediatek,mt8196-imp-iic-wrap-n", data: &IMPN_MCD },
    OfDeviceId { compatible: "mediatek,mt8196-imp-iic-wrap-w", data: &IMPW_MCD },
    OfDeviceId::sentinel(),
];

static mut CLK_MT8196_IMP_IIC_WRAP_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: Driver {
        name: "clk-mt8196-imp_iic_wrap",
        of_match_table: OF_MATCH_CLK_MT8196_IMP_IIC_WRAP.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8196_IMP_IIC_WRAP_DRV);
module_description!("MediaTek MT8196 I2C Wrapper clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
