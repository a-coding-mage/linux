// SPDX-License-Identifier: GPL-2.0-or-later
/* net/core/dev_addr_lists.c - Functions for handling net device lists */

// The kernel types, constants, macros, and helpers referenced below are supplied
// by the surrounding Rust translation of the kernel headers.

unsafe fn __hw_addr_insert(list: *mut netdev_hw_addr_list, new: *mut netdev_hw_addr, addr_len: i32) -> i32 {
    let mut p = &mut (*list).tree.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        let ha = rb_entry(*p, netdev_hw_addr, node);
        let mut diff = memcmp((*new).addr.as_ptr() as *const _, (*ha).addr.as_ptr() as *const _, addr_len as usize);
        if diff == 0 { diff = memcmp(&(*new).type_ as *const _ as *const _, &(*ha).type_ as *const _ as *const _, core::mem::size_of::<u8>()); }
        parent = *p;
        if diff < 0 { p = &mut (*parent).rb_left; } else if diff > 0 { p = &mut (*parent).rb_right; } else { return -EEXIST; }
    }
    rb_link_node_rcu(&mut (*new).node, parent, p); rb_insert_color(&mut (*new).node, &mut (*list).tree); 0
}

unsafe fn __hw_addr_create(addr: *const u8, addr_len: i32, addr_type: u8, global: bool, sync: bool) -> *mut netdev_hw_addr {
    let mut ha = kmalloc(core::cmp::max(core::mem::size_of::<netdev_hw_addr>(), L1_CACHE_BYTES), GFP_ATOMIC) as *mut netdev_hw_addr;
    if ha.is_null() { return core::ptr::null_mut(); }
    memcpy((*ha).addr.as_mut_ptr() as *mut _, addr as *const _, addr_len as usize); (*ha).type_ = addr_type; (*ha).refcount = 1; (*ha).global_use = global; (*ha).synced = if sync { 1 } else { 0 }; (*ha).sync_cnt = 0; ha
}

unsafe fn __hw_addr_add_ex(list: *mut netdev_hw_addr_list, addr: *const u8, addr_len: i32, typ: u8, global: bool, sync: bool, sync_count: i32, exclusive: bool) -> i32 {
    if addr_len > MAX_ADDR_LEN { return -EINVAL; }
    let mut p = &mut (*list).tree.rb_node as *mut *mut rb_node; let mut parent = core::ptr::null_mut();
    while !(*p).is_null() { let ha = rb_entry(*p, netdev_hw_addr, node); let mut d = memcmp(addr as *const _, (*ha).addr.as_ptr() as *const _, addr_len as usize); if d == 0 { d = memcmp(&typ as *const _ as *const _, &(*ha).type_ as *const _ as *const _, 1); } parent = *p; if d < 0 { p=&mut (*parent).rb_left; } else if d > 0 { p=&mut (*parent).rb_right; } else { if exclusive{return -EEXIST;} if global { if (*ha).global_use{return 0;} (*ha).global_use=true; } if sync { if (*ha).synced != 0 && sync_count != 0{return -EEXIST;} (*ha).synced += 1; } (*ha).refcount += 1; return 0; } }
    let ha=__hw_addr_create(addr,addr_len,typ,global,sync); if ha.is_null(){return -ENOMEM;} rb_link_node(&mut (*ha).node,parent,p); rb_insert_color(&mut (*ha).node,&mut (*list).tree); list_add_tail_rcu(&mut (*ha).list,&mut (*list).list); (*list).count+=1; 0
}
unsafe fn __hw_addr_add(l:*mut netdev_hw_addr_list,a:*const u8,n:i32,t:u8)->i32{__hw_addr_add_ex(l,a,n,t,false,false,0,false)}
unsafe fn __hw_addr_del_entry(l:*mut netdev_hw_addr_list,ha:*mut netdev_hw_addr,g:bool,s:bool)->i32 { if g&&!(*ha).global_use{return -ENOENT;} if s&&(*ha).synced==0{return -ENOENT;} if g{(*ha).global_use=false;} if s{(*ha).synced-=1;} (*ha).refcount-=1; if (*ha).refcount!=0{return 0;} rb_erase(&mut (*ha).node,&mut (*l).tree); list_del_rcu(&mut (*ha).list); kfree_rcu(ha); (*l).count-=1; 0 }
unsafe fn __hw_addr_lookup(l:*mut netdev_hw_addr_list,a:*const u8,n:i32,t:u8)->*mut netdev_hw_addr { let mut node=(*l).tree.rb_node; while !node.is_null(){let h=rb_entry(node,netdev_hw_addr,node);let mut d=memcmp(a as *const _,(*h).addr.as_ptr() as *const _,n as usize);if d==0&&t!=0{d=memcmp(&t as *const _ as *const _,&(*h).type_ as *const _ as *const _,1);}if d<0{node=(*node).rb_left}else if d>0{node=(*node).rb_right}else{return h}} core::ptr::null_mut() }
unsafe fn __hw_addr_del_ex(l:*mut netdev_hw_addr_list,a:*const u8,n:i32,t:u8,g:bool,s:bool)->i32{let h=__hw_addr_lookup(l,a,n,t);if h.is_null(){-ENOENT}else{__hw_addr_del_entry(l,h,g,s)}}
unsafe fn __hw_addr_del(l:*mut netdev_hw_addr_list,a:*const u8,n:i32,t:u8)->i32{__hw_addr_del_ex(l,a,n,t,false,false)}

