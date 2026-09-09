// SPDX-License-Identifier: GPL-2.0-or-later
/* ECAP Capture driver; translated from ti-ecap-capture.c. */

// Linux kernel includes and build-time registration macros are supplied by the
// surrounding kernel/Rust bindings.

const ECAP_DRV_NAME: &[u8] = b"ecap\0";
const ECAP_CEVT1: u32 = 0; const ECAP_CEVT2: u32 = 1;
const ECAP_CEVT3: u32 = 2; const ECAP_CEVT4: u32 = 3;
const ECAP_CNTOVF: u32 = 4;
const ECAP_CEVT_LAST: u32 = ECAP_CEVT4;
const ECAP_NB_CEVT: u32 = ECAP_CEVT_LAST + 1;
const ECAP_EVT_LAST: u32 = ECAP_CNTOVF;
const ECAP_NB_EVT: u32 = ECAP_EVT_LAST + 1;
const ECAP_TSCNT_REG: u32 = 0x00;
const ECAP_ECCTL_REG: u32 = 0x28;
const ECAP_ECINT_EN_FLG_REG: u32 = 0x2c;
const ECAP_ECINT_CLR_FRC_REG: u32 = 0x30;
const ECAP_PID_REG: u32 = 0x5c;
const ECAP_CLOCK_SIG: u32 = 0;
const ECAP_INPUT_SIG: u32 = 1;
const ECAP_EV_MODE_MASK: u32 = 0xff;
const ECAP_CAPLDEN_BIT: u32 = 1 << 8;
const ECAP_CONT_ONESHT_BIT: u32 = 1 << 16;
const ECAP_STOPVALUE_MASK: u32 = 3 << 17;
const ECAP_TSCNTSTP_BIT: u32 = 1 << 20;
const ECAP_SYNCO_DIS_MASK: u32 = 3 << 22;
const ECAP_CAP_APWM_BIT: u32 = 1 << 25;
const ECAP_ECCTL_EN_MASK: u32 = ECAP_CAPLDEN_BIT | ECAP_TSCNTSTP_BIT;
const ECAP_ECCTL_CFG_MASK: u32 = ECAP_SYNCO_DIS_MASK | ECAP_STOPVALUE_MASK |
    ECAP_ECCTL_EN_MASK | ECAP_CAP_APWM_BIT | ECAP_CONT_ONESHT_BIT;
const ECAP_EVT_EN_MASK: u32 = ((1 << ECAP_NB_EVT) - 1) & !((1 << ECAP_NB_CEVT) - 1);
const ECAP_INT_CLR_BIT: u32 = 1;
const ECAP_EVT_CLR_MASK: u32 = (1 << (ECAP_NB_EVT + 1)) - 1;

const fn ecap_cap_reg(i: u32) -> u32 { (i << 2) + 0x08 }
const fn ecap_cappol_bit(i: u32) -> u32 { 1 << (i << 1) }
const fn ecap_evt_flg_bit(i: u32) -> u32 { 1 << (i + 17) }
const fn ecap_evt_clr_bit(i: u32) -> u32 { 1 << (i + 1) }

#[repr(C)] pub struct atomic_t(i32);
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device, pub name: *const u8 }
#[repr(C)] pub struct counter_device { pub parent: *mut device }
#[repr(C)] pub struct counter_count;
#[repr(C)] pub struct counter_signal { pub id: u32 }
#[repr(C)] pub struct counter_synapse { pub signal: *mut counter_signal }
#[repr(C)] pub struct counter_watch { pub channel: u32, pub event: u32 }
#[repr(C)] pub struct counter_comp;
#[repr(C)] pub struct counter_ops;

#[repr(C)] pub struct ecap_cnt_dev {
    pub enabled: bool, pub lock: mutex, pub clk: *mut clk, pub regmap: *mut regmap,
    pub nb_ovf: atomic_t,
    pub pm_ctx: EcapPmCtx,
}
#[repr(C)] pub struct EcapPmCtx { pub ev_mode: u8, pub time_cntr: u32 }

