// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level translation of ivpu_hw_ip.c. Register helpers, device
// structures, constants, and kernel services are supplied by surrounding
// translation units and are intentionally left as external dependencies.
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */


const PWR_ISLAND_STATUS_TIMEOUT_US        (5 * USEC_PER_MSEC)

const TIM_SAFE_ENABLE		            0xf1d0dead
const TIM_WATCHDOG_RESET_VALUE            0xffffffff

const ICB_0_IRQ_MASK_37XX ((REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, HOST_IPC_FIFO_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_0_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_1_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_2_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT)))

const ICB_1_IRQ_MASK_37XX ((REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_2_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_3_INT)) | \
			     (REG_FLD(VPU_37XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_4_INT)))

const ICB_0_1_IRQ_MASK_37XX ((((u64)ICB_1_IRQ_MASK_37XX) << 32) | ICB_0_IRQ_MASK_37XX)

const ICB_0_IRQ_MASK_40XX ((REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, HOST_IPC_FIFO_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_0_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_1_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_2_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT)))

const ICB_1_IRQ_MASK_40XX ((REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_2_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_3_INT)) | \
			     (REG_FLD(VPU_40XX_HOST_SS_ICB_STATUS_1, CPU_INT_REDIRECT_4_INT)))

const ICB_0_1_IRQ_MASK_40XX ((((u64)ICB_1_IRQ_MASK_40XX) << 32) | ICB_0_IRQ_MASK_40XX)

const ITF_FIREWALL_VIOLATION_MASK_37XX ((REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, CSS_ROM_CMX)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, CSS_DBG)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, CSS_CTRL)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, DEC400)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, MSS_NCE)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, MSS_MBI)) | \
					  (REG_FLD(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, MSS_MBI_CMX)))

const ITF_FIREWALL_VIOLATION_MASK_40XX ((REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, CSS_ROM_CMX)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, CSS_DBG)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, CSS_CTRL)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, DEC400)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, MSS_NCE)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, MSS_MBI)) | \
					  (REG_FLD(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, MSS_MBI_CMX)))

 fn wait_for_ip_bar(struct ivpu_device *vdev)
{
	return REGV_POLL_FLD(VPU_37XX_HOST_SS_CPR_RST_CLR, AON, 0, 100);
}

 fn host_ss_rst_clr(struct ivpu_device *vdev)
{
	fn val = 0;

	val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_CLR, TOP_NOC, val);
	val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_CLR, DSS_MAS, val);
	val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_CLR, MSS_MAS, val);

	REGV_WR32(VPU_37XX_HOST_SS_CPR_RST_CLR, val);
}

 fn host_ss_noc_qreqn_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_NOC_QREQN);

	if (!REG_TEST_FLD_NUM(VPU_37XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qreqn_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_NOC_QREQN);

	if (!REG_TEST_FLD_NUM(VPU_40XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qreqn_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return host_ss_noc_qreqn_check_37xx(vdev, exp_val);
	else
		return host_ss_noc_qreqn_check_40xx(vdev, exp_val);
}

 fn host_ss_noc_qacceptn_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_NOC_QACCEPTN);

	if (!REG_TEST_FLD_NUM(VPU_37XX_HOST_SS_NOC_QACCEPTN, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qacceptn_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_NOC_QACCEPTN);

	if (!REG_TEST_FLD_NUM(VPU_40XX_HOST_SS_NOC_QACCEPTN, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qacceptn_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return host_ss_noc_qacceptn_check_37xx(vdev, exp_val);
	else
		return host_ss_noc_qacceptn_check_40xx(vdev, exp_val);
}

 fn host_ss_noc_qdeny_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_NOC_QDENY);

	if (!REG_TEST_FLD_NUM(VPU_37XX_HOST_SS_NOC_QDENY, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qdeny_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_NOC_QDENY);

	if (!REG_TEST_FLD_NUM(VPU_40XX_HOST_SS_NOC_QDENY, TOP_SOCMMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn host_ss_noc_qdeny_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return host_ss_noc_qdeny_check_37xx(vdev, exp_val);
	else
		return host_ss_noc_qdeny_check_40xx(vdev, exp_val);
}

 fn top_noc_qrenqn_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_TOP_NOC_QREQN);

	if (!REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QREQN, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qrenqn_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_TOP_NOC_QREQN);

	if (!REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QREQN, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qreqn_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return top_noc_qrenqn_check_37xx(vdev, exp_val);
	else
		return top_noc_qrenqn_check_40xx(vdev, exp_val);
}

fn ivpu_hw_ip_host_ss_configure(struct ivpu_device *vdev)
{
	fn ret;

	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX) {
		ret = wait_for_ip_bar(vdev);
		if (ret) {
			ivpu_err(vdev, "Timed out waiting for NPU IP bar\n");
			return ret;
		}
		host_ss_rst_clr(vdev);
	}

	ret = host_ss_noc_qreqn_check(vdev, 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed qreqn check: %d\n", ret);
		return ret;
	}

	ret = host_ss_noc_qacceptn_check(vdev, 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed qacceptn check: %d\n", ret);
		return ret;
	}

	ret = host_ss_noc_qdeny_check(vdev, 0x0);
	if (ret)
		ivpu_err(vdev, "Failed qdeny check %d\n", ret);

	return ret;
}

 fn idle_gen_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_AON_VPU_IDLE_GEN);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_AON_VPU_IDLE_GEN, EN, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_AON_VPU_IDLE_GEN, EN, val);

	REGV_WR32(VPU_37XX_HOST_SS_AON_VPU_IDLE_GEN, val);
}

 fn idle_gen_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_AON_IDLE_GEN);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_HOST_SS_AON_IDLE_GEN, EN, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_AON_IDLE_GEN, EN, val);

	REGV_WR32(VPU_40XX_HOST_SS_AON_IDLE_GEN, val);
}

