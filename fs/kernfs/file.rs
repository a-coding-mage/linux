// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of fs/kernfs/file.c.  Kernel types and helpers are supplied externally. */

use core::ptr;

#[repr(C)]
pub struct kernfs_open_node {
    pub rcu_head: rcu_head,
    pub event: atomic_t,
    pub poll: wait_queue_head_t,
    pub files: list_head,
    pub nr_mmapped: c_uint,
    pub nr_to_release: c_uint,
}

static mut kernfs_notify_list: *mut kernfs_node = ptr::addr_of_mut!(kernfs_notify_list) as *mut kernfs_node;
static mut kernfs_notify_lock: spinlock_t = spinlock_t::new();

#[inline] unsafe fn kernfs_open_file_mutex_ptr(kn: *mut kernfs_node) -> *mut mutex { kernfs_node_lock_ptr(kn) }
#[inline] unsafe fn kernfs_open_file_mutex_lock(kn: *mut kernfs_node) -> *mut mutex { kernfs_node_lock(kn) }

unsafe fn of_on(of: *mut kernfs_open_file) -> *mut kernfs_open_node {
    rcu_dereference_protected((*(*of).kn).attr.open, !list_empty(&(*of).list))
}
unsafe fn kernfs_get_active_of(of: *mut kernfs_open_file) -> *mut kernfs_open_file {
    if (*of).released || !kernfs_get_active((*of).kn) { ptr::null_mut() } else { of }
}
unsafe fn kernfs_put_active_of(of: *mut kernfs_open_file) { kernfs_put_active((*of).kn); }
unsafe fn kernfs_deref_open_node_locked(kn: *mut kernfs_node) -> *mut kernfs_open_node {
    rcu_dereference_protected((*kn).attr.open, lockdep_is_held(kernfs_open_file_mutex_ptr(kn)))
}
unsafe fn kernfs_of(file: *mut file) -> *mut kernfs_open_file { (*( (*file).private_data as *mut seq_file)).private }
unsafe fn kernfs_ops(kn: *mut kernfs_node) -> *const kernfs_ops { if (*kn).flags & KERNFS_LOCKDEP != 0 { lockdep_assert_held(kn); } (*kn).attr.ops }

unsafe fn kernfs_seq_stop_active(sf: *mut seq_file, v: *mut c_void) {
    let of = (*sf).private; let ops = kernfs_ops((*of).kn);
    if !(*ops).seq_stop.is_none() { (*ops).seq_stop.unwrap()(sf, v); }
    kernfs_put_active_of(of);
}
unsafe fn kernfs_seq_start(sf: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let of = (*sf).private; mutex_lock(&mut (*of).mutex);
    if kernfs_get_active_of(of).is_null() { return ERR_PTR(-ENODEV); }
    let ops = kernfs_ops((*of).kn);
    if let Some(f) = (*ops).seq_start { let n = f(sf, ppos); if n == ERR_PTR(-ENODEV) { kernfs_seq_stop_active(sf,n); } n } else { single_start(sf,ppos) }
}
unsafe fn kernfs_seq_next(sf: *mut seq_file, v: *mut c_void, ppos: *mut loff_t) -> *mut c_void {
    let of=(*sf).private; let ops=kernfs_ops((*of).kn);
    if let Some(f)=(*ops).seq_next { let n=f(sf,v,ppos); if n==ERR_PTR(-ENODEV) { kernfs_seq_stop_active(sf,n); } n } else { *ppos=(*ppos).wrapping_add(1); ptr::null_mut() }
}
unsafe fn kernfs_seq_stop(sf:*mut seq_file,v:*mut c_void){let of=(*sf).private;if v!=ERR_PTR(-ENODEV){kernfs_seq_stop_active(sf,v)}mutex_unlock(&mut (*of).mutex);}
unsafe fn kernfs_seq_show(sf:*mut seq_file,v:*mut c_void)->c_int{let of=(*sf).private;(*of).event=atomic_read(&(*of_on(of)).event);(*kernfs_ops((*of).kn)).seq_show.unwrap()(sf,v)}

#[no_mangle] pub static kernfs_seq_ops: seq_operations=seq_operations{start:Some(kernfs_seq_start),next:Some(kernfs_seq_next),stop:Some(kernfs_seq_stop),show:Some(kernfs_seq_show)};

