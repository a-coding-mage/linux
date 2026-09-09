// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2007-2010
 * Author: Per Forlin <per.forlin@stericsson.com> for ST-Ericsson
 * Author: Jonas Aaberg <jonas.aberg@stericsson.com> for ST-Ericsson
 */

// Linux DMA and STE DMA declarations are supplied by the surrounding crate.

unsafe fn d40_width_to_bits(width: dma_slave_buswidth) -> u8 {
    if width == DMA_SLAVE_BUSWIDTH_1_BYTE { STEDMA40_ESIZE_8_BIT }
    else if width == DMA_SLAVE_BUSWIDTH_2_BYTES { STEDMA40_ESIZE_16_BIT }
    else if width == DMA_SLAVE_BUSWIDTH_8_BYTES { STEDMA40_ESIZE_64_BIT }
    else { STEDMA40_ESIZE_32_BIT }
}

pub unsafe fn d40_log_cfg(cfg: *mut stedma40_chan_cfg, lcsp1: *mut u32, lcsp3: *mut u32) {
    let mut l3: u32 = 0;
    let mut l1: u32 = 0;
    if (*cfg).dir == DMA_MEM_TO_DEV || (*cfg).dir == DMA_MEM_TO_MEM { l1 |= BIT(D40_MEM_LCSP1_SCFG_INCR_POS); }
    if (*cfg).dir == DMA_DEV_TO_MEM || (*cfg).dir == DMA_MEM_TO_MEM { l3 |= BIT(D40_MEM_LCSP3_DCFG_INCR_POS); }
    if (*cfg).dir == DMA_DEV_TO_MEM || (*cfg).dir == DMA_DEV_TO_DEV { l1 |= BIT(D40_MEM_LCSP1_SCFG_MST_POS); }
    if (*cfg).dir == DMA_MEM_TO_DEV || (*cfg).dir == DMA_DEV_TO_DEV { l3 |= BIT(D40_MEM_LCSP3_DCFG_MST_POS); }
    l3 |= BIT(D40_MEM_LCSP3_DCFG_EIM_POS);
    l3 |= (*cfg).dst_info.psize << D40_MEM_LCSP3_DCFG_PSIZE_POS;
    l3 |= (d40_width_to_bits((*cfg).dst_info.data_width) as u32) << D40_MEM_LCSP3_DCFG_ESIZE_POS;
    l1 |= BIT(D40_MEM_LCSP1_SCFG_EIM_POS);
    l1 |= (*cfg).src_info.psize << D40_MEM_LCSP1_SCFG_PSIZE_POS;
    l1 |= (d40_width_to_bits((*cfg).src_info.data_width) as u32) << D40_MEM_LCSP1_SCFG_ESIZE_POS;
    *lcsp1 = l1; *lcsp3 = l3;
}

pub unsafe fn d40_phy_cfg(cfg: *mut stedma40_chan_cfg, src_cfg: *mut u32, dst_cfg: *mut u32) {
    let (mut src, mut dst) = (0u32, 0u32);
    if (*cfg).dir == DMA_DEV_TO_MEM || (*cfg).dir == DMA_DEV_TO_DEV {
        src |= BIT(D40_SREG_CFG_MST_POS); src |= D40_TYPE_TO_EVENT((*cfg).dev_type);
        src |= if (*cfg).src_info.flow_ctrl == STEDMA40_NO_FLOW_CTRL { BIT(D40_SREG_CFG_PHY_TM_POS) } else { 3 << D40_SREG_CFG_PHY_TM_POS };
    }
    if (*cfg).dir == DMA_MEM_TO_DEV || (*cfg).dir == DMA_DEV_TO_DEV {
        dst |= BIT(D40_SREG_CFG_MST_POS); dst |= D40_TYPE_TO_EVENT((*cfg).dev_type);
        dst |= if (*cfg).dst_info.flow_ctrl == STEDMA40_NO_FLOW_CTRL { BIT(D40_SREG_CFG_PHY_TM_POS) } else { 3 << D40_SREG_CFG_PHY_TM_POS };
    }
    dst |= BIT(D40_SREG_CFG_TIM_POS); src |= BIT(D40_SREG_CFG_EIM_POS); dst |= BIT(D40_SREG_CFG_EIM_POS);
    if (*cfg).src_info.psize != STEDMA40_PSIZE_PHY_1 { src |= BIT(D40_SREG_CFG_PHY_PEN_POS); src |= (*cfg).src_info.psize << D40_SREG_CFG_PSIZE_POS; }
    if (*cfg).dst_info.psize != STEDMA40_PSIZE_PHY_1 { dst |= BIT(D40_SREG_CFG_PHY_PEN_POS); dst |= (*cfg).dst_info.psize << D40_SREG_CFG_PSIZE_POS; }
    src |= (d40_width_to_bits((*cfg).src_info.data_width) as u32) << D40_SREG_CFG_ESIZE_POS;
    dst |= (d40_width_to_bits((*cfg).dst_info.data_width) as u32) << D40_SREG_CFG_ESIZE_POS;
    if (*cfg).high_priority { src |= BIT(D40_SREG_CFG_PRI_POS); dst |= BIT(D40_SREG_CFG_PRI_POS); }
    if (*cfg).src_info.big_endian { src |= BIT(D40_SREG_CFG_LBE_POS); }
    if (*cfg).dst_info.big_endian { dst |= BIT(D40_SREG_CFG_LBE_POS); }
    *src_cfg = src; *dst_cfg = dst;
}

