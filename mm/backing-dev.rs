// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)] pub struct backing_dev_info { pub dev: *mut device, pub refcnt: kref, pub min_ratio: u32, pub max_ratio: u32, pub max_prop_frac: u32, pub bdi_list: list_head, pub wb_list: list_head, pub wb_waitq: wait_queue_head, pub last_bdp_sleep: u64, pub capabilities: u32, pub ra_pages: u64, pub io_pages: u64, pub owner: *mut device, pub id: u64, pub rb_node: rb_node, pub wb: bdi_writeback, pub cgwb_tree: radix_tree_root, pub cgwb_release_mutex: mutex, pub wb_switch_rwsem: rw_semaphore, pub dev_name: [u8; 64], pub debug_dir: *mut dentry }
#[repr(C)] pub struct bdi_writeback { pub bdi: *mut backing_dev_info, pub state: usize, pub list_lock: spinlock, pub b_dirty: list_head, pub b_io: list_head, pub b_more_io: list_head, pub b_dirty_time: list_head, pub work_lock: spinlock, pub work_list: list_head, pub dwork: delayed_work, pub bw_dwork: delayed_work, pub writeback_inodes: atomic_t, pub bw_time_stamp: u64, pub balanced_dirty_ratelimit: u64, pub dirty_ratelimit: u64, pub write_bandwidth: u64, pub avg_write_bandwidth: u64, pub completions: fprop_local_percpu, pub stat: [percpu_counter; 8], pub memcg_css: *mut cgroup_subsys_state, pub blkcg_css: *mut cgroup_subsys_state, pub b_attached: list_head, pub switch_work: work_struct, pub switch_wbs_ctxs: llist_head, pub release_work: work_struct, pub refcnt: percpu_ref, pub rcu: rcu_head, pub offline_node: list_head, pub bdi_node: list_head, pub memcg_node: list_head, pub blkcg_node: list_head, pub memcg_completions: fprop_local_percpu }
pub type u64_t = u64; pub type gfp_t = usize; pub type ssize_t = isize; pub type va_list = *mut core::ffi::c_void;
pub struct device; pub struct dentry; pub struct inode; pub struct super_block; pub struct mem_cgroup; pub struct cgroup_subsys_state; pub struct seq_file; pub struct work_struct; pub struct rb_node; pub struct rb_root; pub struct list_head; pub struct wait_queue_head; pub struct kref; pub struct spinlock; pub struct mutex; pub struct rw_semaphore; pub struct radix_tree_root; pub struct delayed_work; pub struct atomic_t; pub struct fprop_local_percpu; pub struct percpu_counter; pub struct llist_head; pub struct percpu_ref; pub struct rcu_head; pub struct blkcg; pub struct class;

pub static mut noop_backing_dev_info: backing_dev_info = unsafe { core::mem::zeroed() };
static mut bdi_unknown_name: &[u8] = b"(unknown)\0";
static mut bdi_id_cursor: u64 = 0;
static mut bdi_tree: rb_root = unsafe { core::mem::zeroed() };
static mut bdi_list: list_head = unsafe { core::mem::zeroed() };
pub static mut bdi_wq: *mut workqueue_struct = core::ptr::null_mut();
pub struct workqueue_struct;

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct wb_stats { nr_dirty: usize, nr_io: usize, nr_more_io: usize, nr_dirty_time: usize, nr_writeback: usize, nr_reclaimable: usize, nr_dirtied: usize, nr_written: usize, dirty_thresh: usize, wb_thresh: usize }

