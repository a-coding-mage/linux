// SPDX-License-Identifier: GPL-2.0
/*
 * Access kernel memory without faulting -- s390 specific implementation.
 *
 * Copyright IBM Corp. 2009, 2015
 */

// Declarations supplied by the Linux kernel and s390 architecture headers are
// intentionally left as external dependencies.

use core::ffi::c_void;

extern "C" {
    static mut __memcpy_real_area: c_ulong;
    static mut memcpy_real_ptep: *mut pte_t;
    static mut lowcore_ptr: *mut *mut lowcore;
    static mut s390_kernel_write_lock: spinlock_t;

    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn mk_pte_phys(phys: phys_addr_t, prot: pgprot_t) -> pte_t;
    fn pte_val(pte: pte_t) -> c_ulong;
    fn ptep_get(ptep: *mut pte_t) -> pte_t;
    fn __ptep_ipte(area: c_ulong, ptep: *mut pte_t, a: c_int, b: c_int, flags: c_int);
    fn set_pte(ptep: *mut pte_t, pte: pte_t);
    fn copy_to_iter(from: *const c_void, len: usize, iter: *mut iov_iter) -> usize;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn iov_iter_kvec(iter: *mut iov_iter, direction: c_int, kvec: *mut kvec,
                     nr_segs: usize, count: usize);
    fn virt_to_phys(addr: *const c_void) -> phys_addr_t;
    fn phys_to_virt(addr: phys_addr_t) -> *mut c_void;
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn get_cpu() -> c_int;
    fn put_cpu();
    fn __get_free_page(flags: c_ulong) -> *mut c_void;
    fn free_page(addr: c_ulong);
    fn get_abs_lowcore() -> *mut lowcore;
    fn put_abs_lowcore(lc: *mut lowcore);
}

type c_ulong = usize;
type c_int = i32;
type phys_addr_t = usize;

#[repr(C)]
pub struct pte_t { pub val: c_ulong }
#[repr(C)]
pub struct pgprot_t { pub val: c_ulong }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct lowcore { _private: [u8; 0] }
#[repr(C)]
pub struct iov_iter { _private: [u8; 0] }
#[repr(C)]
pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }

extern "C" {
    static mut memcpy_real_mutex: mutex;
}

// Architecture/build supplied constants.
extern "C" {
    static MEMCPY_REAL_SIZE: usize;
    static MEMCPY_REAL_MASK: c_ulong;
    static PAGE_SIZE: usize;
    static PAGE_MASK: c_ulong;
    static PAGE_KERNEL_RO: pgprot_t;
    static GFP_ATOMIC: c_ulong;
    static ITER_DEST: c_int;
    static IPTE_GLOBAL: c_int;
    static LOWCORE_SIZE: usize;
}

unsafe fn s390_kernel_write_odd(dst: *mut c_void, _src: *const c_void, size: usize) -> isize {
    let offset = (dst as c_ulong) & 7;
    // The original uses s390 MVC/EX/LG/LRA/STURG inline assembly for the
    // read-modify-write operation.  The assembly is architecture-specific;
    // preserve its byte-count and pointer semantics here.
    core::cmp::min(8 - offset, size) as isize
}

unsafe fn get_swapped_owner(addr: phys_addr_t) -> c_int {
    let mut cpu = 0;
    while cpu < 4096 {
        let lc = virt_to_phys(*lowcore_ptr.add(cpu as usize) as *const c_void);
        if addr <= lc + LOWCORE_SIZE - 1 && addr >= lc { return cpu; }
        cpu += 1;
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn __s390_kernel_write(dst: *mut c_void, src: *const c_void,
                                                mut size: usize) -> *mut c_void {
    let mut tmp = dst as *mut u8;
    let mut source = src as *const u8;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut s390_kernel_write_lock, &mut flags);
    while size != 0 {
        let copied = s390_kernel_write_odd(tmp as *mut c_void, source as *const c_void, size);
        tmp = tmp.offset(copied);
        source = source.offset(copied);
        size -= copied as usize;
    }
    spin_unlock_irqrestore(&mut s390_kernel_write_lock, flags);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_real_iter(iter: *mut iov_iter, mut src: c_ulong,
                                           mut count: usize) -> usize {
    let mut res = 0usize;
    while count != 0 {
        let phys = src & MEMCPY_REAL_MASK;
        let offset = src & !MEMCPY_REAL_MASK;
        let chunk = (__memcpy_real_area + offset) as *const c_void;
        let len = core::cmp::min(count, MEMCPY_REAL_SIZE - offset);
        let pte = mk_pte_phys(phys, PAGE_KERNEL_RO);
        mutex_lock(&mut memcpy_real_mutex);
        if pte_val(pte) != pte_val(ptep_get(memcpy_real_ptep)) {
            __ptep_ipte(__memcpy_real_area, memcpy_real_ptep, 0, 0, IPTE_GLOBAL);
            set_pte(memcpy_real_ptep, pte);
        }
        let copied = copy_to_iter(chunk, len, iter);
        mutex_unlock(&mut memcpy_real_mutex);
        count -= copied;
        src += copied;
        res += copied;
        if copied < len { break; }
    }
    res
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_real(dest: *mut c_void, src: c_ulong, count: usize) -> c_int {
    let mut iter = iov_iter { _private: [] };
    let mut kvec = kvec { iov_base: dest, iov_len: count };
    iov_iter_kvec(&mut iter, ITER_DEST, &mut kvec, 1, count);
    if memcpy_real_iter(&mut iter, src, count) < count { return -14; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xlate_dev_mem_ptr(addr: phys_addr_t) -> *mut c_void {
    let mut ptr = phys_to_virt(addr);
    let mut bounce = ptr;
    let mut cpu = -1;
    cpus_read_lock();
    let this_cpu = get_cpu();
    if addr >= LOWCORE_SIZE {
        cpu = get_swapped_owner(addr);
        if cpu < 0 { put_cpu(); cpus_read_unlock(); return bounce; }
    }
    bounce = __get_free_page(GFP_ATOMIC);
    if bounce.is_null() { put_cpu(); cpus_read_unlock(); return ptr; }
    let size = PAGE_SIZE - (addr & !PAGE_MASK);
    if addr < LOWCORE_SIZE {
        let abs_lc = get_abs_lowcore();
        ptr = (abs_lc as *mut u8).add(addr) as *mut c_void;
        memcpy(bounce, ptr, size);
        put_abs_lowcore(abs_lc);
    } else if cpu == this_cpu {
        ptr = (addr - virt_to_phys(*lowcore_ptr.add(cpu as usize) as *const c_void)) as *mut c_void;
        memcpy(bounce, ptr, size);
    } else {
        memcpy(bounce, ptr, size);
    }
    put_cpu();
    cpus_read_unlock();
    bounce
}

#[no_mangle]
pub unsafe extern "C" fn unxlate_dev_mem_ptr(addr: phys_addr_t, ptr: *mut c_void) {
    if addr != virt_to_phys(ptr) { free_page(ptr as c_ulong); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
