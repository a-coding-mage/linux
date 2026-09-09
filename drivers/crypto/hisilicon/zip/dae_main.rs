// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 HiSilicon Limited. */

// Dependencies are supplied by the surrounding kernel translation.

const DAE_MEM_START_OFFSET: usize = 0x331040;
const DAE_MEM_DONE_OFFSET: usize = 0x331044;
const DAE_MEM_START_MASK: u32 = 0x1;
const DAE_MEM_DONE_MASK: u32 = 0x1;
const DAE_REG_RD_INTVRL_US: u32 = 10;
const DAE_REG_RD_TMOUT_US: u32 = USEC_PER_SEC;

const DAE_ALG_NAME: &str = "hashagg";
const DAE_V5_ALG_NAME: &str = "hashagg\nudma\nhashjoin\ngather";

const DAE_AXI_CFG_OFFSET: usize = 0x331000;
const DAE_AXI_SHUTDOWN_MASK: u32 = BIT(0) | BIT(5);
const DAE_ERR_SOURCE_OFFSET: usize = 0x331C84;
const DAE_ERR_STATUS_OFFSET: usize = 0x331C88;
const DAE_ERR_CE_OFFSET: usize = 0x331CA0;
const DAE_ERR_CE_MASK: u32 = BIT(3);
const DAE_ERR_NFE_OFFSET: usize = 0x331CA4;
const DAE_ERR_NFE_MASK: u32 = 0x17;
const DAE_ERR_FE_OFFSET: usize = 0x331CA8;
const DAE_ERR_FE_MASK: u32 = 0;
const DAE_ECC_MBIT_MASK: u32 = BIT(2);
const DAE_ECC_INFO_OFFSET: usize = 0x33400C;
const DAE_ERR_SHUTDOWN_OFFSET: usize = 0x331CAC;
const DAE_ERR_SHUTDOWN_MASK: u32 = 0x17;
const DAE_ERR_ENABLE_OFFSET: usize = 0x331C80;
const DAE_ERR_ENABLE_MASK: u32 = DAE_ERR_FE_MASK | DAE_ERR_NFE_MASK | DAE_ERR_CE_MASK;
const DAE_AM_CTRL_GLOBAL_OFFSET: usize = 0x330000;
const DAE_AM_RETURN_OFFSET: usize = 0x330150;
const DAE_AM_RETURN_MASK: u32 = 0x3;
const DAE_AXI_SHUTDOWN_EN_MASK: u32 = BIT(0) | BIT(5);

#[repr(C)]
struct HisiDaeHwError {
    int_msk: u32,
    msg: *const core::ffi::c_char,
}

static DAE_HW_ERROR: [HisiDaeHwError; 5] = [
    HisiDaeHwError { int_msk: BIT(0), msg: c"dae_axi_bus_err".as_ptr() },
    HisiDaeHwError { int_msk: BIT(1), msg: c"dae_axi_poison_err".as_ptr() },
    HisiDaeHwError { int_msk: BIT(2), msg: c"dae_ecc_2bit_err".as_ptr() },
    HisiDaeHwError { int_msk: BIT(3), msg: c"dae_ecc_1bit_err".as_ptr() },
    HisiDaeHwError { int_msk: BIT(4), msg: c"dae_fsm_hbeat_err".as_ptr() },
];

#[inline]
unsafe fn dae_is_support(qm: *mut hisi_qm) -> bool {
    if test_bit(QM_SUPPORT_DAE, &(*qm).caps) { return true; }
    false
}

unsafe fn hisi_dae_set_user_domain(qm: *mut hisi_qm) -> i32 {
    if !dae_is_support(qm) { return 0; }
    let mut val = readl((*qm).io_base.add(DAE_MEM_START_OFFSET));
    val |= DAE_MEM_START_MASK;
    writel(val, (*qm).io_base.add(DAE_MEM_START_OFFSET));
    let ret = readl_relaxed_poll_timeout((*qm).io_base.add(DAE_MEM_DONE_OFFSET), &mut val,
        val & DAE_MEM_DONE_MASK, DAE_REG_RD_INTVRL_US, DAE_REG_RD_TMOUT_US);
    if ret != 0 { pci_err((*qm).pdev, "failed to init dae memory!\n"); }
    ret
}

unsafe fn hisi_dae_set_alg(qm: *mut hisi_qm) -> i32 {
    if !dae_is_support(qm) || (*qm).uacce.is_null() { return 0; }
    let alg_name = if (*qm).ver >= QM_HW_V5 { DAE_V5_ALG_NAME } else { DAE_ALG_NAME };
    let algs = (*qm).uacce.as_mut().unwrap().algs.as_mut_ptr() as *mut u8;
    let len = strlen(algs);
    if len + strlen(alg_name.as_ptr()) + 1 >= QM_DEV_ALG_MAX_LEN {
        pci_err((*qm).pdev, "algorithm name is too long!\n");
        return -EINVAL;
    }
    if len != 0 { strcat(algs, c"\n".as_ptr() as *const u8); }
    strcat(algs, alg_name.as_ptr());
    0
}

unsafe fn hisi_dae_master_ooo_ctrl(qm: *mut hisi_qm, enable: bool) {
    let mut axi_val = readl((*qm).io_base.add(DAE_AXI_CFG_OFFSET));
    let err_val;
    if enable { axi_val |= DAE_AXI_SHUTDOWN_MASK; err_val = DAE_ERR_SHUTDOWN_MASK; }
    else { axi_val &= !DAE_AXI_SHUTDOWN_MASK; err_val = 0; }
    writel(axi_val, (*qm).io_base.add(DAE_AXI_CFG_OFFSET));
    writel(err_val, (*qm).io_base.add(DAE_ERR_SHUTDOWN_OFFSET));
}