extern "C" {
    fn bdi_debug_init(); fn bdi_debug_register(bdi: *mut backing_dev_info, name: *const u8); fn bdi_debug_unregister(bdi: *mut backing_dev_info);
    fn wb_init(wb: *mut bdi_writeback, bdi: *mut backing_dev_info, gfp: gfp_t) -> i32; fn wb_shutdown(wb: *mut bdi_writeback); fn wb_exit(wb: *mut bdi_writeback);
    fn cgwb_bdi_init(bdi: *mut backing_dev_info) -> i32; fn cgwb_bdi_unregister(bdi: *mut backing_dev_info); fn cgwb_bdi_register(bdi: *mut backing_dev_info);
    fn wb_get(bdi: *mut backing_dev_info, css: *mut cgroup_subsys_state, gfp: gfp_t) -> *mut bdi_writeback;
    fn wb_put(wb: *mut bdi_writeback); fn wb_update_bandwidth(wb: *mut bdi_writeback); fn wb_workfn(work: *mut work_struct);
    fn alloc_workqueue(name: *const u8, flags: u32, max_active: u32) -> *mut workqueue_struct;
    fn kzalloc_node(size: usize, gfp: gfp_t, node: i32) -> *mut core::ffi::c_void; fn kfree(p: *mut core::ffi::c_void);
    fn kref_init(k: *mut kref); fn kref_put(k: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn device_create(c: *const class, parent: *mut device, dev: u64, drv: *mut core::ffi::c_void, name: *const u8) -> *mut device;
    fn device_unregister(dev: *mut device); fn get_device(dev: *mut device); fn put_device(dev: *mut device); fn dev_name(dev: *mut device) -> *const u8;
    fn bdi_set_min_ratio(bdi: *mut backing_dev_info, ratio: u32) -> i32; fn bdi_set_max_ratio(bdi: *mut backing_dev_info, ratio: u32) -> i32;
    fn bdi_set_min_ratio_no_scale(bdi: *mut backing_dev_info, ratio: u32) -> i32; fn bdi_set_max_ratio_no_scale(bdi: *mut backing_dev_info, ratio: u32) -> i32;
    fn bdi_set_min_bytes(bdi: *mut backing_dev_info, bytes: u64) -> i32; fn bdi_set_max_bytes(bdi: *mut backing_dev_info, bytes: u64) -> i32;
    fn bdi_set_strict_limit(bdi: *mut backing_dev_info, value: u32) -> i32; fn bdi_get_min_bytes(bdi: *mut backing_dev_info) -> u64; fn bdi_get_max_bytes(bdi: *mut backing_dev_info) -> u64;
    fn inode_to_bdi_external(inode: *mut inode) -> *mut backing_dev_info;
}

pub unsafe fn bdi_init(bdi: *mut backing_dev_info) -> i32 { (*bdi).dev = core::ptr::null_mut(); kref_init(&mut (*bdi).refcnt); (*bdi).min_ratio=0; (*bdi).max_ratio=100*100; (*bdi).max_prop_frac=1; cgwb_bdi_init(bdi) }
pub unsafe fn bdi_alloc(node_id: i32) -> *mut backing_dev_info { let bdi=kzalloc_node(core::mem::size_of::<backing_dev_info>(),0,node_id) as *mut backing_dev_info; if bdi.is_null(){return core::ptr::null_mut()} if bdi_init(bdi)!=0 { kfree(bdi as *mut _); return core::ptr::null_mut() } (*bdi).capabilities=1; bdi }
pub unsafe fn bdi_get_by_id(_id: u64) -> *mut backing_dev_info { core::ptr::null_mut() }
pub unsafe fn bdi_register_va(bdi: *mut backing_dev_info, _fmt: *const u8, _args: va_list) -> i32 { if !(*bdi).dev.is_null(){return 0} cgwb_bdi_register(bdi); 0 }
pub unsafe fn bdi_register(bdi: *mut backing_dev_info, fmt: *const u8, _args: va_list) -> i32 { bdi_register_va(bdi,fmt,core::ptr::null_mut()) }
pub unsafe fn bdi_set_owner(bdi: *mut backing_dev_info, owner: *mut device) { (*bdi).owner=owner; get_device(owner); }
pub unsafe fn bdi_unregister(bdi: *mut backing_dev_info) { wb_shutdown(&mut (*bdi).wb); cgwb_bdi_unregister(bdi); if !(*bdi).owner.is_null(){put_device((*bdi).owner);(*bdi).owner=core::ptr::null_mut()} }
unsafe extern "C" fn release_bdi(_ref: *mut kref) {}
pub unsafe fn bdi_put(bdi: *mut backing_dev_info) { kref_put(&mut (*bdi).refcnt,release_bdi); }
pub unsafe fn inode_to_bdi(inode: *mut inode) -> *mut backing_dev_info { if inode.is_null(){&raw mut noop_backing_dev_info}else{inode_to_bdi_external(inode)} }
pub unsafe fn bdi_dev_name(bdi: *mut backing_dev_info) -> *const u8 { if bdi.is_null()||(*bdi).dev.is_null(){bdi_unknown_name.as_ptr()}else{(*bdi).dev_name.as_ptr()} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
