/*
 * Broadcom specific AMBA
 * ChipCommon Power Management Unit driver
 *
 * Copyright 2009, Michael Buesch <m@bues.ch>
 * Copyright 2007, 2011, Broadcom Corporation
 * Copyright 2011, 2012, Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Translated from driver_chipcommon_pmu.c. Constants, types, and external
// functions are supplied by the surrounding BCMA implementation.

pub unsafe fn bcma_chipco_pll_read(cc: *mut bcma_drv_cc, offset: u32) -> u32 {
    bcma_pmu_write32(cc, BCMA_CC_PMU_PLLCTL_ADDR, offset);
    bcma_pmu_read32(cc, BCMA_CC_PMU_PLLCTL_ADDR);
    bcma_pmu_read32(cc, BCMA_CC_PMU_PLLCTL_DATA)
}

pub unsafe fn bcma_chipco_pll_write(cc: *mut bcma_drv_cc, offset: u32, value: u32) {
    bcma_pmu_write32(cc, BCMA_CC_PMU_PLLCTL_ADDR, offset);
    bcma_pmu_read32(cc, BCMA_CC_PMU_PLLCTL_ADDR);
    bcma_pmu_write32(cc, BCMA_CC_PMU_PLLCTL_DATA, value);
}

pub unsafe fn bcma_chipco_pll_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32) {
    bcma_pmu_write32(cc, BCMA_CC_PMU_PLLCTL_ADDR, offset);
    bcma_pmu_read32(cc, BCMA_CC_PMU_PLLCTL_ADDR);
    bcma_pmu_maskset32(cc, BCMA_CC_PMU_PLLCTL_DATA, mask, set);
}

pub unsafe fn bcma_chipco_chipctl_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32) {
    bcma_pmu_write32(cc, BCMA_CC_PMU_CHIPCTL_ADDR, offset);
    bcma_pmu_read32(cc, BCMA_CC_PMU_CHIPCTL_ADDR);
    bcma_pmu_maskset32(cc, BCMA_CC_PMU_CHIPCTL_DATA, mask, set);
}

pub unsafe fn bcma_chipco_regctl_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32) {
    bcma_pmu_write32(cc, BCMA_CC_PMU_REGCTL_ADDR, offset);
    bcma_pmu_read32(cc, BCMA_CC_PMU_REGCTL_ADDR);
    bcma_pmu_maskset32(cc, BCMA_CC_PMU_REGCTL_DATA, mask, set);
}

unsafe fn bcma_pmu_xtalfreq(cc: *mut bcma_drv_cc) -> u32 {
    if bcma_pmu_read32(cc, BCMA_CC_PMU_STAT) & BCMA_CC_PMU_STAT_EXT_LPO_AVAIL == 0 { return 0; }
    bcma_pmu_write32(cc, BCMA_CC_PMU_XTAL_FREQ, 1 << BCMA_CC_PMU_XTAL_FREQ_MEASURE_SHIFT);
    usleep_range(1000, 2000);
    let mut ilp_ctl = bcma_pmu_read32(cc, BCMA_CC_PMU_XTAL_FREQ) & BCMA_CC_PMU_XTAL_FREQ_ILPCTL_MASK;
    bcma_pmu_write32(cc, BCMA_CC_PMU_XTAL_FREQ, 0);
    let alp_hz = ilp_ctl * 32768 / 4;
    (alp_hz + 50000) / 100000 * 100
}

unsafe fn bcma_pmu2_pll_init0(cc: *mut bcma_drv_cc, xtalfreq: u32) {
    let bus = (*(*cc).core).bus;
    let mut freq_tgt_target = 0;
    let mut freq_tgt_current;
    let mut pll0;
    let mut mask;
    if (*(*bus).chipinfo).id == BCMA_CHIP_ID_BCM43142 {
        freq_tgt_target = match xtalfreq { 12000 => 0x50D52, 20000 => 0x307FE, 26000 => 0x254EA, 37400 => 0x19EF8, 52000 => 0x12A75, _ => 0 };
    }
    if freq_tgt_target == 0 { bcma_err(bus, "Unknown TGT frequency for xtalfreq %d\n", xtalfreq); return; }
    pll0 = bcma_chipco_pll_read(cc, BCMA_CC_PMU15_PLL_PLLCTL0);
    freq_tgt_current = (pll0 & BCMA_CC_PMU15_PLL_PC0_FREQTGT_MASK) >> BCMA_CC_PMU15_PLL_PC0_FREQTGT_SHIFT;
    if freq_tgt_current == freq_tgt_target { bcma_debug(bus, "Target TGT frequency already set\n"); return; }
    if (*(*bus).chipinfo).id == BCMA_CHIP_ID_BCM43142 {
        mask = !(BCMA_RES_4314_HT_AVAIL | BCMA_RES_4314_MACPHY_CLK_AVAIL);
        bcma_pmu_mask32(cc, BCMA_CC_PMU_MINRES_MSK, mask);
        bcma_pmu_mask32(cc, BCMA_CC_PMU_MAXRES_MSK, mask);
        bcma_wait_value((*cc).core, BCMA_CLKCTLST, BCMA_CLKCTLST_HAVEHT, 0, 20000);
    }
    pll0 = (pll0 & !BCMA_CC_PMU15_PLL_PC0_FREQTGT_MASK) | (freq_tgt_target << BCMA_CC_PMU15_PLL_PC0_FREQTGT_SHIFT);
    bcma_chipco_pll_write(cc, BCMA_CC_PMU15_PLL_PLLCTL0, pll0);
    if (*cc).pmu.rev >= 2 { bcma_pmu_set32(cc, BCMA_CC_PMU_CTL, BCMA_CC_PMU_CTL_PLL_UPD); }
    // TODO: Do we need to update OTP?
}

unsafe fn bcma_pmu_pll_init(cc: *mut bcma_drv_cc) {
    let bus = (*(*cc).core).bus;
    let mut xtalfreq = bcma_pmu_xtalfreq(cc);
    if (*(*bus).chipinfo).id == BCMA_CHIP_ID_BCM43142 { if xtalfreq == 0 { xtalfreq = 20000; } bcma_pmu2_pll_init0(cc, xtalfreq); }
}

unsafe fn bcma_pmu_resources_init(cc: *mut bcma_drv_cc) {
    let bus = (*(*cc).core).bus;
    let (mut min_msk, mut max_msk) = (0, 0);
    match (*(*bus).chipinfo).id {
        BCMA_CHIP_ID_BCM4313 => { min_msk=0x200D; max_msk=0xFFFF; }
        BCMA_CHIP_ID_BCM43142 => { min_msk=BCMA_RES_4314_LPLDO_PU|BCMA_RES_4314_PMU_SLEEP_DIS|BCMA_RES_4314_PMU_BG_PU|BCMA_RES_4314_CBUCK_LPOM_PU|BCMA_RES_4314_CBUCK_PFM_PU|BCMA_RES_4314_CLDO_PU|BCMA_RES_4314_LPLDO2_LVM|BCMA_RES_4314_WL_PMU_PU|BCMA_RES_4314_LDO3P3_PU|BCMA_RES_4314_OTP_PU|BCMA_RES_4314_WL_PWRSW_PU|BCMA_RES_4314_LQ_AVAIL|BCMA_RES_4314_LOGIC_RET|BCMA_RES_4314_MEM_SLEEP|BCMA_RES_4314_MACPHY_RET|BCMA_RES_4314_WL_CORE_READY; max_msk=0x3FFFFFFF; }
        _ => bcma_debug(bus, "PMU resource config unknown or not needed for device 0x%04X\n", (*(*bus).chipinfo).id),
    }
    if min_msk != 0 { bcma_pmu_write32(cc, BCMA_CC_PMU_MINRES_MSK, min_msk); }
    if max_msk != 0 { bcma_pmu_write32(cc, BCMA_CC_PMU_MAXRES_MSK, max_msk); }
    usleep_range(2000, 2500);
}

pub unsafe fn bcma_chipco_bcm4331_ext_pa_lines_ctl(cc: *mut bcma_drv_cc, enable: bool) {
    let bus = (*(*cc).core).bus; let mut val = bcma_cc_read32(cc, BCMA_CC_CHIPCTL);
    if enable { val |= BCMA_CHIPCTL_4331_EXTPA_EN; if (*(*bus).chipinfo).pkg == 9 || (*(*bus).chipinfo).pkg == 11 { val |= BCMA_CHIPCTL_4331_EXTPA_ON_GPIO2_5; } else if (*(*bus).chipinfo).rev > 0 { val |= BCMA_CHIPCTL_4331_EXTPA_EN2; } } else { val &= !BCMA_CHIPCTL_4331_EXTPA_EN; val &= !BCMA_CHIPCTL_4331_EXTPA_EN2; val &= !BCMA_CHIPCTL_4331_EXTPA_ON_GPIO2_5; }
    bcma_cc_write32(cc, BCMA_CC_CHIPCTL, val);
}

unsafe fn bcma_pmu_workarounds(cc: *mut bcma_drv_cc) {
    let bus = (*(*cc).core).bus;
    match (*(*bus).chipinfo).id {
        BCMA_CHIP_ID_BCM4313 => bcma_chipco_chipctl_maskset(cc,0,!BCMA_CCTRL_4313_12MA_LED_DRIVE,BCMA_CCTRL_4313_12MA_LED_DRIVE),
        BCMA_CHIP_ID_BCM4331|BCMA_CHIP_ID_BCM43431 => bcma_chipco_bcm4331_ext_pa_lines_ctl(cc,true),
        BCMA_CHIP_ID_BCM43224|BCMA_CHIP_ID_BCM43421 => { if (*(*bus).chipinfo).rev == 0 { bcma_cc_maskset32(cc,BCMA_CC_CHIPCTL,!BCMA_CCTRL_43224_GPIO_TOGGLE,BCMA_CCTRL_43224_GPIO_TOGGLE); bcma_chipco_chipctl_maskset(cc,0,!BCMA_CCTRL_43224A0_12MA_LED_DRIVE,BCMA_CCTRL_43224A0_12MA_LED_DRIVE); } else { bcma_chipco_chipctl_maskset(cc,0,!BCMA_CCTRL_43224B0_12MA_LED_DRIVE,BCMA_CCTRL_43224B0_12MA_LED_DRIVE); } }
        _ => bcma_debug(bus,"Workarounds unknown or not needed for device 0x%04X\n",(*(*bus).chipinfo).id),
    }
}

pub unsafe fn bcma_pmu_early_init(cc: *mut bcma_drv_cc) {
    let bus=(*(*cc).core).bus;
    if (*(*cc).core).id.rev >= 35 && (*cc).capabilities_ext & BCMA_CC_CAP_EXT_AOB_PRESENT != 0 { (*cc).pmu.core=bcma_find_core(bus,BCMA_CORE_PMU); if (*cc).pmu.core.is_null() { bcma_warn(bus,"Couldn't find expected PMU core"); } }
    if (*cc).pmu.core.is_null() { (*cc).pmu.core=(*cc).core; }
    let pmucap=bcma_pmu_read32(cc,BCMA_CC_PMU_CAP); (*cc).pmu.rev=pmucap & BCMA_CC_PMU_CAP_REVISION;
    bcma_debug(bus,"Found rev %u PMU (capabilities 0x%08X)\n",(*cc).pmu.rev,pmucap);
}

pub unsafe fn bcma_pmu_init(cc: *mut bcma_drv_cc) { if (*cc).pmu.rev==1 { bcma_pmu_mask32(cc,BCMA_CC_PMU_CTL,!BCMA_CC_PMU_CTL_NOILPONW); } else { bcma_pmu_set32(cc,BCMA_CC_PMU_CTL,BCMA_CC_PMU_CTL_NOILPONW); } bcma_pmu_pll_init(cc); bcma_pmu_resources_init(cc); bcma_pmu_workarounds(cc); }

pub unsafe fn bcma_pmu_get_alp_clock(cc: *mut bcma_drv_cc) -> u32 {
    let bus=(*(*cc).core).bus;
    match (*(*bus).chipinfo).id {
        BCMA_CHIP_ID_BCM4313|BCMA_CHIP_ID_BCM43224|BCMA_CHIP_ID_BCM43225|BCMA_CHIP_ID_BCM43227|BCMA_CHIP_ID_BCM43228|BCMA_CHIP_ID_BCM4331|BCMA_CHIP_ID_BCM43421|BCMA_CHIP_ID_BCM43428|BCMA_CHIP_ID_BCM43431|BCMA_CHIP_ID_BCM4716|BCMA_CHIP_ID_BCM47162|BCMA_CHIP_ID_BCM4748|BCMA_CHIP_ID_BCM4749|BCMA_CHIP_ID_BCM5357|BCMA_CHIP_ID_BCM53572|BCMA_CHIP_ID_BCM6362 => 20000*1000,
        BCMA_CHIP_ID_BCM4706|BCMA_CHIP_ID_BCM5356 => 25000*1000,
        BCMA_CHIP_ID_BCM43460|BCMA_CHIP_ID_BCM4352|BCMA_CHIP_ID_BCM4360 => if (*cc).status & BCMA_CC_CHIPST_4360_XTAL_40MZ != 0 {40000*1000} else {20000*1000},
        _ => { bcma_warn(bus,"No ALP clock specified for %04X device, pmu rev. %d, using default %d Hz\n",(*(*bus).chipinfo).id,(*cc).pmu.rev,BCMA_CC_PMU_ALP_CLOCK); BCMA_CC_PMU_ALP_CLOCK }
    }
}

unsafe fn bcma_pmu_pll_clock(cc:*mut bcma_drv_cc, pll0:u32, m:u32)->u32 { assert!(pll0&3==0 && pll0<=BCMA_CC_PMU4716_MAINPLL_PLL0); assert!(m!=0&&m<=4); let bus=(*(*cc).core).bus; if ((*(*bus).chipinfo).id==BCMA_CHIP_ID_BCM5357||(*(*bus).chipinfo).id==BCMA_CHIP_ID_BCM4749)&&bcma_cc_read32(cc,BCMA_CC_CHIPSTAT)&0x40000!=0{return 133*1000000;} let tmp=bcma_chipco_pll_read(cc,pll0+BCMA_CC_PPL_P1P2_OFF); let p1=(tmp&BCMA_CC_PPL_P1_MASK)>>BCMA_CC_PPL_P1_SHIFT; let p2=(tmp&BCMA_CC_PPL_P2_MASK)>>BCMA_CC_PPL_P2_SHIFT; let tmp=bcma_chipco_pll_read(cc,pll0+BCMA_CC_PPL_M14_OFF); let div=(tmp>>((m-1)*BCMA_CC_PPL_MDIV_WIDTH))&BCMA_CC_PPL_MDIV_MASK; let tmp=bcma_chipco_pll_read(cc,pll0+BCMA_CC_PPL_NM5_OFF); let ndiv=(tmp&BCMA_CC_PPL_NDIV_MASK)>>BCMA_CC_PPL_NDIV_SHIFT; let mut fc=bcma_pmu_get_alp_clock(cc)/1000000; fc=(p1*ndiv*fc)/p2; (fc/div)*1000000 }

unsafe fn bcma_pmu_pll_clock_bcm4706(cc:*mut bcma_drv_cc, pll0:u32, m:u32)->u32 { assert!(m!=0&&m<=4); let tmp=bcma_chipco_pll_read(cc,pll0+BCMA_CC_PMU6_4706_PROCPLL_OFF); let ndiv=(tmp&BCMA_CC_PMU6_4706_PROC_NDIV_INT_MASK)>>BCMA_CC_PMU6_4706_PROC_NDIV_INT_SHIFT; let p1div=(tmp&BCMA_CC_PMU6_4706_PROC_P1DIV_MASK)>>BCMA_CC_PMU6_4706_PROC_P1DIV_SHIFT; let p2div=(tmp&BCMA_CC_PMU6_4706_PROC_P2DIV_MASK)>>BCMA_CC_PMU6_4706_PROC_P2DIV_SHIFT; let mut clock=(25000000/if bcma_cc_read32(cc,BCMA_CC_CHIPSTAT)&BCMA_CC_CHIPST_4706_PKG_OPTION!=0{4}else{2})*ndiv*p2div/p1div; if m==BCMA_CC_PMU5_MAINPLL_SSB{clock/=4;} clock }

pub unsafe fn bcma_pmu_get_bus_clock(cc:*mut bcma_drv_cc)->u32 { let bus=(*(*cc).core).bus; match (*(*bus).chipinfo).id { BCMA_CHIP_ID_BCM4716|BCMA_CHIP_ID_BCM4748|BCMA_CHIP_ID_BCM47162=>bcma_pmu_pll_clock(cc,BCMA_CC_PMU4716_MAINPLL_PLL0,BCMA_CC_PMU5_MAINPLL_SSB), BCMA_CHIP_ID_BCM5356=>bcma_pmu_pll_clock(cc,BCMA_CC_PMU5356_MAINPLL_PLL0,BCMA_CC_PMU5_MAINPLL_SSB), BCMA_CHIP_ID_BCM5357|BCMA_CHIP_ID_BCM4749=>bcma_pmu_pll_clock(cc,BCMA_CC_PMU5357_MAINPLL_PLL0,BCMA_CC_PMU5_MAINPLL_SSB), BCMA_CHIP_ID_BCM4706=>bcma_pmu_pll_clock_bcm4706(cc,BCMA_CC_PMU4706_MAINPLL_PLL0,BCMA_CC_PMU5_MAINPLL_SSB), BCMA_CHIP_ID_BCM53572=>75000000, _=>{bcma_warn(bus,"No bus clock specified for %04X device, pmu rev. %d, using default %d Hz\n",(*(*bus).chipinfo).id,(*cc).pmu.rev,BCMA_CC_PMU_HT_CLOCK);BCMA_CC_PMU_HT_CLOCK} } }

pub unsafe fn bcma_pmu_get_cpu_clock(cc:*mut bcma_drv_cc)->u32 { let bus=(*(*cc).core).bus; if (*(*bus).chipinfo).id==BCMA_CHIP_ID_BCM53572{return 300000000;} if (*cc).pmu.rev>=5 { let pll=match (*(*bus).chipinfo).id { BCMA_CHIP_ID_BCM4706=>return bcma_pmu_pll_clock_bcm4706(cc,BCMA_CC_PMU4706_MAINPLL_PLL0,BCMA_CC_PMU5_MAINPLL_CPU), BCMA_CHIP_ID_BCM5356=>BCMA_CC_PMU5356_MAINPLL_PLL0, BCMA_CHIP_ID_BCM5357|BCMA_CHIP_ID_BCM4749=>BCMA_CC_PMU5357_MAINPLL_PLL0, _=>BCMA_CC_PMU4716_MAINPLL_PLL0 }; return bcma_pmu_pll_clock(cc,pll,BCMA_CC_PMU5_MAINPLL_CPU); } bcma_pmu_get_bus_clock(cc) }

unsafe fn bcma_pmu_spuravoid_pll_write(cc:*mut bcma_drv_cc, offset:u32, value:u32) { bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_ADDR,offset); bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_DATA,value); }

pub unsafe fn bcma_pmu_spuravoid_pllupdate(cc:*mut bcma_drv_cc, spuravoid:i32) {
    let bus=(*(*cc).core).bus; let mut tmp=0; let mut phypll_offset=0; let p1=[1u32,5,5]; let ndiv=[0x30u32,0xf6,0xfc];
    match (*(*bus).chipinfo).id {
        BCMA_CHIP_ID_BCM5357|BCMA_CHIP_ID_BCM4749|BCMA_CHIP_ID_BCM53572=>{phypll_offset=6; bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_ADDR,BCMA_CC_PMU_PLL_CTL0+phypll_offset); tmp=bcma_pmu_read32(cc,BCMA_CC_PMU_PLLCTL_DATA); tmp=(tmp&!BCMA_CC_PMU1_PLL0_PC0_P1DIV_MASK)|(p1[spuravoid as usize]<<BCMA_CC_PMU1_PLL0_PC0_P1DIV_SHIFT); bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_DATA,tmp); bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_ADDR,BCMA_CC_PMU_PLL_CTL2+phypll_offset); tmp=bcma_pmu_read32(cc,BCMA_CC_PMU_PLLCTL_DATA); tmp=(tmp&!BCMA_CC_PMU1_PLL0_PC2_NDIV_INT_MASK)|(ndiv[spuravoid as usize]<<BCMA_CC_PMU1_PLL0_PC2_NDIV_INT_SHIFT); bcma_pmu_write32(cc,BCMA_CC_PMU_PLLCTL_DATA,tmp); tmp=BCMA_CC_PMU_CTL_PLL_UPD;}
        BCMA_CHIP_ID_BCM4331|BCMA_CHIP_ID_BCM43431=>{ let vals=if spuravoid==2{(0x11500014,0x0FC00a08)}else if spuravoid==1{(0x11500014,0x0F600a08)}else{(0x11100014,0x03000a08)}; bcma_pmu_spuravoid_pll_write(cc,BCMA_CC_PMU_PLL_CTL0,vals.0); bcma_pmu_spuravoid_pll_write(cc,BCMA_CC_PMU_PLL_CTL2,vals.1); tmp=BCMA_CC_PMU_CTL_PLL_UPD;}
        BCMA_CHIP_ID_BCM43224|BCMA_CHIP_ID_BCM43225|BCMA_CHIP_ID_BCM43421=>{ let a=if spuravoid==1{[0x11500010,0x000C0C06,0x0F600a08,0,0x2001E920,0x88888815]}else{[0x11100010,0x000c0c06,0x03000a08,0,0x200005c0,0x88888815]}; for i in 0..6{bcma_pmu_spuravoid_pll_write(cc,BCMA_CC_PMU_PLL_CTL0+i,a[i]);} tmp=BCMA_CC_PMU_CTL_PLL_UPD;}
        BCMA_CHIP_ID_BCM4716|BCMA_CHIP_ID_BCM4748|BCMA_CHIP_ID_BCM47162=>{let a=if spuravoid==1{[0x11500060,0x080C0C06,0x0F600000,0,0x2001E924,0x88888815]}else{[0x11100060,0x080c0c06,0x03000000,0,0x200005c0,0x88888815]};for i in 0..6{bcma_pmu_spuravoid_pll_write(cc,BCMA_CC_PMU_PLL_CTL0+i,a[i]);}tmp=BCMA_CC_PMU_CTL_PLL_UPD|BCMA_CC_PMU_CTL_NOILPONW;}
        BCMA_CHIP_ID_BCM43131|BCMA_CHIP_ID_BCM43217|BCMA_CHIP_ID_BCM43227|BCMA_CHIP_ID_BCM43228|BCMA_CHIP_ID_BCM43428=>{let a=if spuravoid==1{[0x01100014,0x040C0C06,0x03140A08,0x00333333,0x202C2820,0x88888815]}else{[0x11100014,0x040c0c06,0x03000a08,0,0x200005c0,0x88888815]};for i in 0..6{bcma_pmu_spuravoid_pll_write(cc,BCMA_CC_PMU_PLL_CTL0+i,a[i]);}tmp=BCMA_CC_PMU_CTL_PLL_UPD;}
        _=>bcma_err(bus,"Unknown spuravoidance settings for chip 0x%04X, not changing PLL\n",(*(*bus).chipinfo).id),
    }
    tmp |= bcma_pmu_read32(cc,BCMA_CC_PMU_CTL); bcma_pmu_write32(cc,BCMA_CC_PMU_CTL,tmp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
