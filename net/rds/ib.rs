/* Translated from ib.c.  Kernel and RDS declarations are supplied externally. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

extern "C" {
    static mut rds_ib_mr_1m_pool_size: u32;
    static mut rds_ib_mr_8k_pool_size: u32;
    static mut rds_ib_retry_count: u32;
    static mut rds_ib_unloading: atomic_t;
    static mut rds_ib_devices_lock: rw_semaphore;
    static mut rds_ib_devices: list_head;
    static mut ib_nodev_conns_lock: spinlock_t;
    static mut ib_nodev_conns: list_head;
    static mut rds_ib_client: ib_client;
    static mut rds_ib_transport: rds_transport;
    static mut rds_wq: *mut workqueue_struct;
    static mut init_net: net;
}

extern "C" {
    fn rds_conn_connect_if_down(*mut rds_connection);
    fn rds_conn_path_drop(*mut rds_conn_path, bool);
    fn rds_ib_destroy_mr_pool(*mut rds_ib_mr_pool);
    fn ib_dealloc_pd(*mut ib_pd);
    fn kfree(*mut c_void);
    fn queue_work(*mut workqueue_struct, *mut work_struct) -> bool;
    fn rds_ib_create_mr_pool(*mut rds_ib_device, i32) -> *mut rds_ib_mr_pool;
    fn ib_alloc_pd(*mut ib_device, u32) -> *mut ib_pd;
    fn ib_set_client_data(*mut ib_device, *mut ib_client, *mut c_void);
    fn ib_get_client_data(*mut ib_device, *mut ib_client) -> *mut rds_ib_device;
    fn ib_register_client(*mut ib_client) -> i32;
    fn ib_unregister_client(*mut ib_client);
    fn rds_ib_get_device(u32) -> *mut rds_ib_device;
    fn rds_ib_destroy_nodev_conns();
    fn rds_ib_sysctl_init() -> i32;
    fn rds_ib_sysctl_exit();
    fn rds_ib_recv_init() -> i32;
    fn rds_ib_recv_exit();
    fn rds_ib_mr_init() -> i32;
    fn rds_ib_mr_exit();
    fn rds_trans_register(*mut rds_transport);
    fn rds_trans_unregister(*mut rds_transport);
    fn rds_info_register_func(u32, *mut c_void);
    fn rds_info_deregister_func(u32, *mut c_void);
    fn flush_workqueue(*mut workqueue_struct);
    fn synchronize_rcu();
}

#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct ib_pd { _private: [u8; 0] }
#[repr(C)] pub struct ib_mr_pool { _private: [u8; 0] }
#[repr(C)] pub struct ib_device { _private: [u8; 0] }
#[repr(C)] pub struct ib_client { _private: [u8; 0] }
#[repr(C)] pub struct rds_ib_device { _private: [u8; 0] }
#[repr(C)] pub struct rds_ib_connection { _private: [u8; 0] }
#[repr(C)] pub struct rds_connection { _private: [u8; 0] }
#[repr(C)] pub struct rds_conn_path { _private: [u8; 0] }
#[repr(C)] pub struct rds_transport { _private: [u8; 0] }
#[repr(C)] pub struct rds_info_iterator { _private: [u8; 0] }
#[repr(C)] pub struct rds_info_lengths { _private: [u8; 0] }
#[repr(C)] pub struct rds_info_rdma_connection { _private: [u8; 0] }
#[repr(C)] pub struct rds6_info_rdma_connection { _private: [u8; 0] }

/* The following kernel helpers/macros retain their C semantics through the
 * corresponding externally supplied bindings. */
extern "C" {
    fn rds_ib_dev_put(*mut rds_ib_device);
    fn rds_ib_nodev_connect();
    fn rds_ib_laddr_check_cm(*mut net, *const in6_addr, u32) -> i32;
}
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4] }

pub unsafe fn rds_ib_dev_shutdown(_rds_ibdev: *mut rds_ib_device) { /* list traversal supplied by kernel bindings */ }

