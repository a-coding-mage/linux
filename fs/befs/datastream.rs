// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/befs/datastream.c
 *
 * Copyright (C) 2001 Will Dyson <will_dyson@pobox.com>
 *
 * Based on portions of file.c by Makoto Kato <m_kato@ga2.so-net.ne.jp>
 *
 * Many thanks to Dominic Giampaolo, author of "Practical File System
 * Design with the Be File System", for such a helpful book.
 */

// Dependencies are supplied by the surrounding filesystem implementation.

pub const BAD_IADDR: befs_inode_addr = befs_inode_addr { allocation_group: 0, start: 0, len: 0 };

unsafe fn befs_find_brun_direct(
    sb: *mut super_block,
    data: *const befs_data_stream,
    blockno: befs_blocknr_t,
    run: *mut befs_block_run,
) -> i32;

unsafe fn befs_find_brun_indirect(
    sb: *mut super_block,
    data: *const befs_data_stream,
    blockno: befs_blocknr_t,
    run: *mut befs_block_run,
) -> i32;

unsafe fn befs_find_brun_dblindirect(
    sb: *mut super_block,
    data: *const befs_data_stream,
    blockno: befs_blocknr_t,
    run: *mut befs_block_run,
) -> i32;

pub unsafe fn befs_read_datastream(
    sb: *mut super_block,
    ds: *const befs_data_stream,
    pos: befs_off_t,
    off: *mut u32,
) -> *mut buffer_head {
    let mut bh: *mut buffer_head;
    let mut run: befs_block_run = core::mem::zeroed();
    let block: befs_blocknr_t;

    befs_debug(sb, "---> %s %llu", "befs_read_datastream", pos);
    block = pos >> (*BEFS_SB(sb)).block_shift;
    if !off.is_null() {
        *off = (pos - (block << (*BEFS_SB(sb)).block_shift)) as u32;
    }

    if befs_fblock2brun(sb, ds, block, &mut run) != BEFS_OK {
        befs_error(sb, "BeFS: Error finding disk addr of block %lu", block as usize);
        befs_debug(sb, "<--- %s ERROR", "befs_read_datastream");
        return core::ptr::null_mut();
    }
    bh = befs_bread_iaddr(sb, run);
    if bh.is_null() {
        befs_error(sb, "BeFS: Error reading block %lu from datastream", block as usize);
        return core::ptr::null_mut();
    }

    befs_debug(sb, "<--- %s read data, starting at %llu", "befs_read_datastream", pos);
    bh
}

pub unsafe fn befs_fblock2brun(
    sb: *mut super_block,
    data: *const befs_data_stream,
    fblock: befs_blocknr_t,
    run: *mut befs_block_run,
) -> i32 {
    let pos: befs_off_t = fblock << (*BEFS_SB(sb)).block_shift;
    if pos < (*data).max_direct_range {
        befs_find_brun_direct(sb, data, fblock, run)
    } else if pos < (*data).max_indirect_range {
        befs_find_brun_indirect(sb, data, fblock, run)
    } else if pos < (*data).max_double_indirect_range {
        befs_find_brun_dblindirect(sb, data, fblock, run)
    } else {
        befs_error(sb, "befs_fblock2brun() was asked to find block %lu, which is not mapped by the datastream\n", fblock as usize);
        BEFS_ERR
    }
}

pub unsafe fn befs_read_lsymlink(
    sb: *mut super_block,
    ds: *const befs_data_stream,
    buff: *mut core::ffi::c_void,
    len: befs_off_t,
) -> usize {
    let mut bytes_read: befs_off_t = 0;
    let mut bh: *mut buffer_head;

    befs_debug(sb, "---> %s length: %llu", "befs_read_lsymlink", len);
    while bytes_read < len {
        bh = befs_read_datastream(sb, ds, bytes_read, core::ptr::null_mut());
        if bh.is_null() {
            befs_error(sb, "BeFS: Error reading datastream block starting from %llu", bytes_read);
            befs_debug(sb, "<--- %s ERROR", "befs_read_lsymlink");
            return bytes_read as usize;
        }
        let plen: usize = if bytes_read + (*BEFS_SB(sb)).block_size < len {
            (*BEFS_SB(sb)).block_size as usize
        } else {
            (len - bytes_read) as usize
        };
        core::ptr::copy_nonoverlapping(
            ((*bh).b_data as *const u8),
            (buff as *mut u8).add(bytes_read as usize),
            plen,
        );
        brelse(bh);
        bytes_read += plen as befs_off_t;
    }
    befs_debug(sb, "<--- %s read %u bytes", "befs_read_lsymlink", bytes_read as u32);
    bytes_read as usize
}

pub unsafe fn befs_count_blocks(sb: *mut super_block, ds: *const befs_data_stream) -> befs_blocknr_t {
    let befs_sb = BEFS_SB(sb);
    let mut datablocks = (*ds).size >> (*befs_sb).block_shift;
    if (*ds).size & ((*befs_sb).block_size - 1) != 0 { datablocks += 1; }
    let mut metablocks: befs_blocknr_t = 1;
    if (*ds).size > (*ds).max_direct_range { metablocks += (*ds).indirect.len as befs_blocknr_t; }
    if (*ds).size > (*ds).max_indirect_range && (*ds).max_indirect_range != 0 {
        let dbl_bytes = (*ds).max_double_indirect_range - (*ds).max_indirect_range;
        let dbl_bruns = dbl_bytes / ((*befs_sb).block_size * BEFS_DBLINDIR_BRUN_LEN);
        let indirblocks = dbl_bruns / befs_iaddrs_per_block(sb) as befs_off_t;
        metablocks += (*ds).double_indirect.len as befs_blocknr_t + indirblocks as befs_blocknr_t;
    }
    datablocks + metablocks
}

