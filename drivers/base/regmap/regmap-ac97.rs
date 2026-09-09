// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - AC'97 support
//
// Copyright 2013 Linaro Ltd.  All rights reserved.

// C dependencies supplied by the surrounding kernel translation.

pub unsafe fn regmap_ac97_default_volatile(
    _dev: *mut device,
    reg: libc::c_uint,
) -> bool {
    match reg {
        AC97_RESET
        | AC97_POWERDOWN
        | AC97_INT_PAGING
        | AC97_EXTENDED_ID
        | AC97_EXTENDED_STATUS
        | AC97_EXTENDED_MID
        | AC97_EXTENDED_MSTATUS
        | AC97_GPIO_STATUS
        | AC97_MISC_AFE
        | AC97_VENDOR_ID1
        | AC97_VENDOR_ID2
        | AC97_CODEC_CLASS_REV
        | AC97_PCI_SVID
        | AC97_PCI_SID
        | AC97_FUNC_SELECT
        | AC97_FUNC_INFO
        | AC97_SENSE_INFO => true,
        _ => false,
    }
}

unsafe fn regmap_ac97_reg_read(
    context: *mut libc::c_void,
    reg: libc::c_uint,
    val: *mut libc::c_uint,
) -> libc::c_int {
    let ac97 = context as *mut snd_ac97;

    *val = ((*(*(*ac97).bus).ops).read)(ac97, reg);

    0
}

unsafe fn regmap_ac97_reg_write(
    context: *mut libc::c_void,
    reg: libc::c_uint,
    val: libc::c_uint,
) -> libc::c_int {
    let ac97 = context as *mut snd_ac97;

    ((*(*(*ac97).bus).ops).write)(ac97, reg, val);

    0
}

static ac97_regmap_bus: regmap_bus = regmap_bus {
    reg_write: Some(regmap_ac97_reg_write),
    reg_read: Some(regmap_ac97_reg_read),
};

pub unsafe fn __regmap_init_ac97(
    ac97: *mut snd_ac97,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const libc::c_char,
) -> *mut regmap {
    __regmap_init(
        &mut (*ac97).dev,
        &ac97_regmap_bus,
        ac97 as *mut libc::c_void,
        config,
        lock_key,
        lock_name,
    )
}

pub unsafe fn __devm_regmap_init_ac97(
    ac97: *mut snd_ac97,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const libc::c_char,
) -> *mut regmap {
    __devm_regmap_init(
        &mut (*ac97).dev,
        &ac97_regmap_bus,
        ac97 as *mut libc::c_void,
        config,
        lock_key,
        lock_name,
    )
}

// EXPORT_SYMBOL_GPL(regmap_ac97_default_volatile);
// EXPORT_SYMBOL_GPL(__regmap_init_ac97);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_ac97);
// MODULE_DESCRIPTION("Register map access API - AC'97 support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
