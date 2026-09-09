/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

use core::ptr;

/* Declarations supplied by the surrounding kernel/JFFS2 sources. */
extern "C" {
    fn jffs2_alloc_raw_inode() -> *mut jffs2_raw_inode;
    fn jffs2_free_raw_inode(ri: *mut jffs2_raw_inode);
    fn jffs2_flash_read(c: *mut jffs2_sb_info, ofs: u32, len: usize,
                        readlen: *mut usize, buf: *mut u8) -> i32;
    fn jffs2_decompress(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                        compr: u16, inbuf: *mut u8, outbuf: *mut u8,
                        csize: u32, dsize: u32) -> i32;
    fn jffs2_lookup_node_frag(tree: *mut jffs2_node_frag_tree,
                               offset: u32) -> *mut jffs2_node_frag;
    fn frag_next(frag: *mut jffs2_node_frag) -> *mut jffs2_node_frag;
    fn crc32(crc: u32, buf: *const core::ffi::c_void, len: usize) -> u32;
    fn je32_to_cpu(v: u32) -> u32;
    fn cpu_to_je32(v: u32) -> u32;
    fn ref_offset(raw: *mut jffs2_raw_node_ref) -> u32;
    fn ref_flags(raw: *mut jffs2_raw_node_ref) -> u32;
}

#[repr(C)] pub struct jffs2_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_inode_info { pub inocache: *mut jffs2_inode_cache, pub fragtree: jffs2_node_frag_tree }
#[repr(C)] pub struct jffs2_inode_cache { pub ino: u32 }
#[repr(C)] pub struct jffs2_node_frag_tree { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_raw_node_ref { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_full_dnode { pub raw: *mut jffs2_raw_node_ref, pub ofs: u32 }
#[repr(C)] pub struct jffs2_node_frag { pub ofs: u32, pub size: u32, pub node: *mut jffs2_full_dnode }
#[repr(C)] pub struct jffs2_raw_inode {
    pub node_crc: u32, pub compr: u8, pub usercompr: u8, pub dsize: u32,
    pub csize: u32, pub offset: u32, pub data_crc: u32,
}

pub const JFFS2_COMPR_NONE: u8 = 0;
pub const JFFS2_COMPR_ZERO: u8 = 5;
pub const GFP_KERNEL: u32 = 0;

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

#[inline] unsafe fn min_u32(a: u32, b: u32) -> u32 { if a < b { a } else { b } }

pub unsafe fn jffs2_read_dnode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
    fd: *mut jffs2_full_dnode, buf: *mut u8, ofs: i32, len: i32) -> i32 {
    let ri = jffs2_alloc_raw_inode();
    if ri.is_null() { return -12; }
    let mut readlen: usize = 0;
    let mut ret = jffs2_flash_read(c, ref_offset((*fd).raw), core::mem::size_of::<jffs2_raw_inode>(), &mut readlen, ri as *mut u8);
    if ret != 0 { jffs2_free_raw_inode(ri); return ret; }
    if readlen != core::mem::size_of::<jffs2_raw_inode>() { jffs2_free_raw_inode(ri); return -5; }
    let mut crc = crc32(0, ri as *const _, core::mem::size_of::<jffs2_raw_inode>() - 8);
    if crc != je32_to_cpu((*ri).node_crc) { ret = -5; jffs2_free_raw_inode(ri); return ret; }
    if (*ri).compr == JFFS2_COMPR_ZERO && je32_to_cpu((*ri).dsize) == 0 && je32_to_cpu((*ri).csize) != 0 {
        (*ri).dsize = (*ri).csize; (*ri).csize = cpu_to_je32(0);
    }
    if ofs + len > je32_to_cpu((*ri).dsize) as i32 { jffs2_free_raw_inode(ri); return -22; }
    if (*ri).compr == JFFS2_COMPR_ZERO { ptr::write_bytes(buf, 0, len as usize); jffs2_free_raw_inode(ri); return 0; }
    let mut readbuf = if (*ri).compr == JFFS2_COMPR_NONE && len == je32_to_cpu((*ri).dsize) { buf } else { kmalloc(je32_to_cpu((*ri).csize) as usize, GFP_KERNEL) };
    if readbuf.is_null() { jffs2_free_raw_inode(ri); return -12; }
    let mut decomprbuf = if (*ri).compr != JFFS2_COMPR_NONE { if len < je32_to_cpu((*ri).dsize) { kmalloc(je32_to_cpu((*ri).dsize) as usize, GFP_KERNEL) } else { buf } } else { readbuf };
    if decomprbuf.is_null() { if readbuf != buf { kfree(readbuf); } jffs2_free_raw_inode(ri); return -12; }
    ret = jffs2_flash_read(c, ref_offset((*fd).raw) + core::mem::size_of::<jffs2_raw_inode>() as u32, je32_to_cpu((*ri).csize) as usize, &mut readlen, readbuf);
    if ret == 0 && readlen != je32_to_cpu((*ri).csize) as usize { ret = -5; }
    if ret == 0 {
        crc = crc32(0, readbuf as *const _, je32_to_cpu((*ri).csize) as usize);
        if crc != je32_to_cpu((*ri).data_crc) { ret = -5; }
        else if (*ri).compr != JFFS2_COMPR_NONE { ret = jffs2_decompress(c, f, (*ri).compr as u16 | ((*ri).usercompr as u16) << 8, readbuf, decomprbuf, je32_to_cpu((*ri).csize), je32_to_cpu((*ri).dsize)); }
    }
    if ret == 0 && len < je32_to_cpu((*ri).dsize) as i32 { ptr::copy(decomprbuf.add(ofs as usize), buf, len as usize); }
    if decomprbuf != buf && decomprbuf != readbuf { kfree(decomprbuf); }
    if readbuf != buf { kfree(readbuf); }
    jffs2_free_raw_inode(ri); ret
}

pub unsafe fn jffs2_read_inode_range(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, mut buf: *mut u8, mut offset: u32, len: u32) -> i32 {
    let end = offset.wrapping_add(len); let mut frag = jffs2_lookup_node_frag(&mut (*f).fragtree, offset);
    while offset < end {
        if frag.is_null() || (*frag).ofs > offset || (*frag).ofs.wrapping_add((*frag).size) <= offset {
            let mut holesize = end - offset; if !frag.is_null() && (*frag).ofs > offset { holesize = min_u32(holesize, (*frag).ofs - offset); }
            ptr::write_bytes(buf, 0, holesize as usize); buf = buf.add(holesize as usize); offset += holesize;
        } else if (*frag).node.is_null() {
            let holeend = min_u32(end, (*frag).ofs + (*frag).size); ptr::write_bytes(buf, 0, (holeend-offset) as usize); buf = buf.add((holeend-offset) as usize); offset = holeend; frag = frag_next(frag);
        } else {
            let fragofs = offset - (*frag).ofs; let readlen = min_u32((*frag).size-fragofs, end-offset);
            let ret = jffs2_read_dnode(c, f, (*frag).node, buf, (fragofs + (*frag).ofs - (*(*frag).node).ofs) as i32, readlen as i32);
            if ret != 0 { ptr::write_bytes(buf, 0, readlen as usize); return ret; }
            buf = buf.add(readlen as usize); offset += readlen; frag = frag_next(frag);
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
