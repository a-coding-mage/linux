// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare eDMA v0 core
 *
 * Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
 */

#[repr(u32)]
enum DwEdmaControl {
    DW_EDMA_V0_CB = 1 << 0,
    DW_EDMA_V0_TCB = 1 << 1,
    DW_EDMA_V0_LLP = 1 << 2,
    DW_EDMA_V0_LIE = 1 << 3,
    DW_EDMA_V0_RIE = 1 << 4,
    DW_EDMA_V0_CCS = 1 << 8,
    DW_EDMA_V0_LLE = 1 << 9,
}

const EDMA_V0_FUNC_NUM_MASK: u32 = 0x1f000;

#[inline]
unsafe fn __dw_regs(dw: *mut dw_edma) -> *mut dw_edma_v0_regs {
    (*(*dw).chip).reg_base
}

unsafe fn set_32(dw: *mut dw_edma, name: *mut u32, value: u32) {
    writel(value, name);
}
unsafe fn get_32(_dw: *mut dw_edma, name: *const u32) -> u32 { readl(name) }

unsafe fn __dw_ch_regs(dw: *mut dw_edma, dir: enum_dw_edma_dir, ch: u16) -> *mut dw_edma_v0_ch_regs {
    if (*(*dw).chip).mf == EDMA_MF_EDMA_LEGACY {
        return &mut (*__dw_regs(dw)).type_.legacy.ch;
    }
    if dir == EDMA_DIR_WRITE {
        &mut (*__dw_regs(dw)).type_.unroll.ch[ch as usize].wr
    } else {
        &mut (*__dw_regs(dw)).type_.unroll.ch[ch as usize].rd
    }
}

unsafe fn writel_ch(dw: *mut dw_edma, dir: enum_dw_edma_dir, ch: u16, value: u32, addr: *mut u32) {
    if (*(*dw).chip).mf == EDMA_MF_EDMA_LEGACY {
        let mut viewport_sel: u32 = (((ch as u32) << 0) & EDMA_V0_VIEWPORT_MASK);
        if dir == EDMA_DIR_READ { viewport_sel |= 1 << 31; }
        let mut flags: c_ulong = 0;
        raw_spin_lock_irqsave(&mut (*dw).lock, &mut flags);
        writel(viewport_sel, &mut (*__dw_regs(dw)).type_.legacy.viewport_sel);
        writel(value, addr);
        raw_spin_unlock_irqrestore(&mut (*dw).lock, flags);
    } else { writel(value, addr); }
}

unsafe fn readl_ch(dw: *mut dw_edma, dir: enum_dw_edma_dir, ch: u16, addr: *const u32) -> u32 {
    let value;
    if (*(*dw).chip).mf == EDMA_MF_EDMA_LEGACY {
        let mut viewport_sel: u32 = (((ch as u32) << 0) & EDMA_V0_VIEWPORT_MASK);
        if dir == EDMA_DIR_READ { viewport_sel |= 1 << 31; }
        let mut flags: c_ulong = 0;
        raw_spin_lock_irqsave(&mut (*dw).lock, &mut flags);
        writel(viewport_sel, &mut (*__dw_regs(dw)).type_.legacy.viewport_sel);
        value = readl(addr);
        raw_spin_unlock_irqrestore(&mut (*dw).lock, flags);
    } else { value = readl(addr); }
    value
}

unsafe fn dw_edma_v0_func_num(chan: *mut dw_edma_chan) -> u32 {
    ((*chan).func_no as u32) << 12 & EDMA_V0_FUNC_NUM_MASK
}

unsafe fn dw_edma_v0_core_ch_power(dw: *mut dw_edma, dir: enum_dw_edma_dir, id: u16, enable: bool) {
    let value = if enable { 1 } else { 0 };
    if id >= EDMA_V0_MAX_NR_CH { return; }
    let reg = match id {
        0 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch0_pwr_en,
        1 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch1_pwr_en,
        2 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch2_pwr_en,
        3 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch3_pwr_en,
        4 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch4_pwr_en,
        5 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch5_pwr_en,
        6 => &mut (*__dw_regs(dw)).type_.unroll.wr_ch6_pwr_en,
        _ => &mut (*__dw_regs(dw)).type_.unroll.wr_ch7_pwr_en,
    };
    let _ = dir;
    writel(value, reg);
}

