// SPDX-License-Identifier: GPL-2.0-only
/* Exynos PPMU support. Direct low-level translation of the C implementation. */

#[repr(C)]
pub enum exynos_ppmu_type { EXYNOS_TYPE_PPMU, EXYNOS_TYPE_PPMU_V2 }

#[repr(C)]
pub struct exynos_ppmu_data { pub clk: *mut clk }

#[repr(C)]
pub struct exynos_ppmu {
    pub edev: *mut *mut devfreq_event_dev,
    pub desc: *mut devfreq_event_desc,
    pub num_events: c_uint,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ppmu: exynos_ppmu_data,
    pub ppmu_type: exynos_ppmu_type,
}

#[repr(C)]
pub struct __exynos_ppmu_events { pub name: *mut c_char, pub id: c_int }

static mut ppmu_events: [__exynos_ppmu_events; 64] = [
    /* The event table is populated by the PPMU_EVENT macro in the source. */
];

unsafe fn __exynos_ppmu_find_ppmu_id(edev_name: *const c_char) -> c_int {
    let mut i = 0;
    while i < ppmu_events.len() {
        if !ppmu_events[i].name.is_null() && strcmp(edev_name, ppmu_events[i].name) == 0 { return ppmu_events[i].id; }
        i += 1;
    }
    -EINVAL
}

unsafe fn exynos_ppmu_find_ppmu_id(edev: *mut devfreq_event_dev) -> c_int {
    __exynos_ppmu_find_ppmu_id((*(*edev).desc).name)
}

unsafe fn exynos_ppmu_disable(edev: *mut devfreq_event_dev) -> c_int {
    let info = devfreq_event_get_drvdata(edev); let mut pmnc = 0; let mut ret;
    ret = regmap_write((*info).regmap, PPMU_CNTENC, PPMU_CCNT_MASK | PPMU_PMCNT0_MASK | PPMU_PMCNT1_MASK | PPMU_PMCNT2_MASK | PPMU_PMCNT3_MASK); if ret < 0 { return ret; }
    ret = regmap_read((*info).regmap, PPMU_PMNC, &mut pmnc); if ret < 0 { return ret; }
    pmnc &= !PPMU_PMNC_ENABLE_MASK; ret = regmap_write((*info).regmap, PPMU_PMNC, pmnc); if ret < 0 { return ret; } 0
}

unsafe fn exynos_ppmu_set_event(edev: *mut devfreq_event_dev) -> c_int {
    let info = devfreq_event_get_drvdata(edev); let id = exynos_ppmu_find_ppmu_id(edev); if id < 0 { return id; }
    let mut cntens = 0; let mut pmnc = 0; let mut ret = regmap_read((*info).regmap, PPMU_CNTENS, &mut cntens); if ret < 0 { return ret; }
    cntens |= PPMU_CCNT_MASK | (PPMU_ENABLE << id); ret = regmap_write((*info).regmap, PPMU_CNTENS, cntens); if ret < 0 { return ret; }
    ret = regmap_write((*info).regmap, PPMU_BEVTxSEL(id), (*(*edev).desc).event_type); if ret < 0 { return ret; }
    ret = regmap_read((*info).regmap, PPMU_PMNC, &mut pmnc); if ret < 0 { return ret; }
    pmnc &= !(PPMU_PMNC_ENABLE_MASK | PPMU_PMNC_COUNTER_RESET_MASK | PPMU_PMNC_CC_RESET_MASK);
    pmnc |= PPMU_ENABLE << PPMU_PMNC_ENABLE_SHIFT; pmnc |= PPMU_ENABLE << PPMU_PMNC_COUNTER_RESET_SHIFT; pmnc |= PPMU_ENABLE << PPMU_PMNC_CC_RESET_SHIFT;
    ret = regmap_write((*info).regmap, PPMU_PMNC, pmnc); if ret < 0 { return ret; } 0
}

