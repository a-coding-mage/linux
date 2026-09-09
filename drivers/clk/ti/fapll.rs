// SPDX-License-Identifier: GPL-2.0-only

// Kernel dependencies supplied by the surrounding translation unit.

const FAPLL_MAIN_MULT_N_SHIFT: u32 = 16;
const FAPLL_MAIN_DIV_P_SHIFT: u32 = 8;
const FAPLL_MAIN_LOCK: u32 = 1 << 7;
const FAPLL_MAIN_PLLEN: u32 = 1 << 3;
const FAPLL_MAIN_BP: u32 = 1 << 2;
const FAPLL_MAIN_LOC_CTL: u32 = 1;
const FAPLL_MAIN_MAX_MULT_N: u32 = 0xffff;
const FAPLL_MAIN_MAX_DIV_P: u32 = 0xff;
const FAPLL_MAIN_CLEAR_MASK: u32 =
    (FAPLL_MAIN_MAX_MULT_N << FAPLL_MAIN_MULT_N_SHIFT)
    | (FAPLL_MAIN_DIV_P_SHIFT << FAPLL_MAIN_DIV_P_SHIFT)
    | FAPLL_MAIN_LOC_CTL;
const FAPLL_PWD_OFFSET: usize = 4;
const MAX_FAPLL_OUTPUTS: usize = 7;
const FAPLL_MAX_RETRIES: i32 = 1000;
const SYNTH_LDMDIV1: u32 = 1 << 8;
const SYNTH_LDFREQ: u32 = 1 << 31;
const SYNTH_PHASE_K: u64 = 8;
const SYNTH_MAX_INT_DIV: u64 = 0xf;
const SYNTH_MAX_DIV_M: u32 = 0xff;

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub ops: *const clk_ops, pub name: *const i8, pub parent_names: *const *const i8, pub num_parents: u32 }
#[repr(C)] pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
}
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_onecell_data { pub clks: *mut *mut clk, pub clk_num: u32 }

extern "C" {
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn udelay(usecs: u32);
    fn pr_err(fmt: *const i8, ...);
    fn pr_warn(fmt: *const i8, ...);
    fn clk_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
    fn clk_register_clkdev(clk: *mut clk, con_id: *const i8, dev_id: *const i8);
    fn clk_put(clk: *mut clk);
    fn iounmap(addr: *mut u8);
    fn of_clk_get_parent_count(node: *mut device_node) -> u32;
    fn of_clk_parent_fill(node: *mut device_node, names: *mut *const i8, count: u32);
    fn of_clk_get(node: *mut device_node, index: u32) -> *mut clk;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_property_read_string_index(node: *mut device_node, prop: *const i8, index: u32, out: *mut *const i8) -> i32;
    fn of_property_read_u32_index(node: *mut device_node, prop: *const i8, index: u32, out: *mut i32) -> i32;
    fn ti_dt_clk_name(node: *mut device_node) -> *const i8;
    fn of_clk_add_provider(node: *mut device_node, get: *const core::ffi::c_void, data: *mut core::ffi::c_void) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut core::ffi::c_void);
}

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const ETIMEDOUT: i32 = 110;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
struct fapll_data {
    hw: clk_hw, base: *mut u8, name: *const i8, clk_ref: *mut clk,
    clk_bypass: *mut clk, outputs: clk_onecell_data, bypass_bit_inverted: bool,
}
#[repr(C)]
struct fapll_synth {
    hw: clk_hw, fd: *mut fapll_data, index: i32, freq: *mut u8, div: *mut u8,
    name: *const i8, clk_pll: *mut clk,
}

unsafe fn to_fapll(hw: *mut clk_hw) -> *mut fapll_data { hw as *mut fapll_data }
unsafe fn to_synth(hw: *mut clk_hw) -> *mut fapll_synth { hw as *mut fapll_synth }
unsafe fn read(addr: *mut u8) -> u32 { readl_relaxed(addr) }
unsafe fn write(v: u32, addr: *mut u8) { writel_relaxed(v, addr) }
unsafe fn div_round_up_u64(n: u64, d: u64) -> u64 { (n + d - 1) / d }

