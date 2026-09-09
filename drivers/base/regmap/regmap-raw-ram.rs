// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - Memory region with raw access
//
// This is intended for testing only
//
// Copyright (c) 2023, Arm Ltd

// C dependencies: linux/clk.h, linux/err.h, linux/io.h, linux/module.h,
// linux/regmap.h, linux/slab.h, linux/swab.h, and internal.h.

unsafe fn decode_reg(endian: regmap_endian, reg: *const core::ffi::c_void) -> u32 {
    let r = reg as *const u16;

    if endian == REGMAP_ENDIAN_BIG {
        be16_to_cpu(r.read()) as u32
    } else {
        le16_to_cpu(r.read()) as u32
    }
}

unsafe extern "C" fn regmap_raw_ram_gather_write(
    context: *mut core::ffi::c_void,
    reg: *const core::ffi::c_void,
    reg_len: usize,
    val: *const core::ffi::c_void,
    val_len: usize,
) -> i32 {
    let data = context as *mut regmap_ram_data;
    let r: u32;
    let our_buf = (*data).vals as *mut u16;
    let mut i: i32;

    if reg_len != 2 {
        return -EINVAL;
    }
    if val_len % 2 != 0 {
        return -EINVAL;
    }

    r = decode_reg((*data).reg_endian, reg);
    if !(*data).noinc_reg.is_null() && (*data).noinc_reg.unwrap()(data, r) {
        core::ptr::copy_nonoverlapping(
            (val as *const u8).add(val_len - 2),
            (our_buf as *mut u8).add((r as usize) * 2),
            2,
        );
        (*data).written.add(r as usize).write(true);
    } else {
        core::ptr::copy_nonoverlapping(
            val as *const u8,
            (our_buf as *mut u8).add((r as usize) * 2),
            val_len,
        );

        i = 0;
        while i < (val_len / 2) as i32 {
            (*data).written.add((r as i32 + i) as usize).write(true);
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn regmap_raw_ram_write(
    context: *mut core::ffi::c_void,
    data: *const core::ffi::c_void,
    count: usize,
) -> i32 {
    regmap_raw_ram_gather_write(
        context,
        data,
        2,
        (data as *const u8).add(2) as *const core::ffi::c_void,
        count - 2,
    )
}

unsafe extern "C" fn regmap_raw_ram_read(
    context: *mut core::ffi::c_void,
    reg: *const core::ffi::c_void,
    reg_len: usize,
    val: *mut core::ffi::c_void,
    val_len: usize,
) -> i32 {
    let data = context as *mut regmap_ram_data;
    let r: u32;
    let our_buf = (*data).vals as *mut u16;
    let mut i: i32;

    if reg_len != 2 {
        return -EINVAL;
    }
    if val_len % 2 != 0 {
        return -EINVAL;
    }

    r = decode_reg((*data).reg_endian, reg);
    if !(*data).noinc_reg.is_null() && (*data).noinc_reg.unwrap()(data, r) {
        i = 0;
        while (i as usize) < val_len {
            core::ptr::copy_nonoverlapping(
                (our_buf as *const u8).add((r as usize) * 2),
                (val as *mut u8).add(i as usize),
                2,
            );
            i += 2;
        }
        (*data).read.add(r as usize).write(true);
    } else {
        core::ptr::copy_nonoverlapping(
            (our_buf as *const u8).add((r as usize) * 2),
            val as *mut u8,
            val_len,
        );

        i = 0;
        while i < (val_len / 2) as i32 {
            (*data).read.add((r as i32 + i) as usize).write(true);
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn regmap_raw_ram_free_context(context: *mut core::ffi::c_void) {
    let data = context as *mut regmap_ram_data;

    kfree((*data).vals);
    kfree((*data).read);
    kfree((*data).written);
    kfree(data);
}

static REGMAP_RAW_RAM: regmap_bus = regmap_bus {
    fast_io: true,
    write: Some(regmap_raw_ram_write),
    gather_write: Some(regmap_raw_ram_gather_write),
    read: Some(regmap_raw_ram_read),
    free_context: Some(regmap_raw_ram_free_context),
};

unsafe extern "C" fn __regmap_init_raw_ram(
    dev: *mut device,
    config: *const regmap_config,
    data: *mut regmap_ram_data,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let map: *mut regmap;

    if (*config).reg_bits != 16 {
        return ERR_PTR(-EINVAL);
    }

    if (*config).max_register == 0 {
        pr_crit!("No max_register specified for RAM regmap\n");
        return ERR_PTR(-EINVAL);
    }

    (*data).read = kzalloc_objs_bool((*config).max_register + 1);
    if (*data).read.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*data).written = kzalloc_objs_bool((*config).max_register + 1);
    if (*data).written.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*data).reg_endian = (*config).reg_format_endian;

    map = __regmap_init(dev, &REGMAP_RAW_RAM, data, config, lock_key, lock_name);

    map
}

// EXPORT_SYMBOL_GPL(__regmap_init_raw_ram);
// MODULE_DESCRIPTION("Register map access API - Memory region with raw access");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
