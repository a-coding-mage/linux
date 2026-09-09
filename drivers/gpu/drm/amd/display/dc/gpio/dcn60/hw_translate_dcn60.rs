// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C headers and register-list macro expansion are supplied by the surrounding
// translation unit.  The source selects DCN_BASE__INST0_SEG2 for this block.
const DCN_BASE__INST0_SEG2: u32 = 0x0000_34C0;

unsafe fn dcn60_offset_to_id(
    offset: u32,
    mask: u32,
    id: *mut gpio_id,
    en: *mut u32,
) -> bool {
    let _ = mask;
    match offset {
        DCN_BASE__INST0_SEG2 + HPD0_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_1;
            true
        }
        DCN_BASE__INST0_SEG2 + HPD1_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_2;
            true
        }
        DCN_BASE__INST0_SEG2 + HPD2_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_3;
            true
        }
        DCN_BASE__INST0_SEG2 + HPD3_DC_HPD_INT_STATUS => {
            *id = GPIO_ID_HPD;
            *en = GPIO_HPD_4;
            true
        }
        DCN_BASE__INST0_SEG2 + DC_I3C0_DC_I3CPAD_CONTROL0 => {
            *en = GPIO_DDC_LINE_DDC1;
            true
        }
        DCN_BASE__INST0_SEG2 + DC_I3C1_DC_I3CPAD_CONTROL0 => {
            *en = GPIO_DDC_LINE_DDC2;
            true
        }
        _ => {
            ASSERT_CRITICAL(false);
            false
        }
    }
}

unsafe fn dcn60_id_to_offset(id: gpio_id, en: u32, info: *mut gpio_pin_info) -> bool {
    let mut result = true;

    match id {
        GPIO_ID_DDC_DATA => match en {
            GPIO_DDC_LINE_DDC1 => (*info).offset = DCN_BASE__INST0_SEG2 + DC_I3C0_DC_I3CPAD_CONTROL0,
            GPIO_DDC_LINE_DDC2 => (*info).offset = DCN_BASE__INST0_SEG2 + DC_I3C1_DC_I3CPAD_CONTROL0,
            _ => { ASSERT_CRITICAL(false); result = false; }
        },
        GPIO_ID_DDC_CLOCK => match en {
            GPIO_DDC_LINE_DDC1 => (*info).offset = DCN_BASE__INST0_SEG2 + DC_I3C0_DC_I3CPAD_CONTROL0,
            GPIO_DDC_LINE_DDC2 => (*info).offset = DCN_BASE__INST0_SEG2 + DC_I3C1_DC_I3CPAD_CONTROL0,
            _ => { ASSERT_CRITICAL(false); result = false; }
        },
        GPIO_ID_HPD => match en {
            GPIO_HPD_1 => { (*info).offset = DCN_BASE__INST0_SEG2 + HPD0_DC_HPD_INT_STATUS; (*info).mask = HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_MASK; }
            GPIO_HPD_2 => { (*info).offset = DCN_BASE__INST0_SEG2 + HPD1_DC_HPD_INT_STATUS; (*info).mask = HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_MASK; }
            GPIO_HPD_3 => { (*info).offset = DCN_BASE__INST0_SEG2 + HPD2_DC_HPD_INT_STATUS; (*info).mask = HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_MASK; }
            GPIO_HPD_4 => { (*info).offset = DCN_BASE__INST0_SEG2 + HPD3_DC_HPD_INT_STATUS; (*info).mask = HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_MASK; }
            _ => { ASSERT_CRITICAL(false); result = false; }
        },
        _ => { ASSERT_CRITICAL(false); result = false; }
    }

    result
}

static FUNCS: hw_translate_funcs = hw_translate_funcs {
    offset_to_id: dcn60_offset_to_id,
    id_to_offset: dcn60_id_to_offset,
};

pub unsafe fn dal_hw_translate_dcn60_init(tr: *mut hw_translate) {
    (*tr).funcs = &FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
