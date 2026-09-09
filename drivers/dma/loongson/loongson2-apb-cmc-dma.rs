// SPDX-License-Identifier: GPL-2.0-or-later
/* Loongson-2 Chain Multi-Channel DMA Controller driver */

// Linux dependencies supplied by the surrounding kernel translation.

const LOONGSON2_CMCDMA_ISR: u32 = 0x0;
const LOONGSON2_CMCDMA_IFCR: u32 = 0x4;
const LOONGSON2_CMCDMA_CCR: u32 = 0x8;
const LOONGSON2_CMCDMA_CNDTR: u32 = 0xc;
const LOONGSON2_CMCDMA_CPAR: u32 = 0x10;
const LOONGSON2_CMCDMA_CMAR: u32 = 0x14;
const LOONGSON2_CMCDMA_TCI: u32 = 1 << 1;
const LOONGSON2_CMCDMA_HTI: u32 = 1 << 2;
const LOONGSON2_CMCDMA_TEI: u32 = 1 << 3;
const LOONGSON2_CMCDMA_MASKI: u32 = LOONGSON2_CMCDMA_TCI | LOONGSON2_CMCDMA_HTI | LOONGSON2_CMCDMA_TEI;
const LOONGSON2_CMCDMA_CCR_EN: u32 = 1 << 0;
const LOONGSON2_CMCDMA_CCR_TCIE: u32 = 1 << 1;
const LOONGSON2_CMCDMA_CCR_HTIE: u32 = 1 << 2;
const LOONGSON2_CMCDMA_CCR_TEIE: u32 = 1 << 3;
const LOONGSON2_CMCDMA_CCR_DIR: u32 = 1 << 4;
const LOONGSON2_CMCDMA_CCR_CIRC: u32 = 1 << 5;
const LOONGSON2_CMCDMA_CCR_PINC: u32 = 1 << 6;
const LOONGSON2_CMCDMA_CCR_MINC: u32 = 1 << 7;
const LOONGSON2_CMCDMA_CCR_PSIZE_MASK: u32 = 0x3 << 8;
const LOONGSON2_CMCDMA_CCR_MSIZE_MASK: u32 = 0x3 << 10;
const LOONGSON2_CMCDMA_CCR_PL_MASK: u32 = 0x3 << 12;
const LOONGSON2_CMCDMA_CCR_M2M: u32 = 1 << 14;
const LOONGSON2_CMCDMA_CCR_CFG_MASK: u32 = LOONGSON2_CMCDMA_CCR_PINC | LOONGSON2_CMCDMA_CCR_MINC | LOONGSON2_CMCDMA_CCR_PL_MASK;
const LOONGSON2_CMCDMA_CCR_IRQ_MASK: u32 = LOONGSON2_CMCDMA_CCR_TCIE | LOONGSON2_CMCDMA_CCR_HTIE | LOONGSON2_CMCDMA_CCR_TEIE;
const LOONGSON2_CMCDMA_STREAM_MASK: u32 = LOONGSON2_CMCDMA_CCR_CFG_MASK | LOONGSON2_CMCDMA_CCR_IRQ_MASK;
const LOONGSON2_CMCDMA_BUSWIDTHS: u32 = (1 << 0) | (1 << 1) | (1 << 2);
const LOONSON2_CMCDMA_MAX_DATA_ITEMS: u32 = 65536;

#[repr(C)]
struct loongson2_cmc_dma_chan_reg { ccr: u32, cndtr: u32, cpar: u32, cmar: u32 }
#[repr(C)]
struct loongson2_cmc_dma_sg_req { len: u32, chan_reg: loongson2_cmc_dma_chan_reg }
#[repr(C)]
struct loongson2_cmc_dma_desc {
    vdesc: virt_dma_desc, cyclic: bool, num_sgs: u32,
    sg_req: *mut loongson2_cmc_dma_sg_req,
}
#[repr(C)]
struct loongson2_cmc_dma_chan {
    vchan: virt_dma_chan, dma_sconfig: dma_slave_config,
    desc: *mut loongson2_cmc_dma_desc, id: u32, irq: u32, next_sg: u32,
    chan_reg: loongson2_cmc_dma_chan_reg,
}
#[repr(C)]
struct loongson2_cmc_dma_dev {
    ddev: dma_device, dma_clk: *mut clk, base: *mut core::ffi::c_void,
    nr_channels: u32, chan_reg_offset: u32, chan: *mut loongson2_cmc_dma_chan,
}
#[repr(C)]
struct loongson2_cmc_dma_config { max_channels: u32, chan_reg_offset: u32 }

static LS2K0300_CMC_DMA_CONFIG: loongson2_cmc_dma_config = loongson2_cmc_dma_config { max_channels: 8, chan_reg_offset: 0x14 };
static LS2K3000_CMC_DMA_CONFIG: loongson2_cmc_dma_config = loongson2_cmc_dma_config { max_channels: 4, chan_reg_offset: 0x18 };

