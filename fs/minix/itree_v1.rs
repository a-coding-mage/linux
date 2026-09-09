// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/buffer_head.h>, <linux/slab.h>, and "minix.h".

pub const DEPTH: usize = 3;
pub const DIRECT: usize = 7; // Only double indirect

pub type BlockT = u16; // 16 bit, host order

#[inline]
fn block_to_cpu(n: BlockT) -> libc::c_ulong {
    n as libc::c_ulong
}

#[inline]
fn cpu_to_block(n: libc::c_ulong) -> BlockT {
    n as BlockT
}

// Supplied by the surrounding kernel/minix translation unit.
extern "C" {
    fn minix_i(inode: *mut inode) -> *mut minix_inode_info;
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_bdev: *mut core::ffi::c_void,
    pub s_maxbytes: u64,
}

#[repr(C)]
pub struct minix_inode_info {
    pub u: minix_inode_union,
}

#[repr(C)]
pub union minix_inode_union {
    pub i1_data: [BlockT; 16],
}

#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}

#[inline]
unsafe fn i_data(inode: *mut inode) -> *mut BlockT {
    (*minix_i(inode)).u.i1_data.as_mut_ptr()
}

unsafe fn block_to_path(inode: *mut inode, mut block: libc::c_long,
                        offsets: *mut libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;

    if block < 0 {
        // printk("MINIX-fs: block_to_path: block %ld < 0 on dev %pg\n", ...)
        return 0;
    }
    if (block as u64).wrapping_mul(BLOCK_SIZE as u64) >= (*(*inode).i_sb).s_maxbytes {
        return 0;
    }

    if block < 7 {
        *offsets.add(n as usize) = block as libc::c_int;
        n += 1;
    } else if {
        block -= 7;
        block < 512
    } {
        *offsets.add(n as usize) = 7;
        n += 1;
        *offsets.add(n as usize) = block as libc::c_int;
        n += 1;
    } else {
        block -= 512;
        *offsets.add(n as usize) = 8;
        n += 1;
        *offsets.add(n as usize) = (block >> 9) as libc::c_int;
        n += 1;
        *offsets.add(n as usize) = (block & 511) as libc::c_int;
        n += 1;
    }
    n
}

// "itree_common.c" is translated/provided by the surrounding source set.
extern "C" {
    fn get_block(inode: *mut inode, block: libc::c_long,
                 bh_result: *mut buffer_head, create: libc::c_int) -> libc::c_int;
    fn truncate(inode: *mut inode);
    fn nblocks(size: libc::c_longlong, sb: *mut super_block) -> libc::c_uint;
}

pub const BLOCK_SIZE: usize = 1024;

#[no_mangle]
pub unsafe extern "C" fn V1_minix_get_block(inode: *mut inode, block: libc::c_long,
                                             bh_result: *mut buffer_head,
                                             create: libc::c_int) -> libc::c_int {
    get_block(inode, block, bh_result, create)
}

#[no_mangle]
pub unsafe extern "C" fn V1_minix_truncate(inode: *mut inode) {
    truncate(inode)
}

#[no_mangle]
pub unsafe extern "C" fn V1_minix_blocks(size: libc::c_longlong,
                                          sb: *mut super_block) -> libc::c_uint {
    nblocks(size, sb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
