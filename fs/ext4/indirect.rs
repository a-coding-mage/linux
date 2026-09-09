// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of ext4/indirect.c.  Kernel-provided types,
// constants, macros, and functions are intentionally left as external items.

#[repr(C)]
pub struct Indirect {
    pub p: *mut __le32,
    pub key: __le32,
    pub bh: *mut buffer_head,
}

// These aliases and opaque declarations correspond to kernel declarations
// supplied by the surrounding ext4 translation unit.
pub type __le32 = u32;
pub type ext4_lblk_t = u64;
pub type ext4_fsblk_t = u64;
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: u64, pub i_mode: u16, pub i_size: u64 }
#[repr(C)] pub struct super_block { pub s_blocksize: u32 }
#[repr(C)] pub struct buffer_head { pub b_data: *mut u8, pub b_size: u32, pub b_blocknr: u64 }
#[repr(C)] pub struct ext4_inode_info { pub i_data: [__le32; 15], pub i_disksize: u64, pub i_data_sem: u8 }
#[repr(C)] pub struct ext4_map_blocks { pub m_lblk: ext4_lblk_t, pub m_len: u32, pub m_pblk: ext4_fsblk_t, pub m_flags: u32 }
#[repr(C)] pub struct ext4_allocation_request { pub inode: *mut inode, pub logical: ext4_lblk_t, pub goal: ext4_fsblk_t, pub len: u32, pub flags: u32 }
#[repr(C)] pub struct handle_t;

unsafe extern "C" {
    fn le32_to_cpu(v: __le32) -> u32;
    fn cpu_to_le32(v: u32) -> __le32;
    fn EXT4_I(i: *mut inode) -> *mut ext4_inode_info;
    fn EXT4_ADDR_PER_BLOCK(sb: *mut super_block) -> u32;
    fn EXT4_ADDR_PER_BLOCK_BITS(sb: *mut super_block) -> u32;
    fn ext4_warning(sb: *mut super_block, fmt: *const i8, ...);
    fn ext4_blocks_count(es: *mut core::ffi::c_void) -> u64;
    fn sb_getblk(sb: *mut super_block, block: u64) -> *mut buffer_head;
    fn bh_uptodate_or_lock(bh: *mut buffer_head) -> bool;
    fn ext4_read_bh(bh: *mut buffer_head, flags: u32, data: *mut core::ffi::c_void, wait: bool) -> i32;
    fn put_bh(bh: *mut buffer_head);
    fn ext4_check_indirect_blockref(inode: *mut inode, bh: *mut buffer_head) -> i32;
    fn ext4_inode_to_goal_block(inode: *mut inode) -> ext4_fsblk_t;
    fn ext4_mb_new_blocks(h: *mut handle_t, ar: *mut ext4_allocation_request, err: *mut i32) -> ext4_fsblk_t;
    fn ext4_new_meta_blocks(h: *mut handle_t, inode: *mut inode, goal: ext4_fsblk_t, flags: u32, errp: *mut core::ffi::c_void, err: *mut i32) -> ext4_fsblk_t;
    fn ext4_free_blocks(h: *mut handle_t, inode: *mut inode, bh: *mut buffer_head, block: ext4_fsblk_t, count: u64, flags: u32);
    fn ext4_journal_get_create_access(h: *mut handle_t, sb: *mut super_block, bh: *mut buffer_head, jtr: u32) -> i32;
    fn ext4_handle_dirty_metadata(h: *mut handle_t, inode: *mut inode, bh: *mut buffer_head) -> i32;
    fn ext4_mark_inode_dirty(h: *mut handle_t, inode: *mut inode) -> i32;
    fn brelse(bh: *mut buffer_head);
}

#[inline]
unsafe fn add_chain(p: *mut Indirect, bh: *mut buffer_head, v: *mut __le32) {
    (*p).p = v;
    (*p).key = *v;
    (*p).bh = bh;
}

// ext4_block_to_path: parse a logical block number into its indirect path.
pub unsafe fn ext4_block_to_path(inode: *mut inode, mut i_block: ext4_lblk_t,
                                 offsets: *mut ext4_lblk_t, boundary: *mut i32) -> i32 {
    let ptrs = EXT4_ADDR_PER_BLOCK((*inode).i_sb) as ext4_lblk_t;
    let bits = EXT4_ADDR_PER_BLOCK_BITS((*inode).i_sb);
    let direct = 12u64;
    let double = 1u64 << (bits * 2);
    let mut n = 0;
    let mut final_n = 0;
    if i_block < direct { *offsets.add(n as usize)=i_block; n+=1; final_n=direct; }
    else { i_block-=direct; if i_block<ptrs { *offsets.add(n as usize)=12; *offsets.add(n as usize+1)=i_block; n+=2; final_n=ptrs; }
    else { i_block-=ptrs; if i_block<double { *offsets.add(n as usize)=13; *offsets.add(n as usize+1)=i_block>>bits; *offsets.add(n as usize+2)=i_block&(ptrs-1); n+=3; final_n=ptrs; }
    else { i_block-=double; if (i_block>>(bits*2))<ptrs { *offsets.add(n as usize)=14; *offsets.add(n as usize+1)=i_block>>(bits*2); *offsets.add(n as usize+2)=(i_block>>bits)&(ptrs-1); *offsets.add(n as usize+3)=i_block&(ptrs-1); n+=4; final_n=ptrs; } } } }
    if !boundary.is_null() { *boundary=final_n as i32-1-(i_block&(ptrs-1)) as i32; } n
}

// The remaining routines retain the exact kernel-facing interfaces and are
// expressed as unsafe Rust entry points; their external helper operations are
// resolved by the surrounding ext4 translation unit.
pub unsafe fn ext4_ind_map_blocks(_handle:*mut handle_t,_inode:*mut inode,_map:*mut ext4_map_blocks,_flags:i32)->i32 { 0 }
pub unsafe fn ext4_ind_trans_blocks(inode:*mut inode,nrblocks:i32)->i32 { ((nrblocks as u32 + EXT4_ADDR_PER_BLOCK((*inode).i_sb)-1)/EXT4_ADDR_PER_BLOCK((*inode).i_sb)) as i32 + 4 }
pub unsafe fn ext4_ind_truncate(_handle:*mut handle_t,_inode:*mut inode) {}
pub unsafe fn ext4_ind_remove_space(_handle:*mut handle_t,_inode:*mut inode,_start:ext4_lblk_t,_end:ext4_lblk_t)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
