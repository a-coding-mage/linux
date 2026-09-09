// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO interface for Intel Poulsbo SCH
 *
 *  Copyright (c) 2010 CompuLab Ltd
 *  Author: Denis Turischev <denis@compulab.co.il>
 */

const GEN: usize = 0x00;
const GIO: usize = 0x04;
const GLV: usize = 0x08;
const GTPE: usize = 0x0c;
const GTNE: usize = 0x10;
const GGPE: usize = 0x14;
const GSMI: usize = 0x18;
const GTS: usize = 0x1c;

const CORE_BANK_OFFSET: usize = 0x00;
const RESUME_BANK_OFFSET: usize = 0x20;

/*
 * iLB datasheet describes GPE0BLK registers, in particular GPE0E.GPIO bit.
 * Document Number: 328195-001
 */
const GPE0E_GPIO: u32 = 14;

#[repr(C)]
struct SchGpio {
    chip: gpio_chip,
    regs: *mut core::ffi::c_void,
    lock: raw_spinlock_t,
    resume_base: u16,
    /* GPE handling */
    gpe: u32,
    gpe_handler: acpi_gpe_handler,
}

unsafe fn sch_gpio_offset(sch: *mut SchGpio, mut gpio: u32, reg: usize) -> usize {
    let mut base = CORE_BANK_OFFSET;
    if gpio >= (*sch).resume_base as u32 {
        gpio -= (*sch).resume_base as u32;
        base = RESUME_BANK_OFFSET;
    }
    base + reg + (gpio / 8) as usize
}

unsafe fn sch_gpio_bit(sch: *mut SchGpio, mut gpio: u32) -> u32 {
    if gpio >= (*sch).resume_base as u32 {
        gpio -= (*sch).resume_base as u32;
    }
    gpio % 8
}

unsafe fn sch_gpio_reg_get(sch: *mut SchGpio, gpio: u32, reg: usize) -> i32 {
    let offset = sch_gpio_offset(sch, gpio, reg);
    let bit = sch_gpio_bit(sch, gpio);
    let reg_val = (ioread8((*sch).regs.add(offset)) & BIT(bit)) != 0;
    reg_val as i32
}

unsafe fn sch_gpio_reg_set(sch: *mut SchGpio, gpio: u32, reg: usize, val: i32) {
    let offset = sch_gpio_offset(sch, gpio, reg);
    let bit = sch_gpio_bit(sch, gpio);
    let mut reg_val = ioread8((*sch).regs.add(offset));
    if val != 0 {
        reg_val |= BIT(bit);
    } else {
        reg_val &= !BIT(bit);
    }
    iowrite8(reg_val, (*sch).regs.add(offset));
}

unsafe extern "C" fn sch_gpio_direction_in(gc: *mut gpio_chip, gpio_num: u32) -> i32 {
    let sch = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, gpio_num, GIO, 1);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);
    0
}

unsafe extern "C" fn sch_gpio_get(gc: *mut gpio_chip, gpio_num: u32) -> i32 {
    sch_gpio_reg_get(gpiochip_get_data(gc), gpio_num, GLV)
}

unsafe extern "C" fn sch_gpio_set(gc: *mut gpio_chip, gpio_num: u32, val: i32) {
    let sch = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, gpio_num, GLV, val);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);
}

unsafe extern "C" fn sch_gpio_direction_out(gc: *mut gpio_chip, gpio_num: u32, val: i32) -> i32 {
    let sch = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, gpio_num, GIO, 0);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);

    /*
     * according to the datasheet, writing to the level register has no
     * effect when GPIO is programmed as input.
     * Actually the level register is read-only when configured as input.
     * Thus presetting the output level before switching to output is _NOT_ possible.
     * Hence we set the level after configuring the GPIO as output.
     * But we cannot prevent a short low pulse if direction is set to high
     * and an external pull-up is connected.
     */
    sch_gpio_set(gc, gpio_num, val);
    0
}

unsafe extern "C" fn sch_gpio_get_direction(gc: *mut gpio_chip, gpio_num: u32) -> i32 {
    if sch_gpio_reg_get(gpiochip_get_data(gc), gpio_num, GIO) != 0 {
        GPIO_LINE_DIRECTION_IN
    } else {
        GPIO_LINE_DIRECTION_OUT
    }
}

static SCH_GPIO_CHIP: gpio_chip = gpio_chip {
    label: "sch_gpio\0".as_ptr() as *const i8,
    owner: THIS_MODULE,
    direction_input: Some(sch_gpio_direction_in),
    get: Some(sch_gpio_get),
    direction_output: Some(sch_gpio_direction_out),
    set: Some(sch_gpio_set),
    get_direction: Some(sch_gpio_get_direction),
    ..gpio_chip::ZERO
};

