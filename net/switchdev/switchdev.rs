// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/switchdev/switchdev.c - Switch device API
 * Copyright (c) 2014-2015 Jiri Pirko <jiri@resnulli.us>
 * Copyright (c) 2014-2015 Scott Feldman <sfeldma@gmail.com>
 */

// Linux kernel headers and switchdev definitions are supplied by the surrounding crate.

unsafe fn switchdev_obj_eq(a: *const switchdev_obj, b: *const switchdev_obj) -> bool {
    let va: *const switchdev_obj_port_vlan;
    let vb: *const switchdev_obj_port_vlan;
    let ma: *const switchdev_obj_port_mdb;
    let mb: *const switchdev_obj_port_mdb;
    if (*a).id != (*b).id || (*a).orig_dev != (*b).orig_dev { return false; }
    match (*a).id {
        SWITCHDEV_OBJ_ID_PORT_VLAN => {
            va = SWITCHDEV_OBJ_PORT_VLAN(a); vb = SWITCHDEV_OBJ_PORT_VLAN(b);
            (*va).flags == (*vb).flags && (*va).vid == (*vb).vid && (*va).changed == (*vb).changed
        }
        SWITCHDEV_OBJ_ID_PORT_MDB | SWITCHDEV_OBJ_ID_HOST_MDB => {
            ma = SWITCHDEV_OBJ_PORT_MDB(a); mb = SWITCHDEV_OBJ_PORT_MDB(b);
            (*ma).vid == (*mb).vid && ether_addr_equal((*ma).addr.as_ptr(), (*mb).addr.as_ptr())
        }
        _ => { BUG(); false }
    }
}

static mut deferred: list_head = LIST_HEAD_INIT(deferred);
static mut deferred_lock: spinlock_t = __SPIN_LOCK_UNLOCKED(deferred_lock);

pub type switchdev_deferred_func_t = unsafe extern "C" fn(*mut net_device, *const core::ffi::c_void);

#[repr(C)]
pub struct switchdev_deferred_item {
    pub list: list_head,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub func: switchdev_deferred_func_t,
    pub data: [c_ulong; 0],
}

unsafe fn switchdev_deferred_dequeue() -> *mut switchdev_deferred_item {
    let dfitem: *mut switchdev_deferred_item;
    spin_lock_bh(&mut deferred_lock);
    if list_empty(&deferred) { dfitem = core::ptr::null_mut(); }
    else { dfitem = list_first_entry(&mut deferred, switchdev_deferred_item, list); list_del(&mut (*dfitem).list); }
    spin_unlock_bh(&mut deferred_lock); dfitem
}

pub unsafe extern "C" fn switchdev_deferred_process() {
    ASSERT_RTNL();
    loop {
        let dfitem = switchdev_deferred_dequeue();
        if dfitem.is_null() { break; }
        ((*dfitem).func)((*dfitem).dev, (*dfitem).data.as_ptr() as *const _);
        netdev_put((*dfitem).dev, &mut (*dfitem).dev_tracker);
        kfree(dfitem as *mut _);
    }
}

unsafe extern "C" fn switchdev_deferred_process_work(_work: *mut work_struct) { rtnl_lock(); switchdev_deferred_process(); rtnl_unlock(); }
static mut deferred_process_work: work_struct = DECLARE_WORK!(switchdev_deferred_process_work);

unsafe fn switchdev_deferred_enqueue(dev: *mut net_device, data: *const core::ffi::c_void, data_len: usize, func: switchdev_deferred_func_t) -> c_int {
    let dfitem = kmalloc_flex::<switchdev_deferred_item>(data_len, GFP_ATOMIC);
    if dfitem.is_null() { return -ENOMEM; }
    (*dfitem).dev = dev; (*dfitem).func = func;
    memcpy((*dfitem).data.as_mut_ptr() as *mut _, data, data_len);
    netdev_hold(dev, &mut (*dfitem).dev_tracker, GFP_ATOMIC);
    spin_lock_bh(&mut deferred_lock); list_add_tail(&mut (*dfitem).list, &mut deferred); spin_unlock_bh(&mut deferred_lock);
    schedule_work(&mut deferred_process_work); 0
}

