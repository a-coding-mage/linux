// SPDX-License-Identifier: GPL-2.0-or-later
/* Search a directory's hash table.
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * https://tools.ietf.org/html/draft-keiser-afs3-directory-object-00
 */

/* Linux headers and local headers from the C translation unit provide the
 * types, constants, and external functions referenced below. */

/* Calculate the name hash. */
pub unsafe fn afs_dir_hash_name(name: *const qstr) -> u32 {
    let p = (*name).name as *const u8;
    let mut hash: u32 = 0;
    let mut i: u32 = 0;
    let mut bucket: i32;

    while i < (*name).len {
        hash = hash.wrapping_mul(173).wrapping_add(*p.add(i as usize) as u32);
        i += 1;
    }
    bucket = (hash & (AFS_DIR_HASHTBL_SIZE - 1)) as i32;
    if hash > INT_MAX as u32 {
        bucket = AFS_DIR_HASHTBL_SIZE as i32 - bucket;
        bucket &= (AFS_DIR_HASHTBL_SIZE - 1) as i32;
    }
    bucket as u32
}

/* Reset a directory iterator. */
unsafe fn afs_dir_reset_iter(iter: *mut afs_dir_iter) -> bool {
    let i_size: u64 = i_size_read(&(*(*iter).dvnode).netfs.inode);
    let nblocks = core::cmp::min(
        i_size / AFS_DIR_BLOCK_SIZE as u64,
        AFS_DIR_MAX_BLOCKS as u64,
    ) as u32;

    if nblocks == 0 {
        return false;
    }
    (*iter).loop_check = nblocks * (AFS_DIR_SLOTS_PER_BLOCK - AFS_DIR_RESV_BLOCKS);
    (*iter).prev_entry = 0;
    true
}

/* Initialise a directory iterator for looking up a name. */
pub unsafe fn afs_dir_init_iter(iter: *mut afs_dir_iter, name: *const qstr) -> bool {
    (*iter).nr_slots = afs_dir_calc_slots((*name).len);
    (*iter).bucket = afs_dir_hash_name(name);
    afs_dir_reset_iter(iter)
}

/* Get a specific block. */
pub unsafe fn afs_dir_find_block(
    iter: *mut afs_dir_iter,
    block: usize,
) -> *mut afs_xdr_dir_block {
    let mut fq = (*iter).fq;
    let dvnode = (*iter).dvnode;
    let blpos = block * AFS_DIR_BLOCK_SIZE as usize;
    let blend = (block + 1) * AFS_DIR_BLOCK_SIZE as usize;
    let mut fpos = (*iter).fpos;
    let mut slot = (*iter).fq_slot;

    if !(*iter).block.is_null() {
        kunmap_local((*iter).block as *mut core::ffi::c_void);
        (*iter).block = core::ptr::null_mut();
    }
    if (*dvnode).directory_size < blend {
        return afs_dir_find_block_fail(iter, dvnode);
    }
    if fq.is_null() || blpos < fpos {
        fq = (*dvnode).directory;
        slot = 0;
        fpos = 0;
    }
    while !fq.is_null() {
        while slot < folioq_count(fq) {
            let fsize = folioq_folio_size(fq, slot);
            if blend <= fpos + fsize {
                let folio = folioq_folio(fq, slot);
                if folio_pos(folio) != fpos {
                    return afs_dir_find_block_fail(iter, dvnode);
                }
                (*iter).fq = fq;
                (*iter).fq_slot = slot;
                (*iter).fpos = fpos;
                (*iter).block = kmap_local_folio(folio, blpos - fpos) as *mut afs_xdr_dir_block;
                return (*iter).block;
            }
            fpos += fsize;
            slot += 1;
        }
        fq = (*fq).next;
        slot = 0;
    }
    afs_dir_find_block_fail(iter, dvnode)
}

unsafe fn afs_dir_find_block_fail(
    iter: *mut afs_dir_iter,
    dvnode: *mut afs_vnode,
) -> *mut afs_xdr_dir_block {
    (*iter).fq = core::ptr::null_mut();
    (*iter).fq_slot = 0;
    afs_invalidate_dir(dvnode, afs_dir_invalid_edit_get_block);
    core::ptr::null_mut()
}

/* Search through a directory bucket. */
pub unsafe fn afs_dir_search_bucket(
    iter: *mut afs_dir_iter,
    name: *const qstr,
    fid: *mut afs_fid,
) -> i32 {
    let meta = afs_dir_find_block(iter, 0);
    if meta.is_null() { return -ESTALE; }
    let mut entry = ntohs((*meta).meta.hashtable[((*iter).bucket & (AFS_DIR_HASHTBL_SIZE - 1)) as usize]) as u32;
    let mut ret = -ESTALE;
    while entry != 0 {
        let blnum = entry / AFS_DIR_SLOTS_PER_BLOCK;
        let slot = entry % AFS_DIR_SLOTS_PER_BLOCK;
        let resv = if blnum == 0 { AFS_DIR_RESV_BLOCKS0 } else { AFS_DIR_RESV_BLOCKS };
        if slot < resv { break; }
        let block = afs_dir_find_block(iter, blnum as usize);
        if block.is_null() { break; }
        let dire = &(*block).dirents[slot as usize];
        if slot + (*iter).nr_slots <= AFS_DIR_SLOTS_PER_BLOCK
            && core::slice::from_raw_parts(dire.u.name, (*name).len as usize)
                == core::slice::from_raw_parts((*name).name, (*name).len as usize)
            && dire.u.name[(*name).len as usize] == 0 {
            (*fid).vnode = ntohl(dire.u.vnode);
            (*fid).unique = ntohl(dire.u.unique);
            ret = entry as i32;
            break;
        }
        (*iter).prev_entry = entry;
        entry = ntohs(dire.u.hash_next) as u32;
        (*iter).loop_check -= 1;
        if (*iter).loop_check == 0 { break; }
    }
    if ret == -ESTALE { afs_invalidate_dir((*iter).dvnode, afs_dir_invalid_iter_stale); }
    ret
}

/* Search the appropriate hash chain in the contents of an AFS directory. */
pub unsafe fn afs_dir_search(
    dvnode: *mut afs_vnode,
    name: *const qstr,
    fid: *mut afs_fid,
    dir_version: *mut afs_dataversion_t,
) -> i32 {
    let mut iter: afs_dir_iter = core::mem::zeroed();
    iter.dvnode = dvnode;
    let mut retry_limit = 3;
    if !afs_dir_init_iter(&mut iter, name) { return -ENOENT; }
    loop {
        retry_limit -= 1;
        if retry_limit < 0 { return -ESTALE; }
        let ret = afs_read_dir(dvnode, core::ptr::null_mut());
        if ret < 0 {
            if ret != -ESTALE || test_bit(AFS_VNODE_DELETED, &(*dvnode).flags) { return ret; }
            continue;
        }
        *dir_version = inode_peek_iversion_raw(&(*dvnode).netfs.inode);
        let result = afs_dir_search_bucket(&mut iter, name, fid);
        up_read(&(*dvnode).validate_lock);
        if result != -ESTALE { return result; }
        afs_dir_reset_iter(&mut iter);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
