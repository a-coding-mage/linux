// SPDX-License-Identifier: GPL-2.0-or-later
/* Ingenic SoC CGU driver */

const MHZ: u64 = 1000 * 1000;

#[inline]
unsafe fn to_clk_info(clk: *mut ingenic_clk) -> *const ingenic_cgu_clk_info {
    &(*(*clk).cgu).clock_info.add((*clk).idx as usize)
}

#[inline]
unsafe fn ingenic_cgu_gate_get(cgu: *mut ingenic_cgu, info: *const ingenic_cgu_gate_info) -> bool {
    ((readl((*cgu).base.add((*info).reg as usize)) & BIT((*info).bit)) != 0) ^ (*info).clear_to_gate
}

#[inline]
unsafe fn ingenic_cgu_gate_set(cgu: *mut ingenic_cgu, info: *const ingenic_cgu_gate_info, val: bool) {
    let mut clkgr = readl((*cgu).base.add((*info).reg as usize));
    if val ^ (*info).clear_to_gate { clkgr |= BIT((*info).bit); } else { clkgr &= !BIT((*info).bit)); }
    writel(clkgr, (*cgu).base.add((*info).reg as usize));
}

unsafe fn ingenic_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_ingenic_clk(hw); let info = to_clk_info(clk); let cgu = (*clk).cgu; let p = &(*info).pll;
    let mut ctl = readl((*cgu).base.add(p.reg as usize));
    let m = ((ctl >> p.m_shift) & GENMASK(p.m_bits - 1, 0)) + p.m_offset;
    let n = ((ctl >> p.n_shift) & GENMASK(p.n_bits - 1, 0)) + p.n_offset;
    let mut od_enc = 0; if p.od_bits > 0 { od_enc = (ctl >> p.od_shift) & GENMASK(p.od_bits - 1, 0); }
    if p.bypass_bit >= 0 { ctl = readl((*cgu).base.add(p.bypass_reg as usize)); if (ctl & BIT(p.bypass_bit as u32)) != 0 { return parent_rate; } }
    let mut od = 0; while od < p.od_max && p.od_encoding[od as usize] != od_enc { od += 1; }
    if p.od_max == 0 { BUG_ON(p.od_bits != 0); } else { BUG_ON(od == p.od_max); } od += 1;
    div_u64(parent_rate as u64 * m as u64 * p.rate_multiplier as u64, n as u64 * od as u64) as c_ulong
}

unsafe fn ingenic_pll_calc_m_n_od(p: *const ingenic_cgu_pll_info, rate: c_ulong, parent_rate: c_ulong, pm: *mut u32, pn: *mut u32, pod: *mut u32) {
    let od = 1; let mut n = parent_rate / (10 * MHZ as c_ulong); n = min(n, 1u64 << (*p).n_bits) as c_ulong; n = max(n, (*p).n_offset as c_ulong);
    let mut m = (rate / MHZ as c_ulong) * od as c_ulong * n / (parent_rate / MHZ as c_ulong); m = min(m, 1u64 << (*p).m_bits) as c_ulong; m = max(m, (*p).m_offset as c_ulong);
    *pm = m as u32; *pn = n as u32; *pod = od;
}

unsafe fn ingenic_pll_calc(info: *const ingenic_cgu_clk_info, rate: c_ulong, parent_rate: c_ulong, pm: *mut u32, pn: *mut u32, pod: *mut u32) -> c_ulong {
    let p = &(*info).pll; let (mut m, mut n, mut od) = (0, 0, 0);
    if let Some(f) = p.calc_m_n_od { f(p, rate, parent_rate, &mut m, &mut n, &mut od); } else { ingenic_pll_calc_m_n_od(p, rate, parent_rate, &mut m, &mut n, &mut od); }
    if !pm.is_null() { *pm = m; } if !pn.is_null() { *pn = n; } if !pod.is_null() { *pod = od; }
    div_u64(parent_rate as u64 * m as u64 * p.rate_multiplier as u64, n as u64 * od as u64) as c_ulong
}

unsafe fn ingenic_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int { let c = to_ingenic_clk(hw); (*req).rate = ingenic_pll_calc(to_clk_info(c), (*req).rate, (*req).best_parent_rate, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); 0 }

#[inline] unsafe fn ingenic_pll_check_stable(cgu: *mut ingenic_cgu, p: *const ingenic_cgu_pll_info) -> c_int { if (*p).stable_bit < 0 { 0 } else { let mut ctl = 0; readl_poll_timeout((*cgu).base.add((*p).reg as usize), &mut ctl, ctl & BIT((*p).stable_bit as u32) != 0, 0, 100 * USEC_PER_MSEC) } }

