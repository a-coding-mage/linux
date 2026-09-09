// SPDX-License-Identifier: GPL-2.0
// Linux kernel dependencies from the original includes are supplied externally.

const RINGBUF_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_RB_OVERWRITE;
const RINGBUF_PGOFF: usize = core::mem::offset_of!(bpf_ringbuf, consumer_pos) >> PAGE_SHIFT;
const RINGBUF_POS_PAGES: usize = 2;
const RINGBUF_NR_META_PAGES: usize = RINGBUF_PGOFF + RINGBUF_POS_PAGES;
const RINGBUF_MAX_RECORD_SZ: usize = u32::MAX as usize / 4;

#[repr(C)]
pub struct bpf_ringbuf {
    pub waitq: wait_queue_head_t,
    pub work: irq_work,
    pub mask: u64,
    pub pages: *mut *mut page,
    pub nr_pages: i32,
    pub overwrite_mode: bool,
    pub spinlock: rqspinlock_t,
    pub busy: atomic_t,
    pub consumer_pos: usize,
    pub producer_pos: usize,
    pub pending_pos: usize,
    pub overwrite_pos: usize,
    pub data: [u8; 0],
}

#[repr(C)] pub struct bpf_ringbuf_map { pub map: bpf_map, pub rb: *mut bpf_ringbuf }
#[repr(C)] pub struct bpf_ringbuf_hdr { pub len: u32, pub pg_off: u32 }

unsafe fn bpf_ringbuf_area_alloc(data_sz: usize, numa_node: i32) -> *mut bpf_ringbuf {
    let flags: gfp_t = GFP_KERNEL_ACCOUNT | __GFP_RETRY_MAYFAIL | __GFP_NOWARN | __GFP_ZERO;
    let nr_meta_pages = RINGBUF_NR_META_PAGES as i32;
    let nr_data_pages = (data_sz >> PAGE_SHIFT) as i32;
    let mut nr_pages = nr_meta_pages + nr_data_pages;
    let array_size = (nr_meta_pages + 2 * nr_data_pages) as usize * core::mem::size_of::<*mut page>();
    let pages = bpf_map_area_alloc(array_size, numa_node) as *mut *mut page;
    if pages.is_null() { return core::ptr::null_mut(); }
    for i in 0..nr_pages {
        let p = alloc_pages_node(numa_node, flags, 0);
        if p.is_null() { nr_pages = i; break; }
        *pages.add(i as usize) = p;
        if i >= nr_meta_pages { *pages.add((nr_data_pages + i) as usize) = p; }
    }
    if nr_pages == nr_meta_pages + nr_data_pages {
        let rb = vmap(pages, (nr_meta_pages + 2 * nr_data_pages) as usize, VM_MAP | VM_USERMAP, PAGE_KERNEL) as *mut bpf_ringbuf;
        if !rb.is_null() {
            kmemleak_not_leak(pages as *mut _);
            (*rb).pages = pages; (*rb).nr_pages = nr_pages; return rb;
        }
    }
    for i in 0..nr_pages { __free_page(*pages.add(i as usize)); }
    bpf_map_area_free(pages as *mut _); core::ptr::null_mut()
}

unsafe extern "C" fn bpf_ringbuf_notify(work: *mut irq_work) {
    let rb = container_of!(work, bpf_ringbuf, work);
    wake_up_all(&mut (*rb).waitq);
}

unsafe fn bpf_ringbuf_alloc(data_sz: usize, numa_node: i32, overwrite_mode: bool) -> *mut bpf_ringbuf {
    let rb = bpf_ringbuf_area_alloc(data_sz, numa_node); if rb.is_null() { return rb; }
    raw_res_spin_lock_init(&mut (*rb).spinlock); atomic_set(&mut (*rb).busy, 0);
    init_waitqueue_head(&mut (*rb).waitq); init_irq_work(&mut (*rb).work, Some(bpf_ringbuf_notify));
    (*rb).mask = (data_sz - 1) as u64; (*rb).consumer_pos = 0; (*rb).producer_pos = 0;
    (*rb).pending_pos = 0; (*rb).overwrite_mode = overwrite_mode; rb
}

