// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Jamie Iles
 *
 * All enquiries to support@picochip.com
 */

// Linux kernel dependencies are supplied by the surrounding kernel bindings.

const GPIO_SWPORTA_DR: u32 = 0x00;
const GPIO_SWPORTA_DDR: u32 = 0x04;
const GPIO_SWPORTB_DR: u32 = 0x0c;
const GPIO_SWPORTB_DDR: u32 = 0x10;
const GPIO_SWPORTC_DR: u32 = 0x18;
const GPIO_SWPORTC_DDR: u32 = 0x1c;
const GPIO_SWPORTD_DR: u32 = 0x24;
const GPIO_SWPORTD_DDR: u32 = 0x28;
const GPIO_INTEN: u32 = 0x30;
const GPIO_INTMASK: u32 = 0x34;
const GPIO_INTTYPE_LEVEL: u32 = 0x38;
const GPIO_INT_POLARITY: u32 = 0x3c;
const GPIO_INTSTATUS: u32 = 0x40;
const GPIO_PORTA_DEBOUNCE: u32 = 0x48;
const GPIO_PORTA_EOI: u32 = 0x4c;
const GPIO_EXT_PORTA: u32 = 0x50;
const GPIO_EXT_PORTB: u32 = 0x54;
const GPIO_EXT_PORTC: u32 = 0x58;
const GPIO_EXT_PORTD: u32 = 0x5c;

const DWAPB_DRIVER_NAME: &str = "gpio-dwapb";
const DWAPB_MAX_PORTS: usize = 4;
const DWAPB_MAX_GPIOS: usize = 32;
const GPIO_EXT_PORT_STRIDE: u32 = 0x04;
const GPIO_SWPORT_DR_STRIDE: u32 = 0x0c;
const GPIO_SWPORT_DDR_STRIDE: u32 = 0x0c;
const GPIO_REG_OFFSET_V1: u32 = 0;
const GPIO_REG_OFFSET_V2: u32 = 1;
const GPIO_REG_OFFSET_MASK: u32 = BIT(0);
const GPIO_INTMASK_V2: u32 = 0x44;
const GPIO_INTTYPE_LEVEL_V2: u32 = 0x34;
const GPIO_INT_POLARITY_V2: u32 = 0x38;
const GPIO_INTSTATUS_V2: u32 = 0x3c;
const GPIO_PORTA_EOI_V2: u32 = 0x40;
const DWAPB_NR_CLOCKS: usize = 2;

#[repr(C)]
pub struct dwapb_port_property {
    pub fwnode: *mut fwnode_handle,
    pub idx: u32,
    pub ngpio: u32,
    pub gpio_base: u32,
    pub irq: [i32; DWAPB_MAX_GPIOS],
}

#[repr(C)]
pub struct dwapb_platform_data {
    pub nports: u32,
    pub properties: [dwapb_port_property; 0],
}

#[repr(C)]
pub struct dwapb_context {
    pub data: u32, pub dir: u32, pub ext: u32, pub int_en: u32,
    pub int_mask: u32, pub int_type: u32, pub int_pol: u32,
    pub int_deb: u32, pub wake_en: u32,
}

#[repr(C)]
pub struct dwapb_gpio_port_irqchip {
    pub nr_irqs: u32,
    pub irq: [u32; DWAPB_MAX_GPIOS],
}

#[repr(C)]
pub struct dwapb_gpio_port {
    pub chip: gpio_generic_chip,
    pub pirq: *mut dwapb_gpio_port_irqchip,
    pub gpio: *mut dwapb_gpio,
    pub ctx: *mut dwapb_context,
    pub idx: u32,
}

#[repr(C)]
pub struct dwapb_gpio {
    pub dev: *mut device,
    pub regs: *mut core::ffi::c_void,
    pub nr_ports: u32,
    pub flags: u32,
    pub rst: *mut reset_control,
    pub clks: [clk_bulk_data; DWAPB_NR_CLOCKS],
    pub clocks_on_for_wake: bool,
    pub ports: [dwapb_gpio_port; 0],
}

#[inline]
unsafe fn to_dwapb_gpio(gc: *mut gpio_chip) -> *mut dwapb_gpio {
    container_of(to_gpio_generic_chip(gc), dwapb_gpio_port, chip).as_ref().unwrap().gpio
}

