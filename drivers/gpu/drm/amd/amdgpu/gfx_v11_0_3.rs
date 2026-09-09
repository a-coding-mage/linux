/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// amdgpu.h, soc21.h, gc/gc_11_0_3_offset.h, gc/gc_11_0_3_sh_mask.h,
// ivsrcid/gfx/irqsrcs_gfx_11_0_0.h, soc15.h, soc15d.h, and gfx_v11_0.h.

unsafe fn gfx_v11_0_3_rlc_gc_fed_irq(
    adev: *mut amdgpu_device,
    source: *mut amdgpu_irq_src,
    entry: *mut amdgpu_iv_entry,
) -> i32 {
    let mut rlc_status0: u32 = 0;
    let mut rlc_status1: u32 = 0;
    let mut ras_if: *mut ras_common_if = core::ptr::null_mut();
    let mut ih_data = ras_dispatch_if {
        entry,
        ..core::mem::zeroed()
    };

    rlc_status0 = RREG32!(SOC15_REG_OFFSET!(GC, 0, regRLC_RLCS_FED_STATUS_0));
    rlc_status1 = RREG32!(SOC15_REG_OFFSET!(GC, 0, regRLC_RLCS_FED_STATUS_1));

    if rlc_status0 == 0 && rlc_status1 == 0 {
        dev_warn!((*adev).dev, "RLC_GC_FED irq is generated, but rlc_status0 and rlc_status1 are empty!\n");
        return 0;
    }

    // Use RLC_RLCS_FED_STATUS_0/1 to distinguish FED error block.
    if REG_GET_FIELD!(rlc_status0, RLC_RLCS_FED_STATUS_0, SDMA0_FED_ERR) != 0
        || REG_GET_FIELD!(rlc_status0, RLC_RLCS_FED_STATUS_0, SDMA1_FED_ERR) != 0
    {
        ras_if = (*adev).sdma.ras_if;
    } else {
        ras_if = (*adev).gfx.ras_if;
    }

    if ras_if.is_null() {
        dev_err!((*adev).dev, "Gfx or sdma ras block not initialized, rlc_status0:0x{:x}.\n", rlc_status0);
        return -EINVAL;
    }

    dev_warn!((*adev).dev, "RLC {} FED IRQ\n", (*ras_if).name);

    if !amdgpu_sriov_vf(adev) {
        ih_data.head = *ras_if;
        amdgpu_ras_interrupt_dispatch(adev, &mut ih_data);
    } else {
        if !(*adev).virt.ops.is_null()
            && (*(*adev).virt.ops).ras_poison_handler.is_some()
        {
            ((*(*adev).virt.ops).ras_poison_handler.unwrap())(adev, (*ras_if).block);
        } else {
            dev_warn!((*adev).dev,
                "No ras_poison_handler interface in SRIOV for {}!\n", (*ras_if).name);
        }
    }

    0
}

unsafe fn gfx_v11_0_3_poison_consumption_handler(
    adev: *mut amdgpu_device,
    entry: *mut amdgpu_iv_entry,
) -> i32 {
    // Workaround: when vmid and pasid are both zero, trigger gpu reset in KGD.
    if !entry.is_null()
        && (*entry).client_id == SOC21_IH_CLIENTID_GFX
        && (*entry).src_id == GFX_11_0_0__SRCID__RLC_GC_FED_INTERRUPT
        && (*entry).vmid == 0
        && (*entry).pasid == 0
    {
        let con = amdgpu_ras_get_context(adev);
        let mut rlc_status0: u32 = 0;

        rlc_status0 = RREG32_SOC15!(GC, 0, regRLC_RLCS_FED_STATUS_0);

        if REG_GET_FIELD!(rlc_status0, RLC_RLCS_FED_STATUS_0, SDMA0_FED_ERR) != 0
            || REG_GET_FIELD!(rlc_status0, RLC_RLCS_FED_STATUS_0, SDMA1_FED_ERR) != 0
        {
            let ras = amdgpu_ras_get_context(adev);
            (*ras).gpu_reset_flags |= AMDGPU_RAS_GPU_RESET_MODE2_RESET;
        }

        if !con.is_null() && !amdgpu_ras_is_rma(adev) {
            amdgpu_ras_reset_gpu(adev);
        }
    }

    0
}

#[no_mangle]
pub static mut gfx_v11_0_3_ras: amdgpu_gfx_ras = amdgpu_gfx_ras {
    rlc_gc_fed_irq: Some(gfx_v11_0_3_rlc_gc_fed_irq),
    poison_consumption_handler: Some(gfx_v11_0_3_poison_consumption_handler),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
