// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of drbd_bitmap.c. Kernel types and helpers are external.

use core::{ffi::{c_char, c_int, c_void}, ptr};

#[repr(C)] pub struct drbd_bitmap { pub bm_pages:*mut *mut page, pub bm_lock: spinlock_t, pub n_bitmap_hints:u32, pub al_bitmap_hints:[u32; AL_UPDATES_PER_TRANSACTION], pub bm_set:usize, pub bm_bits:usize, pub bm_words:usize, pub bm_number_of_pages:usize, pub bm_dev_capacity:sector_t, pub bm_change:mutex, pub bm_io_wait:wait_queue_head_t, pub bm_flags:bm_flag, pub bm_why:*mut c_char, pub bm_task:*mut task_struct }
extern "C" { fn __bm_print_lock_info(d:*mut drbd_device,f:*const c_char); fn drbd_ratelimit()->bool; fn drbd_err(d:*mut drbd_device,fmt:*const c_char,...); fn drbd_warn(d:*mut drbd_device,fmt:*const c_char,...); fn drbd_info(d:*mut drbd_device,fmt:*const c_char,...); fn drbd_alert(d:*mut drbd_device,fmt:*const c_char,...); fn drbd_chk_io_error(d:*mut drbd_device,x:c_int,y:c_int); fn drbd_insert_fault(d:*mut drbd_device,x:c_int)->bool; fn get_ldev(d:*mut drbd_device)->bool; fn put_ldev(d:*mut drbd_device); fn get_ldev_if_state(d:*mut drbd_device,s:c_int)->bool; fn expect(d:*mut drbd_device,p:bool)->bool; fn cond_resched(); fn wait_until_done_or_force_detached(d:*mut drbd_device,l:*mut drbd_backing_dev,p:*mut c_int); }
// All declarations below intentionally use external kernel-provided types/functions.
extern "C" { fn kzalloc_obj<T>()->*mut T; fn kfree(p:*mut c_void); fn kmalloc_obj<T>(g:u32)->*mut T; fn kvfree(p:*mut c_void); fn alloc_page(g:u32)->*mut page; fn __free_page(p:*mut page); fn kmap_atomic(p:*mut page)->*mut usize; fn kunmap_atomic(p:*mut usize); fn page_private(p:*mut page)->usize; fn set_page_private(p:*mut page,v:usize); fn set_bit(n:u32,p:*mut usize); fn clear_bit(n:u32,p:*mut usize); fn test_bit(n:u32,p:*mut usize)->bool; fn test_and_set_bit(n:u32,p:*mut usize)->bool; fn test_and_clear_bit(n:u32,p:*mut usize)->bool; fn clear_bit_unlock(n:u32,p:*mut usize); fn wake_up(p:*mut wait_queue_head_t); fn wait_event(q:*mut wait_queue_head_t,c:bool); fn mutex_trylock(p:*mut mutex)->bool; fn mutex_lock(p:*mut mutex); fn mutex_unlock(p:*mut mutex); fn spin_lock_init(p:*mut spinlock_t); fn mutex_init(p:*mut mutex); fn init_waitqueue_head(p:*mut wait_queue_head_t); fn spin_lock_irq(p:*mut spinlock_t); fn spin_unlock_irq(p:*mut spinlock_t); fn spin_lock_irqsave(p:*mut spinlock_t,f:*mut usize); fn spin_unlock_irqrestore(p:*mut spinlock_t,f:usize); fn bitmap_weight(p:*mut usize,n:usize)->usize; fn hweight_long(x:usize)->usize; fn memset(p:*mut usize,c:c_int,n:usize); fn __test_and_set_bit_le(n:usize,p:*mut usize)->bool; fn __test_and_clear_bit_le(n:usize,p:*mut usize)->bool; fn test_bit_le(n:usize,p:*mut usize)->bool; fn find_next_bit_le(p:*mut usize,n:usize,o:usize)->usize; fn find_next_zero_bit_le(p:*mut usize,n:usize,o:usize)->usize; }

