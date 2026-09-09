/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/highmem.h. External types, constants, and functions
 * are supplied by the surrounding kernel translation. */

use core::ffi::c_void;

extern "C" {
    fn kmap(page: *mut page) -> *mut c_void;
    fn kunmap(page: *const page);
    fn kmap_to_page(addr: *mut c_void) -> *mut page;
    fn kmap_flush_unused();
    fn kmap_local_page(page: *const page) -> *mut c_void;
    fn kmap_local_folio(folio: *const folio, offset: usize) -> *mut c_void;
    fn kmap_atomic(page: *const page) -> *mut c_void;
    fn nr_free_highpages() -> c_ulong;
    fn totalhigh_pages() -> c_ulong;
}

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { pub page: page }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
type c_ulong = usize;

extern "C" {
    fn kunmap_local(addr: *mut c_void);
    fn clear_page(addr: *mut c_void);
    fn clear_pages(addr: *mut c_void, npages: u32);
    fn page_address(page: *mut page) -> *mut c_void;
    fn vma_alloc_folio(gfp: usize, order: u32, vma: *mut vm_area_struct, vaddr: c_ulong) -> *mut folio;
    fn user_alloc_needs_zeroing() -> bool;
    fn kasan_reset_tag(addr: *mut c_void) -> *mut c_void;
    fn page_size(page: *mut page) -> usize;
    fn compound_nr(page: *mut page) -> usize;
    fn flush_dcache_page(page: *mut page);
    fn flush_dcache_folio(folio: *mut folio);
    fn copy_user_page(to: *mut c_void, from: *mut c_void, vaddr: c_ulong, page: *mut page);
    fn copy_page(to: *mut c_void, from: *mut c_void);
    fn kmsan_unpoison_memory(addr: *mut c_void, len: usize);
    fn kmsan_copy_page_meta(to: *mut page, from: *mut page);
    fn memory_failure_queue(pfn: c_ulong, flags: u32);
    fn page_to_pfn(page: *mut page) -> c_ulong;
    fn folio_size(folio: *mut folio) -> usize;
    fn folio_test_highmem(folio: *mut folio) -> bool;
    fn folio_test_partial_kmap(folio: *mut folio) -> bool;
    fn offset_in_page(offset: usize) -> usize;
    fn offset_in_folio(folio: *mut folio, pos: i64) -> usize;
    fn folio_put(folio: *mut folio);
}

#[inline] pub unsafe fn flush_anon_page(_vma: *mut vm_area_struct, _page: *mut page, _vmaddr: c_ulong) {}
#[inline] pub unsafe fn flush_kernel_vmap_range(_vaddr: *mut c_void, _size: i32) {}
#[inline] pub unsafe fn invalidate_kernel_vmap_range(_vaddr: *mut c_void, _size: i32) {}

#[inline]
pub unsafe fn clear_user_page(addr: *mut c_void, _vaddr: c_ulong, _page: *mut page) { clear_page(addr); }

#[inline]
pub unsafe fn clear_user_pages(mut addr: *mut u8, mut vaddr: c_ulong, mut page: *mut page, mut npages: u32) {
    while npages != 0 {
        clear_user_page(addr as *mut c_void, vaddr, page);
        addr = addr.add(PAGE_SIZE);
        vaddr = vaddr.wrapping_add(PAGE_SIZE);
        page = page.add(1);
        npages -= 1;
    }
}

#[inline]
pub unsafe fn clear_user_highpage(page: *mut page, vaddr: c_ulong) {
    let addr = kmap_local_page(page);
    clear_user_page(addr, vaddr, page);
    kunmap_local(addr);
}

#[inline]
pub unsafe fn clear_user_highpages(mut page: *mut page, mut vaddr: c_ulong, mut npages: u32) {
    while npages != 0 { clear_user_highpage(page, vaddr); vaddr = vaddr.wrapping_add(PAGE_SIZE); page = page.add(1); npages -= 1; }
}

#[inline]
pub unsafe fn vma_alloc_zeroed_movable_folio(vma: *mut vm_area_struct, vaddr: c_ulong) -> *mut folio {
    let f = vma_alloc_folio(GFP_HIGHUSER_MOVABLE, 0, vma, vaddr);
    if !f.is_null() && user_alloc_needs_zeroing() { clear_user_highpage(&mut (*f).page, vaddr); }
    f
}

#[inline] pub unsafe fn clear_highpage(page: *mut page) { let a=kmap_local_page(page); clear_page(a); kunmap_local(a); }
#[inline] pub unsafe fn clear_highpage_kasan_tagged(page: *mut page) { let a=kmap_local_page(page); clear_page(kasan_reset_tag(a)); kunmap_local(a); }
#[inline] pub unsafe fn tag_clear_highpages(_page: *mut page, _numpages: i32, clear_pages: bool) -> bool { clear_pages }

extern "C" { fn zero_user_segments(page: *mut page, start1: u32, end1: u32, start2: u32, end2: u32); }
#[inline] pub unsafe fn zero_user_segment(page: *mut page, start: u32, end: u32) { zero_user_segments(page,start,end,0,0); }

