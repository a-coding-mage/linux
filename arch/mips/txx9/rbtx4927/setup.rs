/* Toshiba rbtx4927 specific setup. Rust translation of setup.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// platform bindings.

#[cfg(feature = "CONFIG_PCI")]
unsafe fn tx4927_pci_setup() {
    let extarb = !((__raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_PCIARB) != 0);
    let c = &mut txx9_primary_pcic;
    register_pci_controller(c);
    if __raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_PCI66 != 0 {
        txx9_pci_option = (txx9_pci_option & !TXX9_PCI_OPT_CLK_MASK) | TXX9_PCI_OPT_CLK_66;
    }
    writeb(1, rbtx4927_pcireset_addr);
    txx9_set64(&mut (*tx4927_ccfgptr).clkctr, TX4927_CLKCTR_PCIRST);
    if (txx9_pci_option & TXX9_PCI_OPT_CLK_MASK) == TXX9_PCI_OPT_CLK_66 { tx4927_pciclk66_setup(); }
    mdelay(10);
    txx9_clear64(&mut (*tx4927_ccfgptr).clkctr, TX4927_CLKCTR_PCIRST);
    writeb(0, rbtx4927_pcireset_addr);
    iob();
    tx4927_report_pciclk();
    tx4927_pcic_setup(tx4927_pcicptr, c, extarb);
    if (txx9_pci_option & TXX9_PCI_OPT_CLK_MASK) == TXX9_PCI_OPT_CLK_AUTO && txx9_pci66_check(c, 0, 0) {
        writeb(1, rbtx4927_pcireset_addr);
        txx9_set64(&mut (*tx4927_ccfgptr).clkctr, TX4927_CLKCTR_PCIRST);
        tx4927_pciclk66_setup(); mdelay(10);
        txx9_clear64(&mut (*tx4927_ccfgptr).clkctr, TX4927_CLKCTR_PCIRST);
        writeb(0, rbtx4927_pcireset_addr); iob();
        tx4927_report_pciclk(); tx4927_pcic_setup(tx4927_pcicptr, c, extarb);
    }
    tx4927_setup_pcierr_irq();
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn tx4937_pci_setup() {
    let extarb = !((__raw_readq(&(*tx4938_ccfgptr).ccfg) & TX4938_CCFG_PCIARB) != 0);
    let c = &mut txx9_primary_pcic;
    register_pci_controller(c);
    if __raw_readq(&(*tx4938_ccfgptr).ccfg) & TX4938_CCFG_PCI66 != 0 { txx9_pci_option = (txx9_pci_option & !TXX9_PCI_OPT_CLK_MASK) | TXX9_PCI_OPT_CLK_66; }
    writeb(1, rbtx4927_pcireset_addr); txx9_set64(&mut (*tx4938_ccfgptr).clkctr, TX4938_CLKCTR_PCIRST);
    if (txx9_pci_option & TXX9_PCI_OPT_CLK_MASK) == TXX9_PCI_OPT_CLK_66 { tx4938_pciclk66_setup(); }
    mdelay(10); txx9_clear64(&mut (*tx4938_ccfgptr).clkctr, TX4938_CLKCTR_PCIRST); writeb(0, rbtx4927_pcireset_addr); iob();
    tx4938_report_pciclk(); tx4927_pcic_setup(tx4938_pcicptr, c, extarb);
    if (txx9_pci_option & TXX9_PCI_OPT_CLK_MASK) == TXX9_PCI_OPT_CLK_AUTO && txx9_pci66_check(c, 0, 0) {
        writeb(1, rbtx4927_pcireset_addr); txx9_set64(&mut (*tx4938_ccfgptr).clkctr, TX4938_CLKCTR_PCIRST); tx4938_pciclk66_setup(); mdelay(10);
        txx9_clear64(&mut (*tx4938_ccfgptr).clkctr, TX4938_CLKCTR_PCIRST); writeb(0, rbtx4927_pcireset_addr); iob();
        tx4938_report_pciclk(); tx4927_pcic_setup(tx4938_pcicptr, c, extarb);
    }
    tx4938_setup_pcierr_irq();
}
#[cfg(not(feature = "CONFIG_PCI"))] unsafe fn tx4927_pci_setup() {}
#[cfg(not(feature = "CONFIG_PCI"))] unsafe fn tx4937_pci_setup() {}

// GPIO_LOOKUP_SINGLE(sio_gpio_table, NULL, "TXx9", 15, "sio-dtr", GPIO_ACTIVE_HIGH)
static mut sio_gpio_table: gpiod_lookup_table = unsafe { core::mem::zeroed() };

unsafe fn rbtx4927_gpio_init() {
    gpiod_add_lookup_table(&mut sio_gpio_table);
    let d = gpiod_get(core::ptr::null_mut(), "sio-dtr", GPIOD_OUT_HIGH);
    if IS_ERR(d) { pr_err("Unable to get sio-dtr GPIO descriptor\n"); } else { gpiod_put(d); }
    tx4927_sio_init(0, 0);
}
unsafe fn rbtx4927_arch_init() { txx9_gpio_init(TX4927_PIO_REG & 0xfffffffff, TX4927_NUM_PIO); rbtx4927_gpio_init(); tx4927_pci_setup(); }
unsafe fn rbtx4937_arch_init() { txx9_gpio_init(TX4938_PIO_REG & 0xfffffffff, TX4938_NUM_PIO); rbtx4927_gpio_init(); tx4937_pci_setup(); }

unsafe fn toshiba_rbtx4927_restart(_command: *mut i8) {
    writeb(1, rbtx4927_softresetlock_addr);
    while readb(rbtx4927_softresetlock_addr) & 1 == 0 {}
    writeb(1, rbtx4927_softreset_addr);
    (_machine_halt)();
}
unsafe fn rbtx4927_mem_setup() {
    if TX4927_REV_PCODE() == 0x4927 { rbtx4927_clock_init(); tx4927_setup(); } else { rbtx4937_clock_init(); tx4938_setup(); }
    _machine_restart = toshiba_rbtx4927_restart;
    #[cfg(feature = "CONFIG_PCI")]
    { txx9_alloc_pci_controller(&mut txx9_primary_pcic, RBTX4927_PCIMEM, RBTX4927_PCIMEM_SIZE, RBTX4927_PCIIO, RBTX4927_PCIIO_SIZE); txx9_board_pcibios_setup = tx4927_pcibios_setup; }
    #[cfg(not(feature = "CONFIG_PCI"))]
    { set_io_port_base(KSEG1 + RBTX4927_ISA_IO_OFFSET); }
}
unsafe fn rbtx4927_clock_init() { match (__raw_readq(&(*tx4927_ccfgptr).ccfg) as u64) & TX4927_CCFG_PCIDIVMODE_MASK { TX4927_CCFG_PCIDIVMODE_2_5 | TX4927_CCFG_PCIDIVMODE_5 => txx9_cpu_clock = 166666666, _ => txx9_cpu_clock = 200000000 } }
unsafe fn rbtx4937_clock_init() { match (__raw_readq(&(*tx4938_ccfgptr).ccfg) as u64) & TX4938_CCFG_PCIDIVMODE_MASK { TX4938_CCFG_PCIDIVMODE_8 | TX4938_CCFG_PCIDIVMODE_4 => txx9_cpu_clock = 266666666, TX4938_CCFG_PCIDIVMODE_9 | TX4938_CCFG_PCIDIVMODE_4_5 => txx9_cpu_clock = 300000000, _ => txx9_cpu_clock = 333333333 } }
unsafe fn rbtx4927_time_init() { tx4927_time_init(0); }

unsafe fn toshiba_rbtx4927_rtc_init() { let res = resource { start: RBTX4927_BRAMRTC_BASE - IO_BASE, end: RBTX4927_BRAMRTC_BASE - IO_BASE + 0x800 - 1, flags: IORESOURCE_MEM }; platform_device_register_simple("rtc-ds1742", -1, &res, 1); }
unsafe fn rbtx4927_ne_init() { let res = [resource { start: RBTX4927_RTL_8019_BASE, end: RBTX4927_RTL_8019_BASE + 0x20 - 1, flags: IORESOURCE_IO }, resource { start: RBTX4927_RTL_8019_IRQ, end: 0, flags: IORESOURCE_IRQ }]; platform_device_register_simple("ne", -1, res.as_ptr(), res.len()); }
unsafe fn rbtx4927_mtd_init() { for i in 0..2 { tx4927_mtd_init(i); } }

static mut rbtx4927_gpioled_table: gpiod_lookup_table = unsafe { core::mem::zeroed() };
unsafe fn rbtx4927_gpioled_init() { static leds: [gpio_led; 2] = [gpio_led { name: "gpioled:green:0" }, gpio_led { name: "gpioled:green:1" }]; static mut pdata: gpio_led_platform_data = gpio_led_platform_data { num_leds: 2, leds: leds.as_ptr() }; let pdev = platform_device_alloc("leds-gpio", 0); if pdev.is_null() { return; } (*pdev).dev.platform_data = &mut pdata as *mut _ as *mut core::ffi::c_void; if platform_device_add(pdev) != 0 { platform_device_put(pdev); return; } rbtx4927_gpioled_table.dev_id = dev_name(&(*pdev).dev); gpiod_add_lookup_table(&mut rbtx4927_gpioled_table); }
unsafe fn rbtx4927_device_init() { toshiba_rbtx4927_rtc_init(); rbtx4927_ne_init(); tx4927_wdt_init(); rbtx4927_mtd_init(); if TX4927_REV_PCODE() == 0x4927 { tx4927_dmac_init(2); tx4927_aclc_init(0, 1); } else { tx4938_dmac_init(0, 2); tx4938_aclc_init(); } platform_device_register_simple("txx9aclc-generic", -1, core::ptr::null(), 0); txx9_iocled_init(RBTX4927_LED_ADDR - IO_BASE, 3, "green", core::ptr::null()); rbtx4927_gpioled_init(); }

// Board vectors retain the kernel's externally supplied callback structure.
pub static mut rbtx4927_vec: txx9_board_vec = txx9_board_vec { system: "Toshiba RBTX4927", prom_init: rbtx4927_prom_init, mem_setup: rbtx4927_mem_setup, irq_setup: rbtx4927_irq_setup, time_init: rbtx4927_time_init, device_init: rbtx4927_device_init, arch_init: rbtx4927_arch_init, pci_map_irq: rbtx4927_pci_map_irq };
pub static mut rbtx4937_vec: txx9_board_vec = txx9_board_vec { system: "Toshiba RBTX4937", prom_init: rbtx4927_prom_init, mem_setup: rbtx4927_mem_setup, irq_setup: rbtx4927_irq_setup, time_init: rbtx4927_time_init, device_init: rbtx4927_device_init, arch_init: rbtx4937_arch_init, pci_map_irq: rbtx4927_pci_map_irq };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
