// SPDX-License-Identifier: GPL-2.0
/*
 * Data verification functions, i.e. hooks for ->readahead()
 *
 * Copyright 2019 Google LLC
 */

const FS_VERITY_MAX_PENDING_BLOCKS: usize = 2;

#[repr(C)]
struct FsverityPendingBlock {
    data: *const core::ffi::c_void,
    pos: u64,
    real_hash: [u8; FS_VERITY_MAX_DIGEST_SIZE],
}

#[repr(C)]
struct FsverityVerificationContext {
    vi: *mut FsverityInfo,
    num_pending: i32,
    max_pending: i32,
    pending_blocks: [FsverityPendingBlock; FS_VERITY_MAX_PENDING_BLOCKS],
}

static mut FSVERITY_READ_WORKQUEUE: *mut WorkqueueStruct = core::ptr::null_mut();

pub unsafe fn fsverity_readahead(vi: *mut FsverityInfo, index: PgoffT, nr_pages: CULong) {
    let inode = (*vi).inode;
    let params = &(*vi).tree_params;
    let mut start_hidx = (index as u64) << params.log_blocks_per_page;
    let mut end_hidx = (((index as u64) + nr_pages as u64) << params.log_blocks_per_page) - 1;
    if (*(*inode).i_sb).s_vop.read_ahead_merkle_tree.is_none() { return; }
    for level in 0..params.num_levels {
        let level_start = params.level_start[level as usize];
        let next_start_hidx = start_hidx >> params.log_arity;
        let next_end_hidx = end_hidx >> params.log_arity;
        let start_idx = (level_start + next_start_hidx) >> params.log_blocks_per_page;
        let end_idx = (level_start + next_end_hidx) >> params.log_blocks_per_page;
        ((*(*inode).i_sb).s_vop.read_ahead_merkle_tree.unwrap())(
            inode, start_idx as PgoffT, end_idx - start_idx + 1);
        start_hidx = next_start_hidx;
        end_hidx = next_end_hidx;
    }
}

unsafe fn is_hash_block_verified(vi: *mut FsverityInfo, hpage: *mut Page,
                                  mut hblock_idx: CULong) -> bool {
    if (*vi).hash_block_verified.is_null() { return PageChecked(hpage); }
    if PageChecked(hpage) {
        smp_rmb();
        return test_bit(hblock_idx, (*vi).hash_block_verified);
    }
    let blocks_per_page = (*vi).tree_params.blocks_per_page;
    hblock_idx = round_down(hblock_idx, blocks_per_page);
    for i in 0..blocks_per_page { clear_bit(hblock_idx + i, (*vi).hash_block_verified); }
    smp_wmb();
    SetPageChecked(hpage);
    false
}

unsafe fn verify_data_block(vi: *mut FsverityInfo, dblock: *const FsverityPendingBlock) -> bool {
    let inode = (*vi).inode;
    let data_pos = (*dblock).pos;
    let params = &(*vi).tree_params;
    let hsize = params.digest_size;
    let mut level: i32;
    let mut want_hash_buf = [0u8; FS_VERITY_MAX_DIGEST_SIZE];
    let mut want_hash: *const u8;
    let mut real_hash = [0u8; FS_VERITY_MAX_DIGEST_SIZE];
    struct HBlock { page: *mut Page, addr: *const core::ffi::c_void, index: CULong, hoffset: CUInt }
    let mut hblocks: [HBlock; FS_VERITY_MAX_LEVELS] = core::array::from_fn(|_| HBlock { page: core::ptr::null_mut(), addr: core::ptr::null(), index: 0, hoffset: 0 });
    trace_fsverity_verify_data_block(inode, params, data_pos);
    let mut hidx = data_pos >> params.log_blocksize;
    if data_pos >= (*inode).i_size {
        if memchr_inv((*dblock).data, 0, params.block_size) != 0 {
            fsverity_err(inode, "FILE CORRUPTED!  Data past EOF is not zeroed"); return false;
        }
        return true;
    }
    level = 0;
    while level < params.num_levels as i32 {
        let next_hidx = hidx >> params.log_arity;
        let hblock_idx = params.level_start[level as usize] + next_hidx;
        let hpage_idx = hblock_idx >> params.log_blocks_per_page;
        let hblock_offset = (hblock_idx << params.log_blocksize) & !PAGE_MASK;
        let hoffset = (hidx << params.log_digestsize) & (params.block_size - 1);
        let hpage = (*(*inode).i_sb).s_vop.read_merkle_tree_page(inode, hpage_idx);
        if IS_ERR(hpage) { fsverity_err(inode, "Error %ld reading Merkle tree page %lu", PTR_ERR(hpage), hpage_idx); break; }
        let haddr = (kmap_local_page(hpage) as *const u8).add(hblock_offset as usize) as *const core::ffi::c_void;
        if is_hash_block_verified(vi, hpage, hblock_idx) {
            core::ptr::copy_nonoverlapping((haddr as *const u8).add(hoffset as usize), want_hash_buf.as_mut_ptr(), hsize as usize);
            want_hash = want_hash_buf.as_ptr(); kunmap_local(haddr); put_page(hpage); trace_fsverity_merkle_hit(inode, data_pos, hblock_idx, level, hoffset >> params.log_digestsize); break;
        }
        hblocks[level as usize] = HBlock { page: hpage, addr: haddr, index: hblock_idx, hoffset };
        hidx = next_hidx; level += 1;
    }
    if level == params.num_levels as i32 { want_hash = (*vi).root_hash; }
    while level > 0 {
        level -= 1;
        let hb = &hblocks[level as usize];
        trace_fsverity_verify_merkle_block(inode, hb.index, level, hb.hoffset >> params.log_digestsize);
        fsverity_hash_block(params, hb.addr, real_hash.as_mut_ptr());
        if memcmp(want_hash, real_hash.as_ptr(), hsize) != 0 { break; }
        if !(*vi).hash_block_verified.is_null() { set_bit(hb.index, (*vi).hash_block_verified); } else { SetPageChecked(hb.page); }
        core::ptr::copy_nonoverlapping((hb.addr as *const u8).add(hb.hoffset as usize), want_hash_buf.as_mut_ptr(), hsize as usize);
        want_hash = want_hash_buf.as_ptr(); kunmap_local(hb.addr); put_page(hb.page);
    }
    if memcmp(want_hash, (*dblock).real_hash.as_ptr(), hsize) == 0 { return true; }
    fsverity_err(inode, "FILE CORRUPTED! pos=%llu, level=%d, want_hash=%s:%*phN, real_hash=%s:%*phN", data_pos, level - 1, params.hash_alg.name, hsize, want_hash, params.hash_alg.name, hsize, if level == 0 { (*dblock).real_hash.as_ptr() } else { real_hash.as_ptr() });
    for i in (0..level as usize).rev() { kunmap_local(hblocks[i].addr); put_page(hblocks[i].page); }
    false
}

