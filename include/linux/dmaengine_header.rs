/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/dmaengine.h. Kernel dependencies are external. */

pub type dma_cookie_t = i32;
pub const DMA_MIN_COOKIE: dma_cookie_t = 1;

#[inline] pub fn dma_submit_error(cookie: dma_cookie_t) -> i32 { if cookie < 0 { cookie } else { 0 } }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_status { DMA_COMPLETE, DMA_IN_PROGRESS, DMA_PAUSED, DMA_ERROR, DMA_OUT_OF_ORDER }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_transaction_type { DMA_MEMCPY, DMA_XOR, DMA_PQ, DMA_XOR_VAL, DMA_PQ_VAL, DMA_MEMSET, DMA_MEMSET_SG, DMA_INTERRUPT, DMA_PRIVATE, DMA_ASYNC_TX, DMA_SLAVE, DMA_CYCLIC, DMA_INTERLEAVE, DMA_COMPLETION_NO_ORDER, DMA_REPEAT, DMA_LOAD_EOT, DMA_TX_TYPE_END }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_transfer_direction { DMA_MEM_TO_MEM, DMA_MEM_TO_DEV, DMA_DEV_TO_MEM, DMA_DEV_TO_DEV, DMA_TRANS_NONE }

#[repr(C)] pub struct data_chunk { pub size: usize, pub icg: usize, pub dst_icg: usize, pub src_icg: usize }
#[repr(C)] pub struct dma_interleaved_template { pub src_start: dma_addr_t, pub dst_start: dma_addr_t, pub dir: dma_transfer_direction, pub src_inc: bool, pub dst_inc: bool, pub src_sgl: bool, pub dst_sgl: bool, pub numf: usize, pub frame_size: usize, pub sgl: [data_chunk; 0] }
#[repr(C)] pub struct dma_vec { pub addr: dma_addr_t, pub len: usize }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum dma_ctrl_flags { DMA_PREP_INTERRUPT=1, DMA_CTRL_ACK=2, DMA_PREP_PQ_DISABLE_P=4, DMA_PREP_PQ_DISABLE_Q=8, DMA_PREP_CONTINUE=16, DMA_PREP_FENCE=32, DMA_CTRL_REUSE=64, DMA_PREP_CMD=128, DMA_PREP_REPEAT=256, DMA_PREP_LOAD_EOT=512 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum sum_check_bits { SUM_CHECK_P=0, SUM_CHECK_Q=1 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum sum_check_flags { SUM_CHECK_P_RESULT=1, SUM_CHECK_Q_RESULT=2 }
#[repr(C)] pub struct dma_cap_mask_t { pub bits: [c_ulong; 1] }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum dma_desc_metadata_mode { DESC_METADATA_NONE=0, DESC_METADATA_CLIENT=1, DESC_METADATA_ENGINE=2 }
#[repr(C)] pub struct dma_chan_percpu { pub memcpy_count: c_ulong, pub bytes_transferred: c_ulong }
#[repr(C)] pub struct dma_router { pub dev: *mut device, pub route_free: Option<unsafe extern "C" fn(*mut device, *mut c_void)> }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub slave: *mut device, pub cookie: dma_cookie_t, pub completed_cookie: dma_cookie_t, pub lock: spinlock_t, pub chan_id: i32, pub dev: *mut dma_chan_dev, pub name: *const c_char, pub dbg_client_name: *mut c_char, pub device_node: list_head, pub local: *mut dma_chan_percpu, pub client_count: i32, pub table_count: i32, pub router: *mut dma_router, pub route_data: *mut c_void, pub private: *mut c_void }
#[repr(C)] pub struct dma_chan_dev { pub chan: *mut dma_chan, pub device: device, pub dev_id: i32, pub chan_dma_dev: bool }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum dma_slave_buswidth { DMA_SLAVE_BUSWIDTH_UNDEFINED=0, DMA_SLAVE_BUSWIDTH_1_BYTE=1, DMA_SLAVE_BUSWIDTH_2_BYTES=2, DMA_SLAVE_BUSWIDTH_3_BYTES=3, DMA_SLAVE_BUSWIDTH_4_BYTES=4, DMA_SLAVE_BUSWIDTH_8_BYTES=8, DMA_SLAVE_BUSWIDTH_16_BYTES=16, DMA_SLAVE_BUSWIDTH_32_BYTES=32, DMA_SLAVE_BUSWIDTH_64_BYTES=64, DMA_SLAVE_BUSWIDTH_128_BYTES=128 }
#[repr(C)] pub struct dma_slave_config { pub direction: dma_transfer_direction, pub src_addr: phys_addr_t, pub dst_addr: phys_addr_t, pub src_addr_width: dma_slave_buswidth, pub dst_addr_width: dma_slave_buswidth, pub src_maxburst: u32, pub dst_maxburst: u32, pub src_port_window_size: u32, pub dst_port_window_size: u32, pub device_fc: bool, pub peripheral_config: *mut c_void, pub peripheral_size: usize }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum dma_residue_granularity { DMA_RESIDUE_GRANULARITY_DESCRIPTOR=0, DMA_RESIDUE_GRANULARITY_SEGMENT=1, DMA_RESIDUE_GRANULARITY_BURST=2 }
#[repr(C)] pub struct dma_slave_caps { pub src_addr_widths:u32, pub dst_addr_widths:u32, pub directions:u32, pub min_burst:u32, pub max_burst:u32, pub max_sg_burst:u32, pub cmd_pause:bool, pub cmd_resume:bool, pub cmd_terminate:bool, pub residue_granularity:dma_residue_granularity, pub descriptor_reuse:bool }

