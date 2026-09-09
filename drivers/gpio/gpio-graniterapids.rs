// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Granite Rapids-D vGPIO driver
 *
 * Copyright (c) 2024, Intel Corporation.
 *
 * Author: Aapo Vienamo <aapo.vienamo@linux.intel.com>
 */

// Linux kernel dependencies supplied by the surrounding tree.

const GNR_NUM_PINS: usize = 128;
const GNR_PINS_PER_REG: usize = 32;
const GNR_NUM_REGS: usize = (GNR_NUM_PINS + GNR_PINS_PER_REG - 1) / GNR_PINS_PER_REG;

const GNR_CFG_PADBAR: usize = 0x00;
const GNR_CFG_LOCK_OFFSET: usize = 0x04;
const GNR_GPI_STATUS_OFFSET: usize = 0x14;
const GNR_GPI_ENABLE_OFFSET: usize = 0x24;

const GNR_CFG_DW_HOSTSW_MODE: u32 = 1 << 27;
const GNR_CFG_DW_RX_MASK: u32 = 0x00c0_0000;
const GNR_CFG_DW_INTSEL_MASK: u32 = 0x003f_c000;
const GNR_CFG_DW_RX_DISABLE: u32 = 2 << 22;
const GNR_CFG_DW_RX_EDGE: u32 = 1 << 22;
const GNR_CFG_DW_RX_LEVEL: u32 = 0 << 22;
const GNR_CFG_DW_RXDIS: u32 = 1 << 4;
const GNR_CFG_DW_TXDIS: u32 = 1 << 3;
const GNR_CFG_DW_RXSTATE: u32 = 1 << 1;
const GNR_CFG_DW_TXSTATE: u32 = 1 << 0;

#[repr(C)]
struct GnrGpio {
    gc: gpio_chip,
    reg_base: *mut core::ffi::c_void,
    pad_base: *mut core::ffi::c_void,
    ro_bitmap: [usize; GNR_NUM_PINS / (usize::BITS as usize)],
    lock: raw_spinlock_t,
    pad_backup: [u32; 0],
}

unsafe fn gnr_gpio_get_padcfg_addr(priv_: *const GnrGpio, gpio: u32) -> *mut core::ffi::c_void {
    (*priv_).pad_base.add(gpio as usize * core::mem::size_of::<u32>())
}

unsafe fn gnr_gpio_configure_line(gc: *mut gpio_chip, gpio: u32, clear_mask: u32, set_mask: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    let addr = gnr_gpio_get_padcfg_addr(priv_, gpio);
    if test_bit(gpio, (*priv_).ro_bitmap.as_ptr()) { return -EACCES; }
    let _guard = raw_spinlock_irqsave_guard(&mut (*priv_).lock);
    let mut dw = readl(addr);
    dw &= !clear_mask;
    dw |= set_mask;
    writel(dw, addr);
    0
}

unsafe fn gnr_gpio_request(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    let dw = readl(gnr_gpio_get_padcfg_addr(priv_, gpio));
    if dw & GNR_CFG_DW_HOSTSW_MODE == 0 {
        dev_warn((*gc).parent, "GPIO %u is not owned by host", gpio);
        return -EBUSY;
    }
    0
}

unsafe fn gnr_gpio_get(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *const GnrGpio;
    (readl(gnr_gpio_get_padcfg_addr(priv_, gpio)) & GNR_CFG_DW_RXSTATE != 0) as i32
}

unsafe fn gnr_gpio_set(gc: *mut gpio_chip, gpio: u32, value: i32) -> i32 {
    gnr_gpio_configure_line(gc, gpio, if value != 0 { 0 } else { GNR_CFG_DW_TXSTATE }, if value != 0 { GNR_CFG_DW_TXSTATE } else { 0 })
}

