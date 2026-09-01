// SPDX-License-Identifier: GPL-2.0-only
//
// rt-sdw-common.c
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//

/*
 * This file defines common functions used with Realtek soundwire codecs.
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <linux/bitops.h>
// #include <linux/soundwire/sdw_registers.h>
// #include <sound/jack.h>
// #include "rt-sdw-common.h"

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

type c_int = i32;
type c_uint = u32;
type u8 = u8;

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_err_ratelimited(fmt: *const core::ffi::c_char, ...);

    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_HEADPHONE: c_uint;
    static SND_JACK_HEADSET: c_uint;

    static SDCA_NUM_JACK_CODEC: c_uint;
    static SDCA_NUM_HID: c_uint;
    static RT_SDCA_CTL_DETECTED_MODE: c_uint;
    static RT_SDCA_CTL_SELECTED_MODE: c_uint;
    static RT_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint;
    static RT_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint;

    fn SDW_SDCA_CTL(function_num: c_uint, entity_id: c_uint, control: c_uint, channel: c_uint) -> c_uint;
}

#[inline]
const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

#[inline]
fn set_mask_bits(ptr: *mut c_uint, mask: c_uint, bits: c_uint) {
    unsafe {
        *ptr = (*ptr & !mask) | (bits & mask);
    }
}

/**
 * rt_sdca_index_write - Write a value to Realtek defined register.
 *
 * @map: map for setting.
 * @nid: Realtek-defined ID.
 * @reg: register.
 * @value: value.
 *
 * A value of zero will be returned on success, a negative errno will
 * be returned in error cases.
 */
#[no_mangle]
pub unsafe extern "C" fn rt_sdca_index_write(
    map: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let addr: c_uint = (nid << 20) | reg;
    let ret: c_int;

    ret = regmap_write(map, addr, value);
    if ret < 0 {
        pr_err(
            c"Failed to set value: %06x <= %04x ret=%d\n".as_ptr(),
            addr,
            value,
            ret,
        );
    }

    ret
}
// EXPORT_SYMBOL_GPL(rt_sdca_index_write);

/**
 * rt_sdca_index_read - Read value from Realtek defined register.
 *
 * @map: map for setting.
 * @nid: Realtek-defined ID.
 * @reg: register.
 * @value: value.
 *
 * A value of zero will be returned on success, a negative errno will
 * be returned in error cases.
 */
#[no_mangle]
pub unsafe extern "C" fn rt_sdca_index_read(
    map: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let addr: c_uint = (nid << 20) | reg;
    let ret: c_int;

    ret = regmap_read(map, addr, value);
    if ret < 0 {
        pr_err(
            c"Failed to get value: %06x => %04x ret=%d\n".as_ptr(),
            addr,
            *value,
            ret,
        );
    }

    ret
}
// EXPORT_SYMBOL_GPL(rt_sdca_index_read);

/**
 * rt_sdca_index_update_bits - Update value on Realtek defined register.
 *
 * @map: map for setting.
 * @nid: Realtek-defined ID.
 * @reg: register.
 * @mask: Bitmask to change
 * @val: New value for bitmask
 *
 * A value of zero will be returned on success, a negative errno will
 * be returned in error cases.
 */

#[no_mangle]
pub unsafe extern "C" fn rt_sdca_index_update_bits(
    map: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    let mut tmp: c_uint = 0;
    let mut ret: c_int;

    ret = rt_sdca_index_read(map, nid, reg, &mut tmp);
    if ret < 0 {
        return ret;
    }

    set_mask_bits(&mut tmp, mask, val);
    ret = rt_sdca_index_write(map, nid, reg, tmp);
    ret
}
// EXPORT_SYMBOL_GPL(rt_sdca_index_update_bits);

/**
 * rt_sdca_btn_type - Decision of button type.
 *
 * @buffer: UMP message buffer.
 *
 * A button type will be returned regarding to buffer,
 * it returns zero if buffer cannot be recognized.
 */
#[no_mangle]
pub unsafe extern "C" fn rt_sdca_btn_type(buffer: *mut u8) -> c_int {
    let mut btn_type: u8 = 0;
    let mut ret: c_int = 0;

    btn_type |= *buffer.add(0) & 0xf;
    btn_type |= (*buffer.add(0) >> 4) & 0xf;
    btn_type |= *buffer.add(1) & 0xf;
    btn_type |= (*buffer.add(1) >> 4) & 0xf;

    if (btn_type as c_uint & BIT(0)) != 0 {
        ret |= SND_JACK_BTN_2;
    }
    if (btn_type as c_uint & BIT(1)) != 0 {
        ret |= SND_JACK_BTN_3;
    }
    if (btn_type as c_uint & BIT(2)) != 0 {
        ret |= SND_JACK_BTN_0;
    }
    if (btn_type as c_uint & BIT(3)) != 0 {
        ret |= SND_JACK_BTN_1;
    }

    ret
}
// EXPORT_SYMBOL_GPL(rt_sdca_btn_type);

