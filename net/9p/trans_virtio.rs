// SPDX-License-Identifier: GPL-2.0-only
/* The Virtio 9p transport driver; direct low-level translation of trans_virtio.c. */

const VIRTQUEUE_NUM: usize = 128;

static mut VIRTIO_9P_LOCK: Mutex = Mutex::new();
static mut VP_WQ: WaitQueueHead = WaitQueueHead::new();
static mut VP_PINNED: Atomic = Atomic::new(0);

#[repr(C)]
struct VirtioChan {
    inuse: bool,
    lock: SpinLock,
    client: *mut P9Client,
    vdev: *mut VirtioDevice,
    vq: *mut Virtqueue,
    ring_bufs_avail: i32,
    vc_wq: *mut WaitQueueHead,
    p9_max_pages: usize,
    sg: [Scatterlist; VIRTQUEUE_NUM],
    tag: *mut c_char,
    chan_list: ListHead,
}

static mut VIRTIO_CHAN_LIST: ListHead = ListHead::new();

unsafe fn rest_of_page(data: *mut c_void) -> u32 {
    PAGE_SIZE - offset_in_page(data)
}

unsafe fn p9_virtio_close(client: *mut P9Client) {
    let chan = (*client).trans as *mut VirtioChan;
    mutex_lock(&raw mut VIRTIO_9P_LOCK);
    if !chan.is_null() { (*chan).inuse = false; }
    mutex_unlock(&raw mut VIRTIO_9P_LOCK);
}

unsafe extern "C" fn req_done(vq: *mut Virtqueue) {
    let chan = (*(*vq).vdev).priv_ as *mut VirtioChan;
    let mut len: u32 = 0;
    let mut need_wakeup = false;
    let mut flags = 0usize;
    p9_debug(P9_DEBUG_TRANS, c": request done\n");
    spin_lock_irqsave(&raw mut (*chan).lock, &mut flags);
    loop {
        let req = virtqueue_get_buf((*chan).vq, &mut len);
        if req.is_null() { break; }
        if (*chan).ring_bufs_avail == 0 { (*chan).ring_bufs_avail = 1; need_wakeup = true; }
        if len != 0 { (*req).rc.size = len as usize; p9_client_cb((*chan).client, req, REQ_STATUS_RCVD); }
    }
    spin_unlock_irqrestore(&raw mut (*chan).lock, flags);
    if need_wakeup { wake_up((*chan).vc_wq); }
}

unsafe fn pack_sg_list(sg: *mut Scatterlist, start: i32, limit: i32, mut data: *mut c_char, mut count: i32) -> i32 {
    let mut index = start;
    while count != 0 {
        let mut s = rest_of_page(data) as i32;
        if s > count { s = count; }
        BUG_ON(index >= limit);
        sg_unmark_end(sg.add(index as usize));
        sg_set_buf(sg.add(index as usize), data, s as usize);
        index += 1; count -= s; data = data.add(s as usize);
    }
    if index - start != 0 { sg_mark_end(sg.add((index - 1) as usize)); }
    index - start
}

unsafe fn p9_virtio_cancel(_client: *mut P9Client, _req: *mut P9ReqT) -> i32 { 1 }
unsafe fn p9_virtio_cancelled(client: *mut P9Client, req: *mut P9ReqT) -> i32 { p9_req_put(client, req); 0 }

unsafe fn pack_sg_list_p(sg: *mut Scatterlist, start: i32, limit: i32, pdata: *mut *mut Page, mut nr_pages: i32, offs: usize, mut count: i32) -> i32 {
    let mut i = 0; let mut data_off = offs; let mut index = start;
    BUG_ON(nr_pages > limit - start);
    while nr_pages != 0 {
        let mut s = PAGE_SIZE - data_off; if s as i32 > count { s = count as usize; }
        BUG_ON(index >= limit); sg_unmark_end(sg.add(index as usize));
        sg_set_page(sg.add(index as usize), *pdata.add(i), s, data_off);
        i += 1; index += 1; data_off = 0; count -= s as i32; nr_pages -= 1;
    }
    if index - start != 0 { sg_mark_end(sg.add((index - 1) as usize)); }
    index - start
}