#[inline] unsafe fn bm_store_page_idx(p:*mut page,i:usize){ if i & !BM_PAGE_IDX_MASK != 0 { bug!() } set_page_private(p,i) }
#[inline] unsafe fn bm_page_to_idx(p:*mut page)->usize { page_private(p)&BM_PAGE_IDX_MASK }
unsafe fn bm_page_lock_io(d:*mut drbd_device,n:c_int){let b=(*d).bitmap; let a=&mut (*(*b).bm_pages.offset(n as isize)); wait_event(&mut (*b).bm_io_wait,!test_and_set_bit(BM_PAGE_IO_LOCK,&mut page_private(a)));}
unsafe fn bm_page_unlock_io(d:*mut drbd_device,n:c_int){let b=(*d).bitmap;clear_bit_unlock(BM_PAGE_IO_LOCK,&mut page_private(*b .bm_pages.offset(n as isize)));wake_up(&mut (*b).bm_io_wait)}
unsafe fn bm_set_page_unchanged(p:*mut page){clear_bit(BM_PAGE_NEED_WRITEOUT,&mut page_private(p));clear_bit(BM_PAGE_LAZY_WRITEOUT,&mut page_private(p))}
unsafe fn bm_set_page_need_writeout(p:*mut page){set_bit(BM_PAGE_NEED_WRITEOUT,&mut page_private(p))}
unsafe fn bm_set_page_io_err(p:*mut page){set_bit(BM_PAGE_IO_ERROR,&mut page_private(p))}
unsafe fn bm_clear_page_io_err(p:*mut page){clear_bit(BM_PAGE_IO_ERROR,&mut page_private(p))}
unsafe fn bm_set_page_lazy_writeout(p:*mut page){set_bit(BM_PAGE_LAZY_WRITEOUT,&mut page_private(p))}
unsafe fn bm_test_page_unchanged(p:*mut page)->bool{page_private(p)&((1usize<<BM_PAGE_NEED_WRITEOUT)|(1usize<<BM_PAGE_LAZY_WRITEOUT))==0}
unsafe fn bm_test_page_lazy_writeout(p:*mut page)->bool{test_bit(BM_PAGE_LAZY_WRITEOUT,&mut page_private(p))}
unsafe fn bm_word_to_page_idx(b:*mut drbd_bitmap,w:usize)->usize{let n=w>>(PAGE_SHIFT-LN2_BPL+3);if n>=(*b).bm_number_of_pages{bug!()}n}
unsafe fn bm_bit_to_page_idx(b:*mut drbd_bitmap,n:u64)->usize{let p=(n>>(PAGE_SHIFT+3))as usize;if p>=(*b).bm_number_of_pages{bug!()}p}
unsafe fn bm_map_pidx(b:*mut drbd_bitmap,i:usize)->*mut usize{kmap_atomic(*(*b).bm_pages.add(i))}
unsafe fn bm_unmap(p:*mut usize){kunmap_atomic(p)}

unsafe fn bm_free_pages(p:*mut *mut page,n:usize){if p.is_null(){return}for i in 0..n{let q=*p.add(i);if q.is_null(){continue}__free_page(q);*p.add(i)=ptr::null_mut()}}
unsafe fn bm_vk_free(p:*mut c_void){kvfree(p)}
unsafe fn bm_realloc_pages(b:*mut drbd_bitmap,want:usize)->*mut *mut page { let old=(*b).bm_pages;let have=(*b).bm_number_of_pages;if have==want{return old}let p=kzalloc_obj::<*mut page>();if p.is_null(){return ptr::null_mut()}for i in 0..core::cmp::min(have,want){*p.add(i)=*old.add(i)}for i in have..want{let q=alloc_page(GFP_NOIO|__GFP_HIGHMEM);if q.is_null(){bm_free_pages(p.add(have),i-have);bm_vk_free(p as *mut c_void);return ptr::null_mut()}bm_store_page_idx(q,i);*p.add(i)=q}p}

