// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Copyright (c) 2013 Linaro Ltd.
 * Author: Thomas Abraham <thomas.ab@linaro.org>
 *
 * Rust translation of the Samsung common clock registration helpers.
 * Linux headers and types are supplied by the surrounding kernel bindings.
 */

static mut CLOCK_REG_CACHE_LIST: ListHead = ListHead::new();

pub unsafe fn samsung_clk_save(base: *mut core::ffi::c_void, regmap: *mut Regmap,
    mut rd: *mut SamsungClkRegDump, mut num_regs: u32) {
    while num_regs > 0 {
        if !base.is_null() { (*rd).value = readl(base.add((*rd).offset as usize)); }
        else if !regmap.is_null() { regmap_read(regmap, (*rd).offset, &mut (*rd).value); }
        num_regs -= 1; rd = rd.add(1);
    }
}

pub unsafe fn samsung_clk_restore(base: *mut core::ffi::c_void, regmap: *mut Regmap,
    mut rd: *const SamsungClkRegDump, mut num_regs: u32) {
    while num_regs > 0 {
        if !base.is_null() { writel((*rd).value, base.add((*rd).offset as usize)); }
        else if !regmap.is_null() { regmap_write(regmap, (*rd).offset, (*rd).value); }
        num_regs -= 1; rd = rd.add(1);
    }
}

pub unsafe fn samsung_clk_alloc_reg_dump(rdump: *const usize, nr_rdump: usize) -> *mut SamsungClkRegDump {
    let rd = kzalloc_objs::<SamsungClkRegDump>(nr_rdump);
    if rd.is_null() { return core::ptr::null_mut(); }
    for i in 0..nr_rdump { (*rd.add(i)).offset = *rdump.add(i); }
    rd
}

pub unsafe fn samsung_clk_init(dev: *mut Device, base: *mut core::ffi::c_void,
    nr_clks: usize) -> *mut SamsungClkProvider {
    let ctx = kzalloc_flex::<SamsungClkProvider>(nr_clks);
    if ctx.is_null() { panic!("could not allocate clock provider context.\n"); }
    (*ctx).clk_data.num = nr_clks;
    for i in 0..nr_clks { (*ctx).clk_data.hws[i] = err_ptr(-ENOENT); }
    (*ctx).dev = dev; (*ctx).reg_base = base; spin_lock_init(&mut (*ctx).lock); ctx
}

pub unsafe fn samsung_clk_of_add_provider(np: *mut DeviceNode, ctx: *mut SamsungClkProvider) {
    if !np.is_null() && of_clk_add_hw_provider(np, of_clk_hw_onecell_get, &mut (*ctx).clk_data) != 0 {
        panic!("could not register clk provider\n");
    }
}

pub unsafe fn samsung_clk_add_lookup(ctx: *mut SamsungClkProvider, hw: *mut ClkHw, id: u32) {
    if id != 0 { (*ctx).clk_data.hws[id as usize] = hw; }
}

pub unsafe fn samsung_clk_register_alias(ctx: *mut SamsungClkProvider, mut list: *const SamsungClockAlias, nr_clk: u32) {
    for idx in 0..nr_clk { if (*list).id == 0 { pr_err!("{}: clock id missing for index {}\n", "samsung_clk_register_alias", idx); list=list.add(1); continue; }
        let hw = (*ctx).clk_data.hws[(*list).id as usize];
        if hw.is_null() { pr_err!("{}: failed to find clock {}\n", "samsung_clk_register_alias", (*list).id); list=list.add(1); continue; }
        if clk_hw_register_clkdev(hw, (*list).alias, (*list).dev_name) != 0 { pr_err!("{}: failed to register lookup {}\n", "samsung_clk_register_alias", (*list).alias); } list=list.add(1); }
}

