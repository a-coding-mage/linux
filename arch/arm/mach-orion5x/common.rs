// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/common.c
 *
 * Core functions for Marvell Orion 5x SoCs
 *
 * Maintainer: Tzachi Perelstein <tzachi@marvell.com>
 */

// Kernel and platform dependencies supplied by the surrounding translation.

/****************************************************************************
 * I/O Address Mapping
 ****************************************************************************/
static mut orion5x_io_desc: [map_desc; 2] = [
    map_desc {
        virtual_: ORION5X_REGS_VIRT_BASE as c_ulong,
        pfn: __phys_to_pfn(ORION5X_REGS_PHYS_BASE),
        length: ORION5X_REGS_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        virtual_: ORION5X_PCIE_WA_VIRT_BASE as c_ulong,
        pfn: __phys_to_pfn(ORION5X_PCIE_WA_PHYS_BASE),
        length: ORION5X_PCIE_WA_SIZE,
        type_: MT_DEVICE,
    },
];

pub unsafe fn orion5x_map_io() {
    iotable_init(orion5x_io_desc.as_mut_ptr(), orion5x_io_desc.len());
}

/****************************************************************************
 * CLK tree
 ****************************************************************************/
static mut tclk: *mut clk = core::ptr::null_mut();

pub unsafe fn clk_init() {
    tclk = clk_register_fixed_rate(core::ptr::null_mut(), "tclk\0".as_ptr() as *const i8, core::ptr::null(), 0, orion5x_tclk);
    orion_clkdev_init(tclk);
}

/****************************************************************************
 * EHCI0
 ****************************************************************************/
pub unsafe fn orion5x_ehci0_init() {
    orion_ehci_init(ORION5X_USB0_PHYS_BASE, IRQ_ORION5X_USB0_CTRL, EHCI_PHY_ORION);
}

/****************************************************************************
 * EHCI1
 ****************************************************************************/
pub unsafe fn orion5x_ehci1_init() {
    orion_ehci_1_init(ORION5X_USB1_PHYS_BASE, IRQ_ORION5X_USB1_CTRL);
}

/****************************************************************************
 * GE00
 ****************************************************************************/
pub unsafe fn orion5x_eth_init(eth_data: *mut mv643xx_eth_platform_data) {
    orion_ge00_init(eth_data, ORION5X_ETH_PHYS_BASE, IRQ_ORION5X_ETH_SUM,
                    IRQ_ORION5X_ETH_ERR, MV643XX_TX_CSUM_DEFAULT_LIMIT);
}

/****************************************************************************
 * I2C
 ****************************************************************************/
pub unsafe fn orion5x_i2c_init() {
    orion_i2c_init(I2C_PHYS_BASE, IRQ_ORION5X_I2C, 8);
}

/****************************************************************************
 * SATA
 ****************************************************************************/
pub unsafe fn orion5x_sata_init(sata_data: *mut mv_sata_platform_data) {
    orion_sata_init(sata_data, ORION5X_SATA_PHYS_BASE, IRQ_ORION5X_SATA);
}

/****************************************************************************
 * SPI
 ****************************************************************************/
pub unsafe fn orion5x_spi_init() { orion_spi_init(SPI_PHYS_BASE); }

/****************************************************************************
 * UART0
 ****************************************************************************/
pub unsafe fn orion5x_uart0_init() {
    orion_uart0_init(UART0_VIRT_BASE, UART0_PHYS_BASE, IRQ_ORION5X_UART0, tclk);
}

/****************************************************************************
 * UART1
 ****************************************************************************/
pub unsafe fn orion5x_uart1_init() {
    orion_uart1_init(UART1_VIRT_BASE, UART1_PHYS_BASE, IRQ_ORION5X_UART1, tclk);
}

/****************************************************************************
 * XOR engine
 ****************************************************************************/
pub unsafe fn orion5x_xor_init() {
    orion_xor0_init(ORION5X_XOR_PHYS_BASE, ORION5X_XOR_PHYS_BASE + 0x200,
                   IRQ_ORION5X_XOR0, IRQ_ORION5X_XOR1);
}

/****************************************************************************
 * Cryptographic Engines and Security Accelerator (CESA)
 ****************************************************************************/
