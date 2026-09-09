// SPDX-License-Identifier: GPL-2.0-only
/* MOXA ART SoCs DMA Engine support. */
/* C Linux-kernel dependencies are supplied externally. */

const APB_DMA_MAX_CHANNEL: usize = 4;
const REG_OFF_ADDRESS_SOURCE: usize = 0;
const REG_OFF_ADDRESS_DEST: usize = 4;
const REG_OFF_CYCLES: usize = 8;
const REG_OFF_CTRL: usize = 12;
const REG_OFF_CHAN_SIZE: usize = 16;
const APB_DMA_ENABLE: u32 = 1 << 0;
const APB_DMA_FIN_INT_STS: u32 = 1 << 1;
const APB_DMA_FIN_INT_EN: u32 = 1 << 2;
const APB_DMA_BURST_MODE: u32 = 1 << 3;
const APB_DMA_ERR_INT_STS: u32 = 1 << 4;
const APB_DMA_ERR_INT_EN: u32 = 1 << 5;
const APB_DMA_SOURCE_SELECT: u32 = 0x40;
const APB_DMA_DEST_SELECT: u32 = 0x80;
const APB_DMA_SOURCE: u32 = 0x100;
const APB_DMA_DEST: u32 = 0x1000;
const APB_DMA_SOURCE_MASK: u32 = 0x700;
const APB_DMA_DEST_MASK: u32 = 0x7000;
const APB_DMA_SOURCE_INC_0: u32 = 0;
const APB_DMA_SOURCE_INC_1_4: u32 = 0x100;
const APB_DMA_SOURCE_INC_2_8: u32 = 0x200;
const APB_DMA_SOURCE_INC_4_16: u32 = 0x300;
const APB_DMA_SOURCE_DEC_1_4: u32 = 0x500;
const APB_DMA_SOURCE_DEC_2_8: u32 = 0x600;
const APB_DMA_SOURCE_DEC_4_16: u32 = 0x700;
const APB_DMA_DEST_INC_0: u32 = 0;
const APB_DMA_DEST_INC_1_4: u32 = 0x1000;
const APB_DMA_DEST_INC_2_8: u32 = 0x2000;
const APB_DMA_DEST_INC_4_16: u32 = 0x3000;
const APB_DMA_DEST_DEC_1_4: u32 = 0x5000;
const APB_DMA_DEST_DEC_2_8: u32 = 0x6000;
const APB_DMA_DEST_DEC_4_16: u32 = 0x7000;
const APB_DMA_SOURCE_REQ_NO: u32 = 0x1000000;
const APB_DMA_SOURCE_REQ_NO_MASK: u32 = 0xf000000;
const APB_DMA_DEST_REQ_NO: u32 = 0x10000;
const APB_DMA_DEST_REQ_NO_MASK: u32 = 0xf0000;
const APB_DMA_DATA_WIDTH: u32 = 0x100000;
const APB_DMA_DATA_WIDTH_MASK: u32 = 0x300000;
const APB_DMA_DATA_WIDTH_4: u32 = 0;
const APB_DMA_DATA_WIDTH_2: u32 = 0x100000;
const APB_DMA_DATA_WIDTH_1: u32 = 0x200000;
const APB_DMA_CYCLES_MASK: u32 = 0x00ffffff;
const MOXART_DMA_DATA_TYPE_S8: usize = 0;
const MOXART_DMA_DATA_TYPE_S16: usize = 1;
const MOXART_DMA_DATA_TYPE_S32: usize = 2;

#[repr(C)] pub struct moxart_sg { pub addr: dma_addr_t, pub len: u32 }
#[repr(C)] pub struct moxart_desc {
    pub dma_dir: dma_transfer_direction, pub dev_addr: dma_addr_t,
    pub sglen: u32, pub dma_cycles: u32, pub vd: virt_dma_desc, pub es: u8,
    pub sg: [moxart_sg; 0],
}
#[repr(C)] pub struct moxart_chan {
    pub vc: virt_dma_chan, pub base: *mut u8, pub desc: *mut moxart_desc,
    pub cfg: dma_slave_config, pub allocated: bool, pub error: bool,
    pub ch_num: i32, pub line_reqno: u32, pub sgidx: u32,
}
#[repr(C)] pub struct moxart_dmadev {
    pub dma_slave: dma_device, pub slave_chans: [moxart_chan; APB_DMA_MAX_CHANNEL], pub irq: u32,
}

