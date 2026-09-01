// SPDX-License-Identifier: GPL-2.0+
//
// fs-amp-lib.c --- Common library for FourSemi Audio Amplifiers
//
// Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.

// C dependencies translated as external Rust dependencies:
// linux/crc16.h, linux/device.h, linux/firmware.h, linux/module.h,
// linux/slab.h, and "fs-amp-lib.h".

use core::ffi::{c_char, c_int};
use core::mem::size_of;
use core::ptr;

use crate::*;

unsafe fn fs_get_scene_count(amp_lib: *mut fs_amp_lib) -> c_int {
    let table: *const fs_fwm_table;
    let count: c_int;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } {
        return -EINVAL;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_SCENE as usize] };
    if table.is_null() {
        return -EFAULT;
    }

    count = unsafe { (*table).size as usize / size_of::<fs_scene_index>() } as c_int;
    if count < 1 || count > FS_SCENE_COUNT_MAX {
        unsafe {
            dev_err!(
                (*amp_lib).dev,
                c"Invalid scene count: %d\n".as_ptr(),
                count
            );
        }
        return -ERANGE;
    }

    count
}

unsafe fn fs_get_fwm_string(
    amp_lib: *mut fs_amp_lib,
    offset: c_int,
    pstr: *mut *const c_char,
) {
    let table: *const fs_fwm_table;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || pstr.is_null() {
        return;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_STRING as usize] };
    if !table.is_null()
        && offset > 0
        && (offset as usize) < unsafe { (*table).size as usize + size_of::<fs_fwm_table>() }
    {
        unsafe {
            *pstr = (table as *const c_char).add(offset as usize);
        }
    } else {
        unsafe {
            *pstr = ptr::null();
        }
    }
}

unsafe fn fs_get_scene_reg(
    amp_lib: *mut fs_amp_lib,
    offset: c_int,
    scene: *mut fs_amp_scene,
) {
    let table: *const fs_fwm_table;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || scene.is_null() {
        return;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_REG as usize] };
    if !table.is_null()
        && offset > 0
        && (offset as usize) < unsafe { (*table).size as usize + size_of::<fs_fwm_table>() }
    {
        unsafe {
            (*scene).reg = (table as *const c_char).add(offset as usize) as *mut fs_reg_table;
        }
    } else {
        unsafe {
            (*scene).reg = ptr::null_mut();
        }
    }
}

unsafe fn fs_get_scene_model(
    amp_lib: *mut fs_amp_lib,
    offset: c_int,
    scene: *mut fs_amp_scene,
) {
    let table: *const fs_fwm_table;
    let ptr_: *const c_char;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || scene.is_null() {
        return;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_MODEL as usize] };
    ptr_ = table as *const c_char;
    if !table.is_null()
        && offset > 0
        && (offset as usize) < unsafe { (*table).size as usize + size_of::<fs_fwm_table>() }
    {
        unsafe {
            (*scene).model = ptr_.add(offset as usize) as *mut fs_file_table;
        }
    } else {
        unsafe {
            (*scene).model = ptr::null_mut();
        }
    }
}

unsafe fn fs_get_scene_effect(
    amp_lib: *mut fs_amp_lib,
    offset: c_int,
    scene: *mut fs_amp_scene,
) {
    let table: *const fs_fwm_table;
    let ptr_: *const c_char;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || scene.is_null() {
        return;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_EFFECT as usize] };
    ptr_ = table as *const c_char;
    if !table.is_null()
        && offset > 0
        && (offset as usize) < unsafe { (*table).size as usize + size_of::<fs_fwm_table>() }
    {
        unsafe {
            (*scene).effect = ptr_.add(offset as usize) as *mut fs_file_table;
        }
    } else {
        unsafe {
            (*scene).effect = ptr::null_mut();
        }
    }
}

unsafe fn fs_parse_scene_tables(amp_lib: *mut fs_amp_lib) -> c_int {
    let mut scene_index: *const fs_scene_index;
    let table: *const fs_fwm_table;
    let mut scene: *mut fs_amp_scene;
    let mut idx: c_int;
    let count: c_int;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } {
        return -EINVAL;
    }

    count = unsafe { fs_get_scene_count(amp_lib) };
    if count <= 0 {
        return -EFAULT;
    }

    scene = unsafe {
        devm_kcalloc(
            (*amp_lib).dev,
            count as usize,
            size_of::<fs_amp_scene>(),
            GFP_KERNEL,
        ) as *mut fs_amp_scene
    };
    if scene.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*amp_lib).scene_count = count;
        (*amp_lib).scene = scene;
    }

    table = unsafe { (*amp_lib).table[FS_INDEX_SCENE as usize] };
    scene_index = unsafe { (*table).buf.as_ptr() as *const fs_scene_index };

    idx = 0;
    while idx < count {
        unsafe {
            fs_get_fwm_string(amp_lib, (*scene_index).name, &mut (*scene).name);
            if (*scene).name.is_null() {
                (*scene).name = devm_kasprintf(
                    (*amp_lib).dev,
                    GFP_KERNEL,
                    c"S%d".as_ptr(),
                    idx,
                );
            }
            dev_dbg!(
                (*amp_lib).dev,
                c"scene.%d name: %s\n".as_ptr(),
                idx,
                (*scene).name
            );
            fs_get_scene_reg(amp_lib, (*scene_index).reg, scene);
            fs_get_scene_model(amp_lib, (*scene_index).model, scene);
            fs_get_scene_effect(amp_lib, (*scene_index).effect, scene);
            scene = scene.add(1);
            scene_index = scene_index.add(1);
        }
        idx += 1;
    }

    0
}

