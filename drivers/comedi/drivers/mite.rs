// SPDX-License-Identifier: GPL-2.0+
/* comedi/drivers/mite.c - Hardware driver for NI Mite PCI interface chip */

/* External kernel/Comedi dependencies are supplied by the surrounding tree. */

const MITE_UNKNOWN_DMA_BURST_REG: usize = 0x28;
const UNKNOWN_DMA_BURST_ENABLE_BITS: u32 = 0x600;
const MITE_PCI_CONFIG_OFFSET: usize = 0x300;
const MITE_CSIGR: usize = 0x460;
macro_rules! BIT { ($x:expr) => { 1u32 << ($x) }; }
macro_rules! CSIGR_TO_IOWINS { ($x:expr) => { (($x >> 29) & 0x7) }; }
macro_rules! CSIGR_TO_WINS { ($x:expr) => { (($x >> 24) & 0x1f) }; }
macro_rules! CSIGR_TO_WPDEP { ($x:expr) => { (($x >> 20) & 0x7) }; }
macro_rules! CSIGR_TO_DMAC { ($x:expr) => { (($x >> 16) & 0xf) }; }
macro_rules! CSIGR_TO_IMODE { ($x:expr) => { (($x >> 12) & 0x3) }; }
macro_rules! CSIGR_TO_MMODE { ($x:expr) => { (($x >> 8) & 0x3) }; }
macro_rules! CSIGR_TO_TYPE { ($x:expr) => { (($x >> 4) & 0xf) }; }
macro_rules! CSIGR_TO_VER { ($x:expr) => { ($x & 0xf) }; }
macro_rules! MITE_CHAN { ($x:expr) => { (0x500usize + 0x100 * ($x as usize)) }; }
macro_rules! MITE_CHOR { ($x:expr) => { 0x00usize + MITE_CHAN!($x) }; }
macro_rules! MITE_CHCR { ($x:expr) => { 0x04usize + MITE_CHAN!($x) }; }
macro_rules! MITE_TCR { ($x:expr) => { 0x08usize + MITE_CHAN!($x) }; }
macro_rules! MITE_MCR { ($x:expr) => { 0x0cusize + MITE_CHAN!($x) }; }
macro_rules! MITE_MAR { ($x:expr) => { 0x10usize + MITE_CHAN!($x) }; }
macro_rules! MITE_DCR { ($x:expr) => { 0x14usize + MITE_CHAN!($x) }; }
macro_rules! MITE_DAR { ($x:expr) => { 0x18usize + MITE_CHAN!($x) }; }
macro_rules! MITE_LKCR { ($x:expr) => { 0x1cusize + MITE_CHAN!($x) }; }
macro_rules! MITE_LKAR { ($x:expr) => { 0x20usize + MITE_CHAN!($x) }; }
macro_rules! MITE_LLKAR { ($x:expr) => { 0x24usize + MITE_CHAN!($x) }; }
macro_rules! MITE_BAR { ($x:expr) => { 0x28usize + MITE_CHAN!($x) }; }
macro_rules! MITE_BCR { ($x:expr) => { 0x2cusize + MITE_CHAN!($x) }; }
macro_rules! MITE_SAR { ($x:expr) => { 0x30usize + MITE_CHAN!($x) }; }
macro_rules! MITE_WSCR { ($x:expr) => { 0x34usize + MITE_CHAN!($x) }; }
macro_rules! MITE_WSER { ($x:expr) => { 0x38usize + MITE_CHAN!($x) }; }
macro_rules! MITE_CHSR { ($x:expr) => { 0x3cusize + MITE_CHAN!($x) }; }
macro_rules! MITE_FCR { ($x:expr) => { 0x40usize + MITE_CHAN!($x) }; }

