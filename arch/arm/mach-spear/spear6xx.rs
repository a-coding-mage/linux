// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear6xx/spear6xx.c
 *
 * SPEAr6XX machines common source file
 *
 * Copyright (C) 2009 ST Microelectronics
 * Rajeev Kumar<rajeev-dlh.kumar@st.com>
 *
 * Copyright 2012 Stefan Roese <sr@denx.de>
 */

// Kernel dependencies supplied by the surrounding translation unit.

/* dmac device registration */
static mut spear600_dma_info: [pl08x_channel_data; 48] = [
    pl08x_channel_data { bus_id: "ssp1_rx", min_signal: 0, max_signal: 0, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ssp1_tx", min_signal: 1, max_signal: 1, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "uart0_rx", min_signal: 2, max_signal: 2, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "uart0_tx", min_signal: 3, max_signal: 3, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "uart1_rx", min_signal: 4, max_signal: 4, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "uart1_tx", min_signal: 5, max_signal: 5, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ssp2_rx", min_signal: 6, max_signal: 6, muxval: 0, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ssp2_tx", min_signal: 7, max_signal: 7, muxval: 0, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ssp0_rx", min_signal: 8, max_signal: 8, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ssp0_tx", min_signal: 9, max_signal: 9, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "i2c_rx", min_signal: 10, max_signal: 10, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "i2c_tx", min_signal: 11, max_signal: 11, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "irda", min_signal: 12, max_signal: 12, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "adc", min_signal: 13, max_signal: 13, muxval: 0, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "to_jpeg", min_signal: 14, max_signal: 14, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "from_jpeg", min_signal: 15, max_signal: 15, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras0_rx", min_signal: 0, max_signal: 0, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras0_tx", min_signal: 1, max_signal: 1, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras1_rx", min_signal: 2, max_signal: 2, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras1_tx", min_signal: 3, max_signal: 3, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras2_rx", min_signal: 4, max_signal: 4, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras2_tx", min_signal: 5, max_signal: 5, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras3_rx", min_signal: 6, max_signal: 6, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras3_tx", min_signal: 7, max_signal: 7, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras4_rx", min_signal: 8, max_signal: 8, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras4_tx", min_signal: 9, max_signal: 9, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras5_rx", min_signal: 10, max_signal: 10, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras5_tx", min_signal: 11, max_signal: 11, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras6_rx", min_signal: 12, max_signal: 12, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras6_tx", min_signal: 13, max_signal: 13, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras7_rx", min_signal: 14, max_signal: 14, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ras7_tx", min_signal: 15, max_signal: 15, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: "ext0_rx", min_signal: 0, max_signal: 0, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext0_tx", min_signal: 1, max_signal: 1, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext1_rx", min_signal: 2, max_signal: 2, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext1_tx", min_signal: 3, max_signal: 3, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext2_rx", min_signal: 4, max_signal: 4, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext2_tx", min_signal: 5, max_signal: 5, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext3_rx", min_signal: 6, max_signal: 6, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext3_tx", min_signal: 7, max_signal: 7, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext4_rx", min_signal: 8, max_signal: 8, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext4_tx", min_signal: 9, max_signal: 9, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext5_rx", min_signal: 10, max_signal: 10, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext5_tx", min_signal: 11, max_signal: 11, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext6_rx", min_signal: 12, max_signal: 12, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext6_tx", min_signal: 13, max_signal: 13, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext7_rx", min_signal: 14, max_signal: 14, muxval: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: "ext7_tx", min_signal: 15, max_signal: 15, muxval: 2, periph_buses: PL08X_AHB2 },
];

static mut spear6xx_pl080_plat_data: pl08x_platform_data = pl08x_platform_data {
    memcpy_burst_size: PL08X_BURST_SZ_16,
    memcpy_bus_width: PL08X_BUS_WIDTH_32_BITS,
    memcpy_prot_buff: true,
    memcpy_prot_cache: true,
    lli_buses: PL08X_AHB1,
    mem_buses: PL08X_AHB1,
    get_xfer_signal: Some(pl080_get_signal),
    put_xfer_signal: Some(pl080_put_signal),
    slave_channels: unsafe { spear600_dma_info.as_mut_ptr() },
    num_slave_channels: 48,
};

/*
 * Following will create 16MB static virtual/physical mappings
 * PHYSICAL             VIRTUAL
 * 0xF0000000           0xF0000000
 * 0xF1000000           0xF1000000
 * 0xD0000000           0xFD000000
 * 0xFC000000           0xFC000000
 */
static mut spear6xx_io_desc: [map_desc; 3] = [
    map_desc { virtual_: VA_SPEAR6XX_ML_CPU_BASE as usize, pfn: __phys_to_pfn(SPEAR_ICM3_ML1_2_BASE), length: 2 * SZ_16M, type_: MT_DEVICE },
    map_desc { virtual_: VA_SPEAR_ICM1_2_BASE as usize, pfn: __phys_to_pfn(SPEAR_ICM1_2_BASE), length: SZ_16M, type_: MT_DEVICE },
    map_desc { virtual_: VA_SPEAR_ICM3_SMI_CTRL_BASE as usize, pfn: __phys_to_pfn(SPEAR_ICM3_SMI_CTRL_BASE), length: SZ_16M, type_: MT_DEVICE },
];

/* This will create static memory mapping for selected devices */
unsafe fn spear6xx_map_io() {
    iotable_init(spear6xx_io_desc.as_mut_ptr(), spear6xx_io_desc.len());
}

unsafe fn spear6xx_timer_init() {
    let pclk_name = "pll3_clk";
    let gpt_clk: *mut clk = clk_get_sys("gpt0", core::ptr::null());
    if IS_ERR(gpt_clk) {
        pr_err!("{}:couldn't get clk for gpt\n", "spear6xx_timer_init");
        BUG!();
    }
    let pclk: *mut clk = clk_get(core::ptr::null(), pclk_name);
    if IS_ERR(pclk) {
        pr_err!("{}:couldn't get {} as parent for gpt\n", "spear6xx_timer_init", pclk_name);
        BUG!();
    }
    clk_set_parent(gpt_clk, pclk);
    clk_put(gpt_clk);
    clk_put(pclk);
    spear_setup_of_timer();
}

/* Add auxdata to pass platform data */
static mut spear6xx_auxdata_lookup: [of_dev_auxdata; 2] = [
    of_dev_auxdata { compatible: "arm,pl080", phys_addr: SPEAR_ICM3_DMA_BASE, name: core::ptr::null(), platform_data: unsafe { &mut spear6xx_pl080_plat_data } },
    of_dev_auxdata::default(),
];

unsafe fn spear600_dt_init() {
    of_platform_default_populate(core::ptr::null(), spear6xx_auxdata_lookup.as_mut_ptr(), core::ptr::null_mut());
}

static spear600_dt_board_compat: [*const u8; 2] = [b"st,spear600\0".as_ptr(), core::ptr::null()];

// DT_MACHINE_START(SPEAR600_DT, "ST SPEAr600 (Flattened Device Tree)")
static SPEAR600_DT: machine_desc = machine_desc {
    map_io: Some(spear6xx_map_io),
    init_time: Some(spear6xx_timer_init),
    init_machine: Some(spear600_dt_init),
    restart: Some(spear_restart),
    dt_compat: spear600_dt_board_compat.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
