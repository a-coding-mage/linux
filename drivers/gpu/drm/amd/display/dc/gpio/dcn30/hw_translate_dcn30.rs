/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

/* Dependencies are supplied by the surrounding translation unit. */

/* begin *********************
 * macros to expend register list macro defined in HW object header file */

/* DCN */
macro_rules! block { ($name:ident) => {}; }
macro_rules! reg_num { () => { 0 }; }
macro_rules! base_inner { ($seg:ident) => { DCN_BASE__INST0_SEG$seg }; }
macro_rules! base { ($seg:ident) => { base_inner!($seg) }; }
macro_rules! reg { ($reg_name:ident) => { base!(mm$reg_name_BASE_IDX) + mm$reg_name }; }
macro_rules! sf_hpd { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { $field_name: $reg_name$field_name$post_fix }; }

/* macros to expend register list macro defined in HW object header file
 * end *********************/

static GPIO_OFFSETS: [struct_gpio_id_offset_entry; 17] = [
    /* GENERIC */
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_A),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_B),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_C),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_D),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_E),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_F),
    GPIO_MASK_ENTRY!(DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK, GPIO_ID_GENERIC, GPIO_GENERIC_G),
    /* HPD */
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK, GPIO_ID_HPD, GPIO_HPD_1),
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK, GPIO_ID_HPD, GPIO_HPD_2),
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK, GPIO_ID_HPD, GPIO_HPD_3),
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK, GPIO_ID_HPD, GPIO_HPD_4),
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK, GPIO_ID_HPD, GPIO_HPD_5),
    GPIO_MASK_ENTRY!(DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK, GPIO_ID_HPD, GPIO_HPD_6),
    /* GSL */
    GPIO_MASK_ENTRY!(DC_GPIO_GENLK_A, DC_GPIO_GENLK_A__DC_GPIO_GENLK_CLK_A_MASK, GPIO_ID_GSL, GPIO_GSL_GENLOCK_CLOCK),
    GPIO_MASK_ENTRY!(DC_GPIO_GENLK_A, DC_GPIO_GENLK_A__DC_GPIO_GENLK_VSYNC_A_MASK, GPIO_ID_GSL, GPIO_GSL_GENLOCK_VSYNC),
    GPIO_MASK_ENTRY!(DC_GPIO_GENLK_A, DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_A_A_MASK, GPIO_ID_GSL, GPIO_GSL_SWAPLOCK_A),
    GPIO_MASK_ENTRY!(DC_GPIO_GENLK_A, DC_GPIO_GENLK_A__DC_GPIO_SWAPLOCK_B_A_MASK, GPIO_ID_GSL, GPIO_GSL_SWAPLOCK_B),
];

/* DDC */
static DDC_OFFSET_MAP: [struct_gpio_ddc_offset_entry; 7] = [
    (reg!(DC_GPIO_DDC1_A), GPIO_DDC_LINE_DDC1),
    (reg!(DC_GPIO_DDC2_A), GPIO_DDC_LINE_DDC2),
    (reg!(DC_GPIO_DDC3_A), GPIO_DDC_LINE_DDC3),
    (reg!(DC_GPIO_DDC4_A), GPIO_DDC_LINE_DDC4),
    (reg!(DC_GPIO_DDC5_A), GPIO_DDC_LINE_DDC5),
    (reg!(DC_GPIO_DDC6_A), GPIO_DDC_LINE_DDC6),
    (reg!(DC_GPIO_DDCVGA_A), GPIO_DDC_LINE_DDC_VGA),
];

/*
 * GSL is intentionally omitted here.
 * id_to_offset() for GSL is not implemented on this ASIC.
 */
static GPIO_PINS: [struct_gpio_pin_entry; 27] = [
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC1, DC_GPIO_DDC1_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC2, DC_GPIO_DDC2_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC3, DC_GPIO_DDC3_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC4, DC_GPIO_DDC4_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC5, DC_GPIO_DDC5_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC6, DC_GPIO_DDC6_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC_VGA, DC_GPIO_DDCVGA_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC1, DC_GPIO_DDC1_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC2, DC_GPIO_DDC2_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC3, DC_GPIO_DDC3_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC4, DC_GPIO_DDC4_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC5, DC_GPIO_DDC5_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC6, DC_GPIO_DDC6_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC_VGA, DC_GPIO_DDCVGA_A, DC_GPIO_DDC6_A__DC_GPIO_DDC6CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_A, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_B, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICB_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_C, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICC_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_D, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICD_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_E, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICE_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_F, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICF_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_GENERIC, GPIO_GENERIC_G, DC_GPIO_GENERIC_A, DC_GPIO_GENERIC_A__DC_GPIO_GENERICG_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_1, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD1_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_2, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD2_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_3, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD3_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_4, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD4_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_5, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD5_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_HPD, GPIO_HPD_6, DC_GPIO_HPD_A, DC_GPIO_HPD_A__DC_GPIO_HPD6_A_MASK),
];

unsafe fn offset_to_id(offset: u32, mask: u32, id: *mut enum_gpio_id, en: *mut u32) -> bool {
    if dal_hw_translate_gpio_ddc_offset_to_id(DDC_OFFSET_MAP.as_ptr(), DDC_OFFSET_MAP.len(), offset, en) {
        return true;
    }
    if dal_hw_translate_gpio_offset_to_id(GPIO_OFFSETS.as_ptr(), GPIO_OFFSETS.len(), offset, mask, id, en) {
        return true;
    }
    ASSERT_CRITICAL!(false);
    false
}

unsafe fn id_to_offset(id: enum_gpio_id, en: u32, info: *mut struct_gpio_pin_info) -> bool {
    if dal_hw_translate_id_to_offset(GPIO_PINS.as_ptr(), GPIO_PINS.len(), id, en, info) {
        return true;
    }
    ASSERT_CRITICAL!(false);
    false
}

/* function table */
static FUNCS: struct_hw_translate_funcs = struct_hw_translate_funcs {
    offset_to_id: Some(offset_to_id),
    id_to_offset: Some(id_to_offset),
};

/*
 * dal_hw_translate_dcn30_init
 *
 * @brief
 * Initialize Hw translate function pointers.
 *
 * @param
 * struct hw_translate *tr - [out] struct of function pointers
 */
pub unsafe fn dal_hw_translate_dcn30_init(tr: *mut struct_hw_translate) {
    (*tr).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
