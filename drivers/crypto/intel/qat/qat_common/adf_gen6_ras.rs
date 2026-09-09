// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */
// Linux headers and project headers are supplied by the surrounding translation unit.

unsafe fn enable_errsou_reporting(csr: *mut core::ffi::c_void) {
    ADF_CSR_WR(csr, ADF_GEN6_ERRMSK0, 0);
    ADF_CSR_WR(csr, ADF_GEN6_ERRMSK1, 0);
    ADF_CSR_WR(csr, ADF_GEN6_ERRMSK2, ADF_GEN6_ERRSOU2_PM_INT_BIT);
    ADF_CSR_WR(csr, ADF_GEN6_ERRMSK3, 0);
}
unsafe fn enable_ae_error_reporting(accel_dev: *mut adf_accel_dev, csr: *mut core::ffi::c_void) {
    let ae_mask: u32 = (*GET_HW_DATA(accel_dev)).ae_mask;
    ADF_CSR_WR(csr, ADF_GEN6_HIAECORERRLOGENABLE_CPP0, ae_mask);
    ADF_CSR_WR(csr, ADF_GEN6_HIAEUNCERRLOGENABLE_CPP0, ae_mask);
}
unsafe fn enable_cpp_error_reporting(_accel_dev: *mut adf_accel_dev, csr: *mut core::ffi::c_void) {
    ADF_CSR_WR(csr, ADF_GEN6_HICPPAGENTCMDPARERRLOGENABLE, ADF_6XXX_HICPPAGENTCMDPARERRLOG_MASK);
    ADF_CSR_WR(csr, ADF_GEN6_CPP_CFC_ERR_CTRL, ADF_GEN6_CPP_CFC_ERR_CTRL_MASK);
}
unsafe fn enable_ti_ri_error_reporting(csr: *mut core::ffi::c_void) {
    let mut reg: u32;
    let mask: u32 = ADF_GEN6_RIMEM_PARERR_FATAL_MASK | ADF_GEN6_RIMEM_PARERR_CERR_MASK;
    ADF_CSR_WR(csr, ADF_GEN6_RI_MEM_PAR_ERR_EN0, mask);
    ADF_CSR_WR(csr, ADF_GEN6_RIMISCCTL, ADF_GEN6_RIMISCSTS_BIT);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TI_CI_PAR_ERR_MASK); reg &= !ADF_GEN6_TI_CI_PAR_STS_MASK; ADF_CSR_WR(csr, ADF_GEN6_TI_CI_PAR_ERR_MASK, reg);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TI_PULL0FUB_PAR_ERR_MASK); reg &= !ADF_GEN6_TI_PULL0FUB_PAR_STS_MASK; ADF_CSR_WR(csr, ADF_GEN6_TI_PULL0FUB_PAR_ERR_MASK, reg);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TI_PUSHFUB_PAR_ERR_MASK); reg &= !ADF_GEN6_TI_PUSHFUB_PAR_STS_MASK; ADF_CSR_WR(csr, ADF_GEN6_TI_PUSHFUB_PAR_ERR_MASK, reg);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TI_CD_PAR_ERR_MASK); reg &= !ADF_GEN6_TI_CD_PAR_STS_MASK; ADF_CSR_WR(csr, ADF_GEN6_TI_CD_PAR_ERR_MASK, reg);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TI_TRNSB_PAR_ERR_MASK); reg &= !ADF_GEN6_TI_TRNSB_PAR_STS_MASK; ADF_CSR_WR(csr, ADF_GEN6_TI_TRNSB_PAR_ERR_MASK, reg);
    ADF_CSR_WR(csr, ADF_GEN6_RICPPINTCTL, ADF_GEN6_RICPPINTCTL_MASK);
    ADF_CSR_WR(csr, ADF_GEN6_TICPPINTCTL, ADF_GEN6_TICPPINTCTL_MASK);
    reg = ADF_CSR_RD(csr, ADF_GEN6_TIMISCCTL); reg &= ADF_GEN6_TIMSCCTL_RELAY_MASK; reg |= ADF_GEN6_TIMISCCTL_BIT; ADF_CSR_WR(csr, ADF_GEN6_TIMISCCTL, reg);
}
unsafe fn enable_ssm_error_reporting(_accel_dev: *mut adf_accel_dev, csr: *mut core::ffi::c_void) { ADF_CSR_WR(csr, ADF_GEN6_INTMASKSSM, 0); }
unsafe fn adf_gen6_enable_ras(accel_dev: *mut adf_accel_dev) { let csr = adf_get_pmisc_base(accel_dev); enable_errsou_reporting(csr); enable_ae_error_reporting(accel_dev, csr); enable_cpp_error_reporting(accel_dev, csr); enable_ti_ri_error_reporting(csr); enable_ssm_error_reporting(accel_dev, csr); }

