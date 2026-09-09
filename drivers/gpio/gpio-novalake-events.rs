// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Nova Lake GPIO-signaled ACPI events driver
 *
 * Copyright (c) 2026, Intel Corporation.
 *
 * Author: Alan Borzeszkowski <alan.borzeszkowski@linux.intel.com>
 *
 * Intel client platforms released in 2026 and later (starting with Intel Nova
 * Lake) support two modes of handling ACPI General Purpose Events (GPE):
 * exposed GPIO interrupt mode and legacy mode.
 *
 * By default, the platform uses legacy mode, handling GPEs as usual. If this
 * driver is installed, it signals to the platform (on every boot) that exposed
 * GPIO interrupt mode is supported. The platform then switches to exposed
 * mode, which takes effect on next boot. From the user perspective, this
 * change is transparent.
 *
 * However, if driver is uninstalled while in exposed interrupt mode, GPEs will
 * _not_ be handled until platform falls back to legacy mode. This means that
 * USB keyboard, mouse might not function properly for the fallback duration.
 * Fallback requires two reboots to take effect: on first reboot, platform no
 * longer receives signal from this driver and switches to legacy mode, which
 * takes effect on second boot.
 *
 * Example ACPI event: Power Management Event coming from motherboard PCH,
 * waking system from sleep following USB mouse hotplug.
 *
 * This driver supports up to 128 GPIO pins in each GPE block, per ACPI
 * specification v6.6 section 5.6.4.
 */

// Linux kernel dependencies supplied externally by the surrounding tree.

const fn gpe_blk_reg_size(block_size: usize) -> usize { block_size / 2 }
const fn gpe_reg_pin_count(block_size: usize) -> usize { gpe_blk_reg_size(block_size) * 8 }
const GPE_STS_REG_OFFSET: usize = 0;
const fn gpe_en_reg_offset(block_size: usize) -> usize { gpe_blk_reg_size(block_size) }

#[repr(C)]
pub struct NvlGpio {
    pub gc: GpioChip,
    pub reg_base: *mut u8,
    pub lock: RawSpinlock,
    pub blk_size: usize,
}

unsafe fn nvl_gpio_get_byte_addr(priv_: *mut NvlGpio, reg_offset: usize, gpio: usize) -> *mut u8 {
    (*priv_).reg_base.add(reg_offset + gpio)
}

unsafe fn nvl_gpio_get(gc: *mut GpioChip, gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let byte_idx = (gpio as usize) / 8;
    let bit_idx = (gpio as usize) % 8;
    let addr = nvl_gpio_get_byte_addr(priv_, GPE_STS_REG_OFFSET, byte_idx);
    let _guard = guard_raw_spinlock_irqsave(&mut (*priv_).lock);
    let reg = ioread8(addr);
    ((reg & (1u8 << bit_idx)) != 0) as i32
}

static NVL_GPIO_CHIP: GpioChip = GpioChip {
    owner: THIS_MODULE,
    get: Some(nvl_gpio_get),
};

unsafe fn nvl_gpio_irq_set_type(d: *mut IrqData, irq_type: u32) -> i32 {
    if irq_type & IRQ_TYPE_EDGE_BOTH != 0 {
        irq_set_handler_locked(d, handle_edge_irq);
    } else if irq_type & IRQ_TYPE_LEVEL_MASK != 0 {
        irq_set_handler_locked(d, handle_level_irq);
    }
    0
}

unsafe fn nvl_gpio_irq_mask_unmask(gc: *mut GpioChip, hwirq: usize, mask: bool) {
    let priv_ = gpiochip_get_data(gc);
    let byte_idx = hwirq / 8;
    let bit_idx = hwirq % 8;
    let addr = nvl_gpio_get_byte_addr(priv_, gpe_en_reg_offset((*priv_).blk_size), byte_idx);
    let _guard = guard_raw_spinlock_irqsave(&mut (*priv_).lock);
    let mut reg = ioread8(addr);
    if mask { reg &= !(1u8 << bit_idx); } else { reg |= 1u8 << bit_idx; }
    iowrite8(reg, addr);
}

unsafe fn nvl_gpio_irq_unmask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let hwirq = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    nvl_gpio_irq_mask_unmask(gc, hwirq, false);
}

unsafe fn nvl_gpio_irq_mask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let hwirq = irqd_to_hwirq(d);
    nvl_gpio_irq_mask_unmask(gc, hwirq, true);
    gpiochip_disable_irq(gc, hwirq);
}

unsafe fn nvl_gpio_irq_ack(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc);
    let hwirq = irqd_to_hwirq(d);
    let addr = nvl_gpio_get_byte_addr(priv_, GPE_STS_REG_OFFSET, hwirq / 8);
    let bit_idx = hwirq % 8;
    let _guard = guard_raw_spinlock_irqsave(&mut (*priv_).lock);
    let mut reg = ioread8(addr);
    reg |= 1u8 << bit_idx;
    iowrite8(reg, addr);
}

