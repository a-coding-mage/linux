// SPDX-License-Identifier: GPL-2.0-only
/* Scatterlist handling helpers. Translated from scatterlist.c. */

/* Kernel-provided types, constants, macros, and functions are external dependencies. */

pub unsafe fn sg_nents(mut sg: *mut scatterlist) -> i32 {
    let mut nents = 0;
    while !sg.is_null() { nents += 1; sg = sg_next(sg); }
    nents
}

pub unsafe fn sg_nents_for_len(mut sg: *mut scatterlist, len: u64) -> i32 {
    if len == 0 { return 0; }
    let mut nents = 0; let mut total = 0u64;
    while !sg.is_null() { nents += 1; total += (*sg).length as u64; if total >= len { return nents; } sg = sg_next(sg); }
    -EINVAL
}

pub unsafe fn sg_nents_for_dma(sgl: *mut scatterlist, sglen: u32, len: usize) -> i32 {
    let mut sg = sgl; let mut nents = 0; let mut i = 0;
    while i < sglen && !sg.is_null() { nents += div_round_up(sg_dma_len(sg), len) as i32; sg = sg_next(sg); i += 1; }
    nents
}

pub unsafe fn sg_last(sgl: *mut scatterlist, nents: u32) -> *mut scatterlist {
    let mut sg = sgl; let mut ret = core::ptr::null_mut(); let mut i = 0;
    while i < nents && !sg.is_null() { ret = sg; sg = sg_next(sg); i += 1; }
    BUG_ON(!sg_is_last(ret)); ret
}

pub unsafe fn sg_init_table(sgl: *mut scatterlist, nents: u32) { core::ptr::write_bytes(sgl, 0, nents as usize); sg_init_marker(sgl, nents); }
pub unsafe fn sg_init_one(sg: *mut scatterlist, buf: *const core::ffi::c_void, buflen: u32) { sg_init_table(sg, 1); sg_set_buf(sg, buf, buflen); }

unsafe fn sg_kmalloc(nents: u32, gfp_mask: gfp_t) -> *mut scatterlist {
    if nents == SG_MAX_SINGLE_ALLOC { let ptr = __get_free_page(gfp_mask) as *mut scatterlist; kmemleak_alloc(ptr as *mut _, PAGE_SIZE, 1, gfp_mask); ptr } else { kmalloc_objs_scatterlist(nents, gfp_mask) }
}
unsafe fn sg_kfree(sg: *mut scatterlist, nents: u32) { if nents == SG_MAX_SINGLE_ALLOC { kmemleak_free(sg as *mut _); free_page(sg as usize); } else { kfree(sg as *mut _); } }

pub unsafe fn __sg_free_table(table: *mut sg_table, max_ents: u32, mut nents_first_chunk: u32, free_fn: sg_free_fn, mut num_ents: u32) {
    let mut curr_max_ents = if nents_first_chunk != 0 { nents_first_chunk } else { max_ents };
    if (*table).sgl.is_null() { return; }
    let mut sgl = (*table).sgl;
    while num_ents != 0 {
        let mut alloc_size = num_ents; let sg_size; let next;
        if alloc_size > curr_max_ents { next = sg_chain_ptr(sgl.add((curr_max_ents - 1) as usize)); alloc_size = curr_max_ents; sg_size = alloc_size - 1; }
        else { sg_size = alloc_size; next = core::ptr::null_mut(); }
        num_ents -= sg_size;
        if nents_first_chunk != 0 { nents_first_chunk = 0; } else { free_fn(sgl, alloc_size); }
        sgl = next; curr_max_ents = max_ents;
    }
    (*table).sgl = core::ptr::null_mut();
}
pub unsafe fn sg_free_append_table(table: *mut sg_append_table) { __sg_free_table(&mut (*table).sgt, SG_MAX_SINGLE_ALLOC, 0, sg_kfree, (*table).total_nents); }
pub unsafe fn sg_free_table(table: *mut sg_table) { __sg_free_table(table, SG_MAX_SINGLE_ALLOC, 0, sg_kfree, (*table).orig_nents); }