unsafe fn ingenic_pll_set_rate(hw: *mut clk_hw, req_rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let c = to_ingenic_clk(hw); let cgu = (*c).cgu; let info = to_clk_info(c); let p = &(*info).pll; let mut m=0; let mut n=0; let mut od=0; let rate=ingenic_pll_calc(info,req_rate,parent_rate,&mut m,&mut n,&mut od); let mut flags=0; let mut ret=0;
    spin_lock_irqsave(&mut (*cgu).lock,&mut flags); let mut ctl=readl((*cgu).base.add(p.reg as usize)); ctl &= !(GENMASK(p.m_bits-1,0)<<p.m_shift); ctl |= (m-p.m_offset)<<p.m_shift; ctl &= !(GENMASK(p.n_bits-1,0)<<p.n_shift); ctl |= (n-p.n_offset)<<p.n_shift; if p.od_bits>0 { ctl &= !(GENMASK(p.od_bits-1,0)<<p.od_shift); ctl |= p.od_encoding[(od-1) as usize]<<p.od_shift; } writel(ctl,(*cgu).base.add(p.reg as usize)); if let Some(f)=p.set_rate_hook { f(p,rate,parent_rate); } if p.enable_bit>=0 && ctl&BIT(p.enable_bit as u32)!=0 { ret=ingenic_pll_check_stable(cgu,p); } spin_unlock_irqrestore(&mut (*cgu).lock,flags); ret
}

unsafe fn ingenic_pll_enable(hw:*mut clk_hw)->c_int { let c=to_ingenic_clk(hw); let g=(*c).cgu; let p=&(*to_clk_info(c)).pll; if p.enable_bit<0{return 0} let mut f=0; spin_lock_irqsave(&mut (*g).lock,&mut f); if p.bypass_bit>=0 { let mut x=readl((*g).base.add(p.bypass_reg as usize)); x&=!BIT(p.bypass_bit as u32); writel(x,(*g).base.add(p.bypass_reg as usize)); } let mut x=readl((*g).base.add(p.reg as usize)); x|=BIT(p.enable_bit as u32); writel(x,(*g).base.add(p.reg as usize)); let r=ingenic_pll_check_stable(g,p); spin_unlock_irqrestore(&mut (*g).lock,f); r }
unsafe fn ingenic_pll_disable(hw:*mut clk_hw){let c=to_ingenic_clk(hw);let g=(*c).cgu;let p=&(*to_clk_info(c)).pll;if p.enable_bit<0{return}let mut f=0;spin_lock_irqsave(&mut(*g).lock,&mut f);let mut x=readl((*g).base.add(p.reg as usize));x&=!BIT(p.enable_bit as u32);writel(x,(*g).base.add(p.reg as usize));spin_unlock_irqrestore(&mut(*g).lock,f)}
unsafe fn ingenic_pll_is_enabled(hw:*mut clk_hw)->c_int{let c=to_ingenic_clk(hw);let g=(*c).cgu;let p=&(*to_clk_info(c)).pll;if p.enable_bit<0{return 1}let x=readl((*g).base.add(p.reg as usize));if x&BIT(p.enable_bit as u32)!=0{1}else{0}}

// The remaining non-PLL clock operations and registration routines are direct translations.
// External kernel structures, constants, helpers, and callbacks are supplied by the surrounding crate.

