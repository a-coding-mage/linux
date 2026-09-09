// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/mach-x3proto/gpio.c
 *
 * Renesas SH-X3 Prototype Baseboard GPIO Support.
 *
 * Copyright (C) 2010 - 2012  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation.

const KEYCTLR: usize = 0xb81c0000;
const KEYOUTR: usize = 0xb81c0002;
const KEYDETR: usize = 0xb81c0004;

static mut X3PROTO_GPIO_LOCK: spinlock_t = spinlock_t::new();
static mut x3proto_irq_domain: *mut irq_domain = core::ptr::null_mut();

unsafe fn x3proto_gpio_direction_input(
    _chip: *mut gpio_chip,
    gpio: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut flags: core::ffi::c_ulong = 0;
    let mut data: core::ffi::c_uint;

    spin_lock_irqsave(&mut X3PROTO_GPIO_LOCK, &mut flags);
    data = __raw_readw(KEYCTLR) as core::ffi::c_uint;
    data |= 1u32.wrapping_shl(gpio);
    __raw_writew(data as u16, KEYCTLR);
    spin_unlock_irqrestore(&mut X3PROTO_GPIO_LOCK, flags);

    0
}

unsafe fn x3proto_gpio_get(
    _chip: *mut gpio_chip,
    gpio: core::ffi::c_uint,
) -> core::ffi::c_int {
    ((__raw_readw(KEYDETR) as core::ffi::c_uint
        & 1u32.wrapping_shl(gpio)) != 0) as core::ffi::c_int
}

unsafe fn x3proto_gpio_to_irq(
    chip: *mut gpio_chip,
    gpio: core::ffi::c_uint,
) -> core::ffi::c_int {
    if gpio < (*chip).ngpio {
        irq_create_mapping(x3proto_irq_domain, gpio as irq_hw_number_t)
    } else {
        -ENXIO
    }
}

unsafe extern "C" fn x3proto_gpio_irq_handler(desc: *mut irq_desc) {
    let data = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(data);
    let mask = __raw_readw(KEYDETR) as core::ffi::c_ulong;

    ((*chip).irq_mask_ack)(data);
    for pin in 0..NR_BASEBOARD_GPIOS {
        if (mask & (1usize << pin)) != 0 {
            generic_handle_domain_irq(x3proto_irq_domain, pin as irq_hw_number_t);
        }
    }
    ((*chip).irq_unmask)(data);
}

#[no_mangle]
pub static mut x3proto_gpio_chip: gpio_chip = gpio_chip {
    label: b"x3proto-gpio\0".as_ptr() as *const core::ffi::c_char,
    direction_input: Some(x3proto_gpio_direction_input),
    get: Some(x3proto_gpio_get),
    to_irq: Some(x3proto_gpio_to_irq),
    base: -1,
    ngpio: NR_BASEBOARD_GPIOS,
};

unsafe extern "C" fn x3proto_gpio_irq_map(
    _domain: *mut irq_domain,
    virq: core::ffi::c_uint,
    _hwirq: irq_hw_number_t,
) -> core::ffi::c_int {
    irq_set_chip_and_handler_name(
        virq,
        &mut dummy_irq_chip,
        handle_simple_irq,
        b"gpio\0".as_ptr() as *const core::ffi::c_char,
    );

    0
}

static mut x3proto_gpio_irq_ops: irq_domain_ops = irq_domain_ops {
    map: Some(x3proto_gpio_irq_map),
    xlate: Some(irq_domain_xlate_twocell),
};

pub unsafe extern "C" fn x3proto_gpio_setup() -> core::ffi::c_int {
    let ilsel: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    ilsel = ilsel_enable(ILSEL_KEY);
    if ilsel < 0 {
        return ilsel;
    }

    ret = gpiochip_add_data(&mut x3proto_gpio_chip, core::ptr::null_mut());
    if ret != 0 {
        goto_err_gpio(ilsel, ret);
        return ret;
    }

    x3proto_irq_domain = irq_domain_create_linear(
        core::ptr::null_mut(),
        NR_BASEBOARD_GPIOS,
        &mut x3proto_gpio_irq_ops,
        core::ptr::null_mut(),
    );
    if x3proto_irq_domain.is_null() {
        gpiochip_remove(&mut x3proto_gpio_chip);
        ret = 0;
        synchronize_irq(ilsel);
        ilsel_disable(ILSEL_KEY);
        return ret;
    }

    pr_info(
        b"registering '%s' support, handling GPIOs %u -> %u, bound to IRQ %u\n\0".as_ptr()
            as *const core::ffi::c_char,
        x3proto_gpio_chip.label,
        x3proto_gpio_chip.base,
        x3proto_gpio_chip.base + x3proto_gpio_chip.ngpio as core::ffi::c_int,
        ilsel,
    );

    irq_set_chained_handler(ilsel, Some(x3proto_gpio_irq_handler));
    irq_set_irq_wake(ilsel, 1);

    0
}

unsafe fn goto_err_gpio(ilsel: core::ffi::c_int, _ret: core::ffi::c_int) {
    synchronize_irq(ilsel);
    ilsel_disable(ILSEL_KEY);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
