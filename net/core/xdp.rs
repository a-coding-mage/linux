// SPDX-License-Identifier: GPL-2.0-only
/* net/core/xdp.c
 *
 * Copyright (c) 2017 Jesper Dangaard Brouer, Red Hat Inc.
 */

// Linux kernel headers are external dependencies of this translation.

const REG_STATE_NEW: u32 = 0x0;
const REG_STATE_REGISTERED: u32 = 0x1;
const REG_STATE_UNREGISTERED: u32 = 0x2;
const REG_STATE_UNUSED: u32 = 0x3;
const MEM_ID_MAX: i32 = 0xFFFE;
const MEM_ID_MIN: i32 = 1;

static mut mem_id_pool: ida = ida::UNINIT;
static mut mem_id_lock: mutex = mutex::UNINIT;
static mut mem_id_next: i32 = MEM_ID_MIN;
static mut mem_id_init: bool = false;
static mut mem_id_ht: *mut rhashtable = core::ptr::null_mut();

unsafe fn xdp_mem_id_hashfn(data: *const core::ffi::c_void, _len: u32, _seed: u32) -> u32 {
    let key = *(data as *const u32);
    // BUILD_BUG_ON(sizeof_field(struct xdp_mem_allocator, mem.id) != sizeof(u32));
    key
}

unsafe fn xdp_mem_id_cmp(arg: *mut rhashtable_compare_arg, ptr: *const core::ffi::c_void) -> i32 {
    let xa = ptr as *const xdp_mem_allocator;
    let mem_id = *( (*arg).key as *const u32 );
    ((*xa).mem.id != mem_id) as i32
}

static mem_id_rht_params: rhashtable_params = rhashtable_params {
    nelem_hint: 64, head_offset: offset_of!(xdp_mem_allocator, node),
    key_offset: offset_of!(xdp_mem_allocator, mem.id), key_len: core::mem::size_of::<u32>(),
    max_size: MEM_ID_MAX as u32, min_size: 8, automatic_shrinking: true,
    hashfn: Some(xdp_mem_id_hashfn), obj_cmpfn: Some(xdp_mem_id_cmp),
};

unsafe fn __xdp_mem_allocator_rcu_free(rcu: *mut rcu_head) {
    let xa = container_of!(rcu, xdp_mem_allocator, rcu);
    ida_free(&mut mem_id_pool, (*xa).mem.id);
    kfree(xa as *mut core::ffi::c_void);
}

unsafe fn mem_xa_remove(xa: *mut xdp_mem_allocator) {
    trace_mem_disconnect(xa);
    if rhashtable_remove_fast(mem_id_ht, &mut (*xa).node, &mem_id_rht_params) == 0 {
        call_rcu(&mut (*xa).rcu, __xdp_mem_allocator_rcu_free);
    }
}

unsafe fn mem_allocator_disconnect(allocator: *mut core::ffi::c_void) {
    let mut xa: *mut xdp_mem_allocator;
    let mut iter = core::mem::MaybeUninit::<rhashtable_iter>::zeroed().assume_init();
    mutex_lock(&mut mem_id_lock);
    rhashtable_walk_enter(mem_id_ht, &mut iter);
    loop {
        rhashtable_walk_start(&mut iter);
        loop {
            xa = rhashtable_walk_next(&mut iter);
            if xa.is_null() || IS_ERR(xa as *mut core::ffi::c_void) { break; }
            if (*xa).allocator == allocator { mem_xa_remove(xa); }
        }
        rhashtable_walk_stop(&mut iter);
        if xa != ERR_PTR(-EAGAIN) { break; }
    }
    rhashtable_walk_exit(&mut iter);
    mutex_unlock(&mut mem_id_lock);
}

pub unsafe fn xdp_unreg_mem_model(mem: *mut xdp_mem_info) {
    let id = (*mem).id; let typ = (*mem).typ;
    (*mem).id = 0; (*mem).typ = 0;
    if id == 0 { return; }
    if typ == MEM_TYPE_PAGE_POOL {
        let xa = rhashtable_lookup_fast(mem_id_ht, &id, &mem_id_rht_params);
        page_pool_destroy((*xa).page_pool);
    }
}