#[inline]
pub unsafe fn copy_user_highpage(to: *mut page, from: *mut page, vaddr: c_ulong, _vma: *mut vm_area_struct) {
    let vfrom=kmap_local_page(from); let vto=kmap_local_page(to); copy_user_page(vto,vfrom,vaddr,to);
    kmsan_unpoison_memory(page_address(to), PAGE_SIZE); kunmap_local(vto); kunmap_local(vfrom);
}
#[inline]
pub unsafe fn copy_highpage(to: *mut page, from: *mut page) {
    let vfrom=kmap_local_page(from); let vto=kmap_local_page(to); copy_page(vto,vfrom); kmsan_copy_page_meta(to,from); kunmap_local(vto); kunmap_local(vfrom);
}
#[inline] pub unsafe fn copy_mc_user_highpage(to:*mut page,from:*mut page,vaddr:c_ulong,vma:*mut vm_area_struct)->i32 { copy_user_highpage(to,from,vaddr,vma); 0 }
#[inline] pub unsafe fn copy_mc_highpage(to:*mut page,from:*mut page)->i32 { copy_highpage(to,from); 0 }

#[inline]
pub unsafe fn memcpy_page(dst_page:*mut page,dst_off:usize,src_page:*mut page,src_off:usize,len:usize) { let d=kmap_local_page(dst_page) as *mut u8; let s=kmap_local_page(src_page) as *const u8; core::ptr::copy_nonoverlapping(s.add(src_off),d.add(dst_off),len); kunmap_local(s as *mut c_void); kunmap_local(d as *mut c_void); }
#[inline]
pub unsafe fn memset_page(page:*mut page,offset:usize,val:i32,len:usize) { let a=kmap_local_page(page) as *mut u8; core::ptr::write_bytes(a.add(offset),val as u8,len); kunmap_local(a as *mut c_void); }
#[inline]
pub unsafe fn memcpy_from_page(to:*mut u8,page:*mut page,offset:usize,len:usize) { let f=kmap_local_page(page) as *const u8; core::ptr::copy_nonoverlapping(f.add(offset),to,len); kunmap_local(f as *mut c_void); }
#[inline]
pub unsafe fn memcpy_to_page(page:*mut page,offset:usize,from:*const u8,len:usize) { let t=kmap_local_page(page) as *mut u8; core::ptr::copy_nonoverlapping(from,t.add(offset),len); flush_dcache_page(page); kunmap_local(t as *mut c_void); }
#[inline]
pub unsafe fn memzero_page(page:*mut page,offset:usize,len:usize) { let a=kmap_local_page(page) as *mut u8; core::ptr::write_bytes(a.add(offset),0,len); flush_dcache_page(page); kunmap_local(a as *mut c_void); }

#[inline] pub unsafe fn folio_zero_segments(f:*mut folio,a:u32,b:u32,c:u32,d:u32){zero_user_segments(&mut (*f).page,a,b,c,d)}
#[inline] pub unsafe fn folio_zero_segment(f:*mut folio,a:u32,b:u32){zero_user_segments(&mut (*f).page,a,b,0,0)}
#[inline] pub unsafe fn folio_zero_range(f:*mut folio,a:usize,l:usize){zero_user_segments(&mut (*f).page,a as u32,(a+l) as u32,0,0)}
#[inline] pub unsafe fn folio_release_kmap(f:*mut folio,a:*mut c_void){kunmap_local(a);folio_put(f)}

#[inline]
pub unsafe fn memcpy_folio(df:*mut folio,mut doff:usize,sf:*mut folio,mut soff:usize,mut len:usize){while len>0{let d=kmap_local_folio(df,doff);let s=kmap_local_folio(sf,soff);let n=len.min(PAGE_SIZE-offset_in_page(doff)).min(PAGE_SIZE-offset_in_page(soff));core::ptr::copy_nonoverlapping(s as *const u8,d as *mut u8,n);kunmap_local(s);kunmap_local(d);doff+=n;soff+=n;len-=n;}}
#[inline]
pub unsafe fn memcpy_from_folio(to:*mut u8,f:*mut folio,mut off:usize,mut len:usize){while len>0{let a=kmap_local_folio(f,off);let n=len.min(PAGE_SIZE-offset_in_page(off));core::ptr::copy_nonoverlapping(a as *const u8,to,n);kunmap_local(a);to.add(n);off+=n;len-=n;}}
#[inline]
pub unsafe fn memcpy_to_folio(f:*mut folio,mut off:usize,mut from:*const u8,mut len:usize){while len>0{let a=kmap_local_folio(f,off);let n=len.min(PAGE_SIZE-offset_in_page(off));core::ptr::copy_nonoverlapping(from,a as *mut u8,n);kunmap_local(a);from=from.add(n);off+=n;len-=n;}flush_dcache_folio(f)}
#[inline]
pub unsafe fn folio_zero_tail(f:*mut folio,mut off:usize,mut a:*mut c_void)->*mut c_void{let mut len=folio_size(f)-off;while len>PAGE_SIZE-offset_in_page(off){let n=PAGE_SIZE-offset_in_page(off);core::ptr::write_bytes(a,0,n);kunmap_local(a);len-=n;off+=n;a=kmap_local_folio(f,off);}core::ptr::write_bytes(a,0,len);flush_dcache_folio(f);a}
#[inline]
pub unsafe fn folio_fill_tail(f:*mut folio,off:usize,from:*const u8,len:usize){memcpy_to_folio(f,off,from,len);folio_zero_tail(f,off+len,kmap_local_folio(f,off+len));}
#[inline]
pub unsafe fn memcpy_from_file_folio(to:*mut u8,f:*mut folio,pos:i64,mut len:usize)->usize{let off=offset_in_folio(f,pos);len=len.min(folio_size(f)-off);memcpy_from_folio(to,f,off,len);len}

pub const PAGE_SIZE: usize = 4096;
pub const GFP_HIGHUSER_MOVABLE: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
