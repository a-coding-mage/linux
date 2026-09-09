/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Platform devices for Atheros AR5312 SoCs.
 *
 * Direct Rust translation of ar5312.c.
 */

use core::ptr;

static mut ar5312_rst_base: *mut core::ffi::c_void = ptr::null_mut();
static mut ar5312_misc_irq_domain: *mut irq_domain = ptr::null_mut();

#[inline]
unsafe fn ar5312_rst_reg_read(reg: u32) -> u32 {
    __raw_readl(ar5312_rst_base.add(reg as usize))
}

#[inline]
unsafe fn ar5312_rst_reg_write(reg: u32, val: u32) {
    __raw_writel(val, ar5312_rst_base.add(reg as usize));
}

#[inline]
unsafe fn ar5312_rst_reg_mask(reg: u32, mask: u32, val: u32) {
    let mut ret = ar5312_rst_reg_read(reg);
    ret &= !mask;
    ret |= val;
    ar5312_rst_reg_write(reg, ret);
}

unsafe extern "C" fn ar5312_ahb_err_handler(_cpl: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let proc1 = ar5312_rst_reg_read(AR5312_PROC1);
    let proc_addr = ar5312_rst_reg_read(AR5312_PROCADDR); /* clears error */
    let dma1 = ar5312_rst_reg_read(AR5312_DMA1);
    let dma_addr = ar5312_rst_reg_read(AR5312_DMAADDR); /* clears error */

    pr_emerg!("AHB interrupt: PROCADDR=0x{:8.8x} PROC1=0x{:8.8x} DMAADDR=0x{:8.8x} DMA1=0x{:8.8x}\n",
        proc_addr, proc1, dma_addr, dma1);
    machine_restart("AHB error"); /* Catastrophic failure */
    IRQ_HANDLED
}

unsafe extern "C" fn ar5312_misc_irq_handler(desc: *mut irq_desc) {
    let pending = ar5312_rst_reg_read(AR5312_ISR) & ar5312_rst_reg_read(AR5312_IMR);
    let mut ret = 0;
    if pending != 0 {
        let domain = irq_desc_get_handler_data(desc);
        let nr = pending.trailing_zeros();
        ret = generic_handle_domain_irq(domain, nr);
        if nr == AR5312_MISC_IRQ_TIMER { ar5312_rst_reg_read(AR5312_TIMER); }
    }
    if pending == 0 || ret != 0 { spurious_interrupt(); }
}

/* Enable the specified AR5312_MISC_IRQ interrupt */
unsafe extern "C" fn ar5312_misc_irq_unmask(d: *mut irq_data) {
    ar5312_rst_reg_mask(AR5312_IMR, 0, 1u32 << (*d).hwirq);
}

/* Disable the specified AR5312_MISC_IRQ interrupt */
unsafe extern "C" fn ar5312_misc_irq_mask(d: *mut irq_data) {
    ar5312_rst_reg_mask(AR5312_IMR, 1u32 << (*d).hwirq, 0);
    ar5312_rst_reg_read(AR5312_IMR); /* flush write buffer */
}

static mut ar5312_misc_irq_chip: irq_chip = irq_chip {
    name: c"ar5312-misc".as_ptr(),
    irq_unmask: Some(ar5312_misc_irq_unmask),
    irq_mask: Some(ar5312_misc_irq_mask),
};

unsafe extern "C" fn ar5312_misc_irq_map(d: *mut irq_domain, irq: u32, _hw: irq_hw_number_t) -> i32 {
    irq_set_chip_and_handler(irq, &raw mut ar5312_misc_irq_chip, handle_level_irq);
    0
}

static mut ar5312_misc_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(ar5312_misc_irq_map) };

unsafe fn ar5312_irq_dispatch() {
    let pending = read_c0_status() & read_c0_cause();
    if pending & CAUSEF_IP2 != 0 { do_IRQ(AR5312_IRQ_WLAN0); }
    else if pending & CAUSEF_IP5 != 0 { do_IRQ(AR5312_IRQ_WLAN1); }
    else if pending & CAUSEF_IP6 != 0 { do_IRQ(AR5312_IRQ_MISC); }
    else if pending & CAUSEF_IP7 != 0 { do_IRQ(ATH25_IRQ_CPU_CLOCK); }
    else { spurious_interrupt(); }
}

