// SPDX-License-Identifier: GPL-2.0-only
/* Translation of linux/fs/hpfs/super.c. Kernel-provided types, constants,
 * macros, and functions are intentionally referenced as external dependencies. */

unsafe fn mark_dirty(s: *mut super_block, remount: i32) {
    if (*hpfs_sb(s)).sb_chkdsk != 0 && (remount != 0 || !sb_rdonly(s)) {
        let mut bh: *mut buffer_head = core::ptr::null_mut();
        let sb = hpfs_map_sector(s, 17, &mut bh, 0);
        if !sb.is_null() { (*sb).dirty = 1; (*sb).old_wrote = 0; mark_buffer_dirty(bh); sync_dirty_buffer(bh); brelse(bh); }
    }
}

unsafe fn unmark_dirty(s: *mut super_block) {
    if sb_rdonly(s) { return; }
    sync_blockdev((*s).s_bdev);
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let sb = hpfs_map_sector(s, 17, &mut bh, 0);
    if !sb.is_null() {
        (*sb).dirty = ((*hpfs_sb(s)).sb_chkdsk > 1 - (*hpfs_sb(s)).sb_was_error) as _;
        (*sb).old_wrote = ((*hpfs_sb(s)).sb_chkdsk >= 2 && (*hpfs_sb(s)).sb_was_error == 0) as _;
        mark_buffer_dirty(bh); sync_dirty_buffer(bh); brelse(bh);
    }
}

pub unsafe fn hpfs_error(s: *mut super_block, fmt: *const i8, mut args: ...) {
    let mut vaf: va_format = core::mem::zeroed();
    vaf.fmt = fmt; vaf.va = &mut args;
    pr_err!("filesystem error: %pV", &vaf);
    if (*hpfs_sb(s)).sb_was_error == 0 {
        if (*hpfs_sb(s)).sb_err == 2 { pr_cont!("; crashing the system because you wanted it\n"); mark_dirty(s, 0); panic!("HPFS panic"); }
        else if (*hpfs_sb(s)).sb_err == 1 { if sb_rdonly(s) { pr_cont!("; already mounted read-only\n"); } else { pr_cont!("; remounting read-only\n"); mark_dirty(s, 0); (*s).s_flags |= SB_RDONLY; } }
        else if sb_rdonly(s) { pr_cont!("; going on - but anything won't be destroyed because it's read-only\n"); }
        else { pr_cont!("; corrupted filesystem mounted read/write - your computer will explode within 20 seconds ... but you wanted it so!\n"); }
    } else { pr_cont!("\n"); }
    (*hpfs_sb(s)).sb_was_error = 1;
}

pub unsafe fn hpfs_stop_cycles(s: *mut super_block, key: i32, c1: *mut i32, c2: *mut i32, msg: *mut i8) -> i32 {
    if *c2 != 0 && *c1 == key { hpfs_error(s, c"cycle detected on key %08x in %s".as_ptr(), key, msg); return 1; }
    *c2 += 1; if !((*c2 - 1) & *c2 != 0) { *c1 = key; } 0
}

unsafe fn free_sbi(sbi: *mut hpfs_sb_info) { kfree((*sbi).sb_cp_table); kfree((*sbi).sb_bmp_dir); kfree(sbi as _); }
unsafe fn lazy_free_sbi(rcu: *mut rcu_head) { free_sbi(container_of!(rcu, hpfs_sb_info, rcu)); }
unsafe fn hpfs_put_super(s: *mut super_block) { hpfs_lock(s); unmark_dirty(s); hpfs_unlock(s); call_rcu(&mut (*hpfs_sb(s)).rcu, lazy_free_sbi); }

unsafe fn hpfs_count_one_bitmap(s: *mut super_block, secno: secno) -> u32 {
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let bits = hpfs_map_4sectors(s, secno, &mut qbh, 0);
    if bits.is_null() { return u32::MAX; }
    let count = bitmap_weight(bits, 2048 * BITS_PER_BYTE); hpfs_brelse4(&mut qbh); count
}
unsafe fn count_bitmaps(s: *mut super_block) -> u32 {
    let n_bands = ((*hpfs_sb(s)).sb_fs_size + 0x3fff) >> 14; let mut count = 0;
    for n in 0..COUNT_RD_AHEAD { hpfs_prefetch_bitmap(s, n); }
    for n in 0..n_bands { hpfs_prefetch_bitmap(s, n + COUNT_RD_AHEAD); let c = hpfs_count_one_bitmap(s, le32_to_cpu((*hpfs_sb(s)).sb_bmp_dir.add(n as usize))); if c != u32::MAX { count += c; } }
    count
}
pub unsafe fn hpfs_get_free_dnodes(s: *mut super_block) -> u32 { let sbi = hpfs_sb(s); if (*sbi).sb_n_free_dnodes == u32::MAX { let c = hpfs_count_one_bitmap(s, (*sbi).sb_dmap); if c == u32::MAX { return 0; } (*sbi).sb_n_free_dnodes = c; } (*sbi).sb_n_free_dnodes }