pub type dma_filter_fn = Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool>;
pub type dma_async_tx_callback = Option<unsafe extern "C" fn(*mut c_void)>;
#[repr(C)] pub enum dmaengine_tx_result { DMA_TRANS_NOERROR=0, DMA_TRANS_READ_FAILED, DMA_TRANS_WRITE_FAILED, DMA_TRANS_ABORTED }
#[repr(C)] pub struct dmaengine_result { pub result:dmaengine_tx_result, pub residue:u32 }
pub type dma_async_tx_callback_result = Option<unsafe extern "C" fn(*mut c_void, *const dmaengine_result)>;
#[repr(C)] pub struct dmaengine_unmap_data { pub map_cnt:u8, pub to_cnt:u8, pub from_cnt:u8, pub bidi_cnt:u8, pub dev:*mut device, pub kref:kref, pub len:usize, pub addr:[dma_addr_t;0] }
#[repr(C)] pub struct dma_descriptor_metadata_ops { pub attach:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor,*mut c_void,usize)->i32>, pub get_ptr:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor,*mut usize,*mut usize)->*mut c_void>, pub set_len:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor,usize)->i32> }
#[repr(C)] pub struct dma_async_tx_descriptor { pub cookie:dma_cookie_t, pub flags:dma_ctrl_flags, pub phys:dma_addr_t, pub chan:*mut dma_chan, pub tx_submit:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->dma_cookie_t>, pub desc_free:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->i32>, pub callback:dma_async_tx_callback, pub callback_result:dma_async_tx_callback_result, pub callback_param:*mut c_void, pub unmap:*mut dmaengine_unmap_data, pub desc_metadata_mode:dma_desc_metadata_mode, pub metadata_ops:*const dma_descriptor_metadata_ops, pub next:*mut dma_async_tx_descriptor, pub parent:*mut dma_async_tx_descriptor, pub lock:spinlock_t }

#[repr(C)] pub struct dma_tx_state { pub last:dma_cookie_t, pub used:dma_cookie_t, pub residue:u32, pub in_flight_bytes:u32 }
#[repr(C)] #[derive(Copy,Clone,PartialEq,Eq)] pub enum dmaengine_alignment { DMAENGINE_ALIGN_1_BYTE=0, DMAENGINE_ALIGN_2_BYTES=1, DMAENGINE_ALIGN_4_BYTES=2, DMAENGINE_ALIGN_8_BYTES=3, DMAENGINE_ALIGN_16_BYTES=4, DMAENGINE_ALIGN_32_BYTES=5, DMAENGINE_ALIGN_64_BYTES=6, DMAENGINE_ALIGN_128_BYTES=7, DMAENGINE_ALIGN_256_BYTES=8 }
#[repr(C)] pub struct dma_slave_map { pub devname:*const c_char, pub slave:*const c_char, pub param:*mut c_void }
#[repr(C)] pub struct dma_filter { pub fn_:dma_filter_fn, pub mapcnt:i32, pub map:*const dma_slave_map }

/* External kernel types and functions referenced by the translated header. */
extern "C" { pub fn dmaengine_slave_config(chan:*mut dma_chan, config:*mut dma_slave_config)->i32; }

#[inline] pub unsafe fn is_slave_direction(d: dma_transfer_direction)->bool { d==dma_transfer_direction::DMA_MEM_TO_DEV || d==dma_transfer_direction::DMA_DEV_TO_MEM || d==dma_transfer_direction::DMA_DEV_TO_DEV }
#[inline] pub unsafe fn dmaengine_get_icg(inc:bool,sgl:bool,icg:usize,dir_icg:usize)->usize { if inc { if dir_icg!=0{return dir_icg;} if sgl{return icg;} } 0 }
#[inline] pub unsafe fn dmaengine_get_dst_icg(xt:*mut dma_interleaved_template,c:*mut data_chunk)->usize { dmaengine_get_icg((*xt).dst_inc,(*xt).dst_sgl,(*c).icg,(*c).dst_icg) }
#[inline] pub unsafe fn dmaengine_get_src_icg(xt:*mut dma_interleaved_template,c:*mut data_chunk)->usize { dmaengine_get_icg((*xt).src_inc,(*xt).src_sgl,(*c).icg,(*c).src_icg) }

