// C-to-Rust translation; external kernel symbols and macros are supplied by dependencies.
#![allow(unused, non_snake_case, non_camel_case_types)]

// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */
// #include "adf_common_drv.h"
// #include "adf_gen4_hw_data.h"
// #include "adf_gen4_ras.h"
// #include "adf_sysfs_ras_counters.h"

#define BITS_PER_REG(_n_) (sizeof(_n_) * BITS_PER_BYTE)

unsafe fn enable_errsou_reporting(*mut core::ffi::c_voidcsr)
{
	/* Enable correctable error reporting in ERRSOU0 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK0, 0);

	/* Enable uncorrectable error reporting in ERRSOU1 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK1, 0);

	/*
	 * Enable uncorrectable error reporting in ERRSOU2
	 * but disable PM interrupt and CFC attention interrupt by default
	 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK2,
		   ADF_GEN4_ERRSOU2_PM_INT_BIT |
		   ADF_GEN4_ERRSOU2_CPP_CFC_ATT_INT_BITMASK);

	/*
	 * Enable uncorrectable error reporting in ERRSOU3
	 * but disable RLT error interrupt and VFLR notify interrupt by default
	 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK3,
		   ADF_GEN4_ERRSOU3_RLTERROR_BIT |
		   ADF_GEN4_ERRSOU3_VFLRNOTIFY_BIT);
}

unsafe fn disable_errsou_reporting(*mut core::ffi::c_voidcsr)
{
	let mut val: u32 = 0;

	/* Disable correctable error reporting in ERRSOU0 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK0, ADF_GEN4_ERRSOU0_BIT);

	/* Disable uncorrectable error reporting in ERRSOU1 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK1, ADF_GEN4_ERRSOU1_BITMASK);

	/* Disable uncorrectable error reporting in ERRSOU2 */
	val = ADF_CSR_RD(csr, ADF_GEN4_ERRMSK2);
	val |= ADF_GEN4_ERRSOU2_DIS_BITMASK;
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK2, val);

	/* Disable uncorrectable error reporting in ERRSOU3 */
	ADF_CSR_WR(csr, ADF_GEN4_ERRMSK3, ADF_GEN4_ERRSOU3_BITMASK);
}

unsafe fn enable_ae_error_reporting(*mut adf_accel_devaccel_dev,
				      *mut core::ffi::c_voidcsr)
{
	let mut ae_mask: u32 = GET_HW_DATA(accel_dev)->ae_mask;

	/* Enable Acceleration Engine correctable error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HIAECORERRLOGENABLE_CPP0, ae_mask);

	/* Enable Acceleration Engine uncorrectable error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HIAEUNCERRLOGENABLE_CPP0, ae_mask);
}

unsafe fn disable_ae_error_reporting(*mut core::ffi::c_voidcsr)
{
	/* Disable Acceleration Engine correctable error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HIAECORERRLOGENABLE_CPP0, 0);

	/* Disable Acceleration Engine uncorrectable error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HIAEUNCERRLOGENABLE_CPP0, 0);
}

unsafe fn enable_cpp_error_reporting(*mut adf_accel_devaccel_dev,
				       *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);

	/* Enable HI CPP Agents Command Parity Error Reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HICPPAGENTCMDPARERRLOGENABLE,
		   (*err_mask).cppagentcmdpar_mask);

	ADF_CSR_WR(csr, ADF_GEN4_CPP_CFC_ERR_CTRL,
		   ADF_GEN4_CPP_CFC_ERR_CTRL_BITMASK);
}

unsafe fn disable_cpp_error_reporting(*mut core::ffi::c_voidcsr)
{
	/* Disable HI CPP Agents Command Parity Error Reporting */
	ADF_CSR_WR(csr, ADF_GEN4_HICPPAGENTCMDPARERRLOGENABLE, 0);

	ADF_CSR_WR(csr, ADF_GEN4_CPP_CFC_ERR_CTRL,
		   ADF_GEN4_CPP_CFC_ERR_CTRL_DIS_BITMASK);
}

unsafe fn enable_ti_ri_error_reporting(*mut core::ffi::c_voidcsr)
{
	let mut reg: u32;

	/* Enable RI Memory error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_RI_MEM_PAR_ERR_EN0,
		   ADF_GEN4_RIMEM_PARERR_STS_FATAL_BITMASK |
		   ADF_GEN4_RIMEM_PARERR_STS_UNCERR_BITMASK);

	/* Enable IOSF Primary Command Parity error Reporting */
	ADF_CSR_WR(csr, ADF_GEN4_RIMISCCTL, ADF_GEN4_RIMISCSTS_BIT);

	/* Enable TI Internal Memory Parity Error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_TI_CI_PAR_ERR_MASK, 0);
	ADF_CSR_WR(csr, ADF_GEN4_TI_PULL0FUB_PAR_ERR_MASK, 0);
	ADF_CSR_WR(csr, ADF_GEN4_TI_PUSHFUB_PAR_ERR_MASK, 0);
	ADF_CSR_WR(csr, ADF_GEN4_TI_CD_PAR_ERR_MASK, 0);
	ADF_CSR_WR(csr, ADF_GEN4_TI_TRNSB_PAR_ERR_MASK, 0);

	/* Enable error handling in RI, TI CPP interface control registers */
	ADF_CSR_WR(csr, ADF_GEN4_RICPPINTCTL, ADF_GEN4_RICPPINTCTL_BITMASK);

	ADF_CSR_WR(csr, ADF_GEN4_TICPPINTCTL, ADF_GEN4_TICPPINTCTL_BITMASK);

	/*
	 * Enable error detection and reporting in TIMISCSTS
	 * with bits 1, 2 and 30 value preserved
	 */
	reg = ADF_CSR_RD(csr, ADF_GEN4_TIMISCCTL);
	reg &= ADF_GEN4_TIMSCCTL_RELAY_BITMASK;
	reg |= ADF_GEN4_TIMISCCTL_BIT;
	ADF_CSR_WR(csr, ADF_GEN4_TIMISCCTL, reg);
}