pub unsafe fn samsung_clk_register_fixed_rate(ctx: *mut SamsungClkProvider, mut list: *const SamsungFixedRateClock, nr: u32) {
    for _ in 0..nr { let hw=clk_hw_register_fixed_rate((*ctx).dev,(*list).name,(*list).parent_name,(*list).flags,(*list).fixed_rate); if is_err(hw) { pr_err!("failed to register clock {}\n",(*list).name); } else { samsung_clk_add_lookup(ctx,hw,(*list).id); } list=list.add(1); }
}
pub unsafe fn samsung_clk_register_fixed_factor(ctx: *mut SamsungClkProvider, mut list: *const SamsungFixedFactorClock, nr: u32) {
    for _ in 0..nr { let hw=clk_hw_register_fixed_factor((*ctx).dev,(*list).name,(*list).parent_name,(*list).flags,(*list).mult,(*list).div); if !is_err(hw) { samsung_clk_add_lookup(ctx,hw,(*list).id); } else { pr_err!("failed to register clock {}\n",(*list).name); } list=list.add(1); }
}
pub unsafe fn samsung_clk_register_mux(ctx:*mut SamsungClkProvider,mut list:*const SamsungMuxClock,nr:u32){for _ in 0..nr{let hw=clk_hw_register_mux((*ctx).dev,(*list).name,(*list).parent_names,(*list).num_parents,(*list).flags,(*ctx).reg_base.add((*list).offset),(*list).shift,(*list).width,(*list).mux_flags,&mut (*ctx).lock);if !is_err(hw){samsung_clk_add_lookup(ctx,hw,(*list).id);}else{pr_err!("failed to register clock {}\n",(*list).name);}list=list.add(1);}}
pub unsafe fn samsung_clk_register_div(ctx:*mut SamsungClkProvider,mut list:*const SamsungDivClock,nr:u32){for _ in 0..nr{let hw=if !(*list).table.is_null(){clk_hw_register_divider_table((*ctx).dev,(*list).name,(*list).parent_name,(*list).flags,(*ctx).reg_base.add((*list).offset),(*list).shift,(*list).width,(*list).div_flags,(*list).table,&mut (*ctx).lock)}else{clk_hw_register_divider((*ctx).dev,(*list).name,(*list).parent_name,(*list).flags,(*ctx).reg_base.add((*list).offset),(*list).shift,(*list).width,(*list).div_flags,&mut (*ctx).lock)};if !is_err(hw){samsung_clk_add_lookup(ctx,hw,(*list).id);}else{pr_err!("failed to register clock {}\n",(*list).name);}list=list.add(1);}}

pub unsafe fn samsung_is_auto_capable(np:*mut DeviceNode)->bool{let mut res=Resource::default();if of_address_to_resource(np,0,&mut res)!=0{return false;}if resource_size(&res)!=0x10000{pr_warn!("incorrect res size for automatic clocks\n");return false;}true}
const ACG_MSK:u32=genmask(6,4); const CLK_IDLE:u32=genmask(5,4);
unsafe fn samsung_auto_clk_gate_is_en(hw:*mut ClkHw)->i32{let gate=to_clk_gate(hw);if (readl((*gate).reg)&ACG_MSK)==CLK_IDLE{0}else{1}}
unsafe fn samsung_auto_clk_gate_en(_: *mut ClkHw)->i32{0} unsafe fn samsung_auto_clk_gate_dis(_: *mut ClkHw){}

