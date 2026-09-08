// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * mdp - make dummy policy
 *
 * When pointed at a kernel tree, builds a dummy policy for that kernel
 * with exactly one type with full rights to itself.
 *
 * Copyright (C) IBM Corporation, 2006
 *
 * Authors: Serge E. Hallyn <serue@us.ibm.com>
 */

use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SecurityClassMapping {
    pub name: *const c_char,
    pub perms: [*const c_char; std::mem::size_of::<u32>() * 8 + 1],
}

extern "C" {
    static secclass_map: SecurityClassMapping;
    static initial_sid_to_string: *const *const c_char;
    static selinux_policycap_names: *const *const c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
}

unsafe fn usage(name: *mut c_char) {
    printf(b"usage: %s [-m] policy_file context_file\n\0".as_ptr() as *const c_char, name);
    exit(1);
}

unsafe fn c_string(value: &str) -> *const c_char {
    value.as_ptr() as *const c_char
}

#[inline]
unsafe fn fs_use(fout: *mut FILE, behavior: &str, fstype: &str, mls: bool) {
    let suffix = if mls { ":s0\0" } else { "\0" };
    fprintf(
        fout,
        b"fs_use_%s %s user_u:object_r:base_t%s;\n\0".as_ptr() as *const c_char,
        c_string(behavior),
        c_string(fstype),
        c_string(suffix),
    );
}

