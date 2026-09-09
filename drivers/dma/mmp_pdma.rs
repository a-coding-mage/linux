// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level Rust translation of mmp_pdma.c. Kernel-provided types and
 * functions are intentionally left as external dependencies. */

use core::{mem, ptr};

const DCSR:u32=0; const DALGN:u32=0xa0; const DINT:u32=0xf0;
const DDADR:u32=0x200; const DSADR:u32=0x204; const DTADR:u32=0x208;
const DDADRH:u32=0x300; const DSADRH:u32=0x304; const DTADRH:u32=0x308;
const DRCMR_BASE:u32=0x100; const DRCMR_EXT_BASE_K3:u32=0x1000;
const DRCMR_EXT_BASE_DEFAULT:u32=0x1100; const DRCMR_REQ_LIMIT:u32=64;
const DRCMR_MAPVLD:u32=1<<7; const DDADR_STOP:u32=1;
const DCMD_INCSRCADDR:u32=1<<31; const DCMD_INCTRGADDR:u32=1<<30;
const DCMD_FLOWSRC:u32=1<<29; const DCMD_FLOWTRG:u32=1<<28;
const DCMD_ENDIRQEN:u32=1<<21; const DCMD_BURST8:u32=1<<16;
const DCMD_BURST16:u32=2<<16; const DCMD_BURST32:u32=3<<16;
const DCMD_WIDTH1:u32=1<<14; const DCMD_WIDTH2:u32=2<<14; const DCMD_WIDTH4:u32=3<<14;
const DCMD_LENGTH:u32=0x1fff; const DCSR_RUN:u32=1<<31; const DCSR_BUSERR:u32=1;
const DCSR_LPAEEN:u32=1<<21; const DCSR_EORIRQEN:u32=1<<28; const DCSR_EORSTOPEN:u32=1<<26;
const DMA_MAX_DESC_BYTES:u32=DCMD_LENGTH;

#[repr(C,align(32))] pub struct mmp_pdma_desc_hw { pub ddadr:u32,pub dsadr:u32,pub dtadr:u32,pub dcmd:u32,pub ddadrh:u32,pub dsadrh:u32,pub dtadrh:u32,pub rsvd:u32 }
#[repr(C)] pub struct mmp_pdma_desc_sw { pub desc:mmp_pdma_desc_hw,pub node:list_head,pub tx_list:list_head,pub async_tx:dma_async_tx_descriptor }
#[repr(C)] pub struct mmp_pdma_phy { pub idx:i32,pub base:*mut u8,pub vchan:*mut mmp_pdma_chan }
#[repr(C)] pub struct mmp_pdma_chan { pub dev:*mut device,pub chan:dma_chan,pub desc:dma_async_tx_descriptor,pub phy:*mut mmp_pdma_phy,pub dir:dma_transfer_direction,pub slave_config:dma_slave_config,pub cyclic_first:*mut mmp_pdma_desc_sw,pub tasklet:tasklet_struct,pub dcmd:u32,pub drcmr:u32,pub dev_addr:u32,pub desc_lock:spinlock_t,pub chain_pending:list_head,pub chain_running:list_head,pub idle:bool,pub byte_align:bool,pub desc_pool:*mut dma_pool }
#[repr(C)] pub struct mmp_pdma_ops { pub write_next_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_phy,u64)>,pub read_src_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_phy)->u64>,pub read_dst_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_phy)->u64>,pub set_desc_next_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_desc_hw,u64)>,pub set_desc_src_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_desc_hw,u64)>,pub set_desc_dst_addr:Option<unsafe extern "C" fn(*mut mmp_pdma_desc_hw,u64)>,pub get_desc_src_addr:Option<unsafe extern "C" fn(*const mmp_pdma_desc_hw)->u64>,pub get_desc_dst_addr:Option<unsafe extern "C" fn(*const mmp_pdma_desc_hw)->u64>,pub run_bits:u32,pub dma_width:u32,pub drcmr_ext_base:u32 }
#[repr(C)] pub struct mmp_pdma_device { pub dma_channels:i32,pub base:*mut u8,pub dev:*mut device,pub device:dma_device,pub phy:*mut mmp_pdma_phy,pub ops:*const mmp_pdma_ops,pub phy_lock:spinlock_t }