pub unsafe fn xdp_rxq_info_unreg_mem_model(xdp_rxq: *mut xdp_rxq_info) {
    if (*xdp_rxq).reg_state != REG_STATE_REGISTERED { WARN(1, "Missing register, driver bug"); return; }
    xdp_unreg_mem_model(&mut (*xdp_rxq).mem);
}

pub unsafe fn xdp_rxq_info_unreg(xdp_rxq: *mut xdp_rxq_info) {
    if (*xdp_rxq).reg_state == REG_STATE_UNUSED { return; }
    xdp_rxq_info_unreg_mem_model(xdp_rxq);
    (*xdp_rxq).reg_state = REG_STATE_UNREGISTERED; (*xdp_rxq).dev = core::ptr::null_mut();
}

unsafe fn xdp_rxq_info_init(x: *mut xdp_rxq_info) { memset(x as *mut _, 0, core::mem::size_of::<xdp_rxq_info>()); }

pub unsafe fn __xdp_rxq_info_reg(x: *mut xdp_rxq_info, dev: *mut net_device, queue_index: u32, _napi_id: u32, frag_size: u32) -> i32 {
    if dev.is_null() { WARN(1, "Missing net_device from driver"); return -ENODEV; }
    if (*x).reg_state == REG_STATE_UNUSED { WARN(1, "Driver promised not to register this"); return -EINVAL; }
    if (*x).reg_state == REG_STATE_REGISTERED { WARN(1, "Missing unregister, handled but fix driver"); xdp_rxq_info_unreg(x); }
    xdp_rxq_info_init(x); (*x).dev = dev; (*x).queue_index = queue_index; (*x).frag_size = frag_size;
    (*x).reg_state = REG_STATE_REGISTERED; 0
}

pub unsafe fn xdp_rxq_info_unused(x: *mut xdp_rxq_info) { (*x).reg_state = REG_STATE_UNUSED; }
pub unsafe fn xdp_rxq_info_is_reg(x: *mut xdp_rxq_info) -> bool { (*x).reg_state == REG_STATE_REGISTERED }

unsafe fn __mem_id_init_hash_table() -> i32 {
    if mem_id_init { return 0; }
    let rht = kzalloc_obj::<rhashtable>(); if rht.is_null() { return -ENOMEM; }
    let ret = rhashtable_init(rht, &mem_id_rht_params); if ret < 0 { kfree(rht as *mut _); return ret; }
    mem_id_ht = rht; smp_mb(); mem_id_init = true; 0
}

unsafe fn __mem_id_cyclic_get(gfp: gfp_t) -> i32 {
    let mut retries = 1;
    loop {
        let id = ida_alloc_range(&mut mem_id_pool, mem_id_next, MEM_ID_MAX - 1, gfp);
        if id >= 0 { mem_id_next = id + 1; return id; }
        if id == -ENOSPC && retries > 0 { retries -= 1; mem_id_next = MEM_ID_MIN; continue; }
        return id;
    }
}

unsafe fn __xdp_reg_mem_model(mem: *mut xdp_mem_info, typ: xdp_mem_type, allocator: *mut core::ffi::c_void) -> *mut xdp_mem_allocator {
    if !__is_supported_mem_type(typ) { return ERR_PTR(-EOPNOTSUPP); }
    (*mem).typ = typ;
    if allocator.is_null() { if typ == MEM_TYPE_PAGE_POOL { return ERR_PTR(-EINVAL); } return core::ptr::null_mut(); }
    if !mem_id_init { mutex_lock(&mut mem_id_lock); let ret = __mem_id_init_hash_table(); mutex_unlock(&mut mem_id_lock); if ret < 0 { return ERR_PTR(ret); } }
    let xa = kzalloc_obj::<xdp_mem_allocator>(); if xa.is_null() { return ERR_PTR(-ENOMEM); }
    mutex_lock(&mut mem_id_lock); let id = __mem_id_cyclic_get(GFP_KERNEL); if id < 0 { mutex_unlock(&mut mem_id_lock); kfree(xa as *mut _); return ERR_PTR(id); }
    (*mem).id = id as u32; (*xa).mem = *mem; (*xa).allocator = allocator;
    let ptr = rhashtable_insert_slow(mem_id_ht, &id, &mut (*xa).node);
    if IS_ERR(ptr) { ida_free(&mut mem_id_pool, (*mem).id); (*mem).id = 0; let e = PTR_ERR(ptr); mutex_unlock(&mut mem_id_lock); kfree(xa as *mut _); return ERR_PTR(e); }
    if typ == MEM_TYPE_PAGE_POOL { page_pool_use_xdp_mem(allocator, mem_allocator_disconnect, mem); }
    mutex_unlock(&mut mem_id_lock); xa
}