unsafe fn hisi_dae_hw_error_enable(qm: *mut hisi_qm) {
    if !dae_is_support(qm) { return; }
    writel(DAE_ERR_ENABLE_MASK, (*qm).io_base.add(DAE_ERR_SOURCE_OFFSET));
    writel(DAE_ERR_CE_MASK, (*qm).io_base.add(DAE_ERR_CE_OFFSET));
    writel(DAE_ERR_NFE_MASK, (*qm).io_base.add(DAE_ERR_NFE_OFFSET));
    writel(DAE_ERR_FE_MASK, (*qm).io_base.add(DAE_ERR_FE_OFFSET));
    hisi_dae_master_ooo_ctrl(qm, true);
    writel(DAE_ERR_ENABLE_MASK, (*qm).io_base.add(DAE_ERR_ENABLE_OFFSET));
}

unsafe fn hisi_dae_hw_error_disable(qm: *mut hisi_qm) {
    if !dae_is_support(qm) { return; }
    writel(0, (*qm).io_base.add(DAE_ERR_ENABLE_OFFSET));
    hisi_dae_master_ooo_ctrl(qm, false);
}

unsafe fn hisi_dae_get_hw_err_status(qm: *mut hisi_qm) -> u32 { readl((*qm).io_base.add(DAE_ERR_STATUS_OFFSET)) }

unsafe fn hisi_dae_clear_hw_err_status(qm: *mut hisi_qm, err_sts: u32) {
    if dae_is_support(qm) { writel(err_sts, (*qm).io_base.add(DAE_ERR_SOURCE_OFFSET)); }
}

unsafe fn hisi_dae_disable_error_report(qm: *mut hisi_qm, err_type: u32) {
    writel(DAE_ERR_NFE_MASK & !err_type, (*qm).io_base.add(DAE_ERR_NFE_OFFSET));
}

unsafe fn hisi_dae_enable_error_report(qm: *mut hisi_qm) {
    writel(DAE_ERR_CE_MASK, (*qm).io_base.add(DAE_ERR_CE_OFFSET));
    writel(DAE_ERR_NFE_MASK, (*qm).io_base.add(DAE_ERR_NFE_OFFSET));
}

unsafe fn hisi_dae_log_hw_error(qm: *mut hisi_qm, err_type: u32) {
    for err in &DAE_HW_ERROR {
        if err.int_msk & err_type == 0 { continue; }
        dev_err(&(*(*qm).pdev).dev, "%s [error status=0x%x] found\n", err.msg, err.int_msk);
        if err.int_msk & DAE_ECC_MBIT_MASK != 0 {
            let ecc_info = readl((*qm).io_base.add(DAE_ECC_INFO_OFFSET));
            dev_err(&(*(*qm).pdev).dev, "dae multi ecc sram info 0x%x\n", ecc_info);
        }
    }
}

unsafe fn hisi_dae_get_err_result(qm: *mut hisi_qm) -> acc_err_result {
    if !dae_is_support(qm) { return ACC_ERR_NONE; }
    let err_status = hisi_dae_get_hw_err_status(qm);
    if err_status == 0 { return ACC_ERR_NONE; }
    hisi_dae_log_hw_error(qm, err_status);
    if err_status & DAE_ERR_NFE_MASK != 0 {
        hisi_dae_disable_error_report(qm, err_status);
        return ACC_ERR_NEED_RESET;
    }
    hisi_dae_clear_hw_err_status(qm, err_status);
    hisi_dae_enable_error_report(qm);
    ACC_ERR_RECOVERED
}

unsafe fn hisi_dae_dev_is_abnormal(qm: *mut hisi_qm) -> bool {
    if !dae_is_support(qm) { return false; }
    hisi_dae_get_hw_err_status(qm) & DAE_ERR_NFE_MASK != 0
}

unsafe fn hisi_dae_close_axi_master_ooo(qm: *mut hisi_qm) -> i32 {
    if !dae_is_support(qm) { return 0; }
    let mut val = readl((*qm).io_base.add(DAE_AM_CTRL_GLOBAL_OFFSET));
    val |= BIT(0);
    writel(val, (*qm).io_base.add(DAE_AM_CTRL_GLOBAL_OFFSET));
    let ret = readl_relaxed_poll_timeout((*qm).io_base.add(DAE_AM_RETURN_OFFSET), &mut val,
        val == DAE_AM_RETURN_MASK, DAE_REG_RD_INTVRL_US, DAE_REG_RD_TMOUT_US);
    if ret != 0 { dev_err(&(*(*qm).pdev).dev, "failed to close dae axi ooo!\n"); }
    ret
}

unsafe fn hisi_dae_open_axi_master_ooo(qm: *mut hisi_qm) {
    if !dae_is_support(qm) { return; }
    let val = readl((*qm).io_base.add(DAE_AXI_CFG_OFFSET));
    writel(val & !DAE_AXI_SHUTDOWN_EN_MASK, (*qm).io_base.add(DAE_AXI_CFG_OFFSET));
    writel(val | DAE_AXI_SHUTDOWN_EN_MASK, (*qm).io_base.add(DAE_AXI_CFG_OFFSET));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