unsafe fn disable_ti_ri_error_reporting(*mut core::ffi::c_voidcsr)
{
	let mut reg: u32;

	/* Disable RI Memory error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_RI_MEM_PAR_ERR_EN0, 0);

	/* Disable IOSF Primary Command Parity error Reporting */
	ADF_CSR_WR(csr, ADF_GEN4_RIMISCCTL, 0);

	/* Disable TI Internal Memory Parity Error reporting */
	ADF_CSR_WR(csr, ADF_GEN4_TI_CI_PAR_ERR_MASK,
		   ADF_GEN4_TI_CI_PAR_STS_BITMASK);
	ADF_CSR_WR(csr, ADF_GEN4_TI_PULL0FUB_PAR_ERR_MASK,
		   ADF_GEN4_TI_PULL0FUB_PAR_STS_BITMASK);
	ADF_CSR_WR(csr, ADF_GEN4_TI_PUSHFUB_PAR_ERR_MASK,
		   ADF_GEN4_TI_PUSHFUB_PAR_STS_BITMASK);
	ADF_CSR_WR(csr, ADF_GEN4_TI_CD_PAR_ERR_MASK,
		   ADF_GEN4_TI_CD_PAR_STS_BITMASK);
	ADF_CSR_WR(csr, ADF_GEN4_TI_TRNSB_PAR_ERR_MASK,
		   ADF_GEN4_TI_TRNSB_PAR_STS_BITMASK);

	/* Disable error handling in RI, TI CPP interface control registers */
	ADF_CSR_WR(csr, ADF_GEN4_RICPPINTCTL, 0);

	ADF_CSR_WR(csr, ADF_GEN4_TICPPINTCTL, 0);

	/*
	 * Disable error detection and reporting in TIMISCSTS
	 * with bits 1, 2 and 30 value preserved
	 */
	reg = ADF_CSR_RD(csr, ADF_GEN4_TIMISCCTL);
	reg &= ADF_GEN4_TIMSCCTL_RELAY_BITMASK;
	ADF_CSR_WR(csr, ADF_GEN4_TIMISCCTL, reg);
}

unsafe fn enable_rf_error_reporting(*mut adf_accel_devaccel_dev,
				      *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);

	/* Enable RF parity error in Shared RAM */
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_SRC, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_ATH_CPH, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_CPR_XLT, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_DCPR_UCS, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_PKE, 0);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_WAT_WCP, 0);
}

unsafe fn disable_rf_error_reporting(*mut adf_accel_devaccel_dev,
				       *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);

	/* Disable RF Parity Error reporting in Shared RAM */
	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_SRC,
		   ADF_GEN4_SSMSOFTERRORPARITY_SRC_BIT);

	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_ATH_CPH,
		   (*err_mask).parerr_ath_cph_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_CPR_XLT,
		   (*err_mask).parerr_cpr_xlt_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_DCPR_UCS,
		   (*err_mask).parerr_dcpr_ucs_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_PKE,
		   (*err_mask).parerr_pke_mask);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SSMSOFTERRORPARITYMASK_WAT_WCP,
			   (*err_mask).parerr_wat_wcp_mask);
}

unsafe fn enable_ssm_error_reporting(*mut adf_accel_devaccel_dev,
				       *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut val: u32 = 0;

	/* Enable SSM interrupts */
	ADF_CSR_WR(csr, ADF_GEN4_INTMASKSSM, 0);

	/* Enable shared memory error detection & correction */
	val = ADF_CSR_RD(csr, ADF_GEN4_SSMFEATREN);
	val |= (*err_mask).ssmfeatren_mask;
	ADF_CSR_WR(csr, ADF_GEN4_SSMFEATREN, val);

	/* Enable SER detection in SER_err_ssmsh register */
	ADF_CSR_WR(csr, ADF_GEN4_SER_EN_SSMSH,
		   ADF_GEN4_SER_EN_SSMSH_BITMASK);

	/* Enable SSM soft parity error */
	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_ATH_CPH, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_CPR_XLT, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_DCPR_UCS, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_PKE, 0);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_WAT_WCP, 0);

	/* Enable slice hang interrupt reporting */
	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_ATH_CPH, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_CPR_XLT, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_DCPR_UCS, 0);
	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_PKE, 0);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_WAT_WCP, 0);
}

unsafe fn disable_ssm_error_reporting(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut val: u32 = 0;

	/* Disable SSM interrupts */
	ADF_CSR_WR(csr, ADF_GEN4_INTMASKSSM,
		   ADF_GEN4_INTMASKSSM_BITMASK);

	/* Disable shared memory error detection & correction */
	val = ADF_CSR_RD(csr, ADF_GEN4_SSMFEATREN);
	val &= ADF_GEN4_SSMFEATREN_DIS_BITMASK;
	ADF_CSR_WR(csr, ADF_GEN4_SSMFEATREN, val);

	/* Disable SER detection in SER_err_ssmsh register */
	ADF_CSR_WR(csr, ADF_GEN4_SER_EN_SSMSH, 0);

	/* Disable SSM soft parity error */
	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_ATH_CPH,
		   (*err_mask).parerr_ath_cph_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_CPR_XLT,
		   (*err_mask).parerr_cpr_xlt_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_DCPR_UCS,
		   (*err_mask).parerr_dcpr_ucs_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_PKE,
		   (*err_mask).parerr_pke_mask);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SPPPARERRMSK_WAT_WCP,
			   (*err_mask).parerr_wat_wcp_mask);

	/* Disable slice hang interrupt reporting */
	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_ATH_CPH,
		   (*err_mask).parerr_ath_cph_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_CPR_XLT,
		   (*err_mask).parerr_cpr_xlt_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_DCPR_UCS,
		   (*err_mask).parerr_dcpr_ucs_mask);

	ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_PKE,
		   (*err_mask).parerr_pke_mask);

	if ((*err_mask).parerr_wat_wcp_mask)
		ADF_CSR_WR(csr, ADF_GEN4_SHINTMASKSSM_WAT_WCP,
			   (*err_mask).parerr_wat_wcp_mask);
}

