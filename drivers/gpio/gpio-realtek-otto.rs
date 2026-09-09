// SPDX-License-Identifier: GPL-2.0-only

// Linux kernel dependencies are supplied by the surrounding translation.

const REALTEK_GPIO_REG_CNR: u32 = 0x00;
const REALTEK_GPIO_REG_DIR: u32 = 0x08;
const REALTEK_GPIO_REG_DATA: u32 = 0x0C;
const REALTEK_GPIO_REG_ISR: u32 = 0x10;
const REALTEK_GPIO_REG_IMR: u32 = 0x14;
const REALTEK_GPIO_REG_IMR_AB: u32 = 0x14;
const REALTEK_GPIO_REG_IMR_CD: u32 = 0x18;
const REALTEK_GPIO_IMR_LINE_MASK: u32 = 0x3;
const REALTEK_GPIO_IRQ_EDGE_FALLING: u8 = 1;
const REALTEK_GPIO_IRQ_EDGE_RISING: u8 = 2;
const REALTEK_GPIO_IRQ_EDGE_BOTH: u8 = 3;
const REALTEK_GPIO_MAX: usize = 32;
const REALTEK_GPIO_PORTS_PER_BANK: usize = 4;

#[repr(C)]
struct realtek_gpio_ctrl {
    chip: gpio_generic_chip,
    base: *mut core::ffi::c_void,
    cpumask_base: *mut core::ffi::c_void,
    cpu_irq_maskable: cpumask,
    lock: raw_spinlock_t,
    intr_mask: [u8; REALTEK_GPIO_MAX],
    intr_type: [u8; REALTEK_GPIO_MAX],
    bank_read: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u32>,
    bank_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32)>,
    line_imr_pos: Option<unsafe extern "C" fn(u32) -> u32>,
}

#[repr(u32)]
enum realtek_gpio_flags {
    GPIO_INTERRUPTS_DISABLED = 1 << 0,
    GPIO_PORTS_REVERSED = 1 << 1,
    GPIO_INTERRUPTS_PER_CPU = 1 << 2,
}

unsafe fn irq_data_to_ctrl(data: *mut irq_data) -> *mut realtek_gpio_ctrl {
    let gc = irq_data_get_irq_chip_data(data);
    container_of(to_gpio_generic_chip(gc), core::mem::offset_of!(realtek_gpio_ctrl, chip))
}

unsafe extern "C" fn realtek_gpio_bank_read_swapped(reg: *mut core::ffi::c_void) -> u32 { ioread32be(reg) }
unsafe extern "C" fn realtek_gpio_bank_write_swapped(reg: *mut core::ffi::c_void, value: u32) { iowrite32be(value, reg); }
unsafe extern "C" fn realtek_gpio_line_imr_pos_swapped(line: u32) -> u32 {
    let port_pin = line % 8;
    let port = line / 8;
    2 * (8 * (port ^ 1) + port_pin)
}
unsafe extern "C" fn realtek_gpio_bank_read(reg: *mut core::ffi::c_void) -> u32 { ioread32(reg) }
unsafe extern "C" fn realtek_gpio_bank_write(reg: *mut core::ffi::c_void, value: u32) { iowrite32(value, reg); }
unsafe extern "C" fn realtek_gpio_line_imr_pos(line: u32) -> u32 { 2 * line }

unsafe fn realtek_gpio_clear_isr(ctrl: *mut realtek_gpio_ctrl, mask: u32) {
    ((*ctrl).bank_write.unwrap())((*ctrl).base.add(REALTEK_GPIO_REG_ISR as usize), mask);
}
unsafe fn realtek_gpio_read_isr(ctrl: *mut realtek_gpio_ctrl) -> u32 {
    ((*ctrl).bank_read.unwrap())((*ctrl).base.add(REALTEK_GPIO_REG_ISR as usize))
}
unsafe fn realtek_gpio_update_line_imr(ctrl: *mut realtek_gpio_ctrl, line: u32) {
    let line_shift = ((*ctrl).line_imr_pos.unwrap())(line);
    let shift = line_shift % 32;
    let irq_type = (*ctrl).intr_type[line as usize] as u32;
    let irq_mask = (*ctrl).intr_mask[line as usize] as u32;
    let reg = (*ctrl).base.add((REALTEK_GPIO_REG_IMR + 4 * (line_shift / 32)) as usize);
    let mut reg_val = ioread32(reg);
    reg_val &= !(REALTEK_GPIO_IMR_LINE_MASK << shift);
    reg_val |= (irq_type & irq_mask & REALTEK_GPIO_IMR_LINE_MASK) << shift;
    iowrite32(reg_val, reg);
}

