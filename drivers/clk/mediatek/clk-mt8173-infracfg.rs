// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

macro_rules! GATE_ICG {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &infra_cg_regs, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static mut infra_clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();

static infra_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0040,
    clr_ofs: 0x0044,
    sta_ofs: 0x0048,
};

static ca53_parents: [&'static str; 4] = [
    "clk26m",
    "armca7pll",
    "mainpll",
    "univpll",
];

static ca72_parents: [&'static str; 4] = [
    "clk26m",
    "armca15pll",
    "mainpll",
    "univpll",
];

static cpu_muxes: [mtk_composite; 2] = [
    MUX!(CLK_INFRA_CA53SEL, "infra_ca53_sel", ca53_parents, 0x0000, 0, 2),
    MUX!(CLK_INFRA_CA72SEL, "infra_ca72_sel", ca72_parents, 0x0000, 2, 2),
];

static infra_early_divs: [mtk_fixed_factor; 1] = [
    FACTOR!(CLK_INFRA_CLK_13M, "clk13m", "clk26m", 1, 2),
];

static infra_gates: [mtk_gate; 11] = [
    GATE_ICG!(CLK_INFRA_DBGCLK, "infra_dbgclk", "axi_sel", 0),
    GATE_ICG!(CLK_INFRA_SMI, "infra_smi", "mm_sel", 1),
    GATE_ICG!(CLK_INFRA_AUDIO, "infra_audio", "aud_intbus_sel", 5),
    GATE_ICG!(CLK_INFRA_GCE, "infra_gce", "axi_sel", 6),
    GATE_ICG!(CLK_INFRA_L2C_SRAM, "infra_l2c_sram", "axi_sel", 7),
    GATE_ICG!(CLK_INFRA_M4U, "infra_m4u", "mem_sel", 8),
    GATE_ICG!(CLK_INFRA_CPUM, "infra_cpum", "cpum_ck", 15),
    GATE_ICG!(CLK_INFRA_KP, "infra_kp", "axi_sel", 16),
    GATE_ICG!(CLK_INFRA_CEC, "infra_cec", "clk26m", 18),
    GATE_ICG!(CLK_INFRA_PMICSPI, "infra_pmicspi", "pmicspi_sel", 22),
    GATE_ICG!(CLK_INFRA_PMICWRAP, "infra_pmicwrap", "axi_sel", 23),
];

static mut infrasys_rst_ofs: [u16; 2] = [0x30, 0x34];

static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: unsafe { &infrasys_rst_ofs },
    rst_bank_nr: 2,
};

static of_match_clk_mt8173_infracfg: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8173-infracfg" },
    of_device_id { compatible: core::primitive::str::from_utf8_unchecked(b"") },
];

unsafe fn clk_mt8173_infra_init_early(node: *mut device_node) {
    let mut i: i32;

    infra_clk_data = mtk_alloc_clk_data(CLK_INFRA_NR_CLK);
    if infra_clk_data.is_null() {
        return;
    }

    i = 0;
    while i < CLK_INFRA_NR_CLK {
        (*infra_clk_data).hws[i as usize] = ERR_PTR(-EPROBE_DEFER);
        i += 1;
    }

    mtk_clk_register_factors(
        infra_early_divs.as_ptr(),
        infra_early_divs.len(),
        infra_clk_data,
    );

    of_clk_add_hw_provider(node, of_clk_hw_onecell_get, infra_clk_data);
}

unsafe fn clk_mt8173_infracfg_probe(pdev: *mut platform_device) -> i32 {
    let node: *mut device_node = (*(*pdev).dev).of_node;
    let mut r: i32;
    let mut i: i32;

    if infra_clk_data.is_null() {
        infra_clk_data = mtk_alloc_clk_data(CLK_INFRA_NR_CLK);
        if infra_clk_data.is_null() {
            return -ENOMEM;
        }
    } else {
        i = 0;
        while i < CLK_INFRA_NR_CLK {
            if (*infra_clk_data).hws[i as usize] == ERR_PTR(-EPROBE_DEFER) {
                (*infra_clk_data).hws[i as usize] = ERR_PTR(-ENOENT);
            }
            i += 1;
        }
    }

    r = mtk_clk_register_gates(&mut (*pdev).dev, node, infra_gates.as_ptr(), infra_gates.len(), infra_clk_data);
    if r != 0 {
        return r;
    }

    r = mtk_clk_register_cpumuxes(&mut (*pdev).dev, node, cpu_muxes.as_ptr(), cpu_muxes.len(), infra_clk_data);
    if r != 0 {
        mtk_clk_unregister_gates(infra_gates.as_ptr(), infra_gates.len(), infra_clk_data);
        return r;
    }

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, infra_clk_data);
    if r != 0 {
        mtk_clk_unregister_cpumuxes(cpu_muxes.as_ptr(), cpu_muxes.len(), infra_clk_data);
        mtk_clk_unregister_gates(infra_gates.as_ptr(), infra_gates.len(), infra_clk_data);
        return r;
    }

    r = mtk_register_reset_controller_with_dev(&mut (*pdev).dev, &clk_rst_desc);
    if r != 0 {
        of_clk_del_provider(node);
        mtk_clk_unregister_cpumuxes(cpu_muxes.as_ptr(), cpu_muxes.len(), infra_clk_data);
        mtk_clk_unregister_gates(infra_gates.as_ptr(), infra_gates.len(), infra_clk_data);
        return r;
    }

    0
}

unsafe fn clk_mt8173_infracfg_remove(pdev: *mut platform_device) {
    let node: *mut device_node = (*(*pdev).dev).of_node;
    let clk_data: *mut clk_hw_onecell_data = platform_get_drvdata(pdev);

    of_clk_del_provider(node);
    mtk_clk_unregister_cpumuxes(cpu_muxes.as_ptr(), cpu_muxes.len(), clk_data);
    mtk_clk_unregister_gates(infra_gates.as_ptr(), infra_gates.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut clk_mt8173_infracfg_drv: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt8173-infracfg",
        of_match_table: of_match_clk_mt8173_infracfg.as_ptr(),
    },
    probe: clk_mt8173_infracfg_probe,
    remove: clk_mt8173_infracfg_remove,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
