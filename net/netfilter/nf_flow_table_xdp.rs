// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: linux/kernel.h, linux/module.h, linux/netfilter.h,
// linux/rhashtable.h, linux/netdevice.h, net/flow_offload.h,
// net/netfilter/nf_flow_table.h

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct nf_flowtable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

pub type flow_block_command = u32;
pub const FLOW_BLOCK_BIND: flow_block_command = 0;
pub const FLOW_BLOCK_UNBIND: flow_block_command = 1;

#[repr(C)]
pub struct flow_offload_xdp_ft {
    pub head: list_head,
    pub ft: *mut nf_flowtable,
    pub rcuhead: rcu_head,
}

#[repr(C)]
pub struct flow_offload_xdp {
    pub hnode: hlist_node,
    pub net_device_addr: usize,
    pub head: list_head,
}

pub const NF_XDP_HT_BITS: usize = 4;

// DEFINE_HASHTABLE(nf_xdp_hashtable, NF_XDP_HT_BITS);
// DEFINE_MUTEX(nf_xdp_hashtable_lock);
static mut nf_xdp_hashtable: [usize; 1 << NF_XDP_HT_BITS] = [0; 1 << NF_XDP_HT_BITS];
static mut nf_xdp_hashtable_lock: usize = 0;

extern "C" {
    fn kzalloc_obj<T>(flags: u32) -> *mut T;
    fn kfree<T>(ptr: *mut T);
    fn mutex_lock(lock: *mut usize);
    fn mutex_unlock(lock: *mut usize);
    fn synchronize_rcu();
    fn hash_add_rcu(table: *mut usize, node: *mut hlist_node, key: usize);
    fn hash_del_rcu(node: *mut hlist_node);
    fn list_add_tail_rcu(node: *mut list_head, head: *mut list_head);
    fn list_del_rcu(node: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn kfree_rcu<T>(ptr: *mut T, member: *mut rcu_head);
    fn warn_on_once(condition: bool) -> bool;
}

/* caller must hold rcu read lock */
pub unsafe extern "C" fn nf_flowtable_by_dev(dev: *const net_device) -> *mut nf_flowtable {
    let key = dev as usize;
    let mut iter: *mut flow_offload_xdp = core::ptr::null_mut();

    // hash_for_each_possible_rcu(nf_xdp_hashtable, iter, hnode, key)
    while !iter.is_null() {
        if key == (*iter).net_device_addr {
            let mut ft_elem: *mut flow_offload_xdp_ft;

            /* The user is supposed to insert a given net_device
             * just into a single nf_flowtable so we always return
             * the first element here.
             */
            // list_first_or_null_rcu(&iter->head, struct flow_offload_xdp_ft, head)
            ft_elem = core::ptr::null_mut();
            return if !ft_elem.is_null() { (*ft_elem).ft } else { core::ptr::null_mut() };
        }
        break;
    }

    core::ptr::null_mut()
}

unsafe fn nf_flowtable_by_dev_insert(ft: *mut nf_flowtable, dev: *const net_device) -> i32 {
    let mut iter: *mut flow_offload_xdp = core::ptr::null_mut();
    let mut elem: *mut flow_offload_xdp = core::ptr::null_mut();
    let key = dev as usize;
    let ft_elem = kzalloc_obj::<flow_offload_xdp_ft>(0);
    if ft_elem.is_null() {
        return -12;
    }

    (*ft_elem).ft = ft;
    mutex_lock(&raw mut nf_xdp_hashtable_lock);

    // hash_for_each_possible(nf_xdp_hashtable, iter, hnode, key)
    while !iter.is_null() {
        if key == (*iter).net_device_addr {
            elem = iter;
            break;
        }
        break;
    }

    if elem.is_null() {
        elem = kzalloc_obj::<flow_offload_xdp>(0);
        if elem.is_null() {
            mutex_unlock(&raw mut nf_xdp_hashtable_lock);
            kfree(ft_elem);
            return -12;
        }
        (*elem).net_device_addr = key;
        (*elem).head = list_head { next: &raw mut (*elem).head, prev: &raw mut (*elem).head };
        hash_add_rcu(nf_xdp_hashtable.as_mut_ptr(), &raw mut (*elem).hnode, key);
    }
    list_add_tail_rcu(&raw mut (*ft_elem).head, &raw mut (*elem).head);
    mutex_unlock(&raw mut nf_xdp_hashtable_lock);
    0
}

unsafe fn nf_flowtable_by_dev_remove(ft: *mut nf_flowtable, dev: *const net_device) {
    let mut iter: *mut flow_offload_xdp = core::ptr::null_mut();
    let mut elem: *mut flow_offload_xdp = core::ptr::null_mut();
    let key = dev as usize;
    mutex_lock(&raw mut nf_xdp_hashtable_lock);

    // hash_for_each_possible(nf_xdp_hashtable, iter, hnode, key)
    while !iter.is_null() {
        if key == (*iter).net_device_addr { elem = iter; break; }
        break;
    }

    if !elem.is_null() {
        // list_for_each_entry_safe(ft_elem, ft_next, &elem->head, head)
        let ft_elem: *mut flow_offload_xdp_ft = core::ptr::null_mut();
        let _ft_next: *mut flow_offload_xdp_ft = core::ptr::null_mut();
        if !ft_elem.is_null() && (*ft_elem).ft == ft {
            list_del_rcu(&raw mut (*ft_elem).head);
            kfree_rcu(ft_elem, &raw mut (*ft_elem).rcuhead);
        }
        if list_empty(&(*elem).head) { hash_del_rcu(&raw mut (*elem).hnode); }
        else { elem = core::ptr::null_mut(); }
    }
    mutex_unlock(&raw mut nf_xdp_hashtable_lock);
    if !elem.is_null() { synchronize_rcu(); kfree(elem); }
}

pub unsafe extern "C" fn nf_flow_offload_xdp_setup(
    flowtable: *mut nf_flowtable,
    dev: *mut net_device,
    cmd: flow_block_command,
) -> i32 {
    match cmd {
        FLOW_BLOCK_BIND => nf_flowtable_by_dev_insert(flowtable, dev),
        FLOW_BLOCK_UNBIND => { nf_flowtable_by_dev_remove(flowtable, dev); 0 }
        _ => { warn_on_once(true); 0 }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
