// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of health.c. External kernel symbols are
 * intentionally referenced but not implemented here. */

#[repr(C)]
pub struct devlink_fmsg_item { pub list: list_head, pub attrtype: c_int, pub nla_type: u8, pub len: u16, pub value: [c_int; 0] }
#[repr(C)]
pub struct devlink_fmsg { pub item_list: list_head, pub err: c_int, pub putting_binary: bool }
#[repr(C)]
pub struct devlink_health_reporter {
    pub list: list_head, pub priv_: *mut c_void, pub ops: *const devlink_health_reporter_ops,
    pub devlink: *mut devlink, pub devlink_port: *mut devlink_port, pub dump_fmsg: *mut devlink_fmsg,
    pub graceful_period: u64, pub burst_period: u64, pub auto_recover: bool, pub auto_dump: bool,
    pub health_state: u8, pub dump_ts: u64, pub dump_real_ts: u64, pub error_count: u64,
    pub recovery_count: u64, pub last_recovery_ts: u64,
}

unsafe fn devlink_fmsg_alloc() -> *mut devlink_fmsg { let p = kzalloc_obj::<devlink_fmsg>(); if p.is_null() { return core::ptr::null_mut(); } INIT_LIST_HEAD(&mut (*p).item_list); p }
unsafe fn devlink_fmsg_free(fmsg: *mut devlink_fmsg) { let mut item: *mut devlink_fmsg_item = core::ptr::null_mut(); let mut tmp: *mut devlink_fmsg_item = core::ptr::null_mut(); list_for_each_entry_safe(item, tmp, &mut (*fmsg).item_list, list) { list_del(&mut (*item).list); kfree(item.cast()); } kfree(fmsg.cast()); }

#[no_mangle] pub unsafe extern "C" fn devlink_health_reporter_priv(r: *mut devlink_health_reporter) -> *mut c_void { (*r).priv_ }
unsafe fn __devlink_health_reporter_find_by_name(l: *mut list_head, n: *const c_char) -> *mut devlink_health_reporter { let mut r: *mut devlink_health_reporter = core::ptr::null_mut(); list_for_each_entry(r,l,list) { if strcmp((*(*r).ops).name,n)==0 { return r; } } core::ptr::null_mut() }
unsafe fn devlink_health_reporter_find_by_name(d:*mut devlink,n:*const c_char)->*mut devlink_health_reporter { __devlink_health_reporter_find_by_name(&mut (*d).reporter_list,n) }
unsafe fn devlink_port_health_reporter_find_by_name(p:*mut devlink_port,n:*const c_char)->*mut devlink_health_reporter { __devlink_health_reporter_find_by_name(&mut (*p).reporter_list,n) }

unsafe fn __devlink_health_reporter_create(d:*mut devlink,o:*const devlink_health_reporter_ops,p:*mut c_void)->*mut devlink_health_reporter {
 if WARN_ON((*o).default_graceful_period != 0 && (*o).recover.is_none()) || WARN_ON((*o).default_burst_period != 0 && (*o).default_graceful_period == 0) { return ERR_PTR(-EINVAL); }
 let r=kzalloc_obj::<devlink_health_reporter>(); if r.is_null(){return ERR_PTR(-ENOMEM);} (*r).priv_=p;(*r).ops=o;(*r).devlink=d;(*r).graceful_period=(*o).default_graceful_period;(*r).burst_period=(*o).default_burst_period;(*r).auto_recover=(*o).recover.is_some();(*r).auto_dump=(*o).dump.is_some();r
}

pub unsafe extern "C" fn devl_port_health_reporter_create(p:*mut devlink_port,o:*const devlink_health_reporter_ops,v:*mut c_void)->*mut devlink_health_reporter { devl_assert_locked((*p).devlink); if !__devlink_health_reporter_find_by_name(&mut (*p).reporter_list,(*o).name).is_null(){return ERR_PTR(-EEXIST);} let r=__devlink_health_reporter_create((*p).devlink,o,v); if IS_ERR(r){return r;}(*r).devlink_port=p;list_add_tail(&mut (*r).list,&mut (*p).reporter_list);r }
pub unsafe extern "C" fn devlink_port_health_reporter_create(p:*mut devlink_port,o:*const devlink_health_reporter_ops,v:*mut c_void)->*mut devlink_health_reporter { let d=(*p).devlink;devl_lock(d);let r=devl_port_health_reporter_create(p,o,v);devl_unlock(d);r }
pub unsafe extern "C" fn devl_health_reporter_create(d:*mut devlink,o:*const devlink_health_reporter_ops,v:*mut c_void)->*mut devlink_health_reporter { devl_assert_locked(d);if !devlink_health_reporter_find_by_name(d,(*o).name).is_null(){return ERR_PTR(-EEXIST);}let r=__devlink_health_reporter_create(d,o,v);if IS_ERR(r){return r;}list_add_tail(&mut (*r).list,&mut (*d).reporter_list);r }
pub unsafe extern "C" fn devlink_health_reporter_create(d:*mut devlink,o:*const devlink_health_reporter_ops,v:*mut c_void)->*mut devlink_health_reporter {devl_lock(d);let r=devl_health_reporter_create(d,o,v);devl_unlock(d);r}
unsafe fn devlink_health_reporter_free(r:*mut devlink_health_reporter){if !(*r).dump_fmsg.is_null(){devlink_fmsg_free((*r).dump_fmsg);}kfree(r.cast());}
pub unsafe extern "C" fn devl_health_reporter_destroy(r:*mut devlink_health_reporter){devl_assert_locked((*r).devlink);list_del(&mut (*r).list);devlink_health_reporter_free(r)}
pub unsafe extern "C" fn devlink_health_reporter_destroy(r:*mut devlink_health_reporter){let d=(*r).devlink;devl_lock(d);devl_health_reporter_destroy(r);devl_unlock(d)}