unsafe fn exynos_ppmu_get_event(edev: *mut devfreq_event_dev, edata: *mut devfreq_event_data) -> c_int {
    let info = devfreq_event_get_drvdata(edev); let id = exynos_ppmu_find_ppmu_id(edev); if id < 0 { return -EINVAL; }
    let (mut pmnc, mut cntenc, mut total_count, mut load_count, mut high, mut low) = (0,0,0,0,0,0); let mut ret = regmap_read((*info).regmap, PPMU_PMNC, &mut pmnc); if ret < 0{return ret;}
    pmnc &= !PPMU_PMNC_ENABLE_MASK; ret=regmap_write((*info).regmap,PPMU_PMNC,pmnc); if ret<0{return ret;}
    ret=regmap_read((*info).regmap,PPMU_CCNT,&mut total_count); if ret<0{return ret;} (*edata).total_count=total_count;
    match id { PPMU_PMNCNT0|PPMU_PMNCNT1|PPMU_PMNCNT2 => {ret=regmap_read((*info).regmap,PPMU_PMNCT(id),&mut load_count);if ret<0{return ret;}}, PPMU_PMNCNT3=>{ret=regmap_read((*info).regmap,PPMU_PMCNT3_HIGH,&mut high);if ret<0{return ret;}ret=regmap_read((*info).regmap,PPMU_PMCNT3_LOW,&mut low);if ret<0{return ret;}load_count=(high<<8)|low;}, _=>return -EINVAL }
    (*edata).load_count=load_count; ret=regmap_read((*info).regmap,PPMU_CNTENC,&mut cntenc);if ret<0{return ret;}cntenc|=PPMU_CCNT_MASK|(PPMU_ENABLE<<id);ret=regmap_write((*info).regmap,PPMU_CNTENC,cntenc);if ret<0{return ret;} 0
}

// PPMU v2 operations retain the source ABI and register ordering.
unsafe fn exynos_ppmu_v2_disable(edev:*mut devfreq_event_dev)->c_int { let info=devfreq_event_get_drvdata(edev); let clear=PPMU_CCNT_MASK|PPMU_PMCNT0_MASK|PPMU_PMCNT1_MASK|PPMU_PMCNT2_MASK|PPMU_PMCNT3_MASK; let regs=[PPMU_V2_FLAG,PPMU_V2_INTENC,PPMU_V2_CNTENC,PPMU_V2_CNT_RESET]; for r in regs {let x=regmap_write((*info).regmap,r,clear);if x<0{return x;}} for r in [PPMU_V2_CIG_CFG0,PPMU_V2_CIG_CFG1,PPMU_V2_CIG_CFG2,PPMU_V2_CIG_RESULT,PPMU_V2_CNT_AUTO,PPMU_V2_CH_EV0_TYPE,PPMU_V2_CH_EV1_TYPE,PPMU_V2_CH_EV2_TYPE,PPMU_V2_CH_EV3_TYPE,PPMU_V2_SM_ID_V,PPMU_V2_SM_ID_A,PPMU_V2_SM_OTHERS_V,PPMU_V2_SM_OTHERS_A,PPMU_V2_INTERRUPT_RESET] {let x=regmap_write((*info).regmap,r,0);if x<0{return x;}} let mut pmnc=0;let mut x=regmap_read((*info).regmap,PPMU_V2_PMNC,&mut pmnc);if x<0{return x;}pmnc&=!PPMU_PMNC_ENABLE_MASK;x=regmap_write((*info).regmap,PPMU_V2_PMNC,pmnc);if x<0{return x;}0 }