fn ivpu_hw_ip_idle_gen_enable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		idle_gen_drive_37xx(vdev, true);
	else
		idle_gen_drive_40xx(vdev, true);
}

fn ivpu_hw_ip_idle_gen_disable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		idle_gen_drive_37xx(vdev, false);
	else
		idle_gen_drive_40xx(vdev, false);
}

 void
pwr_island_delay_set_50xx(struct ivpu_device *vdev, fn post, fn post1, fn post2, fn status)
{
	fn val;

	val = REGV_RD32(VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY);
	val = REG_SET_FLD_NUM(VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY, POST_DLY, post, val);
	val = REG_SET_FLD_NUM(VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY, POST1_DLY, post1, val);
	val = REG_SET_FLD_NUM(VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY, POST2_DLY, post2, val);
	REGV_WR32(VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY, val);

	val = REGV_RD32(VPU_50XX_HOST_SS_AON_PWR_ISLAND_STATUS_DLY);
	val = REG_SET_FLD_NUM(VPU_50XX_HOST_SS_AON_PWR_ISLAND_STATUS_DLY, STATUS_DLY, status, val);
	REGV_WR32(VPU_50XX_HOST_SS_AON_PWR_ISLAND_STATUS_DLY, val);
}

 fn pwr_island_trickle_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, MSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, MSS_CPU, val);

	REGV_WR32(VPU_37XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, val);
}

 fn pwr_island_trickle_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, CSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, CSS_CPU, val);

	REGV_WR32(VPU_40XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0, val);
}

 fn pwr_island_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_AON_PWR_ISLAND_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_AON_PWR_ISLAND_EN0, MSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_AON_PWR_ISLAND_EN0, MSS_CPU, val);

	REGV_WR32(VPU_37XX_HOST_SS_AON_PWR_ISLAND_EN0, val);
}

 fn pwr_island_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_AON_PWR_ISLAND_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_HOST_SS_AON_PWR_ISLAND_EN0, CSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_AON_PWR_ISLAND_EN0, CSS_CPU, val);

	REGV_WR32(VPU_40XX_HOST_SS_AON_PWR_ISLAND_EN0, val);
}

 fn pwr_island_enable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX) {
		pwr_island_trickle_drive_37xx(vdev, true);
		ndelay(500);
		pwr_island_drive_37xx(vdev, true);
	} else {
		pwr_island_trickle_drive_40xx(vdev, true);
		ndelay(500);
		pwr_island_drive_40xx(vdev, true);
	}
}

 fn wait_for_pwr_island_status(struct ivpu_device *vdev, fn exp_val)
{
	if (IVPU_WA(punit_disabled))
		return 0;

	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return REGV_POLL_FLD(VPU_37XX_HOST_SS_AON_PWR_ISLAND_STATUS0, MSS_CPU, exp_val,
				     PWR_ISLAND_STATUS_TIMEOUT_US);
	else
		return REGV_POLL_FLD(VPU_40XX_HOST_SS_AON_PWR_ISLAND_STATUS0, CSS_CPU, exp_val,
				     PWR_ISLAND_STATUS_TIMEOUT_US);
}

 fn pwr_island_isolation_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_AON_PWR_ISO_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_AON_PWR_ISO_EN0, MSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_AON_PWR_ISO_EN0, MSS_CPU, val);

	REGV_WR32(VPU_37XX_HOST_SS_AON_PWR_ISO_EN0, val);
}

 fn pwr_island_isolation_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_AON_PWR_ISO_EN0);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_HOST_SS_AON_PWR_ISO_EN0, CSS_CPU, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_AON_PWR_ISO_EN0, CSS_CPU, val);

	REGV_WR32(VPU_40XX_HOST_SS_AON_PWR_ISO_EN0, val);
}

 fn pwr_island_isolation_drive(struct ivpu_device *vdev, fn enable)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		pwr_island_isolation_drive_37xx(vdev, enable);
	else
		pwr_island_isolation_drive_40xx(vdev, enable);
}

 fn pwr_island_isolation_disable(struct ivpu_device *vdev)
{
	pwr_island_isolation_drive(vdev, false);
}

 fn host_ss_clk_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_CPR_CLK_SET);

	if (enable) {
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, TOP_NOC, val);
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, DSS_MAS, val);
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, MSS_MAS, val);
	} else {
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, TOP_NOC, val);
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, DSS_MAS, val);
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_CLK_SET, MSS_MAS, val);
	}

	REGV_WR32(VPU_37XX_HOST_SS_CPR_CLK_SET, val);
}

 fn host_ss_clk_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_CPR_CLK_EN);

	if (enable) {
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, TOP_NOC, val);
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, DSS_MAS, val);
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, CSS_MAS, val);
	} else {
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, TOP_NOC, val);
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, DSS_MAS, val);
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_CLK_EN, CSS_MAS, val);
	}

	REGV_WR32(VPU_40XX_HOST_SS_CPR_CLK_EN, val);
}

 fn host_ss_clk_drive(struct ivpu_device *vdev, fn enable)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		host_ss_clk_drive_37xx(vdev, enable);
	else
		host_ss_clk_drive_40xx(vdev, enable);
}

 fn host_ss_clk_enable(struct ivpu_device *vdev)
{
	host_ss_clk_drive(vdev, true);
}

 fn host_ss_rst_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_CPR_RST_SET);

	if (enable) {
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, TOP_NOC, val);
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, DSS_MAS, val);
		val = REG_SET_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, MSS_MAS, val);
	} else {
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, TOP_NOC, val);
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, DSS_MAS, val);
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_CPR_RST_SET, MSS_MAS, val);
	}

	REGV_WR32(VPU_37XX_HOST_SS_CPR_RST_SET, val);
}

 fn host_ss_rst_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_CPR_RST_EN);

	if (enable) {
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, TOP_NOC, val);
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, DSS_MAS, val);
		val = REG_SET_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, CSS_MAS, val);
	} else {
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, TOP_NOC, val);
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, DSS_MAS, val);
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_CPR_RST_EN, CSS_MAS, val);
	}

	REGV_WR32(VPU_40XX_HOST_SS_CPR_RST_EN, val);
}

 fn host_ss_rst_drive(struct ivpu_device *vdev, fn enable)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		host_ss_rst_drive_37xx(vdev, enable);
	else
		host_ss_rst_drive_40xx(vdev, enable);
}

 fn host_ss_rst_enable(struct ivpu_device *vdev)
{
	host_ss_rst_drive(vdev, true);
}

 fn host_ss_noc_qreqn_top_socmmio_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_NOC_QREQN);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, val);
	REGV_WR32(VPU_37XX_HOST_SS_NOC_QREQN, val);
}

 fn host_ss_noc_qreqn_top_socmmio_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_HOST_SS_NOC_QREQN);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_SS_NOC_QREQN, TOP_SOCMMIO, val);
	REGV_WR32(VPU_40XX_HOST_SS_NOC_QREQN, val);
}

 fn host_ss_noc_qreqn_top_socmmio_drive(struct ivpu_device *vdev, fn enable)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		host_ss_noc_qreqn_top_socmmio_drive_37xx(vdev, enable);
	else
		host_ss_noc_qreqn_top_socmmio_drive_40xx(vdev, enable);
}

 fn host_ss_axi_drive(struct ivpu_device *vdev, fn enable)
{
	fn ret;

	host_ss_noc_qreqn_top_socmmio_drive(vdev, enable);

	ret = host_ss_noc_qacceptn_check(vdev, enable ? 0x1 : 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed HOST SS NOC QACCEPTN check: %d\n", ret);
		return ret;
	}

	ret = host_ss_noc_qdeny_check(vdev, 0x0);
	if (ret)
		ivpu_err(vdev, "Failed HOST SS NOC QDENY check: %d\n", ret);

	return ret;
}

 fn top_noc_qreqn_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_TOP_NOC_QREQN);

	if (enable) {
		val = REG_SET_FLD(VPU_40XX_TOP_NOC_QREQN, CPU_CTRL, val);
		val = REG_SET_FLD(VPU_40XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, val);
	} else {
		val = REG_CLR_FLD(VPU_40XX_TOP_NOC_QREQN, CPU_CTRL, val);
		val = REG_CLR_FLD(VPU_40XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, val);
	}

	REGV_WR32(VPU_40XX_TOP_NOC_QREQN, val);
}

 fn top_noc_qreqn_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_TOP_NOC_QREQN);

	if (enable) {
		val = REG_SET_FLD(VPU_37XX_TOP_NOC_QREQN, CPU_CTRL, val);
		val = REG_SET_FLD(VPU_37XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, val);
	} else {
		val = REG_CLR_FLD(VPU_37XX_TOP_NOC_QREQN, CPU_CTRL, val);
		val = REG_CLR_FLD(VPU_37XX_TOP_NOC_QREQN, HOSTIF_L2CACHE, val);
	}

	REGV_WR32(VPU_37XX_TOP_NOC_QREQN, val);
}

 fn top_noc_qreqn_drive(struct ivpu_device *vdev, fn enable)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		top_noc_qreqn_drive_37xx(vdev, enable);
	else
		top_noc_qreqn_drive_40xx(vdev, enable);
}