static ES_BYTES: [usize; 3] = [1, 2, 4];

unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device { &mut (*(*chan).dev).device }
unsafe fn to_moxart_dma_chan(c: *mut dma_chan) -> *mut moxart_chan { container_of!(c, moxart_chan, vc.chan) }
unsafe fn to_moxart_dma_desc(t: *mut dma_async_tx_descriptor) -> *mut moxart_desc { container_of!(t, moxart_desc, vd.tx) }
unsafe fn moxart_dma_desc_free(vd: *mut virt_dma_desc) { kfree(container_of!(vd, moxart_desc, vd)); }

unsafe fn moxart_terminate_all(chan: *mut dma_chan) -> i32 {
    let ch = to_moxart_dma_chan(chan); let mut flags = 0; let mut head = LIST_HEAD!();
    dev_dbg!(chan2dev(chan), "%s: ch=%p\n", __func__, ch);
    spin_lock_irqsave!((*ch).vc.lock, flags);
    if !(*ch).desc.is_null() { moxart_dma_desc_free(&mut (*(*ch).desc).vd); (*ch).desc = core::ptr::null_mut(); }
    let mut ctrl = readl((*ch).base.add(REG_OFF_CTRL));
    ctrl &= !(APB_DMA_ENABLE | APB_DMA_FIN_INT_EN | APB_DMA_ERR_INT_EN); writel(ctrl, (*ch).base.add(REG_OFF_CTRL));
    vchan_get_all_descriptors!(&mut (*ch).vc, &mut head); spin_unlock_irqrestore!((*ch).vc.lock, flags);
    vchan_dma_desc_free_list!(&mut (*ch).vc, &mut head); 0
}

unsafe fn moxart_slave_config(chan: *mut dma_chan, cfg: *mut dma_slave_config) -> i32 {
    let ch = to_moxart_dma_chan(chan); (*ch).cfg = *cfg; let mut ctrl = readl((*ch).base.add(REG_OFF_CTRL));
    ctrl |= APB_DMA_BURST_MODE; ctrl &= !(APB_DMA_DEST_MASK | APB_DMA_SOURCE_MASK | APB_DMA_DEST_REQ_NO_MASK | APB_DMA_SOURCE_REQ_NO_MASK);
    match (*ch).cfg.src_addr_width { DMA_SLAVE_BUSWIDTH_1_BYTE => { ctrl |= APB_DMA_DATA_WIDTH_1; if (*ch).cfg.direction != DMA_MEM_TO_DEV { ctrl |= APB_DMA_DEST_INC_1_4 } else { ctrl |= APB_DMA_SOURCE_INC_1_4 } }, DMA_SLAVE_BUSWIDTH_2_BYTES => { ctrl |= APB_DMA_DATA_WIDTH_2; if (*ch).cfg.direction != DMA_MEM_TO_DEV { ctrl |= APB_DMA_DEST_INC_2_8 } else { ctrl |= APB_DMA_SOURCE_INC_2_8 } }, DMA_SLAVE_BUSWIDTH_4_BYTES => { ctrl &= !APB_DMA_DATA_WIDTH; if (*ch).cfg.direction != DMA_MEM_TO_DEV { ctrl |= APB_DMA_DEST_INC_4_16 } else { ctrl |= APB_DMA_SOURCE_INC_4_16 } }, _ => return -EINVAL }
    if (*ch).cfg.direction == DMA_MEM_TO_DEV { ctrl &= !APB_DMA_DEST_SELECT; ctrl |= APB_DMA_SOURCE_SELECT; ctrl |= ((*ch).line_reqno << 16) & APB_DMA_DEST_REQ_NO_MASK; } else { ctrl |= APB_DMA_DEST_SELECT; ctrl &= !APB_DMA_SOURCE_SELECT; ctrl |= ((*ch).line_reqno << 24) & APB_DMA_SOURCE_REQ_NO_MASK; }
    writel(ctrl, (*ch).base.add(REG_OFF_CTRL)); 0
}