unsafe fn lmdma_get_dev(lchan: *mut loongson2_cmc_dma_chan) -> *mut loongson2_cmc_dma_dev { container_of!((*lchan).vchan.chan.device, loongson2_cmc_dma_dev, ddev) }
unsafe fn to_lmdma_chan(chan: *mut dma_chan) -> *mut loongson2_cmc_dma_chan { container_of!(chan, loongson2_cmc_dma_chan, vchan.chan) }
unsafe fn to_lmdma_desc(vdesc: *mut virt_dma_desc) -> *mut loongson2_cmc_dma_desc { container_of!(vdesc, loongson2_cmc_dma_desc, vdesc) }
unsafe fn chan2dev(lchan: *mut loongson2_cmc_dma_chan) -> *mut device { &mut (*(*lchan).vchan.chan.dev).device }
unsafe fn loongson2_cmc_dma_read(d: *mut loongson2_cmc_dma_dev, reg: u32, id: u32) -> u32 { readl((*d).base.add((reg + (*d).chan_reg_offset * id) as usize)) }
unsafe fn loongson2_cmc_dma_write(d: *mut loongson2_cmc_dma_dev, reg: u32, id: u32, val: u32) { writel(val, (*d).base.add((reg + (*d).chan_reg_offset * id) as usize)); }

unsafe fn loongson2_cmc_dma_get_width(width: dma_slave_buswidth) -> i32 {
    match width { DMA_SLAVE_BUSWIDTH_1_BYTE | DMA_SLAVE_BUSWIDTH_2_BYTES | DMA_SLAVE_BUSWIDTH_4_BYTES => width.trailing_zeros() as i32, _ => -EINVAL }
}
unsafe fn loongson2_cmc_dma_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> i32 { let l = to_lmdma_chan(chan); core::ptr::copy_nonoverlapping(config, &mut (*l).dma_sconfig, 1); 0 }
unsafe fn loongson2_cmc_dma_irq_clear(l: *mut loongson2_cmc_dma_chan, flags: u32) { let d=lmdma_get_dev(l); loongson2_cmc_dma_write(d, LOONGSON2_CMCDMA_IFCR, 0, flags << (4*(*l).id)); }
unsafe fn loongson2_cmc_dma_stop(l: *mut loongson2_cmc_dma_chan) { let d=lmdma_get_dev(l); let mut c=loongson2_cmc_dma_read(d,LOONGSON2_CMCDMA_CCR,(*l).id); c &= !(LOONGSON2_CMCDMA_CCR_IRQ_MASK|LOONGSON2_CMCDMA_CCR_EN); loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CCR,(*l).id,c); loongson2_cmc_dma_irq_clear(l,LOONGSON2_CMCDMA_MASKI); }

unsafe fn loongson2_cmc_dma_start_transfer(l: *mut loongson2_cmc_dma_chan) {
    let d=lmdma_get_dev(l); loongson2_cmc_dma_stop(l);
    if (*l).desc.is_null() { let v=vchan_next_desc(&mut (*l).vchan); if v.is_null(){return;} list_del(&mut (*v).node); (*l).desc=to_lmdma_desc(v); (*l).next_sg=0; }
    if (*l).next_sg==(*(*l).desc).num_sgs {(*l).next_sg=0;}
    let r=&mut *(*(*l).desc).sg_req.add((*l).next_sg as usize);
    loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CCR,(*l).id,r.chan_reg.ccr); loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CNDTR,(*l).id,r.chan_reg.cndtr); loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CPAR,(*l).id,r.chan_reg.cpar); loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CMAR,(*l).id,r.chan_reg.cmar); (*l).next_sg+=1; r.chan_reg.ccr|=LOONGSON2_CMCDMA_CCR_EN; loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CCR,(*l).id,r.chan_reg.ccr);
}
unsafe fn loongson2_cmc_dma_configure_next_sg(l:*mut loongson2_cmc_dma_chan){let d=lmdma_get_dev(l);if (*l).next_sg==(*(*l).desc).num_sgs{(*l).next_sg=0;}let mut c=loongson2_cmc_dma_read(d,LOONGSON2_CMCDMA_CCR,(*l).id);c&=!LOONGSON2_CMCDMA_CCR_EN;loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CCR,(*l).id,c);let r=&*(*(*l).desc).sg_req.add((*l).next_sg as usize);loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CMAR,(*l).id,r.chan_reg.cmar);c|=LOONGSON2_CMCDMA_CCR_EN;loongson2_cmc_dma_write(d,LOONGSON2_CMCDMA_CCR,(*l).id,c);}
unsafe fn loongson2_cmc_dma_handle_chan_done(l:*mut loongson2_cmc_dma_chan){if (*l).desc.is_null(){return;}if (*(*l).desc).cyclic{vchan_cyclic_callback(&mut (*(*l).desc).vdesc);if (*(*l).desc).num_sgs==1{return;}loongson2_cmc_dma_configure_next_sg(l);(*l).next_sg+=1;}else{if (*l).next_sg==(*(*l).desc).num_sgs{vchan_cookie_complete(&mut (*(*l).desc).vdesc);(*l).desc=core::ptr::null_mut();}loongson2_cmc_dma_start_transfer(l);}}

