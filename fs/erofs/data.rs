// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017-2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2021, Alibaba Cloud
 */
// Translated from data.c; declarations supplied by internal kernel headers are external.

pub const EROFS_ONLINEFOLIO_EIO: u32 = 30;
pub const EROFS_ONLINEFOLIO_DIRTY: u32 = 29;

pub unsafe fn erofs_unmap_metabuf(buf: *mut erofs_buf) {
    if (*buf).base.is_null() { return; }
    kunmap_local((*buf).base);
    (*buf).base = core::ptr::null_mut();
}

pub unsafe fn erofs_put_metabuf(buf: *mut erofs_buf) {
    if (*buf).page.is_null() { return; }
    erofs_unmap_metabuf(buf);
    folio_put(page_folio((*buf).page));
    (*buf).page = core::ptr::null_mut();
}

pub unsafe fn erofs_bread(buf: *mut erofs_buf, offset: erofs_off_t, need_kmap: bool) -> *mut core::ffi::c_void {
    let index: pgoff_t = ((*buf).off + offset) >> PAGE_SHIFT;
    let mut folio: *mut folio = core::ptr::null_mut();
    if !(*buf).page.is_null() {
        folio = page_folio((*buf).page);
        if folio_file_page(folio, index) != (*buf).page { erofs_unmap_metabuf(buf); }
    }
    if folio.is_null() || !folio_contains(folio, index) {
        erofs_put_metabuf(buf);
        folio = read_cache_folio((*buf).mapping, index, if (*buf).mc { Some(erofs_read_meta_folio) } else { None }, core::ptr::null_mut());
        if IS_ERR(folio) { return folio as *mut core::ffi::c_void; }
    }
    (*buf).page = folio_file_page(folio, index);
    if !need_kmap { return core::ptr::null_mut(); }
    if (*buf).base.is_null() { (*buf).base = kmap_local_page((*buf).page); }
    ((*buf).base as *mut u8).add((offset & !PAGE_MASK) as usize) as *mut core::ffi::c_void
}

pub unsafe fn erofs_init_metabuf(buf: *mut erofs_buf, sb: *mut super_block, in_metabox: bool) -> i32 {
    let sbi = EROFS_SB(sb);
    (*buf).mc = false;
    if in_metabox {
        if (*sbi).metabox_inode.is_null() { return -EFSCORRUPTED; }
        (*buf).mapping = (*(*sbi).metabox_inode).i_mapping;
        return 0;
    }
    if erofs_is_fileio_mode(sbi) {
        (*buf).mapping = (*(*sbi).managed_cache).i_mapping;
        (*buf).mc = true;
    } else {
        (*buf).off = (*sbi).dif0.fsoff;
        (*buf).mapping = (*sb).s_bdev.bd_mapping;
    }
    0
}

pub unsafe fn erofs_read_metabuf(buf: *mut erofs_buf, sb: *mut super_block, offset: erofs_off_t, in_metabox: bool) -> *mut core::ffi::c_void {
    let err = erofs_init_metabuf(buf, sb, in_metabox);
    if err != 0 { return ERR_PTR(err as isize); }
    erofs_bread(buf, offset, true)
}

unsafe fn erofs_map_chunks(inode: *mut inode, map: *mut erofs_map_blocks) -> i32 {
    let mut buf = __EROFS_BUF_INITIALIZER;
    let sb = (*inode).i_sb;
    let vi = EROFS_I(inode);
    let unit = if (*vi).chunkformat & EROFS_CHUNK_FORMAT_INDEXES != 0 { core::mem::size_of::<erofs_inode_chunk_index>() as u32 } else { EROFS_BLOCK_MAP_ENTRY_SIZE };
    let addrmask = if (*vi).chunkformat & EROFS_CHUNK_FORMAT_48BIT != 0 { (1u64 << 48) - 1 } else { (1u64 << 32) - 1 };
    let mut nr = (*map).m_la >> (*vi).chunkbits;
    let chunksize = 1u64 << (*vi).chunkbits;
    let pos = ALIGN(erofs_iloc(inode) + (*vi).inode_isize + (*vi).xattr_isize, unit as u64) + unit as u64 * nr;
    let endpos = round_up(pos + 1, (*sb).s_blocksize);
    let idx = erofs_read_metabuf(&mut buf, sb, pos, erofs_inode_in_metabox(inode));
    if IS_ERR(idx) { return PTR_ERR(idx); }
    (*map).m_la = nr << (*vi).chunkbits; (*map).m_llen = 0; nr = 0;
    let mut last = 0; let mut addr;
    loop {
        if unit == EROFS_BLOCK_MAP_ENTRY_SIZE { addr = le32_to_cpu(*(idx as *mut u32).add(nr as usize)) as u64; } else { let p = (idx as *mut erofs_inode_chunk_index).add(nr as usize); addr = (((le16_to_cpu((*p).startblk_hi) as u64) << 32) | le32_to_cpu((*p).startblk_lo) as u64) & addrmask; if addr ^ (EROFS_NULL_ADDR & addrmask) != 0 { addr |= (le16_to_cpu((*p).device_id) as u64 & (*EROFS_SB(sb)).device_id_mask) << 48; } else { addr = EROFS_NULL_ADDR; } }
        if nr == 0 { last = addr; } else { (*map).m_llen += chunksize; if last != EROFS_NULL_ADDR { last += erofs_blknr(sb, chunksize); } }
        if addr != last || pos + (nr + 1) * unit as u64 >= endpos { break; } nr += 1;
    }
    if last != EROFS_NULL_ADDR { (*map).m_pa = erofs_pos(sb, last & addrmask) - (*map).m_llen; (*map).m_deviceid = last >> 48; (*map).m_flags = EROFS_MAP_MAPPED; }
    if addr == last { (*map).m_llen += chunksize; }
    (*map).m_llen = core::cmp::min((*map).m_llen, round_up((*inode).i_size - (*map).m_la, (*sb).s_blocksize));
    erofs_put_metabuf(&mut buf); 0
}