fn ivpu_hw_ip_host_ss_axi_enable(struct ivpu_device *vdev)
{
	return host_ss_axi_drive(vdev, true);
}

 fn top_noc_qacceptn_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_TOP_NOC_QACCEPTN);

	if (!REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QACCEPTN, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QACCEPTN, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qacceptn_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_TOP_NOC_QACCEPTN);

	if (!REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QACCEPTN, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QACCEPTN, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qacceptn_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return top_noc_qacceptn_check_37xx(vdev, exp_val);
	else
		return top_noc_qacceptn_check_40xx(vdev, exp_val);
}

 fn top_noc_qdeny_check_37xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_37XX_TOP_NOC_QDENY);

	if (!REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QDENY, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_37XX_TOP_NOC_QDENY, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qdeny_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_TOP_NOC_QDENY);

	if (!REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QDENY, CPU_CTRL, exp_val, val) ||
	    !REG_TEST_FLD_NUM(VPU_40XX_TOP_NOC_QDENY, HOSTIF_L2CACHE, exp_val, val))
		return -EIO;

	return 0;
}

 fn top_noc_qdeny_check(struct ivpu_device *vdev, fn exp_val)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return top_noc_qdeny_check_37xx(vdev, exp_val);
	else
		return top_noc_qdeny_check_40xx(vdev, exp_val);
}

 fn top_noc_drive(struct ivpu_device *vdev, fn enable)
{
	fn ret;

	top_noc_qreqn_drive(vdev, enable);

	ret = top_noc_qacceptn_check(vdev, enable ? 0x1 : 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed TOP NOC QACCEPTN check: %d\n", ret);
		return ret;
	}

	ret = top_noc_qdeny_check(vdev, 0x0);
	if (ret)
		ivpu_err(vdev, "Failed TOP NOC QDENY check: %d\n", ret);

	return ret;
}

