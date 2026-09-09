// SPDX-License-Identifier: GPL-2.0-only
/* Basic general purpose allocator for special-purpose memory. */

// Kernel headers and symbols referenced by this translation are supplied externally.

#[inline]
unsafe fn chunk_size(chunk: *const gen_pool_chunk) -> usize {
    (*chunk).end_addr - (*chunk).start_addr + 1
}

#[inline]
unsafe fn set_bits_ll(addr: *mut usize, mask_to_set: usize) -> i32 {
    let mut val = core::ptr::read_volatile(addr);
    loop {
        if val & mask_to_set != 0 {
            return -16; // -EBUSY
        }
        core::hint::spin_loop();
        match (*addr).compare_exchange_weak(val, val | mask_to_set, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {
            Ok(_) => break,
            Err(next) => val = next,
        }
    }
    0
}

#[inline]
unsafe fn clear_bits_ll(addr: *mut usize, mask_to_clear: usize) -> i32 {
    let mut val = core::ptr::read_volatile(addr);
    loop {
        if val & mask_to_clear != mask_to_clear {
            return -16; // -EBUSY
        }
        core::hint::spin_loop();
        match (*addr).compare_exchange_weak(val, val & !mask_to_clear, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {
            Ok(_) => break,
            Err(next) => val = next,
        }
    }
    0
}

unsafe fn bitmap_set_ll(map: *mut usize, start: usize, mut nr: usize) -> usize {
    let mut p = map.add(start / BITS_PER_LONG);
    let size = start + nr;
    let mut bits_to_set = BITS_PER_LONG - (start % BITS_PER_LONG);
    let mut mask_to_set = !(0usize) << (start % BITS_PER_LONG);
    while nr >= bits_to_set {
        if set_bits_ll(p, mask_to_set) != 0 { return nr; }
        nr -= bits_to_set;
        bits_to_set = BITS_PER_LONG;
        mask_to_set = !0usize;
        p = p.add(1);
    }
    if nr != 0 {
        mask_to_set &= !0usize >> (BITS_PER_LONG - (size % BITS_PER_LONG));
        if set_bits_ll(p, mask_to_set) != 0 { return nr; }
    }
    0
}

unsafe fn bitmap_clear_ll(map: *mut usize, start: usize, mut nr: usize) -> usize {
    let mut p = map.add(start / BITS_PER_LONG);
    let size = start + nr;
    let mut bits_to_clear = BITS_PER_LONG - (start % BITS_PER_LONG);
    let mut mask_to_clear = !(0usize) << (start % BITS_PER_LONG);
    while nr >= bits_to_clear {
        if clear_bits_ll(p, mask_to_clear) != 0 { return nr; }
        nr -= bits_to_clear;
        bits_to_clear = BITS_PER_LONG;
        mask_to_clear = !0usize;
        p = p.add(1);
    }
    if nr != 0 {
        mask_to_clear &= !0usize >> (BITS_PER_LONG - (size % BITS_PER_LONG));
        if clear_bits_ll(p, mask_to_clear) != 0 { return nr; }
    }
    0
}

// External kernel types and functions are intentionally not implemented here.
#[repr(C)] pub struct gen_pool { pub lock: usize, pub chunks: list_head, pub min_alloc_order: i32, pub algo: genpool_algo_t, pub data: *mut core::ffi::c_void, pub name: *const i8 }
#[repr(C)] pub struct gen_pool_chunk { pub next_chunk: list_head, pub phys_addr: u64, pub start_addr: usize, pub end_addr: usize, pub avail: core::sync::atomic::AtomicUsize, pub owner: *mut core::ffi::c_void, pub bits: [usize; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct genpool_data_align { pub align: i32 }
pub type phys_addr_t = u64;
pub type dma_addr_t = u64;
pub type genpool_algo_t = unsafe extern "C" fn(*mut usize, usize, usize, usize, *mut core::ffi::c_void, *mut gen_pool, usize) -> usize;
const BITS_PER_LONG: usize = usize::BITS as usize;

extern "C" {
    fn kmalloc_node(size: usize, flags: usize, nid: i32) -> *mut gen_pool;
    fn vzalloc_node(size: usize, nid: i32) -> *mut gen_pool_chunk;
    fn vfree(ptr: *mut core::ffi::c_void);
    fn kfree(ptr: *mut gen_pool);
    fn gen_pool_first_fit(_: *mut usize, _: usize, _: usize, _: usize, _: *mut core::ffi::c_void, _: *mut gen_pool, _: usize) -> usize;
    fn gen_pool_first_fit_align(_: *mut usize, _: usize, _: usize, _: usize, _: *mut core::ffi::c_void, _: *mut gen_pool, _: usize) -> usize;
    fn gen_pool_virt_to_phys(pool: *mut gen_pool, addr: usize) -> phys_addr_t;
}

#[no_mangle]
pub unsafe extern "C" fn gen_pool_create(min_alloc_order: i32, nid: i32) -> *mut gen_pool { let pool = kmalloc_node(core::mem::size_of::<gen_pool>(), 0, nid); if !pool.is_null() { (*pool).min_alloc_order = min_alloc_order; (*pool).algo = gen_pool_first_fit; (*pool).data = core::ptr::null_mut(); (*pool).name = core::ptr::null(); } pool }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_add_owner(pool: *mut gen_pool, virt: usize, phys: phys_addr_t, size: usize, nid: i32, owner: *mut core::ffi::c_void) -> i32 { let nbits = size >> (*pool).min_alloc_order; let nbytes = core::mem::size_of::<gen_pool_chunk>() + ((nbits + BITS_PER_LONG - 1) / BITS_PER_LONG) * core::mem::size_of::<usize>(); let chunk = vzalloc_node(nbytes, nid); if chunk.is_null() { return -12; } (*chunk).phys_addr = phys; (*chunk).start_addr = virt; (*chunk).end_addr = virt + size - 1; (*chunk).owner = owner; (*chunk).avail.store(size, core::sync::atomic::Ordering::SeqCst); 0 }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_destroy(_pool: *mut gen_pool) {}

#[no_mangle]
pub unsafe extern "C" fn gen_pool_alloc_algo_owner(pool: *mut gen_pool, size: usize, algo: genpool_algo_t, data: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> usize { if !owner.is_null() { *owner = core::ptr::null_mut(); } if size == 0 { return 0; } let order = (*pool).min_alloc_order as usize; let nbits = (size + (1usize << order) - 1) >> order; let mut c = (*pool).chunks.next; while !c.is_null() { let chunk = c as *mut gen_pool_chunk; let end = chunk_size(chunk) >> order; if size <= (*chunk).avail.load(core::sync::atomic::Ordering::SeqCst) { let start = algo((*chunk).bits.as_mut_ptr(), end, 0, nbits, data, pool, (*chunk).start_addr); if start < end && bitmap_set_ll((*chunk).bits.as_mut_ptr(), start, nbits) == 0 { let addr = (*chunk).start_addr + (start << order); (*chunk).avail.fetch_sub(nbits << order, core::sync::atomic::Ordering::SeqCst); if !owner.is_null() { *owner = (*chunk).owner; } return addr; } } c = (*c).next; } 0 }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_alloc(_pool: *mut gen_pool, _size: usize, _dma: *mut dma_addr_t) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_alloc_algo(pool: *mut gen_pool, size: usize, dma: *mut dma_addr_t, algo: genpool_algo_t, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void { if pool.is_null() { return core::ptr::null_mut(); } let v = gen_pool_alloc_algo_owner(pool, size, algo, data, core::ptr::null_mut()); if v == 0 { return core::ptr::null_mut(); } if !dma.is_null() { *dma = gen_pool_virt_to_phys(pool, v); } v as *mut core::ffi::c_void }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_alloc_align(_pool: *mut gen_pool, _size: usize, _dma: *mut dma_addr_t, _align: i32) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_zalloc(_pool: *mut gen_pool, _size: usize, _dma: *mut dma_addr_t) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_zalloc_algo(_pool: *mut gen_pool, _size: usize, _dma: *mut dma_addr_t, _algo: genpool_algo_t, _data: *mut core::ffi::c_void) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_dma_zalloc_align(_pool: *mut gen_pool, _size: usize, _dma: *mut dma_addr_t, _align: i32) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn gen_pool_free_owner(pool: *mut gen_pool, addr: usize, size: usize, owner: *mut *mut core::ffi::c_void) { let order = (*pool).min_alloc_order as usize; let nbits = (size + (1usize << order) - 1) >> order; let mut c = (*pool).chunks.next; while !c.is_null() { let chunk = c as *mut gen_pool_chunk; if addr >= (*chunk).start_addr && addr <= (*chunk).end_addr { bitmap_clear_ll((*chunk).bits.as_mut_ptr(), (addr - (*chunk).start_addr) >> order, nbits); (*chunk).avail.fetch_add(nbits << order, core::sync::atomic::Ordering::SeqCst); if !owner.is_null() { *owner = (*chunk).owner; } return; } c = (*c).next; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
