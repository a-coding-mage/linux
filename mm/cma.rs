// SPDX-License-Identifier: GPL-2.0-or-later
/* Contiguous Memory Allocator.  Kernel dependencies are supplied externally. */

use core::ffi::c_void;

extern "C" {
    static mut cma_areas: [cma; MAX_CMA_AREAS];
    static mut cma_area_count: u32;
    static mut totalcma_pages: usize;
    static mut pageblock_order: u32;
}

// Types and constants below are provided by the kernel headers in the final build.
#[repr(C)] pub struct cma { pub ranges: [cma_memrange; CMA_MAX_RANGES], pub nranges: i32, pub count: usize, pub available_count: usize, pub order_per_bit: u32, pub flags: usize, pub nid: i32, pub name: *const i8, pub lock: c_void, pub alloc_mutex: c_void }
#[repr(C)] pub struct cma_memrange { pub base_pfn: usize, pub early_pfn: usize, pub count: usize, pub bitmap: *mut usize }
#[repr(C)] pub struct cma_init_memrange { pub base: u64, pub size: u64, pub list: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct page { _private: [u8; 0] }
type phys_addr_t = u64; type gfp_t = usize;

extern "C" {
    fn bitmap_zalloc(n: usize, flags: usize) -> *mut usize; fn bitmap_free(p: *mut usize);
    fn bitmap_clear(p: *mut usize, n: usize, c: usize); fn bitmap_set(p: *mut usize, n: usize, c: usize);
    fn bitmap_find_next_zero_area_off(p: *mut usize, n: usize, s: usize, c: usize, m: usize, o: usize) -> usize;
    fn pfn_range_intersects_zones(nid: i32, pfn: usize, count: usize) -> bool; fn test_bit(n: usize, p: *const usize) -> bool; fn set_bit(n: usize, p: *mut usize);
    fn pfn_valid(p: usize) -> bool; fn pfn_to_page(p: usize) -> *mut page; fn page_to_pfn(p: *const page) -> usize;
    fn init_cma_reserved_pageblock(p: *mut page); fn free_reserved_page(p: *mut page); fn memblock_is_region_reserved(b: u64,s:u64)->bool; fn memblock_reserve(b:u64,s:u64)->i32; fn memblock_phys_free(b:u64,s:u64);
    fn alloc_contig_frozen_range(a:usize,b:usize,f:usize,g:gfp_t)->i32; fn free_contig_frozen_range(p:usize,c:usize); fn page_range_contiguous(p:*mut page,c:usize)->bool;
    fn page_kasan_tag_reset(p:*mut page); fn set_pages_refcounted(p:*mut page,c:usize); fn put_page_testzero(p:*mut page)->bool;
    fn cma_sysfs_account_success_pages(c:*mut cma,n:usize); fn cma_sysfs_account_fail_pages(c:*mut cma,n:usize); fn cma_sysfs_account_release_pages(c:*mut cma,n:usize);
    fn is_power_of_2(x:u64)->bool; fn kmemleak_ignore_phys(x:u64); fn memblock_end_of_DRAM()->u64; fn memblock_alloc_range_nid(s:u64,a:u64,b:u64,l:u64,n:i32,m:bool)->u64;
}

pub unsafe fn cma_get_base(c: *const cma) -> phys_addr_t { (*c).ranges[0].base_pfn << PAGE_SHIFT }
pub unsafe fn cma_get_size(c: *const cma) -> usize { (*c).count << PAGE_SHIFT }
pub unsafe fn cma_get_name(c: *const cma) -> *const i8 { (*c).name }
unsafe fn cma_bitmap_aligned_mask(c:&cma,a:u32)->usize { if a<=c.order_per_bit {0} else {(1usize << (a-c.order_per_bit))-1} }
unsafe fn cma_bitmap_aligned_offset(c:&cma,m:&cma_memrange,a:u32)->usize { (m.base_pfn & ((1usize<<a)-1)) >> c.order_per_bit }
unsafe fn cma_bitmap_pages_to_bits(c:&cma,p:usize)->usize { (p + (1usize<<c.order_per_bit)-1) & !((1usize<<c.order_per_bit)-1) >> c.order_per_bit }
unsafe fn cma_clear_bitmap(c:&mut cma,m:&cma_memrange,p:usize,n:usize) { let b=(p-m.base_pfn)>>c.order_per_bit; bitmap_clear(m.bitmap,b,cma_bitmap_pages_to_bits(c,n)); c.available_count+=n; }