unsafe fn orion5x_crypto_init() {
    mvebu_mbus_add_window_by_id(ORION_MBUS_SRAM_TARGET, ORION_MBUS_SRAM_ATTR,
                                ORION5X_SRAM_PHYS_BASE, ORION5X_SRAM_SIZE);
    orion_crypto_init(ORION5X_CRYPTO_PHYS_BASE, ORION5X_SRAM_PHYS_BASE,
                      SZ_8K, IRQ_ORION5X_CESA);
}

/****************************************************************************
 * Watchdog
 ****************************************************************************/
static mut orion_wdt_resource: [resource; 2] = [
    DEFINE_RES_MEM(TIMER_PHYS_BASE, 0x04),
    DEFINE_RES_MEM(RSTOUTn_MASK_PHYS, 0x04),
];

static mut orion_wdt_device: platform_device = platform_device {
    name: "orion_wdt\0".as_ptr() as *const i8,
    id: -1,
    num_resources: orion_wdt_resource.len(),
    resource: orion_wdt_resource.as_mut_ptr(),
};

unsafe fn orion5x_wdt_init() { platform_device_register(&mut orion_wdt_device); }

/****************************************************************************
 * Time handling
 ****************************************************************************/
pub unsafe fn orion5x_init_early() {
    let mut rev: u32 = 0;
    let mut dev: u32 = 0;
    let mbus_soc_name: *const i8;
    orion_time_set_base(TIMER_VIRT_BASE);
    orion5x_pcie_id(&mut dev, &mut rev);
    if dev == MV88F5281_DEV_ID { mbus_soc_name = "marvell,orion5x-88f5281-mbus\0".as_ptr() as *const i8; }
    else if dev == MV88F5182_DEV_ID { mbus_soc_name = "marvell,orion5x-88f5182-mbus\0".as_ptr() as *const i8; }
    else if dev == MV88F5181_DEV_ID { mbus_soc_name = "marvell,orion5x-88f5181-mbus\0".as_ptr() as *const i8; }
    else if dev == MV88F6183_DEV_ID { mbus_soc_name = "marvell,orion5x-88f6183-mbus\0".as_ptr() as *const i8; }
    else { mbus_soc_name = core::ptr::null(); }
    mvebu_mbus_init(mbus_soc_name, ORION5X_BRIDGE_WINS_BASE, ORION5X_BRIDGE_WINS_SZ,
                    ORION5X_DDR_WINS_BASE, ORION5X_DDR_WINS_SZ);
}

pub unsafe fn orion5x_setup_wins() {
    /* The PCIe windows will no longer be statically allocated here once
     * Orion5x is migrated to the pci-mvebu driver. */
    mvebu_mbus_add_window_remap_by_id(ORION_MBUS_PCIE_IO_TARGET, ORION_MBUS_PCIE_IO_ATTR,
                                      ORION5X_PCIE_IO_PHYS_BASE, ORION5X_PCIE_IO_SIZE,
                                      ORION5X_PCIE_IO_BUS_BASE);
    mvebu_mbus_add_window_by_id(ORION_MBUS_PCIE_MEM_TARGET, ORION_MBUS_PCIE_MEM_ATTR,
                                ORION5X_PCIE_MEM_PHYS_BASE, ORION5X_PCIE_MEM_SIZE);
    mvebu_mbus_add_window_remap_by_id(ORION_MBUS_PCI_IO_TARGET, ORION_MBUS_PCI_IO_ATTR,
                                      ORION5X_PCI_IO_PHYS_BASE, ORION5X_PCI_IO_SIZE,
                                      ORION5X_PCI_IO_BUS_BASE);
    mvebu_mbus_add_window_by_id(ORION_MBUS_PCI_MEM_TARGET, ORION_MBUS_PCI_MEM_ATTR,
                                ORION5X_PCI_MEM_PHYS_BASE, ORION5X_PCI_MEM_SIZE);
}

pub static mut orion5x_tclk: i32 = 0;

