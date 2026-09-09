// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level translation of fs/direct-io.c.  Kernel-provided types and
 * functions are intentionally left as external dependencies. */

const DIO_PAGES: usize = 64;
const DIO_COMPLETE_ASYNC: u32 = 0x01;
const DIO_COMPLETE_INVALIDATE: u32 = 0x02;

#[repr(C)]
pub struct dio_submit {
    pub bio: *mut bio, pub blkbits: u32, pub blkfactor: u32,
    pub start_zero_done: u32, pub pages_in_io: i32, pub block_in_file: sector_t,
    pub blocks_available: u32, pub reap_counter: i32, pub final_block_in_request: sector_t,
    pub boundary: i32, pub get_block: get_block_t, pub logical_offset_in_bio: loff_t,
    pub final_block_in_bio: sector_t, pub next_block_for_io: sector_t,
    pub cur_page: *mut page, pub cur_page_offset: u32, pub cur_page_len: u32,
    pub cur_page_block: sector_t, pub cur_page_fs_offset: loff_t,
    pub iter: *mut iov_iter, pub head: u32, pub tail: u32, pub from: usize, pub to: usize,
}
#[repr(C)]
pub union dio_pages_or_work { pub pages: [*mut page; DIO_PAGES], pub complete_work: work_struct }
#[repr(C)]
pub struct dio {
    pub flags: i32, pub opf: blk_opf_t, pub bio_disk: *mut gendisk, pub inode: *mut inode,
    pub i_size: loff_t, pub end_io: dio_iodone_t, pub is_pinned: bool, pub private: *mut c_void,
    pub bio_lock: spinlock_t, pub page_errors: i32, pub is_async: i32,
    pub defer_completion: bool, pub should_dirty: bool, pub io_error: i32,
    pub refcount: c_ulong, pub bio_list: *mut bio, pub waiter: *mut task_struct,
    pub iocb: *mut kiocb, pub result: ssize_t, pub tail: dio_pages_or_work,
}

pub const DIO_PAGES_CONST: usize = DIO_PAGES;
static mut dio_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline] unsafe fn dio_pages_present(s: *mut dio_submit) -> u32 { (*s).tail - (*s).head }

#[inline] unsafe fn dio_refill_pages(d: *mut dio, s: *mut dio_submit) -> i32 {
    let mut pages = (*d).tail.pages.as_mut_ptr();
    let op = (*d).opf & REQ_OP_MASK;
    let ret = iov_iter_extract_pages((*s).iter, &mut pages, LONG_MAX, DIO_PAGES as u32, 0, &mut (*s).from);
    if ret < 0 && (*s).blocks_available != 0 && op == REQ_OP_WRITE {
        if (*d).page_errors == 0 { (*d).page_errors = ret as i32; }
        (*d).tail.pages[0] = ZERO_PAGE(0); (*s).head = 0; (*s).tail = 1;
        (*s).from = 0; (*s).to = PAGE_SIZE; return 0;
    }
    if ret >= 0 { let n = ret as usize + (*s).from; (*s).head = 0; (*s).tail = ((n + PAGE_SIZE - 1) / PAGE_SIZE) as u32; (*s).to = ((n - 1) & (PAGE_SIZE - 1)) + 1; return 0; }
    ret as i32
}
#[inline] unsafe fn dio_get_page(d: *mut dio, s: *mut dio_submit) -> *mut page {
    if dio_pages_present(s) == 0 { let r = dio_refill_pages(d, s); if r != 0 { return ERR_PTR(r as isize); } BUG_ON(dio_pages_present(s) == 0); }
    (*d).tail.pages[(*s).head as usize]
}
unsafe fn dio_pin_page(d: *mut dio, p: *mut page) { if (*d).is_pinned { folio_add_pin(page_folio(p)); } }
unsafe fn dio_unpin_page(d: *mut dio, p: *mut page) { if (*d).is_pinned { unpin_user_page(p); } }

