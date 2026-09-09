/* Translated from ar2315.c. */

// Kernel headers and local headers provide the types, constants, macros, and
// external functions referenced below.

static mut ar2315_rst_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ar2315_misc_irq_domain: *mut irq_domain = core::ptr::null_mut();

unsafe fn ar2315_rst_reg_read(reg: u32) -> u32 {
    __raw_readl((ar2315_rst_base as *mut u8).add(reg as usize) as *const u32)
}

unsafe fn ar2315_rst_reg_write(reg: u32, val: u32) {
    __raw_writel(val, (ar2315_rst_base as *mut u8).add(reg as usize) as *mut u32);
}

unsafe fn ar2315_rst_reg_mask(reg: u32, mask: u32, val: u32) {
    let mut ret = ar2315_rst_reg_read(reg);
    ret &= !mask;
    ret |= val;
    ar2315_rst_reg_write(reg, ret);
}

unsafe extern "C" fn ar2315_ahb_err_handler(_cpl: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    ar2315_rst_reg_write(AR2315_AHB_ERR0, AR2315_AHB_ERROR_DET);
    ar2315_rst_reg_read(AR2315_AHB_ERR1);
    pr_emerg!("AHB fatal error\n");
    machine_restart(b"AHB error\0".as_ptr() as *const i8);
    IRQ_HANDLED
}

unsafe extern "C" fn ar2315_misc_irq_handler(desc: *mut irq_desc) {
    let pending = ar2315_rst_reg_read(AR2315_ISR) & ar2315_rst_reg_read(AR2315_IMR);
    let mut ret = 0;
    if pending != 0 {
        let domain = irq_desc_get_handler_data(desc);
        let nr = __ffs(pending);
        if nr == AR2315_MISC_IRQ_GPIO {
            ar2315_rst_reg_write(AR2315_ISR, AR2315_ISR_GPIO);
        } else if nr == AR2315_MISC_IRQ_WATCHDOG {
            ar2315_rst_reg_write(AR2315_ISR, AR2315_ISR_WD);
        }
        ret = generic_handle_domain_irq(domain, nr);
    }
    if pending == 0 || ret != 0 { spurious_interrupt(); }
}

unsafe extern "C" fn ar2315_misc_irq_unmask(d: *mut irq_data) {
    ar2315_rst_reg_mask(AR2315_IMR, 0, BIT((*d).hwirq));
}

unsafe extern "C" fn ar2315_misc_irq_mask(d: *mut irq_data) {
    ar2315_rst_reg_mask(AR2315_IMR, BIT((*d).hwirq), 0);
}

static mut ar2315_misc_irq_chip: irq_chip = irq_chip {
    name: b"ar2315-misc\0".as_ptr() as *const i8,
    irq_unmask: Some(ar2315_misc_irq_unmask),
    irq_mask: Some(ar2315_misc_irq_mask),
};

unsafe extern "C" fn ar2315_misc_irq_map(_d: *mut irq_domain, irq: u32, _hw: irq_hw_number_t) -> i32 {
    irq_set_chip_and_handler(irq, &mut ar2315_misc_irq_chip, handle_level_irq);
    0
}

static ar2315_misc_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(ar2315_misc_irq_map) };

/* Interrupt dispatch establishes the interrupt priority by dispatch order. */
unsafe fn ar2315_irq_dispatch() {
    let pending = read_c0_status() & read_c0_cause();
    if pending & CAUSEF_IP3 != 0 { do_IRQ(AR2315_IRQ_WLAN0); }
    else if pending & CAUSEF_IP5 != 0 { do_IRQ(AR2315_IRQ_LCBUS_PCI); }
    else if pending & CAUSEF_IP2 != 0 { do_IRQ(AR2315_IRQ_MISC); }
    else if pending & CAUSEF_IP7 != 0 { do_IRQ(ATH25_IRQ_CPU_CLOCK); }
    else { spurious_interrupt(); }
}