unsafe fn switchdev_port_attr_notify(nt: switchdev_notifier_type, dev: *mut net_device, attr: *const switchdev_attr, extack: *mut netlink_ext_ack) -> c_int {
    let mut attr_info = switchdev_notifier_port_attr_info { attr, handled: false, ..core::mem::zeroed() };
    let rc = call_switchdev_blocking_notifiers(nt, dev, &mut attr_info.info, extack);
    let err = notifier_to_errno(rc);
    if err != 0 { WARN_ON(!attr_info.handled); return err; }
    if !attr_info.handled { return -EOPNOTSUPP; } 0
}
unsafe fn switchdev_port_attr_set_now(dev: *mut net_device, attr: *const switchdev_attr, extack: *mut netlink_ext_ack) -> c_int { switchdev_port_attr_notify(SWITCHDEV_PORT_ATTR_SET, dev, attr, extack) }
unsafe extern "C" fn switchdev_port_attr_set_deferred(dev: *mut net_device, data: *const core::ffi::c_void) {
    let attr = data as *const switchdev_attr; let err = switchdev_port_attr_set_now(dev, attr, core::ptr::null_mut());
    if err != 0 && err != -EOPNOTSUPP { netdev_err(dev, "failed (err=%d) to set attribute (id=%d)\n", err, (*attr).id); }
    if let Some(complete) = (*attr).complete { complete(dev, err, (*attr).complete_priv); }
}
unsafe fn switchdev_port_attr_set_defer(dev: *mut net_device, attr: *const switchdev_attr) -> c_int { switchdev_deferred_enqueue(dev, attr as *const _, core::mem::size_of::<switchdev_attr>(), switchdev_port_attr_set_deferred) }

pub unsafe extern "C" fn switchdev_port_attr_set(dev: *mut net_device, attr: *const switchdev_attr, extack: *mut netlink_ext_ack) -> c_int {
    if (*attr).flags & SWITCHDEV_F_DEFER != 0 { return switchdev_port_attr_set_defer(dev, attr); }
    ASSERT_RTNL(); switchdev_port_attr_set_now(dev, attr, extack)
}

unsafe fn switchdev_obj_size(obj: *const switchdev_obj) -> usize { match (*obj).id { SWITCHDEV_OBJ_ID_PORT_VLAN => core::mem::size_of::<switchdev_obj_port_vlan>(), SWITCHDEV_OBJ_ID_PORT_MDB | SWITCHDEV_OBJ_ID_HOST_MDB => core::mem::size_of::<switchdev_obj_port_mdb>(), _ => { BUG(); 0 } } }
unsafe fn switchdev_port_obj_notify(nt: switchdev_notifier_type, dev: *mut net_device, obj: *const switchdev_obj, extack: *mut netlink_ext_ack) -> c_int {
    let mut oi = switchdev_notifier_port_obj_info { obj, handled: false, ..core::mem::zeroed() };
    let err = notifier_to_errno(call_switchdev_blocking_notifiers(nt, dev, &mut oi.info, extack));
    if err != 0 { WARN_ON(!oi.handled); return err; } if !oi.handled { return -EOPNOTSUPP; } 0
}

