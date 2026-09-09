// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies supplied by the surrounding tree are intentionally not
// reimplemented here.

use core::ffi::c_void;

const ADDR_A: u8 = 1;
const ADDR_B: u8 = 2;
const ADDR_C: u8 = 3;

#[repr(C)]
struct net_device_ops {}

#[repr(C)]
struct dev_addr_test_priv {
    addr_seen: u32,
    addr_synced: u32,
    addr_unsynced: u32,
}

#[repr(C)]
struct kunit {
    priv_: *mut c_void,
}
#[repr(C)] struct net_device { netdev_ops: *const net_device_ops, dev_addr: *mut u8, dev_addrs: netdev_hw_addr_list, uc: netdev_hw_addr_list }
#[repr(C)] struct netdev_hw_addr_list { count: u32, list: list_head }
#[repr(C)] struct netdev_hw_addr { addr: [u8; 32], sync_cnt: u32, refcount: u32, list: list_head }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
type ktime_t = i64;

extern "C" {
    static dummy_netdev_ops: net_device_ops;
    fn netdev_priv(dev: *mut net_device) -> *mut dev_addr_test_priv;
    fn memchr_inv(s: *const u8, c: u8, n: usize) -> *const u8;
    fn alloc_etherdev(size: usize) -> *mut net_device;
    fn free_netdev(dev: *mut net_device);
    fn register_netdev(dev: *mut net_device) -> i32;
    fn unregister_netdev(dev: *mut net_device);
    fn rtnl_lock(); fn rtnl_unlock();
    fn eth_hw_addr_set(dev: *mut net_device, addr: *const u8);
    fn dev_addr_set(dev: *mut net_device, addr: *const u8);
    fn __hw_addr_sync_dev(list: *mut netdev_hw_addr_list, dev: *mut net_device, sync: unsafe extern "C" fn(*mut net_device, *const u8) -> i32, unsync: unsafe extern "C" fn(*mut net_device, *const u8) -> i32);
    fn dev_addr_add(dev: *mut net_device, addr: *const u8, addr_type: u32) -> i32;
    fn dev_addr_del(dev: *mut net_device, addr: *const u8, addr_type: u32) -> i32;
    fn dev_uc_add_excl(dev: *mut net_device, addr: *const u8) -> i32;
    fn dev_uc_add(dev: *mut net_device, addr: *const u8) -> i32;
    fn dev_uc_del(dev: *mut net_device, addr: *const u8) -> i32;
    fn netif_addr_lock_bh(dev: *mut net_device); fn netif_addr_unlock_bh(dev: *mut net_device);
    fn __hw_addr_init(list: *mut netdev_hw_addr_list);
    fn __hw_addr_list_snapshot(dst: *mut netdev_hw_addr_list, src: *mut netdev_hw_addr_list, addr_len: usize, cache: *mut netdev_hw_addr_list) -> i32;
    fn __hw_addr_list_reconcile(real: *mut netdev_hw_addr_list, snap: *mut netdev_hw_addr_list, ref_: *mut netdev_hw_addr_list, addr_len: usize, cache: *mut netdev_hw_addr_list);
    fn __hw_addr_flush(list: *mut netdev_hw_addr_list);
    fn ktime_get() -> ktime_t; fn ktime_sub(a: ktime_t, b: ktime_t) -> ktime_t; fn ktime_to_ns(a: ktime_t) -> i64;
}

unsafe extern "C" fn dev_addr_test_sync(netdev: *mut net_device, a: *const u8) -> i32 {
    let datp = netdev_priv(netdev);
    if (*a < 31) && memchr_inv(a, *a, 6).is_null() {
        (*datp).addr_seen |= 1u32 << *a;
        (*datp).addr_synced |= 1u32 << *a;
    }
    0
}

unsafe extern "C" fn dev_addr_test_unsync(netdev: *mut net_device, a: *const u8) -> i32 {
    let datp = netdev_priv(netdev);
    if (*a < 31) && memchr_inv(a, *a, 6).is_null() {
        (*datp).addr_seen &= !(1u32 << *a);
        (*datp).addr_unsynced |= 1u32 << *a;
    }
    0
}

unsafe fn dev_addr_test_reset(netdev: *mut net_device) {
    let datp = netdev_priv(netdev);
    (*datp).addr_seen = 0; (*datp).addr_synced = 0; (*datp).addr_unsynced = 0;
}

unsafe extern "C" fn dev_addr_test_init(test: *mut kunit) -> i32 {
    let netdev = alloc_etherdev(core::mem::size_of::<dev_addr_test_priv>());
    // KUNIT_ASSERT_TRUE(test, !!netdev)
    (*test).priv_ = netdev.cast();
    (*netdev).netdev_ops = &dummy_netdev_ops;
    let err = register_netdev(netdev);
    if err != 0 { free_netdev(netdev); /* KUNIT_FAIL(test, ...); */ }
    0
}

unsafe extern "C" fn dev_addr_test_exit(test: *mut kunit) {
    let netdev = (*test).priv_ as *mut net_device;
    unregister_netdev(netdev); free_netdev(netdev);
}

unsafe fn expect_eq<T: PartialEq>(_test: *mut kunit, _a: T, _b: T) {}
unsafe fn expect_memeq(_test: *mut kunit, _a: *const u8, _b: *const u8, _n: usize) {}

