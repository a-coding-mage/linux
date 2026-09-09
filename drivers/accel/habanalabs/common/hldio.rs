// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2024 HabanaLabs, Ltd.
// All Rights Reserved.

// NVMe Direct I/O implementation for habanalabs driver.
// The original implementation assumes no IOMMU, READ-only operations, no
// sparse files, kernel version >= 6.9, page alignment, PCI_P2PDMA support,
// and suitable PCI topology.

const IO_STABILIZE_TIMEOUT: u64 = 10_000_000;

#[repr(C)]
pub struct hl_dio_fd {
    // Back pointer in case we need it in async completion
    pub ctx: *mut hl_ctx,
    // Associated fd struct
    pub filp: *mut file,
}

#[repr(C)]
pub struct hl_direct_io {
    pub f: hl_dio_fd,
    pub kio: kiocb,
    pub bv: *mut bio_vec,
    pub iter: iov_iter,
    pub device_va: u64,
    pub off_bytes: u64,
    pub len_bytes: u64,
    pub type_: u32,
}

pub unsafe fn hl_device_supports_nvme(hdev: *mut hl_device) -> bool {
    (*hdev).asic_prop.supports_nvme
}

unsafe fn hl_dio_fd_register(ctx: *mut hl_ctx, fd: i32, f: *mut hl_dio_fd) -> i32 {
    let hdev = (*ctx).hdev;
    let mut rc: i32;
    (*f).filp = fget(fd);
    if (*f).filp.is_null() { return -ENOENT; }

    if (*(*f).filp).f_flags & O_DIRECT == 0 {
        dev_err((*hdev).dev, "file is not in the direct mode\n");
        rc = -EINVAL;
        goto_fput(f, rc);
    }
    if (*(*(*f).filp).f_op).read_iter.is_none() {
        dev_err((*hdev).dev, "read iter is not supported, need to fall back to legacy\n");
        rc = -EINVAL;
        goto_fput(f, rc);
    }

    let inode = file_inode((*f).filp);
    let sb = (*inode).i_sb;
    let bd = (*sb).s_bdev;
    let gd = (*bd).bd_disk;
    if ((*inode).i_blocks << (*sb).s_blocksize_bits) < i_size_read(inode) {
        dev_err((*hdev).dev, "sparse files are not currently supported\n");
        rc = -EINVAL;
        goto_fput(f, rc);
    }
    if bd.is_null() || gd.is_null() {
        dev_err((*hdev).dev, "invalid block device\n");
        rc = -ENODEV;
        goto_fput(f, rc);
    }
    let disk_dev = disk_to_dev(gd);
    if !dma_pci_p2pdma_supported(disk_dev) {
        dev_err((*hdev).dev, "device does not support PCI P2P DMA\n");
        rc = -EOPNOTSUPP;
        goto_fput(f, rc);
    }
    (*f).ctx = ctx;
    return 0;

    fn goto_fput(f: *mut hl_dio_fd, rc: i32) -> i32 {
        unsafe { fput((*f).filp); }
        rc
    }
}

unsafe fn hl_dio_fd_unregister(f: *mut hl_dio_fd) { fput((*f).filp); }

unsafe fn hl_dio_count_io(hdev: *mut hl_device) -> i64 {
    let mut sum: i64 = 0;
    let mut i: i32 = 0;
    for_each_possible_cpu!(i) { sum += *per_cpu!((*hdev).hldio.inflight_ios, i); }
    sum
}

unsafe fn hl_dio_get_iopath(ctx: *mut hl_ctx) -> bool {
    let hdev = (*ctx).hdev;
    if (*hdev).hldio.io_enabled {
        this_cpu_inc!((*hdev).hldio.inflight_ios);
        if !(*hdev).hldio.io_enabled {
            this_cpu_dec!((*hdev).hldio.inflight_ios);
            return false;
        }
        hl_ctx_get(ctx);
        return true;
    }
    false
}

unsafe fn hl_dio_put_iopath(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev;
    hl_ctx_put(ctx);
    this_cpu_dec!((*hdev).hldio.inflight_ios);
}

unsafe fn hl_dio_set_io_enabled(hdev: *mut hl_device, enabled: bool) {
    (*hdev).hldio.io_enabled = enabled;
}

