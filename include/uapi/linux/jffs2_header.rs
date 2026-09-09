/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in the
 * jffs2 directory.
 */

/* Values we may expect to find in the 'magic' field */
pub const JFFS2_OLD_MAGIC_BITMASK: u32 = 0x1984;
pub const JFFS2_MAGIC_BITMASK: u32 = 0x1985;
pub const KSAMTIB_CIGAM_2SFFJ: u32 = 0x8519; /* For detecting wrong-endian fs */
pub const JFFS2_EMPTY_BITMASK: u32 = 0xffff;
pub const JFFS2_DIRTY_BITMASK: u32 = 0x0000;

/* Summary node MAGIC marker */
pub const JFFS2_SUM_MAGIC: u32 = 0x02851885;

/* We only allow a single char for length, and 0xFF is empty flash so
   we don't want it confused with a real length. Hence max 254.
*/
pub const JFFS2_MAX_NAME_LEN: u32 = 254;
pub const JFFS2_MIN_DATA_LEN: u32 = 128;

pub const JFFS2_COMPR_NONE: u32 = 0x00;
pub const JFFS2_COMPR_ZERO: u32 = 0x01;
pub const JFFS2_COMPR_RTIME: u32 = 0x02;
pub const JFFS2_COMPR_RUBINMIPS: u32 = 0x03;
pub const JFFS2_COMPR_COPY: u32 = 0x04;
pub const JFFS2_COMPR_DYNRUBIN: u32 = 0x05;
pub const JFFS2_COMPR_ZLIB: u32 = 0x06;
pub const JFFS2_COMPR_LZO: u32 = 0x07;
pub const JFFS2_COMPAT_MASK: u32 = 0xc000;
pub const JFFS2_NODE_ACCURATE: u32 = 0x2000;
pub const JFFS2_FEATURE_INCOMPAT: u32 = 0xc000;
pub const JFFS2_FEATURE_ROCOMPAT: u32 = 0x8000;
pub const JFFS2_FEATURE_RWCOMPAT_COPY: u32 = 0x4000;
pub const JFFS2_FEATURE_RWCOMPAT_DELETE: u32 = 0x0000;

pub const JFFS2_NODETYPE_DIRENT: u32 = JFFS2_FEATURE_INCOMPAT | JFFS2_NODE_ACCURATE | 1;
pub const JFFS2_NODETYPE_INODE: u32 = JFFS2_FEATURE_INCOMPAT | JFFS2_NODE_ACCURATE | 2;
pub const JFFS2_NODETYPE_CLEANMARKER: u32 = JFFS2_FEATURE_RWCOMPAT_DELETE | JFFS2_NODE_ACCURATE | 3;
pub const JFFS2_NODETYPE_PADDING: u32 = JFFS2_FEATURE_RWCOMPAT_DELETE | JFFS2_NODE_ACCURATE | 4;
pub const JFFS2_NODETYPE_SUMMARY: u32 = JFFS2_FEATURE_RWCOMPAT_DELETE | JFFS2_NODE_ACCURATE | 6;
pub const JFFS2_NODETYPE_XATTR: u32 = JFFS2_FEATURE_INCOMPAT | JFFS2_NODE_ACCURATE | 8;
pub const JFFS2_NODETYPE_XREF: u32 = JFFS2_FEATURE_INCOMPAT | JFFS2_NODE_ACCURATE | 9;

pub const JFFS2_XPREFIX_USER: u32 = 1;
pub const JFFS2_XPREFIX_SECURITY: u32 = 2;
pub const JFFS2_XPREFIX_ACL_ACCESS: u32 = 3;
pub const JFFS2_XPREFIX_ACL_DEFAULT: u32 = 4;
pub const JFFS2_XPREFIX_TRUSTED: u32 = 5;
pub const JFFS2_ACL_VERSION: u32 = 0x0001;
pub const JFFS2_INO_FLAG_PREREAD: u32 = 1;
pub const JFFS2_INO_FLAG_USERCOMPR: u32 = 2;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct jint32_t { pub v32: __u32 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct jmode_t { pub m: __u32 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct jint16_t { pub v16: __u16 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct jffs2_unknown_node { pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t }

#[repr(C, packed)]
pub struct jffs2_raw_dirent {
    pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t,
    pub pino: jint32_t, pub version: jint32_t, pub ino: jint32_t, pub mctime: jint32_t,
    pub nsize: __u8, pub type_: __u8, pub unused: [__u8; 2], pub node_crc: jint32_t,
    pub name_crc: jint32_t, pub name: [__u8; 0],
}

#[repr(C, packed)]
pub struct jffs2_raw_inode {
    pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t,
    pub ino: jint32_t, pub version: jint32_t, pub mode: jmode_t, pub uid: jint16_t, pub gid: jint16_t,
    pub isize: jint32_t, pub atime: jint32_t, pub mtime: jint32_t, pub ctime: jint32_t,
    pub offset: jint32_t, pub csize: jint32_t, pub dsize: jint32_t, pub compr: __u8,
    pub usercompr: __u8, pub flags: jint16_t, pub data_crc: jint32_t, pub node_crc: jint32_t,
    pub data: [__u8; 0],
}

#[repr(C, packed)]
pub struct jffs2_raw_xattr {
    pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t,
    pub xid: jint32_t, pub version: jint32_t, pub xprefix: __u8, pub name_len: __u8,
    pub value_len: jint16_t, pub data_crc: jint32_t, pub node_crc: jint32_t, pub data: [__u8; 0],
}

#[repr(C, packed)]
pub struct jffs2_raw_xref {
    pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t,
    pub ino: jint32_t, pub xid: jint32_t, pub xseqno: jint32_t, pub node_crc: jint32_t,
}

#[repr(C, packed)]
pub struct jffs2_raw_summary {
    pub magic: jint16_t, pub nodetype: jint16_t, pub totlen: jint32_t, pub hdr_crc: jint32_t,
    pub sum_num: jint32_t, pub cln_mkr: jint32_t, pub padded: jint32_t, pub sum_crc: jint32_t,
    pub node_crc: jint32_t, pub sum: [jint32_t; 0],
}

#[repr(C)]
pub union jffs2_node_union {
    pub i: jffs2_raw_inode,
    pub d: jffs2_raw_dirent,
    pub x: jffs2_raw_xattr,
    pub r: jffs2_raw_xref,
    pub s: jffs2_raw_summary,
    pub u: jffs2_unknown_node,
}

/* Data payload for device nodes. */
#[repr(C)]
pub union jffs2_device_node {
    pub old_id: jint16_t,
    pub new_id: jint32_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
