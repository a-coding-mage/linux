/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding headers and other translation units.

#[allow(improper_ctypes)]
extern "C" {
    fn dcn35_calc_blocks_to_gate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    fn dcn35_calc_blocks_to_ungate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
}

pub unsafe fn dcn351_calc_blocks_to_gate(
    dc: *mut dc,
    context: *mut dc_state,
    update_state: *mut pg_block_update,
) {
    let mut i: i32;
    let mut j: i32;

    dcn35_calc_blocks_to_gate(dc, context, update_state);

    i = (*(*dc).res_pool).pipe_count - 1;
    while i >= 0 {
        if !(*update_state).pg_pipe_res_update[PG_HUBP][i as usize]
            && !(*update_state).pg_pipe_res_update[PG_DPP][i as usize]
        {
            j = i - 1;
            while j >= 0 {
                (*update_state).pg_pipe_res_update[PG_HUBP][j as usize] = false;
                (*update_state).pg_pipe_res_update[PG_DPP][j as usize] = false;
                j -= 1;
            }
            break;
        }
        i -= 1;
    }
}

pub unsafe fn dcn351_calc_blocks_to_ungate(
    dc: *mut dc,
    context: *mut dc_state,
    update_state: *mut pg_block_update,
) {
    let mut i: i32;
    let mut j: i32;

    dcn35_calc_blocks_to_ungate(dc, context, update_state);

    i = (*(*dc).res_pool).pipe_count - 1;
    while i >= 0 {
        if (*update_state).pg_pipe_res_update[PG_HUBP][i as usize]
            && (*update_state).pg_pipe_res_update[PG_DPP][i as usize]
        {
            j = i - 1;
            while j >= 0 {
                (*update_state).pg_pipe_res_update[PG_HUBP][j as usize] = true;
                (*update_state).pg_pipe_res_update[PG_DPP][j as usize] = true;
                j -= 1;
            }
            break;
        }
        i -= 1;
    }
}

/**
 * dcn351_hw_block_power_down() - power down sequence
 *
 * The following sequence describes the ON-OFF (ONO) for power down:
 *
 * ONO Region 11, DCPG 19: dsc3
 * ONO Region 10, DCPG 3: dchubp3, dpp3
 * ONO Region 9, DCPG 18: dsc2
 * ONO Region 8, DCPG 2: dchubp2, dpp2
 * ONO Region 7, DCPG 17: dsc1
 * ONO Region 6, DCPG 1: dchubp1, dpp1
 * ONO Region 5, DCPG 16: dsc0
 * ONO Region 4, DCPG 0: dchubp0, dpp0
 * ONO Region 3, DCPG 25: hpo - SKIPPED. Should be kept on
 * ONO Region 2, DCPG 24: mpc opp optc dwb
 * ONO Region 1, DCPG 23: dchubbub dchvm dchubbubmem - SKIPPED. PMFW will pwr dwn at IPS2 entry
 * ONO Region 0, DCPG 22: dccg dio dcio - SKIPPED. will be pwr dwn after lono timer is armed
 */
pub unsafe fn dcn351_hw_block_power_down(
    dc: *mut dc,
    update_state: *mut pg_block_update,
) {
    let mut i: i32 = 0;
    let pg_cntl = (*(*dc).res_pool).pg_cntl;

    if pg_cntl.is_null() || (*dc).debug.ignore_pg {
        return;
    }

    i = (*(*dc).res_pool).pipe_count - 1;
    while i >= 0 {
        if (*update_state).pg_pipe_res_update[PG_DSC][i as usize] {
            if let Some(f) = (*(*pg_cntl).funcs).dsc_pg_control {
                f(pg_cntl, i, false);
            }
        }
        if (*update_state).pg_pipe_res_update[PG_HUBP][i as usize]
            && (*update_state).pg_pipe_res_update[PG_DPP][i as usize]
        {
            if let Some(f) = (*(*pg_cntl).funcs).hubp_dpp_pg_control {
                f(pg_cntl, i, false);
            }
        }
        i -= 1;
    }

    // domain25 currently always on.
    // this will need all the clients to unregister optc interrupts, let dmubfw handle this
    if let Some(f) = (*(*pg_cntl).funcs).plane_otg_pg_control {
        f(pg_cntl, false);
    }
    // domain23 currently always on.
    // domain22 currently always on.
}

/**
 * dcn351_hw_block_power_up() - power up sequence
 *
 * The following sequence describes the ON-OFF (ONO) for power up:
 *
 * ONO Region 0, DCPG 22: dccg dio dcio - SKIPPED
 * ONO Region 1, DCPG 23: dchubbub dchvm dchubbubmem - SKIPPED. PMFW will power up at IPS2 exit
 * ONO Region 2, DCPG 24: mpc opp optc dwb
 * ONO Region 3, DCPG 25: hpo - SKIPPED
 * ONO Region 4, DCPG 0: dchubp0, dpp0
 * ONO Region 5, DCPG 16: dsc0
 * ONO Region 6, DCPG 1: dchubp1, dpp1
 * ONO Region 7, DCPG 17: dsc1
 * ONO Region 8, DCPG 2: dchubp2, dpp2
 * ONO Region 9, DCPG 18: dsc2
 * ONO Region 10, DCPG 3: dchubp3, dpp3
 * ONO Region 11, DCPG 19: dsc3
 */
pub unsafe fn dcn351_hw_block_power_up(
    dc: *mut dc,
    update_state: *mut pg_block_update,
) {
    let mut i: u32 = 0;
    let pg_cntl = (*(*dc).res_pool).pg_cntl;

    if pg_cntl.is_null() || (*dc).debug.ignore_pg {
        return;
    }

    // domain22 currently always on.
    // domain23 currently always on.
    // this will need all the clients to unregister optc interrupts, let dmubfw handle this
    if let Some(f) = (*(*pg_cntl).funcs).plane_otg_pg_control {
        f(pg_cntl, true);
    }
    // domain25 currently always on.

    while i < (*(*dc).res_pool).pipe_count as u32 {
        let p = i as usize;
        if (*update_state).pg_pipe_res_update[PG_HUBP][p]
            && (*update_state).pg_pipe_res_update[PG_DPP][p]
        {
            if let Some(f) = (*(*pg_cntl).funcs).hubp_dpp_pg_control {
                f(pg_cntl, i, true);
            }
        }
        if (*update_state).pg_pipe_res_update[PG_DSC][p] {
            if let Some(f) = (*(*pg_cntl).funcs).dsc_pg_control {
                f(pg_cntl, i, true);
            }
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
