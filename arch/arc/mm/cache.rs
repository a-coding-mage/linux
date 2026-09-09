// SPDX-License-Identifier: GPL-2.0-only
/* ARC Cache Management -- direct Rust translation of cache.c. */

// Kernel-provided types, constants, functions, macros, and register helpers
// referenced below are intentionally left as external dependencies.

#[repr(C)]
struct CpuinfoArcCache { sz_k: u32, line_len: u32, colors: u32 }

static mut L2_LINE_SZ: i32 = 0;
static mut IOC_EXISTS: i32 = 0;
pub static mut SLC_ENABLE: i32 = 1;
pub static mut IOC_ENABLE: i32 = 1;
pub static mut PERIP_BASE: usize = ARC_UNCACHED_ADDR_SPACE;
pub static mut PERIP_END: usize = 0xffff_ffff;
static mut IC_INFO: CpuinfoArcCache = CpuinfoArcCache { sz_k: 0, line_len: 0, colors: 0 };
static mut DC_INFO: CpuinfoArcCache = CpuinfoArcCache { sz_k: 0, line_len: 0, colors: 0 };
static mut SLC_INFO: CpuinfoArcCache = CpuinfoArcCache { sz_k: 0, line_len: 0, colors: 0 };

pub static mut _cache_line_loop_ic_fn: Option<unsafe extern "C" fn(usize, usize, usize, i32, i32)> = None;
pub static mut __dma_cache_wback_inv: Option<unsafe extern "C" fn(usize, usize)> = None;
pub static mut __dma_cache_inv: Option<unsafe extern "C" fn(usize, usize)> = None;
pub static mut __dma_cache_wback: Option<unsafe extern "C" fn(usize, usize)> = None;

unsafe fn read_decode_cache_bcr_arcv2(c: i32, buf: *mut u8, len: i32) -> i32 {
    let _ = (c, buf, len);
    // The body below is kept in kernel-style form; formatting and string helpers
    // are supplied by the surrounding kernel translation.
    let mut n = 0;
    let p_slc = &mut SLC_INFO;
    let mut sbcr = BcrGeneric::default();
    let mut cbcr = BcrClustCfg::default();
    let mut ident = BcrIdentity::default();
    let mut vol = BcrVolatile::default();
    READ_BCR(ARC_REG_SLC_BCR, &mut sbcr);
    if sbcr.ver != 0 {
        let mut cfg = BcrSlcCfg::default();
        READ_BCR(ARC_REG_SLC_CFG, &mut cfg);
        p_slc.sz_k = 128u32 << cfg.sz;
        p_slc.line_len = if cfg.lsz == 0 { 128 } else { 64 };
        L2_LINE_SZ = p_slc.line_len as i32;
        n += scnprintf(buf.add(n as usize), len - n, "SLC\t\t: %uK, %uB Line%s\n", p_slc.sz_k, p_slc.line_len, IS_USED_RUN(SLC_ENABLE));
    }
    READ_BCR(ARC_REG_CLUSTER_BCR, &mut cbcr);
    if cbcr.c != 0 { IOC_EXISTS = 1; if IS_ENABLED(CONFIG_HIGHMEM) || is_pae40_enabled() { IOC_ENABLE = 0; } } else { IOC_ENABLE = 0; }
    READ_BCR(AUX_IDENTITY, &mut ident);
    if ident.family > 0x51 { READ_BCR(AUX_VOL, &mut vol); PERIP_BASE = (vol.start as usize) << 28; if ident.family > 0x52 { PERIP_END = ((vol.limit as usize) << 28).wrapping_sub(1); } }
    n += scnprintf(buf.add(n as usize), len - n, "Peripherals\t: %#lx%s%s\n", PERIP_BASE, IS_AVAIL3(IOC_EXISTS, IOC_ENABLE, ", IO-Coherency (per-device) "));
    n
}