fn ivpu_hw_ip_top_noc_enable(struct ivpu_device *vdev)
{
	return top_noc_drive(vdev, true);
}

 fn dpu_active_drive_37xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_37XX_HOST_SS_AON_DPU_ACTIVE);

	if (enable)
		val = REG_SET_FLD(VPU_37XX_HOST_SS_AON_DPU_ACTIVE, DPU_ACTIVE, val);
	else
		val = REG_CLR_FLD(VPU_37XX_HOST_SS_AON_DPU_ACTIVE, DPU_ACTIVE, val);

	REGV_WR32(VPU_37XX_HOST_SS_AON_DPU_ACTIVE, val);
}

 fn pwr_island_delay_set(struct ivpu_device *vdev)
{
	fn high = vdev->hw->pll.profiling_freq == PLL_PROFILING_FREQ_HIGH;
	fn post, post1, post2, status;

	if (ivpu_hw_ip_gen(vdev) < IVPU_HW_IP_50XX)
		return;

	switch (ivpu_device_id(vdev)) {
	case PCI_DEVICE_ID_WCL:
	case PCI_DEVICE_ID_PTL_P:
		post = high ? 18 : 0;
		post1 = 0;
		post2 = 0;
		status = high ? 46 : 3;
		break;

	case PCI_DEVICE_ID_NVL:
		post = high ? 198 : 17;
		post1 = 0;
		post2 = high ? 198 : 17;
		status = 0;
		break;

	default:
		dump_stack();
		ivpu_err(vdev, "Unknown device ID\n");
		return;
	}

	pwr_island_delay_set_50xx(vdev, post, post1, post2, status);
}

fn ivpu_hw_ip_pwr_domain_enable(struct ivpu_device *vdev)
{
	fn ret;

	pwr_island_delay_set(vdev);
	pwr_island_enable(vdev);

	ret = wait_for_pwr_island_status(vdev, 0x1);
	if (ret) {
		ivpu_err(vdev, "Timed out waiting for power island status\n");
		return ret;
	}

	ret = top_noc_qreqn_check(vdev, 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed TOP NOC QREQN check %d\n", ret);
		return ret;
	}

	host_ss_clk_enable(vdev);
	pwr_island_isolation_disable(vdev);
	host_ss_rst_enable(vdev);

	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		dpu_active_drive_37xx(vdev, true);

	return ret;
}

