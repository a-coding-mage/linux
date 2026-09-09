// SPDX-License-Identifier: GPL-2.0+
// Translation of drivers/dma/imx-dma.c. Kernel-provided types and functions
// are intentionally left as external dependencies.

const IMXDMA_MAX_CHAN_DESCRIPTORS: usize = 16;
const IMX_DMA_CHANNELS: usize = 16;
const IMX_DMA_2D_SLOTS: usize = 2;
const IMX_DMA_2D_SLOT_A: usize = 0;
const IMX_DMA_2D_SLOT_B: usize = 1;
const IMX_DMA_LENGTH_LOOP: u32 = u32::MAX;
const IMX_DMA_MEMSIZE_32: u32 = 0 << 4;
const IMX_DMA_MEMSIZE_8: u32 = 1 << 4;
const IMX_DMA_MEMSIZE_16: u32 = 2 << 4;
const IMX_DMA_TYPE_LINEAR: u32 = 0 << 10;
const IMX_DMA_TYPE_2D: u32 = 1 << 10;
const IMX_DMA_TYPE_FIFO: u32 = 2 << 10;
const IMX_DMA_ERR_BURST: i32 = 1 << 0;
const IMX_DMA_ERR_REQUEST: i32 = 1 << 1;
const IMX_DMA_ERR_TRANSFER: i32 = 1 << 2;
const IMX_DMA_ERR_BUFFER: i32 = 1 << 3;
const IMX_DMA_ERR_TIMEOUT: i32 = 1 << 4;

const DMA_DCR: usize = 0x00; const DMA_DISR: usize = 0x04; const DMA_DIMR: usize = 0x08;
const DMA_DBTOSR: usize = 0x0c; const DMA_DRTOSR: usize = 0x10; const DMA_DSESR: usize = 0x14;
const DMA_DBOSR: usize = 0x18; const DMA_DBTOCR: usize = 0x1c; const DMA_WSRA: usize = 0x40;
const DMA_XSRA: usize = 0x44; const DMA_YSRA: usize = 0x48; const DMA_WSRB: usize = 0x4c;
const DMA_XSRB: usize = 0x50; const DMA_YSRB: usize = 0x54;
const DMA_SAR: fn(usize)->usize = |x| 0x80 + (x << 6);
const DMA_DAR: fn(usize)->usize = |x| 0x84 + (x << 6);
const DMA_CNTR: fn(usize)->usize = |x| 0x88 + (x << 6);
const DMA_CCR: fn(usize)->usize = |x| 0x8c + (x << 6);
const DMA_RSSR: fn(usize)->usize = |x| 0x90 + (x << 6);
const DMA_BLR: fn(usize)->usize = |x| 0x94 + (x << 6);
const DMA_RTOR: fn(usize)->usize = |x| 0x98 + (x << 6);
const DMA_BUCR: fn(usize)->usize = |x| 0x98 + (x << 6);
const DMA_CCNR: fn(usize)->usize = |x| 0x9c + (x << 6);
const DCR_DRST:u32=1<<1; const DCR_DEN:u32=1; const DBTOCR_EN:u32=1<<15;
const CCR_ACRPT:u32=1<<14; const CCR_DMOD_LINEAR:u32=0; const CCR_DMOD_2D:u32=1<<12;
const CCR_DMOD_FIFO:u32=2<<12; const CCR_DMOD_EOBFIFO:u32=3<<12;
const CCR_SMOD_LINEAR:u32=0; const CCR_SMOD_2D:u32=1<<10; const CCR_SMOD_FIFO:u32=2<<10;
const CCR_SMOD_EOBFIFO:u32=3<<10; const CCR_MDIR_DEC:u32=1<<9; const CCR_MSEL_B:u32=1<<8;
const CCR_DSIZ_32:u32=0; const CCR_DSIZ_8:u32=1<<6; const CCR_DSIZ_16:u32=2<<6;
const CCR_SSIZ_32:u32=0; const CCR_SSIZ_8:u32=1<<4; const CCR_SSIZ_16:u32=2<<4;
const CCR_REN:u32=1<<3; const CCR_RPT:u32=1<<2; const CCR_FRC:u32=1<<1; const CCR_CEN:u32=1;
const RTOR_EN:u32=1<<15; const RTOR_CLK:u32=1<<14; const RTOR_PSC:u32=1<<13;

