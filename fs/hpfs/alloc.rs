// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/alloc.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  HPFS bitmap operations
 */

unsafe fn hpfs_claim_alloc(s: *mut super_block, sec: secno) {
    let sbi = hpfs_sb(s);
    if (*sbi).sb_n_free != (-1i32 as _) {
        if (*sbi).sb_n_free == 0 {
            hpfs_error(s, c"free count underflow, allocating sector %08x", sec);
            (*sbi).sb_n_free = -1i32 as _;
            return;
        }
        (*sbi).sb_n_free -= 1;
    }
}

unsafe fn hpfs_claim_free(s: *mut super_block, sec: secno) {
    let sbi = hpfs_sb(s);
    if (*sbi).sb_n_free != (-1i32 as _) {
        if (*sbi).sb_n_free >= (*sbi).sb_fs_size {
            hpfs_error(s, c"free count overflow, freeing sector %08x", sec);
            (*sbi).sb_n_free = -1i32 as _;
            return;
        }
        (*sbi).sb_n_free += 1;
    }
}

unsafe fn hpfs_claim_dirband_alloc(s: *mut super_block, sec: secno) {
    let sbi = hpfs_sb(s);
    if (*sbi).sb_n_free_dnodes != (-1i32 as _) {
        if (*sbi).sb_n_free_dnodes == 0 {
            hpfs_error(s, c"dirband free count underflow, allocating sector %08x", sec);
            (*sbi).sb_n_free_dnodes = -1i32 as _;
            return;
        }
        (*sbi).sb_n_free_dnodes -= 1;
    }
}

unsafe fn hpfs_claim_dirband_free(s: *mut super_block, sec: secno) {
    let sbi = hpfs_sb(s);
    if (*sbi).sb_n_free_dnodes != (-1i32 as _) {
        if (*sbi).sb_n_free_dnodes >= (*sbi).sb_dirband_size / 4 {
            hpfs_error(s, c"dirband free count overflow, freeing sector %08x", sec);
            (*sbi).sb_n_free_dnodes = -1i32 as _;
            return;
        }
        (*sbi).sb_n_free_dnodes += 1;
    }
}

unsafe fn chk_if_allocated(s: *mut super_block, sec: secno, msg: *mut i8) -> i32 {
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let mut bmp = hpfs_map_bitmap(s, sec >> 14, &mut qbh, c"chk".as_ptr());
    if bmp.is_null() { return 1; }
    if (le32_to_cpu(*bmp.add(((sec & 0x3fff) >> 5) as usize)) >> (sec & 0x1f)) & 1 != 0 {
        hpfs_error(s, c"sector '%s' - %08x not allocated in bitmap", msg, sec);
        hpfs_brelse4(&mut qbh);
        return 1;
    }
    hpfs_brelse4(&mut qbh);
    let sbi = hpfs_sb(s);
    if sec >= (*sbi).sb_dirband_start && sec < (*sbi).sb_dirband_start + (*sbi).sb_dirband_size {
        let ssec = (sec - (*sbi).sb_dirband_start) / 4;
        bmp = hpfs_map_dnode_bitmap(s, &mut qbh);
        if bmp.is_null() { return 1; }
        if (le32_to_cpu(*bmp.add((ssec >> 5) as usize)) >> (ssec & 0x1f)) & 1 != 0 {
            hpfs_error(s, c"sector '%s' - %08x not allocated in directory bitmap", msg, sec);
            hpfs_brelse4(&mut qbh);
            return 1;
        }
        hpfs_brelse4(&mut qbh);
    }
    0
}

pub unsafe fn hpfs_chk_sectors(s: *mut super_block, start: secno, len: i32, msg: *mut i8) -> i32 {
    if start.wrapping_add(len as _) < start || start < 0x12 || start.wrapping_add(len as _) > (*hpfs_sb(s)).sb_fs_size {
        hpfs_error(s, c"sector(s) '%s' badly placed at %08x", msg, start);
        return 1;
    }
    if (*hpfs_sb(s)).sb_chk >= 2 {
        for i in 0..len { if chk_if_allocated(s, start + i as _, msg) != 0 { return 1; } }
    }
    0
}