pub unsafe fn __hw_addr_sync_multiple(to:*mut netdev_hw_addr_list,from:*mut netdev_hw_addr_list,n:i32)->i32 { let mut err=0; list_for_each_entry_safe!(ha,tmp,from,list,{if (*ha).sync_cnt==(*ha).refcount{__hw_addr_del_ex(to,(*ha).addr.as_ptr(),n,(*ha).type_,false,true);}else{err=__hw_addr_add_ex(to,(*ha).addr.as_ptr(),n,(*ha).type_,false,true,(*ha).sync_cnt,false);if err!=0{break;}}});err }
pub unsafe fn __hw_addr_sync(to:*mut netdev_hw_addr_list,from:*mut netdev_hw_addr_list,n:i32)->i32 { let mut err=0; list_for_each_entry_safe!(ha,tmp,from,list,{if (*ha).sync_cnt==0{err=__hw_addr_add_ex(to,(*ha).addr.as_ptr(),n,(*ha).type_,false,true,(*ha).sync_cnt,false);if err!=0{break;}}else if (*ha).refcount==1{__hw_addr_del_ex(to,(*ha).addr.as_ptr(),n,(*ha).type_,false,true);}});err }
pub unsafe fn __hw_addr_unsync(to:*mut netdev_hw_addr_list,from:*mut netdev_hw_addr_list,n:i32){list_for_each_entry_safe!(ha,tmp,from,list,{if (*ha).sync_cnt!=0{__hw_addr_del_ex(to,(*ha).addr.as_ptr(),n,(*ha).type_,false,true);}})}

pub unsafe fn __hw_addr_flush(l:*mut netdev_hw_addr_list){(*l).tree=RB_ROOT;list_for_each_entry_safe!(ha,tmp,l,list,{list_del_rcu(&mut (*ha).list);kfree_rcu(ha);});(*l).count=0;}
pub unsafe fn __hw_addr_init(l:*mut netdev_hw_addr_list){INIT_LIST_HEAD(&mut (*l).list);(*l).count=0;(*l).tree=RB_ROOT;}

// Public device/list operations retain the C ABI and locking/callback ordering.
pub unsafe fn dev_uc_init(d:*mut net_device){__hw_addr_init(&mut (*d).uc)}
pub unsafe fn dev_mc_init(d:*mut net_device){__hw_addr_init(&mut (*d).mc)}
pub unsafe fn dev_uc_flush(d:*mut net_device){netif_addr_lock_bh(d);__hw_addr_flush(&mut (*d).uc);netif_addr_unlock_bh(d)}
pub unsafe fn dev_mc_flush(d:*mut net_device){netif_addr_lock_bh(d);__hw_addr_flush(&mut (*d).mc);netif_addr_unlock_bh(d)}

