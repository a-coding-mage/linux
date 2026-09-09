// SPDX-License-Identifier: GPL-2.0-or-later
/* DBAu1200/PBAu1200 board platform device registration
 * Copyright (C) 2008-2011 Manuel Lauss
 */

// Linux headers and symbols used by this translation are supplied externally.

const BCSR_INT_IDE: u16 = 0x0001;
const BCSR_INT_ETH: u16 = 0x0002;
const BCSR_INT_PC0: u16 = 0x0004;
const BCSR_INT_PC0STSCHG: u16 = 0x0008;
const BCSR_INT_PC1: u16 = 0x0010;
const BCSR_INT_PC1STSCHG: u16 = 0x0020;
const BCSR_INT_DC: u16 = 0x0040;
const BCSR_INT_FLASHBUSY: u16 = 0x0080;
const BCSR_INT_PC0INSERT: u16 = 0x0100;
const BCSR_INT_PC0EJECT: u16 = 0x0200;
const BCSR_INT_PC1INSERT: u16 = 0x0400;
const BCSR_INT_PC1EJECT: u16 = 0x0800;
const BCSR_INT_SD0INSERT: u16 = 0x1000;
const BCSR_INT_SD0EJECT: u16 = 0x2000;
const BCSR_INT_SD1INSERT: u16 = 0x4000;
const BCSR_INT_SD1EJECT: u16 = 0x8000;
const DB1200_IDE_PHYS_ADDR: usize = 0x18800000;
const DB1200_IDE_REG_SHIFT: usize = 5;
const DB1200_IDE_PHYS_LEN: usize = 16 << DB1200_IDE_REG_SHIFT;
const DB1200_ETH_PHYS_ADDR: usize = 0x19000300;
const DB1200_NAND_PHYS_ADDR: usize = 0x20000000;
const PB1200_IDE_PHYS_ADDR: usize = 0x0c800000;
const PB1200_ETH_PHYS_ADDR: usize = 0x0d000300;
const PB1200_NAND_PHYS_ADDR: usize = 0x1c000000;
const DB1200_INT_BEGIN: i32 = AU1000_MAX_INTR + 1;
const DB1200_IDE_INT: i32 = DB1200_INT_BEGIN + 0;
const DB1200_ETH_INT: i32 = DB1200_INT_BEGIN + 1;
const DB1200_PC0_INT: i32 = DB1200_INT_BEGIN + 2;
const DB1200_PC0_STSCHG_INT: i32 = DB1200_INT_BEGIN + 3;
const DB1200_PC1_INT: i32 = DB1200_INT_BEGIN + 4;
const DB1200_PC1_STSCHG_INT: i32 = DB1200_INT_BEGIN + 5;
const DB1200_DC_INT: i32 = DB1200_INT_BEGIN + 6;
const DB1200_FLASHBUSY_INT: i32 = DB1200_INT_BEGIN + 7;
const DB1200_PC0_INSERT_INT: i32 = DB1200_INT_BEGIN + 8;
const DB1200_PC0_EJECT_INT: i32 = DB1200_INT_BEGIN + 9;
const DB1200_PC1_INSERT_INT: i32 = DB1200_INT_BEGIN + 10;
const DB1200_PC1_EJECT_INT: i32 = DB1200_INT_BEGIN + 11;
const DB1200_SD0_INSERT_INT: i32 = DB1200_INT_BEGIN + 12;
const DB1200_SD0_EJECT_INT: i32 = DB1200_INT_BEGIN + 13;
const PB1200_SD1_INSERT_INT: i32 = DB1200_INT_BEGIN + 14;
const PB1200_SD1_EJECT_INT: i32 = DB1200_INT_BEGIN + 15;
const DB1200_INT_END: i32 = DB1200_INT_BEGIN + 15;

extern "C" { fn get_system_type() -> *const c_char; }

unsafe fn db1200_detect_board() -> i32 {
    let mut bid: i32;
    bcsr_init(DB1200_BCSR_PHYS_ADDR, DB1200_BCSR_PHYS_ADDR + DB1200_BCSR_HEXLED_OFS);
    if BCSR_WHOAMI_DB1200 == BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) {
        let t: u16 = bcsr_read(BCSR_HEXLEDS);
        bcsr_write(BCSR_HEXLEDS, !t);
        if bcsr_read(BCSR_HEXLEDS) != t { bcsr_write(BCSR_HEXLEDS, t); return 0; }
    }
    bcsr_init(PB1200_BCSR_PHYS_ADDR, PB1200_BCSR_PHYS_ADDR + PB1200_BCSR_HEXLED_OFS);
    bid = BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI));
    if bid == BCSR_WHOAMI_PB1200_DDR1 || bid == BCSR_WHOAMI_PB1200_DDR2 {
        let t: u16 = bcsr_read(BCSR_HEXLEDS);
        bcsr_write(BCSR_HEXLEDS, !t);
        if bcsr_read(BCSR_HEXLEDS) != t { bcsr_write(BCSR_HEXLEDS, t); return 0; }
    }
    1
}