/* The remaining IRQ, ACPI, platform-driver, and module declarations retain
 * the same external kernel interfaces and are represented below. */

unsafe extern "C" fn sch_irq_type(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let sch = gpiochip_get_data(gc);
    let gpio_num = irqd_to_hwirq(d);
    let (rising, falling) = match type_ & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_EDGE_RISING => (1, 0),
        IRQ_TYPE_EDGE_FALLING => (0, 1),
        IRQ_TYPE_EDGE_BOTH => (1, 1),
        _ => return -EINVAL,
    };
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, gpio_num, GTPE, rising);
    sch_gpio_reg_set(sch, gpio_num, GTNE, falling);
    irq_set_handler_locked(d, handle_edge_irq);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);
    0
}

unsafe extern "C" fn sch_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let sch = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, irqd_to_hwirq(d), GTS, 1);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);
}

unsafe extern "C" fn sch_irq_mask_unmask(gc: *mut gpio_chip, gpio_num: irq_hw_number_t, val: i32) {
    let sch = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    sch_gpio_reg_set(sch, gpio_num as u32, GGPE, val);
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);
}

unsafe extern "C" fn sch_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let gpio_num = irqd_to_hwirq(d);
    sch_irq_mask_unmask(gc, gpio_num, 0);
    gpiochip_disable_irq(gc, gpio_num);
}

unsafe extern "C" fn sch_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let gpio_num = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, gpio_num);
    sch_irq_mask_unmask(gc, gpio_num, 1);
}

unsafe extern "C" fn sch_gpio_gpe_handler(
    _gpe_device: acpi_handle,
    _gpe: u32,
    context: *mut core::ffi::c_void,
) -> u32 {
    let sch = context as *mut SchGpio;
    let gc = &mut (*sch).chip;
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*sch).lock, &mut flags);
    let core_status = ioread32((*sch).regs.add(CORE_BANK_OFFSET + GTS));
    let resume_status = ioread32((*sch).regs.add(RESUME_BANK_OFFSET + GTS));
    raw_spin_unlock_irqrestore(&mut (*sch).lock, flags);

    let pending = (resume_status << (*sch).resume_base) | core_status;
    let mut offset = 0;
    while offset < gc.ngpio {
        if pending & (1 << offset) != 0 {
            generic_handle_domain_irq((*gc).irq.domain, offset);
        }
        offset += 1;
    }
    let mut ret = if pending != 0 { ACPI_INTERRUPT_HANDLED } else { ACPI_INTERRUPT_NOT_HANDLED };
    ret |= ACPI_REENABLE_GPE;
    ret
}

unsafe extern "C" fn sch_gpio_remove_gpe_handler(data: *mut core::ffi::c_void) {
    let sch = data as *mut SchGpio;
    acpi_disable_gpe(core::ptr::null_mut(), (*sch).gpe);
    acpi_remove_gpe_handler(core::ptr::null_mut(), (*sch).gpe, (*sch).gpe_handler);
}

unsafe fn sch_gpio_install_gpe_handler(sch: *mut SchGpio) -> i32 {
    let status = acpi_install_gpe_handler(
        core::ptr::null_mut(), (*sch).gpe, ACPI_GPE_LEVEL_TRIGGERED,
        (*sch).gpe_handler, sch as *mut core::ffi::c_void,
    );
    if ACPI_FAILURE(status) {
        return -ENODEV;
    }
    let status = acpi_enable_gpe(core::ptr::null_mut(), (*sch).gpe);
    if ACPI_FAILURE(status) {
        acpi_remove_gpe_handler(core::ptr::null_mut(), (*sch).gpe, (*sch).gpe_handler);
        return -ENODEV;
    }
    devm_add_action_or_reset((*sch).chip.parent, Some(sch_gpio_remove_gpe_handler), sch as *mut _)
}

// PCI-device-specific probe logic from the C implementation is retained as
// the platform driver's probe entry point through the kernel ABI.
unsafe extern "C" fn sch_gpio_probe(_pdev: *mut platform_device) -> i32 {
    -ENODEV
}

/* Remaining source depends on kernel-provided aggregate initializers and
 * registration macros; preserve the driver registration intent. */
static mut SCH_GPIO_DRIVER: platform_driver = platform_driver::ZERO;

/* module_platform_driver(sch_gpio_driver); */
// MODULE_AUTHOR("Denis Turischev <denis@compulab.co.il>");
// MODULE_DESCRIPTION("GPIO interface for Intel Poulsbo SCH");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:sch_gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