fn ivpu_hw_ip_read_perf_timer_counter(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return REGV_RD64(VPU_37XX_CPU_SS_TIM_PERF_FREE_CNT);
	else
		return REGV_RD64(VPU_40XX_CPU_SS_TIM_PERF_EXT_FREE_CNT);
}

 fn ivpu_hw_ip_snoop_disable_37xx(struct ivpu_device *vdev)
{
	fn val = REGV_RD32(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES);

	val = REG_SET_FLD(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES, NOSNOOP_OVERRIDE_EN, val);
	val = REG_CLR_FLD(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES, AW_NOSNOOP_OVERRIDE, val);

	if (ivpu_is_force_snoop_enabled(vdev))
		val = REG_CLR_FLD(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES, AR_NOSNOOP_OVERRIDE, val);
	else
		val = REG_SET_FLD(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES, AR_NOSNOOP_OVERRIDE, val);

	REGV_WR32(VPU_37XX_HOST_IF_TCU_PTW_OVERRIDES, val);
}

 fn ivpu_hw_ip_snoop_disable_40xx(struct ivpu_device *vdev)
{
	fn val = REGV_RD32(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES);

	val = REG_SET_FLD(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES, SNOOP_OVERRIDE_EN, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES, AW_SNOOP_OVERRIDE, val);

	if (ivpu_is_force_snoop_enabled(vdev))
		val = REG_SET_FLD(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES, AR_SNOOP_OVERRIDE, val);
	else
		val = REG_CLR_FLD(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES, AR_SNOOP_OVERRIDE, val);

	REGV_WR32(VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES, val);
}

fn ivpu_hw_ip_snoop_disable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return ivpu_hw_ip_snoop_disable_37xx(vdev);
	else
		return ivpu_hw_ip_snoop_disable_40xx(vdev);
}

 fn ivpu_hw_ip_tbu_mmu_enable_37xx(struct ivpu_device *vdev)
{
	fn val = REGV_RD32(VPU_37XX_HOST_IF_TBU_MMUSSIDV);

	val = REG_SET_FLD(VPU_37XX_HOST_IF_TBU_MMUSSIDV, TBU0_AWMMUSSIDV, val);
	val = REG_SET_FLD(VPU_37XX_HOST_IF_TBU_MMUSSIDV, TBU0_ARMMUSSIDV, val);
	val = REG_SET_FLD(VPU_37XX_HOST_IF_TBU_MMUSSIDV, TBU2_AWMMUSSIDV, val);
	val = REG_SET_FLD(VPU_37XX_HOST_IF_TBU_MMUSSIDV, TBU2_ARMMUSSIDV, val);

	REGV_WR32(VPU_37XX_HOST_IF_TBU_MMUSSIDV, val);
}

 fn ivpu_hw_ip_tbu_mmu_enable_40xx(struct ivpu_device *vdev)
{
	fn val = REGV_RD32(VPU_40XX_HOST_IF_TBU_MMUSSIDV);

	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU0_AWMMUSSIDV, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU0_ARMMUSSIDV, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU1_AWMMUSSIDV, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU1_ARMMUSSIDV, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU2_AWMMUSSIDV, val);
	val = REG_SET_FLD(VPU_40XX_HOST_IF_TBU_MMUSSIDV, TBU2_ARMMUSSIDV, val);

	REGV_WR32(VPU_40XX_HOST_IF_TBU_MMUSSIDV, val);
}

fn ivpu_hw_ip_tbu_mmu_enable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return ivpu_hw_ip_tbu_mmu_enable_37xx(vdev);
	else
		return ivpu_hw_ip_tbu_mmu_enable_40xx(vdev);
}