unsafe fn disable_errsou_reporting(csr: *mut core::ffi::c_void) { let mut val: u32; ADF_CSR_WR(csr, ADF_GEN6_ERRMSK0, ADF_GEN6_ERRSOU0_MASK); ADF_CSR_WR(csr, ADF_GEN6_ERRMSK1, ADF_GEN6_ERRMSK1_MASK); val=ADF_CSR_RD(csr, ADF_GEN6_ERRMSK2); val|=ADF_GEN6_ERRSOU2_DIS_MASK; ADF_CSR_WR(csr,ADF_GEN6_ERRMSK2,val); ADF_CSR_WR(csr,ADF_GEN6_ERRMSK3,ADF_GEN6_ERRSOU3_DIS_MASK); }
unsafe fn disable_ae_error_reporting(csr: *mut core::ffi::c_void) { ADF_CSR_WR(csr, ADF_GEN6_HIAECORERRLOGENABLE_CPP0,0); ADF_CSR_WR(csr,ADF_GEN6_HIAEUNCERRLOGENABLE_CPP0,0); }
unsafe fn disable_cpp_error_reporting(csr: *mut core::ffi::c_void) { ADF_CSR_WR(csr,ADF_GEN6_HICPPAGENTCMDPARERRLOGENABLE,0); ADF_CSR_WR(csr,ADF_GEN6_CPP_CFC_ERR_CTRL,ADF_GEN6_CPP_CFC_ERR_CTRL_DIS_MASK); }
unsafe fn disable_ti_ri_error_reporting(csr: *mut core::ffi::c_void) { let mut reg: u32; ADF_CSR_WR(csr,ADF_GEN6_RI_MEM_PAR_ERR_EN0,0); reg=ADF_CSR_RD(csr,ADF_GEN6_RIMISCCTL); reg&=!ADF_GEN6_RIMISCSTS_BIT; ADF_CSR_WR(csr,ADF_GEN6_RIMISCCTL,reg); ADF_CSR_WR(csr,ADF_GEN6_TI_CI_PAR_ERR_MASK,ADF_GEN6_TI_CI_PAR_STS_MASK); ADF_CSR_WR(csr,ADF_GEN6_TI_PULL0FUB_PAR_ERR_MASK,ADF_GEN6_TI_PULL0FUB_PAR_STS_MASK); ADF_CSR_WR(csr,ADF_GEN6_TI_PUSHFUB_PAR_ERR_MASK,ADF_GEN6_TI_PUSHFUB_PAR_STS_MASK); ADF_CSR_WR(csr,ADF_GEN6_TI_CD_PAR_ERR_MASK,ADF_GEN6_TI_CD_PAR_STS_MASK); ADF_CSR_WR(csr,ADF_GEN6_TI_TRNSB_PAR_ERR_MASK,ADF_GEN6_TI_TRNSB_PAR_STS_MASK); reg=ADF_CSR_RD(csr,ADF_GEN6_RICPPINTCTL); reg&=!ADF_GEN6_RICPPINTCTL_MASK; ADF_CSR_WR(csr,ADF_GEN6_RICPPINTCTL,reg); reg=ADF_CSR_RD(csr,ADF_GEN6_TICPPINTCTL); reg&=!ADF_GEN6_TICPPINTCTL_MASK; ADF_CSR_WR(csr,ADF_GEN6_TICPPINTCTL,reg); reg=ADF_CSR_RD(csr,ADF_GEN6_TIMISCCTL); reg&=ADF_GEN6_TIMSCCTL_RELAY_MASK; ADF_CSR_WR(csr,ADF_GEN6_TIMISCCTL,reg); }
unsafe fn disable_ssm_error_reporting(csr: *mut core::ffi::c_void) { ADF_CSR_WR(csr,ADF_GEN6_INTMASKSSM,ADF_GEN6_INTMASKSSM_MASK); }
unsafe fn adf_gen6_disable_ras(accel_dev: *mut adf_accel_dev) { let csr=adf_get_pmisc_base(accel_dev); disable_errsou_reporting(csr); disable_ae_error_reporting(csr); disable_cpp_error_reporting(csr); disable_ti_ri_error_reporting(csr); disable_ssm_error_reporting(csr); }