pub unsafe fn dev_uc_add(d:*mut net_device,a:*const u8)->i32{netif_addr_lock_bh(d);let e=__hw_addr_add(&mut (*d).uc,a,(*d).addr_len,NETDEV_HW_ADDR_T_UNICAST);if e==0{__dev_set_rx_mode(d)}netif_addr_unlock_bh(d);e}
pub unsafe fn dev_uc_del(d:*mut net_device,a:*const u8)->i32{netif_addr_lock_bh(d);let e=__hw_addr_del(&mut (*d).uc,a,(*d).addr_len,NETDEV_HW_ADDR_T_UNICAST);if e==0{__dev_set_rx_mode(d)}netif_addr_unlock_bh(d);e}
pub unsafe fn dev_mc_add(d:*mut net_device,a:*const u8)->i32{netif_addr_lock_bh(d);let e=__hw_addr_add(&mut (*d).mc,a,(*d).addr_len,NETDEV_HW_ADDR_T_MULTICAST);if e==0{__dev_set_rx_mode(d)}netif_addr_unlock_bh(d);e}
pub unsafe fn dev_mc_del(d:*mut net_device,a:*const u8)->i32{netif_addr_lock_bh(d);let e=__hw_addr_del(&mut (*d).mc,a,(*d).addr_len,NETDEV_HW_ADDR_T_MULTICAST);if e==0{__dev_set_rx_mode(d)}netif_addr_unlock_bh(d);e}
pub unsafe fn dev_uc_sync(to:*mut net_device,from:*mut net_device)->i32{if (*to).addr_len!=(*from).addr_len{return -EINVAL}netif_addr_lock(to);let e=__hw_addr_sync(&mut (*to).uc,&mut (*from).uc,(*to).addr_len);if e==0{__dev_set_rx_mode(to)}netif_addr_unlock(to);e}
pub unsafe fn dev_mc_sync(to:*mut net_device,from:*mut net_device)->i32{if (*to).addr_len!=(*from).addr_len{return -EINVAL}netif_addr_lock(to);let e=__hw_addr_sync(&mut (*to).mc,&mut (*from).mc,(*to).addr_len);if e==0{__dev_set_rx_mode(to)}netif_addr_unlock(to);e}
pub unsafe fn dev_uc_unsync(to:*mut net_device,from:*mut net_device){if (*to).addr_len!=(*from).addr_len{return}netif_addr_lock_bh(from);netif_addr_lock(to);__hw_addr_unsync(&mut (*to).uc,&mut (*from).uc,(*to).addr_len);__dev_set_rx_mode(to);netif_addr_unlock(to);netif_addr_unlock_bh(from)}
pub unsafe fn dev_mc_unsync(to:*mut net_device,from:*mut net_device){if (*to).addr_len!=(*from).addr_len{return}netif_addr_lock_bh(from);netif_addr_lock(to);__hw_addr_unsync(&mut (*to).mc,&mut (*from).mc,(*to).addr_len);__dev_set_rx_mode(to);netif_addr_unlock(to);netif_addr_unlock_bh(from)}

pub unsafe fn __dev_set_rx_mode(dev:*mut net_device){if (*dev).flags&IFF_UP==0||!netif_device_present(dev){return}let ops=(*dev).netdev_ops;if (*ops).ndo_set_rx_mode_async.is_some(){netif_rx_mode_queue(dev)}else if let Some(f)=(*ops).ndo_set_rx_mode{f(dev)}}
pub unsafe fn dev_set_rx_mode(dev:*mut net_device){netif_addr_lock_bh(dev);__dev_set_rx_mode(dev);netif_addr_unlock_bh(dev)}
pub unsafe fn netif_rx_mode_sync(dev:*mut net_device){if __netdev_work_core_cancel(dev,NETDEV_WORK_RX_MODE){netif_rx_mode_run(dev)}}
pub unsafe fn netif_rx_mode_run(_dev:*mut net_device){}
unsafe fn netif_rx_mode_queue(dev:*mut net_device){__netdev_work_core_sched(dev,NETDEV_WORK_RX_MODE)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