unsafe fn ingenic_clk_get_parent(hw:*mut clk_hw)->u8{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;let mut idx=0;if (*i).typ&CGU_CLK_MUX!=0{let r=readl((*g).base.add((*i).mux.reg as usize));let h=((r>>(*i).mux.shift)&GENMASK((*i).mux.bits-1,0)) as u8;for n in 0..h{if (*i).parents[n as usize]!=-1{idx+=1}}}idx}
unsafe fn ingenic_clk_set_parent(hw:*mut clk_hw,idx:u8)->c_int{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;if (*i).typ&CGU_CLK_MUX!=0{let mut h=0;let mut cur=0;let np=1<<(*i).mux.bits;while h<np{if (*i).parents[h as usize]!=-1{if cur==idx{break}cur+=1}h+=1}BUG_ON(cur!=idx);let mask=GENMASK((*i).mux.bits-1,0)<<(*i).mux.shift;let mut f=0;spin_lock_irqsave(&mut(*g).lock,&mut f);let mut r=readl((*g).base.add((*i).mux.reg as usize));r&=!mask;r|=h<<(*i).mux.shift;writel(r,(*g).base.add((*i).mux.reg as usize));spin_unlock_irqrestore(&mut(*g).lock,f);return 0}if idx!=0{-EINVAL}else{0}}
unsafe fn ingenic_clk_recalc_rate(hw:*mut clk_hw,parent:c_ulong)->c_ulong{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;let mut rate=parent;if (*i).typ&CGU_CLK_DIV!=0{let p=ingenic_clk_get_parent(hw);if (*i).div.bypass_mask&BIT(p as u32)==0{let r=readl((*g).base.add((*i).div.reg as usize));let mut d=(r>>(*i).div.shift)&GENMASK((*i).div.bits-1,0);if !(*i).div.div_table.is_null(){d=*(*i).div.div_table.add(d as usize)}else{d=(d+1)*(*i).div.div}rate/=d as c_ulong}}else if (*i).typ&CGU_CLK_FIXDIV!=0{rate/=(*i).fixdiv.div as c_ulong}rate}
unsafe fn ingenic_clk_calc_hw_div(i:*const ingenic_cgu_clk_info,div:u32)->u32{let mut bi=0;let mut best=u32::MAX;for n in 0..(1<<(*i).div.bits){let d=*(*i).div.div_table.add(n as usize);if d==0{break}if d>=div&&d<best{best=d;bi=n;if div==best{break}}}bi}
unsafe fn ingenic_clk_calc_div(hw:*mut clk_hw,i:*const ingenic_cgu_clk_info,parent:c_ulong,req:c_ulong)->u32{let p=ingenic_clk_get_parent(hw);if (*i).div.bypass_mask&BIT(p as u32)!=0{return 1}let mut d=((parent+req-1)/req) as u32;if !(*i).div.div_table.is_null(){return *(*i).div.div_table.add(ingenic_clk_calc_hw_div(i,d) as usize)}d=d.clamp((*i).div.div,(*i).div.div<<(*i).div.bits);d=((d+(*i).div.div-1)/(*i).div.div)*(*i).div.div;d}
unsafe fn ingenic_clk_determine_rate(hw:*mut clk_hw,r:*mut clk_rate_request)->c_int{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let mut d=1;if (*i).typ&CGU_CLK_DIV!=0{d=ingenic_clk_calc_div(hw,i,(*r).best_parent_rate,(*r).rate)}else if (*i).typ&CGU_CLK_FIXDIV!=0{d=(*i).fixdiv.div}else if clk_hw_can_set_rate_parent(hw){(*r).best_parent_rate=(*r).rate}(*r).rate=((*r).best_parent_rate+d as c_ulong-1)/d as c_ulong;0}
unsafe fn ingenic_clk_check_stable(g:*mut ingenic_cgu,i:*const ingenic_cgu_clk_info)->c_int{let mut r=0;readl_poll_timeout((*g).base.add((*i).div.reg as usize),&mut r,r&BIT((*i).div.busy_bit as u32)==0,0,100*USEC_PER_MSEC)}
unsafe fn ingenic_clk_set_rate(hw:*mut clk_hw,req:c_ulong,parent:c_ulong)->c_int{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;if (*i).typ&CGU_CLK_DIV==0{return -EINVAL}let d=ingenic_clk_calc_div(hw,i,parent,req);if (parent+d as c_ulong-1)/d as c_ulong!=req{return -EINVAL}let hd=if !(*i).div.div_table.is_null(){ingenic_clk_calc_hw_div(i,d)}else{d/(*i).div.div-1};let mut f=0;spin_lock_irqsave(&mut(*g).lock,&mut f);let mut r=readl((*g).base.add((*i).div.reg as usize));let m=GENMASK((*i).div.bits-1,0);r&=!(m<<(*i).div.shift);r|=hd<<(*i).div.shift;if (*i).div.stop_bit!=-1{r&=!BIT((*i).div.stop_bit as u32)}if (*i).div.ce_bit!=-1{r|=BIT((*i).div.ce_bit as u32)}writel(r,(*g).base.add((*i).div.reg as usize));let ret=if (*i).div.busy_bit!=-1{ingenic_clk_check_stable(g,i)}else{0};spin_unlock_irqrestore(&mut(*g).lock,f);ret}
unsafe fn ingenic_clk_enable(hw:*mut clk_hw)->c_int{let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;if (*i).typ&CGU_CLK_GATE!=0{let mut f=0;spin_lock_irqsave(&mut(*g).lock,&mut f);ingenic_cgu_gate_set(g,&(*i).gate,false);spin_unlock_irqrestore(&mut(*g).lock,f);if (*i).gate.delay_us!=0{udelay((*i).gate.delay_us)}}0}
unsafe fn ingenic_clk_disable(hw:*mut clk_hw){let c=to_ingenic_clk(hw);let i=to_clk_info(c);let g=(*c).cgu;if (*i).typ&CGU_CLK_GATE!=0{let mut f=0;spin_lock_irqsave(&mut(*g).lock,&mut f);ingenic_cgu_gate_set(g,&(*i).gate,true);spin_unlock_irqrestore(&mut(*g).lock,f)}}
unsafe fn ingenic_clk_is_enabled(hw:*mut clk_hw)->c_int{let c=to_ingenic_clk(hw);let i=to_clk_info(c);if (*i).typ&CGU_CLK_GATE!=0&&!ingenic_cgu_gate_get((*c).cgu,&(*i).gate){1}else if (*i).typ&CGU_CLK_GATE!=0{0}else{1}}

