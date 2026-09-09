// SPDX-License-Identifier: GPL-2.0
// Translated from nitrox_hal.c. Kernel dependencies and register definitions
// are supplied by the surrounding crate.

const PLL_REF_CLK: u64 = 50;
const MAX_CSR_RETRIES: i32 = 10;

unsafe fn emu_enable_cores(ndev: *mut nitrox_device) {
    let mut emu_se: emu_se_enable = core::mem::zeroed();
    let mut emu_ae: emu_ae_enable = core::mem::zeroed();
    emu_ae.value = 0;
    emu_ae.s.enable = 0xfffff;
    emu_se.value = 0;
    emu_se.s.enable = 0xffff;
    for i in 0..NR_CLUSTERS {
        nitrox_write_csr(ndev, EMU_AE_ENABLEX(i), emu_ae.value);
        nitrox_write_csr(ndev, EMU_SE_ENABLEX(i), emu_se.value);
    }
}

pub unsafe fn nitrox_config_emu_unit(ndev: *mut nitrox_device) {
    let mut emu_wd_int: emu_wd_int_ena_w1s = core::mem::zeroed();
    let mut emu_ge_int: emu_ge_int_ena_w1s = core::mem::zeroed();
    emu_enable_cores(ndev);
    emu_ge_int.value = 0;
    emu_ge_int.s.se_ge = 0xffff;
    emu_ge_int.s.ae_ge = 0xfffff;
    emu_wd_int.value = 0;
    emu_wd_int.s.se_wd = 1;
    for i in 0..NR_CLUSTERS {
        nitrox_write_csr(ndev, EMU_WD_INT_ENA_W1SX(i), emu_wd_int.value);
        nitrox_write_csr(ndev, EMU_GE_INT_ENA_W1SX(i), emu_ge_int.value);
    }
}

unsafe fn reset_pkt_input_ring(ndev: *mut nitrox_device, ring: i32) {
    let mut ctl: nps_pkt_in_instr_ctl = core::mem::zeroed();
    let mut cnts: nps_pkt_in_done_cnts = core::mem::zeroed();
    let mut retries = MAX_CSR_RETRIES;
    let offset = NPS_PKT_IN_INSTR_CTLX(ring);
    ctl.value = nitrox_read_csr(ndev, offset);
    ctl.s.enb = 0;
    nitrox_write_csr(ndev, offset, ctl.value);
    usleep_range(100, 150);
    loop {
        ctl.value = nitrox_read_csr(ndev, offset);
        if ctl.s.enb == 0 { break; }
        udelay(50);
        retries -= 1;
        if retries < 0 { break; }
    }
    let offset = NPS_PKT_IN_DONE_CNTSX(ring);
    cnts.value = nitrox_read_csr(ndev, offset);
    nitrox_write_csr(ndev, offset, cnts.value);
    usleep_range(50, 100);
}

pub unsafe fn enable_pkt_input_ring(ndev: *mut nitrox_device, ring: i32) {
    let mut ctl: nps_pkt_in_instr_ctl = core::mem::zeroed();
    let mut retries = MAX_CSR_RETRIES;
    let offset = NPS_PKT_IN_INSTR_CTLX(ring);
    ctl.value = nitrox_read_csr(ndev, offset);
    ctl.s.is64b = 1; ctl.s.enb = 1;
    nitrox_write_csr(ndev, offset, ctl.value);
    loop {
        ctl.value = nitrox_read_csr(ndev, offset);
        if ctl.s.enb != 0 { break; }
        udelay(50); retries -= 1; if retries < 0 { break; }
    }
}

pub unsafe fn nitrox_config_pkt_input_rings(ndev: *mut nitrox_device) {
    for i in 0..(*ndev).nr_queues {
        let cmdq = &(*ndev).pkt_inq[i as usize];
        let mut size: nps_pkt_in_instr_rsize = core::mem::zeroed();
        let mut dbell: nps_pkt_in_instr_baoff_dbell = core::mem::zeroed();
        reset_pkt_input_ring(ndev, i);
        nitrox_write_csr(ndev, NPS_PKT_IN_INSTR_BADDRX(i), cmdq.dma);
        size.value = 0; size.s.rsize = (*ndev).qlen;
        nitrox_write_csr(ndev, NPS_PKT_IN_INSTR_RSIZEX(i), size.value);
        nitrox_write_csr(ndev, NPS_PKT_IN_INT_LEVELSX(i), 0xffffffff);
        dbell.value = 0; dbell.s.dbell = 0xffffffff;
        nitrox_write_csr(ndev, NPS_PKT_IN_INSTR_BAOFF_DBELLX(i), dbell.value);
        enable_pkt_input_ring(ndev, i);
    }
}