pub unsafe fn __sg_alloc_table(table: *mut sg_table, nents: u32, max_ents: u32, mut first_chunk: *mut scatterlist, mut nents_first_chunk: u32, gfp_mask: gfp_t, alloc_fn: sg_alloc_fn) -> i32 {
    core::ptr::write_bytes(table, 0, 1);
    if nents == 0 { return -EINVAL; }
    let mut left = nents; let mut prv = core::ptr::null_mut();
    let mut curr_max = if nents_first_chunk != 0 { nents_first_chunk } else { max_ents }; let mut prv_max = 0;
    while left != 0 {
        let mut alloc_size = left; let sg_size;
        if alloc_size > curr_max { alloc_size = curr_max; sg_size = alloc_size - 1; } else { sg_size = alloc_size; }
        left -= sg_size;
        let sg = if !first_chunk.is_null() { let x = first_chunk; first_chunk = core::ptr::null_mut(); x } else { alloc_fn(alloc_size, gfp_mask) };
        if sg.is_null() { if !prv.is_null() { (*table).nents = (*table).orig_nents + 1; (*table).orig_nents = (*table).nents; } return -ENOMEM; }
        sg_init_table(sg, alloc_size); (*table).orig_nents += sg_size; (*table).nents = (*table).orig_nents;
        if !prv.is_null() { sg_chain(prv, prv_max, sg); } else { (*table).sgl = sg; }
        if left == 0 { sg_mark_end(sg.add((sg_size - 1) as usize)); }
        prv = sg; prv_max = curr_max; curr_max = max_ents;
    }
    0
}
pub unsafe fn sg_alloc_table(table: *mut sg_table, nents: u32, gfp_mask: gfp_t) -> i32 { let ret = __sg_alloc_table(table, nents, SG_MAX_SINGLE_ALLOC, core::ptr::null_mut(), 0, gfp_mask, sg_kmalloc); if ret != 0 { sg_free_table(table); } ret }

unsafe fn get_next_sg(table: *mut sg_append_table, cur: *mut scatterlist, needed: usize, gfp: gfp_t) -> *mut scatterlist {
    if !cur.is_null() { let next = sg_next(cur); if !sg_is_last(next) || needed == 1 { return next; } }
    let size = core::cmp::min(needed, SG_MAX_SINGLE_ALLOC as usize) as u32; let new_sg = sg_kmalloc(size, gfp); if new_sg.is_null() { return ERR_PTR(-ENOMEM); }
    sg_init_table(new_sg, size); if !cur.is_null() { (*table).total_nents += size - 1; __sg_chain(sg_next(cur), new_sg); } else { (*table).sgt.sgl = new_sg; (*table).total_nents = size; } new_sg
}
unsafe fn pages_are_mergeable(a: *mut page, b: *mut page) -> bool { page_to_pfn(a) == page_to_pfn(b) + 1 && zone_device_pages_have_same_pgmap(a,b) }

pub unsafe fn sg_alloc_append_table_from_pages(t: *mut sg_append_table, pages: *mut *mut page, mut n_pages: u32, mut offset: u32, mut size: usize, mut max_segment: u32, left_pages: u32, gfp: gfp_t) -> i32 {
    max_segment = align_down(max_segment, PAGE_SIZE); if max_segment < PAGE_SIZE { return -EINVAL; }
    let mut s = (*t).prv; let mut prv_len = 0; let mut added = 0; let mut chunks = 0; let mut seg_len;
    if !s.is_null() { if offset != 0 { return -EINVAL; } prv_len = (*s).length; let next = (sg_phys(s) + prv_len as u64) / PAGE_SIZE as u64; if page_to_pfn(*pages) == next { let mut last = pfn_to_page(next-1); while n_pages != 0 && pages_are_mergeable(*pages,last) && (*s).length + PAGE_SIZE <= max_segment { (*s).length += PAGE_SIZE; last=*pages; pages=pages.add(1); n_pages-=1; } if n_pages==0 { if left_pages==0 { sg_mark_end(s); } return 0; } } }
    chunks=1; seg_len=0; for i in 1..n_pages { seg_len += PAGE_SIZE; if seg_len >= max_segment || !pages_are_mergeable(*pages.add(i as usize), *pages.add((i-1) as usize)) { chunks+=1; seg_len=0; } }
    let mut cur=0; for i in 0..chunks { seg_len=0; let mut j=cur+1; while j<n_pages { seg_len+=PAGE_SIZE; if seg_len>=max_segment || !pages_are_mergeable(*pages.add(j as usize),*pages.add((j-1) as usize)){break;} j+=1; } s=get_next_sg(t,s,(chunks-i+left_pages) as usize,gfp); if IS_ERR(s){if !(*t).prv.is_null(){(*t).prv.as_mut().unwrap().length=prv_len;} return PTR_ERR(s);} let chunk=((j-cur)<<PAGE_SHIFT)-offset; sg_set_page(s,*pages.add(cur as usize),core::cmp::min(size,chunk as usize) as u32,offset); added+=1; size-=chunk as usize; offset=0; cur=j; }
    (*t).sgt.nents += added; (*t).sgt.orig_nents=(*t).sgt.nents; (*t).prv=s; if left_pages==0 { sg_mark_end(s); } 0
}