#[inline]
fn gpio_reg_v2_convert(offset: u32) -> u32 {
    match offset {
        GPIO_INTMASK => GPIO_INTMASK_V2,
        GPIO_INTTYPE_LEVEL => GPIO_INTTYPE_LEVEL_V2,
        GPIO_INT_POLARITY => GPIO_INT_POLARITY_V2,
        GPIO_INTSTATUS => GPIO_INTSTATUS_V2,
        GPIO_PORTA_EOI => GPIO_PORTA_EOI_V2,
        _ => offset,
    }
}

#[inline]
unsafe fn gpio_reg_convert(gpio: *mut dwapb_gpio, offset: u32) -> u32 {
    if ((*gpio).flags & GPIO_REG_OFFSET_MASK) == GPIO_REG_OFFSET_V2 { gpio_reg_v2_convert(offset) } else { offset }
}

#[inline]
unsafe fn dwapb_read(gpio: *mut dwapb_gpio, offset: u32) -> u32 {
    let chip = &mut (*gpio.add(1) as *mut dwapb_gpio_port).as_mut().unwrap().chip;
    gpio_generic_read_reg(chip, (*gpio).regs.add(gpio_reg_convert(gpio, offset) as usize))
}

#[inline]
unsafe fn dwapb_write(gpio: *mut dwapb_gpio, offset: u32, val: u32) {
    let chip = &mut (*gpio.add(1) as *mut dwapb_gpio_port).as_mut().unwrap().chip;
    gpio_generic_write_reg(chip, (*gpio).regs.add(gpio_reg_convert(gpio, offset) as usize), val);
}

unsafe fn dwapb_offs_to_port(gpio: *mut dwapb_gpio, offs: u32) -> *mut dwapb_gpio_port {
    for i in 0..(*gpio).nr_ports as usize {
        let port = (*gpio).ports.as_mut_ptr().add(i);
        if (*port).idx == offs / DWAPB_MAX_GPIOS as u32 { return port; }
    }
    core::ptr::null_mut()
}

unsafe fn dwapb_toggle_trigger(gpio: *mut dwapb_gpio, offs: u32) {
    let port = dwapb_offs_to_port(gpio, offs);
    if port.is_null() { return; }
    let gc = &mut (*port).chip.gc;
    let mut pol = dwapb_read(gpio, GPIO_INT_POLARITY);
    let val = ((*gc).get.unwrap())(gc, offs % DWAPB_MAX_GPIOS as u32);
    if val != 0 { pol &= !BIT(offs); } else { pol |= BIT(offs); }
    dwapb_write(gpio, GPIO_INT_POLARITY, pol);
}

unsafe fn dwapb_irq_init_hw(gc: *mut gpio_chip) -> i32 {
    let gpio = to_dwapb_gpio(gc);
    dwapb_write(gpio, GPIO_INTEN, 0);
    dwapb_write(gpio, GPIO_INTMASK, 0xffff_ffff);
    dwapb_write(gpio, GPIO_PORTA_EOI, 0xffff_ffff);
    0
}

unsafe fn dwapb_do_irq(gpio: *mut dwapb_gpio) -> u32 {
    let gen_gc = &mut (*gpio.ports.as_mut_ptr()).chip;
    let irq_status = dwapb_read(gpio, GPIO_INTSTATUS);
    for hwirq in 0..DWAPB_MAX_GPIOS {
        if irq_status & BIT(hwirq as u32) == 0 { continue; }
        let gpio_irq = irq_find_mapping((*gen_gc).gc.irq.domain, hwirq as u32);
        let irq_type = irq_get_trigger_type(gpio_irq);
        generic_handle_irq(gpio_irq);
        if (irq_type & IRQ_TYPE_SENSE_MASK) == IRQ_TYPE_EDGE_BOTH { dwapb_toggle_trigger(gpio, hwirq as u32); }
    }
    irq_status
}

unsafe fn dwapb_irq_handler(desc: *mut irq_desc) {
    let gpio = irq_desc_get_handler_data(desc) as *mut dwapb_gpio;
    let chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc); dwapb_do_irq(gpio); chained_irq_exit(chip, desc);
}

unsafe fn dwapb_irq_handler_mfd(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    IRQ_RETVAL(dwapb_do_irq(dev_id as *mut dwapb_gpio))
}

