// SPDX-License-Identifier: GPL-2.0+
/* Renesas SuperH DMA Engine support. Literal translation of shdmac.c. */

const SAR: u32 = 0x00;
const DAR: u32 = 0x04;
const TCR: u32 = 0x08;
const CHCR: u32 = 0x0c;
const DMAOR: u32 = 0x40;
const TEND: u32 = 0x18;
const SH_DMAE_DRV_NAME: &str = "sh-dma-engine";
const LOG2_DEFAULT_XFER_SIZE: i32 = 2;
const SH_DMA_SLAVE_NUMBER: i32 = 256;
const SH_DMA_TCR_MAX: usize = 16 * 1024 * 1024 - 1;

static mut sh_dmae_lock: usize = 0;
static mut sh_dmae_devices: usize = 0;

unsafe fn channel_clear(sh_dc: *mut sh_dmae_chan) {
    let shdev = to_sh_dev(sh_dc);
    let chan_pdata = (*(*shdev).pdata).channel.add((*sh_dc).shdma_chan.id as usize);
    let val: u32 = if (*(*shdev).pdata).chclr_bitwise { 1 << (*chan_pdata).chclr_bit } else { 0 };
    __raw_writel(val, (*shdev).chan_reg.add((*chan_pdata).chclr_offset as usize));
}
unsafe fn sh_dmae_writel(c: *mut sh_dmae_chan, data: u32, reg: u32) { __raw_writel(data, (*c).base.add(reg as usize)); }
unsafe fn sh_dmae_readl(c: *mut sh_dmae_chan, reg: u32) -> u32 { __raw_readl((*c).base.add(reg as usize)) }
unsafe fn dmaor_read(d: *mut sh_dmae_device) -> u16 { let a=(*d).chan_reg.add(DMAOR as usize); if (*(*d).pdata).dmaor_is_32bit { __raw_readl(a) as u16 } else { __raw_readw(a) } }
unsafe fn dmaor_write(d: *mut sh_dmae_device, data: u16) { let a=(*d).chan_reg.add(DMAOR as usize); if (*(*d).pdata).dmaor_is_32bit { __raw_writel(data as u32,a) } else { __raw_writew(data,a) } }
unsafe fn chcr_write(c: *mut sh_dmae_chan, data: u32) { let d=to_sh_dev(c); __raw_writel(data,(*c).base.add((*d).chcr_offset as usize)); }
unsafe fn chcr_read(c: *mut sh_dmae_chan) -> u32 { let d=to_sh_dev(c); __raw_readl((*c).base.add((*d).chcr_offset as usize)) }

unsafe fn sh_dmae_ctl_stop(d: *mut sh_dmae_device) { let _g=spin_lock_irqsave(&mut sh_dmae_lock); let v=dmaor_read(d); dmaor_write(d,v & !(DMAOR_NMIF|DMAOR_AE|DMAOR_DME)); }
unsafe fn sh_dmae_rst(d: *mut sh_dmae_device) -> i32 {
    let _g=spin_lock_irqsave(&mut sh_dmae_lock); let mut v=dmaor_read(d)&!(DMAOR_NMIF|DMAOR_AE|DMAOR_DME);
    if (*(*d).pdata).chclr_present { for i in 0..(*(*d).pdata).channel_num { let c=*(*d).chan.add(i as usize); if !c.is_null(){channel_clear(c);} } }
    dmaor_write(d,v|(*(*d).pdata).dmaor_init); v=dmaor_read(d);
    if v & (DMAOR_AE|DMAOR_NMIF) != 0 { dev_warn((*d).shdma_dev.dma_dev.dev,"Can't initialize DMAOR.\n"); return -EIO; }
    if (*(*d).pdata).dmaor_init & !v != 0 { dev_warn((*d).shdma_dev.dma_dev.dev,"DMAOR=0x%x hasn't latched the initial value 0x%x.\n",v,(*(*d).pdata).dmaor_init); } 0
}
unsafe fn dmae_is_busy(c:*mut sh_dmae_chan)->bool { let v=chcr_read(c); v&(CHCR_DE|CHCR_TE)==CHCR_DE }
unsafe fn calc_xmit_shift(c:*mut sh_dmae_chan,chcr:u32)->u32 { let p=(*to_sh_dev(c)).pdata; let mut n=((chcr&(*p).ts_low_mask)>>(*p).ts_low_shift)|((chcr&(*p).ts_high_mask)>>(*p).ts_high_shift); if n>=(*p).ts_shift_num {n=0;} (*p).ts_shift[n as usize] }
unsafe fn log2size_to_chcr(c:*mut sh_dmae_chan,l:i32)->u32 { let p=(*to_sh_dev(c)).pdata; let mut i=0; while i<(*p).ts_shift_num && (*p).ts_shift[i as usize]!=l {i+=1;} if i==(*p).ts_shift_num{i=0;} ((i<<(*p).ts_low_shift)&(*p).ts_low_mask)|((i<<(*p).ts_high_shift)&(*p).ts_high_mask) }
unsafe fn dmae_set_reg(c:*mut sh_dmae_chan,h:*mut sh_dmae_regs){sh_dmae_writel(c,(*h).sar,SAR);sh_dmae_writel(c,(*h).dar,DAR);sh_dmae_writel(c,(*h).tcr>>(*c).xmit_shift,TCR);}
unsafe fn dmae_start(c:*mut sh_dmae_chan){let d=to_sh_dev(c);if (*(*d).pdata).needs_tend_set{sh_dmae_writel(c,0xffff_ffff,TEND);}let v=chcr_read(c)|CHCR_DE|(*d).chcr_ie_bit;chcr_write(c,v&!CHCR_TE);}
unsafe fn dmae_init(c:*mut sh_dmae_chan){let v=DM_INC|SM_INC|RS_AUTO|log2size_to_chcr(c,LOG2_DEFAULT_XFER_SIZE);(*c).xmit_shift=calc_xmit_shift(c,v);chcr_write(c,v);}
unsafe fn dmae_set_chcr(c:*mut sh_dmae_chan,v:u32)->i32{if dmae_is_busy(c){return -EBUSY;}(*c).xmit_shift=calc_xmit_shift(c,v);chcr_write(c,v);0}
unsafe fn dmae_set_dmars(c:*mut sh_dmae_chan,v:u16)->i32{let d=to_sh_dev(c);let p=(*d).pdata;let cp=&(*p).channel[(*c).shdma_chan.id as usize];if dmae_is_busy(c){return -EBUSY;}if (*p).no_dmars{return 0;}let mut a=(*d).dmars;if a.is_null(){a=(*d).chan_reg;}a=a.add(cp.dmars as usize);__raw_writew((__raw_readw(a)&(0xff00>>cp.dmars_bit))|(v<<cp.dmars_bit),a);0}