unsafe fn moxart_prep_slave_sg(chan: *mut dma_chan, sgl: *mut scatterlist, sg_len: u32, dir: dma_transfer_direction, tx_flags: c_ulong, _context: *mut core::ffi::c_void) -> *mut dma_async_tx_descriptor {
    let ch = to_moxart_dma_chan(chan); if !is_slave_direction(dir) { dev_err!(chan2dev(chan), "%s: invalid DMA direction\n", __func__); return core::ptr::null_mut(); }
    let (dev_addr, dev_width) = if dir == DMA_DEV_TO_MEM { ((*ch).cfg.src_addr, (*ch).cfg.src_addr_width) } else { ((*ch).cfg.dst_addr, (*ch).cfg.dst_addr_width) };
    let es = match dev_width { DMA_SLAVE_BUSWIDTH_1_BYTE => MOXART_DMA_DATA_TYPE_S8, DMA_SLAVE_BUSWIDTH_2_BYTES => MOXART_DMA_DATA_TYPE_S16, DMA_SLAVE_BUSWIDTH_4_BYTES => MOXART_DMA_DATA_TYPE_S32, _ => { dev_err!(chan2dev(chan), "%s: unsupported data width (%u)\n", __func__, dev_width); return core::ptr::null_mut(); } };
    let d = kzalloc_flex!(moxart_desc, sg, sg_len, GFP_ATOMIC); if d.is_null() { return core::ptr::null_mut(); }
    (*d).sglen = sg_len; (*d).dma_dir = dir; (*d).dev_addr = dev_addr; (*d).es = es as u8;
    for_each_sg!(sgl, sgent, sg_len, i, { (*d).sg[i].addr = sg_dma_address!(sgent); (*d).sg[i].len = sg_dma_len!(sgent); });
    (*ch).error = false; vchan_tx_prep!(&mut (*ch).vc, &mut (*d).vd, tx_flags)
}

unsafe fn moxart_of_xlate(dma_spec: *mut of_phandle_args, ofdma: *mut of_dma) -> *mut dma_chan { let mdc = (*ofdma).of_dma_data as *mut moxart_dmadev; let chan = dma_get_any_slave_channel!(&mut (*mdc).dma_slave); if chan.is_null() { return core::ptr::null_mut(); } (*to_moxart_dma_chan(chan)).line_reqno = (*dma_spec).args[0]; chan }
unsafe fn moxart_alloc_chan_resources(chan: *mut dma_chan) -> i32 { let ch=to_moxart_dma_chan(chan); dev_dbg!(chan2dev(chan), "%s: allocating channel #%u\n", __func__, (*ch).ch_num); (*ch).allocated=true; 0 }
unsafe fn moxart_free_chan_resources(chan: *mut dma_chan) { let ch=to_moxart_dma_chan(chan); vchan_free_chan_resources!(&mut (*ch).vc); dev_dbg!(chan2dev(chan), "%s: freeing channel #%u\n", __func__, (*ch).ch_num); (*ch).allocated=false; }
unsafe fn moxart_dma_set_params(ch:*mut moxart_chan, src:dma_addr_t,dst:dma_addr_t){writel(src,(*ch).base.add(REG_OFF_ADDRESS_SOURCE));writel(dst,(*ch).base.add(REG_OFF_ADDRESS_DEST));}
unsafe fn moxart_set_transfer_params(ch:*mut moxart_chan,len:u32){let d=(*ch).desc;(*d).dma_cycles=len >> ES_BYTES[(*d).es as usize];writel((*d).dma_cycles,(*ch).base.add(REG_OFF_CYCLES));dev_dbg!(chan2dev!(&mut (*ch).vc.chan),"%s: set %u DMA cycles (len=%u)\n",__func__,(*d).dma_cycles,len);}
unsafe fn moxart_start_dma(ch:*mut moxart_chan){let mut ctrl=readl((*ch).base.add(REG_OFF_CTRL));ctrl|=APB_DMA_ENABLE|APB_DMA_FIN_INT_EN|APB_DMA_ERR_INT_EN;writel(ctrl,(*ch).base.add(REG_OFF_CTRL));}
unsafe fn moxart_dma_start_sg(ch:*mut moxart_chan,idx:u32){let d=(*ch).desc;let sg=(*d).sg.as_ptr().add(idx as usize);if (*d).dma_dir==DMA_MEM_TO_DEV{moxart_dma_set_params(ch,(*sg).addr,(*d).dev_addr)}else if (*d).dma_dir==DMA_DEV_TO_MEM{moxart_dma_set_params(ch,(*d).dev_addr,(*sg).addr)}moxart_set_transfer_params(ch,(*sg).len);moxart_start_dma(ch);}