#[inline]
fn get_entry_point_addr(struct ivpu_device *vdev)
{
	if (ivpu_fw_is_warm_boot(vdev))
		return vdev->fw->warm_boot_entry_point;
	else
		return vdev->fw->cold_boot_entry_point;
}

 fn soc_cpu_boot_37xx(struct ivpu_device *vdev)
{
	fn val;

	val = REGV_RD32(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC);
	val = REG_SET_FLD(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, IRQI_RSTRUN0, val);

	val = REG_CLR_FLD(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, IRQI_RSTVEC, val);
	REGV_WR32(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, val);

	val = REG_SET_FLD(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, IRQI_RESUME0, val);
	REGV_WR32(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, val);

	val = REG_CLR_FLD(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, IRQI_RESUME0, val);
	REGV_WR32(VPU_37XX_CPU_SS_MSSCPU_CPR_LEON_RT_VEC, val);

	val = get_entry_point_addr(vdev) >> 9;
	REGV_WR32(VPU_37XX_HOST_SS_LOADING_ADDRESS_LO, val);

	val = REG_SET_FLD(VPU_37XX_HOST_SS_LOADING_ADDRESS_LO, DONE, val);
	REGV_WR32(VPU_37XX_HOST_SS_LOADING_ADDRESS_LO, val);

	return 0;
}

 fn cpu_noc_qacceptn_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_CPU_SS_CPR_NOC_QACCEPTN);

	if (!REG_TEST_FLD_NUM(VPU_40XX_CPU_SS_CPR_NOC_QACCEPTN, TOP_MMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn cpu_noc_qdeny_check_40xx(struct ivpu_device *vdev, fn exp_val)
{
	fn val = REGV_RD32(VPU_40XX_CPU_SS_CPR_NOC_QDENY);

	if (!REG_TEST_FLD_NUM(VPU_40XX_CPU_SS_CPR_NOC_QDENY, TOP_MMIO, exp_val, val))
		return -EIO;

	return 0;
}

 fn cpu_noc_top_mmio_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn val = REGV_RD32(VPU_40XX_CPU_SS_CPR_NOC_QREQN);

	if (enable)
		val = REG_SET_FLD(VPU_40XX_CPU_SS_CPR_NOC_QREQN, TOP_MMIO, val);
	else
		val = REG_CLR_FLD(VPU_40XX_CPU_SS_CPR_NOC_QREQN, TOP_MMIO, val);
	REGV_WR32(VPU_40XX_CPU_SS_CPR_NOC_QREQN, val);
}

 fn soc_cpu_drive_40xx(struct ivpu_device *vdev, fn enable)
{
	fn ret;

	cpu_noc_top_mmio_drive_40xx(vdev, enable);

	ret = cpu_noc_qacceptn_check_40xx(vdev, enable ? 0x1 : 0x0);
	if (ret) {
		ivpu_err(vdev, "Failed qacceptn check: %d\n", ret);
		return ret;
	}

	ret = cpu_noc_qdeny_check_40xx(vdev, 0x0);
	if (ret)
		ivpu_err(vdev, "Failed qdeny check: %d\n", ret);

	return ret;
}

 fn soc_cpu_set_entry_point_40xx(struct ivpu_device *vdev, fn entry_point)
{
	fn val64;
	fn val;

	val64 = entry_point;
	val64 <<= ffs(VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO_IMAGE_LOCATION_MASK) - 1;
	REGV_WR64(VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO, val64);

	val = REGV_RD32(VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO);
	val = REG_SET_FLD(VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO, DONE, val);
	REGV_WR32(VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO, val);
}

 fn soc_cpu_boot_40xx(struct ivpu_device *vdev)
{
	fn ret;

	ret = soc_cpu_drive_40xx(vdev, true);
	if (ret) {
		ivpu_err(vdev, "Failed to enable SOC CPU: %d\n", ret);
		return ret;
	}

	soc_cpu_set_entry_point_40xx(vdev, get_entry_point_addr(vdev));

	return 0;
}

 fn soc_cpu_boot_60xx(struct ivpu_device *vdev)
{
	soc_cpu_set_entry_point_40xx(vdev, vdev->fw->cold_boot_entry_point);

	return 0;
}

fn ivpu_hw_ip_soc_cpu_boot(struct ivpu_device *vdev)
{
	fn ret;

	switch (ivpu_hw_ip_gen(vdev)) {
	case IVPU_HW_IP_37XX:
		ret = soc_cpu_boot_37xx(vdev);
		break;

	case IVPU_HW_IP_40XX:
	case IVPU_HW_IP_50XX:
		ret = soc_cpu_boot_40xx(vdev);
		break;

	default:
		ret = soc_cpu_boot_60xx(vdev);
	}

	if (ret)
		return ret;

	ivpu_dbg(vdev, PM, "Booting firmware, mode: %s\n",
		 ivpu_fw_is_warm_boot(vdev) ? "warm boot" : "cold boot");

	return 0;
}

 fn wdt_disable_37xx(struct ivpu_device *vdev)
{
	fn val;

	/* Enable writing and set non-zero WDT value */
	REGV_WR32(VPU_37XX_CPU_SS_TIM_SAFE, TIM_SAFE_ENABLE);
	REGV_WR32(VPU_37XX_CPU_SS_TIM_WATCHDOG, TIM_WATCHDOG_RESET_VALUE);

	/* Enable writing and disable watchdog timer */
	REGV_WR32(VPU_37XX_CPU_SS_TIM_SAFE, TIM_SAFE_ENABLE);
	REGV_WR32(VPU_37XX_CPU_SS_TIM_WDOG_EN, 0);

	/* Now clear the timeout interrupt */
	val = REGV_RD32(VPU_37XX_CPU_SS_TIM_GEN_CONFIG);
	val = REG_CLR_FLD(VPU_37XX_CPU_SS_TIM_GEN_CONFIG, WDOG_TO_INT_CLR, val);
	REGV_WR32(VPU_37XX_CPU_SS_TIM_GEN_CONFIG, val);
}

 fn wdt_disable_40xx(struct ivpu_device *vdev)
{
	fn val;

	REGV_WR32(VPU_40XX_CPU_SS_TIM_SAFE, TIM_SAFE_ENABLE);
	REGV_WR32(VPU_40XX_CPU_SS_TIM_WATCHDOG, TIM_WATCHDOG_RESET_VALUE);

	REGV_WR32(VPU_40XX_CPU_SS_TIM_SAFE, TIM_SAFE_ENABLE);
	REGV_WR32(VPU_40XX_CPU_SS_TIM_WDOG_EN, 0);

	val = REGV_RD32(VPU_40XX_CPU_SS_TIM_GEN_CONFIG);
	val = REG_CLR_FLD(VPU_40XX_CPU_SS_TIM_GEN_CONFIG, WDOG_TO_INT_CLR, val);
	REGV_WR32(VPU_40XX_CPU_SS_TIM_GEN_CONFIG, val);
}

fn ivpu_hw_ip_wdt_disable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return wdt_disable_37xx(vdev);
	else
		return wdt_disable_40xx(vdev);
}

 fn ipc_rx_count_get_37xx(struct ivpu_device *vdev)
{
	fn count = readl(vdev->regv + VPU_37XX_HOST_SS_TIM_IPC_FIFO_STAT);

	return REG_GET_FLD(VPU_37XX_HOST_SS_TIM_IPC_FIFO_STAT, FILL_LEVEL, count);
}

 fn ipc_rx_count_get_40xx(struct ivpu_device *vdev)
{
	fn count = readl(vdev->regv + VPU_40XX_HOST_SS_TIM_IPC_FIFO_STAT);

	return REG_GET_FLD(VPU_40XX_HOST_SS_TIM_IPC_FIFO_STAT, FILL_LEVEL, count);
}