unsafe fn ti_fapll_clock_is_bypass(fd: *mut fapll_data) -> bool {
    let v = read((*fd).base);
    if (*fd).bypass_bit_inverted { (v & FAPLL_MAIN_BP) == 0 } else { (v & FAPLL_MAIN_BP) != 0 }
}
unsafe fn ti_fapll_set_bypass(fd: *mut fapll_data) { let mut v = read((*fd).base); if (*fd).bypass_bit_inverted { v &= !FAPLL_MAIN_BP } else { v |= FAPLL_MAIN_BP }; write(v, (*fd).base); }
unsafe fn ti_fapll_clear_bypass(fd: *mut fapll_data) { let mut v = read((*fd).base); if (*fd).bypass_bit_inverted { v |= FAPLL_MAIN_BP } else { v &= !FAPLL_MAIN_BP }; write(v, (*fd).base); }
unsafe fn ti_fapll_wait_lock(fd: *mut fapll_data) -> i32 {
    let mut retries = FAPLL_MAX_RETRIES;
    loop { let v = read((*fd).base); if v == 0 { break } if v & FAPLL_MAIN_LOCK != 0 { return 0 } if retries <= 0 { break } retries -= 1; udelay(1); }
    pr_err(b"%s failed to lock\0".as_ptr() as *const i8, (*fd).name); -ETIMEDOUT
}
unsafe extern "C" fn ti_fapll_enable(hw: *mut clk_hw) -> i32 { let fd = to_fapll(hw); let mut v = read((*fd).base); v |= FAPLL_MAIN_PLLEN; write(v, (*fd).base); ti_fapll_wait_lock(fd); 0 }
unsafe extern "C" fn ti_fapll_disable(hw: *mut clk_hw) { let fd = to_fapll(hw); let mut v = read((*fd).base); v &= !FAPLL_MAIN_PLLEN; write(v, (*fd).base); }
unsafe extern "C" fn ti_fapll_is_enabled(hw: *mut clk_hw) -> i32 { let fd = to_fapll(hw); (read((*fd).base) & FAPLL_MAIN_PLLEN) as i32 }
unsafe extern "C" fn ti_fapll_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize { let fd = to_fapll(hw); if ti_fapll_clock_is_bypass(fd) { return parent_rate; } let v = read((*fd).base); let p = (v >> 8) & 0xff; let n = v >> 16; let mut rate = parent_rate as u64; if p != 0 { rate /= p as u64; } if n != 0 { rate *= n as u64; } rate as usize }
unsafe extern "C" fn ti_fapll_get_parent(hw: *mut clk_hw) -> u8 { if ti_fapll_clock_is_bypass(to_fapll(hw)) { 1 } else { 0 } }
unsafe fn ti_fapll_set_div_mult(rate: usize, parent_rate: usize, pre: *mut u32, mult: *mut u32) -> i32 { if rate < parent_rate { pr_warn(b"FAPLL main divider rates unsupported\0".as_ptr() as *const i8); return -EINVAL; } *mult = (rate / parent_rate) as u32; if *mult > FAPLL_MAIN_MAX_MULT_N { return -EINVAL; } *pre = 1; 0 }
unsafe extern "C" fn ti_fapll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { if (*req).rate == 0 { return -EINVAL; } let mut p=0; let mut n=0; let e=ti_fapll_set_div_mult((*req).rate,(*req).best_parent_rate,&mut p,&mut n); if e != 0 { (*req).rate=e as usize; return 0; } (*req).rate=(*req).best_parent_rate/p as usize*n as usize; 0 }
unsafe extern "C" fn ti_fapll_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 { if rate==0{return -EINVAL}; let fd=to_fapll(hw); let mut p=0;let mut n=0;let e=ti_fapll_set_div_mult(rate,parent_rate,&mut p,&mut n);if e!=0{return e};ti_fapll_set_bypass(fd);let mut v=read((*fd).base);v&=!FAPLL_MAIN_CLEAR_MASK;v|=p<<FAPLL_MAIN_DIV_P_SHIFT;v|=n<<FAPLL_MAIN_MULT_N_SHIFT;write(v,(*fd).base);if ti_fapll_is_enabled(hw)!=0{ti_fapll_wait_lock(fd);}ti_fapll_clear_bypass(fd);0 }

static TI_FAPLL_OPS: clk_ops = clk_ops { enable:Some(ti_fapll_enable), disable:Some(ti_fapll_disable), is_enabled:Some(ti_fapll_is_enabled), recalc_rate:Some(ti_fapll_recalc_rate), get_parent:Some(ti_fapll_get_parent), determine_rate:Some(ti_fapll_determine_rate), set_rate:Some(ti_fapll_set_rate) };

