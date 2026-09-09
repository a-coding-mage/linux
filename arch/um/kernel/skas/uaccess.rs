// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

unsafe fn virt_to_pte(mm: *mut mm_struct, addr: c_ulong) -> *mut pte_t {
    let mut pgd: *mut pgd_t;
    let mut p4d: *mut p4d_t;
    let mut pud: *mut pud_t;
    let mut pmd: *mut pmd_t;

    if mm.is_null() {
        return core::ptr::null_mut();
    }

    pgd = pgd_offset(mm, addr);
    if !pgd_present(*pgd) {
        return core::ptr::null_mut();
    }

    p4d = p4d_offset(pgd, addr);
    if !p4d_present(*p4d) {
        return core::ptr::null_mut();
    }

    pud = pud_offset(p4d, addr);
    if !pud_present(*pud) {
        return core::ptr::null_mut();
    }

    pmd = pmd_offset(pud, addr);
    if !pmd_present(*pmd) {
        return core::ptr::null_mut();
    }

    pte_offset_kernel(pmd, addr)
}

unsafe fn maybe_map(virt: c_ulong, is_write: c_int) -> *mut pte_t {
    let mut pte = virt_to_pte((*current).mm, virt);
    let mut dummy_code: c_int = 0;

    if pte.is_null() || !pte_present(*pte) || (is_write != 0 && !pte_write(*pte)) {
        let err = handle_page_fault(virt, 0, is_write, 1, &mut dummy_code);
        if err != 0 {
            return core::ptr::null_mut();
        }
        pte = virt_to_pte((*current).mm, virt);
    }
    if !pte_present(*pte) {
        pte = core::ptr::null_mut();
    }

    pte
}

unsafe fn do_op_one_page(
    mut addr: c_ulong,
    len: c_int,
    is_write: c_int,
    op: unsafe extern "C" fn(c_ulong, c_int, *mut c_void) -> c_int,
    arg: *mut c_void,
) -> c_int {
    let pte = maybe_map(addr, is_write);
    if pte.is_null() {
        return -1;
    }

    let page = pte_page(*pte);
    #[cfg(target_pointer_width = "64")]
    {
        pagefault_disable();
        addr = page_address(page) as c_ulong + (addr & !PAGE_MASK);
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        addr = kmap_atomic(page) as c_ulong + (addr & !PAGE_MASK);
    }
    let n = op(addr, len, arg);

    #[cfg(target_pointer_width = "64")]
    pagefault_enable();
    #[cfg(not(target_pointer_width = "64"))]
    kunmap_atomic(addr as *mut c_void);

    n
}

unsafe fn buffer_op(
    mut addr: c_ulong,
    len: c_int,
    is_write: c_int,
    op: unsafe extern "C" fn(c_ulong, c_int, *mut c_void) -> c_int,
    arg: *mut c_void,
) -> c_long {
    let size = core::cmp::min(PAGE_ALIGN(addr) - addr, len as c_ulong);
    let mut remain = len as c_long;

    let mut n = do_op_one_page(addr, size as c_int, is_write, op, arg);
    if n != 0 {
        remain = if n < 0 { remain } else { 0 };
        return remain;
    }

    addr += size;
    remain -= size as c_long;
    if remain == 0 {
        return remain;
    }

    while addr < ((addr + remain as c_ulong) & PAGE_MASK) {
        n = do_op_one_page(addr, PAGE_SIZE as c_int, is_write, op, arg);
        if n != 0 {
            remain = if n < 0 { remain } else { 0 };
            return remain;
        }
        addr += PAGE_SIZE;
        remain -= PAGE_SIZE as c_long;
    }
    if remain == 0 {
        return remain;
    }

    n = do_op_one_page(addr, remain as c_int, is_write, op, arg);
    if n != 0 {
        remain = if n < 0 { remain } else { 0 };
        return remain;
    }

    0
}

unsafe extern "C" fn copy_chunk_from_user(from: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    let to_ptr = arg as *mut c_ulong;
    let to = *to_ptr;
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, len as usize);
    *to_ptr += len as c_ulong;
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong {
    buffer_op(from as c_ulong, n as c_int, 0, copy_chunk_from_user, &to as *const _ as *mut c_void) as c_ulong
}

unsafe extern "C" fn copy_chunk_to_user(to: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    let from_ptr = arg as *mut c_ulong;
    let from = *from_ptr;
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, len as usize);
    *from_ptr += len as c_ulong;
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_copy_to_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong {
    buffer_op(to as c_ulong, n as c_int, 1, copy_chunk_to_user, &from as *const _ as *mut c_void) as c_ulong
}

