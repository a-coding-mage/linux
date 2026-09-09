// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/magicpanel/setup.c
 *
 *  Copyright (C) 2007  Markus Brunner, Mark Jonas
 *
 *  Magic Panel Release 2 board setup
 */

// Linux headers and board-specific symbols are supplied by the surrounding build.

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn __raw_writew(value: u16, addr: usize);
    fn __raw_writeb(value: u8, addr: usize);
    fn mdelay(ms: u32);
    fn udelay(us: u32);
    fn gpio_request(gpio: u32, label: *const core::ffi::c_char) -> i32;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn evt2irq(event: u32) -> u32;
    fn regulator_register_fixed(id: i32, supplies: *mut RegulatorConsumerSupply, count: usize) -> i32;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
    fn plat_irq_setup_pins(mode: u32);
    fn irq_set_irq_type(irq: u32, kind: u32);
    fn intc_set_priority(irq: u32, priority: u32);
}

#[repr(C)]
struct RegulatorConsumerSupply { supply: *const core::ffi::c_char, dev_name: *const core::ffi::c_char }
#[repr(C)]
struct Resource { start: usize, end: usize, flags: u32 }
#[repr(C)]
struct Device { platform_data: *mut core::ffi::c_void }
#[repr(C)]
struct PlatformDevice { name: *const core::ffi::c_char, id: i32, num_resources: usize, resource: *mut Resource, dev: Device }
#[repr(C)]
struct Smsc911xPlatformConfig { phy_interface: u32, irq_polarity: u32, irq_type: u32, flags: u32 }
#[repr(C)]
struct HeartbeatData { flags: u32 }
#[repr(C)]
struct MtdPartition { name: *const core::ffi::c_char, offset: usize, size: usize, mask_flags: u32 }
#[repr(C)]
struct PhysmapFlashData { parts: *mut MtdPartition, nr_parts: usize, width: u32 }
#[repr(C)]
struct ShMachineVector { mv_name: *const core::ffi::c_char, mv_setup: Option<unsafe extern "C" fn(*mut *mut core::ffi::c_char),> , mv_init_irq: Option<unsafe extern "C" fn()> }

const IORESOURCE_MEM: u32 = 0x0000_0200;
const IORESOURCE_IRQ: u32 = 0x0000_0400;
const MTD_WRITEABLE: u32 = 0x400;
const MTDPART_OFS_NXTBLK: usize = 0;ÿff_ffff;
const MTDPART_SIZ_FULL: usize = 0;ÿff_ffff;
const IRQ_MODE_IRQ: u32 = 0;
const IRQ_TYPE_LEVEL_LOW: u32 = 1;
const IRQ_TYPE_EDGE_RISING: u32 = 2;
const IRQ_TYPE_EDGE_FALLING: u32 = 3;
const HEARTBEAT_INVERTED: u32 = 1;
const PHY_INTERFACE_MODE_MII: u32 = 0;
const SMSC911X_IRQ_POLARITY_ACTIVE_LOW: u32 = 0;
const SMSC911X_IRQ_TYPE_OPEN_DRAIN: u32 = 0;
const SMSC911X_USE_32BIT: u32 = 1;

extern "C" {
    static mut PORT_PMDR: usize;
    static mut CS2BCR: usize; static mut CS2WCR: usize;
    static mut CS4BCR: usize; static mut CS4WCR: usize;
    static mut CS5ABCR: usize; static mut CS5AWCR: usize;
    static mut CS5BBCR: usize; static mut CS5BWCR: usize;
    static mut CS6ABCR: usize; static mut CS6AWCR: usize;
    static mut PORT_PACR: usize; static mut PORT_PBCR: usize; static mut PORT_PCCR: usize;
    static mut PORT_PDCR: usize; static mut PORT_PECR: usize; static mut PORT_PFCR: usize;
    static mut PORT_PGCR: usize; static mut PORT_PHCR: usize; static mut PORT_PJCR: usize;
    static mut PORT_PKCR: usize; static mut PORT_PLCR: usize; static mut PORT_PMCR: usize;
    static mut PORT_PPCR: usize; static mut PORT_PPDR: usize; static mut PORT_PSCR: usize;
    static mut PORT_PTCR: usize; static mut PORT_PUCR: usize; static mut PORT_PVCR: usize;
    static mut PORT_PSELA: usize; static mut PORT_PSELB: usize; static mut PORT_PSELC: usize;
    static mut PORT_PSELD: usize; static mut PORT_UTRCTL: usize; static mut PORT_UCLKCR_W: usize;
    static mut PA_LED: usize;
    static mut CONFIG_SH_MAGIC_PANEL_R2_VERSION: i32;
    static mut MPR2_MTD_BOOTLOADER_SIZE: usize; static mut MPR2_MTD_KERNEL_SIZE: usize;
    static mut GPIO_FN_A25: u32; static mut GPIO_FN_A24: u32; static mut GPIO_FN_A23: u32;
    static mut GPIO_FN_A22: u32; static mut GPIO_FN_A21: u32; static mut GPIO_FN_A20: u32;
    static mut GPIO_FN_A19: u32; static mut GPIO_FN_A0: u32;
}

