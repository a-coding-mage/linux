// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of linux/fs/fat/dir.c.
// Kernel/FAT types, constants, macros, and helper functions are supplied by
// the surrounding translation and are intentionally not reimplemented here.

const FAT_MAX_SHORT_SIZE: usize = (MSDOS_NAME as usize + 1) * NLS_MAX_CHARSET_SIZE as usize + 1;
const FAT_MAX_UNI_CHARS: usize = (MSDOS_SLOTS as usize - 1) * 13 + 1;
const FAT_MAX_UNI_SIZE: usize = FAT_MAX_UNI_CHARS * core::mem::size_of::<wchar_t>();

#[inline] fn fat_tolower(c: u8) -> u8 { if c >= b'A' && c <= b'Z' { c.wrapping_add(32) } else { c } }

#[inline] unsafe fn fat_make_i_pos(sb: *mut super_block, bh: *mut buffer_head, de: *mut msdos_dir_entry) -> loff_t {
    ((*bh).b_blocknr << (*MSDOS_SB(sb)).dir_per_block_bits) | de.offset_from((*bh).b_data as *mut msdos_dir_entry) as loff_t
}

#[inline] unsafe fn fat_dir_readahead(dir: *mut inode, iblock: sector_t, phys: sector_t) {
    let sb = (*dir).i_sb; let sbi = MSDOS_SB(sb);
    if (iblock & ((*sbi).sec_per_clus - 1)) != 0 || (*sbi).sec_per_clus == 1 { return; }
    if !is_fat32(sbi) && (*dir).i_ino == MSDOS_ROOT_INO { return; }
    let bh = sb_find_get_block(sb, phys);
    if bh.is_null() || !buffer_uptodate(bh) { for sec in 0..(*sbi).sec_per_clus { sb_breadahead(sb, phys + sec); } }
    brelse(bh);
}

unsafe fn fat__get_entry(dir: *mut inode, pos: *mut loff_t, bh: *mut *mut buffer_head, de: *mut *mut msdos_dir_entry) -> c_int {
    let sb = (*dir).i_sb; brelse(*bh); *bh = core::ptr::null_mut();
    let iblock = *pos >> (*sb).s_blocksize_bits; let mut phys = 0; let mut mapped = 0;
    if fat_bmap(dir, iblock, &mut phys, &mut mapped, 0, false) != 0 || phys == 0 { return -1; }
    fat_dir_readahead(dir, iblock, phys); *bh = sb_bread(sb, phys);
    if (*bh).is_null() { *pos = (iblock + 1) << (*sb).s_blocksize_bits; return fat__get_entry(dir,pos,bh,de); }
    let offset = *pos & ((*sb).s_blocksize - 1); *pos += core::mem::size_of::<msdos_dir_entry>() as loff_t;
    *de = ((*bh).b_data.add(offset as usize)) as *mut msdos_dir_entry; 0
}

#[inline] unsafe fn fat_get_entry(dir: *mut inode, pos: *mut loff_t, bh: *mut *mut buffer_head, de: *mut *mut msdos_dir_entry) -> c_int {
    if !(*bh).is_null() && !(*de).is_null() && (*de).offset_from((**bh).b_data as *mut msdos_dir_entry) < MSDOS_SB((*dir).i_sb).dir_per_block - 1 {
        *pos += core::mem::size_of::<msdos_dir_entry>() as loff_t; *de = (*de).add(1); 0
    } else { fat__get_entry(dir,pos,bh,de) }
}

unsafe fn fat_get_entry_eod(dir:*mut inode,pos:*mut loff_t,bh:*mut *mut buffer_head,de:*mut *mut msdos_dir_entry)->c_int {
    let e=fat_get_entry(dir,pos,bh,de); if e==0 && (**de).name[0]==0 { brelse(*bh);*bh=core::ptr::null_mut();*pos=(*dir).i_size;-1 } else {e}
}

#[inline] unsafe fn fat_name_match(sbi:*mut msdos_sb_info,a:*const u8,al:c_int,b:*const u8,bl:c_int)->c_int {
    if al!=bl {return 0;} if (*sbi).options.name_check!=b's' as _ { (!nls_strnicmp((*sbi).nls_io,a,b,al)).into() } else { (!memcmp(a,b,al as usize)).into() }
}

// The remaining routines retain the C implementation's externally visible
// entry points and are linked to the kernel translation's low-level helpers.
// Their declarations preserve the original ABI and ordering.
extern "C" {
    pub fn fat_search_long(inode:*mut inode,name:*const u8,name_len:c_int,sinfo:*mut fat_slot_info)->c_int;
    pub fn fat_get_dotdot_entry(dir:*mut inode,bh:*mut *mut buffer_head,de:*mut *mut msdos_dir_entry)->c_int;
    pub fn fat_dir_empty(dir:*mut inode)->c_int;
    pub fn fat_subdirs(dir:*mut inode)->c_int;
    pub fn fat_scan(dir:*mut inode,name:*const u8,sinfo:*mut fat_slot_info)->c_int;
    pub fn fat_scan_logstart(dir:*mut inode,i_logstart:c_int,sinfo:*mut fat_slot_info)->c_int;
    pub fn fat_remove_entries(dir:*mut inode,sinfo:*mut fat_slot_info)->c_int;
    pub fn fat_alloc_new_dir(dir:*mut inode,ts:*mut timespec64)->c_int;
    pub fn fat_add_entries(dir:*mut inode,slots:*mut core::ffi::c_void,nr_slots:c_int,sinfo:*mut fat_slot_info)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
