// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mv78xx0/common.c
 *
 * Core functions for Marvell MV78xx0 SoCs
 */

// C dependencies supplied by the surrounding kernel translation.

static mut TCLK: *mut clk = core::ptr::null_mut();

unsafe fn get_tclk() -> i32;

/*****************************************************************************
 * Common bits
 ****************************************************************************/
pub unsafe fn mv78xx0_core_index() -> i32 {
    let extra: u32;

    /*
     * Read Extra Features register.
     */
    core::arch::asm!("mrc p15, 1, {0}, c15, c1, 0", out(reg) extra);

    if (extra & 0x00004000) != 0 { 1 } else { 0 }
}

unsafe fn get_hclk() -> i32 {
    /*
     * HCLK tick rate is configured by DEV_D[7:5] pins.
     */
    match (readl(SAMPLE_AT_RESET_LOW) >> 5) & 7 {
        0 => 166666667,
        1 => 200000000,
        2 => 266666667,
        3 => 333333333,
        4 => 400000000,
        _ => panic!("unknown HCLK PLL setting: {:08x}\n", readl(SAMPLE_AT_RESET_LOW)),
    }
}

unsafe fn get_pclk_l2clk(hclk: i32, core_index: i32, pclk: *mut i32, l2clk: *mut i32) {
    let cfg: u32;

    /*
     * Core #0 PCLK/L2CLK is configured by bits [13:8], core #1
     * PCLK/L2CLK by bits [19:14].
     */
    if core_index == 0 {
        cfg = (readl(SAMPLE_AT_RESET_LOW) >> 8) & 0x3f;
    } else {
        cfg = (readl(SAMPLE_AT_RESET_LOW) >> 14) & 0x3f;
    }

    /*
     * Bits [11:8] ([17:14] for core #1) configure the PCLK:HCLK
     * ratio (1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5, 5, 5.5, 6).
     */
    *pclk = (((hclk as i64) * (2 + (cfg & 0xf) as i64)) >> 1) as i32;

    /*
     * Bits [13:12] ([19:18] for core #1) configure the PCLK:L2CLK
     * ratio (1, 2, 3).
     */
    *l2clk = *pclk / ((((cfg >> 4) & 3) + 1) as i32);
}

unsafe fn get_tclk() -> i32 {
    let tclk_freq: i32;

    /*
     * TCLK tick rate is configured by DEV_A[2:0] strap pins.
     */
    tclk_freq = match (readl(SAMPLE_AT_RESET_HIGH) >> 6) & 7 {
        1 => 166666667,
        3 => 200000000,
        _ => panic!("unknown TCLK PLL setting: {:08x}\n", readl(SAMPLE_AT_RESET_HIGH)),
    };

    tclk_freq
}

/*****************************************************************************
 * I/O Address Mapping
 ****************************************************************************/
static mut MV78XX0_IO_DESC: [map_desc; 2] = [
    map_desc {
        virtual_: MV78XX0_CORE_REGS_VIRT_BASE as usize,
        pfn: 0,
        length: MV78XX0_CORE_REGS_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        virtual_: MV78XX0_REGS_VIRT_BASE as usize,
        pfn: __phys_to_pfn(MV78XX0_REGS_PHYS_BASE),
        length: MV78XX0_REGS_SIZE,
        type_: MT_DEVICE,
    },
];

pub unsafe fn mv78xx0_map_io() {
    let phys: usize;

    /*
     * Map the right set of per-core registers depending on
     * which core we are running on.
     */
    if mv78xx0_core_index() == 0 {
        phys = MV78XX0_CORE0_REGS_PHYS_BASE;
    } else {
        phys = MV78XX0_CORE1_REGS_PHYS_BASE;
    }
    MV78XX0_IO_DESC[0].pfn = __phys_to_pfn(phys);

    iotable_init(MV78XX0_IO_DESC.as_ptr(), MV78XX0_IO_DESC.len());
}

/*****************************************************************************
 * CLK tree
 ****************************************************************************/
unsafe fn clk_init() {
    TCLK = clk_register_fixed_rate(core::ptr::null_mut(), "tclk", core::ptr::null(), 0, get_tclk());

    orion_clkdev_init(TCLK);
}