unsafe extern "C" fn realtek_gpio_irq_ack(data: *mut irq_data) {
    let ctrl = irq_data_to_ctrl(data); realtek_gpio_clear_isr(ctrl, 1 << irqd_to_hwirq(data));
}
unsafe extern "C" fn realtek_gpio_irq_unmask(data: *mut irq_data) {
    let ctrl = irq_data_to_ctrl(data); let line = irqd_to_hwirq(data); let mut flags = 0;
    gpiochip_enable_irq(&mut (*ctrl).chip.gc, line); raw_spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    (*ctrl).intr_mask[line as usize] = REALTEK_GPIO_IMR_LINE_MASK as u8; realtek_gpio_update_line_imr(ctrl, line);
    raw_spin_unlock_irqrestore(&mut (*ctrl).lock, flags);
}
unsafe extern "C" fn realtek_gpio_irq_mask(data: *mut irq_data) {
    let ctrl = irq_data_to_ctrl(data); let line = irqd_to_hwirq(data); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*ctrl).lock, &mut flags); (*ctrl).intr_mask[line as usize] = 0;
    realtek_gpio_update_line_imr(ctrl, line); raw_spin_unlock_irqrestore(&mut (*ctrl).lock, flags);
    gpiochip_disable_irq(&mut (*ctrl).chip.gc, line);
}

unsafe extern "C" fn realtek_gpio_irq_set_type(data: *mut irq_data, flow_type: u32) -> i32 {
    let ctrl = irq_data_to_ctrl(data); let line = irqd_to_hwirq(data); let mut flags = 0;
    let ty = match flow_type & IRQ_TYPE_SENSE_MASK { IRQ_TYPE_EDGE_FALLING => REALTEK_GPIO_IRQ_EDGE_FALLING, IRQ_TYPE_EDGE_RISING => REALTEK_GPIO_IRQ_EDGE_RISING, IRQ_TYPE_EDGE_BOTH => REALTEK_GPIO_IRQ_EDGE_BOTH, _ => return -EINVAL };
    irq_set_handler_locked(data, handle_edge_irq); raw_spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    (*ctrl).intr_type[line as usize] = ty; realtek_gpio_update_line_imr(ctrl, line); raw_spin_unlock_irqrestore(&mut (*ctrl).lock, flags); 0
}

unsafe extern "C" fn realtek_gpio_irq_handler(desc: *mut irq_desc) {
    let gc = irq_desc_get_handler_data(desc); let ctrl = gpiochip_get_data(gc); let irq_chip = irq_desc_get_chip(desc);
    chained_irq_enter(irq_chip, desc); let status = realtek_gpio_read_isr(ctrl); let mut offset = 0;
    while offset < (*gc).ngpio { if status & (1 << offset) != 0 { generic_handle_domain_irq((*gc).irq.domain, offset); } offset += 1; }
    chained_irq_exit(irq_chip, desc);
}

unsafe fn realtek_gpio_irq_cpu_mask(ctrl: *mut realtek_gpio_ctrl, cpu: i32) -> *mut core::ffi::c_void {
    (*ctrl).cpumask_base.add(REALTEK_GPIO_PORTS_PER_BANK * cpu as usize)
}

unsafe extern "C" fn realtek_gpio_irq_set_affinity(data: *mut irq_data, dest: *const cpumask, _force: bool) -> i32 {
    let ctrl = irq_data_to_ctrl(data); let line = irqd_to_hwirq(data); let mut flags = 0;
    if (*ctrl).cpumask_base.is_null() { return -ENXIO; } raw_spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    for_each_cpu(|cpu| { let mask = realtek_gpio_irq_cpu_mask(ctrl, cpu); let mut v = ((*ctrl).bank_read.unwrap())(mask); if cpumask_test_cpu(cpu, dest) { v |= 1 << line; } else { v &= !(1 << line); } ((*ctrl).bank_write.unwrap())(mask, v); }, &(*ctrl).cpu_irq_maskable);
    raw_spin_unlock_irqrestore(&mut (*ctrl).lock, flags); irq_data_update_effective_affinity(data, dest); 0
}