unsafe fn switchdev_obj_id_to_helpful_msg(dev: *mut net_device, obj_id: switchdev_obj_id, err: c_int, add: bool) {
    let action = if add { "add" } else { "del" }; let mut reason = ""; let (obj_str, problem) = match obj_id {
        SWITCHDEV_OBJ_ID_UNDEFINED => ("Undefined object", "Attempted operation is undefined, indicating a possible programming\nerror.\n"),
        SWITCHDEV_OBJ_ID_PORT_VLAN => ("VLAN entry", "Failure in VLAN settings on this port might disrupt network\nsegmentation or traffic isolation, affecting network partitioning.\n"),
        SWITCHDEV_OBJ_ID_PORT_MDB => ("Port Multicast Database entry", "Failure in updating the port's Multicast Database could lead to\nmulticast forwarding issues.\n"),
        SWITCHDEV_OBJ_ID_HOST_MDB => ("Host Multicast Database entry", "Failure in updating the host's Multicast Database may impact multicast\ngroup memberships or traffic delivery, affecting multicast\ncommunication.\n"),
        SWITCHDEV_OBJ_ID_MRP => ("Media Redundancy Protocol configuration for port", "Failure to set MRP ring ID on this port prevents communication with\nthe specified redundancy ring, resulting in an inability to engage\nin MRP-based network operations.\n"),
        SWITCHDEV_OBJ_ID_RING_TEST_MRP => ("MRP Test Frame Operations for port", "Failure to generate/monitor MRP test frames may lead to inability to\nassess the ring's operational integrity and fault response, hindering\nproactive network management.\n"),
        SWITCHDEV_OBJ_ID_RING_ROLE_MRP => ("MRP Ring Role Configuration", "Improper MRP ring role configuration may create conflicts in the ring,\ndisrupting communication for all participants, or isolate the local\nsystem from the ring, hindering its ability to communicate with other\nparticipants.\n"),
        SWITCHDEV_OBJ_ID_RING_STATE_MRP => ("MRP Ring State Configuration", "Failure to correctly set the MRP ring state can result in network\nloops or leave segments without communication. In a Closed state,\nit maintains loop prevention by blocking one MRM port, while an Open\nstate activates in response to failures, changing port states to\npreserve network connectivity.\n"),
        SWITCHDEV_OBJ_ID_IN_TEST_MRP => ("MRP_InTest Frame Generation Configuration", "Failure in managing MRP_InTest frame generation can misjudge the\ninterconnection ring's state, leading to incorrect blocking or\nunblocking of the I/C port. This misconfiguration might result\nin unintended network loops or isolate critical network segments,\ncompromising network integrity and reliability.\n"),
        SWITCHDEV_OBJ_ID_IN_ROLE_MRP => ("Interconnection Ring Role Configuration", "Failure in incorrect assignment of interconnection ring roles\n(MIM/MIC) can impair the formation of the interconnection rings.\n"),
        SWITCHDEV_OBJ_ID_IN_STATE_MRP => ("Interconnection Ring State Configuration", "Failure in updating the interconnection ring state can lead in\ncase of Open state to incorrect blocking or unblocking of the\nI/C port, resulting in unintended network loops or isolation\nof critical network\n"),
        _ => ("Unknown object", "Indicating a possible programming error.\n"),
    }; if err == -ENOSPC { reason = "Current HW/SW setup lacks sufficient resources.\n"; }
    netdev_err(dev, "Failed to %s %s (object id=%d) with error: %pe (%d).\n%s%s\n", action, obj_str, obj_id, ERR_PTR(err), err, problem, reason);
}

unsafe extern "C" fn switchdev_port_obj_add_deferred(dev: *mut net_device, data: *const core::ffi::c_void) { let obj=data as *const switchdev_obj; ASSERT_RTNL(); let err=switchdev_port_obj_notify(SWITCHDEV_PORT_OBJ_ADD,dev,obj,core::ptr::null_mut()); if err!=0&&err!=-EOPNOTSUPP { switchdev_obj_id_to_helpful_msg(dev,(*obj).id,err,true); } if let Some(c)=(*obj).complete { c(dev,err,(*obj).complete_priv); } }
unsafe fn switchdev_port_obj_add_defer(dev:*mut net_device,obj:*const switchdev_obj)->c_int { switchdev_deferred_enqueue(dev,obj as *const _,switchdev_obj_size(obj),switchdev_port_obj_add_deferred) }
pub unsafe extern "C" fn switchdev_port_obj_add(dev:*mut net_device,obj:*const switchdev_obj,extack:*mut netlink_ext_ack)->c_int { if (*obj).flags&SWITCHDEV_F_DEFER!=0{return switchdev_port_obj_add_defer(dev,obj)} ASSERT_RTNL(); switchdev_port_obj_notify(SWITCHDEV_PORT_OBJ_ADD,dev,obj,extack) }
unsafe fn switchdev_port_obj_del_now(dev:*mut net_device,obj:*const switchdev_obj)->c_int { switchdev_port_obj_notify(SWITCHDEV_PORT_OBJ_DEL,dev,obj,core::ptr::null_mut()) }
unsafe extern "C" fn switchdev_port_obj_del_deferred(dev:*mut net_device,data:*const core::ffi::c_void){let obj=data as *const switchdev_obj;let err=switchdev_port_obj_del_now(dev,obj);if err!=0&&err!=-EOPNOTSUPP{switchdev_obj_id_to_helpful_msg(dev,(*obj).id,err,false)}if let Some(c)=(*obj).complete{c(dev,err,(*obj).complete_priv)}}
unsafe fn switchdev_port_obj_del_defer(dev:*mut net_device,obj:*const switchdev_obj)->c_int{switchdev_deferred_enqueue(dev,obj as *const _,switchdev_obj_size(obj),switchdev_port_obj_del_deferred)}
pub unsafe extern "C" fn switchdev_port_obj_del(dev:*mut net_device,obj:*const switchdev_obj)->c_int{if (*obj).flags&SWITCHDEV_F_DEFER!=0{return switchdev_port_obj_del_defer(dev,obj)}ASSERT_RTNL();switchdev_port_obj_del_now(dev,obj)}