unsafe fn enable_aram_error_reporting(*mut core::ffi::c_voidcsr)
{
	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMCERRUERR_EN,
		   ADF_GEN4_REG_ARAMCERRUERR_EN_BITMASK);

	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMCERR,
		   ADF_GEN4_REG_ARAMCERR_EN_BITMASK);

	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMUERR,
		   ADF_GEN4_REG_ARAMUERR_EN_BITMASK);

	ADF_CSR_WR(csr, ADF_GEN4_REG_CPPMEMTGTERR,
		   ADF_GEN4_REG_CPPMEMTGTERR_EN_BITMASK);
}

unsafe fn disable_aram_error_reporting(*mut core::ffi::c_voidcsr)
{
	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMCERRUERR_EN, 0);
	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMCERR, 0);
	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMUERR, 0);
	ADF_CSR_WR(csr, ADF_GEN4_REG_CPPMEMTGTERR, 0);
}

unsafe fn adf_gen4_enable_ras(*mut adf_accel_devaccel_dev)
{
	*mut core::ffi::c_voidaram_csr = adf_get_aram_base(accel_dev);
	*mut core::ffi::c_voidcsr = adf_get_pmisc_base(accel_dev);

	enable_errsou_reporting(csr);
	enable_ae_error_reporting(accel_dev, csr);
	enable_cpp_error_reporting(accel_dev, csr);
	enable_ti_ri_error_reporting(csr);
	enable_rf_error_reporting(accel_dev, csr);
	enable_ssm_error_reporting(accel_dev, csr);
	enable_aram_error_reporting(aram_csr);
}

unsafe fn adf_gen4_disable_ras(*mut adf_accel_devaccel_dev)
{
	*mut core::ffi::c_voidaram_csr = adf_get_aram_base(accel_dev);
	*mut core::ffi::c_voidcsr = adf_get_pmisc_base(accel_dev);

	disable_errsou_reporting(csr);
	disable_ae_error_reporting(csr);
	disable_cpp_error_reporting(csr);
	disable_ti_ri_error_reporting(csr);
	disable_rf_error_reporting(accel_dev, csr);
	disable_ssm_error_reporting(accel_dev, csr);
	disable_aram_error_reporting(aram_csr);
}

unsafe fn adf_gen4_process_errsou0(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr)
{
	let mut aecorrerr: u32 = ADF_CSR_RD(csr, ADF_GEN4_HIAECORERRLOG_CPP0);

	aecorrerr &= GET_HW_DATA(accel_dev)->ae_mask;

	dev_warn(&GET_DEV(accel_dev),
		 "Correctable error detected in AE: 0x%x\n",
		 aecorrerr);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);

	/* Clear interrupt from ERRSOU0 */
	ADF_CSR_WR(csr, ADF_GEN4_HIAECORERRLOG_CPP0, aecorrerr);
}

unsafe fn adf_handle_cpp_aeunc(*mut adf_accel_devaccel_dev,
				 *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut aeuncorerr: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_HIAEUNCERRLOG_CPP0_BIT))
		return false;

	aeuncorerr = ADF_CSR_RD(csr, ADF_GEN4_HIAEUNCERRLOG_CPP0);
	aeuncorerr &= GET_HW_DATA(accel_dev)->ae_mask;

	dev_err(&GET_DEV(accel_dev),
		"Uncorrectable error detected in AE: 0x%x\n",
		aeuncorerr);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_HIAEUNCERRLOG_CPP0, aeuncorerr);

	return false;
}

unsafe fn adf_handle_cppcmdparerr(*mut adf_accel_devaccel_dev,
				    *mut core::ffi::c_voidcsr, u32 errsou)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut cmdparerr: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_HICPPAGENTCMDPARERRLOG_BIT))
		return false;

	cmdparerr = ADF_CSR_RD(csr, ADF_GEN4_HICPPAGENTCMDPARERRLOG);
	cmdparerr &= (*err_mask).cppagentcmdpar_mask;

	dev_err(&GET_DEV(accel_dev),
		"HI CPP agent command parity error: 0x%x\n",
		cmdparerr);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

	ADF_CSR_WR(csr, ADF_GEN4_HICPPAGENTCMDPARERRLOG, cmdparerr);

	return true;
}

unsafe fn adf_handle_ri_mem_par_err(*mut adf_accel_devaccel_dev,
				      *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut *reset_required = false;
	let mut rimem_parerr_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_RIMEM_PARERR_STS_BIT))
		return false;

	rimem_parerr_sts = ADF_CSR_RD(csr, ADF_GEN4_RIMEM_PARERR_STS);
	rimem_parerr_sts &= ADF_GEN4_RIMEM_PARERR_STS_UNCERR_BITMASK |
			    ADF_GEN4_RIMEM_PARERR_STS_FATAL_BITMASK;

	if (rimem_parerr_sts & ADF_GEN4_RIMEM_PARERR_STS_UNCERR_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"RI Memory Parity uncorrectable error: 0x%x\n",
			rimem_parerr_sts);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	if (rimem_parerr_sts & ADF_GEN4_RIMEM_PARERR_STS_FATAL_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"RI Memory Parity fatal error: 0x%x\n",
			rimem_parerr_sts);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);
		*reset_required = true;
	}

	ADF_CSR_WR(csr, ADF_GEN4_RIMEM_PARERR_STS, rimem_parerr_sts);

	return *reset_required;
}

