// SPDX-License-Identifier: GPL-2.0+
/* Translation of comedi_buf.c. Kernel types and helpers are supplied externally. */

#[cfg(feature = "page_kernel_nocache")]
const COMEDI_PAGE_PROTECTION: usize = PAGE_KERNEL_NOCACHE;
#[cfg(not(feature = "page_kernel_nocache"))]
const COMEDI_PAGE_PROTECTION: usize = PAGE_KERNEL;

unsafe fn comedi_buf_map_kref_release(kref: *mut kref) {
    let bm = container_of!(kref, comedi_buf_map, refcount);
    let bm = &mut *bm;
    if !bm.page_list.is_null() {
        if bm.dma_dir != DMA_NONE {
            for i in 0..bm.n_pages {
                let buf = &mut *bm.page_list.add(i as usize);
                dma_free_coherent(bm.dma_hw_dev, PAGE_SIZE, buf.virt_addr, buf.dma_addr);
            }
        } else {
            for i in 0..bm.n_pages {
                let buf = &mut *bm.page_list.add(i as usize);
                ClearPageReserved(virt_to_page(buf.virt_addr));
                free_page(buf.virt_addr as usize);
            }
        }
        vfree(bm.page_list as *mut _);
    }
    if bm.dma_dir != DMA_NONE { put_device(bm.dma_hw_dev); }
    kfree(bm as *mut _);
}

unsafe fn __comedi_buf_free(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let async_ = (*s).async_;
    (*async_).prealloc_bufsz = 0;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*s).spin_lock, &mut flags);
    let bm = (*async_).buf_map;
    (*async_).buf_map = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut (*s).spin_lock, flags);
    comedi_buf_map_put(bm);
}

unsafe fn comedi_buf_map_alloc(dev: *mut comedi_device, dma_dir: enum_dma_data_direction, n_pages: u32) -> *mut comedi_buf_map {
    let bm = kzalloc_obj::<comedi_buf_map>();
    if bm.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*bm).refcount);
    (*bm).dma_dir = dma_dir;
    if dma_dir != DMA_NONE { (*bm).dma_hw_dev = get_device((*dev).hw_dev); }
    (*bm).page_list = vzalloc(core::mem::size_of::<comedi_buf_page>() * n_pages as usize) as *mut comedi_buf_page;
    if (*bm).page_list.is_null() { comedi_buf_map_put(bm); return core::ptr::null_mut(); }
    let mut i = 0;
    if dma_dir != DMA_NONE {
        while i < n_pages {
            let buf = &mut *(*bm).page_list.add(i as usize);
            buf.virt_addr = dma_alloc_coherent((*bm).dma_hw_dev, PAGE_SIZE, &mut buf.dma_addr, GFP_KERNEL);
            if buf.virt_addr.is_null() { break; }
            i += 1;
        }
    } else {
        while i < n_pages {
            let buf = &mut *(*bm).page_list.add(i as usize);
            buf.virt_addr = get_zeroed_page(GFP_KERNEL) as *mut core::ffi::c_void;
            if buf.virt_addr.is_null() { break; }
            SetPageReserved(virt_to_page(buf.virt_addr));
            i += 1;
        }
    }
    (*bm).n_pages = i;
    if i < n_pages { comedi_buf_map_put(bm); return core::ptr::null_mut(); }
    bm
}

unsafe fn __comedi_buf_alloc(dev: *mut comedi_device, s: *mut comedi_subdevice, n_pages: u32) {
    let async_ = (*s).async_;
    if !IS_ENABLED!(CONFIG_HAS_DMA) && (*s).async_dma_dir != DMA_NONE { dev_err((*dev).class_dev, "dma buffer allocation not supported\n"); return; }
    let bm = comedi_buf_map_alloc(dev, (*s).async_dma_dir, n_pages);
    if bm.is_null() { return; }
    let mut flags = 0;
    spin_lock_irqsave(&mut (*s).spin_lock, &mut flags);
    (*async_).buf_map = bm;
    spin_unlock_irqrestore(&mut (*s).spin_lock, flags);
    (*async_).prealloc_bufsz = n_pages << PAGE_SHIFT;
}

pub unsafe fn comedi_buf_map_get(bm: *mut comedi_buf_map) { if !bm.is_null() { kref_get(&mut (*bm).refcount); } }
pub unsafe fn comedi_buf_map_put(bm: *mut comedi_buf_map) -> i32 { if !bm.is_null() { kref_put(&mut (*bm).refcount, comedi_buf_map_kref_release) } else { 1 } }

pub unsafe fn comedi_buf_map_access(bm: *mut comedi_buf_map, offset: usize, mut buf: *mut u8, len: i32, write: i32) -> i32 {
    let mut pgoff = offset_in_page(offset);
    let mut pg = offset >> PAGE_SHIFT;
    let mut done = 0;
    while done < len && pg < (*bm).n_pages as usize {
        let l = core::cmp::min(len - done, PAGE_SIZE as i32 - pgoff as i32) as usize;
        let b = (*(*bm).page_list.add(pg)).virt_addr.cast::<u8>().add(pgoff);
        if write != 0 { core::ptr::copy_nonoverlapping(buf, b, l); } else { core::ptr::copy_nonoverlapping(b, buf, l); }
        buf = buf.add(l); done += l as i32; pg += 1; pgoff = 0;
    }
    done
}