/**
 * rt_sdca_headset_detect - Headset jack type detection.
 *
 * @map: map for setting.
 * @entity_id: SDCA entity ID.
 *
 * A headset jack type will be returned, a negative errno will
 * be returned in error cases.
 */
#[no_mangle]
pub unsafe extern "C" fn rt_sdca_headset_detect(map: *mut regmap, entity_id: c_uint) -> c_int {
    let mut det_mode: c_uint = 0;
    let jack_type: c_uint;
    let mut ret: c_int;

    /* get detected_mode */
    ret = regmap_read(
        map,
        SDW_SDCA_CTL(
            SDCA_NUM_JACK_CODEC,
            entity_id,
            RT_SDCA_CTL_DETECTED_MODE,
            0,
        ),
        &mut det_mode,
    );

    if ret < 0 {
        pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt_sdca_headset_detect".as_ptr(), ret);
        return ret;
    }

    match det_mode {
        0x03 => {
            jack_type = SND_JACK_HEADPHONE;
        }
        0x05 => {
            jack_type = SND_JACK_HEADSET;
        }
        _ => {
            jack_type = 0;
        }
    }

    /* write selected_mode */
    if det_mode != 0 {
        ret = regmap_write(
            map,
            SDW_SDCA_CTL(
                SDCA_NUM_JACK_CODEC,
                entity_id,
                RT_SDCA_CTL_SELECTED_MODE,
                0,
            ),
            det_mode,
        );
        if ret < 0 {
            pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt_sdca_headset_detect".as_ptr(), ret);
            return ret;
        }
    }

    jack_type as c_int
}
// EXPORT_SYMBOL_GPL(rt_sdca_headset_detect);

/**
 * rt_sdca_button_detect - Read UMP message and decide button type.
 *
 * @map: map for setting.
 * @entity_id: SDCA entity ID.
 * @hid_buf_addr: HID buffer address.
 * @hid_id: Report ID for HID.
 *
 * A button type will be returned regarding to buffer,
 * it returns zero if buffer cannot be recognized.
 */
#[no_mangle]
pub unsafe extern "C" fn rt_sdca_button_detect(
    map: *mut regmap,
    entity_id: c_uint,
    hid_buf_addr: c_uint,
    hid_id: c_uint,
) -> c_int {
    let mut btn_type: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut idx: c_uint;
    let mut val: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut buf: [u8; 3] = [0; 3];
    let mut ret: c_int;

    /* get current UMP message owner */
    ret = regmap_read(
        map,
        SDW_SDCA_CTL(
            SDCA_NUM_HID,
            entity_id,
            RT_SDCA_CTL_HIDTX_CURRENT_OWNER,
            0,
        ),
        &mut owner,
    );
    if ret < 0 {
        return 0;
    }

    /* if owner is device then there is no button event from device */
    if owner == 1 {
        return 0;
    }

    /* read UMP message offset */
    ret = regmap_read(
        map,
        SDW_SDCA_CTL(
            SDCA_NUM_HID,
            entity_id,
            RT_SDCA_CTL_HIDTX_MESSAGE_OFFSET,
            0,
        ),
        &mut offset,
    );
    if ret >= 0 {
        idx = 0;
        while idx < buf.len() as c_uint {
            ret = regmap_read(map, hid_buf_addr + offset + idx, &mut val);
            if ret < 0 {
                break;
            }
            buf[idx as usize] = (val & 0xff) as u8;
            idx += 1;
        }

        /* Report ID for HID */
        if ret >= 0 && buf[0] as c_uint == hid_id {
            btn_type = rt_sdca_btn_type(buf.as_mut_ptr()) as c_uint;
        }
    }

    /* Host is owner, so set back to device */
    if owner == 0 {
        /* set owner to device */
        regmap_write(
            map,
            SDW_SDCA_CTL(
                SDCA_NUM_HID,
                entity_id,
                RT_SDCA_CTL_HIDTX_CURRENT_OWNER,
                0,
            ),
            0x01,
        );
    }

    btn_type as c_int
}
// EXPORT_SYMBOL_GPL(rt_sdca_button_detect);

// MODULE_DESCRIPTION("Realtek soundwire common functions");
// MODULE_AUTHOR("jack yu <jack.yu@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