unsafe fn kernfs_file_read_iter(iocb:*mut kiocb,iter:*mut iov_iter)->isize{let of=kernfs_of((*iocb).ki_filp);let mut len=min(iov_iter_count(iter),PAGE_SIZE);let mut buf=(*of).prealloc_buf;if !buf.is_null(){mutex_lock(&mut (*of).prealloc_mutex)}else{buf=kmalloc(len,GFP_KERNEL)}if buf.is_null(){return -ENOMEM}mutex_lock(&mut (*of).mutex);if kernfs_get_active_of(of).is_null(){len=-ENODEV;mutex_unlock(&mut (*of).mutex);return kernfs_file_read_out(of,buf,len)}(*of).event=atomic_read(&(*of_on(of)).event);let ops=kernfs_ops((*of).kn);len=if let Some(f)=(*ops).read{f(of,buf,len,(*iocb).ki_pos)}else{-EINVAL};kernfs_put_active_of(of);mutex_unlock(&mut (*of).mutex);if len>=0&&copy_to_iter(buf,len,iter)!=len{len=-EFAULT}if len>=0{(*iocb).ki_pos+=len}kernfs_file_read_out(of,buf,len)}
unsafe fn kernfs_file_read_out(of:*mut kernfs_open_file,buf:*mut c_char,len:isize)->isize{if buf==(*of).prealloc_buf{mutex_unlock(&mut (*of).prealloc_mutex)}else{kfree(buf)}len}
unsafe fn kernfs_fop_read_iter(i:*mut kiocb,x:*mut iov_iter)->isize{if (*kernfs_of((*i).ki_filp).kn).flags&KERNFS_HAS_SEQ_SHOW!=0{seq_read_iter(i,x)}else{kernfs_file_read_iter(i,x)}}
unsafe fn kernfs_fop_write_iter(i:*mut kiocb,it:*mut iov_iter)->isize{let of=kernfs_of((*i).ki_filp);let mut len=iov_iter_count(it);if (*of).atomic_write_len!=0&&len>(*of).atomic_write_len{return -E2BIG}if (*of).atomic_write_len==0{len=min(len,PAGE_SIZE)}let mut b=(*of).prealloc_buf;if !b.is_null(){mutex_lock(&mut (*of).prealloc_mutex)}else{b=kmalloc(len+1,GFP_KERNEL)}if b.is_null(){return -ENOMEM}if copy_from_iter(b,len,it)!=len{len=-EFAULT}else{*b.add(len as usize)=0;mutex_lock(&mut (*of).mutex);if kernfs_get_active_of(of).is_null(){len=-ENODEV}else{let o=kernfs_ops((*of).kn);len=if let Some(f)=(*o).write{f(of,b,len,(*i).ki_pos)}else{-EINVAL};kernfs_put_active_of(of);mutex_unlock(&mut (*of).mutex);if len>0{(*i).ki_pos+=len}}}if b==(*of).prealloc_buf{mutex_unlock(&mut (*of).prealloc_mutex)}else{kfree(b)}len}

unsafe fn kernfs_vma_open(v:*mut vm_area_struct){let of=kernfs_of((*v).vm_file);if let Some(vm)=(*of).vm_ops{if !kernfs_get_active_of(of).is_null(){if let Some(f)=vm.open{f(v)}kernfs_put_active_of(of)}}}
unsafe fn kernfs_vma_fault(m:*mut vm_fault)->vm_fault_t{let of=kernfs_of((*(*m).vma).vm_file);if (*of).vm_ops.is_none()||kernfs_get_active_of(of).is_null(){return VM_FAULT_SIGBUS}let r=(*of).vm_ops.unwrap().fault.map(|f|f(m)).unwrap_or(VM_FAULT_SIGBUS);kernfs_put_active_of(of);r}
unsafe fn kernfs_vma_page_mkwrite(m:*mut vm_fault)->vm_fault_t{let of=kernfs_of((*(*m).vma).vm_file);if (*of).vm_ops.is_none()||kernfs_get_active_of(of).is_null(){return VM_FAULT_SIGBUS}let r=if let Some(f)=(*of).vm_ops.unwrap().page_mkwrite{f(m)}else{file_update_time((*(*m).vma).vm_file);0};kernfs_put_active_of(of);r}
unsafe fn kernfs_vma_access(v:*mut vm_area_struct,a:ulong,b:*mut c_void,l:c_int,w:c_int)->c_int{let of=kernfs_of((*v).vm_file);if (*of).vm_ops.is_none()||kernfs_get_active_of(of).is_null(){return -EINVAL}let r=(*of).vm_ops.unwrap().access.map(|f|f(v,a,b,l,w)).unwrap_or(-EINVAL);kernfs_put_active_of(of);r}
pub static kernfs_vm_ops: vm_operations_struct=vm_operations_struct{open:Some(kernfs_vma_open),fault:Some(kernfs_vma_fault),page_mkwrite:Some(kernfs_vma_page_mkwrite),access:Some(kernfs_vma_access)};

// The remaining file-operation and notification routines retain the C control flow and
// kernel helper calls; declarations are resolved by the surrounding kernel translation.
unsafe fn kernfs_fop_poll(f:*mut file,w:*mut poll_table)->__poll_t{let of=kernfs_of(f);if kernfs_get_active_of(of).is_null(){return DEFAULT_POLLMASK|EPOLLERR|EPOLLPRI}let r=(*(*of).kn).attr.ops.as_ref().unwrap().poll.map(|p|p(of,w)).unwrap_or_else(||kernfs_generic_poll(of,w));kernfs_put_active_of(of);r}
pub unsafe fn kernfs_generic_poll(of:*mut kernfs_open_file,w:*mut poll_table)->__poll_t{let on=of_on(of);poll_wait((*of).file,&mut (*on).poll,w);if (*of).event!=atomic_read(&(*on).event){DEFAULT_POLLMASK|EPOLLERR|EPOLLPRI}else{DEFAULT_POLLMASK}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
