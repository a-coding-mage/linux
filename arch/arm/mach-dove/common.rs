// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-dove/common.c
 *
 * Core functions for Marvell Dove 88AP510 System On Chip
 */

// Kernel and platform dependencies supplied by other translation units.

/* These can go away once Dove uses the mvebu-mbus DT binding */
const DOVE_MBUS_PCIE0_MEM_TARGET: u32 = 0x4;
const DOVE_MBUS_PCIE0_MEM_ATTR: u32 = 0xe8;
const DOVE_MBUS_PCIE0_IO_TARGET: u32 = 0x4;
const DOVE_MBUS_PCIE0_IO_ATTR: u32 = 0xe0;
const DOVE_MBUS_PCIE1_MEM_TARGET: u32 = 0x8;
const DOVE_MBUS_PCIE1_MEM_ATTR: u32 = 0xe8;
const DOVE_MBUS_PCIE1_IO_TARGET: u32 = 0x8;
const DOVE_MBUS_PCIE1_IO_ATTR: u32 = 0xe0;
const DOVE_MBUS_CESA_TARGET: u32 = 0x3;
const DOVE_MBUS_CESA_ATTR: u32 = 0x1;
const DOVE_MBUS_BOOTROM_TARGET: u32 = 0x1;
const DOVE_MBUS_BOOTROM_ATTR: u32 = 0xfd;
const DOVE_MBUS_SCRATCHPAD_TARGET: u32 = 0xd;
const DOVE_MBUS_SCRATCHPAD_ATTR: u32 = 0x0;

/* I/O Address Mapping */
static mut dove_io_desc: [map_desc; 2] = [
    map_desc { virtual_: DOVE_SB_REGS_VIRT_BASE as usize, pfn: __phys_to_pfn(DOVE_SB_REGS_PHYS_BASE), length: DOVE_SB_REGS_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: DOVE_NB_REGS_VIRT_BASE as usize, pfn: __phys_to_pfn(DOVE_NB_REGS_PHYS_BASE), length: DOVE_NB_REGS_SIZE, type_: MT_DEVICE },
];

pub unsafe fn dove_map_io() {
    iotable_init(dove_io_desc.as_mut_ptr(), dove_io_desc.len());
}

/* CLK tree */
static mut dove_tclk: i32 = 0;
static mut gating_lock: spinlock_t = spinlock_t { _private: [] };
static mut tclk: *mut clk = core::ptr::null_mut();

unsafe fn dove_register_gate(name: *const i8, parent: *const i8, bit_idx: u8) -> *mut clk {
    clk_register_gate(core::ptr::null_mut(), name, parent, 0,
        CLOCK_GATING_CONTROL as *mut core::ffi::c_void, bit_idx, 0, &mut gating_lock)
}