unsafe fn __is_supported_mem_type(typ: xdp_mem_type) -> bool { if typ == MEM_TYPE_PAGE_POOL { return is_page_pool_compiled_in(); } typ < MEM_TYPE_MAX }
pub unsafe fn xdp_reg_mem_model(mem: *mut xdp_mem_info, typ: xdp_mem_type, allocator: *mut core::ffi::c_void) -> i32 { let xa=__xdp_reg_mem_model(mem,typ,allocator); if IS_ERR(xa as *mut _){PTR_ERR(xa as *mut _)}else{0} }
pub unsafe fn xdp_rxq_info_reg_mem_model(x: *mut xdp_rxq_info, typ: xdp_mem_type, allocator: *mut core::ffi::c_void) -> i32 { if (*x).reg_state != REG_STATE_REGISTERED { WARN(1,"Missing register, driver bug"); return -EFAULT; } let xa=__xdp_reg_mem_model(&mut (*x).mem,typ,allocator); if IS_ERR(xa as *mut _){return PTR_ERR(xa as *mut _);} if typ==MEM_TYPE_XSK_BUFF_POOL&&!allocator.is_null(){xsk_pool_set_rxq_info(allocator,x);} if trace_mem_connect_enabled()&&!xa.is_null(){trace_mem_connect(xa,x);} 0 }

pub unsafe fn xdp_reg_page_pool(pool: *mut page_pool) -> i32 { let mut mem=core::mem::zeroed(); xdp_reg_mem_model(&mut mem,MEM_TYPE_PAGE_POOL,pool as *mut _) }
pub unsafe fn xdp_unreg_page_pool(pool: *const page_pool) { let mut mem=xdp_mem_info{typ:MEM_TYPE_PAGE_POOL,id:(*pool).xdp_mem_id}; xdp_unreg_mem_model(&mut mem); }
pub unsafe fn xdp_rxq_info_attach_page_pool(x:*mut xdp_rxq_info,pool:*const page_pool){let mem=xdp_mem_info{typ:MEM_TYPE_PAGE_POOL,id:(*pool).xdp_mem_id};xdp_rxq_info_attach_mem_model(x,&mem);}

pub unsafe fn __xdp_return(mut netmem: netmem_ref, typ: xdp_mem_type, mut napi_direct: bool, xdp:*mut xdp_buff){match typ{MEM_TYPE_PAGE_POOL=>{netmem=netmem_compound_head(netmem);if napi_direct&&xdp_return_frame_no_direct(){napi_direct=false;}page_pool_put_full_netmem(netmem_get_pp(netmem),netmem,napi_direct)},MEM_TYPE_PAGE_SHARED=>page_frag_free(__netmem_address(netmem)),MEM_TYPE_PAGE_ORDER0=>put_page(__netmem_to_page(netmem)),MEM_TYPE_XSK_BUFF_POOL=>xsk_buff_free(xdp),_=>{WARN(1,"Incorrect XDP memory type (%d) usage",typ);}}}
pub unsafe fn xdp_return_frame(xdpf:*mut xdp_frame){if xdp_frame_has_frags(xdpf){let s=xdp_get_shared_info_from_frame(xdpf);for i in 0..(*s).nr_frags{__xdp_return(skb_frag_netmem(&mut (*s).frags[i as usize]),(*xdpf).mem_type,false,core::ptr::null_mut());}}__xdp_return(virt_to_netmem((*xdpf).data),(*xdpf).mem_type,false,core::ptr::null_mut());}
pub unsafe fn xdp_return_frame_rx_napi(xdpf:*mut xdp_frame){if xdp_frame_has_frags(xdpf){let s=xdp_get_shared_info_from_frame(xdpf);for i in 0..(*s).nr_frags{__xdp_return(skb_frag_netmem(&mut (*s).frags[i as usize]),(*xdpf).mem_type,true,core::ptr::null_mut());}}__xdp_return(virt_to_netmem((*xdpf).data),(*xdpf).mem_type,true,core::ptr::null_mut());}

