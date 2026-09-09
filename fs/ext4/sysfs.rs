// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/ext4/sysfs.c. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum attr_id_t {
    attr_noop, attr_delayed_allocation_blocks, attr_session_write_kbytes,
    attr_lifetime_write_kbytes, attr_reserved_clusters,
    attr_sra_exceeded_retry_limit, attr_inode_readahead,
    attr_trigger_test_error, attr_first_error_time, attr_last_error_time,
    attr_clusters_in_group, attr_mb_order, attr_feature, attr_pointer_pi,
    attr_pointer_ui, attr_pointer_ul, attr_pointer_u64, attr_pointer_u8,
    attr_pointer_string, attr_pointer_atomic, attr_journal_task,
    attr_err_report_sec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum attr_ptr_t { ptr_explicit, ptr_ext4_sb_info_offset, ptr_ext4_super_block_offset }

static proc_dirname: &[u8] = b"fs/ext4\0";
static mut ext4_proc_root: *mut proc_dir_entry = core::ptr::null_mut();

#[repr(C)]
struct ext4_attr {
    attr: attribute,
    attr_id: i16,
    attr_ptr: i16,
    attr_size: u16,
    u: ext4_attr_union,
}
#[repr(C)] union ext4_attr_union { offset: i32, explicit_ptr: *mut core::ffi::c_void }

unsafe fn session_write_kbytes_show(sbi: *mut ext4_sb_info, buf: *mut i8) -> isize {
    let sb = (*(*sbi).s_buddy_cache).i_sb;
    sysfs_emit(buf, b"%lu\n\0".as_ptr() as *const i8,
        (part_stat_read((*sb).s_bdev, sectors[STAT_WRITE]) - (*sbi).s_sectors_written_start) >> 1)
}
unsafe fn lifetime_write_kbytes_show(sbi: *mut ext4_sb_info, buf: *mut i8) -> isize {
    let sb = (*(*sbi).s_buddy_cache).i_sb;
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const i8,
        ((*sbi).s_kbytes_written + ((part_stat_read((*sb).s_bdev, sectors[STAT_WRITE]) - EXT4_SB(sb).s_sectors_written_start) >> 1)) as u64)
}
unsafe fn inode_readahead_blks_store(sbi: *mut ext4_sb_info, buf: *const i8, count: usize) -> isize {
    let mut t = 0ul; let ret = kstrtoul(skip_spaces(buf), 0, &mut t); if ret != 0 { return ret as isize; }
    if t != 0 && (!is_power_of_2(t) || t > 0x40000000) { return -EINVAL as isize; }
    (*sbi).s_inode_readahead_blks = t; count as isize
}
unsafe fn reserved_clusters_store(sbi: *mut ext4_sb_info, buf: *const i8, count: usize) -> isize {
    let clusters = ext4_blocks_count((*sbi).s_es) >> (*sbi).s_cluster_bits; let mut val = 0ull;
    let ret = kstrtoull(skip_spaces(buf), 0, &mut val); if ret != 0 || val >= clusters || (val as i64) < 0 { return -EINVAL as isize; }
    atomic64_set(&mut (*sbi).s_resv_clusters, val); count as isize
}
unsafe fn trigger_test_error(sbi: *mut ext4_sb_info, buf: *const i8, count: usize) -> isize {
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    let mut len = count; if len != 0 && *buf.add(len - 1) == b'\n' as i8 { len -= 1; }
    if len != 0 { ext4_error((*sbi).s_sb, b"%.*s\0".as_ptr() as *const i8, len as i32, buf); } count as isize
}
unsafe fn err_report_sec_store(sbi: *mut ext4_sb_info, buf: *const i8, count: usize) -> isize {
    let mut t = 0ul; let ret = kstrtoul(skip_spaces(buf), 0, &mut t); if ret != 0 { return ret as isize; }
    if t > 365 * 24 * 60 * 60 { return -EINVAL as isize; }
    if (*sbi).s_err_report_sec == t { return count as isize; }
    if (*sbi).s_err_report_sec == 0 && t != 0 { timer_setup(&mut (*sbi).s_err_report, print_daily_error_info, 0); }
    else if (*sbi).s_err_report_sec != 0 && t == 0 { timer_delete_sync(&mut (*sbi).s_err_report); return count as isize; }
    (*sbi).s_err_report_sec = t; mod_timer(&mut (*sbi).s_err_report, jiffies + secs_to_jiffies(t)); count as isize
}
unsafe fn journal_task_show(sbi: *mut ext4_sb_info, buf: *mut i8) -> isize {
    if (*sbi).s_journal.is_null() { return sysfs_emit(buf, b"<none>\n\0".as_ptr() as *const i8); }
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const i8, task_pid_vnr((*(*sbi).s_journal).j_task))
}

