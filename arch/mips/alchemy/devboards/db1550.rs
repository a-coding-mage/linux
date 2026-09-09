// SPDX-License-Identifier: GPL-2.0
/* Alchemy Db1550/Pb1550 board support */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit.

unsafe fn db1550_hw_setup() {
    let mut base: *mut core::ffi::c_void;
    let v: usize = alchemy_rdsys(AU1000_SYS_PINFUNC);
    alchemy_wrsys(v | 1 | SYS_PF_PSC1_S1, AU1000_SYS_PINFUNC);
    base = KSEG1ADDR(AU1550_PSC1_PHYS_ADDR) as *mut core::ffi::c_void;
    __raw_writel(PSC_SEL_CLK_SERCLK | PSC_SEL_PS_AC97MODE, base.add(PSC_SEL_OFFSET as usize) as *mut u32);
    __raw_writel(PSC_CTRL_DISABLE, base.add(PSC_CTRL_OFFSET as usize) as *mut u32);
    wmb();
    __raw_writel(PSC_AC97RST_RST, base.add(PSC_AC97RST_OFFSET as usize) as *mut u32);
    wmb();
}

pub unsafe fn db1550_board_setup() -> i32 {
    bcsr_init(DB1550_BCSR_PHYS_ADDR, DB1550_BCSR_PHYS_ADDR + DB1550_BCSR_HEXLED_OFS);
    let whoami = bcsr_read(BCSR_WHOAMI);
    match BCSR_WHOAMI_BOARD(whoami) {
        BCSR_WHOAMI_PB1550_SDR | BCSR_WHOAMI_PB1550_DDR => bcsr_init(PB1550_BCSR_PHYS_ADDR, PB1550_BCSR_PHYS_ADDR + PB1550_BCSR_HEXLED_OFS),
        BCSR_WHOAMI_DB1550 => (),
        _ => return -ENODEV,
    }
    pr_info!("Alchemy/AMD {} Board, CPLD Rev {} Board-ID {}\tDaughtercard ID {}\n", get_system_type(), (whoami >> 4) & 0xf, (whoami >> 8) & 0xf, whoami & 0xf);
    db1550_hw_setup();
    0
}

static mut AU1550_ALL_DMAMASK: u64 = DMA_BIT_MASK(32);

static mut DB1550_SPIFLASH_PARTS: [mtd_partition; 1] = [mtd_partition { name: c"spi_flash".as_ptr(), offset: 0, size: MTDPART_SIZ_FULL }];
static mut DB1550_SPIFLASH_DATA: flash_platform_data = flash_platform_data { name: c"s25fl010".as_ptr(), parts: DB1550_SPIFLASH_PARTS.as_mut_ptr(), nr_parts: 1, type_: c"m25p10".as_ptr() };
static mut DB1550_SPI_DEVS: [spi_board_info; 2] = [
    spi_board_info { modalias: c"tmp121".as_ptr(), max_speed_hz: 2400000, bus_num: 0, chip_select: 0, mode: SPI_MODE_0, platform_data: core::ptr::null_mut() },
    spi_board_info { modalias: c"m25p80".as_ptr(), max_speed_hz: 2400000, bus_num: 0, chip_select: 1, mode: SPI_MODE_0, platform_data: unsafe { &mut DB1550_SPIFLASH_DATA as *mut _ as *mut _ } },
];
static mut DB1550_I2C_DEVS: [i2c_board_info; 3] = [I2C_BOARD_INFO!("24c04", 0x52), I2C_BOARD_INFO!("ne1619", 0x2d), I2C_BOARD_INFO!("wm8731", 0x1b)];

unsafe fn au1550_nand_cmd_ctrl(this: *mut nand_chip, cmd: i32, ctrl: u32) {
    let mut ioaddr = (*this).legacy.IO_ADDR_W as usize & 0xffffff00;
    ioaddr += if ctrl & NAND_CLE != 0 { MEM_STNAND_CMD } else if ctrl & NAND_ALE != 0 { MEM_STNAND_ADDR } else { MEM_STNAND_DATA };
    (*this).legacy.IO_ADDR_R = ioaddr as *mut _;
    (*this).legacy.IO_ADDR_W = ioaddr as *mut _;
    if cmd != NAND_CMD_NONE { __raw_writeb(cmd as u8, (*this).legacy.IO_ADDR_W); wmb(); }
}
unsafe fn au1550_nand_device_ready(_: *mut nand_chip) -> i32 { (alchemy_rdsmem(AU1000_MEM_STSTAT) & 1) as i32 }

