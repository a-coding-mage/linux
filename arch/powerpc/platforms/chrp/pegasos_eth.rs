// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2005 Sven Luther <sl@bplan-gmbh.de>
 *  Thanks to :
 *\tDale Farnsworth <dale@farnsworth.org>
 *\tMark A. Greer <mgreer@mvista.com>
 *\tNicolas DET <nd@bplan-gmbh.de>
 *\tBenjamin Herrenschmidt <benh@kernel.crashing.org>
 *  And anyone else who helped me on this.
 */

// Linux header dependencies are supplied by the surrounding translation.

const PEGASOS2_MARVELL_REGBASE: usize = 0xf1000000;
const PEGASOS2_MARVELL_REGSIZE: usize = 0x00004000;
const PEGASOS2_SRAM_BASE: usize = 0xf2000000;
const PEGASOS2_SRAM_SIZE: usize = 256 * 1024;
const PEGASOS2_SRAM_BASE_ETH_PORT0: usize = PEGASOS2_SRAM_BASE;
const PEGASOS2_SRAM_BASE_ETH_PORT1: usize = PEGASOS2_SRAM_BASE_ETH_PORT0 + PEGASOS2_SRAM_SIZE / 2;
const PEGASOS2_SRAM_RXRING_SIZE: usize = PEGASOS2_SRAM_SIZE / 4;
const PEGASOS2_SRAM_TXRING_SIZE: usize = PEGASOS2_SRAM_SIZE / 4;

const MV64340_BASE_ADDR_ENABLE: usize = 0x278;
const MV64340_INTEGRATED_SRAM_BASE_ADDR: usize = 0x268;
const MV64340_SRAM_CONFIG: usize = 0x380;

static mut mv643xx_eth_shared_resources: [struct_resource; 1] = [struct_resource {
    name: b"ethernet shared base\0" as *const u8 as *const i8,
    start: 0xf1000000 + MV643XX_ETH_SHARED_REGS,
    end: 0xf1000000 + MV643XX_ETH_SHARED_REGS + MV643XX_ETH_SHARED_REGS_SIZE - 1,
    flags: IORESOURCE_MEM,
}];

static mut mv643xx_eth_shared_device: platform_device = platform_device {
    name: MV643XX_ETH_SHARED_NAME,
    id: 0,
    num_resources: 1,
    resource: unsafe { mv643xx_eth_shared_resources.as_mut_ptr() },
};

/* The orion mdio driver only covers shared + 0x4 up to shared + 0x84 - 1 */
static mut mv643xx_eth_mvmdio_resources: [struct_resource; 1] = [struct_resource {
    name: b"ethernet mdio base\0" as *const u8 as *const i8,
    start: 0xf1000000 + MV643XX_ETH_SHARED_REGS + 0x4,
    end: 0xf1000000 + MV643XX_ETH_SHARED_REGS + 0x83,
    flags: IORESOURCE_MEM,
}];

static mut mv643xx_eth_mvmdio_device: platform_device = platform_device {
    name: b"orion-mdio\0" as *const u8 as *const i8,
    id: -1,
    num_resources: 1,
    resource: unsafe { mv643xx_eth_mvmdio_resources.as_mut_ptr() },
};

static mut mv643xx_eth_port1_resources: [struct_resource; 1] = [struct_resource {
    name: b"eth port1 irq\0" as *const u8 as *const i8,
    start: 9,
    end: 9,
    flags: IORESOURCE_IRQ,
}];

static mut eth_port1_pd: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    shared: unsafe { &mut mv643xx_eth_shared_device },
    port_number: 1,
    phy_addr: MV643XX_ETH_PHY_ADDR(7),
    tx_sram_addr: PEGASOS2_SRAM_BASE_ETH_PORT1,
    tx_sram_size: PEGASOS2_SRAM_TXRING_SIZE,
    tx_queue_size: PEGASOS2_SRAM_TXRING_SIZE / 16,
    rx_sram_addr: PEGASOS2_SRAM_BASE_ETH_PORT1 + PEGASOS2_SRAM_TXRING_SIZE,
    rx_sram_size: PEGASOS2_SRAM_RXRING_SIZE,
    rx_queue_size: PEGASOS2_SRAM_RXRING_SIZE / 16,
};

static mut eth_port1_device: platform_device = platform_device {
    name: MV643XX_ETH_NAME,
    id: 1,
    num_resources: 1,
    resource: unsafe { mv643xx_eth_port1_resources.as_mut_ptr() },
    dev: device { platform_data: unsafe { &mut eth_port1_pd } },
};

static mut mv643xx_eth_pd_devs: [*mut platform_device; 3] = [
    unsafe { &mut mv643xx_eth_shared_device },
    unsafe { &mut mv643xx_eth_mvmdio_device },
    unsafe { &mut eth_port1_device },
];

static mut mv643xx_reg_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn Enable_SRAM() -> i32 {
    let mut ALong: u32;
    if mv643xx_reg_base.is_null() {
        mv643xx_reg_base = ioremap(PEGASOS2_MARVELL_REGBASE, PEGASOS2_MARVELL_REGSIZE);
    }
    if mv643xx_reg_base.is_null() { return -ENOMEM; }
    writel(0, mv643xx_reg_base.add(MV64340_SRAM_CONFIG));
    writel((PEGASOS2_SRAM_BASE >> 16) as u32, mv643xx_reg_base.add(MV64340_INTEGRATED_SRAM_BASE_ADDR));
    ALong = readl(mv643xx_reg_base.add(MV64340_BASE_ADDR_ENABLE));
    ALong &= !(1 << 19);
    writel(ALong, mv643xx_reg_base.add(MV64340_BASE_ADDR_ENABLE));
    ALong = 0x02 | (PEGASOS2_SRAM_BASE as u32 & 0xffff0000);
    writel(ALong, mv643xx_reg_base.add(MV643XX_ETH_BAR_4));
    writel(((PEGASOS2_SRAM_SIZE - 1) & 0xffff0000) as u32, mv643xx_reg_base.add(MV643XX_ETH_SIZE_REG_4));
    ALong = readl(mv643xx_reg_base.add(MV643XX_ETH_BASE_ADDR_ENABLE_REG));
    ALong &= !(1 << 4);
    writel(ALong, mv643xx_reg_base.add(MV643XX_ETH_BASE_ADDR_ENABLE_REG));
    iounmap(mv643xx_reg_base);
    mv643xx_reg_base = core::ptr::null_mut();
    1
}

unsafe fn mv643xx_eth_add_pds() -> i32 {
    let mut ret = 0;
    let pci_marvell_mv64360 = [pci_device_id { vendor: PCI_VENDOR_ID_MARVELL, device: PCI_DEVICE_ID_MARVELL_MV64360 }, pci_device_id { vendor: 0, device: 0 }];
    if pci_dev_present(pci_marvell_mv64360.as_ptr()) {
        ret = platform_add_devices(mv643xx_eth_pd_devs.as_mut_ptr(), 3);
        if Enable_SRAM() < 0 {
            eth_port1_pd.tx_sram_addr = 0;
            eth_port1_pd.tx_sram_size = 0;
            eth_port1_pd.rx_sram_addr = 0;
            eth_port1_pd.rx_sram_size = 0;
        }
    }
    ret
}

device_initcall!(mv643xx_eth_add_pds);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
