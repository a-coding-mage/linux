// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C headers:
// hw_translate_dcn42b.h, dm_services.h, include/gpio_types.h,
// ../hw_translate.h, dcn/dcn_4_2_1_offset.h, dcn/dcn_4_2_1_sh_mask.h,
// dpcs/dpcs_4_0_1_offset.h, dpcs/dpcs_4_0_1_sh_mask.h

const DCN_BASE_INST0_SEG2: u32 = 0x000034C0;

// C preprocessor mapping:
// #define block HPD
// #define reg_num 0
// #define BASE_INNER(seg) DCN_BASE__INST0_SEG ## seg
// #define BASE(seg) BASE_INNER(seg)
// #define REG(reg_name) BASE(reg ## reg_name ## _BASE_IDX) + reg ## reg_name
// #define SF_HPD(reg_name, field_name, post_fix) \
//     .field_name = reg_name ## __ ## field_name ## post_fix

unsafe fn offset_to_id(
    offset: u32,
    mask: u32,
    id: *mut enum_gpio_id,
    en: *mut u32,
) -> bool {
    let _ = mask;
    match offset {
        REG_HPD0_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_1;
            true
        }
        REG_HPD1_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_2;
            true
        }
        REG_HPD2_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_3;
            true
        }
        /* Not in DNC42B
        REG_HPD3_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_4;
            true
        }
        REG_HPD4_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_5;
            true
        }
        REG_DC_GPIO_DDC1_A => { *en = GPIO_DDC_LINE_DDC1; true }
        REG_DC_GPIO_DDC2_A => { *en = GPIO_DDC_LINE_DDC2; true }
        REG_DC_GPIO_DDC3_A => { *en = GPIO_DDC_LINE_DDC3; true }
        REG_DC_GPIO_DDC4_A => { *en = GPIO_DDC_LINE_DDC4; true }
        REG_DC_GPIO_DDC5_A => { *en = GPIO_DDC_LINE_DDC5; true }
        */
        REG_DC_GPIO_DDCVGA_A => {
            *en = GPIO_DDC_LINE_DDC_VGA;
            true
        }
        _ => {
            // ASSERT_CRITICAL(false);
            false
        }
    }
}

unsafe fn id_to_offset(
    id: enum_gpio_id,
    en: u32,
    info: *mut gpio_pin_info,
) -> bool {
    let mut result = true;
    match id {
        GPIO_ID_DDC_DATA => {
            (*info).mask = DC_GPIO_DDCVGA_A__DC_GPIO_DDCVGADATA_A_MASK;
            match en {
                /* Not in DCN42B
                GPIO_DDC_LINE_DDC1 => { (*info).offset = REG_DC_GPIO_DDC1_A; }
                GPIO_DDC_LINE_DDC2 => { (*info).offset = REG_DC_GPIO_DDC2_A; }
                GPIO_DDC_LINE_DDC3 => { (*info).offset = REG_DC_GPIO_DDC3_A; }
                GPIO_DDC_LINE_DDC4 => { (*info).offset = REG_DC_GPIO_DDC4_A; }
                GPIO_DDC_LINE_DDC5 => { (*info).offset = REG_DC_GPIO_DDC5_A; }
                */
                GPIO_DDC_LINE_DDC_VGA => {
                    (*info).offset = REG_DC_GPIO_DDCVGA_A;
                }
                GPIO_DDC_LINE_I2C_PAD | _ => {
                    ASSERT_CRITICAL(false);
                    result = false;
                }
            }
        }
        GPIO_ID_DDC_CLOCK => {
            (*info).mask = DC_GPIO_DDCVGA_A__DC_GPIO_DDCVGACLK_A_MASK;
            match en {
                /* Not in DCN42B
                GPIO_DDC_LINE_DDC1 => { (*info).offset = REG_DC_GPIO_DDC1_A; }
                GPIO_DDC_LINE_DDC2 => { (*info).offset = REG_DC_GPIO_DDC2_A; }
                GPIO_DDC_LINE_DDC3 => { (*info).offset = REG_DC_GPIO_DDC3_A; }
                GPIO_DDC_LINE_DDC4 => { (*info).offset = REG_DC_GPIO_DDC4_A; }
                GPIO_DDC_LINE_DDC5 => { (*info).offset = REG_DC_GPIO_DDC5_A; }
                */
                GPIO_DDC_LINE_DDC_VGA => {
                    (*info).offset = REG_DC_GPIO_DDCVGA_A;
                }
                GPIO_DDC_LINE_I2C_PAD | _ => {
                    ASSERT_CRITICAL(false);
                    result = false;
                }
            }
        }
        GPIO_ID_SYNC | GPIO_ID_VIP_PAD | _ => {
            ASSERT_CRITICAL(false);
            result = false;
        }
    }

    if result {
        (*info).offset_y = (*info).offset + 2;
        (*info).offset_en = (*info).offset + 1;
        (*info).offset_mask = (*info).offset - 1;
        (*info).mask_y = (*info).mask;
        (*info).mask_en = (*info).mask;
        (*info).mask_mask = (*info).mask;
    }
    result
}

/* function table */
static FUNCS: hw_translate_funcs = hw_translate_funcs {
    offset_to_id: Some(offset_to_id),
    id_to_offset: Some(id_to_offset),
};

/*
 * dal_hw_translate_dcn42b_init
 *
 * @brief
 * Initialize Hw translate function pointers.
 *
 * @param
 * struct hw_translate *tr - [out] struct of function pointers
 *
 */
unsafe fn dal_hw_translate_dcn42b_init(tr: *mut hw_translate) {
    (*tr).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
