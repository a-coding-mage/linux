/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * Authors: Xiangliang.Yu@amd.com
 */





















/* VI golden setting */
static xgpu_fiji_mgcg_cgcg_init: &[u32] = &[
	mmRLC_CGTT_MGCG_OVERRIDE, 0xffffffff, 0xffffffff,
	mmGRBM_GFX_INDEX, 0xffffffff, 0xe0000000,
	mmCB_CGTT_SCLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_BCI_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_CP_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_CPC_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_CPF_CLK_CTRL, 0xffffffff, 0x40000100,
	mmCGTT_DRM_CLK_CTRL0, 0xffffffff, 0x00600100,
	mmCGTT_GDS_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_IA_CLK_CTRL, 0xffffffff, 0x06000100,
	mmCGTT_PA_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_WD_CLK_CTRL, 0xffffffff, 0x06000100,
	mmCGTT_PC_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_RLC_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_SC_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_SPI_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_SQ_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_SQG_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL0, 0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL1, 0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL2, 0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL3, 0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL4, 0xffffffff, 0x00000100,
	mmCGTT_TCI_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_TCP_CLK_CTRL, 0xffffffff, 0x00000100,
	mmCGTT_VGT_CLK_CTRL, 0xffffffff, 0x06000100,
	mmDB_CGTT_CLK_CTRL_0, 0xffffffff, 0x00000100,
	mmTA_CGTT_CTRL, 0xffffffff, 0x00000100,
	mmTCA_CGTT_SCLK_CTRL, 0xffffffff, 0x00000100,
	mmTCC_CGTT_SCLK_CTRL, 0xffffffff, 0x00000100,
	mmTD_CGTT_CTRL, 0xffffffff, 0x00000100,
	mmGRBM_GFX_INDEX, 0xffffffff, 0xe0000000,
	mmCGTS_SM_CTRL_REG, 0xffffffff, 0x96e00200,
	mmCP_RB_WPTR_POLL_CNTL, 0xffffffff, 0x00900100,
	mmRLC_CGCG_CGLS_CTRL, 0xffffffff, 0x0020003c,
	mmPCIE_INDEX, 0xffffffff, 0x0140001c,
	mmPCIE_DATA, 0x000f0000, 0x00000000,
	mmSMC_IND_INDEX_4, 0xffffffff, 0xC060000C,
	mmSMC_IND_DATA_4, 0xc0000fff, 0x00000100,
	mmXDMA_CLOCK_GATING_CNTL, 0xffffffff, 0x00000100,
	mmXDMA_MEM_POWER_CNTL, 0x00000101, 0x00000000,
	mmMC_MEM_POWER_LS, 0xffffffff, 0x00000104,
	mmCGTT_DRM_CLK_CTRL0, 0xff000fff, 0x00000100,
	mmHDP_XDP_CGTT_BLK_CTRL, 0xc0000fff, 0x00000104,
	mmCP_MEM_SLP_CNTL, 0x00000001, 0x00000001,
	mmSDMA0_CLK_CTRL, 0xff000ff0, 0x00000100,
	mmSDMA1_CLK_CTRL, 0xff000ff0, 0x00000100,
};

static xgpu_fiji_golden_settings_a10: &[u32] = &[
	mmCB_HW_CONTROL_3, 0x000001ff, 0x00000040,
	mmDB_DEBUG2, 0xf00fffff, 0x00000400,
	mmDCI_CLK_CNTL, 0x00000080, 0x00000000,
	mmFBC_DEBUG_COMP, 0x000000f0, 0x00000070,
	mmFBC_MISC, 0x1f311fff, 0x12300000,
	mmHDMI_CONTROL, 0x31000111, 0x00000011,
	mmPA_SC_ENHANCE, 0xffffffff, 0x20000001,
	mmPA_SC_LINE_STIPPLE_STATE, 0x0000ff0f, 0x00000000,
	mmSDMA0_CHICKEN_BITS, 0xfc910007, 0x00810007,
	mmSDMA0_GFX_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA0_RLC0_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA0_RLC1_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_CHICKEN_BITS, 0xfc910007, 0x00810007,
	mmSDMA1_GFX_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_RLC0_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_RLC1_IB_CNTL, 0x800f0111, 0x00000100,
	mmSQ_RANDOM_WAVE_PRI, 0x001fffff, 0x000006fd,
	mmTA_CNTL_AUX, 0x000f000f, 0x000b0000,
	mmTCC_EXE_DISABLE, 0x00000002, 0x00000002,
	mmTCP_ADDR_CONFIG, 0x000003ff, 0x000000ff,
	mmVGT_RESET_DEBUG, 0x00000004, 0x00000004,
	mmVM_PRT_APERTURE0_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE1_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE2_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE3_LOW_ADDR, 0x0fffffff, 0x0fffffff,
};