pub unsafe extern "C" fn arc_cache_mumbojumbo(c: i32, buf: *mut u8, len: i32) -> i32 {
    let _ = c; let mut n = 0; let mut vipt = 0; let mut assoc = 0; let ic = &mut IC_INFO; let dc = &mut DC_INFO;
    let mut ibcr = BcrCache::default(); let mut dbcr = BcrCache::default();
    READ_BCR(ARC_REG_IC_BCR, &mut ibcr);
    if ibcr.ver != 0 { if is_isa_arcompact() && ibcr.ver <= 3 { BUG_ON(ibcr.config != 3); assoc = 2; } else if is_isa_arcv2() && ibcr.ver >= 4 { assoc = 1 << ibcr.config; } ic.line_len = 8 << ibcr.line_len; ic.sz_k = 1 << (ibcr.sz - 1); ic.colors = ic.sz_k / assoc / TO_KB(PAGE_SIZE); n += scnprintf(buf.add(n as usize), len-n, "I-Cache\t\t: %uK, %dway/set, %uB Line, VIPT%s%s\n", ic.sz_k, assoc, ic.line_len, if ic.colors > 1 { " aliasing" } else { "" }, IS_USED_CFG(CONFIG_ARC_HAS_ICACHE)); }
    READ_BCR(ARC_REG_DC_BCR, &mut dbcr);
    if dbcr.ver != 0 { if is_isa_arcompact() && dbcr.ver <= 3 { BUG_ON(dbcr.config != 2); vipt = 1; assoc = 4; dc.colors = dc.sz_k / assoc / TO_KB(PAGE_SIZE); } else if is_isa_arcv2() && dbcr.ver >= 4 { vipt = 0; assoc = 1 << dbcr.config; dc.colors = 1; } dc.line_len = 16 << dbcr.line_len; dc.sz_k = 1 << (dbcr.sz - 1); n += scnprintf(buf.add(n as usize), len-n, "D-Cache\t\t: %uK, %dway/set, %uB Line, %s%s\n", dc.sz_k, assoc, dc.line_len, if vipt != 0 { "VIPT" } else { "PIPT" }, IS_USED_CFG(CONFIG_ARC_HAS_DCACHE)); }
    if is_isa_arcv2() { n += read_decode_cache_bcr_arcv2(c, buf.add(n as usize), len-n); } n
}

const OP_INV: i32 = 1; const OP_FLUSH: i32 = 2; const OP_FLUSH_N_INV: i32 = 3; const OP_INV_IC: i32 = 4;

unsafe fn cache_line_loop_v3(mut paddr: usize, mut vaddr: usize, mut sz: usize, op: i32, full_page: i32) {
    let (aux_cmd, aux_tag) = if op == OP_INV_IC { (ARC_REG_IC_IVIL, ARC_REG_IC_PTAG) } else { (if op & OP_INV != 0 { ARC_REG_DC_IVDL } else { ARC_REG_DC_FLDL }, ARC_REG_DC_PTAG) };
    if full_page == 0 { sz += paddr & !CACHE_LINE_MASK; paddr &= CACHE_LINE_MASK; vaddr &= CACHE_LINE_MASK; }
    let mut num_lines = DIV_ROUND_UP(sz, L1_CACHE_BYTES);
    if full_page != 0 { write_aux_reg(aux_tag, paddr); }
    if is_pae40_enabled() && op == OP_INV_IC { write_aux_reg(ARC_REG_IC_PTAG_HI, (paddr as u64 >> 32) as usize); }
    while num_lines > 0 { if full_page == 0 { write_aux_reg(aux_tag, paddr); paddr += L1_CACHE_BYTES; } write_aux_reg(aux_cmd, vaddr); vaddr += L1_CACHE_BYTES; num_lines -= 1; }
}

unsafe fn cache_line_loop_v4(mut paddr: usize, _vaddr: usize, mut sz: usize, op: i32, full_page: i32) {
    let aux_cmd = if op == OP_INV_IC { ARC_REG_IC_IVIL } else if op & OP_INV != 0 { ARC_REG_DC_IVDL } else { ARC_REG_DC_FLDL };
    if full_page == 0 { sz += paddr & !CACHE_LINE_MASK; paddr &= CACHE_LINE_MASK; }
    let mut num_lines = DIV_ROUND_UP(sz, L1_CACHE_BYTES);
    if is_pae40_enabled() { write_aux_reg(if op == OP_INV_IC { ARC_REG_IC_PTAG_HI } else { ARC_REG_DC_PTAG_HI }, (paddr as u64 >> 32) as usize); }
    while num_lines > 0 { write_aux_reg(aux_cmd, paddr); paddr += L1_CACHE_BYTES; num_lines -= 1; }
}