unsafe fn reset_pkt_solicit_port(ndev: *mut nitrox_device, port: i32) {
    let mut ctl: nps_pkt_slc_ctl = core::mem::zeroed();
    let mut cnts: nps_pkt_slc_cnts = core::mem::zeroed();
    let mut retries = MAX_CSR_RETRIES;
    let offset = NPS_PKT_SLC_CTLX(port);
    ctl.value = nitrox_read_csr(ndev, offset); ctl.s.enb = 0;
    nitrox_write_csr(ndev, offset, ctl.value); usleep_range(100, 150);
    loop { ctl.value = nitrox_read_csr(ndev, offset); if ctl.s.enb == 0 { break; } udelay(50); retries -= 1; if retries < 0 { break; } }
    let offset = NPS_PKT_SLC_CNTSX(port); cnts.value = nitrox_read_csr(ndev, offset);
    nitrox_write_csr(ndev, offset, cnts.value); usleep_range(50, 100);
}

pub unsafe fn enable_pkt_solicit_port(ndev: *mut nitrox_device, port: i32) {
    let mut ctl: nps_pkt_slc_ctl = core::mem::zeroed(); let mut retries = MAX_CSR_RETRIES;
    let offset = NPS_PKT_SLC_CTLX(port); ctl.value = 0; ctl.s.enb = 1; ctl.s.z = 1; ctl.s.rh = 1;
    nitrox_write_csr(ndev, offset, ctl.value);
    loop { ctl.value = nitrox_read_csr(ndev, offset); if ctl.s.enb != 0 { break; } udelay(50); retries -= 1; if retries < 0 { break; } }
}

unsafe fn config_pkt_solicit_port(ndev: *mut nitrox_device, port: i32) {
    let mut int_levels: nps_pkt_slc_int_levels = core::mem::zeroed(); reset_pkt_solicit_port(ndev, port);
    int_levels.value = 0; int_levels.s.timet = 0x3fffff;
    nitrox_write_csr(ndev, NPS_PKT_SLC_INT_LEVELSX(port), int_levels.value); enable_pkt_solicit_port(ndev, port);
}

pub unsafe fn nitrox_config_pkt_solicit_ports(ndev: *mut nitrox_device) { for i in 0..(*ndev).nr_queues { config_pkt_solicit_port(ndev, i); } }

unsafe fn enable_nps_core_interrupts(ndev: *mut nitrox_device) {
    let mut x: nps_core_int_ena_w1s = core::mem::zeroed(); x.value = 0;
    x.s.host_wr_err=1; x.s.host_wr_timeout=1; x.s.exec_wr_timeout=1; x.s.npco_dma_malform=1; x.s.host_nps_wr_err=1;
    nitrox_write_csr(ndev, NPS_CORE_INT_ENA_W1S, x.value);
}
pub unsafe fn nitrox_config_nps_core_unit(ndev: *mut nitrox_device) {
    nitrox_write_csr(ndev, NPS_CORE_CONTROL, 1); let mut x: nps_core_gbl_vfcfg = core::mem::zeroed(); x.value=0; x.s.ilk_disable=1; x.s.cfg=__NDEV_MODE_PF; nitrox_write_csr(ndev,NPS_CORE_GBL_VFCFG,x.value); enable_nps_core_interrupts(ndev);
}
unsafe fn enable_nps_pkt_interrupts(ndev: *mut nitrox_device) { for r in [NPS_PKT_IN_RERR_LO_ENA_W1S,NPS_PKT_IN_RERR_HI_ENA_W1S,NPS_PKT_IN_ERR_TYPE_ENA_W1S,NPS_PKT_SLC_RERR_HI_ENA_W1S,NPS_PKT_SLC_RERR_LO_ENA_W1S,NPS_PKT_SLC_ERR_TYPE_ENA_W1S] { nitrox_write_csr(ndev,r,!0u64); } }
pub unsafe fn nitrox_config_nps_pkt_unit(ndev: *mut nitrox_device) { nitrox_config_pkt_input_rings(ndev); nitrox_config_pkt_solicit_ports(ndev); enable_nps_pkt_interrupts(ndev); }