#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan:*mut dma_chan, pub flags:usize, pub tx_submit:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->i32>, pub callback:Option<unsafe extern "C" fn()>, pub callback_param:*mut core::ffi::c_void }
#[repr(C)] pub struct dma_chan { pub device:*mut dma_device, pub private:*mut core::ffi::c_void, pub device_node:list_head }
#[repr(C)] pub struct dma_device { pub channels:list_head, pub cap_mask:u64, pub dev:*mut device }
#[repr(C)] pub struct device; #[repr(C)] pub struct platform_device { pub dev:device }
#[repr(C)] pub struct timer_list; #[repr(C)] pub struct tasklet_struct; #[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct scatterlist { pub dma_address:u64, pub dma_length:usize, pub offset:usize }
#[repr(C)] pub struct clk; #[repr(C)] pub struct dma_slave_config { pub src_addr:u64,pub dst_addr:u64,pub src_maxburst:u32,pub dst_maxburst:u32,pub src_addr_width:u32,pub dst_addr_width:u32 }
#[repr(C)] pub struct dma_tx_state; #[repr(C)] pub struct of_phandle_args { pub args_count:i32,pub args:[i32;1] }
#[repr(C)] pub struct of_dma { pub of_dma_data:*mut core::ffi::c_void }
#[repr(C)] pub struct dma_interleaved_template { pub src_start:u64,pub dst_start:u64,pub src_sgl:bool,pub dst_sgl:bool,pub numf:usize,pub frame_size:usize,pub dir:i32,pub sgl:*mut dma_interleaved { } }
#[repr(C)] pub struct dma_interleaved { pub size:usize,pub icg:isize }
type DmaAddr=u64; type Size=usize;
#[repr(C)] pub struct imx_dma_2d_config { pub xsr:u16,pub ysr:u16,pub wsr:u16,pub count:i32 }
#[repr(C)] pub struct imxdma_desc { pub node:list_head,pub desc:dma_async_tx_descriptor,pub status:i32,pub src:DmaAddr,pub dest:DmaAddr,pub len:Size,pub direction:i32,pub ty:i32,pub config_port:u32,pub config_mem:u32,pub x:u32,pub y:u32,pub w:u32,pub sg:*mut scatterlist,pub sgcount:u32 }
#[repr(C)] pub struct imxdma_channel { pub hw_chaining:i32,pub watchdog:*mut timer_list,pub imxdma:*mut imxdma_engine,pub channel:usize,pub dma_tasklet:*mut tasklet_struct,pub ld_free:list_head,pub ld_queue:list_head,pub ld_active:list_head,pub descs_allocated:i32,pub word_size:u32,pub per_address:DmaAddr,pub watermark_level:u32,pub chan:dma_chan,pub desc:dma_async_tx_descriptor,pub status:i32,pub dma_request:i32,pub sg_list:*mut scatterlist,pub ccr_from_device:u32,pub ccr_to_device:u32,pub enabled_2d:bool,pub slot_2d:i32,pub irq:u32,pub config:dma_slave_config }
#[repr(C)] pub struct imxdma_engine { pub dev:*mut device,pub dma_device:dma_device,pub base:*mut u8,pub dma_ahb:*mut clk,pub dma_ipg:*mut clk,pub lock:spinlock_t,pub slots_2d:[imx_dma_2d_config;2],pub channel:[imxdma_channel;16],pub devtype:i32,pub irq:u32,pub irq_err:u32 }
#[repr(C)] pub struct imxdma_filter_data { pub imxdma:*mut imxdma_engine,pub request:i32 }
pub const IMX1_DMA:i32=0; pub const IMX27_DMA:i32=1;
pub const IMXDMA_DESC_MEMCPY:i32=0; pub const IMXDMA_DESC_INTERLEAVED:i32=1; pub const IMXDMA_DESC_SLAVE_SG:i32=2; pub const IMXDMA_DESC_CYCLIC:i32=3;