unsafe fn adf_gen6_process_errsou0(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void) { let mut ae=ADF_CSR_RD(csr,ADF_GEN6_HIAECORERRLOG_CPP0); ae&=(*GET_HW_DATA(accel_dev)).ae_mask; dev_warn(&GET_DEV(accel_dev),"Correctable error detected: %#x\n",ae); ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_CORR); ADF_CSR_WR(csr,ADF_GEN6_HIAECORERRLOG_CPP0,ae); let errsou=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU0); if errsou&ADF_GEN6_ERRSOU0_MASK!=0 { dev_warn(&GET_DEV(accel_dev),"errsou0 still set: %#x\n",errsou); } }

unsafe fn handle_masked(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void, status:u32, regno: u32, mask:u32, level: u32, msg:&'static str) { if status&mask!=0 { dev_err(&GET_DEV(accel_dev),msg,status); ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,level); ADF_CSR_WR(csr,regno,status); } }
unsafe fn adf_gen6_handle_interrupt(accel_dev:*mut adf_accel_dev, reset_required:*mut bool)->bool { let csr=adf_get_pmisc_base(accel_dev); let mut handled=false; let mut e=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU0); if e&ADF_GEN6_ERRSOU0_MASK!=0 { adf_gen6_process_errsou0(accel_dev,csr); handled=true; } e=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU1); if e&ADF_GEN6_ERRSOU1_MASK!=0 { adf_gen6_process_errsou1(accel_dev,csr,e); handled=true; } e=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU2); if e&ADF_GEN6_ERRSOU2_MASK!=0 { adf_gen6_process_errsou2(accel_dev,csr,e); handled=true; } e=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU3); if e&ADF_GEN6_ERRSOU3_MASK!=0 { adf_gen6_process_errsou3(accel_dev,csr,e); handled=true; } adf_gen6_is_reset_required(accel_dev,csr,reset_required); handled }

unsafe fn adf_gen6_is_reset_required(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void, reset_required:*mut bool) { let gensts=ADF_CSR_RD(csr,ADF_GEN6_GENSTS); let dev_state=FIELD_GET(ADF_GEN6_GENSTS_DEVICE_STATE_MASK,gensts); let reset=FIELD_GET(ADF_GEN6_GENSTS_RESET_TYPE_MASK,gensts); if dev_state==ADF_GEN6_GENSTS_DEVHALT && reset==ADF_GEN6_GENSTS_PFLR { *reset_required=true; return; } if reset==ADF_GEN6_GENSTS_COLD_RESET { dev_err(&GET_DEV(accel_dev),"Fatal error, cold reset required\n"); } *reset_required=false; }

