// SPDX-License-Identifier: GPL-2.0-only
/* mm/readahead.c - address_space-level file readahead. */
// Linux headers, trace points, and internal dependencies are supplied externally.

pub unsafe fn file_ra_state_init(ra: *mut file_ra_state, mapping: *mut address_space) {
    (*ra).ra_pages = (*inode_to_bdi((*mapping).host)).ra_pages;
    (*ra).prev_pos = -1;
}

unsafe fn read_pages(rac: *mut readahead_control) {
    let aops = (*(*rac).mapping).a_ops;
    let mut folio: *mut folio;
    let mut plug: blk_plug = core::mem::zeroed();
    if readahead_count(rac) == 0 { return; }
    if (*rac)._workingset { psi_memstall_enter(&mut (*rac)._pflags); }
    blk_start_plug(&mut plug);
    if !(*aops).readahead.is_none() {
        ((*aops).readahead.unwrap())(rac);
        while { folio = readahead_folio(rac); !folio.is_null() } {
            folio_get(folio); filemap_remove_folio(folio); folio_unlock(folio); folio_put(folio);
        }
    } else {
        while { folio = readahead_folio(rac); !folio.is_null() } {
            ((*aops).read_folio)((*rac).file, folio);
        }
    }
    blk_finish_plug(&mut plug);
    if (*rac)._workingset { psi_memstall_leave(&mut (*rac)._pflags); }
    (*rac)._workingset = false;
    BUG_ON(readahead_count(rac));
}

unsafe fn ractl_alloc_folio(ractl: *mut readahead_control, gfp_mask: gfp_t, order: u32) -> *mut folio {
    let f = filemap_alloc_folio(gfp_mask, order, core::ptr::null_mut());
    if !f.is_null() && (*ractl).dropbehind { __folio_set_dropbehind(f); }
    f
}

pub unsafe fn page_cache_ra_unbounded(ractl: *mut readahead_control, mut nr_to_read: u64, lookahead_size: u64) {
    let mapping = (*ractl).mapping;
    let mut index = readahead_index(ractl);
    let gfp_mask = readahead_gfp_mask(mapping);
    let mut mark = ULONG_MAX;
    let mut i = 0u64;
    let min_nrpages = mapping_min_folio_nrpages(mapping) as u64;
    let nofs = memalloc_nofs_save();
    lockdep_assert_held(&mut (*mapping).invalidate_lock);
    trace_page_cache_ra_unbounded((*mapping).host, index, nr_to_read, lookahead_size);
    index = mapping_align_index(mapping, index);
    if lookahead_size <= nr_to_read {
        let ra_folio_index = round_up(readahead_index(ractl) + nr_to_read - lookahead_size, min_nrpages);
        mark = ra_folio_index - index;
    }
    nr_to_read += readahead_index(ractl) - index;
    (*ractl)._index = index;
    while i < nr_to_read {
        let mut f = xa_load(&(*mapping).i_pages, index + i);
        if !f.is_null() && !xa_is_value(f) {
            read_pages(ractl); (*ractl)._index += min_nrpages; i = (*ractl)._index - index; continue;
        }
        f = ractl_alloc_folio(ractl, gfp_mask, mapping_min_folio_order(mapping));
        if f.is_null() { break; }
        let ret = filemap_add_folio(mapping, f, index + i, gfp_mask);
        if ret < 0 {
            folio_put(f); if ret == -ENOMEM { break; }
            read_pages(ractl); (*ractl)._index += min_nrpages; i = (*ractl)._index - index; continue;
        }
        if i == mark { folio_set_readahead(f); }
        (*ractl)._workingset |= folio_test_workingset(f);
        (*ractl)._nr_pages += min_nrpages; i += min_nrpages;
    }
    read_pages(ractl); memalloc_nofs_restore(nofs);
}

unsafe fn do_page_cache_ra(ractl: *mut readahead_control, mut nr_to_read: u64, mut lookahead_size: u64) {
    let mapping = (*ractl).mapping; let index = readahead_index(ractl);
    let isize = i_size_read((*mapping).host); if isize == 0 { return; }
    let end_index = ((isize - 1) >> PAGE_SHIFT) as u64; if index > end_index { return; }
    if nr_to_read > end_index - index { nr_to_read = end_index - index + 1; lookahead_size = 0; }
    filemap_invalidate_lock_shared(mapping); page_cache_ra_unbounded(ractl, nr_to_read, lookahead_size); filemap_invalidate_unlock_shared(mapping);
}

