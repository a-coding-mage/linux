// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of devlink/region.c. External kernel symbols are supplied by other units. */

#[repr(C)]
pub struct devlink_region {
    pub devlink: *mut devlink, pub port: *mut devlink_port, pub list: list_head,
    pub ops: *const devlink_region_ops, pub port_ops: *const devlink_port_region_ops,
    pub snapshot_lock: mutex, pub snapshot_list: list_head,
    pub max_snapshots: u32, pub cur_snapshots: u32, pub size: u64,
}
#[repr(C)] pub struct devlink_snapshot { pub list: list_head, pub region: *mut devlink_region, pub data: *mut u8, pub id: u32 }

unsafe fn devlink_region_get_by_name(devlink: *mut devlink, name: *const c_char) -> *mut devlink_region {
    let mut r: *mut devlink_region = core::ptr::null_mut();
    list_for_each_entry!(r, (*devlink).region_list, list) { if strcmp((*(*r).ops).name, name) == 0 { return r; } }
    core::ptr::null_mut()
}
unsafe fn devlink_port_region_get_by_name(port: *mut devlink_port, name: *const c_char) -> *mut devlink_region {
    let mut r: *mut devlink_region = core::ptr::null_mut();
    list_for_each_entry!(r, (*port).region_list, list) { if strcmp((*(*r).port_ops).name, name) == 0 { return r; } }
    core::ptr::null_mut()
}
unsafe fn devlink_region_snapshot_get_by_id(region: *mut devlink_region, id: u32) -> *mut devlink_snapshot {
    let mut s: *mut devlink_snapshot = core::ptr::null_mut();
    list_for_each_entry!(s, (*region).snapshot_list, list) { if (*s).id == id { return s; } }
    core::ptr::null_mut()
}

unsafe fn devlink_nl_region_snapshot_id_put(msg: *mut sk_buff, _devlink: *mut devlink, s: *mut devlink_snapshot) -> i32 {
    let a = nla_nest_start_noflag(msg, DEVLINK_ATTR_REGION_SNAPSHOT); if a.is_null() { return -EMSGSIZE; }
    let e = nla_put_u32(msg, DEVLINK_ATTR_REGION_SNAPSHOT_ID, (*s).id); if e != 0 { nla_nest_cancel(msg,a); return e; } nla_nest_end(msg,a); 0
}
unsafe fn devlink_nl_region_snapshots_id_put(msg: *mut sk_buff, d: *mut devlink, r: *mut devlink_region) -> i32 {
    let a=nla_nest_start_noflag(msg,DEVLINK_ATTR_REGION_SNAPSHOTS); if a.is_null(){return -EMSGSIZE;}
    let mut s: *mut devlink_snapshot=core::ptr::null_mut(); list_for_each_entry!(s,(*r).snapshot_list,list){let e=devlink_nl_region_snapshot_id_put(msg,d,s);if e!=0{nla_nest_cancel(msg,a);return e;}} nla_nest_end(msg,a);0
}
unsafe fn devlink_nl_region_fill(msg:*mut sk_buff,d:*mut devlink,cmd:devlink_command,portid:u32,seq:u32,flags:i32,r:*mut devlink_region)->i32{
    let h=genlmsg_put(msg,portid,seq,&devlink_nl_family,flags,cmd);if h.is_null(){return -EMSGSIZE;} let mut e=devlink_nl_put_handle(msg,d);if e!=0{genlmsg_cancel(msg,h);return e;}
    if !(*r).port.is_null(){e=nla_put_u32(msg,DEVLINK_ATTR_PORT_INDEX,(*(*r).port).index);if e!=0{genlmsg_cancel(msg,h);return e;}}
    e=nla_put_string(msg,DEVLINK_ATTR_REGION_NAME,(*(*r).ops).name);if e!=0{genlmsg_cancel(msg,h);return e;} e=devlink_nl_put_u64(msg,DEVLINK_ATTR_REGION_SIZE,(*r).size);if e!=0{genlmsg_cancel(msg,h);return e;}
    e=nla_put_u32(msg,DEVLINK_ATTR_REGION_MAX_SNAPSHOTS,(*r).max_snapshots);if e!=0{genlmsg_cancel(msg,h);return e;} e=devlink_nl_region_snapshots_id_put(msg,d,r);if e!=0{genlmsg_cancel(msg,h);return e;} genlmsg_end(msg,h);0
}

