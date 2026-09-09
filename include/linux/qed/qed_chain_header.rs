/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Direct Rust translation of qed_chain.h. External kernel types/functions are dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_chain_mode { QED_CHAIN_MODE_NEXT_PTR, QED_CHAIN_MODE_SINGLE, QED_CHAIN_MODE_PBL }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_chain_use_mode { QED_CHAIN_USE_TO_PRODUCE, QED_CHAIN_USE_TO_CONSUME, QED_CHAIN_USE_TO_CONSUME_PRODUCE }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_chain_cnt_type { QED_CHAIN_CNT_TYPE_U16, QED_CHAIN_CNT_TYPE_U32 }

#[repr(C)] pub struct qed_chain_next { pub next_phys: regpair, pub next_virt: *mut core::ffi::c_void }
#[repr(C)] pub struct qed_chain_pbl_u16 { pub prod_page_idx: u16, pub cons_page_idx: u16 }
#[repr(C)] pub struct qed_chain_pbl_u32 { pub prod_page_idx: u32, pub cons_page_idx: u32 }
#[repr(C)] pub struct qed_chain_u16 { pub prod_idx: u16, pub cons_idx: u16 }
#[repr(C)] pub struct qed_chain_u32 { pub prod_idx: u32, pub cons_idx: u32 }
#[repr(C)] pub struct addr_tbl_entry { pub virt_addr: *mut core::ffi::c_void, pub dma_map: dma_addr_t }

#[repr(C)] pub union qed_chain_pbl_c { pub u16_: qed_chain_pbl_u16, pub u32_: qed_chain_pbl_u32 }
#[repr(C)] pub struct qed_chain_pbl { pub pp_addr_tbl: *mut addr_tbl_entry, pub c: qed_chain_pbl_c }
#[repr(C)] pub union qed_chain_u { pub chain16: qed_chain_u16, pub chain32: qed_chain_u32 }
#[repr(C)] pub struct qed_chain_pbl_sp { pub table_virt: *mut __le64, pub table_phys: dma_addr_t, pub table_size: usize }
#[repr(C)] pub struct qed_chain {
    pub p_prod_elem: *mut core::ffi::c_void, pub p_cons_elem: *mut core::ffi::c_void,
    pub pbl: qed_chain_pbl, pub u: qed_chain_u, pub capacity: u32, pub page_cnt: u32,
    pub mode: qed_chain_mode, pub elem_per_page: u16, pub elem_per_page_mask: u16,
    pub elem_size: u16, pub next_page_mask: u16, pub usable_per_page: u16, pub elem_unusable: u8,
    pub cnt_type: qed_chain_cnt_type, pub page_size: u32, pub pbl_sp: qed_chain_pbl_sp,
    pub p_virt_addr: *mut core::ffi::c_void, pub p_phys_addr: dma_addr_t, pub size: u32,
    pub intended_use: qed_chain_use_mode, pub b_external_pbl: bool,
}
#[repr(C)] pub struct qed_chain_init_params {
    pub mode: qed_chain_mode, pub intended_use: qed_chain_use_mode, pub cnt_type: qed_chain_cnt_type,
    pub page_size: u32, pub num_elems: u32, pub elem_size: usize,
    pub ext_pbl_virt: *mut core::ffi::c_void, pub ext_pbl_phys: dma_addr_t,
}

