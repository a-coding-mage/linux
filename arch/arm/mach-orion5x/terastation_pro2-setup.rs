// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Buffalo Terastation Pro II/Live Board Setup
 *
 * Maintainer: Sylver Bruneau <sylver.bruneau@googlemail.com>
 */

// External kernel headers and platform definitions are supplied by the surrounding build.

const TSP2_NOR_BOOT_BASE: usize = 0xf4000000;
const TSP2_NOR_BOOT_SIZE: usize = SZ_256K;

static mut TSP2_NOR_FLASH_DATA: physmap_flash_data = physmap_flash_data { width: 1 };
static mut TSP2_NOR_FLASH_RESOURCE: resource = resource {
    flags: IORESOURCE_MEM,
    start: TSP2_NOR_BOOT_BASE,
    end: TSP2_NOR_BOOT_BASE + TSP2_NOR_BOOT_SIZE - 1,
};
static mut TSP2_NOR_FLASH: platform_device = platform_device {
    name: "physmap-flash",
    id: 0,
    dev: device { platform_data: unsafe { &mut TSP2_NOR_FLASH_DATA as *mut _ } },
    num_resources: 1,
    resource: unsafe { &mut TSP2_NOR_FLASH_RESOURCE as *mut _ },
};

const TSP2_PCI_SLOT0_OFFS: u8 = 7;
const TSP2_PCI_SLOT0_IRQ_PIN: i32 = 11;

unsafe fn tsp2_pci_preinit() {
    let pin = TSP2_PCI_SLOT0_IRQ_PIN;
    if gpio_request(pin, "PCI Int1") == 0 {
        if gpio_direction_input(pin) == 0 {
            irq_set_irq_type(gpio_to_irq(pin), IRQ_TYPE_LEVEL_LOW);
        } else {
            printk!(KERN_ERR, "tsp2_pci_preinit failed to set_irq_type pin %d\n", pin);
            gpio_free(pin);
        }
    } else {
        printk!(KERN_ERR, "tsp2_pci_preinit failed to gpio_request %d\n", pin);
    }
}

unsafe fn tsp2_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 { return irq; }
    if slot == TSP2_PCI_SLOT0_OFFS { return gpio_to_irq(TSP2_PCI_SLOT0_IRQ_PIN); }
    -1
}

static mut TSP2_PCI: hw_pci = hw_pci {
    nr_controllers: 2,
    preinit: Some(tsp2_pci_preinit),
    setup: Some(orion5x_pci_sys_setup),
    scan: Some(orion5x_pci_sys_scan_bus),
    map_irq: Some(tsp2_pci_map_irq),
};

unsafe fn tsp2_pci_init() -> i32 {
    if machine_is_terastation_pro2() { pci_common_init(&mut TSP2_PCI); }
    0
}

subsys_initcall!(tsp2_pci_init);

static mut TSP2_ETH_DATA: mv643xx_eth_platform_data = mv643xx_eth_platform_data { phy_addr: 0 };

const TSP2_RTC_GPIO: i32 = 9;
static mut TSP2_I2C_RTC: i2c_board_info = I2C_BOARD_INFO!("rs5c372a", 0x32);

const fn uart1_reg(x: usize) -> usize { UART1_VIRT_BASE + ((x) << 2) }

unsafe fn tsp2_miconread(buf: *mut u8, count: i32) -> i32 {
    let mut i = 0;
    while i < count {
        let mut timeout = 10;
        while (readl(uart1_reg(UART_LSR)) & UART_LSR_DR) == 0 {
            timeout -= 1;
            if timeout == 0 { break; }
            udelay(1000);
        }
        if timeout == 0 { break; }
        *buf.add(i as usize) = readl(uart1_reg(UART_RX)) as u8;
        i += 1;
    }
    i
}

unsafe fn tsp2_miconwrite(buf: *const u8, mut count: i32) -> i32 {
    let mut i = 0;
    while count > 0 {
        while (readl(uart1_reg(UART_LSR)) & UART_LSR_THRE) == 0 { barrier(); }
        writel(*buf.add(i as usize) as u32, uart1_reg(UART_TX));
        i += 1;
        count -= 1;
    }
    0
}