unsafe fn hpfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let s = (*dentry).d_sb; let sbi = hpfs_sb(s); let id = huge_encode_dev((*(*s).s_bdev).bd_dev); hpfs_lock(s);
    if (*sbi).sb_n_free == u32::MAX { (*sbi).sb_n_free = count_bitmaps(s); }
    (*buf).f_type = (*s).s_magic; (*buf).f_bsize = 512; (*buf).f_blocks = (*sbi).sb_fs_size; (*buf).f_bfree = (*sbi).sb_n_free; (*buf).f_bavail = (*sbi).sb_n_free; (*buf).f_files = (*sbi).sb_dirband_size / 4; (*buf).f_ffree = hpfs_get_free_dnodes(s); (*buf).f_fsid = u64_to_fsid(id); (*buf).f_namelen = 254; hpfs_unlock(s); 0
}

pub unsafe fn hpfs_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    match cmd { FITRIM => { let mut range: fstrim_range = core::mem::zeroed(); let mut n_trimmed: secno = 0; if !capable(CAP_SYS_ADMIN) { return -EPERM as _; } if copy_from_user(&mut range, arg as _, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT as _; } let r = hpfs_trim_fs((*file_inode(file)).i_sb, range.start >> 9, (range.start + range.len) >> 9, (range.minlen + 511) >> 9, &mut n_trimmed); if r != 0 { return r as _; } range.len = (n_trimmed as u64) << 9; if copy_to_user(arg as _, &range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT as _; } 0 }, _ => -ENOIOCTLCMD as _ }
}

static mut hpfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn hpfs_alloc_inode(sb: *mut super_block) -> *mut inode { let ei = alloc_inode_sb(sb, hpfs_inode_cachep, GFP_NOFS); if ei.is_null() { return core::ptr::null_mut(); } &mut (*ei).vfs_inode }
unsafe fn hpfs_free_inode(inode: *mut inode) { kmem_cache_free(hpfs_inode_cachep, hpfs_i(inode)); }
unsafe fn init_once(foo: *mut core::ffi::c_void) { inode_init_once(&mut (*(foo as *mut hpfs_inode_info)).vfs_inode); }
unsafe fn init_inodecache() -> i32 { hpfs_inode_cachep = kmem_cache_create(c"hpfs_inode_cache".as_ptr(), core::mem::size_of::<hpfs_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, init_once); if hpfs_inode_cachep.is_null() { -ENOMEM } else { 0 } }
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(hpfs_inode_cachep); }

#[repr(C)] pub struct hpfs_fc_context { pub uid: kuid_t, pub gid: kgid_t, pub umask: umode_t, pub lowercase: i32, pub eas: i32, pub chk: i32, pub errs: i32, pub chkdsk: i32, pub timeshift: i32 }
unsafe fn hpfs_help() { pr_info!("\nHPFS filesystem options:\n      help              do not mount and display this text\n      uid=xxx           set uid of files that don't have uid specified in eas\n      gid=xxx           set gid of files that don't have gid specified in eas\n      umask=xxx         set mode of files that don't have mode specified in eas\n      case=lower        lowercase all files\n      case=asis         do not lowercase files (default)\n      check=none        no fs checks - kernel may crash on corrupted filesystem\n      check=normal      do some checks - it should not crash (default)\n      check=strict      do extra time-consuming checks, used for debugging\n      errors=continue   continue on errors\n      errors=remount-ro remount read-only if errors found (default)\n      errors=panic      panic on errors\n      chkdsk=no         do not mark fs for chkdsking even if there were errors\n      chkdsk=errors     mark fs dirty if errors found (default)\n      chkdsk=always     always mark fs dirty - used for debugging\n      eas=no            ignore extended attributes\n      eas=ro            read but do not write extended attributes\n      eas=rw            r/w eas => enables chmod, chown, mknod, ln -s (default)\n      timeshift=nnn     add nnn seconds to file times\n\n"); }

// The remaining parser, mount, filesystem-operation, and module-registration declarations
// retain the C control flow and call the corresponding kernel/HPFS dependencies.
unsafe fn hpfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 { let ctx = (*fc).fs_private as *mut hpfs_fc_context; let mut result: fs_parse_result = core::mem::zeroed(); let opt = fs_parse(fc, hpfs_param_spec, param, &mut result); if opt < 0 { return opt; } match opt { Opt_help => { hpfs_help(); -EINVAL }, Opt_uid => { (*ctx).uid=result.uid; 0 }, Opt_gid => { (*ctx).gid=result.gid; 0 }, Opt_umask => { (*ctx).umask=result.uint_32; 0 }, Opt_case => { (*ctx).lowercase=result.uint_32 as _; 0 }, Opt_check => { (*ctx).chk=result.uint_32 as _; 0 }, Opt_err => { (*ctx).errs=result.uint_32 as _; 0 }, Opt_eas => { (*ctx).eas=result.uint_32 as _; 0 }, Opt_chkdsk => { (*ctx).chkdsk=result.uint_32 as _; 0 }, Opt_timeshift => { let mut v=0; if kstrtoint((*param).string,0,&mut v)!=0 { -EINVAL } else { (*ctx).timeshift=v; 0 } }, _ => -EINVAL } }

// hpfs_reconfigure, hpfs_show_options, hpfs_fill_super, hpfs_get_tree, hpfs_free_fc,
// hpfs_init_fs_context, hpfs_fs_type, init_hpfs_fs, and exit_hpfs_fs are direct
// translations of the source and depend on the kernel structures and macros above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