unsafe fn d40_phy_fill_lli(lli: *mut d40_phy_lli, data: dma_addr_t, data_size: u32, next_lli: dma_addr_t, reg_cfg: u32, info: *mut stedma40_half_channel_info, flags: u32) -> i32 {
    let addr_inc = flags & LLI_ADDR_INC != 0; let term_int = flags & LLI_TERM_INT != 0;
    let data_width = (*info).data_width; let psize = (*info).psize;
    let num_elems = if psize == STEDMA40_PSIZE_PHY_1 { 1 } else { 2 << psize };
    if !IS_ALIGNED(data, data_width) || data_size < num_elems * data_width { return -EINVAL; }
    (*lli).reg_elt = (data_size / data_width) << D40_SREG_ELEM_PHY_ECNT_POS;
    if addr_inc { (*lli).reg_elt |= data_width << D40_SREG_ELEM_PHY_EIDX_POS; }
    (*lli).reg_ptr = data; (*lli).reg_cfg = reg_cfg;
    (*lli).reg_lnk = if next_lli == 0 { BIT(D40_SREG_LNK_PHY_TCP_POS) } else { next_lli };
    if term_int { (*lli).reg_cfg |= BIT(D40_SREG_CFG_TIM_POS); } else { (*lli).reg_cfg &= !BIT(D40_SREG_CFG_TIM_POS); }
    0
}

unsafe fn d40_seg_size(size: i32, data_width1: i32, data_width2: i32) -> i32 {
    let max_w = max(data_width1, data_width2) as u32; let min_w = min(data_width1, data_width2) as u32;
    let mut seg_max = ALIGN(STEDMA40_MAX_SEG_SIZE * min_w, max_w);
    if seg_max > STEDMA40_MAX_SEG_SIZE { seg_max -= max_w; }
    if size as u32 <= seg_max { size } else if size as u32 <= 2 * seg_max { ALIGN((size / 2) as u32, max_w) as i32 } else { seg_max as i32 }
}

// The remaining helpers retain the original pointer-based LLI construction.
pub unsafe fn d40_phy_sg_to_lli(sg: *mut scatterlist, sg_len: i32, target: dma_addr_t, lli_sg: *mut d40_phy_lli, lli_phys: dma_addr_t, reg_cfg: u32, info: *mut stedma40_half_channel_info, otherinfo: *mut stedma40_half_channel_info, flags: u32) -> i32 {
    let mut total_size = 0; let mut current_sg = sg; let mut lli = lli_sg; let mut l_phys;
    let mut flags = flags | if target == 0 { LLI_ADDR_INC } else { 0 };
    for_each_sg!(sg, current_sg, sg_len, i, {
        let sg_addr = sg_dma_address!(current_sg); let len = sg_dma_len!(current_sg); let dst = if target != 0 { target } else { sg_addr }; total_size += len;
        if i == sg_len - 1 { flags |= LLI_TERM_INT | LLI_LAST_LINK; }
        l_phys = ALIGN(lli_phys + (lli.offset_from(lli_sg) as u64) * core::mem::size_of::<d40_phy_lli>() as u64, D40_LLI_ALIGN);
        lli = d40_phy_buf_to_lli(lli, dst, len, l_phys, lli_phys, reg_cfg, info, otherinfo, flags); if lli.is_null() { return -EINVAL; }
    });
    total_size
}

