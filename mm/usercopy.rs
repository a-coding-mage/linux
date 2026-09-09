// SPDX-License-Identifier: GPL-2.0-only
/*
 * This implements the various checks for CONFIG_HARDENED_USERCOPY*,
 * which are designed to protect kernel memory from needless exposure
 * and overwrite under many unintended conditions. This code is based
 * on PAX_USERCOPY, which is:
 *
 * Copyright (C) 2001-2016 PaX Team, Bradley Spengler, Open Source
 * Security Inc.
 */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_void};

extern "C" {
    static current: *mut c_void;
    static current_stack_pointer: *mut c_void;
    static _stext: u8;
    static _etext: u8;

    fn task_stack_page(task: *mut c_void) -> *const c_void;
    fn arch_within_stack_frames(stack: *const c_void, stackend: *const c_void,
                                obj: *const c_void, len: usize) -> i32;
    fn lm_alias(addr: usize) -> usize;
    fn is_kmap_addr(ptr: *const c_void) -> bool;
    fn offset_in_page(ptr: *const c_void) -> usize;
    fn is_vmalloc_addr(ptr: *const c_void) -> bool;
    fn pagefault_disabled() -> bool;
    fn find_vmap_area(addr: usize) -> *mut vmap_area;
    fn virt_addr_valid(ptr: *const c_void) -> bool;
    fn virt_to_page(ptr: *const c_void) -> *mut page;
    fn page_slab(page: *mut page) -> *mut slab;
    fn page_compound(page: *mut page) -> bool;
    fn compound_head(page: *mut page) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn page_size(page: *mut page) -> usize;
    fn __check_heap_object(ptr: *const c_void, n: usize, slab: *mut slab,
                           to_user: bool);
    fn kstrtobool(str_: *mut c_char, val: *mut bool) -> i32;
    fn static_branch_enable(key: *mut c_void);
    fn static_branch_disable(key: *mut c_void);
    fn pr_emerg(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn bug();
}

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct slab { _private: [u8; 0] }
#[repr(C)] pub struct vmap_area { pub va_start: usize, pub va_end: usize }

const THREAD_SIZE: usize = 0; // supplied by the target architecture
const PAGE_SIZE: usize = 0; // supplied by the target architecture
const NOT_STACK: i32 = 0;
const GOOD_FRAME: i32 = 1;
const GOOD_STACK: i32 = 2;
const BAD_STACK: i32 = 3;

#[inline]
unsafe fn check_stack_object(obj: *const c_void, len: usize) -> i32 {
    let stack = task_stack_page(current);
    let stackend = (stack as usize + THREAD_SIZE) as *const c_void;

    if obj as usize + len <= stack as usize || stackend as usize <= obj as usize {
        return NOT_STACK;
    }
    if obj as usize < stack as usize || stackend as usize < obj as usize + len {
        return BAD_STACK;
    }

    let ret = arch_within_stack_frames(stack, stackend, obj, len);
    if ret != 0 { return ret; }

    // CONFIG_ARCH_HAS_CURRENT_STACK_POINTER is a build-time condition.
    #[cfg(CONFIG_ARCH_HAS_CURRENT_STACK_POINTER)]
    {
        // CONFIG_STACK_GROWSUP selects the alternate stack direction.
        #[cfg(CONFIG_STACK_GROWSUP)]
        if current_stack_pointer as usize < obj as usize + len { return BAD_STACK; }
        #[cfg(not(CONFIG_STACK_GROWSUP))]
        if obj as usize < current_stack_pointer as usize { return BAD_STACK; }
    }
    GOOD_STACK
}

#[no_mangle]
pub unsafe extern "C" fn usercopy_abort(name: *const c_char, detail: *const c_char,
                                          to_user: bool, offset: usize, len: usize) -> ! {
    // pr_emerg("Kernel memory %s attempt detected ...", ...);
    let _ = (name, detail, to_user, offset, len);
    bug();
}

#[inline]
unsafe fn overlaps(ptr: usize, n: usize, low: usize, high: usize) -> bool {
    let check_high = ptr.wrapping_add(n);
    if ptr >= high || check_high <= low { return false; }
    true
}

#[inline]
unsafe fn check_kernel_text_object(ptr: usize, n: usize, to_user: bool) {
    let textlow = &_stext as *const u8 as usize;
    let texthigh = &_etext as *const u8 as usize;
    if overlaps(ptr, n, textlow, texthigh) {
        usercopy_abort(core::ptr::null(), core::ptr::null(), to_user,
                       ptr.wrapping_sub(textlow), n);
    }
    let textlow_linear = lm_alias(textlow);
    if textlow_linear == textlow { return; }
    let texthigh_linear = lm_alias(texthigh);
    if overlaps(ptr, n, textlow_linear, texthigh_linear) {
        usercopy_abort(core::ptr::null(), core::ptr::null(), to_user,
                       ptr.wrapping_sub(textlow_linear), n);
    }
}

#[inline]
unsafe fn check_bogus_address(ptr: usize, n: usize, to_user: bool) {
    if ptr.wrapping_add(n.wrapping_sub(1)) < ptr {
        usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, 0,
                       ptr.wrapping_add(n));
    }
    if ptr == 0 || ptr < 4096 {
        usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, ptr, n);
    }
}

#[inline]
unsafe fn check_heap_object(ptr: *const c_void, n: usize, to_user: bool) {
    let addr = ptr as usize;
    let mut offset: usize;
    if is_kmap_addr(ptr) {
        offset = offset_in_page(ptr);
        if n > PAGE_SIZE - offset { usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, offset, n); }
        return;
    }
    if is_vmalloc_addr(ptr) && !pagefault_disabled() {
        let area = find_vmap_area(addr);
        if area.is_null() { usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, 0, n); }
        if n > (*area).va_end - addr {
            offset = addr - (*area).va_start;
            usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, offset, n);
        }
        return;
    }
    if !virt_addr_valid(ptr) { return; }
    let mut pg = virt_to_page(ptr);
    let sl = page_slab(pg);
    if !sl.is_null() {
        __check_heap_object(ptr, n, sl, to_user);
    } else if page_compound(pg) {
        pg = compound_head(pg);
        offset = addr - page_address(pg) as usize;
        if n > page_size(pg) - offset { usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, offset, n); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __check_object_size(ptr: *const c_void, n: usize, to_user: bool) {
    if n == 0 { return; }
    check_bogus_address(ptr as usize, n, to_user);
    match check_stack_object(ptr, n) {
        NOT_STACK => {}
        GOOD_FRAME | GOOD_STACK => return,
        _ => usercopy_abort(core::ptr::null(), core::ptr::null(), to_user, 0, n),
    }
    check_heap_object(ptr, n, to_user);
    check_kernel_text_object(ptr as usize, n, to_user);
}

static mut enable_checks: bool = true; // IS_ENABLED(CONFIG_HARDENED_USERCOPY_DEFAULT_ON)

unsafe extern "C" fn parse_hardened_usercopy(str_: *mut c_char) -> i32 {
    if kstrtobool(str_, &mut enable_checks) != 0 { /* pr_warn(...) */ }
    1
}

unsafe extern "C" fn set_hardened_usercopy() -> i32 {
    if enable_checks { static_branch_enable(core::ptr::null_mut()); }
    else { static_branch_disable(core::ptr::null_mut()); }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