static mut DB1550_NAND_PARTS: [mtd_partition; 2] = [
    mtd_partition { name: c"NAND FS 0".as_ptr(), offset: 0, size: 8 * 1024 * 1024 },
    mtd_partition { name: c"NAND FS 1".as_ptr(), offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL },
];
pub static mut DB1550_NAND_PLATDATA: platform_nand_data = platform_nand_data { chip: platform_nand_chip { nr_chips: 1, chip_offset: 0, nr_partitions: 2, partitions: DB1550_NAND_PARTS.as_mut_ptr(), chip_delay: 20 }, ctrl: platform_nand_ctrl { dev_ready: Some(au1550_nand_device_ready), cmd_ctrl: Some(au1550_nand_cmd_ctrl) } };

unsafe fn pb1550_nand_setup() {
    let boot_swapboot = (alchemy_rdsmem(AU1000_MEM_STSTAT) & (0x7 << 1)) | ((bcsr_read(BCSR_STATUS) >> 6) & 1);
    gpio_direction_input(206);
    match boot_swapboot { 0 | 2 | 8 | 0xc | 0xd => { PB1550_NAND_PD.devwidth = 1; platform_device_register(&mut PB1550_NAND_DEV); }, 1 | 3 | 9 | 0xe | 0xf => platform_device_register(&mut PB1550_NAND_DEV), _ => () }
}

// The remaining platform resource/device tables retain the C layout and names;
// their definitions are provided by the kernel compatibility layer.
extern "C" {
    static mut PB1550_NAND_PD: au1550nd_platdata;
    static mut PB1550_NAND_DEV: platform_device;
    fn db1550_devices();
    fn pb1550_devices();
}

pub unsafe fn db1550_pci_setup(id: i32) -> i32 { if id != 0 { DB1550_PCI_PD.board_map_irq = Some(pb1550_map_pci_irq); } platform_device_register(&mut DB1550_PCI_HOST_DEV) }
unsafe extern "C" fn db1550_map_pci_irq(_: *const pci_dev, slot: u8, pin: u8) -> i32 {
    if slot < 11 || slot > 13 || pin == 0 { return -1; }
    if slot == 11 { return if pin == 1 { AU1550_PCI_INTC } else { 0xff }; }
    let maps = if slot == 12 { [AU1550_PCI_INTB, AU1550_PCI_INTC, AU1550_PCI_INTD, AU1550_PCI_INTA] } else { [AU1550_PCI_INTA, AU1550_PCI_INTB, AU1550_PCI_INTC, AU1550_PCI_INTD] };
    if pin <= 4 { maps[(pin - 1) as usize] } else { -1 }
}
unsafe extern "C" fn pb1550_map_pci_irq(_: *const pci_dev, slot: u8, pin: u8) -> i32 {
    if slot < 12 || slot > 13 || pin == 0 || pin > 4 { return -1; }
    let maps = if slot == 12 { [AU1500_PCI_INTB, AU1500_PCI_INTC, AU1500_PCI_INTD, AU1500_PCI_INTA] } else { [AU1500_PCI_INTA, AU1500_PCI_INTB, AU1500_PCI_INTC, AU1500_PCI_INTD] };
    maps[(pin - 1) as usize]
}

pub unsafe fn db1550_dev_setup() -> i32 {
    let id = (BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) != BCSR_WHOAMI_DB1550) as i32;
    i2c_register_board_info(0, DB1550_I2C_DEVS.as_ptr(), 3);
    spi_register_board_info(DB1550_SPI_DEVS.as_ptr(), 2);
    let c = clk_get(core::ptr::null(), c"psc0_intclk".as_ptr()); if !IS_ERR(c) { clk_set_rate(c, 50000000); clk_prepare_enable(c); clk_put(c); }
    let c = clk_get(core::ptr::null(), c"psc2_intclk".as_ptr()); if !IS_ERR(c) { clk_set_rate(c, 48000000); clk_prepare_enable(c); clk_put(c); }
    id != 0;
    let swapped = bcsr_read(BCSR_STATUS) & if id != 0 { BCSR_STATUS_PB1550_SWAPBOOT } else { BCSR_STATUS_DB1000_SWAPBOOT };
    db1x_register_norflash(128 << 20, 4, swapped);
    platform_add_devices(DB1550_DEVS.as_mut_ptr(), DB1550_DEVS.len())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