pub unsafe fn samsung_register_auto_gate(dev:*mut Device,np:*mut DeviceNode,name:*const i8,parent_name:*const i8,parent_hw:*const ClkHw,parent_data:*const ClkParentData,flags:usize,reg:*mut core::ffi::c_void,bit_idx:u8,gate_flags:u8,lock:*mut Spinlock)->*mut ClkHw{let gate=kzalloc_obj::<ClkGate>();if gate.is_null(){return err_ptr(-ENOMEM);}let mut init=ClkInitData::default();init.name=name;init.ops=&SAMSUNG_AUTO_CLK_GATE_OPS;init.flags=flags;init.parent_names=if !parent_name.is_null(){&parent_name}else{core::ptr::null()};init.parent_hws=if !parent_hw.is_null(){&parent_hw}else{core::ptr::null()};init.parent_data=parent_data;init.num_parents=if !parent_name.is_null()||!parent_hw.is_null()||!parent_data.is_null(){1}else{0};(*gate).reg=reg;(*gate).bit_idx=bit_idx;(*gate).flags=gate_flags;(*gate).lock=lock;(*gate).hw.init=&init;let hw=&mut (*gate).hw;if (if !dev.is_null()||np.is_null(){clk_hw_register(dev,hw)}else{of_clk_hw_register(np,hw)})!=0{kfree(gate);return err_ptr(-EINVAL);}hw}

pub unsafe fn samsung_clk_register_gate(ctx:*mut SamsungClkProvider,mut list:*const SamsungGateClock,nr:u32){for _ in 0..nr{let reg=(*ctx).reg_base.add((*list).offset);let hw=if (*ctx).auto_clock_gate&&(*ctx).gate_dbg_offset!=0{samsung_register_auto_gate((*ctx).dev,core::ptr::null_mut(),(*list).name,(*list).parent_name,core::ptr::null(),core::ptr::null(),(*list).flags,reg.add((*ctx).gate_dbg_offset),(*list).bit_idx,(*list).gate_flags,&mut (*ctx).lock)}else{clk_hw_register_gate((*ctx).dev,(*list).name,(*list).parent_name,(*list).flags,reg,(*list).bit_idx,(*list).gate_flags,&mut (*ctx).lock)};if !is_err(hw){samsung_clk_add_lookup(ctx,hw,(*list).id);}else{pr_err!("failed to register clock {}\n",(*list).name);}list=list.add(1);}}

pub unsafe fn samsung_clk_of_register_fixed_ext(ctx:*mut SamsungClkProvider, fixed:*mut SamsungFixedRateClock,nr:u32,matches:*const OfDeviceId){let mut np=core::ptr::null_mut();let mut m=core::ptr::null();while for_each_matching_node_and_match(&mut np,matches,&mut m){let mut freq=0;if of_property_read_u32(np,b"clock-frequency\0".as_ptr() as *const i8,&mut freq)==0{(*fixed.add((*m).data as usize)).fixed_rate=freq;} }samsung_clk_register_fixed_rate(ctx,fixed,nr);}