unsafe fn adf_gen6_process_errsou1(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void,errsou:u32) {
    if errsou & ADF_GEN6_ERRSOU1_CPP0_MEUNC_BIT != 0 { let mut v=ADF_CSR_RD(csr,ADF_GEN6_HIAEUNCERRLOG_CPP0)&(*GET_HW_DATA(accel_dev)).ae_mask; if v!=0 { dev_err(&GET_DEV(accel_dev),"Uncorrectable error detected: %#x\n",v); ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_UNCORR); ADF_CSR_WR(csr,ADF_GEN6_HIAEUNCERRLOG_CPP0,v); } }
    if errsou & ADF_GEN6_ERRSOU1_CPP_CMDPARERR_BIT != 0 { let v=ADF_CSR_RD(csr,ADF_GEN6_HICPPAGENTCMDPARERRLOG)&ADF_6XXX_HICPPAGENTCMDPARERRLOG_MASK; if v!=0 { dev_err(&GET_DEV(accel_dev),"HI CPP agent command parity error: %#x\n",v); ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_FATAL); ADF_CSR_WR(csr,ADF_GEN6_HICPPAGENTCMDPARERRLOG,v); } }
    if errsou & ADF_GEN6_ERRSOU1_RIMEM_PARERR_STS_BIT != 0 { let v=ADF_CSR_RD(csr,ADF_GEN6_RIMEM_PARERR_STS)&(ADF_GEN6_RIMEM_PARERR_CERR_MASK|ADF_GEN6_RIMEM_PARERR_FATAL_MASK); if v&ADF_GEN6_RIMEM_PARERR_CERR_MASK!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_CORR); } if v&ADF_GEN6_RIMEM_PARERR_FATAL_MASK!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_FATAL); } ADF_CSR_WR(csr,ADF_GEN6_RIMEM_PARERR_STS,v); }
    let v=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU1); if v&ADF_GEN6_ERRSOU1_MASK!=0 { dev_warn(&GET_DEV(accel_dev),"errsou1 still set: %#x\n",v); }
}
unsafe fn adf_gen6_process_errsou2(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void,errsou:u32) { if errsou&ADF_GEN6_ERRSOU2_CPP_CFC_ERR_STATUS_BIT!=0 { let v=ADF_CSR_RD(csr,ADF_GEN6_CPP_CFC_ERR_STATUS); if v&ADF_GEN6_CPP_CFC_ERR_STATUS_DATAPAR_BIT!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_UNCORR); } if v&(ADF_GEN6_CPP_CFC_ERR_STATUS_CMDPAR_BIT|ADF_GEN6_CPP_CFC_FATAL_ERR_BIT)!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_FATAL); } ADF_CSR_WR(csr,ADF_GEN6_CPP_CFC_ERR_STATUS_CLR,ADF_GEN6_CPP_CFC_ERR_STATUS_CLR_MASK); } let v=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU2); if v&ADF_GEN6_ERRSOU2_MASK!=0 { dev_warn(&GET_DEV(accel_dev),"errsou2 still set: %#x\n",v); } }
unsafe fn adf_gen6_process_errsou3(accel_dev:*mut adf_accel_dev,csr:*mut core::ffi::c_void,errsou:u32) { if errsou&ADF_GEN6_ERRSOU3_TIMISCSTS_BIT!=0 { let v=ADF_CSR_RD(csr,ADF_GEN6_TIMISCSTS); if v!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_FATAL); } } if errsou&ADF_GEN6_ERRSOU3_RICPPINTSTS_MASK!=0 { let v=ADF_CSR_RD(csr,ADF_GEN6_RICPPINTSTS)&ADF_GEN6_RICPPINTSTS_MASK; if v!=0 { ADF_RAS_ERR_CTR_INC((*accel_dev).ras_errors,ADF_RAS_UNCORR); ADF_CSR_WR(csr,ADF_GEN6_RICPPINTSTS,v); } } let v=ADF_CSR_RD(csr,ADF_GEN6_ERRSOU3); if v&ADF_GEN6_ERRSOU3_MASK!=0 { dev_warn(&GET_DEV(accel_dev),"errsou3 still set: %#x\n",v); } }

pub unsafe fn adf_gen6_init_ras_ops(ras_ops:*mut adf_ras_ops) { (*ras_ops).enable_ras_errors=Some(adf_gen6_enable_ras); (*ras_ops).disable_ras_errors=Some(adf_gen6_disable_ras); (*ras_ops).handle_interrupt=Some(adf_gen6_handle_interrupt); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
