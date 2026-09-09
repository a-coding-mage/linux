// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Broadcom Corporation
 * Copyright 2014 Linaro Limited
 */

// Dependencies supplied by the surrounding clock framework and device-tree
// bindings are intentionally left external to this translation.

macro_rules! bcm21664_ccu_common {
    ($name:ident, $capname:ident) => {
        kona_ccu_common!(BCM21664, $name, $capname)
    };
}

/* Root CCU */

static mut frac_1m_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x214, 16, 0, 1),
    clocks: clocks!("ref_crystal"),
};

static mut root_ccu_data: ccu_data = ccu_data {
    bcm21664_ccu_common!(root, ROOT),
    // no policy control
    kona_clks: kona_clks! {
        [BCM21664_ROOT_CCU_FRAC_1M] => kona_clk!(root, frac_1m, peri),
        [BCM21664_ROOT_CCU_CLOCK_COUNT] => LAST_KONA_CLK,
    },
};

/* AON CCU */

static mut hub_timer_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x0414, 16, 0, 1),
    hyst: hyst!(0x0414, 8, 9),
    clocks: clocks!("bbl_32k", "frac_1m", "dft_19_5m"),
    sel: selector!(0x0a10, 0, 2),
    trig: trigger!(0x0a40, 4),
};

static mut aon_ccu_data: ccu_data = ccu_data {
    bcm21664_ccu_common!(aon, AON),
    policy: ccu_policy! {
        enable: ccu_lvm_en!(0x0034, 0),
        control: ccu_policy_ctl!(0x000c, 0, 1, 2),
    },
    kona_clks: kona_clks! {
        [BCM21664_AON_CCU_HUB_TIMER] => kona_clk!(aon, hub_timer, peri),
        [BCM21664_AON_CCU_CLOCK_COUNT] => LAST_KONA_CLK,
    },
};

/* Master CCU */

static mut sdio1_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x0358, 18, 2, 3),
    clocks: clocks!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"),
    sel: selector!(0x0a28, 0, 3),
    div: divider!(0x0a28, 4, 14),
    trig: trigger!(0x0afc, 9),
};

static mut sdio2_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x035c, 18, 2, 3),
    clocks: clocks!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"),
    sel: selector!(0x0a2c, 0, 3),
    div: divider!(0x0a2c, 4, 14),
    trig: trigger!(0x0afc, 10),
};

static mut sdio3_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x0364, 18, 2, 3),
    clocks: clocks!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"),
    sel: selector!(0x0a34, 0, 3),
    div: divider!(0x0a34, 4, 14),
    trig: trigger!(0x0afc, 12),
};

static mut sdio4_data: peri_clk_data = peri_clk_data {
    gate: hw_sw_gate!(0x0360, 18, 2, 3),
    clocks: clocks!("ref_crystal", "var_52m", "ref_52m", "var_96m", "ref_96m"),
    sel: selector!(0x0a30, 0, 3),
    div: divider!(0x0a30, 4, 14),
    trig: trigger!(0x0afc, 11),
};

static mut sdio1_sleep_data: peri_clk_data = peri_clk_data {
    clocks: clocks!("ref_32k"), // Verify
    gate: hw_sw_gate!(0x0358, 18, 2, 3),
};
static mut sdio2_sleep_data: peri_clk_data = peri_clk_data {
    clocks: clocks!("ref_32k"), // Verify
    gate: hw_sw_gate!(0x035c, 18, 2, 3),
};
static mut sdio3_sleep_data: peri_clk_data = peri_clk_data {
    clocks: clocks!("ref_32k"), // Verify
    gate: hw_sw_gate!(0x0364, 18, 2, 3),
};
static mut sdio4_sleep_data: peri_clk_data = peri_clk_data {
    clocks: clocks!("ref_32k"), // Verify
    gate: hw_sw_gate!(0x0360, 18, 2, 3),
};

static mut master_ccu_data: ccu_data = ccu_data {
    bcm21664_ccu_common!(master, MASTER),
    policy: ccu_policy! {
        enable: ccu_lvm_en!(0x0034, 0),
        control: ccu_policy_ctl!(0x000c, 0, 1, 2),
    },
    kona_clks: kona_clks! {
        [BCM21664_MASTER_CCU_SDIO1] => kona_clk!(master, sdio1, peri),
        [BCM21664_MASTER_CCU_SDIO2] => kona_clk!(master, sdio2, peri),
        [BCM21664_MASTER_CCU_SDIO3] => kona_clk!(master, sdio3, peri),
        [BCM21664_MASTER_CCU_SDIO4] => kona_clk!(master, sdio4, peri),
        [BCM21664_MASTER_CCU_SDIO1_SLEEP] => kona_clk!(master, sdio1_sleep, peri),
        [BCM21664_MASTER_CCU_SDIO2_SLEEP] => kona_clk!(master, sdio2_sleep, peri),
        [BCM21664_MASTER_CCU_SDIO3_SLEEP] => kona_clk!(master, sdio3_sleep, peri),
        [BCM21664_MASTER_CCU_SDIO4_SLEEP] => kona_clk!(master, sdio4_sleep, peri),
        [BCM21664_MASTER_CCU_CLOCK_COUNT] => LAST_KONA_CLK,
    },
};