static NVL_GPIO_IRQ_CHIP: IrqChip = IrqChip {
    name: "gpio-novalake",
    irq_ack: Some(nvl_gpio_irq_ack), irq_mask: Some(nvl_gpio_irq_mask),
    irq_unmask: Some(nvl_gpio_irq_unmask), irq_set_type: Some(nvl_gpio_irq_set_type),
    flags: IRQCHIP_IMMUTABLE,
};

unsafe fn nvl_gpio_irq(_irq: i32, data: *mut core::ffi::c_void) -> Irqreturn {
    let priv_ = data as *mut NvlGpio;
    let block_size = (*priv_).blk_size;
    let mut handled = 0;
    for i in 0..block_size {
        let reg = (*priv_).reg_base.add(i);
        let (mut pending, enabled);
        { let _guard = scoped_guard_raw_spinlock(&mut (*priv_).lock);
          pending = ioread8(reg.add(GPE_STS_REG_OFFSET));
          enabled = ioread8(reg.add(gpe_en_reg_offset(block_size))); }
        pending &= enabled;
        for bit_idx in 0..8 { if pending & (1 << bit_idx) != 0 {
            generic_handle_domain_irq((*priv_).gc.irq.domain, i * 8 + bit_idx);
        }}
        handled += (pending != 0) as i32;
    }
    irq_retval(handled)
}

/* UUID for GPE device _DSM: 079406e6-bdea-49cf-8563-03e2811901cb */
static NVL_GPE_DSM_GUID: Guid = Guid::init(0x079406e6, 0xbdea, 0x49cf,
    [0x85, 0x63, 0x03, 0xe2, 0x81, 0x19, 0x01, 0xcb]);

const DSM_GPE_MODE_REV: u32 = 1;
const DSM_GPE_MODE_FN_INDEX: u32 = 1;
const DSM_ENABLE_GPE_MODE: u64 = 1;

unsafe fn nvl_acpi_enable_gpe_mode(dev: *mut Device) -> i32 {
    let mut argv4: [AcpiObject; 2] = core::mem::zeroed();
    argv4[0].type_ = ACPI_TYPE_PACKAGE;
    argv4[0].package.count = 1;
    argv4[0].package.elements = &mut argv4[1];
    argv4[1].integer.type_ = ACPI_TYPE_INTEGER;
    argv4[1].integer.value = DSM_ENABLE_GPE_MODE;
    let obj = acpi_evaluate_dsm_typed(acpi_handle(dev), &NVL_GPE_DSM_GUID,
        DSM_GPE_MODE_REV, DSM_GPE_MODE_FN_INDEX, argv4.as_mut_ptr(), ACPI_TYPE_BUFFER);
    if obj.is_null() { return -EIO; }
    acpi_free(obj);
    0
}

// The remaining platform-driver registration and probe declarations mirror the
// C implementation and refer to kernel types and helpers supplied externally.
unsafe fn nvl_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if res.is_null() { return -ENXIO; }
    let ioresource_size = resource_size(res);
    if ioresource_size == 0 || ioresource_size % 2 != 0 || ioresource_size > 0x20 {
        return dev_err_probe(dev, -EINVAL, "invalid GPE block length, resource: %pR\n", res);
    }
    let regs = devm_ioport_map(dev, (*res).start, ioresource_size);
    if regs.is_null() { return -ENOMEM; }
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<NvlGpio>(), GFP_KERNEL) as *mut NvlGpio;
    if priv_.is_null() { return -ENOMEM; }
    raw_spin_lock_init(&mut (*priv_).lock);
    (*priv_).reg_base = regs as *mut u8;
    (*priv_).blk_size = ioresource_size;
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let ret = devm_request_irq(dev, irq, nvl_gpio_irq, IRQF_SHARED, dev_name(dev), priv_ as *mut _);
    if ret != 0 { return ret; }
    (*priv_).gc = NVL_GPIO_CHIP;
    (*priv_).gc.label = dev_name(dev); (*priv_).gc.parent = dev;
    (*priv_).gc.ngpio = gpe_reg_pin_count(ioresource_size); (*priv_).gc.base = -1;
    let girq = &mut (*priv_).gc.irq;
    gpio_irq_chip_set_chip(girq, &NVL_GPIO_IRQ_CHIP);
    girq.parent_handler = None; girq.num_parents = 0; girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE; girq.handler = handle_bad_irq;
    let ret = devm_gpiochip_add_data(dev, &mut (*priv_).gc, priv_ as *mut _);
    if ret != 0 { return ret; }
    nvl_acpi_enable_gpe_mode(dev)
}

static NVL_GPIO_ACPI_MATCH: [AcpiDeviceId; 2] = [AcpiDeviceId::new("INTC1114"), AcpiDeviceId::empty()];
static NVL_GPIO_DRIVER: PlatformDriver = PlatformDriver::new("gpio-novalake-events", &NVL_GPIO_ACPI_MATCH, nvl_gpio_probe);
module_platform_driver!(NVL_GPIO_DRIVER);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Alan Borzeszkowski <alan.borzeszkowski@linux.intel.com>");
// MODULE_DESCRIPTION("Intel Nova Lake ACPI GPIO events driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