unsafe fn adf_handle_ti_ci_par_sts(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ti_ci_par_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	ti_ci_par_sts = ADF_CSR_RD(csr, ADF_GEN4_TI_CI_PAR_STS);
	ti_ci_par_sts &= ADF_GEN4_TI_CI_PAR_STS_BITMASK;

	if (ti_ci_par_sts) {
		dev_err(&GET_DEV(accel_dev),
			"TI Memory Parity Error: 0x%x\n", ti_ci_par_sts);
		ADF_CSR_WR(csr, ADF_GEN4_TI_CI_PAR_STS, ti_ci_par_sts);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	return false;
}

unsafe fn adf_handle_ti_pullfub_par_sts(*mut adf_accel_devaccel_dev,
					  *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ti_pullfub_par_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	ti_pullfub_par_sts = ADF_CSR_RD(csr, ADF_GEN4_TI_PULL0FUB_PAR_STS);
	ti_pullfub_par_sts &= ADF_GEN4_TI_PULL0FUB_PAR_STS_BITMASK;

	if (ti_pullfub_par_sts) {
		dev_err(&GET_DEV(accel_dev),
			"TI Pull Parity Error: 0x%x\n", ti_pullfub_par_sts);

		ADF_CSR_WR(csr, ADF_GEN4_TI_PULL0FUB_PAR_STS,
			   ti_pullfub_par_sts);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	return false;
}

unsafe fn adf_handle_ti_pushfub_par_sts(*mut adf_accel_devaccel_dev,
					  *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ti_pushfub_par_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	ti_pushfub_par_sts = ADF_CSR_RD(csr, ADF_GEN4_TI_PUSHFUB_PAR_STS);
	ti_pushfub_par_sts &= ADF_GEN4_TI_PUSHFUB_PAR_STS_BITMASK;

	if (ti_pushfub_par_sts) {
		dev_err(&GET_DEV(accel_dev),
			"TI Push Parity Error: 0x%x\n", ti_pushfub_par_sts);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_TI_PUSHFUB_PAR_STS,
			   ti_pushfub_par_sts);
	}

	return false;
}

unsafe fn adf_handle_ti_cd_par_sts(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ti_cd_par_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	ti_cd_par_sts = ADF_CSR_RD(csr, ADF_GEN4_TI_CD_PAR_STS);
	ti_cd_par_sts &= ADF_GEN4_TI_CD_PAR_STS_BITMASK;

	if (ti_cd_par_sts) {
		dev_err(&GET_DEV(accel_dev),
			"TI CD Parity Error: 0x%x\n", ti_cd_par_sts);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_TI_CD_PAR_STS, ti_cd_par_sts);
	}

	return false;
}

unsafe fn adf_handle_ti_trnsb_par_sts(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ti_trnsb_par_sts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	ti_trnsb_par_sts = ADF_CSR_RD(csr, ADF_GEN4_TI_TRNSB_PAR_STS);
	ti_trnsb_par_sts &= ADF_GEN4_TI_TRNSB_PAR_STS_BITMASK;

	if (ti_trnsb_par_sts) {
		dev_err(&GET_DEV(accel_dev),
			"TI TRNSB Parity Error: 0x%x\n", ti_trnsb_par_sts);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_TI_TRNSB_PAR_STS, ti_trnsb_par_sts);
	}

	return false;
}

unsafe fn adf_handle_iosfp_cmd_parerr(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut rimiscsts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU1_TIMEM_PARERR_STS_BIT))
		return false;

	rimiscsts = ADF_CSR_RD(csr, ADF_GEN4_RIMISCSTS);
	rimiscsts &= ADF_GEN4_RIMISCSTS_BIT;

	dev_err(&GET_DEV(accel_dev),
		"Command Parity error detected on IOSFP: 0x%x\n",
		rimiscsts);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

	ADF_CSR_WR(csr, ADF_GEN4_RIMISCSTS, rimiscsts);

	return true;
}

unsafe fn adf_gen4_process_errsou1(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 errsou,
				     bool **reset_required)
{
	**reset_required |= adf_handle_cpp_aeunc(accel_dev, csr, errsou);
	**reset_required |= adf_handle_cppcmdparerr(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ri_mem_par_err(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ti_ci_par_sts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ti_pullfub_par_sts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ti_pushfub_par_sts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ti_cd_par_sts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ti_trnsb_par_sts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_iosfp_cmd_parerr(accel_dev, csr, errsou);
}

unsafe fn adf_handle_uerrssmsh(*mut adf_accel_devaccel_dev,
				 *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	let mut reg: u32;

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_UERRSSMSH_BIT))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_UERRSSMSH);
	reg &= ADF_GEN4_UERRSSMSH_BITMASK;

	dev_err(&GET_DEV(accel_dev),
		"Uncorrectable error on ssm shared memory: 0x%x\n",
		reg);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_UERRSSMSH, reg);

	return false;
}

unsafe fn adf_handle_cerrssmsh(*mut adf_accel_devaccel_dev,
				 *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	let mut reg: u32;

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_CERRSSMSH_BIT))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_CERRSSMSH);
	reg &= ADF_GEN4_CERRSSMSH_ERROR_BIT;

	dev_warn(&GET_DEV(accel_dev),
		 "Correctable error on ssm shared memory: 0x%x\n",
		 reg);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);

	ADF_CSR_WR(csr, ADF_GEN4_CERRSSMSH, reg);

	return false;
}

unsafe fn adf_handle_pperr_err(*mut adf_accel_devaccel_dev,
				 *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	let mut reg: u32;

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_PPERR_BIT))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_PPERR);
	reg &= ADF_GEN4_PPERR_BITMASK;

	dev_err(&GET_DEV(accel_dev),
		"Uncorrectable error CPP transaction on memory target: 0x%x\n",
		reg);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_PPERR, reg);

	return false;
}

unsafe fn adf_poll_slicehang_csr(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr, u32 slice_hang_offset,
				   *mut core::ffi::c_charslice_name)
{
	let mut slice_hang_reg: u32 = ADF_CSR_RD(csr, slice_hang_offset);

	if (!slice_hang_reg)
		return;

	dev_err(&GET_DEV(accel_dev),
		"Slice %s hang error encountered\n", slice_name);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
}

unsafe fn adf_handle_slice_hang_error(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr, u32 iastatssm)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_SLICEHANG_ERR_BIT))
		return false;

	adf_poll_slicehang_csr(accel_dev, csr,
			       ADF_GEN4_SLICEHANGSTATUS_ATH_CPH, "ath_cph");
	adf_poll_slicehang_csr(accel_dev, csr,
			       ADF_GEN4_SLICEHANGSTATUS_CPR_XLT, "cpr_xlt");
	adf_poll_slicehang_csr(accel_dev, csr,
			       ADF_GEN4_SLICEHANGSTATUS_DCPR_UCS, "dcpr_ucs");
	adf_poll_slicehang_csr(accel_dev, csr,
			       ADF_GEN4_SLICEHANGSTATUS_PKE, "pke");

	if ((*err_mask).parerr_wat_wcp_mask)
		adf_poll_slicehang_csr(accel_dev, csr,
				       ADF_GEN4_SLICEHANGSTATUS_WAT_WCP,
				       "wat_wcp");

	return false;
}

