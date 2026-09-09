// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmp mix(div and mux) clock operation source file
 *
 * Copyright (C) 2014 Marvell
 * Chao Xie <chao.xie@marvell.com>
 */

// Linux clock-provider, allocation, I/O, error, and local clk.h dependencies.

use core::ffi::{c_char, c_void};

#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub flags: u64, pub parent_names: *const *const c_char, pub num_parents: u8, pub ops: *const clk_ops }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize, pub best_parent_hw: *mut clk_hw }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct mmp_clk_mix_reg_info { pub reg_clk_ctrl: *mut u32, pub reg_clk_sel: *mut u32, pub width_div: u8, pub shift_div: u8, pub width_mux: u8, pub shift_mux: u8, pub bit_fc: u8 }
#[repr(C)] pub struct mmp_clk_mix_clk_table { pub parent_index: u8, pub rate: usize, pub divisor: usize, pub valid: u8 }
#[repr(C)] pub struct mmp_clk_mix_config { pub reg_info: mmp_clk_mix_reg_info, pub table: *const mmp_clk_mix_clk_table, pub table_size: u32, pub mux_table: *const u32, pub div_flags: u32, pub mux_flags: u32 }
#[repr(C)] pub struct mmp_clk_mix { pub hw: clk_hw, pub reg_info: mmp_clk_mix_reg_info, pub table: *mut mmp_clk_mix_clk_table, pub table_size: u32, pub mux_table: *mut u32, pub div_flags: u32, pub mux_flags: u32, pub lock: *mut spinlock_t, pub r#type: u32 }
#[repr(C)] pub struct clk_ops { pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->i32>, pub set_rate_and_parent: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize,u8)->i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->i32>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw,u8)->i32>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize)->usize>, pub init: Option<unsafe extern "C" fn(*mut clk_hw)->i32> }

extern "C" {
    fn clk_hw_get_num_parents(hw: *mut clk_hw) -> i32; fn clk_hw_get_parent_by_index(hw: *mut clk_hw, i: u8) -> *mut clk_hw; fn clk_hw_get_rate(hw: *mut clk_hw) -> usize; fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn readl(p: *mut u32) -> u32; fn writel(v: u32, p: *mut u32); fn clk_register(d: *mut device, hw: *mut clk_hw) -> *mut clk;
    fn spin_lock_irqsave(l: *mut spinlock_t, f: *mut usize); fn spin_unlock_irqrestore(l: *mut spinlock_t, f: usize);
    fn kfree(p: *mut c_void); fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
}

const CLK_DIVIDER_ONE_BASED:u32=1; const CLK_DIVIDER_POWER_OF_TWO:u32=2; const CLK_MUX_INDEX_BIT:u32=1; const CLK_MUX_INDEX_ONE:u32=2;
const MMP_CLK_MIX_TYPE_V1:u32=1; const MMP_CLK_MIX_TYPE_V2:u32=2; const MMP_CLK_MIX_TYPE_V3:u32=3; const CLK_GET_RATE_NOCACHE:u64=1;
const EINVAL:i32=22; const ENOMEM:i32=12; const EBUSY:i32=16;
unsafe fn bits_mask(w:u8,s:u8)->u32 { ((1u32.wrapping_shl(w as u32)).wrapping_sub(1)) << s }
unsafe fn bits_set(v:u32,w:u8,s:u8)->u32 { (v & ((1u32.wrapping_shl(w as u32)).wrapping_sub(1))) << s }
unsafe fn bits_get(v:u32,w:u8,s:u8)->u32 { (v >> s) & ((1u32.wrapping_shl(w as u32)).wrapping_sub(1)) }
unsafe fn mix(p:*mut clk_hw)->*mut mmp_clk_mix { p as *mut mmp_clk_mix }