unsafe fn fsverity_init_verification_context(ctx: *mut FsverityVerificationContext, vi: *mut FsverityInfo) {
    (*ctx).vi = vi; (*ctx).num_pending = 0;
    (*ctx).max_pending = if (*vi).tree_params.hash_alg.algo_id == HASH_ALGO_SHA256 && sha256_finup_2x_is_optimized() { 2 } else { 1 };
}

unsafe fn fsverity_clear_pending_blocks(ctx: *mut FsverityVerificationContext) {
    for i in (0..(*ctx).num_pending as usize).rev() { kunmap_local((*ctx).pending_blocks[i].data); (*ctx).pending_blocks[i].data = core::ptr::null(); }
    (*ctx).num_pending = 0;
}

unsafe fn fsverity_verify_pending_blocks(ctx: *mut FsverityVerificationContext) -> bool {
    let vi = (*ctx).vi; let params = &(*vi).tree_params;
    if (*ctx).num_pending == 2 { sha256_finup_2x(if params.hashstate.is_null() { core::ptr::null() } else { &mut (*params.hashstate).sha256 }, (*ctx).pending_blocks[0].data, (*ctx).pending_blocks[1].data, params.block_size, (*ctx).pending_blocks[0].real_hash.as_mut_ptr(), (*ctx).pending_blocks[1].real_hash.as_mut_ptr()); }
    else { for i in 0..(*ctx).num_pending as usize { fsverity_hash_block(params, (*ctx).pending_blocks[i].data, (*ctx).pending_blocks[i].real_hash.as_mut_ptr()); } }
    for i in 0..(*ctx).num_pending as usize { if !verify_data_block(vi, &(*ctx).pending_blocks[i]) { return false; } }
    fsverity_clear_pending_blocks(ctx); true
}

unsafe fn fsverity_add_data_blocks(ctx: *mut FsverityVerificationContext, data_folio: *mut Folio, mut len: usize, mut offset: usize) -> bool {
    let vi = (*ctx).vi; let params = &(*vi).tree_params; let block_size = params.block_size; let pos = ((*data_folio).index as u64) << PAGE_SHIFT;
    if WARN_ON_ONCE(len == 0 || ((len | offset) % block_size) != 0) || WARN_ON_ONCE(!folio_test_locked(data_folio) || folio_test_uptodate(data_folio)) { return false; }
    while len != 0 { let i = (*ctx).num_pending as usize; (*ctx).pending_blocks[i].data = kmap_local_folio(data_folio, offset); (*ctx).pending_blocks[i].pos = pos + offset as u64; (*ctx).num_pending += 1; if (*ctx).num_pending == (*ctx).max_pending && !fsverity_verify_pending_blocks(ctx) { return false; } offset += block_size; len -= block_size; }
    true
}

pub unsafe fn fsverity_verify_blocks(vi: *mut FsverityInfo, folio: *mut Folio, len: usize, offset: usize) -> bool {
    let mut ctx: FsverityVerificationContext = core::mem::zeroed(); fsverity_init_verification_context(&mut ctx, vi);
    if fsverity_add_data_blocks(&mut ctx, folio, len, offset) && fsverity_verify_pending_blocks(&mut ctx) { return true; }
    fsverity_clear_pending_blocks(&mut ctx); false
}

#[cfg(CONFIG_BLOCK)]
pub unsafe fn fsverity_verify_bio(vi: *mut FsverityInfo, bio: *mut Bio) {
    let mut ctx: FsverityVerificationContext = core::mem::zeroed();
    fsverity_init_verification_context(&mut ctx, vi);
    let mut fi: FolioIter = core::mem::zeroed();
    bio_for_each_folio_all!(fi, bio, {
        if !fsverity_add_data_blocks(&mut ctx, fi.folio, fi.length, fi.offset) { fsverity_clear_pending_blocks(&mut ctx); (*bio).bi_status = BLK_STS_IOERR; return; }
    });
    if !fsverity_verify_pending_blocks(&mut ctx) { fsverity_clear_pending_blocks(&mut ctx); (*bio).bi_status = BLK_STS_IOERR; }
}

pub unsafe fn fsverity_enqueue_verify_work(work: *mut WorkStruct) { queue_work(FSVERITY_READ_WORKQUEUE, work); }

pub unsafe fn fsverity_init_workqueue() {
    FSVERITY_READ_WORKQUEUE = alloc_workqueue("fsverity_read_queue", WQ_HIGHPRI | WQ_PERCPU, num_online_cpus());
    if FSVERITY_READ_WORKQUEUE.is_null() { panic!("failed to allocate fsverity_read_queue"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
