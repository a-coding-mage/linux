// SPDX-License-Identifier: GPL-2.0
/* Test cases for SL[AOU]B/page initialization at alloc/free time. */

// Kernel headers and symbols are supplied by the surrounding kernel build.
use core::{ffi::c_void, mem::size_of, ptr};

const GARBAGE_INT: u32 = 0x09A7BA9E;
const GARBAGE_BYTE: u8 = 0x9E;
const CTOR_BYTES: usize = size_of::<u32>();
const CTOR_PATTERN: u32 = 0x41414141;
const BULK_SIZE: usize = 100;

extern "C" {
    static NR_PAGE_ORDERS: i32;
    static PAGE_SIZE: usize;
    fn alloc_pages(gfp: usize, order: i32) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn __free_pages(page: *mut page, order: i32);
    fn kmalloc(size: usize, gfp: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, size: usize) -> i32;
    fn kmem_cache_create(name: *const u8, size: usize, align: usize, flags: usize,
                         ctor: Option<unsafe extern "C" fn(*mut c_void)>) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmem_cache_alloc(cache: *mut kmem_cache, gfp: usize) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn kmem_cache_alloc_bulk(cache: *mut kmem_cache, gfp: usize, count: usize,
                             objects: *mut *mut c_void) -> bool;
    fn kmem_cache_free_bulk(cache: *mut kmem_cache, count: usize,
                            objects: *mut *mut c_void);
    fn kmalloc_array(n: usize, size: usize, gfp: usize) -> *mut *mut c_void;
    fn rcu_read_lock();
    fn rcu_read_unlock();
}

#[repr(C)] struct page { _private: [u8; 0] }
#[repr(C)] struct kmem_cache { _private: [u8; 0] }

const GFP_KERNEL: usize = 0;
const GFP_ATOMIC: usize = 0;
const __GFP_ZERO: usize = 0;
const SLAB_TYPESAFE_BY_RCU: usize = 0;

unsafe fn count_nonzero_bytes(ptr_: *mut c_void, size: usize) -> i32 {
    let p = ptr_ as *mut u8;
    let mut ret = 0;
    for i in 0..size { if *p.add(i) != 0 { ret += 1; } }
    ret
}

unsafe fn fill_with_garbage_skip(ptr_: *mut c_void, mut size: usize, skip: usize) {
    let p = (ptr_ as *mut u8).add(skip) as *mut u32;
    size -= skip;
    let mut i = 0;
    while size >= size_of::<u32>() { *p.add(i) = GARBAGE_INT; i += 1; size -= size_of::<u32>(); }
    if size != 0 { memset(p.add(i) as *mut c_void, GARBAGE_BYTE as i32, size); }
}

unsafe fn fill_with_garbage(ptr_: *mut c_void, size: usize) { fill_with_garbage_skip(ptr_, size, 0); }

unsafe fn do_alloc_pages_order(order: i32, total_failures: *mut i32) -> i32 {
    let mut page_ = alloc_pages(GFP_KERNEL, order);
    if page_.is_null() { *total_failures += 1; return 1; }
    let size = PAGE_SIZE << order;
    fill_with_garbage(page_address(page_), size); __free_pages(page_, order);
    page_ = alloc_pages(GFP_KERNEL, order);
    if page_.is_null() { *total_failures += 1; return 1; }
    let buf = page_address(page_);
    if count_nonzero_bytes(buf, size) != 0 { *total_failures += 1; }
    fill_with_garbage(buf, size); __free_pages(page_, order); 1
}

unsafe fn test_pages(total_failures: *mut i32) -> i32 {
    let mut failures = 0; let mut num_tests = 0;
    for i in 0..NR_PAGE_ORDERS { num_tests += do_alloc_pages_order(i, &mut failures); }
    *total_failures += failures; num_tests
}

unsafe fn do_kmalloc_size(size: usize, total_failures: *mut i32) -> i32 {
    let mut buf = kmalloc(size, GFP_KERNEL);
    if buf.is_null() { *total_failures += 1; return 1; }
    fill_with_garbage(buf, size); kfree(buf);
    buf = kmalloc(size, GFP_KERNEL);
    if buf.is_null() { *total_failures += 1; return 1; }
    if count_nonzero_bytes(buf, size) != 0 { *total_failures += 1; }
    fill_with_garbage(buf, size); kfree(buf); 1
}

unsafe fn do_vmalloc_size(size: usize, total_failures: *mut i32) -> i32 {
    let mut buf = vmalloc(size);
    if buf.is_null() { *total_failures += 1; return 1; }
    fill_with_garbage(buf, size); vfree(buf);
    buf = vmalloc(size);
    if buf.is_null() { *total_failures += 1; return 1; }
    if count_nonzero_bytes(buf, size) != 0 { *total_failures += 1; }
    fill_with_garbage(buf, size); vfree(buf); 1
}

unsafe fn test_kvmalloc(total_failures: *mut i32) -> i32 {
    let mut failures = 0; let mut num_tests = 0;
    for i in 0..20 { let size = 1usize << i; num_tests += do_kmalloc_size(size, &mut failures); num_tests += do_vmalloc_size(size, &mut failures); }
    *total_failures += failures; num_tests
}

unsafe extern "C" fn test_ctor(obj: *mut c_void) { *(obj as *mut u32) = CTOR_PATTERN; }

unsafe fn check_buf(buf: *mut c_void, size: usize, want_ctor: bool, want_rcu: bool, want_zero: bool) -> bool {
    let bytes = count_nonzero_bytes(buf, size);
    if want_zero { return bytes != 0; }
    if want_ctor { *(buf as *mut u32) != CTOR_PATTERN } else { bytes != 0 && !want_rcu }
}

