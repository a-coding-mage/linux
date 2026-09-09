/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependencies supplied by the surrounding translation unit.

unsafe extern "C" {
    fn BREAK_TO_DEBUGGER();
}

unsafe fn index_from_vector(vector: u32) -> u32 {
    let mut result: u32 = 0;
    let mut mask: u32 = 1;
    loop {
        if vector == mask { return result; }
        result += 1;
        mask <<= 1;
        if mask == 0 { break; }
    }
    BREAK_TO_DEBUGGER();
    GPIO_ENUM_UNKNOWN
}

unsafe fn offset_to_id(offset: u32, mask: u32, id: *mut gpio_id, en: *mut u32) -> bool {
    match offset {
        mmDC_GPIO_GENERIC_A => {
            *id = GPIO_ID_GENERIC;
            match mask {
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK => { *en = GPIO_GENERIC_A; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK => { *en = GPIO_GENERIC_B; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK => { *en = GPIO_GENERIC_C; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK => { *en = GPIO_GENERIC_D; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK => { *en = GPIO_GENERIC_E; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK => { *en = GPIO_GENERIC_F; true }
                DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK => { *en = GPIO_GENERIC_G; true }
                _ => { BREAK_TO_DEBUGGER(); false }
            }
        }
        mmDC_GPIO_HPD_A => {
            *id = GPIO_ID_HPD;
            match mask {
                DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK => { *en = GPIO_HPD_1; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK => { *en = GPIO_HPD_2; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK => { *en = GPIO_HPD_3; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK => { *en = GPIO_HPD_4; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK => { *en = GPIO_HPD_5; true }
                DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK => { *en = GPIO_HPD_6; true }
                _ => { BREAK_TO_DEBUGGER(); false }
            }
        }
        mmDC_GPIO_SYNCA_A => {
            *id = GPIO_ID_SYNC;
            match mask {
                DC_GPIO_SYNCA_A__DC_GPIO_HSYNCA_A_MASK => { *en = GPIO_SYNC_HSYNC_A; true }
                DC_GPIO_SYNCA_A__DC_GPIO_VSYNCA_A_MASK => { *en = GPIO_SYNC_VSYNC_A; true }
                _ => { BREAK_TO_DEBUGGER(); false }
            }
        }
        mmDC_GPIO_GENLK_A => {
            *id = GPIO_ID_GSL;
            match mask {
                DC_GPIO_GENLK_A__DC_GPIO_GENLK_CLK_A_MASK => { *en = GPIO_GSL_GENLOCK_CLOCK; true }
                DC_GPIO_GENLK_A__DC_GPIO_GENLK_VSYNC_A_MASK => { *en = GPIO_GSL_GENLOCK_VSYNC; true }
                DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_A_A_MASK => { *en = GPIO_GSL_SWAPLOCK_A; true }
                DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_B_A_MASK => { *en = GPIO_GSL_SWAPLOCK_B; true }
                _ => { BREAK_TO_DEBUGGER(); false }
            }
        }
        mmGPIOPAD_A => {
            *id = GPIO_ID_GPIO_PAD;
            *en = index_from_vector(mask);
            *en <= GPIO_GPIO_PAD_MAX
        }
        mmDC_GPIO_DDC1_A => { *en = GPIO_DDC_LINE_DDC1; true }
        mmDC_GPIO_DDC2_A => { *en = GPIO_DDC_LINE_DDC2; true }
        mmDC_GPIO_DDC3_A => { *en = GPIO_DDC_LINE_DDC3; true }
        mmDC_GPIO_DDC4_A => { *en = GPIO_DDC_LINE_DDC4; true }
        mmDC_GPIO_DDC5_A => { *en = GPIO_DDC_LINE_DDC5; true }
        mmDC_GPIO_DDC6_A => { *en = GPIO_DDC_LINE_DDC6; true }
        mmDC_GPIO_DDCVGA_A => { *en = GPIO_DDC_LINE_DDC_VGA; true }
        mmDC_GPIO_I2CPAD_A => { *en = GPIO_DDC_LINE_I2C_PAD; true }
        mmDC_GPIO_PWRSEQ_A | mmDC_GPIO_PAD_STRENGTH_1 | mmDC_GPIO_PAD_STRENGTH_2 | mmDC_GPIO_DEBUG => false,
        _ => { BREAK_TO_DEBUGGER(); false }
    }
}

unsafe fn id_to_offset(id: gpio_id, en: u32, info: *mut gpio_pin_info) -> bool {
    let mut result = true;
    match id {
        GPIO_ID_DDC_DATA | GPIO_ID_DDC_CLOCK => {
            (*info).mask = if id == GPIO_ID_DDC_DATA { DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK } else { DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK };
            (*info).offset = match en {
                GPIO_DDC_LINE_DDC1 => mmDC_GPIO_DDC1_A, GPIO_DDC_LINE_DDC2 => mmDC_GPIO_DDC2_A,
                GPIO_DDC_LINE_DDC3 => mmDC_GPIO_DDC3_A, GPIO_DDC_LINE_DDC4 => mmDC_GPIO_DDC4_A,
                GPIO_DDC_LINE_DDC5 => mmDC_GPIO_DDC5_A, GPIO_DDC_LINE_DDC6 => mmDC_GPIO_DDC6_A,
                GPIO_DDC_LINE_DDC_VGA => mmDC_GPIO_DDCVGA_A, GPIO_DDC_LINE_I2C_PAD => mmDC_GPIO_I2CPAD_A,
                _ => { BREAK_TO_DEBUGGER(); result = false; 0 }
            };
        }
        GPIO_ID_GENERIC => { (*info).offset = mmDC_GPIO_GENERIC_A; (*info).mask = match en {
            GPIO_GENERIC_A => DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK, GPIO_GENERIC_B => DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK,
            GPIO_GENERIC_C => DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK, GPIO_GENERIC_D => DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK,
            GPIO_GENERIC_E => DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK, GPIO_GENERIC_F => DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK,
            GPIO_GENERIC_G => DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK, _ => { BREAK_TO_DEBUGGER(); result = false; 0 }
        }; }
        GPIO_ID_HPD => { (*info).offset = mmDC_GPIO_HPD_A; (*info).mask = match en {
            GPIO_HPD_1 => DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK, GPIO_HPD_2 => DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK,
            GPIO_HPD_3 => DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK, GPIO_HPD_4 => DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK,
            GPIO_HPD_5 => DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK, GPIO_HPD_6 => DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK,
            _ => { BREAK_TO_DEBUGGER(); result = false; 0 }
        }; }
        GPIO_ID_SYNC => match en { GPIO_SYNC_HSYNC_A => { (*info).offset=mmDC_GPIO_SYNCA_A; (*info).mask=DC_GPIO_SYNCA_A__DC_GPIO_HSYNCA_A_MASK; }, GPIO_SYNC_VSYNC_A => { (*info).offset=mmDC_GPIO_SYNCA_A; (*info).mask=DC_GPIO_SYNCA_A__DC_GPIO_VSYNCA_A_MASK; }, _ => { BREAK_TO_DEBUGGER(); result=false; } },
        GPIO_ID_GSL => { (*info).offset=mmDC_GPIO_GENLK_A; (*info).mask=match en { GPIO_GSL_GENLOCK_CLOCK=>DC_GPIO_GENLK_A__DC_GPIO_GENLK_CLK_A_MASK, GPIO_GSL_GENLOCK_VSYNC=>DC_GPIO_GENLK_A__DC_GPIO_GENLK_VSYNC_A_MASK, GPIO_GSL_SWAPLOCK_A=>DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_A_A_MASK, GPIO_GSL_SWAPLOCK_B=>DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_B_A_MASK, _=>{BREAK_TO_DEBUGGER(); result=false; 0} }; }
        GPIO_ID_GPIO_PAD => { (*info).offset=mmGPIOPAD_A; (*info).mask=1 << en; result=(*info).mask <= GPIO_GPIO_PAD_MAX; }
        _ => { BREAK_TO_DEBUGGER(); result=false; }
    }
    if result { (*info).offset_y=(*info).offset+2; (*info).offset_en=(*info).offset+1; (*info).offset_mask=(*info).offset-1; (*info).mask_y=(*info).mask; (*info).mask_en=(*info).mask; (*info).mask_mask=(*info).mask; }
    result
}

static funcs: hw_translate_funcs = hw_translate_funcs { offset_to_id: Some(offset_to_id), id_to_offset: Some(id_to_offset) };

pub unsafe fn dal_hw_translate_dce60_init(translate: *mut hw_translate) { (*translate).funcs = &funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