unsafe fn dw_edma_v0_core_engine_disable(dw: *mut dw_edma, dir: enum_dw_edma_dir) -> c_int {
    set_rw_32(dw, dir, "engine_en", 0);
    let mut value = 0;
    let ret = read_poll_timeout(get_rw_32, &mut value, (value & 1) == 0, 100, 200000, false, dw, dir, "engine_en");
    if ret != 0 { dev_warn((*(*dw).chip).dev, if dir == EDMA_DIR_WRITE { "write" } else { "read" }); }
    ret
}

unsafe fn dw_edma_v0_core_dir_off(dw: *mut dw_edma, dir: enum_dw_edma_dir) -> c_int {
    set_rw_32(dw, dir, "int_mask", EDMA_V0_DONE_INT_MASK | EDMA_V0_ABORT_INT_MASK);
    let ret = if (*(*dw).chip).mf == EDMA_MF_HDMA_COMPAT {
        let count = if dir == EDMA_DIR_WRITE { (*dw).wr_ch_cnt } else { (*dw).rd_ch_cnt };
        for id in 0..count { dw_edma_v0_core_ch_power(dw, dir, id, false); }
        0
    } else { dw_edma_v0_core_engine_disable(dw, dir) };
    set_rw_32(dw, dir, "int_clear", EDMA_V0_DONE_INT_MASK | EDMA_V0_ABORT_INT_MASK);
    ret
}

unsafe fn dw_edma_v0_core_off(dw: *mut dw_edma) {
    set_both_32(dw, "int_mask", EDMA_V0_DONE_INT_MASK | EDMA_V0_ABORT_INT_MASK);
    set_both_32(dw, "int_clear", EDMA_V0_DONE_INT_MASK | EDMA_V0_ABORT_INT_MASK);
    set_both_32(dw, "engine_en", 0);
}

unsafe fn dw_edma_v0_core_quiesce(dw: *mut dw_edma) -> c_int {
    let mut ret = 0;
    if (*dw).wr_ch_cnt != 0 { ret = dw_edma_v0_core_dir_off(dw, EDMA_DIR_WRITE); }
    if (*dw).rd_ch_cnt != 0 { let err = dw_edma_v0_core_dir_off(dw, EDMA_DIR_READ); if ret == 0 { ret = err; } }
    ret
}

unsafe fn dw_edma_v0_core_ch_quiesce(chan: *mut dw_edma_chan) -> c_int { dw_edma_v0_core_dir_off((*chan).dw, (*chan).dir) }

unsafe fn dw_edma_v0_core_ch_count(dw: *mut dw_edma, dir: enum_dw_edma_dir) -> u16 {
    let mask = if dir == EDMA_DIR_WRITE { EDMA_V0_WRITE_CH_COUNT_MASK } else { EDMA_V0_READ_CH_COUNT_MASK };
    let mut num_ch = field_get(mask, get_32(dw, &(*__dw_regs(dw)).ctrl));
    if num_ch > EDMA_V0_MAX_NR_CH { num_ch = EDMA_V0_MAX_NR_CH; }
    num_ch as u16
}

unsafe fn dw_edma_v0_core_ch_status(chan: *mut dw_edma_chan) -> dma_status {
    let tmp = field_get(EDMA_V0_CH_STATUS_MASK, get_ch_32((*chan).dw, (*chan).dir, (*chan).id, "ch_control1"));
    if tmp == 1 { DMA_IN_PROGRESS } else if tmp == 3 { DMA_COMPLETE } else { DMA_ERROR }
}

unsafe fn dw_edma_v0_core_clear_done_int(chan: *mut dw_edma_chan) { set_rw_32((*chan).dw, (*chan).dir, "int_clear", field_prep(EDMA_V0_DONE_INT_MASK, 1 << (*chan).id)); }
unsafe fn dw_edma_v0_core_clear_abort_int(chan: *mut dw_edma_chan) { set_rw_32((*chan).dw, (*chan).dir, "int_clear", field_prep(EDMA_V0_ABORT_INT_MASK, 1 << (*chan).id)); }

