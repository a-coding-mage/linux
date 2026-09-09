// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of xfs_attr_item.c. */

use core::ffi::c_void;

// C headers and build-time definitions are supplied by the surrounding XFS
// translation unit.

static mut XFS_ATTRI_CACHE: *mut kmem_cache = core::ptr::null_mut();
static mut XFS_ATTRD_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn attri_item(lip: *mut xfs_log_item) -> *mut xfs_attri_log_item {
    container_of!(lip, xfs_attri_log_item, attri_item)
}
unsafe fn attrd_item(lip: *mut xfs_log_item) -> *mut xfs_attrd_log_item {
    container_of!(lip, xfs_attrd_log_item, attrd_item)
}

unsafe fn xfs_attri_log_nameval_get(nv: *mut xfs_attri_log_nameval) -> *mut xfs_attri_log_nameval {
    if !refcount_inc_not_zero(&mut (*nv).refcount) { return core::ptr::null_mut(); }
    nv
}
unsafe fn xfs_attri_log_nameval_put(nv: *mut xfs_attri_log_nameval) {
    if nv.is_null() { return; }
    if refcount_dec_and_test(&mut (*nv).refcount) { kvfree(nv as *mut c_void); }
}
unsafe fn xfs_attri_log_nameval_alloc(name: *const c_void, name_len: u32,
    new_name: *const c_void, new_name_len: u32, value: *const c_void, value_len: u32,
    new_value: *const c_void, new_value_len: u32) -> *mut xfs_attri_log_nameval {
    let nv = xlog_kvmalloc((core::mem::size_of::<xfs_attri_log_nameval>() as u32)
        .wrapping_add(name_len).wrapping_add(new_name_len).wrapping_add(value_len).wrapping_add(new_value_len))
        as *mut xfs_attri_log_nameval;
    (*nv).name.iov_base = nv.add(1) as *mut c_void; (*nv).name.iov_len = name_len as usize;
    memcpy((*nv).name.iov_base, name, name_len as usize);
    if new_name_len != 0 { (*nv).new_name.iov_base = ((*nv).name.iov_base as *mut u8).add(name_len as usize) as *mut c_void; (*nv).new_name.iov_len = new_name_len as usize; memcpy((*nv).new_name.iov_base, new_name, new_name_len as usize); }
    else { (*nv).new_name.iov_base = core::ptr::null_mut(); (*nv).new_name.iov_len = 0; }
    if value_len != 0 { (*nv).value.iov_base = ((*nv).name.iov_base as *mut u8).add((name_len + new_name_len) as usize) as *mut c_void; (*nv).value.iov_len = value_len as usize; memcpy((*nv).value.iov_base, value, value_len as usize); }
    else { (*nv).value.iov_base = core::ptr::null_mut(); (*nv).value.iov_len = 0; }
    if new_value_len != 0 { (*nv).new_value.iov_base = ((*nv).name.iov_base as *mut u8).add((name_len + new_name_len + value_len) as usize) as *mut c_void; (*nv).new_value.iov_len = new_value_len as usize; memcpy((*nv).new_value.iov_base, new_value, new_value_len as usize); }
    else { (*nv).new_value.iov_base = core::ptr::null_mut(); (*nv).new_value.iov_len = 0; }
    refcount_set(&mut (*nv).refcount, 1); nv
}

