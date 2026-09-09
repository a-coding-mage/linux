/* Direct Rust translation of rdma.c. External kernel and RDS symbols are
 * intentionally referenced but not defined here. */

unsafe fn rds_pages_in_vec(vec: *mut rds_iovec) -> u32 {
    if ((*vec).addr.wrapping_add((*vec).bytes) <= (*vec).addr) || (*vec).bytes > u32::MAX as u64 { return 0; }
    (((*vec).addr.wrapping_add((*vec).bytes).wrapping_add(PAGE_SIZE as u64 - 1)) >> PAGE_SHIFT)
        as u32 - (((*vec).addr >> PAGE_SHIFT) as u32)
}

unsafe fn rds_mr_tree_walk(root: *mut rb_root, key: u64, insert: *mut rds_mr) -> *mut rds_mr {
    let mut p = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p;
        let mr = rb_entry(parent, rds_mr, r_rb_node);
        if key < (*mr).r_key { p = &mut (*parent).rb_left; }
        else if key > (*mr).r_key { p = &mut (*parent).rb_right; }
        else { return mr; }
    }
    if !insert.is_null() {
        rb_link_node(&mut (*insert).r_rb_node, parent, p);
        rb_insert_color(&mut (*insert).r_rb_node, root);
        kref_get(&mut (*insert).r_kref);
    }
    core::ptr::null_mut()
}

unsafe fn rds_destroy_mr(mr: *mut rds_mr) {
    let rs = (*mr).r_sock; let mut trans_private: *mut core::ffi::c_void = core::ptr::null_mut(); let mut flags = 0ul;
    rdsdebug!("RDS: destroy mr key is %x refcnt %u\n", (*mr).r_key, kref_read(&(*mr).r_kref));
    spin_lock_irqsave(&mut (*rs).rs_rdma_lock, &mut flags);
    if !RB_EMPTY_NODE(&(*mr).r_rb_node) { rb_erase(&mut (*mr).r_rb_node, &mut (*rs).rs_rdma_keys); }
    trans_private = (*mr).r_trans_private; (*mr).r_trans_private = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock, flags);
    if !trans_private.is_null() { ((*(*mr).r_trans).free_mr)(trans_private, (*mr).r_invalidate); }
}

pub unsafe fn __rds_put_mr_final(kref: *mut kref) { let mr = container_of!(kref, rds_mr, r_kref); rds_destroy_mr(mr); sock_put(rds_rs_to_sk((*mr).r_sock)); kfree(mr); }

pub unsafe fn rds_rdma_drop_keys(rs: *mut rds_sock) {
    let mut flags=0ul; spin_lock_irqsave(&mut (*rs).rs_rdma_lock,&mut flags);
    loop { let node=rb_first(&mut (*rs).rs_rdma_keys); if node.is_null(){break;} let mr=rb_entry(node,rds_mr,r_rb_node);
        if (*mr).r_trans==(*rs).rs_transport { (*mr).r_invalidate=0; } rb_erase(&mut (*mr).r_rb_node,&mut (*rs).rs_rdma_keys); RB_CLEAR_NODE!(&mut (*mr).r_rb_node);
        spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock,flags); kref_put(&mut (*mr).r_kref,__rds_put_mr_final); spin_lock_irqsave(&mut (*rs).rs_rdma_lock,&mut flags); }
    spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock,flags); if !(*rs).rs_transport.is_null() && (*(*rs).rs_transport).flush_mrs.is_some(){((*(*rs).rs_transport).flush_mrs)();}
}

unsafe fn rds_pin_pages(addr: usize, nr: u32, pages: *mut *mut page, write: i32) -> i32 { let mut flags=FOLL_LONGTERM; if write!=0 {flags|=FOLL_WRITE;} let mut ret=pin_user_pages_fast(addr,nr,flags,pages); if ret>=0 && ret<nr as i32 {unpin_user_pages(pages,ret);ret=-EFAULT;} ret }

/* The remaining routines retain the original kernel-facing operations and layouts. */
pub unsafe fn rds_get_mr(rs:*mut rds_sock,optval:sockptr_t,optlen:i32)->i32 { let mut a=core::mem::MaybeUninit::<rds_get_mr_args>::uninit(); if optlen!=core::mem::size_of::<rds_get_mr_args>() as i32{return -EINVAL;} if copy_from_sockptr(a.as_mut_ptr(),optval,core::mem::size_of::<rds_get_mr_args>()){return -EFAULT;} __rds_rdma_map(rs,a.as_mut_ptr(),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) }

pub unsafe fn rds_get_mr_for_dest(rs:*mut rds_sock,optval:sockptr_t,optlen:i32)->i32 { let mut a=core::mem::MaybeUninit::<rds_get_mr_for_dest_args>::uninit(); if optlen!=core::mem::size_of::<rds_get_mr_for_dest_args>() as i32{return -EINVAL;} if copy_from_sockptr(a.as_mut_ptr(),optval,core::mem::size_of::<rds_get_mr_for_dest_args>()){return -EFAULT;} let mut n=core::mem::MaybeUninit::<rds_get_mr_args>::zeroed().assume_init(); n.vec=(*a.as_ptr()).vec;n.cookie_addr=(*a.as_ptr()).cookie_addr;n.flags=(*a.as_ptr()).flags;__rds_rdma_map(rs,&mut n,core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) }