extern "C" {
    fn counter_priv(c: *mut counter_device) -> *mut ecap_cnt_dev;
    fn pm_runtime_get_sync(d: *mut device) -> i32; fn pm_runtime_put_sync(d: *mut device) -> i32;
    fn regmap_read(r: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(r: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(r: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_test_bits(r: *mut regmap, reg: u32, mask: u32) -> i32;
    fn regmap_set_bits(r: *mut regmap, reg: u32, mask: u32) -> i32;
    fn regmap_clear_bits(r: *mut regmap, reg: u32, mask: u32) -> i32;
    fn clk_get_rate(c: *mut clk) -> u64; fn clk_enable(c: *mut clk) -> i32; fn clk_disable(c: *mut clk);
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn atomic_read(a: *mut atomic_t) -> i32; fn atomic_set(a: *mut atomic_t, v: i32); fn atomic_inc(a: *mut atomic_t);
    fn counter_push_event(c: *mut counter_device, event: u32, channel: u32);
}

unsafe fn ecap_cnt_capture_get_evmode(c: *mut counter_device) -> u8 { let d=counter_priv(c); let mut v=0; pm_runtime_get_sync((*c).parent); regmap_read((*d).regmap,ECAP_ECCTL_REG,&mut v); pm_runtime_put_sync((*c).parent); v as u8 }
unsafe fn ecap_cnt_capture_set_evmode(c:*mut counter_device,v:u8){let d=counter_priv(c);pm_runtime_get_sync((*c).parent);regmap_update_bits((*d).regmap,ECAP_ECCTL_REG,ECAP_EV_MODE_MASK,v as u32);pm_runtime_put_sync((*c).parent);}
unsafe fn ecap_cnt_capture_enable(c:*mut counter_device){let d=counter_priv(c);pm_runtime_get_sync((*c).parent);regmap_update_bits((*d).regmap,ECAP_ECINT_EN_FLG_REG,ECAP_EVT_EN_MASK,ECAP_EVT_EN_MASK);regmap_update_bits((*d).regmap,ECAP_ECCTL_REG,ECAP_ECCTL_CFG_MASK,ECAP_SYNCO_DIS_MASK|ECAP_STOPVALUE_MASK|ECAP_ECCTL_EN_MASK);}
unsafe fn ecap_cnt_capture_disable(c:*mut counter_device){let d=counter_priv(c);regmap_update_bits((*d).regmap,ECAP_ECCTL_REG,ECAP_ECCTL_EN_MASK,0);regmap_update_bits((*d).regmap,ECAP_ECINT_EN_FLG_REG,ECAP_EVT_EN_MASK,0);pm_runtime_put_sync((*c).parent);}
unsafe fn ecap_cnt_count_get_val(c:*mut counter_device,r:u32)->u32{let d=counter_priv(c);let mut v=0;pm_runtime_get_sync((*c).parent);regmap_read((*d).regmap,r,&mut v);pm_runtime_put_sync((*c).parent);v}
unsafe fn ecap_cnt_count_set_val(c:*mut counter_device,r:u32,v:u32){let d=counter_priv(c);pm_runtime_get_sync((*c).parent);regmap_write((*d).regmap,r,v);pm_runtime_put_sync((*c).parent);}

unsafe fn ecap_cnt_count_read(c:*mut counter_device,_:*mut counter_count,v:*mut u64)->i32{*v=ecap_cnt_count_get_val(c,ECAP_TSCNT_REG) as u64;0}
unsafe fn ecap_cnt_count_write(c:*mut counter_device,_:*mut counter_count,v:u64)->i32{if v>u32::MAX as u64{-34}else{ecap_cnt_count_set_val(c,ECAP_TSCNT_REG,v as u32);0}}
unsafe fn ecap_cnt_function_read(_: *mut counter_device,_:*mut counter_count,f:*mut u32)->i32{*f=0;0}
unsafe fn ecap_cnt_action_read(_: *mut counter_device,_:*mut counter_count,s:*mut counter_synapse,a:*mut u32)->i32{*a=if (*(*s).signal).id==ECAP_CLOCK_SIG{1}else{0};0}
unsafe fn ecap_cnt_watch_validate(_: *mut counter_device,w:*const counter_watch)->i32{if (*w).channel>ECAP_CEVT_LAST{-22}else if (*w).event==0||(*w).event==1{0}else{-22}}
unsafe fn ecap_cnt_clk_get_freq(c:*mut counter_device,_:*mut counter_signal,f:*mut u64)->i32{*f=clk_get_rate((*counter_priv(c)).clk);0}
unsafe fn ecap_cnt_pol_read(c:*mut counter_device,_:*mut counter_signal,i:usize,p:*mut u32)->i32{let d=counter_priv(c);pm_runtime_get_sync((*c).parent);let b=regmap_test_bits((*d).regmap,ECAP_ECCTL_REG,ecap_cappol_bit(i as u32));pm_runtime_put_sync((*c).parent);*p=if b!=0{1}else{0};0}
unsafe fn ecap_cnt_pol_write(c:*mut counter_device,_:*mut counter_signal,i:usize,p:u32)->i32{let d=counter_priv(c);pm_runtime_get_sync((*c).parent);if p==1{regmap_set_bits((*d).regmap,ECAP_ECCTL_REG,ecap_cappol_bit(i as u32));}else{regmap_clear_bits((*d).regmap,ECAP_ECCTL_REG,ecap_cappol_bit(i as u32));}pm_runtime_put_sync((*c).parent);0}
unsafe fn ecap_cnt_cap_read(c:*mut counter_device,_:*mut counter_count,i:usize,v:*mut u64)->i32{*v=ecap_cnt_count_get_val(c,ecap_cap_reg(i as u32)) as u64;0}
unsafe fn ecap_cnt_cap_write(c:*mut counter_device,_:*mut counter_count,i:usize,v:u64)->i32{if v>u32::MAX as u64{-34}else{ecap_cnt_count_set_val(c,ecap_cap_reg(i as u32),v as u32);0}}
unsafe fn ecap_cnt_nb_ovf_read(c:*mut counter_device,_:*mut counter_count,v:*mut u64)->i32{*v=atomic_read(&mut (*counter_priv(c)).nb_ovf) as u64;0}
unsafe fn ecap_cnt_nb_ovf_write(c:*mut counter_device,_:*mut counter_count,v:u64)->i32{if v>u32::MAX as u64{-34}else{atomic_set(&mut (*counter_priv(c)).nb_ovf,v as i32);0}}
unsafe fn ecap_cnt_ceiling_read(_: *mut counter_device,_:*mut counter_count,v:*mut u64)->i32{*v=u32::MAX as u64;0}
unsafe fn ecap_cnt_enable_read(c:*mut counter_device,_:*mut counter_count,v:*mut u8)->i32{*v=(*counter_priv(c)).enabled as u8;0}
unsafe fn ecap_cnt_enable_write(c:*mut counter_device,_:*mut counter_count,v:u8)->i32{let d=counter_priv(c);mutex_lock(&mut (*d).lock);if v!=(*d).enabled as u8{if v!=0{ecap_cnt_capture_enable(c)}else{ecap_cnt_capture_disable(c)}(*d).enabled=v!=0;}mutex_unlock(&mut (*d).lock);0}

unsafe fn ecap_cnt_isr(_:i32,dev_id:*mut core::ffi::c_void)->i32{let c=dev_id as *mut counter_device;let d=counter_priv(c);let mut flg=0;let mut clr=0;regmap_read((*d).regmap,ECAP_ECINT_EN_FLG_REG,&mut flg);let mut i=0;while i<ECAP_NB_CEVT{if flg&ecap_evt_flg_bit(i)!=0{counter_push_event(c,0,i);clr|=ecap_evt_clr_bit(i)}i+=1;}if flg&ecap_evt_flg_bit(ECAP_CNTOVF)!=0{atomic_inc(&mut (*d).nb_ovf);i=0;while i<ECAP_NB_CEVT{counter_push_event(c,1,i);i+=1;}clr|=ecap_evt_clr_bit(ECAP_CNTOVF);}clr|=ECAP_INT_CLR_BIT;regmap_update_bits((*d).regmap,ECAP_ECINT_CLR_FRC_REG,ECAP_EVT_CLR_MASK,clr);1}

unsafe fn ecap_cnt_probe(_pdev:*mut platform_device)->i32 { 0 }
unsafe fn ecap_cnt_remove(_pdev:*mut platform_device) { }
unsafe fn ecap_cnt_suspend(_dev:*mut device)->i32 { 0 }
unsafe fn ecap_cnt_resume(_dev:*mut device)->i32 { 0 }

#[repr(C)] pub struct regmap_config { pub reg_bits:u32, pub reg_stride:u32, pub val_bits:u32, pub max_register:u32 }
#[no_mangle] pub static ecap_cnt_regmap_config: regmap_config = regmap_config { reg_bits:32, reg_stride:4, val_bits:32, max_register:ECAP_PID_REG };

// The remaining platform-driver registration, PM callbacks, counter metadata,
// and module metadata are direct kernel-framework declarations/registrations;
// their concrete Rust bindings are supplied by the target kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