unsafe fn dw_edma_v0_core_handle_int(dw_irq: *mut dw_edma_irq, dir: enum_dw_edma_dir, done: dw_edma_handler_t, abort: dw_edma_handler_t) -> irqreturn_t {
    let dw = (*dw_irq).dw;
    let (total, off, mask) = if dir == EDMA_DIR_WRITE { ((*dw).wr_ch_cnt, 0, (*dw_irq).wr_mask) } else { ((*dw).rd_ch_cnt, (*dw).wr_ch_cnt, (*dw_irq).rd_mask) };
    let mut ret = IRQ_NONE;
    let sts = get_rw_32(dw, dir, "int_status");
    let mut val = field_get(EDMA_V0_DONE_INT_MASK, sts) & *mask;
    for pos in for_each_set_bit(val, total) { let chan = &mut (*dw).chan[(pos + off) as usize]; if !dw_edma_core_ch_ignore_irq(chan) { dw_edma_v0_core_clear_done_int(chan); done(chan); ret = IRQ_HANDLED; } }
    val = field_get(EDMA_V0_ABORT_INT_MASK, sts) & *mask;
    for pos in for_each_set_bit(val, total) { let chan = &mut (*dw).chan[(pos + off) as usize]; if !dw_edma_core_ch_ignore_irq(chan) { dw_edma_v0_core_clear_abort_int(chan); abort(chan); ret = IRQ_HANDLED; } }
    ret
}

unsafe fn dw_edma_v0_write_ll_data(chan: *mut dw_edma_chan, i: c_int, control: u32, size: u32, sar: u64, dar: u64) {
    let ofs = (i as usize) * core::mem::size_of::<dw_edma_v0_lli>();
    if (*(*chan).dw).chip.flags & DW_EDMA_CHIP_LOCAL != 0 { let lli = (*chan).ll_region.vaddr.mem.add(ofs) as *mut dw_edma_v0_lli; (*lli).transfer_size=size; (*lli).sar.reg=sar; (*lli).dar.reg=dar; dma_wmb(); (*lli).control=control; }
    else { let lli = (*chan).ll_region.vaddr.io.add(ofs) as *mut dw_edma_v0_lli; writel(size,&mut (*lli).transfer_size); writeq(sar,&mut (*lli).sar.reg); writeq(dar,&mut (*lli).dar.reg); writel(control,&mut (*lli).control); }
}

unsafe fn dw_edma_v0_write_ll_link(chan: *mut dw_edma_chan, i: c_int, control: u32, pointer: u64) {
    let ofs = (i as usize) * core::mem::size_of::<dw_edma_v0_lli>();
    if (*(*chan).dw).chip.flags & DW_EDMA_CHIP_LOCAL != 0 { let llp = (*chan).ll_region.vaddr.mem.add(ofs) as *mut dw_edma_v0_llp; (*llp).llp.reg=pointer; dma_wmb(); (*llp).control=control; }
    else { let llp = (*chan).ll_region.vaddr.io.add(ofs) as *mut dw_edma_v0_llp; writeq(pointer,&mut (*llp).llp.reg); writel(control,&mut (*llp).control); }
}

unsafe fn dw_edma_v0_core_ch_enable(chan: *mut dw_edma_chan) {
    let dw=(*chan).dw; set_rw_32(dw,(*chan).dir,"engine_en",1); if (*(*dw).chip).mf==EDMA_MF_HDMA_COMPAT { dw_edma_v0_core_ch_power(dw,(*chan).dir,(*chan).id,true); }
    let mut flags=0; raw_spin_lock_irqsave(&mut (*dw).lock,&mut flags); let mut tmp=get_rw_32(dw,(*chan).dir,"int_mask"); if (*chan).irq_mode==DW_EDMA_CH_IRQ_REMOTE { tmp|=field_prep(EDMA_V0_DONE_INT_MASK,1<<(*chan).id)|field_prep(EDMA_V0_ABORT_INT_MASK,1<<(*chan).id); } else { tmp&=!field_prep(EDMA_V0_DONE_INT_MASK,1<<(*chan).id); tmp&=!field_prep(EDMA_V0_ABORT_INT_MASK,1<<(*chan).id); } set_rw_32(dw,(*chan).dir,"int_mask",tmp); tmp=get_rw_32(dw,(*chan).dir,"linked_list_err_en")|field_prep(EDMA_V0_LINKED_LIST_ERR_MASK,1<<(*chan).id); set_rw_32(dw,(*chan).dir,"linked_list_err_en",tmp); raw_spin_unlock_irqrestore(&mut (*dw).lock,flags);
    set_ch_32(dw,(*chan).dir,(*chan).id,"ch_control1",DW_EDMA_V0_CCS as u32|DW_EDMA_V0_LLE as u32|dw_edma_v0_func_num(chan)); set_ch_32(dw,(*chan).dir,(*chan).id,"llp.lsb",lower_32_bits((*chan).ll_region.paddr)); set_ch_32(dw,(*chan).dir,(*chan).id,"llp.msb",upper_32_bits((*chan).ll_region.paddr));
}