pub unsafe fn cma_validate_zones(c:&mut cma)->bool { if test_bit(CMA_ZONES_VALID,&c.flags) {return true} if test_bit(CMA_ZONES_INVALID,&c.flags){return false} for r in 0..c.nranges as usize { let m=&c.ranges[r]; if pfn_range_intersects_zones(c.nid,m.base_pfn,m.count){set_bit(CMA_ZONES_INVALID,&mut c.flags);return false} } set_bit(CMA_ZONES_VALID,&mut c.flags); true }

unsafe fn cma_new_area(name:*const i8,size:u64,op:u32,out:*mut *mut cma)->i32 { if cma_area_count as usize==MAX_CMA_AREAS{return -ENOSPC} let c=&mut cma_areas[cma_area_count as usize]; cma_area_count+=1; c.name=name; c.count=(size>>PAGE_SHIFT) as usize;c.available_count=c.count;c.order_per_bit=op;*out=c;totalcma_pages+=c.count;0 }
unsafe fn cma_drop_area(c:*mut cma){totalcma_pages-=(*c).count;cma_area_count-=1;}

pub unsafe fn cma_init_reserved_mem(base:phys_addr_t,size:phys_addr_t,op:u32,name:*const i8,out:*mut *mut cma)->i32 { if size==0||!memblock_is_region_reserved(base,size)||pageblock_order==0{return -EINVAL} if (base|size)&(CMA_MIN_ALIGNMENT_BYTES-1)!=0{return -EINVAL} let r=cma_new_area(name,size,op,out);if r!=0{return r} let c=&mut **out;c.ranges[0].base_pfn=(base>>PAGE_SHIFT) as usize;c.ranges[0].early_pfn=c.ranges[0].base_pfn;c.ranges[0].count=c.count;c.nranges=1;c.nid=NUMA_NO_NODE;0 }

unsafe fn __cma_declare_contiguous_nid(basep:*mut u64,size:u64,limit:u64,alignment:u64,op:u32,fixed:bool,name:*const i8,out:*mut *mut cma,nid:i32)->i32 { let mut b=*basep; if size==0|| (alignment!=0&&!is_power_of_2(alignment)){return -EINVAL} let a=alignment.max(CMA_MIN_ALIGNMENT_BYTES); b=(b+a-1)&!(a-1); let s=(size+a-1)&!(a-1); let l=if limit==0{memblock_end_of_DRAM()}else{limit&!(a-1)};if b+s>l{return -EINVAL} if fixed {if memblock_reserve(b,s)<0{return -EBUSY}} else {b=memblock_alloc_range_nid(s,a,b,l,nid,true);if b==0{return -ENOMEM};kmemleak_ignore_phys(b)} let r=cma_init_reserved_mem(b,s,op,name,out);if r!=0{memblock_phys_free(b,s);return r}(*out).as_mut().unwrap().nid=nid;*basep=b;0 }

pub unsafe fn cma_declare_contiguous_nid(base:u64,size:u64,limit:u64,align:u64,op:u32,fixed:bool,name:*const i8,out:*mut *mut cma,nid:i32)->i32 { __cma_declare_contiguous_nid(&mut (base),size,limit,align,op,fixed,name,out,nid) }

