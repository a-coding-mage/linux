// SPDX-License-Identifier: GPL-2.0-only
/*
 * Emma Mobile GPIO Support - GIO
 *
 *  Copyright (C) 2012 Magnus Damm
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct em_gio_priv {
    pub base0: *mut core::ffi::c_void,
    pub base1: *mut core::ffi::c_void,
    pub sense_lock: spinlock_t,
    pub pdev: *mut platform_device,
    pub gpio_chip: gpio_chip,
    pub irq_chip: irq_chip,
    pub irq_domain: *mut irq_domain,
}

pub const GIO_E1: i32 = 0x00;
pub const GIO_E0: i32 = 0x04;
pub const GIO_EM: i32 = 0x04;
pub const GIO_OL: i32 = 0x08;
pub const GIO_OH: i32 = 0x0c;
pub const GIO_I: i32 = 0x10;
pub const GIO_IIA: i32 = 0x14;
pub const GIO_IEN: i32 = 0x18;
pub const GIO_IDS: i32 = 0x1c;
pub const GIO_IIM: i32 = 0x1c;
pub const GIO_RAW: i32 = 0x20;
pub const GIO_MST: i32 = 0x24;
pub const GIO_IIR: i32 = 0x28;
pub const GIO_IDT0: i32 = 0x40;
pub const GIO_IDT1: i32 = 0x44;
pub const GIO_IDT2: i32 = 0x48;
pub const GIO_IDT3: i32 = 0x4c;
pub const GIO_RAWBL: i32 = 0x50;
pub const GIO_RAWBH: i32 = 0x54;
pub const GIO_IRBL: i32 = 0x58;
pub const GIO_IRBH: i32 = 0x5c;

#[inline]
fn gio_idt(n: i32) -> i32 { GIO_IDT0 + n * 4 }

#[inline]
unsafe fn em_gio_read(p: *mut em_gio_priv, offs: i32) -> usize {
    if offs < GIO_IDT0 { ioread32((*p).base0.add(offs as usize)) as usize }
    else { ioread32((*p).base1.add((offs - GIO_IDT0) as usize)) as usize }
}

#[inline]
unsafe fn em_gio_write(p: *mut em_gio_priv, offs: i32, value: usize) {
    if offs < GIO_IDT0 { iowrite32(value as u32, (*p).base0.add(offs as usize)); }
    else { iowrite32(value as u32, (*p).base1.add((offs - GIO_IDT0) as usize)); }
}

unsafe fn em_gio_irq_disable(d: *mut irq_data) {
    let p = irq_data_get_irq_chip_data(d);
    em_gio_write(p, GIO_IDS, 1usize << irqd_to_hwirq(d));
}

unsafe fn em_gio_irq_enable(d: *mut irq_data) {
    let p = irq_data_get_irq_chip_data(d);
    em_gio_write(p, GIO_IEN, 1usize << irqd_to_hwirq(d));
}

unsafe fn em_gio_irq_reqres(d: *mut irq_data) -> i32 {
    let p = irq_data_get_irq_chip_data(d);
    let ret = gpiochip_lock_as_irq(&mut (*p).gpio_chip, irqd_to_hwirq(d));
    if ret != 0 { dev_err((*p).gpio_chip.parent, "unable to lock HW IRQ %lu for IRQ\n", irqd_to_hwirq(d)); return ret; }
    0
}

unsafe fn em_gio_irq_relres(d: *mut irq_data) {
    let p = irq_data_get_irq_chip_data(d);
    gpiochip_unlock_as_irq(&mut (*p).gpio_chip, irqd_to_hwirq(d));
}

const fn gio_async(x: u8) -> u8 { x + 8 }
static mut EM_GIO_SENSE_TABLE: [u8; IRQ_TYPE_SENSE_MASK as usize + 1] = [0; IRQ_TYPE_SENSE_MASK as usize + 1];

unsafe fn em_gio_irq_set_type(d: *mut irq_data, ty: u32) -> i32 {
    let value = EM_GIO_SENSE_TABLE[(ty & IRQ_TYPE_SENSE_MASK) as usize];
    let p = irq_data_get_irq_chip_data(d);
    let offset = irqd_to_hwirq(d) as usize;
    if value == 0 { return -22; }
    let reg = gio_idt((offset >> 3) as i32);
    let shift = ((offset & 7) << 4) as usize;
    let mut flags = 0usize;
    pr_debug!("gio: sense irq = %d, mode = %d\n", offset, value);
    spin_lock_irqsave(&mut (*p).sense_lock, &mut flags);
    let mut tmp = em_gio_read(p, GIO_IIA); em_gio_write(p, GIO_IIA, tmp & !(1usize << offset));
    tmp = em_gio_read(p, reg); tmp = (tmp & !(0xfusize << shift)) | ((value as usize) << shift); em_gio_write(p, reg, tmp);
    em_gio_write(p, GIO_IIR, 1usize << offset);
    tmp = em_gio_read(p, GIO_IIA); em_gio_write(p, GIO_IIA, tmp | (1usize << offset));
    spin_unlock_irqrestore(&mut (*p).sense_lock, flags); 0
}

unsafe fn em_gio_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let p = dev_id as *mut em_gio_priv; let mut handled = 0;
    loop { let pending = em_gio_read(p, GIO_MST); if pending == 0 { break; } let offset = pending.trailing_zeros(); em_gio_write(p, GIO_IIR, 1usize << offset); generic_handle_domain_irq((*p).irq_domain, offset); handled += 1; }
    if handled != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

#[inline] unsafe fn gpio_to_priv(chip: *mut gpio_chip) -> *mut em_gio_priv { gpiochip_get_data(chip) }

unsafe fn em_gio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 { em_gio_write(gpio_to_priv(chip), GIO_E0, 1usize << offset); 0 }
unsafe fn em_gio_get(chip: *mut gpio_chip, offset: u32) -> i32 { ((em_gio_read(gpio_to_priv(chip), GIO_I) & (1usize << offset)) != 0) as i32 }
unsafe fn __em_gio_set(chip: *mut gpio_chip, reg: u32, shift: u32, value: i32) { em_gio_write(gpio_to_priv(chip), reg as i32, (1usize << (shift + 16)) | ((value as usize) << shift)); }
unsafe fn em_gio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 { if offset < 16 { __em_gio_set(chip, GIO_OL as u32, offset, value); } else { __em_gio_set(chip, GIO_OH as u32, offset - 16, value); } 0 }
unsafe fn em_gio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 { em_gio_set(chip, offset, value); em_gio_write(gpio_to_priv(chip), GIO_E1, 1usize << offset); 0 }
unsafe fn em_gio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 { irq_create_mapping((*gpio_to_priv(chip)).irq_domain, offset) }
unsafe fn em_gio_free(chip: *mut gpio_chip, offset: u32) { pinctrl_gpio_free(chip, offset); em_gio_direction_input(chip, offset); }

// Remaining driver registration and probe declarations are provided by the kernel bindings.
+

unsafe fn em_gio_irq_domain_map(h: *mut irq_domain, irq: u32, hwirq: irq_hw_number_t) -> i32 {
    let p = (*h).host_data as *mut em_gio_priv;
    pr_debug!("gio: map hw irq = %d, irq = %d\n", hwirq as i32, irq);
    irq_set_chip_data(irq, (*h).host_data);
    irq_set_chip_and_handler(irq, &mut (*p).irq_chip, handle_level_irq);
    0
}

#[repr(C)]
static mut EM_GIO_IRQ_DOMAIN_OPS: irq_domain_ops = irq_domain_ops {
    map: Some(em_gio_irq_domain_map),
    xlate: Some(irq_domain_xlate_twocell),
};

unsafe fn em_gio_irq_domain_remove(data: *mut core::ffi::c_void) {
    irq_domain_remove(data as *mut irq_domain);
}

unsafe fn em_gio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let p = devm_kzalloc(dev, core::mem::size_of::<em_gio_priv>(), GFP_KERNEL) as *mut em_gio_priv;
    if p.is_null() { return -12; }
    (*p).pdev = pdev;
    platform_set_drvdata(pdev, p as *mut core::ffi::c_void);
    spin_lock_init(&mut (*p).sense_lock);

    let irq0 = platform_get_irq(pdev, 0); if irq0 < 0 { return irq0; }
    let irq1 = platform_get_irq(pdev, 1); if irq1 < 0 { return irq1; }
    (*p).base0 = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*p).base0) { return ptr_err((*p).base0); }
    (*p).base1 = devm_platform_ioremap_resource(pdev, 1);
    if is_err((*p).base1) { return ptr_err((*p).base1); }

    let mut ngpios = 0u32;
    if of_property_read_u32((*dev).of_node, "ngpios", &mut ngpios) != 0 { dev_err(dev, "Missing ngpios OF property\n"); return -22; }

    let gpio_chip = &mut (*p).gpio_chip;
    gpio_chip.direction_input = Some(em_gio_direction_input);
    gpio_chip.get = Some(em_gio_get);
    gpio_chip.direction_output = Some(em_gio_direction_output);
    gpio_chip.set = Some(em_gio_set);
    gpio_chip.to_irq = Some(em_gio_to_irq);
    gpio_chip.request = Some(pinctrl_gpio_request);
    gpio_chip.free = Some(em_gio_free);
    gpio_chip.label = dev_name(dev);
    gpio_chip.parent = dev;
    gpio_chip.owner = THIS_MODULE;
    gpio_chip.base = -1;
    gpio_chip.ngpio = ngpios;

    let irq_chip = &mut (*p).irq_chip;
    irq_chip.name = "gpio-em";
    irq_chip.irq_mask = Some(em_gio_irq_disable);
    irq_chip.irq_unmask = Some(em_gio_irq_enable);
    irq_chip.irq_set_type = Some(em_gio_irq_set_type);
    irq_chip.irq_request_resources = Some(em_gio_irq_reqres);
    irq_chip.irq_release_resources = Some(em_gio_irq_relres);
    irq_chip.flags = IRQCHIP_SKIP_SET_WAKE | IRQCHIP_MASK_ON_SUSPEND;

    (*p).irq_domain = irq_domain_create_simple(dev_fwnode(dev), ngpios, 0, &EM_GIO_IRQ_DOMAIN_OPS, p as *mut core::ffi::c_void);
    if (*p).irq_domain.is_null() { dev_err(dev, "cannot initialize irq domain\n"); return -6; }
    let ret = devm_add_action_or_reset(dev, Some(em_gio_irq_domain_remove), (*p).irq_domain as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    if devm_request_irq(dev, irq0, Some(em_gio_irq_handler), 0, dev_name(dev), p as *mut core::ffi::c_void) != 0 { dev_err(dev, "failed to request low IRQ\n"); return -2; }
    if devm_request_irq(dev, irq1, Some(em_gio_irq_handler), 0, dev_name(dev), p as *mut core::ffi::c_void) != 0 { dev_err(dev, "failed to request high IRQ\n"); return -2; }
    let ret = devm_gpiochip_add_data(dev, gpio_chip, p as *mut core::ffi::c_void);
    if ret != 0 { dev_err(dev, "failed to add GPIO controller\n"); return ret; }
    0
}

#[repr(C)]
pub struct of_device_id { pub compatible: *const u8 }
static mut EM_GIO_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: b"renesas,em-gio\0".as_ptr() }, of_device_id { compatible: core::ptr::null() }
];
#[repr(C)]
pub struct platform_driver { pub probe: Option<unsafe fn(*mut platform_device) -> i32>, pub name: *const u8, pub of_match_table: *mut of_device_id }
static mut EM_GIO_DEVICE_DRIVER: platform_driver = platform_driver { probe: Some(em_gio_probe), name: b"em_gio\0".as_ptr(), of_match_table: EM_GIO_DT_IDS.as_mut_ptr() };

unsafe fn em_gio_init() -> i32 { platform_driver_register(&mut EM_GIO_DEVICE_DRIVER) }
unsafe fn em_gio_exit() { platform_driver_unregister(&mut EM_GIO_DEVICE_DRIVER); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
