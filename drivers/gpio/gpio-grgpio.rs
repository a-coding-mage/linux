// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Aeroflex Gaisler GRGPIO General Purpose I/O cores.
 *
 * 2013 (c) Aeroflex Gaisler AB
 *
 * This driver supports the GRGPIO GPIO core available in the GRLIB VHDL
 * IP core library.
 *
 * Full documentation of the GRGPIO core can be found here:
 * http://www.gaisler.com/products/grlib/grip.pdf
 *
 * See "Documentation/devicetree/bindings/gpio/gpio-grgpio.txt" for
 * information on open firmware properties.
 *
 * Contributors: Andreas Larsson <andreas@gaisler.com>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const GRGPIO_MAX_NGPIO: usize = 32;
const GRGPIO_DATA: usize = 0x00;
const GRGPIO_OUTPUT: usize = 0x04;
const GRGPIO_DIR: usize = 0x08;
const GRGPIO_IMASK: usize = 0x0c;
const GRGPIO_IPOL: usize = 0x10;
const GRGPIO_IEDGE: usize = 0x14;
const GRGPIO_BYPASS: usize = 0x18;
const GRGPIO_IMAP_BASE: usize = 0x20;

#[repr(C)]
struct grgpio_uirq {
    refcnt: atomic_t,
    uirq: u8,
}

#[repr(C)]
struct grgpio_lirq {
    index: i8,
    irq: u8,
}

#[repr(C)]
struct grgpio_priv {
    chip: gpio_generic_chip,
    regs: *mut core::ffi::c_void,
    dev: *mut device,
    imask: u32,
    domain: *mut irq_domain,
    uirqs: [grgpio_uirq; GRGPIO_MAX_NGPIO],
    lirqs: [grgpio_lirq; GRGPIO_MAX_NGPIO],
}

unsafe fn grgpio_set_imask(priv_: *mut grgpio_priv, offset: u32, val: i32) {
    if val != 0 {
        (*priv_).imask |= 1u32.wrapping_shl(offset);
    } else {
        (*priv_).imask &= !(1u32.wrapping_shl(offset));
    }
    gpio_generic_write_reg(&mut (*priv_).chip, (*priv_).regs.add(GRGPIO_IMASK), (*priv_).imask);
}

unsafe fn grgpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut grgpio_priv;
    if offset >= (*gc).ngpio || (*priv_).lirqs[offset as usize].index < 0 {
        return -ENXIO;
    }
    irq_create_mapping((*priv_).domain, offset)
}

unsafe fn grgpio_irq_set_type(d: *mut irq_data, ty: u32) -> i32 {
    let priv_ = irq_data_get_irq_chip_data(d) as *mut grgpio_priv;
    let mask = 1u32.wrapping_shl((*d).hwirq);
    let (pol, edge) = match ty {
        IRQ_TYPE_LEVEL_LOW => (0, 0),
        IRQ_TYPE_LEVEL_HIGH => (mask, 0),
        IRQ_TYPE_EDGE_FALLING => (0, mask),
        IRQ_TYPE_EDGE_RISING => (mask, mask),
        _ => return -EINVAL,
    };
    let _guard = gpio_generic_lock_irqsave(&mut (*priv_).chip);
    let ipol = gpio_generic_read_reg(&mut (*priv_).chip, (*priv_).regs.add(GRGPIO_IPOL)) & !mask;
    let iedge = gpio_generic_read_reg(&mut (*priv_).chip, (*priv_).regs.add(GRGPIO_IEDGE)) & !mask;
    gpio_generic_write_reg(&mut (*priv_).chip, (*priv_).regs.add(GRGPIO_IPOL), ipol | pol);
    gpio_generic_write_reg(&mut (*priv_).chip, (*priv_).regs.add(GRGPIO_IEDGE), iedge | edge);
    0
}

unsafe fn grgpio_irq_mask(d: *mut irq_data) {
    let priv_ = irq_data_get_irq_chip_data(d) as *mut grgpio_priv;
    let offset = (*d).hwirq as u32;
    let _guard = gpio_generic_lock_irqsave(&mut (*priv_).chip);
    grgpio_set_imask(priv_, offset, 0);
    gpiochip_disable_irq(&mut (*priv_).chip.gc, (*d).hwirq);
}

