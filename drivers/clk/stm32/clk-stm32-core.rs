// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) STMicroelectronics 2022 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
 */

// Linux clock, device, OF, IO, slab, spinlock, clk-stm32-core, and reset-stm32
// declarations are supplied by the surrounding translation unit.

static mut RLOCK: spinlock_t = spinlock_t::new();

unsafe fn stm32_rcc_clock_init(dev: *mut device, match_: *const of_device_id, base: *mut core::ffi::c_void) -> i32 {
    let data = (*match_).data as *const stm32_rcc_match_data;
    let mut clk_data = (*data).hw_clks;
    let mut hws: *mut *mut clk_hw;
    let max_binding = (*data).maxbinding;
    clk_data = devm_kzalloc(dev, struct_size(clk_data, max_binding), GFP_KERNEL);
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).num = max_binding;
    hws = (*clk_data).hws;
    for n in 0..max_binding { *hws.add(n as usize) = ERR_PTR(-ENOENT); }
    for n in 0..(*data).num_clocks {
        let cfg_clock = &*(*data).tab_clocks.add(n as usize);
        let mut hw: *mut clk_hw = ERR_PTR(-ENOENT);
        if !(*data).check_security.is_none() && ((*data).check_security.unwrap())((*dev).of_node, base, cfg_clock) != 0 { continue; }
        if let Some(func) = (*cfg_clock).func { hw = func(dev, data, base, &mut RLOCK, cfg_clock); }
        if IS_ERR(hw) { dev_err(dev, "Can't register clk %d: %ld\n", n, PTR_ERR(hw)); return PTR_ERR(hw) as i32; }
        if (*cfg_clock).id != NO_ID { *hws.add((*cfg_clock).id as usize) = hw; }
    }
    devm_of_clk_add_hw_provider(dev, Some(of_clk_hw_onecell_get), clk_data)
}

pub unsafe fn stm32_rcc_init(dev: *mut device, match_data: *const of_device_id, base: *mut core::ffi::c_void) -> i32 {
    let match_ = of_match_node(match_data, dev_of_node(dev));
    if match_.is_null() { dev_err(dev, "match data not found\n"); return -ENODEV; }
    let rcc_match_data = (*match_).data as *const stm32_rcc_match_data;
    let mut err = stm32_rcc_reset_init(dev, (*rcc_match_data).reset_data, base);
    if err != 0 { pr_err("stm32 reset failed to initialize\n"); return err; }
    err = stm32_rcc_clock_init(dev, match_, base);
    if err != 0 { pr_err("stm32 clock failed to initialize\n"); return err; }
    0
}

unsafe fn stm32_mux_get_parent(base: *mut u8, data: *mut clk_stm32_clock_data, mux_id: u16) -> u8 {
    let mux = &(*data).muxes[mux_id as usize];
    let mask = BIT((*mux).width) - 1;
    ((readl(base.add((*mux).offset as usize)) >> (*mux).shift) & mask) as u8
}

unsafe fn stm32_mux_set_parent(base: *mut u8, data: *mut clk_stm32_clock_data, mux_id: u16, index: u8) -> i32 {
    let mux = &(*data).muxes[mux_id as usize];
    let mask = BIT((*mux).width) - 1;
    let addr = base.add((*mux).offset as usize);
    let mut reg = readl(addr);
    reg &= !(mask << (*mux).shift); reg |= (index as u32) << (*mux).shift; writel(reg, addr); 0
}

unsafe fn stm32_gate_endisable(base: *mut u8, data: *mut clk_stm32_clock_data, gate_id: u16, enable: i32) {
    let gate = &(*data).gates[gate_id as usize]; let addr = base.add((*gate).offset as usize);
    if enable != 0 { if (*data).gate_cpt[gate_id as usize] > 0 { (*data).gate_cpt[gate_id as usize] += 1; return; } (*data).gate_cpt[gate_id as usize] += 1; if (*gate).set_clr != 0 { writel(BIT((*gate).bit_idx), addr); } else { writel(readl(addr) | BIT((*gate).bit_idx), addr); } }
    else { (*data).gate_cpt[gate_id as usize] -= 1; if (*data).gate_cpt[gate_id as usize] > 0 { return; } if (*gate).set_clr != 0 { writel(BIT((*gate).bit_idx), addr.add((*gate).set_clr as usize)); } else { writel(readl(addr) & !BIT((*gate).bit_idx), addr); } }
}