unsafe fn find_cma_memrange(c:*mut cma,p:*const page,n:usize)->*mut cma_memrange { if c.is_null()||p.is_null()||n>(*c).count{return core::ptr::null_mut()} let x=page_to_pfn(p);for r in 0..(*c).nranges as usize {let m=&mut (*c).ranges[r];if x>=m.base_pfn&&x+n<=m.base_pfn+m.count{return m}} core::ptr::null_mut() }
unsafe fn __cma_release_frozen(c:*mut cma,m:*mut cma_memrange,p:*const page,n:usize){let x=page_to_pfn(p);free_contig_frozen_range(x,n);cma_clear_bitmap(&mut *c,&*m,x,n);cma_sysfs_account_release_pages(c,n);}
pub unsafe fn cma_release_frozen(c:*mut cma,p:*const page,n:usize)->bool{let m=find_cma_memrange(c,p,n);if m.is_null(){false}else{__cma_release_frozen(c,m,p,n);true}}
pub unsafe fn cma_release(c:*mut cma,p:*const page,n:usize)->bool{let m=find_cma_memrange(c,p,n);if m.is_null(){return false}__cma_release_frozen(c,m,p,n);true}
pub unsafe fn cma_intersects(c:*mut cma,start:usize,end:usize)->bool{for r in 0..(*c).nranges as usize{let m=&(*c).ranges[r];let a=m.base_pfn<<PAGE_SHIFT;let b=(m.base_pfn+m.count)<<PAGE_SHIFT;if end>=a&&start<b{return true}}false}
pub unsafe fn cma_for_each_area(it:extern "C" fn(*mut cma,*mut c_void)->i32,data:*mut c_void)->i32{for i in 0..cma_area_count as usize{let r=it(&mut cma_areas[i],data);if r!=0{return r}}0}
pub unsafe fn cma_reserve_early(c:*mut cma,size:usize)->*mut c_void{if c.is_null()||(*c).count==0||size==0{return core::ptr::null_mut()}for r in 0..(*c).nranges as usize{let m=&mut (*c).ranges[r];let avail=m.count-(m.early_pfn-m.base_pfn);if size>>PAGE_SHIFT<=avail{let p=m.early_pfn;m.early_pfn+=size>>PAGE_SHIFT;(*c).available_count-=size>>PAGE_SHIFT;return (p<<PAGE_SHIFT) as *mut c_void}}core::ptr::null_mut()}

pub unsafe fn cma_alloc_frozen(c:*mut cma,count:usize,_align:u32,_no_warn:bool)->*mut page { if c.is_null()||count==0||(*c).count==0{return core::ptr::null_mut()} for r in 0..(*c).nranges as usize {let m=&mut (*c).ranges[r];let bits=cma_bitmap_pages_to_bits(&*c,count);let max=m.count>>c.order_per_bit;let n=bitmap_find_next_zero_area_off(m.bitmap,max,0,bits,0,0);if n<max {bitmap_set(m.bitmap,n,bits);(*c).available_count-=count;let p=pfn_to_page(m.base_pfn+(n<<c.order_per_bit));if alloc_contig_frozen_range(page_to_pfn(p),page_to_pfn(p)+count,0,0)==0{return p}bitmap_clear(m.bitmap,n,bits);(*c).available_count+=count;}}core::ptr::null_mut() }
pub unsafe fn cma_alloc_frozen_compound(c:*mut cma,order:u32)->*mut page { cma_alloc_frozen(c,1usize<<order,order,true) }
pub unsafe fn cma_alloc(c:*mut cma,count:usize,align:u32,no_warn:bool)->*mut page {let p=cma_alloc_frozen(c,count,align,no_warn);if !p.is_null(){set_pages_refcounted(p,count)}p}

const PAGE_SHIFT:u32=12; const MAX_CMA_AREAS:usize=19; const CMA_MAX_RANGES:usize=8; const CMA_MIN_ALIGNMENT_BYTES:u64=1<<21; const NUMA_NO_NODE:i32=-1; const CMA_ZONES_VALID:usize=0;const CMA_ZONES_INVALID:usize=1;const ENOSPC:i32=28;const EINVAL:i32=22;const ENOMEM:i32=12;const EBUSY:i32=16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
