// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Broadcom Corporation
 * Copyright 2013 Linaro Limited
 */

// Dependencies supplied by the corresponding clock framework and bindings.

macro_rules! bcm281xx_ccu_common {
    ($name:ident, $ucase_name:ident) => { kona_ccu_common!(BCM281XX, $name, $ucase_name) };
}

/* Root CCU */

static mut frac_1m_data: peri_clk_data = peri_clk_data {
    gate: HW_SW_GATE!(0x214, 16, 0, 1),
    trig: TRIGGER!(0x0e04, 0),
    div: FRAC_DIVIDER!(0x0e00, 0, 22, 16),
    clocks: CLOCKS!("ref_crystal"),
};

static mut root_ccu_data: ccu_data = ccu_data {
    bcm281xx_ccu_common!(root, ROOT),
    kona_clks: [
        [BCM281XX_ROOT_CCU_FRAC_1M] = KONA_CLK!(root, frac_1m, peri),
        [BCM281XX_ROOT_CCU_CLOCK_COUNT] = LAST_KONA_CLK,
    ],
};

/* AON CCU */

static mut hub_timer_data: peri_clk_data = peri_clk_data {
    gate: HW_SW_GATE!(0x0414, 16, 0, 1),
    clocks: CLOCKS!("bbl_32k", "frac_1m", "dft_19_5m"),
    sel: SELECTOR!(0x0a10, 0, 2),
    trig: TRIGGER!(0x0a40, 4),
};

static mut pmu_bsc_data: peri_clk_data = peri_clk_data {
    gate: HW_SW_GATE!(0x0418, 16, 0, 1),
    clocks: CLOCKS!("ref_crystal", "pmu_bsc_var", "bbl_32k"),
    sel: SELECTOR!(0x0a04, 0, 2),
    div: DIVIDER!(0x0a04, 3, 4),
    trig: TRIGGER!(0x0a40, 0),
};

static mut pmu_bsc_var_data: peri_clk_data = peri_clk_data {
    clocks: CLOCKS!("var_312m", "ref_312m"),
    sel: SELECTOR!(0x0a00, 0, 2),
    div: DIVIDER!(0x0a00, 4, 5),
    trig: TRIGGER!(0x0a40, 2),
};

static mut aon_ccu_data: ccu_data = ccu_data {
    bcm281xx_ccu_common!(aon, AON),
    kona_clks: [
        [BCM281XX_AON_CCU_HUB_TIMER] = KONA_CLK!(aon, hub_timer, peri),
        [BCM281XX_AON_CCU_PMU_BSC] = KONA_CLK!(aon, pmu_bsc, peri),
        [BCM281XX_AON_CCU_PMU_BSC_VAR] = KONA_CLK!(aon, pmu_bsc_var, peri),
        [BCM281XX_AON_CCU_CLOCK_COUNT] = LAST_KONA_CLK,
    ],
};

/* Hub CCU */

static mut tmon_1m_data: peri_clk_data = peri_clk_data {
    gate: HW_SW_GATE!(0x04a4, 18, 2, 3),
    clocks: CLOCKS!("ref_crystal", "frac_1m"),
    sel: SELECTOR!(0x0e74, 0, 2),
    trig: TRIGGER!(0x0e84, 1),
};

static mut hub_ccu_data: ccu_data = ccu_data {
    bcm281xx_ccu_common!(hub, HUB),
    kona_clks: [
        [BCM281XX_HUB_CCU_TMON_1M] = KONA_CLK!(hub, tmon_1m, peri),
        [BCM281XX_HUB_CCU_CLOCK_COUNT] = LAST_KONA_CLK,
    ],
};

/* Master CCU */