/*****************************************************************************
 * EHCI
 ****************************************************************************/
pub unsafe fn mv78xx0_ehci0_init() { orion_ehci_init(USB0_PHYS_BASE, IRQ_MV78XX0_USB_0, EHCI_PHY_NA); }
pub unsafe fn mv78xx0_ehci1_init() { orion_ehci_1_init(USB1_PHYS_BASE, IRQ_MV78XX0_USB_1); }
pub unsafe fn mv78xx0_ehci2_init() { orion_ehci_2_init(USB2_PHYS_BASE, IRQ_MV78XX0_USB_2); }

/*****************************************************************************
 * GE00
 ****************************************************************************/
pub unsafe fn mv78xx0_ge00_init(eth_data: *mut mv643xx_eth_platform_data) {
    orion_ge00_init(eth_data, GE00_PHYS_BASE, IRQ_MV78XX0_GE00_SUM, IRQ_MV78XX0_GE_ERR, MV643XX_TX_CSUM_DEFAULT_LIMIT);
}
pub unsafe fn mv78xx0_ge01_init(eth_data: *mut mv643xx_eth_platform_data) {
    orion_ge01_init(eth_data, GE01_PHYS_BASE, IRQ_MV78XX0_GE01_SUM, MV643XX_TX_CSUM_DEFAULT_LIMIT);
}
pub unsafe fn mv78xx0_ge10_init(eth_data: *mut mv643xx_eth_platform_data) {
    let (mut dev, mut rev) = (0u32, 0u32);
    mv78xx0_pcie_id(&mut dev, &mut rev);
    if dev == MV78X00_Z0_DEV_ID { (*eth_data).phy_addr = MV643XX_ETH_PHY_NONE; (*eth_data).speed = SPEED_1000; (*eth_data).duplex = DUPLEX_FULL; }
    orion_ge10_init(eth_data, GE10_PHYS_BASE, IRQ_MV78XX0_GE10_SUM);
}
pub unsafe fn mv78xx0_ge11_init(eth_data: *mut mv643xx_eth_platform_data) {
    let (mut dev, mut rev) = (0u32, 0u32);
    mv78xx0_pcie_id(&mut dev, &mut rev);
    if dev == MV78X00_Z0_DEV_ID { (*eth_data).phy_addr = MV643XX_ETH_PHY_NONE; (*eth_data).speed = SPEED_1000; (*eth_data).duplex = DUPLEX_FULL; }
    orion_ge11_init(eth_data, GE11_PHYS_BASE, IRQ_MV78XX0_GE11_SUM);
}

/*****************************************************************************
 * I2C, SATA, UART
 ****************************************************************************/
pub unsafe fn mv78xx0_i2c_init() { orion_i2c_init(I2C_0_PHYS_BASE, IRQ_MV78XX0_I2C_0, 8); orion_i2c_1_init(I2C_1_PHYS_BASE, IRQ_MV78XX0_I2C_1, 8); }
pub unsafe fn mv78xx0_sata_init(sata_data: *mut mv_sata_platform_data) { orion_sata_init(sata_data, SATA_PHYS_BASE, IRQ_MV78XX0_SATA); }
pub unsafe fn mv78xx0_uart0_init() { orion_uart0_init(UART0_VIRT_BASE, UART0_PHYS_BASE, IRQ_MV78XX0_UART_0, TCLK); }
pub unsafe fn mv78xx0_uart1_init() { orion_uart1_init(UART1_VIRT_BASE, UART1_PHYS_BASE, IRQ_MV78XX0_UART_1, TCLK); }
pub unsafe fn mv78xx0_uart2_init() { orion_uart2_init(UART2_VIRT_BASE, UART2_PHYS_BASE, IRQ_MV78XX0_UART_2, TCLK); }
pub unsafe fn mv78xx0_uart3_init() { orion_uart3_init(UART3_VIRT_BASE, UART3_PHYS_BASE, IRQ_MV78XX0_UART_3, TCLK); }

/*****************************************************************************
 * Time handling
 ****************************************************************************/