unsafe extern "C" fn ti_fapll_synth_enable(hw:*mut clk_hw)->i32{let s=to_synth(hw);let a=(*s).fd as *mut u8;let mut v=read(a.add(FAPLL_PWD_OFFSET));v&=!(1u32<<(*s).index);write(v,a.add(FAPLL_PWD_OFFSET));0}
unsafe extern "C" fn ti_fapll_synth_disable(hw:*mut clk_hw){let s=to_synth(hw);let a=(*s).fd as *mut u8;let mut v=read(a.add(FAPLL_PWD_OFFSET));v|=1u32<<(*s).index;write(v,a.add(FAPLL_PWD_OFFSET));}
unsafe extern "C" fn ti_fapll_synth_is_enabled(hw:*mut clk_hw)->i32{let s=to_synth(hw);let a=(*s).fd as *mut u8;((read(a.add(FAPLL_PWD_OFFSET))&(1u32<<(*s).index))==0) as i32}
unsafe extern "C" fn ti_fapll_synth_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize{let s=to_synth(hw);if (*s).div.is_null(){return 32768}if ti_fapll_clock_is_bypass((*s).fd){return parent_rate}let mut rate=parent_rate as u64;if !(*s).freq.is_null(){let v=read((*s).freq);let intd=((v>>24)&0xf) as u64;let frac=(v&0xffffff) as u64;let df=intd*10000000+frac;rate=rate*10000000/df* SYNTH_PHASE_K;}let m=(read((*s).div)&SYNTH_MAX_DIV_M) as u64;div_round_up_u64(rate,m) as usize}
unsafe fn ti_fapll_synth_get_frac_rate(hw:*mut clk_hw,parent_rate:usize)->usize{let s=to_synth(hw);let m=(read((*s).div)&SYNTH_MAX_DIV_M) as usize;ti_fapll_synth_recalc_rate(hw,parent_rate)*m}
unsafe fn ti_fapll_synth_set_frac_rate(s:*mut fapll_synth,rate:usize,parent_rate:usize)->u32{let mut m=div_round_up_u64(parent_rate as u64*SYNTH_PHASE_K,rate as u64)/SYNTH_MAX_INT_DIV;if m>SYNTH_MAX_DIV_M as u64{return EINVAL as u32}if m==0{m=1}let(mut id,mut fd)=(0u64,0u64);while m<SYNTH_MAX_DIV_M as u64{id=div_round_up_u64(parent_rate as u64*SYNTH_PHASE_K*10000000,rate as u64*m);fd=id%10000000;id/=10000000;if id<=SYNTH_MAX_INT_DIV{break}m+=1}if id>SYNTH_MAX_INT_DIV{return EINVAL as u32}let mut v=read((*s).freq);v&=!0x1fffffff;v|=((id as u32)&0xf)<<24;v|=(fd as u32)&0xffffff;v|=SYNTH_LDFREQ;write(v,(*s).freq);m as u32}
unsafe extern "C" fn ti_fapll_synth_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32{let s=to_synth(hw);if ti_fapll_clock_is_bypass((*s).fd)||(*s).div.is_null()||(*req).rate==0{return -EINVAL}let r=if (*s).freq.is_null(){let f=ti_fapll_synth_get_frac_rate(hw,(*req).best_parent_rate);div_round_up_u64(f as u64,div_round_up_u64(f as u64,(*req).rate as u64))}else{let x=(*req).best_parent_rate as u64*SYNTH_PHASE_K;if (*req).rate as u64>x{x}else if (*req).rate as u64<div_round_up_u64(x,SYNTH_MAX_INT_DIV*SYNTH_MAX_DIV_M as u64){div_round_up_u64(x,SYNTH_MAX_INT_DIV*SYNTH_MAX_DIV_M as u64)}else{(*req).rate as u64}};(*req).rate=r as usize;0}
unsafe extern "C" fn ti_fapll_synth_set_rate(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32{let s=to_synth(hw);if ti_fapll_clock_is_bypass((*s).fd)||(*s).div.is_null()||rate==0{return -EINVAL}let f=ti_fapll_synth_get_frac_rate(hw,parent_rate);let mut post=0u32;let mut post_rate=0usize;if f<rate{if (*s).freq.is_null(){return -EINVAL}}else{post=div_round_up_u64(f as u64,rate as u64) as u32;if post!=0&&post<=SYNTH_MAX_DIV_M{post_rate=div_round_up_u64(f as u64,post as u64) as usize}if (*s).freq.is_null()&&post_rate==0{return -EINVAL}}if post_rate!=rate&&!(*s).freq.is_null(){post=ti_fapll_synth_set_frac_rate(s,rate,parent_rate)}let mut v=read((*s).div);v&=!SYNTH_MAX_DIV_M;v|=post;v|=SYNTH_LDMDIV1;write(v,(*s).div);0}

static TI_FAPLL_SYNTH_OPS:clk_ops=clk_ops{enable:Some(ti_fapll_synth_enable),disable:Some(ti_fapll_synth_disable),is_enabled:Some(ti_fapll_synth_is_enabled),recalc_rate:Some(ti_fapll_synth_recalc_rate),get_parent:None,determine_rate:Some(ti_fapll_synth_determine_rate),set_rate:Some(ti_fapll_synth_set_rate)};

unsafe extern "C" fn ti_fapll_setup(node:*mut device_node){
    let fd=kzalloc(core::mem::size_of::<fapll_data>(),GFP_KERNEL) as *mut fapll_data;if fd.is_null(){return}
    (*fd).outputs.clks=kzalloc(core::mem::size_of::<*mut clk>()*(MAX_FAPLL_OUTPUTS+1),GFP_KERNEL) as *mut *mut clk;
    if (*fd).outputs.clks.is_null(){kfree(fd as *mut _);return}
    let init=kzalloc(core::mem::size_of::<clk_init_data>(),GFP_KERNEL) as *mut clk_init_data;if init.is_null(){kfree((*fd).outputs.clks as *mut _);kfree(fd as *mut _);return}
    (*init).ops=&TI_FAPLL_OPS;let name=ti_dt_clk_name(node);(*init).name=name;let mut parents:[*const i8;2]=[core::ptr::null();2];let np=of_clk_get_parent_count(node);if np!=2{pr_err(b"%pOFn must have two parents\0".as_ptr() as *const i8,node);return}of_clk_parent_fill(node,parents.as_mut_ptr(),2);(*init).parent_names=parents.as_ptr();(*init).num_parents=2;
    (*fd).clk_ref=of_clk_get(node,0);(*fd).clk_bypass=of_clk_get(node,1);(*fd).base=of_iomap(node,0);if (*fd).base.is_null(){return}(*fd).name=name;(*fd).hw.init=init;
    let pll=clk_register(core::ptr::null_mut(),&mut (*fd).hw);if pll.is_null(){iounmap((*fd).base);return}*(*fd).outputs.clks=pll;(*fd).outputs.clk_num+=1;
    let mut i=0;while i<MAX_FAPLL_OUTPUTS{let mut out: *const i8=core::ptr::null();if of_property_read_string_index(node,b"clock-output-names\0".as_ptr() as *const i8,i as u32,&mut out)!=0{i+=1;continue}let mut inst=i as i32;if of_property_read_u32_index(node,b"clock-indices\0".as_ptr() as *const i8,i as u32,&mut inst)!=0{inst=i as i32}let freq=(*fd).base.add((inst as usize)*8);let div=freq.add(4);let mut f=freq;if (freq as usize&0xffff)==0x04a8{f=core::ptr::null_mut()}else if read(freq)==0{f=core::ptr::null_mut()}let s=kzalloc(core::mem::size_of::<fapll_synth>(),GFP_KERNEL) as *mut fapll_synth;if !s.is_null(){(*s).hw.init=&{clk_init_data{ops:&TI_FAPLL_SYNTH_OPS,name:out,parent_names:&name,num_parents:1}};(*s).fd=fd;(*s).index=inst;(*s).freq=f;(*s).div=if f.is_null(){core::ptr::null_mut()}else{div};(*s).name=out;(*s).clk_pll=pll;let c=clk_register(core::ptr::null_mut(),&mut (*s).hw);if !c.is_null(){*(*fd).outputs.clks.add(inst as usize)=c;(*fd).outputs.clk_num+=1;clk_register_clkdev(c,out,core::ptr::null())}}i+=1}
    of_clk_add_provider(node,core::ptr::null(),&mut (*fd).outputs as *mut _ as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