static mut DUMMY_SUPPLIES: [RegulatorConsumerSupply; 2] = [
    RegulatorConsumerSupply { supply: b"vddvario\0".as_ptr() as _, dev_name: b"smsc911x\0".as_ptr() as _ },
    RegulatorConsumerSupply { supply: b"vdd33a\0".as_ptr() as _, dev_name: b"smsc911x\0".as_ptr() as _ },
];

unsafe fn ethernet_reset_finished() -> i32 {
    if (__raw_readl(0xA8000084) & 1) != 0 { return 1; }
    for _ in 0..10 { mdelay(10); if (__raw_readl(0xA8000084) & 1) != 0 { return 1; } }
    0
}

unsafe fn reset_ethernet() { __raw_writeb(__raw_readl(PORT_PMDR) as u8 & !0x10, PORT_PMDR); udelay(200); __raw_writeb(__raw_readl(PORT_PMDR) as u8 | 0x10, PORT_PMDR); }

unsafe fn setup_chip_select() {
    __raw_writel(0x36db0400, CS2BCR); __raw_writel(0x000003c0, CS2WCR);
    __raw_writel(0x00000200, CS4BCR); __raw_writel(0x00100981, CS4WCR);
    __raw_writel(0x00000200, CS5ABCR); __raw_writel(0x00100981, CS5AWCR);
    __raw_writel(0x00000200, CS5BBCR); __raw_writel(0x00100981, CS5BWCR);
    __raw_writel(0x00000200, CS6ABCR); __raw_writel(0x001009C1, CS6AWCR);
}

unsafe fn setup_port_multiplexing() {
    __raw_writew(0x5555, PORT_PACR); __raw_writew(0x5555, PORT_PBCR); __raw_writew(0x5500, PORT_PCCR);
    __raw_writew(0x5555, PORT_PDCR); __raw_writew(0x3C00, PORT_PECR); __raw_writew(0x0002, PORT_PFCR);
    __raw_writew(0x03D5, PORT_PGCR); __raw_writew(0x0050, PORT_PHCR); __raw_writew(0x0000, PORT_PJCR);
    __raw_writew(0x00FF, PORT_PKCR); __raw_writew(0x0000, PORT_PLCR); __raw_writew(0x5552, PORT_PMCR);
    let v = if CONFIG_SH_MAGIC_PANEL_R2_VERSION == 2 { 0x30 } else if CONFIG_SH_MAGIC_PANEL_R2_VERSION == 3 { 0xF0 } else { 0 };
    __raw_writeb(v, PORT_PMDR); __raw_writew(0x0100, PORT_PPCR); __raw_writeb(0x10, PORT_PPDR);
    for p in [GPIO_FN_A25, GPIO_FN_A24, GPIO_FN_A23, GPIO_FN_A22, GPIO_FN_A21, GPIO_FN_A20, GPIO_FN_A19, GPIO_FN_A0] { gpio_request(p, core::ptr::null()); }
    __raw_writew(0x0140, PORT_PSCR); __raw_writew(0x0001, PORT_PTCR); __raw_writew(0x0240, PORT_PUCR); __raw_writew(0x0142, PORT_PVCR);
}

unsafe extern "C" fn mpr2_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    __raw_writew(0xAABC, PORT_PSELA); __raw_writew(0x3C00, PORT_PSELB); __raw_writew(0, PORT_PSELC); __raw_writew(0, PORT_PSELD);
    __raw_writew(0x0101, PORT_UTRCTL); __raw_writew(0xA5C0, PORT_UCLKCR_W); setup_chip_select(); setup_port_multiplexing(); reset_ethernet();
    if ethernet_reset_finished() == 0 { printk(b"Ethernet not ready\n\0".as_ptr() as _); }
}

unsafe extern "C" fn init_mpr2_IRQ() {
    plat_irq_setup_pins(IRQ_MODE_IRQ);
    irq_set_irq_type(evt2irq(0x600), IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(evt2irq(0x620), IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(evt2irq(0x640), IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(evt2irq(0x660), IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(evt2irq(0x680), IRQ_TYPE_EDGE_RISING); irq_set_irq_type(evt2irq(0x6a0), IRQ_TYPE_EDGE_FALLING);
    intc_set_priority(evt2irq(0x600), 13); intc_set_priority(evt2irq(0x620), 13); intc_set_priority(evt2irq(0x640), 13); intc_set_priority(evt2irq(0x660), 6);
}

#[no_mangle]
pub static mut MV_MPR2: ShMachineVector = ShMachineVector { mv_name: b"mpr2\0".as_ptr() as _, mv_setup: Some(mpr2_setup), mv_init_irq: Some(init_mpr2_IRQ) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