static mut sdio1_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0358, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"), sel: SELECTOR!(0x0a28, 0, 3), div: DIVIDER!(0x0a28, 4, 14), trig: TRIGGER!(0x0afc, 9) };
static mut sdio2_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x035c, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"), sel: SELECTOR!(0x0a2c, 0, 3), div: DIVIDER!(0x0a2c, 4, 14), trig: TRIGGER!(0x0afc, 10) };
static mut sdio3_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0364, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"), sel: SELECTOR!(0x0a34, 0, 3), div: DIVIDER!(0x0a34, 4, 14), trig: TRIGGER!(0x0afc, 12) };
static mut sdio4_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0360, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"), sel: SELECTOR!(0x0a30, 0, 3), div: DIVIDER!(0x0a30, 4, 14), trig: TRIGGER!(0x0afc, 11) };
static mut usb_ic_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0354, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_96m", "ref_96m"), div: FIXED_DIVIDER!(2), sel: SELECTOR!(0x0a24, 0, 2), trig: TRIGGER!(0x0afc, 7) };
/* also called usbh_48m */
static mut hsic2_48m_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0370, 18, 2, 3), clocks: CLOCKS!("ref_crystal", "var_96m", "ref_96m"), sel: SELECTOR!(0x0a38, 0, 2), div: FIXED_DIVIDER!(2), trig: TRIGGER!(0x0afc, 5) };
/* also called usbh_12m */
static mut hsic2_12m_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0370, 20, 4, 5), div: DIVIDER!(0x0a38, 12, 2), clocks: CLOCKS!("ref_crystal", "var_96m", "ref_96m"), pre_div: FIXED_DIVIDER!(2), sel: SELECTOR!(0x0a38, 0, 2), trig: TRIGGER!(0x0afc, 5) };

static mut master_ccu_data: ccu_data = ccu_data {
    bcm281xx_ccu_common!(master, MASTER),
    kona_clks: [
        [BCM281XX_MASTER_CCU_SDIO1] = KONA_CLK!(master, sdio1, peri),
        [BCM281XX_MASTER_CCU_SDIO2] = KONA_CLK!(master, sdio2, peri),
        [BCM281XX_MASTER_CCU_SDIO3] = KONA_CLK!(master, sdio3, peri),
        [BCM281XX_MASTER_CCU_SDIO4] = KONA_CLK!(master, sdio4, peri),
        [BCM281XX_MASTER_CCU_USB_IC] = KONA_CLK!(master, usb_ic, peri),
        [BCM281XX_MASTER_CCU_HSIC2_48M] = KONA_CLK!(master, hsic2_48m, peri),
        [BCM281XX_MASTER_CCU_HSIC2_12M] = KONA_CLK!(master, hsic2_12m, peri),
        [BCM281XX_MASTER_CCU_CLOCK_COUNT] = LAST_KONA_CLK,
    ],
};

/* Slave CCU */

macro_rules! slave_peri { ($gate:expr, $clocks:expr, $sel:expr, $div:expr, $trig:expr) => { peri_clk_data { gate: $gate, clocks: $clocks, sel: $sel, div: $div, trig: $trig } }; }
static mut uartb_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0400,18,2,3), CLOCKS!("ref_crystal","var_156m","ref_156m"), SELECTOR!(0x0a10,0,2), FRAC_DIVIDER!(0x0a10,4,12,8), TRIGGER!(0x0afc,2));
static mut uartb2_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0404,18,2,3), CLOCKS!("ref_crystal","var_156m","ref_156m"), SELECTOR!(0x0a14,0,2), FRAC_DIVIDER!(0x0a14,4,12,8), TRIGGER!(0x0afc,3));
static mut uartb3_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0408,18,2,3), CLOCKS!("ref_crystal","var_156m","ref_156m"), SELECTOR!(0x0a18,0,2), FRAC_DIVIDER!(0x0a18,4,12,8), TRIGGER!(0x0afc,4));
static mut uartb4_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0408,18,2,3), CLOCKS!("ref_crystal","var_156m","ref_156m"), SELECTOR!(0x0a1c,0,2), FRAC_DIVIDER!(0x0a1c,4,12,8), TRIGGER!(0x0afc,5));
static mut ssp0_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0410,18,2,3), CLOCKS!("ref_crystal","var_104m","ref_104m","var_96m","ref_96m"), SELECTOR!(0x0a20,0,3), DIVIDER!(0x0a20,4,14), TRIGGER!(0x0afc,6));
static mut ssp2_data: peri_clk_data = slave_peri!(HW_SW_GATE!(0x0418,18,2,3), CLOCKS!("ref_crystal","var_104m","ref_104m","var_96m","ref_96m"), SELECTOR!(0x0a28,0,3), DIVIDER!(0x0a28,4,14), TRIGGER!(0x0afc,8));
static mut bsc1_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0458,18,2,3), clocks: CLOCKS!("ref_crystal","var_104m","ref_104m","var_13m","ref_13m"), sel: SELECTOR!(0x0a64,0,3), trig: TRIGGER!(0x0afc,23) };
static mut bsc2_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x045c,18,2,3), clocks: CLOCKS!("ref_crystal","var_104m","ref_104m","var_13m","ref_13m"), sel: SELECTOR!(0x0a68,0,3), trig: TRIGGER!(0x0afc,24) };
static mut bsc3_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0484,18,2,3), clocks: CLOCKS!("ref_crystal","var_104m","ref_104m","var_13m","ref_13m"), sel: SELECTOR!(0x0a84,0,3), trig: TRIGGER!(0x0b00,2) };
static mut pwm_data: peri_clk_data = peri_clk_data { gate: HW_SW_GATE!(0x0468,18,2,3), clocks: CLOCKS!("ref_crystal","var_104m"), sel: SELECTOR!(0x0a70,0,2), div: DIVIDER!(0x0a70,4,3), trig: TRIGGER!(0x0afc,15) };