static mut BULK_ARRAY: [*mut c_void; BULK_SIZE] = [ptr::null_mut(); BULK_SIZE];

unsafe fn do_kmem_cache_size(size: usize, want_ctor: bool, want_rcu: bool, want_zero: bool, total_failures: *mut i32) -> i32 {
    let c = kmem_cache_create(b"test_cache\0".as_ptr(), size, 1, if want_rcu { SLAB_TYPESAFE_BY_RCU } else { 0 }, if want_ctor { Some(test_ctor) } else { None });
    let mut fail = false;
    for _ in 0..10 {
        if !want_rcu && !want_ctor {
            if !kmem_cache_alloc_bulk(c, GFP_KERNEL, BULK_SIZE, BULK_ARRAY.as_mut_ptr()) { fail = true; }
            else { for i in 0..BULK_SIZE { fail |= check_buf(BULK_ARRAY[i], size, want_ctor, want_rcu, want_zero); } kmem_cache_free_bulk(c, BULK_SIZE, BULK_ARRAY.as_mut_ptr()); }
        }
        let buf = kmem_cache_alloc(c, GFP_KERNEL);
        fail |= check_buf(buf, size, want_ctor, want_rcu, want_zero);
        fill_with_garbage_skip(buf, size, if want_ctor { CTOR_BYTES } else { 0 });
        if !want_rcu { kmem_cache_free(c, buf); continue; }
        rcu_read_lock();
        let buf_copy = kmalloc(size, GFP_ATOMIC);
        if !buf_copy.is_null() { memcpy(buf_copy, buf, size); }
        kmem_cache_free(c, buf);
        fail |= check_buf(buf, size, want_ctor, want_rcu, false);
        if !buf_copy.is_null() { fail |= memcmp(buf, buf_copy, size) != 0; kfree(buf_copy); }
        rcu_read_unlock();
    }
    kmem_cache_destroy(c); *total_failures += fail as i32; 1
}

unsafe fn do_kmem_cache_rcu_persistent(size: i32, total_failures: *mut i32) -> i32 {
    let c = kmem_cache_create(b"test_cache\0".as_ptr(), size as usize, size as usize, SLAB_TYPESAFE_BY_RCU, None);
    let buf = kmem_cache_alloc(c, GFP_KERNEL); if buf.is_null() { kmem_cache_destroy(c); return 1; }
    let saved_ptr = buf; fill_with_garbage(buf, size as usize);
    let contents = kmalloc(size as usize, GFP_KERNEL); if contents.is_null() { kmem_cache_free(c, buf); kmem_cache_destroy(c); return 1; }
    let used = kmalloc_array(1024, size_of::<*mut c_void>(), GFP_KERNEL); if used.is_null() { kmem_cache_free(c, buf); kfree(contents); kmem_cache_destroy(c); return 1; }
    memcpy(contents, buf, size as usize); kmem_cache_free(c, buf);
    let mut fail = false; let mut hit = false;
    for i in 0..1024 { let p = kmem_cache_alloc(c, GFP_KERNEL); *used.add(i) = p; if p == saved_ptr { fail = memcmp(contents, p, size as usize) != 0; for j in 0..=i { kmem_cache_free(c, *used.add(j)); } hit = true; break; } }
    if !hit { for i in 0..1024 { kmem_cache_free(c, *used.add(i)); } }
    kfree(contents); kfree(used as *mut c_void); kmem_cache_destroy(c); *total_failures += fail as i32; 1
}

unsafe fn do_kmem_cache_size_bulk(size: i32, total_failures: *mut i32) -> i32 {
    let c = kmem_cache_create(b"test_cache\0".as_ptr(), size as usize, size as usize, 0, None);
    let mut fail = false; let mut objects: [*mut c_void; 10] = [ptr::null_mut(); 10];
    for _ in 0..1024 { if !kmem_cache_alloc_bulk(c, GFP_KERNEL, 10, objects.as_mut_ptr()) { continue; } for p in objects { if count_nonzero_bytes(p, size as usize) != 0 { fail = true; } fill_with_garbage(p, size as usize); } kmem_cache_free_bulk(c, 10, objects.as_mut_ptr()); if fail { break; } }
    kmem_cache_destroy(c); *total_failures += fail as i32; 1
}

unsafe fn test_kmemcache(total_failures: *mut i32) -> i32 {
    let mut failures = 0; let mut tests = 0;
    for i in 0..10 { let size = 8 << i; for flags in 0..8 { let ctor = flags & 1 != 0; let rcu = flags & 2 != 0; let zero = flags & 4 != 0; if ctor && zero { continue; } tests += do_kmem_cache_size(size, ctor, rcu, zero, &mut failures); } tests += do_kmem_cache_size_bulk(size, &mut failures); }
    *total_failures += failures; tests
}

unsafe fn test_rcu_persistent(total_failures: *mut i32) -> i32 { let mut failures = 0; let mut tests = 0; for i in 0..10 { tests += do_kmem_cache_rcu_persistent(8 << i, &mut failures); } *total_failures += failures; tests }

#[no_mangle]
pub unsafe extern "C" fn test_meminit_init() -> i32 {
    let mut failures = 0; let mut tests = 0;
    tests += test_pages(&mut failures); tests += test_kvmalloc(&mut failures); tests += test_kmemcache(&mut failures); tests += test_rcu_persistent(&mut failures);
    if failures == 0 { 0 } else { -22 }
}

// module_init(test_meminit_init);
// MODULE_DESCRIPTION("Test cases for SL[AOU]B/page initialization at alloc/free time");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
