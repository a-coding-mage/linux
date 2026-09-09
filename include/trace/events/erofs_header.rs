/* SPDX-License-Identifier: GPL-2.0-only */
//! Rust translation of `trace/events/erofs.h`.
//!
//! The Linux tracepoint framework is supplied externally.  The declarations
//! below preserve the tracepoint data and formatting at the source level.

// `TRACE_SYSTEM erofs`; C header guards and includes are intentionally omitted.

#[inline]
pub fn show_dev(dev: u64) -> (u32, u32) {
    (((dev >> 20) & 0xfff) as u32, (dev & 0xfffff) as u32)
}

#[inline]
pub unsafe fn show_dev_nid<E>(entry: *const E, _dev: u64, _nid: u64) -> (u32, u32, u64) {
    // C expression: show_dev(entry->dev), entry->nid.
    let _ = entry;
    (_dev as u32, (_dev >> 32) as u32, _nid)
}

#[inline]
pub const fn show_file_type(type_: i32) -> &'static str {
    match type_ {
        0 => "FILE",
        1 => "DIR",
        _ => "",
    }
}

// These constants are provided by the EROFS implementation.
extern "C" {
    pub static EROFS_GET_BLOCKS_FIEMAP: u32;
    pub static EROFS_GET_BLOCKS_READMORE: u32;
    pub static EROFS_GET_BLOCKS_FINDTAIL: u32;
    pub static EROFS_MAP_MAPPED: u32;
    pub static EROFS_MAP_META: u32;
    pub static EROFS_MAP_PARTIAL_MAPPED: u32;
    pub static EROFS_MAP_PARTIAL_REF: u32;
    pub static EROFS_MAP_FRAGMENT: u32;
}

// `__print_flags` in the kernel tracepoint API joins the matching symbolic
// names using the supplied separator.
pub fn show_map_flags(flags: u32) -> String {
    let values = [
        (unsafe { EROFS_GET_BLOCKS_FIEMAP }, "FIEMAP"),
        (unsafe { EROFS_GET_BLOCKS_READMORE }, "READMORE"),
        (unsafe { EROFS_GET_BLOCKS_FINDTAIL }, "FINDTAIL"),
    ];
    values.iter().filter(|(bit, _)| flags & *bit != 0).map(|(_, s)| *s).collect::<Vec<_>>().join("|")
}

pub fn show_mflags(flags: u32) -> String {
    let values = [
        (unsafe { EROFS_MAP_MAPPED }, "M"),
        (unsafe { EROFS_MAP_META }, "I"),
        (unsafe { EROFS_MAP_PARTIAL_MAPPED }, "T"),
        (unsafe { EROFS_MAP_PARTIAL_REF }, "P"),
        (unsafe { EROFS_MAP_FRAGMENT }, "R"),
    ];
    values.iter().filter(|(bit, _)| flags & *bit != 0).map(|(_, s)| *s).collect::<Vec<_>>().join("")
}

// The following trace-event declarations retain the original event names,
// prototypes, entry fields, assignments, and printk formats.  Their actual
// registration and field extraction are performed by the external tracepoint
// framework, equivalent to TRACE_EVENT in the C header.

/// TRACE_EVENT(erofs_lookup):
/// TP_PROTO(struct inode *dir, struct dentry *dentry, unsigned int flags)
/// fields: dev_t dev; erofs_nid_t nid; string name; unsigned int flags
/// assign: dev = dir->i_sb->s_dev; nid = EROFS_I(dir)->nid;
///         name = dentry->d_name.name; flags = flags
/// print: "dev = (%d,%d), pnid = %llu, name:%s, flags:%x"
pub const EROFS_LOOKUP: &str = "erofs_lookup";

/// TRACE_EVENT(erofs_fill_inode): fields dev, nid, blkaddr, ofs.
/// Assigns s_dev, EROFS_I(inode)->nid, erofs_blknr(...), erofs_blkoff(...).
/// print: "dev = (%d,%d), nid = %llu, blkaddr %llu ofs %u"
pub const EROFS_FILL_INODE: &str = "erofs_fill_inode";

/// TRACE_EVENT(erofs_read_folio): fields dev, nid, dir, index, order, raw.
/// print: "dev = (%d,%d), nid = %llu, %s, index = %lu, order = %u, raw = %d"
pub const EROFS_READ_FOLIO: &str = "erofs_read_folio";

/// TRACE_EVENT(erofs_readahead): fields dev, nid, start, nrpage, raw.
/// print: "dev = (%d,%d), nid = %llu, start = %lu nrpage = %u raw = %d"
pub const EROFS_READAHEAD: &str = "erofs_readahead";

/// TRACE_EVENT(erofs_map_blocks_enter): fields dev, nid, la, llen, flags.
/// print: "dev = (%d,%d), nid = %llu, la %llu llen %llu flags %s"
pub const EROFS_MAP_BLOCKS_ENTER: &str = "erofs_map_blocks_enter";

/// TRACE_EVENT(erofs_map_blocks_exit): fields dev, nid, flags, la, pa, llen,
/// plen, mflags, ret.
/// print: "dev = (%d,%d), nid = %llu, flags %s la %llu pa %llu llen %llu
/// plen %llu mflags %s ret %d"
pub const EROFS_MAP_BLOCKS_EXIT: &str = "erofs_map_blocks_exit";

// `trace/define_trace.h` is a build-time inclusion hook in the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