unsafe fn fs_parse_all_tables(amp_lib: *mut fs_amp_lib) -> c_int {
    let mut table: *const fs_fwm_table;
    let mut index: *const fs_fwm_index;
    let ptr_: *const c_char;
    let mut idx: c_int;
    let count: c_int;
    let ret: c_int;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || unsafe { (*amp_lib).hdr.is_null() } {
        return -EINVAL;
    }

    /* Parse all fwm tables */
    table = unsafe { (*(*amp_lib).hdr).params.as_ptr() as *const fs_fwm_table };
    index = unsafe { (*table).buf.as_ptr() as *const fs_fwm_index };
    count = unsafe { (*table).size as usize / size_of::<fs_fwm_index>() } as c_int;

    idx = 0;
    while idx < count {
        unsafe {
            if (*index).type_ >= FS_INDEX_MAX {
                return -ERANGE;
            }
            ptr_ = (table as *const c_char).add((*index).offset as c_int as usize);
            (*amp_lib).table[(*index).type_ as usize] = ptr_ as *mut fs_fwm_table;
            index = index.add(1);
        }
        idx += 1;
    }

    /* Parse all scene tables */
    ret = unsafe { fs_parse_scene_tables(amp_lib) };
    if ret != 0 {
        unsafe {
            dev_err!(
                (*amp_lib).dev,
                c"Failed to parse scene: %d\n".as_ptr(),
                ret
            );
        }
    }

    ret
}

unsafe fn fs_verify_firmware(amp_lib: *mut fs_amp_lib) -> c_int {
    let hdr: *const fs_fwm_header;
    let crcsum: c_int;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || unsafe { (*amp_lib).hdr.is_null() } {
        return -EINVAL;
    }

    hdr = unsafe { (*amp_lib).hdr };

    /* Verify the crcsum code */
    crcsum = unsafe {
        crc16(
            0x0000,
            &(*hdr).crc_size as *const _ as *const c_char,
            (*hdr).crc_size,
        )
    } as c_int;
    if crcsum != unsafe { (*hdr).crc16 as c_int } {
        unsafe {
            dev_err!(
                (*amp_lib).dev,
                c"Failed to checksum: %x-%x\n".as_ptr(),
                crcsum,
                (*hdr).crc16
            );
        }
        return -EFAULT;
    }

    /* Verify the devid(chip_type) */
    if unsafe { (*amp_lib).devid } != unsafe { LO_U16((*hdr).chip_type) } {
        unsafe {
            dev_err!(
                (*amp_lib).dev,
                c"DEVID dismatch: %04X#%04X\n".as_ptr(),
                (*amp_lib).devid,
                (*hdr).chip_type
            );
        }
        return -EINVAL;
    }

    0
}

unsafe fn fs_print_firmware_info(amp_lib: *mut fs_amp_lib) {
    let hdr: *const fs_fwm_header;
    let mut pro_name: *const c_char = ptr::null();
    let mut dev_name: *const c_char = ptr::null();

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || unsafe { (*amp_lib).hdr.is_null() } {
        return;
    }

    hdr = unsafe { (*amp_lib).hdr };

    unsafe {
        fs_get_fwm_string(amp_lib, (*hdr).project, &mut pro_name);
        fs_get_fwm_string(amp_lib, (*hdr).device, &mut dev_name);

        dev_info!(
            (*amp_lib).dev,
            c"Project: %s Device: %s\n".as_ptr(),
            if !pro_name.is_null() { pro_name } else { c"null".as_ptr() },
            if !dev_name.is_null() { dev_name } else { c"null".as_ptr() }
        );

        dev_info!(
            (*amp_lib).dev,
            c"Date: %04d%02d%02d-%02d%02d\n".as_ptr(),
            (*hdr).date.year,
            (*hdr).date.month,
            (*hdr).date.day,
            (*hdr).date.hour,
            (*hdr).date.minute
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn fs_amp_load_firmware(
    amp_lib: *mut fs_amp_lib,
    name: *const c_char,
) -> c_int {
    // Original C used: const struct firmware *cont __free(firmware) = NULL;
    let mut cont: *const firmware = ptr::null();
    let hdr: *mut fs_fwm_header;
    let mut ret: c_int;

    if amp_lib.is_null() || unsafe { (*amp_lib).dev.is_null() } || name.is_null() {
        return -EINVAL;
    }

    ret = unsafe { request_firmware(&mut cont, name, (*amp_lib).dev) };
    if ret != 0 {
        unsafe {
            dev_err!(
                (*amp_lib).dev,
                c"Failed to request %s: %d\n".as_ptr(),
                name,
                ret
            );
        }
        return ret;
    }

    unsafe {
        dev_info!(
            (*amp_lib).dev,
            c"Loading %s - size: %zu\n".as_ptr(),
            name,
            (*cont).size
        );
    }

    hdr = unsafe {
        devm_kmemdup(
            (*amp_lib).dev,
            (*cont).data,
            (*cont).size,
            GFP_KERNEL,
        ) as *mut fs_fwm_header
    };
    if hdr.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*amp_lib).hdr = hdr;
    }
    ret = unsafe { fs_verify_firmware(amp_lib) };
    if ret != 0 {
        unsafe {
            (*amp_lib).hdr = ptr::null_mut();
        }
        return ret;
    }

    ret = unsafe { fs_parse_all_tables(amp_lib) };
    if ret != 0 {
        unsafe {
            (*amp_lib).hdr = ptr::null_mut();
        }
        return ret;
    }

    unsafe {
        fs_print_firmware_info(amp_lib);
    }

    0
}
// EXPORT_SYMBOL_GPL(fs_amp_load_firmware);

// MODULE_AUTHOR("Nick Li <nick.li@foursemi.com>");
// MODULE_DESCRIPTION("FourSemi audio amplifier library");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