static xgpu_fiji_golden_common_all: &[u32] = &[
	mmGRBM_GFX_INDEX, 0xffffffff, 0xe0000000,
	mmPA_SC_RASTER_CONFIG, 0xffffffff, 0x3a00161a,
	mmPA_SC_RASTER_CONFIG_1, 0xffffffff, 0x0000002e,
	mmGB_ADDR_CONFIG, 0xffffffff, 0x22011003,
	mmSPI_RESOURCE_RESERVE_CU_0, 0xffffffff, 0x00000800,
	mmSPI_RESOURCE_RESERVE_CU_1, 0xffffffff, 0x00000800,
	mmSPI_RESOURCE_RESERVE_EN_CU_0, 0xffffffff, 0x00007FBF,
	mmSPI_RESOURCE_RESERVE_EN_CU_1, 0xffffffff, 0x00007FAF,
	mmGRBM_GFX_INDEX, 0xffffffff, 0xe0000000,
	mmSPI_CONFIG_CNTL_1, 0x0000000f, 0x00000009,
};

static xgpu_tonga_mgcg_cgcg_init: &[u32] = &[
	mmRLC_CGTT_MGCG_OVERRIDE,   0xffffffff, 0xffffffff,
	mmGRBM_GFX_INDEX,           0xffffffff, 0xe0000000,
	mmCB_CGTT_SCLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_BCI_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_CP_CLK_CTRL,         0xffffffff, 0x00000100,
	mmCGTT_CPC_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_CPF_CLK_CTRL,        0xffffffff, 0x40000100,
	mmCGTT_DRM_CLK_CTRL0,       0xffffffff, 0x00600100,
	mmCGTT_GDS_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_IA_CLK_CTRL,         0xffffffff, 0x06000100,
	mmCGTT_PA_CLK_CTRL,         0xffffffff, 0x00000100,
	mmCGTT_WD_CLK_CTRL,         0xffffffff, 0x06000100,
	mmCGTT_PC_CLK_CTRL,         0xffffffff, 0x00000100,
	mmCGTT_RLC_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_SC_CLK_CTRL,         0xffffffff, 0x00000100,
	mmCGTT_SPI_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_SQ_CLK_CTRL,         0xffffffff, 0x00000100,
	mmCGTT_SQG_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL0,        0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL1,        0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL2,        0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL3,        0xffffffff, 0x00000100,
	mmCGTT_SX_CLK_CTRL4,        0xffffffff, 0x00000100,
	mmCGTT_TCI_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_TCP_CLK_CTRL,        0xffffffff, 0x00000100,
	mmCGTT_VGT_CLK_CTRL,        0xffffffff, 0x06000100,
	mmDB_CGTT_CLK_CTRL_0,       0xffffffff, 0x00000100,
	mmTA_CGTT_CTRL,             0xffffffff, 0x00000100,
	mmTCA_CGTT_SCLK_CTRL,       0xffffffff, 0x00000100,
	mmTCC_CGTT_SCLK_CTRL,       0xffffffff, 0x00000100,
	mmTD_CGTT_CTRL,             0xffffffff, 0x00000100,
	mmGRBM_GFX_INDEX,           0xffffffff, 0xe0000000,
	mmCGTS_CU0_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU0_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU0_TA_SQC_CTRL_REG, 0xffffffff, 0x00040007,
	mmCGTS_CU0_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU0_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU1_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU1_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU1_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU1_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU1_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU2_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU2_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU2_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU2_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU2_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU3_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU3_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU3_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU3_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU3_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU4_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU4_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU4_TA_SQC_CTRL_REG, 0xffffffff, 0x00040007,
	mmCGTS_CU4_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU4_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU5_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU5_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU5_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU5_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU5_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU6_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU6_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU6_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU6_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU6_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_CU7_SP0_CTRL_REG,    0xffffffff, 0x00010000,
	mmCGTS_CU7_LDS_SQ_CTRL_REG, 0xffffffff, 0x00030002,
	mmCGTS_CU7_TA_CTRL_REG,     0xffffffff, 0x00040007,
	mmCGTS_CU7_SP1_CTRL_REG,    0xffffffff, 0x00060005,
	mmCGTS_CU7_TD_TCP_CTRL_REG, 0xffffffff, 0x00090008,
	mmCGTS_SM_CTRL_REG,         0xffffffff, 0x96e00200,
	mmCP_RB_WPTR_POLL_CNTL,     0xffffffff, 0x00900100,
	mmRLC_CGCG_CGLS_CTRL,       0xffffffff, 0x0020003c,
	mmPCIE_INDEX,               0xffffffff, 0x0140001c,
	mmPCIE_DATA,                0x000f0000, 0x00000000,
	mmSMC_IND_INDEX_4,          0xffffffff, 0xC060000C,
	mmSMC_IND_DATA_4,           0xc0000fff, 0x00000100,
	mmXDMA_CLOCK_GATING_CNTL,   0xffffffff, 0x00000100,
	mmXDMA_MEM_POWER_CNTL,      0x00000101, 0x00000000,
	mmMC_MEM_POWER_LS,          0xffffffff, 0x00000104,
	mmCGTT_DRM_CLK_CTRL0,       0xff000fff, 0x00000100,
	mmHDP_XDP_CGTT_BLK_CTRL,    0xc0000fff, 0x00000104,
	mmCP_MEM_SLP_CNTL,          0x00000001, 0x00000001,
	mmSDMA0_CLK_CTRL,           0xff000ff0, 0x00000100,
	mmSDMA1_CLK_CTRL,           0xff000ff0, 0x00000100,
};

