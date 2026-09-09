// SPDX-License-Identifier: GPL-2.0
/* Public API and common code for kernel->userspace relay file support. */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut RELAY_CHANNELS_MUTEX: Mutex = Mutex::new();
static mut RELAY_CHANNELS: ListHead = ListHead::new();

unsafe fn relay_buf_fault(vmf: *mut VmFault) -> VmFaultT {
    let buf = (*(*vmf).vma).vm_private_data as *mut RchanBuf;
    let pgoff = (*vmf).pgoff;
    if buf.is_null() { return VM_FAULT_OOM; }
    let page = vmalloc_to_page((*buf).start.add(pgoff << PAGE_SHIFT));
    if page.is_null() { return VM_FAULT_SIGBUS; }
    get_page(page);
    (*vmf).page = page;
    0
}

static RELAY_FILE_MMAP_OPS: VmOperationsStruct = VmOperationsStruct { fault: Some(relay_buf_fault) };

unsafe fn relay_alloc_page_array(n_pages: u32) -> *mut *mut Page {
    kvzalloc_objs::<*mut Page>(n_pages)
}
unsafe fn relay_free_page_array(array: *mut *mut Page) { kvfree(array as *mut u8); }

unsafe fn relay_mmap_prepare_buf(buf: *mut RchanBuf, desc: *mut VmAreaDesc) -> i32 {
    let length = vma_desc_size(desc);
    if buf.is_null() { return -EBADF; }
    if length != (*(*buf).chan).alloc_size as u64 { return -EINVAL; }
    (*desc).vm_ops = &RELAY_FILE_MMAP_OPS;
    vma_desc_set_flags(desc, VMA_DONTEXPAND_BIT);
    (*desc).private_data = buf as *mut _;
    0
}

unsafe fn relay_alloc_buf(buf: *mut RchanBuf, size: *mut usize) -> *mut u8 {
    *size = page_align(*size);
    let n_pages = (*size >> PAGE_SHIFT) as u32;
    (*buf).page_array = relay_alloc_page_array(n_pages);
    if (*buf).page_array.is_null() { return core::ptr::null_mut(); }
    let mut i = 0;
    while i < n_pages {
        *(*buf).page_array.add(i as usize) = alloc_page(GFP_KERNEL | __GFP_ZERO);
        if (*(*buf).page_array.add(i as usize)).is_null() { break; }
        set_page_private(*(*buf).page_array.add(i as usize), buf as usize);
        i += 1;
    }
    if i != n_pages {
        for j in 0..i { __free_page(*(*buf).page_array.add(j as usize)); }
        relay_free_page_array((*buf).page_array);
        return core::ptr::null_mut();
    }
    let mem = vmap((*buf).page_array, n_pages, VM_MAP, PAGE_KERNEL);
    if mem.is_null() {
        for j in 0..i { __free_page(*(*buf).page_array.add(j as usize)); }
        relay_free_page_array((*buf).page_array);
        return core::ptr::null_mut();
    }
    (*buf).page_count = n_pages;
    mem
}

unsafe fn relay_create_buf(chan: *mut Rchan) -> *mut RchanBuf {
    if (*chan).n_subbufs > KMALLOC_MAX_SIZE / core::mem::size_of::<usize>() { return core::ptr::null_mut(); }
    let buf = kzalloc_obj::<RchanBuf>();
    if buf.is_null() { return core::ptr::null_mut(); }
    (*buf).padding = kmalloc_objs::<usize>((*chan).n_subbufs);
    if (*buf).padding.is_null() { kfree(buf); return core::ptr::null_mut(); }
    (*buf).start = relay_alloc_buf(buf, &mut (*chan).alloc_size);
    if (*buf).start.is_null() { kfree((*buf).padding); kfree(buf); return core::ptr::null_mut(); }
    (*buf).chan = chan;
    kref_get(&mut (*chan).kref);
    buf
}

unsafe fn relay_destroy_channel(kref: *mut Kref) {
    let chan = container_of!(kref, Rchan, kref);
    free_percpu((*chan).buf);
    kfree(chan);
}
unsafe fn relay_destroy_buf(buf: *mut RchanBuf) {
    let chan = (*buf).chan;
    if !(*buf).start.is_null() {
        vunmap((*buf).start);
        for i in 0..(*buf).page_count { __free_page(*(*buf).page_array.add(i as usize)); }
        relay_free_page_array((*buf).page_array);
    }
    *per_cpu_ptr((*chan).buf, (*buf).cpu) = core::ptr::null_mut();
    kfree((*buf).padding); kfree(buf);
    kref_put(&mut (*chan).kref, relay_destroy_channel);
}
unsafe fn relay_remove_buf(kref: *mut Kref) { relay_destroy_buf(container_of!(kref, RchanBuf, kref)); }
unsafe fn relay_buf_empty(buf: *mut RchanBuf) -> i32 {
    if (*buf).subbufs_produced.wrapping_sub((*buf).subbufs_consumed) != 0 { 0 } else { 1 }
}
pub unsafe fn relay_buf_full(buf: *mut RchanBuf) -> i32 {
    let ready = (*buf).subbufs_produced.wrapping_sub((*buf).subbufs_consumed);
    if ready >= (*(*buf).chan).n_subbufs { 1 } else { 0 }
}