unsafe fn d40_phy_buf_to_lli(mut lli: *mut d40_phy_lli, mut addr: dma_addr_t, size: u32, lli_phys: dma_addr_t, first_phys: dma_addr_t, reg_cfg: u32, info: *mut stedma40_half_channel_info, otherinfo: *mut stedma40_half_channel_info, mut flags: u32) -> *mut d40_phy_lli {
    let addr_inc = flags & LLI_ADDR_INC != 0; let term_int = flags & LLI_TERM_INT != 0; let lastlink = flags & LLI_LAST_LINK != 0; let cyclic = flags & LLI_CYCLIC != 0;
    let mut next = lli_phys; let mut size_rest = size;
    if term_int { flags &= !LLI_TERM_INT; }
    while size_rest != 0 {
        let size_seg = d40_seg_size(size_rest as i32, (*info).data_width as i32, (*otherinfo).data_width as i32) as u32; size_rest -= size_seg;
        if size_rest == 0 && term_int { flags |= LLI_TERM_INT; }
        next = if size_rest == 0 && lastlink { if cyclic { first_phys } else { 0 } } else { ALIGN(next + core::mem::size_of::<d40_phy_lli>() as u64, D40_LLI_ALIGN) };
        if d40_phy_fill_lli(lli, addr, size_seg, next, reg_cfg, info, flags) != 0 { return core::ptr::null_mut(); }
        lli = lli.add(1); if addr_inc { addr += size_seg as u64; }
    }
    lli
}

unsafe fn d40_log_lli_link(dst: *mut d40_log_lli, src: *mut d40_log_lli, next: i32, flags: u32) {
    let (slos, dlos) = if next != -EINVAL { ((next * 2) as u32, (next * 2 + 1) as u32) } else { (0, 0) };
    if flags & LLI_TERM_INT != 0 { (*dst).lcsp13 |= D40_MEM_LCSP1_SCFG_TIM_MASK | D40_MEM_LCSP3_DTCP_MASK; }
    (*src).lcsp13 = ((*src).lcsp13 & !D40_MEM_LCSP1_SLOS_MASK) | (slos << D40_MEM_LCSP1_SLOS_POS);
    (*dst).lcsp13 = ((*dst).lcsp13 & !D40_MEM_LCSP1_SLOS_MASK) | (dlos << D40_MEM_LCSP1_SLOS_POS);
}

pub unsafe fn d40_log_lli_lcpa_write(lcpa: *mut d40_log_lli_full, dst: *mut d40_log_lli, src: *mut d40_log_lli, next: i32, flags: u32) { d40_log_lli_link(dst, src, next, flags); writel_relaxed!((*src).lcsp02, &mut (*lcpa).lcsp0); writel_relaxed!((*src).lcsp13, &mut (*lcpa).lcsp1); writel_relaxed!((*dst).lcsp02, &mut (*lcpa).lcsp2); writel_relaxed!((*dst).lcsp13, &mut (*lcpa).lcsp3); }
pub unsafe fn d40_log_lli_lcla_write(lcla: *mut d40_log_lli, dst: *mut d40_log_lli, src: *mut d40_log_lli, next: i32, flags: u32) { d40_log_lli_link(dst, src, next, flags); writel_relaxed!((*src).lcsp02, &mut (*lcla).lcsp02); writel_relaxed!((*src).lcsp13, &mut (*lcla).lcsp13); writel_relaxed!((*dst).lcsp02, &mut (*lcla.add(1)).lcsp02); writel_relaxed!((*dst).lcsp13, &mut (*lcla.add(1)).lcsp13); }

unsafe fn d40_log_fill_lli(lli: *mut d40_log_lli, data: dma_addr_t, data_size: u32, reg_cfg: u32, data_width: u32, flags: u32) { (*lli).lcsp13 = reg_cfg; (*lli).lcsp02 = ((data_size / data_width) << D40_MEM_LCSP0_ECNT_POS) & D40_MEM_LCSP0_ECNT_MASK; BUG_ON!((data_size / data_width) > STEDMA40_MAX_SEG_SIZE); (*lli).lcsp02 |= data as u32 & D40_MEM_LCSP0_SPTR_MASK; (*lli).lcsp13 |= data as u32 & D40_MEM_LCSP1_SPTR_MASK; if flags & LLI_ADDR_INC != 0 { (*lli).lcsp13 |= D40_MEM_LCSP1_SCFG_INCR_MASK; } }

pub unsafe fn d40_log_sg_to_lli(sg: *mut scatterlist, sg_len: i32, dev_addr: dma_addr_t, lli_sg: *mut d40_log_lli, lcsp13: u32, data_width1: u32, data_width2: u32) -> i32 { let mut total = 0; let mut lli = lli_sg; let flags = if dev_addr == 0 { LLI_ADDR_INC } else { 0 }; for_each_sg!(sg, current_sg, sg_len, _i, { let a = if dev_addr != 0 { dev_addr } else { sg_dma_address!(current_sg) }; let mut rest = sg_dma_len!(current_sg); total += rest; while rest != 0 { let n = d40_seg_size(rest as i32, data_width1 as i32, data_width2 as i32) as u32; d40_log_fill_lli(lli, a, n, lcsp13, data_width1, flags); lli = lli.add(1); rest -= n; } }); total }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
