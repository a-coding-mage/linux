/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

/*
 * jfs_btree.h: B+-tree
 *
 * JFS B+-tree (dtree and xtree) common definitions
 */

/*
 * basic btree page - btpage
 *
 * struct btpage {
 *     s64 next;        right sibling bn
 *     s64 prev;        left sibling bn
 *
 *     u8 flag;
 *     u8 rsrvd[7];     type specific
 *     s64 self;        self address
 *
 *     u8 entry[4064];
 * };
 */

/* btpaget_t flag */
pub const BT_TYPE: u8 = 0x07;       /* B+-tree index */
pub const BT_ROOT: u8 = 0x01;       /* root page */
pub const BT_LEAF: u8 = 0x02;       /* leaf page */
pub const BT_INTERNAL: u8 = 0x04;   /* internal page */
pub const BT_RIGHTMOST: u8 = 0x10;  /* rightmost page */
pub const BT_LEFTMOST: u8 = 0x20;   /* leftmost page */
pub const BT_SWAPPED: u8 = 0x80;    /* used by fsck for endian swapping */

/* btorder (in inode) */
pub const BT_RANDOM: u16 = 0x0000;
pub const BT_SEQUENTIAL: u16 = 0x0001;
pub const BT_LOOKUP: u16 = 0x0010;
pub const BT_INSERT: u16 = 0x0020;
pub const BT_DELETE: u16 = 0x0040;

/*
 * btree page buffer cache access
 *
 * External types, constants, and functions referenced here are supplied by
 * the surrounding translation unit.
 */
#[macro_export]
macro_rules! BT_IS_ROOT {
    ($mp:expr) => {
        (($mp).xflag & COMMIT_PAGE) == 0
    };
}

#[macro_export]
macro_rules! BT_PAGE {
    ($ip:expr, $mp:expr, $type:ty, $root:ident) => {
        if BT_IS_ROOT!($mp) {
            &mut JFS_IP!($ip).$root as *mut _ as *mut $type
        } else {
            ($mp).data as *mut _ as *mut $type
        }
    };
}

#[macro_export]
macro_rules! BT_GETPAGE {
    ($ip:expr, $bn:expr, $mp:ident, $type:ty, $size:expr, $p:ident, $rc:ident, $root:ident) => {{
        if ($bn) == 0 {
            $mp = &mut JFS_IP!($ip).bxflag as *mut _ as *mut struct metapage;
            $p = &mut JFS_IP!($ip).$root as *mut _ as *mut $type;
            $rc = 0;
        } else {
            $mp = read_metapage(($ip), ($bn), ($size), 1);
            if !$mp.is_null() {
                $rc = 0;
                $p = (*$mp).data as *mut _ as *mut $type;
            } else {
                $p = core::ptr::null_mut();
                jfs_err!("bread failed!");
                $rc = -EIO;
            }
        }
    }};
}

#[macro_export]
macro_rules! BT_MARK_DIRTY {
    ($mp:expr, $ip:expr) => {{
        if BT_IS_ROOT!($mp) {
            mark_inode_dirty($ip);
        } else {
            mark_metapage_dirty($mp);
        }
    }};
}

#[macro_export]
macro_rules! BT_PUTPAGE {
    ($mp:expr) => {{
        if !BT_IS_ROOT!($mp) {
            release_metapage($mp);
        }
    }};
}

/*
 * btree traversal stack
 *
 * record the path traversed during the search;
 * top frame record the leaf page/entry selected.
 */
#[repr(C)]
pub struct btframe {
    pub bn: i64,
    pub index: i16,
    pub lastindex: i16,
    pub mp: *mut metapage,
}

#[repr(C)]
pub struct btstack {
    pub top: *mut btframe,
    pub nsplit: i32,
    pub stack: [btframe; MAXTREEHEIGHT],
}

#[macro_export]
macro_rules! BT_CLR {
    ($btstack:expr) => {
        ($btstack).top = ($btstack).stack.as_mut_ptr()
    };
}

#[macro_export]
macro_rules! BT_STACK_FULL {
    ($btstack:expr) => {
        ($btstack).top == unsafe { ($btstack).stack.as_mut_ptr().add(MAXTREEHEIGHT - 1) }
    };
}

#[macro_export]
macro_rules! BT_PUSH {
    ($btstack:expr, $bn:expr, $index:expr) => {{
        assert!(!BT_STACK_FULL!($btstack));
        unsafe {
            (*($btstack).top).bn = $bn;
            (*($btstack).top).index = $index;
            ($btstack).top = ($btstack).top.add(1);
        }
    }};
}

#[macro_export]
macro_rules! BT_POP {
    ($btstack:expr) => {
        if ($btstack).top == ($btstack).stack.as_mut_ptr() {
            core::ptr::null_mut()
        } else {
            ($btstack).top = unsafe { ($btstack).top.sub(1) };
            ($btstack).top
        }
    };
}

#[macro_export]
macro_rules! BT_STACK {
    ($btstack:expr) => {
        if ($btstack).top == ($btstack).stack.as_mut_ptr() {
            core::ptr::null_mut()
        } else {
            ($btstack).top
        }
    };
}

pub unsafe fn BT_STACK_DUMP(btstack: *mut btstack) {
    let mut i: i32 = 0;
    printk!("btstack dump:\n");
    while i < MAXTREEHEIGHT {
        printk!(
            KERN_ERR!(),
            "bn = {:Lx}, index = {}\n",
            (*btstack).stack[i as usize].bn,
            (*btstack).stack[i as usize].index
        );
        i += 1;
    }
}

/* retrieve search results */
#[macro_export]
macro_rules! BT_GETSEARCH {
    ($ip:expr, $leaf:expr, $bn:ident, $mp:ident, $type:ty, $p:ident, $index:ident, $root:ident) => {{
        $bn = (*$leaf).bn;
        $mp = (*$leaf).mp;
        if $bn != 0 {
            $p = (*$mp).data as *mut _ as *mut $type;
        } else {
            $p = &mut JFS_IP!($ip).$root as *mut _ as *mut $type;
        }
        $index = (*$leaf).index;
    }};
}

#[macro_export]
macro_rules! BT_PUTSEARCH {
    ($btstack:expr) => {{
        if !BT_IS_ROOT!((*($btstack).top).mp) {
            release_metapage((*($btstack).top).mp);
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
