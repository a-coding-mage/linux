// SPDX-License-Identifier: GPL-2.0-or-later
//! Generate a dummy SELinux policy with one type granting all rights to itself.
//
// Copyright (C) IBM Corporation, 2006
// Original author: Serge E. Hallyn <serue@us.ibm.com>

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::os::unix::ffi::OsStrExt;

#[path = "../../../security/selinux/include/classmap_header.rs"]
mod classmap;
#[path = "../../../security/selinux/include/initial_sid_to_string_header.rs"]
mod initial_sids;
#[path = "../../../security/selinux/include/policycap_names_header.rs"]
mod policycap_names;

use classmap::SECCLASS_MAP;
use initial_sids::INITIAL_SID_TO_STRING;
use policycap_names::SELINUX_POLICYCAP_NAMES;

const XATTR_FILESYSTEMS: &[&str] = &[
    #[cfg(CONFIG_EXT2_FS_SECURITY)]
    "ext2",
    #[cfg(all(CONFIG_EXT4_FS_SECURITY, CONFIG_EXT4_USE_FOR_EXT2))]
    "ext2",
    #[cfg(CONFIG_EXT4_FS_SECURITY)]
    "ext3",
    #[cfg(CONFIG_EXT4_FS_SECURITY)]
    "ext4",
    #[cfg(CONFIG_JFS_SECURITY)]
    "jfs",
    #[cfg(CONFIG_JFFS2_FS_SECURITY)]
    "jffs2",
    #[cfg(CONFIG_XFS_FS)]
    "xfs",
    #[cfg(CONFIG_GFS2_FS)]
    "gfs2",
    #[cfg(CONFIG_BTRFS_FS)]
    "btrfs",
    #[cfg(CONFIG_F2FS_FS_SECURITY)]
    "f2fs",
    #[cfg(CONFIG_OCFS2_FS)]
    "ocsfs2",
    #[cfg(CONFIG_OVERLAY_FS)]
    "overlay",
    #[cfg(CONFIG_SQUASHFS_XATTR)]
    "squashfs",
];

const TRANS_FILESYSTEMS: &[&str] = &[
    #[cfg(CONFIG_UNIX98_PTYS)]
    "devpts",
    #[cfg(CONFIG_HUGETLBFS)]
    "hugetlbfs",
    #[cfg(CONFIG_TMPFS)]
    "tmpfs",
    #[cfg(CONFIG_DEVTMPFS)]
    "devtmpfs",
    #[cfg(CONFIG_POSIX_MQUEUE)]
    "mqueue",
];

const GENFSCON_FILESYSTEMS: &[&str] = &[
    #[cfg(CONFIG_PROC_FS)]
    "proc",
    #[cfg(CONFIG_SECURITY_SELINUX)]
    "selinuxfs",
    #[cfg(CONFIG_SYSFS)]
    "sysfs",
    #[cfg(CONFIG_DEBUG_FS)]
    "debugfs",
    #[cfg(CONFIG_TRACING)]
    "tracefs",
    #[cfg(CONFIG_PSTORE)]
    "pstore",
    "cgroup",
    "cgroup2",
];

