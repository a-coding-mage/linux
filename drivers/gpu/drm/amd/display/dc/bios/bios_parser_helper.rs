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

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, atom.h, bios_parser_types.h, bios_parser_helper.h,
// command_table_helper.h, command_table.h, bios_parser_types_internal.h,
// and reg_helper.h.

pub unsafe fn bios_get_image(
    bp: *mut dc_bios,
    offset: u32,
    size: u32,
) -> *mut u8 {
    if (*bp).bios.is_null() {
        return core::ptr::null_mut();
    }

    if offset > (*bp).bios_size || size > (*bp).bios_size - offset {
        return core::ptr::null_mut();
    }

    (*bp).bios.add(offset as usize)
}

pub unsafe fn bios_is_accelerated_mode(bios: *mut dc_bios) -> bool {
    let mut acc_mode: u32 = 0;
    REG_GET!(bios, BIOS_SCRATCH_6, S6_ACC_MODE, &mut acc_mode);
    acc_mode == 1
}

pub unsafe fn bios_set_scratch_acc_mode_change(bios: *mut dc_bios, state: u32) {
    REG_UPDATE!(bios, BIOS_SCRATCH_6, S6_ACC_MODE, state);
}

pub unsafe fn bios_set_scratch_critical_state(bios: *mut dc_bios, state: bool) {
    let critial_state: u32 = if state { 1 } else { 0 };
    REG_UPDATE!(bios, BIOS_SCRATCH_6, S6_CRITICAL_STATE, critial_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