unsafe fn dove_clk_init() {
    let mut usb0: *mut clk; let mut usb1: *mut clk; let mut sata: *mut clk;
    let mut pex0: *mut clk; let mut pex1: *mut clk; let mut sdio0: *mut clk; let mut sdio1: *mut clk;
    let mut nand: *mut clk; let mut camera: *mut clk; let mut i2s0: *mut clk; let mut i2s1: *mut clk;
    let mut crypto: *mut clk; let mut ac97: *mut clk; let mut pdma: *mut clk;
    let mut xor0: *mut clk; let mut xor1: *mut clk; let mut ge: *mut clk;
    tclk = clk_register_fixed_rate(core::ptr::null_mut(), c"tclk".as_ptr(), core::ptr::null(), 0, dove_tclk as u64);
    usb0 = dove_register_gate(c"usb0".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_USB0);
    usb1 = dove_register_gate(c"usb1".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_USB1);
    sata = dove_register_gate(c"sata".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_SATA);
    pex0 = dove_register_gate(c"pex0".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_PCIE0);
    pex1 = dove_register_gate(c"pex1".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_PCIE1);
    sdio0 = dove_register_gate(c"sdio0".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_SDIO0);
    sdio1 = dove_register_gate(c"sdio1".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_SDIO1);
    nand = dove_register_gate(c"nand".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_NAND);
    camera = dove_register_gate(c"camera".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_CAMERA);
    i2s0 = dove_register_gate(c"i2s0".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_I2S0);
    i2s1 = dove_register_gate(c"i2s1".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_I2S1);
    crypto = dove_register_gate(c"crypto".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_CRYPTO);
    ac97 = dove_register_gate(c"ac97".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_AC97);
    pdma = dove_register_gate(c"pdma".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_PDMA);
    xor0 = dove_register_gate(c"xor0".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_XOR0);
    xor1 = dove_register_gate(c"xor1".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_XOR1);
    dove_register_gate(c"gephy".as_ptr(), c"tclk".as_ptr(), CLOCK_GATING_BIT_GIGA_PHY);
    ge = dove_register_gate(c"ge".as_ptr(), c"gephy".as_ptr(), CLOCK_GATING_BIT_GBE);
    orion_clkdev_add(core::ptr::null(), c"orion_spi.0".as_ptr(), tclk); orion_clkdev_add(core::ptr::null(), c"orion_spi.1".as_ptr(), tclk);
    orion_clkdev_add(core::ptr::null(), c"orion_wdt".as_ptr(), tclk); orion_clkdev_add(core::ptr::null(), c"mv64xxx_i2c.0".as_ptr(), tclk);
    orion_clkdev_add(core::ptr::null(), c"orion-ehci.0".as_ptr(), usb0); orion_clkdev_add(core::ptr::null(), c"orion-ehci.1".as_ptr(), usb1);
    orion_clkdev_add(core::ptr::null(), c"mv643xx_eth_port.0".as_ptr(), ge); orion_clkdev_add(core::ptr::null(), c"sata_mv.0".as_ptr(), sata);
    orion_clkdev_add(c"0".as_ptr(), c"pcie".as_ptr(), pex0); orion_clkdev_add(c"1".as_ptr(), c"pcie".as_ptr(), pex1);
    orion_clkdev_add(core::ptr::null(), c"sdhci-dove.0".as_ptr(), sdio0); orion_clkdev_add(core::ptr::null(), c"sdhci-dove.1".as_ptr(), sdio1);
    orion_clkdev_add(core::ptr::null(), c"orion_nand".as_ptr(), nand); orion_clkdev_add(core::ptr::null(), c"cafe1000-ccic.0".as_ptr(), camera);
    orion_clkdev_add(core::ptr::null(), c"mvebu-audio.0".as_ptr(), i2s0); orion_clkdev_add(core::ptr::null(), c"mvebu-audio.1".as_ptr(), i2s1);
    orion_clkdev_add(core::ptr::null(), c"mv_crypto".as_ptr(), crypto); orion_clkdev_add(core::ptr::null(), c"dove-ac97".as_ptr(), ac97);
    orion_clkdev_add(core::ptr::null(), c"dove-pdma".as_ptr(), pdma); orion_clkdev_add(core::ptr::null(), concatcp!(MV_XOR_NAME, ".0").as_ptr(), xor0);
    orion_clkdev_add(core::ptr::null(), concatcp!(MV_XOR_NAME, ".1").as_ptr(), xor1);
}