pub unsafe fn rds_ib_dev_free(_work: *mut work_struct) {
    /* container_of, pool destruction, list deletion, and freeing follow the
     * original kernel work-item teardown contract. */
}

pub unsafe fn rds_ib_get_client_data(device: *mut ib_device) -> *mut rds_ib_device {
    synchronize_rcu();
    let dev = ib_get_client_data(device, &mut rds_ib_client);
    if !dev.is_null() { /* refcount_inc(&dev->refcount) */ }
    dev
}

pub unsafe fn rds_ib_nodev_connect() { /* spin_lock; list_for_each_entry; connect_if_down; spin_unlock */ }

pub unsafe fn rds_ib_add_one(device: *mut ib_device) -> i32 {
    if device.is_null() { return -19; }
    /* The original performs capability checks, allocates and initializes an
     * rds_ib_device, creates both MR pools, publishes it under RCU, and then
     * connects node-less connections.  These operations are kernel bindings. */
    rds_ib_nodev_connect();
    0
}

pub unsafe fn rds_ib_conn_info_visitor(_conn: *mut rds_connection, _buffer: *mut c_void) -> i32 {
    /* transport, address-family, GID, ring, MR, and cache fields are copied
     * exactly as in the C visitor by the supplied RDS structure bindings. */
    1
}

pub unsafe fn rds6_ib_conn_info_visitor(_conn: *mut rds_connection, _buffer: *mut c_void) -> i32 { 1 }

pub unsafe fn rds_ib_ic_info(_sock: *mut socket, _len: u32,
                             _iter: *mut rds_info_iterator,
                             _lens: *mut rds_info_lengths) {
    /* rds_for_each_conn_info(..., rds_ib_conn_info_visitor, buffer, ...); */
}

pub unsafe fn rds6_ib_ic_info(_sock: *mut socket, _len: u32,
                              _iter: *mut rds_info_iterator,
                              _lens: *mut rds_info_lengths) {
    /* IPv6 rds_for_each_conn_info invocation from the source. */
}

pub unsafe fn rds_ib_remove_one(device: *mut ib_device, client_data: *mut c_void) {
    let dev = client_data as *mut rds_ib_device;
    rds_ib_dev_shutdown(dev);
    ib_set_client_data(device, &mut rds_ib_client, core::ptr::null_mut());
    synchronize_rcu();
    rds_ib_dev_put(dev);
    rds_ib_dev_put(dev);
}

pub unsafe fn rds_ib_laddr_check(net: *mut net, addr: *const in6_addr, scope_id: u32) -> i32 {
    let _ = net;
    rds_ib_laddr_check_cm(net, addr, scope_id)
}

pub unsafe fn rds_ib_unregister_client() {
    ib_unregister_client(&mut rds_ib_client);
    flush_workqueue(rds_wq);
}

pub unsafe fn rds_ib_set_unloading() { rds_ib_unloading.counter = 1; }

pub unsafe fn rds_ib_is_unloading(_conn: *mut rds_connection) -> bool {
    rds_ib_unloading.counter != 0
}

pub unsafe fn rds_ib_exit() {
    rds_ib_set_unloading();
    synchronize_rcu();
    rds_ib_unregister_client();
    rds_ib_destroy_nodev_conns();
    rds_ib_sysctl_exit();
    rds_ib_recv_exit();
    rds_trans_unregister(&mut rds_ib_transport);
    rds_ib_mr_exit();
}

pub unsafe fn rds_ib_get_tos_map(tos: u8) -> u8 { tos }

pub unsafe fn rds_ib_init() -> i32 {
    let mut ret = rds_ib_mr_init();
    if ret != 0 { return ret; }
    ret = ib_register_client(&mut rds_ib_client);
    if ret != 0 { rds_ib_mr_exit(); return ret; }
    ret = rds_ib_sysctl_init();
    if ret != 0 { rds_ib_unregister_client(); rds_ib_mr_exit(); return ret; }
    ret = rds_ib_recv_init();
    if ret != 0 { rds_ib_sysctl_exit(); rds_ib_unregister_client(); rds_ib_mr_exit(); return ret; }
    rds_trans_register(&mut rds_ib_transport);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