// Registration/setup declarations and operations retain the source interfaces.
unsafe fn ingenic_register_clock(cgu:*mut ingenic_cgu,idx:u32)->c_int {
    let info=&(*cgu).clock_info.add(idx as usize); let mut clk_init=core::mem::zeroed::<clk_init_data>(); let mut parent_names:[*const c_char;4]=[core::ptr::null();4]; let mut c:*mut ingenic_clk=core::ptr::null_mut();
    if (*info).typ==CGU_CLK_EXT { let clk=of_clk_get_by_name((*cgu).np,(*info).name); if IS_ERR(clk){return -ENODEV} let e=clk_register_clkdev(clk,(*info).name,core::ptr::null());if e!=0{clk_put(clk);return e}(*cgu).clocks.clks.add(idx as usize).write(clk);return 0 }
    if (*info).typ==0{return -EINVAL} c=kzalloc_obj::<ingenic_clk>();if c.is_null(){return -ENOMEM}(*c).cgu=cgu;(*c).idx=idx;(*c).hw.init=&mut clk_init;clk_init.name=(*info).name;clk_init.parent_names=parent_names.as_mut_ptr();let mut caps=(*info).typ;if caps&CGU_CLK_DIV!=0{caps&=!CGU_CLK_DIV}else if caps&CGU_CLK_CUSTOM==0{clk_init.flags|=CLK_SET_RATE_PARENT}
    if caps&(CGU_CLK_MUX|CGU_CLK_CUSTOM)!=0{let n=if caps&CGU_CLK_MUX!=0{1<<(*info).mux.bits}else{4};for j in 0..n{if (*info).parents[j as usize]!=-1{parent_names[clk_init.num_parents as usize]=__clk_get_name(*(*cgu).clocks.clks.add((*info).parents[j as usize] as usize));clk_init.num_parents+=1}}}else{parent_names[0]=__clk_get_name(*(*cgu).clocks.clks.add((*info).parents[0] as usize));clk_init.num_parents=1}
    if caps&CGU_CLK_CUSTOM!=0{clk_init.ops=(*info).custom.clk_ops;caps&=!CGU_CLK_CUSTOM}else if caps&CGU_CLK_PLL!=0{clk_init.ops=&ingenic_pll_ops;caps&=!CGU_CLK_PLL}else{clk_init.ops=&ingenic_clk_ops}caps&=!(CGU_CLK_GATE|CGU_CLK_FIXDIV);if caps&CGU_CLK_MUX!=0{if caps&CGU_CLK_MUX_GLITCHFREE==0{clk_init.flags|=CLK_SET_PARENT_GATE}caps&=!(CGU_CLK_MUX|CGU_CLK_MUX_GLITCHFREE)}if caps!=0{kfree(c);return -EINVAL}let clk=clk_register(core::ptr::null_mut(),&mut(*c).hw);if IS_ERR(clk){kfree(c);return PTR_ERR(clk)}let e=clk_register_clkdev(clk,(*info).name,core::ptr::null());if e!=0{return e}(*cgu).clocks.clks.add(idx as usize).write(clk);0
}

pub unsafe fn ingenic_cgu_new(info:*const ingenic_cgu_clk_info,n:u32,np:*mut device_node)->*mut ingenic_cgu{let c=kzalloc_obj::<ingenic_cgu>();if c.is_null(){return core::ptr::null_mut()}(*c).base=of_iomap(np,0);if (*c).base.is_null(){kfree(c);return core::ptr::null_mut()}(*c).np=np;(*c).clock_info=info;(*c).clocks.clk_num=n;spin_lock_init(&mut(*c).lock);c}
pub unsafe fn ingenic_cgu_register_clocks(c:*mut ingenic_cgu)->c_int{(*c).clocks.clks=kzalloc_objs::<*mut clk>((*c).clocks.clk_num);if (*c).clocks.clks.is_null(){return -ENOMEM}for i in 0..(*c).clocks.clk_num{let e=ingenic_register_clock(c,i);if e!=0{return e}}of_clk_add_provider((*c).np,of_clk_src_onecell_get,&mut(*c).clocks)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
