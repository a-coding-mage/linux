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

// C dependencies supplied by the surrounding translation unit.

#[cfg(feature = "CONFIG_DRM_AMD_DC_SI")]
unsafe extern "C" {
    fn dal_hw_factory_dce60_init(factory: *mut hw_factory);
}
unsafe extern "C" {
    fn dal_hw_factory_dce80_init(factory: *mut hw_factory);
    fn dal_hw_factory_dce110_init(factory: *mut hw_factory);
    fn dal_hw_factory_dce120_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn10_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn20_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn21_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn30_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn315_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn32_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn401_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn42_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn42b_init(factory: *mut hw_factory);
    fn dal_hw_factory_dcn60_init(factory: *mut hw_factory);
    fn ASSERT_CRITICAL(condition: bool);
}

// These types and enum constants are declared by the translated headers.
#[allow(non_camel_case_types)]
pub type hw_factory = crate::hw_factory;
use crate::{dce_environment, dce_version};

pub unsafe fn dal_hw_factory_init(
    factory: *mut hw_factory,
    dce_version: dce_version,
    dce_environment: dce_environment,
) -> bool {
    let _ = dce_environment;

    match dce_version {
        #[cfg(feature = "CONFIG_DRM_AMD_DC_SI")]
        crate::DCE_VERSION_6_0
        | crate::DCE_VERSION_6_1
        | crate::DCE_VERSION_6_4 => {
            dal_hw_factory_dce60_init(factory);
            true
        }
        crate::DCE_VERSION_8_0
        | crate::DCE_VERSION_8_1
        | crate::DCE_VERSION_8_3 => {
            dal_hw_factory_dce80_init(factory);
            true
        }
        crate::DCE_VERSION_10_0 => {
            dal_hw_factory_dce110_init(factory);
            true
        }
        crate::DCE_VERSION_11_0
        | crate::DCE_VERSION_11_2
        | crate::DCE_VERSION_11_22 => {
            dal_hw_factory_dce110_init(factory);
            true
        }
        crate::DCE_VERSION_12_0 | crate::DCE_VERSION_12_1 => {
            dal_hw_factory_dce120_init(factory);
            true
        }
        crate::DCN_VERSION_1_0 | crate::DCN_VERSION_1_01 => {
            dal_hw_factory_dcn10_init(factory);
            true
        }
        crate::DCN_VERSION_2_0 => {
            dal_hw_factory_dcn20_init(factory);
            true
        }
        crate::DCN_VERSION_2_01 | crate::DCN_VERSION_2_1 => {
            dal_hw_factory_dcn21_init(factory);
            true
        }
        crate::DCN_VERSION_3_0
        | crate::DCN_VERSION_3_01
        | crate::DCN_VERSION_3_02
        | crate::DCN_VERSION_3_03
        | crate::DCN_VERSION_3_1
        | crate::DCN_VERSION_3_14
        | crate::DCN_VERSION_3_16 => {
            dal_hw_factory_dcn30_init(factory);
            true
        }
        crate::DCN_VERSION_3_15 => {
            dal_hw_factory_dcn315_init(factory);
            true
        }
        crate::DCN_VERSION_3_2
        | crate::DCN_VERSION_3_21
        | crate::DCN_VERSION_3_5
        | crate::DCN_VERSION_3_51
        | crate::DCN_VERSION_3_6 => {
            dal_hw_factory_dcn32_init(factory);
            true
        }
        crate::DCN_VERSION_4_01 => {
            dal_hw_factory_dcn401_init(factory);
            true
        }
        crate::DCN_VERSION_4_2 => {
            dal_hw_factory_dcn42_init(factory);
            true
        }
        crate::DCN_VERSION_4_2B => {
            dal_hw_factory_dcn42b_init(factory);
            true
        }
        crate::DCN_VERSION_6_0 => {
            dal_hw_factory_dcn60_init(factory);
            true
        }
        _ => {
            ASSERT_CRITICAL(false);
            false
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