unsafe fn reset_aqm_ring(ndev: *mut nitrox_device, ring: i32) {
    let mut en: aqmq_en=core::mem::zeroed(); let mut stat: aqmq_activity_stat=core::mem::zeroed(); let mut cnt: aqmq_cmp_cnt=core::mem::zeroed(); let mut retries=MAX_CSR_RETRIES;
    en.value=0; en.queue_enable=0; nitrox_write_csr(ndev,AQMQ_ENX(ring),en.value); usleep_range(100,150);
    loop { stat.value=nitrox_read_csr(ndev,AQMQ_ACTIVITY_STATX(ring)); if stat.queue_active==0 {break;} udelay(50); retries-=1; if retries<0 {break;} }
    cnt.value=nitrox_read_csr(ndev,AQMQ_CMP_CNTX(ring)); nitrox_write_csr(ndev,AQMQ_CMP_CNTX(ring),cnt.value); usleep_range(50,100);
}
pub unsafe fn enable_aqm_ring(ndev:*mut nitrox_device,ring:i32){let mut x:aqmq_en=core::mem::zeroed();x.value=0;x.queue_enable=1;nitrox_write_csr(ndev,AQMQ_ENX(ring),x.value);usleep_range(50,100);}
pub unsafe fn nitrox_config_aqm_rings(ndev:*mut nitrox_device){for ring in 0..(*ndev).nr_queues{let cmdq=(*ndev).aqmq[ring as usize];let mut d:aqmq_drbl=core::mem::zeroed();let mut q:aqmq_qsz=core::mem::zeroed();let mut t:aqmq_cmp_thr=core::mem::zeroed();reset_aqm_ring(ndev,ring);d.value=0;d.dbell_count=0xffffffff;nitrox_write_csr(ndev,AQMQ_DRBLX(ring),d.value);nitrox_write_csr(ndev,AQMQ_NXT_CMDX(ring),0);nitrox_write_csr(ndev,AQMQ_BADRX(ring),cmdq.dma);q.value=0;q.host_queue_size=(*ndev).qlen;nitrox_write_csr(ndev,AQMQ_QSZX(ring),q.value);t.value=0;t.commands_completed_threshold=1;nitrox_write_csr(ndev,AQMQ_CMP_THRX(ring),t.value);enable_aqm_ring(ndev,ring);}}
unsafe fn enable_aqm_interrupts(ndev:*mut nitrox_device){for r in [AQM_DBELL_OVF_LO_ENA_W1S,AQM_DBELL_OVF_HI_ENA_W1S,AQM_DMA_RD_ERR_LO_ENA_W1S,AQM_DMA_RD_ERR_HI_ENA_W1S,AQM_EXEC_NA_LO_ENA_W1S,AQM_EXEC_NA_HI_ENA_W1S,AQM_EXEC_ERR_LO_ENA_W1S,AQM_EXEC_ERR_HI_ENA_W1S]{nitrox_write_csr(ndev,r,!0u64);}}
pub unsafe fn nitrox_config_aqm_unit(ndev:*mut nitrox_device){nitrox_config_aqm_rings(ndev);enable_aqm_interrupts(ndev);}
pub unsafe fn nitrox_config_pom_unit(ndev:*mut nitrox_device){let mut x: pom_int_ena_w1s=core::mem::zeroed();x.value=0;x.s.illegal_dport=1;nitrox_write_csr(ndev,POM_INT_ENA_W1S,x.value);for i in 0..(*ndev).hw.se_cores{nitrox_write_csr(ndev,POM_PERF_CTL,1u64<<i);}}
pub unsafe fn nitrox_config_rand_unit(ndev:*mut nitrox_device){let mut x:efl_rnm_ctl_status=core::mem::zeroed();x.value=nitrox_read_csr(ndev,EFL_RNM_CTL_STATUS);x.s.ent_en=1;x.s.rng_en=1;nitrox_write_csr(ndev,EFL_RNM_CTL_STATUS,x.value);}
pub unsafe fn nitrox_config_efl_unit(ndev:*mut nitrox_device){for i in 0..NR_CLUSTERS{let mut x:efl_core_int_ena_w1s=core::mem::zeroed();x.value=0;x.s.len_ovr=1;x.s.d_left=1;x.s.epci_decode_err=1;nitrox_write_csr(ndev,EFL_CORE_INT_ENA_W1SX(i),x.value);nitrox_write_csr(ndev,EFL_CORE_VF_ERR_INT0_ENA_W1SX(i),!0u64);nitrox_write_csr(ndev,EFL_CORE_VF_ERR_INT1_ENA_W1SX(i),!0u64);}}
pub unsafe fn nitrox_config_bmi_unit(ndev:*mut nitrox_device){let mut c:bmi_ctl=core::mem::zeroed();let mut i:bmi_int_ena_w1s=core::mem::zeroed();c.value=nitrox_read_csr(ndev,BMI_CTL);c.s.max_pkt_len=0xff;c.s.nps_free_thrsh=0xff;c.s.nps_hdrq_thrsh=0x7a;nitrox_write_csr(ndev,BMI_CTL,c.value);i.value=0;i.s.max_len_err_nps=1;i.s.pkt_rcv_err_nps=1;i.s.fpf_undrrn=1;nitrox_write_csr(ndev,BMI_INT_ENA_W1S,i.value);}
pub unsafe fn nitrox_config_bmo_unit(ndev:*mut nitrox_device){let mut x:bmo_ctl2=core::mem::zeroed();x.value=nitrox_read_csr(ndev,BMO_CTL2);x.s.nps_slc_buf_thrsh=0xff;nitrox_write_csr(ndev,BMO_CTL2,x.value);}
pub unsafe fn invalidate_lbc(ndev:*mut nitrox_device){let mut c:lbc_inval_ctl=core::mem::zeroed();let mut s:lbc_inval_status=core::mem::zeroed();let mut retries=MAX_CSR_RETRIES;c.value=nitrox_read_csr(ndev,LBC_INVAL_CTL);c.s.cam_inval_start=1;nitrox_write_csr(ndev,LBC_INVAL_CTL,c.value);loop{s.value=nitrox_read_csr(ndev,LBC_INVAL_STATUS);if s.s.done!=0{break;}udelay(50);retries-=1;if retries<0{break;}}}
pub unsafe fn nitrox_config_lbc_unit(ndev:*mut nitrox_device){let mut x:lbc_int_ena_w1s=core::mem::zeroed();invalidate_lbc(ndev);x.value=0;x.s.dma_rd_err=1;x.s.over_fetch_err=1;x.s.cam_inval_abort=1;x.s.cam_hard_err=1;nitrox_write_csr(ndev,LBC_INT_ENA_W1S,x.value);for r in [LBC_PLM_VF1_64_INT_ENA_W1S,LBC_PLM_VF65_128_INT_ENA_W1S,LBC_ELM_VF1_64_INT_ENA_W1S,LBC_ELM_VF65_128_INT_ENA_W1S]{nitrox_write_csr(ndev,r,!0u64);}}
pub unsafe fn config_nps_core_vfcfg_mode(ndev:*mut nitrox_device,mode:vf_mode){let mut x:nps_core_gbl_vfcfg=core::mem::zeroed();x.value=nitrox_read_csr(ndev,NPS_CORE_GBL_VFCFG);x.s.cfg=mode&7;nitrox_write_csr(ndev,NPS_CORE_GBL_VFCFG,x.value);}