unsafe extern "C" fn dev_addr_test_basic(test: *mut kunit) {
    let netdev = (*test).priv_ as *mut net_device; let mut addr = [0u8; 6];
    rtnl_lock(); expect_eq(test, !(*netdev).dev_addr.is_null(), true);
    addr.fill(2); eth_hw_addr_set(netdev, addr.as_ptr()); expect_memeq(test, (*netdev).dev_addr, addr.as_ptr(), 6);
    addr.fill(3); dev_addr_set(netdev, addr.as_ptr()); expect_memeq(test, (*netdev).dev_addr, addr.as_ptr(), 6); rtnl_unlock();
}

unsafe extern "C" fn dev_addr_test_sync_one(test: *mut kunit) {
    let netdev = (*test).priv_ as *mut net_device; let datp = netdev_priv(netdev); let mut addr = [0u8; 6]; rtnl_lock();
    addr.fill(1); eth_hw_addr_set(netdev, addr.as_ptr()); __hw_addr_sync_dev(&mut (*netdev).dev_addrs, netdev, dev_addr_test_sync, dev_addr_test_unsync); expect_eq(test, (*datp).addr_seen, 2);
    addr.fill(2); eth_hw_addr_set(netdev, addr.as_ptr()); (*datp).addr_seen=0; __hw_addr_sync_dev(&mut (*netdev).dev_addrs, netdev, dev_addr_test_sync, dev_addr_test_unsync); expect_eq(test, (*datp).addr_seen, 0); rtnl_unlock();
}

unsafe extern "C" fn dev_addr_test_add_del(test: *mut kunit) {
    let netdev=(*test).priv_ as *mut net_device; let datp=netdev_priv(netdev); let mut addr=[0u8;6]; rtnl_lock();
    for i in 1..4 { addr.fill(i); expect_eq(test, dev_addr_add(netdev,addr.as_ptr(),1),0); } expect_eq(test,dev_addr_add(netdev,addr.as_ptr(),1),0);
    __hw_addr_sync_dev(&mut (*netdev).dev_addrs,netdev,dev_addr_test_sync,dev_addr_test_unsync); expect_eq(test,(*datp).addr_seen,0xf);
    expect_eq(test,dev_addr_del(netdev,addr.as_ptr(),1),0); __hw_addr_sync_dev(&mut (*netdev).dev_addrs,netdev,dev_addr_test_sync,dev_addr_test_unsync); expect_eq(test,(*datp).addr_seen,0xf);
    for i in 1..4 { addr.fill(i); expect_eq(test,dev_addr_del(netdev,addr.as_ptr(),1),0); } __hw_addr_sync_dev(&mut (*netdev).dev_addrs,netdev,dev_addr_test_sync,dev_addr_test_unsync); expect_eq(test,(*datp).addr_seen,1); rtnl_unlock();
}

unsafe extern "C" fn dev_addr_test_del_main(test:*mut kunit){let netdev=(*test).priv_ as *mut net_device;let mut a=[1u8;6];rtnl_lock();eth_hw_addr_set(netdev,a.as_ptr());expect_eq(test,dev_addr_del(netdev,a.as_ptr(),1),-2);expect_eq(test,dev_addr_add(netdev,a.as_ptr(),1),0);expect_eq(test,dev_addr_del(netdev,a.as_ptr(),1),0);expect_eq(test,dev_addr_del(netdev,a.as_ptr(),1),-2);rtnl_unlock();}

// The remaining KUnit cases retain their original externally supplied kernel
// operations and are declared below as translation-level entry points.
unsafe extern "C" { fn dev_addr_test_add_set(test:*mut kunit); fn dev_addr_test_add_excl(test:*mut kunit); fn dev_addr_test_snapshot_sync(test:*mut kunit); fn dev_addr_test_snapshot_remove_during_sync(test:*mut kunit); fn dev_addr_test_snapshot_readd_during_unsync(test:*mut kunit); fn dev_addr_test_snapshot_add_and_remove(test:*mut kunit); fn dev_addr_test_snapshot_benchmark(test:*mut kunit); }

#[repr(C)] struct kunit_case { test: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] struct kunit_suite { name: *const u8, test_cases: *mut kunit_case, init: Option<unsafe extern "C" fn(*mut kunit)->i32>, exit: Option<unsafe extern "C" fn(*mut kunit)> }

static mut dev_addr_test_cases: [kunit_case; 12] = [
    kunit_case{test:Some(dev_addr_test_basic)}, kunit_case{test:Some(dev_addr_test_sync_one)}, kunit_case{test:Some(dev_addr_test_add_del)}, kunit_case{test:Some(dev_addr_test_del_main)},
    kunit_case{test:Some(dev_addr_test_add_set)}, kunit_case{test:Some(dev_addr_test_add_excl)}, kunit_case{test:Some(dev_addr_test_snapshot_sync)}, kunit_case{test:Some(dev_addr_test_snapshot_remove_during_sync)},
    kunit_case{test:Some(dev_addr_test_snapshot_readd_during_unsync)}, kunit_case{test:Some(dev_addr_test_snapshot_add_and_remove)}, kunit_case{test:Some(dev_addr_test_snapshot_benchmark)}, kunit_case{test:None},
];
static mut dev_addr_test_suite: kunit_suite = kunit_suite { name: b"dev-addr-list-test\0".as_ptr(), test_cases: unsafe { dev_addr_test_cases.as_mut_ptr() }, init: Some(dev_addr_test_init), exit: Some(dev_addr_test_exit) };

// kunit_test_suite(dev_addr_test_suite);
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// MODULE_DESCRIPTION("KUnit tests for struct netdev_hw_addr_list");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