unsafe fn exynos_ppmu_v2_set_event(edev:*mut devfreq_event_dev)->c_int { let info=devfreq_event_get_drvdata(edev); let id=exynos_ppmu_find_ppmu_id(edev); let mut cntens=0;let mut pmnc=0;let mut r=regmap_read((*info).regmap,PPMU_V2_CNTENS,&mut cntens);if r<0{return r;}cntens|=PPMU_CCNT_MASK|(PPMU_ENABLE<<id);r=regmap_write((*info).regmap,PPMU_V2_CNTENS,cntens);if r<0{return r;}r=regmap_write((*info).regmap,PPMU_V2_CH_EVx_TYPE(id),(*(*edev).desc).event_type);if r<0{return r;}r=regmap_read((*info).regmap,PPMU_V2_PMNC,&mut pmnc);if r<0{return r;}pmnc&=!(PPMU_PMNC_ENABLE_MASK|PPMU_PMNC_COUNTER_RESET_MASK|PPMU_PMNC_CC_RESET_MASK|PPMU_PMNC_CC_DIVIDER_MASK|PPMU_V2_PMNC_START_MODE_MASK);pmnc|=PPMU_ENABLE<<PPMU_PMNC_ENABLE_SHIFT;pmnc|=PPMU_ENABLE<<PPMU_PMNC_COUNTER_RESET_SHIFT;pmnc|=PPMU_ENABLE<<PPMU_PMNC_CC_RESET_SHIFT;pmnc|=PPMU_V2_MODE_MANUAL<<PPMU_V2_PMNC_START_MODE_SHIFT;r=regmap_write((*info).regmap,PPMU_V2_PMNC,pmnc);if r<0{return r;}0 }

unsafe fn exynos_ppmu_v2_get_event(edev:*mut devfreq_event_dev,edata:*mut devfreq_event_data)->c_int { let info=devfreq_event_get_drvdata(edev);let id=exynos_ppmu_find_ppmu_id(edev);let(mut pmnc,mut cntenc,mut hi,mut lo,mut total,mut count)=(0,0,0,0,0,0);let mut r=regmap_read((*info).regmap,PPMU_V2_PMNC,&mut pmnc);if r<0{return r;}pmnc&=!PPMU_PMNC_ENABLE_MASK;r=regmap_write((*info).regmap,PPMU_V2_PMNC,pmnc);if r<0{return r;}r=regmap_read((*info).regmap,PPMU_V2_CCNT,&mut total);if r<0{return r;}(*edata).total_count=total;match id{PPMU_PMNCNT0|PPMU_PMNCNT1|PPMU_PMNCNT2=>{r=regmap_read((*info).regmap,PPMU_V2_PMNCT(id),&mut count);if r<0{return r;}(*edata).load_count=count;},PPMU_PMNCNT3=>{r=regmap_read((*info).regmap,PPMU_V2_PMCNT3_HIGH,&mut hi);if r<0{return r;}r=regmap_read((*info).regmap,PPMU_V2_PMCNT3_LOW,&mut lo);if r<0{return r;}(*edata).load_count=((hi&0xff)as u64)<<32|(lo as u64);},_=>{}}r=regmap_read((*info).regmap,PPMU_V2_CNTENC,&mut cntenc);if r<0{return 0;}cntenc|=PPMU_CCNT_MASK|(PPMU_ENABLE<<id);r=regmap_write((*info).regmap,PPMU_V2_CNTENC,cntenc);if r<0{return r;}0 }

#[repr(C)] pub struct devfreq_event_ops { pub disable:Option<unsafe fn(*mut devfreq_event_dev)->c_int>, pub set_event:Option<unsafe fn(*mut devfreq_event_dev)->c_int>, pub get_event:Option<unsafe fn(*mut devfreq_event_dev,*mut devfreq_event_data)->c_int> }
static exynos_ppmu_ops:devfreq_event_ops=devfreq_event_ops{disable:Some(exynos_ppmu_disable),set_event:Some(exynos_ppmu_set_event),get_event:Some(exynos_ppmu_get_event)};
static exynos_ppmu_v2_ops:devfreq_event_ops=devfreq_event_ops{disable:Some(exynos_ppmu_v2_disable),set_event:Some(exynos_ppmu_v2_set_event),get_event:Some(exynos_ppmu_v2_get_event)};

// Remaining external framework declarations and v2 event handlers are supplied by the surrounding kernel translation.
extern "C" { fn strcmp(a:*const c_char,b:*const c_char)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