pub unsafe fn force_page_cache_ra(ractl: *mut readahead_control, mut nr_to_read: u64) {
    let mapping = (*ractl).mapping; let ra = (*ractl).ra; let bdi = inode_to_bdi((*mapping).host);
    if (*mapping).a_ops.readahead.is_none() && (*mapping).a_ops.read_folio.is_none() { return; }
    let max_pages = max((*bdi).io_pages, (*ra).ra_pages); nr_to_read = min(nr_to_read, max_pages);
    while nr_to_read != 0 { let chunk = min((2 * 1024 * 1024) / PAGE_SIZE, nr_to_read); do_page_cache_ra(ractl, chunk, 0); nr_to_read -= chunk; }
}

unsafe fn get_init_ra_size(size: u64, maxv: u64) -> u64 { let mut n = roundup_pow_of_two(size); if n <= maxv / 32 { n *= 4; } else if n <= maxv / 4 { n *= 2; } else { n = maxv; } n }
unsafe fn get_next_ra_size(ra: *mut file_ra_state, maxv: u64) -> u64 { let cur = (*ra).size; if cur < maxv / 16 { 4 * cur } else if cur <= maxv / 2 { 2 * cur } else { maxv } }

unsafe fn ra_alloc_folio(ractl: *mut readahead_control, index: u64, mark: u64, order: u32, gfp: gfp_t) -> i32 {
    let f = ractl_alloc_folio(ractl, gfp, order); if f.is_null() { return -ENOMEM; }
    let mark = round_down(mark, 1u64 << order); if index == mark { folio_set_readahead(f); }
    let err = filemap_add_folio((*ractl).mapping, f, index, gfp); if err != 0 { folio_put(f); return err; }
    (*ractl)._nr_pages += 1u64 << order; (*ractl)._workingset |= folio_test_workingset(f); 0
}

pub unsafe fn page_cache_ra_order(ractl: *mut readahead_control, ra: *mut file_ra_state) {
    let mapping = (*ractl).mapping; let start = readahead_index(ractl); let mut index = start;
    let min_order = mapping_min_folio_order(mapping); let mut limit = ((i_size_read((*mapping).host)-1)>>PAGE_SHIFT) as u64;
    let mut mark; let mut err = 0; let gfp = readahead_gfp_mask(mapping); let mut order = (*ra).order;
    trace_page_cache_ra_order((*mapping).host, start, ra);
    if !mapping_large_folio_support(mapping) { (*ra).order=0; return; }
    if limit > index + (*ra).size - 1 { limit=index+(*ra).size-1; mark=index+(*ra).size-(*ra).async_size; } else { mark=ULONG_MAX; }
    order=min(mapping_max_folio_order(mapping),order); order=min(order,ilog2((*ra).size)); order=max(order,min_order); (*ra).order=order;
    let nofs=memalloc_nofs_save(); filemap_invalidate_lock_shared(mapping); (*ractl)._index=mapping_align_index(mapping,index); index=readahead_index(ractl);
    while index<=limit { let mut o=order; if index & ((1u64<<o)-1)!=0 { o=__ffs(index); } while o>min_order && index+(1u64<<o)-1>limit { o-=1; } err=ra_alloc_folio(ractl,index,mark,o,gfp); if err!=0 { break; } index+=1u64<<o; }
    read_pages(ractl); filemap_invalidate_unlock_shared(mapping); memalloc_nofs_restore(nofs);
    if err!=0 && (*ra).size > index-start { do_page_cache_ra(ractl,(*ra).size-(index-start),(*ra).async_size); }
}

unsafe fn ractl_max_pages(r: *mut readahead_control, req: u64) -> u64 { let b=inode_to_bdi((*(*r).mapping).host); let mut m=(*(*r).ra).ra_pages; if req>m && (*b).io_pages>m { m=min(req,(*b).io_pages); } m }