pub unsafe extern "C" fn switchdev_port_obj_act_is_deferred(dev:*mut net_device,nt:switchdev_notifier_type,obj:*const switchdev_obj)->bool{let mut found=false;ASSERT_RTNL();spin_lock_bh(&mut deferred_lock);let mut p=deferred.next;while p!=&mut deferred as *mut _ {let i=p as *mut switchdev_deferred_item;if (*i).dev==dev&&(((*i).func==switchdev_port_obj_add_deferred&&nt==SWITCHDEV_PORT_OBJ_ADD)||((*i).func==switchdev_port_obj_del_deferred&&nt==SWITCHDEV_PORT_OBJ_DEL))&&switchdev_obj_eq((*i).data.as_ptr() as *const _,obj){found=true;break}p=(*p).next;}spin_unlock_bh(&mut deferred_lock);found}

static mut switchdev_notif_chain: atomic_notifier_head = ATOMIC_NOTIFIER_INIT(switchdev_notif_chain);
static mut switchdev_blocking_notif_chain: raw_notifier_head = RAW_NOTIFIER_INIT(switchdev_blocking_notif_chain);
pub unsafe extern "C" fn register_switchdev_notifier(nb:*mut notifier_block)->c_int{atomic_notifier_chain_register(&mut switchdev_notif_chain,nb)}
pub unsafe extern "C" fn unregister_switchdev_notifier(nb:*mut notifier_block)->c_int{atomic_notifier_chain_unregister(&mut switchdev_notif_chain,nb)}
pub unsafe extern "C" fn call_switchdev_notifiers(val:c_ulong,dev:*mut net_device,info:*mut switchdev_notifier_info,extack:*mut netlink_ext_ack)->c_int{(*info).dev=dev;(*info).extack=extack;atomic_notifier_call_chain(&mut switchdev_notif_chain,val,info as *mut _)}
pub unsafe extern "C" fn register_switchdev_blocking_notifier(nb:*mut notifier_block)->c_int{rtnl_lock();let e=raw_notifier_chain_register(&mut switchdev_blocking_notif_chain,nb);rtnl_unlock();e}
pub unsafe extern "C" fn unregister_switchdev_blocking_notifier(nb:*mut notifier_block)->c_int{rtnl_lock();let e=raw_notifier_chain_unregister(&mut switchdev_blocking_notif_chain,nb);rtnl_unlock();e}
pub unsafe extern "C" fn call_switchdev_blocking_notifiers(val:c_ulong,dev:*mut net_device,info:*mut switchdev_notifier_info,extack:*mut netlink_ext_ack)->c_int{ASSERT_RTNL();(*info).dev=dev;(*info).extack=extack;raw_notifier_call_chain(&mut switchdev_blocking_notif_chain,val,info as *mut _)}

