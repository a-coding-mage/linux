// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2013, The Linux Foundation. All rights reserved. */
// External Linux/kernel declarations and constants are supplied by the surrounding translation.

unsafe fn ns_to_src(s: *mut src_sel, mut ns: u32) -> u32 { ns >>= (*s).src_sel_shift; ns &= SRC_SEL_MASK; ns }
unsafe fn src_to_ns(s: *mut src_sel, src: u8, mut ns: u32) -> u32 { let mut mask = SRC_SEL_MASK << (*s).src_sel_shift; ns &= !mask; ns |= (src as u32) << (*s).src_sel_shift; ns }

unsafe fn clk_rcg_get_parent(hw: *mut clk_hw) -> u8 {
    let rcg = to_clk_rcg(hw); let num_parents = clk_hw_get_num_parents(hw); let mut ns=0; let ret=regmap_read((*rcg).clkr.regmap,(*rcg).ns_reg,&mut ns); if ret != 0 { pr_debug("%s: Clock %s has invalid parent, using default.\n", "clk_rcg_get_parent", clk_hw_get_name(hw)); return 0; }
    ns=ns_to_src(&mut (*rcg).s,ns); for i in 0..num_parents { if ns==(*rcg).s.parent_map[i as usize].cfg { return i as u8; } } pr_debug("%s: Clock %s has invalid parent, using default.\n", "clk_rcg_get_parent", clk_hw_get_name(hw)); 0
}
unsafe fn reg_to_bank(rcg:*mut clk_dyn_rcg, mut bank:u32)->i32 { bank &= 1u32<<(*rcg).mux_sel_bit; if bank != 0 {1} else {0} }
unsafe fn clk_dyn_rcg_get_parent(hw:*mut clk_hw)->u8 { let rcg=to_clk_dyn_rcg(hw); let np=clk_hw_get_num_parents(hw); let mut ns=0; let mut reg=0; let ret=regmap_read((*rcg).clkr.regmap,(*rcg).bank_reg,&mut reg); if ret!=0 {return 0} let bank=reg_to_bank(rcg,reg) as usize; let s=&mut (*rcg).s[bank]; if regmap_read((*rcg).clkr.regmap,(*rcg).ns_reg[bank],&mut ns)!=0{return 0}; ns=ns_to_src(s,ns); for i in 0..np {if ns==s.parent_map[i as usize].cfg{return i as u8}} 0 }
unsafe fn clk_rcg_set_parent(hw:*mut clk_hw,index:u8)->i32 { let r=to_clk_rcg(hw); let mut ns=0; regmap_read((*r).clkr.regmap,(*r).ns_reg,&mut ns); ns=src_to_ns(&mut (*r).s,(*r).s.parent_map[index as usize].cfg,ns); regmap_write((*r).clkr.regmap,(*r).ns_reg,ns); 0 }
unsafe fn md_to_m(mn:*mut mn,mut md:u32)->u32 {md>>=(*mn).m_val_shift;md&=(1u32<<(*mn).width)-1;md}
unsafe fn ns_to_pre_div(p:*mut pre_div,mut ns:u32)->u32 {ns>>=(*p).pre_div_shift;ns&=(1u32<<(*p).pre_div_width)-1;ns}
unsafe fn pre_div_to_ns(p:*mut pre_div,pre:u8,mut ns:u32)->u32 {let mask=((1u32<<(*p).pre_div_width)-1)<<(*p).pre_div_shift;ns&=!mask;ns|=(pre as u32)<<(*p).pre_div_shift;ns}
unsafe fn mn_to_md(mn:*mut mn,m:u32,n:u32,mut md:u32)->u32 {let w=(1u32<<(*mn).width)-1;let mask=(w<<(*mn).m_val_shift)|w;md&=!mask;if n!=0{md|=m<<(*mn).m_val_shift;md|=!n&w;}md}
unsafe fn ns_m_to_n(mn:*mut mn,ns:u32,m:u32)->u32 {((!ns>>(*mn).n_val_shift)&((1u32<<(*mn).width)-1))+m}
unsafe fn reg_to_mnctr_mode(mn:*mut mn,mut v:u32)->u32 {v>>=(*mn).mnctr_mode_shift;v&MNCTR_MODE_MASK}
unsafe fn mn_to_ns(mn:*mut mn,m:u32,n:u32,mut ns:u32)->u32 {let mask=((1u32<<(*mn).width)-1)<<(*mn).n_val_shift;ns&=!mask;if n!=0{ns|=((!(n-m)&((1u32<<(*mn).width)-1))<<(*mn).n_val_shift)}ns}
unsafe fn mn_to_reg(mn:*mut mn,m:u32,n:u32,mut v:u32)->u32 {let mask=(MNCTR_MODE_MASK<<(*mn).mnctr_mode_shift)|(1u32<<(*mn).mnctr_en_bit);v&=!mask;if n!=0{v|=1u32<<(*mn).mnctr_en_bit;v|=MNCTR_MODE_DUAL<<(*mn).mnctr_mode_shift;}v}