#[inline]
unsafe fn gen_fscon(fout: *mut FILE, fstype: &str, prefix: &str, mls: bool) {
    let suffix = if mls { ":s0\0" } else { "\0" };
    fprintf(
        fout,
        b"genfscon %s %s user_u:object_r:base_t%s\n\0".as_ptr() as *const c_char,
        c_string(fstype),
        c_string(prefix),
        c_string(suffix),
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut mls = false;
    if argc < 3 {
        usage(*argv);
    }

    let mut arg = argv.add(1);
    if argc == 4 && strcmp(*arg.add(0), b"-m\0".as_ptr() as *const c_char) == 0 {
        mls = true;
        arg = arg.add(1);
    }
    let polout = *arg;
    let ctxout = *arg.add(1);
    let mut fout = fopen(polout, b"w\0".as_ptr() as *const c_char);
    if fout.is_null() {
        printf(b"Could not open %s for writing\n\0".as_ptr() as *const c_char, polout);
        usage(*argv);
    }

    let mut i = 0usize;
    while !(*(&secclass_map as *const SecurityClassMapping).add(i)).name.is_null() {
        let map = &*(&secclass_map as *const SecurityClassMapping).add(i);
        fprintf(fout, b"class %s\n\0".as_ptr() as *const c_char, map.name);
        i += 1;
    }
    fprintf(fout, b"\n\0".as_ptr() as *const c_char);

    // The C source computes sizeof(initial_sid_to_string) from the included table.
    // The external table is traversed through its terminating null entry here.
    i = 1;
    while !(*initial_sid_to_string.add(i)).is_null() {
        let name = *initial_sid_to_string.add(i);
        if !name.is_null() {
            fprintf(fout, b"sid %s\n\0".as_ptr() as *const c_char, name);
        } else {
            fprintf(fout, b"sid unused%d\n\0".as_ptr() as *const c_char, i as c_int);
        }
        i += 1;
    }
    fprintf(fout, b"\n\0".as_ptr() as *const c_char);

    i = 0;
    while !(*(&secclass_map as *const SecurityClassMapping).add(i)).name.is_null() {
        let map = &*(&secclass_map as *const SecurityClassMapping).add(i);
        fprintf(fout, b"class %s\n{\n\0".as_ptr() as *const c_char, map.name);
        let mut j = 0usize;
        while !map.perms[j].is_null() {
            fprintf(fout, b"\t%s\n\0".as_ptr() as *const c_char, map.perms[j]);
            j += 1;
        }
        fprintf(fout, b"}\n\n\0".as_ptr() as *const c_char);
        i += 1;
    }
    fprintf(fout, b"\n\0".as_ptr() as *const c_char);

    if mls {
        fprintf(fout, b"sensitivity s0;\nsensitivity s1;\ndominance { s0 s1 }\ncategory c0;\ncategory c1;\nlevel s0:c0.c1;\nlevel s1:c0.c1;\n\0".as_ptr() as *const c_char);
        i = 0;
        while !(*(&secclass_map as *const SecurityClassMapping).add(i)).name.is_null() {
            let map = &*(&secclass_map as *const SecurityClassMapping).add(i);
            fprintf(fout, b"mlsconstrain %s {\n\0".as_ptr() as *const c_char, map.name);
            let mut j = 0usize;
            while !map.perms[j].is_null() {
                fprintf(fout, b"\t%s\n\0".as_ptr() as *const c_char, map.perms[j]);
                j += 1;
            }
            fprintf(fout, b"} (l2 eq h2 and h1 dom h2);\n\n\0".as_ptr() as *const c_char);
            i += 1;
        }
    }

    i = 0;
    while !(*selinux_policycap_names.add(i)).is_null() {
        fprintf(fout, b"policycap %s;\n\0".as_ptr() as *const c_char, *selinux_policycap_names.add(i));
        i += 1;
    }
    fprintf(fout, b"type base_t;\nrole base_r;\nrole base_r types { base_t };\n\0".as_ptr() as *const c_char);
    i = 0;
    while !(*(&secclass_map as *const SecurityClassMapping).add(i)).name.is_null() {
        fprintf(fout, b"allow base_t base_t:%s *;\n\0".as_ptr() as *const c_char, (*(&secclass_map as *const SecurityClassMapping).add(i)).name);
        i += 1;
    }
    fprintf(fout, b"user user_u roles { base_r }\0".as_ptr() as *const c_char);
    if mls { fprintf(fout, b" level s0 range s0 - s1:c0.c1\0".as_ptr() as *const c_char); }
    fprintf(fout, b";\n\0".as_ptr() as *const c_char);

    i = 1;
    while !(*initial_sid_to_string.add(i)).is_null() {
        let name = *initial_sid_to_string.add(i);
        if !name.is_null() { fprintf(fout, b"sid %s \0".as_ptr() as *const c_char, name); }
        else { fprintf(fout, b"sid unused%d\n\0".as_ptr() as *const c_char, i as c_int); }
        fprintf(fout, b"user_u:base_r:base_t%s\n\0".as_ptr() as *const c_char, if mls { b":s0\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char });
        i += 1;
    }
    fprintf(fout, b"\n\0".as_ptr() as *const c_char);

    fs_use(fout, "task", "pipefs", mls);
    fs_use(fout, "task", "sockfs", mls);
    // Build-time CONFIG_* conditionals from the C source are represented by cfg gates.
    #[cfg(feature = "CONFIG_EXT2_FS_SECURITY")] fs_use(fout, "xattr", "ext2", mls);
    #[cfg(feature = "CONFIG_EXT4_USE_FOR_EXT2")] fs_use(fout, "xattr", "ext2", mls);
    #[cfg(feature = "CONFIG_EXT4_FS_SECURITY")] { fs_use(fout, "xattr", "ext3", mls); fs_use(fout, "xattr", "ext4", mls); }
    #[cfg(feature = "CONFIG_JFS_SECURITY")] fs_use(fout, "xattr", "jfs", mls);
    #[cfg(feature = "CONFIG_JFFS2_FS_SECURITY")] fs_use(fout, "xattr", "jffs2", mls);
    #[cfg(feature = "CONFIG_XFS_FS")] fs_use(fout, "xattr", "xfs", mls);
    #[cfg(feature = "CONFIG_GFS2_FS")] fs_use(fout, "xattr", "gfs2", mls);
    #[cfg(feature = "CONFIG_BTRFS_FS")] fs_use(fout, "xattr", "btrfs", mls);
    #[cfg(feature = "CONFIG_F2FS_FS_SECURITY")] fs_use(fout, "xattr", "f2fs", mls);
    #[cfg(feature = "CONFIG_OCFS2_FS")] fs_use(fout, "xattr", "ocsfs2", mls);
    #[cfg(feature = "CONFIG_OVERLAY_FS")] fs_use(fout, "xattr", "overlay", mls);
    #[cfg(feature = "CONFIG_SQUASHFS_XATTR")] fs_use(fout, "xattr", "squashfs", mls);
    #[cfg(feature = "CONFIG_UNIX98_PTYS")] fs_use(fout, "trans", "devpts", mls);
    #[cfg(feature = "CONFIG_HUGETLBFS")] fs_use(fout, "trans", "hugetlbfs", mls);
    #[cfg(feature = "CONFIG_TMPFS")] fs_use(fout, "trans", "tmpfs", mls);
    #[cfg(feature = "CONFIG_DEVTMPFS")] fs_use(fout, "trans", "devtmpfs", mls);
    #[cfg(feature = "CONFIG_POSIX_MQUEUE")] fs_use(fout, "trans", "mqueue", mls);
    #[cfg(feature = "CONFIG_PROC_FS")] gen_fscon(fout, "proc", "/", mls);
    #[cfg(feature = "CONFIG_SECURITY_SELINUX")] gen_fscon(fout, "selinuxfs", "/", mls);
    #[cfg(feature = "CONFIG_SYSFS")] gen_fscon(fout, "sysfs", "/", mls);
    #[cfg(feature = "CONFIG_DEBUG_FS")] gen_fscon(fout, "debugfs", "/", mls);
    #[cfg(feature = "CONFIG_TRACING")] gen_fscon(fout, "tracefs", "/", mls);
    #[cfg(feature = "CONFIG_PSTORE")] gen_fscon(fout, "pstore", "/", mls);
    gen_fscon(fout, "cgroup", "/", mls);
    gen_fscon(fout, "cgroup2", "/", mls);
    fclose(fout);

    fout = fopen(ctxout, b"w\0".as_ptr() as *const c_char);
    if fout.is_null() { printf(b"Wrote policy, but cannot open %s for writing\n\0".as_ptr() as *const c_char, ctxout); usage(*argv); }
    fprintf(fout, b"/ user_u:object_r:base_t%s\n/.* user_u:object_r:base_t%s\n\0".as_ptr() as *const c_char, if mls { b":s0\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char }, if mls { b":s0\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char });
    fclose(fout);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