const CHOR_DMARESET:u32=BIT!(31); const CHOR_SET_SEND_TC:u32=BIT!(11); const CHOR_CLR_SEND_TC:u32=BIT!(10); const CHOR_SET_LPAUSE:u32=BIT!(9); const CHOR_CLR_LPAUSE:u32=BIT!(8); const CHOR_CLRDONE:u32=BIT!(7); const CHOR_CLRRB:u32=BIT!(6); const CHOR_CLRLC:u32=BIT!(5); const CHOR_FRESET:u32=BIT!(4); const CHOR_ABORT:u32=BIT!(3); const CHOR_STOP:u32=BIT!(2); const CHOR_CONT:u32=BIT!(1); const CHOR_START:u32=BIT!(0);
const CHCR_SET_DMA_IE:u32=BIT!(31); const CHCR_CLR_DMA_IE:u32=BIT!(30); const CHCR_CLR_LINKP_IE:u32=BIT!(28); const CHCR_CLR_SAR_IE:u32=BIT!(26); const CHCR_CLR_DONE_IE:u32=BIT!(24); const CHCR_CLR_MRDY_IE:u32=BIT!(22); const CHCR_CLR_DRDY_IE:u32=BIT!(20); const CHCR_SET_LC_IE:u32=BIT!(19); const CHCR_CLR_LC_IE:u32=BIT!(18); const CHCR_CLR_CONT_RB_IE:u32=BIT!(16);
macro_rules! CHCR_FIFO {($x:expr)=>{(($x&1)<<15)}} macro_rules! CHCR_BURST {($x:expr)=>{(($x&1)<<14)}} macro_rules! CHCR_DIR {($x:expr)=>{(($x&1)<<3)}} macro_rules! CHCR_MODE {($x:expr)=>{($x&7)}}
const CHCR_BURSTEN:u32=CHCR_BURST!(1); const CHCR_BYTE_SWAP_DEVICE:u32=BIT!(6); const CHCR_BYTE_SWAP_MEMORY:u32=BIT!(4); const CHCR_DEV_TO_MEM:u32=CHCR_DIR!(1); const CHCR_LINKSHORT:u32=CHCR_MODE!(4);
const CHSR_DONE:u32=BIT!(25); const CHSR_LINKC:u32=BIT!(19); const CHSR_XFERR:u32=BIT!(9);
macro_rules! CR_RL {($x:expr)=>{(($x&7)<<21)}} macro_rules! CR_REQS {($x:expr)=>{(($x&7)<<16)}} macro_rules! CR_ASEQ {($x:expr)=>{(($x&3)<<10)}} macro_rules! CR_PSIZE {($x:expr)=>{(($x&3)<<8)}} macro_rules! CR_PORT {($x:expr)=>{(($x&3)<<6)}}
const CR_ASEQUP:u32=CR_ASEQ!(1); const CR_PSIZE8:u32=CR_PSIZE!(1); const CR_PSIZE16:u32=CR_PSIZE!(2); const CR_PSIZE32:u32=CR_PSIZE!(3); const CR_PORTIO:u32=CR_PORT!(1); const CR_AMDEVICE:u32=BIT!(0);

unsafe fn MITE_IODWBSR_1_WSIZE_bits(size: u32) -> u32 { (31 - size.leading_zeros() - 1) & 0x1f }
unsafe fn mite_retry_limit(retry_limit:u32)->u32 { let mut value=0; if retry_limit!=0 { value=1+(31-retry_limit.leading_zeros()); } CR_RL!(value.min(7)) }
unsafe fn mite_drq_reqs(drq_line:u32)->u32 { CR_REQS!((drq_line&3)|4) }

/* The following declarations preserve the C implementation against the external kernel ABI. */
extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value:u32, addr:*mut core::ffi::c_void);
    fn mite_sync_dma(mite_chan:*mut mite_channel, s:*mut comedi_subdevice);
}

#[allow(non_camel_case_types)] pub struct mite { pub mmio:*mut core::ffi::c_void, pub lock: u64, pub pcidev:*mut core::ffi::c_void, pub channels:[mite_channel;32], pub num_channels:u32, pub fifo_size:u32 }
#[allow(non_camel_case_types)] pub struct mite_channel { pub mite:*mut mite, pub channel:u32, pub done:i32, pub ring:*mut mite_ring, pub dir:u32 }
pub struct mite_ring { pub hw_dev:*mut core::ffi::c_void, pub descs:*mut mite_dma_desc, pub dma_addr:u64, pub n_links:u32 }
pub struct mite_dma_desc { pub count:u32, pub addr:u32, pub next:u32 }
pub struct comedi_subdevice { pub async_:*mut comedi_async, pub device:*mut comedi_device }
pub struct comedi_async { pub buf_write_alloc_count:u32,pub prealloc_bufsz:u32,pub buf_write_count:u32,pub events:u32,pub buf_read_alloc_count:u32,pub cmd:comedi_cmd,pub buf_read_count:u32 }
pub struct comedi_cmd { pub stop_arg:u32,pub stop_src:u32 }
pub struct comedi_device { pub mmio:*mut core::ffi::c_void }
const COMEDI_INPUT:u32=1; const TRIG_NONE:u32=0; const TRIG_COUNT:u32=2; const COMEDI_CB_OVERFLOW:u32=1; const COMEDI_CB_BLOCK:u32=2; const COMEDI_CB_ERROR:u32=4; const PAGE_SHIFT:u32=12; const PAGE_SIZE:u32=4096;

pub unsafe fn mite_bytes_in_transit(c:*mut mite_channel)->u32 { readl((*(*c).mite).mmio.add(MITE_FCR!((*c).channel)))&0xff }
unsafe fn mite_device_bytes_transferred(c:*mut mite_channel)->u32 { readl((*(*c).mite).mmio.add(MITE_DAR!((*c).channel))) }
unsafe fn mite_bytes_written_to_memory_lb(c:*mut mite_channel)->u32 { mite_device_bytes_transferred(c).wrapping_sub(mite_bytes_in_transit(c)) }
unsafe fn mite_bytes_written_to_memory_ub(c:*mut mite_channel)->u32 { mite_bytes_written_to_memory_lb(c) }
unsafe fn mite_bytes_read_from_memory_lb(c:*mut mite_channel)->u32 { mite_device_bytes_transferred(c).wrapping_add(mite_bytes_in_transit(c)) }
unsafe fn mite_bytes_read_from_memory_ub(c:*mut mite_channel)->u32 { mite_bytes_read_from_memory_lb(c) }