unsafe extern "C" fn ringbuf_map_alloc(attr: *mut union_bpf_attr) -> *mut bpf_map {
    let mut overwrite_mode = false;
    if (*attr).map_flags & !RINGBUF_CREATE_FLAG_MASK != 0 { return ERR_PTR(-EINVAL); }
    if (*attr).map_flags & BPF_F_RB_OVERWRITE != 0 {
        if (*attr).map_type != BPF_MAP_TYPE_RINGBUF { return ERR_PTR(-EINVAL); } overwrite_mode = true;
    }
    if (*attr).key_size != 0 || (*attr).value_size != 0 || !is_power_of_2((*attr).max_entries) || !PAGE_ALIGNED((*attr).max_entries) { return ERR_PTR(-EINVAL); }
    let m = bpf_map_area_alloc(core::mem::size_of::<bpf_ringbuf_map>(), NUMA_NO_NODE) as *mut bpf_ringbuf_map;
    if m.is_null() { return ERR_PTR(-ENOMEM); }
    bpf_map_init_from_attr(&mut (*m).map, attr);
    (*m).rb = bpf_ringbuf_alloc((*attr).max_entries as usize, (*m).map.numa_node, overwrite_mode);
    if (*m).rb.is_null() { bpf_map_area_free(m as *mut _); return ERR_PTR(-ENOMEM); } &mut (*m).map
}