unsafe fn alloc_in_bmp(s: *mut super_block, near: secno, n: u32, forward: u32) -> secno {
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let bs = near & !0x3fff;
    let mut nr = (near & 0x3fff) & !(n - 1);
    if n != 1 && n != 4 { hpfs_error(s, c"Bad allocation size: %d", n); return 0; }
    let bmp = if bs != !0x3fff { hpfs_map_bitmap(s, near >> 14, &mut qbh, c"aib".as_ptr()) } else { hpfs_map_dnode_bitmap(s, &mut qbh) };
    if bmp.is_null() { return 0; }
    let mut ret = 0;
    if tstbits(bmp, nr, n + forward) == 0 { ret = bs + nr; }
    else {
        let mut q = nr + n; let mut b = 0; let mut a;
        while { a = tstbits(bmp, q, n + forward); a != 0 } {
            q += a; if n != 1 { q = ((q - 1) & !(n - 1)) + n; }
            if b == 0 { if q >> 5 != nr >> 5 { b = 1; q = nr & 0x1f; } } else if q > nr { break; }
        }
        if a == 0 { ret = bs + q; }
        else {
            nr >>= 5; let first = nr; let mut i = nr;
            loop {
                if le32_to_cpu(*bmp.add(i as usize)) != 0 {
                    q = i << 5;
                    if i > 0 { let mut k = le32_to_cpu(*bmp.add((i - 1) as usize)); while k & 0x80000000 != 0 { q -= 1; k <<= 1; } }
                    if n != 1 { q = ((q - 1) & !(n - 1)) + n; }
                    while { a = tstbits(bmp, q, n + forward); a != 0 } { q += a; if n != 1 { q = ((q - 1) & !(n - 1)) + n; } if q >> 5 > i { break; } }
                    if a == 0 { ret = bs + q; break; }
                }
                i = (i + 1) & 0x1ff; if i == first { break; }
            }
        }
    }
    if ret != 0 { *bmp.add(((ret & 0x3fff) >> 5) as usize) &= cpu_to_le32(!(((1 << n) - 1) << (ret & 0x1f))); hpfs_mark_4buffers_dirty(&mut qbh); }
    hpfs_brelse4(&mut qbh); ret
}