pub unsafe fn erofs_map_blocks(inode: *mut inode, map: *mut erofs_map_blocks) -> i32 {
    let sb = (*inode).i_sb; let vi = EROFS_I(inode); let tailinline = (*vi).datalayout == EROFS_INODE_FLAT_INLINE; let mut err = 0;
    trace_erofs_map_blocks_enter(inode, map, 0); (*map).m_deviceid = 0; (*map).m_flags = 0;
    if (*map).m_la < (*inode).i_size { if (*vi).datalayout == EROFS_INODE_CHUNK_BASED { err = erofs_map_chunks(inode, map); } else if tailinline || (*vi).startblk != EROFS_NULL_ADDR { let pos = erofs_pos(sb, erofs_iblks(inode) - tailinline as u64); (*map).m_flags = EROFS_MAP_MAPPED; if (*map).m_la < pos { (*map).m_pa = erofs_pos(sb, (*vi).startblk) + (*map).m_la; (*map).m_llen = pos - (*map).m_la; } else { (*map).m_pa = erofs_iloc(inode) + (*vi).inode_isize + (*vi).xattr_isize + erofs_blkoff(sb, (*map).m_la); (*map).m_llen = (*inode).i_size - (*map).m_la; (*map).m_flags |= EROFS_MAP_META; } } }
    (*map).m_plen = if err != 0 { 0 } else { (*map).m_llen }; trace_erofs_map_blocks_exit(inode, map, 0, err); err
}

pub unsafe fn erofs_onlinefolio_init(folio: *mut folio) { (*folio).private = 1 as *mut core::ffi::c_void; }
pub unsafe fn erofs_onlinefolio_split(folio: *mut folio) { atomic_inc((*folio).private as *mut atomic_t); }
pub unsafe fn erofs_onlinefolio_end(folio: *mut folio, err: i32, dirty: bool) {
    let mut orig; let v;
    loop { orig = atomic_read((*folio).private as *mut atomic_t); DBG_BUGON(orig <= 0); v = ((dirty as i32) << EROFS_ONLINEFOLIO_DIRTY) | ((orig - 1) | (((err != 0) as i32) << EROFS_ONLINEFOLIO_EIO)); if atomic_cmpxchg((*folio).private as *mut atomic_t, orig, v) == orig { break; } }
    if v & ((1 << EROFS_ONLINEFOLIO_DIRTY) - 1) != 0 { return; }
    (*folio).private = core::ptr::null_mut(); if v & (1 << EROFS_ONLINEFOLIO_DIRTY) != 0 { flush_dcache_folio(folio); } folio_end_read(folio, v & (1 << EROFS_ONLINEFOLIO_EIO) == 0);
}

#[repr(C)] pub struct erofs_iomap_iter_ctx { pub page: *mut page, pub base: *mut core::ffi::c_void, pub realinode: *mut inode }
pub unsafe fn erofs_fiemap(inode: *mut inode, fieinfo: *mut fiemap_extent_info, start: u64, len: u64) -> i32 { if erofs_inode_is_data_compressed((*EROFS_I(inode)).datalayout) { if !IS_ENABLED(CONFIG_EROFS_FS_ZIP) { return -EOPNOTSUPP; } return iomap_fiemap(inode, fieinfo, start, len, &z_erofs_iomap_report_ops); } iomap_fiemap(inode, fieinfo, start, len, &erofs_iomap_ops) }
pub unsafe fn erofs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t { iomap_bmap(mapping, block, &erofs_iomap_ops) }
pub unsafe fn erofs_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t { let inode = file_inode((*iocb).ki_filp); if iov_iter_count(to) == 0 { return 0; } if IS_ENABLED(CONFIG_FS_DAX) && IS_DAX(inode) { return dax_iomap_rw(iocb, to, &erofs_iomap_ops); } if (*iocb).ki_flags & IOCB_DIRECT != 0 && !(*inode).i_sb.s_bdev.is_null() { return iomap_dio_rw(iocb, to, &erofs_iomap_ops, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0); } filemap_read(iocb, to, 0) }

pub unsafe fn erofs_file_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t { let inode = (*(*file).f_mapping).host; let mut off = offset; if whence == SEEK_HOLE { off = iomap_seek_hole(inode, off, &erofs_iomap_ops); } else if whence == SEEK_DATA { off = iomap_seek_data(inode, off, &erofs_iomap_ops); } else { return generic_file_llseek(file, offset, whence); } if off < 0 { return off; } vfs_setpos(file, off, (*(*inode).i_sb).s_maxbytes) }

// The remaining kernel iomap/file-operation callbacks retain their C ABI and are declared externally here.
extern "C" {
    pub static erofs_aops: address_space_operations;
    pub static erofs_file_fops: file_operations;
    pub fn erofs_map_dev(sb: *mut super_block, map: *mut erofs_map_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