static xgpu_tonga_golden_settings_a11: &[u32] = &[
	mmCB_HW_CONTROL, 0xfffdf3cf, 0x00007208,
	mmCB_HW_CONTROL_3, 0x00000040, 0x00000040,
	mmDB_DEBUG2, 0xf00fffff, 0x00000400,
	mmDCI_CLK_CNTL, 0x00000080, 0x00000000,
	mmFBC_DEBUG_COMP, 0x000000f0, 0x00000070,
	mmFBC_MISC, 0x1f311fff, 0x12300000,
	mmGB_GPU_ID, 0x0000000f, 0x00000000,
	mmHDMI_CONTROL, 0x31000111, 0x00000011,
	mmMC_ARB_WTM_GRPWT_RD, 0x00000003, 0x00000000,
	mmMC_HUB_RDREQ_DMIF_LIMIT, 0x0000007f, 0x00000028,
	mmMC_HUB_WDP_UMC, 0x00007fb6, 0x00000991,
	mmPA_SC_ENHANCE, 0xffffffff, 0x20000001,
	mmPA_SC_FIFO_DEPTH_CNTL, 0x000003ff, 0x000000fc,
	mmPA_SC_LINE_STIPPLE_STATE, 0x0000ff0f, 0x00000000,
	mmRLC_CGCG_CGLS_CTRL, 0x00000003, 0x0000003c,
	mmSDMA0_CHICKEN_BITS, 0xfc910007, 0x00810007,
	mmSDMA0_CLK_CTRL, 0xff000fff, 0x00000000,
	mmSDMA0_GFX_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA0_RLC0_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA0_RLC1_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_CHICKEN_BITS, 0xfc910007, 0x00810007,
	mmSDMA1_CLK_CTRL, 0xff000fff, 0x00000000,
	mmSDMA1_GFX_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_RLC0_IB_CNTL, 0x800f0111, 0x00000100,
	mmSDMA1_RLC1_IB_CNTL, 0x800f0111, 0x00000100,
	mmSQ_RANDOM_WAVE_PRI, 0x001fffff, 0x000006fd,
	mmTA_CNTL_AUX, 0x000f000f, 0x000b0000,
	mmTCC_CTRL, 0x00100000, 0xf31fff7f,
	mmTCC_EXE_DISABLE, 0x00000002, 0x00000002,
	mmTCP_ADDR_CONFIG, 0x000003ff, 0x000002fb,
	mmTCP_CHAN_STEER_HI, 0xffffffff, 0x0000543b,
	mmTCP_CHAN_STEER_LO, 0xffffffff, 0xa9210876,
	mmVGT_RESET_DEBUG, 0x00000004, 0x00000004,
	mmVM_PRT_APERTURE0_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE1_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE2_LOW_ADDR, 0x0fffffff, 0x0fffffff,
	mmVM_PRT_APERTURE3_LOW_ADDR, 0x0fffffff, 0x0fffffff,
};