unsafe fn befs_find_brun_direct(sb: *mut super_block, data: *const befs_data_stream, blockno: befs_blocknr_t, run: *mut befs_block_run) -> i32 {
    let array = (*data).direct.as_ptr();
    let mut sum: befs_blocknr_t = 0;
    for i in 0..BEFS_NUM_DIRECT_BLOCKS {
        let item = &*array.add(i as usize);
        if blockno >= sum && blockno < sum + item.len as befs_blocknr_t {
            let offset = blockno - sum;
            (*run).allocation_group = item.allocation_group;
            (*run).start = item.start + offset as _;
            (*run).len = item.len - offset as _;
            return BEFS_OK;
        }
        sum += item.len as befs_blocknr_t;
    }
    befs_error(sb, "%s failed to find file block %lu", "befs_find_brun_direct", blockno as usize);
    BEFS_ERR
}

unsafe fn befs_find_brun_indirect(sb: *mut super_block, data: *const befs_data_stream, blockno: befs_blocknr_t, run: *mut befs_block_run) -> i32 {
    let indirect = (*data).indirect;
    let indirblockno = iaddr2blockno(sb, &indirect);
    let arraylen = befs_iaddrs_per_block(sb);
    let indir_start_blk = (*data).max_direct_range >> (*BEFS_SB(sb)).block_shift;
    let search_blk = blockno - indir_start_blk;
    let mut sum: befs_blocknr_t = 0;
    for i in 0..indirect.len {
        let indirblock = sb_bread(sb, indirblockno + i as _);
        if indirblock.is_null() { return BEFS_ERR; }
        let array = (*indirblock).b_data as *const befs_disk_block_run;
        for j in 0..arraylen {
            let item = &*array.add(j as usize);
            let len = fs16_to_cpu(sb, item.len) as befs_blocknr_t;
            if search_blk >= sum && search_blk < sum + len {
                let offset = search_blk - sum;
                (*run).allocation_group = fs32_to_cpu(sb, item.allocation_group);
                (*run).start = fs16_to_cpu(sb, item.start) as _ + offset as _;
                (*run).len = fs16_to_cpu(sb, item.len) as _ - offset as _;
                brelse(indirblock);
                return BEFS_OK;
            }
            sum += len;
        }
        brelse(indirblock);
    }
    BEFS_ERR
}

unsafe fn befs_find_brun_dblindirect(sb: *mut super_block, data: *const befs_data_stream, blockno: befs_blocknr_t, run: *mut befs_block_run) -> i32 {
    let indir_start_blk = (*data).max_indirect_range >> (*BEFS_SB(sb)).block_shift;
    let dbl_indir_off = blockno - indir_start_blk;
    let iblklen: usize = BEFS_DBLINDIR_BRUN_LEN as usize;
    let diblklen = iblklen * befs_iaddrs_per_block(sb) * BEFS_DBLINDIR_BRUN_LEN as usize;
    let dblindir_indx = dbl_indir_off as usize / diblklen;
    let dblindir_leftover = dbl_indir_off as usize % diblklen;
    let indir_indx = dblindir_leftover / diblklen;
    let dbl_which_block = dblindir_indx / befs_iaddrs_per_block(sb);
    if dbl_which_block > (*data).double_indirect.len as usize { return BEFS_ERR; }
    let dbl_indir_block = sb_bread(sb, iaddr2blockno(sb, &(*data).double_indirect) + dbl_which_block as _);
    if dbl_indir_block.is_null() { return BEFS_ERR; }
    let iaddr_array = (*dbl_indir_block).b_data as *const befs_disk_inode_addr;
    let dbl_block_indx = dblindir_indx - dbl_which_block * befs_iaddrs_per_block(sb);
    let indir_run = fsrun_to_cpu(sb, *iaddr_array.add(dbl_block_indx));
    brelse(dbl_indir_block);
    let which_block = indir_indx / befs_iaddrs_per_block(sb);
    if which_block > indir_run.len as usize { return BEFS_ERR; }
    let indir_block = sb_bread(sb, iaddr2blockno(sb, &indir_run) + which_block as _);
    if indir_block.is_null() { return BEFS_ERR; }
    let block_indx = indir_indx - which_block * befs_iaddrs_per_block(sb);
    let array = (*indir_block).b_data as *const befs_disk_inode_addr;
    *run = fsrun_to_cpu(sb, *array.add(block_indx));
    brelse(indir_block);
    let blockno_at_run_start = indir_start_blk + (diblklen * dblindir_indx + iblklen * indir_indx) as _;
    let offset = blockno - blockno_at_run_start;
    (*run).start += offset as _;
    (*run).len -= offset as _;
    BEFS_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