unsafe fn adf_handle_spp_pullcmd_err(*mut adf_accel_devaccel_dev,
				       *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut *reset_required = false;
	let mut reg: u32;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLCMDPARERR_ATH_CPH);
	reg &= (*err_mask).parerr_ath_cph_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull command fatal error ATH_CPH: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLCMDPARERR_ATH_CPH, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLCMDPARERR_CPR_XLT);
	reg &= (*err_mask).parerr_cpr_xlt_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull command fatal error CPR_XLT: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLCMDPARERR_CPR_XLT, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLCMDPARERR_DCPR_UCS);
	reg &= (*err_mask).parerr_dcpr_ucs_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull command fatal error DCPR_UCS: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLCMDPARERR_DCPR_UCS, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLCMDPARERR_PKE);
	reg &= (*err_mask).parerr_pke_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull command fatal error PKE: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLCMDPARERR_PKE, reg);

		*reset_required = true;
	}

	if ((*err_mask).parerr_wat_wcp_mask) {
		reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLCMDPARERR_WAT_WCP);
		reg &= (*err_mask).parerr_wat_wcp_mask;
		if (reg) {
			dev_err(&GET_DEV(accel_dev),
				"SPP pull command fatal error WAT_WCP: 0x%x\n", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

			ADF_CSR_WR(csr, ADF_GEN4_SPPPULLCMDPARERR_WAT_WCP, reg);

			*reset_required = true;
		}
	}

	return *reset_required;
}

unsafe fn adf_handle_spp_pulldata_err(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut reg: u32;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLDATAPARERR_ATH_CPH);
	reg &= (*err_mask).parerr_ath_cph_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull data err ATH_CPH: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLDATAPARERR_ATH_CPH, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLDATAPARERR_CPR_XLT);
	reg &= (*err_mask).parerr_cpr_xlt_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull data err CPR_XLT: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLDATAPARERR_CPR_XLT, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLDATAPARERR_DCPR_UCS);
	reg &= (*err_mask).parerr_dcpr_ucs_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull data err DCPR_UCS: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLDATAPARERR_DCPR_UCS, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLDATAPARERR_PKE);
	reg &= (*err_mask).parerr_pke_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP pull data err PKE: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPULLDATAPARERR_PKE, reg);
	}

	if ((*err_mask).parerr_wat_wcp_mask) {
		reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPULLDATAPARERR_WAT_WCP);
		reg &= (*err_mask).parerr_wat_wcp_mask;
		if (reg) {
			dev_err(&GET_DEV(accel_dev),
				"SPP pull data err WAT_WCP: 0x%x\n", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

			ADF_CSR_WR(csr, ADF_GEN4_SPPPULLDATAPARERR_WAT_WCP, reg);
		}
	}

	return false;
}

unsafe fn adf_handle_spp_pushcmd_err(*mut adf_accel_devaccel_dev,
				       *mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut *reset_required = false;
	let mut reg: u32;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHCMDPARERR_ATH_CPH);
	reg &= (*err_mask).parerr_ath_cph_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push command fatal error ATH_CPH: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHCMDPARERR_ATH_CPH, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHCMDPARERR_CPR_XLT);
	reg &= (*err_mask).parerr_cpr_xlt_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push command fatal error CPR_XLT: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHCMDPARERR_CPR_XLT, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHCMDPARERR_DCPR_UCS);
	reg &= (*err_mask).parerr_dcpr_ucs_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push command fatal error DCPR_UCS: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHCMDPARERR_DCPR_UCS, reg);

		*reset_required = true;
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHCMDPARERR_PKE);
	reg &= (*err_mask).parerr_pke_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push command fatal error PKE: 0x%x\n",
			reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHCMDPARERR_PKE, reg);

		*reset_required = true;
	}

	if ((*err_mask).parerr_wat_wcp_mask) {
		reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHCMDPARERR_WAT_WCP);
		reg &= (*err_mask).parerr_wat_wcp_mask;
		if (reg) {
			dev_err(&GET_DEV(accel_dev),
				"SPP push command fatal error WAT_WCP: 0x%x\n", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

			ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHCMDPARERR_WAT_WCP, reg);

			*reset_required = true;
		}
	}

	return *reset_required;
}

unsafe fn adf_handle_spp_pushdata_err(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr)
{
	*mut adf_dev_err_maskerr_mask = GET_ERR_MASK(accel_dev);
	let mut reg: u32;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHDATAPARERR_ATH_CPH);
	reg &= (*err_mask).parerr_ath_cph_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push data err ATH_CPH: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHDATAPARERR_ATH_CPH, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHDATAPARERR_CPR_XLT);
	reg &= (*err_mask).parerr_cpr_xlt_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push data err CPR_XLT: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHDATAPARERR_CPR_XLT, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHDATAPARERR_DCPR_UCS);
	reg &= (*err_mask).parerr_dcpr_ucs_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push data err DCPR_UCS: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHDATAPARERR_DCPR_UCS, reg);
	}

	reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHDATAPARERR_PKE);
	reg &= (*err_mask).parerr_pke_mask;
	if (reg) {
		dev_err(&GET_DEV(accel_dev),
			"SPP push data err PKE: 0x%x\n", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

		ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHDATAPARERR_PKE, reg);
	}

	if ((*err_mask).parerr_wat_wcp_mask) {
		reg = ADF_CSR_RD(csr, ADF_GEN4_SPPPUSHDATAPARERR_WAT_WCP);
		reg &= (*err_mask).parerr_wat_wcp_mask;
		if (reg) {
			dev_err(&GET_DEV(accel_dev),
				"SPP push data err WAT_WCP: 0x%x\n", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

			ADF_CSR_WR(csr, ADF_GEN4_SPPPUSHDATAPARERR_WAT_WCP,
				   reg);
		}
	}

	return false;
}

unsafe fn adf_handle_spppar_err(*mut adf_accel_devaccel_dev,
				  *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	let mut *reset_required: bool;

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_SPPPARERR_BIT))
		return false;

	*reset_required = adf_handle_spp_pullcmd_err(accel_dev, csr);
	*reset_required |= adf_handle_spp_pulldata_err(accel_dev, csr);
	*reset_required |= adf_handle_spp_pushcmd_err(accel_dev, csr);
	*reset_required |= adf_handle_spp_pushdata_err(accel_dev, csr);

	return *reset_required;
}

