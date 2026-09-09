// SPDX-License-Identifier: GPL-2.0
// Translated from fib_notifier.c. Kernel headers and external symbols are
// supplied by the surrounding Rust kernel bindings.

use core::ffi::c_void;

static mut FIB_NOTIFIER_NET_ID: u32 = 0;

#[repr(C)]
pub struct FibNotifierNet {
    pub fib_notifier_ops: ListHead,
    pub fib_chain: AtomicNotifierHead,
}

extern "C" {
    fn notifier_to_errno(err: i32) -> i32;
    fn atomic_notifier_call_chain(chain: *mut AtomicNotifierHead, val: u32, v: *mut FibNotifierInfo) -> i32;
    fn atomic_notifier_chain_register(chain: *mut AtomicNotifierHead, nb: *mut NotifierBlock) -> i32;
    fn atomic_notifier_chain_unregister(chain: *mut AtomicNotifierHead, nb: *mut NotifierBlock) -> i32;
    fn net_generic(net: *mut Net, id: u32) -> *mut c_void;
    fn try_module_get(owner: *mut Module) -> bool;
    fn module_put(owner: *mut Module);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn fib_seq_read(ops: *mut FibNotifierOps, net: *mut Net) -> u32;
    fn fib_dump(ops: *mut FibNotifierOps, net: *mut Net, nb: *mut NotifierBlock, extack: *mut NetlinkExtAck) -> i32;
    fn list_add_tail_rcu(new: *mut ListHead, head: *mut ListHead);
    fn list_del_rcu(entry: *mut ListHead);
    fn kmemdup(src: *const c_void, size: usize, flags: u32) -> *mut FibNotifierOps;
    fn kfree(ptr: *mut FibNotifierOps);
    fn kfree_rcu(ptr: *mut FibNotifierOps, field: *mut RcuHead);
    fn init_list_head(list: *mut ListHead);
    fn atomic_init_notifier_head(head: *mut AtomicNotifierHead);
    fn list_empty(list: *const ListHead) -> bool;
    fn warn_on_once(condition: bool) -> bool;
    fn register_pernet_subsys(ops: *mut PernetOperations) -> i32;
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct AtomicNotifierHead { pub head: *mut NotifierBlock }
#[repr(C)] pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, u32, *mut FibNotifierInfo) -> i32> }
#[repr(C)] pub struct FibNotifierInfo { _private: [u8; 0] }
#[repr(C)] pub struct Net { _private: [u8; 0] }
#[repr(C)] pub struct NetlinkExtAck { _private: [u8; 0] }
#[repr(C)] pub struct Module { _private: [u8; 0] }
#[repr(C)] pub struct RcuHead { _private: [u8; 0] }
#[repr(C)] pub struct FibNotifierOps {
    pub list: ListHead,
    pub rcu: RcuHead,
    pub owner: *mut Module,
    pub family: u8,
    pub fib_seq_read: Option<unsafe extern "C" fn(*mut Net) -> u32>,
    pub fib_dump: Option<unsafe extern "C" fn(*mut Net, *mut NotifierBlock, *mut NetlinkExtAck) -> i32>,
}
#[repr(C)] pub struct PernetOperations {
    pub init: Option<unsafe extern "C" fn(*mut Net) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut Net)>,
    pub id: *mut u32,
    pub size: usize,
}

unsafe fn fn_net(net: *mut Net) -> *mut FibNotifierNet {
    net_generic(net, FIB_NOTIFIER_NET_ID) as *mut FibNotifierNet
}

#[no_mangle]
pub unsafe extern "C" fn call_fib_notifier(nb: *mut NotifierBlock, event_type: u32, info: *mut FibNotifierInfo) -> i32 {
    notifier_to_errno(((*nb).notifier_call.unwrap())(nb, event_type, info))
}

#[no_mangle]
pub unsafe extern "C" fn call_fib_notifiers(net: *mut Net, event_type: u32, info: *mut FibNotifierInfo) -> i32 {
    let fn_net = fn_net(net);
    notifier_to_errno(atomic_notifier_call_chain(&mut (*fn_net).fib_chain, event_type, info))
}

unsafe fn fib_seq_sum(net: *mut Net) -> u32 {
    let fn_net = fn_net(net); let mut fib_seq = 0;
    rcu_read_lock();
    // list_for_each_entry_rcu(ops, &fn_net->fib_notifier_ops, list)
    let mut ops = (*fn_net).fib_notifier_ops.next as *mut FibNotifierOps;
    while !ops.is_null() {
        if try_module_get((*ops).owner) {
            fib_seq = fib_seq.wrapping_add(((*ops).fib_seq_read.unwrap())(net));
            module_put((*ops).owner);
        }
        ops = (*ops).list.next as *mut FibNotifierOps;
    }
    rcu_read_unlock(); fib_seq
}

