/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub unsafe fn dal_hw_translate_init(
    translate: *mut hw_translate,
    dce_version: dce_version,
    dce_environment: dce_environment,
) -> bool {
    let _ = dce_environment;
    match dce_version {
        #[cfg(CONFIG_DRM_AMD_DC_SI)]
        DCE_VERSION_6_0 | DCE_VERSION_6_1 | DCE_VERSION_6_4 => {
            dal_hw_translate_dce60_init(translate);
            true
        }
        DCE_VERSION_8_0 | DCE_VERSION_8_1 | DCE_VERSION_8_3 => {
            dal_hw_translate_dce80_init(translate);
            true
        }
        DCE_VERSION_10_0 | DCE_VERSION_11_0 | DCE_VERSION_11_2 | DCE_VERSION_11_22 => {
            dal_hw_translate_dce110_init(translate);
            true
        }
        DCE_VERSION_12_0 | DCE_VERSION_12_1 => {
            dal_hw_translate_dce120_init(translate);
            true
        }
        DCN_VERSION_1_0 | DCN_VERSION_1_01 => {
            dal_hw_translate_dcn10_init(translate);
            true
        }
        DCN_VERSION_2_0 => {
            dal_hw_translate_dcn20_init(translate);
            true
        }
        DCN_VERSION_2_01 | DCN_VERSION_2_1 => {
            dal_hw_translate_dcn21_init(translate);
            true
        }
        DCN_VERSION_3_0 | DCN_VERSION_3_01 | DCN_VERSION_3_02 | DCN_VERSION_3_03 |
        DCN_VERSION_3_1 | DCN_VERSION_3_14 | DCN_VERSION_3_16 => {
            dal_hw_translate_dcn30_init(translate);
            true
        }
        DCN_VERSION_3_15 => {
            dal_hw_translate_dcn315_init(translate);
            true
        }
        DCN_VERSION_3_2 | DCN_VERSION_3_21 | DCN_VERSION_3_5 | DCN_VERSION_3_51 |
        DCN_VERSION_3_6 => {
            dal_hw_translate_dcn32_init(translate);
            true
        }
        DCN_VERSION_4_01 => {
            dal_hw_translate_dcn401_init(translate);
            true
        }
        DCN_VERSION_4_2 => {
            dal_hw_translate_dcn42_init(translate);
            true
        }
        DCN_VERSION_4_2B => {
            dal_hw_translate_dcn42b_init(translate);
            true
        }
        DCN_VERSION_6_0 => {
            dal_hw_translate_dcn60_init(translate);
            true
        }
        _ => {
            BREAK_TO_DEBUGGER();
            false
        }
    }
}

pub unsafe fn dal_hw_translate_gpio_offset_to_id(
    table: *const gpio_id_offset_entry,
    table_size: u32,
    offset: u32,
    mask: u32,
    id: *mut gpio_id,
    en: *mut u32,
) -> bool {
    for i in 0..table_size {
        let entry = &*table.add(i as usize);
        if entry.offset != offset {
            continue;
        }
        if entry.check_mask && entry.mask != mask {
            continue;
        }
        *id = entry.id;
        *en = entry.en;
        return true;
    }
    false
}

/* we don't care about the GPIO_ID for DDC
 * in DdcHandle it will use GPIO_ID_DDC_DATA/GPIO_ID_DDC_CLOCK
 * directly in the create method
 */
pub unsafe fn dal_hw_translate_gpio_ddc_offset_to_id(
    table: *const gpio_ddc_offset_entry,
    table_size: u32,
    offset: u32,
    en: *mut u32,
) -> bool {
    for i in 0..table_size {
        let entry = &*table.add(i as usize);
        if entry.offset != offset {
            continue;
        }
        *en = entry.en;
        return true;
    }
    false
}

pub unsafe fn dal_hw_translate_id_to_offset(
    table: *const gpio_pin_entry,
    table_size: u32,
    id: gpio_id,
    en: u32,
    info: *mut gpio_pin_info,
) -> bool {
    for i in 0..table_size {
        let entry = &*table.add(i as usize);
        if entry.id != id || entry.en != en {
            continue;
        }
        (*info).offset = entry.offset;
        (*info).mask = entry.mask;
        (*info).offset_y = (*info).offset + 2;
        (*info).offset_en = (*info).offset + 1;
        (*info).offset_mask = (*info).offset - 1;
        (*info).mask_y = (*info).mask;
        (*info).mask_en = (*info).mask;
        (*info).mask_mask = (*info).mask;
        return true;
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