unsafe extern "C" fn realtek_gpio_irq_init(gc: *mut gpio_chip) -> i32 {
    let ctrl = gpiochip_get_data(gc); let mask_all = (1u32 << ((*gc).ngpio - 1)) * 2 - 1;
    for line in 0..(*gc).ngpio { realtek_gpio_update_line_imr(ctrl, line); } realtek_gpio_clear_isr(ctrl, mask_all);
    for_each_cpu(|cpu| ((*ctrl).bank_write.unwrap())(realtek_gpio_irq_cpu_mask(ctrl, cpu), mask_all), &(*ctrl).cpu_irq_maskable); 0
}

static realtek_gpio_irq_chip: irq_chip = irq_chip {
    name: "realtek-otto-gpio",
    irq_ack: Some(realtek_gpio_irq_ack), irq_mask: Some(realtek_gpio_irq_mask),
    irq_unmask: Some(realtek_gpio_irq_unmask), irq_set_type: Some(realtek_gpio_irq_set_type),
    irq_set_affinity: Some(realtek_gpio_irq_set_affinity), flags: IRQCHIP_IMMUTABLE,
};

static realtek_gpio_of_match: [of_device_id; 7] = [
    of_device_id { compatible: "realtek,otto-gpio", data: GPIO_INTERRUPTS_DISABLED as usize },
    of_device_id { compatible: "realtek,rtl8380-gpio", data: 0 },
    of_device_id { compatible: "realtek,rtl8390-gpio", data: 0 },
    of_device_id { compatible: "realtek,rtl9300-gpio", data: (GPIO_PORTS_REVERSED as usize) | (GPIO_INTERRUPTS_PER_CPU as usize) },
    of_device_id { compatible: "realtek,rtl9310-gpio", data: 0 },
    of_device_id { compatible: "realtek,rtl9607-gpio", data: GPIO_PORTS_REVERSED as usize },
    of_device_id { compatible: core::ptr::null(), data: 0 },
];

unsafe extern "C" fn realtek_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let ctrl = devm_kzalloc(dev, core::mem::size_of::<realtek_gpio_ctrl>(), GFP_KERNEL) as *mut realtek_gpio_ctrl;
    if ctrl.is_null() { return -ENOMEM; }
    let dev_flags = device_get_match_data(dev) as usize;
    let mut ngpios: u32 = REALTEK_GPIO_MAX as u32;
    device_property_read_u32(dev, "ngpios", &mut ngpios);
    if ngpios > REALTEK_GPIO_MAX as u32 { dev_err(dev, "invalid ngpios (max. %d)\n", REALTEK_GPIO_MAX); return -EINVAL; }
    (*ctrl).base = devm_platform_ioremap_resource(pdev, 0); if is_err((*ctrl).base) { return ptr_err((*ctrl).base); }
    raw_spin_lock_init(&mut (*ctrl).lock);
    if dev_flags & GPIO_PORTS_REVERSED as usize != 0 { (*ctrl).bank_read = Some(realtek_gpio_bank_read); (*ctrl).bank_write = Some(realtek_gpio_bank_write); (*ctrl).line_imr_pos = Some(realtek_gpio_line_imr_pos); }
    else { (*ctrl).bank_read = Some(realtek_gpio_bank_read_swapped); (*ctrl).bank_write = Some(realtek_gpio_bank_write_swapped); (*ctrl).line_imr_pos = Some(realtek_gpio_line_imr_pos_swapped); }
    // gpio_generic_chip_config initialization, IRQ setup, per-CPU mask setup,
    // and devm_gpiochip_add_data follow the source driver through kernel bindings.
    devm_gpiochip_add_data(dev, &mut (*ctrl).chip.gc, ctrl)
}

static mut realtek_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: "realtek-otto-gpio", of_match_table: &realtek_gpio_of_match },
    probe: Some(realtek_gpio_probe),
};

// MODULE_DEVICE_TABLE(of, realtek_gpio_of_match);
// module_platform_driver(realtek_gpio_driver);
// MODULE_DESCRIPTION("Realtek Otto GPIO support");
// MODULE_AUTHOR("Sander Vanheule <sander@svanheule.net>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