unsafe fn tsp2_miconsend(data: *const u8, count: i32) -> i32 {
    let mut checksum: u8 = 0;
    for i in 0..count { checksum = checksum.wrapping_sub(*data.add(i as usize)); }
    let mut recv_buf = [0u8; 40];
    let mut send_buf = [0u8; 40];
    let mut correct_ack = [0u8; 3];
    let mut retry = 2;
    loop {
        tsp2_miconwrite(data, count);
        tsp2_miconwrite(&checksum, 1);
        if tsp2_miconread(recv_buf.as_mut_ptr(), recv_buf.len() as i32) <= 3 {
            printk!(KERN_ERR, ">%s: receive failed.\n", "tsp2_miconsend");
            send_buf.fill(0xff);
            tsp2_miconwrite(send_buf.as_ptr(), send_buf.len() as i32);
            mdelay(100);
            tsp2_miconread(recv_buf.as_mut_ptr(), recv_buf.len() as i32);
        } else {
            correct_ack[0] = 0x01;
            correct_ack[1] = *data.add(1);
            correct_ack[2] = 0x00;
            if (recv_buf[0] as u32 + recv_buf[1] as u32 + recv_buf[2] as u32 + recv_buf[3] as u32) & 0xff != 0 {
                printk!(KERN_ERR, ">%s: Checksum Error : Received data[%02x, %02x, %02x, %02x]\n", "tsp2_miconsend", recv_buf[0], recv_buf[1], recv_buf[2], recv_buf[3]);
            } else if correct_ack == recv_buf[..3] {
                mdelay(10);
                return 0;
            }
            printk!(KERN_ERR, ">%s: Error : NAK or Illegal Data Received\n", "tsp2_miconsend");
        }
        if retry == 0 { break; }
        retry -= 1;
    }
    mdelay(10);
    -1
}

unsafe fn tsp2_power_off() {
    let watchdogkill = [0x01u8, 0x35, 0x00];
    let shutdownwait = [0x00u8, 0x0c];
    let poweroff = [0x00u8, 0x06];
    let divisor = (orion5x_tclk + (8 * 38400)) / (16 * 38400);
    pr_info!("%s: triggering power-off...\n", "tsp2_power_off");
    writel(0x83, uart1_reg(UART_LCR));
    writel(divisor & 0xff, uart1_reg(UART_DLL));
    writel((divisor >> 8) & 0xff, uart1_reg(UART_DLM));
    writel(0x1b, uart1_reg(UART_LCR)); writel(0x00, uart1_reg(UART_IER));
    writel(0x07, uart1_reg(UART_FCR)); writel(0x00, uart1_reg(UART_MCR));
    tsp2_miconsend(watchdogkill.as_ptr(), watchdogkill.len() as i32);
    tsp2_miconsend(shutdownwait.as_ptr(), shutdownwait.len() as i32);
    tsp2_miconsend(poweroff.as_ptr(), poweroff.len() as i32);
}

static mut TSP2_MPP_MODES: [u32; 21] = [MPP0_PCIE_RST_OUTn, MPP1_UNUSED, MPP2_UNUSED, MPP3_UNUSED, MPP4_NAND, MPP5_NAND, MPP6_NAND, MPP7_NAND, MPP8_GPIO, MPP9_GPIO, MPP10_UNUSED, MPP11_GPIO, MPP12_UNUSED, MPP13_GPIO, MPP14_GPIO, MPP15_UNUSED, MPP16_UART, MPP17_UART, MPP18_UART, MPP19_UART, 0];

unsafe fn tsp2_init() {
    orion5x_init();
    orion5x_mpp_conf(TSP2_MPP_MODES.as_ptr());
    mvebu_mbus_add_window_by_id(ORION_MBUS_DEVBUS_BOOT_TARGET, ORION_MBUS_DEVBUS_BOOT_ATTR, TSP2_NOR_BOOT_BASE, TSP2_NOR_BOOT_SIZE);
    platform_device_register(&mut TSP2_NOR_FLASH);
    orion5x_ehci0_init(); orion5x_eth_init(&mut TSP2_ETH_DATA); orion5x_i2c_init();
    orion5x_uart0_init(); orion5x_uart1_init();
    if gpio_request(TSP2_RTC_GPIO, "rtc") == 0 {
        if gpio_direction_input(TSP2_RTC_GPIO) == 0 { TSP2_I2C_RTC.irq = gpio_to_irq(TSP2_RTC_GPIO); }
        else { gpio_free(TSP2_RTC_GPIO); }
    }
    if TSP2_I2C_RTC.irq == 0 { pr_warn!("tsp2_init: failed to get RTC IRQ\n"); }
    i2c_register_board_info(0, &mut TSP2_I2C_RTC, 1);
    register_platform_power_off(Some(tsp2_power_off));
}

// MACHINE_START(TERASTATION_PRO2, "Buffalo Terastation Pro II/Live")
// Maintainer: Sylver Bruneau <sylver.bruneau@googlemail.com>
// .atag_offset = 0x100, .nr_irqs = ORION5X_NR_IRQS, .init_machine = tsp2_init,
// .map_io = orion5x_map_io, .init_early = orion5x_init_early,
// .init_irq = orion5x_init_irq, .init_time = orion5x_timer_init,
// .fixup = tag_fixup_mem32, .restart = orion5x_restart
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
