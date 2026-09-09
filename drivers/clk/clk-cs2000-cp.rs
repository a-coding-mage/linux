// SPDX-License-Identifier: GPL-2.0
/*
 * CS2000  --  CIRRUS LOGIC Fractional-N Clock Synthesizer & Clock Multiplier
 *
 * Copyright (C) 2015 Renesas Electronics Corporation
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */
// Linux kernel dependencies supplied by the surrounding repository.

const CH_MAX: i32 = 4;
const RATIO_REG_SIZE: u32 = 4;
const DEVICE_ID: u32 = 0x1;
const DEVICE_CTRL: u32 = 0x2;
const DEVICE_CFG1: u32 = 0x3;
const DEVICE_CFG2: u32 = 0x4;
const GLOBAL_CFG: u32 = 0x5;
const FUNC_CFG1: u32 = 0x16;
const FUNC_CFG2: u32 = 0x17;

const REVISION_MASK: u32 = 0x7;
const REVISION_B2_B3: u32 = 0x4;
const REVISION_C1: u32 = 0x6;
const PLL_UNLOCK: u32 = 1 << 7;
const AUXOUTDIS: u32 = 1 << 1;
const CLKOUTDIS: u32 = 1 << 0;
const ENDEV1: u32 = 0x1;
const AUTORMOD: u32 = 1 << 3;
const FRACNSRC_MASK: u32 = 1;
const FRACNSRC_STATIC: u8 = 0;
const FRACNSRC_DYNAMIC: u8 = 1;
const FREEZE: u32 = 1 << 7;
const ENDEV2: u32 = 0x1;
const CLKSKIPEN: u32 = 1 << 7;
const LFRATIO_MASK: u32 = 1 << 3;
const LFRATIO_20_12: u32 = 0;
const LFRATIO_12_20: u32 = 1 << 3;
const CLK_IN: u8 = 0;
const REF_CLK: u8 = 1;
const CLK_MAX: usize = 2;

const fn ratio_add(x: u32, nth: u32) -> u32 { 6 + x * 4 + nth }
const fn ratio_val(x: u32, nth: u32) -> u32 { (x >> (24 - 8 * nth)) & 0xff }
const fn val_ratio(x: u32, nth: u32) -> u32 { (x & 0xff) << (24 - 8 * nth) }
const fn rsel(x: u32) -> u32 { (x & 3) << 3 }
const fn auxoutsrc(x: u32) -> u32 { (x & 3) << 1 }
const fn lockclk(x: u32) -> u32 { (x & 3) << 1 }
const fn refclkdiv(x: u32) -> u32 { (x & 3) << 3 }
const RSEL_MASK: u32 = rsel(3);
const AUXOUTSRC_MASK: u32 = auxoutsrc(3);
const LOCKCLK_MASK: u32 = lockclk(3);
const REFCLKDIV_MASK: u32 = refclkdiv(3);

#[repr(C)]
pub struct Cs2000Priv {
    pub hw: clk_hw,
    pub client: *mut i2c_client,
    pub clk_in: *mut clk,
    pub ref_clk: *mut clk,
    pub regmap: *mut regmap,
    pub dynamic_mode: bool,
    pub lf_ratio: bool,
    pub clk_skip: bool,
    pub saved_rate: c_ulong,
    pub saved_parent_rate: c_ulong,
}

// External kernel types and functions are supplied by other translation units.
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct clk;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct regmap_config;
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const c_char, pub num_parents: usize }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_ops { pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong)->c_ulong>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong,c_ulong)->c_int>, pub prepare: Option<unsafe extern "C" fn(*mut clk_hw)->c_int>, pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)> }
type c_int = i32; type c_ulong = usize; type c_char = i8;

extern "C" {
    fn regmap_update_bits(_: *mut regmap,u32,u32,u32)->c_int; fn regmap_read(_: *mut regmap,u32,*mut u32)->c_int; fn regmap_write(_: *mut regmap,u32,u32)->c_int;
    fn udelay(_: u32); fn devm_clk_get(_: *mut device,*const c_char)->*mut clk; fn clk_get_rate(_: *mut clk)->c_ulong; fn __clk_get_name(_: *mut clk)->*const c_char;
    fn devm_kzalloc(_: *mut device, _: usize, _: u32)->*mut Cs2000Priv; fn devm_regmap_init_i2c(_: *mut i2c_client,*const regmap_config)->*mut regmap;
    fn i2c_set_clientdata(_: *mut i2c_client,*mut Cs2000Priv); fn i2c_get_clientdata(_: *mut i2c_client)->*mut Cs2000Priv; fn dev_get_drvdata(_: *mut device)->*mut Cs2000Priv;
    fn clk_hw_register(_: *mut device,*mut clk_hw)->c_int; fn clk_hw_unregister(_: *mut clk_hw); fn of_clk_add_hw_provider(_: *mut device_node, _: *const (), _: *mut clk_hw)->c_int; fn of_clk_del_provider(_: *mut device_node);
}

unsafe fn cs2000_rate_to_ratio(rate_in:u32, rate_out:u32, lf:bool)->u32 { (((rate_out as u64) << if lf {12} else {20}) / rate_in as u64) as u32 }
unsafe fn cs2000_ratio_to_rate(ratio:u32, rate_in:u32, lf:bool)->c_ulong { ((ratio as u64 * rate_in as u64) >> if lf {12} else {20}) as c_ulong }