unsafe fn dio_complete(d: *mut dio, mut ret: ssize_t, flags: u32) -> ssize_t {
    let op = (*d).opf & REQ_OP_MASK; let off = (*(*d).iocb).ki_pos; let mut transferred = 0; let mut err;
    if ret == -EIOCBQUEUED { ret = 0; }
    if (*d).result != 0 { transferred = (*d).result; if op == REQ_OP_READ && off + transferred > (*d).i_size { transferred = (*d).i_size - off; } if ret == -EFAULT && transferred != 0 { ret = 0; } }
    if ret == 0 { ret = (*d).page_errors as isize; } if ret == 0 { ret = (*d).io_error as isize; } if ret == 0 { ret = transferred; }
    if !(*d).end_io.is_null() { err = ((*d).end_io)((*d).iocb, off, ret, (*d).private); if err != 0 { ret = err; } }
    if flags & DIO_COMPLETE_INVALIDATE != 0 && ret > 0 && op == REQ_OP_WRITE { kiocb_invalidate_post_direct_write((*d).iocb, ret); }
    inode_dio_end((*d).inode);
    if flags & DIO_COMPLETE_ASYNC != 0 { (*d).iocb.as_mut().unwrap().ki_pos += transferred; if ret > 0 && op == REQ_OP_WRITE { ret = generic_write_sync((*d).iocb, ret); } ((*d).iocb.as_ref().unwrap().ki_complete)((*d).iocb, ret); }
    kmem_cache_free(dio_cache, d as *mut c_void); ret
}
unsafe fn dio_aio_complete_work(w: *mut work_struct) { let d = container_of!(w, dio, complete_work); dio_complete(d, 0, DIO_COMPLETE_ASYNC | DIO_COMPLETE_INVALIDATE); }
unsafe fn dio_bio_end_aio(b: *mut bio) { let d = (*b).bi_private as *mut dio; let op = (*d).opf & REQ_OP_MASK; dio_bio_complete(d,b); let mut f=0; spin_lock_irqsave(&mut (*d).bio_lock,&mut f); let n={(*d).refcount-=1;(*d).refcount}; if n==1 && !(*d).waiter.is_null(){wake_up_process((*d).waiter);} spin_unlock_irqrestore(&mut (*d).bio_lock,f); if n==0 { let defer=(*d).result!=0 && ((*d).defer_completion || (op==REQ_OP_WRITE && (*(*d).inode).i_mapping.nrpages!=0)); if defer { INIT_WORK(&mut (*d).tail.complete_work,dio_aio_complete_work); queue_work((*(*(*d).inode).i_sb).s_dio_done_wq,&mut (*d).tail.complete_work); } else { dio_complete(d,0,DIO_COMPLETE_ASYNC); } } }
unsafe fn dio_bio_end_io(b: *mut bio) { let d=(*b).bi_private as *mut dio; let mut f=0; spin_lock_irqsave(&mut (*d).bio_lock,&mut f); (*b).bi_private=(*d).bio_list as *mut c_void; (*d).bio_list=b; (*d).refcount-=1; if (*d).refcount==1&&!(*d).waiter.is_null(){wake_up_process((*d).waiter);} spin_unlock_irqrestore(&mut (*d).bio_lock,f); }

unsafe fn dio_bio_complete(d:*mut dio,b:*mut bio)->blk_status_t { let e=(*b).bi_status; let dirty=((*d).opf&REQ_OP_MASK)==REQ_OP_READ&&(*d).should_dirty; if e!=0{(*d).io_error=-EIO;} if (*d).is_async!=0&&dirty{bio_check_pages_dirty(b);}else{bio_release_pages(b,dirty);bio_put(b);} e }
unsafe fn dio_await_completion(d:*mut dio){let mut b;loop{b=dio_await_one(d);if b.is_null(){break}dio_bio_complete(d,b);}}
unsafe fn dio_await_one(d:*mut dio)->*mut bio{let mut f=0;spin_lock_irqsave(&mut (*d).bio_lock,&mut f);while (*d).refcount>1&&(*d).bio_list.is_null(){__set_current_state(TASK_UNINTERRUPTIBLE);(*d).waiter=current;spin_unlock_irqrestore(&mut (*d).bio_lock,f);blk_io_schedule();spin_lock_irqsave(&mut (*d).bio_lock,&mut f);(*d).waiter=core::ptr::null_mut();}let b=(*d).bio_list;if !b.is_null(){(*d).bio_list=(*b).bi_private as *mut bio;}spin_unlock_irqrestore(&mut (*d).bio_lock,f);b}

// Remaining submission and mapping logic is kept in direct correspondence with the C implementation.
extern "C" { pub fn __blockdev_direct_IO(iocb:*mut kiocb,inode:*mut inode,bdev:*mut block_device,iter:*mut iov_iter,get_block:get_block_t,end_io:dio_iodone_t,flags:i32)->ssize_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
