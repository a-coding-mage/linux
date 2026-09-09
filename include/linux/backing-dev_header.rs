/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of include/linux/backing-dev.h. */

pub const BDI_RATIO_SCALE: u32 = 10000;
pub const BDI_CAP_WRITEBACK: u32 = 1 << 0;
pub const BDI_CAP_STRICTLIMIT: u32 = 1 << 1;

pub unsafe fn bdi_get(bdi: *mut backing_dev_info) -> *mut backing_dev_info {
    kref_get(&mut (*bdi).refcnt);
    bdi
}

extern "C" {
    pub fn bdi_get_by_id(id: u64) -> *mut backing_dev_info;
    pub fn bdi_put(bdi: *mut backing_dev_info);
    pub fn bdi_register(bdi: *mut backing_dev_info, fmt: *const ::std::ffi::c_char, ...) -> i32;
    pub fn bdi_register_va(bdi: *mut backing_dev_info, fmt: *const ::std::ffi::c_char, args: va_list) -> i32;
    pub fn bdi_set_owner(bdi: *mut backing_dev_info, owner: *mut device);
    pub fn bdi_unregister(bdi: *mut backing_dev_info);
    pub fn bdi_alloc(node_id: i32) -> *mut backing_dev_info;
    pub fn wb_start_background_writeback(wb: *mut bdi_writeback);
    pub fn wb_workfn(work: *mut work_struct);
    pub fn wb_wait_for_completion(done: *mut wb_completion);
    pub static mut bdi_lock: spinlock_t;
    pub static mut bdi_list: list_head;
    pub static mut bdi_wq: *mut workqueue_struct;
    pub fn bdi_get_min_bytes(bdi: *mut backing_dev_info) -> u64;
    pub fn bdi_get_max_bytes(bdi: *mut backing_dev_info) -> u64;
    pub fn bdi_set_min_ratio(bdi: *mut backing_dev_info, min_ratio: u32) -> i32;
    pub fn bdi_set_max_ratio(bdi: *mut backing_dev_info, max_ratio: u32) -> i32;
    pub fn bdi_set_min_ratio_no_scale(bdi: *mut backing_dev_info, min_ratio: u32) -> i32;
    pub fn bdi_set_max_ratio_no_scale(bdi: *mut backing_dev_info, max_ratio: u32) -> i32;
    pub fn bdi_set_min_bytes(bdi: *mut backing_dev_info, min_bytes: u64) -> i32;
    pub fn bdi_set_max_bytes(bdi: *mut backing_dev_info, max_bytes: u64) -> i32;
    pub fn bdi_set_strict_limit(bdi: *mut backing_dev_info, strict_limit: u32) -> i32;
    pub static mut noop_backing_dev_info: backing_dev_info;
    pub fn bdi_init(bdi: *mut backing_dev_info) -> i32;
    pub fn inode_to_bdi(inode: *mut inode) -> *mut backing_dev_info;
    pub fn bdi_dev_name(bdi: *mut backing_dev_info) -> *const ::std::ffi::c_char;
}

pub unsafe fn wb_has_dirty_io(wb: *mut bdi_writeback) -> bool {
    test_bit(WB_has_dirty_io, &mut (*wb).state) != 0
}

pub unsafe fn bdi_has_dirty_io(bdi: *mut backing_dev_info) -> bool {
    atomic_long_read(&(*bdi).tot_write_bandwidth) != 0
}

pub unsafe fn wb_stat_mod(wb: *mut bdi_writeback, item: wb_stat_item, amount: i64) {
    percpu_counter_add_batch(&mut (*wb).stat[item as usize], amount, WB_STAT_BATCH);
}

pub unsafe fn wb_stat(wb: *mut bdi_writeback, item: wb_stat_item) -> i64 {
    percpu_counter_read_positive(&(*wb).stat[item as usize])
}

pub unsafe fn wb_stat_sum(wb: *mut bdi_writeback, item: wb_stat_item) -> i64 {
    percpu_counter_sum_positive(&(*wb).stat[item as usize])
}

pub unsafe fn wb_stat_error() -> usize {
    #[cfg(CONFIG_SMP)]
    { nr_cpu_ids * WB_STAT_BATCH }
    #[cfg(not(CONFIG_SMP))]
    { 1 }
}

pub unsafe fn writeback_in_progress(wb: *mut bdi_writeback) -> bool {
    test_bit(WB_writeback_running, &mut (*wb).state) != 0
}

pub unsafe fn mapping_can_writeback(mapping: *mut address_space) -> bool {
    ((*inode_to_bdi((*mapping).host)).capabilities & BDI_CAP_WRITEBACK) != 0
}

