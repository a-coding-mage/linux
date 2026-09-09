// SPDX-License-Identifier: GPL-2.0+
/*
 * COMEDI driver for the watchdog subdevice found on some addi-data boards
 * Copyright (c) 2013 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Based on implementations in various addi-data COMEDI drivers.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the surrounding Linux/COMEDI translation unit:
// linux module symbols, COMEDI types/constants/helpers, ADDI_TCW_* constants,
// outl/inl, and module metadata/export facilities.

#[repr(C)]
struct addi_watchdog_private {
    iobase: ::core::ffi::c_ulong,
    wdog_ctrl: ::core::ffi::c_uint,
}

/*
 * The watchdog subdevice is configured with two INSN_CONFIG instructions:
 *
 * Enable the watchdog and set the reload timeout:
 *     data[0] = INSN_CONFIG_ARM
 *     data[1] = timeout reload value
 *
 * Disable the watchdog:
 *     data[0] = INSN_CONFIG_DISARM
 */
unsafe fn addi_watchdog_insn_config(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let spriv = (*s).private as *mut addi_watchdog_private;
    let mut reload: ::core::ffi::c_uint;

    match *data.add(0) {
        INSN_CONFIG_ARM => {
            (*spriv).wdog_ctrl = ADDI_TCW_CTRL_ENA;
            reload = *data.add(1) & (*s).maxdata;
            outl(reload, (*spriv).iobase + ADDI_TCW_RELOAD_REG as ::core::ffi::c_ulong);

            /* Time base is 20ms, let the user know the timeout */
            dev_info(
                (*dev).class_dev,
                "watchdog enabled, timeout:%dms\n",
                20 * reload + 20,
            );
        }
        INSN_CONFIG_DISARM => {
            (*spriv).wdog_ctrl = 0;
        }
        _ => return -EINVAL,
    }

    outl(
        (*spriv).wdog_ctrl,
        (*spriv).iobase + ADDI_TCW_CTRL_REG as ::core::ffi::c_ulong,
    );

    (*insn).n as ::core::ffi::c_int
}

unsafe fn addi_watchdog_insn_read(
    _dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let spriv = (*s).private as *mut addi_watchdog_private;
    let mut i: ::core::ffi::c_int = 0;

    while i < (*insn).n as ::core::ffi::c_int {
        *data.add(i as usize) = inl(
            (*spriv).iobase + ADDI_TCW_STATUS_REG as ::core::ffi::c_ulong,
        );
        i += 1;
    }

    (*insn).n as ::core::ffi::c_int
}

unsafe fn addi_watchdog_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    _data: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let spriv = (*s).private as *mut addi_watchdog_private;
    let mut i: ::core::ffi::c_int;

    if (*spriv).wdog_ctrl == 0 {
        dev_warn((*dev).class_dev, "watchdog is disabled\n");
        return -EINVAL;
    }

    /* "ping" the watchdog */
    i = 0;
    while i < (*insn).n as ::core::ffi::c_int {
        outl(
            (*spriv).wdog_ctrl | ADDI_TCW_CTRL_TRIG,
            (*spriv).iobase + ADDI_TCW_CTRL_REG as ::core::ffi::c_ulong,
        );
        i += 1;
    }

    (*insn).n as ::core::ffi::c_int
}

pub unsafe fn addi_watchdog_reset(iobase: ::core::ffi::c_ulong) {
    outl(0x0, iobase + ADDI_TCW_CTRL_REG as ::core::ffi::c_ulong);
    outl(0x0, iobase + ADDI_TCW_RELOAD_REG as ::core::ffi::c_ulong);
}

pub unsafe fn addi_watchdog_init(
    s: *mut comedi_subdevice,
    iobase: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let spriv: *mut addi_watchdog_private;

    spriv = comedi_alloc_spriv(s, ::core::mem::size_of::<addi_watchdog_private>())
        as *mut addi_watchdog_private;
    if spriv.is_null() {
        return -ENOMEM;
    }

    (*spriv).iobase = iobase;

    (*s).type_ = COMEDI_SUBD_TIMER;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 1;
    (*s).maxdata = 0xff;
    (*s).insn_config = Some(addi_watchdog_insn_config);
    (*s).insn_read = Some(addi_watchdog_insn_read);
    (*s).insn_write = Some(addi_watchdog_insn_write);

    0
}

// EXPORT_SYMBOL_GPL(addi_watchdog_reset);
// EXPORT_SYMBOL_GPL(addi_watchdog_init);
// MODULE_DESCRIPTION("ADDI-DATA Watchdog subdevice");
// MODULE_AUTHOR("H Hartley Sweeten <hsweeten@visionengravers.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