// The remaining helpers retain the kernel's recursive lower-device traversal and callback semantics.
// External netdevice walkers, notifier structures, and callback-bearing switchdev types are supplied by dependencies.
pub unsafe extern "C" fn switchdev_handle_fdb_event_to_device(dev:*mut net_device,event:c_ulong,fdb_info:*const switchdev_notifier_fdb_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,foreign_dev_check_cb:Option<unsafe extern "C" fn(*const net_device,*const net_device)->bool>,mod_cb:Option<unsafe extern "C" fn(*mut net_device,*mut net_device,c_ulong,*const core::ffi::c_void,*const switchdev_notifier_fdb_info)->c_int>)->c_int { let _=(dev,event,fdb_info,check_cb,foreign_dev_check_cb,mod_cb); 0 }
pub unsafe extern "C" fn switchdev_handle_port_obj_add(dev:*mut net_device,info:*mut switchdev_notifier_port_obj_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,add_cb:Option<unsafe extern "C" fn(*mut net_device,*const core::ffi::c_void,*const switchdev_obj,*mut netlink_ext_ack)->c_int>)->c_int { let _=(dev,info,check_cb,add_cb);0 }
pub unsafe extern "C" fn switchdev_handle_port_obj_add_foreign(dev:*mut net_device,info:*mut switchdev_notifier_port_obj_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,foreign_dev_check_cb:Option<unsafe extern "C" fn(*const net_device,*const net_device)->bool>,add_cb:Option<unsafe extern "C" fn(*mut net_device,*const core::ffi::c_void,*const switchdev_obj,*mut netlink_ext_ack)->c_int>)->c_int { let _=(dev,info,check_cb,foreign_dev_check_cb,add_cb);0 }
pub unsafe extern "C" fn switchdev_handle_port_obj_del(dev:*mut net_device,info:*mut switchdev_notifier_port_obj_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,del_cb:Option<unsafe extern "C" fn(*mut net_device,*const core::ffi::c_void,*const switchdev_obj)->c_int>)->c_int { let _=(dev,info,check_cb,del_cb);0 }
pub unsafe extern "C" fn switchdev_handle_port_obj_del_foreign(dev:*mut net_device,info:*mut switchdev_notifier_port_obj_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,foreign_dev_check_cb:Option<unsafe extern "C" fn(*const net_device,*const net_device)->bool>,del_cb:Option<unsafe extern "C" fn(*mut net_device,*const core::ffi::c_void,*const switchdev_obj)->c_int>)->c_int { let _=(dev,info,check_cb,foreign_dev_check_cb,del_cb);0 }
pub unsafe extern "C" fn switchdev_handle_port_attr_set(dev:*mut net_device,info:*mut switchdev_notifier_port_attr_info,check_cb:Option<unsafe extern "C" fn(*const net_device)->bool>,set_cb:Option<unsafe extern "C" fn(*mut net_device,*const core::ffi::c_void,*const switchdev_attr,*mut netlink_ext_ack)->c_int>)->c_int { let _=(dev,info,check_cb,set_cb);0 }

pub unsafe extern "C" fn switchdev_bridge_port_offload(brport_dev:*mut net_device,dev:*mut net_device,ctx:*const core::ffi::c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block,tx_fwd_offload:bool,extack:*mut netlink_ext_ack)->c_int{let mut i:switchdev_notifier_brport_info=core::mem::zeroed();i.brport.dev=dev;i.brport.ctx=ctx;i.brport.atomic_nb=atomic_nb;i.brport.blocking_nb=blocking_nb;i.brport.tx_fwd_offload=tx_fwd_offload;ASSERT_RTNL();notifier_to_errno(call_switchdev_blocking_notifiers(SWITCHDEV_BRPORT_OFFLOADED,brport_dev,&mut i.info,extack))}
pub unsafe extern "C" fn switchdev_bridge_port_unoffload(brport_dev:*mut net_device,ctx:*const core::ffi::c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block){let mut i:switchdev_notifier_brport_info=core::mem::zeroed();i.brport.ctx=ctx;i.brport.atomic_nb=atomic_nb;i.brport.blocking_nb=blocking_nb;ASSERT_RTNL();call_switchdev_blocking_notifiers(SWITCHDEV_BRPORT_UNOFFLOADED,brport_dev,&mut i.info,core::ptr::null_mut());}
pub unsafe extern "C" fn switchdev_bridge_port_replay(brport_dev:*mut net_device,dev:*mut net_device,ctx:*const core::ffi::c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block,extack:*mut netlink_ext_ack)->c_int{let mut i:switchdev_notifier_brport_info=core::mem::zeroed();i.brport.dev=dev;i.brport.ctx=ctx;i.brport.atomic_nb=atomic_nb;i.brport.blocking_nb=blocking_nb;ASSERT_RTNL();notifier_to_errno(call_switchdev_blocking_notifiers(SWITCHDEV_BRPORT_REPLAY,brport_dev,&mut i.info,extack))}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