unsafe fn bpf_ringbuf_free(rb: *mut bpf_ringbuf) { irq_work_sync(&mut (*rb).work); let pages=(*rb).pages; let n=(*rb).nr_pages; vunmap(rb as *mut _); for i in 0..n { __free_page(*pages.add(i as usize)); } bpf_map_area_free(pages as *mut _); }
unsafe extern "C" fn ringbuf_map_free(map: *mut bpf_map) { let m=container_of!(map,bpf_ringbuf_map,map); bpf_ringbuf_free((*m).rb); bpf_map_area_free(m as *mut _); }
unsafe extern "C" fn ringbuf_map_lookup_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> *mut core::ffi::c_void { ERR_PTR(-ENOTSUPP) }
unsafe extern "C" fn ringbuf_map_update_elem(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: u64) -> i64 { -ENOTSUPP }
unsafe extern "C" fn ringbuf_map_delete_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> i64 { -ENOTSUPP }
unsafe extern "C" fn ringbuf_map_get_next_key(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 { -ENOTSUPP }

unsafe fn ringbuf_avail_data_sz(rb:*mut bpf_ringbuf)->usize { let c=smp_load_acquire(&(*rb).consumer_pos); let p=smp_load_acquire(&(*rb).producer_pos); if (*rb).overwrite_mode { let o=smp_load_acquire(&(*rb).overwrite_pos); core::cmp::min(p-c,p-o) } else { p-c } }
unsafe fn ringbuf_total_data_sz(rb:*const bpf_ringbuf)->usize { (*rb).mask as usize + 1 }

unsafe fn bpf_ringbuf_rec_pg_off(rb:*mut bpf_ringbuf,h:*mut bpf_ringbuf_hdr)->usize { (h as usize - rb as usize)>>PAGE_SHIFT }
unsafe fn bpf_ringbuf_restore_from_rec(h:*mut bpf_ringbuf_hdr)->*mut bpf_ringbuf { ((h as usize & PAGE_MASK) - ((*h).pg_off as usize<<PAGE_SHIFT)) as *mut bpf_ringbuf }
unsafe fn bpf_ringbuf_has_space(rb:*const bpf_ringbuf,n:usize,c:usize,p:usize)->bool { if n-p>(*rb).mask as usize{return false;} if (*rb).overwrite_mode{return true;} n-c<=(*rb).mask as usize }
unsafe fn bpf_ringbuf_round_up_hdr_len(mut n:u32)->u32 { n &= !BPF_RINGBUF_DISCARD_BIT; round_up(n + BPF_RINGBUF_HDR_SZ,8) }

// Remaining helper and BPF-call declarations retain the original kernel ABI.
// Their implementations are supplied by the surrounding kernel translation.
unsafe fn __bpf_ringbuf_reserve(rb:*mut bpf_ringbuf,size:u64)->*mut core::ffi::c_void {
    if size as usize > RINGBUF_MAX_RECORD_SZ { return core::ptr::null_mut(); }
    let len=round_up(size as u32+BPF_RINGBUF_HDR_SZ,8) as usize;
    if len>ringbuf_total_data_sz(rb){return core::ptr::null_mut();}
    let cons=smp_load_acquire(&(*rb).consumer_pos); let mut flags=0usize;
    if raw_res_spin_lock_irqsave(&mut (*rb).spinlock,&mut flags){return core::ptr::null_mut();}
    let prod=(*rb).producer_pos; let pend=(*rb).pending_pos; let new_prod=prod+len;
    if !bpf_ringbuf_has_space(rb,new_prod,cons,pend){raw_res_spin_unlock_irqrestore(&mut (*rb).spinlock,flags);return core::ptr::null_mut();}
    let hdr=((*rb).data.as_mut_ptr().add((prod&(*rb).mask as usize))) as *mut bpf_ringbuf_hdr;
    (*hdr).len=size as u32|BPF_RINGBUF_BUSY_BIT; (*hdr).pg_off=bpf_ringbuf_rec_pg_off(rb,hdr) as u32;
    smp_store_release(&mut (*rb).producer_pos,new_prod); raw_res_spin_unlock_irqrestore(&mut (*rb).spinlock,flags);
    hdr.cast::<u8>().add(BPF_RINGBUF_HDR_SZ as usize).cast()
}

unsafe fn bpf_ringbuf_commit(sample:*mut core::ffi::c_void,flags:u64,discard:bool) {
    let hdr=(sample as *mut u8).sub(BPF_RINGBUF_HDR_SZ as usize) as *mut bpf_ringbuf_hdr;
    let rb=bpf_ringbuf_restore_from_rec(hdr); let mut len=(*hdr).len^BPF_RINGBUF_BUSY_BIT;
    if discard {len|=BPF_RINGBUF_DISCARD_BIT;} core::ptr::write_volatile(&mut (*hdr).len,len);
    let rec=hdr as usize-(*rb).data.as_ptr() as usize; let cons=smp_load_acquire(&(*rb).consumer_pos)&(*rb).mask as usize;
    if flags&BPF_RB_FORCE_WAKEUP!=0 || (cons==rec && flags&BPF_RB_NO_WAKEUP==0){irq_work_queue(&mut (*rb).work);}
}

unsafe fn __bpf_user_ringbuf_peek(rb:*mut bpf_ringbuf,sample:*mut *mut core::ffi::c_void,size:*mut u32)->i32 {
    let prod=smp_load_acquire(&(*rb).producer_pos); let cons=smp_load_acquire(&(*rb).consumer_pos);
    if prod%8!=0{return -EINVAL;} if cons>=prod{return -ENODATA;}
    let h=(*rb).data.as_mut_ptr().add((cons&(*rb).mask as usize)) as *mut u32; let x=smp_load_acquire(h);
    let flags=x&(BPF_RINGBUF_BUSY_BIT|BPF_RINGBUF_DISCARD_BIT); let n=x&!flags; let total=round_up(n+BPF_RINGBUF_HDR_SZ,8) as u64;
    if total>prod-cons{return -EINVAL;} if total>ringbuf_total_data_sz(rb) as u64{return -E2BIG;} if flags&BPF_RINGBUF_DISCARD_BIT!=0{smp_store_release(&mut (*rb).consumer_pos,cons+total as usize);return -EAGAIN;} if flags&BPF_RINGBUF_BUSY_BIT!=0{return -ENODATA;}
    *sample=(*rb).data.as_mut_ptr().add(((cons+BPF_RINGBUF_HDR_SZ as u64)&(*rb).mask as u64) as usize) as *mut _; *size=n; 0
}

unsafe fn __bpf_user_ringbuf_sample_release(rb:*mut bpf_ringbuf,size:usize,_flags:u64){let n=round_up(size as u32+BPF_RINGBUF_HDR_SZ,8) as usize;let c=(*rb).consumer_pos;smp_store_release(&mut (*rb).consumer_pos,c+n);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