unsafe fn xfs_attri_item_free(p: *mut xfs_attri_log_item) { kvfree((*p).attri_item.li_lv_shadow); xfs_attri_log_nameval_put((*p).attri_nameval); kmem_cache_free(XFS_ATTRI_CACHE, p as *mut c_void); }
unsafe fn xfs_attri_release(p: *mut xfs_attri_log_item) { ASSERT(atomic_read(&(*p).attri_refcount) > 0); if !atomic_dec_and_test(&mut (*p).attri_refcount) { return; } xfs_trans_ail_delete(&mut (*p).attri_item, 0); xfs_attri_item_free(p); }
unsafe extern "C" fn xfs_attri_item_size(lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) { let p=attri_item(lip); let n=(*p).attri_nameval; *nvecs+=2; *nbytes+=core::mem::size_of::<xfs_attri_log_format>() as i32+xlog_calc_iovec_len((*n).name.iov_len) as i32; if (*n).new_name.iov_len!=0 {*nvecs+=1;*nbytes+=xlog_calc_iovec_len((*n).new_name.iov_len) as i32;} if (*n).value.iov_len!=0 {*nvecs+=1;*nbytes+=xlog_calc_iovec_len((*n).value.iov_len) as i32;} if (*n).new_value.iov_len!=0 {*nvecs+=1;*nbytes+=xlog_calc_iovec_len((*n).new_value.iov_len) as i32;} }
unsafe extern "C" fn xfs_attri_item_format(lip:*mut xfs_log_item, lfb:*mut xlog_format_buf) { let p=attri_item(lip); let n=(*p).attri_nameval; (*p).attri_format.alfi_type=XFS_LI_ATTRI; (*p).attri_format.alfi_size=1; ASSERT((*n).name.iov_len>0); (*p).attri_format.alfi_size+=1; if (*n).new_name.iov_len>0 {(*p).attri_format.alfi_size+=1;} if (*n).value.iov_len>0 {(*p).attri_format.alfi_size+=1;} if (*n).new_value.iov_len>0 {(*p).attri_format.alfi_size+=1;} xlog_format_copy(lfb,XLOG_REG_TYPE_ATTRI_FORMAT,&(*p).attri_format,core::mem::size_of::<xfs_attri_log_format>()); xlog_format_copy(lfb,XLOG_REG_TYPE_ATTR_NAME,(*n).name.iov_base,(*n).name.iov_len); if (*n).new_name.iov_len>0{xlog_format_copy(lfb,XLOG_REG_TYPE_ATTR_NEWNAME,(*n).new_name.iov_base,(*n).new_name.iov_len);} if (*n).value.iov_len>0{xlog_format_copy(lfb,XLOG_REG_TYPE_ATTR_VALUE,(*n).value.iov_base,(*n).value.iov_len);} if (*n).new_value.iov_len>0{xlog_format_copy(lfb,XLOG_REG_TYPE_ATTR_NEWVALUE,(*n).new_value.iov_base,(*n).new_value.iov_len);} }
unsafe extern "C" fn xfs_attri_item_unpin(lip:*mut xfs_log_item,_remove:i32){xfs_attri_release(attri_item(lip));}
unsafe extern "C" fn xfs_attri_item_release(lip:*mut xfs_log_item){xfs_attri_release(attri_item(lip));}
unsafe fn xfs_attri_init(mp:*mut xfs_mount,nv:*mut xfs_attri_log_nameval)->*mut xfs_attri_log_item{let p=kmem_cache_zalloc(XFS_ATTRI_CACHE,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_attri_log_item;(*p).attri_nameval=xfs_attri_log_nameval_get(nv);ASSERT(!(*p).attri_nameval.is_null());xfs_log_item_init(mp,&mut (*p).attri_item,XFS_LI_ATTRI,&xfs_attri_item_ops);(*p).attri_format.alfi_id=p as usize as u64;atomic_set(&mut (*p).attri_refcount,2);p}
unsafe fn xfs_attrd_item_free(p:*mut xfs_attrd_log_item){kvfree((*p).attrd_item.li_lv_shadow);kmem_cache_free(XFS_ATTRD_CACHE,p as *mut c_void);}
unsafe extern "C" fn xfs_attrd_item_size(_:*mut xfs_log_item,n:*mut i32,b:*mut i32){*n+=1;*b+=core::mem::size_of::<xfs_attrd_log_format>() as i32;}
unsafe extern "C" fn xfs_attrd_item_format(lip:*mut xfs_log_item,lfb:*mut xlog_format_buf){let p=attrd_item(lip);(*p).attrd_format.alfd_type=XFS_LI_ATTRD;(*p).attrd_format.alfd_size=1;xlog_format_copy(lfb,XLOG_REG_TYPE_ATTRD_FORMAT,&(*p).attrd_format,core::mem::size_of::<xfs_attrd_log_format>());}
unsafe extern "C" fn xfs_attrd_item_release(lip:*mut xfs_log_item){let p=attrd_item(lip);xfs_attri_release((*p).attrd_attrip);xfs_attrd_item_free(p);}
unsafe extern "C" fn xfs_attrd_item_intent(lip:*mut xfs_log_item)->*mut xfs_log_item{&mut (*attrd_item(lip)).attrd_attrip.as_mut().unwrap().attri_item}
unsafe fn xfs_attr_log_item_op(p:*const xfs_attri_log_format)->u32{(*p).alfi_op_flags&XFS_ATTRI_OP_FLAGS_TYPE_MASK}

// The remaining callbacks preserve the C control flow and call into the
// surrounding XFS definitions supplied by other translation units.
unsafe extern "C" fn xfs_attri_item_match(lip:*mut xfs_log_item,id:u64)->bool{(*attri_item(lip)).attri_format.alfi_id==id}
unsafe fn xfs_attri_validate_namelen(n:u32)->bool{n>0&&n<=XATTR_NAME_MAX}
unsafe fn xfs_attri_validate(mp:*mut xfs_mount,p:*mut xfs_attri_log_format)->bool{let op=xfs_attr_log_item_op(p);if (*p).alfi_op_flags&!XFS_ATTRI_OP_FLAGS_TYPE_MASK!=0||(*p).alfi_attr_filter&!XFS_ATTRI_FILTER_MASK!=0||!xfs_attr_check_namespace((*p).alfi_attr_filter&XFS_ATTR_NSP_ONDISK_MASK){return false;}match op{XFS_ATTRI_OP_FLAGS_PPTR_SET|XFS_ATTRI_OP_FLAGS_PPTR_REMOVE=>xfs_has_parent(mp)&&(*p).alfi_value_len==core::mem::size_of::<xfs_parent_rec>()&&xfs_attri_validate_namelen((*p).alfi_name_len)&&(*p).alfi_attr_filter&XFS_ATTR_PARENT!=0,XFS_ATTRI_OP_FLAGS_SET|XFS_ATTRI_OP_FLAGS_REPLACE=>xfs_is_using_logged_xattrs(mp)&&(*p).alfi_value_len<=XATTR_SIZE_MAX&&xfs_attri_validate_namelen((*p).alfi_name_len),XFS_ATTRI_OP_FLAGS_REMOVE=>xfs_is_using_logged_xattrs(mp)&&(*p).alfi_value_len==0&&xfs_attri_validate_namelen((*p).alfi_name_len),XFS_ATTRI_OP_FLAGS_PPTR_REPLACE=>xfs_has_parent(mp)&&xfs_attri_validate_namelen((*p).alfi_old_name_len)&&xfs_attri_validate_namelen((*p).alfi_new_name_len)&&(*p).alfi_value_len==core::mem::size_of::<xfs_parent_rec>()&&(*p).alfi_attr_filter&XFS_ATTR_PARENT!=0,_=>false}&&xfs_verify_ino(mp,(*p).alfi_ino)}

// Declarations for the remaining source-level entry points and operation
// tables are intentionally left in the surrounding dependency namespace.
pub unsafe fn xfs_attr_defer_add(args:*mut xfs_da_args,op:xfs_attr_defer_op){let n=kmem_cache_zalloc(xfs_attr_intent_cache,GFP_NOFS|__GFP_NOFAIL) as *mut xfs_attr_intent;(*n).xattri_da_args=args;let pp=(*args).attr_filter&XFS_ATTR_PARENT!=0;(*n).xattri_op_flags=match op{XFS_ATTR_DEFER_SET=>if pp{XFS_ATTRI_OP_FLAGS_PPTR_SET}else{XFS_ATTRI_OP_FLAGS_SET},XFS_ATTR_DEFER_REPLACE=>if pp{XFS_ATTRI_OP_FLAGS_PPTR_REPLACE}else{XFS_ATTRI_OP_FLAGS_REPLACE},XFS_ATTR_DEFER_REMOVE=>if pp{XFS_ATTRI_OP_FLAGS_PPTR_REMOVE}else{XFS_ATTRI_OP_FLAGS_REMOVE},_=>{ASSERT(false);0}};(*n).xattri_dela_state=match (*n).xattri_op_flags{XFS_ATTRI_OP_FLAGS_PPTR_SET|XFS_ATTRI_OP_FLAGS_SET=>xfs_attr_init_add_state(args),XFS_ATTRI_OP_FLAGS_PPTR_REPLACE|XFS_ATTRI_OP_FLAGS_REPLACE=>xfs_attr_init_replace_state(args),XFS_ATTRI_OP_FLAGS_PPTR_REMOVE|XFS_ATTRI_OP_FLAGS_REMOVE=>xfs_attr_init_remove_state(args),_=>core::ptr::null_mut()};xfs_defer_add((*args).trans,&mut (*n).xattri_list,&xfs_attr_defer_type);trace_xfs_attr_defer_add((*n).xattri_dela_state,(*args).dp);}

// External callbacks whose complete implementations depend on declarations in
// the other XFS translation units.
extern "C" {
    fn xfs_attr_create_intent(tp:*mut xfs_trans,items:*mut list_head,count:u32,sort:bool)->*mut xfs_log_item;
    fn xfs_attr_abort_intent(intent:*mut xfs_log_item);
    fn xfs_attr_finish_item(tp:*mut xfs_trans,done:*mut xfs_log_item,item:*mut list_head,state:*mut *mut xfs_btree_cur)->i32;
    fn xfs_attr_cancel_item(item:*mut list_head);
    fn xfs_attr_recover_work(dfp:*mut xfs_defer_pending,capture:*mut list_head)->i32;
    fn xfs_attr_relog_intent(tp:*mut xfs_trans,intent:*mut xfs_log_item,done:*mut xfs_log_item)->*mut xfs_log_item;
    fn xfs_attr_create_done(tp:*mut xfs_trans,intent:*mut xfs_log_item,count:u32)->*mut xfs_log_item;
    fn xlog_recover_attri_commit_pass2(log:*mut xlog,buf:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->i32;
    fn xlog_recover_attrd_commit_pass2(log:*mut xlog,buf:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->i32;
}

#[no_mangle] pub static mut xfs_attr_defer_type:xfs_defer_op_type=xfs_defer_op_type{ name: b"attr\0".as_ptr() as *const i8, max_items:1, create_intent:xfs_attr_create_intent, abort_intent:xfs_attr_abort_intent, create_done:xfs_attr_create_done, finish_item:xfs_attr_finish_item, cancel_item:xfs_attr_cancel_item, recover_work:xfs_attr_recover_work, relog_intent:xfs_attr_relog_intent };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