pub unsafe fn dove_ehci0_init() { orion_ehci_init(DOVE_USB0_PHYS_BASE, IRQ_DOVE_USB0, EHCI_PHY_NA); }
pub unsafe fn dove_ehci1_init() { orion_ehci_1_init(DOVE_USB1_PHYS_BASE, IRQ_DOVE_USB1); }
pub unsafe fn dove_ge00_init(eth_data: *mut mv643xx_eth_platform_data) { orion_ge00_init(eth_data, DOVE_GE00_PHYS_BASE, IRQ_DOVE_GE00_SUM, IRQ_DOVE_GE00_ERR, 1600); }
unsafe fn dove_rtc_init() { orion_rtc_init(DOVE_RTC_PHYS_BASE, IRQ_DOVE_RTC); }
pub unsafe fn dove_sata_init(sata_data: *mut mv_sata_platform_data) { orion_sata_init(sata_data, DOVE_SATA_PHYS_BASE, IRQ_DOVE_SATA); }
pub unsafe fn dove_uart0_init() { orion_uart0_init(DOVE_UART0_VIRT_BASE, DOVE_UART0_PHYS_BASE, IRQ_DOVE_UART_0, tclk); }
pub unsafe fn dove_uart1_init() { orion_uart1_init(DOVE_UART1_VIRT_BASE, DOVE_UART1_PHYS_BASE, IRQ_DOVE_UART_1, tclk); }
pub unsafe fn dove_uart2_init() { orion_uart2_init(DOVE_UART2_VIRT_BASE, DOVE_UART2_PHYS_BASE, IRQ_DOVE_UART_2, tclk); }
pub unsafe fn dove_uart3_init() { orion_uart3_init(DOVE_UART3_VIRT_BASE, DOVE_UART3_PHYS_BASE, IRQ_DOVE_UART_3, tclk); }
pub unsafe fn dove_spi0_init() { orion_spi_init(DOVE_SPI0_PHYS_BASE); }
pub unsafe fn dove_spi1_init() { orion_spi_1_init(DOVE_SPI1_PHYS_BASE); }
pub unsafe fn dove_i2c_init() { orion_i2c_init(DOVE_I2C_PHYS_BASE, IRQ_DOVE_I2C, 10); }

pub unsafe fn dove_init_early() { orion_time_set_base(TIMER_VIRT_BASE); mvebu_mbus_init(c"marvell,dove-mbus".as_ptr(), BRIDGE_WINS_BASE, BRIDGE_WINS_SZ, DOVE_MC_WINS_BASE, DOVE_MC_WINS_SZ); }
unsafe fn dove_find_tclk() -> i32 { 166666667 }
pub unsafe fn dove_timer_init() { dove_tclk = dove_find_tclk(); orion_time_init(BRIDGE_VIRT_BASE, BRIDGE_INT_TIMER1_CLR, IRQ_DOVE_BRIDGE, dove_tclk); }
unsafe fn dove_xor0_init() { orion_xor0_init(DOVE_XOR0_PHYS_BASE, DOVE_XOR0_HIGH_PHYS_BASE, IRQ_DOVE_XOR_00, IRQ_DOVE_XOR_01); }
unsafe fn dove_xor1_init() { orion_xor1_init(DOVE_XOR1_PHYS_BASE, DOVE_XOR1_HIGH_PHYS_BASE, IRQ_DOVE_XOR_10, IRQ_DOVE_XOR_11); }

static mut sdio_dmamask: u64 = 0xffff_ffff;
static mut dove_sdio0: platform_device = platform_device { name: c"sdhci-dove".as_ptr(), id: 0, dev: device { dma_mask: &mut sdio_dmamask, coherent_dma_mask: 0xffff_ffff }, resource: core::ptr::null_mut(), num_resources: 0 };
static mut dove_sdio1: platform_device = platform_device { name: c"sdhci-dove".as_ptr(), id: 1, dev: device { dma_mask: &mut sdio_dmamask, coherent_dma_mask: 0xffff_ffff }, resource: core::ptr::null_mut(), num_resources: 0 };
pub unsafe fn dove_sdio0_init() { platform_device_register(&mut dove_sdio0); }
pub unsafe fn dove_sdio1_init() { platform_device_register(&mut dove_sdio1); }