unsafe fn _get_maxdiv(m:*mut mmp_clk_mix)->u32 { let mask=(1u32<<(*m).reg_info.width_div)-1; if (*m).div_flags&CLK_DIVIDER_ONE_BASED!=0{return mask} if (*m).div_flags&CLK_DIVIDER_POWER_OF_TWO!=0{return 1<<mask} if !(*m).table.is_null(){let mut p=(*m).table;let mut x=0;while (*p).div!=0{if (*p).div>x{x=(*p).div}p=p.add(1)}return x} mask+1 }
unsafe fn _get_div(m:*mut mmp_clk_mix,v:u32)->u32 { if (*m).div_flags&CLK_DIVIDER_ONE_BASED!=0{return v} if (*m).div_flags&CLK_DIVIDER_POWER_OF_TWO!=0{return 1<<v} if !(*m).table.is_null(){let mut p=(*m).table;while (*p).div!=0{if (*p).val==v{return (*p).div}p=p.add(1)}return 0} v+1 }
unsafe fn _get_mux(m:*mut mmp_clk_mix,v:u32)->u32 { if (*m).mux_flags&CLK_MUX_INDEX_BIT!=0{return v.trailing_zeros()} if (*m).mux_flags&CLK_MUX_INDEX_ONE!=0{return v-1} if !(*m).mux_table.is_null(){let n=clk_hw_get_num_parents(&mut (*m).hw);for i in 0..n{if *(*m).mux_table.add(i as usize)==v{return i as u32}}return 0}v }
unsafe fn _get_div_val(m:*mut mmp_clk_mix,d:u32)->u32 {if (*m).div_flags&CLK_DIVIDER_ONE_BASED!=0{return d}if (*m).div_flags&CLK_DIVIDER_POWER_OF_TWO!=0{return d.trailing_zeros()}if !(*m).table.is_null(){let mut p=(*m).table;while (*p).div!=0{if (*p).div==d{return (*p).val}p=p.add(1)}return 0}d-1}
unsafe fn _get_mux_val(m:*mut mmp_clk_mix,x:u8)->u32 {if !(*m).mux_table.is_null(){*(*m).mux_table.add(x as usize)}else{x as u32}}