pub unsafe fn xdp_return_frame_bulk(xdpf:*mut xdp_frame,bq:*mut xdp_frame_bulk){if (*xdpf).mem_type!=MEM_TYPE_PAGE_POOL{xdp_return_frame(xdpf);return;}if (*bq).count==XDP_BULK_QUEUE_SIZE{xdp_flush_frame_bulk(bq);}if xdp_frame_has_frags(xdpf){let s=xdp_get_shared_info_from_frame(xdpf);for i in 0..(*s).nr_frags{(*bq).q[(*bq).count as usize]=skb_frag_netmem(&mut (*s).frags[i as usize]);(*bq).count+=1;if (*bq).count==XDP_BULK_QUEUE_SIZE{xdp_flush_frame_bulk(bq);}}}(*bq).q[(*bq).count as usize]=virt_to_netmem((*xdpf).data);(*bq).count+=1;}
pub unsafe fn xdp_return_frag(netmem:netmem_ref,xdp:*const xdp_buff){__xdp_return(netmem,(*(*xdp).rxq).mem.typ,true,core::ptr::null_mut());}
pub unsafe fn xdp_return_buff(xdp:*mut xdp_buff){if xdp_buff_has_frags(xdp){let s=xdp_get_shared_info_from_buff(xdp);for i in 0..(*s).nr_frags{__xdp_return(skb_frag_netmem(&mut (*s).frags[i as usize]),(*(*xdp).rxq).mem.typ,true,xdp);}}__xdp_return(virt_to_netmem((*xdp).data),(*(*xdp).rxq).mem.typ,true,xdp);}

pub unsafe fn xdp_attachment_setup(info:*mut xdp_attachment_info,bpf:*mut netdev_bpf){if !(*info).prog.is_null(){bpf_prog_put((*info).prog);}(*info).prog=(*bpf).prog;(*info).flags=(*bpf).flags;}

// The remaining declarations are external kernel-provided helpers and types.
extern "C" {
    fn xdp_rxq_info_attach_mem_model(x:*mut xdp_rxq_info, mem:*const xdp_mem_info);
    fn xdp_flush_frame_bulk(bq:*mut xdp_frame_bulk);
}

pub unsafe fn xdp_convert_zc_to_xdp_frame(xdp:*mut xdp_buff)->*mut xdp_frame{let metasize=if xdp_data_meta_unsupported(xdp){0}else{((*xdp).data as usize-(*xdp).data_meta as usize) as u32};let totsize=((*xdp).data_end as usize-(*xdp).data as usize) as u32+metasize;if core::mem::size_of::<xdp_frame>() as u32+totsize>SKB_WITH_OVERHEAD(PAGE_SIZE){return core::ptr::null_mut();}let page=dev_alloc_page();if page.is_null(){return core::ptr::null_mut();}let addr=page_to_virt(page) as *mut u8;let xdpf=addr as *mut xdp_frame;memset(xdpf as *mut _,0,core::mem::size_of::<xdp_frame>());let src=if metasize!=0{(*xdp).data_meta}else{(*xdp).data};memcpy(addr.add(core::mem::size_of::<xdp_frame>()),src,totsize as usize);(*xdpf).data=addr.add(core::mem::size_of::<xdp_frame>()).add(metasize as usize);(*xdpf).len=totsize-metasize;(*xdpf).headroom=metasize;(*xdpf).metasize=metasize;(*xdpf).frame_sz=PAGE_SIZE;(*xdpf).mem_type=MEM_TYPE_PAGE_ORDER0;xsk_buff_free(xdp);xdpf}
pub unsafe fn xdp_warn(msg:*const i8,func:*const i8,line:i32){WARN(1,"XDP_WARN: %s(line:%d): %s\n",func,line,msg);}

pub unsafe fn bpf_xdp_metadata_rx_timestamp(_ctx:*const xdp_md,_timestamp:*mut u64)->i32{-EOPNOTSUPP}
pub unsafe fn bpf_xdp_metadata_rx_hash(_ctx:*const xdp_md,_hash:*mut u32,_rss_type:*mut xdp_rss_hash_type)->i32{-EOPNOTSUPP}
pub unsafe fn bpf_xdp_metadata_rx_vlan_tag(_ctx:*const xdp_md,_vlan_proto:*mut __be16,_vlan_tci:*mut u16)->i32{-EOPNOTSUPP}