unsafe extern "C" fn strncpy_chunk_from_user(from: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    let to_ptr = arg as *mut *mut c_char;
    let to = *to_ptr;
    let n = strnlen(from as *const c_void, len as usize) as c_int;
    memcpy_and_pad(to as *mut c_void, len as usize, from as *const c_void, n as usize, 0);
    *to_ptr = (*to_ptr).add(n as usize);
    if n < len { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user(dst: *mut c_char, src: *const c_char, count: c_long) -> c_long {
    if !access_ok(src, 1) { return -EFAULT as c_long; }
    let mut ptr = dst;
    let n = buffer_op(src as c_ulong, count as c_int, 0, strncpy_chunk_from_user, &mut ptr as *mut _ as *mut c_void);
    if n != 0 { return -EFAULT as c_long; }
    strnlen(dst as *const c_void, count as usize) as c_long
}

unsafe extern "C" fn clear_chunk(addr: c_ulong, len: c_int, _unused: *mut c_void) -> c_int {
    core::ptr::write_bytes(addr as *mut u8, 0, len as usize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __clear_user(mem: *mut c_void, len: c_ulong) -> c_ulong {
    buffer_op(mem as c_ulong, len as c_int, 1, clear_chunk, core::ptr::null_mut()) as c_ulong
}

unsafe extern "C" fn strnlen_chunk(str_: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    let len_ptr = arg as *mut c_int;
    let n = strnlen(str_ as *const c_void, len as usize) as c_int;
    *len_ptr += n;
    if n < len { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn strnlen_user(str_: *const c_char, len: c_long) -> c_long {
    let mut count: c_int = 0;
    if !access_ok(str_, 1) { return -EFAULT as c_long; }
    let n = buffer_op(str_ as c_ulong, len as c_int, 0, strnlen_chunk, &mut count as *mut _ as *mut c_void);
    if n == 0 { (count + 1) as c_long } else { 0 }
}

/* Atomic arithmetic operation with constant argument and comparison of the previous futex value. */
#[no_mangle]
pub unsafe extern "C" fn arch_futex_atomic_op_inuser(op: c_int, oparg: u32, oval: *mut c_int, uaddr: *mut u32) -> c_int {
    let mut oldval: c_int;
    let mut ret = -EFAULT;
    let mut addr = uaddr as c_ulong;
    if !access_ok(uaddr, core::mem::size_of::<u32>()) { return -EFAULT; }
    preempt_disable();
    let pte = maybe_map(addr, 1);
    if pte.is_null() { preempt_enable(); return ret; }
    let page = pte_page(*pte);
    #[cfg(target_pointer_width = "64")]
    { pagefault_disable(); addr = page_address(page) as c_ulong + (addr & !PAGE_MASK); }
    #[cfg(not(target_pointer_width = "64"))]
    { addr = kmap_atomic(page) as c_ulong + (addr & !PAGE_MASK); }
    let uaddr = addr as *mut u32;
    oldval = *uaddr as c_int;
    ret = 0;
    match op {
        FUTEX_OP_SET => *uaddr = oparg,
        FUTEX_OP_ADD => *uaddr = (*uaddr).wrapping_add(oparg),
        FUTEX_OP_OR => *uaddr |= oparg,
        FUTEX_OP_ANDN => *uaddr &= !oparg,
        FUTEX_OP_XOR => *uaddr ^= oparg,
        _ => ret = -ENOSYS,
    }
    #[cfg(target_pointer_width = "64")]
    pagefault_enable();
    #[cfg(not(target_pointer_width = "64"))]
    kunmap_atomic(addr as *mut c_void);
    preempt_enable();
    if ret == 0 { *oval = oldval; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn futex_atomic_cmpxchg_inatomic(uval: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32) -> c_int {
    if !access_ok(uaddr, core::mem::size_of::<u32>()) { return -EFAULT; }
    preempt_disable();
    let pte = maybe_map(uaddr as c_ulong, 1);
    if pte.is_null() { preempt_enable(); return -EFAULT; }
    let page = pte_page(*pte);
    #[cfg(target_pointer_width = "64")]
    { pagefault_disable(); uaddr = (page_address(page) as c_ulong + ((uaddr as c_ulong) & !PAGE_MASK)) as *mut u32; }
    #[cfg(not(target_pointer_width = "64"))]
    { uaddr = (kmap_atomic(page) as c_ulong + ((uaddr as c_ulong) & !PAGE_MASK)) as *mut u32; }
    *uval = *uaddr;
    let _ = core::sync::atomic::AtomicU32::from_ptr(uaddr).compare_exchange(oldval, newval, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst);
    #[cfg(target_pointer_width = "64")]
    pagefault_enable();
    #[cfg(not(target_pointer_width = "64"))]
    kunmap_atomic(uaddr as *mut c_void);
    preempt_enable();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