pub unsafe fn ar2315_arch_init_irq() {
    ath25_irq_dispatch = Some(ar2315_irq_dispatch);
    let domain = irq_domain_create_linear(core::ptr::null_mut(), AR2315_MISC_IRQ_COUNT, &ar2315_misc_irq_domain_ops, core::ptr::null_mut());
    if domain.is_null() { panic!("Failed to add IRQ domain"); }
    let irq = irq_create_mapping(domain, AR2315_MISC_IRQ_AHB);
    if request_irq(irq, Some(ar2315_ahb_err_handler), 0, b"ar2315-ahb-error\0".as_ptr() as *const i8, core::ptr::null_mut()) != 0 { pr_err!("Failed to register ar2315-ahb-error interrupt\n"); }
    irq_set_chained_handler_and_data(AR2315_IRQ_MISC, Some(ar2315_misc_irq_handler), domain);
    ar2315_misc_irq_domain = domain;
}

pub unsafe fn ar2315_init_devices() {
    ath25_find_config(AR2315_SPI_READ_BASE, AR2315_SPI_READ_SIZE);
    ath25_add_wmac(0, AR2315_WLAN0_BASE, AR2315_IRQ_WLAN0);
}

unsafe extern "C" fn ar2315_restart(_command: *mut i8) {
    let mips_reset_vec: extern "C" fn() = core::mem::transmute(0xbfc00000usize);
    local_irq_disable();
    ar2315_rst_reg_write(AR2315_COLD_RESET, AR2317_RESET_SYSTEM);
    /* TODO: implement the GPIO reset workaround. */
    mips_reset_vec();
}

static clockctl1_predivide_table: [i32; 4] = [1, 2, 4, 5];
static pllc_divide_table: [i32; 5] = [2, 3, 4, 6, 3];

unsafe fn ar2315_sys_clk(clock_ctl: u32) -> u32 {
    let pllc_ctrl = ar2315_rst_reg_read(AR2315_PLLC_CTL);
    let refdiv = clockctl1_predivide_table[ATH25_REG_MS(pllc_ctrl, AR2315_PLLC_REF_DIV) as usize] as u32;
    let fdiv = ATH25_REG_MS(pllc_ctrl, AR2315_PLLC_FDBACK_DIV);
    let divby2 = ATH25_REG_MS(pllc_ctrl, AR2315_PLLC_ADD_FDBACK_DIV) + 1;
    let mut pllc_out = (40000000 / refdiv) * (2 * divby2) * fdiv;
    let clk_div = match clock_ctl & AR2315_CPUCLK_CLK_SEL_M {
        0 | 1 => pllc_divide_table[ATH25_REG_MS(pllc_ctrl, AR2315_PLLC_CLKM_DIV) as usize] as u32,
        2 => pllc_divide_table[ATH25_REG_MS(pllc_ctrl, AR2315_PLLC_CLKC_DIV) as usize] as u32,
        _ => { pllc_out = 40000000; 1 }
    };
    let cpu_div = { let v = ATH25_REG_MS(clock_ctl, AR2315_CPUCLK_CLK_DIV) * 2; if v == 0 { 1 } else { v } };
    pllc_out / (clk_div * cpu_div)
}

unsafe fn ar2315_cpu_frequency() -> u32 { ar2315_sys_clk(ar2315_rst_reg_read(AR2315_CPUCLK)) }
unsafe fn ar2315_apb_frequency() -> u32 { ar2315_sys_clk(ar2315_rst_reg_read(AR2315_AMBACLK)) }

pub unsafe fn ar2315_plat_time_init() { mips_hpt_frequency = ar2315_cpu_frequency() / 2; }