pub unsafe fn bpf_xdp_metadata_kfunc_id(id:i32)->u32{xdp_metadata_kfunc_ids_unsorted[id as usize]}
pub unsafe fn bpf_dev_bound_kfunc_id(btf_id:u32)->bool{btf_id_set8_contains(&xdp_metadata_kfunc_ids,btf_id)}
unsafe fn xdp_metadata_init()->i32{register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP,&xdp_metadata_kfunc_set)}

pub unsafe fn xdp_set_features_flag_locked(dev:*mut net_device,mut val:xdp_features_t){val&=NETDEV_XDP_ACT_MASK;if (*dev).xdp_features==val{return;}netdev_assert_locked_or_invisible(dev);(*dev).xdp_features=val;if (*dev).reg_state==NETREG_REGISTERED{call_netdevice_notifiers(NETDEV_XDP_FEAT_CHANGE,dev);}}
pub unsafe fn xdp_set_features_flag(dev:*mut net_device,val:xdp_features_t){netdev_lock(dev);xdp_set_features_flag_locked(dev,val);netdev_unlock(dev);}
pub unsafe fn xdp_features_set_redirect_target_locked(dev:*mut net_device,support_sg:bool){let mut val=(*dev).xdp_features|NETDEV_XDP_ACT_NDO_XMIT;if support_sg{val|=NETDEV_XDP_ACT_NDO_XMIT_SG;}xdp_set_features_flag_locked(dev,val);}
pub unsafe fn xdp_features_set_redirect_target(dev:*mut net_device,support_sg:bool){netdev_lock(dev);xdp_features_set_redirect_target_locked(dev,support_sg);netdev_unlock(dev);}
pub unsafe fn xdp_features_clear_redirect_target_locked(dev:*mut net_device){let val=(*dev).xdp_features&!(NETDEV_XDP_ACT_NDO_XMIT|NETDEV_XDP_ACT_NDO_XMIT_SG);xdp_set_features_flag_locked(dev,val);}
pub unsafe fn xdp_features_clear_redirect_target(dev:*mut net_device){netdev_lock(dev);xdp_features_clear_redirect_target_locked(dev);netdev_unlock(dev);}

pub unsafe fn xdp_build_skb_from_buff(xdp:*const xdp_buff)->*mut sk_buff{let rxq=(*xdp).rxq;let skb=napi_build_skb((*xdp).data_hard_start,(*xdp).frame_sz);if skb.is_null(){return core::ptr::null_mut();}skb_reserve(skb,(*xdp).data as usize-(*xdp).data_hard_start as usize);__skb_put(skb,(*xdp).data_end as usize-(*xdp).data as usize);let metalen=(*xdp).data as usize-(*xdp).data_meta as usize;if metalen>0{skb_metadata_set(skb,metalen as i32);}if (*rxq).mem.typ==MEM_TYPE_PAGE_POOL{skb_mark_for_recycle(skb);}skb_record_rx_queue(skb,(*rxq).queue_index);(*skb).protocol=eth_type_trans(skb,(*rxq).dev);skb}
pub unsafe fn xdp_build_skb_from_zc(_xdp:*mut xdp_buff)->*mut sk_buff{core::ptr::null_mut()}
pub unsafe fn __xdp_build_skb_from_frame(_xdpf:*mut xdp_frame,_skb:*mut sk_buff,_dev:*mut net_device)->*mut sk_buff{core::ptr::null_mut()}
pub unsafe fn xdp_build_skb_from_frame(_xdpf:*mut xdp_frame,_dev:*mut net_device)->*mut sk_buff{core::ptr::null_mut()}
pub unsafe fn xdpf_clone(xdpf:*mut xdp_frame)->*mut xdp_frame{let headroom=(*xdpf).headroom+core::mem::size_of::<xdp_frame>() as u32;let totalsize=headroom+(*xdpf).len;if totalsize>SKB_WITH_OVERHEAD(PAGE_SIZE){return core::ptr::null_mut();}let page=dev_alloc_page();if page.is_null(){return core::ptr::null_mut();}let addr=page_to_virt(page) as *mut u8;memcpy(addr,xdpf,totalsize as usize);let n=addr as *mut xdp_frame;(*n).data=addr.add(headroom as usize);(*n).frame_sz=PAGE_SIZE;(*n).mem_type=MEM_TYPE_PAGE_ORDER0;n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