unsafe fn calc_rate(mut rate:usize,m:u32,n:u32,mode:u32,pre:u32)->usize {if pre!=0{rate/=pre as usize+1}if mode!=0{rate=rate.wrapping_mul(m as usize)/n as usize}rate}
unsafe fn clk_rcg_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize {let r=to_clk_rcg(hw);let mut ns=0;regmap_read((*r).clkr.regmap,(*r).ns_reg,&mut ns);let pre=ns_to_pre_div(&mut (*r).p,ns);let(mut m,mut n,mut mode)=(0,0,0);if (*r).mn.width!=0{let mut md=0;regmap_read((*r).clkr.regmap,(*r).md_reg,&mut md);m=md_to_m(&mut (*r).mn,md);n=ns_m_to_n(&mut (*r).mn,ns,m);if (*r).clkr.enable_reg!=(*r).ns_reg{regmap_read((*r).clkr.regmap,(*r).clkr.enable_reg,&mut mode)}else{mode=ns}mode=reg_to_mnctr_mode(&mut (*r).mn,mode)}calc_rate(parent_rate,m,n,mode,pre)}

#[repr(C)] pub struct frac_entry { pub num:i32, pub den:i32 }
pub static pixel_table:[frac_entry;5]=[frac_entry{num:1,den:1},frac_entry{num:1,den:2},frac_entry{num:1,den:3},frac_entry{num:3,den:16},frac_entry{num:0,den:0}];

// The remaining clock-operation entry points preserve the C ABI-facing implementation shape.
unsafe fn clk_rcg_set_rate(hw:*mut clk_hw,rate:usize,_parent:usize)->i32 {let r=to_clk_rcg(hw);let f=qcom_find_freq((*r).freq_tbl,rate);if f.is_null(){return -22}__clk_rcg_set_rate(r,f)}
unsafe fn __clk_rcg_set_rate(r:*mut clk_rcg,f:*const freq_tbl)->i32 {let mut ns=0;let mut md=0;let mut ctl=0;let mn=&mut (*r).mn;let mut mask=0;let reset=if (*mn).reset_in_cc{(*r).clkr.enable_reg}else{(*r).ns_reg};if (*mn).width!=0{mask=1u32<<(*mn).mnctr_reset_bit;regmap_update_bits((*r).clkr.regmap,reset,mask,mask);regmap_read((*r).clkr.regmap,(*r).md_reg,&mut md);regmap_write((*r).clkr.regmap,(*r).md_reg,mn_to_md(mn,(*f).m,(*f).n,md));regmap_read((*r).clkr.regmap,(*r).ns_reg,&mut ns);if (*r).clkr.enable_reg!=(*r).ns_reg{regmap_read((*r).clkr.regmap,(*r).clkr.enable_reg,&mut ctl);regmap_write((*r).clkr.regmap,(*r).clkr.enable_reg,mn_to_reg(mn,(*f).m,(*f).n,ctl))}else{ns=mn_to_reg(mn,(*f).m,(*f).n,ns)}ns=mn_to_ns(mn,(*f).m,(*f).n,ns)}else{regmap_read((*r).clkr.regmap,(*r).ns_reg,&mut ns)}ns=pre_div_to_ns(&mut (*r).p,(*f).pre_div-1,ns);regmap_write((*r).clkr.regmap,(*r).ns_reg,ns);regmap_update_bits((*r).clkr.regmap,reset,mask,0);0}

