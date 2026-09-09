// SPDX-License-Identifier: GPL-2.0
/* Generic part */

#[repr(C)]
struct Indirect {
    p: *mut block_t,
    key: block_t,
    bh: *mut buffer_head,
}

static mut pointers_lock: rwlock_t = RWLOCK_INITIALIZER;

unsafe fn add_chain(p: *mut Indirect, bh: *mut buffer_head, v: *mut block_t) {
    (*p).p = v;
    (*p).key = *v;
    (*p).bh = bh;
}

unsafe fn verify_chain(mut from: *mut Indirect, to: *mut Indirect) -> c_int {
    while from <= to && (*from).key == *(*from).p {
        from = from.add(1);
    }
    (from > to) as c_int
}

unsafe fn block_end(bh: *mut buffer_head) -> *mut block_t {
    ((*bh).b_data as *mut u8).add((*bh).b_size as usize) as *mut block_t
}

unsafe fn get_branch(
    inode: *mut inode,
    mut depth: c_int,
    offsets: *mut c_int,
    chain: *mut Indirect,
    err: *mut c_int,
) -> *mut Indirect {
    let sb = (*inode).i_sb;
    let mut p = chain;
    let mut bh: *mut buffer_head;

    *err = 0;
    add_chain(chain, core::ptr::null_mut(), i_data(inode).add(*offsets as usize));
    if (*p).key == 0 { return p; }
    while { depth -= 1; depth != 0 } {
        bh = sb_bread(sb, block_to_cpu((*p).key));
        if bh.is_null() { *err = -EIO; return p; }
        read_lock(&mut pointers_lock);
        if verify_chain(chain, p) == 0 {
            read_unlock(&mut pointers_lock);
            brelse(bh);
            *err = -EAGAIN;
            return p;
        }
        p = p.add(1);
        add_chain(p, bh, ((*bh).b_data as *mut block_t).add(*offsets.add((chain as usize - chain as usize) + 1) as usize));
        offsets = offsets.add(1);
        read_unlock(&mut pointers_lock);
        if (*p).key == 0 { return p; }
    }
    core::ptr::null_mut()
}

unsafe fn alloc_branch(inode: *mut inode, num: c_int, offsets: *mut c_int, branch: *mut Indirect) -> c_int {
    let mut n = 0;
    let mut parent = minix_new_block(inode);
    let mut err = -ENOSPC;
    (*branch).key = cpu_to_block(parent);
    if parent != 0 {
        while n + 1 < num {
            n += 1;
            let nr = minix_new_block(inode);
            if nr == 0 { break; }
            (*branch.add(n as usize)).key = cpu_to_block(nr);
            let bh = sb_getblk((*inode).i_sb, parent);
            if bh.is_null() { minix_free_block(inode, nr); err = -ENOMEM; break; }
            lock_buffer(bh);
            memset((*bh).b_data, 0, (*bh).b_size as usize);
            (*branch.add(n as usize)).bh = bh;
            (*branch.add(n as usize)).p = ((*bh).b_data as *mut block_t).add(*offsets.add(n as usize) as usize);
            *(*branch.add(n as usize)).p = (*branch.add(n as usize)).key;
            set_buffer_uptodate(bh);
            unlock_buffer(bh);
            mmb_mark_buffer_dirty(bh, &mut minix_i(inode).i_metadata_bhs);
            parent = nr;
        }
    }
    if n + 1 == num { return 0; }
    let mut i = 1;
    while i < n { bforget((*branch.add(i as usize)).bh); i += 1; }
    i = 0;
    while i < n { minix_free_block(inode, block_to_cpu((*branch.add(i as usize)).key)); i += 1; }
    err
}

unsafe fn splice_branch(inode: *mut inode, chain: *mut Indirect, where_: *mut Indirect, num: c_int) -> c_int {
    write_lock(&mut pointers_lock);
    if verify_chain(chain, where_.sub(1)) == 0 || *(*where_).p != 0 { write_unlock(&mut pointers_lock); let mut i=1; while i<num { bforget((*where_.add(i as usize)).bh); i+=1; } i=0; while i<num { minix_free_block(inode, block_to_cpu((*where_.add(i as usize)).key)); i+=1; } return -EAGAIN; }
    *(*where_).p = (*where_).key;
    write_unlock(&mut pointers_lock);
    inode_set_ctime_current(inode);
    if !(*where_).bh.is_null() { mmb_mark_buffer_dirty((*where_).bh, &mut minix_i(inode).i_metadata_bhs); }
    mark_inode_dirty(inode);
    0
}

