// SPDX-License-Identifier: GPL-2.0
// io_uring zero-copy receive provider, translated from unchanged zcrx.c.
// All external layouts and C ABI types come from configured kernel headers.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/io_uring_zcrx_generated.rs"));
}
use bindings::*;
use core::mem::{align_of, size_of, size_of_val, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_uint, c_ulong, c_void};

const ZCRX_MAX_AREAS: u32 = 1024;
const IO_RQ_MAX_ENTRIES: u32 = 32768;
const IO_SKBS_PER_CALL_LIMIT: u32 = 20;
const ZCRX_FLUSH_BATCH: usize = 32;
fn neg(e: u32) -> c_int { -(e as c_int) }
fn is_err<T>(p: *const T) -> bool { p as usize >= usize::MAX - MAX_ERRNO as usize + 1 }
fn ptr_err<T>(p: *const T) -> c_int { p as isize as c_int }
unsafe fn poison<T>() -> *mut T { rust_zcrx_poison().cast() }

// The guards retain C guard/scoped_guard release order across every early exit.
struct BhLock(*mut spinlock_t);
impl BhLock { unsafe fn new(p: *mut spinlock_t) -> Self { rust_zcrx_spin_lock_bh(p); Self(p) } }
impl Drop for BhLock { fn drop(&mut self) { unsafe { rust_zcrx_spin_unlock_bh(self.0) } } }
struct MutexLock(*mut mutex);
impl MutexLock { unsafe fn new(p: *mut mutex) -> Self { rust_zcrx_mutex_lock(p); Self(p) } }
impl Drop for MutexLock { fn drop(&mut self) { unsafe { rust_zcrx_mutex_unlock(self.0) } } }
struct FdGuard(fd);
impl Drop for FdGuard { fn drop(&mut self) { unsafe { rust_zcrx_fdput(self.0) } } }