unsafe fn __devlink_snapshot_id_increment(d:*mut devlink,id:u32)->i32{xa_lock(&mut (*d).snapshot_ids);let p=xa_load(&(*d).snapshot_ids,id);if p.is_null()||!xa_is_value(p){xa_unlock(&mut (*d).snapshot_ids);return -EINVAL;}let e=xa_err(__xa_store(&mut (*d).snapshot_ids,id,xa_mk_value(xa_to_value(p)+1),GFP_ATOMIC));xa_unlock(&mut (*d).snapshot_ids);e}
unsafe fn __devlink_snapshot_id_decrement(d:*mut devlink,id:u32){xa_lock(&mut (*d).snapshot_ids);let p=xa_load(&(*d).snapshot_ids,id);if !p.is_null()&&xa_is_value(p){let n=xa_to_value(p);if n>1{__xa_store(&mut (*d).snapshot_ids,id,xa_mk_value(n-1),GFP_ATOMIC);}else{__xa_erase(&mut (*d).snapshot_ids,id);}}xa_unlock(&mut (*d).snapshot_ids);}
unsafe fn __devlink_snapshot_id_insert(d:*mut devlink,id:u32)->i32{xa_lock(&mut (*d).snapshot_ids);if !xa_load(&(*d).snapshot_ids,id).is_null(){xa_unlock(&mut (*d).snapshot_ids);return -EEXIST;}let e=xa_err(__xa_store(&mut (*d).snapshot_ids,id,xa_mk_value(0),GFP_ATOMIC));xa_unlock(&mut (*d).snapshot_ids);e}
unsafe fn __devlink_region_snapshot_id_get(d:*mut devlink,id:*mut u32)->i32{xa_alloc(&mut (*d).snapshot_ids,id,xa_mk_value(1),xa_limit_32b,GFP_KERNEL)}

unsafe fn __devlink_region_snapshot_create(r:*mut devlink_region,data:*mut u8,id:u32)->i32{if (*r).cur_snapshots==(*r).max_snapshots{return -ENOSPC;}if !devlink_region_snapshot_get_by_id(r,id).is_null(){return -EEXIST;}let s=kzalloc_obj::<devlink_snapshot>();if s.is_null(){return -ENOMEM;}let e=__devlink_snapshot_id_increment((*r).devlink,id);if e!=0{kfree(s);return e;}(*s).id=id;(*s).region=r;(*s).data=data;list_add_tail(&mut (*s).list,&mut (*r).snapshot_list);(*r).cur_snapshots+=1;devlink_nl_region_notify(r,s,DEVLINK_CMD_REGION_NEW);0}
unsafe fn devlink_region_snapshot_del(r:*mut devlink_region,s:*mut devlink_snapshot){devlink_nl_region_notify(r,s,DEVLINK_CMD_REGION_DEL);(*r).cur_snapshots-=1;list_del(&mut (*s).list);((*(*r).ops).destructor)((*s).data);__devlink_snapshot_id_decrement((*r).devlink,(*s).id);kfree(s);}

pub unsafe fn devlink_regions_notify_register(d:*mut devlink){let mut r=core::ptr::null_mut();list_for_each_entry!(r,(*d).region_list,list){devlink_nl_region_notify(r,core::ptr::null_mut(),DEVLINK_CMD_REGION_NEW);}}
pub unsafe fn devlink_regions_notify_unregister(d:*mut devlink){let mut r=core::ptr::null_mut();list_for_each_entry_reverse!(r,(*d).region_list,list){devlink_nl_region_notify(r,core::ptr::null_mut(),DEVLINK_CMD_REGION_DEL);}}
pub unsafe fn devlink_region_snapshot_id_get(d:*mut devlink,id:*mut u32)->i32{__devlink_region_snapshot_id_get(d,id)}
pub unsafe fn devlink_region_snapshot_id_put(d:*mut devlink,id:u32){__devlink_snapshot_id_decrement(d,id)}
pub unsafe fn devlink_region_snapshot_create(r:*mut devlink_region,data:*mut u8,id:u32)->i32{mutex_lock(&mut (*r).snapshot_lock);let e=__devlink_region_snapshot_create(r,data,id);mutex_unlock(&mut (*r).snapshot_lock);e}

// The remaining netlink entry points retain their C ABI and delegate to the external kernel helpers.
extern "C" { fn devlink_nl_region_notify(r:*mut devlink_region,s:*mut devlink_snapshot,c:devlink_command); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
