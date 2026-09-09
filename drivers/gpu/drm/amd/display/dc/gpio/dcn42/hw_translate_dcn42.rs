// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translation unit.

pub const DCN_BASE__INST0_SEG2: u32 = 0x0000_34C0;

/* Register-list expansion constants from the C implementation. */
pub const BLOCK_HPD: u32 = 0;
pub const REG_NUM: u32 = 0;

static GPIO_OFFSETS: &[gpio_id_offset_entry] = &[
    GPIO_ENTRY!(HPD0_DC_HPD_INT_STATUS, GPIO_ID_HPD, GPIO_HPD_1),
    GPIO_ENTRY!(HPD1_DC_HPD_INT_STATUS, GPIO_ID_HPD, GPIO_HPD_2),
    GPIO_ENTRY!(HPD2_DC_HPD_INT_STATUS, GPIO_ID_HPD, GPIO_HPD_3),
    GPIO_ENTRY!(HPD3_DC_HPD_INT_STATUS, GPIO_ID_HPD, GPIO_HPD_4),
    GPIO_ENTRY!(HPD4_DC_HPD_INT_STATUS, GPIO_ID_HPD, GPIO_HPD_5),
];

static DDC_OFFSET_MAP: &[gpio_ddc_offset_entry] = &[
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDC1_A), line: GPIO_DDC_LINE_DDC1 },
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDC2_A), line: GPIO_DDC_LINE_DDC2 },
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDC3_A), line: GPIO_DDC_LINE_DDC3 },
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDC4_A), line: GPIO_DDC_LINE_DDC4 },
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDC5_A), line: GPIO_DDC_LINE_DDC5 },
    gpio_ddc_offset_entry { offset: REG!(DC_GPIO_DDCVGA_A), line: GPIO_DDC_LINE_DDC_VGA },
];

static GPIO_PINS: &[gpio_pin_entry] = &[
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC1, DC_GPIO_DDC1_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC2, DC_GPIO_DDC2_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC3, DC_GPIO_DDC3_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC4, DC_GPIO_DDC4_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC5, DC_GPIO_DDC5_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_DATA, GPIO_DDC_LINE_DDC_VGA, DC_GPIO_DDCVGA_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1DATA_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC1, DC_GPIO_DDC1_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC2, DC_GPIO_DDC2_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC3, DC_GPIO_DDC3_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC4, DC_GPIO_DDC4_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC5, DC_GPIO_DDC5_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
    GPIO_PIN_ENTRY!(GPIO_ID_DDC_CLOCK, GPIO_DDC_LINE_DDC_VGA, DC_GPIO_DDCVGA_A, DC_GPIO_DDC1_A__DC_GPIO_DDC1CLK_A_MASK),
];

unsafe fn offset_to_id(offset: u32, mask: u32, id: *mut gpio_id, en: *mut u32) -> bool {
    if dal_hw_translate_gpio_ddc_offset_to_id(DDC_OFFSET_MAP, offset, en) {
        return true;
    }
    if dal_hw_translate_gpio_offset_to_id(GPIO_OFFSETS, offset, mask, id, en) {
        return true;
    }
    ASSERT_CRITICAL!(false);
    false
}

unsafe fn id_to_offset(id: gpio_id, en: u32, info: *mut gpio_pin_info) -> bool {
    if dal_hw_translate_id_to_offset(GPIO_PINS, id, en, info) {
        return true;
    }
    ASSERT_CRITICAL!(false);
    false
}

static FUNCS: hw_translate_funcs = hw_translate_funcs {
    offset_to_id: Some(offset_to_id),
    id_to_offset: Some(id_to_offset),
};

/// Initialize Hw translate function pointers.
pub unsafe fn dal_hw_translate_dcn42_init(tr: *mut hw_translate) {
    (*tr).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