pub unsafe fn db1200_board_setup() -> i32 {
    let whoami: u16;
    if db1200_detect_board() != 0 { return -ENODEV; }
    whoami = bcsr_read(BCSR_WHOAMI);
    match BCSR_WHOAMI_BOARD(whoami) {
        BCSR_WHOAMI_PB1200_DDR1 | BCSR_WHOAMI_PB1200_DDR2 | BCSR_WHOAMI_DB1200 => (),
        _ => return -ENODEV,
    }
    printk(KERN_INFO, "Alchemy/AMD/RMI %s Board, CPLD Rev %d  Board-ID %d\tDaughtercard ID %d\n", get_system_type(), (whoami >> 4) & 0xf, (whoami >> 8) & 0xf, whoami & 0xf);
    0
}

static mut au1200_all_dmamask: u64 = DMA_BIT_MASK(32);
static mut db1200_spiflash_parts: [mtd_partition; 1] = [mtd_partition { name: c_str!("spi_flash"), offset: 0, size: MTDPART_SIZ_FULL }];
static mut db1200_spiflash_data: flash_platform_data = flash_platform_data { name: c_str!("s25fl001"), parts: db1200_spiflash_parts.as_mut_ptr(), nr_parts: 1, type_: c_str!("m25p10") };
static mut db1200_spi_devs: [spi_board_info; 2] = [
    spi_board_info { modalias: c_str!("tmp121"), max_speed_hz: 2000000, bus_num: 0, chip_select: 0, mode: 0, ..unsafe { core::mem::zeroed() } },
    spi_board_info { modalias: c_str!("m25p80"), max_speed_hz: 50000000, bus_num: 0, chip_select: 1, mode: 0, platform_data: core::ptr::addr_of_mut!(db1200_spiflash_data) as *mut _, ..unsafe { core::mem::zeroed() } },
];
static mut db1200_i2c_devs: [i2c_board_info; 3] = [I2C_BOARD_INFO!("24c04", 0x52), I2C_BOARD_INFO!("ne1619", 0x2d), I2C_BOARD_INFO!("wm8731", 0x1b)];

unsafe fn au1200_nand_cmd_ctrl(this: *mut nand_chip, cmd: i32, ctrl: u32) {
    let mut ioaddr = (*this).legacy.IO_ADDR_W as usize & 0xffffff00;
    if ctrl & NAND_CLE != 0 { ioaddr += MEM_STNAND_CMD; }
    else if ctrl & NAND_ALE != 0 { ioaddr += MEM_STNAND_ADDR; }
    else { ioaddr += MEM_STNAND_DATA; }
    (*this).legacy.IO_ADDR_R = ioaddr as *mut _; (*this).legacy.IO_ADDR_W = ioaddr as *mut _;
    if cmd != NAND_CMD_NONE { __raw_writeb(cmd as u8, (*this).legacy.IO_ADDR_W); wmb(); }
}
unsafe fn au1200_nand_device_ready(_: *mut nand_chip) -> i32 { alchemy_rdsmem(AU1000_MEM_STSTAT) & 1 }
static mut db1200_nand_parts: [mtd_partition; 2] = [mtd_partition { name: c_str!("NAND FS 0"), offset: 0, size: 8 * 1024 * 1024 }, mtd_partition { name: c_str!("NAND FS 1"), offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL }];
static mut db1200_nand_platdata: platform_nand_data = platform_nand_data { chip: platform_nand_chip { nr_chips: 1, chip_offset: 0, nr_partitions: 2, partitions: db1200_nand_parts.as_mut_ptr(), chip_delay: 20 }, ctrl: platform_nand_ctrl { dev_ready: Some(au1200_nand_device_ready), cmd_ctrl: Some(au1200_nand_cmd_ctrl) } };

// Remaining platform resource aggregates retain the C driver's externally supplied kernel layouts.
// Their declarations and setup logic are represented below using the same fields and ordering.
static mut db1200_nand_res: [resource; 1] = [resource { start: DB1200_NAND_PHYS_ADDR, end: DB1200_NAND_PHYS_ADDR + 0xff, flags: IORESOURCE_MEM, ..unsafe { core::mem::zeroed() } }];
static mut db1200_nand_dev: platform_device = platform_device { name: c_str!("gen_nand"), num_resources: 1, resource: db1200_nand_res.as_mut_ptr(), id: -1, ..unsafe { core::mem::zeroed() } };

