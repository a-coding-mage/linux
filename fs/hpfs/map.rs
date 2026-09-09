// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/map.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  mapping structures to memory with some minimal checks
 */

// Declarations supplied by hpfs_fn.h and the rest of the translated kernel.

pub unsafe fn hpfs_map_dnode_bitmap(s: *mut super_block, qbh: *mut quad_buffer_head) -> *mut __le32 {
    hpfs_map_4sectors(s, (*hpfs_sb(s)).sb_dmap, qbh, 0)
}

pub unsafe fn hpfs_map_bitmap(s: *mut super_block, bmp_block: u32,
                              qbh: *mut quad_buffer_head, id: *mut i8) -> *mut __le32 {
    let n_bands = ((*hpfs_sb(s)).sb_fs_size + 0x3fff) >> 14;
    if (*hpfs_sb(s)).sb_chk != 0 && bmp_block >= n_bands {
        hpfs_error(s, b"hpfs_map_bitmap called with bad parameter: %08x at %s\0".as_ptr() as *const i8, bmp_block, id);
        return core::ptr::null_mut();
    }
    let sec = le32_to_cpu((*hpfs_sb(s)).sb_bmp_dir[bmp_block as usize]);
    if sec == 0 || sec > (*hpfs_sb(s)).sb_fs_size - 4 {
        hpfs_error(s, b"invalid bitmap block pointer %08x -> %08x at %s\0".as_ptr() as *const i8, bmp_block, sec, id);
        return core::ptr::null_mut();
    }
    let ret = hpfs_map_4sectors(s, sec, qbh, 4);
    if !ret.is_null() { hpfs_prefetch_bitmap(s, bmp_block + 1); }
    ret
}

pub unsafe fn hpfs_prefetch_bitmap(s: *mut super_block, bmp_block: u32) {
    let n_bands = ((*hpfs_sb(s)).sb_fs_size + 0x3fff) >> 14;
    if bmp_block >= n_bands { return; }
    let to_prefetch = le32_to_cpu((*hpfs_sb(s)).sb_bmp_dir[bmp_block as usize]);
    let next_prefetch = if bmp_block + 1 >= n_bands { 0 } else {
        le32_to_cpu((*hpfs_sb(s)).sb_bmp_dir[(bmp_block + 1) as usize])
    };
    hpfs_prefetch_sectors(s, to_prefetch, 4 + 4 * (to_prefetch + 4 == next_prefetch) as u32);
}

/* Load first code page into kernel memory, returning the upper/lowercase table. */
pub unsafe fn hpfs_load_code_page(s: *mut super_block, cps: secno) -> *mut u8 {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let cp = hpfs_map_sector(s, cps, &mut bh, 0) as *mut code_page_directory;
    if cp.is_null() { return core::ptr::null_mut(); }
    if le32_to_cpu((*cp).magic) != CP_DIR_MAGIC {
        pr_err(b"Code page directory magic doesn't match (magic = %08x)\n\0".as_ptr() as *const i8, le32_to_cpu((*cp).magic)); brelse(bh); return core::ptr::null_mut();
    }
    if le32_to_cpu((*cp).n_code_pages) == 0 { pr_err(b"n_code_pages == 0\n\0".as_ptr() as *const i8); brelse(bh); return core::ptr::null_mut(); }
    let cpds = le32_to_cpu((*cp).array[0].code_page_data);
    let cpi = le16_to_cpu((*cp).array[0].index) as usize;
    brelse(bh);
    if cpi >= 3 { pr_err(b"Code page index out of array\n\0".as_ptr() as *const i8); return core::ptr::null_mut(); }
    let cpd = hpfs_map_sector(s, cpds, &mut bh, 0) as *mut code_page_data;
    if cpd.is_null() { return core::ptr::null_mut(); }
    if le16_to_cpu((*cpd).offs[cpi]) > 0x178 { pr_err(b"Code page index out of sector\n\0".as_ptr() as *const i8); brelse(bh); return core::ptr::null_mut(); }
    let ptr = (cpd as *mut u8).add(le16_to_cpu((*cpd).offs[cpi]) as usize + 6);
    let cp_table = kmalloc(256, GFP_KERNEL) as *mut u8;
    if cp_table.is_null() { pr_err(b"out of memory for code page table\n\0".as_ptr() as *const i8); brelse(bh); return core::ptr::null_mut(); }
    memcpy(cp_table as *mut core::ffi::c_void, ptr as *const core::ffi::c_void, 128); brelse(bh);
    for i in 128..256 { *cp_table.add(i) = i as u8; }
    for i in 128..256 { let v = *cp_table.add(i - 128); if v != i as u8 && v >= 128 { *cp_table.add(v as usize) = i as u8; } }
    cp_table
}