static xgpu_tonga_golden_common_all: &[u32] = &[
	mmGRBM_GFX_INDEX,               0xffffffff, 0xe0000000,
	mmPA_SC_RASTER_CONFIG,          0xffffffff, 0x16000012,
	mmPA_SC_RASTER_CONFIG_1,        0xffffffff, 0x0000002A,
	mmGB_ADDR_CONFIG,               0xffffffff, 0x22011002,
	mmSPI_RESOURCE_RESERVE_CU_0,    0xffffffff, 0x00000800,
	mmSPI_RESOURCE_RESERVE_CU_1,    0xffffffff, 0x00000800,
	mmSPI_RESOURCE_RESERVE_EN_CU_0, 0xffffffff, 0x00007FBF,
};

unsafe fn xgpu_vi_init_golden_registers(amdgpu_device *adev)
{
	match adev->asic_type {
	case CHIP_FIJI =>
		amdgpu_device_program_register_sequence(adev,
							xgpu_fiji_mgcg_cgcg_init,
							ARRAY_SIZE(
								xgpu_fiji_mgcg_cgcg_init));
		amdgpu_device_program_register_sequence(adev,
							xgpu_fiji_golden_settings_a10,
							ARRAY_SIZE(
								xgpu_fiji_golden_settings_a10));
		amdgpu_device_program_register_sequence(adev,
							xgpu_fiji_golden_common_all,
							ARRAY_SIZE(
								xgpu_fiji_golden_common_all));
		return;
	case CHIP_TONGA =>
		amdgpu_device_program_register_sequence(adev,
							xgpu_tonga_mgcg_cgcg_init,
							ARRAY_SIZE(
								xgpu_tonga_mgcg_cgcg_init));
		amdgpu_device_program_register_sequence(adev,
							xgpu_tonga_golden_settings_a11,
							ARRAY_SIZE(
								xgpu_tonga_golden_settings_a11));
		amdgpu_device_program_register_sequence(adev,
							xgpu_tonga_golden_common_all,
							ARRAY_SIZE(
								xgpu_tonga_golden_common_all));
		return;
	_ =>
		dev_err(adev->dev, "Doesn't support chip type %d\n", adev->asic_type);
		return;
	}
}

/*
 * Mailbox communication between GPU hypervisor and VFs
 */
static unsafe fn xgpu_vi_mailbox_send_ack(amdgpu_device *adev)
{
	u32 reg;
	int timeout = VI_MAILBOX_TIMEDOUT;
	u32 mask = REG_FIELD_MASK(MAILBOX_CONTROL, RCV_MSG_VALID);

	reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	reg = REG_SET_FIELD(reg, MAILBOX_CONTROL, RCV_MSG_ACK, 1);
	WREG32_NO_KIQ(mmMAILBOX_CONTROL, reg);

	/*Wait for RCV_MSG_VALID to be 0*/
	reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	while (reg & mask) {
		if (timeout <= 0) {
			pr_err("RCV_MSG_VALID is not cleared\n");
			return;
		}
		mdelay(1);
		timeout -= 1;

		reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	}
}

static unsafe fn xgpu_vi_mailbox_set_valid(amdgpu_device *adev, bool val)
{
	u32 reg;

	reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	reg = REG_SET_FIELD(reg, MAILBOX_CONTROL,
			    TRN_MSG_VALID, val ? 1 : 0);
	WREG32_NO_KIQ(mmMAILBOX_CONTROL, reg);
}

static unsafe fn xgpu_vi_mailbox_trans_msg(amdgpu_device *adev,
				      idh_request req)
{
	u32 reg;

	reg = RREG32_NO_KIQ(mmMAILBOX_MSGBUF_TRN_DW0);
	reg = REG_SET_FIELD(reg, MAILBOX_MSGBUF_TRN_DW0,
			    MSGBUF_DATA, req);
	WREG32_NO_KIQ(mmMAILBOX_MSGBUF_TRN_DW0, reg);

	xgpu_vi_mailbox_set_valid(adev, true);
}

unsafe fn xgpu_vi_mailbox_rcv_msg(amdgpu_device *adev,
				   idh_event event) -> i32
{
	u32 reg;
	u32 mask = REG_FIELD_MASK(MAILBOX_CONTROL, RCV_MSG_VALID);

	/* workaround: host driver doesn't set VALID for CMPL now */
	if (event != IDH_FLR_NOTIFICATION_CMPL) {
		reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
		if (!(reg & mask))
			return -ENOENT;
	}

	reg = RREG32_NO_KIQ(mmMAILBOX_MSGBUF_RCV_DW0);
	if (reg != event)
		return -ENOENT;

	/* send ack to PF */
	xgpu_vi_mailbox_send_ack(adev);

	return 0;
}

