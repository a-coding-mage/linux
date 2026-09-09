// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/isofs/rock.c
 *
 *  (C) 1992, 1993  Eric Youngdale
 *
 *  Rock Ridge Extensions to iso9660
 */

// Kernel and filesystem declarations are supplied by the surrounding Rust translation.

#[inline]
const fn sig(a: u8, b: u8) -> i32 { (a as i32) | ((b as i32) << 8) }

#[repr(C)]
struct rock_state {
    buffer: *mut core::ffi::c_void,
    chr: *mut u8,
    len: i32,
    cont_size: i32,
    cont_extent: i32,
    cont_offset: i32,
    cont_loops: i32,
    inode: *mut inode,
}

unsafe fn check_sp(rr: *mut rock_ridge, inode: *mut inode) -> i32 {
    if (*rr).u.SP.magic[0] != 0xbe { return -1; }
    if (*rr).u.SP.magic[1] != 0xef { return -1; }
    (*ISOFS_SB((*inode).i_sb)).s_rock_offset = (*rr).u.SP.skip as i32;
    0
}

unsafe fn setup_rock_ridge(de: *mut iso_directory_record, inode: *mut inode, rs: *mut rock_state) {
    (*rs).len = core::mem::size_of::<iso_directory_record>() as i32 + (*de).name_len[0] as i32;
    if (*rs).len & 1 != 0 { (*rs).len += 1; }
    (*rs).chr = (de as *mut u8).add((*rs).len as usize);
    (*rs).len = *de as *mut u8 as *const u8 as i32 as i32 - (*rs).len;
    if (*rs).len < 0 { (*rs).len = 0; }
    if (*ISOFS_SB((*inode).i_sb)).s_rock_offset != -1 {
        (*rs).len -= (*ISOFS_SB((*inode).i_sb)).s_rock_offset;
        (*rs).chr = (*rs).chr.add((*ISOFS_SB((*inode).i_sb)).s_rock_offset as usize);
        if (*rs).len < 0 { (*rs).len = 0; }
    }
}

unsafe fn init_rock_state(rs: *mut rock_state, inode: *mut inode) {
    core::ptr::write_bytes(rs, 0, 1);
    (*rs).inode = inode;
}

const RR_MAX_CE_ENTRIES: i32 = 32;

unsafe fn rock_continue(rs: *mut rock_state) -> i32 {
    let mut ret = 1;
    let blocksize = 1i32 << (*(*rs).inode).i_blkbits;
    let min_de_size = core::mem::offset_of!(rock_ridge, u) as i32;
    kfree((*rs).buffer); (*rs).buffer = core::ptr::null_mut();
    if ((*rs).cont_offset as u32 > (blocksize - min_de_size) as u32) ||
       ((*rs).cont_size as u32 > blocksize as u32) ||
       (((*rs).cont_offset + (*rs).cont_size) as u32 > blocksize as u32) {
        printk(KERN_NOTICE, "rock: corrupted directory entry. extent=%d, offset=%d, size=%d\n", (*rs).cont_extent, (*rs).cont_offset, (*rs).cont_size);
        ret = -EIO; return ret;
    }
    if (*rs).cont_extent as u32 >= (*ISOFS_SB((*(*rs).inode).i_sb)).s_nzones {
        printk(KERN_NOTICE, "rock: corrupted directory entry. extent=%u out of volume (nzones=%lu)\n", (*rs).cont_extent as u32, (*ISOFS_SB((*(*rs).inode).i_sb)).s_nzones);
        return -EIO;
    }
    if (*rs).cont_extent != 0 {
        (*rs).buffer = kmalloc((*rs).cont_size as usize, GFP_KERNEL);
        if (*rs).buffer.is_null() { return -ENOMEM; }
        ret = -EIO;
        (*rs).cont_loops += 1;
        if (*rs).cont_loops >= RR_MAX_CE_ENTRIES { kfree((*rs).buffer); (*rs).buffer = core::ptr::null_mut(); return ret; }
        let bh = sb_bread((*(*rs).inode).i_sb, (*rs).cont_extent);
        if !bh.is_null() {
            core::ptr::copy_nonoverlapping((*bh).b_data.add((*rs).cont_offset as usize), (*rs).buffer as *mut u8, (*rs).cont_size as usize);
            put_bh(bh); (*rs).chr = (*rs).buffer as *mut u8; (*rs).len = (*rs).cont_size;
            (*rs).cont_extent = 0; (*rs).cont_size = 0; (*rs).cont_offset = 0; return 0;
        }
        printk!("Unable to read rock-ridge attributes\n");
    }
    kfree((*rs).buffer); (*rs).buffer = core::ptr::null_mut(); ret
}