pub const QED_CHAIN_PAGE_SIZE: u32 = SZ_4K;
#[inline] pub const fn elems_per_page(elem_size: usize, page_size: usize) -> usize { page_size / elem_size }
#[inline] pub fn unusable_elems_per_page(elem_size: usize, mode: qed_chain_mode) -> u8 { if matches!(mode, qed_chain_mode::QED_CHAIN_MODE_NEXT_PTR) { (1 + (core::mem::size_of::<qed_chain_next>() - 1) / elem_size) as u8 } else { 0 } }
#[inline] pub fn usable_elems_per_page(elem_size: usize, page_size: usize, mode: qed_chain_mode) -> u32 { (elems_per_page(elem_size, page_size) - unusable_elems_per_page(elem_size, mode) as usize) as u32 }
#[inline] pub fn qed_chain_page_cnt(elem_cnt: u32, elem_size: usize, page_size: usize, mode: qed_chain_mode) -> u32 { (elem_cnt + usable_elems_per_page(elem_size,page_size,mode)-1) / usable_elems_per_page(elem_size,page_size,mode) }
#[inline] pub unsafe fn is_chain_u16(p: *const qed_chain) -> bool { (*p).cnt_type == qed_chain_cnt_type::QED_CHAIN_CNT_TYPE_U16 }
#[inline] pub unsafe fn is_chain_u32(p: *const qed_chain) -> bool { (*p).cnt_type == qed_chain_cnt_type::QED_CHAIN_CNT_TYPE_U32 }

#[inline] pub unsafe fn qed_chain_get_prod_idx(c:*const qed_chain)->u16 { (*c).u.chain16.prod_idx }
#[inline] pub unsafe fn qed_chain_get_cons_idx(c:*const qed_chain)->u16 { (*c).u.chain16.cons_idx }
#[inline] pub unsafe fn qed_chain_get_prod_idx_u32(c:*const qed_chain)->u32 { (*c).u.chain32.prod_idx }
#[inline] pub unsafe fn qed_chain_get_cons_idx_u32(c:*const qed_chain)->u32 { (*c).u.chain32.cons_idx }
#[inline] pub unsafe fn qed_chain_get_elem_used(c:*const qed_chain)->u16 { let mut p=qed_chain_get_prod_idx(c) as u32; let n=qed_chain_get_cons_idx(c) as u32; if p<n {p+=65536}; let mut x=(p-n) as u16; if (*c).mode==qed_chain_mode::QED_CHAIN_MODE_NEXT_PTR {x=x.wrapping_sub((p/(*c).elem_per_page as u32-n/(*c).elem_per_page as u32) as u16)} x }
#[inline] pub unsafe fn qed_chain_get_elem_left(c:*const qed_chain)->u16 { ((*c).capacity as u16).wrapping_sub(qed_chain_get_elem_used(c)) }
#[inline] pub unsafe fn qed_chain_get_elem_used_u32(c:*const qed_chain)->u32 { let mut p=qed_chain_get_prod_idx_u32(c) as u64; let n=qed_chain_get_cons_idx_u32(c) as u64; if p<n {p+=4294967296}; let mut x=(p-n) as u32; if (*c).mode==qed_chain_mode::QED_CHAIN_MODE_NEXT_PTR {x=x.wrapping_sub((p/(*c).elem_per_page as u64-n/(*c).elem_per_page as u64) as u32)} x }
#[inline] pub unsafe fn qed_chain_get_elem_left_u32(c:*const qed_chain)->u32 {(*c).capacity-qed_chain_get_elem_used_u32(c)}
#[inline] pub unsafe fn qed_chain_get_usable_per_page(c:*const qed_chain)->u16 {(*c).usable_per_page}
#[inline] pub unsafe fn qed_chain_get_unusable_per_page(c:*const qed_chain)->u8 {(*c).elem_unusable}
#[inline] pub unsafe fn qed_chain_get_page_cnt(c:*const qed_chain)->u32 {(*c).page_cnt}
#[inline] pub unsafe fn qed_chain_get_pbl_phys(c:*const qed_chain)->dma_addr_t {(*c).pbl_sp.table_phys}

