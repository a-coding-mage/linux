Warning: truncated output (original token count: 15782)
Total output lines: 1707

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
 */


const MMHUB_NUM_INSTANCES: usize = 2;
const MMHUB_INSTANCE_REGISTER_OFFSET: u32 = 0x3000;

unsafe fn mmhub_v9_4_get_fb_location(struct amdgpu_device *adev)
{
	/* The base should be same b/t 2 mmhubs on Acrturus. Read one here. */
	u64 base = RREG32_SOC15(MMHUB, 0, mmVMSHAREDVC0_MC_VM_FB_LOCATION_BASE);
	u64 top = RREG32_SOC15(MMHUB, 0, mmVMSHAREDVC0_MC_VM_FB_LOCATION_TOP);

	base &= VMSHAREDVC0_MC_VM_FB_LOCATION_BASE__FB_BASE_MASK;
	base <<= 24;

	top &= VMSHAREDVC0_MC_VM_FB_LOCATION_TOP__FB_TOP_MASK;
	top <<= 24;

	adev.gmc.fb_start = base;
	adev.gmc.fb_end = top;

	return base;
}

unsafe fn mmhub_v9_4_setup_hubid_vm_pt_regs(struct amdgpu_device *adev, i32 hubid,
				u32 vmid, u64 value)
{
	struct amdgpu_vmhub *hub = &adev.vmhub[AMDGPU_MMHUB0(0)];

	WREG32_SOC15_OFFSET(MMHUB, 0,
			    mmVML2VC0_VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
			    hub.ctx…14782 tokens truncated…
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA1_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA2_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA3_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA4_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA5_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA6_ERR_STATUS), 0, 0, 0 },
	{ SOC15_REG_ENTRY(MMHUB, 0, mmMMEA7_ERR_STATUS), 0, 0, 0 },
};

unsafe fn mmhub_v9_4_query_ras_error_status(struct amdgpu_device *adev)
{
	i32 i;
	u32 reg_value;

	if (!amdgpu_ras_is_supported(adev, AMDGPU_RAS_BLOCK__MMHUB))
		return;

	for (i = 0; i < ARRAY_SIZE(mmhub_v9_4_err_status_regs); i++) {
		reg_value =
			RREG32(SOC15_REG_ENTRY_OFFSET(mmhub_v9_4_err_status_regs[i]));
		if (REG_GET_FIELD(reg_value, MMEA0_ERR_STATUS, SDP_RDRSP_STATUS) ||
		    REG_GET_FIELD(reg_value, MMEA0_ERR_STATUS, SDP_WRRSP_STATUS) ||
		    REG_GET_FIELD(reg_value, MMEA0_ERR_STATUS, SDP_RDRSP_DATAPARITY_ERROR)) {
			/* SDP read/write error/parity error in FUE_IS_FATAL mode
			 * can cause system fatal error in arcturas. Harvest the error
			 * status before GPU reset */
			dev_warn(adev.dev, "MMHUB EA err detected at instance: %d, status: 0x%x!\n",
					i, reg_value);
		}
	}
}

const amdgpu_ras_block_hw_ops mmhub_v9_4_ras_hw_ops = {
	.query_ras_error_count = mmhub_v9_4_query_ras_error_count,
	.reset_ras_error_count = mmhub_v9_4_reset_ras_error_count,
	.query_ras_error_status = mmhub_v9_4_query_ras_error_status,
};

struct amdgpu_mmhub_ras mmhub_v9_4_ras = {
	.ras_block = {
		.hw_ops = &mmhub_v9_4_ras_hw_ops,
	},
};

const amdgpu_mmhub_funcs mmhub_v9_4_funcs = {
	.get_fb_location = mmhub_v9_4_get_fb_location,
	.init = mmhub_v9_4_init,
	.gart_enable = mmhub_v9_4_gart_enable,
	.set_fault_enable_default = mmhub_v9_4_set_fault_enable_default,
	.gart_disable = mmhub_v9_4_gart_disable,
	.set_clockgating = mmhub_v9_4_set_clockgating,
	.get_clockgating = mmhub_v9_4_get_clockgating,
	.setup_vm_pt_regs = mmhub_v9_4_setup_vm_pt_regs,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