unsafe fn relay_subbuf_start(buf: *mut RchanBuf, subbuf: *mut u8, prev: *mut u8) -> i32 {
    let full = relay_buf_full(buf);
    if full != 0 { (*buf).stats.full_count += 1; }
    match (*(*buf).chan).cb.as_ref().unwrap().subbuf_start {
        Some(f) => f(buf, subbuf as *mut _, prev as *mut _),
        None => if full != 0 { 0 } else { 1 },
    }
}
unsafe fn wakeup_readers(work: *mut IrqWork) {
    let buf = container_of!(work, RchanBuf, wakeup_work);
    wake_up_interruptible(&mut (*buf).read_wait);
}
unsafe fn __relay_reset(buf: *mut RchanBuf, init: u32) {
    if init != 0 { init_waitqueue_head(&mut (*buf).read_wait); kref_init(&mut (*buf).kref); init_irq_work(&mut (*buf).wakeup_work, wakeup_readers); }
    else { irq_work_sync(&mut (*buf).wakeup_work); }
    (*buf).subbufs_produced = 0; (*buf).subbufs_consumed = 0; (*buf).bytes_consumed = 0;
    (*buf).finalized = 0; (*buf).data = (*buf).start; (*buf).offset = 0;
    (*buf).stats.full_count = 0; (*buf).stats.big_count = 0;
    for i in 0..(*buf).chan.as_ref().unwrap().n_subbufs { *(*buf).padding.add(i) = 0; }
    relay_subbuf_start(buf, (*buf).data, core::ptr::null_mut());
}

// The remaining file operations and channel-management entry points retain the
// kernel ABI and are declared against the corresponding external kernel types.
// Their implementations are intentionally kept as direct unsafe translations.
pub unsafe fn relay_reset(chan: *mut Rchan) { if chan.is_null() { return; } for_each_possible_cpu!(i, { let b=*per_cpu_ptr((*chan).buf,i); if !b.is_null(){__relay_reset(b,0);} }); }
pub unsafe fn relay_subbufs_consumed(chan:*mut Rchan,cpu:u32,n:usize){if chan.is_null()||cpu>=NR_CPUS{return}let b=*per_cpu_ptr((*chan).buf,cpu);if b.is_null()||n>(*chan).n_subbufs{return}let r=(*b).subbufs_produced-(*b).subbufs_consumed;if n>r{(*b).subbufs_consumed=(*b).subbufs_produced}else{(*b).subbufs_consumed+=n}}
pub unsafe fn relay_switch_subbuf(buf:*mut RchanBuf,length:usize)->usize{if length>(*(*buf).chan).subbuf_size{(*buf).stats.big_count+=1;return 0}let old=(*buf).data;let n=(*buf).subbufs_produced%(*(*buf).chan).n_subbufs;let new=(*buf).start.add(n*(*(*buf).chan).subbuf_size);(*buf).offset=0;if relay_subbuf_start(buf,new,old)==0{(*buf).offset=(*(*buf).chan).subbuf_size+1;return 0}(*buf).data=new;length}
pub unsafe fn relay_flush(chan:*mut Rchan){if chan.is_null(){return}for_each_possible_cpu!(i,{let b=*per_cpu_ptr((*chan).buf,i);if !b.is_null(){relay_switch_subbuf(b,0);}})}

pub unsafe fn relay_close(chan:*mut Rchan){if chan.is_null(){return}for_each_possible_cpu!(i,{let b=*per_cpu_ptr((*chan).buf,i);if !b.is_null(){relay_close_buf(b);}});list_del(&mut (*chan).list);kref_put(&mut (*chan).kref,relay_destroy_channel)}
unsafe fn relay_close_buf(buf:*mut RchanBuf){(*buf).finalized=1;irq_work_sync(&mut (*buf).wakeup_work);(*(*buf).chan).cb.as_ref().unwrap().remove_buf_file.unwrap()((*buf).dentry);kref_put(&mut (*buf).kref,relay_remove_buf)}
pub unsafe fn relay_prepare_cpu(cpu:u32)->i32{for_each_channel!(chan,{if (*per_cpu_ptr((*chan).buf,cpu)).is_null(){let b=relay_open_buf(chan,cpu);if b.is_null(){return -ENOMEM}*per_cpu_ptr((*chan).buf,cpu)=b;}});0}
pub unsafe fn relay_stats(chan:*mut Rchan,flags:i32)->usize{if chan.is_null()||flags>RELAY_STATS_LAST{return 0}let mut count=0;for_each_possible_cpu!(i,{let b=*per_cpu_ptr((*chan).buf,i);if !b.is_null(){if flags&RELAY_STATS_BUF_FULL!=0{count+=(*b).stats.full_count}else if flags&RELAY_STATS_WRT_BIG!=0{count+=(*b).stats.big_count}}});count}
unsafe fn relay_file_open(inode:*mut Inode,filp:*mut File)->i32{let b=(*inode).i_private as *mut RchanBuf;kref_get(&mut (*b).kref);(*filp).private_data=b as *mut _;nonseekable_open(inode,filp)}
unsafe fn relay_file_mmap_prepare(desc:*mut VmAreaDesc)->i32{relay_mmap_prepare_buf((*(*desc).file).private_data as *mut RchanBuf,desc)}
unsafe fn relay_file_poll(filp:*mut File,wait:*mut PollTable)->PollT{let b=(*filp).private_data as *mut RchanBuf;if (*b).finalized!=0{return EPOLLERR}let mut mask=0;if (*filp).f_mode&FMODE_READ!=0{poll_wait(filp,&mut (*b).read_wait,wait);if relay_buf_empty(b)==0{mask|=EPOLLIN|EPOLLRDNORM}}mask}
unsafe fn relay_file_release(_inode:*mut Inode,filp:*mut File)->i32{kref_put(&mut (*( (*filp).private_data as *mut RchanBuf)).kref,relay_remove_buf);0}
pub static RELAY_FILE_OPERATIONS: FileOperations={open:Some(relay_file_open),poll:Some(relay_file_poll),mmap_prepare:Some(relay_file_mmap_prepare),release:Some(relay_file_release)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