unsafe fn gnr_gpio_get_direction(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    if readl(gnr_gpio_get_padcfg_addr(priv_, gpio)) & GNR_CFG_DW_TXDIS != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe fn gnr_gpio_direction_input(gc: *mut gpio_chip, gpio: u32) -> i32 {
    gnr_gpio_configure_line(gc, gpio, GNR_CFG_DW_RXDIS, 0)
}

unsafe fn gnr_gpio_direction_output(gc: *mut gpio_chip, gpio: u32, value: i32) -> i32 {
    gnr_gpio_configure_line(gc, gpio, GNR_CFG_DW_TXDIS, if value != 0 { GNR_CFG_DW_TXSTATE } else { 0 })
}

static mut GNR_GPIO_CHIP: gpio_chip = gpio_chip {
    owner: THIS_MODULE, request: Some(gnr_gpio_request), get: Some(gnr_gpio_get), set: Some(gnr_gpio_set),
    get_direction: Some(gnr_gpio_get_direction), direction_input: Some(gnr_gpio_direction_input), direction_output: Some(gnr_gpio_direction_output),
};

unsafe fn gnr_gpio_get_reg_addr(priv_: *const GnrGpio, base: usize, gpio: u32) -> *mut core::ffi::c_void {
    (*priv_).reg_base.add(base + gpio as usize * core::mem::size_of::<u32>())
}

unsafe fn gnr_gpio_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    let gpio = irqd_to_hwirq(d) as usize;
    let addr = gnr_gpio_get_reg_addr(priv_, GNR_GPI_STATUS_OFFSET, (gpio / GNR_PINS_PER_REG) as u32);
    let _guard = raw_spinlock_irqsave_guard(&mut (*priv_).lock);
    writel(readl(addr) | (1 << (gpio % GNR_PINS_PER_REG)), addr);
}

unsafe fn gnr_gpio_irq_mask_unmask(gc: *mut gpio_chip, gpio: usize, mask: bool) {
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    let addr = gnr_gpio_get_reg_addr(priv_, GNR_GPI_ENABLE_OFFSET, (gpio / GNR_PINS_PER_REG) as u32);
    let _guard = raw_spinlock_irqsave_guard(&mut (*priv_).lock);
    let bit = 1 << (gpio % GNR_PINS_PER_REG);
    let reg = readl(addr);
    writel(if mask { reg & !bit } else { reg | bit }, addr);
}

unsafe fn gnr_gpio_irq_mask(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip; let hwirq = irqd_to_hwirq(d); gnr_gpio_irq_mask_unmask(gc, hwirq as usize, true); gpiochip_disable_irq(gc, hwirq); }
unsafe fn gnr_gpio_irq_unmask(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip; let hwirq = irqd_to_hwirq(d); gpiochip_enable_irq(gc, hwirq); gnr_gpio_irq_mask_unmask(gc, hwirq as usize, false); }

unsafe fn gnr_gpio_irq_set_type(d: *mut irq_data, irq_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let priv_ = gpiochip_get_data(gc) as *mut GnrGpio;
    let hwirq = irqd_to_hwirq(d);
    if readl(gnr_gpio_get_padcfg_addr(priv_, hwirq as u32)) & GNR_CFG_DW_INTSEL_MASK == 0 { dev_dbg((*gc).parent, "GPIO %lu cannot be used as IRQ", hwirq); return -EPERM; }
    let set = match irq_type { IRQ_TYPE_NONE => GNR_CFG_DW_RX_DISABLE, IRQ_TYPE_EDGE_RISING => { irq_set_handler_locked(d, handle_edge_irq); GNR_CFG_DW_RX_EDGE }, IRQ_TYPE_LEVEL_HIGH => { irq_set_handler_locked(d, handle_level_irq); GNR_CFG_DW_RX_LEVEL }, _ => return -EINVAL };
    gnr_gpio_configure_line(gc, hwirq as u32, GNR_CFG_DW_RX_MASK, set)
}

// The remaining platform-driver registration and power-management declarations retain their
// kernel interfaces; dependent kernel types and helpers are supplied by the surrounding tree.
unsafe fn gnr_gpio_init_pin_ro_bits(_dev: *mut device, cfg_lock_base: *const core::ffi::c_void, ro_bitmap: *mut usize) { let mut tmp = [0u32; GNR_NUM_REGS]; memcpy_fromio(tmp.as_mut_ptr(), cfg_lock_base, core::mem::size_of_val(&tmp)); bitmap_from_arr32(ro_bitmap, tmp.as_ptr(), GNR_NUM_PINS); }