unsafe fn clk_rcg_set_floor_rate(hw:*mut clk_hw,rate:usize,_:usize)->i32{let r=to_clk_rcg(hw);let f=qcom_find_freq_floor((*r).freq_tbl,rate);if f.is_null(){-22}else{__clk_rcg_set_rate(r,f)}}
unsafe fn clk_rcg_bypass_set_rate(hw:*mut clk_hw,_:usize,_:usize)->i32{let r=to_clk_rcg(hw);__clk_rcg_set_rate(r,(*r).freq_tbl)}
unsafe fn clk_rcg_bypass2_set_rate(hw:*mut clk_hw,_:usize,_:usize)->i32{let r=to_clk_rcg(hw);let mut ns=0;if regmap_read((*r).clkr.regmap,(*r).ns_reg,&mut ns)!=0{return -1}let src=ns_to_src(&mut (*r).s,ns);let mut f=freq_tbl::zero();f.pre_div=ns_to_pre_div(&mut (*r).p,ns)+1;for i in 0..clk_hw_get_num_parents(hw){if src==(*r).s.parent_map[i as usize].cfg{f.src=(*r).s.parent_map[i as usize].src;return __clk_rcg_set_rate(r,&f)}}-22}
unsafe fn clk_rcg_bypass2_set_rate_and_parent(hw:*mut clk_hw,r:usize,p:usize,_:u8)->i32{clk_rcg_bypass2_set_rate(hw,r,p)}
unsafe fn clk_rcg_pixel_set_rate(hw:*mut clk_hw,rate:usize,parent:usize)->i32{let r=to_clk_rcg(hw);let mut f=freq_tbl::zero();f.pre_div=1;for x in pixel_table.iter(){if x.num==0{break}let request=rate*x.den as usize/x.num as usize;if parent>=request.saturating_sub(100000)&&parent<=request+100000{f.m=x.num as u32;f.n=x.den as u32;return __clk_rcg_set_rate(r,&f)}}-22}
unsafe fn clk_rcg_pixel_set_rate_and_parent(hw:*mut clk_hw,r:usize,p:usize,_:u8)->i32{clk_rcg_pixel_set_rate(hw,r,p)}
unsafe fn clk_rcg_esc_set_rate(hw:*mut clk_hw,rate:usize,parent:usize)->i32{let r=to_clk_rcg(hw);if rate==0{return -22}let d=parent/rate;if d>=1&&d<=1usize<<(*r).p.pre_div_width{let mut f=freq_tbl::zero();f.pre_div=d as u32;return __clk_rcg_set_rate(r,&f)}-22}
unsafe fn clk_rcg_esc_set_rate_and_parent(hw:*mut clk_hw,r:usize,p:usize,_:u8)->i32{clk_rcg_esc_set_rate(hw,r,p)}
unsafe fn clk_rcg_lcc_enable(hw:*mut clk_hw)->i32{let r=to_clk_rcg(hw);regmap_update_bits((*r).clkr.regmap,(*r).ns_reg,1<<10,1<<10)}
unsafe fn clk_rcg_lcc_disable(hw:*mut clk_hw){let r=to_clk_rcg(hw);regmap_update_bits((*r).clkr.regmap,(*r).ns_reg,1<<10,0);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
