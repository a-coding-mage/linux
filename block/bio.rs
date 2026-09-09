// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2001 Jens Axboe <axboe@kernel.dk> */

// Kernel headers and symbols referenced below are supplied by the surrounding kernel translation.

const ALLOC_CACHE_THRESHOLD: u32 = 16;
const ALLOC_CACHE_MAX: u32 = 256;
const BIO_INLINE_VECS: u16 = 4;

#[repr(C)]
pub struct bio_alloc_cache { pub free_list: *mut bio, pub free_list_irq: *mut bio, pub nr: u32, pub nr_irq: u32 }
#[repr(C)]
pub struct biovec_slab { pub nr_vecs: i32, pub name: *mut i8, pub slab: *mut kmem_cache }
#[repr(C)]
pub struct bio_slab { pub slab: *mut kmem_cache, pub slab_ref: u32, pub slab_size: u32, pub name: [i8; 12] }

extern "C" {
    static mut fs_bio_set: bio_set;
    static mut bvec_slabs: [biovec_slab; 4];
    static mut bio_slab_lock: mutex;
    static mut bio_slabs: xarray;
}

unsafe fn biovec_slab(nr_vecs: u16) -> *mut biovec_slab {
    match nr_vecs { 5..=16 => &mut bvec_slabs[0], 17..=64 => &mut bvec_slabs[1], 65..=128 => &mut bvec_slabs[2], 129..=BIO_MAX_VECS => &mut bvec_slabs[3], _ => { BUG(); core::ptr::null_mut() } }
}

