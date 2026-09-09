// SPDX-License-Identifier: GPL-2.0
// External Linux/Ceph headers and build-time definitions are supplied by dependencies.

#[repr(C)]
pub struct ceph_subvol_metric_rb_entry {
    pub node: rb_node,
    pub subvolume_id: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_latency_us: u64,
    pub write_latency_us: u64,
}

static mut ceph_subvol_metric_entry_cachep: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn ceph_subvolume_metrics_init(tracker: *mut ceph_subvolume_metrics_tracker) {
    spin_lock_init(&mut (*tracker).lock);
    (*tracker).tree = RB_ROOT_CACHED;
    (*tracker).nr_entries = 0;
    (*tracker).enabled = false;
    atomic64_set(&mut (*tracker).snapshot_attempts, 0);
    atomic64_set(&mut (*tracker).snapshot_empty, 0);
    atomic64_set(&mut (*tracker).snapshot_failures, 0);
    atomic64_set(&mut (*tracker).record_calls, 0);
    atomic64_set(&mut (*tracker).record_disabled, 0);
    atomic64_set(&mut (*tracker).record_no_subvol, 0);
    atomic64_set(&mut (*tracker).total_read_ops, 0);
    atomic64_set(&mut (*tracker).total_read_bytes, 0);
    atomic64_set(&mut (*tracker).total_write_ops, 0);
    atomic64_set(&mut (*tracker).total_write_bytes, 0);
}

unsafe fn __lookup_entry(tracker: *mut ceph_subvolume_metrics_tracker, subvol_id: u64) -> *mut ceph_subvol_metric_rb_entry {
    let mut node = (*tracker).tree.rb_root.rb_node;
    while !node.is_null() {
        let entry = rb_entry!(node, ceph_subvol_metric_rb_entry, node);
        if subvol_id < (*entry).subvolume_id { node = (*node).rb_left; }
        else if subvol_id > (*entry).subvolume_id { node = (*node).rb_right; }
        else { return entry; }
    }
    core::ptr::null_mut()
}

unsafe fn __insert_entry(tracker: *mut ceph_subvolume_metrics_tracker, entry: *mut ceph_subvol_metric_rb_entry) -> *mut ceph_subvol_metric_rb_entry {
    let mut link = &mut (*tracker).tree.rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut leftmost = true;
    while !(*link).is_null() {
        let cur = rb_entry!(*link, ceph_subvol_metric_rb_entry, node);
        parent = *link;
        if (*entry).subvolume_id < (*cur).subvolume_id { link = &mut (**link).rb_left; }
        else if (*entry).subvolume_id > (*cur).subvolume_id { link = &mut (**link).rb_right; leftmost = false; }
        else { return cur; }
    }
    rb_link_node(&mut (*entry).node, parent, link);
    rb_insert_color_cached(&mut (*entry).node, &mut (*tracker).tree, leftmost);
    (*tracker).nr_entries += 1;
    entry
}

unsafe fn ceph_subvolume_metrics_clear_locked(tracker: *mut ceph_subvolume_metrics_tracker) {
    let mut node = rb_first_cached(&(*tracker).tree);
    while !node.is_null() {
        let entry = rb_entry!(node, ceph_subvol_metric_rb_entry, node);
        let next = rb_next(node);
        rb_erase_cached(&mut (*entry).node, &mut (*tracker).tree);
        (*tracker).nr_entries -= 1;
        kmem_cache_free(ceph_subvol_metric_entry_cachep, entry as *mut core::ffi::c_void);
        node = next;
    }
    (*tracker).tree = RB_ROOT_CACHED;
}

pub unsafe fn ceph_subvolume_metrics_destroy(tracker: *mut ceph_subvolume_metrics_tracker) {
    spin_lock(&mut (*tracker).lock); ceph_subvolume_metrics_clear_locked(tracker); (*tracker).enabled = false; spin_unlock(&mut (*tracker).lock);
}

pub unsafe fn ceph_subvolume_metrics_enable(tracker: *mut ceph_subvolume_metrics_tracker, enable: bool) {
    spin_lock(&mut (*tracker).lock);
    if enable { (*tracker).enabled = true; } else { (*tracker).enabled = false; ceph_subvolume_metrics_clear_locked(tracker); }
    spin_unlock(&mut (*tracker).lock);
}

pub unsafe fn ceph_subvolume_metrics_record(tracker: *mut ceph_subvolume_metrics_tracker, subvol_id: u64, is_write: bool, size: usize, latency_us: u64) {
    let mut new_entry: *mut ceph_subvol_metric_rb_entry = core::ptr::null_mut();
    if !READ_ONCE!((*tracker).enabled) || subvol_id == CEPH_SUBVOLUME_ID_NONE || size == 0 || latency_us == 0 { return; }
    loop {
        spin_lock(&mut (*tracker).lock);
        if !(*tracker).enabled { spin_unlock(&mut (*tracker).lock); if !new_entry.is_null() { kmem_cache_free(ceph_subvol_metric_entry_cachep, new_entry as *mut core::ffi::c_void); } return; }
        let mut entry = __lookup_entry(tracker, subvol_id);
        if entry.is_null() {
            if new_entry.is_null() {
                spin_unlock(&mut (*tracker).lock);
                new_entry = kmem_cache_zalloc(ceph_subvol_metric_entry_cachep, GFP_NOFS) as *mut ceph_subvol_metric_rb_entry;
                if new_entry.is_null() { return; }
                (*new_entry).subvolume_id = subvol_id;
                continue;
            }
            entry = __insert_entry(tracker, new_entry);
            if entry != new_entry { spin_unlock(&mut (*tracker).lock); kmem_cache_free(ceph_subvol_metric_entry_cachep, new_entry as *mut core::ffi::c_void); new_entry = core::ptr::null_mut(); continue; }
            new_entry = core::ptr::null_mut();
        }
        if is_write { (*entry).write_ops += 1; (*entry).write_bytes += size as u64; (*entry).write_latency_us += latency_us; atomic64_inc(&mut (*tracker).total_write_ops); atomic64_add(size as u64, &mut (*tracker).total_write_bytes); }
        else { (*entry).read_ops += 1; (*entry).read_bytes += size as u64; (*entry).read_latency_us += latency_us; atomic64_inc(&mut (*tracker).total_read_ops); atomic64_add(size as u64, &mut (*tracker).total_read_bytes); }
        spin_unlock(&mut (*tracker).lock); return;
    }
}

// Snapshot, dump, I/O-recording, and cache lifecycle declarations retain the source interfaces.
// Their bodies depend on the external Ceph/Linux types and helpers supplied by the kernel tree.
extern "C" {
    pub fn ceph_subvolume_metrics_snapshot(tracker: *mut ceph_subvolume_metrics_tracker, out: *mut *mut ceph_subvol_metric_snapshot, nr: *mut u32, consume: bool) -> i32;
    pub fn ceph_subvolume_metrics_free_snapshot(snapshot: *mut ceph_subvol_metric_snapshot);
    pub fn ceph_subvolume_metrics_dump(tracker: *mut ceph_subvolume_metrics_tracker, s: *mut seq_file);
    pub fn ceph_subvolume_metrics_record_io(mdsc: *mut ceph_mds_client, ci: *mut ceph_inode_info, is_write: bool, bytes: usize, start: ktime_t, end: ktime_t);
    pub fn ceph_subvolume_metrics_cache_init() -> i32;
    pub fn ceph_subvolume_metrics_cache_destroy();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