unsafe fn get_core_option(se:u8,ae:u8)->*const u8{if ae==AE_MAX_CORES{if se==SE_MAX_CORES{b"60\0".as_ptr()}else if se==40{b"60s\0".as_ptr()}else{b"\0".as_ptr()}}else if ae==AE_MAX_CORES/2{b"30\0".as_ptr()}else{b"60i\0".as_ptr()}}
unsafe fn get_feature_option(zip:u8,freq:i32)->*const u8{if zip==0{b"\0".as_ptr()}else if zip<ZIP_MAX_CORES{b"-C15\0".as_ptr()}else if freq>=850{b"-C45\0".as_ptr()}else if freq>=750{b"-C35\0".as_ptr()}else if freq>=550{b"-C25\0".as_ptr()}else{b"\0".as_ptr()}}

pub unsafe fn nitrox_get_hwinfo(ndev:*mut nitrox_device){let mut f:emu_fuse_map=core::mem::zeroed();let mut b:rst_boot=core::mem::zeroed();let mut d:fus_dat1=core::mem::zeroed();let mut name=[0u8;IFNAMSIZ*2];b.value=nitrox_read_csr(ndev,RST_BOOT);(*ndev).hw.freq=(b.pnr_mul+3)*PLL_REF_CLK;for i in 0..NR_CLUSTERS{f.value=nitrox_read_csr(ndev,EMU_FUSE_MAPX(i));if f.s.valid!=0{(*ndev).hw.ae_cores+=AE_CORES_PER_CLUSTER-hweight32(f.s.ae_fuse);(*ndev).hw.se_cores+=SE_CORES_PER_CLUSTER-hweight16(f.s.se_fuse);}}d.value=nitrox_read_csr(ndev,FUS_DAT1);if d.nozip==0{(*ndev).hw.zip_cores=ZIP_MAX_CORES-hweight8(d.zip_info);}snprintf(name.as_mut_ptr(),name.len(),b"CNN55%s-%3dBG676%s-1.%u\0".as_ptr(),get_core_option((*ndev).hw.se_cores,(*ndev).hw.ae_cores),(*ndev).hw.freq,get_feature_option((*ndev).hw.zip_cores,(*ndev).hw.freq),(*ndev).hw.revision_id);strscpy((*ndev).hw.partname.as_mut_ptr(),name.as_ptr());}
pub unsafe fn enable_pf2vf_mbox_interrupts(ndev:*mut nitrox_device){nitrox_write_csr(ndev,NPS_PKT_MBOX_INT_LO_ENA_W1S,!0u64);nitrox_write_csr(ndev,NPS_PKT_MBOX_INT_HI_ENA_W1S,!0u64);}
pub unsafe fn disable_pf2vf_mbox_interrupts(ndev:*mut nitrox_device){nitrox_write_csr(ndev,NPS_PKT_MBOX_INT_LO_ENA_W1C,!0u64);nitrox_write_csr(ndev,NPS_PKT_MBOX_INT_HI_ENA_W1C,!0u64);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