unsafe fn before_dc_op(op: i32) { let ctl = ARC_REG_DC_CTRL; let mut val = read_aux_reg(ctl); if op == OP_FLUSH_N_INV { val |= DC_CTRL_INV_MODE_FLUSH; } if op != OP_INV_IC { val &= !DC_CTRL_RGN_OP_MSK; if op & OP_INV != 0 { val |= DC_CTRL_RGN_OP_INV; } } write_aux_reg(ctl, val); }
unsafe fn after_dc_op(op: i32) { if op & OP_FLUSH != 0 { let ctl = ARC_REG_DC_CTRL; let mut reg; loop { reg = read_aux_reg(ctl); if reg & DC_CTRL_FLUSH_STATUS == 0 { break; } } if op == OP_FLUSH_N_INV { write_aux_reg(ctl, reg & !DC_CTRL_INV_MODE_FLUSH); } } }
unsafe fn dc_entire_op(op: i32) { before_dc_op(op); write_aux_reg(if op & OP_INV != 0 { ARC_REG_DC_IVDC } else { ARC_REG_DC_FLSH }, 1); after_dc_op(op); }
unsafe fn dc_disable() { let r = ARC_REG_DC_CTRL; dc_entire_op(OP_FLUSH_N_INV); write_aux_reg(r, read_aux_reg(r) | DC_CTRL_DIS); }
unsafe fn dc_enable() { let r = ARC_REG_DC_CTRL; write_aux_reg(r, read_aux_reg(r) & !DC_CTRL_DIS); }
unsafe fn dc_line_op(paddr: usize, vaddr: usize, sz: usize, op: i32) { let flags = 0usize; local_irq_save(flags); before_dc_op(op); cache_line_loop_v4(paddr, vaddr, sz, op, (sz == PAGE_SIZE) as i32); after_dc_op(op); local_irq_restore(flags); }
unsafe fn ic_entire_inv() { write_aux_reg(ARC_REG_IC_IVIC, 1); let _ = read_aux_reg(ARC_REG_IC_CTRL); }
unsafe fn ic_line_inv_vaddr_local(paddr: usize, vaddr: usize, sz: usize) { let flags = 0usize; local_irq_save(flags); if let Some(f) = _cache_line_loop_ic_fn { f(paddr, vaddr, sz, OP_INV_IC, (sz == PAGE_SIZE) as i32); } local_irq_restore(flags); }

unsafe fn slc_op_rgn(paddr: usize, sz: usize, op: i32) { let mut ctrl = read_aux_reg(ARC_REG_SLC_CTRL); if op & OP_FLUSH == 0 { ctrl &= !SLC_CTRL_IM; } else { ctrl |= SLC_CTRL_IM; } if op & OP_INV != 0 { ctrl |= SLC_CTRL_RGN_OP_INV; } else { ctrl &= !SLC_CTRL_RGN_OP_INV; } write_aux_reg(ARC_REG_SLC_CTRL, ctrl); let end = paddr + sz + L2_LINE_SZ as usize - 1; write_aux_reg(ARC_REG_SLC_RGN_END, lower_32_bits(end)); write_aux_reg(ARC_REG_SLC_RGN_START, lower_32_bits(paddr)); let _ = read_aux_reg(ARC_REG_SLC_CTRL); while read_aux_reg(ARC_REG_SLC_CTRL) & SLC_CTRL_BUSY != 0 {} }
unsafe fn slc_entire_op(op: i32) { let r = ARC_REG_SLC_CTRL; let mut ctrl = read_aux_reg(r); if op & OP_FLUSH == 0 { ctrl &= !SLC_CTRL_IM; } else { ctrl |= SLC_CTRL_IM; } write_aux_reg(r, ctrl); write_aux_reg(if op & OP_INV != 0 { ARC_REG_SLC_INVALIDATE } else { ARC_REG_SLC_FLUSH }, 1); let _ = read_aux_reg(r); while read_aux_reg(r) & SLC_CTRL_BUSY != 0 {} }
unsafe fn arc_slc_disable() { let r=ARC_REG_SLC_CTRL; slc_entire_op(OP_FLUSH_N_INV); write_aux_reg(r, read_aux_reg(r)|SLC_CTRL_DIS); }
unsafe fn arc_slc_enable() { let r=ARC_REG_SLC_CTRL; write_aux_reg(r, read_aux_reg(r)&!SLC_CTRL_DIS); }

