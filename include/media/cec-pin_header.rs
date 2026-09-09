/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cec-pin.h - low-level CEC pin control
 *
 * Copyright 2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

// Translated from the C header. Definitions supplied by the included headers
// are expected to be available to the surrounding translation unit.

/**
 * struct cec_pin_ops - low-level CEC pin operations
 * @read:\tread the CEC pin. Returns > 0 if high, 0 if low, or an error
 *\t\tif negative.
 * @low:\tdrive the CEC pin low.
 * @high:\tstop driving the CEC pin. The pull-up will drive the pin
 *\t\thigh, unless someone else is driving the pin low.
 * @enable_irq:\toptional, enable the interrupt to detect pin voltage changes.
 * @disable_irq: optional, disable the interrupt.
 * @free:\toptional. Free any allocated resources. Called when the
 *\t\tadapter is deleted.
 * @status:\toptional, log status information.
 * @read_hpd:\toptional. Read the HPD pin. Returns > 0 if high, 0 if low or
 *\t\tan error if negative.
 * @read_5v:\toptional. Read the 5V pin. Returns > 0 if high, 0 if low or
 *\t\tan error if negative.
 * @received:\toptional. High-level CEC message callback. Allows the driver
 *\t\tto process CEC messages.
 *
 * These operations (except for the @received op) are used by the
 * cec pin framework to manipulate the CEC pin.
 */
#[repr(C)]
pub struct cec_pin_ops {
    pub read: Option<unsafe extern "C" fn(adap: *mut cec_adapter) -> ::core::ffi::c_int>,
    pub low: Option<unsafe extern "C" fn(adap: *mut cec_adapter)>,
    pub high: Option<unsafe extern "C" fn(adap: *mut cec_adapter)>,
    pub enable_irq: Option<unsafe extern "C" fn(adap: *mut cec_adapter) -> bool>,
    pub disable_irq: Option<unsafe extern "C" fn(adap: *mut cec_adapter)>,
    pub free: Option<unsafe extern "C" fn(adap: *mut cec_adapter)>,
    pub status: Option<unsafe extern "C" fn(
        adap: *mut cec_adapter,
        file: *mut seq_file,
    )>,
    pub read_hpd: Option<unsafe extern "C" fn(adap: *mut cec_adapter) -> ::core::ffi::c_int>,
    pub read_5v: Option<unsafe extern "C" fn(adap: *mut cec_adapter) -> ::core::ffi::c_int>,

    /* High-level CEC message callback */
    pub received: Option<unsafe extern "C" fn(
        adap: *mut cec_adapter,
        msg: *mut cec_msg,
    ) -> ::core::ffi::c_int>,
}

/**
 * cec_pin_changed() - update pin state from interrupt
 *
 * @adap:\tpointer to the cec adapter
 * @value:\twhen true the pin is high, otherwise it is low
 *
 * If changes of the CEC voltage are detected via an interrupt, then
 * cec_pin_changed is called from the interrupt with the new value.
 */
extern "C" {
    pub fn cec_pin_changed(adap: *mut cec_adapter, value: bool);
}

/**
 * cec_pin_allocate_adapter() - allocate a pin-based cec adapter
 *
 * @pin_ops:\tlow-level pin operations
 * @priv:\twill be stored in adap->priv and can be used by the adapter ops.
 *\t\tUse cec_get_drvdata(adap) to get the priv pointer.
 * @name:\tthe name of the CEC adapter. Note: this name will be copied.
 * @caps:\tcapabilities of the CEC adapter. This will be ORed with
 *\t\tCEC_CAP_MONITOR_ALL and CEC_CAP_MONITOR_PIN.
 *
 * Allocate a cec adapter using the cec pin framework.
 *
 * Return: a pointer to the cec adapter or an error pointer
 */
extern "C" {
    pub fn cec_pin_allocate_adapter(
        pin_ops: *const cec_pin_ops,
        priv_: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        caps: u32,
    ) -> *mut cec_adapter;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