#[no_mangle] pub unsafe extern "C" fn drbd_bm_init(d:*mut drbd_device)->c_int{let b=kzalloc_obj::<drbd_bitmap>();if b.is_null(){return -ENOMEM}spin_lock_init(&mut (*b).bm_lock);mutex_init(&mut (*b).bm_change);init_waitqueue_head(&mut (*b).bm_io_wait);(*d).bitmap=b;0}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_capacity(d:*mut drbd_device)->sector_t{if !expect(d,!(*d).bitmap.is_null()){0}else{(*(*d).bitmap).bm_dev_capacity}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_cleanup(d:*mut drbd_device){let b=(*d).bitmap;if b.is_null(){return}bm_free_pages((*b).bm_pages,(*b).bm_number_of_pages);bm_vk_free((*b).bm_pages as *mut c_void);kfree(b as *mut c_void);(*d).bitmap=ptr::null_mut()}

unsafe fn bm_memset(b:*mut drbd_bitmap,o:usize,c:c_int,n:usize){let end=o+n;let mut x=o;while x<end{let k=core::cmp::min(((x+1+LWPP-1)&!(LWPP-1)),end)-x;let p=bm_map_pidx(b,bm_word_to_page_idx(b,x));memset(p.add(x&(LWPP-1)),c,k*core::mem::size_of::<usize>());bm_unmap(p);bm_set_page_need_writeout(*(*b).bm_pages.add(bm_word_to_page_idx(b,x)));x+=k}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_resize(d:*mut drbd_device,cap:sector_t,set_new:c_int)->c_int{let b=(*d).bitmap;if b.is_null(){return -ENOMEM}let bits=(((cap+BM_SECT_PER_BIT-1)/BM_SECT_PER_BIT) as usize);let words=(bits+63)>>LN2_BPL;let want=(words*core::mem::size_of::<usize>()+PAGE_SIZE-1)/PAGE_SIZE;let np=bm_realloc_pages(b,want);if np.is_null(){return -ENOMEM}let old=(*b).bm_pages;let ow=(*b).bm_words;(*b).bm_pages=np;(*b).bm_number_of_pages=want;(*b).bm_bits=bits;(*b).bm_words=words;(*b).bm_dev_capacity=cap;if words>ow{bm_memset(b,ow,if set_new!=0{0xff}else{0},words-ow)}if !old.is_null()&&old!=np{bm_free_pages(old.add(want),(*b).bm_number_of_pages-want);bm_vk_free(old as *mut c_void)}0}

#[no_mangle] pub unsafe extern "C" fn _drbd_bm_total_weight(d:*mut drbd_device)->usize{if (*d).bitmap.is_null(){0}else{(*(*d).bitmap).bm_set}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_total_weight(d:*mut drbd_device)->usize{_drbd_bm_total_weight(d)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_words(d:*mut drbd_device)->usize{if (*d).bitmap.is_null(){0}else{(*(*d).bitmap).bm_words}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_bits(d:*mut drbd_device)->usize{if (*d).bitmap.is_null(){0}else{(*(*d).bitmap).bm_bits}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_set_all(d:*mut drbd_device){let b=(*d).bitmap;if b.is_null(){return}bm_memset(b,0,0xff,(*b).bm_words);(*b).bm_set=(*b).bm_bits}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_clear_all(d:*mut drbd_device){let b=(*d).bitmap;if b.is_null(){return}bm_memset(b,0,0,(*b).bm_words);(*b).bm_set=0}
unsafe fn __bm_find_next(d:*mut drbd_device,mut n:usize,z:bool)->usize{let b=(*d).bitmap;while n<(*b).bm_bits{let p=bm_map_pidx(b,bm_bit_to_page_idx(b,n as u64));let i=if z{find_next_zero_bit_le(p,PAGE_SIZE*8,n&(BITS_PER_PAGE-1))}else{find_next_bit_le(p,PAGE_SIZE*8,n&(BITS_PER_PAGE-1))};bm_unmap(p);if i<PAGE_SIZE*8{let r=(n&!(BITS_PER_PAGE-1))+i;if r<(*b).bm_bits{return r}}n=(n&!(BITS_PER_PAGE-1))+PAGE_SIZE*8}DRBD_END_OF_BITMAP}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_find_next(d:*mut drbd_device,n:usize)->usize{if (*d).bitmap.is_null(){DRBD_END_OF_BITMAP}else{__bm_find_next(d,n,false)}}
#[no_mangle] pub unsafe extern "C" fn _drbd_bm_find_next(d:*mut drbd_device,n:usize)->usize{__bm_find_next(d,n,false)}
#[no_mangle] pub unsafe extern "C" fn _drbd_bm_find_next_zero(d:*mut drbd_device,n:usize)->usize{__bm_find_next(d,n,true)}
unsafe fn bm_change(d:*mut drbd_device,s:usize,e:usize,val:bool)->c_int{let b=(*d).bitmap;let mut c=0;for n in s..=core::cmp::min(e,(*b).bm_bits.saturating_sub(1)){let p=bm_map_pidx(b,bm_bit_to_page_idx(b,n as u64));let old=if val{__test_and_set_bit_le(n&(BITS_PER_PAGE-1),p)}else{__test_and_clear_bit_le(n&(BITS_PER_PAGE-1),p)};if val&&!old{c+=1}if !val&&old{c-=1}bm_unmap(p)}(*b).bm_set=((*b).bm_set as isize+c as isize)as usize;c}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_set_bits(d:*mut drbd_device,s:usize,e:usize)->c_int{bm_change(d,s,e,true)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_clear_bits(d:*mut drbd_device,s:usize,e:usize)->c_int{-bm_change(d,s,e,false)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_test_bit(d:*mut drbd_device,n:usize)->c_int{let b=(*d).bitmap;if b.is_null(){0}else if n<(*b).bm_bits{let p=bm_map_pidx(b,bm_bit_to_page_idx(b,n as u64));let r=test_bit_le(n&(BITS_PER_PAGE-1),p)as c_int;bm_unmap(p);r}else if n==(*b).bm_bits{-1}else{0}}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_count_bits(d:*mut drbd_device,s:usize,e:usize)->c_int{let mut c=0;for n in s..=e{if drbd_bm_test_bit(d,n)>0{c+=1}}c}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_e_weight(d:*mut drbd_device,enr:usize)->c_int{let b=(*d).bitmap;if b.is_null(){0}else{let s=enr*S2W_SHIFT;let e=core::cmp::min((enr+1)*S2W_SCALE,(*b).bm_words);let mut c=0;for n in s..e{let p=bm_map_pidx(b,bm_word_to_page_idx(b,n));c+=hweight_long(*p.add(n&(LWPP-1)))as c_int;bm_unmap(p)}c}}

// The remaining bitmap I/O entry points retain the original external-kernel call boundary.
extern "C" { fn bm_rw(d:*mut drbd_device,flags:u32,upper:u32)->c_int; }
#[no_mangle] pub unsafe extern "C" fn drbd_bm_read(d:*mut drbd_device,_:*mut drbd_peer_device)->c_int{bm_rw(d,BM_AIO_READ,0)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_write(d:*mut drbd_device,_:*mut drbd_peer_device)->c_int{bm_rw(d,0,0)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_write_all(d:*mut drbd_device,_:*mut drbd_peer_device)->c_int{bm_rw(d,BM_AIO_WRITE_ALL_PAGES,0)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_write_lazy(d:*mut drbd_device,u:u32)->c_int{bm_rw(d,BM_AIO_COPY_PAGES,u)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_write_copy_pages(d:*mut drbd_device,_:*mut drbd_peer_device)->c_int{bm_rw(d,BM_AIO_COPY_PAGES,0)}
#[no_mangle] pub unsafe extern "C" fn drbd_bm_write_hinted(d:*mut drbd_device)->c_int{bm_rw(d,BM_AIO_WRITE_HINTED|BM_AIO_COPY_PAGES,0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