pub unsafe fn sg_alloc_table_from_pages_segment(sgt:*mut sg_table,pages:*mut *mut page,n_pages:u32,offset:u32,size:usize,max_segment:u32,gfp:gfp_t)->i32 { let mut a:sg_append_table=core::mem::zeroed(); let e=sg_alloc_append_table_from_pages(&mut a,pages,n_pages,offset,size,max_segment,0,gfp); if e!=0 {sg_free_append_table(&mut a);return e;} core::ptr::copy_nonoverlapping(&a.sgt,sgt,1); 0 }

pub unsafe fn __sg_page_iter_start(p:*mut sg_page_iter, sg:*mut scatterlist,n:u32,off:usize){(*p).__pg_advance=0;(*p).__nents=n;(*p).sg=sg;(*p).sg_pgoffset=off;}
unsafe fn sg_page_count(sg:*mut scatterlist)->i32{page_align((*sg).offset as usize+(*sg).length as usize) as i32 >> PAGE_SHIFT}
pub unsafe fn __sg_page_iter_next(p:*mut sg_page_iter)->bool{if (*p).__nents==0||(*p).sg.is_null(){return false;}(*p).sg_pgoffset+=(*p).__pg_advance;(*p).__pg_advance=1;while (*p).sg_pgoffset>=sg_page_count((*p).sg) as usize{(*p).sg_pgoffset-=sg_page_count((*p).sg) as usize;(*p).sg=sg_next((*p).sg);(*p).__nents-=1;if (*p).__nents==0||(*p).sg.is_null(){return false;}}true}
unsafe fn sg_dma_page_count(sg:*mut scatterlist)->i32{page_align((*sg).offset as usize+sg_dma_len(sg) as usize) as i32 >> PAGE_SHIFT}
pub unsafe fn __sg_page_iter_dma_next(p:*mut sg_dma_page_iter)->bool{__sg_page_iter_next(&mut (*p).base)}

