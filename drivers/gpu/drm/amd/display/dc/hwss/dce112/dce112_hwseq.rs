/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// External declarations and register definitions are supplied by the
// corresponding translated dependency files.

#[repr(C)]
struct dce112_hw_seq_reg_offsets {
    crtc: u32,
}

static reg_offsets: [dce112_hw_seq_reg_offsets; 6] = [
    dce112_hw_seq_reg_offsets { crtc: mmCRTC0_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce112_hw_seq_reg_offsets { crtc: mmCRTC1_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce112_hw_seq_reg_offsets { crtc: mmCRTC2_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce112_hw_seq_reg_offsets { crtc: mmCRTC3_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce112_hw_seq_reg_offsets { crtc: mmCRTC4_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce112_hw_seq_reg_offsets { crtc: mmCRTC5_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
];

#[inline]
unsafe fn hw_reg_crtc(reg: u32, id: usize) -> u32 {
    reg.wrapping_add(reg_offsets[id].crtc)
}

unsafe fn dce112_init_pte(ctx: *mut dc_context) {
    let addr: u32;
    let mut value: u32 = 0;
    let mut chunk_int: u32 = 0;
    let mut chunk_mul: u32 = 0;

    addr = mmDVMM_PTE_REQ;
    value = dm_read_reg(ctx, addr);

    chunk_int = get_reg_field_value(value, DVMM_PTE_REQ, HFLIP_PTEREQ_PER_CHUNK_INT);
    chunk_mul = get_reg_field_value(value, DVMM_PTE_REQ, HFLIP_PTEREQ_PER_CHUNK_MULTIPLIER);

    if chunk_int != 0x4 || chunk_mul != 0x4 {
        set_reg_field_value(&mut value, 255, DVMM_PTE_REQ, MAX_PTEREQ_TO_ISSUE);
        set_reg_field_value(&mut value, 4, DVMM_PTE_REQ, HFLIP_PTEREQ_PER_CHUNK_INT);
        set_reg_field_value(&mut value, 4, DVMM_PTE_REQ, HFLIP_PTEREQ_PER_CHUNK_MULTIPLIER);
        dm_write_reg(ctx, addr, value);
    }
}

unsafe fn dce112_enable_display_power_gating(
    dc: *mut dc,
    controller_id: u8,
    dcb: *mut dc_bios,
    power_gating: pipe_gating_control,
) -> bool {
    let mut bp_result = BP_RESULT_OK;
    let cntl;
    let ctx = (*dc).ctx;

    if power_gating == PIPE_GATING_CONTROL_INIT {
        cntl = ASIC_PIPE_INIT;
    } else if power_gating == PIPE_GATING_CONTROL_ENABLE {
        cntl = ASIC_PIPE_ENABLE;
    } else {
        cntl = ASIC_PIPE_DISABLE;
    }

    if power_gating != PIPE_GATING_CONTROL_INIT || controller_id == 0 {
        bp_result = ((*(*dcb).funcs).enable_disp_power_gating)(
            dcb,
            controller_id.wrapping_add(1),
            cntl,
        );

        /* Revert MASTER_UPDATE_MODE to 0 because bios sets it 2
         * by default when command table is called
         */
        dm_write_reg(ctx, hw_reg_crtc(mmCRTC_MASTER_UPDATE_MODE, controller_id as usize), 0);
    }

    if power_gating != PIPE_GATING_CONTROL_ENABLE {
        dce112_init_pte(ctx);
    }

    if bp_result == BP_RESULT_OK { true } else { false }
}

unsafe fn dce112_hw_sequencer_construct(dc: *mut dc) {
    /* All registers used by dce11.2 match those in dce11 in offset and
     * structure
     */
    dce110_hw_sequencer_construct(dc);
    (*(*dc).hwseq).funcs.enable_display_power_gating =
        Some(dce112_enable_display_power_gating);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