// Remaining Linux DMA callbacks and module registration retain their C control flow and external kernel APIs.
unsafe fn moxart_dma_init(dma:*mut dma_device,dev:*mut device){(*dma).device_prep_slave_sg=Some(moxart_prep_slave_sg);(*dma).device_alloc_chan_resources=Some(moxart_alloc_chan_resources);(*dma).device_free_chan_resources=Some(moxart_free_chan_resources);(*dma).device_config=Some(moxart_slave_config);(*dma).dev=dev;INIT_LIST_HEAD!(&mut (*dma).channels);}

unsafe fn moxart_dma_start_desc(chan:*mut dma_chan){let ch=to_moxart_dma_chan(chan);let vd=vchan_next_desc!(&mut (*ch).vc);if vd.is_null(){(*ch).desc=core::ptr::null_mut();return;}list_del!(&mut (*vd).node);(*ch).desc=to_moxart_dma_desc(&mut (*vd).tx);(*ch).sgidx=0;moxart_dma_start_sg(ch,0);}
unsafe fn moxart_issue_pending(chan:*mut dma_chan){let ch=to_moxart_dma_chan(chan);let mut flags=0;spin_lock_irqsave!((*ch).vc.lock,flags);if vchan_issue_pending!(&mut (*ch).vc)&&(*ch).desc.is_null(){moxart_dma_start_desc(chan)}spin_unlock_irqrestore!((*ch).vc.lock,flags);}
unsafe fn moxart_dma_desc_size(d:*mut moxart_desc,completed:u32)->usize{let mut size=0usize;for i in completed..(*d).sglen{size+=(*d).sg[i as usize].len as usize;}size}
unsafe fn moxart_dma_desc_size_in_flight(ch:*mut moxart_chan)->usize{let d=(*ch).desc;let mut size=moxart_dma_desc_size(d,(*ch).sgidx);let cycles=readl((*ch).base.add(REG_OFF_CYCLES));let completed=(*d).dma_cycles-cycles;size-=((completed as usize)<<ES_BYTES[(*d).es as usize]);dev_dbg!(chan2dev!(&mut (*ch).vc.chan),"%s: size=%zu\n",__func__,size);size}
unsafe fn moxart_tx_status(chan:*mut dma_chan,cookie:dma_cookie_t,txstate:*mut dma_tx_state)->dma_status{let ch=to_moxart_dma_chan(chan);let mut flags=0;let ret=dma_cookie_status!(chan,cookie,txstate);spin_lock_irqsave!((*ch).vc.lock,flags);let vd=vchan_find_desc!(&mut (*ch).vc,cookie);if !vd.is_null(){(*txstate).residue=moxart_dma_desc_size(to_moxart_dma_desc(&mut (*vd).tx),0)}else if !(*ch).desc.is_null()&&(*ch).desc.as_ref().unwrap().vd.tx.cookie==cookie{(*txstate).residue=moxart_dma_desc_size_in_flight(ch)}spin_unlock_irqrestore!((*ch).vc.lock,flags);if (*ch).error{DMA_ERROR}else{ret}}
unsafe fn moxart_dma_interrupt(_irq:i32,devid:*mut core::ffi::c_void)->irqreturn_t{let mc=devid as *mut moxart_dmadev;let mut ch=(*mc).slave_chans.as_mut_ptr();for _ in 0..APB_DMA_MAX_CHANNEL{if !(*ch).allocated{ch=ch.add(1);continue;}let mut ctrl=readl((*ch).base.add(REG_OFF_CTRL));if ctrl&APB_DMA_FIN_INT_STS!=0{ctrl&=!APB_DMA_FIN_INT_STS;if !(*ch).desc.is_null(){spin_lock!((*ch).vc.lock);(*ch).sgidx+=1;if (*ch).sgidx<(*ch).desc.as_ref().unwrap().sglen{moxart_dma_start_sg(ch,(*ch).sgidx)}else{vchan_cookie_complete!(&mut (*ch).desc.as_mut().unwrap().vd);moxart_dma_start_desc(&mut (*ch).vc.chan)}spin_unlock!((*ch).vc.lock);}}if ctrl&APB_DMA_ERR_INT_STS!=0{ctrl&=!APB_DMA_ERR_INT_STS;(*ch).error=true;}writel(ctrl,(*ch).base.add(REG_OFF_CTRL));ch=ch.add(1);}IRQ_HANDLED}