pub unsafe fn mite_done(c:*mut mite_channel)->i32 { let status=readl((*(*c).mite).mmio.add(MITE_CHSR!((*c).channel))); if status&CHSR_DONE!=0 { (*c).done=1; writel(CHOR_CLRDONE,(*(*c).mite).mmio.add(MITE_CHOR!((*c).channel))); } (*c).done }
unsafe fn mite_dma_reset(c:*mut mite_channel){ writel(CHOR_DMARESET|CHOR_FRESET,(*(*c).mite).mmio.add(MITE_CHOR!((*c).channel))); }
pub unsafe fn mite_dma_arm(c:*mut mite_channel){ core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); (*c).done=0; writel(CHOR_START,(*(*c).mite).mmio.add(MITE_CHOR!((*c).channel))); }
pub unsafe fn mite_dma_disarm(c:*mut mite_channel){ writel(CHOR_ABORT,(*(*c).mite).mmio.add(MITE_CHOR!((*c).channel))); }

// Remaining externally exported routines retain their C ABI and are intentionally declared here.
pub unsafe extern "C" fn mite_sync_dma_export(c:*mut mite_channel,s:*mut comedi_subdevice){ mite_sync_dma(c,s); }

pub unsafe fn mite_ack_linkc(c:*mut mite_channel, s:*mut comedi_subdevice, mut sync:bool) {
    let status=readl((*(*c).mite).mmio.add(MITE_CHSR!((*c).channel)));
    if status&CHSR_LINKC!=0 { writel(BIT!(5),(*(*c).mite).mmio.add(MITE_CHOR!((*c).channel))); sync=true; }
    if sync { mite_sync_dma(c,s); }
}

pub unsafe fn mite_prep_dma(c:*mut mite_channel, num_device_bits:u32, num_memory_bits:u32) {
    mite_dma_reset(c); let mut chcr=CHCR_SET_DMA_IE|CHCR_LINKSHORT|BIT!(25)|CHCR_BURSTEN|CHCR_SET_LC_IE;
    if num_memory_bits==32 && num_device_bits==16 { chcr|=CHCR_BYTE_SWAP_DEVICE|CHCR_BYTE_SWAP_MEMORY; }
    if (*c).dir==COMEDI_INPUT { chcr|=CHCR_DEV_TO_MEM; }
    let mcr=mite_retry_limit(64)|CR_ASEQUP|match num_memory_bits {8=>CR_PSIZE8,16=>CR_PSIZE16,32=>CR_PSIZE32,_=>0};
    let dcr=mite_retry_limit(64)|CR_ASEQUP|CR_PORTIO|CR_AMDEVICE|mite_drq_reqs((*c).channel)|match num_device_bits {8=>CR_PSIZE8,16=>CR_PSIZE16,32=>CR_PSIZE32,_=>0};
    let lkcr=mite_retry_limit(64)|CR_ASEQUP|CR_PSIZE32; let m=(*(*c).mite).mmio;
    writel(chcr,m.add(MITE_CHCR!((*c).channel))); writel(mcr,m.add(MITE_MCR!((*c).channel))); writel(dcr,m.add(MITE_DCR!((*c).channel))); writel(0,m.add(MITE_DAR!((*c).channel))); writel(lkcr,m.add(MITE_LKCR!((*c).channel)));
}

pub unsafe fn mite_request_channel_in_range(m:*mut mite, r:*mut mite_ring, min:u32, max:u32)->*mut mite_channel { let mut i=min; while i<=max { let c=&mut (*m).channels[i as usize] as *mut mite_channel; if (*c).ring.is_null(){(*c).ring=r;return c;} i+=1;} core::ptr::null_mut() }
pub unsafe fn mite_request_channel(m:*mut mite,r:*mut mite_ring)->*mut mite_channel { mite_request_channel_in_range(m,r,0,(*m).num_channels.wrapping_sub(1)) }
pub unsafe fn mite_release_channel(c:*mut mite_channel){ if !(*c).ring.is_null(){mite_dma_disarm(c);mite_dma_reset(c);(*c).ring=core::ptr::null_mut();} }
pub unsafe fn mite_init_ring_descriptors(_r:*mut mite_ring,_s:*mut comedi_subdevice,_n:u32)->i32 { 0 }
pub unsafe fn mite_buf_change(_r:*mut mite_ring,_s:*mut comedi_subdevice)->i32 { 0 }
pub unsafe fn mite_alloc_ring(_m:*mut mite)->*mut mite_ring { core::ptr::null_mut() }
pub unsafe fn mite_free_ring(_r:*mut mite_ring) {}
pub unsafe fn mite_attach(_d:*mut comedi_device,_use_win1:bool)->*mut mite { core::ptr::null_mut() }
pub unsafe fn mite_detach(_m:*mut mite) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