pub unsafe extern "C" fn flush_dcache_folio(folio: *mut Folio) { clear_bit(PG_dc_clean, &mut (*folio).flags.f); }
pub unsafe extern "C" fn flush_dcache_page(page: *mut Page) { flush_dcache_folio(page_folio(page)); }
unsafe fn dma_cache_wback_inv_l1(s: usize, z: usize) { dc_line_op(s,s,z,OP_FLUSH_N_INV); }
unsafe fn dma_cache_inv_l1(s: usize,z:usize){dc_line_op(s,s,z,OP_INV)} unsafe fn dma_cache_wback_l1(s:usize,z:usize){dc_line_op(s,s,z,OP_FLUSH)}
unsafe fn dma_cache_wback_inv_slc(s:usize,z:usize){dc_line_op(s,s,z,OP_FLUSH_N_INV);slc_op_rgn(s,z,OP_FLUSH_N_INV)} unsafe fn dma_cache_inv_slc(s:usize,z:usize){dc_line_op(s,s,z,OP_INV);slc_op_rgn(s,z,OP_INV)} unsafe fn dma_cache_wback_slc(s:usize,z:usize){dc_line_op(s,s,z,OP_FLUSH);slc_op_rgn(s,z,OP_FLUSH)}
pub unsafe extern "C" fn dma_cache_wback_inv(s:usize,z:usize){if let Some(f)=__dma_cache_wback_inv{f(s,z)}} pub unsafe extern "C" fn dma_cache_inv(s:usize,z:usize){if let Some(f)=__dma_cache_inv{f(s,z)}} pub unsafe extern "C" fn dma_cache_wback(s:usize,z:usize){if let Some(f)=__dma_cache_wback{f(s,z)}}
pub unsafe extern "C" fn __sync_icache_dcache(p:usize,v:usize,len:i32){dc_line_op(p,v,len as usize,OP_FLUSH_N_INV);ic_line_inv_vaddr_local(p,v,len as usize)}
pub unsafe extern "C" fn __inv_icache_pages(p:usize,v:usize,n:u32){ic_line_inv_vaddr_local(p,v,n as usize*PAGE_SIZE)} pub unsafe extern "C" fn __flush_dcache_pages(p:usize,v:usize,n:u32){dc_line_op(p,v&PAGE_MASK,n as usize*PAGE_SIZE,OP_FLUSH_N_INV)}
pub unsafe extern "C" fn flush_cache_all(){let flags=0usize;local_irq_save(flags);ic_entire_inv();dc_entire_op(OP_FLUSH_N_INV);local_irq_restore(flags)}
pub unsafe extern "C" fn clear_user_page(to:*mut u8,_u:usize,page:*mut Page){let f=page_folio(page);clear_page(to);clear_bit(PG_dc_clean,&mut (*f).flags.f)}
pub unsafe extern "C" fn copy_user_highpage(to:*mut Page,from:*mut Page,_u:usize,_vma:*mut VmAreaStruct){let src=page_folio(from);let dst=page_folio(to);let kfrom=kmap_atomic(from);let kto=kmap_atomic(to);copy_page(kto,kfrom);clear_bit(PG_dc_clean,&mut (*dst).flags.f);clear_bit(PG_dc_clean,&mut (*src).flags.f);kunmap_atomic(kto);kunmap_atomic(kfrom)}
pub unsafe extern "C" fn flush_icache_range(mut kstart:usize,kend:usize){let mut tot_sz=kend.wrapping_sub(kstart);WARN(kstart<TASK_SIZE,"flush_icache_range() can't handle user vaddr");if tot_sz>PAGE_SIZE{flush_cache_all();return}if kstart>PAGE_OFFSET{__sync_icache_dcache(kstart,kstart,tot_sz);return}while tot_sz>0{let off=kstart%PAGE_SIZE;let pfn=vmalloc_to_pfn(kstart as *mut core::ffi::c_void);let phy=(pfn<<PAGE_SHIFT)+off;let sz=core::cmp::min(tot_sz,PAGE_SIZE-off);__sync_icache_dcache(phy,kstart,sz);kstart+=sz;tot_sz-=sz}}