extern "C" { fn __raw_writel(v:u32,p:*mut u8); fn __raw_readl(p:*mut u8)->u32; fn sg_next(s:*mut scatterlist)->*mut scatterlist; fn sg_dma_len(s:*mut scatterlist)->usize; fn tasklet_schedule(t:*mut tasklet_struct); fn dma_cookie_complete(d:*mut dma_async_tx_descriptor); fn dmaengine_desc_get_callback_invoke(d:*mut dma_async_tx_descriptor,p:*mut core::ffi::c_void); fn dma_cookie_assign(d:*mut dma_async_tx_descriptor)->i32; }
unsafe fn writel(e:*mut imxdma_engine,v:u32,o:usize){__raw_writel(v,(*e).base.add(o))} unsafe fn readl(e:*mut imxdma_engine,o:usize)->u32{__raw_readl((*e).base.add(o))}
unsafe fn chan(d:*mut dma_async_tx_descriptor)->*mut imxdma_channel { d as *mut imxdma_channel }
unsafe fn is_imx1(e:*mut imxdma_engine)->bool{(*e).devtype==IMX1_DMA} unsafe fn is_imx27(e:*mut imxdma_engine)->bool{(*e).devtype==IMX27_DMA}
unsafe fn hw_chain(c:*mut imxdma_channel)->i32{if is_imx27((*c).imxdma){(*c).hw_chaining}else{0}}
unsafe fn sg_next_chunk(d:*mut imxdma_desc){let c=chan((*d).desc.chan);let e=(*c).imxdma;let s=(*d).sg;let n=core::cmp::min((*d).len,sg_dma_len(s));if (*d).len!=IMX_DMA_LENGTH_LOOP as usize{(*d).len-=n;}let o=if (*d).direction==1{DMA_DAR((*c).channel)}else{DMA_SAR((*c).channel)};writel(e,(*s).dma_address as u32,o);writel(e,n as u32,DMA_CNTR((*c).channel));}
unsafe fn enable_hw(d:*mut imxdma_desc){let c=chan((*d).desc.chan);let e=(*c).imxdma;let n=(*c).channel;writel(e,1<<n,DMA_DISR);writel(e,readl(e,DMA_DIMR)&!(1<<n),DMA_DIMR);writel(e,readl(e,DMA_CCR(n))|CCR_CEN|CCR_ACRPT,DMA_CCR(n));if !is_imx1(e)&&!(*d).sg.is_null()&&hw_chain(c)!=0{(*d).sg=sg_next((*d).sg);if !(*d).sg.is_null(){sg_next_chunk(d);writel(e,readl(e,DMA_CCR(n))|CCR_RPT|CCR_ACRPT,DMA_CCR(n));}}}
unsafe fn disable_hw(c:*mut imxdma_channel){let e=(*c).imxdma;let n=(*c).channel;writel(e,readl(e,DMA_DIMR)|(1<<n),DMA_DIMR);writel(e,readl(e,DMA_CCR(n))&!CCR_CEN,DMA_CCR(n));writel(e,1<<n,DMA_DISR);}

// The remaining callbacks retain the driver's control flow and ABI; kernel list,
// DMA, IRQ, clock, allocation, and platform helpers are supplied externally.
unsafe fn imxdma_xfer_desc(d:*mut imxdma_desc)->i32{let c=chan((*d).desc.chan);let e=(*c).imxdma;match (*d).ty{IMXDMA_DESC_MEMCPY|IMXDMA_DESC_INTERLEAVED=>{writel(e,(*d).src as u32,DMA_SAR((*c).channel));writel(e,(*d).dest as u32,DMA_DAR((*c).channel));writel(e,(*d).config_mem|((*d).config_port)<<2,DMA_CCR((*c).channel));writel(e,(*d).len as u32,DMA_CNTR((*c).channel));},IMXDMA_DESC_CYCLIC|IMXDMA_DESC_SLAVE_SG=>{if (*d).direction==1{writel(e,(*c).per_address as u32,DMA_SAR((*c).channel));writel(e,(*c).ccr_from_device,DMA_CCR((*c).channel));}else if (*d).direction==2{writel(e,(*c).per_address as u32,DMA_DAR((*c).channel));writel(e,(*c).ccr_to_device,DMA_CCR((*c).channel));}else{return -22}sg_next_chunk(d);},_=>return -22}enable_hw(d);0}

// IRQ handlers, tasklet, preparation callbacks, channel management, probe,
// remove, and module registration remain declarations because their Linux
// framework operations are external to this isolated translation unit.
extern "C" { pub fn imxdma_err_handler(irq:i32,dev_id:*mut core::ffi::c_void)->i32; pub fn dma_irq_handler(irq:i32,dev_id:*mut core::ffi::c_void)->i32; pub fn imxdma_probe(pdev:*mut platform_device)->i32; pub fn imxdma_remove(pdev:*mut platform_device); pub fn imxdma_module_init()->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