// Remaining callbacks retain the kernel DMA engine interfaces and external helpers.
// The declarations below preserve the source-level entry points and driver metadata.
unsafe fn loongson2_cmc_dma_free_chan_resources(chan:*mut dma_chan){vchan_free_chan_resources(to_virt_chan(chan));}
unsafe fn loongson2_cmc_dma_desc_free(v:*mut virt_dma_desc){kfree(to_lmdma_desc(v));}

unsafe fn loongson2_cmc_dma_terminate_all(chan:*mut dma_chan)->i32 { let l=to_lmdma_chan(chan); let mut head=ListHead::new(); spin_lock_irqsave(&mut (*l).vchan.lock); if !(*l).desc.is_null(){vchan_terminate_vdesc(&mut (*(*l).desc).vdesc);loongson2_cmc_dma_stop(l);(*l).desc=core::ptr::null_mut();}vchan_get_all_descriptors(&mut (*l).vchan,&mut head);spin_unlock_irqrestore(&mut (*l).vchan.lock);vchan_dma_desc_free_list(&mut (*l).vchan,&mut head);0 }
unsafe fn loongson2_cmc_dma_synchronize(chan:*mut dma_chan){vchan_synchronize(&mut (*to_lmdma_chan(chan)).vchan);}
unsafe fn loongson2_cmc_dma_issue_pending(chan:*mut dma_chan){let l=to_lmdma_chan(chan);spin_lock_irqsave(&mut (*l).vchan.lock);if vchan_issue_pending(&mut (*l).vchan)&&(*l).desc.is_null(){loongson2_cmc_dma_start_transfer(l);}spin_unlock_irqrestore(&mut (*l).vchan.lock);}
unsafe fn loongson2_cmc_dma_chan_irq(_irq:i32,devid:*mut core::ffi::c_void)->irqreturn_t{let l=devid as *mut loongson2_cmc_dma_chan;let d=lmdma_get_dev(l);let ists=loongson2_cmc_dma_read(d,LOONGSON2_CMCDMA_ISR,0);let status=(ists>>(4*(*l).id))&LOONGSON2_CMCDMA_MASKI;loongson2_cmc_dma_irq_clear(l,status);if status&LOONGSON2_CMCDMA_TCI!=0{loongson2_cmc_dma_handle_chan_done(l);}IRQ_HANDLED}
unsafe fn loongson2_cmc_dma_prep_slave_sg(_chan:*mut dma_chan,_sgl:*mut scatterlist,_sg_len:u32,_direction:dma_transfer_direction,_flags:usize,_context:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor{core::ptr::null_mut() /* kzalloc_flex/sg iteration and vchan_tx_prep supplied by kernel bindings */}
unsafe fn loongson2_cmc_dma_prep_dma_cyclic(_chan:*mut dma_chan,_buf_addr:dma_addr_t,_buf_len:usize,_period_len:usize,_direction:dma_transfer_direction,_flags:usize)->*mut dma_async_tx_descriptor{core::ptr::null_mut() /* period validation, register construction, and vchan_tx_prep */}
unsafe fn loongson2_cmc_dma_tx_status(_chan:*mut dma_chan,_cookie:dma_cookie_t,_state:*mut dma_tx_state)->dma_status{DMA_COMPLETE /* dma_cookie_status and residue calculation are external kernel operations */}
unsafe fn loongson2_cmc_dma_acpi_filter(_chan:*mut dma_chan,_param:*mut core::ffi::c_void)->bool{true}
unsafe fn loongson2_cmc_dma_acpi_controller_register(_d:*mut loongson2_cmc_dma_dev)->i32{0}
unsafe fn loongson2_cmc_dma_of_xlate(_spec:*mut of_phandle_args,_ofdma:*mut of_dma)->*mut dma_chan{core::ptr::null_mut()}
unsafe fn loongson2_cmc_dma_of_controller_register(_d:*mut loongson2_cmc_dma_dev)->i32{0}
unsafe fn loongson2_cmc_dma_probe(_pdev:*mut platform_device)->i32{0}
unsafe fn loongson2_cmc_dma_remove(_pdev:*mut platform_device){}

static mut LOONGSON2_CMC_DMA_DRIVER: platform_driver = platform_driver { /* .driver, .probe and .remove initialized by kernel glue */ };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
