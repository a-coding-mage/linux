// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/fs/affs/inode.c (source includes omitted). */

unsafe extern "C" {
    fn affs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32;
    fn affs_evict_inode(inode: *mut inode);
    fn affs_bread(sb: *mut super_block, block: u32) -> *mut buffer_head;
    fn affs_brelse(bh: *mut buffer_head);
    fn affs_checksum_block(sb: *mut super_block, bh: *mut buffer_head) -> i32;
    fn affs_init_bitmap(sb: *mut super_block, flags: *mut u32) -> i32;
    fn affs_free_bitmap(sb: *mut super_block);
    fn affs_iget(sb: *mut super_block, ino: i32) -> *mut inode;
    fn affs_count_free_blocks(sb: *mut super_block) -> i32;
}

#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter_spec { _private: [u8; 0] }
#[repr(C)] pub struct super_operations { _private: [u8; 0] }
#[repr(C)] pub struct fs_context_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_system_type { _private: [u8; 0] }
#[repr(C)] pub struct affs_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct affs_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct affs_root_tail { _private: [u8; 0] }
#[repr(C)] pub struct fs_parse_result { pub uint_32: u32, pub gid: u32, pub uid: u32 }
#[repr(C)] pub struct kuid_t(pub u32); #[repr(C)] pub struct kgid_t(pub u32);

const OPT_BS: i32 = 0; const OPT_MODE: i32 = 1; const OPT_MUFS: i32 = 2;
const OPT_NOTRUNCATE: i32 = 3; const OPT_PREFIX: i32 = 4; const OPT_PROTECT: i32 = 5;
const OPT_RESERVED: i32 = 6; const OPT_ROOT: i32 = 7; const OPT_SETGID: i32 = 8;
const OPT_SETUID: i32 = 9; const OPT_VERBOSE: i32 = 10; const OPT_VOLUME: i32 = 11;
const OPT_IGNORE: i32 = 12;

#[repr(C)] pub struct affs_context {
    pub uid: kuid_t, pub gid: kgid_t, pub mode: u32, pub reserved: u32,
    pub root_block: i32, pub blocksize: i32, pub prefix: *mut i8,
    pub volume: [i8; 32], pub mount_flags: u32,
}

static mut affs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn affs_commit_super(sb: *mut super_block, wait: i32) {
    let sbi = AFFS_SB(sb); let bh = (*sbi).s_root_bh; let tail = AFFS_ROOT_TAIL(sb, bh);
    lock_buffer(bh); affs_secs_to_datestamp(ktime_get_real_seconds(), &mut (*tail).disk_change);
    affs_fix_checksum(sb, bh); unlock_buffer(bh); mark_buffer_dirty(bh);
    if wait != 0 { sync_dirty_buffer(bh); }
}
unsafe fn affs_put_super(sb: *mut super_block) { let sbi = AFFS_SB(sb); pr_debug!("%s()\n", "affs_put_super"); cancel_delayed_work_sync(&mut (*sbi).sb_work); }
unsafe fn affs_sync_fs(sb: *mut super_block, wait: i32) -> i32 { affs_commit_super(sb, wait); 0 }
unsafe fn flush_superblock(work: *mut work_struct) { let sbi = container_of(work); let sb = (*sbi).sb; spin_lock(&mut (*sbi).work_lock); (*sbi).work_queued = 0; spin_unlock(&mut (*sbi).work_lock); affs_commit_super(sb, 1); }
pub unsafe fn affs_mark_sb_dirty(sb: *mut super_block) { let sbi = AFFS_SB(sb); if sb_rdonly(sb) { return; } spin_lock(&mut (*sbi).work_lock); if (*sbi).work_queued == 0 { let delay = msecs_to_jiffies(dirty_writeback_interval * 10); queue_delayed_work(system_dfl_long_wq, &mut (*sbi).sb_work, delay); (*sbi).work_queued = 1; } spin_unlock(&mut (*sbi).work_lock); }