unsafe fn hl_dio_validate_io(hdev: *mut hl_device, io: *mut hl_direct_io) -> bool {
    if (*io).device_va & !PAGE_MASK != 0 { dev_dbg((*hdev).dev, "device address must be 4K aligned\n"); return false; }
    if (*io).len_bytes & !PAGE_MASK != 0 { dev_dbg((*hdev).dev, "IO length must be 4K aligned\n"); return false; }
    if (*io).off_bytes & !PAGE_MASK != 0 { dev_dbg((*hdev).dev, "IO offset must be 4K aligned\n"); return false; }
    true
}

unsafe fn hl_dio_va2page(hdev: *mut hl_device, ctx: *mut hl_ctx, device_va: u64) -> *mut page {
    let hldio = &mut (*hdev).hldio;
    let mut device_pa = 0u64;
    let rc = hl_mmu_va_to_pa(ctx, device_va, &mut device_pa);
    if rc != 0 { dev_err((*hdev).dev, "device virtual address translation error: %#llx (%d)", device_va, rc); return core::ptr::null_mut(); }
    let mut i = 0;
    while i < hldio.np2prs {
        let p = &mut *hldio.p2prs.add(i as usize);
        if device_pa >= p.device_pa && device_pa < p.device_pa + p.size { return *p.p2ppages.add(((device_pa - p.device_pa) >> PAGE_SHIFT) as usize); }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn hl_direct_io(hdev: *mut hl_device, io: *mut hl_direct_io) -> isize {
    if !hl_dio_validate_io(hdev, io) { return -EINVAL as isize; }
    if !hl_dio_get_iopath((*io).f.ctx) { dev_info((*hdev).dev, "can't schedule a new IO, IO is disabled\n"); return -ESHUTDOWN as isize; }
    init_sync_kiocb(&mut (*io).kio, (*io).f.filp);
    (*io).kio.ki_pos = (*io).off_bytes;
    let npages = (*io).len_bytes >> PAGE_SHIFT;
    (*io).bv = vzalloc(npages * core::mem::size_of::<bio_vec>());
    if (*io).bv.is_null() { return -ENOMEM as isize; }
    let mut device_va = (*io).device_va;
    let mut i = 0;
    while i < npages {
        let bv = &mut *(*io).bv.add(i as usize);
        bv.bv_page = hl_dio_va2page(hdev, (*io).f.ctx, device_va);
        if bv.bv_page.is_null() { dev_err((*hdev).dev, "error getting page struct for device va %#llx", device_va); vfree((*io).bv); hl_dio_put_iopath((*io).f.ctx); return -EFAULT as isize; }
        bv.bv_offset = 0; bv.bv_len = PAGE_SIZE;
        i += 1; device_va += PAGE_SIZE;
    }
    iov_iter_bvec(&mut (*io).iter, (*io).type_, (*io).bv, 1, (*io).len_bytes);
    let rc = if let Some(read_iter) = (*(*io).f.filp).f_op.read_iter { read_iter(&mut (*io).kio, &mut (*io).iter) } else { -EINVAL as isize };
    vfree((*io).bv); hl_dio_put_iopath((*io).f.ctx); dev_dbg((*hdev).dev, "IO ended with %ld\n", rc); rc
}

#[allow(unused_variables)]
unsafe fn hl_direct_io_complete(kio: *mut kiocb, ret: i64, ret2: i64) {
    let io = container_of!(kio, hl_direct_io, kio);
    dev_dbg((*(*io).f.ctx).hdev.dev, "IO completed with %ld\n", ret);
    hl_dio_put_iopath((*io).f.ctx); hl_dio_fd_unregister(&mut (*io).f);
}

pub unsafe fn hl_dio_ssd2hl(hdev: *mut hl_device, ctx: *mut hl_ctx, fd: i32, device_va: u64, off_bytes: i64, len_bytes: usize, len_read: *mut usize) -> i32 {
    dev_dbg((*hdev).dev, "SSD2HL fd=%d va=%#llx len=%#lx\n", fd, device_va, len_bytes);
    let io = kzalloc_obj::<hl_direct_io>();
    if io.is_null() { return -ENOMEM; }
    *io = core::mem::zeroed(); (*io).device_va = device_va; (*io).len_bytes = len_bytes as u64; (*io).off_bytes = off_bytes as u64; (*io).type_ = READ;
    let mut rc = hl_dio_fd_register(ctx, fd, &mut (*io).f);
    if rc == 0 { rc = hl_direct_io(hdev, io) as i32; if rc >= 0 { *len_read = rc as usize; rc = 0; } hl_dio_fd_unregister(&mut (*io).f); }
    kfree(io); rc
}

unsafe fn hl_p2p_region_fini(hdev: *mut hl_device, p2pr: *mut hl_p2p_region) {
    if !(*p2pr).p2ppages.is_null() { vfree((*p2pr).p2ppages); (*p2pr).p2ppages = core::ptr::null_mut(); }
    if !(*p2pr).p2pmem.is_null() { dev_dbg((*hdev).dev, "freeing P2P mem from %p, size=%#llx\n", (*p2pr).p2pmem, (*p2pr).size); pci_free_p2pmem((*hdev).pdev, (*p2pr).p2pmem, (*p2pr).size); (*p2pr).p2pmem = core::ptr::null_mut(); }
}

pub unsafe fn hl_p2p_region_fini_all(hdev: *mut hl_device) { let mut i=0; while i < (*hdev).hldio.np2prs { hl_p2p_region_fini(hdev, (*hdev).hldio.p2prs.add(i as usize)); i+=1; } kvfree((*hdev).hldio.p2prs); (*hdev).hldio.p2prs=core::ptr::null_mut(); (*hdev).hldio.np2prs=0; }

pub unsafe fn hl_p2p_region_init(hdev: *mut hl_device, p2pr: *mut hl_p2p_region) -> i32 {
    let mut rc = pci_p2pdma_add_resource((*hdev).pdev, (*p2pr).bar, (*p2pr).size, (*p2pr).bar_offset);
    if rc != 0 { dev_err((*hdev).dev, "error adding p2p resource: %d\n", rc); hl_p2p_region_fini(hdev,p2pr); return rc; }
    (*p2pr).p2pmem = pci_alloc_p2pmem((*hdev).pdev, (*p2pr).size);
    if (*p2pr).p2pmem.is_null() { dev_err((*hdev).dev, "error allocating p2p memory\n"); hl_p2p_region_fini(hdev,p2pr); return -ENOMEM; }
    (*p2pr).p2ppages = vmalloc(((*p2pr).size >> PAGE_SHIFT) * core::mem::size_of::<*mut page>()) as *mut *mut page;
    if (*p2pr).p2ppages.is_null() { hl_p2p_region_fini(hdev,p2pr); return -ENOMEM; }
    let n = (*p2pr).size >> PAGE_SHIFT; let mut i=0; let mut addr=(*p2pr).p2pmem;
    while i<n { *(*p2pr).p2ppages.add(i as usize)=virt_to_page(addr); if (*(*p2pr).p2ppages.add(i as usize)).is_null() { hl_p2p_region_fini(hdev,p2pr); return -EFAULT; } i+=1; addr=addr.add(PAGE_SIZE as usize); }
    0
}

pub unsafe fn hl_dio_start(hdev: *mut hl_device) -> i32 { dev_dbg((*hdev).dev,"initializing HLDIO\n"); (*hdev).hldio.inflight_ios=alloc_percpu::<i64>(); if (*hdev).hldio.inflight_ios.is_null(){return -ENOMEM;} hl_dio_set_io_enabled(hdev,true); 0 }

pub unsafe fn hl_dio_stop(hdev: *mut hl_device) { dev_dbg((*hdev).dev,"deinitializing HLDIO\n"); if (*hdev).hldio.io_enabled { hl_dio_set_io_enabled(hdev,false); hl_poll_timeout_condition!(hdev, !hl_dio_count_io(hdev), 1000, IO_STABILIZE_TIMEOUT); } if !(*hdev).hldio.inflight_ios.is_null(){free_percpu((*hdev).hldio.inflight_ios);(*hdev).hldio.inflight_ios=core::ptr::null_mut();} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