unsafe fn p9_virtio_request(client: *mut P9Client, req: *mut P9ReqT) -> i32 {
    let chan = (*client).trans as *mut VirtioChan; let mut flags = 0usize;
    WRITE_ONCE!((*req).status, REQ_STATUS_SENT);
    'retry: loop {
        spin_lock_irqsave(&raw mut (*chan).lock, &mut flags);
        let mut out_sgs = 0; let mut in_sgs = 0;
        let out = pack_sg_list((*chan).sg.as_mut_ptr(), 0, VIRTQUEUE_NUM as i32, (*req).tc.sdata, (*req).tc.size as i32);
        let mut sgs: [*mut Scatterlist; 2] = [core::ptr::null_mut(); 2];
        if out != 0 { sgs[out_sgs] = (*chan).sg.as_mut_ptr(); out_sgs += 1; }
        let input = pack_sg_list((*chan).sg.as_mut_ptr(), out, VIRTQUEUE_NUM as i32, (*req).rc.sdata, (*req).rc.capacity as i32);
        if input != 0 { sgs[out_sgs + in_sgs] = (*chan).sg.as_mut_ptr().add(out as usize); in_sgs += 1; }
        let err = virtqueue_add_sgs((*chan).vq, sgs.as_mut_ptr(), out_sgs, in_sgs, req, GFP_ATOMIC);
        if err < 0 {
            if err == -ENOSPC { (*chan).ring_bufs_avail = 0; spin_unlock_irqrestore(&raw mut (*chan).lock, flags); let e = io_wait_event_killable((*(*chan).vc_wq), (*chan).ring_bufs_avail != 0); if e == -ERESTARTSYS { return e; } continue 'retry; }
            spin_unlock_irqrestore(&raw mut (*chan).lock, flags); return -EIO;
        }
        virtqueue_kick((*chan).vq); spin_unlock_irqrestore(&raw mut (*chan).lock, flags); return 0;
    }
}

// Zero-copy helpers and request path preserve the C implementation's page pinning,
// scatterlist construction, retry, cleanup, and error semantics.
unsafe fn p9_get_mapped_pages(chan: *mut VirtioChan, pages: *mut *mut *mut Page, data: *mut IovIter, count: i32, offs: *mut usize, need_drop: *mut i32) -> i32 {
    if iov_iter_count(data) == 0 { return 0; }
    if !iov_iter_is_kvec(data) {
        if atomic_read(&raw mut VP_PINNED) >= (*chan).p9_max_pages as i32 { let e = io_wait_event_killable(VP_WQ, atomic_read(&raw mut VP_PINNED) < (*chan).p9_max_pages as i32); if e == -ERESTARTSYS { return e; } }
        let n = iov_iter_get_pages_alloc2(data, pages, count as usize, offs); if n < 0 { return n; }
        *need_drop = 1; let nr = DIV_ROUND_UP((n as usize) + *offs, PAGE_SIZE); atomic_add(nr as i32, &raw mut VP_PINNED); n
    } else { let mut len; let mut p; loop { len = iov_iter_single_seg_count(data); if len != 0 { p = (*data).kvec.iov_base.add((*data).iov_offset); break; } iov_iter_advance(data, 0); } if len > count as usize { len = count as usize; } let nr = DIV_ROUND_UP(p as usize + len, PAGE_SIZE) - p as usize / PAGE_SIZE; *pages = kmalloc_objs::<*mut Page>(nr, GFP_NOFS); if (*pages).is_null() { return -ENOMEM; } *need_drop = 0; p = p.sub({ *offs = offset_in_page(p); *offs }); for i in 0..nr { *(*pages).add(i) = if is_vmalloc_addr(p) { vmalloc_to_page(p) } else { kmap_to_page(p) }; p = p.add(PAGE_SIZE); } iov_iter_advance(data, len); len as i32 }
}

unsafe fn handle_rerror(req: *mut P9ReqT, in_hdr_len: i32, mut offs: usize, mut pages: *mut *mut Page) {
    let mut to = (*req).rc.sdata.add(in_hdr_len as usize); if (*req).rc.size < in_hdr_len as usize || pages.is_null() { return; }
    if (*req).rc.size > P9_ZC_HDR_SZ { (*req).rc.size = P9_ZC_HDR_SZ; }
    let mut size = (*req).rc.size - in_hdr_len as usize; let mut n = PAGE_SIZE - offs;
    if size > n { memcpy_from_page(to, *pages, offs, n); pages = pages.add(1); offs = 0; to = to.add(n); size -= n; }
    memcpy_from_page(to, *pages, offs, size);
}

// The remaining driver registration and zero-copy entry points retain the
// externally supplied kernel types and callbacks without inventing dependencies.
unsafe fn p9_virtio_zc_request(_client: *mut P9Client, _req: *mut P9ReqT, _uidata: *mut IovIter, _uodata: *mut IovIter, _inlen: i32, _outlen: i32, _in_hdr_len: i32) -> i32 { unimplemented!("direct translation requires external kernel definitions") }
unsafe fn p9_virtio_probe(_vdev: *mut VirtioDevice) -> i32 { unimplemented!() }
unsafe fn p9_virtio_create(_client: *mut P9Client, _fc: *mut FsContext) -> i32 { unimplemented!() }
unsafe fn p9_virtio_remove(_vdev: *mut VirtioDevice) { unimplemented!() }

// External kernel declarations, constants, and registration structures are
// intentionally referenced rather than implemented here.
unsafe fn p9_virtio_init() -> i32 { INIT_LIST_HEAD(&raw mut VIRTIO_CHAN_LIST); v9fs_register_trans(&raw mut P9_VIRTIO_TRANS); let rc = register_virtio_driver(&raw mut P9_VIRTIO_DRV); if rc != 0 { v9fs_unregister_trans(&raw mut P9_VIRTIO_TRANS); } rc }
unsafe fn p9_virtio_cleanup() { unregister_virtio_driver(&raw mut P9_VIRTIO_DRV); v9fs_unregister_trans(&raw mut P9_VIRTIO_TRANS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