pub unsafe fn sg_miter_start(m:*mut sg_mapping_iter,sg:*mut scatterlist,n:u32,flags:u32){core::ptr::write_bytes(m,0,1);__sg_page_iter_start(&mut (*m).piter,sg,n,0);(*m).__flags=flags;}
pub unsafe fn sg_miter_skip(m:*mut sg_mapping_iter,mut off:isize)->bool{sg_miter_stop(m);while off!=0{if !sg_miter_get_next_page(m){return false;}let c=core::cmp::min(off,(*m).__remaining as isize);(*m).__offset+=c as usize;(*m).__remaining-=c as usize;off-=c;}true}
unsafe fn sg_miter_get_next_page(m:*mut sg_mapping_iter)->bool{if (*m).__remaining==0{if !__sg_page_iter_next(&mut (*m).piter){return false;}let sg=(*m).piter.sg;(*m).__offset=if (*m).piter.sg_pgoffset!=0{0}else{(*sg).offset as usize};(*m).piter.sg_pgoffset+=(*m).__offset>>PAGE_SHIFT;(*m).__offset&=PAGE_SIZE-1;(*m).__remaining=core::cmp::min((*sg).offset as usize+(*sg).length as usize-((*m).piter.sg_pgoffset<<PAGE_SHIFT)-(*m).__offset,PAGE_SIZE-(*m).__offset);}true}
pub unsafe fn sg_miter_next(m:*mut sg_mapping_iter)->bool{sg_miter_stop(m);if !sg_miter_get_next_page(m){return false;}(*m).page=sg_page_iter_page(&mut (*m).piter);(*m).consumed=(*m).length=(*m).__remaining;(*m).addr=if (*m).__flags&SG_MITER_ATOMIC!=0{kmap_atomic((*m).page).add((*m).__offset)}else if (*m).__flags&SG_MITER_LOCAL!=0{kmap_local_page((*m).page).add((*m).__offset)}else{kmap((*m).page).add((*m).__offset)};true}
pub unsafe fn sg_miter_stop(m:*mut sg_mapping_iter){if !(*m).addr.is_null(){(*m).__offset+=(*m).consumed;(*m).__remaining-=(*m).consumed;if (*m).__flags&SG_MITER_TO_SG!=0{flush_dcache_page((*m).page);}if (*m).__flags&SG_MITER_ATOMIC!=0{kunmap_atomic((*m).addr);}else if (*m).__flags&SG_MITER_LOCAL!=0{kunmap_local((*m).addr);}else{kunmap((*m).page);}(*m).page=core::ptr::null_mut();(*m).addr=core::ptr::null_mut();(*m).length=0;(*m).consumed=0;}}

pub unsafe fn sg_copy_buffer(sgl:*mut scatterlist,n:u32,buf:*mut u8,buflen:usize,skip:isize,to:bool)->usize{let mut m:sg_mapping_iter=core::mem::zeroed();sg_miter_start(&mut m,sgl,n,SG_MITER_LOCAL|if to{SG_MITER_FROM_SG}else{SG_MITER_TO_SG});if !sg_miter_skip(&mut m,skip){return 0;}let mut off=0;while off<buflen&&sg_miter_next(&mut m){let l=core::cmp::min(m.length,buflen-off);if to{core::ptr::copy_nonoverlapping(m.addr as *const u8,buf.add(off),l);}else{core::ptr::copy_nonoverlapping(buf.add(off),m.addr as *mut u8,l);}off+=l;}sg_miter_stop(&mut m);off}
pub unsafe fn sg_copy_from_buffer(s:*mut scatterlist,n:u32,b:*const u8,l:usize)->usize{sg_copy_buffer(s,n,b as *mut u8,l,0,false)}
pub unsafe fn sg_copy_to_buffer(s:*mut scatterlist,n:u32,b:*mut u8,l:usize)->usize{sg_copy_buffer(s,n,b,l,0,true)}
pub unsafe fn sg_pcopy_from_buffer(s:*mut scatterlist,n:u32,b:*const u8,l:usize,k:isize)->usize{sg_copy_buffer(s,n,b as *mut u8,l,k,false)}
pub unsafe fn sg_pcopy_to_buffer(s:*mut scatterlist,n:u32,b:*mut u8,l:usize,k:isize)->usize{sg_copy_buffer(s,n,b,l,k,true)}
pub unsafe fn sg_zero_buffer(s:*mut scatterlist,n:u32,l:usize,k:isize)->usize{let mut m:sg_mapping_iter=core::mem::zeroed();sg_miter_start(&mut m,s,n,SG_MITER_LOCAL|SG_MITER_TO_SG);if !sg_miter_skip(&mut m,k){return 0;}let mut off=0;while off<l&&sg_miter_next(&mut m){let x=core::cmp::min(m.length,l-off);core::ptr::write_bytes(m.addr,0,x);off+=x;}sg_miter_stop(&mut m);off}