pub unsafe fn ar5312_arch_init_irq() {
    ath25_irq_dispatch = Some(ar5312_irq_dispatch);
    let domain = irq_domain_create_linear(ptr::null_mut(), AR5312_MISC_IRQ_COUNT, &raw mut ar5312_misc_irq_domain_ops, ptr::null_mut());
    if domain.is_null() { panic!("Failed to add IRQ domain"); }
    let irq = irq_create_mapping(domain, AR5312_MISC_IRQ_AHB_PROC);
    if request_irq(irq, Some(ar5312_ahb_err_handler), 0, c"ar5312-ahb-error".as_ptr(), ptr::null_mut()) != 0 {
        pr_err!("Failed to register ar5312-ahb-error interrupt\n");
    }
    irq_set_chained_handler_and_data(AR5312_IRQ_MISC, Some(ar5312_misc_irq_handler), domain);
    ar5312_misc_irq_domain = domain;
}

static mut ar5312_flash_data: physmap_flash_data = physmap_flash_data { width: 2 };
static mut ar5312_flash_resource: resource = resource { start: AR5312_FLASH_BASE, end: AR5312_FLASH_BASE + AR5312_FLASH_SIZE - 1, flags: IORESOURCE_MEM };
static mut ar5312_physmap_flash: platform_device = platform_device {
    name: c"physmap-flash".as_ptr(), id: 0, dev: platform_device_dev { platform_data: &raw mut ar5312_flash_data as *mut _ },
    resource: &raw mut ar5312_flash_resource, num_resources: 1,
};

unsafe fn ar5312_flash_init() {
    let flashctl_base = ioremap(AR5312_FLASHCTL_BASE, AR5312_FLASHCTL_SIZE);
    let mut ctl = __raw_readl(flashctl_base.add(AR5312_FLASHCTL0 as usize));
    ctl &= AR5312_FLASHCTL_MW;
    match ctl { AR5312_FLASHCTL_MW16 => ar5312_flash_data.width = 2, AR5312_FLASHCTL_MW8 | _ => ar5312_flash_data.width = 1 }
    ctl |= AR5312_FLASHCTL_E | AR5312_FLASHCTL_AC_8M | AR5312_FLASHCTL_RBLE;
    ctl |= 0x01 << AR5312_FLASHCTL_IDCY_S; ctl |= 0x07 << AR5312_FLASHCTL_WST1_S; ctl |= 0x07 << AR5312_FLASHCTL_WST2_S;
    __raw_writel(ctl, flashctl_base.add(AR5312_FLASHCTL0 as usize));
    for reg in [AR5312_FLASHCTL1, AR5312_FLASHCTL2] { ctl = __raw_readl(flashctl_base.add(reg as usize)); ctl &= !(AR5312_FLASHCTL_E | AR5312_FLASHCTL_AC); __raw_writel(ctl, flashctl_base.add(reg as usize)); }
    iounmap(flashctl_base);
}

pub unsafe fn ar5312_init_devices() {
    ar5312_flash_init();
    ath25_find_config(AR5312_FLASH_BASE, AR5312_FLASH_SIZE);
    let config = ath25_board.config;
    if (current_cpu_data.processor_id & 0xff) == 0x0a { ath25_soc = ATH25_SOC_AR2313; }
    else if (*config).flags & BD_ISCASPER != 0 { ath25_soc = ATH25_SOC_AR2312; }
    else { ath25_soc = ATH25_SOC_AR5312; }
    platform_device_register(&raw mut ar5312_physmap_flash);
    match ath25_soc {
        ATH25_SOC_AR5312 => { if ath25_board.radio.is_null() { return; } if (*config).flags & BD_WLAN0 == 0 { } else { ath25_add_wmac(0, AR5312_WLAN0_BASE, AR5312_IRQ_WLAN0); } }
        ATH25_SOC_AR2312 | ATH25_SOC_AR2313 => { if ath25_board.radio.is_null() { return; } }
        _ => {}
    }
    if (*config).flags & BD_WLAN1 != 0 { ath25_add_wmac(1, AR5312_WLAN1_BASE, AR5312_IRQ_WLAN1); }
}