unsafe fn xgpu_vi_poll_ack(amdgpu_device *adev) -> i32
{
	int r = 0, timeout = VI_MAILBOX_TIMEDOUT;
	u32 mask = REG_FIELD_MASK(MAILBOX_CONTROL, TRN_MSG_ACK);
	u32 reg;

	reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	while (!(reg & mask)) {
		if (timeout <= 0) {
			pr_err("Doesn't get ack from pf.\n");
			r = -ETIME;
			return;
		}
		mdelay(5);
		timeout -= 5;

		reg = RREG32_NO_KIQ(mmMAILBOX_CONTROL);
	}

	return r;
}

unsafe fn xgpu_vi_poll_msg(amdgpu_device *adev, idh_event event) -> i32
{
	int r = 0, timeout = VI_MAILBOX_TIMEDOUT;

	r = xgpu_vi_mailbox_rcv_msg(adev, event);
	while (r) {
		if (timeout <= 0) {
			pr_err("Doesn't get ack from pf.\n");
			r = -ETIME;
			return;
		}
		mdelay(5);
		timeout -= 5;

		r = xgpu_vi_mailbox_rcv_msg(adev, event);
	}

	return r;
}

unsafe fn xgpu_vi_send_access_requests(amdgpu_device *adev,
					idh_request request) -> i32
{
	int r;

	xgpu_vi_mailbox_trans_msg(adev, request);

	/* start to poll ack */
	r = xgpu_vi_poll_ack(adev);
	if (r)
		return r;

	xgpu_vi_mailbox_set_valid(adev, false);

	/* start to check msg if request is idh_req_gpu_init_access */
	if (request == IDH_REQ_GPU_INIT_ACCESS ||
		request == IDH_REQ_GPU_FINI_ACCESS ||
		request == IDH_REQ_GPU_RESET_ACCESS) {
		r = xgpu_vi_poll_msg(adev, IDH_READY_TO_ACCESS_GPU);
		if (r) {
			pr_err("Doesn't get ack from pf, give up\n");
			return r;
		}
	}

	return 0;
}

unsafe fn xgpu_vi_request_reset(amdgpu_device *adev) -> i32
{
	return xgpu_vi_send_access_requests(adev, IDH_REQ_GPU_RESET_ACCESS);
}

unsafe fn xgpu_vi_wait_reset_cmpl(amdgpu_device *adev) -> i32
{
	return xgpu_vi_poll_msg(adev, IDH_FLR_NOTIFICATION_CMPL);
}

unsafe fn xgpu_vi_request_full_gpu_access(amdgpu_device *adev,
					   bool init) -> i32
{
	idh_request req;

	req = init ? IDH_REQ_GPU_INIT_ACCESS : IDH_REQ_GPU_FINI_ACCESS;
	return xgpu_vi_send_access_requests(adev, req);
}

unsafe fn xgpu_vi_release_full_gpu_access(amdgpu_device *adev,
					   bool init) -> i32
{
	idh_request req;
	int r = 0;

	req = init ? IDH_REL_GPU_INIT_ACCESS : IDH_REL_GPU_FINI_ACCESS;
	r = xgpu_vi_send_access_requests(adev, req);

	return r;
}

/* add support mailbox interrupts */
unsafe fn xgpu_vi_mailbox_ack_irq(amdgpu_device *adev,
				   amdgpu_irq_src *source,
				   amdgpu_iv_entry *entry) -> i32
{
	DRM_DEBUG("get ack intr and do nothing.\n");
	return 0;
}

unsafe fn xgpu_vi_set_mailbox_ack_irq(amdgpu_device *adev,
				       amdgpu_irq_src *src,
				       unsigned type,
				       amdgpu_interrupt_state state) -> i32
{
	u32 tmp = RREG32_NO_KIQ(mmMAILBOX_INT_CNTL);

	tmp = REG_SET_FIELD(tmp, MAILBOX_INT_CNTL, ACK_INT_EN,
			    (state == AMDGPU_IRQ_STATE_ENABLE) ? 1 : 0);
	WREG32_NO_KIQ(mmMAILBOX_INT_CNTL, tmp);

	return 0;
}