unsafe fn dw_edma_v0_sync_ll_data(chan: *mut dw_edma_chan) { if (*(*chan).dw).chip.flags & DW_EDMA_CHIP_LOCAL == 0 { readl((*chan).ll_region.vaddr.io as *const u32); } }

unsafe fn dw_edma_v0_core_ch_config(chan: *mut dw_edma_chan) {
    let dw=(*chan).dw; let mut tmp=0; set_rw_32(dw,(*chan).dir,"done_imwr.lsb",(*chan).msi.address_lo); set_rw_32(dw,(*chan).dir,"done_imwr.msb",(*chan).msi.address_hi); set_rw_32(dw,(*chan).dir,"abort_imwr.lsb",(*chan).msi.address_lo); set_rw_32(dw,(*chan).dir,"abort_imwr.msb",(*chan).msi.address_hi);
    let name=match (*chan).id {0|1=>"ch01_imwr_data",2|3=>"ch23_imwr_data",4|5=>"ch45_imwr_data",_=>"ch67_imwr_data"}; tmp=get_rw_32(dw,(*chan).dir,name); if (*chan).id&1!=0 {tmp&=EDMA_V0_CH_EVEN_MSI_DATA_MASK;tmp|=field_prep(EDMA_V0_CH_ODD_MSI_DATA_MASK,(*chan).msi.data);} else {tmp&=EDMA_V0_CH_ODD_MSI_DATA_MASK;tmp|=field_prep(EDMA_V0_CH_EVEN_MSI_DATA_MASK,(*chan).msi.data);} set_rw_32(dw,(*chan).dir,name,tmp);
}

unsafe fn dw_edma_v0_core_ll_data(chan:*mut dw_edma_chan,burst:*mut dw_edma_burst,idx:u32,cb:bool,irq:bool){let mut control=0;if cb{control|=DW_EDMA_V0_CB as u32;}if irq{control|=DW_EDMA_V0_LIE as u32;if (*(*chan).dw).chip.flags&DW_EDMA_CHIP_LOCAL==0&&(*chan).irq_mode==DW_EDMA_CH_IRQ_REMOTE{control|=DW_EDMA_V0_RIE as u32;}}dw_edma_v0_write_ll_data(chan,idx as c_int,control,(*burst).sz,(*burst).sar,(*burst).dar);}
unsafe fn dw_edma_v0_core_ll_link(chan:*mut dw_edma_chan,idx:u32,cb:bool,addr:u64){let mut control=DW_EDMA_V0_LLP as u32|DW_EDMA_V0_TCB as u32;if !cb{control|=DW_EDMA_V0_CB as u32;}dw_edma_v0_write_ll_link(chan,idx as c_int,control,addr);}
unsafe fn dw_edma_v0_core_ch_doorbell(chan:*mut dw_edma_chan){dw_edma_v0_sync_ll_data(chan);set_rw_32((*chan).dw,(*chan).dir,"doorbell",field_prep(EDMA_V0_DOORBELL_CH_MASK,(*chan).id));}
unsafe fn dw_edma_v0_core_debugfs_on(dw:*mut dw_edma){dw_edma_v0_debugfs_on(dw);}
unsafe fn dw_edma_v0_core_ack_emulated_irq(dw:*mut dw_edma){set_both_32(dw,"int_clear",0);}
unsafe fn dw_edma_v0_core_db_offset(_dw:*mut dw_edma)->resource_size_t{core::mem::offset_of!(dw_edma_v0_regs,rd_int_status) as resource_size_t}

static dw_edma_v0_core: dw_edma_core_ops = dw_edma_core_ops { off:Some(dw_edma_v0_core_off),quiesce:Some(dw_edma_v0_core_quiesce),ch_quiesce:Some(dw_edma_v0_core_ch_quiesce),ch_count:Some(dw_edma_v0_core_ch_count),ch_status:Some(dw_edma_v0_core_ch_status),handle_int:Some(dw_edma_v0_core_handle_int),ll_data:Some(dw_edma_v0_core_ll_data),ll_link:Some(dw_edma_v0_core_ll_link),ch_doorbell:Some(dw_edma_v0_core_ch_doorbell),ch_enable:Some(dw_edma_v0_core_ch_enable),ch_config:Some(dw_edma_v0_core_ch_config),debugfs_on:Some(dw_edma_v0_core_debugfs_on),ack_emulated_irq:Some(dw_edma_v0_core_ack_emulated_irq),db_offset:Some(dw_edma_v0_core_db_offset) };

pub unsafe fn dw_edma_v0_core_register(dw:*mut dw_edma){(*dw).core=&dw_edma_v0_core;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