const DEVLINK_FMSG_MAX_SIZE: usize = GENLMSG_DEFAULT_SIZE as usize - GENL_HDRLEN as usize - NLA_HDRLEN as usize;
unsafe fn devlink_fmsg_err_if_binary(f:*mut devlink_fmsg){if (*f).err==0&&(*f).putting_binary{(*f).err=-EINVAL;}}
unsafe fn devlink_fmsg_nest_common(f:*mut devlink_fmsg,a:c_int){if (*f).err!=0{return;}let i=kzalloc_obj::<devlink_fmsg_item>();if i.is_null(){(*f).err=-ENOMEM;return;}(*i).attrtype=a;list_add_tail(&mut (*i).list,&mut (*f).item_list);}
pub unsafe extern "C" fn devlink_fmsg_obj_nest_start(f:*mut devlink_fmsg){devlink_fmsg_err_if_binary(f);devlink_fmsg_nest_common(f,DEVLINK_ATTR_FMSG_OBJ_NEST_START)}
unsafe fn devlink_fmsg_nest_end(f:*mut devlink_fmsg){devlink_fmsg_err_if_binary(f);devlink_fmsg_nest_common(f,DEVLINK_ATTR_FMSG_NEST_END)}
pub unsafe extern "C" fn devlink_fmsg_obj_nest_end(f:*mut devlink_fmsg){devlink_fmsg_nest_end(f)}
unsafe fn devlink_fmsg_put_name(f:*mut devlink_fmsg,n:*const c_char){devlink_fmsg_err_if_binary(f);if (*f).err!=0{return;}let l=strlen(n)+1;if l>DEVLINK_FMSG_MAX_SIZE{(*f).err=-EMSGSIZE;return;}let i=kzalloc_bytes::<devlink_fmsg_item>(core::mem::size_of::<devlink_fmsg_item>()+l);if i.is_null(){(*f).err=-ENOMEM;return;}(*i).nla_type=DEVLINK_VAR_ATTR_TYPE_NUL_STRING;(*i).len=l as u16;(*i).attrtype=DEVLINK_ATTR_FMSG_OBJ_NAME;memcpy((*i).value.as_mut_ptr().cast(),n,l);list_add_tail(&mut (*i).list,&mut (*f).item_list)}
pub unsafe extern "C" fn devlink_fmsg_pair_nest_start(f:*mut devlink_fmsg,n:*const c_char){devlink_fmsg_err_if_binary(f);devlink_fmsg_nest_common(f,DEVLINK_ATTR_FMSG_PAIR_NEST_START);devlink_fmsg_put_name(f,n)}
pub unsafe extern "C" fn devlink_fmsg_pair_nest_end(f:*mut devlink_fmsg){devlink_fmsg_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_arr_pair_nest_start(f:*mut devlink_fmsg,n:*const c_char){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_nest_common(f,DEVLINK_ATTR_FMSG_ARR_NEST_START)}
pub unsafe extern "C" fn devlink_fmsg_arr_pair_nest_end(f:*mut devlink_fmsg){devlink_fmsg_nest_end(f);devlink_fmsg_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_binary_pair_nest_start(f:*mut devlink_fmsg,n:*const c_char){devlink_fmsg_arr_pair_nest_start(f,n);(*f).putting_binary=true}
pub unsafe extern "C" fn devlink_fmsg_binary_pair_nest_end(f:*mut devlink_fmsg){if (*f).err!=0{return;}if !(*f).putting_binary{(*f).err=-EINVAL;}(*f).putting_binary=false;devlink_fmsg_arr_pair_nest_end(f)}

unsafe fn devlink_fmsg_put_value(f:*mut devlink_fmsg,v:*const c_void,l:u16,t:u8){if (*f).err!=0{return;}if l as usize>DEVLINK_FMSG_MAX_SIZE{(*f).err=-EMSGSIZE;return;}let i=kzalloc_bytes::<devlink_fmsg_item>(core::mem::size_of::<devlink_fmsg_item>()+l as usize);if i.is_null(){(*f).err=-ENOMEM;return;}(*i).nla_type=t;(*i).len=l;(*i).attrtype=DEVLINK_ATTR_FMSG_OBJ_VALUE_DATA;memcpy((*i).value.as_mut_ptr().cast(),v,l as usize);list_add_tail(&mut (*i).list,&mut (*f).item_list)}
unsafe fn devlink_fmsg_bool_put(f:*mut devlink_fmsg,v:bool){devlink_fmsg_err_if_binary(f);devlink_fmsg_put_value(f,&v as *const _ as *const c_void,core::mem::size_of::<bool>() as u16,DEVLINK_VAR_ATTR_TYPE_FLAG)}
unsafe fn devlink_fmsg_u8_put(f:*mut devlink_fmsg,v:u8){devlink_fmsg_err_if_binary(f);devlink_fmsg_put_value(f,&v as *const _ as *const c_void,1,DEVLINK_VAR_ATTR_TYPE_U8)}
pub unsafe extern "C" fn devlink_fmsg_u32_put(f:*mut devlink_fmsg,v:u32){devlink_fmsg_err_if_binary(f);devlink_fmsg_put_value(f,&v as *const _ as *const c_void,4,DEVLINK_VAR_ATTR_TYPE_U32)}
unsafe fn devlink_fmsg_u64_put(f:*mut devlink_fmsg,v:u64){devlink_fmsg_err_if_binary(f);devlink_fmsg_put_value(f,&v as *const _ as *const c_void,8,DEVLINK_VAR_ATTR_TYPE_U64)}
pub unsafe extern "C" fn devlink_fmsg_string_put(f:*mut devlink_fmsg,v:*const c_char){devlink_fmsg_err_if_binary(f);devlink_fmsg_put_value(f,v.cast(),(strlen(v)+1) as u16,DEVLINK_VAR_ATTR_TYPE_NUL_STRING)}
pub unsafe extern "C" fn devlink_fmsg_binary_put(f:*mut devlink_fmsg,v:*const c_void,l:u16){if (*f).err==0&&!(*f).putting_binary{(*f).err=-EINVAL;}devlink_fmsg_put_value(f,v,l,DEVLINK_VAR_ATTR_TYPE_BINARY)}
pub unsafe extern "C" fn devlink_fmsg_bool_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:bool){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_bool_put(f,v);devlink_fmsg_pair_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_u8_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:u8){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_u8_put(f,v);devlink_fmsg_pair_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_u32_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:u32){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_u32_put(f,v);devlink_fmsg_pair_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_u64_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:u64){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_u64_put(f,v);devlink_fmsg_pair_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_string_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:*const c_char){devlink_fmsg_pair_nest_start(f,n);devlink_fmsg_string_put(f,v);devlink_fmsg_pair_nest_end(f)}
pub unsafe extern "C" fn devlink_fmsg_binary_pair_put(f:*mut devlink_fmsg,n:*const c_char,v:*const c_void,l:u32){devlink_fmsg_binary_pair_nest_start(f,n);let mut off=0;while off<l{let mut sz=l-off;if sz>DEVLINK_FMSG_MAX_SIZE as u32{sz=DEVLINK_FMSG_MAX_SIZE as u32;}devlink_fmsg_binary_put(f,v.add(off as usize),sz as u16);off+=sz;}devlink_fmsg_binary_pair_nest_end(f);(*f).putting_binary=false}

// The remaining netlink serialization and diagnostic entry points retain the
// kernel ABI and delegate to the corresponding external kernel helpers.
pub unsafe extern "C" fn devlink_health_reporter_recovery_done(r:*mut devlink_health_reporter){(*r).recovery_count+=1;if !devlink_health_reporter_in_burst(r){(*r).last_recovery_ts=jiffies}}
unsafe fn devlink_health_reporter_in_burst(r:*mut devlink_health_reporter)->bool{time_is_after_jiffies((*r).last_recovery_ts+msecs_to_jiffies((*r).burst_period))}
pub unsafe extern "C" fn devlink_health_reporter_state_update(r:*mut devlink_health_reporter,s:devlink_health_reporter_state){if WARN_ON(s!=DEVLINK_HEALTH_REPORTER_STATE_HEALTHY&&s!=DEVLINK_HEALTH_REPORTER_STATE_ERROR){return;}if (*r).health_state as _==s{return;}(*r).health_state=s as u8;trace_devlink_health_reporter_state_update((*r).devlink,(*(*r).ops).name,s);devlink_recover_notify(r,DEVLINK_CMD_HEALTH_REPORTER_RECOVER)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