unsafe fn adf_handle_ssmcpppar_err(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	u32 reg, bits_num = BITS_PER_REG(reg);
	let mut *reset_required = false;
	usize errs_bits;
	let mut bit_iterator: u32;

	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_SSMCPPERR_BIT))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SSMCPPERR);
	reg &= ADF_GEN4_SSMCPPERR_FATAL_BITMASK | ADF_GEN4_SSMCPPERR_UNCERR_BITMASK;
	if (reg & ADF_GEN4_SSMCPPERR_FATAL_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"Fatal SSM CPP parity error: 0x%x\n", reg);

		errs_bits = reg & ADF_GEN4_SSMCPPERR_FATAL_BITMASK;
		for_each_set_bit(bit_iterator, &errs_bits, bits_num) {
			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);
		}
		*reset_required = true;
	}

	if (reg & ADF_GEN4_SSMCPPERR_UNCERR_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"non-Fatal SSM CPP parity error: 0x%x\n", reg);
		errs_bits = reg & ADF_GEN4_SSMCPPERR_UNCERR_BITMASK;

		for_each_set_bit(bit_iterator, &errs_bits, bits_num) {
			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
		}
	}

	ADF_CSR_WR(csr, ADF_GEN4_SSMCPPERR, reg);

	return *reset_required;
}

unsafe fn adf_handle_rf_parr_err(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	if (!(iastatssm & ADF_GEN4_IAINTSTATSSM_SSMSOFTERRORPARITY_BIT))
		return;

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	dev_err(&GET_DEV(accel_dev), "Slice ssm soft parity error reported");

	return;
}

unsafe fn adf_handle_ser_err_ssmsh(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 iastatssm)
{
	u32 reg, bits_num = BITS_PER_REG(reg);
	let mut *reset_required = false;
	usize errs_bits;
	let mut bit_iterator: u32;

	if (!(iastatssm & (ADF_GEN4_IAINTSTATSSM_SER_ERR_SSMSH_CERR_BIT |
			 ADF_GEN4_IAINTSTATSSM_SER_ERR_SSMSH_UNCERR_BIT)))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_SER_ERR_SSMSH);
	reg &= ADF_GEN4_SER_ERR_SSMSH_FATAL_BITMASK |
	       ADF_GEN4_SER_ERR_SSMSH_UNCERR_BITMASK |
	       ADF_GEN4_SER_ERR_SSMSH_CERR_BITMASK;
	if (reg & ADF_GEN4_SER_ERR_SSMSH_FATAL_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"Fatal SER_SSMSH_ERR: 0x%x\n", reg);

		errs_bits = reg & ADF_GEN4_SER_ERR_SSMSH_FATAL_BITMASK;
		for_each_set_bit(bit_iterator, &errs_bits, bits_num) {
			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);
		}

		*reset_required = true;
	}

	if (reg & ADF_GEN4_SER_ERR_SSMSH_UNCERR_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"non-fatal SER_SSMSH_ERR: 0x%x\n", reg);

		errs_bits = reg & ADF_GEN4_SER_ERR_SSMSH_UNCERR_BITMASK;
		for_each_set_bit(bit_iterator, &errs_bits, bits_num) {
			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
		}
	}

	if (reg & ADF_GEN4_SER_ERR_SSMSH_CERR_BITMASK) {
		dev_warn(&GET_DEV(accel_dev),
			 "Correctable SER_SSMSH_ERR: 0x%x\n", reg);

		errs_bits = reg & ADF_GEN4_SER_ERR_SSMSH_CERR_BITMASK;
		for_each_set_bit(bit_iterator, &errs_bits, bits_num) {
			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);
		}
	}

	ADF_CSR_WR(csr, ADF_GEN4_SER_ERR_SSMSH, reg);

	return *reset_required;
}