pub unsafe fn bdi_wb_dirty_exceeded(bdi: *mut backing_dev_info) -> i32 {
    (*bdi).wb.dirty_exceeded
}

pub unsafe fn bdi_wb_stat_mod(inode: *mut inode, item: wb_stat_item, amount: i64) {
    wb_stat_mod(&mut (*inode_to_bdi(inode)).wb, item, amount);
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
extern "C" {
    pub fn wb_get_lookup(bdi: *mut backing_dev_info, memcg_css: *mut cgroup_subsys_state) -> *mut bdi_writeback;
    pub fn wb_get_create(bdi: *mut backing_dev_info, memcg_css: *mut cgroup_subsys_state, gfp: gfp_t) -> *mut bdi_writeback;
    pub fn wb_memcg_offline(memcg: *mut mem_cgroup);
    pub fn wb_blkcg_offline(css: *mut cgroup_subsys_state);
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn inode_cgwb_enabled(inode: *mut inode) -> bool {
    cgroup_subsys_on_dfl(memory_cgrp_subsys) &&
        cgroup_subsys_on_dfl(io_cgrp_subsys) &&
        ((*inode_to_bdi(inode)).capabilities & BDI_CAP_WRITEBACK) != 0 &&
        ((*(*inode).i_sb).s_iflags & SB_I_CGROUPWB) != 0
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn wb_find_current(bdi: *mut backing_dev_info) -> *mut bdi_writeback {
    let memcg_css = task_css(current, memory_cgrp_id);
    if (*memcg_css).parent.is_null() { return &mut (*bdi).wb; }
    let wb = radix_tree_lookup(&mut (*bdi).cgwb_tree, (*memcg_css).id) as *mut bdi_writeback;
    if !wb.is_null() && (*wb).blkcg_css == task_css(current, io_cgrp_id) { wb } else { core::ptr::null_mut() }
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn wb_get_create_current(bdi: *mut backing_dev_info, gfp: gfp_t) -> *mut bdi_writeback {
    rcu_read_lock();
    let mut wb = wb_find_current(bdi);
    if !wb.is_null() && !wb_tryget(wb) { wb = core::ptr::null_mut(); }
    rcu_read_unlock();
    if wb.is_null() {
        let memcg_css = task_get_css(current, memory_cgrp_id);
        wb = wb_get_create(bdi, memcg_css, gfp);
        css_put(memcg_css);
    }
    wb
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn inode_to_wb(inode: *const inode) -> *mut bdi_writeback { (*inode).i_wb }

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn inode_to_wb_wbc(inode: *mut inode, wbc: *mut writeback_control) -> *mut bdi_writeback {
    if !(*wbc).wb.is_null() { (*wbc).wb } else { &mut (*inode_to_bdi(inode)).wb }
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn unlocked_inode_to_wb_begin(inode: *mut inode, cookie: *mut wb_lock_cookie) -> *mut bdi_writeback {
    rcu_read_lock();
    (*cookie).locked = inode_state_read_once(inode) & I_WB_SWITCH;
    smp_rmb();
    if (*cookie).locked != 0 { xa_lock_irqsave(&mut (*(*inode).i_mapping).i_pages, &mut (*cookie).flags); }
    (*inode).i_wb
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe fn unlocked_inode_to_wb_end(inode: *mut inode, cookie: *mut wb_lock_cookie) {
    if (*cookie).locked != 0 { xa_unlock_irqrestore(&mut (*(*inode).i_mapping).i_pages, (*cookie).flags); }
    rcu_read_unlock();
}

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn inode_cgwb_enabled(_inode: *mut inode) -> bool { false }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn wb_find_current(bdi: *mut backing_dev_info) -> *mut bdi_writeback { &mut (*bdi).wb }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn wb_get_create_current(bdi: *mut backing_dev_info, _gfp: gfp_t) -> *mut bdi_writeback { &mut (*bdi).wb }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn inode_to_wb(inode: *mut inode) -> *mut bdi_writeback { &mut (*inode_to_bdi(inode)).wb }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn inode_to_wb_wbc(inode: *mut inode, _wbc: *mut writeback_control) -> *mut bdi_writeback { inode_to_wb(inode) }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn unlocked_inode_to_wb_begin(inode: *mut inode, _cookie: *mut wb_lock_cookie) -> *mut bdi_writeback { inode_to_wb(inode) }

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn unlocked_inode_to_wb_end(_inode: *mut inode, _cookie: *mut wb_lock_cookie) {}

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn wb_memcg_offline(_memcg: *mut mem_cgroup) {}

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
pub unsafe fn wb_blkcg_offline(_css: *mut cgroup_subsys_state) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
