// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kstrtobool(buf: *mut c_char, result: *mut bool) -> c_int;
    fn kmap_local_page(page: *mut page) -> *mut c_void;
    fn kasan_disable_current();
    fn kasan_enable_current();
    fn kasan_reset_tag(addr: *mut c_void) -> *mut c_void;
    fn memset(addr: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn kunmap_local(addr: *mut c_void);
    fn memchr_inv(addr: *mut u8, value: c_int, size: usize) -> *mut u8;
    fn __ratelimit(state: *mut ratelimit_state) -> bool;
    fn print_hex_dump(
        level: *const c_char,
        prefix_str: *const c_char,
        prefix_type: c_int,
        rowsize: c_int,
        groupsize: c_int,
        buf: *const c_void,
        len: usize,
        ascii: bool,
    );
    fn dump_stack();
    fn dump_page(page: *mut page, reason: *const c_char);
    fn pr_err(format: *const c_char, ...);
}

#[repr(C)]
pub struct ratelimit_state {
    _private: [u8; 0],
}

pub const PAGE_POISON: u8 = 0xaa;
pub const PAGE_SIZE: usize = 4096;
pub const HZ: usize = 100;
pub const KERN_ERR: &[u8] = b"KERN_ERR\0";
pub const DUMP_PREFIX_ADDRESS: c_int = 1;

#[no_mangle]
pub static mut _page_poisoning_enabled_early: bool = false;

#[no_mangle]
pub static mut _page_poisoning_enabled: bool = false;

unsafe fn early_page_poison_param(buf: *mut c_char) -> c_int {
    kstrtobool(buf, core::ptr::addr_of_mut!(_page_poisoning_enabled_early))
}

unsafe fn poison_page(page: *mut page) {
    let addr = kmap_local_page(page);

    // KASAN still thinks the page is in-use, so skip it.
    kasan_disable_current();
    memset(kasan_reset_tag(addr), PAGE_POISON as c_int, PAGE_SIZE);
    kasan_enable_current();
    kunmap_local(addr);
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_poison_pages(page: *mut page, n: c_int) {
    let mut i = 0;
    while i < n {
        poison_page(page.add(i as usize));
        i += 1;
    }
}

unsafe fn single_bit_flip(a: u8, b: u8) -> bool {
    let error = a ^ b;
    error != 0 && (error & error.wrapping_sub(1)) == 0
}

unsafe fn check_poison_mem(page: *mut page, mem: *mut u8, bytes: usize) {
    // DEFINE_RATELIMIT_STATE(ratelimit, 5 * HZ, 10)
    static mut RATELIMIT: ratelimit_state = ratelimit_state { _private: [] };
    let start = memchr_inv(mem, PAGE_POISON as c_int, bytes);
    if start.is_null() {
        return;
    }

    let mut end = mem.add(bytes - 1);
    while end > start {
        if *end != PAGE_POISON {
            break;
        }
        end = end.sub(1);
    }

    if !__ratelimit(core::ptr::addr_of_mut!(RATELIMIT)) {
        return;
    } else if start == end && single_bit_flip(*start, PAGE_POISON) {
        pr_err(b"pagealloc: single bit error\n\0".as_ptr() as *const c_char);
    } else {
        pr_err(b"pagealloc: memory corruption\n\0".as_ptr() as *const c_char);
    }

    print_hex_dump(
        KERN_ERR.as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        DUMP_PREFIX_ADDRESS,
        16,
        1,
        start as *const c_void,
        end.offset_from(start) as usize + 1,
        true,
    );
    dump_stack();
    dump_page(page, b"pagealloc: corrupted page details\0".as_ptr() as *const c_char);
}

unsafe fn unpoison_page(page: *mut page) {
    let addr = kmap_local_page(page);
    kasan_disable_current();
    /*
     * Page poisoning when enabled poisons each and every page
     * that is freed to buddy. Thus no extra check is done to
     * see if a page was poisoned.
     */
    check_poison_mem(page, kasan_reset_tag(addr) as *mut u8, PAGE_SIZE);
    kasan_enable_current();
    kunmap_local(addr);
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_unpoison_pages(page: *mut page, n: c_int) {
    let mut i = 0;
    while i < n {
        unpoison_page(page.add(i as usize));
        i += 1;
    }
}

// #ifndef CONFIG_ARCH_SUPPORTS_DEBUG_PAGEALLOC
#[no_mangle]
pub unsafe extern "C" fn __kernel_map_pages(_page: *mut page, _numpages: c_int, _enable: c_int) {
    // This function does nothing, all work is done via poison pages.
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
