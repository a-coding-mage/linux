// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - Memory region
//
// This is intended for testing only
//
// Copyright (c) 2023, Arm Ltd

// C dependencies supplied by the surrounding kernel translation are referenced
// here but are not reimplemented in this file.

unsafe fn regmap_ram_write(
    context: *mut core::ffi::c_void,
    reg: core::ffi::c_uint,
    val: core::ffi::c_uint,
) -> core::ffi::c_int {
    let data = context as *mut regmap_ram_data;

    unsafe {
        (*data).vals[reg as usize] = val;
        (*data).written[reg as usize] = true;
    }

    0
}

unsafe fn regmap_ram_read(
    context: *mut core::ffi::c_void,
    reg: core::ffi::c_uint,
    val: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let data = context as *mut regmap_ram_data;

    unsafe {
        *val = (*data).vals[reg as usize];
        (*data).read[reg as usize] = true;
    }

    0
}

unsafe fn regmap_ram_free_context(context: *mut core::ffi::c_void) {
    let data = context as *mut regmap_ram_data;

    unsafe {
        kfree((*data).vals);
        kfree((*data).read);
        kfree((*data).written);
        kfree(data);
    }
}

static REGMAP_RAM: regmap_bus = regmap_bus {
    fast_io: true,
    reg_write: Some(regmap_ram_write),
    reg_read: Some(regmap_ram_read),
    free_context: Some(regmap_ram_free_context),
};

pub unsafe fn __regmap_init_ram(
    dev: *mut device,
    config: *const regmap_config,
    data: *mut regmap_ram_data,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let mut map: *mut regmap;

    unsafe {
        if (*config).max_register == 0 {
            pr_crit!("No max_register specified for RAM regmap\n");
            return ERR_PTR(-EINVAL);
        }

        (*data).read = kzalloc_objs::<bool>((*config).max_register + 1);
        if (*data).read.is_null() {
            return ERR_PTR(-ENOMEM);
        }

        (*data).written = kzalloc_objs::<bool>((*config).max_register + 1);
        if (*data).written.is_null() {
            kfree((*data).read);
            return ERR_PTR(-ENOMEM);
        }

        map = __regmap_init(dev, &REGMAP_RAM, data, config, lock_key, lock_name);
        if (IS_ERR(map)) {
            kfree((*data).read);
            kfree((*data).written);
        }
    }

    map
}

// EXPORT_SYMBOL_GPL(__regmap_init_ram);

// MODULE_DESCRIPTION("Register map access API - Memory region");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