pub unsafe fn ar2315_plat_mem_setup() {
    let sdram_base = ioremap(AR2315_SDRAMCTL_BASE, AR2315_SDRAMCTL_SIZE);
    let memcfg = __raw_readl((sdram_base as *mut u8).add(AR2315_MEM_CFG as usize) as *const u32);
    let mut memsize = 1 + ATH25_REG_MS(memcfg, AR2315_MEM_CFG_DATA_WIDTH);
    memsize <<= 1 + ATH25_REG_MS(memcfg, AR2315_MEM_CFG_COL_WIDTH);
    memsize <<= 1 + ATH25_REG_MS(memcfg, AR2315_MEM_CFG_ROW_WIDTH);
    memsize <<= 3;
    memblock_add(0, memsize as u64);
    iounmap(sdram_base);
    ar2315_rst_base = ioremap(AR2315_RST_BASE, AR2315_RST_SIZE);
    let devid = ar2315_rst_reg_read(AR2315_SREV) & AR2315_REV_CHIP;
    ath25_soc = match devid { 0x91 => ATH25_SOC_AR2318, 0x90 => ATH25_SOC_AR2317, 0x87 => ATH25_SOC_AR2316, _ => ATH25_SOC_AR2315 };
    ath25_board.devid = devid;
    let config = read_c0_config();
    write_c0_config(config & !0x3);
    ar2315_rst_reg_write(AR2315_AHB_ERR0, AR2315_AHB_ERROR_DET);
    ar2315_rst_reg_read(AR2315_AHB_ERR1);
    ar2315_rst_reg_write(AR2315_WDT_CTRL, AR2315_WDT_CTRL_IGNORE);
    _machine_restart = Some(ar2315_restart);
}

// #ifdef CONFIG_PCI_AR2315
static mut ar2315_pci_res: [resource; 3] = [
    resource { name: b"ar2315-pci-ctrl\0".as_ptr() as *const i8, flags: IORESOURCE_MEM, start: AR2315_PCI_BASE, end: AR2315_PCI_BASE + AR2315_PCI_SIZE - 1 },
    resource { name: b"ar2315-pci-ext\0".as_ptr() as *const i8, flags: IORESOURCE_MEM, start: AR2315_PCI_EXT_BASE, end: AR2315_PCI_EXT_BASE + AR2315_PCI_EXT_SIZE - 1 },
    resource { name: b"ar2315-pci\0".as_ptr() as *const i8, flags: IORESOURCE_IRQ, start: AR2315_IRQ_LCBUS_PCI, end: AR2315_IRQ_LCBUS_PCI },
];
// #endif CONFIG_PCI_AR2315

pub unsafe fn ar2315_arch_init() {
    let irq = irq_create_mapping(ar2315_misc_irq_domain, AR2315_MISC_IRQ_UART0);
    ath25_serial_setup(AR2315_UART0_BASE, irq, ar2315_apb_frequency());

    // #ifdef CONFIG_PCI_AR2315
    if ath25_soc == ATH25_SOC_AR2315 {
        ar2315_rst_reg_mask(AR2315_RESET, 0, AR2315_RESET_PCIDMA);
        msleep(20);
        ar2315_rst_reg_mask(AR2315_RESET, AR2315_RESET_PCIDMA, 0);
        msleep(20);
        ar2315_rst_reg_mask(AR2315_ENDIAN_CTL, 0,
            AR2315_CONFIG_PCIAHB | AR2315_CONFIG_PCIAHB_BRIDGE);
        ar2315_rst_reg_write(AR2315_PCICLK, AR2315_PCICLK_PLLC_CLKM |
            (AR2315_PCICLK_IN_FREQ_DIV_6 << AR2315_PCICLK_DIV_S));
        ar2315_rst_reg_mask(AR2315_AHB_ARB_CTL, 0, AR2315_ARB_PCI);
        ar2315_rst_reg_mask(AR2315_IF_CTL,
            AR2315_IF_PCI_CLK_MASK | AR2315_IF_MASK,
            AR2315_IF_PCI | AR2315_IF_PCI_HOST | AR2315_IF_PCI_INTR |
            (AR2315_IF_PCI_CLK_OUTPUT_CLK << AR2315_IF_PCI_CLK_SHIFT));
        platform_device_register_simple(b"ar2315-pci\0".as_ptr() as *const i8,
            -1, ar2315_pci_res.as_mut_ptr(), ar2315_pci_res.len());
    }
    // #endif CONFIG_PCI_AR2315
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