unsafe fn grgpio_irq_unmask(d: *mut irq_data) {
    let priv_ = irq_data_get_irq_chip_data(d) as *mut grgpio_priv;
    gpiochip_enable_irq(&mut (*priv_).chip.gc, (*d).hwirq);
    let _guard = gpio_generic_lock_irqsave(&mut (*priv_).chip);
    grgpio_set_imask(priv_, (*d).hwirq as u32, 1);
}

static grgpio_irq_chip: irq_chip = irq_chip {
    name: "grgpio\0".as_ptr() as *const i8,
    irq_mask: Some(grgpio_irq_mask),
    irq_unmask: Some(grgpio_irq_unmask),
    irq_set_type: Some(grgpio_irq_set_type),
    flags: IRQCHIP_IMMUTABLE,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn grgpio_irq_handler(irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let priv_ = dev as *mut grgpio_priv;
    let ngpio = (*priv_).chip.gc.ngpio as usize;
    let _guard = gpio_generic_lock_irqsave(&mut (*priv_).chip);
    let mut matched = false;
    for i in 0..ngpio {
        let lirq = &mut (*priv_).lirqs[i];
        if (*priv_).imask & (1u32 << i) != 0 && lirq.index >= 0
            && (*priv_).uirqs[lirq.index as usize].uirq as i32 == irq
        {
            generic_handle_irq(lirq.irq as u32);
            matched = true;
        }
    }
    if !matched {
        dev_warn((*priv_).dev, "No gpio line matched irq %d\n\0".as_ptr() as *const i8, irq);
    }
    IRQ_HANDLED
}

// The remaining irq-domain and probe operations retain the C driver's external
// kernel interfaces; declarations below are intentionally low-level mappings.
unsafe fn grgpio_irq_map(d: *mut irq_domain, irq: u32, hwirq: irq_hw_number_t) -> i32 {
    let priv_ = (*d).host_data as *mut grgpio_priv;
    if priv_.is_null() { return -EINVAL; }
    let lirq = &mut (*priv_).lirqs[hwirq as usize];
    if lirq.index < 0 { return -EINVAL; }
    lirq.irq = irq as u8;
    let uirq = &mut (*priv_).uirqs[lirq.index as usize];
    if atomic_fetch_add(1, &mut uirq.refcnt) == 0 {
        let ret = request_irq(uirq.uirq as i32, Some(grgpio_irq_handler), 0, dev_name((*priv_).dev), priv_ as *mut _);
        if ret != 0 { atomic_dec(&mut uirq.refcnt); return ret; }
    }
    irq_set_chip_data(irq, priv_ as *mut _);
    irq_set_chip_and_handler(irq, &grgpio_irq_chip, Some(handle_simple_irq));
    irq_set_noprobe(irq);
    0
}

unsafe fn grgpio_irq_unmap(d: *mut irq_domain, irq: u32) {
    let priv_ = (*d).host_data as *mut grgpio_priv;
    irq_set_chip_and_handler(irq, core::ptr::null(), None);
    irq_set_chip_data(irq, core::ptr::null_mut());
    let _guard = gpio_generic_lock_irqsave(&mut (*priv_).chip);
    for i in 0..(*priv_).chip.gc.ngpio as usize {
        let lirq = &mut (*priv_).lirqs[i];
        if lirq.irq as u32 == irq {
            grgpio_set_imask(priv_, i as u32, 0);
            lirq.irq = 0;
            let uirq = &mut (*priv_).uirqs[lirq.index as usize];
            if atomic_dec_and_test(&mut uirq.refcnt) {
                gpio_generic_chip_unlock_irqrestore(&mut (*priv_).chip);
                free_irq(uirq.uirq as i32, priv_ as *mut _);
                return;
            }
            break;
        }
    }
    gpio_generic_chip_unlock_irqrestore(&mut (*priv_).chip);
}

unsafe fn grgpio_irq_domain_remove(data: *mut core::ffi::c_void) { irq_domain_remove(data as *mut irq_domain); }

static grgpio_irq_domain_ops: irq_domain_ops = irq_domain_ops {
    map: Some(grgpio_irq_map),
    unmap: Some(grgpio_irq_unmap),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn grgpio_probe(ofdev: *mut platform_device) -> i32 {
    let dev = &mut (*ofdev).dev;
    let np = (*dev).of_node;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<grgpio_priv>(), GFP_KERNEL) as *mut grgpio_priv;
    if priv_.is_null() { return -ENOMEM; }
    let regs = devm_platform_ioremap_resource(ofdev, 0);
    if IS_ERR(regs) { return PTR_ERR(regs); }
    let config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: regs.add(GRGPIO_DATA),
        set: regs.add(GRGPIO_OUTPUT),
        dirout: regs.add(GRGPIO_DIR),
        flags: GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
        ..core::mem::zeroed()
    };
    let err = gpio_generic_chip_init(&mut (*priv_).chip, &config);
    if err != 0 { return err; }
    (*priv_).regs = regs;
    (*priv_).imask = gpio_generic_read_reg(&mut (*priv_).chip, regs.add(GRGPIO_IMASK));
    (*priv_).dev = dev;
    (*priv_).chip.gc.owner = THIS_MODULE;
    (*priv_).chip.gc.to_irq = Some(grgpio_to_irq);
    (*priv_).chip.gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF\0".as_ptr() as *const i8, np);
    if (*priv_).chip.gc.label.is_null() { return -ENOMEM; }
    (*priv_).chip.gc.base = -1;
    let mut prop = 0u32;
    if of_property_read_u32(np, "nbits\0".as_ptr() as *const i8, &mut prop) != 0 || prop == 0 || prop as usize > GRGPIO_MAX_NGPIO {
        (*priv_).chip.gc.ngpio = GRGPIO_MAX_NGPIO as u32;
    } else { (*priv_).chip.gc.ngpio = prop; }
    let mut size = 0i32;
    let irqmap = of_get_property(np, "irqmap\0".as_ptr() as *const i8, &mut size) as *const i32;
    if !irqmap.is_null() {
        if size < (*priv_).chip.gc.ngpio as i32 { return -EINVAL; }
        (*priv_).domain = irq_domain_create_linear(dev_fwnode(dev), (*priv_).chip.gc.ngpio, &grgpio_irq_domain_ops, priv_ as *mut _);
        if (*priv_).domain.is_null() { return -EINVAL; }
        let err = devm_add_action_or_reset(dev, Some(grgpio_irq_domain_remove), (*priv_).domain as *mut _);
        if err != 0 { return err; }
        for i in 0..(*priv_).chip.gc.ngpio as usize {
            let lirq = &mut (*priv_).lirqs[i];
            lirq.index = *irqmap.add(i) as i8;
            if lirq.index < 0 { continue; }
            let ret = platform_get_irq(ofdev, lirq.index as u32);
            if ret <= 0 { continue; }
            (*priv_).uirqs[lirq.index as usize].uirq = ret as u8;
            atomic_set(&mut (*priv_).uirqs[lirq.index as usize].refcnt, 0);
        }
    }
    let err = devm_gpiochip_add_data(dev, &mut (*priv_).chip.gc, priv_ as *mut _);
    if err != 0 { return err; }
    0
}

static grgpio_match: [of_device_id; 3] = [
    of_device_id { name: "GAISLER_GPIO\0".as_ptr() as *const i8, ..unsafe { core::mem::zeroed() } },
    of_device_id { name: "01_01a\0".as_ptr() as *const i8, ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

static mut grgpio_driver: platform_driver = platform_driver {
    driver: device_driver { name: "grgpio\0".as_ptr() as *const i8, of_match_table: grgpio_match.as_ptr(), ..unsafe { core::mem::zeroed() } },
    probe: Some(grgpio_probe),
    ..unsafe { core::mem::zeroed() }
};

// MODULE_DEVICE_TABLE(of, grgpio_match);
// module_platform_driver(grgpio_driver);
// MODULE_AUTHOR("Aeroflex Gaisler AB.");
// MODULE_DESCRIPTION("Driver for Aeroflex Gaisler GRGPIO");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
