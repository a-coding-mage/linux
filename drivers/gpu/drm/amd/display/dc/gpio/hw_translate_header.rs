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
 *
 */

#[repr(C)]
pub struct hw_translate_funcs {
    pub offset_to_id: Option<unsafe extern "C" fn(
        offset: u32,
        mask: u32,
        id: *mut gpio_id,
        en: *mut u32,
    ) -> bool>,
    pub id_to_offset: Option<unsafe extern "C" fn(
        id: gpio_id,
        en: u32,
        info: *mut gpio_pin_info,
    ) -> bool>,
}

#[repr(C)]
pub struct hw_translate {
    pub funcs: *const hw_translate_funcs,
}

extern "C" {
    pub fn dal_hw_translate_init(
        translate: *mut hw_translate,
        dce_version: dce_version,
        dce_environment: dce_environment,
    ) -> bool;

    pub fn dal_hw_translate_gpio_offset_to_id(
        table: *const gpio_id_offset_entry,
        table_size: u32,
        offset: u32,
        mask: u32,
        id: *mut gpio_id,
        en: *mut u32,
    ) -> bool;

    pub fn dal_hw_translate_gpio_ddc_offset_to_id(
        table: *const gpio_ddc_offset_entry,
        table_size: u32,
        offset: u32,
        en: *mut u32,
    ) -> bool;

    pub fn dal_hw_translate_id_to_offset(
        table: *const gpio_pin_entry,
        table_size: u32,
        id: gpio_id,
        en: u32,
        info: *mut gpio_pin_info,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