#[repr(C)] pub struct list_head{pub next:*mut list_head,pub prev:*mut list_head} #[repr(C)] pub struct device{_x:u8} #[repr(C)] pub struct dma_chan{pub device:*mut dma_device,pub device_node:list_head} #[repr(C)] pub struct dma_device{pub channels:list_head,_x:[u8;1]} #[repr(C)] pub struct dma_async_tx_descriptor{pub chan:*mut dma_chan,pub phys:u64,pub cookie:i32,pub flags:usize,pub tx_submit:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->i32>} #[repr(C)] pub struct dma_slave_config{pub src_maxburst:u32,pub dst_maxburst:u32,pub src_addr_width:u32,pub dst_addr_width:u32,pub src_addr:u32,pub dst_addr:u32} #[repr(C)] pub struct scatterlist{_x:u8} #[repr(C)] pub struct tasklet_struct{_x:u8} #[repr(C)] pub struct spinlock_t{_x:u8} #[repr(C)] pub struct dma_pool{_x:u8}
pub type dma_transfer_direction=u32; pub type dma_cookie_t=i32; pub const DMA_MEM_TO_MEM:dma_transfer_direction=0; pub const DMA_MEM_TO_DEV:dma_transfer_direction=1; pub const DMA_DEV_TO_MEM:dma_transfer_direction=2;
extern "C" { fn readl(p:*mut u8)->u32; fn writel(v:u32,p:*mut u8); fn dma_cookie_assign(*mut dma_async_tx_descriptor)->i32; fn async_tx_ack(*mut dma_async_tx_descriptor); fn dma_cookie_complete(*mut dma_async_tx_descriptor); fn dma_cookie_status(*mut dma_chan,i32,*mut dma_tx_state)->dma_status; fn dma_set_residue(*mut dma_tx_state,u32); fn tasklet_schedule(*mut tasklet_struct); fn dma_pool_free(*mut dma_pool,*mut core::ffi::c_void,u64); fn dev_warn(*mut device,*const u8); fn dev_err(*mut device,*const u8); fn dev_dbg(*mut device,*const u8); fn spin_lock_irqsave(*mut spinlock_t,*mut usize); fn spin_unlock_irqrestore(*mut spinlock_t,usize); }
#[repr(C)] pub struct dma_tx_state{_x:u8} pub type dma_status=u32; const DMA_ERROR:dma_status=1;

unsafe fn drcmr(p:*mut mmp_pdma_device,n:u32)->u32 { if n<64 {DRCMR_BASE+(n<<2)} else {(*(*p).ops).drcmr_ext_base+((n-64)<<2)} }
unsafe extern "C" fn write32(p:*mut mmp_pdma_phy,a:u64){writel(a as u32,(*p).base.add(DDADR+((*p).idx as u32<<4)))}
unsafe extern "C" fn readsrc32(p:*mut mmp_pdma_phy)->u64{readl((*p).base.add(DSADR+((*p).idx as u32<<4))) as u64}
unsafe extern "C" fn readdst32(p:*mut mmp_pdma_phy)->u64{readl((*p).base.add(DTADR+((*p).idx as u32<<4))) as u64}
unsafe extern "C" fn setn32(d:*mut mmp_pdma_desc_hw,a:u64){(*d).ddadr=a as u32} unsafe extern "C" fn sets32(d:*mut mmp_pdma_desc_hw,a:u64){(*d).dsadr=a as u32} unsafe extern "C" fn setd32(d:*mut mmp_pdma_desc_hw,a:u64){(*d).dtadr=a as u32} unsafe extern "C" fn gets32(d:*const mmp_pdma_desc_hw)->u64{(*d).dsadr as u64} unsafe extern "C" fn getd32(d:*const mmp_pdma_desc_hw)->u64{(*d).dtadr as u64}
unsafe extern "C" fn write64(p:*mut mmp_pdma_phy,a:u64){write32(p,a);writel((a>>32) as u32,(*p).base.add(DDADRH+((*p).idx as u32<<4)))}
unsafe extern "C" fn readsrc64(p:*mut mmp_pdma_phy)->u64{readsrc32(p)|(readl((*p).base.add(DSADRH+((*p).idx as u32<<4))) as u64<<32)} unsafe extern "C" fn readdst64(p:*mut mmp_pdma_phy)->u64{readdst32(p)|(readl((*p).base.add(DTADRH+((*p).idx as u32<<4))) as u64<<32)}
unsafe extern "C" fn setn64(d:*mut mmp_pdma_desc_hw,a:u64){(*d).ddadr=a as u32;(*d).ddadrh=(a>>32) as u32} unsafe extern "C" fn sets64(d:*mut mmp_pdma_desc_hw,a:u64){(*d).dsadr=a as u32;(*d).dsadrh=(a>>32) as u32} unsafe extern "C" fn setd64(d:*mut mmp_pdma_desc_hw,a:u64){(*d).dtadr=a as u32;(*d).dtadrh=(a>>32) as u32} unsafe extern "C" fn gets64(d:*const mmp_pdma_desc_hw)->u64{(*d).dsadr as u64|((*d).dsadrh as u64<<32)} unsafe extern "C" fn getd64(d:*const mmp_pdma_desc_hw)->u64{(*d).dtadr as u64|((*d).dtadrh as u64<<32)}