unsafe fn stm32_gate_disable_unused(base: *mut u8, data: *mut clk_stm32_clock_data, gate_id: u16) { let gate=&(*data).gates[gate_id as usize]; let addr=base.add((*gate).offset as usize); if (*data).gate_cpt[gate_id as usize]>0{return;} if (*gate).set_clr!=0{writel(BIT((*gate).bit_idx),addr.add((*gate).set_clr as usize));}else{writel(readl(addr)&!BIT((*gate).bit_idx),addr);} }
unsafe fn stm32_gate_is_enabled(base:*mut u8,data:*mut clk_stm32_clock_data,gate_id:u16)->i32{let gate=&(*data).gates[gate_id as usize]; if readl(base.add((*gate).offset as usize))&BIT((*gate).bit_idx)!=0{1}else{0}}

unsafe fn _get_table_div(table:*const clk_div_table, val:u32)->u32{let mut p=table;while (*p).div!=0{if (*p).val==val{return (*p).div;}p=p.add(1);}0}
unsafe fn _get_div(table:*const clk_div_table,val:u32,flags:ulong,_width:u8)->u32{if flags&CLK_DIVIDER_ONE_BASED!=0{return val;}if flags&CLK_DIVIDER_POWER_OF_TWO!=0{return 1<<val;}if !table.is_null(){return _get_table_div(table,val);}val+1}
unsafe fn stm32_divider_get_rate(base:*mut u8,data:*mut clk_stm32_clock_data,div_id:u16,parent_rate:ulong)->ulong{let d=&(*data).dividers[div_id as usize];let val=(readl(base.add(d.offset as usize))>>d.shift)&clk_div_mask(d.width);let div=_get_div(d.table,val,d.flags,d.width);if div==0{return parent_rate;}DIV_ROUND_UP_ULL(parent_rate as u64,div as u64) as ulong}
unsafe fn stm32_divider_set_rate(base:*mut u8,data:*mut clk_stm32_clock_data,div_id:u16,rate:ulong,parent_rate:ulong)->i32{let d=&(*data).dividers[div_id as usize];let value=divider_get_val(rate,parent_rate,d.table,d.width,d.flags);if value<0{return value;}let mut val;if d.flags&CLK_DIVIDER_HIWORD_MASK!=0{val=clk_div_mask(d.width)<<(d.shift+16);}else{val=readl(base.add(d.offset as usize));val&=!(clk_div_mask(d.width)<<d.shift);}val|=(value as u32)<<d.shift;writel(val,base.add(d.offset as usize));0}

pub const MUX_SAFE_POSITION: u32 = 0;

unsafe fn clk_stm32_mux_get_parent(hw:*mut clk_hw)->u8{let m=to_clk_stm32_mux(hw);stm32_mux_get_parent((*m).base,(*m).clock_data,(*m).mux_id)}
unsafe fn clk_stm32_mux_set_parent(hw:*mut clk_hw,index:u8)->i32{let m=to_clk_stm32_mux(hw);let mut flags=0;spin_lock_irqsave((*m).lock,&mut flags);stm32_mux_set_parent((*m).base,(*m).clock_data,(*m).mux_id,index);spin_unlock_irqrestore((*m).lock,flags);0}
pub static CLK_STM32_MUX_OPS: clk_ops = clk_ops{determine_rate:Some(__clk_mux_determine_rate),get_parent:Some(clk_stm32_mux_get_parent),set_parent:Some(clk_stm32_mux_set_parent)};

