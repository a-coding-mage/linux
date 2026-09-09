/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// The Linux printk/scheduler headers are external dependencies of this translation.
// CONFIG_JFFS2_FS_DEBUG defaults to zero when not supplied by the build.
pub const CONFIG_JFFS2_FS_DEBUG: i32 = 0;

// When CONFIG_JFFS2_FS_DEBUG > 0: JFFS2_DBG_PARANOIA_CHECKS and
// JFFS2_DBG_DUMPS, together with the level-one subsystem message selections,
// are enabled. When it is greater than one, the level-two selections are enabled.
// Sanity checks are enabled by default.

#[allow(non_camel_case_types)]
pub enum jffs2_sb_info {}
#[allow(non_camel_case_types)]
pub enum jffs2_eraseblock {}
#[allow(non_camel_case_types)]
pub enum jffs2_inode_info {}

pub type uint32_t = u32;

// C preprocessor logging macros are represented as Rust forwarding macros.
// The supplied logging symbols are external dependencies.
#[macro_export]
macro_rules! jffs2_dbg {
    ($level:expr, $($arg:tt)*) => {
        if $crate::CONFIG_JFFS2_FS_DEBUG >= ($level as i32) {
            unsafe { $crate::pr_debug!($($arg)*); }
        }
    };
}

#[macro_export]
macro_rules! D1 { ($($arg:tt)*) => { $($arg)* }; }
#[macro_export]
macro_rules! D2 { ($($arg:tt)*) => { $($arg)* }; }

#[macro_export]
macro_rules! JFFS2_ERROR { ($($arg:tt)*) => { unsafe { $crate::pr_err!($($arg)*); } }; }
#[macro_export]
macro_rules! JFFS2_WARNING { ($($arg:tt)*) => { unsafe { $crate::pr_warn!($($arg)*); } }; }
#[macro_export]
macro_rules! JFFS2_NOTICE { ($($arg:tt)*) => { unsafe { $crate::pr_notice!($($arg)*); } }; }
#[macro_export]
macro_rules! JFFS2_DEBUG { ($($arg:tt)*) => { unsafe { $crate::printk!($($arg)*); } }; }

// The following subsystem macros expand to the debug logger when their
// corresponding C selection macro is enabled, and otherwise to no_printk.
#[macro_export]
macro_rules! dbg_readinode { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_readinode2 { ($($arg:tt)*) => { $crate::no_printk!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_fragtree { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_fragtree2 { ($($arg:tt)*) => { $crate::no_printk!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_dentlist { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_noderef { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_inocache { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_summary { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_fsbuild { ($($arg:tt)*) => { $crate::JFFS2_DEBUG!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_memalloc { ($($arg:tt)*) => { $crate::no_printk!($($arg)*) }; }
#[macro_export]
macro_rules! dbg_xattr { ($($arg:tt)*) => { $crate::no_printk!($($arg)*) }; }

extern "C" {
    pub fn __jffs2_dbg_acct_sanity_check_nolock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_acct_sanity_check(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_fragtree_paranoia_check(f: *mut jffs2_inode_info);
    pub fn __jffs2_dbg_fragtree_paranoia_check_nolock(f: *mut jffs2_inode_info);
    pub fn __jffs2_dbg_acct_paranoia_check(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_acct_paranoia_check_nolock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_prewrite_paranoia_check(c: *mut jffs2_sb_info, ofs: uint32_t, len: i32);
    pub fn __jffs2_dbg_dump_jeb(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_dump_jeb_nolock(jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_dump_block_lists(c: *mut jffs2_sb_info);
    pub fn __jffs2_dbg_dump_block_lists_nolock(c: *mut jffs2_sb_info);
    pub fn __jffs2_dbg_dump_node_refs(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_dump_node_refs_nolock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    pub fn __jffs2_dbg_dump_fragtree(f: *mut jffs2_inode_info);
    pub fn __jffs2_dbg_dump_fragtree_nolock(f: *mut jffs2_inode_info);
    pub fn __jffs2_dbg_dump_buffer(buf: *mut u8, len: i32, offs: uint32_t);
    pub fn __jffs2_dbg_dump_node(c: *mut jffs2_sb_info, ofs: uint32_t);
}

// The C wrapper macros below are enabled or empty according to the build-time
// JFFS2_DBG_PARANOIA_CHECKS, JFFS2_DBG_DUMPS, and JFFS2_DBG_SANITY_CHECKS selections.
#[macro_export]
macro_rules! jffs2_dbg_fragtree_paranoia_check { ($f:expr) => { unsafe { $crate::__jffs2_dbg_fragtree_paranoia_check($f) } }; }
#[macro_export]
macro_rules! jffs2_dbg_fragtree_paranoia_check_nolock { ($f:expr) => { unsafe { $crate::__jffs2_dbg_fragtree_paranoia_check_nolock($f) } }; }
#[macro_export]
macro_rules! jffs2_dbg_acct_paranoia_check { ($c:expr, $j:expr) => { unsafe { $crate::__jffs2_dbg_acct_paranoia_check($c, $j) } }; }
#[macro_export]
macro_rules! jffs2_dbg_acct_paranoia_check_nolock { ($c:expr, $j:expr) => { unsafe { $crate::__jffs2_dbg_acct_paranoia_check_nolock($c, $j) } }; }
#[macro_export]
macro_rules! jffs2_dbg_prewrite_paranoia_check { ($c:expr, $o:expr, $l:expr) => { unsafe { $crate::__jffs2_dbg_prewrite_paranoia_check($c, $o, $l) } }; }
#[macro_export]
macro_rules! jffs2_dbg_acct_sanity_check { ($c:expr, $j:expr) => { unsafe { $crate::__jffs2_dbg_acct_sanity_check($c, $j) } }; }
#[macro_export]
macro_rules! jffs2_dbg_acct_sanity_check_nolock { ($c:expr, $j:expr) => { unsafe { $crate::__jffs2_dbg_acct_sanity_check_nolock($c, $j) } }; }

#[macro_export]
macro_rules! jffs2_dbg_dump_jeb { ($c:expr, $j:expr) => { unsafe { $crate::__jffs2_dbg_dump_jeb($c, $j) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_jeb_nolock { ($j:expr) => { unsafe { $crate::__jffs2_dbg_dump_jeb_nolock($j) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_block_lists { ($c:expr) => { unsafe { $crate::__jffs2_dbg_dump_block_lists($c) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_block_lists_nolock { ($c:expr) => { unsafe { $crate::__jffs2_dbg_dump_block_lists_nolock($c) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_fragtree { ($f:expr) => { unsafe { $crate::__jffs2_dbg_dump_fragtree($f) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_fragtree_nolock { ($f:expr) => { unsafe { $crate::__jffs2_dbg_dump_fragtree_nolock($f) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_buffer { ($b:expr, $l:expr, $o:expr) => { unsafe { $crate::__jffs2_dbg_dump_buffer($b, $l, $o) } }; }
#[macro_export]
macro_rules! jffs2_dbg_dump_node { ($c:expr, $o:expr) => { unsafe { $crate::__jffs2_dbg_dump_node($c, $o) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