unsafe fn ar5312_restart(_command: *mut i8) { local_irq_disable(); loop { ar5312_rst_reg_write(AR5312_RESET, AR5312_RESET_SYSTEM); } }

static mut clockctl1_predivide_table: [u32; 4] = [1, 2, 4, 5];

unsafe fn ar5312_cpu_frequency() -> u32 {
    let scratch = ar5312_rst_reg_read(AR5312_SCRATCH); if scratch != 0 { return scratch; }
    let mut devid = ar5312_rst_reg_read(AR5312_REV); devid = (devid & AR5312_REV_MAJ) >> AR5312_REV_MAJ_S;
    let (predivide_mask, predivide_shift, multiplier_mask, multiplier_shift, doubler_mask) = if devid == AR5312_REV_MAJ_AR2313 { (AR2313_CLOCKCTL1_PREDIVIDE_MASK, AR2313_CLOCKCTL1_PREDIVIDE_SHIFT, AR2313_CLOCKCTL1_MULTIPLIER_MASK, AR2313_CLOCKCTL1_MULTIPLIER_SHIFT, AR2313_CLOCKCTL1_DOUBLER_MASK) } else { (AR5312_CLOCKCTL1_PREDIVIDE_MASK, AR5312_CLOCKCTL1_PREDIVIDE_SHIFT, AR5312_CLOCKCTL1_MULTIPLIER_MASK, AR5312_CLOCKCTL1_MULTIPLIER_SHIFT, AR5312_CLOCKCTL1_DOUBLER_MASK) };
    let clock_ctl1 = ar5312_rst_reg_read(AR5312_CLOCKCTL1);
    let predivide_select = (clock_ctl1 & predivide_mask) >> predivide_shift;
    let predivisor = clockctl1_predivide_table[predivide_select as usize];
    let mut multiplier = (clock_ctl1 & multiplier_mask) >> multiplier_shift;
    if clock_ctl1 & doubler_mask != 0 { multiplier <<= 1; }
    (40000000 / predivisor) * multiplier
}

#[inline] unsafe fn ar5312_sys_frequency() -> u32 { ar5312_cpu_frequency() / 4 }

pub unsafe fn ar5312_plat_time_init() { mips_hpt_frequency = ar5312_cpu_frequency() / 2; }

pub unsafe fn ar5312_plat_mem_setup() {
    let sdram_base = ioremap(AR5312_SDRAMCTL_BASE, AR5312_SDRAMCTL_SIZE);
    let memcfg = __raw_readl(sdram_base.add(AR5312_MEM_CFG1 as usize));
    let bank0_ac = ATH25_REG_MS(memcfg, AR5312_MEM_CFG1_AC0); let bank1_ac = ATH25_REG_MS(memcfg, AR5312_MEM_CFG1_AC1);
    let memsize = (if bank0_ac != 0 { 1 << (bank0_ac + 1) } else { 0 }) + (if bank1_ac != 0 { 1 << (bank1_ac + 1) } else { 0 });
    memblock_add(0, memsize << 20); iounmap(sdram_base);
    ar5312_rst_base = ioremap(AR5312_RST_BASE, AR5312_RST_SIZE);
    let mut devid = ar5312_rst_reg_read(AR5312_REV); devid >>= AR5312_REV_WMAC_MIN_S; devid &= AR5312_REV_CHIP; ath25_board.devid = devid as u16;
    ar5312_rst_reg_read(AR5312_PROCADDR); ar5312_rst_reg_read(AR5312_DMAADDR); ar5312_rst_reg_write(AR5312_WDT_CTRL, AR5312_WDT_CTRL_IGNORE); _machine_restart = Some(ar5312_restart);
}

pub unsafe fn ar5312_arch_init() { let irq = irq_create_mapping(ar5312_misc_irq_domain, AR5312_MISC_IRQ_UART0); ath25_serial_setup(AR5312_UART0_BASE, irq, ar5312_sys_frequency()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