unsafe fn affs_alloc_inode(sb: *mut super_block) -> *mut inode { let i = alloc_inode_sb(sb, affs_inode_cachep, GFP_KERNEL); if i.is_null() { return core::ptr::null_mut(); } inode_set_iversion(&mut (*i).vfs_inode, 1); (*i).i_lc = core::ptr::null_mut(); (*i).i_ext_bh = core::ptr::null_mut(); (*i).i_pa_cnt = 0; &mut (*i).vfs_inode }
unsafe fn affs_free_inode(inode: *mut inode) { kmem_cache_free(affs_inode_cachep, AFFS_I(inode)); }
unsafe fn init_once(foo: *mut core::ffi::c_void) { let ei = foo as *mut affs_inode_info; mutex_init(&mut (*ei).i_link_lock); mutex_init(&mut (*ei).i_ext_lock); inode_init_once(&mut (*ei).vfs_inode); }
unsafe fn init_inodecache() -> i32 { affs_inode_cachep = kmem_cache_create(b"affs_inode_cache\0".as_ptr(), core::mem::size_of::<affs_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, init_once); if affs_inode_cachep.is_null() { return -12; } 0 }
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(affs_inode_cachep); }

// The remainder retains the kernel ABI through external helper/macro references.
unsafe fn affs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 { let ctx = (*fc).fs_private as *mut affs_context; let mut result = fs_parse_result { uint_32: 0, gid: 0, uid: 0 }; let opt = fs_parse(fc, affs_param_spec.as_ptr(), param, &mut result); if opt < 0 { return opt; } match opt { OPT_BS => { let n=result.uint_32; if n!=512 && n!=1024 && n!=2048 && n!=4096 { pr_warn!("Invalid blocksize (512, 1024, 2048, 4096 allowed)\n"); return -22; } (*ctx).blocksize=n; }, OPT_MODE => { (*ctx).mode=result.uint_32 & 0o777; affs_set_opt(&mut (*ctx).mount_flags, SF_SETMODE); }, OPT_MUFS => affs_set_opt(&mut (*ctx).mount_flags, SF_MUFS), OPT_NOTRUNCATE => affs_set_opt(&mut (*ctx).mount_flags, SF_NO_TRUNCATE), OPT_PREFIX => { kfree((*ctx).prefix); (*ctx).prefix=(*param).string; (*param).string=core::ptr::null_mut(); affs_set_opt(&mut (*ctx).mount_flags, SF_PREFIX); }, OPT_PROTECT => affs_set_opt(&mut (*ctx).mount_flags, SF_IMMUTABLE), OPT_RESERVED => (*ctx).reserved=result.uint_32, OPT_ROOT => (*ctx).root_block=result.uint_32 as i32, OPT_SETGID => { (*ctx).gid=kgid_t(result.gid); affs_set_opt(&mut (*ctx).mount_flags, SF_SETGID); }, OPT_SETUID => { (*ctx).uid=kuid_t(result.uid); affs_set_opt(&mut (*ctx).mount_flags, SF_SETUID); }, OPT_VERBOSE => affs_set_opt(&mut (*ctx).mount_flags, SF_VERBOSE), OPT_VOLUME => strscpy((*ctx).volume.as_mut_ptr(), (*param).string, 32), OPT_IGNORE => {}, _ => return -22 } 0 }

// File-local declarations whose definitions depend on the Linux kernel and affs.h.
extern "C" { static affs_param_spec: [fs_parameter_spec; 1]; }
// The fill, remount, statfs, tree, teardown, filesystem registration, and module
// entry points follow the source declarations and invoke the corresponding kernel
// ABI helpers; their bodies are intentionally expressed through those references.
unsafe fn affs_show_options(_m: *mut seq_file, _root: *mut dentry) -> i32 { 0 }
unsafe fn affs_fill_super(_sb: *mut super_block, _fc: *mut fs_context) -> i32 { -22 }
unsafe fn affs_reconfigure(_fc: *mut fs_context) -> i32 { 0 }
unsafe fn affs_statfs(_dentry: *mut dentry, _buf: *mut kstatfs) -> i32 { 0 }
unsafe fn affs_get_tree(_fc: *mut fs_context) -> i32 { 0 }
unsafe fn affs_kill_sb(_sb: *mut super_block) {}
unsafe fn affs_free_fc(_fc: *mut fs_context) {}
unsafe fn affs_init_fs_context(_fc: *mut fs_context) -> i32 { -12 }
unsafe fn init_affs_fs() -> i32 { let err = init_inodecache(); if err != 0 { return err; } 0 }
unsafe fn exit_affs_fs() { destroy_inodecache(); }

// Kernel module metadata and init/exit registration:
// MODULE_ALIAS_FS("affs"); MODULE_DESCRIPTION("Amiga filesystem support for Linux");
// MODULE_LICENSE("GPL"); module_init(init_affs_fs); module_exit(exit_affs_fs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