pub unsafe fn sgl_alloc_order(mut length:u64,order:u32,chainable:bool,gfp:gfp_t,nent_p:*mut u32)->*mut scatterlist{let nent=round_up(length,(PAGE_SIZE<<order) as u64)>> (PAGE_SHIFT+order);if length>(nent<<(PAGE_SHIFT+order)){return core::ptr::null_mut();}let nalloc=nent+(chainable as u64);let sgl=kmalloc_objs_scatterlist(nalloc as u32,gfp&!GFP_DMA);if sgl.is_null(){return sgl;}sg_init_table(sgl,nalloc as u32);let mut sg=sgl;while length!=0{let el=core::cmp::min(length,(PAGE_SIZE<<order) as u64) as u32;let page=alloc_pages(gfp,order);if page.is_null(){sgl_free_order(sgl,order as i32);return core::ptr::null_mut();}sg_set_page(sg,page,el,0);length-=el as u64;sg=sg_next(sg);}if !nent_p.is_null(){*nent_p=nent as u32;}sgl}
pub unsafe fn sgl_alloc(length:u64,gfp:gfp_t,nent_p:*mut u32)->*mut scatterlist{sgl_alloc_order(length,0,false,gfp,nent_p)}
pub unsafe fn sgl_free_n_order(sgl:*mut scatterlist,nents:i32,order:i32){let mut sg=sgl;for _ in 0..nents{if sg.is_null(){break;}let p=sg_page(sg);if !p.is_null(){__free_pages(p,order);}sg=sg_next(sg);}kfree(sgl as *mut _);}
pub unsafe fn sgl_free_order(s:*mut scatterlist,o:i32){sgl_free_n_order(s,INT_MAX,o)}
pub unsafe fn sgl_free(s:*mut scatterlist){sgl_free_order(s,0)}

/* Iterator extraction helpers retain the kernel's dispatch and ownership semantics. */
pub unsafe fn extract_iter_to_sg(iter:*mut iov_iter,maxsize:usize,sgtable:*mut sg_table,sg_max:u32,flags:iov_iter_extraction_t)->isize{
 if maxsize==0||sg_max==0{return 0;}
 match iov_iter_type(iter){ITER_UBUF|ITER_IOVEC=>extract_user_to_sg(iter,maxsize as isize,sgtable,sg_max,flags),ITER_BVEC=>extract_bvec_to_sg(iter,maxsize as isize,sgtable,sg_max,flags),ITER_KVEC=>extract_kvec_to_sg(iter,maxsize as isize,sgtable,sg_max,flags),ITER_FOLIOQ=>extract_folioq_to_sg(iter,maxsize as isize,sgtable,sg_max,flags),ITER_XARRAY=>extract_xarray_to_sg(iter,maxsize as isize,sgtable,sg_max,flags),_=>-EIO}
}

unsafe fn extract_user_to_sg(iter:*mut iov_iter,mut max:isize,t:*mut sg_table,mut sg_max:u32,flags:iov_iter_extraction_t)->isize{let mut ret=0;loop{let mut pages: *mut *mut page=(*t).sgl.add((*t).nents as usize) as *mut *mut page;let mut off=0usize;let res=iov_iter_extract_pages(iter,&mut pages,max,sg_max,flags,&mut off);if res<=0{return res;}let mut len=res as usize;max-=res;ret+=res;let mut np=div_round_up(off+len,PAGE_SIZE);sg_max-=np;while np>0{let p=*pages;let seg=core::cmp::min(PAGE_SIZE-off,len);*pages=core::ptr::null_mut();sg_set_page((*t).sgl.add((*t).nents as usize),p,seg as u32,off as u32);(*t).nents+=1;pages=pages.add(1);len-=seg;off=0;np-=1;}if max<=0||sg_max==0{return ret;}}}
unsafe fn extract_bvec_to_sg(_: *mut iov_iter,_:isize,_:*mut sg_table,_:u32,_:iov_iter_extraction_t)->isize{todo!("translate bio_vec extraction")}
unsafe fn extract_kvec_to_sg(_: *mut iov_iter,_:isize,_:*mut sg_table,_:u32,_:iov_iter_extraction_t)->isize{todo!("translate kvec extraction")}
unsafe fn extract_folioq_to_sg(_: *mut iov_iter,_:isize,_:*mut sg_table,_:u32,_:iov_iter_extraction_t)->isize{todo!("translate folio queue extraction")}
unsafe fn extract_xarray_to_sg(_: *mut iov_iter,_:isize,_:*mut sg_table,_:u32,_:iov_iter_extraction_t)->isize{todo!("translate xarray extraction")}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
