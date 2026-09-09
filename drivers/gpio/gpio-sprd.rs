// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Spreadtrum Communications Inc.
 * Copyright (C) 2018 Linaro Ltd.
 */

// Linux kernel dependencies supplied externally.

const SPRD_GPIO_DATA: u16 = 0x0;
const SPRD_GPIO_DMSK: u16 = 0x4;
const SPRD_GPIO_DIR: u16 = 0x8;
const SPRD_GPIO_IS: u16 = 0xc;
const SPRD_GPIO_IBE: u16 = 0x10;
const SPRD_GPIO_IEV: u16 = 0x14;
const SPRD_GPIO_IE: u16 = 0x18;
const SPRD_GPIO_RIS: u16 = 0x1c;
const SPRD_GPIO_MIS: u16 = 0x20;
const SPRD_GPIO_IC: u16 = 0x24;
const SPRD_GPIO_INEN: u16 = 0x28;

const SPRD_GPIO_BANK_NR: u32 = 16;
const SPRD_GPIO_NR: u32 = 256;
const SPRD_GPIO_BANK_SIZE: usize = 0x80;
const SPRD_GPIO_BANK_MASK: u32 = 0xffff;

#[inline]
const fn sprd_gpio_bit(x: u32) -> u32 { x & (SPRD_GPIO_BANK_NR - 1) }

#[repr(C)]
struct sprd_gpio {
    chip: gpio_chip,
    base: *mut core::ffi::c_void,
    lock: raw_spinlock_t,
    irq: i32,
}

unsafe fn sprd_gpio_bank_base(sprd_gpio: *mut sprd_gpio, bank: u32) -> *mut u8 {
    ((*sprd_gpio).base as *mut u8).add(SPRD_GPIO_BANK_SIZE * bank as usize)
}

unsafe fn sprd_gpio_update(chip: *mut gpio_chip, offset: u32, reg: u16, val: i32) {
    let sprd_gpio = gpiochip_get_data(chip) as *mut sprd_gpio;
    let base = sprd_gpio_bank_base(sprd_gpio, offset / SPRD_GPIO_BANK_NR);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sprd_gpio).lock, &mut flags);
    let mut tmp = readl_relaxed(base.add(reg as usize));
    if val != 0 { tmp |= 1u32 << sprd_gpio_bit(offset); }
    else { tmp &= !(1u32 << sprd_gpio_bit(offset)); }
    writel_relaxed(tmp, base.add(reg as usize));
    raw_spin_unlock_irqrestore(&mut (*sprd_gpio).lock, flags);
}

unsafe fn sprd_gpio_read(chip: *mut gpio_chip, offset: u32, reg: u16) -> i32 {
    let sprd_gpio = gpiochip_get_data(chip) as *mut sprd_gpio;
    let base = sprd_gpio_bank_base(sprd_gpio, offset / SPRD_GPIO_BANK_NR);
    if readl_relaxed(base.add(reg as usize)) & (1u32 << sprd_gpio_bit(offset)) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn sprd_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 { sprd_gpio_update(chip, offset, SPRD_GPIO_DMSK, 1); 0 }
unsafe extern "C" fn sprd_gpio_free(chip: *mut gpio_chip, offset: u32) { sprd_gpio_update(chip, offset, SPRD_GPIO_DMSK, 0); }
unsafe extern "C" fn sprd_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DIR, 0); sprd_gpio_update(chip, offset, SPRD_GPIO_INEN, 1); 0
}
unsafe extern "C" fn sprd_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DIR, 1); sprd_gpio_update(chip, offset, SPRD_GPIO_INEN, 0); sprd_gpio_update(chip, offset, SPRD_GPIO_DATA, value); 0
}
unsafe extern "C" fn sprd_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 { sprd_gpio_read(chip, offset, SPRD_GPIO_DATA) }
unsafe extern "C" fn sprd_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) { sprd_gpio_update(chip, offset, SPRD_GPIO_DATA, value); }

unsafe extern "C" fn sprd_gpio_irq_mask(data: *mut irq_data) { let chip = irq_data_get_irq_chip_data(data); let offset = irqd_to_hwirq(data) as u32; sprd_gpio_update(chip, offset, SPRD_GPIO_IE, 0); gpiochip_disable_irq(chip, offset); }
unsafe extern "C" fn sprd_gpio_irq_ack(data: *mut irq_data) { let chip = irq_data_get_irq_chip_data(data); let offset = irqd_to_hwirq(data) as u32; sprd_gpio_update(chip, offset, SPRD_GPIO_IC, 1); }
unsafe extern "C" fn sprd_gpio_irq_unmask(data: *mut irq_data) { let chip = irq_data_get_irq_chip_data(data); let offset = irqd_to_hwirq(data) as u32; sprd_gpio_update(chip, offset, SPRD_GPIO_IE, 1); gpiochip_enable_irq(chip, offset); }