unsafe fn dwapb_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let val = BIT(irqd_to_hwirq(d));
    guard_gpio_generic_lock_irqsave(gen_gc);
    dwapb_write(gpio, GPIO_PORTA_EOI, val);
}

unsafe fn dwapb_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let hwirq = irqd_to_hwirq(d);
    scoped_guard_gpio_generic_lock_irqsave(gen_gc);
    let val = dwapb_read(gpio, GPIO_INTMASK) | BIT(hwirq); dwapb_write(gpio, GPIO_INTMASK, val);
    gpiochip_disable_irq(gc, hwirq);
}

unsafe fn dwapb_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let hwirq = irqd_to_hwirq(d); gpiochip_enable_irq(gc, hwirq);
    guard_gpio_generic_lock_irqsave(gen_gc);
    let val = dwapb_read(gpio, GPIO_INTMASK) & !BIT(hwirq); dwapb_write(gpio, GPIO_INTMASK, val);
}

unsafe fn dwapb_irq_enable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let hwirq = irqd_to_hwirq(d); guard_gpio_generic_lock_irqsave(gen_gc);
    dwapb_write(gpio, GPIO_INTEN, dwapb_read(gpio, GPIO_INTEN) | BIT(hwirq));
    dwapb_write(gpio, GPIO_INTMASK, dwapb_read(gpio, GPIO_INTMASK) & !BIT(hwirq));
}

unsafe fn dwapb_irq_disable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let hwirq = irqd_to_hwirq(d); guard_gpio_generic_lock_irqsave(gen_gc);
    dwapb_write(gpio, GPIO_INTMASK, dwapb_read(gpio, GPIO_INTMASK) | BIT(hwirq));
    dwapb_write(gpio, GPIO_INTEN, dwapb_read(gpio, GPIO_INTEN) & !BIT(hwirq));
}

unsafe fn dwapb_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let gen_gc = to_gpio_generic_chip(gc);
    let gpio = to_dwapb_gpio(gc); let bit = irqd_to_hwirq(d); guard_gpio_generic_lock_irqsave(gen_gc);
    let mut level = dwapb_read(gpio, GPIO_INTTYPE_LEVEL); let mut polarity = dwapb_read(gpio, GPIO_INT_POLARITY);
    match type_ {
        IRQ_TYPE_EDGE_BOTH => { level |= BIT(bit); dwapb_toggle_trigger(gpio, bit); }
        IRQ_TYPE_EDGE_RISING => { level |= BIT(bit); polarity |= BIT(bit); }
        IRQ_TYPE_EDGE_FALLING => { level |= BIT(bit); polarity &= !BIT(bit); }
        IRQ_TYPE_LEVEL_HIGH => { level &= !BIT(bit); polarity |= BIT(bit); }
        IRQ_TYPE_LEVEL_LOW => { level &= !BIT(bit); polarity &= !BIT(bit); }
        _ => {}
    }
    if type_ & IRQ_TYPE_LEVEL_MASK != 0 { irq_set_handler_locked(d, handle_level_irq); }
    else if type_ & IRQ_TYPE_EDGE_BOTH != 0 { irq_set_handler_locked(d, handle_edge_irq); }
    dwapb_write(gpio, GPIO_INTTYPE_LEVEL, level);
    if type_ != IRQ_TYPE_EDGE_BOTH { dwapb_write(gpio, GPIO_INT_POLARITY, polarity); }
    0
}

unsafe fn dwapb_irq_set_wake(d: *mut irq_data, enable: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let gpio = to_dwapb_gpio(gc);
    let ctx = (*gpio.ports.as_mut_ptr()).ctx; let bit = irqd_to_hwirq(d); let mut wake_en = (*ctx).wake_en;
    if enable != 0 { wake_en |= BIT(bit); } else { wake_en &= !BIT(bit); }
    (*ctx).wake_en = wake_en; 0
}

unsafe fn dwapb_gpio_set_debounce(gc: *mut gpio_chip, offset: u32, debounce: u32) -> i32 {
    let port = gpiochip_get_data(gc) as *mut dwapb_gpio_port; let gen_gc = to_gpio_generic_chip(gc);
    let gpio = (*port).gpio; guard_gpio_generic_lock_irqsave(gen_gc);
    let mut val = dwapb_read(gpio, GPIO_PORTA_DEBOUNCE); if debounce != 0 { val |= BIT(offset); } else { val &= !BIT(offset); }
    dwapb_write(gpio, GPIO_PORTA_DEBOUNCE, val); 0
}