unsafe fn cs2000_ratio_set(p:*mut Cs2000Priv,ch:i32,ri:u32,ro:u32)->c_int { if ch<0 || ch>=CH_MAX{return -22}; let v=cs2000_rate_to_ratio(ri,ro,(*p).lf_ratio); for i in 0..RATIO_REG_SIZE { let r=regmap_write((*p).regmap,ratio_add(ch as u32,i),ratio_val(v,i)); if r<0{return r} } 0 }
unsafe fn cs2000_ratio_get(p:*mut Cs2000Priv,ch:i32)->u32 { let mut v=0; for i in 0..RATIO_REG_SIZE { let mut t=0; if regmap_read((*p).regmap,ratio_add(ch as u32,i),&mut t)<0{return 0}; v|=val_ratio(t,i) } v }
unsafe fn cs2000_set_rate(p:*mut Cs2000Priv,rate:c_ulong,parent:c_ulong)->c_int { let ch=0; let r=regmap_update_bits((*p).regmap,GLOBAL_CFG,FREEZE,FREEZE); if r<0{return r}; (*p).lf_ratio=(*p).dynamic_mode && rate/parent>4096; let r=regmap_update_bits((*p).regmap,FUNC_CFG2,LFRATIO_MASK,if (*p).lf_ratio{LFRATIO_20_12}else{LFRATIO_12_20}); if r<0{return r}; let r=cs2000_ratio_set(p,ch,parent as u32,rate as u32); if r<0{return r}; (*p).saved_rate=rate; (*p).saved_parent_rate=parent; regmap_update_bits((*p).regmap,GLOBAL_CFG,FREEZE,0) }

// The remaining driver entry points retain the original kernel-facing interfaces.
unsafe fn cs2000_recalc_rate(p:*mut Cs2000Priv,parent:c_ulong)->c_ulong { cs2000_ratio_to_rate(cs2000_ratio_get(p,0),parent as u32,(*p).lf_ratio) }
unsafe fn cs2000_get_parent(p:*mut Cs2000Priv)->u8 { if (*p).dynamic_mode {CLK_IN} else {REF_CLK} }

unsafe fn cs2000_enable_dev_config(p:*mut Cs2000Priv,en:bool)->c_int { let e=if en{ENDEV1}else{0}; let r=regmap_update_bits((*p).regmap,DEVICE_CFG1,ENDEV1,e); if r<0{return r}; let r=regmap_update_bits((*p).regmap,GLOBAL_CFG,ENDEV2,if en{ENDEV2}else{0}); if r<0{return r}; regmap_update_bits((*p).regmap,FUNC_CFG1,CLKSKIPEN,if en&&(*p).clk_skip{CLKSKIPEN}else{0}) }
unsafe fn cs2000_ref_clk_bound_rate(p:*mut Cs2000Priv,rate:u32)->c_int { let v=if rate>=32000000&&rate<56000000{0}else if rate>=16000000&&rate<28000000{1}else if rate>=8000000&&rate<14000000{2}else{return -22}; regmap_update_bits((*p).regmap,FUNC_CFG1,REFCLKDIV_MASK,refclkdiv(v)) }
unsafe fn cs2000_clk_out_enable(p:*mut Cs2000Priv,en:bool)->c_int { regmap_update_bits((*p).regmap,DEVICE_CTRL,AUXOUTDIS|CLKOUTDIS,if en{0}else{AUXOUTDIS|CLKOUTDIS}) }
unsafe fn cs2000_wait_pll_lock(p:*mut Cs2000Priv)->c_int { for _ in 0..256 { let mut v=0; let r=regmap_read((*p).regmap,DEVICE_CTRL,&mut v); if r<0{return r}; if v&PLL_UNLOCK==0{return 0}; udelay(1) } -110 }
unsafe fn cs2000_enable(p:*mut Cs2000Priv)->c_int { let r=cs2000_enable_dev_config(p,true); if r<0{return r}; let r=cs2000_clk_out_enable(p,true); if r<0{return r}; cs2000_wait_pll_lock(p) }
unsafe fn cs2000_disable(p:*mut Cs2000Priv) { cs2000_enable_dev_config(p,false); cs2000_clk_out_enable(p,false); }
unsafe fn cs2000_determine_rate(p:*mut Cs2000Priv,req:*mut clk_rate_request)->c_int { let r=cs2000_rate_to_ratio((*req).best_parent_rate as u32,(*req).rate as u32,(*p).lf_ratio); (*req).rate=cs2000_ratio_to_rate(r,(*req).best_parent_rate as u32,(*p).lf_ratio); 0 }
unsafe fn cs2000_set_saved_rate(p:*mut Cs2000Priv)->c_int { cs2000_set_rate(p,(*p).saved_rate,(*p).saved_parent_rate) }
unsafe fn cs2000_remove(client:*mut i2c_client) { let p=i2c_get_clientdata(client); if !p.is_null(){clk_hw_unregister(&mut (*p).hw);} }
unsafe fn cs2000_resume(dev:*mut device)->c_int { cs2000_set_saved_rate(dev_get_drvdata(dev)) }
unsafe fn cs2000_probe(client:*mut i2c_client)->c_int { let p=devm_kzalloc(&mut (*client).dev,core::mem::size_of::<Cs2000Priv>(),0); if p.is_null(){return -12}; (*p).client=client; i2c_set_clientdata(client,p); (*p).regmap=devm_regmap_init_i2c(client,core::ptr::null()); if (*p).regmap.is_null(){return -22}; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