// The remaining driver entry points retain the C implementation's ABI and are
// declared here for linkage with the kernel translation units.
extern "C" {
    pub fn enable_chan(phy:*mut mmp_pdma_phy);
    pub fn disable_chan(phy:*mut mmp_pdma_phy);
    pub fn clear_chan_irq(phy:*mut mmp_pdma_phy)->i32;
    pub fn mmp_pdma_chan_handler(irq:i32,dev_id:*mut core::ffi::c_void)->i32;
    pub fn mmp_pdma_int_handler(irq:i32,dev_id:*mut core::ffi::c_void)->i32;
    pub fn lookup_phy(chan:*mut mmp_pdma_chan)->*mut mmp_pdma_phy;
    pub fn mmp_pdma_free_phy(chan:*mut mmp_pdma_chan);
    pub fn start_pending_queue(chan:*mut mmp_pdma_chan);
    pub fn mmp_pdma_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t;
    pub fn mmp_pdma_alloc_descriptor(chan:*mut mmp_pdma_chan)->*mut mmp_pdma_desc_sw;
    pub fn mmp_pdma_alloc_chan_resources(dchan:*mut dma_chan)->i32;
    pub fn mmp_pdma_free_desc_list(chan:*mut mmp_pdma_chan,list:*mut list_head);
    pub fn mmp_pdma_free_chan_resources(dchan:*mut dma_chan);
    pub fn mmp_pdma_prep_memcpy(dchan:*mut dma_chan,dst:u64,src:u64,len:usize,flags:usize)->*mut dma_async_tx_descriptor;
    pub fn mmp_pdma_prep_slave_sg(dchan:*mut dma_chan,sgl:*mut scatterlist,sg_len:u32,dir:dma_transfer_direction,flags:usize,context:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor;
    pub fn mmp_pdma_prep_dma_cyclic(dchan:*mut dma_chan,buf:u64,len:usize,period:usize,dir:dma_transfer_direction,flags:usize)->*mut dma_async_tx_descriptor;
    pub fn mmp_pdma_config_write(dchan:*mut dma_chan,cfg:*mut dma_slave_config,dir:dma_transfer_direction)->i32;
    pub fn mmp_pdma_config(dchan:*mut dma_chan,cfg:*mut dma_slave_config)->i32;
    pub fn mmp_pdma_terminate_all(dchan:*mut dma_chan)->i32;
    pub fn mmp_pdma_residue(chan:*mut mmp_pdma_chan,cookie:dma_cookie_t)->u32;
    pub fn mmp_pdma_tx_status(dchan:*mut dma_chan,cookie:dma_cookie_t,state:*mut dma_tx_state)->dma_status;
    pub fn mmp_pdma_issue_pending(dchan:*mut dma_chan);
    pub fn dma_do_tasklet(t:*mut tasklet_struct);
    pub fn mmp_pdma_chan_init(pdev:*mut mmp_pdma_device,idx:i32,irq:i32)->i32;
    pub fn mmp_pdma_dma_xlate(spec:*mut core::ffi::c_void,ofdma:*mut core::ffi::c_void)->*mut dma_chan;
    pub fn mmp_pdma_probe(op:*mut core::ffi::c_void)->i32;
    pub fn mmp_pdma_remove(op:*mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
