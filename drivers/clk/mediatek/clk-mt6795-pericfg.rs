// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the kernel clock, reset, platform, and device-tree APIs.

static mut MT6795_PERI_CLK_LOCK: SpinLock = DEFINE_SPINLOCK!();

static PERI_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0008,
    clr_ofs: 0x0010,
    sta_ofs: 0x0018,
};

static UART_CK_SEL_PARENTS: [&'static str; 2] = ["clk26m", "uart_sel"];

static PERI_CLKS: [MtkComposite; 4] = [
    MUX!(CLK_PERI_UART0_SEL, "uart0_ck_sel", UART_CK_SEL_PARENTS, 0x40c, 0, 1),
    MUX!(CLK_PERI_UART1_SEL, "uart1_ck_sel", UART_CK_SEL_PARENTS, 0x40c, 1, 1),
    MUX!(CLK_PERI_UART2_SEL, "uart2_ck_sel", UART_CK_SEL_PARENTS, 0x40c, 2, 1),
    MUX!(CLK_PERI_UART3_SEL, "uart3_ck_sel", UART_CK_SEL_PARENTS, 0x40c, 3, 1),
];

static PERI_GATES: [MtkGate; 30] = [
    GATE_MTK!(CLK_PERI_NFI, "peri_nfi", "axi_sel", PERI_CG_REGS, 0, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_THERM, "peri_therm", "axi_sel", PERI_CG_REGS, 1, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM1, "peri_pwm1", "axi_sel", PERI_CG_REGS, 2, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM2, "peri_pwm2", "axi_sel", PERI_CG_REGS, 3, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM3, "peri_pwm3", "axi_sel", PERI_CG_REGS, 4, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM4, "peri_pwm4", "axi_sel", PERI_CG_REGS, 5, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM5, "peri_pwm5", "axi_sel", PERI_CG_REGS, 6, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM6, "peri_pwm6", "axi_sel", PERI_CG_REGS, 7, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM7, "peri_pwm7", "axi_sel", PERI_CG_REGS, 8, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_PWM, "peri_pwm", "axi_sel", PERI_CG_REGS, 9, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_USB0, "peri_usb0", "usb30_sel", PERI_CG_REGS, 10, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_USB1, "peri_usb1", "usb20_sel", PERI_CG_REGS, 11, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_AP_DMA, "peri_ap_dma", "axi_sel", PERI_CG_REGS, 12, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_MSDC30_0, "peri_msdc30_0", "msdc50_0_sel", PERI_CG_REGS, 13, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_MSDC30_1, "peri_msdc30_1", "msdc30_1_sel", PERI_CG_REGS, 14, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_MSDC30_2, "peri_msdc30_2", "msdc30_2_sel", PERI_CG_REGS, 15, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_MSDC30_3, "peri_msdc30_3", "msdc30_3_sel", PERI_CG_REGS, 16, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_NLI_ARB, "peri_nli_arb", "axi_sel", PERI_CG_REGS, 17, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_IRDA, "peri_irda", "irda_sel", PERI_CG_REGS, 18, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_UART0, "peri_uart0", "axi_sel", PERI_CG_REGS, 19, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_UART1, "peri_uart1", "axi_sel", PERI_CG_REGS, 20, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_UART2, "peri_uart2", "axi_sel", PERI_CG_REGS, 21, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_UART3, "peri_uart3", "axi_sel", PERI_CG_REGS, 22, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_I2C0, "peri_i2c0", "axi_sel", PERI_CG_REGS, 23, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_I2C1, "peri_i2c1", "axi_sel", PERI_CG_REGS, 24, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_I2C2, "peri_i2c2", "axi_sel", PERI_CG_REGS, 25, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_I2C3, "peri_i2c3", "axi_sel", PERI_CG_REGS, 26, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_I2C4, "peri_i2c4", "axi_sel", PERI_CG_REGS, 27, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_AUXADC, "peri_auxadc", "clk26m", PERI_CG_REGS, 28, MTK_CLK_GATE_OPS_SETCLR),
    GATE_MTK!(CLK_PERI_SPI0, "peri_spi0", "spi_sel", PERI_CG_REGS, 29, MTK_CLK_GATE_OPS_SETCLR),
];

static PERI_RST_OFS: [u16; 1] = [0x0];

static PERI_IDX_MAP: [u16; 3] = [
    [MT6795_PERI_NFI_SW_RST] = 14,
    [MT6795_PERI_THERM_SW_RST] = 16,
    [MT6795_PERI_MSDC1_SW_RST] = 20,
];

static CLK_RST_DESC: MtkClkRstDesc = MtkClkRstDesc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: PERI_RST_OFS,
    rst_bank_nr: ARRAY_SIZE!(PERI_RST_OFS),
    rst_idx_map: PERI_IDX_MAP,
    rst_idx_map_nr: ARRAY_SIZE!(PERI_IDX_MAP),
};

static OF_MATCH_CLK_MT6795_PERICFG: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt6795-pericfg" },
    OfDeviceId { /* sentinel */ },
];

unsafe fn clk_mt6795_pericfg_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut clk_data: *mut ClkHwOnecellData;
    let node: *mut DeviceNode = (*pdev).dev.of_node;
    let base: *mut core::ffi::c_void;
    let mut ret: i32;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(base) {
        return PTR_ERR!(base);
    }

    clk_data = mtk_alloc_clk_data(CLK_PERI_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    ret = mtk_register_reset_controller_with_dev(&mut (*pdev).dev, &CLK_RST_DESC);
    if ret != 0 {
        goto!(free_clk_data);
    }

    ret = mtk_clk_register_gates(&mut (*pdev).dev, node, PERI_GATES.as_ptr(), ARRAY_SIZE!(PERI_GATES), clk_data);
    if ret != 0 {
        goto!(free_clk_data);
    }

    ret = mtk_clk_register_composites(&mut (*pdev).dev, PERI_CLKS.as_ptr(), ARRAY_SIZE!(PERI_CLKS), base, &mut MT6795_PERI_CLK_LOCK, clk_data);
    if ret != 0 {
        goto!(unregister_gates);
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        goto!(unregister_composites);
    }

    return 0;

unregister_composites:
    mtk_clk_unregister_composites(PERI_CLKS.as_ptr(), ARRAY_SIZE!(PERI_CLKS), clk_data);
unregister_gates:
    mtk_clk_unregister_gates(PERI_GATES.as_ptr(), ARRAY_SIZE!(PERI_GATES), clk_data);
free_clk_data:
    mtk_free_clk_data(clk_data);
    ret
}

unsafe fn clk_mt6795_pericfg_remove(pdev: *mut PlatformDevice) {
    let node: *mut DeviceNode = (*pdev).dev.of_node;
    let clk_data: *mut ClkHwOnecellData = platform_get_drvdata(pdev);

    of_clk_del_provider(node);
    mtk_clk_unregister_composites(PERI_CLKS.as_ptr(), ARRAY_SIZE!(PERI_CLKS), clk_data);
    mtk_clk_unregister_gates(PERI_GATES.as_ptr(), ARRAY_SIZE!(PERI_GATES), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT6795_PERICFG_DRV: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: "clk-mt6795-pericfg",
        of_match_table: OF_MATCH_CLK_MT6795_PERICFG,
    },
    probe: clk_mt6795_pericfg_probe,
    remove: clk_mt6795_pericfg_remove,
};

module_platform_driver!(CLK_MT6795_PERICFG_DRV);

MODULE_DESCRIPTION!("MediaTek MT6795 pericfg clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
