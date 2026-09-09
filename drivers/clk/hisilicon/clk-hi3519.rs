// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hi3519 Clock Driver
 *
 * Copyright (c) 2015-2016 HiSilicon Technologies Co., Ltd.
 */

// C dependencies: <dt-bindings/clock/hi3519-clock.h>, <linux/clk-provider.h>,
// <linux/module.h>, <linux/platform_device.h>, "clk.h", and "reset.h".

const HI3519_INNER_CLK_OFFSET: u32 = 64;
const HI3519_FIXED_24M: u32 = 65;
const HI3519_FIXED_50M: u32 = 66;
const HI3519_FIXED_75M: u32 = 67;
const HI3519_FIXED_125M: u32 = 68;
const HI3519_FIXED_150M: u32 = 69;
const HI3519_FIXED_200M: u32 = 70;
const HI3519_FIXED_250M: u32 = 71;
const HI3519_FIXED_300M: u32 = 72;
const HI3519_FIXED_400M: u32 = 73;
const HI3519_FMC_MUX: u32 = 74;
const HI3519_NR_CLKS: u32 = 128;

#[repr(C)]
struct hi3519_crg_data {
    clk_data: *mut hisi_clock_data,
    rstc: *mut hisi_reset_controller,
}

static hi3519_fixed_rate_clks: [hisi_fixed_rate_clock; 9] = [
    hisi_fixed_rate_clock { id: HI3519_FIXED_24M, name: "24m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 24000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_50M, name: "50m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 50000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_75M, name: "75m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 75000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_125M, name: "125m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 125000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_150M, name: "150m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 150000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_200M, name: "200m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 200000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_250M, name: "250m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 250000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_300M, name: "300m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 300000000 },
    hisi_fixed_rate_clock { id: HI3519_FIXED_400M, name: "400m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 400000000 },
];

static fmc_mux_p: [&str; 8] = ["24m", "75m", "125m", "150m", "200m", "250m", "300m", "400m"];
static mut fmc_mux_table: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

static hi3519_mux_clks: [hisi_mux_clock; 1] = [
    hisi_mux_clock { id: HI3519_FMC_MUX, name: "fmc_mux", parent_names: fmc_mux_p.as_ptr(), num_parents: 8, flags: CLK_SET_RATE_PARENT, offset: 0xc0, shift: 2, width: 3, mask: 0, table: unsafe { fmc_mux_table.as_ptr() } },
];

static hi3519_gate_clks: [hisi_gate_clock; 9] = [
    hisi_gate_clock { id: HI3519_FMC_CLK, name: "clk_fmc", parent_name: "fmc_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc0, bit_idx: 1, mask: 0 },
    hisi_gate_clock { id: HI3519_UART0_CLK, name: "clk_uart0", parent_name: "24m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 20, mask: 0 },
    hisi_gate_clock { id: HI3519_UART1_CLK, name: "clk_uart1", parent_name: "24m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 21, mask: 0 },
    hisi_gate_clock { id: HI3519_UART2_CLK, name: "clk_uart2", parent_name: "24m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 22, mask: 0 },
    hisi_gate_clock { id: HI3519_UART3_CLK, name: "clk_uart3", parent_name: "24m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 23, mask: 0 },
    hisi_gate_clock { id: HI3519_UART4_CLK, name: "clk_uart4", parent_name: "24m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 24, mask: 0 },
    hisi_gate_clock { id: HI3519_SPI0_CLK, name: "clk_spi0", parent_name: "50m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 16, mask: 0 },
    hisi_gate_clock { id: HI3519_SPI1_CLK, name: "clk_spi1", parent_name: "50m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 17, mask: 0 },
    hisi_gate_clock { id: HI3519_SPI2_CLK, name: "clk_spi2", parent_name: "50m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit_idx: 18, mask: 0 },
];

unsafe fn hi3519_clk_register(pdev: *mut platform_device) -> *mut hisi_clock_data {
    let clk_data = hisi_clk_alloc(pdev, HI3519_NR_CLKS);
    if clk_data.is_null() { return ERR_PTR(-ENOMEM); }
    let mut ret = hisi_clk_register_fixed_rate(hi3519_fixed_rate_clks.as_ptr(), hi3519_fixed_rate_clks.len(), clk_data);
    if ret != 0 { return ERR_PTR(ret); }
    ret = hisi_clk_register_mux(hi3519_mux_clks.as_ptr(), hi3519_mux_clks.len(), clk_data);
    if ret != 0 { hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks.as_ptr(), hi3519_fixed_rate_clks.len(), clk_data); return ERR_PTR(ret); }
    ret = hisi_clk_register_gate(hi3519_gate_clks.as_ptr(), hi3519_gate_clks.len(), clk_data);
    if ret != 0 {
        hisi_clk_unregister_mux(hi3519_mux_clks.as_ptr(), hi3519_mux_clks.len(), clk_data);
        hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks.as_ptr(), hi3519_fixed_rate_clks.len(), clk_data);
        return ERR_PTR(ret);
    }
    ret = of_clk_add_provider((*pdev).dev.of_node, of_clk_src_onecell_get, &mut (*clk_data).clk_data);
    if ret != 0 {
        hisi_clk_unregister_gate(hi3519_gate_clks.as_ptr(), hi3519_gate_clks.len(), clk_data);
        hisi_clk_unregister_mux(hi3519_mux_clks.as_ptr(), hi3519_mux_clks.len(), clk_data);
        hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks.as_ptr(), hi3519_fixed_rate_clks.len(), clk_data);
        return ERR_PTR(ret);
    }
    clk_data
}

unsafe fn hi3519_clk_unregister(pdev: *mut platform_device) {
    let crg = platform_get_drvdata(pdev) as *mut hi3519_crg_data;
    of_clk_del_provider((*pdev).dev.of_node);
    hisi_clk_unregister_gate(hi3519_gate_clks.as_ptr(), hi3519_gate_clks.len(), (*crg).clk_data);
    hisi_clk_unregister_mux(hi3519_mux_clks.as_ptr(), hi3519_mux_clks.len(), (*crg).clk_data);
    hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks.as_ptr(), hi3519_fixed_rate_clks.len(), (*crg).clk_data);
}

unsafe fn hi3519_clk_probe(pdev: *mut platform_device) -> i32 {
    let crg = devm_kmalloc(&mut (*pdev).dev, core::mem::size_of::<hi3519_crg_data>(), GFP_KERNEL) as *mut hi3519_crg_data;
    if crg.is_null() { return -ENOMEM; }
    (*crg).rstc = hisi_reset_init(pdev);
    if (*crg).rstc.is_null() { return -ENOMEM; }
    (*crg).clk_data = hi3519_clk_register(pdev);
    if IS_ERR((*crg).clk_data) { hisi_reset_exit((*crg).rstc); return PTR_ERR((*crg).clk_data); }
    platform_set_drvdata(pdev, crg as *mut core::ffi::c_void);
    0
}

unsafe fn hi3519_clk_remove(pdev: *mut platform_device) {
    let crg = platform_get_drvdata(pdev) as *mut hi3519_crg_data;
    hisi_reset_exit((*crg).rstc);
    hi3519_clk_unregister(pdev);
}

static hi3519_clk_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "hisilicon,hi3519-crg" },
    of_device_id::default(),
];

static mut hi3519_clk_driver: platform_driver = platform_driver {
    probe: Some(hi3519_clk_probe), remove: Some(hi3519_clk_remove),
    driver: device_driver { name: "hi3519-clk", of_match_table: hi3519_clk_match_table.as_ptr() },
};

unsafe fn hi3519_clk_init() -> i32 { platform_driver_register(&mut hi3519_clk_driver) }
// core_initcall(hi3519_clk_init);
unsafe fn hi3519_clk_exit() { platform_driver_unregister(&mut hi3519_clk_driver); }
// module_exit(hi3519_clk_exit);
// MODULE_DEVICE_TABLE(of, hi3519_clk_match_table);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("HiSilicon Hi3519 Clock Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