pub unsafe fn samsung_cmu_register_clocks(ctx:*mut SamsungClkProvider,cmu:*const SamsungCmuInfo,np:*mut DeviceNode){if (*cmu).auto_clock_gate&&samsung_is_auto_capable(np){(*ctx).auto_clock_gate=true;}(*ctx).gate_dbg_offset=(*cmu).gate_dbg_offset;(*ctx).option_offset=(*cmu).option_offset;(*ctx).drcg_offset=(*cmu).drcg_offset;(*ctx).memclk_offset=(*cmu).memclk_offset;if !(*cmu).pll_clks.is_null(){samsung_clk_register_pll(ctx,(*cmu).pll_clks,(*cmu).nr_pll_clks);}if !(*cmu).mux_clks.is_null(){samsung_clk_register_mux(ctx,(*cmu).mux_clks,(*cmu).nr_mux_clks);}if !(*cmu).div_clks.is_null(){samsung_clk_register_div(ctx,(*cmu).div_clks,(*cmu).nr_div_clks);}if !(*cmu).gate_clks.is_null(){samsung_clk_register_gate(ctx,(*cmu).gate_clks,(*cmu).nr_gate_clks);}if !(*cmu).fixed_clks.is_null(){samsung_clk_register_fixed_rate(ctx,(*cmu).fixed_clks,(*cmu).nr_fixed_clks);}if !(*cmu).fixed_factor_clks.is_null(){samsung_clk_register_fixed_factor(ctx,(*cmu).fixed_factor_clks,(*cmu).nr_fixed_factor_clks);}if !(*cmu).cpu_clks.is_null(){samsung_clk_register_cpu(ctx,(*cmu).cpu_clks,(*cmu).nr_cpu_clks);}}
const DRCG_EN_MSK:u32=0xffff_ffff;const MEMCLK_EN:u32=1;
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn samsung_clk_suspend(_: *mut core::ffi::c_void)->i32{let mut c=ListIter::<SamsungClockRegCache>::new(&mut CLOCK_REG_CACHE_LIST);while let Some(r)=c.next(){samsung_clk_save((*r).reg_base,(*r).sysreg,(*r).rdump,(*r).rd_num);samsung_clk_restore((*r).reg_base,(*r).sysreg,(*r).rsuspend,(*r).rsuspend_num);}0}
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn samsung_clk_resume(_: *mut core::ffi::c_void){let mut c=ListIter::<SamsungClockRegCache>::new(&mut CLOCK_REG_CACHE_LIST);while let Some(r)=c.next(){samsung_clk_restore((*r).reg_base,(*r).sysreg,(*r).rdump,(*r).rd_num);}}
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn samsung_clk_extended_sleep_init(reg_base:*mut core::ffi::c_void,sysreg:*mut Regmap,rdump:*const usize,nr_rdump:usize,rsuspend:*const SamsungClkRegDump,nr_rsuspend:usize){let c=kzalloc_flex::<SamsungClockRegCache>(nr_rdump);if c.is_null(){panic!("could not allocate register reg_cache.\n");}(*c).rd_num=nr_rdump;for i in 0..nr_rdump{(*c).rdump[i].offset=*rdump.add(i);}if list_empty(&mut CLOCK_REG_CACHE_LIST){register_syscore(&SAMSUNG_CLK_SYSCORE);}(*c).reg_base=reg_base;(*c).sysreg=sysreg;(*c).rsuspend=rsuspend;(*c).rsuspend_num=nr_rsuspend;list_add_tail(&mut (*c).node,&mut CLOCK_REG_CACHE_LIST);}
pub unsafe fn samsung_en_dyn_root_clk_gating(np:*mut DeviceNode,ctx:*mut SamsungClkProvider,cmu:*const SamsungCmuInfo,cmu_has_pm:bool){if !(*ctx).auto_clock_gate{return;}(*ctx).sysreg=syscon_regmap_lookup_by_phandle(np,b"samsung,sysreg\0".as_ptr() as *const i8);if is_err((*ctx).sysreg){pr_warn!("Unable to get CMU sysreg\n");(*ctx).sysreg=core::ptr::null_mut();}else{regmap_write((*ctx).sysreg,(*ctx).drcg_offset,DRCG_EN_MSK);if (*ctx).memclk_offset!=0{regmap_write_bits((*ctx).sysreg,(*ctx).memclk_offset,MEMCLK_EN,0);}if !cmu_has_pm{samsung_clk_extended_sleep_init(core::ptr::null_mut(),(*ctx).sysreg,(*cmu).sysreg_clk_regs,(*cmu).nr_sysreg_clk_regs,core::ptr::null(),0);}}}
pub unsafe fn samsung_cmu_register_one(np:*mut DeviceNode,cmu:*const SamsungCmuInfo)->*mut SamsungClkProvider{let base=of_iomap(np,0);if base.is_null(){panic!("failed to map registers\n");}let ctx=samsung_clk_init(core::ptr::null_mut(),base,(*cmu).nr_clk_ids);samsung_cmu_register_clocks(ctx,cmu,np);if !(*cmu).clk_regs.is_null(){samsung_clk_extended_sleep_init(base,core::ptr::null_mut(),(*cmu).clk_regs,(*cmu).nr_clk_regs,(*cmu).suspend_regs,(*cmu).nr_suspend_regs);}samsung_clk_of_add_provider(np,ctx);samsung_en_dyn_root_clk_gating(np,ctx,cmu,false);ctx}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