unsafe fn adf_handle_iaintstatssm(*mut adf_accel_devaccel_dev,
				    *mut core::ffi::c_voidcsr)
{
	let mut iastatssm: u32 = ADF_CSR_RD(csr, ADF_GEN4_IAINTSTATSSM);
	let mut *reset_required: bool;

	iastatssm &= ADF_GEN4_IAINTSTATSSM_BITMASK;
	if (!iastatssm)
		return false;

	*reset_required = adf_handle_uerrssmsh(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_cerrssmsh(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_pperr_err(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_slice_hang_error(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_spppar_err(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_ssmcpppar_err(accel_dev, csr, iastatssm);
	*reset_required |= adf_handle_ser_err_ssmsh(accel_dev, csr, iastatssm);
	adf_handle_rf_parr_err(accel_dev, csr, iastatssm);

	ADF_CSR_WR(csr, ADF_GEN4_IAINTSTATSSM, iastatssm);

	return *reset_required;
}

unsafe fn adf_handle_exprpssmcmpr(*mut adf_accel_devaccel_dev,
				    *mut core::ffi::c_voidcsr)
{
	let mut reg: u32 = ADF_CSR_RD(csr, ADF_GEN4_EXPRPSSMCPR);

	reg &= ADF_GEN4_EXPRPSSMCPR_UNCERR_BITMASK;
	if (!reg)
		return false;

	dev_err(&GET_DEV(accel_dev),
		"Uncorrectable error exception in SSM CMP: 0x%x", reg);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_EXPRPSSMCPR, reg);

	return false;
}

unsafe fn adf_handle_exprpssmxlt(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr)
{
	let mut reg: u32 = ADF_CSR_RD(csr, ADF_GEN4_EXPRPSSMXLT);

	reg &= ADF_GEN4_EXPRPSSMXLT_UNCERR_BITMASK |
	       ADF_GEN4_EXPRPSSMXLT_CERR_BIT;
	if (!reg)
		return false;

	if (reg & ADF_GEN4_EXPRPSSMXLT_UNCERR_BITMASK) {
		dev_err(&GET_DEV(accel_dev),
			"Uncorrectable error exception in SSM XLT: 0x%x", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	if (reg & ADF_GEN4_EXPRPSSMXLT_CERR_BIT) {
		dev_warn(&GET_DEV(accel_dev),
			 "Correctable error exception in SSM XLT: 0x%x", reg);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);
	}

	ADF_CSR_WR(csr, ADF_GEN4_EXPRPSSMXLT, reg);

	return false;
}

unsafe fn adf_handle_exprpssmdcpr(*mut adf_accel_devaccel_dev,
				    *mut core::ffi::c_voidcsr)
{
	let mut reg: u32;
	i32 i;

	for i in 0..(ADF_GEN4_DCPR_SLICES_NUM as i32) {
		reg = ADF_CSR_RD(csr, ADF_GEN4_EXPRPSSMDCPR(i));
		reg &= ADF_GEN4_EXPRPSSMDCPR_UNCERR_BITMASK |
		       ADF_GEN4_EXPRPSSMDCPR_CERR_BITMASK;
		if (!reg)
			continue;

		if (reg & ADF_GEN4_EXPRPSSMDCPR_UNCERR_BITMASK) {
			dev_err(&GET_DEV(accel_dev),
				"Uncorrectable error exception in SSM DCMP: 0x%x", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
		}

		if (reg & ADF_GEN4_EXPRPSSMDCPR_CERR_BITMASK) {
			dev_warn(&GET_DEV(accel_dev),
				 "Correctable error exception in SSM DCMP: 0x%x", reg);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);
		}

		ADF_CSR_WR(csr, ADF_GEN4_EXPRPSSMDCPR(i), reg);
	}

	return false;
}

unsafe fn adf_handle_ssm(*mut adf_accel_devaccel_dev, *mut core::ffi::c_voidcsr,
			   u32 errsou)
{
	let mut *reset_required: bool;

	if (!(errsou & ADF_GEN4_ERRSOU2_SSM_ERR_BIT))
		return false;

	*reset_required = adf_handle_iaintstatssm(accel_dev, csr);
	*reset_required |= adf_handle_exprpssmcmpr(accel_dev, csr);
	*reset_required |= adf_handle_exprpssmxlt(accel_dev, csr);
	*reset_required |= adf_handle_exprpssmdcpr(accel_dev, csr);

	return *reset_required;
}

unsafe fn adf_handle_cpp_cfc_err(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut *reset_required = false;
	let mut reg: u32;

	if (!(errsou & ADF_GEN4_ERRSOU2_CPP_CFC_ERR_STATUS_BIT))
		return false;

	reg = ADF_CSR_RD(csr, ADF_GEN4_CPP_CFC_ERR_STATUS);
	if (reg & ADF_GEN4_CPP_CFC_ERR_STATUS_DATAPAR_BIT) {
		dev_err(&GET_DEV(accel_dev),
			"CPP_CFC_ERR: data parity: 0x%x", reg);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	if (reg & ADF_GEN4_CPP_CFC_ERR_STATUS_CMDPAR_BIT) {
		dev_err(&GET_DEV(accel_dev),
			"CPP_CFC_ERR: command parity: 0x%x", reg);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		*reset_required = true;
	}

	if (reg & ADF_GEN4_CPP_CFC_ERR_STATUS_MERR_BIT) {
		dev_err(&GET_DEV(accel_dev),
			"CPP_CFC_ERR: multiple errors: 0x%x", reg);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		*reset_required = true;
	}

	ADF_CSR_WR(csr, ADF_GEN4_CPP_CFC_ERR_STATUS_CLR,
		   ADF_GEN4_CPP_CFC_ERR_STATUS_CLR_BITMASK);

	return *reset_required;
}

unsafe fn adf_gen4_process_errsou2(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, u32 errsou,
				     bool **reset_required)
{
	**reset_required |= adf_handle_ssm(accel_dev, csr, errsou);
	**reset_required |= adf_handle_cpp_cfc_err(accel_dev, csr, errsou);
}

unsafe fn adf_handle_timiscsts(*mut adf_accel_devaccel_dev,
				 *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut timiscsts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_TIMISCSTS_BIT))
		return false;

	timiscsts = ADF_CSR_RD(csr, ADF_GEN4_TIMISCSTS);

	dev_err(&GET_DEV(accel_dev),
		"Fatal error in Transmit Interface: 0x%x\n", timiscsts);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

	return true;
}

unsafe fn adf_handle_ricppintsts(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ricppintsts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_RICPPINTSTS_BITMASK))
		return false;

	ricppintsts = ADF_CSR_RD(csr, ADF_GEN4_RICPPINTSTS);
	ricppintsts &= ADF_GEN4_RICPPINTSTS_BITMASK;

	dev_err(&GET_DEV(accel_dev),
		"RI CPP Uncorrectable Error: 0x%x\n", ricppintsts);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_RICPPINTSTS, ricppintsts);

	return false;
}

unsafe fn adf_handle_ticppintsts(*mut adf_accel_devaccel_dev,
				   *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut ticppintsts: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_TICPPINTSTS_BITMASK))
		return false;

	ticppintsts = ADF_CSR_RD(csr, ADF_GEN4_TICPPINTSTS);
	ticppintsts &= ADF_GEN4_TICPPINTSTS_BITMASK;

	dev_err(&GET_DEV(accel_dev),
		"TI CPP Uncorrectable Error: 0x%x\n", ticppintsts);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

	ADF_CSR_WR(csr, ADF_GEN4_TICPPINTSTS, ticppintsts);

	return false;
}

unsafe fn adf_handle_aramcerr(*mut adf_accel_devaccel_dev,
				*mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut aram_cerr: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_REG_ARAMCERR_BIT))
		return false;

	aram_cerr = ADF_CSR_RD(csr, ADF_GEN4_REG_ARAMCERR);
	aram_cerr &= ADF_GEN4_REG_ARAMCERR_BIT;

	dev_warn(&GET_DEV(accel_dev),
		 "ARAM correctable error : 0x%x\n", aram_cerr);

	ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_CORR);

	aram_cerr |= ADF_GEN4_REG_ARAMCERR_EN_BITMASK;

	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMCERR, aram_cerr);

	return false;
}