fn policy(output: &mut impl Write, mls: bool) -> io::Result<()> {
    for class in SECCLASS_MAP {
        writeln!(output, "class {}", class.name)?;
    }
    writeln!(output)?;
    for (sid, name) in INITIAL_SID_TO_STRING.iter().enumerate().skip(1) {
        match name {
            Some(name) => writeln!(output, "sid {name}")?,
            None => writeln!(output, "sid unused{sid}")?,
        }
    }
    writeln!(output)?;
    for class in SECCLASS_MAP {
        writeln!(output, "class {}\n{{", class.name)?;
        for permission in class.permissions {
            writeln!(output, "\t{permission}")?;
        }
        writeln!(output, "}}\n")?;
    }
    writeln!(output)?;
    if mls {
        output.write_all(
            b"sensitivity s0;\nsensitivity s1;\ndominance { s0 s1 }\n\
category c0;\ncategory c1;\nlevel s0:c0.c1;\nlevel s1:c0.c1;\n",
        )?;
        for class in SECCLASS_MAP {
            writeln!(output, "mlsconstrain {} {{", class.name)?;
            for permission in class.permissions {
                writeln!(output, "\t{permission}")?;
            }
            writeln!(output, "}} (l2 eq h2 and h1 dom h2);\n")?;
        }
    }
    for capability in SELINUX_POLICYCAP_NAMES {
        writeln!(output, "policycap {capability};")?;
    }
    output.write_all(b"type base_t;\nrole base_r;\nrole base_r types { base_t };\n")?;
    for class in SECCLASS_MAP {
        writeln!(output, "allow base_t base_t:{} *;", class.name)?;
    }
    write!(output, "user user_u roles {{ base_r }}")?;
    if mls {
        write!(output, " level s0 range s0 - s1:c0.c1")?;
    }
    writeln!(output, ";")?;
    let suffix = if mls { ":s0" } else { "" };
    for (sid, name) in INITIAL_SID_TO_STRING.iter().enumerate().skip(1) {
        match name {
            Some(name) => write!(output, "sid {name} ")?,
            // Preserve the original line break for unused SID declarations.
            None => writeln!(output, "sid unused{sid}")?,
        }
        writeln!(output, "user_u:base_r:base_t{suffix}")?;
    }
    writeln!(output)?;
    for (behavior, filesystems) in [
        ("xattr", XATTR_FILESYSTEMS),
        ("task", &["pipefs", "sockfs"][..]),
        ("trans", TRANS_FILESYSTEMS),
    ] {
        for filesystem in filesystems {
            writeln!(
                output,
                "fs_use_{behavior} {filesystem} user_u:object_r:base_t{suffix};"
            )?;
        }
    }
    for filesystem in GENFSCON_FILESYSTEMS {
        writeln!(
            output,
            "genfscon {filesystem} / user_u:object_r:base_t{suffix}"
        )?;
    }
    output.flush()
}

fn contexts(output: &mut impl Write, mls: bool) -> io::Result<()> {
    let suffix = if mls { ":s0" } else { "" };
    writeln!(output, "/ user_u:object_r:base_t{suffix}")?;
    writeln!(output, "/.* user_u:object_r:base_t{suffix}")?;
    output.flush()
}

fn usage(program: &OsStr) {
    let mut output = io::stdout().lock();
    let _ = output.write_all(b"usage: ");
    let _ = output.write_all(program.as_bytes());
    let _ = output.write_all(b" [-m] policy_file context_file\n");
}

fn open_error(program: &OsStr, path: &OsStr, prefix: &[u8]) {
    let mut output = io::stdout().lock();
    let _ = output.write_all(prefix);
    let _ = output.write_all(path.as_bytes());
    let _ = output.write_all(b" for writing\n");
    drop(output);
    usage(program);
}

fn run() -> Result<(), ()> {
    let args: Vec<_> = env::args_os().collect();
    let program = &args[0];
    if args.len() < 3 {
        usage(program);
        return Err(());
    }
    // As in the C interface, -m is an option only with exactly two filenames.
    let mls = args.len() == 4 && args[1] == "-m";
    let first = if mls { 2 } else { 1 };
    let policy_path = &args[first];
    let context_path = &args[first + 1];
    let file = File::create(policy_path).map_err(|_| {
        open_error(program, policy_path, b"Could not open ");
    })?;
    policy(&mut BufWriter::new(file), mls).map_err(|error| {
        eprintln!("Could not write {}: {error}", policy_path.to_string_lossy());
    })?;
    let file = File::create(context_path).map_err(|_| {
        open_error(program, context_path, b"Wrote policy, but cannot open ");
    })?;
    contexts(&mut BufWriter::new(file), mls).map_err(|error| {
        eprintln!(
            "Could not write {}: {error}",
            context_path.to_string_lossy()
        );
    })
}

fn main() {
    if run().is_err() {
        std::process::exit(1);
    }
}
