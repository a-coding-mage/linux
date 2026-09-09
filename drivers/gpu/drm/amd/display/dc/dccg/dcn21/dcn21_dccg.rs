/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

unsafe fn to_dcn_dccg<'a>(dccg: *mut dccg) -> *mut dcn_dccg {
    container_of!(dccg, dcn_dccg, base)
}

unsafe fn dccg21_update_dpp_dto(dccg: *mut dccg, dpp_inst: i32, req_dppclk: i32) {
    let dccg_dcn = &mut *to_dcn_dccg(dccg);

    if (*dccg).ref_dppclk != 0 {
        let ref_dppclk = (*dccg).ref_dppclk;
        let modulo = ref_dppclk / 10000;
        let phase;

        if req_dppclk != 0 {
            /*
             * program DPP DTO phase and modulo as below
             * phase = ceiling(dpp_pipe_clk_mhz / 10)
             * module = trunc(dpp_global_clk_mhz / 10)
             *
             * storing frequencies in registers allow dmcub fw
             * to run time lower clocks when possible for power saving
             *
             * ceiling phase and truncate modulo guarentees the divided
             * down per pipe dpp clock has high enough frequency
             */
            phase = (req_dppclk + 9999) / 10000;

            if phase > modulo {
                /* phase > modulo result in screen corruption
                 * ie phase = 30, mod = 29 for 4k@60 HDMI
                 * in these case we don't want pipe clock to be divided
                 */
                phase = modulo;
            }
        } else {
            /*
             *  set phase to 10 if dpp isn't used to
             *  prevent hard hang if access dpp register
             *  on unused pipe
             *
             *  DTO should be on to divide down un-used
             *  pipe clock for power saving
             */
            phase = 10;
        }

        REG_SET_2!(dccg_dcn, DPPCLK_DTO_PARAM[dpp_inst], 0,
            DPPCLK0_DTO_PHASE, phase,
            DPPCLK0_DTO_MODULO, modulo);
        REG_UPDATE!(dccg_dcn, DPPCLK_DTO_CTRL,
            DPPCLK_DTO_ENABLE[dpp_inst], 1);
    }

    (*dccg).pipe_dppclk_khz[dpp_inst as usize] = req_dppclk;
}

/*
 * On DCN21 S0i3 resume, BIOS programs MICROSECOND_TIME_BASE_DIV to
 * 0x00120464 as a marker that golden init has already been done.
 * dcn21_s0i3_golden_init_wa() reads this marker later in bios_golden_init()
 * to decide whether to skip golden init.
 *
 * dccg2_init() unconditionally overwrites MICROSECOND_TIME_BASE_DIV to
 * 0x00120264, destroying the marker before it can be read.
 *
 * Guard the call: if the S0i3 marker is present, skip init so the
 * WA can function correctly. bios_golden_init() will handle init in that case.
 *
 * DCN21 uses 48MHz refclk, not 100MHz, so we must explicitly set the correct
 * values (48MHz is taken from rn_clk_mgr_construct()).
 */
unsafe fn dccg21_init(dccg: *mut dccg) {
    let dccg_dcn = &mut *to_dcn_dccg(dccg);

    if dccg2_is_s0i3_golden_init_wa_done!(dccg) {
        return;
    }

    /* 48MHz refclk from rn_clk_mgr_construct() */
    REG_WRITE!(dccg_dcn, MICROSECOND_TIME_BASE_DIV, 0x00120230);
    REG_WRITE!(dccg_dcn, MILLISECOND_TIME_BASE_DIV, 0x0010bb80);
    REG_WRITE!(dccg_dcn, DISPCLK_FREQ_CHANGE_CNTL, 0x0e01003c);

    if REG!(dccg_dcn, REFCLK_CNTL) != 0 {
        REG_WRITE!(dccg_dcn, REFCLK_CNTL, 0);
    }
}

static dccg21_funcs: dccg_funcs = dccg_funcs {
    update_dpp_dto: Some(dccg21_update_dpp_dto),
    get_dccg_ref_freq: Some(dccg2_get_dccg_ref_freq),
    set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en),
    otg_add_pixel: Some(dccg2_otg_add_pixel),
    otg_drop_pixel: Some(dccg2_otg_drop_pixel),
    dccg_init: Some(dccg21_init),
    refclk_setup: Some(dccg2_refclk_setup),
    allow_clock_gating: Some(dccg2_allow_clock_gating),
    enable_memory_low_power: Some(dccg2_enable_memory_low_power),
    is_s0i3_golden_init_wa_done: Some(dccg2_is_s0i3_golden_init_wa_done),
};

unsafe fn dccg21_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn = kzalloc_obj!(dcn_dccg);
    if dccg_dcn.is_null() {
        BREAK_TO_DEBUGGER!();
        return core::ptr::null_mut();
    }

    let base = &mut (*dccg_dcn).base;
    (*base).ctx = ctx;
    (*base).funcs = &dccg21_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;

    base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
