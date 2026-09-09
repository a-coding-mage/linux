// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 TangoTec Ltd.
 * Author: Baruch Siach <baruch@tkos.co.il>
 *
 * Driver for the Xtensa LX4 GPIO32 Option
 *
 * Documentation: Xtensa LX4 Microprocessor Data Book, Section 2.22
 *
 * GPIO32 is a standard optional extension to the Xtensa architecture core that
 * provides preconfigured output and input ports for intra SoC signaling. The
 * GPIO32 option is implemented as 32bit Tensilica Instruction Extension (TIE)
 * output state called EXPSTATE, and 32bit input wire called IMPWIRE. This
 * driver treats input and output states as two distinct devices.
 *
 * Access to GPIO32 specific instructions is controlled by the CPENABLE
 * (Coprocessor Enable Bits) register. By default Xtensa Linux startup code
 * disables access to all coprocessors. This driver sets the CPENABLE bit
 * corresponding to GPIO32 before any GPIO32 specific instruction, and restores
 * CPENABLE state after that.
 *
 * This driver is currently incompatible with SMP. The GPIO32 extension is not
 * guaranteed to be available in all cores. Moreover, each core controls a
 * different set of IO wires. A theoretical SMP aware version of this driver
 * would need to have a per core workqueue to do the actual GPIO manipulation.
 */

// Required external kernel symbols and build-time Xtensa configuration are
// supplied by the surrounding kernel translation.

#[cfg(xchal_have_cp)]
#[inline]
unsafe fn enable_cp(cpenable: *mut libc::c_ulong) -> libc::c_ulong {
    let mut flags: libc::c_ulong = 0;
    local_irq_save(&mut flags);
    *cpenable = xtensa_get_sr(cpenable);
    xtensa_set_sr(*cpenable | (1 as libc::c_ulong << XCHAL_CP_ID_XTIOP), cpenable);
    flags
}

#[cfg(xchal_have_cp)]
#[inline]
unsafe fn disable_cp(flags: libc::c_ulong, cpenable: libc::c_ulong) {
    xtensa_set_sr(cpenable, cpenable);
    local_irq_restore(flags);
}

#[cfg(not(xchal_have_cp))]
#[inline]
unsafe fn enable_cp(cpenable: *mut libc::c_ulong) -> libc::c_ulong {
    *cpenable = 0; // avoid uninitialized value warning
    0
}

#[cfg(not(xchal_have_cp))]
#[inline]
unsafe fn disable_cp(_flags: libc::c_ulong, _cpenable: libc::c_ulong) {}

unsafe extern "C" fn xtensa_impwire_get_direction(
    _gc: *mut gpio_chip,
    _offset: libc::c_uint,
) -> libc::c_int {
    GPIO_LINE_DIRECTION_IN // input only
}

unsafe extern "C" fn xtensa_impwire_get_value(
    _gc: *mut gpio_chip,
    offset: libc::c_uint,
) -> libc::c_int {
    let mut saved_cpenable: libc::c_ulong = 0;
    let flags = enable_cp(&mut saved_cpenable);
    let mut impwire: u32;
    core::arch::asm!("read_impwire {0}", out(reg) impwire);
    disable_cp(flags, saved_cpenable);
    ((impwire & (1u32 << offset)) != 0) as libc::c_int
}

unsafe extern "C" fn xtensa_expstate_get_direction(
    _gc: *mut gpio_chip,
    _offset: libc::c_uint,
) -> libc::c_int {
    GPIO_LINE_DIRECTION_OUT // output only
}

unsafe extern "C" fn xtensa_expstate_get_value(
    _gc: *mut gpio_chip,
    offset: libc::c_uint,
) -> libc::c_int {
    let mut saved_cpenable: libc::c_ulong = 0;
    let flags = enable_cp(&mut saved_cpenable);
    let mut expstate: u32;
    core::arch::asm!("rur.expstate {0}", out(reg) expstate);
    disable_cp(flags, saved_cpenable);
    ((expstate & (1u32 << offset)) != 0) as libc::c_int
}

unsafe extern "C" fn xtensa_expstate_set_value(
    _gc: *mut gpio_chip,
    offset: libc::c_uint,
    value: libc::c_int,
) -> libc::c_int {
    let mask = 1u32 << offset;
    let val = if value != 0 { 1u32 << offset } else { 0 };
    let mut saved_cpenable: libc::c_ulong = 0;
    let flags = enable_cp(&mut saved_cpenable);
    core::arch::asm!("wrmsk_expstate {0}, {1}", in(reg) val, in(reg) mask);
    disable_cp(flags, saved_cpenable);
    0
}

static mut impwire_chip: gpio_chip = gpio_chip {
    label: b"impwire\\0".as_ptr() as *const libc::c_char,
    base: -1,
    ngpio: 32,
    get_direction: Some(xtensa_impwire_get_direction),
    get: Some(xtensa_impwire_get_value),
    ..gpio_chip::ZERO
};

static mut expstate_chip: gpio_chip = gpio_chip {
    label: b"expstate\\0".as_ptr() as *const libc::c_char,
    base: -1,
    ngpio: 32,
    get_direction: Some(xtensa_expstate_get_direction),
    get: Some(xtensa_expstate_get_value),
    set: Some(xtensa_expstate_set_value),
    ..gpio_chip::ZERO
};

unsafe extern "C" fn xtensa_gpio_probe(_pdev: *mut platform_device) -> libc::c_int {
    let ret = gpiochip_add_data(&raw mut impwire_chip, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }
    gpiochip_add_data(&raw mut expstate_chip, core::ptr::null_mut())
}

static mut xtensa_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"xtensa-gpio\\0".as_ptr() as *const libc::c_char,
        ..device_driver::ZERO
    },
    probe: Some(xtensa_gpio_probe),
    ..platform_driver::ZERO
};

unsafe extern "C" fn xtensa_gpio_init() -> libc::c_int {
    let pdev = platform_device_register_simple(
        b"xtensa-gpio\\0".as_ptr() as *const libc::c_char,
        0,
        core::ptr::null(),
        0,
    );
    if is_err(pdev) {
        return ptr_err(pdev);
    }
    platform_driver_register(&raw mut xtensa_gpio_driver)
}

// device_initcall(xtensa_gpio_init);
// MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
// MODULE_DESCRIPTION("Xtensa LX4 GPIO32 driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