unsafe fn sh_dmae_start_xfer(s:*mut shdma_chan, sd:*mut shdma_desc){let c=container_of_chan(s);let d=container_of_desc(sd);dmae_set_reg(c,&mut (*d).hw);dmae_start(c);}
unsafe fn sh_dmae_channel_busy(s:*mut shdma_chan)->bool{dmae_is_busy(container_of_chan(s))}
unsafe fn sh_dmae_setup_xfer(s:*mut shdma_chan,id:i32)->i32{let c=container_of_chan(s);if id>=0{let cfg=(*c).config;let r=dmae_set_dmars(c,(*cfg).mid_rid);if r<0{return r;}return dmae_set_chcr(c,(*cfg).chcr);}dmae_init(c);0}
unsafe fn dmae_halt(c:*mut sh_dmae_chan){let d=to_sh_dev(c);let v=chcr_read(c)&!(CHCR_DE|CHCR_TE|(*d).chcr_ie_bit);chcr_write(c,v);}
unsafe fn sh_dmae_halt(s:*mut shdma_chan){dmae_halt(container_of_chan(s));}
unsafe fn sh_dmae_desc_setup(s:*mut shdma_chan,sd:*mut shdma_desc,src:dma_addr_t,dst:dma_addr_t,len:*mut usize)->i32{let d=container_of_desc(sd);if *len>(*s).max_xfer_len{*len=(*s).max_xfer_len;}(*d).hw.sar=src;(*d).hw.dar=dst;(*d).hw.tcr=*len;0}
unsafe fn sh_dmae_chan_irq(s:*mut shdma_chan,_irq:i32)->bool{let c=container_of_chan(s);if chcr_read(c)&CHCR_TE==0{return false;}dmae_halt(c);true}
unsafe fn sh_dmae_get_partial(s:*mut shdma_chan,sd:*mut shdma_desc)->usize{let c=container_of_chan(s);let d=container_of_desc(sd);(*d).hw.tcr-(sh_dmae_readl(c,TCR) as usize<<(*c).xmit_shift)}
unsafe fn sh_dmae_desc_completed(s:*mut shdma_chan,sd:*mut shdma_desc)->bool{let c=container_of_chan(s);let d=container_of_desc(sd);let sar=sh_dmae_readl(c,SAR) as dma_addr_t;let dar=sh_dmae_readl(c,DAR) as dma_addr_t;((*sd).direction==DMA_DEV_TO_MEM&&(*d).hw.dar+(*d).hw.tcr==dar)||((*sd).direction!=DMA_DEV_TO_MEM&&(*d).hw.sar+(*d).hw.tcr==sar)}

// Remaining Linux driver registration, PM, IRQ, notifier, probe, remove, and module metadata
// retain their C interfaces and are supplied by the surrounding kernel bindings.
extern "C" { fn sh_dmae_probe(pdev:*mut platform_device)->i32; fn sh_dmae_remove(pdev:*mut platform_device); fn sh_dmae_init()->i32; fn sh_dmae_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