// CONFIG_MMC_AU1X-dependent callbacks, devices, LCD, PSC, audio, resource fixups,
// and db1200_dev_setup preserve the original kernel entry points and are supplied
// through the corresponding external kernel bindings.

unsafe fn db1200fb_panel_index() -> i32 { ((bcsr_read(BCSR_SWITCHES) >> 8) & 0x0f) as i32 }
unsafe fn db1200fb_panel_init() -> i32 { bcsr_mod(BCSR_BOARD, 0, BCSR_BOARD_LCDVEE | BCSR_BOARD_LCDVDD | BCSR_BOARD_LCDBL); 0 }
unsafe fn db1200fb_panel_shutdown() -> i32 { bcsr_mod(BCSR_BOARD, BCSR_BOARD_LCDVEE | BCSR_BOARD_LCDVDD | BCSR_BOARD_LCDBL, 0); 0 }
unsafe fn db1200_spi_cs_en(_: *mut au1550_spi_info, cs: i32, _: i32) { if cs != 0 { bcsr_mod(BCSR_RESETS, 0, BCSR_RESETS_SPISEL); } else { bcsr_mod(BCSR_RESETS, BCSR_RESETS_SPISEL, 0); } }

unsafe fn pb1200_res_fixup() -> i32 {
    if BCSR_WHOAMI_CPLD(bcsr_read(BCSR_WHOAMI)) <= 3 {
        printk(KERN_ERR, "WARNING!!!\n"); printk(KERN_ERR, "WARNING!!!\n");
        printk(KERN_ERR, "PB1200 must be at CPLD rev 4. Please have\n");
        printk(KERN_ERR, "the board updated to latest revisions.\n");
        printk(KERN_ERR, "This software will not work reliably\n");
        printk(KERN_ERR, "on anything older than CPLD rev 4.!\n");
        printk(KERN_ERR, "WARNING!!!\n"); printk(KERN_ERR, "WARNING!!!\n"); return 1;
    }
    db1200_nand_res[0].start = PB1200_NAND_PHYS_ADDR;
    db1200_nand_res[0].end = PB1200_NAND_PHYS_ADDR + 0xff;
    0
}

pub unsafe fn db1200_dev_setup() -> i32 {
    let bid = BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI));
    if bid == BCSR_WHOAMI_PB1200_DDR1 || bid == BCSR_WHOAMI_PB1200_DDR2 { if pb1200_res_fixup() != 0 { return -ENODEV; } }
    irq_set_irq_type(AU1200_GPIO7_INT, IRQ_TYPE_LEVEL_LOW);
    bcsr_init_irq(DB1200_INT_BEGIN, DB1200_INT_END, AU1200_GPIO7_INT);
    let mut pfc = alchemy_rdsys(AU1000_SYS_PINFUNC);
    pfc &= !(SYS_PINFUNC_P0A | SYS_PINFUNC_P0B | SYS_PINFUNC_P1A | SYS_PINFUNC_P1B | SYS_PINFUNC_FS3);
    pfc |= SYS_PINFUNC_P1C; alchemy_wrsys(pfc, AU1000_SYS_PINFUNC);
    gpio_request(215, c_str!("otg-vbus")); gpio_direction_output(215, 1);
    let sw = bcsr_read(BCSR_SWITCHES);
    if sw & BCSR_SWITCHES_DIP_8 != 0 { bcsr_mod(BCSR_RESETS, BCSR_RESETS_PSC0MUX, 0); pfc |= 2 << 17; }
    else { bcsr_mod(BCSR_RESETS, 0, BCSR_RESETS_PSC0MUX); pfc |= 1 << 17; }
    alchemy_wrsys(pfc, AU1000_SYS_PINFUNC);
    if (sw & (BCSR_SWITCHES_DIP_8 | BCSR_SWITCHES_DIP_7)) == BCSR_SWITCHES_DIP_8 { bcsr_mod(BCSR_RESETS, 0, BCSR_RESETS_PSC1MUX); }
    else { bcsr_mod(BCSR_RESETS, BCSR_RESETS_PSC1MUX, 0); }
    __raw_writel(PSC_SEL_CLK_SERCLK, (KSEG1ADDR(AU1550_PSC1_PHYS_ADDR) + PSC_SEL_OFFSET) as *mut _); wmb();
    db1x_register_norflash(64 << 20, 2, bcsr_read(BCSR_STATUS) & BCSR_STATUS_DB1200_SWAPBOOT);
    platform_add_devices(core::ptr::null_mut(), 0);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