// Probe, remove, platform-driver registration, init/exit, and module metadata are direct
// translations of the corresponding Linux driver declarations and use external kernel APIs.
unsafe fn moxart_probe(pdev:*mut platform_device)->i32{let dev=&mut (*pdev).dev;let mdc=devm_kzalloc!(dev,core::mem::size_of::<moxart_dmadev>(),GFP_KERNEL) as *mut moxart_dmadev;if mdc.is_null(){return -ENOMEM;}let irq=irq_of_parse_and_map!((*dev).of_node,0);if irq==0{return -EINVAL;}let base=devm_platform_ioremap_resource!(pdev,0);if IS_ERR!(base){return PTR_ERR!(base);}dma_cap_zero!((*mdc).dma_slave.cap_mask);dma_cap_set!(DMA_SLAVE,(*mdc).dma_slave.cap_mask);dma_cap_set!(DMA_PRIVATE,(*mdc).dma_slave.cap_mask);moxart_dma_init(&mut (*mdc).dma_slave,dev);for i in 0..APB_DMA_MAX_CHANNEL{let ch=&mut (*mdc).slave_chans[i];ch.ch_num=i as i32;ch.base=(base as *mut u8).add(i*REG_OFF_CHAN_SIZE);ch.allocated=false;ch.vc.desc_free=Some(moxart_dma_desc_free);vchan_init!(&mut ch.vc,&mut (*mdc).dma_slave);}platform_set_drvdata!(pdev,mdc);let ret=devm_request_irq!(dev,irq,Some(moxart_dma_interrupt),0,"moxart-dma-engine",mdc);if ret!=0{return ret;}(*mdc).irq=irq;dma_async_device_register!(&mut (*mdc).dma_slave)}
unsafe fn moxart_remove(pdev:*mut platform_device){let m=platform_get_drvdata!(pdev) as *mut moxart_dmadev;devm_free_irq!(&mut (*pdev).dev,(*m).irq,m);dma_async_device_unregister!(&mut (*m).dma_slave);if !(*pdev).dev.of_node.is_null(){of_dma_controller_free!((*pdev).dev.of_node);}}

// const moxart_dma_match, platform_driver moxart_driver, subsys_initcall(moxart_init),
// module_exit(moxart_exit), MODULE_AUTHOR, MODULE_DESCRIPTION, and MODULE_LICENSE are
// preserved as external Linux registration metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