pub unsafe fn comedi_buf_map_from_subdev_get(s: *mut comedi_subdevice) -> *mut comedi_buf_map {
    let async_ = (*s).async_; if async_.is_null() { return core::ptr::null_mut(); }
    let mut flags = 0; spin_lock_irqsave(&mut (*s).spin_lock, &mut flags);
    let mut bm = (*async_).buf_map;
    if !bm.is_null() && (*bm).n_pages != 0 { comedi_buf_map_get(bm); } else { bm = core::ptr::null_mut(); }
    spin_unlock_irqrestore(&mut (*s).spin_lock, flags); bm
}

pub unsafe fn comedi_buf_is_mmapped(s: *mut comedi_subdevice) -> bool { let bm = (*(*s).async_).buf_map; !bm.is_null() && kref_read(&(*bm).refcount) > 1 }

pub unsafe fn comedi_buf_alloc(dev: *mut comedi_device, s: *mut comedi_subdevice, mut new_size: usize) -> i32 {
    let async_ = (*s).async_; lockdep_assert_held(&(*dev).mutex);
    new_size = (new_size + PAGE_SIZE - 1) & PAGE_MASK;
    if (*async_).prealloc_bufsz == new_size { return 0; }
    __comedi_buf_free(dev, s);
    if new_size != 0 { __comedi_buf_alloc(dev, s, (new_size >> PAGE_SHIFT) as u32); if (*async_).prealloc_bufsz == 0 { return -ENOMEM; } }
    0
}

pub unsafe fn comedi_buf_reset(s: *mut comedi_subdevice) {
    let a = &mut *(*s).async_;
    a.buf_write_alloc_count = 0; a.buf_write_count = 0; a.buf_read_alloc_count = 0; a.buf_read_count = 0;
    a.buf_write_ptr = 0; a.buf_read_ptr = 0; a.cur_chan = 0; a.scans_done = 0; a.scan_progress = 0;
    a.munge_chan = 0; a.munge_count = 0; a.munge_ptr = 0; a.events = 0;
}

pub unsafe fn comedi_buf_write_n_available(s: *mut comedi_subdevice) -> u32 { let a=&*(*s).async_; a.buf_read_count + a.prealloc_bufsz - a.buf_write_count }
unsafe fn comedi_buf_write_n_unalloc(s:*mut comedi_subdevice)->u32 { let a=&*(*s).async_; a.buf_read_count+a.prealloc_bufsz-a.buf_write_alloc_count }
pub unsafe fn _comedi_buf_write_alloc(s:*mut comedi_subdevice, mut n:u32)->u32 { let a=&mut *(*s).async_; n=n.min(comedi_buf_write_n_unalloc(s)); a.buf_write_alloc_count+=n; smp_mb(); n }
pub unsafe fn comedi_buf_write_alloc(s:*mut comedi_subdevice, n:u32)->u32 { if comedi_get_is_subdevice_running(s) { let r=_comedi_buf_write_alloc(s,n); comedi_put_is_subdevice_running(s); r } else { 0 } }

unsafe fn comedi_buf_munge(s:*mut comedi_subdevice, mut num:u32)->u32 { let a=&mut *(*s).async_; let pages=(*a.buf_map).page_list; let mut count=0; let sample=comedi_bytes_per_sample(s); if (*s).munge.is_none() || (a.cmd.flags & CMDF_RAWDATA)!=0 || a.cmd.chanlist_len==0 { a.munge_count+=num; return num; } num-=num%sample; while count<num { let page=(a.munge_ptr>>PAGE_SHIFT) as usize; let off=offset_in_page(a.munge_ptr); let block=(num-count).min(PAGE_SIZE as u32-off); ((*s).munge.unwrap())((*s).device,s,(*pages.add(page)).virt_addr.cast::<u8>().add(off as usize),block,a.munge_chan); smp_wmb(); a.munge_chan=(a.munge_chan+block/sample)%a.cmd.chanlist_len; a.munge_count+=block; a.munge_ptr+=block; if a.munge_ptr==a.prealloc_bufsz {a.munge_ptr=0;} count+=block; } count }

pub unsafe fn comedi_buf_write_n_allocated(s:*mut comedi_subdevice)->u32 { let a=&*(*s).async_; a.buf_write_alloc_count-a.buf_write_count }
pub unsafe fn _comedi_buf_write_free(s:*mut comedi_subdevice,mut n:u32)->u32 { let a=&mut *(*s).async_; n=n.min(comedi_buf_write_n_allocated(s)); a.buf_write_count+=n; a.buf_write_ptr+=n; comedi_buf_munge(s,a.buf_write_count-a.munge_count); if a.buf_write_ptr>=a.prealloc_bufsz {a.buf_write_ptr%=a.prealloc_bufsz;} n }
pub unsafe fn comedi_buf_write_free(s:*mut comedi_subdevice,n:u32)->u32 { if comedi_get_is_subdevice_running(s){let r=_comedi_buf_write_free(s,n);comedi_put_is_subdevice_running(s);r}else{0} }