fn ivpu_hw_ip_ipc_rx_count_get(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return ipc_rx_count_get_37xx(vdev);
	else
		return ipc_rx_count_get_40xx(vdev);
}

fn ivpu_hw_ip_irq_enable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX) {
		REGV_WR32(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, ITF_FIREWALL_VIOLATION_MASK_37XX);
		REGV_WR64(VPU_37XX_HOST_SS_ICB_ENABLE_0, ICB_0_1_IRQ_MASK_37XX);
	} else {
		REGV_WR32(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, ITF_FIREWALL_VIOLATION_MASK_40XX);
		REGV_WR64(VPU_40XX_HOST_SS_ICB_ENABLE_0, ICB_0_1_IRQ_MASK_40XX);
	}
}

fn ivpu_hw_ip_irq_disable(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX) {
		REGV_WR64(VPU_37XX_HOST_SS_ICB_ENABLE_0, 0x0ull);
		REGV_WR32(VPU_37XX_HOST_SS_FW_SOC_IRQ_EN, 0x0);
	} else {
		REGV_WR64(VPU_40XX_HOST_SS_ICB_ENABLE_0, 0x0ull);
		REGV_WR32(VPU_40XX_HOST_SS_FW_SOC_IRQ_EN, 0x0ul);
	}
}

 fn diagnose_failure_37xx(struct ivpu_device *vdev)
{
	fn reg = REGV_RD32(VPU_37XX_HOST_SS_ICB_STATUS_0) & ICB_0_IRQ_MASK_37XX;

	if (ipc_rx_count_get_37xx(vdev))
		ivpu_err(vdev, "IPC FIFO queue not empty, missed IPC IRQ");

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT, reg))
		ivpu_err(vdev, "WDT MSS timeout detected\n");

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT, reg))
		ivpu_err(vdev, "WDT NCE timeout detected\n");

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT, reg))
		ivpu_err(vdev, "NOC Firewall irq detected\n");
}

 fn diagnose_failure_40xx(struct ivpu_device *vdev)
{
	fn reg = REGV_RD32(VPU_40XX_HOST_SS_ICB_STATUS_0) & ICB_0_IRQ_MASK_40XX;

	if (ipc_rx_count_get_40xx(vdev))
		ivpu_err(vdev, "IPC FIFO queue not empty, missed IPC IRQ");

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT, reg))
		ivpu_err(vdev, "WDT MSS timeout detected\n");

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT, reg))
		ivpu_err(vdev, "WDT NCE timeout detected\n");

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT, reg))
		ivpu_err(vdev, "NOC Firewall irq detected\n");
}

fn ivpu_hw_ip_diagnose_failure(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		diagnose_failure_37xx(vdev);
	else
		diagnose_failure_40xx(vdev);
}