unsafe fn gnr_gpio_irq(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let priv_ = data as *mut GnrGpio;
    let mut handled = 0;
    for i in 0..GNR_NUM_REGS {
        let reg = (*priv_).reg_base.add(i * core::mem::size_of::<u32>());
        let (pending, enabled) = {
            let _guard = raw_spinlock_guard(&mut (*priv_).lock);
            (readl(reg.add(GNR_GPI_STATUS_OFFSET)), readl(reg.add(GNR_GPI_ENABLE_OFFSET)))
        };
        let pending = pending & enabled;
        for bit_idx in 0..GNR_PINS_PER_REG {
            if pending & (1 << bit_idx) != 0 {
                generic_handle_domain_irq((*priv_).gc.irq.domain, (i * GNR_PINS_PER_REG + bit_idx) as u32);
            }
        }
        if pending != 0 { handled += 1; }
    }
    IRQ_RETVAL(handled)
}

unsafe fn gnr_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let priv_ = devm_kzalloc(dev, struct_size::<GnrGpio>(GNR_NUM_PINS), GFP_KERNEL) as *mut GnrGpio;
    if priv_.is_null() { return -ENOMEM; }
    raw_spin_lock_init(&mut (*priv_).lock);
    let regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) { return PTR_ERR(regs); }
    (*priv_).reg_base = regs;
    let offset = readl((*priv_).reg_base.add(GNR_CFG_PADBAR));
    (*priv_).pad_base = (*priv_).reg_base.add(offset as usize);
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let ret = devm_request_irq(dev, irq, Some(gnr_gpio_irq), IRQF_SHARED | IRQF_NO_THREAD, dev_name(dev), priv_ as *mut _);
    if ret != 0 { return dev_err_probe(dev, ret, "failed to request interrupt\n"); }
    gnr_gpio_init_pin_ro_bits(dev, (*priv_).reg_base.add(GNR_CFG_LOCK_OFFSET), (*priv_).ro_bitmap.as_mut_ptr());
    (*priv_).gc = GNR_GPIO_CHIP;
    (*priv_).gc.label = dev_name(dev); (*priv_).gc.parent = dev; (*priv_).gc.ngpio = GNR_NUM_PINS as u32; (*priv_).gc.base = -1;
    let girq = &mut (*priv_).gc.irq;
    gpio_irq_chip_set_chip(girq, &GNR_GPIO_IRQ_CHIP);
    girq.parent_handler = None; girq.num_parents = 0; girq.parents = core::ptr::null_mut(); girq.default_type = IRQ_TYPE_NONE; girq.handler = handle_bad_irq;
    platform_set_drvdata(pdev, priv_ as *mut _);
    devm_gpiochip_add_data(dev, &mut (*priv_).gc, priv_ as *mut _)
}

unsafe fn gnr_gpio_suspend(dev: *mut device) -> i32 { let priv_ = dev_get_drvdata(dev) as *mut GnrGpio; let _guard = raw_spinlock_irqsave_guard(&mut (*priv_).lock); for i in 0..(*priv_).gc.ngpio as usize { if !test_bit(i as u32, (*priv_).ro_bitmap.as_ptr()) { (*priv_).pad_backup[i] = readl(gnr_gpio_get_padcfg_addr(priv_, i as u32)); } } 0 }
unsafe fn gnr_gpio_resume(dev: *mut device) -> i32 { let priv_ = dev_get_drvdata(dev) as *mut GnrGpio; let _guard = raw_spinlock_irqsave_guard(&mut (*priv_).lock); for i in 0..(*priv_).gc.ngpio as usize { if !test_bit(i as u32, (*priv_).ro_bitmap.as_ptr()) { writel((*priv_).pad_backup[i], gnr_gpio_get_padcfg_addr(priv_, i as u32)); } } 0 }

static GNR_GPIO_IRQ_CHIP: irq_chip = irq_chip { name: "gpio-graniterapids", irq_ack: Some(gnr_gpio_irq_ack), irq_mask: Some(gnr_gpio_irq_mask), irq_unmask: Some(gnr_gpio_irq_unmask), irq_set_type: Some(gnr_gpio_irq_set_type), flags: IRQCHIP_IMMUTABLE };

// DEFINE_SIMPLE_DEV_PM_OPS, ACPI match table, platform_driver, module_platform_driver,
// MODULE_LICENSE, MODULE_AUTHOR, and MODULE_DESCRIPTION preserve their kernel declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
