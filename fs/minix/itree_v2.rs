// SPDX-License-Identifier: GPL-2.0
// Dependency intent: Linux buffer-head definitions and minix definitions are
// supplied by the surrounding kernel translation.

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_blocksize_bits: u32,
    pub s_blocksize: u64,
    pub s_maxbytes: u64,
    pub s_bdev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}

type u32_ = u32;
type block_t = u32_;
type loff_t = i64;

const DIRECT: usize = 7;
const DEPTH: usize = 4; // Have triple indirect
const DIRCOUNT: usize = 7;

// Supplied by minix.h in the original source.
extern "C" {
    fn minix_i(inode: *mut inode) -> *mut minix_inode_info;
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn get_block(
        inode: *mut inode,
        block: i64,
        bh_result: *mut buffer_head,
        create: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn truncate(inode: *mut inode);
    fn nblocks(size: loff_t, sb: *mut super_block) -> u32;
}

#[repr(C)]
pub struct minix_inode_info {
    pub u: minix_inode_union,
}

#[repr(C)]
pub union minix_inode_union {
    pub i2_data: [block_t; 16],
}

#[inline]
unsafe fn block_to_cpu(n: block_t) -> u64 {
    n as u64
}

#[inline]
unsafe fn cpu_to_block(n: u64) -> block_t {
    n as block_t
}

#[inline]
unsafe fn i_data(inode: *mut inode) -> *mut block_t {
    (*minix_i(inode)).u.i2_data.as_mut_ptr()
}

#[inline]
unsafe fn indircount(sb: *mut super_block) -> usize {
    1usize << ((*sb).s_blocksize_bits - 2)
}

unsafe fn block_to_path(inode: *mut inode, mut block: i64, offsets: *mut i32) -> i32 {
    let mut n: i32 = 0;
    let sb = (*inode).i_sb;

    if block < 0 {
        // Original printk format: MINIX-fs: block %ld < 0 on dev %pg
        printk(
            b"MINIX-fs: block %ld < 0 on dev %pg\0".as_ptr() as *const core::ffi::c_char,
            block,
            (*sb).s_bdev,
        );
        return 0;
    }
    if (block as u64).wrapping_mul((*sb).s_blocksize) >= (*sb).s_maxbytes {
        return 0;
    }

    let count = indircount(sb) as i64;
    if block < DIRCOUNT as i64 {
        *offsets.add(n as usize) = block as i32;
        n += 1;
    } else if {
        block -= DIRCOUNT as i64;
        block < count
    } {
        *offsets.add(n as usize) = DIRCOUNT as i32;
        n += 1;
        *offsets.add(n as usize) = block as i32;
        n += 1;
    } else if {
        block -= count;
        block < count.wrapping_mul(count)
    } {
        *offsets.add(n as usize) = (DIRCOUNT + 1) as i32;
        n += 1;
        *offsets.add(n as usize) = (block / count) as i32;
        n += 1;
        *offsets.add(n as usize) = (block % count) as i32;
        n += 1;
    } else {
        block -= count.wrapping_mul(count);
        *offsets.add(n as usize) = (DIRCOUNT + 2) as i32;
        n += 1;
        *offsets.add(n as usize) = ((block / count) / count) as i32;
        n += 1;
        *offsets.add(n as usize) = ((block / count) % count) as i32;
        n += 1;
        *offsets.add(n as usize) = (block % count) as i32;
        n += 1;
    }
    n
}

pub unsafe fn V2_minix_get_block(
    inode: *mut inode,
    block: i64,
    bh_result: *mut buffer_head,
    create: i32,
) -> i32 {
    get_block(inode, block, bh_result, create)
}

pub unsafe fn V2_minix_truncate(inode: *mut inode) {
    truncate(inode);
}

pub unsafe fn V2_minix_blocks(size: loff_t, sb: *mut super_block) -> u32 {
    nblocks(size, sb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