unsafe fn _filter_clk_table(m:*mut mmp_clk_mix,t:*mut mmp_clk_mix_clk_table,n:u32){for i in 0..n{let x=&mut *t.add(i as usize);let p=clk_hw_get_parent_by_index(&mut (*m).hw,x.parent_index);let r=clk_hw_get_rate(p);if r%x.rate!=0{x.valid=0}else{x.divisor=r/x.rate;x.valid=1}}}
unsafe fn _set_rate(m:*mut mmp_clk_mix,mux:u32,div:u32,cm:u32,cd:u32)->i32{if cm==0&&cd==0{return -EINVAL}let mut f=0; if !(*m).lock.is_null(){spin_lock_irqsave((*m).lock,&mut f)}let r=&(*m).reg_info;let mut x=if (*m).r#type==MMP_CLK_MIX_TYPE_V1||(*m).r#type==MMP_CLK_MIX_TYPE_V2{readl(r.reg_clk_ctrl)}else{readl(r.reg_clk_sel)};if cd{x&=!bits_mask(r.width_div,r.shift_div);x|=bits_set(div,r.width_div,r.shift_div)}if cm{x&=!bits_mask(r.width_mux,r.shift_mux);x|=bits_set(mux,r.width_mux,r.shift_mux)}if (*m).r#type==MMP_CLK_MIX_TYPE_V1{writel(x,r.reg_clk_ctrl)}else if (*m).r#type==MMP_CLK_MIX_TYPE_V2{writel(x|(1<<r.bit_fc),r.reg_clk_ctrl);let mut t=50;while t!=0&&readl(r.reg_clk_ctrl)&(1<<r.bit_fc)!=0{t-=1}if t==0{if !(*m).lock.is_null(){spin_unlock_irqrestore((*m).lock,f)}return -EBUSY}}else{let mut q=readl(r.reg_clk_ctrl)|(1<<r.bit_fc);writel(q,r.reg_clk_ctrl);writel(x,r.reg_clk_sel);q&=!(1<<r.bit_fc)}if !(*m).lock.is_null(){spin_unlock_irqrestore((*m).lock,f)}0}

unsafe extern "C" fn mmp_clk_mix_get_parent(hw:*mut clk_hw)->u8{let m=mix(hw);let r=&(*m).reg_info;let x=if (*m).r#type==MMP_CLK_MIX_TYPE_V1||(*m).r#type==MMP_CLK_MIX_TYPE_V2{readl(r.reg_clk_ctrl)}else{readl(r.reg_clk_sel)};_get_mux(m,bits_get(x,r.width_mux,r.shift_mux)) as u8}
unsafe extern "C" fn mmp_clk_mix_recalc_rate(hw:*mut clk_hw,pr:usize)->usize{let m=mix(hw);let r=&(*m).reg_info;let x=if (*m).r#type==MMP_CLK_MIX_TYPE_V1||(*m).r#type==MMP_CLK_MIX_TYPE_V2{readl(r.reg_clk_ctrl)}else{readl(r.reg_clk_sel)};pr/(_get_div(m,bits_get(x,r.width_div,r.shift_div)) as usize)}
unsafe extern "C" fn mmp_clk_mix_set_rate_and_parent(hw:*mut clk_hw,rate:usize,pr:usize,index:u8)->i32{let m=mix(hw);_set_rate(m,_get_mux_val(m,index),_get_div_val(m,(pr/rate) as u32),1,1)}
unsafe extern "C" fn mmp_clk_set_parent(hw:*mut clk_hw,index:u8)->i32{let m=mix(hw);if !(*m).table.is_null(){for i in 0..(*m).table_size{let x=&*(*m).table.add(i as usize);if x.valid!=0&&x.parent_index==index{return _set_rate(m,_get_mux_val(m,index),_get_div_val(m,x.divisor as u32),1,1)}}return -EINVAL}_set_rate(m,_get_mux_val(m,index),0,1,0)}
unsafe extern "C" fn mmp_clk_set_rate(hw:*mut clk_hw,rate:usize,pr:usize)->i32{let m=mix(hw);let d=(pr/rate)as u32;if !(*m).table.is_null(){for i in 0..(*m).table_size{let x=&*(*m).table.add(i as usize);let p=clk_hw_get_parent_by_index(hw,x.parent_index);if x.valid!=0&&clk_hw_get_rate(p)==pr&&x.divisor as u32==d{return _set_rate(m,_get_mux_val(m,x.parent_index),_get_div_val(m,d),1,1)}}return -EINVAL}for i in 0..clk_hw_get_num_parents(hw){if clk_hw_get_rate(clk_hw_get_parent_by_index(hw,i as u8))==pr{return _set_rate(m,_get_mux_val(m,i as u8),_get_div_val(m,d),1,1)}}-EINVAL}
unsafe extern "C" fn mmp_clk_mix_init(hw:*mut clk_hw)->i32{let m=mix(hw);if !(*m).table.is_null(){_filter_clk_table(m,(*m).table,(*m).table_size)}0}
unsafe extern "C" fn mmp_clk_mix_determine_rate(hw:*mut clk_hw,q:*mut clk_rate_request)->i32{let m=mix(hw);let mut best=usize::MAX;let mut bp=core::ptr::null_mut();let mut br=0;let mut rr=0;if !(*m).table.is_null(){for i in 0..(*m).table_size{let x=&*(*m).table.add(i as usize);if x.valid==0{continue}let p=clk_hw_get_parent_by_index(hw,x.parent_index);let r=clk_hw_get_rate(p);let z=r/x.divisor;let g=if z>(*q).rate{z-(*q).rate}else{(*q).rate-z};if bp.is_null()||g<best{bp=p;br=r;rr=z;best=g}}}else{for i in 0..clk_hw_get_num_parents(hw){let p=clk_hw_get_parent_by_index(hw,i as u8);let r=clk_hw_get_rate(p);for j in 0.._get_maxdiv(m){let z=r/(_get_div(m,j)as usize);let g=if z>(*q).rate{z-(*q).rate}else{(*q).rate-z};if bp.is_null()||g<best{bp=p;br=r;rr=z;best=g}}}}if bp.is_null(){return -EINVAL}(*q).best_parent_rate=br;(*q).best_parent_hw=bp;(*q).rate=rr;0}

pub static mmp_clk_mix_ops: clk_ops=clk_ops{determine_rate:Some(mmp_clk_mix_determine_rate),set_rate_and_parent:Some(mmp_clk_mix_set_rate_and_parent),set_rate:Some(mmp_clk_set_rate),set_parent:Some(mmp_clk_set_parent),get_parent:Some(mmp_clk_mix_get_parent),recalc_rate:Some(mmp_clk_mix_recalc_rate),init:Some(mmp_clk_mix_init)};

pub unsafe extern "C" fn mmp_clk_register_mix(_dev:*mut device,_name:*const c_char,_parents:*const *const c_char,_num:u8,_flags:u64,_config:*mut mmp_clk_mix_config,_lock:*mut spinlock_t)->*mut clk { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