pub unsafe fn rds_free_mr(rs:*mut rds_sock,optval:sockptr_t,optlen:i32)->i32 { let mut a=core::mem::MaybeUninit::<rds_free_mr_args>::uninit();if optlen!=core::mem::size_of::<rds_free_mr_args>() as i32{return -EINVAL;}if copy_from_sockptr(a.as_mut_ptr(),optval,core::mem::size_of::<rds_free_mr_args>()){return -EFAULT;}let a=a.assume_init();if a.cookie==0 {if (*rs).rs_transport.is_null()||(*(*rs).rs_transport).flush_mrs.is_none(){return -EINVAL;}((*(*rs).rs_transport).flush_mrs)();return 0;}let mut f=0ul;spin_lock_irqsave(&mut (*rs).rs_rdma_lock,&mut f);let mr=rds_mr_tree_walk(&mut (*rs).rs_rdma_keys,rds_rdma_cookie_key(a.cookie),core::ptr::null_mut());if !mr.is_null(){rb_erase(&mut (*mr).r_rb_node,&mut (*rs).rs_rdma_keys);RB_CLEAR_NODE!(&mut (*mr).r_rb_node);if a.flags&RDS_RDMA_INVALIDATE!=0{(*mr).r_invalidate=1;}}spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock,f);if mr.is_null(){return -EINVAL;}kref_put(&mut (*mr).r_kref,__rds_put_mr_final);0 }

/* Declarations for the large map/CM operations, whose external structures and
 * helpers are supplied by the surrounding RDS translation. */
extern "C" { fn __rds_rdma_map(rs:*mut rds_sock,args:*mut rds_get_mr_args,cookie:*mut u64,mr:*mut *mut rds_mr,cp:*mut rds_conn_path)->i32; }

pub unsafe fn rds_rdma_unuse(rs:*mut rds_sock,key:u32,force:i32){let mut f=0ul;spin_lock_irqsave(&mut (*rs).rs_rdma_lock,&mut f);let mr=rds_mr_tree_walk(&mut (*rs).rs_rdma_keys,key as u64,core::ptr::null_mut());if mr.is_null(){spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock,f);return;}kref_get(&mut (*mr).r_kref);let mut zot=0;if (*mr).r_use_once!=0||force!=0{rb_erase(&mut (*mr).r_rb_node,&mut (*rs).rs_rdma_keys);RB_CLEAR_NODE!(&mut (*mr).r_rb_node);zot=1;}spin_unlock_irqrestore(&mut (*rs).rs_rdma_lock,f);if (*(*mr).r_trans).sync_mr.is_some(){((*(*mr).r_trans).sync_mr)((*mr).r_trans_private,DMA_FROM_DEVICE);}kref_put(&mut (*mr).r_kref,__rds_put_mr_final);if zot!=0{kref_put(&mut (*mr).r_kref,__rds_put_mr_final);}}

pub unsafe fn rds_rdma_op_unpin_pages(ro:*mut rm_rdma_op){for i in 0..(*ro).op_nents{let mut p=sg_page(&mut (*ro).op_sg.add(i));unpin_user_pages_dirty_lock(&mut p,1,(*ro).op_write==0);}}
pub unsafe fn rds_rdma_free_op(ro:*mut rm_rdma_op){if !(*ro).op_odp_mr.is_null(){kref_put(&mut (*(*ro).op_odp_mr).r_kref,__rds_put_mr_final);}else if in_task()!=0||(*ro).op_write!=0{rds_rdma_op_unpin_pages(ro);}else{(*ro).op_unpin_deferred=1;}kfree((*ro).op_notifier);(*ro).op_notifier=core::ptr::null_mut();(*ro).op_active=0;(*ro).op_odp_mr=core::ptr::null_mut();}
pub unsafe fn rds_atomic_op_unpin_page(ao:*mut rm_atomic_op){let mut p=sg_page((*ao).op_sg);unpin_user_pages_dirty_lock(&mut p,1,true);}
pub unsafe fn rds_atomic_free_op(ao:*mut rm_atomic_op){if in_task()!=0{rds_atomic_op_unpin_page(ao);}else{(*ao).op_unpin_deferred=1;}kfree((*ao).op_notifier);(*ao).op_notifier=core::ptr::null_mut();(*ao).op_active=0;}

pub unsafe fn rds_rdma_pages(iov:*mut rds_iovec,n:i32)->i32{let mut total=0i32;for i in 0..n{let p=rds_pages_in_vec(iov.add(i as usize));if p==0{return -EINVAL;}total=total.wrapping_add(p as i32);if total<0{return -EINVAL;}}total}
pub unsafe fn rds_rdma_extra_size(args:*mut rds_rdma_args,iov:*mut rds_iov_vector)->i32{if (*args).nr_local==0{return -EINVAL;}if (*args).nr_local>UIO_MAXIOV{return -EMSGSIZE;}(*iov).iov=kzalloc_objs::<rds_iovec>((*args).nr_local);if (*iov).iov.is_null(){return -ENOMEM;}if copy_from_user((*iov).iov,(*args).local_vec_addr as *const _,(*args).nr_local as usize*core::mem::size_of::<rds_iovec>()){return -EFAULT;}(*iov).len=(*args).nr_local;let mut t=0i32;for i in 0..(*args).nr_local{let p=rds_pages_in_vec((*iov).iov.add(i as usize));if p==0{return -EINVAL;}t+=p as i32;if t<0{return -EINVAL;}}t*core::mem::size_of::<scatterlist>() as i32}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