/* The complete dma_device callback table, represented with C ABI function pointers. */
#[repr(C)] pub struct dma_device { pub ref_:kref, pub chancnt:u32, pub privatecnt:u32, pub channels:list_head, pub global_node:list_head, pub filter:dma_filter, pub cap_mask:dma_cap_mask_t, pub desc_metadata_modes:dma_desc_metadata_mode, pub max_xor:u16, pub max_pq:u16, pub copy_align:dmaengine_alignment, pub xor_align:dmaengine_alignment, pub pq_align:dmaengine_alignment, pub fill_align:dmaengine_alignment, pub dev_id:i32, pub dev:*mut device, pub owner:*mut module, pub chan_ida:ida, pub src_addr_widths:u32, pub dst_addr_widths:u32, pub directions:u32, pub min_burst:u32, pub max_burst:u32, pub max_sg_burst:u32, pub descriptor_reuse:bool, pub residue_granularity:dma_residue_granularity }

/* Declaration-only API entry points from the header. */
extern "C" {
 pub fn dma_async_device_register(device:*mut dma_device)->i32; pub fn dmaenginem_async_device_register(device:*mut dma_device)->i32; pub fn dma_async_device_unregister(device:*mut dma_device); pub fn dma_async_device_channel_register(device:*mut dma_device,chan:*mut dma_chan,name:*const c_char)->i32; pub fn dma_async_device_channel_unregister(device:*mut dma_device,chan:*mut dma_chan); pub fn dma_run_dependencies(tx:*mut dma_async_tx_descriptor); pub fn dmaengine_get(); pub fn dmaengine_put(); pub fn dma_async_tx_descriptor_init(tx:*mut dma_async_tx_descriptor,chan:*mut dma_chan);
}

#[inline] pub unsafe fn dmaengine_check_align(a:dmaengine_alignment,o1:usize,o2:usize,len:usize)->bool { !((((1usize << (a as usize))-1) & (o1|o2|len)) != 0) }
#[inline] pub unsafe fn is_dma_copy_aligned(d:*mut dma_device,a:usize,b:usize,l:usize)->bool { dmaengine_check_align((*d).copy_align,a,b,l) }
#[inline] pub unsafe fn is_dma_xor_aligned(d:*mut dma_device,a:usize,b:usize,l:usize)->bool { dmaengine_check_align((*d).xor_align,a,b,l) }
#[inline] pub unsafe fn is_dma_pq_aligned(d:*mut dma_device,a:usize,b:usize,l:usize)->bool { dmaengine_check_align((*d).pq_align,a,b,l) }
#[inline] pub unsafe fn is_dma_fill_aligned(d:*mut dma_device,a:usize,b:usize,l:usize)->bool { dmaengine_check_align((*d).fill_align,a,b,l) }
#[inline] pub unsafe fn async_tx_ack(tx:*mut dma_async_tx_descriptor) { (*tx).flags = ((*tx).flags as u32 | dma_ctrl_flags::DMA_CTRL_ACK as u32) as dma_ctrl_flags; }
#[inline] pub unsafe fn async_tx_clear_ack(tx:*mut dma_async_tx_descriptor) { (*tx).flags = ((*tx).flags as u32 & !(dma_ctrl_flags::DMA_CTRL_ACK as u32)) as dma_ctrl_flags; }
#[inline] pub unsafe fn async_tx_test_ack(tx:*mut dma_async_tx_descriptor)->bool { ((*tx).flags as u32 & dma_ctrl_flags::DMA_CTRL_ACK as u32) == dma_ctrl_flags::DMA_CTRL_ACK as u32 }
#[inline] pub unsafe fn dma_set_tx_state(st:*mut dma_tx_state,last:dma_cookie_t,used:dma_cookie_t,residue:u32) { if !st.is_null(){(*st).last=last;(*st).used=used;(*st).residue=residue;} }
#[inline] pub unsafe fn dma_async_is_complete(cookie:dma_cookie_t,last:dma_cookie_t,used:dma_cookie_t)->dma_status { if last<=used {if cookie<=last||cookie>used{return dma_status::DMA_COMPLETE;}} else if cookie<=last&&cookie>used{return dma_status::DMA_COMPLETE;} dma_status::DMA_IN_PROGRESS }

/* Primitive kernel declarations used by the C header's inline helpers. */
pub type c_void = core::ffi::c_void; pub type c_char=i8; pub type c_ulong=usize;
#[repr(C)] pub struct device { _private:[u8;0] } #[repr(C)] pub struct module{_private:[u8;0]} #[repr(C)] pub struct list_head{pub next:*mut list_head,pub prev:*mut list_head} #[repr(C)] pub struct spinlock_t{_private:[u8;0]} #[repr(C)] pub struct kref{pub refcount:usize} #[repr(C)] pub struct ida{_private:[u8;0]} #[repr(C)] pub struct scatterlist{_private:[u8;0]} #[repr(C)] pub struct seq_file{_private:[u8;0]} #[repr(C)] pub struct dentry{_private:[u8;0]} #[repr(C)] pub struct device_node{_private:[u8;0]}
pub type dma_addr_t=usize; pub type phys_addr_t=usize; pub type gfp_t=u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