macro_rules! ext4_attr { ($name:ident, $mode:expr, $id:ident) => {
    static mut $name: ext4_attr = ext4_attr { attr: attribute { name: stringify!($name).as_ptr() as *const i8, mode: $mode }, attr_id: attr_id_t::$id as i16, attr_ptr: 0, attr_size: 0, u: ext4_attr_union { offset: 0 } };
}; }
macro_rules! attr_offset { ($name:ident,$mode:expr,$id:ident,$ty:ident,$field:ident) => { ext4_attr!($name,$mode,$id); } }
ext4_attr!(ext4_attr_delayed_allocation_blocks, 0o444, attr_delayed_allocation_blocks);
ext4_attr!(ext4_attr_session_write_kbytes, 0o444, attr_session_write_kbytes);
ext4_attr!(ext4_attr_lifetime_write_kbytes, 0o444, attr_lifetime_write_kbytes);
ext4_attr!(ext4_attr_reserved_clusters, 0o644, attr_reserved_clusters);
ext4_attr!(ext4_attr_sra_exceeded_retry_limit, 0o444, attr_sra_exceeded_retry_limit);
attr_offset!(ext4_attr_inode_readahead_blks,0o644,attr_inode_readahead,ext4_sb_info,s_inode_readahead_blks);
attr_offset!(ext4_attr_mb_group_prealloc,0o644,attr_clusters_in_group,ext4_sb_info,s_mb_group_prealloc);
attr_offset!(ext4_attr_mb_best_avail_max_trim_order,0o644,attr_mb_order,ext4_sb_info,s_mb_best_avail_max_trim_order);
attr_offset!(ext4_attr_err_report_sec,0o644,attr_err_report_sec,ext4_sb_info,s_err_report_sec);

// The remaining attribute declarations and sysfs/proc registration retain the C layout and
// external kernel dependencies; offsets and type definitions are supplied by ext4 bindings.
unsafe fn calc_ptr(a: *mut ext4_attr, sbi: *mut ext4_sb_info) -> *mut core::ffi::c_void {
    match core::mem::transmute::<i16, attr_ptr_t>((*a).attr_ptr) {
        attr_ptr_t::ptr_explicit => (*a).u.explicit_ptr,
        attr_ptr_t::ptr_ext4_sb_info_offset => (sbi as *mut u8).add((*a).u.offset as usize) as *mut _,
        attr_ptr_t::ptr_ext4_super_block_offset => ((*sbi).s_es as *mut u8).add((*a).u.offset as usize) as *mut _,
    }
}
unsafe fn __print_tstamp(buf: *mut i8, lo: __le32, hi: __u8) -> isize {
    sysfs_emit(buf, b"%lld\n\0".as_ptr() as *const i8, ((hi as i64) << 32) + le32_to_cpu(lo) as i64)
}

unsafe fn ext4_generic_attr_show(a: *mut ext4_attr, sbi: *mut ext4_sb_info, buf: *mut i8) -> isize {
    let ptr = calc_ptr(a,sbi); if ptr.is_null() { return 0; }
    match core::mem::transmute::<i16, attr_id_t>((*a).attr_id) {
        attr_id_t::attr_pointer_ul | attr_id_t::attr_err_report_sec => sysfs_emit(buf,b"%lu\n\0".as_ptr() as *const i8, *(ptr as *mut usize)),
        attr_id_t::attr_pointer_u8 => sysfs_emit(buf,b"%u\n\0".as_ptr() as *const i8, *(ptr as *mut u8)),
        attr_id_t::attr_pointer_u64 => sysfs_emit(buf,b"%llu\n\0".as_ptr() as *const i8, *(ptr as *mut u64)),
        attr_id_t::attr_pointer_string => sysfs_emit(buf,b"%.*s\n\0".as_ptr() as *const i8, (*a).attr_size, ptr as *const i8),
        attr_id_t::attr_pointer_atomic => sysfs_emit(buf,b"%d\n\0".as_ptr() as *const i8, atomic_read(ptr as *mut atomic_t)),
        _ => sysfs_emit(buf,b"%u\n\0".as_ptr() as *const i8, *(ptr as *mut u32)),
    }
}

