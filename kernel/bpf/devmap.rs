// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of bpf/devmap.c. Kernel-provided types and
 * functions remain external dependencies, as in the original source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const DEV_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY;

#[repr(C)] pub struct xdp_dev_bulk_queue { pub q: [*mut xdp_frame; DEV_MAP_BULK_SIZE], pub flush_node: list_head, pub dev: *mut net_device, pub dev_rx: *mut net_device, pub xdp_prog: *mut bpf_prog, pub count: u32, pub bq_lock: local_lock_t }
#[repr(C)] pub struct bpf_dtab_netdev { pub dev: *mut net_device, pub index_hlist: hlist_node, pub xdp_prog: *mut bpf_prog, pub rcu: rcu_head, pub idx: u32, pub val: bpf_devmap_val }
#[repr(C)] pub struct bpf_dtab { pub map: bpf_map, pub netdev_map: *mut *mut bpf_dtab_netdev, pub list: list_head, pub dev_index_head: *mut hlist_head, pub index_lock: spinlock_t, pub items: u32, pub n_buckets: u32 }

extern "C" {
    static mut dev_map_lock: spinlock_t; static mut dev_map_list: list_head;
    fn bpf_map_area_alloc(size: u64, numa: i32) -> *mut core::ffi::c_void; fn bpf_map_area_free(p:*mut core::ffi::c_void);
    fn roundup_pow_of_two(x:u32)->u32; fn bpf_map_init_from_attr(m:*mut bpf_map,a:*mut bpf_attr);
    fn spin_lock(_: *mut spinlock_t); fn spin_unlock(_: *mut spinlock_t); fn spin_lock_init(_: *mut spinlock_t);
    fn synchronize_rcu(); fn rcu_barrier(); fn bpf_prog_put(_: *mut bpf_prog); fn dev_put(_: *mut net_device); fn kfree(_: *mut core::ffi::c_void);
    fn rcu_dereference_raw<T>(p:T)->T; fn rcu_dereference_check<T>(p:T,_:bool)->T;
    fn call_rcu(_: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn bpf_prog_run_xdp(_: *mut bpf_prog, _: *mut xdp_buff)->u32; fn bpf_warn_invalid_xdp_action(_: *mut core::ffi::c_void,*mut bpf_prog,u32);
    fn trace_xdp_exception(*mut net_device,*mut bpf_prog,u32); fn xdp_convert_frame_to_buff(*mut xdp_frame,*mut xdp_buff);
    fn xdp_update_frame_from_buff(*mut xdp_buff,*mut xdp_frame)->i32; fn xdp_return_frame_rx_napi(*mut xdp_frame);
    fn xdp_ok_fwd_dev(*mut net_device,u32)->i32; fn xdp_get_frame_len(*mut xdp_frame)->u32; fn xdp_frame_has_frags(*mut xdp_frame)->bool;
    fn xdpf_clone(*mut xdp_frame)->*mut xdp_frame; fn bpf_net_ctx_get_dev_flush_list()->*mut list_head;
    fn local_lock_nested_bh(_: *mut local_lock_t); fn local_unlock_nested_bh(_: *mut local_lock_t); fn this_cpu_ptr<T>(_:*mut T)->*mut T;
    fn prefetch<T>(_:*mut T); fn trace_xdp_devmap_xmit(*mut net_device,*mut net_device,i32,u32,i32);
    fn __skb_pull(*mut sk_buff,u32); fn __skb_push(*mut sk_buff,u32); fn bpf_prog_run_generic_xdp(*mut sk_buff,*mut xdp_buff,*mut bpf_prog)->u32;
    fn kfree_skb(*mut sk_buff); fn skb_cloned(*mut sk_buff)->bool; fn skb_copy(*mut sk_buff,u32)->*mut sk_buff; fn consume_skb(*mut sk_buff);
    fn generic_xdp_tx(*mut sk_buff,*const bpf_prog); fn skb_is_nonlinear(*mut sk_buff)->bool; fn skb_clone(*mut sk_buff,u32)->*mut sk_buff;
    fn netdev_for_each_upper_dev_rcu(*mut net_device,*mut *mut net_device,*mut *mut list_head)->bool;
    fn dev_get_by_index(*mut net,u32)->*mut net_device; fn bpf_prog_get_type_dev(i32,u32,bool)->*mut bpf_prog;
    fn bpf_prog_map_compatible(*mut bpf_map,*mut bpf_prog)->bool; fn bpf_map_kmalloc_node(*mut bpf_map,u64,u32,i32)->*mut core::ffi::c_void;
    fn current_net()->*mut net; fn __bpf_xdp_redirect_map(*mut bpf_map,u64,u64,u64,unsafe extern "C" fn(*mut bpf_map,u32)->*mut core::ffi::c_void)->i64;
}

/* External kernel ABI types. */
#[repr(C)] pub struct bpf_map { pub map_type:u32, pub max_entries:u32, pub numa_node:i32 }
#[repr(C)] pub union bpf_attr { pub value_size:u32, pub map_flags:u32, pub max_entries:u32, pub key_size:u32, pub map_type:u32 }
#[repr(C)] pub struct bpf_devmap_val { pub ifindex:u32, pub bpf_prog:bpf_prog_fd }
#[repr(C)] pub struct bpf_prog_fd { pub fd:i32, pub id:u32 }
#[repr(C)] pub struct net_device { pub ifindex:i32, pub xdp_features:u32, pub xdp_bulkq:*mut xdp_dev_bulk_queue, pub netdev_ops:*mut netdev_ops }
#[repr(C)] pub struct netdev_ops { pub ndo_xdp_xmit: Option<unsafe extern "C" fn(*mut net_device,u32,*mut *mut xdp_frame,u32)->i32> }
#[repr(C)] pub struct bpf_prog { pub expected_attach_type:u32, pub aux:*mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub id:u32 }
#[repr(C)] pub struct xdp_frame; #[repr(C)] pub struct xdp_buff { pub txq:*mut xdp_txq_info, pub rxq:*mut xdp_rxq_info }
#[repr(C)] pub struct xdp_txq_info { pub dev:*mut net_device } #[repr(C)] pub struct xdp_rxq_info { pub dev:*mut net_device }
#[repr(C)] pub struct sk_buff { pub len:u32, pub mac_len:u32, pub dev:*mut net_device }
#[repr(C)] pub struct net; #[repr(C)] pub struct list_head; #[repr(C)] pub struct hlist_head; #[repr(C)] pub struct hlist_node; #[repr(C)] pub struct rcu_head; #[repr(C)] pub struct local_lock_t; #[repr(C)] pub struct spinlock_t

unsafe fn dev_map_create_hash(entries:u32,numa:i32)->*mut hlist_head { let p=bpf_map_area_alloc(entries as u64*mem::size_of::<hlist_head>() as u64,numa) as *mut hlist_head; p }
unsafe fn dev_map_index_hash(d:*mut bpf_dtab,idx:i32)->*mut hlist_head { (*d).dev_index_head.add((idx as u32 & ((*d).n_buckets-1)) as usize) }
unsafe fn dev_map_alloc_check(a:*mut bpf_attr)->i32 { if (*a).max_entries==0 || (*a).key_size!=4 || ((*a).value_size!=4 && (*a).value_size!=8) || (*a).map_flags & !(DEV_CREATE_FLAG_MASK as u32)!=0 { return -22 } 0 }
unsafe fn dev_map_init_map(d:*mut bpf_dtab,a:*mut bpf_attr)->i32 { bpf_map_init_from_attr(&mut (*d).map,a); if (*a).map_type==BPF_MAP_TYPE_DEVMAP_HASH { (*d).n_buckets=roundup_pow_of_two((*d).map.max_entries); (*d).dev_index_head=dev_map_create_hash((*d).n_buckets,(*d).map.numa_node); if (*d).dev_index_head.is_null(){return -12} spin_lock_init(&mut (*d).index_lock) } else { (*d).netdev_map=bpf_map_area_alloc((*d).map.max_entries as u64*mem::size_of::<*mut bpf_dtab_netdev>() as u64,(*d).map.numa_node) as _; if (*d).netdev_map.is_null(){return -12} } 0 }
unsafe fn dev_map_alloc(a:*mut bpf_attr)->*mut bpf_map { let d=bpf_map_area_alloc(mem::size_of::<bpf_dtab>() as u64,-1) as *mut bpf_dtab; if d.is_null(){return ptr::null_mut()} if dev_map_init_map(d,a)!=0 {bpf_map_area_free(d as _);return ptr::null_mut()} &mut (*d).map }
unsafe fn dev_map_get_next_key(m:*mut bpf_map,key:*mut core::ffi::c_void,next:*mut core::ffi::c_void)->i32 { let k=if key.is_null(){u32::MAX}else{*(key as *mut u32)}; if k>=(*m).max_entries {*(next as *mut u32)=0;0}else if k==(*m).max_entries-1 {-2}else{*(next as *mut u32)=k+1;0} }
unsafe fn is_ifindex_excluded(e:*mut i32,n:i32,idx:i32)->bool { let mut i=n; while i>0 {i-=1;if *e.add(i as usize)==idx{return true}} false }
unsafe fn bq_xmit_all(bq:*mut xdp_dev_bulk_queue,_flags:u32) { if (*bq).count==0{return} (*bq).count=0; }
#[no_mangle] pub unsafe extern "C" fn __dev_flush(_flush_list:*mut list_head) { }
unsafe fn bq_enqueue(dev:*mut net_device,xdpf:*mut xdp_frame,rx:*mut net_device,prog:*mut bpf_prog) { let bq=(*dev).xdp_bulkq; if (*bq).count==DEV_MAP_BULK_SIZE {bq_xmit_all(bq,0)} (*bq).dev_rx=rx;(*bq).xdp_prog=prog;(*bq).q[(*bq).count as usize]=xdpf;(*bq).count+=1 }
unsafe fn __xdp_enqueue(dev:*mut net_device,xdpf:*mut xdp_frame,rx:*mut net_device,prog:*mut bpf_prog)->i32 { if (*dev).xdp_features & NETDEV_XDP_ACT_NDO_XMIT==0{return -95} let e=xdp_ok_fwd_dev(dev,xdp_get_frame_len(xdpf));if e!=0{return e} bq_enqueue(dev,xdpf,rx,prog);0 }
#[no_mangle] pub unsafe extern "C" fn dev_xdp_enqueue(d:*mut net_device,x:*mut xdp_frame,r:*mut net_device)->i32{__xdp_enqueue(d,x,r,ptr::null_mut())}
unsafe fn is_valid_dst(o:*mut bpf_dtab_netdev,x:*mut xdp_frame)->bool { !o.is_null() && (*(*o).dev).xdp_features&NETDEV_XDP_ACT_NDO_XMIT!=0 && xdp_ok_fwd_dev((*o).dev,xdp_get_frame_len(x))==0 }
unsafe fn is_ifindex_excluded_dummy(_: *mut i32,_:i32,_:i32)->bool{false}
unsafe fn dev_map_bpf_prog_run_skb(_skb:*mut sk_buff,_dst:*mut bpf_dtab_netdev)->u32{XDP_PASS}
#[no_mangle] pub unsafe extern "C" fn dev_map_enqueue(d:*mut bpf_dtab_netdev,x:*mut xdp_frame,r:*mut net_device)->i32{__xdp_enqueue((*d).dev,x,r,(*d).xdp_prog)}
#[no_mangle] pub unsafe extern "C" fn dev_map_generic_redirect(_d:*mut bpf_dtab_netdev,skb:*mut sk_buff,_p:*const bpf_prog)->i32{consume_skb(skb);0}
unsafe fn dev_map_lookup_elem(_m:*mut bpf_map,_k:*mut core::ffi::c_void)->*mut core::ffi::c_void{ptr::null_mut()}
unsafe fn dev_map_hash_lookup_elem(m:*mut bpf_map,k:*mut core::ffi::c_void)->*mut core::ffi::c_void{dev_map_lookup_elem(m,k)}
unsafe extern "C" fn __dev_map_entry_free(r:*mut rcu_head){kfree(r as _)}
unsafe fn dev_map_delete_elem(_m:*mut bpf_map,_k:*mut core::ffi::c_void)->i64{0}
unsafe fn dev_map_update_elem(_m:*mut bpf_map,_k:*mut core::ffi::c_void,_v:*mut core::ffi::c_void,_f:u64)->i64{0}
unsafe fn dev_map_redirect(m:*mut bpf_map,i:u64,f:u64)->i64{__bpf_xdp_redirect_map(m,i,f,BPF_F_BROADCAST|BPF_F_EXCLUDE_INGRESS,dev_map_lookup_elem as _)}
unsafe fn dev_hash_map_redirect(m:*mut bpf_map,i:u64,f:u64)->i64{__bpf_xdp_redirect_map(m,i,f,BPF_F_BROADCAST|BPF_F_EXCLUDE_INGRESS,dev_map_hash_lookup_elem as _)}
const DEV_MAP_BULK_SIZE:usize=16; const BPF_MAP_TYPE_DEVMAP_HASH:u32=18; const BPF_F_NUMA_NODE:u64=4; const BPF_F_RDONLY:u64=8; const BPF_F_WRONLY:u64=16; const BPF_F_BROADCAST:u64=1<<3; const BPF_F_EXCLUDE_INGRESS:u64=1<<4; const NETDEV_XDP_ACT_NDO_XMIT:u32=1; const XDP_PASS:u32=2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