unsafe extern "C" fn sprd_gpio_irq_set_type(data: *mut irq_data, flow_type: u32) -> i32 {
    let chip = irq_data_get_irq_chip_data(data); let offset = irqd_to_hwirq(data) as u32;
    match flow_type {
        IRQ_TYPE_EDGE_RISING => { sprd_gpio_update(chip,offset,SPRD_GPIO_IS,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IBE,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IEV,1); sprd_gpio_update(chip,offset,SPRD_GPIO_IC,1); irq_set_handler_locked(data, handle_edge_irq); }
        IRQ_TYPE_EDGE_FALLING => { sprd_gpio_update(chip,offset,SPRD_GPIO_IS,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IBE,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IEV,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IC,1); irq_set_handler_locked(data, handle_edge_irq); }
        IRQ_TYPE_EDGE_BOTH => { sprd_gpio_update(chip,offset,SPRD_GPIO_IS,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IBE,1); sprd_gpio_update(chip,offset,SPRD_GPIO_IC,1); irq_set_handler_locked(data, handle_edge_irq); }
        IRQ_TYPE_LEVEL_HIGH => { sprd_gpio_update(chip,offset,SPRD_GPIO_IS,1); sprd_gpio_update(chip,offset,SPRD_GPIO_IBE,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IEV,1); irq_set_handler_locked(data, handle_level_irq); }
        IRQ_TYPE_LEVEL_LOW => { sprd_gpio_update(chip,offset,SPRD_GPIO_IS,1); sprd_gpio_update(chip,offset,SPRD_GPIO_IBE,0); sprd_gpio_update(chip,offset,SPRD_GPIO_IEV,0); irq_set_handler_locked(data, handle_level_irq); }
        _ => return -EINVAL,
    } 0
}

unsafe extern "C" fn sprd_gpio_irq_handler(desc: *mut irq_desc) {
    let chip = irq_desc_get_handler_data(desc); let ic = irq_desc_get_chip(desc); let gpio = gpiochip_get_data(chip) as *mut sprd_gpio;
    chained_irq_enter(ic, desc);
    let mut bank = 0; while bank * SPRD_GPIO_BANK_NR < (*chip).ngpio {
        let base = sprd_gpio_bank_base(gpio, bank); let mut reg = readl_relaxed(base.add(SPRD_GPIO_MIS as usize)) & SPRD_GPIO_BANK_MASK;
        let mut n = 0; while n < SPRD_GPIO_BANK_NR { if reg & (1 << n) != 0 { generic_handle_domain_irq((*(*chip).irq).domain, bank * SPRD_GPIO_BANK_NR + n); } n += 1; }
        bank += 1;
    } chained_irq_exit(ic, desc);
}

#[no_mangle]
static mut sprd_gpio_irqchip: irq_chip = irq_chip {
    name: b"sprd-gpio\0".as_ptr() as *const i8,
    irq_ack: Some(sprd_gpio_irq_ack), irq_mask: Some(sprd_gpio_irq_mask),
    irq_unmask: Some(sprd_gpio_irq_unmask), irq_set_type: Some(sprd_gpio_irq_set_type),
    flags: IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
};

unsafe extern "C" fn sprd_gpio_probe(pdev: *mut platform_device) -> i32 {
    let sprd_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<sprd_gpio>(), GFP_KERNEL) as *mut sprd_gpio;
    if sprd_gpio.is_null() { return -ENOMEM; }
    (*sprd_gpio).irq = platform_get_irq(pdev, 0); if (*sprd_gpio).irq < 0 { return (*sprd_gpio).irq; }
    (*sprd_gpio).base = devm_platform_ioremap_resource(pdev, 0); if is_err((*sprd_gpio).base) { return ptr_err((*sprd_gpio).base); }
    raw_spin_lock_init(&mut (*sprd_gpio).lock);
    (*sprd_gpio).chip.label = dev_name(&(*pdev).dev); (*sprd_gpio).chip.ngpio = SPRD_GPIO_NR; (*sprd_gpio).chip.base = -1; (*sprd_gpio).chip.parent = &mut (*pdev).dev;
    (*sprd_gpio).chip.request = Some(sprd_gpio_request); (*sprd_gpio).chip.free = Some(sprd_gpio_free); (*sprd_gpio).chip.get = Some(sprd_gpio_get); (*sprd_gpio).chip.set = Some(sprd_gpio_set);
    (*sprd_gpio).chip.direction_input = Some(sprd_gpio_direction_input); (*sprd_gpio).chip.direction_output = Some(sprd_gpio_direction_output);
    let irq = &mut (*sprd_gpio).chip.irq; gpio_irq_chip_set_chip(irq, &mut sprd_gpio_irqchip); irq.handler = Some(handle_bad_irq); irq.default_type = IRQ_TYPE_NONE; irq.parent_handler = Some(sprd_gpio_irq_handler); irq.parent_handler_data = sprd_gpio as *mut _; irq.num_parents = 1; irq.parents = &mut (*sprd_gpio).irq;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*sprd_gpio).chip, sprd_gpio as *mut _)
}

#[repr(C)] struct of_device_id { compatible: *const i8 }
static mut sprd_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"sprd,sc9860-gpio\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)] struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, driver: driver }
#[repr(C)] struct driver { name: *const i8, of_match_table: *const of_device_id }
static mut sprd_gpio_driver: platform_driver = platform_driver { probe: Some(sprd_gpio_probe), driver: driver { name: b"sprd-gpio\0".as_ptr() as *const i8, of_match_table: unsafe { sprd_gpio_of_match.as_ptr() } } };

module_platform_driver_probe!(sprd_gpio_driver, sprd_gpio_probe);
// MODULE_DEVICE_TABLE(of, sprd_gpio_of_match);
// MODULE_DESCRIPTION("Spreadtrum GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