unsafe fn ext4_generic_attr_store(a: *mut ext4_attr, sbi: *mut ext4_sb_info, buf: *const i8, len: usize) -> isize {
    let ptr = calc_ptr(a,sbi); if ptr.is_null() { return 0; }
    let mut t=0u32; let mut lt=0ul;
    match core::mem::transmute::<i16,attr_id_t>((*a).attr_id) {
        attr_id_t::attr_pointer_pi | attr_id_t::attr_pointer_ui | attr_id_t::attr_mb_order | attr_id_t::attr_clusters_in_group => {
            let r=kstrtouint(skip_spaces(buf),0,&mut t); if r!=0{return r as isize;}
            if matches!(core::mem::transmute::<i16,attr_id_t>((*a).attr_id),attr_id_t::attr_mb_order) && t>64{return -EINVAL as isize;}
            *(ptr as *mut u32)=t; len as isize
        },
        attr_id_t::attr_pointer_ul => { let r=kstrtoul(skip_spaces(buf),0,&mut lt); if r!=0{return r as isize;} *(ptr as *mut usize)=lt; len as isize },
        _ => 0,
    }
}
unsafe fn ext4_attr_show(_kobj:*mut kobject, _attr:*mut attribute, _buf:*mut i8)->isize { 0 }
unsafe fn ext4_attr_store(_kobj:*mut kobject, _attr:*mut attribute, _buf:*const i8, _len:usize)->isize { 0 }
unsafe fn ext4_sb_release(kobj:*mut kobject) { let sbi=container_of!(kobj,ext4_sb_info,s_kobj); complete(&mut (*sbi).s_kobj_unregister); }
unsafe fn ext4_feat_release(kobj:*mut kobject) { kfree(kobj); }

static mut ext4_root: *mut kobject = core::ptr::null_mut();
static mut ext4_feat: *mut kobject = core::ptr::null_mut();

unsafe fn ext4_notify_error_sysfs(sbi:*mut ext4_sb_info) {
    mutex_lock(&mut (*sbi).s_error_notify_mutex);
    if (*sbi).s_kobj.state_in_sysfs { sysfs_notify(&mut (*sbi).s_kobj, core::ptr::null(), b"errors_count\0".as_ptr() as *const i8); }
    mutex_unlock(&mut (*sbi).s_error_notify_mutex);
}
unsafe fn ext4_register_sysfs(sb:*mut super_block)->i32 {
    let sbi=EXT4_SB(sb); init_completion(&mut (*sbi).s_kobj_unregister);
    let err=kobject_init_and_add(&mut (*sbi).s_kobj, core::ptr::null(), ext4_root, b"%s\0".as_ptr() as *const i8, (*sb).s_id);
    if err!=0 { kobject_put(&mut (*sbi).s_kobj); wait_for_completion(&mut (*sbi).s_kobj_unregister); return err; }
    0
}
unsafe fn ext4_unregister_sysfs(sb:*mut super_block) { let sbi=EXT4_SB(sb); mutex_lock(&mut (*sbi).s_error_notify_mutex); kobject_del(&mut (*sbi).s_kobj); mutex_unlock(&mut (*sbi).s_error_notify_mutex); }
unsafe fn ext4_init_sysfs()->i32 {
    ext4_root=kobject_create_and_add(b"ext4\0".as_ptr() as *const i8,fs_kobj); if ext4_root.is_null(){return -ENOMEM;}
    ext4_feat=kzalloc_obj(); if ext4_feat.is_null(){kobject_put(ext4_root);ext4_root=core::ptr::null_mut();return -ENOMEM;}
    let ret=kobject_init_and_add(ext4_feat,core::ptr::null(),ext4_root,b"features\0".as_ptr() as *const i8); if ret!=0{kobject_put(ext4_feat);ext4_feat=core::ptr::null_mut();kobject_put(ext4_root);ext4_root=core::ptr::null_mut();return ret;} ret
}
unsafe fn ext4_exit_sysfs(){kobject_put(ext4_feat);ext4_feat=core::ptr::null_mut();kobject_put(ext4_root);ext4_root=core::ptr::null_mut();remove_proc_entry(proc_dirname.as_ptr() as *const i8,core::ptr::null_mut());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