unsafe fn create_bio_slab(size: u32) -> *mut bio_slab {
    let bslab = kzalloc_obj::<bio_slab>(); if bslab.is_null() { return core::ptr::null_mut(); }
    snprintf((*bslab).name.as_mut_ptr(), 12, c"bio-%d".as_ptr(), size);
    (*bslab).slab = kmem_cache_create((*bslab).name.as_ptr(), size, ARCH_KMALLOC_MINALIGN, SLAB_HWCACHE_ALIGN | SLAB_TYPESAFE_BY_RCU, core::ptr::null_mut());
    if (*bslab).slab.is_null() { kfree(bslab as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    (*bslab).slab_ref = 1; (*bslab).slab_size = size;
    if xa_err(xa_store(&mut bio_slabs, size as usize, bslab, GFP_KERNEL)) == 0 { return bslab; }
    kmem_cache_destroy((*bslab).slab); kfree(bslab as *mut core::ffi::c_void); core::ptr::null_mut()
}

unsafe fn bs_bio_slab_size(bs: *mut bio_set) -> u32 { (*bs).front_pad + core::mem::size_of::<bio>() as u32 + (*bs).back_pad }
unsafe fn bio_slab_addr(bio: *mut bio) -> *mut core::ffi::c_void { (bio as *mut u8).sub((*(*bio).bi_pool).front_pad as usize) as *mut _ }

pub unsafe extern "C" fn bio_uninit(bio: *mut bio) {
    #[cfg(feature = "CONFIG_BLK_CGROUP")] { if !(*bio).bi_blkg.is_null() { blkg_put((*bio).bi_blkg); (*bio).bi_blkg = core::ptr::null_mut(); } }
    if bio_integrity(bio) { bio_integrity_free(bio); } bio_crypt_free_ctx(bio);
}

pub unsafe extern "C" fn bio_init(bio: *mut bio, bdev: *mut block_device, table: *mut bio_vec, max_vecs: u16, opf: blk_opf_t) {
    core::ptr::write_bytes(bio, 0, 1); (*bio).bi_bdev=bdev; (*bio).bi_opf=opf; (*bio).bi_io_vec=table; (*bio).bi_max_vecs=max_vecs; (*bio).__bi_remaining=1; (*bio).__bi_cnt=1; (*bio).bi_cookie=BLK_QC_T_NONE;
}

pub unsafe extern "C" fn bio_reset(bio: *mut bio, bdev: *mut block_device, opf: blk_opf_t) { let bv=(*bio).bi_io_vec; bio_uninit(bio); core::ptr::write_bytes(bio,0,BIO_RESET_BYTES as usize); (*bio).__bi_remaining=1; (*bio).bi_io_vec=bv; (*bio).bi_bdev=bdev; if !bdev.is_null(){bio_associate_blkg(bio);} (*bio).bi_opf=opf; }

pub unsafe extern "C" fn bio_reuse(bio: *mut bio, opf: blk_opf_t) { let n=(*bio).bi_vcnt; let end=(*bio).bi_end_io; let private=(*bio).bi_private; bio_reset(bio,(*bio).bi_bdev,opf); for i in 0..n { (*bio).bi_iter.bi_size += (*(*bio).bi_io_vec.add(i as usize)).bv_len; } (*bio).bi_vcnt=n; (*bio).bi_private=private; (*bio).bi_end_io=end; }

unsafe fn __bio_chain_endio(bio: *mut bio) -> *mut bio { let parent=(*bio).bi_private as *mut bio; if (*bio).bi_status!=0 && (*parent).bi_status==0 {(*parent).bi_status=(*bio).bi_status;} bio_put(bio); parent }
unsafe extern "C" fn bio_chain_endio(_: *mut bio) { BUG(); }
pub unsafe extern "C" fn bio_chain(bio:*mut bio,parent:*mut bio){ BUG_ON(!(*bio).bi_private.is_null()||!(*bio).bi_end_io.is_none()); (*bio).bi_private=parent as *mut _; (*bio).bi_end_io=Some(bio_chain_endio); bio_inc_remaining(parent); }
pub unsafe extern "C" fn bio_chain_and_submit(prev:*mut bio,new:*mut bio)->*mut bio{if !prev.is_null(){bio_chain(prev,new);submit_bio(prev);}new}
pub unsafe extern "C" fn blk_next_bio(bio:*mut bio,bdev:*mut block_device,nr_pages:u32,opf:blk_opf_t,gfp:gfp_t)->*mut bio{bio_chain_and_submit(bio,bio_alloc(bdev,nr_pages,opf,gfp))}

pub unsafe extern "C" fn __bio_add_page(bio:*mut bio,page:*mut page,len:u32,off:u32){ bvec_set_page((*bio).bi_io_vec.add((*bio).bi_vcnt as usize),page,len,off);(*bio).bi_iter.bi_size+=len;(*bio).bi_vcnt+=1; }
pub unsafe extern "C" fn bio_add_page(bio:*mut bio,page:*mut page,len:u32,offset:u32)->i32{if bio_flagged(bio,BIO_CLONED)||len==0||(*bio).bi_iter.bi_size>BIO_MAX_SIZE-len{return 0;} if (*bio).bi_vcnt>=(*bio).bi_max_vecs{return 0;} __bio_add_page(bio,page,len,offset);len as i32}
pub unsafe extern "C" fn bio_add_virt_nofail(bio:*mut bio,vaddr:*mut core::ffi::c_void,len:u32){__bio_add_page(bio,virt_to_page(vaddr),len,offset_in_page(vaddr));}
pub unsafe extern "C" fn bio_add_folio(bio:*mut bio,folio:*mut folio,len:usize,off:usize)->bool{if len>BIO_MAX_SIZE as usize{return false;}bio_add_page(bio,folio_page(folio,(off/PAGE_SIZE) as u64),len as u32,(off%PAGE_SIZE) as u32)>0}

pub unsafe extern "C" fn __bio_advance(bio:*mut bio,bytes:u32){if bio_integrity(bio){bio_integrity_advance(bio,bytes);}bio_crypt_advance(bio,bytes);bio_advance_iter(bio,&mut (*bio).bi_iter,bytes);}
pub unsafe extern "C" fn bio_free_pages(bio:*mut bio){let mut bv: *mut bio_vec=core::ptr::null_mut();let mut i=0;bio_for_each_segment_all(bv,bio,&mut i){__free_page((*bv).bv_page);}}

// Remaining helpers retain the kernel ABI and are declared for translation linkage.
extern "C" { fn bio_alloc_bioset(bdev:*mut block_device,nr_vecs:u16,opf:blk_opf_t,gfp:gfp_t,bs:*mut bio_set)->*mut bio; fn bio_put(bio:*mut bio); fn submit_bio(bio:*mut bio); fn bio_alloc(bdev:*mut block_device,nr:u32,opf:blk_opf_t,gfp:gfp_t)->*mut bio; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
