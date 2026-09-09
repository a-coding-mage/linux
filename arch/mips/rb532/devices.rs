// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  RouterBoard 500 Platform devices
 *
 *  Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
 *  Copyright (C) 2007 Florian Fainelli <florian@openwrt.org>
 */
// C dependencies: linux/kernel.h, linux/export.h, linux/hex.h, linux/init.h,
// linux/ctype.h, linux/string.h, linux/platform_device.h, linux/mtd/platnand.h,
// linux/mtd/mtd.h, linux/gpio/machine.h, linux/gpio/property.h,
// linux/gpio_keys.h, linux/input.h, linux/property.h, linux/serial_8250.h,
// asm/bootinfo.h, and asm/mach-rc32434 headers.

const ETH0_RX_DMA_ADDR: usize = DMA0_BASE_ADDR + 0 * DMA_CHAN_OFFSET;
const ETH0_TX_DMA_ADDR: usize = DMA0_BASE_ADDR + 1 * DMA_CHAN_OFFSET;

extern "C" {
    static mut idt_cpu_freq: c_uint;
}

static mut dev3: mpmc_device = mpmc_device::default();

static rb532_gpio0_node: software_node = software_node { name: "gpio0" };

#[no_mangle]
pub unsafe extern "C" fn set_latch_u5(or_mask: u8, nand_mask: u8) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut dev3.lock, &mut flags);
    dev3.state = (dev3.state | or_mask) & !nand_mask;
    writeb(dev3.state, dev3.base);
    spin_unlock_irqrestore(&mut dev3.lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn get_latch_u5() -> u8 {
    dev3.state
}

static mut korina_dev0_res: [resource; 5] = [
    resource { name: "emac", start: ETH0_BASE_ADDR, end: ETH0_BASE_ADDR + core::mem::size_of::<eth_regs>(), flags: IORESOURCE_MEM },
    resource { name: "rx", start: ETH0_DMA_RX_IRQ, end: ETH0_DMA_RX_IRQ, flags: IORESOURCE_IRQ },
    resource { name: "tx", start: ETH0_DMA_TX_IRQ, end: ETH0_DMA_TX_IRQ, flags: IORESOURCE_IRQ },
    resource { name: "dma_rx", start: ETH0_RX_DMA_ADDR, end: ETH0_RX_DMA_ADDR + DMA_CHAN_OFFSET - 1, flags: IORESOURCE_MEM },
    resource { name: "dma_tx", start: ETH0_TX_DMA_ADDR, end: ETH0_TX_DMA_ADDR + DMA_CHAN_OFFSET - 1, flags: IORESOURCE_MEM },
];

static mut korina_dev0_data: korina_device = korina_device { name: "korina0", mac: [0xde, 0xca, 0xff, 0xc0, 0xff, 0xee] };
static mut korina_dev0: platform_device = platform_device { id: -1, name: "korina", resource: korina_dev0_res.as_ptr(), num_resources: 5, dev: platform_device_dev { platform_data: unsafe { (&korina_dev0_data.mac as *const _) as *mut c_void } } };

static mut cf_slot0_res: [resource; 2] = [
    resource { name: "cf_membase", start: 0, end: 0, flags: IORESOURCE_MEM },
    resource { name: "cf_irq", start: 8 + 4 * 32 + CF_GPIO_NUM, end: 8 + 4 * 32 + CF_GPIO_NUM, flags: IORESOURCE_IRQ },
];
static mut cf_slot0_gpio_table: gpiod_lookup_table = gpiod_lookup_table { dev_id: "pata-rb532-cf", table: [GPIO_LOOKUP("gpio0", CF_GPIO_NUM, core::ptr::null(), GPIO_ACTIVE_HIGH), GPIO_LOOKUP_END] };
static mut cf_slot0: platform_device = platform_device { id: -1, name: "pata-rb532-cf", resource: cf_slot0_res.as_ptr(), num_resources: 2, ..platform_device::zeroed() };

unsafe extern "C" fn rb532_cmd_ctrl(chip: *mut nand_chip, cmd: c_int, ctrl: c_uint) {
    let mut orbits: u8;
    let mut nandbits: u8;
    if ctrl & NAND_CTRL_CHANGE != 0 {
        orbits = ((ctrl & NAND_CLE) << 1) as u8;
        orbits |= ((ctrl & NAND_ALE) >> 1) as u8;
        nandbits = ((!ctrl & NAND_CLE) << 1) as u8;
        nandbits |= ((!ctrl & NAND_ALE) >> 1) as u8;
        set_latch_u5(orbits, nandbits);
    }
    if cmd != NAND_CMD_NONE {
        writeb(cmd as u8, (*chip).legacy.IO_ADDR_W);
    }
}

static mut nand_slot0_res: [resource; 1] = [resource { name: "nand_membase", start: 0, end: 0, flags: IORESOURCE_MEM }];
static mut rb532_nand_data: platform_nand_data = platform_nand_data { ctrl: platform_nand_ctrl { cmd_ctrl: Some(rb532_cmd_ctrl) }, ..platform_nand_data::zeroed() };
static nand0_properties: [property_entry; 2] = [PROPERTY_ENTRY_GPIO!("ready-gpios", &rb532_gpio0_node, GPIO_RDY, GPIO_ACTIVE_HIGH), property_entry::empty()];
static mut rb532_partition_info: [mtd_partition; 2] = [
    mtd_partition { name: "Routerboard NAND boot", offset: 0, size: 4 * 1024 * 1024 },
    mtd_partition { name: "rootfs", offset: MTDPART_OFS_NXTBLK, size: MTDPART_SIZ_FULL },
];
static mut rb532_led: platform_device = platform_device { name: "rb532-led", id: -1, ..platform_device::zeroed() };
static mut rb532_wdt_res: [resource; 1] = [resource { name: "rb532_wdt_res", start: INTEG0_BASE_ADDR, end: INTEG0_BASE_ADDR + core::mem::size_of::<integ>(), flags: IORESOURCE_MEM }];
static mut rb532_wdt: platform_device = platform_device { name: "rc32434_wdt", id: -1, resource: rb532_wdt_res.as_ptr(), num_resources: 1, ..platform_device::zeroed() };
static mut rb532_uart_res: [plat_serial8250_port; 2] = [
    plat_serial8250_port { type_: PORT_16550A, mapbase: REGBASE + UART0BASE, mapsize: 0x1000, irq: UART0_IRQ, regshift: 2, iotype: UPIO_MEM, flags: UPF_BOOT_AUTOCONF | UPF_IOREMAP, ..plat_serial8250_port::zeroed() },
    plat_serial8250_port { flags: 0, ..plat_serial8250_port::zeroed() },
];
static mut rb532_uart: platform_device = platform_device { name: "serial8250", id: PLAT8250_DEV_PLATFORM, dev: platform_device_dev { platform_data: rb532_uart_res.as_mut_ptr() as *mut c_void } };
static mut rb532_devs: [*mut platform_device; 5] = [&mut korina_dev0, &mut cf_slot0, &mut rb532_led, &mut rb532_uart, &mut rb532_wdt];

const GPIOBASE: usize = 0x050000;
static mut rb532_gpio_reg0_res: [resource; 1] = [resource { name: "gpio_reg0", start: REGBASE + GPIOBASE, end: REGBASE + GPIOBASE + core::mem::size_of::<rb532_gpio_reg>() - 1, flags: IORESOURCE_MEM }];
static mut rb532_button_properties: [property_entry; 2] = [PROPERTY_ENTRY_GPIO!("button-gpios", &rb532_gpio0_node, GPIO_BTN_S1, GPIO_ACTIVE_LOW), property_entry::empty()];

const NAND_CHIP_DELAY: u32 = 25;

unsafe extern "C" fn rb532_nand_setup() {
    match mips_machtype {
        MACH_MIKROTIK_RB532A => set_latch_u5(LO_FOFF | LO_CEX, LO_ULED | LO_ALE | LO_CLE | LO_WPX),
        _ => set_latch_u5(LO_WPX | LO_FOFF | LO_CEX, LO_ULED | LO_ALE | LO_CLE),
    }
    rb532_nand_data.chip.nr_chips = 1;
    rb532_nand_data.chip.nr_partitions = 2;
    rb532_nand_data.chip.partitions = rb532_partition_info.as_mut_ptr();
    rb532_nand_data.chip.chip_delay = NAND_CHIP_DELAY;
}

unsafe extern "C" fn plat_setup_devices() -> c_int {
    if readl(IDT434_REG_BASE + DEV1MASK) == 0 { rb532_devs[2] = core::ptr::null_mut(); } else { cf_slot0_res[0].start = readl(IDT434_REG_BASE + DEV1BASE); cf_slot0_res[0].end = cf_slot0_res[0].start + 0x1000; }
    nand_slot0_res[0].start = readl(IDT434_REG_BASE + DEV2BASE);
    nand_slot0_res[0].end = nand_slot0_res[0].start + 0x1000;
    dev3.base = ioremap(readl(IDT434_REG_BASE + DEV3BASE), 1);
    if dev3.base.is_null() { printk(KERN_ERR, "rb532: cannot remap device controller 3\n"); return -ENXIO; }
    rb532_nand_setup();
    rb532_uart_res[0].uartclk = idt_cpu_freq;
    let mut pd = platform_device_register_full(&rb532_gpio_devinfo);
    let mut ret = PTR_ERR_OR_ZERO(pd);
    if ret != 0 { pr_err!("failed to create the GPIO device: %d\n", ret); return ret; }
    gpiod_add_lookup_table(&mut cf_slot0_gpio_table);
    ret = platform_add_devices(rb532_devs.as_mut_ptr(), 5);
    if ret != 0 { return ret; }
    pd = platform_device_register_full(&nand0_info); ret = PTR_ERR_OR_ZERO(pd);
    if ret != 0 { pr_err!("failed to create NAND slot0 device: %d\n", ret); return ret; }
    pd = platform_device_register_full(&rb532_button_info); ret = PTR_ERR_OR_ZERO(pd);
    if ret != 0 { pr_err!("failed to create RB532 button device: %d\n", ret); return ret; }
    0
}

// #ifdef CONFIG_NET
unsafe extern "C" fn setup_kmac(s: *mut c_char) -> c_int { printk(KERN_INFO, "korina mac = %s\n", s); if !mac_pton(s, korina_dev0_data.mac.as_mut_ptr()) { printk(KERN_ERR, "Invalid mac\n"); } 1 }
// __setup("kmac=", setup_kmac);
// #endif /* CONFIG_NET */
// arch_initcall(plat_setup_devices);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