/* The remaining inline operations retain C pointer semantics. */
#[inline] pub unsafe fn qed_chain_get_capacity(c:*mut qed_chain)->u32 {(*c).capacity}
#[inline] pub unsafe fn qed_chain_reset(c:*mut qed_chain) { if is_chain_u16(c) {(*c).u.chain16= qed_chain_u16{prod_idx:0,cons_idx:0}} else {(*c).u.chain32=qed_chain_u32{prod_idx:0,cons_idx:0}}; (*c).p_cons_elem=(*c).p_virt_addr; (*c).p_prod_elem=(*c).p_virt_addr; }
#[inline] pub unsafe fn qed_chain_advance_page(c:*mut qed_chain, n:*mut *mut core::ffi::c_void, i:*mut core::ffi::c_void, p:*mut core::ffi::c_void){if (*c).mode==qed_chain_mode::QED_CHAIN_MODE_SINGLE{*n=(*c).p_virt_addr}else if (*c).mode==qed_chain_mode::QED_CHAIN_MODE_NEXT_PTR{let x=*(*n as *mut qed_chain_next);*n=x.next_virt}else{let x=if is_chain_u16(c){let q=p as *mut u16;*q=q.wrapping_add(1);if *q==(*c).page_cnt as u16{*q=0};*q as usize}else{let q=p as *mut u32;*q=q.wrapping_add(1);if *q==(*c).page_cnt{*q=0};*q as usize};*n=(*(*c).pbl.pp_addr_tbl.add(x)).virt_addr}}
#[inline] pub unsafe fn qed_chain_produce(c:*mut qed_chain)->*mut core::ffi::c_void{let r=(*c).p_prod_elem;(*c).p_prod_elem=(*c).p_prod_elem.cast::<u8>().add((*c).elem_size as usize).cast();if is_chain_u16(c){(*c).u.chain16.prod_idx=(*c).u.chain16.prod_idx.wrapping_add(1)}else{(*c).u.chain32.prod_idx=(*c).u.chain32.prod_idx.wrapping_add(1)};r}
#[inline] pub unsafe fn qed_chain_consume(c:*mut qed_chain)->*mut core::ffi::c_void{let r=(*c).p_cons_elem;(*c).p_cons_elem=(*c).p_cons_elem.cast::<u8>().add((*c).elem_size as usize).cast();if is_chain_u16(c){(*c).u.chain16.cons_idx=(*c).u.chain16.cons_idx.wrapping_add(1)}else{(*c).u.chain32.cons_idx=(*c).u.chain32.cons_idx.wrapping_add(1)};r}
#[inline] pub unsafe fn qed_chain_get_last_elem(c:*mut qed_chain)->*mut core::ffi::c_void{if (*c).p_virt_addr.is_null(){core::ptr::null_mut()}else{(*c).p_virt_addr.cast::<u8>().add((*c).elem_size as usize*((*c).usable_per_page as usize-1)).cast()}}
#[inline] pub unsafe fn qed_chain_set_prod(c:*mut qed_chain,i:u32,e:*mut core::ffi::c_void){if is_chain_u16(c){(*c).u.chain16.prod_idx=i as u16}else{(*c).u.chain32.prod_idx=i};(*c).p_prod_elem=e}
#[inline] pub unsafe fn qed_chain_recycle_consumed(c:*mut qed_chain){if is_chain_u16(c){(*c).u.chain16.prod_idx=(*c).u.chain16.prod_idx.wrapping_add(1)}else{(*c).u.chain32.prod_idx=(*c).u.chain32.prod_idx.wrapping_add(1)}}
#[inline] pub unsafe fn qed_chain_return_produced(c:*mut qed_chain){if is_chain_u16(c){(*c).u.chain16.cons_idx=(*c).u.chain16.cons_idx.wrapping_add(1)}else{(*c).u.chain32.cons_idx=(*c).u.chain32.cons_idx.wrapping_add(1)}}
#[inline] pub unsafe fn qed_chain_pbl_zero_mem(c:*mut qed_chain){if (*c).mode==qed_chain_mode::QED_CHAIN_MODE_PBL{for i in 0..(*c).page_cnt as usize{core::ptr::write_bytes((*(*c).pbl.pp_addr_tbl.add(i)).virt_addr,0,(*c).page_size as usize)}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