pub unsafe fn page_cache_sync_ra(r: *mut readahead_control, mut req: u64) {
    let index=readahead_index(r); let forced=!(*r).file.is_null() && ((*(*r).file).f_mode & FMODE_RANDOM)!=0; let ra=(*r).ra; let mut forced=forced;
    trace_page_cache_sync_ra((*(*r).mapping).host,index,ra,req);
    if (*ra).ra_pages==0 || blk_cgroup_congested() { if (*r).file.is_null(){return;} req=1; forced=true; }
    if forced { force_page_cache_ra(r,req); return; }
    let maxp=ractl_max_pages(r,req); let prev=(((*ra).prev_pos as u64)>>PAGE_SHIFT); if index==0 || req>maxp || index-prev<=1 { (*ra).start=index; (*ra).size=get_init_ra_size(req,maxp); (*ra).async_size=if (*ra).size>req {(*ra).size-req} else {(*ra).size>>1}; } else { rcu_read_lock(); let miss=page_cache_prev_miss((*r).mapping,index-1,maxp); rcu_read_unlock(); let cont=index-miss-1; if cont<=req { do_page_cache_ra(r,req,0); return; } let cont=if miss==ULONG_MAX {cont*2}else{cont}; (*ra).start=index; (*ra).size=min(cont+req,maxp); (*ra).async_size=1; }
    (*ra).order=0; (*r)._index=(*ra).start; page_cache_ra_order(r,ra);
}

pub unsafe fn page_cache_async_ra(r: *mut readahead_control, folio: *mut folio, req: u64) {
    let ra=(*r).ra; if (*ra).ra_pages==0 || folio_test_writeback(folio) { return; }
    let index=readahead_index(r); trace_page_cache_async_ra((*(*r).mapping).host,index,ra,req); folio_clear_readahead(folio); if blk_cgroup_congested(){return;}
    let maxp=ractl_max_pages(r,req); let expected=round_down((*ra).start+(*ra).size-(*ra).async_size,folio_nr_pages(folio));
    if index==expected { (*ra).start+=(*ra).size; (*ra).size=max((*ra).size,get_next_ra_size(ra,maxp)); } else { rcu_read_lock(); let start=page_cache_next_miss((*r).mapping,index+1,maxp); rcu_read_unlock(); if start==0 || start-index>maxp{return;} (*ra).start=start; (*ra).size=get_next_ra_size(ra,start-index+req); }
    (*ra).order+=2; let align=1u64<<min((*ra).order,ffs(maxp)-1); let end=(*ra).start+(*ra).size; let aligned=round_down(end,align); if aligned>(*ra).start {(*ra).size-=end-aligned;} (*ra).async_size=(*ra).size; (*r)._index=(*ra).start; page_cache_ra_order(r,ra);
}

pub unsafe fn ksys_readahead(fd: i32, offset: i64, count: usize) -> isize { let f=fd_file(fd); if f.is_null() || ((*f).f_mode&FMODE_READ)==0{return -EBADF;} if (*f).f_mapping.is_null() || (*(*f).f_mapping).a_ops.is_null(){return -EINVAL;} let inode=file_inode(f); if !S_ISREG((*inode).i_mode)&&!S_ISBLK((*inode).i_mode)||IS_ANON_FILE(inode){return -EINVAL;} vfs_fadvise(f,offset,count,POSIX_FADV_WILLNEED) }

pub unsafe fn readahead_expand(r: *mut readahead_control, new_start: i64, mut new_len: usize) {
    let m=(*r).mapping; let ra=(*r).ra; let g=readahead_gfp_mask(m); let minp=mapping_min_folio_nrpages(m); let order=mapping_min_folio_order(m); let ni=(new_start as u64)/PAGE_SIZE;
    while (*r)._index>ni { let mut index=(*r)._index-1; let mut f=xa_load(&(*m).i_pages,index); if !f.is_null()&&!xa_is_value(f){return;} f=ractl_alloc_folio(r,g,order); if f.is_null(){return;} index=mapping_align_index(m,index); if filemap_add_folio(m,f,index,g)<0{folio_put(f);return;} if folio_test_workingset(f)&&!(*r)._workingset{(*r)._workingset=true;psi_memstall_enter(&mut (*r)._pflags);} (*r)._nr_pages+=minp; (*r)._index=(*f).index; }
    new_len += new_start as u64 - readahead_pos(r); let new_nr=DIV_ROUND_UP(new_len,PAGE_SIZE);
    while (*r)._nr_pages<new_nr { let mut index=(*r)._index+(*r)._nr_pages; let mut f=xa_load(&(*m).i_pages,index); if !f.is_null()&&!xa_is_value(f){return;} f=ractl_alloc_folio(r,g,order); if f.is_null(){return;} index=mapping_align_index(m,index); if filemap_add_folio(m,f,index,g)<0{folio_put(f);return;} if folio_test_workingset(f)&&!(*r)._workingset{(*r)._workingset=true;psi_memstall_enter(&mut (*r)._pflags);} (*r)._nr_pages+=minp; if !ra.is_null(){(*ra).size+=minp;(*ra).async_size+=minp;} }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