pub unsafe fn _comedi_buf_read_n_available(s:*mut comedi_subdevice)->u32 { let a=&*(*s).async_; if a.is_null(){return 0;} let n=a.munge_count-a.buf_read_count; smp_rmb(); n }
pub unsafe fn comedi_buf_read_n_available(s:*mut comedi_subdevice)->u32 { if comedi_get_is_subdevice_running(s){let r=_comedi_buf_read_n_available(s);comedi_put_is_subdevice_running(s);r}else{0} }
pub unsafe fn _comedi_buf_read_alloc(s:*mut comedi_subdevice,mut n:u32)->u32 { let a=&mut *(*s).async_; n=n.min(a.munge_count-a.buf_read_alloc_count); a.buf_read_alloc_count+=n; smp_rmb(); n }
pub unsafe fn comedi_buf_read_alloc(s:*mut comedi_subdevice,n:u32)->u32 { if comedi_get_is_subdevice_running(s){let r=_comedi_buf_read_alloc(s,n);comedi_put_is_subdevice_running(s);r}else{0} }
unsafe fn comedi_buf_read_n_allocated(a:*mut comedi_async)->u32{(*a).buf_read_alloc_count-(*a).buf_read_count}
pub unsafe fn _comedi_buf_read_free(s:*mut comedi_subdevice,mut n:u32)->u32 { let a=&mut *(*s).async_; smp_mb(); n=n.min(comedi_buf_read_n_allocated(a)); a.buf_read_count+=n;a.buf_read_ptr+=n;a.buf_read_ptr%=a.prealloc_bufsz;n }
pub unsafe fn comedi_buf_read_free(s:*mut comedi_subdevice,n:u32)->u32 { if comedi_get_is_subdevice_running(s){let r=_comedi_buf_read_free(s,n);comedi_put_is_subdevice_running(s);r}else{0} }

unsafe fn comedi_buf_memcpy_to(s:*mut comedi_subdevice,mut data:*const u8,mut n:u32){let a=&*(*s).async_;let pages=(*a.buf_map).page_list;let mut p=a.buf_write_ptr;while n!=0{let page=(p>>PAGE_SHIFT)as usize;let off=offset_in_page(p);let b=n.min(PAGE_SIZE as u32-off);core::ptr::copy_nonoverlapping(data,(*pages.add(page)).virt_addr.cast::<u8>().add(off as usize),b as usize);data=data.add(b as usize);n-=b;p+=b;if p==a.prealloc_bufsz{p=0;}}}
unsafe fn comedi_buf_memcpy_from(s:*mut comedi_subdevice,mut dest:*mut u8,mut n:u32){let a=&*(*s).async_;let pages=(*a.buf_map).page_list;let mut p=a.buf_read_ptr;while n!=0{let page=(p>>PAGE_SHIFT)as usize;let off=offset_in_page(p);let b=n.min(PAGE_SIZE as u32-off);core::ptr::copy_nonoverlapping((*pages.add(page)).virt_addr.cast::<u8>().add(off as usize),dest,b as usize);n-=b;dest=dest.add(b as usize);p+=b;if p==a.prealloc_bufsz{p=0;}}}

unsafe fn _comedi_buf_write_samples(s:*mut comedi_subdevice,data:*const u8,mut ns:u32)->u32{let max=comedi_bytes_to_samples(s,comedi_buf_write_n_unalloc(s));if ns>max{dev_warn((*s).device.class_dev,"buffer overrun\n");(*(*s).async_).events|=COMEDI_CB_OVERFLOW;ns=max;}if ns==0{return 0;}let n=comedi_samples_to_bytes(s,ns);let n=_comedi_buf_write_alloc(s,n);comedi_buf_memcpy_to(s,data,n);_comedi_buf_write_free(s,n);_comedi_inc_scan_progress(s,n);(*(*s).async_).events|=COMEDI_CB_BLOCK;n}
pub unsafe fn comedi_buf_write_samples(s:*mut comedi_subdevice,d:*const u8,n:u32)->u32{if comedi_get_is_subdevice_running(s){let r=_comedi_buf_write_samples(s,d,n);comedi_put_is_subdevice_running(s);r}else{0}}
unsafe fn _comedi_buf_read_samples(s:*mut comedi_subdevice,d:*mut u8,mut ns:u32)->u32{ns=ns.min(comedi_bytes_to_samples(s,_comedi_buf_read_n_available(s)));if ns==0{return 0;}let n=_comedi_buf_read_alloc(s,comedi_samples_to_bytes(s,ns));comedi_buf_memcpy_from(s,d,n);_comedi_buf_read_free(s,n);_comedi_inc_scan_progress(s,n);(*(*s).async_).events|=COMEDI_CB_BLOCK;n}
pub unsafe fn comedi_buf_read_samples(s:*mut comedi_subdevice,d:*mut u8,n:u32)->u32{if comedi_get_is_subdevice_running(s){let r=_comedi_buf_read_samples(s,d,n);comedi_put_is_subdevice_running(s);r}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