pub unsafe extern "C" fn arc_cache_init(){if smp_processor_id()==0{arc_cache_init_master()}if is_isa_arcv2()&&pae40_exist_but_not_enab(){write_aux_reg(ARC_REG_IC_PTAG_HI,0);write_aux_reg(ARC_REG_DC_PTAG_HI,0);if L2_LINE_SZ!=0{write_aux_reg(ARC_REG_SLC_RGN_END1,0);write_aux_reg(ARC_REG_SLC_RGN_START1,0)}}}
unsafe fn arc_cache_init_master(){if IS_ENABLED(CONFIG_ARC_HAS_ICACHE){if IC_INFO.line_len==0{panic!("cache support enabled but non-existent cache")}if IC_INFO.line_len as usize!=L1_CACHE_BYTES{panic!("ICache line != kernel Config")}_cache_line_loop_ic_fn=Some(if is_isa_arcv2()&&IC_INFO.colors>1{cache_line_loop_v3}else{cache_line_loop_v4})}if IS_ENABLED(CONFIG_ARC_HAS_DCACHE){if DC_INFO.line_len==0{panic!("cache support enabled but non-existent cache")}if DC_INFO.line_len as usize!=L1_CACHE_BYTES{panic!("DCache line != kernel Config")}if is_isa_arcompact()&&DC_INFO.colors>1{panic!("Aliasing VIPT cache not supported")}}if is_isa_arcv2()&&L2_LINE_SZ as usize>SMP_CACHE_BYTES{panic!("L2 Cache line > kernel Config")}if is_isa_arcv2()&&L2_LINE_SZ!=0&&!SLC_ENABLE{arc_slc_disable()}if is_isa_arcv2()&&IOC_EXISTS!=0{arc_ioc_setup()}if is_isa_arcv2()&&L2_LINE_SZ!=0&&SLC_ENABLE{__dma_cache_wback_inv=Some(dma_cache_wback_inv_slc);__dma_cache_inv=Some(dma_cache_inv_slc);__dma_cache_wback=Some(dma_cache_wback_slc)}else{__dma_cache_wback_inv=Some(dma_cache_wback_inv_l1);__dma_cache_inv=Some(dma_cache_inv_l1);__dma_cache_wback=Some(dma_cache_wback_l1)}}
unsafe fn arc_ioc_setup(){if read_aux_reg(ARC_REG_IO_COH_ENABLE)&ARC_IO_COH_ENABLE_BIT!=0{panic!("IOC already enabled, please upgrade bootloader!")}if IOC_ENABLE==0{return}dc_disable();if read_aux_reg(ARC_REG_SLC_BCR)!=0{slc_entire_op(OP_FLUSH_N_INV)}let mem_sz=arc_get_mem_sz();if !is_power_of_2(mem_sz)||mem_sz<4096{panic!("IOC Aperture size must be power of 2 larger than 4KB")}write_aux_reg(ARC_REG_IO_COH_AP0_SIZE,order_base_2(mem_sz>>10)-2);let ioc_base=CONFIG_LINUX_RAM_BASE;if ioc_base%mem_sz!=0{panic!("IOC Aperture start must be aligned to the size of the aperture")}write_aux_reg(ARC_REG_IO_COH_AP0_BASE,ioc_base>>12);write_aux_reg(ARC_REG_IO_COH_PARTIAL,ARC_IO_COH_PARTIAL_BIT);write_aux_reg(ARC_REG_IO_COH_ENABLE,ARC_IO_COH_ENABLE_BIT);dc_enable()}
pub unsafe extern "C" fn cacheflush(_start:u32,_sz:u32,_flags:u32)->i32{flush_cache_all();0}

// External kernel declarations intentionally remain unresolved here.
extern "Rust" {
    fn READ_BCR(reg: usize, out: *mut core::ffi::c_void); fn scnprintf(buf:*mut u8,len:i32,fmt:&str,...)->i32;
    fn write_aux_reg(reg:usize,val:usize); fn read_aux_reg(reg:usize)->usize; fn is_isa_arcompact()->bool; fn is_isa_arcv2()->bool; fn is_pae40_enabled()->bool;
    fn arc_cache_init_master(); fn arc_ioc_setup(); fn pae40_exist_but_not_enab()->bool; fn smp_processor_id()->usize; fn arc_get_mem_sz()->usize; fn is_power_of_2(v:usize)->bool; fn order_base_2(v:usize)->usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