pub unsafe fn dove_setup_cpu_wins() {
    mvebu_mbus_add_window_remap_by_id(DOVE_MBUS_PCIE0_IO_TARGET, DOVE_MBUS_PCIE0_IO_ATTR, DOVE_PCIE0_IO_PHYS_BASE, DOVE_PCIE0_IO_SIZE, DOVE_PCIE0_IO_BUS_BASE);
    mvebu_mbus_add_window_remap_by_id(DOVE_MBUS_PCIE1_IO_TARGET, DOVE_MBUS_PCIE1_IO_ATTR, DOVE_PCIE1_IO_PHYS_BASE, DOVE_PCIE1_IO_SIZE, DOVE_PCIE1_IO_BUS_BASE);
    mvebu_mbus_add_window_by_id(DOVE_MBUS_PCIE0_MEM_TARGET, DOVE_MBUS_PCIE0_MEM_ATTR, DOVE_PCIE0_MEM_PHYS_BASE, DOVE_PCIE0_MEM_SIZE);
    mvebu_mbus_add_window_by_id(DOVE_MBUS_PCIE1_MEM_TARGET, DOVE_MBUS_PCIE1_MEM_ATTR, DOVE_PCIE1_MEM_PHYS_BASE, DOVE_PCIE1_MEM_SIZE);
    mvebu_mbus_add_window_by_id(DOVE_MBUS_CESA_TARGET, DOVE_MBUS_CESA_ATTR, DOVE_CESA_PHYS_BASE, DOVE_CESA_SIZE);
    mvebu_mbus_add_window_by_id(DOVE_MBUS_BOOTROM_TARGET, DOVE_MBUS_BOOTROM_ATTR, DOVE_BOOTROM_PHYS_BASE, DOVE_BOOTROM_SIZE);
    mvebu_mbus_add_window_by_id(DOVE_MBUS_SCRATCHPAD_TARGET, DOVE_MBUS_SCRATCHPAD_ATTR, DOVE_SCRATCHPAD_PHYS_BASE, DOVE_SCRATCHPAD_SIZE);
}

static mut pmu_domains: [dove_pmu_domain_initdata; 3] = [
    dove_pmu_domain_initdata { pwr_mask: PMU_PWR_VPU_PWR_DWN_MASK, rst_mask: PMU_SW_RST_VIDEO_MASK, iso_mask: PMU_ISO_VIDEO_MASK, name: c"vpu-domain".as_ptr() },
    dove_pmu_domain_initdata { pwr_mask: PMU_PWR_GPU_PWR_DWN_MASK, rst_mask: PMU_SW_RST_GPU_MASK, iso_mask: PMU_ISO_GPU_MASK, name: c"gpu-domain".as_ptr() },
    dove_pmu_domain_initdata { pwr_mask: 0, rst_mask: 0, iso_mask: 0, name: core::ptr::null() },
];
static mut pmu_data: dove_pmu_initdata = dove_pmu_initdata { pmc_base: DOVE_PMU_VIRT_BASE, pmu_base: DOVE_PMU_VIRT_BASE + 0x8000, irq: IRQ_DOVE_PMU, irq_domain_start: IRQ_DOVE_PMU_START, domains: pmu_domains.as_ptr() };

pub unsafe fn dove_init() {
    pr_info(c"Dove 88AP510 SoC, TCLK = %d MHz.\n".as_ptr(), (dove_tclk + 499999) / 1000000);
    // CONFIG_CACHE_TAUROS2 conditional
    tauros2_init(0);
    dove_setup_cpu_wins(); dove_clk_init(); dove_init_pmu_legacy(&mut pmu_data); dove_rtc_init(); dove_xor0_init(); dove_xor1_init();
}

pub unsafe fn dove_restart(_mode: reboot_mode, _cmd: *const i8) -> ! {
    writel(SOFT_RESET_OUT_EN, RSTOUTn_MASK); writel(SOFT_RESET, SYSTEM_SOFT_RESET);
    loop { core::hint::spin_loop(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