pub unsafe fn mv78xx0_init_early() {
    orion_time_set_base(TIMER_VIRT_BASE);
    if mv78xx0_core_index() == 0 { mvebu_mbus_init("marvell,mv78xx0-mbus", BRIDGE_WINS_CPU0_BASE, BRIDGE_WINS_SZ, DDR_WINDOW_CPU0_BASE, DDR_WINDOW_CPU_SZ); }
    else { mvebu_mbus_init("marvell,mv78xx0-mbus", BRIDGE_WINS_CPU1_BASE, BRIDGE_WINS_SZ, DDR_WINDOW_CPU1_BASE, DDR_WINDOW_CPU_SZ); }
}
pub unsafe fn mv78xx0_timer_init() { orion_time_init(BRIDGE_VIRT_BASE, BRIDGE_INT_TIMER1_CLR, IRQ_MV78XX0_TIMER_1, get_tclk()); }

/****************************************************************************
* XOR engine
****************************************************************************/
pub unsafe fn mv78xx0_xor_init() { orion_xor0_init(XOR_PHYS_BASE, XOR_PHYS_BASE + 0x200, IRQ_MV78XX0_XOR_0, IRQ_MV78XX0_XOR_1); }

/****************************************************************************
 * Cryptographic Engines and Security Accelerator (CESA)
 ****************************************************************************/
pub unsafe fn mv78xx0_crypto_init() {
    mvebu_mbus_add_window_by_id(MV78XX0_MBUS_SRAM_TARGET, MV78XX0_MBUS_SRAM_ATTR, MV78XX0_SRAM_PHYS_BASE, MV78XX0_SRAM_SIZE);
    orion_crypto_init(CRYPTO_PHYS_BASE, MV78XX0_SRAM_PHYS_BASE, SZ_8K, IRQ_MV78XX0_CRYPTO);
}

/*****************************************************************************
 * General
 ****************************************************************************/
unsafe fn mv78xx0_id() -> *const u8 {
    let (mut dev, mut rev) = (0u32, 0u32);
    mv78xx0_pcie_id(&mut dev, &mut rev);
    if dev == MV78X00_Z0_DEV_ID { if rev == MV78X00_REV_Z0 { b"MV78X00-Z0\0".as_ptr() } else { b"MV78X00-Rev-Unsupported\0".as_ptr() } }
    else if dev == MV78100_DEV_ID { if rev == MV78100_REV_A0 { b"MV78100-A0\0".as_ptr() } else if rev == MV78100_REV_A1 { b"MV78100-A1\0".as_ptr() } else { b"MV78100-Rev-Unsupported\0".as_ptr() } }
    else if dev == MV78200_DEV_ID { if rev == MV78100_REV_A0 { b"MV78200-A0\0".as_ptr() } else { b"MV78200-Rev-Unsupported\0".as_ptr() } }
    else { b"Device-Unknown\0".as_ptr() }
}

unsafe fn is_l2_writethrough() -> i32 { if (readl(CPU_CONTROL) & L2_WRITETHROUGH) != 0 { 1 } else { 0 } }

pub unsafe fn mv78xx0_init() {
    let core_index = mv78xx0_core_index();
    let hclk = get_hclk();
    let (mut pclk, mut l2clk) = (0, 0);
    get_pclk_l2clk(hclk, core_index, &mut pclk, &mut l2clk);
    printk(KERN_INFO, mv78xx0_id());
    printk("core #%d, ", core_index); printk("PCLK = %dMHz, ", (pclk + 499999) / 1000000);
    printk("L2 = %dMHz, ", (l2clk + 499999) / 1000000); printk("HCLK = %dMHz, ", (hclk + 499999) / 1000000);
    printk("TCLK = %dMHz\n", (get_tclk() + 499999) / 1000000);
    if IS_ENABLED(CONFIG_CACHE_FEROCEON_L2) { feroceon_l2_init(is_l2_writethrough()); }
    clk_init();
}

pub unsafe fn mv78xx0_restart(_mode: reboot_mode, _cmd: *const u8) {
    /* Enable soft reset to assert RSTOUTn. */
    writel(SOFT_RESET_OUT_EN, RSTOUTn_MASK);
    /* Assert soft reset. */
    writel(SOFT_RESET, SYSTEM_SOFT_RESET);
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
