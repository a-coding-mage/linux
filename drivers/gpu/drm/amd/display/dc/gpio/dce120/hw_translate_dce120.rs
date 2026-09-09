/*
 * Copyright 2013-15 Advanced Micro Devices, Inc.
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
 */

// Dependencies supplied by the surrounding translation unit.

macro_rules! BASE_INNER { ($seg:ident) => { DCE_BASE__INST0_SEG$seg }; }
macro_rules! BASE { ($seg:ident) => { BASE_INNER!($seg) }; }
macro_rules! REG { ($name:ident) => { BASE!(mm$name##_BASE_IDX) + mm$name }; }

unsafe fn offset_to_id(offset: u32, mask: u32, id: *mut gpio_id, en: *mut u32) -> bool {
    match offset {
        REG!(DC_GPIO_GENERIC_A) => {
            *id = GPIO_ID_GENERIC;
            match mask {
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK => { *en = GPIO_GENERIC_A; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK => { *en = GPIO_GENERIC_B; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK => { *en = GPIO_GENERIC_C; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK => { *en = GPIO_GENERIC_D; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK => { *en = GPIO_GENERIC_E; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK => { *en = GPIO_GENERIC_F; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK => { *en = GPIO_GENERIC_G; true }
                _ => { ASSERT_CRITICAL!(false); false }
            }
        }
        REG!(DC_GPIO_HPD_A) => {
            *id = GPIO_ID_HPD;
            match mask {
                DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK => { *en = GPIO_HPD_1; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK => { *en = GPIO_HPD_2; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK => { *en = GPIO_HPD_3; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK => { *en = GPIO_HPD_4; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK => { *en = GPIO_HPD_5; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK => { *en = GPIO_HPD_6; true }
                _ => { ASSERT_CRITICAL!(false); false }
            }
        }
        REG!(DC_GPIO_SYNCA_A) => {
            *id = GPIO_ID_SYNC;
            match mask {
                DC_GPIO_SYNCA_A__DC_GPIO_HSYNCA_A_MASK => { *en = GPIO_SYNC_HSYNC_A; true }
                DC_GPIO_SYNCA_A__DC_GPIO_VSYNCA_A_MASK => { *en = GPIO_SYNC_VSYNC_A; true }
                _ => { ASSERT_CRITICAL!(false); false }
            }
        }
        REG!(DC_GPIO_GENLK_A) => {
            *id = GPIO_ID_GSL;
            match mask {
                DC_GPIO_GENLK_A__DC_GPIO_GENLK_CLK_A_MASK => { *en = GPIO_GSL_GENLOCK_CLOCK; true }
                DC_GPIO_GENLK_A__DC_GPIO_GENLK_VSYNC_A_MASK => { *en = GPIO_GSL_GENLOCK_VSYNC; true }
                DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_A_A_MASK => { *en = GPIO_GSL_SWAPLOCK_A; true }
                DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_B_A_MASK => { *en = GPIO_GSL_SWAPLOCK_B; true }
                _ => { ASSERT_CRITICAL!(false); false }
            }
        }
        REG!(DC_GPIO_DDC1_A) => { *en = GPIO_DDC_LINE_DDC1; true }
        REG!(DC_GPIO_DDC2_A) => { *en = GPIO_DDC_LINE_DDC2; true }
        REG!(DC_GPIO_DDC3_A) => { *en = GPIO_DDC_LINE_DDC3; true }
        REG!(DC_GPIO_DDC4_A) => { *en = GPIO_DDC_LINE_DDC4; true }
        REG!(DC_GPIO_DDC5_A) => { *en = GPIO_DDC_LINE_DDC5; true }
        REG!(DC_GPIO_DDC6_A) => { *en = GPIO_DDC_LINE_DDC6; true }
        REG!(DC_GPIO_DDCVGA_A) => { *en = GPIO_DDC_LINE_DDC_VGA; true }
        REG!(DC_GPIO_I2CPAD_A) => { *en = GPIO_DDC_LINE_I2C_PAD; true }
        REG!(DC_GPIO_PWRSEQ_A) | REG!(DC_GPIO_PAD_STRENGTH_1) |
        REG!(DC_GPIO_PAD_STRENGTH_2) | REG!(DC_GPIO_DEBUG) => false,
        _ => { ASSERT_CRITICAL!(false); false }
    }
}

unsafe fn id_to_offset(id: gpio_id, en: u32, info: *mut gpio_pin_info) -> bool {
    let mut result = true;
    match id {
        GPIO_ID_DDC_DATA | GPIO_ID_DDC_CLOCK => {
            (*info).mask = if id == GPIO_ID_DDC_DATA { DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK } else { DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK };
            (*info).offset = match en {
                GPIO_DDC_LINE_DDC1 => REG!(DC_GPIO_DDC1_A), GPIO_DDC_LINE_DDC2 => REG!(DC_GPIO_DDC2_A),
                GPIO_DDC_LINE_DDC3 => REG!(DC_GPIO_DDC3_A), GPIO_DDC_LINE_DDC4 => REG!(DC_GPIO_DDC4_A),
                GPIO_DDC_LINE_DDC5 => REG!(DC_GPIO_DDC5_A), GPIO_DDC_LINE_DDC6 => REG!(DC_GPIO_DDC6_A),
                GPIO_DDC_LINE_DDC_VGA => REG!(DC_GPIO_DDCVGA_A), GPIO_DDC_LINE_I2C_PAD => REG!(DC_GPIO_I2CPAD_A),
                _ => { ASSERT_CRITICAL!(false); result = false; 0 }
            };
        }
        GPIO_ID_GENERIC => { (*info).offset = REG!(DC_GPIO_GENERIC_A); (*info).mask = match en {
            GPIO_GENERIC_A => DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK, GPIO_GENERIC_B => DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK,
            GPIO_GENERIC_C => DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK, GPIO_GENERIC_D => DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK,
            GPIO_GENERIC_E => DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK, GPIO_GENERIC_F => DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK,
            GPIO_GENERIC_G => DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK, _ => { ASSERT_CRITICAL!(false); result = false; 0 }
        }; }
        GPIO_ID_HPD => { (*info).offset = REG!(DC_GPIO_HPD_A); (*info).mask = match en {
            GPIO_HPD_1 => DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK, GPIO_HPD_2 => DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK,
            GPIO_HPD_3 => DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK, GPIO_HPD_4 => DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK,
            GPIO_HPD_5 => DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK, GPIO_HPD_6 => DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK,
            _ => { ASSERT_CRITICAL!(false); result = false; 0 }
        }; }
        GPIO_ID_SYNC => match en {
            GPIO_SYNC_HSYNC_A => { (*info).offset = REG!(DC_GPIO_SYNCA_A); (*info).mask = DC_GPIO_SYNCA_A__DC_GPIO_HSYNCA_A_MASK; }
            GPIO_SYNC_VSYNC_A => { (*info).offset = REG!(DC_GPIO_SYNCA_A); (*info).mask = DC_GPIO_SYNCA_A__DC_GPIO_VSYNCA_A_MASK; }
            _ => { ASSERT_CRITICAL!(false); result = false; }
        },
        GPIO_ID_GSL => { (*info).offset = REG!(DC_GPIO_GENLK_A); (*info).mask = match en {
            GPIO_GSL_GENLOCK_CLOCK => DC_GPIO_GENLK_A__DC_GPIO_GENLK_CLK_A_MASK, GPIO_GSL_GENLOCK_VSYNC => DC_GPIO_GENLK_A__DC_GPIO_GENLK_VSYNC_A_MASK,
            GPIO_GSL_SWAPLOCK_A => DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_A_A_MASK, GPIO_GSL_SWAPLOCK_B => DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_B_A_MASK,
            _ => { ASSERT_CRITICAL!(false); result = false; 0 }
        }; }
        _ => { ASSERT_CRITICAL!(false); result = false; }
    }
    if result { (*info).offset_y = (*info).offset + 2; (*info).offset_en = (*info).offset + 1; (*info).offset_mask = (*info).offset - 1; (*info).mask_y = (*info).mask; (*info).mask_en = (*info).mask; (*info).mask_mask = (*info).mask; }
    result
}

static funcs: hw_translate_funcs = hw_translate_funcs { offset_to_id: Some(offset_to_id), id_to_offset: Some(id_to_offset) };

pub unsafe fn dal_hw_translate_dce120_init(tr: *mut hw_translate) { (*tr).funcs = &funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