pub unsafe fn hpfs_alloc_sector(s: *mut super_block, near: secno, n: u32, mut forward: i32) -> secno {
    let sbi = hpfs_sb(s); let f_p = forward < 0; if f_p { forward = -forward; }
    let n_bmps = ((*sbi).sb_fs_size + 0x4000 - 1) >> 14; let mut near_bmp = n_bmps / 2; let mut sec = 0;
    if near != 0 && near < (*sbi).sb_fs_size { sec = alloc_in_bmp(s, near, n, if f_p { forward as _ } else { (forward / 4) as _ }); if sec != 0 { return sec; } near_bmp = near >> 14; }
    if !f_p && forward as u32 > (*sbi).sb_max_fwd_alloc { forward = (*sbi).sb_max_fwd_alloc as i32; }
    'less_fwd: loop {
        for i in 0..n_bmps {
            let x = near_bmp + i; if x < n_bmps { sec = alloc_in_bmp(s, x << 14, n, forward as _); if sec != 0 { (*sbi).sb_c_bitmap = x; break 'less_fwd; } }
            if !f_p && near_bmp >= i + 1 { sec = alloc_in_bmp(s, (near_bmp - i - 1) << 14, n, forward as _); if sec != 0 { (*sbi).sb_c_bitmap = near_bmp - i - 1; break 'less_fwd; } }
            if f_p && x >= n_bmps { sec = alloc_in_bmp(s, (x - n_bmps) << 14, n, forward as _); if sec != 0 { (*sbi).sb_c_bitmap = x - n_bmps; break 'less_fwd; } }
        }
        if !f_p && forward != 0 { (*sbi).sb_max_fwd_alloc = forward as u32 * 3 / 4; forward /= 2; continue; }
        break;
    }
    if sec != 0 { for i in 0..n { hpfs_claim_alloc(s, sec + i); } }
    if sec != 0 && f_p { for i in 0..forward { if hpfs_alloc_if_possible(s, sec + n + i as _) == 0 { hpfs_error(s, c"Prealloc doesn't work! Wanted %d, allocated at %08x, can't allocate %d", forward, sec, i); return 0; } } }
    sec
}

unsafe fn alloc_in_dirband(s: *mut super_block, near: secno) -> secno {
    let sbi = hpfs_sb(s); let mut nr = near; if nr < (*sbi).sb_dirband_start { nr = (*sbi).sb_dirband_start; } if nr >= (*sbi).sb_dirband_start + (*sbi).sb_dirband_size { nr = (*sbi).sb_dirband_start + (*sbi).sb_dirband_size - 4; }
    nr = (nr - (*sbi).sb_dirband_start) >> 2; let sec = alloc_in_bmp(s, (!0x3fff) | nr, 1, 0); if sec == 0 { return 0; } hpfs_claim_dirband_alloc(s, sec); ((sec & 0x3fff) << 2) + (*sbi).sb_dirband_start
}

pub unsafe fn hpfs_alloc_if_possible(s: *mut super_block, sec: secno) -> i32 {
    let mut qbh: quad_buffer_head = core::mem::zeroed(); let bmp = hpfs_map_bitmap(s, sec >> 14, &mut qbh, c"aip".as_ptr()); if bmp.is_null() { return 0; }
    let p = bmp.add(((sec & 0x3fff) >> 5) as usize); if le32_to_cpu(*p) & (1 << (sec & 0x1f)) != 0 { *p &= cpu_to_le32(!(1 << (sec & 0x1f))); hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh); hpfs_claim_alloc(s, sec); return 1; } hpfs_brelse4(&mut qbh); 0
}

pub unsafe fn hpfs_free_sectors(s: *mut super_block, mut sec: secno, mut n: u32) {
    let sbi = hpfs_sb(s); if n == 0 { return; } if sec < 0x12 { hpfs_error(s, c"Trying to free reserved sector %08x", sec); return; }
    (*sbi).sb_max_fwd_alloc += if n > 0xffff { 0xffff } else { n }; if (*sbi).sb_max_fwd_alloc > 0xffffff { (*sbi).sb_max_fwd_alloc = 0xffffff; }
    loop { let mut qbh: quad_buffer_head = core::mem::zeroed(); let bmp = hpfs_map_bitmap(s, sec >> 14, &mut qbh, c"free".as_ptr()); if bmp.is_null() { return; } loop { let p = bmp.add(((sec & 0x3fff) >> 5) as usize); if le32_to_cpu(*p) >> (sec & 0x1f) & 1 != 0 { hpfs_error(s, c"sector %08x not allocated", sec); hpfs_brelse4(&mut qbh); return; } *p |= cpu_to_le32(1 << (sec & 0x1f)); hpfs_claim_free(s, sec); n -= 1; if n == 0 { hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh); return; } sec += 1; if sec & 0x3fff == 0 { hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh); break; } } }
}

pub unsafe fn hpfs_check_free_dnodes(s: *mut super_block, mut n: i32) -> i32 {
    let sbi = hpfs_sb(s); let mut qbh: quad_buffer_head = core::mem::zeroed();
    let bmp = hpfs_map_dnode_bitmap(s, &mut qbh);
    if !bmp.is_null() { for j in 0..512 { let mut k = le32_to_cpu(*bmp.add(j)); while k != 0 { if k & 1 != 0 { n -= 1; if n == 0 { hpfs_brelse4(&mut qbh); return 0; } } k >>= 1; } } hpfs_brelse4(&mut qbh); }
    let n_bmps = ((*sbi).sb_fs_size + 0x4000 - 1) >> 14; let b = (*sbi).sb_c_bitmap & 0x0fffffff; let mut i = 0;
    loop { if (*sbi).sb_c_bitmap != -1i32 as _ { i = b; } if i == b { i += 1; } if i >= n_bmps { return 1; } let bmp = hpfs_map_bitmap(s, i, &mut qbh, c"chkdn".as_ptr()); if !bmp.is_null() { for j in 0..512 { let v = le32_to_cpu(*bmp.add(j)); if v != 0 { let mut k = 0xf; while k != 0 { if v & k == k { n -= 1; if n == 0 { hpfs_brelse4(&mut qbh); return 0; } } k <<= 4; } } } hpfs_brelse4(&mut qbh); } i += 1; }
}

pub unsafe fn hpfs_free_dnode(s: *mut super_block, dno: dnode_secno) { let sbi = hpfs_sb(s); if (*sbi).sb_chk != 0 && dno & 3 != 0 { hpfs_error(s, c"hpfs_free_dnode: dnode %08x not aligned", dno); return; } if dno < (*sbi).sb_dirband_start || dno >= (*sbi).sb_dirband_start + (*sbi).sb_dirband_size { hpfs_free_sectors(s, dno, 4); } else { let mut qbh: quad_buffer_head = core::mem::zeroed(); let bmp = hpfs_map_dnode_bitmap(s, &mut qbh); if bmp.is_null() { return; } let p = bmp.add(((dno - (*sbi).sb_dirband_start) / 4 >> 5) as usize); *p |= cpu_to_le32(1 << ((dno - (*sbi).sb_dirband_start) / 4 & 0x1f)); hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh); hpfs_claim_dirband_free(s, dno); } }

// Structure initialization follows the original fixed on-disk layouts.
pub unsafe fn hpfs_alloc_dnode(s: *mut super_block, near: secno, dno: *mut dnode_secno, qbh: *mut quad_buffer_head) -> *mut dnode { *dno = alloc_in_dirband(s, near); if *dno == 0 { *dno = hpfs_alloc_sector(s, near, 4, 0); } if *dno == 0 { return core::ptr::null_mut(); } let d = hpfs_get_4sectors(s, *dno, qbh); if d.is_null() { hpfs_free_dnode(s, *dno); return core::ptr::null_mut(); } core::ptr::write_bytes(d as *mut u8, 0, 2048); (*d).magic = cpu_to_le32(DNODE_MAGIC); (*d).first_free = cpu_to_le32(52); (*d).dirent[0] = 32; (*d).dirent[2] = 8; (*d).dirent[30] = 1; (*d).dirent[31] = 255; (*d).self_ = cpu_to_le32(*dno); d }

pub unsafe fn hpfs_alloc_fnode(s: *mut super_block, near: secno, fno: *mut fnode_secno, bh: *mut *mut buffer_head) -> *mut fnode { *fno = hpfs_alloc_sector(s, near, 1, FNODE_ALLOC_FWD); if *fno == 0 { return core::ptr::null_mut(); } let f = hpfs_get_sector(s, *fno, bh); if f.is_null() { hpfs_free_sectors(s, *fno, 1); return core::ptr::null_mut(); } core::ptr::write_bytes(f as *mut u8, 0, 512); (*f).magic = cpu_to_le32(FNODE_MAGIC); (*f).ea_offs = cpu_to_le16(0xc4); (*f).btree.n_free_nodes = 8; (*f).btree.first_free = cpu_to_le16(8); f }

pub unsafe fn hpfs_alloc_anode(s: *mut super_block, near: secno, ano: *mut anode_secno, bh: *mut *mut buffer_head) -> *mut anode { *ano = hpfs_alloc_sector(s, near, 1, ANODE_ALLOC_FWD); if *ano == 0 { return core::ptr::null_mut(); } let a = hpfs_get_sector(s, *ano, bh); if a.is_null() { hpfs_free_sectors(s, *ano, 1); return core::ptr::null_mut(); } core::ptr::write_bytes(a as *mut u8, 0, 512); (*a).magic = cpu_to_le32(ANODE_MAGIC); (*a).self_ = cpu_to_le32(*ano); (*a).btree.n_free_nodes = 40; (*a).btree.n_used_nodes = 0; (*a).btree.first_free = cpu_to_le16(8); a }

unsafe fn find_run(bmp: *mut __le32, idx: &mut u32) -> u32 { while tstbits(bmp, *idx, 1) != 0 { *idx += 1; if *idx >= 0x4000 { return 0; } } let mut len = 1; while tstbits(bmp, *idx + len, 1) == 0 { len += 1; } len }
unsafe fn do_trim(s: *mut super_block, start: secno, len: u32, limit_start: secno, limit_end: secno, minlen: u32, result: &mut u32) -> i32 { if fatal_signal_pending(current) != 0 { return -EINTR; } let mut a = start; let mut e = start + len; if a < limit_start { a = limit_start; } if e > limit_end { e = limit_end; } if a >= e || e-a < minlen { return 0; } let err = sb_issue_discard(s, a, e-a, GFP_NOFS, 0); if err != 0 { return err; } *result += e-a; 0 }

pub unsafe fn hpfs_trim_fs(s: *mut super_block, start: u64, end: u64, minlen: u64, result: *mut u32) -> i32 { *result = 0; if start >= (*hpfs_sb(s)).sb_fs_size as u64 || minlen > 0x4000 { return 0; } let e = if end == 0 || end > (*hpfs_sb(s)).sb_fs_size as u64 { (*hpfs_sb(s)).sb_fs_size as u64 } else { end }; let mut p = start >> 14; while p < ((e + 0x3fff) >> 14) { let mut qbh: quad_buffer_head = core::mem::zeroed(); hpfs_lock(s); if sb_rdonly(s) { hpfs_unlock(s); return -EROFS; } let bmp = hpfs_map_bitmap(s, p as _, &mut qbh, c"trim".as_ptr()); if bmp.is_null() { hpfs_unlock(s); return -EIO; } let mut idx = 0; while idx < 0x4000 { let l = find_run(bmp, &mut idx); if l == 0 { break; } let a = core::cmp::max(start, (p << 14) + idx as u64); let z = core::cmp::min(e, (p << 14) + (idx + l) as u64); if z - a >= minlen { let r = sb_issue_discard(s, a as _, (z-a) as _, GFP_NOFS, 0); if r != 0 { hpfs_brelse4(&mut qbh); hpfs_unlock(s); return r; } *result += (z-a) as u32; } idx += l; } hpfs_brelse4(&mut qbh); hpfs_unlock(s); p += 1; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