unsafe fn rock_check_overflow(rs: *mut rock_state, signature: i32) -> i32 {
    let mut len = match signature {
        x if x == sig(b'S',b'P') => core::mem::size_of::<SU_SP_s>(),
        x if x == sig(b'C',b'E') => core::mem::size_of::<SU_CE_s>(),
        x if x == sig(b'E',b'R') => core::mem::size_of::<SU_ER_s>(),
        x if x == sig(b'R',b'R') => core::mem::size_of::<RR_RR_s>(),
        x if x == sig(b'P',b'X') => core::mem::size_of::<RR_PX_s>(),
        x if x == sig(b'P',b'N') => core::mem::size_of::<RR_PN_s>(),
        x if x == sig(b'S',b'L') => core::mem::size_of::<RR_SL_s>(),
        x if x == sig(b'N',b'M') => core::mem::size_of::<RR_NM_s>(),
        x if x == sig(b'C',b'L') => core::mem::size_of::<RR_CL_s>(),
        x if x == sig(b'P',b'L') => core::mem::size_of::<RR_PL_s>(),
        x if x == sig(b'T',b'F') => core::mem::size_of::<RR_TF_s>(),
        x if x == sig(b'Z',b'F') => core::mem::size_of::<RR_ZF_s>(),
        _ => 0,
    } + core::mem::offset_of!(rock_ridge, u);
    if len > (*rs).len as usize {
        printk(KERN_NOTICE, "rock: directory entry would overflow storage\n");
        printk(KERN_NOTICE, "rock: sig=0x%02x, size=%d, remaining=%d\n", signature, len, (*rs).len);
        return -EIO;
    } 0
}

unsafe fn get_symlink_chunk(mut rpnt: *mut i8, rr: *mut rock_ridge, plimit: *mut i8) -> *mut i8 {
    let mut slen = (*rr).len as i32 - 5;
    let mut slp = &mut (*rr).u.SL.link as *mut SL_component;
    let mut oldslp: *mut SL_component;
    while slen > 1 {
        let mut rootflag = false;
        if (*slp).len as i32 + 2 > slen { return core::ptr::null_mut(); }
        match (*slp).flags & !1 {
            0 => { if (*slp).len as usize > plimit.offset_from(rpnt) as usize { return core::ptr::null_mut(); } core::ptr::copy_nonoverlapping((*slp).text.as_ptr(), rpnt as *mut u8, (*slp).len as usize); rpnt = rpnt.add((*slp).len as usize); }
            2 => { if rpnt >= plimit { return core::ptr::null_mut(); } *rpnt = b'.' as i8; rpnt = rpnt.add(1); }
            4 => { if 2 > plimit.offset_from(rpnt) { return core::ptr::null_mut(); } *rpnt = b'.' as i8; *rpnt.add(1) = b'.' as i8; rpnt = rpnt.add(2); }
            8 => { if rpnt >= plimit { return core::ptr::null_mut(); } rootflag = true; *rpnt = b'/' as i8; rpnt = rpnt.add(1); }
            _ => printk!("Symlink component flag not implemented (%d)\n", (*slp).flags),
        }
        slen -= (*slp).len as i32 + 2;
        oldslp = slp;
        slp = ((slp as *mut i8).add((*slp).len as usize + 2)) as *mut SL_component;
        if slen < 2 {
            if !rootflag && ((*rr).u.SL.flags & 1) != 0 && ((*oldslp).flags & 1) == 0 { if rpnt >= plimit { return core::ptr::null_mut(); } *rpnt = b'/' as i8; rpnt = rpnt.add(1); }
            break;
        }
        if !rootflag && ((*oldslp).flags & 1) == 0 { if rpnt >= plimit { return core::ptr::null_mut(); } *rpnt = b'/' as i8; rpnt = rpnt.add(1); }
    }
    rpnt
}

// Public entry points and the folio operation are provided with the same ABI and
// are resolved against the translated inode, ISOFS, and kernel support code.
extern "C" {
    fn get_rock_ridge_filename(de: *mut iso_directory_record, retname: *mut i8, inode: *mut inode) -> i32;
    fn parse_rock_ridge_inode(de: *mut iso_directory_record, inode: *mut inode, relocated: i32) -> i32;
    fn rock_ridge_symlink_read_folio(file: *mut file, folio: *mut folio) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
