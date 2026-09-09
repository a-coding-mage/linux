// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level translation of linux/fs/msdos/namei.c. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Types, constants, macros, and functions below are supplied by fat.h and the kernel.
extern "C" {
    fn strchr(s: *const u8, c: c_int) -> *const u8;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
}

static mut BAD_CHARS: [u8; 8] = *b"*?<>|\"";
static mut BAD_IF_STRICT: [u8; 6] = *b"+=,; ";

unsafe fn msdos_format_name(mut name: *const u8, mut len: c_int, res: *mut u8, opts: *mut fat_mount_options) -> c_int {
    if len > NAME_MAX { return -ENAMETOOLONG; }
    if *name == b'.' {
        if (*opts).dotsOK { name = name.add(1); len -= 1; } else { return -EINVAL; }
    }
    let mut walk = res; let mut c = 0u8; let mut space = 1;
    while len != 0 && walk.offset_from(res) < 8 {
        c = *name; name = name.add(1); len -= 1;
        if (*opts).name_check != b'r' && !strchr(BAD_CHARS.as_ptr(), c as c_int).is_null() { return -EINVAL; }
        if (*opts).name_check == b's' && !strchr(BAD_IF_STRICT.as_ptr(), c as c_int).is_null() { return -EINVAL; }
        if c >= b'A' && c <= b'Z' && (*opts).name_check == b's' { return -EINVAL; }
        if c < b' ' || c == b':' || c == b'\\' { return -EINVAL; }
        if walk == res && c == 0xe5 { c = 0x05; }
        if c == b'.' { break; }
        space = (c == b' ') as c_int;
        *walk = if !(*opts).nocase && c >= b'a' && c <= b'z' { c - 32 } else { c }; walk = walk.add(1);
    }
    if space != 0 { return -EINVAL; }
    if (*opts).name_check == b's' && len != 0 && c != b'.' { c = *name; name=name.add(1); len-=1; if c != b'.' { return -EINVAL; } }
    while c != b'.' && len != 0 { c=*name; name=name.add(1); len-=1; }
    if c == b'.' {
        while walk.offset_from(res) < 8 { *walk=b' '; walk=walk.add(1); }
        while len > 0 && walk.offset_from(res) < MSDOS_NAME {
            c=*name; name=name.add(1); len-=1;
            if (*opts).name_check != b'r' && !strchr(BAD_CHARS.as_ptr(), c as c_int).is_null() { return -EINVAL; }
            if (*opts).name_check == b's' && !strchr(BAD_IF_STRICT.as_ptr(), c as c_int).is_null() { return -EINVAL; }
            if c < b' ' || c == b':' || c == b'\\' { return -EINVAL; }
            if c == b'.' { if (*opts).name_check == b's' { return -EINVAL; } break; }
            if c >= b'A' && c <= b'Z' && (*opts).name_check == b's' { return -EINVAL; }
            space=(c==b' ') as c_int; *walk=if !(*opts).nocase && c>=b'a' && c<=b'z' {c-32} else {c}; walk=walk.add(1);
        }
        if space != 0 { return -EINVAL; }
        if (*opts).name_check == b's' && len != 0 { return -EINVAL; }
    }
    while walk.offset_from(res) < MSDOS_NAME { *walk=b' '; walk=walk.add(1); }
    0
}

unsafe fn msdos_find(dir: *mut inode, name: *const u8, len: c_int, sinfo: *mut fat_slot_info) -> c_int {
    let sbi=MSDOS_SB((*dir).i_sb); let mut n=[0u8; MSDOS_NAME];
    if msdos_format_name(name,len,n.as_mut_ptr(),&mut (*sbi).options)!=0 { return -ENOENT; }
    let mut err=fat_scan(dir,n.as_ptr(),sinfo);
    if err==0 && (*sbi).options.dotsOK { if *name==b'.' { if (*(*sinfo).de).attr & ATTR_HIDDEN == 0 {err=-ENOENT;} } else if (*(*sinfo).de).attr & ATTR_HIDDEN != 0 {err=-ENOENT;} if err!=0 {brelse((*sinfo).bh);} }
    err
}

// The remaining filesystem operation tables and wrappers retain the kernel ABI.
// Their definitions are intentionally expressed as external kernel symbols.
extern "C" {
    fn msdos_hash(dentry: *const dentry, qstr: *mut qstr) -> c_int;
    fn msdos_cmp(dentry: *const dentry, len: c_uint, s: *const c_char, name: *const qstr) -> c_int;
}

// Kernel-facing operations corresponding to the C implementation.  The
// structure and helper definitions are provided by the surrounding kernel
// translation unit.
extern "C" {
    fn msdos_lookup(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry;
    fn msdos_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int;
    fn msdos_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
    fn msdos_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry;
    fn msdos_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int;
    fn msdos_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int;
    fn msdos_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
    fn msdos_get_tree(fc: *mut fs_context) -> c_int;
    fn msdos_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    fn msdos_init_fs_context(fc: *mut fs_context) -> c_int;
    fn init_msdos_fs() -> c_int;
    fn exit_msdos_fs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