static unsafe fn xgpu_vi_mailbox_flr_work(work_struct *work)
{
	amdgpu_virt *virt = container_of(work, amdgpu_virt, flr_work);
	amdgpu_device *adev = container_of(virt, amdgpu_device, virt);

	/* Trigger recovery due to world switch failure */
	if (amdgpu_device_should_recover_gpu(adev)) {
		amdgpu_reset_context reset_context;
		memset(&reset_context, 0, sizeof(reset_context));

		reset_context.method = AMD_RESET_METHOD_NONE;
		reset_context.reset_req_dev = adev;
		clear_bit(AMDGPU_NEED_FULL_RESET, &reset_context.flags);
		set_bit(AMDGPU_HOST_FLR, &reset_context.flags);

		amdgpu_device_gpu_recover(adev, core::ptr::null_mut(), &reset_context);
	}
}

unsafe fn xgpu_vi_set_mailbox_rcv_irq(amdgpu_device *adev,
				       amdgpu_irq_src *src,
				       unsigned type,
				       amdgpu_interrupt_state state) -> i32
{
	u32 tmp = RREG32_NO_KIQ(mmMAILBOX_INT_CNTL);

	tmp = REG_SET_FIELD(tmp, MAILBOX_INT_CNTL, VALID_INT_EN,
			    (state == AMDGPU_IRQ_STATE_ENABLE) ? 1 : 0);
	WREG32_NO_KIQ(mmMAILBOX_INT_CNTL, tmp);

	return 0;
}

unsafe fn xgpu_vi_mailbox_rcv_irq(amdgpu_device *adev,
				   amdgpu_irq_src *source,
				   amdgpu_iv_entry *entry) -> i32
{
	int r;

	/* trigger gpu-reset by hypervisor only if TDR disabled */
	if (!amdgpu_gpu_recovery) {
		/* see what event we get */
		r = xgpu_vi_mailbox_rcv_msg(adev, IDH_FLR_NOTIFICATION);

		/* only handle FLR_NOTIFY now */
		if (!r)
			WARN_ONCE(!amdgpu_reset_domain_schedule(adev->reset_domain,
								&adev->virt.flr_work),
				  "Failed to queue work! at %s",
				  __func__);
	}

	return 0;
}

static static xgpu_vi_mailbox_ack_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
	.set = xgpu_vi_set_mailbox_ack_irq,
	.process = xgpu_vi_mailbox_ack_irq,
};

static static xgpu_vi_mailbox_rcv_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
	.set = xgpu_vi_set_mailbox_rcv_irq,
	.process = xgpu_vi_mailbox_rcv_irq,
};

unsafe fn xgpu_vi_mailbox_set_irq_funcs(amdgpu_device *adev)
{
	adev->virt.ack_irq.num_types = 1;
	adev->virt.ack_irq.funcs = &xgpu_vi_mailbox_ack_irq_funcs;
	adev->virt.rcv_irq.num_types = 1;
	adev->virt.rcv_irq.funcs = &xgpu_vi_mailbox_rcv_irq_funcs;
}

unsafe fn xgpu_vi_mailbox_add_irq_id(amdgpu_device *adev) -> i32
{
	int r;

	r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, 135, &adev->virt.rcv_irq);
	if (r)
		return r;

	r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, 138, &adev->virt.ack_irq);
	if (r) {
		amdgpu_irq_put(adev, &adev->virt.rcv_irq, 0);
		return r;
	}

	return 0;
}

unsafe fn xgpu_vi_mailbox_get_irq(amdgpu_device *adev) -> i32
{
	int r;

	r = amdgpu_irq_get(adev, &adev->virt.rcv_irq, 0);
	if (r)
		return r;
	r = amdgpu_irq_get(adev, &adev->virt.ack_irq, 0);
	if (r) {
		amdgpu_irq_put(adev, &adev->virt.rcv_irq, 0);
		return r;
	}

	INIT_WORK(&adev->virt.flr_work, xgpu_vi_mailbox_flr_work);

	return 0;
}

unsafe fn xgpu_vi_mailbox_put_irq(amdgpu_device *adev)
{
	amdgpu_irq_put(adev, &adev->virt.ack_irq, 0);
	amdgpu_irq_put(adev, &adev->virt.rcv_irq, 0);
}

static xgpu_vi_virt_ops: amdgpu_virt_ops = amdgpu_virt_ops {
	.req_full_gpu		= xgpu_vi_request_full_gpu_access,
	.rel_full_gpu		= xgpu_vi_release_full_gpu_access,
	.reset_gpu		= xgpu_vi_request_reset,
	.wait_reset             = xgpu_vi_wait_reset_cmpl,
	.trans_msg		= core::ptr::null_mut(), /* Does not need to trans VF errors to host. */
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