unsafe fn dwapb_gpio_set_config(gc: *mut gpio_chip, offset: u32, config: c_ulong) -> i32 {
    if pinconf_to_config_param(config) == PIN_CONFIG_INPUT_DEBOUNCE { return dwapb_gpio_set_debounce(gc, offset, pinconf_to_config_argument(config)); }
    gpiochip_generic_config(gc, offset, config)
}

unsafe fn dwapb_convert_irqs(pirq: *mut dwapb_gpio_port_irqchip, pp: *mut dwapb_port_property) -> i32 {
    for i in 0..(*pp).ngpio as usize { if (*pp).irq[i] != 0 { (*pirq).irq[(*pirq).nr_irqs as usize] = (*pp).irq[i] as u32; (*pirq).nr_irqs += 1; } }
    if (*pirq).nr_irqs != 0 { 0 } else { -ENOENT }
}

extern "C" {
    fn dwapb_get_irq(dev: *mut device, fwnode: *mut fwnode_handle, pp: *mut dwapb_port_property);
    fn dwapb_gpio_get_pdata(dev: *mut device) -> *mut dwapb_platform_data;
    fn dwapb_assert_reset(data: *mut core::ffi::c_void);
    fn dwapb_get_reset(gpio: *mut dwapb_gpio) -> i32;
    fn dwapb_disable_clks(data: *mut core::ffi::c_void);
    fn dwapb_get_clks(gpio: *mut dwapb_gpio) -> i32;
    fn dwapb_gpio_add_port(gpio: *mut dwapb_gpio, pp: *mut dwapb_port_property, offs: u32) -> i32;
    fn dwapb_gpio_probe(pdev: *mut platform_device) -> i32;
    fn dwapb_gpio_suspend(dev: *mut device) -> i32;
    fn dwapb_gpio_suspend_noirq(dev: *mut device) -> i32;
    fn dwapb_gpio_resume_noirq(dev: *mut device) -> i32;
    fn dwapb_gpio_resume(dev: *mut device) -> i32;
}

// Direct equivalents of the C device tables and PM/driver objects.  The
// concrete kernel structure definitions and registration macros are external.
#[repr(C)]
struct dwapb_of_device_id { compatible: *const u8, data: *mut core::ffi::c_void }
#[repr(C)]
struct dwapb_acpi_device_id { id: *const u8, driver_data: u64 }

static DWAPB_OF_MATCH: [dwapb_of_device_id; 3] = [
    dwapb_of_device_id { compatible: b"snps,dw-apb-gpio\0".as_ptr(), data: GPIO_REG_OFFSET_V1 as usize as *mut _ },
    dwapb_of_device_id { compatible: b"apm,xgene-gpio-v2\0".as_ptr(), data: GPIO_REG_OFFSET_V2 as usize as *mut _ },
    dwapb_of_device_id { compatible: core::ptr::null(), data: core::ptr::null_mut() },
];
static DWAPB_ACPI_MATCH: [dwapb_acpi_device_id; 6] = [
    dwapb_acpi_device_id { id: b"HISI0181\0".as_ptr(), driver_data: GPIO_REG_OFFSET_V1 as u64 },
    dwapb_acpi_device_id { id: b"APMC0D07\0".as_ptr(), driver_data: GPIO_REG_OFFSET_V1 as u64 },
    dwapb_acpi_device_id { id: b"APMC0D81\0".as_ptr(), driver_data: GPIO_REG_OFFSET_V2 as u64 },
    dwapb_acpi_device_id { id: b"FUJI200A\0".as_ptr(), driver_data: GPIO_REG_OFFSET_V1 as u64 },
    dwapb_acpi_device_id { id: b"LECA0001\0".as_ptr(), driver_data: GPIO_REG_OFFSET_V1 as u64 },
    dwapb_acpi_device_id { id: core::ptr::null(), driver_data: 0 },
];

// C headers and build-time registration macros: MODULE_DEVICE_TABLE,
// SYSTEM_SLEEP_PM_OPS, NOIRQ_SYSTEM_SLEEP_OPS, module_platform_driver, and
// MODULE_* metadata are intentionally retained as linkage/dependency intent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
