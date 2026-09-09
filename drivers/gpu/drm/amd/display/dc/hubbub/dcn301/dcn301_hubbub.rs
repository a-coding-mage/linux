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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
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

// Dependencies supplied by the surrounding DCN implementation:
// dm_services.h, dcn301_hubbub.h, and reg_helper.h.

// C preprocessor aliases retained as Rust-access intent:
// REG(reg) => hubbub1->regs->reg
// DC_LOGGER => hubbub1->base.ctx->logger
// CTX => hubbub1->base.ctx
// FN(reg_name, field_name) =>
//     (hubbub1->shifts->field_name, hubbub1->masks->field_name)

static HUBBUB301_FUNCS: hubbub_funcs = hubbub_funcs {
    update_dchub: hubbub2_update_dchub,
    init_dchub_sys_ctx: hubbub21_init_dchub,
    init_vm_ctx: hubbub2_init_vm_ctx,
    dcc_support_swizzle: hubbub3_dcc_support_swizzle,
    dcc_support_pixel_format: hubbub2_dcc_support_pixel_format,
    get_dcc_compression_cap: hubbub3_get_dcc_compression_cap,
    wm_read_state: hubbub21_wm_read_state,
    get_dchub_ref_freq: hubbub2_get_dchub_ref_freq,
    program_watermarks: hubbub3_program_watermarks,
    allow_self_refresh_control: hubbub1_allow_self_refresh_control,
    is_allow_self_refresh_enabled: hubbub1_is_allow_self_refresh_enabled,
    verify_allow_pstate_change_high: hubbub1_verify_allow_pstate_change_high,
    force_wm_propagate_to_pipes: hubbub3_force_wm_propagate_to_pipes,
    force_pstate_change_control: hubbub3_force_pstate_change_control,
    init_watermarks: hubbub3_init_watermarks,
    hubbub_read_state: hubbub2_read_state,
};

pub unsafe fn hubbub301_construct(
    hubbub3: *mut dcn20_hubbub,
    ctx: *mut dc_context,
    hubbub_regs: *const dcn_hubbub_registers,
    hubbub_shift: *const dcn_hubbub_shift,
    hubbub_mask: *const dcn_hubbub_mask,
) {
    (*hubbub3).base.ctx = ctx;
    (*hubbub3).base.funcs = &HUBBUB301_FUNCS;
    (*hubbub3).regs = hubbub_regs;
    (*hubbub3).shifts = hubbub_shift;
    (*hubbub3).masks = hubbub_mask;

    (*hubbub3).debug_test_index_pstate = 0xB;
    (*hubbub3).detile_buf_size = 184 * 1024; /* 184KB for DCN3 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