unsafe fn zcrx_next_area_id(z: *mut io_zcrx_ifq) -> u32 { (*z).nr_areas }
fn zcrx_area_id_to_token(id: u32) -> u64 { (id as u64) << IORING_ZCRX_AREA_SHIFT }
unsafe fn io_pp_to_ifq(pp: *mut page_pool) -> *mut io_zcrx_ifq { (*pp).mp_priv.cast() }
unsafe fn io_zcrx_iov_to_area(n: *const net_iov) -> *mut io_zcrx_area {
    rust_zcrx_niov_area(n)
}
unsafe fn zcrx_set_ring_ctx(z: *mut io_zcrx_ifq, ctx: *mut io_ring_ctx) -> bool {
    let _lock = BhLock::new(addr_of_mut!((*z).ctx_lock));
    if !(*z).master_ctx.is_null() { return false; }
    rust_zcrx_percpu_get(rust_zcrx_ctx_refs(ctx));
    (*z).master_ctx = ctx;
    true
}
unsafe fn io_zcrx_iov_page(n: *const net_iov) -> *mut page {
    let a = io_zcrx_iov_to_area(n);
    rust_zcrx_lockdep_assert(!(*a).mem.is_dmabuf);
    let shift = (*(*a).ifq).niov_shift - RUST_ZCRX_PAGE_SHIFT;
    *(*a).mem.pages.add((rust_zcrx_niov_idx(n) << shift) as usize)
}
unsafe fn io_area_max_shift(mem: *mut io_zcrx_mem) -> c_int {
    let sgt = (*mem).sgt;
    let mut sg = (*sgt).sgl;
    let mut shift = u32::MAX;
    for _ in 0..(*sgt).nents {
        shift = shift.min(rust_zcrx_sg_dma_len(sg).trailing_zeros());
        sg = rust_zcrx_sg_next(sg);
    }
    shift as c_int
}
unsafe fn io_populate_area_dma(ifq: *mut io_zcrx_ifq, a: *mut io_zcrx_area) -> c_int {
    let size = 1u32 << (*ifq).niov_shift;
    let sgt = (*a).mem.sgt;
    let mut sg = (*sgt).sgl;
    let mut idx = 0;
    for _ in 0..(*sgt).nents {
        let mut dma = rust_zcrx_sg_dma_address(sg);
        let mut len = rust_zcrx_sg_dma_len(sg) as c_ulong;
        if rust_zcrx_warn_once_01(len % size as c_ulong != 0) { return neg(EINVAL); }
        while len != 0 && idx < (*a).nia.num_niovs {
            if rust_zcrx_niov_set_dma((*a).nia.niovs.add(idx as usize), dma) != 0 {
                return neg(EFAULT);
            }
            len -= size as c_ulong;
            dma = dma.wrapping_add(size as dma_addr_t);
            idx += 1;
        }
        sg = rust_zcrx_sg_next(sg);
    }
    if rust_zcrx_warn_once_02(idx != (*a).nia.num_niovs) { neg(EFAULT) } else { 0 }
}
unsafe fn io_unmap_dmabuf(mem: *mut io_zcrx_mem) {
    #[cfg(CONFIG_DMA_SHARED_BUFFER)] {
        if !(*mem).sgt.is_null() { dma_buf_unmap_attachment_unlocked((*mem).attach, (*mem).sgt, DMA_FROM_DEVICE); }
        if !(*mem).attach.is_null() { dma_buf_detach((*mem).dmabuf, (*mem).attach); }
        (*mem).sgt = null_mut();
        (*mem).attach = null_mut();
    }
}
unsafe fn io_release_dmabuf(mem: *mut io_zcrx_mem) {
    #[cfg(CONFIG_DMA_SHARED_BUFFER)] {
        if !(*mem).dmabuf.is_null() { dma_buf_put((*mem).dmabuf); }
        (*mem).dmabuf = null_mut();
    }
}
unsafe fn io_import_dmabuf(ifq: *mut io_zcrx_ifq, mem: *mut io_zcrx_mem,
    reg: *const io_uring_zcrx_area_reg) -> c_int {
    if (*ifq).dev.is_null() || (*reg).addr as c_ulong != 0 { return neg(EINVAL); }
    #[cfg(not(CONFIG_DMA_SHARED_BUFFER))] { return neg(EINVAL); }
    #[cfg(CONFIG_DMA_SHARED_BUFFER)] {
        (*mem).is_dmabuf = true;
        let ret = 'import: {
            (*mem).dmabuf = dma_buf_get((*reg).dmabuf_fd as c_int);
            if is_err((*mem).dmabuf) {
                let ret = ptr_err((*mem).dmabuf); (*mem).dmabuf = null_mut(); break 'import ret;
            }
            (*mem).attach = dma_buf_attach((*mem).dmabuf, (*ifq).dev);
            if is_err((*mem).attach) {
                let ret = ptr_err((*mem).attach); (*mem).attach = null_mut(); break 'import ret;
            }
            (*mem).sgt = dma_buf_map_attachment_unlocked((*mem).attach, DMA_FROM_DEVICE);
            if is_err((*mem).sgt) {
                let ret = ptr_err((*mem).sgt); (*mem).sgt = null_mut(); break 'import ret;
            }
            let mut total: c_ulong = 0;
            let mut sg = (*(*mem).sgt).sgl;
            for _ in 0..(*(*mem).sgt).nents {
                total = total.wrapping_add(rust_zcrx_sg_dma_len(sg) as c_ulong);
                sg = rust_zcrx_sg_next(sg);
            }
            if total != (*reg).len as c_ulong { break 'import neg(EINVAL); }
            (*mem).size = (*reg).len as c_ulong;
            return 0;
        };
        io_unmap_dmabuf(mem);
        io_release_dmabuf(mem);
        ret
    }
}
unsafe fn io_count_account_pages(pages: *mut *mut page, nr: c_uint) -> c_ulong {
    let mut last = null_mut();
    let mut res: c_ulong = 0;
    for i in 0..nr {
        let folio = rust_zcrx_page_folio(*pages.add(i as usize));
        if folio == last { continue; }
        last = folio;
        res = res.wrapping_add(rust_zcrx_folio_nr_pages(folio));
    }
    res
}
unsafe fn io_import_umem(ifq: *mut io_zcrx_ifq, mem: *mut io_zcrx_mem,
    reg: *const io_uring_zcrx_area_reg) -> c_int {
    if (*reg).dmabuf_fd != 0 { return neg(EINVAL); }
    if (*reg).addr == 0 { return neg(EFAULT); }
    let mut nr_pages: c_int = 0;
    let pages = io_pin_pages((*reg).addr as c_ulong, (*reg).len as c_ulong, addr_of_mut!(nr_pages));
    if is_err(pages) { return ptr_err(pages); }
    let mut mapped = false;
    let ret = 'import: {
        let ret = rust_zcrx_sg_alloc(addr_of_mut!((*mem).page_sg_table), pages,
            nr_pages as c_uint, (nr_pages as c_ulong) << RUST_ZCRX_PAGE_SHIFT);
        if ret != 0 { break 'import ret; }
        if !(*ifq).dev.is_null() {
            let ret = rust_zcrx_dma_map((*ifq).dev, addr_of_mut!((*mem).page_sg_table));
            if ret < 0 { break 'import ret; }
            mapped = true;
        }
        (*mem).account_pages = io_count_account_pages(pages, nr_pages as c_uint);
        let ret = io_account_mem((*ifq).user, (*ifq).mm_account, (*mem).account_pages);
        if ret < 0 { (*mem).account_pages = 0; break 'import ret; }
        (*mem).sgt = addr_of_mut!((*mem).page_sg_table);
        (*mem).pages = pages;
        (*mem).nr_folios = nr_pages as c_ulong;
        (*mem).size = (*reg).len as c_ulong;
        return ret;
    };
    if mapped { rust_zcrx_dma_unmap((*ifq).dev, addr_of_mut!((*mem).page_sg_table)); }
    sg_free_table(addr_of_mut!((*mem).page_sg_table));
    unpin_user_pages(pages, nr_pages as c_ulong);
    kvfree(pages.cast());
    ret
}
unsafe fn io_release_area_mem(mem: *mut io_zcrx_mem) {
    if (*mem).is_dmabuf { io_release_dmabuf(mem); }
    else if !(*mem).pages.is_null() {
        unpin_user_pages((*mem).pages, (*mem).nr_folios);
        sg_free_table((*mem).sgt);
        kvfree((*mem).pages.cast());
    }
    (*mem).pages = poison();
    (*mem).sgt = poison();
}
unsafe fn io_import_area(ifq: *mut io_zcrx_ifq, mem: *mut io_zcrx_mem,
    reg: *const io_uring_zcrx_area_reg) -> c_int {
    if (*reg).flags & !IORING_ZCRX_AREA_DMABUF != 0 || (*reg).rq_area_token != 0
        || (*reg).__resv2[0] != 0 || (*reg).__resv2[1] != 0 { return neg(EINVAL); }
    let ret = io_validate_user_buf_range((*reg).addr, (*reg).len);
    if ret != 0 { return ret; }
    if ((*reg).addr | (*reg).len) & (RUST_ZCRX_PAGE_SIZE as u64 - 1) != 0 { return neg(EINVAL); }
    if (*reg).flags & IORING_ZCRX_AREA_DMABUF != 0 { io_import_dmabuf(ifq, mem, reg) }
    else { io_import_umem(ifq, mem, reg) }
}
unsafe fn __io_zcrx_unmap_area(ifq: *mut io_zcrx_ifq, a: *mut io_zcrx_area) {
    rust_zcrx_assert_mutex(addr_of_mut!((*ifq).pp_lock));
    if a.is_null() || !(*a).is_mapped { return; }
    (*a).is_mapped = false;
    if !(*a).nia.niovs.is_null() {
        for i in 0..(*a).nia.num_niovs { rust_zcrx_niov_set_dma((*a).nia.niovs.add(i as usize), 0); }
    }
    if (*a).mem.is_dmabuf { io_unmap_dmabuf(addr_of_mut!((*a).mem)); }
    else { rust_zcrx_dma_unmap((*ifq).dev, addr_of_mut!((*a).mem.page_sg_table)); }
}
unsafe fn io_zcrx_unmap_areas(ifq: *mut io_zcrx_ifq) {
    rust_zcrx_assert_mutex(addr_of_mut!((*ifq).pp_lock));
    for i in 0..(*ifq).nr_areas { __io_zcrx_unmap_area(ifq, *(*ifq).areas.add(i as usize)); }
}
unsafe fn zcrx_sync_for_device(pp: *mut page_pool, z: *mut io_zcrx_ifq,
    netmems: *mut netmem_ref, nr: c_uint) {
    #[cfg(all(CONFIG_HAS_DMA, CONFIG_DMA_NEED_SYNC))] {
        let dev = (*pp).p.dev;
        if !rust_zcrx_dma_need_sync(dev) { return; }
        let size = 1u32 << (*z).niov_shift;
        for i in 0..nr {
            let dma = rust_zcrx_netmem_dma(*netmems.add(i as usize));
            rust_zcrx_dma_sync(dev, dma.wrapping_add((*pp).p.offset as dma_addr_t), size as usize, (*pp).p.dma_dir);
        }
    }
}
unsafe fn io_get_user_counter(n: *mut net_iov) -> *mut atomic_t {
    (*io_zcrx_iov_to_area(n)).user_refs.add(rust_zcrx_niov_idx(n) as usize)
}
unsafe fn io_zcrx_put_niov_uref(n: *mut net_iov, refs: c_uint) -> bool {
    let p = io_get_user_counter(n);
    let mut old = rust_zcrx_atomic_read(p);
    loop {
        // C compares int with unsigned; preserve that conversion.
        if (old as c_uint) < refs { return false; }
        let new = (old as c_uint).wrapping_sub(refs) as c_int;
        if rust_zcrx_atomic_try_cmpxchg(p, addr_of_mut!(old), new) { return true; }
    }
}
unsafe fn io_zcrx_get_niov_uref(n: *mut net_iov) { rust_zcrx_atomic_inc(io_get_user_counter(n)); }
unsafe fn io_fill_zcrx_offsets(off: *mut io_uring_zcrx_offsets) {
    (*off).head = RUST_ZCRX_HEAD_OFFSET;
    (*off).tail = RUST_ZCRX_TAIL_OFFSET;
    (*off).rqes = RUST_ZCRX_RQES_OFFSET;
}
unsafe fn io_allocate_rbuf_ring(ctx: *mut io_ring_ctx, ifq: *mut io_zcrx_ifq,
    reg: *mut io_uring_zcrx_ifq_reg, rd: *mut io_uring_region_desc, id: u32) -> c_int {
    io_fill_zcrx_offsets(addr_of_mut!((*reg).offsets));
    let off = (*reg).offsets.rqes as usize;
    let size = off + size_of::<io_uring_zcrx_rqe>() * (*reg).rq_entries as usize;
    if size as u64 > (*rd).size { return neg(EINVAL); }
    let mmap_offset = (IORING_MAP_OFF_ZCRX_REGION as u64).wrapping_add((id as u64) << IORING_OFF_ZCRX_SHIFT);
    let ret = io_create_region(ctx, addr_of_mut!((*ifq).rq_region), rd, mmap_offset as c_ulong);
    if ret < 0 { return ret; }
    let ptr = rust_zcrx_region_ptr(addr_of_mut!((*ifq).rq_region));
    (*ifq).rq.ring = ptr.cast();
    (*ifq).rq.rqes = ptr.cast::<u8>().add(off).cast();
    core::ptr::write_bytes((*ifq).rq.ring, 0, 1);
    0
}
unsafe fn io_free_rbuf_ring(ifq: *mut io_zcrx_ifq) {
    io_free_region((*ifq).user, addr_of_mut!((*ifq).rq_region));
    (*ifq).rq.ring = poison(); (*ifq).rq.rqes = poison(); (*ifq).notif_stats = poison();
}
unsafe fn io_zcrx_free_area(ifq: *mut io_zcrx_ifq, a: *mut io_zcrx_area) {
    if rust_zcrx_warn_once_03((*a).is_mapped) { return; }
    io_release_area_mem(addr_of_mut!((*a).mem));
    if (*a).mem.account_pages != 0 { io_unaccount_mem((*ifq).user, (*ifq).mm_account, (*a).mem.account_pages); }
    kvfree((*a).freelist.cast()); kvfree((*a).nia.niovs.cast()); kvfree((*a).user_refs.cast()); kfree(a.cast());
}
unsafe fn io_zcrx_append_area(ifq: *mut io_zcrx_ifq, a: *mut io_zcrx_area) -> c_int {
    if (*ifq).kern_readable != !(*a).mem.is_dmabuf || (*ifq).nr_areas + 1 > ZCRX_MAX_AREAS { return neg(EINVAL); }
    let old = (*ifq).areas;
    let nr = (*ifq).nr_areas;
    let areas: *mut *mut io_zcrx_area = rust_zcrx_kmalloc_array((nr + 1) as usize, size_of::<*mut io_zcrx_area>()).cast();
    if areas.is_null() { return neg(ENOMEM); }
    if !old.is_null() { core::ptr::copy_nonoverlapping(old, areas, nr as usize); }
    *areas.add(nr as usize) = a;
    {
        let _rq = BhLock::new(addr_of_mut!((*ifq).rq.lock));
        let _alloc = BhLock::new(addr_of_mut!((*ifq).alloc_lock));
        (*ifq).areas = areas; (*ifq).nr_areas = nr + 1;
    }
    kfree(old.cast());
    0
}
unsafe fn __zcrx_create_area(ifq: *mut io_zcrx_ifq, reg: *mut io_uring_zcrx_area_reg,
    rx_buf_len: u32) -> c_int {
    rust_zcrx_assert_mutex(addr_of_mut!((*ifq).pp_lock));
    let mut shift = RUST_ZCRX_PAGE_SHIFT;
    if rx_buf_len != 0 {
        if !rx_buf_len.is_power_of_two() || rx_buf_len < RUST_ZCRX_PAGE_SIZE { return neg(EINVAL); }
        shift = rx_buf_len.trailing_zeros();
    }
    if (*ifq).niov_shift != 0 && (*ifq).niov_shift != shift { return neg(EINVAL); }
    if (*ifq).dev.is_null() && shift != RUST_ZCRX_PAGE_SHIFT { return neg(EOPNOTSUPP); }
    let a: *mut io_zcrx_area = rust_zcrx_kzalloc(size_of::<io_zcrx_area>()).cast();
    if a.is_null() { return neg(ENOMEM); }
    (*a).ifq = ifq;
    let ret = 'area: {
        let ret = io_import_area(ifq, addr_of_mut!((*a).mem), reg);
        if ret != 0 { break 'area ret; }
        if !(*ifq).dev.is_null() { (*a).is_mapped = true; }
        if !(*ifq).dev.is_null() && shift as c_int > io_area_max_shift(addr_of_mut!((*a).mem)) { break 'area neg(ERANGE); }
        (*ifq).niov_shift = shift;
        let nr = ((*a).mem.size >> shift) as c_uint;
        (*a).nia.num_niovs = nr as usize;
        (*a).nia.niovs = rust_zcrx_kvmalloc_array(nr as usize, size_of::<net_iov>()).cast();
        if (*a).nia.niovs.is_null() { break 'area neg(ENOMEM); }
        (*a).freelist = rust_zcrx_kvmalloc_array(nr as usize, size_of::<u32>()).cast();
        if (*a).freelist.is_null() { break 'area neg(ENOMEM); }
        (*a).user_refs = rust_zcrx_kvmalloc_array(nr as usize, size_of::<atomic_t>()).cast();
        if (*a).user_refs.is_null() { break 'area neg(ENOMEM); }
        for i in 0..nr {
            rust_zcrx_niov_init((*a).nia.niovs.add(i as usize), addr_of_mut!((*a).nia));
            *(*a).freelist.add(i as usize) = i;
            rust_zcrx_atomic_set((*a).user_refs.add(i as usize), 0);
        }
        if !(*ifq).dev.is_null() {
            let ret = io_populate_area_dma(ifq, a);
            if ret != 0 { break 'area ret; }
        }
        (*a).free_count = nr;
        (*a).area_id = zcrx_next_area_id(ifq) as u16;
        (*reg).rq_area_token = zcrx_area_id_to_token((*a).area_id as u32);
        let ret = io_zcrx_append_area(ifq, a);
        if ret == 0 { return 0; }
        ret
    };
    __io_zcrx_unmap_area(ifq, a);
    io_zcrx_free_area(ifq, a);
    ret
}
unsafe fn io_zcrx_create_area(ifq: *mut io_zcrx_ifq, a: *mut io_uring_zcrx_area_reg,
    reg: *mut io_uring_zcrx_ifq_reg) -> c_int {
    let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock));
    __zcrx_create_area(ifq, a, (*reg).rx_buf_len)
}
unsafe fn io_zcrx_ifq_alloc(_ctx: *mut io_ring_ctx) -> *mut io_zcrx_ifq {
    let p: *mut io_zcrx_ifq = rust_zcrx_kzalloc(size_of::<io_zcrx_ifq>()).cast();
    if p.is_null() { return p; }
    (*p).if_rxq = u32::MAX;
    rust_zcrx_spin_init_ctx(addr_of_mut!((*p).ctx_lock));
    rust_zcrx_spin_init_rq(addr_of_mut!((*p).rq.lock));
    rust_zcrx_spin_init_alloc(addr_of_mut!((*p).alloc_lock));
    rust_zcrx_mutex_init(addr_of_mut!((*p).pp_lock));
    rust_zcrx_ref_set(addr_of_mut!((*p).refs), 1);
    rust_zcrx_ref_set(addr_of_mut!((*p).user_refs), 1);
    p
}
unsafe fn io_zcrx_drop_netdev(ifq: *mut io_zcrx_ifq) {
    rust_zcrx_assert_mutex(addr_of_mut!((*ifq).pp_lock));
    if (*ifq).netdev.is_null() { return; }
    rust_zcrx_netdev_put((*ifq).netdev, addr_of_mut!((*ifq).netdev_tracker));
    (*ifq).netdev = null_mut();
}
unsafe fn io_close_queue(ifq: *mut io_zcrx_ifq) {
    let netdev;
    let mut tracker;
    let mut p: pp_memory_provider_params = zeroed();
    p.mp_ops = addr_of!(rust_zcrx_mp_ops); p.mp_priv = ifq.cast();
    {
        let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock));
        netdev = (*ifq).netdev;
        tracker = core::ptr::read(addr_of!((*ifq).netdev_tracker));
        (*ifq).netdev = null_mut();
    }
    if !netdev.is_null() {
        rust_zcrx_netdev_lock(netdev);
        if (*ifq).if_rxq != u32::MAX { netif_mp_close_rxq(netdev, (*ifq).if_rxq, addr_of_mut!(p)); }
        { let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock)); io_zcrx_unmap_areas(ifq); }
        rust_zcrx_netdev_unlock(netdev);
        rust_zcrx_netdev_put(netdev, addr_of_mut!(tracker));
    }
    (*ifq).if_rxq = u32::MAX;
}
unsafe fn io_zcrx_ifq_free(ifq: *mut io_zcrx_ifq) {
    if rust_zcrx_warn_once_04((*ifq).if_rxq != u32::MAX) { return; }
    if rust_zcrx_warn_once_05(!(*ifq).netdev.is_null()) { return; }
    if rust_zcrx_warn_once_06(!(*ifq).master_ctx.is_null()) { return; }
    for i in 0..(*ifq).nr_areas { io_zcrx_free_area(ifq, *(*ifq).areas.add(i as usize)); }
    if !(*ifq).mm_account.is_null() { rust_zcrx_mmdrop((*ifq).mm_account); }
    if !(*ifq).dev.is_null() { put_device((*ifq).dev); }
    io_free_rbuf_ring(ifq);
    free_uid((*ifq).user);
    rust_zcrx_mutex_destroy(addr_of_mut!((*ifq).pp_lock));
    kfree((*ifq).areas.cast()); kfree(ifq.cast());
}
unsafe fn io_put_zcrx_ifq(ifq: *mut io_zcrx_ifq) {
    if rust_zcrx_ref_dec_and_test(addr_of_mut!((*ifq).refs)) { io_zcrx_ifq_free(ifq); }
}
unsafe fn io_zcrx_return_niov_freelist(n: *mut net_iov) {
    let a = io_zcrx_iov_to_area(n);
    let _lock = BhLock::new(addr_of_mut!((*(*a).ifq).alloc_lock));
    if rust_zcrx_warn_once_07((*a).free_count as usize >= (*a).nia.num_niovs) { return; }
    *(*a).freelist.add((*a).free_count as usize) = rust_zcrx_niov_idx(n);
    (*a).free_count += 1;
}
unsafe fn zcrx_get_free_niov(a: *mut io_zcrx_area) -> *mut net_iov {
    rust_zcrx_assert_spin(addr_of_mut!((*(*a).ifq).alloc_lock));
    if (*a).free_count == 0 { return null_mut(); }
    (*a).free_count -= 1;
    let idx = *(*a).freelist.add((*a).free_count as usize);
    (*a).nia.niovs.add(idx as usize)
}
unsafe fn io_zcrx_return_niov(n: *mut net_iov) {
    let nm = rust_zcrx_niov_to_netmem(n);
    if (*n).desc.pp.is_null() { io_zcrx_return_niov_freelist(n); return; }
    rust_zcrx_pp_put_unrefed((*n).desc.pp, nm);
}
unsafe fn io_zcrx_scrub_area(_ifq: *mut io_zcrx_ifq, a: *mut io_zcrx_area) {
    for i in 0..(*a).nia.num_niovs {
        let n = (*a).nia.niovs.add(i as usize);
        if rust_zcrx_atomic_read(io_get_user_counter(n)) == 0 { continue; }
        let nr = rust_zcrx_atomic_xchg(io_get_user_counter(n), 0);
        if nr != 0 && rust_zcrx_pp_unref(rust_zcrx_niov_to_netmem(n), nr as _) == 0 { io_zcrx_return_niov(n); }
    }
}
unsafe fn io_zcrx_scrub(ifq: *mut io_zcrx_ifq) {
    let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock));
    for i in 0..(*ifq).nr_areas { io_zcrx_scrub_area(ifq, *(*ifq).areas.add(i as usize)); }
}
unsafe fn zcrx_unregister_user(ifq: *mut io_zcrx_ifq, ctx: *mut io_ring_ctx) {
    {
        let _lock = BhLock::new(addr_of_mut!((*ifq).ctx_lock));
        if !ctx.is_null() && (*ifq).master_ctx == ctx {
            (*ifq).master_ctx = null_mut();
            rust_zcrx_percpu_put(rust_zcrx_ctx_refs(ctx));
        }
    }
    if rust_zcrx_ref_dec_and_test(addr_of_mut!((*ifq).user_refs)) { io_close_queue(ifq); io_zcrx_scrub(ifq); }
}
unsafe fn zcrx_unregister(ifq: *mut io_zcrx_ifq, ctx: *mut io_ring_ctx) {
    zcrx_unregister_user(ifq, ctx); io_put_zcrx_ifq(ifq);
}
#[no_mangle]
pub unsafe extern "C" fn io_zcrx_get_region(ctx: *mut io_ring_ctx, id: c_uint) -> *mut io_mapped_region {
    let ifq: *mut io_zcrx_ifq = xa_load(addr_of_mut!((*ctx).zcrx_ctxs), id as c_ulong).cast();
    rust_zcrx_assert_mutex(addr_of_mut!((*ctx).mmap_lock));
    if ifq.is_null() { null_mut() } else { addr_of_mut!((*ifq).rq_region) }
}
#[export_name = "rust_zcrx_box_release"]
unsafe extern "C" fn zcrx_box_release(_inode: *mut inode, file: *mut file) -> c_int {
    let ifq: *mut io_zcrx_ifq = (*file).private_data.cast();
    if rust_zcrx_warn_once_08(ifq.is_null()) { return neg(EFAULT); }
    zcrx_unregister(ifq, null_mut()); 0
}
unsafe fn zcrx_export(_ctx: *mut io_ring_ctx, ifq: *mut io_zcrx_ifq, ctrl: *mut zcrx_ctrl,
    arg: *mut c_void) -> c_int {
    let ce = rust_zcrx_ctrl_export(ctrl);
    if !rust_zcrx_mem_is_zero(ce.cast(), size_of::<zcrx_ctrl_export>()) { return neg(EINVAL); }
    rust_zcrx_ref_inc(addr_of_mut!((*ifq).refs)); rust_zcrx_ref_inc(addr_of_mut!((*ifq).user_refs));
    let file = anon_inode_create_getfile(c"[zcrx]".as_ptr().cast(), addr_of!(rust_zcrx_box_fops), ifq.cast(), O_CLOEXEC as c_int, null_mut());
    if is_err(file) { zcrx_unregister(ifq, null_mut()); return ptr_err(file); }
    let fd = get_unused_fd_flags(O_CLOEXEC);
    if fd < 0 { fput(file); return fd; }
    (*ce).zcrx_fd = fd as u32;
    if rust_zcrx_copy_to_user(arg, ctrl.cast(), size_of::<zcrx_ctrl>()) != 0 {
        fput(file); put_unused_fd(fd as c_uint); return neg(EFAULT);
    }
    fd_install(fd as c_uint, file); 0
}
unsafe fn import_zcrx(ctx: *mut io_ring_ctx, arg: *mut io_uring_zcrx_ifq_reg,
    reg: *mut io_uring_zcrx_ifq_reg) -> c_int {
    if rust_zcrx_ctx_flags(ctx) & IORING_SETUP_DEFER_TASKRUN == 0
        || rust_zcrx_ctx_flags(ctx) & (IORING_SETUP_CQE32 | IORING_SETUP_CQE_MIXED) == 0 { return neg(EINVAL); }
    if (*reg).if_rxq != 0 || (*reg).rq_entries != 0 || (*reg).area_ptr != 0
        || (*reg).region_ptr != 0 || (*reg).event_desc != 0 || (*reg).flags & !ZCRX_REG_IMPORT != 0 { return neg(EINVAL); }
    let fdg = FdGuard(rust_zcrx_fdget((*reg).if_idx));
    let file = rust_zcrx_fd_file(fdg.0);
    if file.is_null() || (*file).f_op != addr_of!(rust_zcrx_box_fops) || (*file).private_data.is_null() { return neg(EBADF); }
    let ifq: *mut io_zcrx_ifq = (*file).private_data.cast();
    rust_zcrx_ref_inc(addr_of_mut!((*ifq).refs)); rust_zcrx_ref_inc(addr_of_mut!((*ifq).user_refs));
    let mut id = 0;
    let ret = {
        let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
        rust_zcrx_xa_alloc(addr_of_mut!((*ctx).zcrx_ctxs), addr_of_mut!(id))
    };
    if ret != 0 { zcrx_unregister(ifq, null_mut()); return ret; }
    (*reg).zcrx_id = id;
    io_fill_zcrx_offsets(addr_of_mut!((*reg).offsets));
    let ret = if rust_zcrx_copy_to_user(arg.cast(), reg.cast(), size_of::<io_uring_zcrx_ifq_reg>()) != 0 {
        neg(EFAULT)
    } else {
        let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
        if rust_zcrx_xa_store(addr_of_mut!((*ctx).zcrx_ctxs), id as c_ulong, ifq.cast()).is_null() { return 0; }
        neg(ENOMEM)
    };
    { let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock)); xa_erase(addr_of_mut!((*ctx).zcrx_ctxs), id as c_ulong); }
    zcrx_unregister(ifq, null_mut());
    ret
}
unsafe fn zcrx_register_netdev(ifq: *mut io_zcrx_ifq, reg: *mut io_uring_zcrx_ifq_reg,
    area: *mut io_uring_zcrx_area_reg) -> c_int {
    let mut p: pp_memory_provider_params = zeroed();
    let q = (*reg).if_rxq;
    (*ifq).netdev = netdev_get_by_index_lock(rust_zcrx_current_net(), (*reg).if_idx as c_int);
    if (*ifq).netdev.is_null() { return neg(ENODEV); }
    rust_zcrx_netdev_hold((*ifq).netdev, addr_of_mut!((*ifq).netdev_tracker));
    let ret = 'device: {
        (*ifq).dev = netdev_queue_get_dma_dev((*ifq).netdev, q, NETDEV_QUEUE_TYPE_RX);
        if (*ifq).dev.is_null() { break 'device neg(EOPNOTSUPP); }
        get_device((*ifq).dev);
        let ret = io_zcrx_create_area(ifq, area, reg);
        if ret != 0 { break 'device ret; }
        if (*reg).rx_buf_len != 0 { p.rx_page_size = 1u32 << (*ifq).niov_shift; }
        p.mp_ops = addr_of!(rust_zcrx_mp_ops); p.mp_priv = ifq.cast();
        let ret = netif_mp_open_rxq((*ifq).netdev, q, addr_of_mut!(p), null_mut());
        if ret != 0 { break 'device ret; }
        (*ifq).if_rxq = q;
        0
    };
    rust_zcrx_netdev_unlock((*ifq).netdev);
    ret
}
unsafe fn zcrx_validate_notif_stats(ifq: *mut io_zcrx_ifq, reg: *const io_uring_zcrx_ifq_reg,
    n: *const zcrx_event_desc) -> c_int {
    let off = (*n).stats_offset as usize;
    let used = (*reg).offsets.rqes as usize + size_of::<io_uring_zcrx_rqe>() * (*reg).rq_entries as usize;
    if off % align_of::<zcrx_stats>() != 0 { return neg(EINVAL); }
    if off < used { return neg(ERANGE); }
    let Some(end) = off.checked_add(size_of::<zcrx_stats>()) else { return neg(ERANGE); };
    if end > rust_zcrx_region_size(addr_of_mut!((*ifq).rq_region)) { return neg(ERANGE); }
    (*ifq).notif_stats = rust_zcrx_region_ptr(addr_of_mut!((*ifq).rq_region)).cast::<u8>().add(off).cast();
    core::ptr::write_bytes((*ifq).notif_stats, 0, 1);
    0
}
#[no_mangle]
pub unsafe extern "C" fn io_register_zcrx(ctx: *mut io_ring_ctx, arg: *mut io_uring_zcrx_ifq_reg) -> c_int {
    if !rust_zcrx_capable(CAP_NET_ADMIN as c_int) { return neg(EPERM); }
    if rust_zcrx_ctx_flags(ctx) & IORING_SETUP_DEFER_TASKRUN == 0
        || rust_zcrx_ctx_flags(ctx) & (IORING_SETUP_CQE32 | IORING_SETUP_CQE_MIXED) == 0 { return neg(EINVAL); }
    let mut reg: io_uring_zcrx_ifq_reg = zeroed();
    let mut rd: io_uring_region_desc = zeroed();
    let mut area: io_uring_zcrx_area_reg = zeroed();
    let mut notif: zcrx_event_desc = zeroed();
    if rust_zcrx_copy_from_user(addr_of_mut!(reg).cast(), arg.cast(), size_of_val(&reg)) != 0 { return neg(EFAULT); }
    if !rust_zcrx_mem_is_zero(addr_of!(reg.__resv).cast(), size_of_val(&reg.__resv)) || reg.zcrx_id != 0 { return neg(EINVAL); }
    if reg.flags & !RUST_ZCRX_SUPPORTED_REG_FLAGS != 0 { return neg(EINVAL); }
    if reg.flags & ZCRX_REG_IMPORT != 0 { return import_zcrx(ctx, arg, addr_of_mut!(reg)); }
    if rust_zcrx_copy_from_user(addr_of_mut!(rd).cast(), reg.region_ptr as usize as *const c_void, size_of_val(&rd)) != 0 { return neg(EFAULT); }
    if reg.if_rxq == u32::MAX || reg.rq_entries == 0 { return neg(EINVAL); }
    if (reg.if_rxq != 0 || reg.if_idx != 0) && reg.flags & ZCRX_REG_NODEV != 0 { return neg(EINVAL); }
    if reg.rq_entries > IO_RQ_MAX_ENTRIES {
        if rust_zcrx_ctx_flags(ctx) & IORING_SETUP_CLAMP == 0 { return neg(EINVAL); }
        reg.rq_entries = IO_RQ_MAX_ENTRIES;
    }
    reg.rq_entries = reg.rq_entries.next_power_of_two();
    if rust_zcrx_copy_from_user(addr_of_mut!(area).cast(), reg.area_ptr as usize as *const c_void, size_of_val(&area)) != 0 { return neg(EFAULT); }
    if area.rq_area_token != 0 { return neg(EINVAL); }
    if reg.event_desc != 0 && rust_zcrx_copy_from_user(addr_of_mut!(notif).cast(), reg.event_desc as usize as *const c_void, size_of_val(&notif)) != 0 { return neg(EFAULT); }
    if notif.type_mask & !RUST_ZCRX_EVENT_TYPE_MASK != 0 || notif.flags & !ZCRX_EVENT_DESC_FLAG_STATS != 0 { return neg(EINVAL); }
    if notif.flags & ZCRX_EVENT_DESC_FLAG_STATS == 0 && notif.stats_offset != 0 { return neg(EINVAL); }
    if !rust_zcrx_mem_is_zero(addr_of!(notif.__resv2).cast(), size_of_val(&notif.__resv2)) { return neg(EINVAL); }
    let ifq = io_zcrx_ifq_alloc(ctx);
    if ifq.is_null() { return neg(ENOMEM); }
    (*ifq).notif_data = notif.user_data; (*ifq).allowed_notif_mask = notif.type_mask;
    if !(*ctx).user.is_null() { rust_zcrx_get_uid((*ctx).user); (*ifq).user = (*ctx).user; }
    if !(*ctx).mm_account.is_null() { rust_zcrx_mmgrab((*ctx).mm_account); (*ifq).mm_account = (*ctx).mm_account; }
    (*ifq).rq.nr_entries = reg.rq_entries;
    let mut id = 0;
    let ret = {
        let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
        rust_zcrx_xa_alloc(addr_of_mut!((*ctx).zcrx_ctxs), addr_of_mut!(id))
    };
    if ret != 0 { zcrx_unregister(ifq, ctx); return ret; }
    let ret = 'register: {
        let ret = io_allocate_rbuf_ring(ctx, ifq, addr_of_mut!(reg), addr_of_mut!(rd), id);
        if ret != 0 { break 'register ret; }
        if notif.flags & ZCRX_EVENT_DESC_FLAG_STATS != 0 {
            let ret = zcrx_validate_notif_stats(ifq, addr_of!(reg), addr_of!(notif));
            if ret != 0 { break 'register ret; }
        }
        (*ifq).kern_readable = area.flags & IORING_ZCRX_AREA_DMABUF == 0;
        let ret = if reg.flags & ZCRX_REG_NODEV == 0 { zcrx_register_netdev(ifq, addr_of_mut!(reg), addr_of_mut!(area)) }
            else { io_zcrx_create_area(ifq, addr_of_mut!(area), addr_of_mut!(reg)) };
        if ret != 0 { break 'register ret; }
        rust_zcrx_warn_once_09((*ifq).niov_shift == 0);
        reg.zcrx_id = id;
        {
            let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
            if !rust_zcrx_xa_store(addr_of_mut!((*ctx).zcrx_ctxs), id as c_ulong, ifq.cast()).is_null() { break 'register neg(ENOMEM); }
        }
        reg.rx_buf_len = 1u32 << (*ifq).niov_shift;
        if rust_zcrx_copy_to_user(arg.cast(), addr_of!(reg).cast(), size_of_val(&reg)) != 0
            || rust_zcrx_copy_to_user(reg.region_ptr as usize as *mut c_void, addr_of!(rd).cast(), size_of_val(&rd)) != 0
            || rust_zcrx_copy_to_user(reg.area_ptr as usize as *mut c_void, addr_of!(area).cast(), size_of_val(&area)) != 0 { break 'register neg(EFAULT); }
        if notif.type_mask != 0 { zcrx_set_ring_ctx(ifq, ctx); }
        return 0;
    };
    { let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock)); xa_erase(addr_of_mut!((*ctx).zcrx_ctxs), id as c_ulong); }
    zcrx_unregister(ifq, ctx);
    ret
}
unsafe fn is_zcrx_entry_marked(ctx: *mut io_ring_ctx, id: c_ulong) -> bool { rust_zcrx_xa_get_mark(addr_of_mut!((*ctx).zcrx_ctxs), id) }
unsafe fn set_zcrx_entry_mark(ctx: *mut io_ring_ctx, id: c_ulong) { rust_zcrx_xa_set_mark(addr_of_mut!((*ctx).zcrx_ctxs), id); }
#[no_mangle]
pub unsafe extern "C" fn io_terminate_zcrx(ctx: *mut io_ring_ctx) {
    rust_zcrx_assert_mutex(rust_zcrx_ctx_uring_lock(ctx));
    let mut id: c_ulong = 0;
    loop {
        let ifq: *mut io_zcrx_ifq = {
            let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
            rust_zcrx_xa_find(addr_of_mut!((*ctx).zcrx_ctxs), addr_of_mut!(id)).cast()
        };
        if ifq.is_null() { break; }
        if rust_zcrx_warn_once_10(is_zcrx_entry_marked(ctx, id)) { break; }
        set_zcrx_entry_mark(ctx, id);
        id = id.wrapping_add(1);
        zcrx_unregister_user(ifq, ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn io_unregister_zcrx(ctx: *mut io_ring_ctx) {
    rust_zcrx_assert_mutex(rust_zcrx_ctx_uring_lock(ctx));
    loop {
        let ifq: *mut io_zcrx_ifq;
        {
            let _lock = MutexLock::new(addr_of_mut!((*ctx).mmap_lock));
            let mut id = 0;
            ifq = rust_zcrx_xa_find(addr_of_mut!((*ctx).zcrx_ctxs), addr_of_mut!(id)).cast();
            if !ifq.is_null() {
                if rust_zcrx_warn_once_11(!is_zcrx_entry_marked(ctx, id)) { break; }
                xa_erase(addr_of_mut!((*ctx).zcrx_ctxs), id);
            }
        }
        if ifq.is_null() { break; }
        if rust_zcrx_ref_read(addr_of!((*ifq).user_refs)) == 0 { io_zcrx_scrub(ifq); }
        io_put_zcrx_ifq(ifq);
    }
    xa_destroy(addr_of_mut!((*ctx).zcrx_ctxs));
}
struct zcrx_rq_iter { rqes_left: c_int, flushed: bool }
unsafe fn __zcrx_rq_entries(rq: *mut zcrx_rq) -> u32 { (*rq).cached_tail.wrapping_sub((*rq).cached_head).min((*rq).nr_entries) }
unsafe fn zcrx_rq_entries(rq: *mut zcrx_rq) -> u32 {
    (*rq).cached_tail = rust_zcrx_load_acquire(addr_of!((*(*rq).ring).tail));
    __zcrx_rq_entries(rq)
}
unsafe fn zcrx_next_rqe(rq: *mut zcrx_rq, mask: c_uint) -> *mut io_uring_zcrx_rqe {
    let idx = (*rq).cached_head & mask;
    (*rq).cached_head = (*rq).cached_head.wrapping_add(1);
    (*rq).rqes.add(idx as usize)
}
unsafe fn zcrx_rq_iter_init(it: &mut zcrx_rq_iter, rq: *mut zcrx_rq) {
    it.rqes_left = __zcrx_rq_entries(rq).min(RUST_ZCRX_REFILL_CAP) as c_int; it.flushed = false;
}
unsafe fn zcrx_rq_iter_next(it: &mut zcrx_rq_iter, rq: *mut zcrx_rq, rqe: &mut *mut io_uring_zcrx_rqe) -> bool {
    it.rqes_left -= 1;
    if it.rqes_left < 0 {
        if it.flushed { return false; }
        (*rq).cached_tail = rust_zcrx_load_acquire(addr_of!((*(*rq).ring).tail));
        it.rqes_left = __zcrx_rq_entries(rq).min(RUST_ZCRX_REFILL_CAP) as c_int;
        it.flushed = true; it.rqes_left -= 1;
        if it.rqes_left < 0 { return false; }
    }
    *rqe = zcrx_next_rqe(rq, (*rq).nr_entries - 1); true
}
unsafe fn io_parse_rqe(rqe: *mut io_uring_zcrx_rqe, ifq: *mut io_zcrx_ifq, n: &mut *mut net_iov) -> bool {
    let off = rust_zcrx_read_once_u64(addr_of!((*rqe).off));
    rust_zcrx_assert_spin(addr_of_mut!((*ifq).rq.lock));
    let mut ai = (off >> IORING_ZCRX_AREA_SHIFT) as c_uint;
    let mut ni = ((off & !RUST_ZCRX_AREA_MASK) >> (*ifq).niov_shift) as c_uint;
    if (*rqe).__pad != 0 || ai >= (*ifq).nr_areas { return false; }
    ai = rust_zcrx_array_index_nospec(ai as c_ulong, (*ifq).nr_areas as c_ulong) as c_uint;
    let a = *(*ifq).areas.add(ai as usize);
    if ni as usize >= (*a).nia.num_niovs { return false; }
    ni = rust_zcrx_array_index_nospec(ni as c_ulong, (*a).nia.num_niovs as c_ulong) as c_uint;
    *n = (*a).nia.niovs.add(ni as usize); true
}
unsafe fn zcrx_put_refill_niov(n: *mut net_iov, pp: *mut page_pool, refs: c_uint) -> bool {
    let nm = rust_zcrx_niov_to_netmem(n);
    if !io_zcrx_put_niov_uref(n, refs) { return false; }
    if rust_zcrx_pp_unref(nm, refs as _) != 0 { return false; }
    if (*n).desc.pp != pp { io_zcrx_return_niov(n); return false; }
    true
}
unsafe fn io_zcrx_ring_refill(pp: *mut page_pool, ifq: *mut io_zcrx_ifq,
    netmems: *mut netmem_ref, to_alloc: c_uint) -> c_uint {
    let rq = addr_of_mut!((*ifq).rq);
    let _lock = BhLock::new(addr_of_mut!((*rq).lock));
    let mut it = zcrx_rq_iter { rqes_left: 0, flushed: false };
    zcrx_rq_iter_init(&mut it, rq);
    let mut n = null_mut(); let mut refs = 0; let mut allocated = 0; let mut rqe = null_mut();
    while allocated < to_alloc.wrapping_sub(1) && zcrx_rq_iter_next(&mut it, rq, &mut rqe) {
        let mut next = null_mut();
        if !io_parse_rqe(rqe, ifq, &mut next) { continue; }
        if n == next { refs += 1; continue; }
        if !n.is_null() && zcrx_put_refill_niov(n, pp, refs) {
            *netmems.add(allocated as usize) = rust_zcrx_niov_to_netmem(n); allocated += 1;
        }
        n = next; refs = 1;
    }
    if !n.is_null() && zcrx_put_refill_niov(n, pp, refs) {
        *netmems.add(allocated as usize) = rust_zcrx_niov_to_netmem(n); allocated += 1;
    }
    rust_zcrx_store_release(addr_of_mut!((*(*rq).ring).head), (*rq).cached_head);
    allocated
}
unsafe fn io_zcrx_refill_slow(pp: *mut page_pool, ifq: *mut io_zcrx_ifq,
    netmems: *mut netmem_ref, to_alloc: c_uint) -> c_uint {
    let _lock = BhLock::new(addr_of_mut!((*ifq).alloc_lock));
    let mut ai = 0; let mut allocated = 0;
    while allocated < to_alloc {
        let n = zcrx_get_free_niov(*(*ifq).areas.add(ai as usize));
        if n.is_null() {
            ai += 1; if ai >= (*ifq).nr_areas { break; } continue;
        }
        net_mp_niov_set_page_pool(pp, n);
        *netmems.add(allocated as usize) = rust_zcrx_niov_to_netmem(n); allocated += 1;
    }
    allocated
}
unsafe extern "C" fn zcrx_notif_tw(tw_req: io_tw_req, _tw: io_tw_token_t) {
    let req = tw_req.req; let ctx = (*req).ctx;
    io_post_aux_cqe(ctx, (*req).cqe.user_data, (*req).cqe.res, 0);
    rust_zcrx_percpu_put(rust_zcrx_ctx_refs(ctx));
    io_poison_req(req); kmem_cache_free(req_cachep, req.cast());
}
unsafe fn zcrx_stat_add(p: *mut u64, v: i64) {
    rust_zcrx_write_once_u64(p, rust_zcrx_read_once_u64(p).wrapping_add(v as u64));
}
unsafe fn zcrx_send_notif(ifq: *mut io_zcrx_ifq, ty: c_uint) {
    let mask = 1u32 << ty;
    if mask & (*ifq).allowed_notif_mask == 0 { return; }
    let _lock = BhLock::new(addr_of_mut!((*ifq).ctx_lock));
    if (*ifq).master_ctx.is_null() || mask & (*ifq).fired_notifs != 0 { return; }
    let req: *mut io_kiocb = rust_zcrx_alloc_notif_req().cast();
    if req.is_null() { return; }
    (*ifq).fired_notifs |= mask;
    (*req).opcode = RUST_ZCRX_OP_NOP as u8;
    (*req).cqe.user_data = (*ifq).notif_data; (*req).cqe.res = ty as c_int;
    (*req).ctx = (*ifq).master_ctx;
    rust_zcrx_percpu_get(rust_zcrx_ctx_refs((*req).ctx));
    (*req).tctx = null_mut();
    rust_zcrx_set_task_work(req, Some(zcrx_notif_tw));
    rust_zcrx_task_work_add(req);
}
#[export_name = "rust_zcrx_pp_alloc_netmems"]
unsafe extern "C" fn io_pp_zc_alloc_netmems(pp: *mut page_pool, _gfp: gfp_t) -> netmem_ref {
    let ifq = io_pp_to_ifq(pp);
    let nms = addr_of_mut!((*pp).alloc.cache).cast::<netmem_ref>();
    if rust_zcrx_warn_once_12((*pp).alloc.count != 0) { return 0; }
    let mut allocated = io_zcrx_ring_refill(pp, ifq, nms, PP_ALLOC_CACHE_REFILL);
    if allocated == 0 {
        allocated = io_zcrx_refill_slow(pp, ifq, nms, PP_ALLOC_CACHE_REFILL);
        if allocated == 0 { zcrx_send_notif(ifq, ZCRX_EVENT_ALLOC_FAIL); return 0; }
    }
    zcrx_sync_for_device(pp, ifq, nms, allocated);
    allocated -= 1; (*pp).alloc.count += allocated;
    *nms.add(allocated as usize)
}
#[export_name = "rust_zcrx_pp_release_netmem"]
unsafe extern "C" fn io_pp_zc_release_netmem(_pp: *mut page_pool, nm: netmem_ref) -> bool {
    if rust_zcrx_warn_once_13(!rust_zcrx_netmem_is_niov(nm)) { return false; }
    let n = rust_zcrx_netmem_to_niov(nm);
    net_mp_niov_clear_page_pool(n); io_zcrx_return_niov_freelist(n); false
}
#[export_name = "rust_zcrx_pp_init"]
unsafe extern "C" fn io_pp_zc_init(pp: *mut page_pool) -> c_int {
    let ifq = io_pp_to_ifq(pp);
    if rust_zcrx_warn_once_14(ifq.is_null()) { return neg(EINVAL); }
    if rust_zcrx_warn_once_15((*ifq).dev != (*pp).p.dev) { return neg(EINVAL); }
    if rust_zcrx_warn_once_16(!rust_zcrx_pp_dma_map(pp)) { return neg(EOPNOTSUPP); }
    if (*pp).p.order + RUST_ZCRX_PAGE_SHIFT != (*ifq).niov_shift { return neg(EINVAL); }
    if (*pp).p.dma_dir != DMA_FROM_DEVICE { return neg(EOPNOTSUPP); }
    rust_zcrx_ref_inc(addr_of_mut!((*ifq).refs)); 0
}
#[export_name = "rust_zcrx_pp_destroy"]
unsafe extern "C" fn io_pp_zc_destroy(pp: *mut page_pool) { io_put_zcrx_ifq(io_pp_to_ifq(pp)); }
#[export_name = "rust_zcrx_pp_nl_fill"]
unsafe extern "C" fn io_pp_nl_fill(priv_: *mut c_void, rsp: *mut sk_buff, rxq: *mut netdev_rx_queue) -> c_int {
    let ifq: *mut io_zcrx_ifq = priv_.cast();
    let ty = if !rxq.is_null() { NETDEV_A_QUEUE_IO_URING } else { NETDEV_A_PAGE_POOL_IO_URING };
    let nest = rust_zcrx_nla_nest_start(rsp, ty as c_int);
    if nest.is_null() { return neg(EMSGSIZE); }
    if rust_zcrx_nla_put_uint(rsp, NETDEV_A_IO_URING_PROVIDER_INFO_RX_BUF_LEN as c_int, 1u64 << (*ifq).niov_shift) != 0 {
        rust_zcrx_nla_nest_cancel(rsp, nest); return neg(EMSGSIZE);
    }
    rust_zcrx_nla_nest_end(rsp, nest); 0
}
#[export_name = "rust_zcrx_pp_uninstall"]
unsafe extern "C" fn io_pp_uninstall(priv_: *mut c_void, rxq: *mut netdev_rx_queue) {
    let ifq: *mut io_zcrx_ifq = priv_.cast();
    let p = addr_of_mut!((*rxq).mp_params);
    let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock));
    io_zcrx_unmap_areas(ifq); io_zcrx_drop_netdev(ifq);
    (*p).mp_ops = core::ptr::null(); (*p).mp_priv = null_mut();
}
unsafe fn zcrx_parse_rq(nms: *mut netmem_ref, nr: c_uint, z: *mut io_zcrx_ifq, rq: *mut zcrx_rq) -> c_uint {
    let mask = (*rq).nr_entries - 1; let nr = nr.min(zcrx_rq_entries(rq));
    let mut i = 0;
    while i < nr {
        let rqe = zcrx_next_rqe(rq, mask); let mut n = null_mut();
        if !io_parse_rqe(rqe, z, &mut n) { break; }
        *nms.add(i as usize) = rust_zcrx_niov_to_netmem(n); i += 1;
    }
    rust_zcrx_store_release(addr_of_mut!((*(*rq).ring).head), (*rq).cached_head);
    i
}
unsafe fn zcrx_return_buffers(nms: *mut netmem_ref, nr: c_uint) {
    for i in 0..nr {
        let nm = *nms.add(i as usize); let n = rust_zcrx_netmem_to_niov(nm);
        if !io_zcrx_put_niov_uref(n, 1) { continue; }
        if !rust_zcrx_pp_unref_and_test(nm) { continue; }
        io_zcrx_return_niov(n);
    }
}
unsafe fn zcrx_flush_rq(_ctx: *mut io_ring_ctx, z: *mut io_zcrx_ifq, ctrl: *mut zcrx_ctrl) -> c_int {
    let f = rust_zcrx_ctrl_flush(ctrl);
    if !rust_zcrx_mem_is_zero(addr_of!((*f).__resv).cast(), size_of_val(&(*f).__resv)) { return neg(EINVAL); }
    let mut nms = [0 as netmem_ref; ZCRX_FLUSH_BATCH]; let mut total: c_uint = 0;
    loop {
        let rq = addr_of_mut!((*z).rq);
        let nr = {
            let _lock = BhLock::new(addr_of_mut!((*rq).lock));
            let nr = zcrx_parse_rq(nms.as_mut_ptr(), ZCRX_FLUSH_BATCH as c_uint, z, rq);
            zcrx_return_buffers(nms.as_mut_ptr(), nr); nr
        };
        total = total.wrapping_add(nr);
        if rust_zcrx_fatal_signal_pending() { break; }
        rust_zcrx_cond_resched();
        if nr != ZCRX_FLUSH_BATCH as c_uint || total >= (*z).rq.nr_entries { break; }
    }
    0
}
unsafe fn zcrx_arm_notif(_ctx: *mut io_ring_ctx, z: *mut io_zcrx_ifq, ctrl: *mut zcrx_ctrl) -> c_int {
    let a = rust_zcrx_ctrl_arm(ctrl);
    if (*a).event_type >= __ZCRX_EVENT_TYPE_LAST { return neg(EINVAL); }
    if !rust_zcrx_mem_is_zero(addr_of!((*a).__resv).cast(), size_of_val(&(*a).__resv)) { return neg(EINVAL); }
    let _lock = BhLock::new(addr_of_mut!((*z).ctx_lock));
    let mask = 1u32 << (*a).event_type;
    if mask & !(*z).fired_notifs != 0 { return neg(EINVAL); }
    (*z).fired_notifs &= !mask; 0
}
unsafe fn zcrx_ctrl_add_area(_ctx: *mut io_ring_ctx, ifq: *mut io_zcrx_ifq, ctrl: *mut zcrx_ctrl) -> c_int {
    let ca = rust_zcrx_ctrl_area(ctrl);
    let uptr = (*ca).area_ptr as usize as *mut io_uring_zcrx_area_reg;
    let mut reg: io_uring_zcrx_area_reg = zeroed();
    if rust_zcrx_copy_from_user(addr_of_mut!(reg).cast(), uptr.cast(), size_of_val(&reg)) != 0 { return neg(EFAULT); }
    if !rust_zcrx_mem_is_zero(addr_of!((*ca).__resv).cast(), size_of_val(&(*ca).__resv)) || reg.rq_area_token != 0 { return neg(EINVAL); }
    let _lock = MutexLock::new(addr_of_mut!((*ifq).pp_lock));
    if !(*ifq).dev.is_null() && (*ifq).netdev.is_null() { return neg(EFAULT); }
    // Appending an area cannot safely be rolled back: publish its token first.
    reg.rq_area_token = zcrx_area_id_to_token(zcrx_next_area_id(ifq));
    if rust_zcrx_copy_to_user(uptr.cast(), addr_of!(reg).cast(), size_of_val(&reg)) != 0 { return neg(EFAULT); }
    reg.rq_area_token = 0;
    __zcrx_create_area(ifq, addr_of_mut!(reg), 1u32 << (*ifq).niov_shift)
}
#[no_mangle]
pub unsafe extern "C" fn io_zcrx_ctrl(ctx: *mut io_ring_ctx, arg: *mut c_void, nr_args: c_uint) -> c_int {
    const { assert!(size_of::<zcrx_ctrl_export>() == size_of::<zcrx_ctrl_flush_rq>()); }
    const { assert!(size_of::<zcrx_ctrl_export>() == size_of::<zcrx_ctrl_arm_event>()); }
    if nr_args != 0 { return neg(EINVAL); }
    let mut ctrl: zcrx_ctrl = zeroed();
    if rust_zcrx_copy_from_user(addr_of_mut!(ctrl).cast(), arg, size_of_val(&ctrl)) != 0 { return neg(EFAULT); }
    if !rust_zcrx_mem_is_zero(addr_of!(ctrl.__resv).cast(), size_of_val(&ctrl.__resv)) { return neg(EFAULT); }
    let z: *mut io_zcrx_ifq = xa_load(addr_of_mut!((*ctx).zcrx_ctxs), ctrl.zcrx_id as c_ulong).cast();
    if z.is_null() { return neg(ENXIO); }
    match ctrl.op {
        ZCRX_CTRL_FLUSH_RQ => zcrx_flush_rq(ctx, z, addr_of_mut!(ctrl)),
        ZCRX_CTRL_EXPORT => zcrx_export(ctx, z, addr_of_mut!(ctrl), arg),
        ZCRX_CTRL_ARM_EVENT => zcrx_arm_notif(ctx, z, addr_of_mut!(ctrl)),
        ZCRX_CTRL_ADD_AREA => zcrx_ctrl_add_area(ctx, z, addr_of_mut!(ctrl)),
        _ => neg(EOPNOTSUPP),
    }
}
unsafe fn io_zcrx_queue_cqe(req: *mut io_kiocb, n: *mut net_iov, ifq: *mut io_zcrx_ifq,
    off: c_int, len: c_int) -> bool {
    let ctx = (*req).ctx; let mut cqe: *mut io_uring_cqe = null_mut();
    if !rust_zcrx_defer_get_cqe(ctx, addr_of_mut!(cqe)) { return false; }
    (*cqe).user_data = (*req).cqe.user_data; (*cqe).res = len; (*cqe).flags = IORING_CQE_F_MORE;
    if rust_zcrx_ctx_flags(ctx) & IORING_SETUP_CQE_MIXED != 0 { (*cqe).flags |= IORING_CQE_F_32; }
    let a = io_zcrx_iov_to_area(n);
    // Preserve the C unsigned-int shift/add before its conversion to u64.
    let offset = (off as c_uint).wrapping_add(rust_zcrx_niov_idx(n).wrapping_shl((*ifq).niov_shift)) as u64;
    let rcqe = cqe.add(1).cast::<io_uring_zcrx_cqe>();
    (*rcqe).off = offset.wrapping_add(zcrx_area_id_to_token((*a).area_id as u32));
    (*rcqe).__pad = 0; true
}
unsafe fn io_alloc_fallback_niov(ifq: *mut io_zcrx_ifq) -> *mut net_iov {
    if !(*ifq).kern_readable { return null_mut(); }
    let _lock = BhLock::new(addr_of_mut!((*ifq).alloc_lock));
    for i in 0..(*ifq).nr_areas {
        let n = zcrx_get_free_niov(*(*ifq).areas.add(i as usize));
        if !n.is_null() { rust_zcrx_pp_fragment(rust_zcrx_niov_to_netmem(n), 1); return n; }
    }
    null_mut()
}
struct io_copy_cache { page: *mut page, offset: c_ulong, size: usize }
unsafe fn io_copy_page(cc: &mut io_copy_cache, mut src_page: *mut page,
    mut src_offset: c_uint, len: usize) -> isize {
    let mut copied = 0; let mut len = len.min(cc.size);
    while len != 0 {
        let mut dst_page = cc.page; let mut dst_offset = cc.offset as c_uint; let mut n = len;
        if rust_zcrx_partial_kmap(rust_zcrx_page_folio(dst_page)) || rust_zcrx_partial_kmap(rust_zcrx_page_folio(src_page)) {
            dst_page = dst_page.add((dst_offset / RUST_ZCRX_PAGE_SIZE) as usize);
            dst_offset &= RUST_ZCRX_PAGE_SIZE - 1;
            src_page = src_page.add((src_offset / RUST_ZCRX_PAGE_SIZE) as usize);
            src_offset &= RUST_ZCRX_PAGE_SIZE - 1;
            n = (RUST_ZCRX_PAGE_SIZE - src_offset).min(RUST_ZCRX_PAGE_SIZE - dst_offset) as usize;
            n = n.min(len);
        }
        let dst_addr = rust_zcrx_kmap(dst_page).cast::<u8>().add(dst_offset as usize);
        let src_addr = rust_zcrx_kmap(src_page).cast::<u8>().add(src_offset as usize);
        core::ptr::copy_nonoverlapping(src_addr, dst_addr, n);
        rust_zcrx_kunmap(src_addr.cast()); rust_zcrx_kunmap(dst_addr.cast());
        cc.size -= n; cc.offset = cc.offset.wrapping_add(n as c_ulong);
        src_offset = src_offset.wrapping_add(n as c_uint); len -= n; copied += n;
    }
    copied as isize
}
unsafe fn io_zcrx_copy_chunk(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq, src_page: *mut page,
    mut src_offset: c_uint, mut len: usize) -> isize {
    let mut copied = 0; let mut ret = 0;
    while len != 0 {
        let n = io_alloc_fallback_niov(ifq);
        if n.is_null() { ret = neg(ENOMEM); break; }
        let mut cc = io_copy_cache { page: io_zcrx_iov_page(n), offset: 0, size: RUST_ZCRX_PAGE_SIZE as usize };
        let count = io_copy_page(&mut cc, src_page, src_offset, len) as usize;
        if !io_zcrx_queue_cqe(req, n, ifq, 0, count as c_int) {
            io_zcrx_return_niov(n); ret = neg(ENOSPC); break;
        }
        io_zcrx_get_niov_uref(n);
        src_offset = src_offset.wrapping_add(count as c_uint); len -= count; copied += count;
    }
    if copied != 0 { copied as isize } else { ret as isize }
}
unsafe fn io_zcrx_copy_frag(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq,
    frag: *const skb_frag_t, off: c_int, len: c_int) -> c_int {
    let page = rust_zcrx_frag_page(frag);
    let ret = io_zcrx_copy_chunk(req, ifq, page, (off as c_uint).wrapping_add(rust_zcrx_frag_off(frag)), len as usize) as c_int;
    if ret > 0 {
        if !(*ifq).notif_stats.is_null() {
            zcrx_stat_add(addr_of_mut!((*(*ifq).notif_stats).copy_count), 1);
            zcrx_stat_add(addr_of_mut!((*(*ifq).notif_stats).copy_bytes), ret as i64);
        }
        zcrx_send_notif(ifq, ZCRX_EVENT_COPY);
    }
    ret
}
unsafe fn io_zcrx_recv_frag(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq,
    frag: *const skb_frag_t, off: c_int, len: c_int) -> c_int {
    if !rust_zcrx_frag_is_niov(frag) { return io_zcrx_copy_frag(req, ifq, frag, off, len); }
    let n = rust_zcrx_netmem_to_niov((*frag).netmem); let pp = (*n).desc.pp;
    if pp.is_null() || (*pp).mp_ops != addr_of!(rust_zcrx_mp_ops) || io_pp_to_ifq(pp) != ifq { return neg(EFAULT); }
    if !io_zcrx_queue_cqe(req, n, ifq, (off as c_uint).wrapping_add(rust_zcrx_frag_off(frag)) as c_int, len) { return neg(ENOSPC); }
    // Page-pool reference must precede the user reference to prevent recycling.
    rust_zcrx_pp_ref(rust_zcrx_niov_to_netmem(n)); io_zcrx_get_niov_uref(n); len
}
struct io_zcrx_args { req: *mut io_kiocb, ifq: *mut io_zcrx_ifq, nr_skbs: c_uint }
unsafe extern "C" fn io_zcrx_recv_skb(desc: *mut read_descriptor_t, skb: *mut sk_buff,
    mut offset: c_uint, len: usize) -> c_int {
    let args: *mut io_zcrx_args = rust_zcrx_desc_data(desc).cast();
    let ifq = (*args).ifq; let req = (*args).req; let start_off = offset;
    let mut len = len.min((*desc).count); let mut ret: c_int = 0;
    // TCP calls this once again at count==0; no budget charge on that callback.
    if len == 0 { return 0; }
    let old_nr = (*args).nr_skbs; (*args).nr_skbs = old_nr.wrapping_add(1);
    if old_nr > IO_SKBS_PER_CALL_LIMIT { return neg(EAGAIN); }
    'receive: {
        let headlen = rust_zcrx_skb_headlen(skb);
        if offset < headlen {
            let count = ((headlen - offset) as usize).min(len);
            let src_page = rust_zcrx_virt_to_page((*skb).data.cast());
            let src_offset = ((*skb).data as usize & (RUST_ZCRX_PAGE_SIZE as usize - 1)) as c_uint;
            let copied = io_zcrx_copy_chunk(req, ifq, src_page, src_offset.wrapping_add(offset), count);
            if copied < 0 { ret = copied as c_int; break 'receive; }
            offset = offset.wrapping_add(copied as c_uint); len -= copied as usize;
            if len == 0 || offset != headlen { break 'receive; }
        }
        let mut start = headlen; let shi = rust_zcrx_skb_shinfo(skb);
        for i in 0..(*shi).nr_frags {
            let frag = addr_of!((*shi).frags).cast::<skb_frag_t>().add(i as usize);
            let end = start.wrapping_add(rust_zcrx_frag_size(frag));
            if rust_zcrx_warn(start as usize > (offset as usize).wrapping_add(len)) { return neg(EFAULT); }
            if offset < end {
                let count = ((end - offset) as usize).min(len) as c_uint; let frag_off = offset.wrapping_sub(start);
                ret = io_zcrx_recv_frag(req, ifq, frag, frag_off as c_int, count as c_int);
                if ret < 0 { break 'receive; }
                offset = offset.wrapping_add(ret as c_uint); len -= ret as usize;
                if len == 0 || ret as c_uint != count { break 'receive; }
            }
            start = end;
        }
        let mut frag_iter = (*shi).frag_list;
        while !frag_iter.is_null() {
            if rust_zcrx_warn(start as usize > (offset as usize).wrapping_add(len)) { return neg(EFAULT); }
            let end = start.wrapping_add((*frag_iter).len);
            if offset < end {
                let count = ((end - offset) as usize).min(len) as c_uint; let frag_off = offset.wrapping_sub(start);
                let saved = (*desc).count;
                ret = io_zcrx_recv_skb(desc, frag_iter, frag_off, count as usize);
                (*desc).count = saved;
                if ret < 0 { break 'receive; }
                offset = offset.wrapping_add(ret as c_uint); len -= ret as usize;
                if len == 0 || ret as c_uint != count { break 'receive; }
            }
            start = end; frag_iter = rust_zcrx_skb_next(frag_iter);
        }
    }
    if offset == start_off { return ret; }
    let consumed = offset.wrapping_sub(start_off);
    (*desc).count -= consumed as usize;
    consumed as c_int
}
unsafe fn io_zcrx_tcp_recvmsg(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq,
    sk: *mut sock, _flags: c_int, issue_flags: c_uint, outlen: *mut c_uint) -> c_int {
    let len = *outlen;
    let mut args = io_zcrx_args { req, ifq, nr_skbs: 0 };
    let mut desc: read_descriptor_t = zeroed();
    desc.count = (if len != 0 { len } else { u32::MAX }) as usize;
    rust_zcrx_desc_set_data(addr_of_mut!(desc), addr_of_mut!(args).cast());
    rust_zcrx_lock_sock(sk);
    let mut ret = tcp_read_sock(sk, addr_of_mut!(desc), Some(io_zcrx_recv_skb));
    if len != 0 && ret > 0 { *outlen = len.wrapping_sub(ret as c_uint); }
    if ret <= 0 {
        if ret == 0 && !rust_zcrx_sock_done(sk) {
            if (*sk).sk_err != 0 { ret = rust_zcrx_sock_error(sk); }
            else if rust_zcrx_sock_shutdown(sk) & RCV_SHUTDOWN != 0 { /* EOF */ }
            else if rust_zcrx_sock_state(sk) as u32 == TCP_CLOSE { ret = neg(ENOTCONN); }
            else { ret = neg(EAGAIN); }
        }
    } else if args.nr_skbs > IO_SKBS_PER_CALL_LIMIT && issue_flags & RUST_ZCRX_F_MULTISHOT != 0 {
        ret = IOU_REQUEUE as c_int;
    } else if rust_zcrx_sock_done(sk) {
        ret = if issue_flags & RUST_ZCRX_F_MULTISHOT != 0 { IOU_REQUEUE as c_int } else { neg(EAGAIN) };
    }
    release_sock(sk); ret
}
#[no_mangle]
pub unsafe extern "C" fn io_zcrx_recv(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq,
    sock: *mut socket, flags: c_uint, issue_flags: c_uint, len: *mut c_uint) -> c_int {
    let sk = (*sock).sk;
    if !rust_zcrx_is_tcp_recvmsg(sk) { return neg(EPROTONOSUPPORT); }
    rust_zcrx_sock_rps_record_flow(sk);
    io_zcrx_tcp_recvmsg(req, ifq, sk, flags as c_int, issue_flags, len)
}