unsafe fn fib_net_dump(net: *mut Net, nb: *mut NotifierBlock, extack: *mut NetlinkExtAck) -> i32 {
    let fn_net = fn_net(net); let mut err = 0; rcu_read_lock();
    let mut ops = (*fn_net).fib_notifier_ops.next as *mut FibNotifierOps;
    while !ops.is_null() {
        if try_module_get((*ops).owner) { err = ((*ops).fib_dump.unwrap())(net, nb, extack); module_put((*ops).owner); if err != 0 { break; } }
        ops = (*ops).list.next as *mut FibNotifierOps;
    }
    rcu_read_unlock(); err
}

unsafe fn fib_dump_is_consistent(net: *mut Net, nb: *mut NotifierBlock, cb: Option<unsafe extern "C" fn(*mut NotifierBlock)>, fib_seq: u32) -> bool {
    let chain = &mut (*fn_net(net)).fib_chain;
    atomic_notifier_chain_register(chain, nb);
    if fib_seq == fib_seq_sum(net) { return true; }
    atomic_notifier_chain_unregister(chain, nb); if let Some(cb) = cb { cb(nb); } false
}

pub const FIB_DUMP_MAX_RETRIES: i32 = 5;
#[no_mangle]
pub unsafe extern "C" fn register_fib_notifier(net: *mut Net, nb: *mut NotifierBlock, cb: Option<unsafe extern "C" fn(*mut NotifierBlock)>, extack: *mut NetlinkExtAck) -> i32 {
    let mut retries = 0;
    loop { let seq = fib_seq_sum(net); let err = fib_net_dump(net, nb, extack); if err != 0 { return err; } if fib_dump_is_consistent(net, nb, cb, seq) { return 0; } retries += 1; if retries >= FIB_DUMP_MAX_RETRIES { return -16; } }
}

#[no_mangle]
pub unsafe extern "C" fn unregister_fib_notifier(net: *mut Net, nb: *mut NotifierBlock) -> i32 { atomic_notifier_chain_unregister(&mut (*fn_net(net)).fib_chain, nb) }

unsafe fn fib_notifier_ops_register_inner(ops: *mut FibNotifierOps, net: *mut Net) -> i32 {
    let fn_net = fn_net(net); let mut o = (*fn_net).fib_notifier_ops.next as *mut FibNotifierOps;
    while !o.is_null() { if (*ops).family == (*o).family { return -17; } o = (*o).list.next as *mut FibNotifierOps; }
    list_add_tail_rcu(&mut (*ops).list, &mut (*fn_net).fib_notifier_ops); 0
}

#[no_mangle]
pub unsafe extern "C" fn fib_notifier_ops_register(tmpl: *const FibNotifierOps, net: *mut Net) -> *mut FibNotifierOps {
    let ops = kmemdup(tmpl as *const c_void, core::mem::size_of::<FibNotifierOps>(), 0);
    if ops.is_null() { return (-12isize) as *mut FibNotifierOps; }
    let err = fib_notifier_ops_register_inner(ops, net); if err != 0 { kfree(ops); return (err as isize) as *mut FibNotifierOps; } ops
}

#[no_mangle]
pub unsafe extern "C" fn fib_notifier_ops_unregister(ops: *mut FibNotifierOps) { list_del_rcu(&mut (*ops).list); kfree_rcu(ops, &mut (*ops).rcu); }

unsafe extern "C" fn fib_notifier_net_init(net: *mut Net) -> i32 { let n = fn_net(net); init_list_head(&mut (*n).fib_notifier_ops); atomic_init_notifier_head(&mut (*n).fib_chain); 0 }
unsafe extern "C" fn fib_notifier_net_exit(net: *mut Net) { let n = fn_net(net); warn_on_once(!list_empty(&(*n).fib_notifier_ops)); }

static mut FIB_NOTIFIER_NET_OPS: PernetOperations = PernetOperations { init: Some(fib_notifier_net_init), exit: Some(fib_notifier_net_exit), id: unsafe { &raw mut FIB_NOTIFIER_NET_ID }, size: core::mem::size_of::<FibNotifierNet>() };

unsafe extern "C" fn fib_notifier_init() -> i32 { register_pernet_subsys(&raw mut FIB_NOTIFIER_NET_OPS) }

// Equivalent of subsys_initcall(fib_notifier_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