fn ivpu_hw_ip_irq_clear(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		REGV_WR64(VPU_37XX_HOST_SS_ICB_CLEAR_0, ICB_0_1_IRQ_MASK_37XX);
	else
		REGV_WR64(VPU_40XX_HOST_SS_ICB_CLEAR_0, ICB_0_1_IRQ_MASK_40XX);
}

 fn irq_wdt_nce_handler(struct ivpu_device *vdev)
{
	ivpu_pm_trigger_recovery(vdev, "WDT NCE IRQ");
}

 fn irq_wdt_mss_handler(struct ivpu_device *vdev)
{
	ivpu_hw_ip_wdt_disable(vdev);
	ivpu_pm_trigger_recovery(vdev, "WDT MSS IRQ");
}

 fn irq_noc_firewall_handler(struct ivpu_device *vdev)
{
	atomic_inc(&vdev->hw->firewall_irq_counter);

	ivpu_dbg(vdev, IRQ, "NOC Firewall interrupt detected, counter %d\n",
		 atomic_read(&vdev->hw->firewall_irq_counter));
}

/* Handler for IRQs from NPU core */
fn ivpu_hw_ip_irq_handler_37xx(struct ivpu_device *vdev, fn irq)
{
	fn status = REGV_RD32(VPU_37XX_HOST_SS_ICB_STATUS_0) & ICB_0_IRQ_MASK_37XX;

	if (!status)
		return false;

	REGV_WR32(VPU_37XX_HOST_SS_ICB_CLEAR_0, status);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_0_INT, status))
		ivpu_mmu_irq_evtq_handler(vdev);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, HOST_IPC_FIFO_INT, status))
		ivpu_ipc_irq_handler(vdev);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_1_INT, status))
		ivpu_dbg(vdev, IRQ, "MMU sync complete\n");

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_2_INT, status))
		ivpu_mmu_irq_gerr_handler(vdev);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT, status))
		irq_wdt_mss_handler(vdev);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT, status))
		irq_wdt_nce_handler(vdev);

	if (REG_TEST_FLD(VPU_37XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT, status))
		irq_noc_firewall_handler(vdev);

	return true;
}

/* Handler for IRQs from NPU core */
fn ivpu_hw_ip_irq_handler_40xx(struct ivpu_device *vdev, fn irq)
{
	fn status = REGV_RD32(VPU_40XX_HOST_SS_ICB_STATUS_0) & ICB_0_IRQ_MASK_40XX;

	if (!status)
		return false;

	REGV_WR32(VPU_40XX_HOST_SS_ICB_CLEAR_0, status);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_0_INT, status))
		ivpu_mmu_irq_evtq_handler(vdev);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, HOST_IPC_FIFO_INT, status))
		ivpu_ipc_irq_handler(vdev);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_1_INT, status))
		ivpu_dbg(vdev, IRQ, "MMU sync complete\n");

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, MMU_IRQ_2_INT, status))
		ivpu_mmu_irq_gerr_handler(vdev);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_0_INT, status))
		irq_wdt_mss_handler(vdev);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, CPU_INT_REDIRECT_1_INT, status))
		irq_wdt_nce_handler(vdev);

	if (REG_TEST_FLD(VPU_40XX_HOST_SS_ICB_STATUS_0, NOC_FIREWALL_INT, status))
		irq_noc_firewall_handler(vdev);

	return true;
}

 fn db_set_37xx(struct ivpu_device *vdev, fn db_id)
{
	fn reg_stride = VPU_37XX_CPU_SS_DOORBELL_1 - VPU_37XX_CPU_SS_DOORBELL_0;
	fn val = REG_FLD(VPU_37XX_CPU_SS_DOORBELL_0, SET);

	REGV_WR32I(VPU_37XX_CPU_SS_DOORBELL_0, reg_stride, db_id, val);
}

 fn db_set_40xx(struct ivpu_device *vdev, fn db_id)
{
	fn reg_stride = VPU_40XX_CPU_SS_DOORBELL_1 - VPU_40XX_CPU_SS_DOORBELL_0;
	fn val = REG_FLD(VPU_40XX_CPU_SS_DOORBELL_0, SET);

	REGV_WR32I(VPU_40XX_CPU_SS_DOORBELL_0, reg_stride, db_id, val);
}

fn ivpu_hw_ip_db_set(struct ivpu_device *vdev, fn db_id)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		db_set_37xx(vdev, db_id);
	else
		db_set_40xx(vdev, db_id);
}

fn ivpu_hw_ip_ipc_rx_addr_get(struct ivpu_device *vdev)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		return REGV_RD32(VPU_37XX_HOST_SS_TIM_IPC_FIFO_ATM);
	else
		return REGV_RD32(VPU_40XX_HOST_SS_TIM_IPC_FIFO_ATM);
}

fn ivpu_hw_ip_ipc_tx_set(struct ivpu_device *vdev, fn vpu_addr)
{
	if (ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX)
		REGV_WR32(VPU_37XX_CPU_SS_TIM_IPC_FIFO, vpu_addr);
	else
		REGV_WR32(VPU_40XX_CPU_SS_TIM_IPC_FIFO, vpu_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