unsafe fn get_block(inode: *mut inode, block: sector_t, bh: *mut buffer_head, create: c_int) -> c_int {
    let mut err = -EIO;
    let mut offsets = [0 as c_int; DEPTH];
    let mut chain = [core::mem::zeroed::<Indirect>(); DEPTH];
    let depth = block_to_path(inode, block, offsets.as_mut_ptr());
    if depth == 0 { return err; }
    loop {
        let mut partial = get_branch(inode, depth, offsets.as_mut_ptr(), chain.as_mut_ptr(), &mut err);
        if partial.is_null() {
            map_bh(bh, (*inode).i_sb, block_to_cpu(chain[(depth-1) as usize].key));
            partial = chain.as_mut_ptr().add((depth-1) as usize);
            while partial > chain.as_mut_ptr() { partial=partial.sub(1); brelse((*partial).bh); }
            return err;
        }
        if create == 0 || err == -EIO {
            while partial > chain.as_mut_ptr() { brelse((*partial).bh); partial=partial.sub(1); }
            return err;
        }
        if err == -EAGAIN { while partial > chain.as_mut_ptr() { brelse((*partial).bh); partial=partial.sub(1); } continue; }
        let left = depth - ((partial as usize - chain.as_mut_ptr() as usize) / core::mem::size_of::<Indirect>()) as c_int;
        err = alloc_branch(inode, left, offsets.add(((partial as usize-chain.as_mut_ptr() as usize)/core::mem::size_of::<Indirect>()) as usize), partial);
        if err != 0 { while partial > chain.as_mut_ptr() { brelse((*partial).bh); partial=partial.sub(1); } return err; }
        if splice_branch(inode, chain.as_mut_ptr(), partial, left) < 0 { while partial > chain.as_mut_ptr() { brelse((*partial).bh); partial=partial.sub(1); } continue; }
        set_buffer_new(bh);
        map_bh(bh, (*inode).i_sb, block_to_cpu(chain[(depth-1) as usize].key));
        while partial > chain.as_mut_ptr() { partial=partial.sub(1); brelse((*partial).bh); }
        return err;
    }
}

unsafe fn all_zeroes(mut p: *mut block_t, q: *mut block_t) -> c_int { while p < q { if *p != 0 { return 0; } p=p.add(1); } 1 }

unsafe fn free_data(inode: *mut inode, mut p: *mut block_t, q: *mut block_t) { while p < q { let nr=block_to_cpu(*p); if nr != 0 { *p=0; minix_free_block(inode,nr); } p=p.add(1); } }

unsafe fn free_branches(inode: *mut inode, mut p: *mut block_t, q: *mut block_t, mut depth: c_int) { if depth != 0 { depth-=1; while p<q { let nr=block_to_cpu(*p); if nr!=0 { *p=0; let bh=sb_bread((*inode).i_sb,nr); if !bh.is_null() { free_branches(inode,(*bh).b_data as *mut block_t,block_end(bh),depth); bforget(bh); minix_free_block(inode,nr); mark_inode_dirty(inode); } } p=p.add(1); } } else { free_data(inode,p,q); } }

unsafe fn find_shared(inode: *mut inode, depth: c_int, offsets: *mut c_int, chain: *mut Indirect, top: *mut block_t) -> *mut Indirect {
    *top=0; let mut k=depth; while k>1 && *offsets.add((k-1) as usize)==0 { k-=1; }
    let mut err=0; let partial0=get_branch(inode,k,offsets,chain,&mut err); let mut partial=partial0;
    write_lock(&mut pointers_lock);
    if partial.is_null() { partial=chain.add((k-1) as usize); }
    if (*partial).key==0 && *(*partial).p!=0 { write_unlock(&mut pointers_lock); return partial; }
    let mut p=partial; while p>chain && all_zeroes((*p.sub(1)).bh.as_ref().unwrap().b_data as *mut block_t,(*p).p)!=0 { p=p.sub(1); }
    if p==chain.add((k-1) as usize) && p>chain { (*p).p=(*p).p.sub(1); } else { *top=*(*p).p; *(*p).p=0; }
    write_unlock(&mut pointers_lock); while partial>p { partial=partial.sub(1); brelse((*partial).bh); } partial
}

// The remaining routines retain the original C algorithm and rely on declarations supplied by other translation units.
unsafe fn truncate(inode: *mut inode) { let sb=(*inode).i_sb; let idata=i_data(inode); let mut offsets=[0 as c_int; DEPTH]; let mut chain=[core::mem::zeroed::<Indirect>(); DEPTH]; let iblock=((*inode).i_size + (*sb).s_blocksize as i64 - 1) >> (*sb).s_blocksize_bits; block_truncate_page((*inode).i_mapping,(*inode).i_size,get_block); let n=block_to_path(inode,iblock,offsets.as_mut_ptr()); if n==0{return;} if n==1 { free_data(inode,idata.add(offsets[0] as usize),idata.add(DIRECT as usize)); } else { let mut nr=0; let partial=find_shared(inode,n,offsets.as_mut_ptr(),chain.as_mut_ptr(),&mut nr); if nr!=0 { free_branches(inode,&mut nr,&mut nr.add(1),n-1); } while partial>chain.as_mut_ptr() { free_branches(inode,(*partial).p.add(1),block_end((*partial).bh),n-1); brelse((*partial).bh); partial=partial.sub(1); } } inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode)); mark_inode_dirty(inode); }

unsafe fn nblocks(size: loff_t, sb: *mut super_block) -> c_uint { let k=(*sb).s_blocksize_bits-10; let mut blocks=((size+(*sb).s_blocksize as i64-1) >> (BLOCK_SIZE_BITS+k)) as c_uint; let mut res=blocks; let mut direct=DIRECT as c_uint; let mut i=DEPTH as c_uint; while {i-=1;i!=0} && blocks>direct { blocks-=direct; blocks+=(*sb).s_blocksize as c_uint/core::mem::size_of::<block_t>() as c_uint-1; blocks/=(*sb).s_blocksize as c_uint/core::mem::size_of::<block_t>() as c_uint; res+=blocks; direct=1; } res }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
