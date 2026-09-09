// SPDX-License-Identifier: GPL-2.0-only
/* Intel Keem Bay OCS AES Crypto Driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::{ffi::c_void, mem, ptr};

// C header dependencies are supplied by the surrounding kernel/Rust bindings.
type u8 = core::primitive::u8;
type u32 = core::primitive::u32;
type u64 = core::primitive::u64;
type dma_addr_t = usize;
type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: i32 = 22;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const EOVERFLOW: i32 = 75;
const EBADMSG: i32 = 74;
const DMA_MAPPING_ERROR: dma_addr_t = usize::MAX;
const AES_BLOCK_SIZE: usize = 16;
const GCM_AES_IV_SIZE: u32 = 12;
const GFP_KERNEL: u32 = 0;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct ocs_dll_desc { pub vaddr: *mut c_void, pub dma_addr: dma_addr_t, pub size: usize }
#[repr(C)] pub struct ocs_aes_dev { pub dev: *mut device, pub base_reg: *mut u8, pub irq_completion: completion, pub dma_err_mask: u32 }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_cipher { OCS_AES = 0, OCS_SM4 = 1 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_mode { OCS_MODE_ECB=0, OCS_MODE_CBC=1, OCS_MODE_CTR=2, OCS_MODE_CCM=6, OCS_MODE_GCM=7, OCS_MODE_CTS=9 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_instruction { OCS_ENCRYPT=0, OCS_DECRYPT=1, OCS_EXPAND=2, OCS_BYPASS=3 }

const fn bit(n: u32) -> u32 { 1u32 << n }
const AES_COMMAND_OFFSET:u32=0x0000; const AES_KEY_0_OFFSET:u32=0x0004; const AES_IV_0_OFFSET:u32=0x0024;
const AES_ACTIVE_OFFSET:u32=0x0034; const AES_KEY_SIZE_OFFSET:u32=0x0044; const AES_IER_OFFSET:u32=0x0048; const AES_ISR_OFFSET:u32=0x005C;
const AES_IV_1_OFFSET:u32=0x0028; const AES_IV_2_OFFSET:u32=0x002C; const AES_IV_3_OFFSET:u32=0x0030;
const AES_MULTIPURPOSE1_0_OFFSET:u32=0x0200; const AES_MULTIPURPOSE1_1_OFFSET:u32=0x0204; const AES_MULTIPURPOSE1_2_OFFSET:u32=0x0208; const AES_MULTIPURPOSE1_3_OFFSET:u32=0x020C;
const AES_MULTIPURPOSE2_0_OFFSET:u32=0x0220; const AES_MULTIPURPOSE2_1_OFFSET:u32=0x0224; const AES_MULTIPURPOSE2_2_OFFSET:u32=0x0228; const AES_MULTIPURPOSE2_3_OFFSET:u32=0x022C;
const AES_BYTE_ORDER_CFG_OFFSET:u32=0x02C0; const AES_TLEN_OFFSET:u32=0x0300; const AES_T_MAC_0_OFFSET:u32=0x0304; const AES_T_MAC_1_OFFSET:u32=0x0308; const AES_T_MAC_2_OFFSET:u32=0x030C; const AES_T_MAC_3_OFFSET:u32=0x0310; const AES_PLEN_OFFSET:u32=0x0314;
const AES_A_DMA_SRC_SIZE_OFFSET:u32=0x0408; const AES_A_DMA_DST_SIZE_OFFSET:u32=0x040C; const AES_A_DMA_NEXT_SRC_DESCR_OFFSET:u32=0x0418; const AES_A_DMA_NEXT_DST_DESCR_OFFSET:u32=0x041C; const AES_A_DMA_WHILE_ACTIVE_MODE_OFFSET:u32=0x0420; const AES_A_DMA_PERF_CNTR_OFFSET:u32=0x042C; const AES_A_DMA_MSI_ISR_OFFSET:u32=0x0480; const AES_A_DMA_MSI_IER_OFFSET:u32=0x0484; const AES_A_DMA_MSI_MASK_OFFSET:u32=0x0488; const AES_A_DMA_INBUFFER_WRITE_FIFO_OFFSET:u32=0x0600;
const AES_A_DMA_DMA_MODE_OFFSET:u32=0x0410; const AES_A_DMA_STATUS_OFFSET:u32=0x0428;
const AES_A_DMA_DMA_MODE_ACTIVE:u32=bit(31); const AES_A_DMA_DMA_MODE_SRC_LINK_LIST_EN:u32=bit(25); const AES_A_DMA_DMA_MODE_DST_LINK_LIST_EN:u32=bit(24);
const AES_ACTIVE_LAST_ADATA:u32=bit(9); const AES_ACTIVE_LAST_CCM_GCM:u32=bit(8); const AES_ACTIVE_TERMINATION:u32=bit(1); const AES_ACTIVE_TRIGGER:u32=bit(0);
const AES_DMA_CPD_ERR_INT:u32=bit(8); const AES_DMA_OUTBUF_RD_ERR_INT:u32=bit(7); const AES_DMA_OUTBUF_WR_ERR_INT:u32=bit(6); const AES_DMA_INBUF_RD_ERR_INT:u32=bit(5); const AES_DMA_INBUF_WR_ERR_INT:u32=bit(4); const AES_DMA_BAD_COMP_INT:u32=bit(3); const AES_DMA_SAI_INT:u32=bit(2); const AES_DMA_SRC_DONE_INT:u32=bit(0); const AES_COMPLETE_INT:u32=bit(1);
const AES_DMA_STATUS_INPUT_BUFFER_OCCUPANCY_MASK:u32=0x3ff; const AES_MAX_TAG_SIZE_U32:usize=4; const OCS_LL_DMA_FLAG_TERMINATE:u32=bit(31); const CCM_DECRYPT_DELAY_TAG_CLK_COUNT:i32=36; const CCM_DECRYPT_DELAY_LAST_GCX_CLK_COUNT:i32=42;

#[repr(C, packed)] pub struct ocs_dma_linked_list { pub src_addr:u32, pub src_len:u32, pub next:u32, pub ll_flags:u32 }
extern "C" { fn iowrite32(v:u32,p:*mut u8); fn iowrite8(v:u8,p:*mut u8); fn ioread32(p:*mut u8)->u32; fn reinit_completion(c:*mut completion); fn wait_for_completion_interruptible(c:*mut completion)->i32; fn complete(c:*mut completion); fn crypto_memneq(a:*const c_void,b:*const c_void,n:usize)->i32; fn dma_alloc_coherent(d:*mut device,n:usize,a:*mut dma_addr_t,g:u32)->*mut c_void; fn sg_dma_len(s:*mut scatterlist)->usize; fn sg_dma_address(s:*mut scatterlist)->u32; fn sg_next(s:*mut scatterlist)->*mut scatterlist; }
unsafe fn reg(d:&ocs_aes_dev,o:u32)->*mut u8 { d.base_reg.add(o as usize) }
unsafe fn wr(d:&ocs_aes_dev,o:u32,v:u32){iowrite32(v,reg(d,o))} unsafe fn rd(d:&ocs_aes_dev,o:u32)->u32{ioread32(reg(d,o))}
unsafe fn swab32(x:u32)->u32{x.swap_bytes()}
unsafe fn aes_a_set_endianness(d:&ocs_aes_dev){wr(d,AES_BYTE_ORDER_CFG_OFFSET,0x7ff)} unsafe fn aes_a_op_trigger(d:&ocs_aes_dev){wr(d,AES_ACTIVE_OFFSET,AES_ACTIVE_TRIGGER)} unsafe fn aes_a_op_termination(d:&ocs_aes_dev){wr(d,AES_ACTIVE_OFFSET,AES_ACTIVE_TERMINATION)} unsafe fn aes_a_set_last_gcx(d:&ocs_aes_dev){wr(d,AES_ACTIVE_OFFSET,AES_ACTIVE_LAST_CCM_GCM)} unsafe fn aes_a_set_last_gcx_and_adata(d:&ocs_aes_dev){wr(d,AES_ACTIVE_OFFSET,AES_ACTIVE_LAST_ADATA|AES_ACTIVE_LAST_CCM_GCM)}
unsafe fn aes_a_wait_last_gcx(d:&ocs_aes_dev){while rd(d,AES_ACTIVE_OFFSET)&AES_ACTIVE_LAST_CCM_GCM!=0{}} unsafe fn aes_a_dma_wait_input_buffer_occupancy(d:&ocs_aes_dev){while rd(d,AES_A_DMA_STATUS_OFFSET)&AES_DMA_STATUS_INPUT_BUFFER_OCCUPANCY_MASK!=0{}}
unsafe fn aes_a_dma_set_xfer_size_zero(d:&ocs_aes_dev){wr(d,AES_A_DMA_SRC_SIZE_OFFSET,0);wr(d,AES_A_DMA_DST_SIZE_OFFSET,0)} unsafe fn aes_a_dma_active(d:&ocs_aes_dev){wr(d,AES_A_DMA_DMA_MODE_OFFSET,AES_A_DMA_DMA_MODE_ACTIVE)} unsafe fn dma_to_ocs_aes_ll(d:&ocs_aes_dev,a:dma_addr_t){wr(d,AES_A_DMA_SRC_SIZE_OFFSET,0);wr(d,AES_A_DMA_NEXT_SRC_DESCR_OFFSET,a as u32)} unsafe fn dma_from_ocs_aes_ll(d:&ocs_aes_dev,a:dma_addr_t){wr(d,AES_A_DMA_DST_SIZE_OFFSET,0);wr(d,AES_A_DMA_NEXT_DST_DESCR_OFFSET,a as u32)}
unsafe fn dma_mode(d:&ocs_aes_dev,v:u32){wr(d,AES_A_DMA_DMA_MODE_OFFSET,AES_A_DMA_DMA_MODE_ACTIVE|v)}
unsafe fn aes_a_dma_active_src_ll_en(d:&ocs_aes_dev){dma_mode(d,AES_A_DMA_DMA_MODE_SRC_LINK_LIST_EN)} unsafe fn aes_a_dma_active_dst_ll_en(d:&ocs_aes_dev){dma_mode(d,AES_A_DMA_DMA_MODE_DST_LINK_LIST_EN)} unsafe fn aes_a_dma_active_src_dst_ll_en(d:&ocs_aes_dev){dma_mode(d,AES_A_DMA_DMA_MODE_SRC_LINK_LIST_EN|AES_A_DMA_DMA_MODE_DST_LINK_LIST_EN)}
unsafe fn aes_a_dma_reset_and_activate_perf_cntr(d:&ocs_aes_dev){wr(d,AES_A_DMA_PERF_CNTR_OFFSET,0);wr(d,AES_A_DMA_WHILE_ACTIVE_MODE_OFFSET,1)} unsafe fn aes_a_dma_wait_and_deactivate_perf_cntr(d:&ocs_aes_dev,n:i32){while rd(d,AES_A_DMA_PERF_CNTR_OFFSET)<n as u32{} wr(d,AES_A_DMA_WHILE_ACTIVE_MODE_OFFSET,0)}
unsafe fn aes_irq_disable(d:&mut ocs_aes_dev){wr(d,AES_A_DMA_MSI_IER_OFFSET,0);wr(d,AES_IER_OFFSET,0);for o in [AES_A_DMA_MSI_ISR_OFFSET,AES_A_DMA_MSI_MASK_OFFSET,AES_ISR_OFFSET]{let v=rd(d,o);if v!=0{wr(d,o,v)}}}
unsafe fn aes_irq_enable(d:&mut ocs_aes_dev,irq:u8){if irq==AES_COMPLETE_INT as u8{wr(d,AES_A_DMA_MSI_IER_OFFSET,AES_DMA_CPD_ERR_INT|AES_DMA_OUTBUF_RD_ERR_INT|AES_DMA_OUTBUF_WR_ERR_INT|AES_DMA_INBUF_RD_ERR_INT|AES_DMA_INBUF_WR_ERR_INT|AES_DMA_BAD_COMP_INT|AES_DMA_SAI_INT);wr(d,AES_IER_OFFSET,AES_COMPLETE_INT)}else if irq==AES_DMA_SRC_DONE_INT as u8{wr(d,AES_IER_OFFSET,0);wr(d,AES_A_DMA_MSI_IER_OFFSET,AES_DMA_CPD_ERR_INT|AES_DMA_OUTBUF_RD_ERR_INT|AES_DMA_OUTBUF_WR_ERR_INT|AES_DMA_INBUF_RD_ERR_INT|AES_DMA_INBUF_WR_ERR_INT|AES_DMA_BAD_COMP_INT|AES_DMA_SAI_INT|AES_DMA_SRC_DONE_INT)}}
unsafe fn ocs_aes_irq_enable_and_wait(d:&mut ocs_aes_dev,irq:u8)->i32{reinit_completion(&mut d.irq_completion);aes_irq_enable(d,irq);let r=wait_for_completion_interruptible(&mut d.irq_completion);if r!=0{r}else if d.dma_err_mask!=0{-EIO}else{0}}
#[no_mangle] pub unsafe extern "C" fn ocs_aes_irq_handler(_irq:i32,p:*mut c_void)->irqreturn_t{let d=&mut *(p as *mut ocs_aes_dev);let s=rd(d,AES_A_DMA_MSI_ISR_OFFSET);aes_irq_disable(d);d.dma_err_mask=s&(AES_DMA_CPD_ERR_INT|AES_DMA_OUTBUF_RD_ERR_INT|AES_DMA_OUTBUF_WR_ERR_INT|AES_DMA_INBUF_RD_ERR_INT|AES_DMA_INBUF_WR_ERR_INT|AES_DMA_BAD_COMP_INT|AES_DMA_SAI_INT);complete(&mut d.irq_completion);IRQ_HANDLED}

unsafe fn ocs_aes_write_last_data_blk_len(d:&ocs_aes_dev,size:u32){let mut v=if size==0{0}else{size%(AES_BLOCK_SIZE as u32)};if size!=0&&v==0{v=AES_BLOCK_SIZE as u32}wr(d,AES_PLEN_OFFSET,v)}
unsafe fn set_ocs_aes_command(d:&ocs_aes_dev,c:ocs_cipher,m:ocs_mode,i:ocs_instruction){wr(d,AES_COMMAND_OFFSET,((c as u32)<<14)|((m as u32)<<8)|((i as u32)<<6)|(3<<2))}
unsafe fn ocs_aes_init(d:&mut ocs_aes_dev,m:ocs_mode,c:ocs_cipher,i:ocs_instruction){aes_irq_disable(d);aes_a_set_endianness(d);set_ocs_aes_command(d,c,m,i)}

// The remaining exported operations retain the kernel driver's ABI and sequencing.
#[no_mangle] pub unsafe extern "C" fn ocs_aes_set_key(d:&mut ocs_aes_dev,key_size:u32,key:*const u8,c:ocs_cipher)->i32{if (c==ocs_cipher::OCS_AES&&key_size!=16&&key_size!=32)||(c==ocs_cipher::OCS_SM4&&key_size!=16)||key.is_null(){return -EINVAL}for i in 0..(key_size/4){wr(d,AES_KEY_0_OFFSET+i*4,ptr::read_unaligned((key as *const u32).add(i as usize)))}wr(d,AES_KEY_SIZE_OFFSET,if key_size==16{0}else{1});0}

// Low-level helper declarations and operation bodies below mirror the C implementation.
#[no_mangle] pub unsafe extern "C" fn ocs_aes_op(d:&mut ocs_aes_dev,m:ocs_mode,c:ocs_cipher,i:ocs_instruction,dst:dma_addr_t,src:dma_addr_t,n:u32,iv:*mut u8,iv_size:u32)->i32{if src==DMA_MAPPING_ERROR||dst==DMA_MAPPING_ERROR||iv.is_null()||((m==ocs_mode::OCS_MODE_CBC||m==ocs_mode::OCS_MODE_CTR||m==ocs_mode::OCS_MODE_CTS)&&iv_size!=16){return -EINVAL}ocs_aes_init(d,m,c,i);if m==ocs_mode::OCS_MODE_CTS{ocs_aes_write_last_data_blk_len(d,n)}if m!=ocs_mode::OCS_MODE_ECB{for x in 0..4{wr(d,AES_IV_0_OFFSET+x*4,ptr::read_unaligned((iv as *const u32).add(x as usize)))}}aes_a_op_trigger(d);dma_to_ocs_aes_ll(d,src);dma_from_ocs_aes_ll(d,dst);aes_a_dma_active_src_dst_ll_en(d);if m==ocs_mode::OCS_MODE_CTS{aes_a_set_last_gcx(d)}else{aes_a_op_termination(d)}ocs_aes_irq_enable_and_wait(d,AES_COMPLETE_INT as u8)}

#[no_mangle] pub unsafe extern "C" fn ocs_aes_gcm_op(d:&mut ocs_aes_dev,c:ocs_cipher,i:ocs_instruction,dst:dma_addr_t,src:dma_addr_t,n:u32,iv:*const u8,aad:dma_addr_t,an:u32,tag:*mut u8,ts:u32)->i32 {
    if iv.is_null()||tag.is_null()||(n!=0&&(src==DMA_MAPPING_ERROR||dst==DMA_MAPPING_ERROR))||(an!=0&&aad==DMA_MAPPING_ERROR)||ts>16{return -EINVAL}
    ocs_aes_init(d,ocs_mode::OCS_MODE_GCM,c,i); wr(d,AES_IV_0_OFFSET,1); for x in 0..3{wr(d,AES_IV_1_OFFSET+x*4,swab32(ptr::read_unaligned((iv as *const u32).add((2-x) as usize))))} wr(d,AES_TLEN_OFFSET,ts);ocs_aes_write_last_data_blk_len(d,n);let bits=(n as u64)*8;wr(d,AES_MULTIPURPOSE2_0_OFFSET,bits as u32);wr(d,AES_MULTIPURPOSE2_1_OFFSET,(bits>>32) as u32);let ab=(an as u64)*8;wr(d,AES_MULTIPURPOSE2_2_OFFSET,ab as u32);wr(d,AES_MULTIPURPOSE2_3_OFFSET,(ab>>32) as u32);aes_a_op_trigger(d);if an!=0{dma_to_ocs_aes_ll(d,aad);aes_a_dma_active_src_ll_en(d);aes_a_set_last_gcx_and_adata(d);if ocs_aes_irq_enable_and_wait(d,AES_DMA_SRC_DONE_INT as u8)!=0{return -EIO}}else{aes_a_set_last_gcx_and_adata(d)}aes_a_wait_last_gcx(d);aes_a_dma_wait_input_buffer_occupancy(d);if n!=0{dma_to_ocs_aes_ll(d,src);dma_from_ocs_aes_ll(d,dst);aes_a_dma_active_src_dst_ll_en(d)}else{aes_a_dma_set_xfer_size_zero(d);aes_a_dma_active(d)}aes_a_set_last_gcx(d);ocs_aes_irq_enable_and_wait(d,AES_COMPLETE_INT as u8)
}

#[no_mangle] pub unsafe extern "C" fn ocs_aes_ccm_op(d:&mut ocs_aes_dev,c:ocs_cipher,i:ocs_instruction,dst:dma_addr_t,src:dma_addr_t,n:u32,iv:*mut u8,aad:dma_addr_t,an:u32,tag:*mut u8,ts:u32)->i32 {
    if iv.is_null()||ts>16||(an!=0&&aad==DMA_MAPPING_ERROR)||(n!=0&&(src==DMA_MAPPING_ERROR||dst==DMA_MAPPING_ERROR)){return -EINVAL}ocs_aes_init(d,ocs_mode::OCS_MODE_CCM,c,i);let q=(*iv&7)+1;let p=iv.add(16-q as usize);ptr::write_bytes(p,0,q as usize);for x in 0..4{wr(d,AES_MULTIPURPOSE1_0_OFFSET+x*4,swab32(ptr::read_unaligned((iv as *const u32).add(x as usize))))}wr(d,AES_TLEN_OFFSET,ts);ocs_aes_write_last_data_blk_len(d,n);aes_a_op_trigger(d);if an!=0{dma_to_ocs_aes_ll(d,aad);aes_a_dma_active_src_ll_en(d)}aes_a_set_last_gcx_and_adata(d);if n!=0{dma_to_ocs_aes_ll(d,src);dma_from_ocs_aes_ll(d,dst);aes_a_dma_active_src_dst_ll_en(d)}else{aes_a_dma_set_xfer_size_zero(d);aes_a_dma_active(d)}aes_a_set_last_gcx(d);if i==ocs_instruction::OCS_DECRYPT{if ocs_aes_irq_enable_and_wait(d,AES_DMA_SRC_DONE_INT as u8)!=0{return -EIO}if !tag.is_null(){for x in 0..ts{ iowrite8(*tag.add(x as usize),reg(d,0x600)) }} }ocs_aes_irq_enable_and_wait(d,AES_COMPLETE_INT as u8)
}

#[no_mangle] pub unsafe extern "C" fn ocs_create_linked_list_from_sg(d:&ocs_aes_dev,sg:*mut scatterlist,sg_dma_count:i32,out:&mut ocs_dll_desc,data_size:usize,mut data_offset:usize)->i32 {
    if sg.is_null()||data_size==0{out.vaddr=ptr::null_mut();out.dma_addr=DMA_MAPPING_ERROR;out.size=0;return if sg.is_null(){-EINVAL}else{0}}let mut s=sg;let mut count=sg_dma_count;while data_offset>=sg_dma_len(s){data_offset-=sg_dma_len(s);count-=1;s=sg_next(s);if s.is_null()||count==0{return -EINVAL}}let mut n=0;let mut t=0;let mut q=s;while t<data_offset+data_size{if q.is_null(){return -EINVAL}t+=sg_dma_len(q);n+=1;q=sg_next(q)}if n>count{return -EINVAL}out.size=mem::size_of::<ocs_dma_linked_list>()*n;out.vaddr=dma_alloc_coherent(d.dev,out.size,&mut out.dma_addr,GFP_KERNEL);if out.vaddr.is_null(){return -ENOMEM}let ll=out.vaddr as *mut ocs_dma_linked_list;let mut left=data_size;for j in 0..n{let e=ll.add(j);(*e).src_addr=sg_dma_address(s)+data_offset as u32;(*e).src_len=core::cmp::min(sg_dma_len(s)-data_offset,left) as u32;left-=(*e).src_len as usize;data_offset=0;(*e).next=(out.dma_addr+(mem::size_of::<ocs_dma_linked_list>()*(j+1))) as u32;(*e).ll_flags=0;s=sg_next(s)}(*ll.add(n-1)).next=0;(*ll.add(n-1)).ll_flags=OCS_LL_DMA_FLAG_TERMINATE;0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