unsafe fn clk_stm32_gate_endisable(hw:*mut clk_hw,enable:i32){let g=to_clk_stm32_gate(hw);let mut flags=0;spin_lock_irqsave((*g).lock,&mut flags);stm32_gate_endisable((*g).base,(*g).clock_data,(*g).gate_id,enable);spin_unlock_irqrestore((*g).lock,flags)}
unsafe fn clk_stm32_gate_enable(hw:*mut clk_hw)->i32{clk_stm32_gate_endisable(hw,1);0}
unsafe fn clk_stm32_gate_disable(hw:*mut clk_hw){clk_stm32_gate_endisable(hw,0)}
unsafe fn clk_stm32_gate_is_enabled(hw:*mut clk_hw)->i32{let g=to_clk_stm32_gate(hw);stm32_gate_is_enabled((*g).base,(*g).clock_data,(*g).gate_id)}
unsafe fn clk_stm32_gate_disable_unused(hw:*mut clk_hw){let g=to_clk_stm32_gate(hw);let mut flags=0;spin_lock_irqsave((*g).lock,&mut flags);stm32_gate_disable_unused((*g).base,(*g).clock_data,(*g).gate_id);spin_unlock_irqrestore((*g).lock,flags)}
pub static CLK_STM32_GATE_OPS:clk_ops=clk_ops{enable:Some(clk_stm32_gate_enable),disable:Some(clk_stm32_gate_disable),is_enabled:Some(clk_stm32_gate_is_enabled),disable_unused:Some(clk_stm32_gate_disable_unused)};

unsafe fn clk_stm32_divider_recalc_rate(hw:*mut clk_hw,parent_rate:ulong)->ulong{let d=to_clk_stm32_divider(hw);if (*d).div_id==NO_STM32_DIV{return parent_rate;}stm32_divider_get_rate((*d).base,(*d).clock_data,(*d).div_id,parent_rate)}
unsafe fn clk_stm32_divider_set_rate(hw:*mut clk_hw,rate:ulong,parent_rate:ulong)->i32{let d=to_clk_stm32_divider(hw);if (*d).div_id==NO_STM32_DIV{return rate as i32;}let mut flags=0;spin_lock_irqsave((*d).lock,&mut flags);let ret=stm32_divider_set_rate((*d).base,(*d).clock_data,(*d).div_id,rate,parent_rate);spin_unlock_irqrestore((*d).lock,flags);ret}
pub static CLK_STM32_DIVIDER_OPS:clk_ops=clk_ops{recalc_rate:Some(clk_stm32_divider_recalc_rate),set_rate:Some(clk_stm32_divider_set_rate),determine_rate:Some(divider_determine_rate)};

unsafe fn register_common(dev:*mut device,hw:*mut clk_hw)->*mut clk_hw{let err=devm_clk_hw_register(dev,hw);if err!=0{ERR_PTR(err)}else{hw}}
pub unsafe fn clk_stm32_mux_register(dev:*mut device,data:*const stm32_rcc_match_data,base:*mut u8,lock:*mut spinlock_t,cfg:*const clock_config)->*mut clk_hw{let m=(*cfg).clock_cfg as *mut clk_stm32_mux;(*m).base=base;(*m).lock=lock;(*m).clock_data=(*data).clock_data;register_common(dev,&mut (*m).hw)}
pub unsafe fn clk_stm32_gate_register(dev:*mut device,data:*const stm32_rcc_match_data,base:*mut u8,lock:*mut spinlock_t,cfg:*const clock_config)->*mut clk_hw{let g=(*cfg).clock_cfg as *mut clk_stm32_gate;(*g).base=base;(*g).lock=lock;(*g).clock_data=(*data).clock_data;register_common(dev,&mut (*g).hw)}
pub unsafe fn clk_stm32_div_register(dev:*mut device,data:*const stm32_rcc_match_data,base:*mut u8,lock:*mut spinlock_t,cfg:*const clock_config)->*mut clk_hw{let d=(*cfg).clock_cfg as *mut clk_stm32_div;(*d).base=base;(*d).lock=lock;(*d).clock_data=(*data).clock_data;register_common(dev,&mut (*d).hw)}
pub unsafe fn clk_stm32_composite_register(dev:*mut device,data:*const stm32_rcc_match_data,base:*mut u8,lock:*mut spinlock_t,cfg:*const clock_config)->*mut clk_hw{let c=(*cfg).clock_cfg as *mut clk_stm32_composite;(*c).base=base;(*c).lock=lock;(*c).clock_data=(*data).clock_data;register_common(dev,&mut (*c).hw)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