pub unsafe fn hpfs_load_bitmap_directory(s: *mut super_block, bmp: secno) -> *mut __le32 {
    let n = (((*hpfs_sb(s)).sb_fs_size + 0x200000 - 1) >> 21) as usize;
    let b = kmalloc_array(n, 512, GFP_KERNEL) as *mut __le32;
    if b.is_null() { pr_err(b"can't allocate memory for bitmap directory\n\0".as_ptr() as *const i8); return core::ptr::null_mut(); }
    for i in 0..n { let mut bh = core::ptr::null_mut(); let d = hpfs_map_sector(s, bmp + i as u32, &mut bh, (n-i-1) as u32) as *mut __le32; if d.is_null() { kfree(b as *mut core::ffi::c_void); return core::ptr::null_mut(); } memcpy((b as *mut u8).add(512*i) as *mut core::ffi::c_void, d as *const core::ffi::c_void, 512); brelse(bh); }
    b
}

pub unsafe fn hpfs_load_hotfix_map(s: *mut super_block, spareblock: *mut hpfs_spare_block) {
    let mut qbh = core::mem::MaybeUninit::<quad_buffer_head>::uninit();
    let n_hotfixes = le32_to_cpu((*spareblock).n_spares); let n_used_hotfixes = le32_to_cpu((*spareblock).n_spares_used);
    if n_hotfixes > 256 || n_used_hotfixes > n_hotfixes { hpfs_error(s, b"invalid number of hotfixes: %u, used: %u\0".as_ptr() as *const i8, n_hotfixes, n_used_hotfixes); return; }
    let directory = hpfs_map_4sectors(s, le32_to_cpu((*spareblock).hotfix_map), qbh.as_mut_ptr(), 0); if directory.is_null() { hpfs_error(s, b"can't load hotfix map\0".as_ptr() as *const i8); return; }
    for i in 0..n_used_hotfixes as usize { (*hpfs_sb(s)).hotfix_from[i] = le32_to_cpu(*directory.add(i)); (*hpfs_sb(s)).hotfix_to[i] = le32_to_cpu(*directory.add(n_hotfixes as usize+i)); }
    (*hpfs_sb(s)).n_hotfixes = n_used_hotfixes; hpfs_brelse4(qbh.as_mut_ptr());
}

pub unsafe fn hpfs_map_fnode(s: *mut super_block, ino: ino_t, bhp: *mut *mut buffer_head) -> *mut fnode {
    if (*hpfs_sb(s)).sb_chk != 0 && hpfs_chk_sectors(s, ino, 1, b"fnode\0".as_ptr() as *const i8) != 0 { return core::ptr::null_mut(); }
    let fnode = hpfs_map_sector(s, ino, bhp, FNODE_RD_AHEAD) as *mut fnode;
    if !fnode.is_null() && (*hpfs_sb(s)).sb_chk != 0 { if le32_to_cpu((*fnode).magic) != FNODE_MAGIC { hpfs_error(s, b"bad magic on fnode %08lx\0".as_ptr() as *const i8, ino); brelse(*bhp); return core::ptr::null_mut(); } }
    fnode
}

pub unsafe fn hpfs_map_anode(s: *mut super_block, ano: anode_secno, bhp: *mut *mut buffer_head) -> *mut anode {
    if (*hpfs_sb(s)).sb_chk != 0 && hpfs_chk_sectors(s, ano, 1, b"anode\0".as_ptr() as *const i8) != 0 { return core::ptr::null_mut(); }
    let anode = hpfs_map_sector(s, ano, bhp, ANODE_RD_AHEAD) as *mut anode;
    if !anode.is_null() && (*hpfs_sb(s)).sb_chk != 0 && le32_to_cpu((*anode).magic) != ANODE_MAGIC { hpfs_error(s, b"bad magic on anode %08x\0".as_ptr() as *const i8, ano); brelse(*bhp); return core::ptr::null_mut(); }
    anode
}

pub unsafe fn hpfs_map_dnode(s: *mut super_block, secno: u32, qbh: *mut quad_buffer_head) -> *mut dnode {
    if (*hpfs_sb(s)).sb_chk != 0 { if hpfs_chk_sectors(s, secno, 4, b"dnode\0".as_ptr() as *const i8) != 0 || secno & 3 != 0 { hpfs_error(s, b"dnode %08x not byte-aligned\0".as_ptr() as *const i8, secno); return core::ptr::null_mut(); } }
    let dnode = hpfs_map_4sectors(s, secno, qbh, DNODE_RD_AHEAD) as *mut dnode;
    if !dnode.is_null() && (*hpfs_sb(s)).sb_chk != 0 && le32_to_cpu((*dnode).magic) != DNODE_MAGIC { hpfs_error(s, b"bad magic on dnode %08x\0".as_ptr() as *const i8, secno); hpfs_brelse4(qbh); return core::ptr::null_mut(); }
    dnode
}

pub unsafe fn hpfs_fnode_dno(s: *mut super_block, ino: ino_t) -> dnode_secno {
    let mut bh = core::ptr::null_mut(); let fnode = hpfs_map_fnode(s, ino, &mut bh); if fnode.is_null() { return 0; }
    let dno = le32_to_cpu((*fnode).u.external[0].disk_secno); brelse(bh); dno
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