static mut slave_ccu_data: ccu_data = ccu_data {
    bcm281xx_ccu_common!(slave, SLAVE),
    kona_clks: [
        [BCM281XX_SLAVE_CCU_UARTB] = KONA_CLK!(slave, uartb, peri),
        [BCM281XX_SLAVE_CCU_UARTB2] = KONA_CLK!(slave, uartb2, peri),
        [BCM281XX_SLAVE_CCU_UARTB3] = KONA_CLK!(slave, uartb3, peri),
        [BCM281XX_SLAVE_CCU_UARTB4] = KONA_CLK!(slave, uartb4, peri),
        [BCM281XX_SLAVE_CCU_SSP0] = KONA_CLK!(slave, ssp0, peri),
        [BCM281XX_SLAVE_CCU_SSP2] = KONA_CLK!(slave, ssp2, peri),
        [BCM281XX_SLAVE_CCU_BSC1] = KONA_CLK!(slave, bsc1, peri),
        [BCM281XX_SLAVE_CCU_BSC2] = KONA_CLK!(slave, bsc2, peri),
        [BCM281XX_SLAVE_CCU_BSC3] = KONA_CLK!(slave, bsc3, peri),
        [BCM281XX_SLAVE_CCU_PWM] = KONA_CLK!(slave, pwm, peri),
        [BCM281XX_SLAVE_CCU_CLOCK_COUNT] = LAST_KONA_CLK,
    ],
};

/* Device tree match table callback functions */

unsafe fn kona_dt_root_ccu_setup(node: *mut device_node) { kona_dt_ccu_setup(&raw mut root_ccu_data, node); }
unsafe fn kona_dt_aon_ccu_setup(node: *mut device_node) { kona_dt_ccu_setup(&raw mut aon_ccu_data, node); }
unsafe fn kona_dt_hub_ccu_setup(node: *mut device_node) { kona_dt_ccu_setup(&raw mut hub_ccu_data, node); }
unsafe fn kona_dt_master_ccu_setup(node: *mut device_node) { kona_dt_ccu_setup(&raw mut master_ccu_data, node); }
unsafe fn kona_dt_slave_ccu_setup(node: *mut device_node) { kona_dt_ccu_setup(&raw mut slave_ccu_data, node); }

CLK_OF_DECLARE!(bcm281xx_root_ccu, BCM281XX_DT_ROOT_CCU_COMPAT, kona_dt_root_ccu_setup);
CLK_OF_DECLARE!(bcm281xx_aon_ccu, BCM281XX_DT_AON_CCU_COMPAT, kona_dt_aon_ccu_setup);
CLK_OF_DECLARE!(bcm281xx_hub_ccu, BCM281XX_DT_HUB_CCU_COMPAT, kona_dt_hub_ccu_setup);
CLK_OF_DECLARE!(bcm281xx_master_ccu, BCM281XX_DT_MASTER_CCU_COMPAT, kona_dt_master_ccu_setup);
CLK_OF_DECLARE!(bcm281xx_slave_ccu, BCM281XX_DT_SLAVE_CCU_COMPAT, kona_dt_slave_ccu_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