unsafe fn orion5x_find_tclk() -> i32 {
    let mut dev = 0u32;
    let mut rev = 0u32;
    orion5x_pcie_id(&mut dev, &mut rev);
    if dev == MV88F6183_DEV_ID && (readl(MPP_RESET_SAMPLE) & 0x00000200) == 0 { return 133333333; }
    166666667
}

pub unsafe fn orion5x_timer_init() {
    orion5x_tclk = orion5x_find_tclk();
    orion_time_init(ORION5X_BRIDGE_VIRT_BASE, BRIDGE_INT_TIMER1_CLR,
                    IRQ_ORION5X_BRIDGE, orion5x_tclk);
}

/****************************************************************************
 * General
 ****************************************************************************/
/* Identify device ID and rev from PCIe configuration header space '0'. */
pub unsafe fn orion5x_id(dev: *mut u32, rev: *mut u32, dev_name: *mut *mut i8) {
    orion5x_pcie_id(dev, rev);
    *dev_name = if *dev == MV88F5281_DEV_ID {
        if *rev == MV88F5281_REV_D2 { "MV88F5281-D2\0" }
        else if *rev == MV88F5281_REV_D1 { "MV88F5281-D1\0" }
        else if *rev == MV88F5281_REV_D0 { "MV88F5281-D0\0" }
        else { "MV88F5281-Rev-Unsupported\0" }
    } else if *dev == MV88F5182_DEV_ID {
        if *rev == MV88F5182_REV_A2 { "MV88F5182-A2\0" } else { "MV88F5182-Rev-Unsupported\0" }
    } else if *dev == MV88F5181_DEV_ID {
        if *rev == MV88F5181_REV_B1 { "MV88F5181-Rev-B1\0" }
        else if *rev == MV88F5181L_REV_A1 { "MV88F5181L-Rev-A1\0" }
        else { "MV88F5181(L)-Rev-Unsupported\0" }
    } else if *dev == MV88F6183_DEV_ID {
        if *rev == MV88F6183_REV_B0 { "MV88F6183-Rev-B0\0" } else { "MV88F6183-Rev-Unsupported\0" }
    } else { "Device-Unknown\0" }.as_ptr() as *mut i8;
}

pub unsafe fn orion5x_init() {
    let mut dev = 0u32;
    let mut rev = 0u32;
    let mut dev_name: *mut i8 = core::ptr::null_mut();
    orion5x_id(&mut dev, &mut rev, &mut dev_name);
    printk(KERN_INFO, "Orion ID: %s. TCLK=%d.\0".as_ptr() as *const i8, dev_name, orion5x_tclk);
    orion5x_setup_wins();
    clk_init();
    if dev == MV88F5281_DEV_ID && rev == MV88F5281_REV_D0 {
        printk(KERN_INFO, "Orion: Applying 5281 D0 WFI workaround.\0".as_ptr() as *const i8);
        cpu_idle_poll_ctrl(true);
    }
    if (dev == MV88F5181_DEV_ID && rev >= MV88F5181L_REV_A0) || dev == MV88F5182_DEV_ID || dev == MV88F6183_DEV_ID {
        orion5x_crypto_init();
    }
    orion5x_wdt_init();
}

pub unsafe fn orion5x_restart(_mode: reboot_mode, _cmd: *const i8) {
    orion5x_setbits(RSTOUTn_MASK, 1 << 2);
    orion5x_setbits(CPU_SOFT_RESET, 1);
    mdelay(200);
    orion5x_clrbits(CPU_SOFT_RESET, 1);
}

/* Many orion-based systems have buggy bootloader implementations.
 * This is a common fixup for bogus memory tags. */
pub unsafe fn tag_fixup_mem32(mut t: *mut tag, _from: *mut *mut i8) {
    while (*t).hdr.size != 0 {
        if (*t).hdr.tag == ATAG_MEM && (((*t).u.mem.size == 0) || ((*t).u.mem.size & !PAGE_MASK) != 0 || ((*t).u.mem.start & !PAGE_MASK) != 0) {
            printk(KERN_WARNING, "Clearing invalid memory bank %dKB@0x%08x\0".as_ptr() as *const i8,
                   (*t).u.mem.size / 1024, (*t).u.mem.start);
            (*t).hdr.tag = 0;
        }
        t = tag_next(t);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