/* Slave CCU */

static mut uartb_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0400, 18, 2, 3), clocks: clocks!("ref_crystal", "var_156m", "ref_156m"), sel: selector!(0x0a10, 0, 2), div: frac_divider!(0x0a10, 4, 12, 8), trig: trigger!(0x0afc, 2) };
static mut uartb2_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0404, 18, 2, 3), clocks: clocks!("ref_crystal", "var_156m", "ref_156m"), sel: selector!(0x0a14, 0, 2), div: frac_divider!(0x0a14, 4, 12, 8), trig: trigger!(0x0afc, 3) };
static mut uartb3_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0408, 18, 2, 3), clocks: clocks!("ref_crystal", "var_156m", "ref_156m"), sel: selector!(0x0a18, 0, 2), div: frac_divider!(0x0a18, 4, 12, 8), trig: trigger!(0x0afc, 4) };

static mut bsc1_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0458, 18, 2, 3), clocks: clocks!("ref_crystal", "var_104m", "ref_104m", "var_13m", "ref_13m"), sel: selector!(0x0a64, 0, 3), trig: trigger!(0x0afc, 23) };
static mut bsc2_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x045c, 18, 2, 3), clocks: clocks!("ref_crystal", "var_104m", "ref_104m", "var_13m", "ref_13m"), sel: selector!(0x0a68, 0, 3), trig: trigger!(0x0afc, 24) };
static mut bsc3_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0470, 18, 2, 3), clocks: clocks!("ref_crystal", "var_104m", "ref_104m", "var_13m", "ref_13m"), sel: selector!(0x0a7c, 0, 3), trig: trigger!(0x0afc, 18) };
static mut bsc4_data: peri_clk_data = peri_clk_data { gate: hw_sw_gate!(0x0474, 18, 2, 3), clocks: clocks!("ref_crystal", "var_104m", "ref_104m", "var_13m", "ref_13m"), sel: selector!(0x0a80, 0, 3), trig: trigger!(0x0afc, 19) };

static mut slave_ccu_data: ccu_data = ccu_data {
    bcm21664_ccu_common!(slave, SLAVE),
    policy: ccu_policy! { enable: ccu_lvm_en!(0x0034, 0), control: ccu_policy_ctl!(0x000c, 0, 1, 2) },
    kona_clks: kona_clks! {
        [BCM21664_SLAVE_CCU_UARTB] => kona_clk!(slave, uartb, peri),
        [BCM21664_SLAVE_CCU_UARTB2] => kona_clk!(slave, uartb2, peri),
        [BCM21664_SLAVE_CCU_UARTB3] => kona_clk!(slave, uartb3, peri),
        [BCM21664_SLAVE_CCU_BSC1] => kona_clk!(slave, bsc1, peri),
        [BCM21664_SLAVE_CCU_BSC2] => kona_clk!(slave, bsc2, peri),
        [BCM21664_SLAVE_CCU_BSC3] => kona_clk!(slave, bsc3, peri),
        [BCM21664_SLAVE_CCU_BSC4] => kona_clk!(slave, bsc4, peri),
        [BCM21664_SLAVE_CCU_CLOCK_COUNT] => LAST_KONA_CLK,
    },
};

/* Device tree match table callback functions */

unsafe fn kona_dt_root_ccu_setup(node: *mut device_node) {
    kona_dt_ccu_setup(&mut root_ccu_data, node);
}
unsafe fn kona_dt_aon_ccu_setup(node: *mut device_node) {
    kona_dt_ccu_setup(&mut aon_ccu_data, node);
}
unsafe fn kona_dt_master_ccu_setup(node: *mut device_node) {
    kona_dt_ccu_setup(&mut master_ccu_data, node);
}
unsafe fn kona_dt_slave_ccu_setup(node: *mut device_node) {
    kona_dt_ccu_setup(&mut slave_ccu_data, node);
}

clk_of_declare!(bcm21664_root_ccu, BCM21664_DT_ROOT_CCU_COMPAT, kona_dt_root_ccu_setup);
clk_of_declare!(bcm21664_aon_ccu, BCM21664_DT_AON_CCU_COMPAT, kona_dt_aon_ccu_setup);
clk_of_declare!(bcm21664_master_ccu, BCM21664_DT_MASTER_CCU_COMPAT, kona_dt_master_ccu_setup);
clk_of_declare!(bcm21664_slave_ccu, BCM21664_DT_SLAVE_CCU_COMPAT, kona_dt_slave_ccu_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