unsafe fn adf_handle_aramuerr(*mut adf_accel_devaccel_dev,
				*mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut *reset_required = false;
	let mut aramuerr: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_REG_ARAMUERR_BIT))
		return false;

	aramuerr = ADF_CSR_RD(csr, ADF_GEN4_REG_ARAMUERR);
	aramuerr &= ADF_GEN4_REG_ARAMUERR_ERROR_BIT |
		    ADF_GEN4_REG_ARAMUERR_MULTI_ERRORS_BIT;

	if (!aramuerr)
		return false;

	if (aramuerr & ADF_GEN4_REG_ARAMUERR_MULTI_ERRORS_BIT) {
		dev_err(&GET_DEV(accel_dev),
			"ARAM multiple uncorrectable errors: 0x%x\n", aramuerr);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		*reset_required = true;
	} else {
		dev_err(&GET_DEV(accel_dev),
			"ARAM uncorrectable error: 0x%x\n", aramuerr);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	aramuerr |= ADF_GEN4_REG_ARAMUERR_EN_BITMASK;

	ADF_CSR_WR(csr, ADF_GEN4_REG_ARAMUERR, aramuerr);

	return *reset_required;
}

unsafe fn adf_handle_reg_cppmemtgterr(*mut adf_accel_devaccel_dev,
					*mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut *reset_required = false;
	let mut cppmemtgterr: u32;

	if (!(errsou & ADF_GEN4_ERRSOU3_REG_ARAMUERR_BIT))
		return false;

	cppmemtgterr = ADF_CSR_RD(csr, ADF_GEN4_REG_CPPMEMTGTERR);
	cppmemtgterr &= ADF_GEN4_REG_CPPMEMTGTERR_BITMASK |
			ADF_GEN4_REG_CPPMEMTGTERR_MULTI_ERRORS_BIT;
	if (!cppmemtgterr)
		return false;

	if (cppmemtgterr & ADF_GEN4_REG_CPPMEMTGTERR_MULTI_ERRORS_BIT) {
		dev_err(&GET_DEV(accel_dev),
			"Misc memory target multiple uncorrectable errors: 0x%x\n",
			cppmemtgterr);

		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_FATAL);

		*reset_required = true;
	} else {
		dev_err(&GET_DEV(accel_dev),
			"Misc memory target uncorrectable error: 0x%x\n", cppmemtgterr);
		ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);
	}

	cppmemtgterr |= ADF_GEN4_REG_CPPMEMTGTERR_EN_BITMASK;

	ADF_CSR_WR(csr, ADF_GEN4_REG_CPPMEMTGTERR, cppmemtgterr);

	return *reset_required;
}

unsafe fn adf_handle_atufaultstatus(*mut adf_accel_devaccel_dev,
				      *mut core::ffi::c_voidcsr, u32 errsou)
{
	let mut i: u32;
	let mut max_rp_num: u32 = GET_HW_DATA(accel_dev)->num_banks;

	if (!(errsou & ADF_GEN4_ERRSOU3_ATUFAULTSTATUS_BIT))
		return false;

	for i in 0..(max_rp_num as i32) {
		let mut atufaultstatus: u32 = ADF_CSR_RD(csr, ADF_GEN4_ATUFAULTSTATUS(i));

		atufaultstatus &= ADF_GEN4_ATUFAULTSTATUS_BIT;

		if (atufaultstatus) {
			dev_err(&GET_DEV(accel_dev),
				"Ring Pair (%u) ATU detected fault: 0x%x\n", i,
				atufaultstatus);

			ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors, ADF_RAS_UNCORR);

			ADF_CSR_WR(csr, ADF_GEN4_ATUFAULTSTATUS(i), atufaultstatus);
		}
	}

	return false;
}

unsafe fn adf_gen4_process_errsou3(*mut adf_accel_devaccel_dev,
				     *mut core::ffi::c_voidcsr, *mut core::ffi::c_voidaram_csr,
				     u32 errsou, bool **reset_required)
{
	**reset_required |= adf_handle_timiscsts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ricppintsts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_ticppintsts(accel_dev, csr, errsou);
	**reset_required |= adf_handle_aramcerr(accel_dev, aram_csr, errsou);
	**reset_required |= adf_handle_aramuerr(accel_dev, aram_csr, errsou);
	**reset_required |= adf_handle_reg_cppmemtgterr(accel_dev, aram_csr, errsou);
	**reset_required |= adf_handle_atufaultstatus(accel_dev, csr, errsou);
}

unsafe fn adf_gen4_handle_interrupt(*mut adf_accel_devaccel_dev,
				      bool **reset_required)
{
	*mut core::ffi::c_voidaram_csr = adf_get_aram_base(accel_dev);
	*mut core::ffi::c_voidcsr = adf_get_pmisc_base(accel_dev);
	let mut errsou: u32 = ADF_CSR_RD(csr, ADF_GEN4_ERRSOU0);
	let mut handled = false;

	**reset_required = false;

	if (errsou & ADF_GEN4_ERRSOU0_BIT) {
		adf_gen4_process_errsou0(accel_dev, csr);
		handled = true;
	}

	errsou = ADF_CSR_RD(csr, ADF_GEN4_ERRSOU1);
	if (errsou & ADF_GEN4_ERRSOU1_BITMASK) {
		adf_gen4_process_errsou1(accel_dev, csr, errsou, *reset_required);
		handled = true;
	}

	errsou = ADF_CSR_RD(csr, ADF_GEN4_ERRSOU2);
	if (errsou & ADF_GEN4_ERRSOU2_BITMASK) {
		adf_gen4_process_errsou2(accel_dev, csr, errsou, *reset_required);
		handled = true;
	}

	errsou = ADF_CSR_RD(csr, ADF_GEN4_ERRSOU3);
	if (errsou & ADF_GEN4_ERRSOU3_BITMASK) {
		adf_gen4_process_errsou3(accel_dev, csr, aram_csr, errsou, *reset_required);
		handled = true;
	}

	return handled;
}

pub unsafe fn adf_gen4_init_ras_ops(*mut adf_ras_opsras_ops)
{
	(*ras_ops).enable_ras_errors = adf_gen4_enable_ras;
	(*ras_ops).disable_ras_errors = adf_gen4_disable_ras;
	(*ras_ops).handle_interrupt = adf_gen4_handle_interrupt;
}
EXPORT_SYMBOL_GPL(adf_gen4_init_ras_ops);



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
