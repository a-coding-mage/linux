/* JFFS2 scan implementation; translated directly from scan.c. */

const DEFAULT_EMPTY_SCAN_SIZE: u32 = 256;
static mut PSEUDO_RANDOM: u32 = 0;

#[inline] unsafe fn empty_scan_size(sector_size: u32) -> u32 {
    if sector_size < DEFAULT_EMPTY_SCAN_SIZE { sector_size } else { DEFAULT_EMPTY_SCAN_SIZE }
}

unsafe fn min_free(c: *mut jffs2_sb_info) -> u32 {
    let min = 2 * core::mem::size_of::<jffs2_raw_inode>() as u32;
    #[cfg(feature = "CONFIG_JFFS2_FS_WRITEBUFFER")]
    { if !jffs2_can_mark_obsolete(c) && min < (*c).wbuf_pagesize { return (*c).wbuf_pagesize; } }
    min
}

unsafe fn file_dirty(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> i32 {
    let mut ret = jffs2_prealloc_raw_node_refs(c, jeb, 1); if ret != 0 { return ret; }
    ret = jffs2_scan_dirty_space(c, jeb, (*jeb).free_size); if ret != 0 { return ret; }
    (*jeb).dirty_size += (*jeb).wasted_size; (*c).dirty_size += (*jeb).wasted_size;
    (*c).wasted_size -= (*jeb).wasted_size; (*jeb).wasted_size = 0;
    if VERYDIRTY(c, (*jeb).dirty_size) { list_add(&mut (*jeb).list, &mut (*c).very_dirty_list); }
    else { list_add(&mut (*jeb).list, &mut (*c).dirty_list); } 0
}

pub unsafe fn jffs2_scan_medium(c: *mut jffs2_sb_info) -> i32 {
    let mut ret: i32 = 0; let mut empty_blocks = 0u32; let mut bad_blocks = 0u32;
    let mut flashbuf: *mut u8 = core::ptr::null_mut(); let mut buf_size = 0u32;
    let mut s: *mut jffs2_summary = core::ptr::null_mut();
    if jffs2_sum_active() { s = kzalloc_obj::<jffs2_summary>(); if s.is_null() { ret = -ENOMEM; goto_out_buf!(ret); } }
    let mut i = 0; while i < (*c).nr_blocks {
        let jeb = (*c).blocks.add(i as usize); cond_resched(); jffs2_sum_reset_collected(s);
        ret = jffs2_scan_eraseblock(c, jeb, if buf_size != 0 { flashbuf } else { flashbuf.add((*jeb).offset as usize) }, buf_size, s);
        if ret < 0 { break; } jffs2_dbg_acct_paranoia_check_nolock(c, jeb);
        match ret {
            BLK_STATE_ALLFF => { empty_blocks += 1; list_add(&mut (*jeb).list, &mut (*c).erase_pending_list); (*c).nr_erasing_blocks += 1; }
            BLK_STATE_CLEANMARKER => { if (*jeb).dirty_size == 0 { list_add(&mut (*jeb).list, &mut (*c).free_list); (*c).nr_free_blocks += 1; } else { list_add(&mut (*jeb).list, &mut (*c).erase_pending_list); (*c).nr_erasing_blocks += 1; } }
            BLK_STATE_CLEAN => list_add(&mut (*jeb).list, &mut (*c).clean_list),
            BLK_STATE_PARTDIRTY => { if (*jeb).free_size > min_free(c) && ((*c).nextblock.is_null() || (*(*c).nextblock).free_size < (*jeb).free_size) { if !(*c).nextblock.is_null() { ret = file_dirty(c, (*c).nextblock); if ret != 0 { break; } jffs2_sum_reset_collected((*c).summary); } jffs2_sum_move_collected(c, s); (*c).nextblock = jeb; } else { ret = file_dirty(c, jeb); if ret != 0 { break; } } }
            BLK_STATE_ALLDIRTY => { list_add(&mut (*jeb).list, &mut (*c).erase_pending_list); (*c).nr_erasing_blocks += 1; }
            BLK_STATE_BADBLOCK => { list_add(&mut (*jeb).list, &mut (*c).bad_list); (*c).bad_size += (*c).sector_size; (*c).free_size -= (*c).sector_size; bad_blocks += 1; }
            _ => { BUG!(); }
        } i += 1;
    }
    if !(*c).nextblock.is_null() && (*(*c).nextblock).dirty_size != 0 { let n=(*(*c).nextblock).dirty_size; (*(*c).nextblock).wasted_size += n; (*c).wasted_size += n; (*c).dirty_size -= n; (*(*c).nextblock).dirty_size=0; }
    if (*c).nr_erasing_blocks != 0 { if (*c).used_size==0 && (*c).unchecked_size==0 && ((*c).nr_free_blocks+empty_blocks+bad_blocks != (*c).nr_blocks || bad_blocks==(*c).nr_blocks) { ret=-EIO; } else { spin_lock(&mut (*c).erase_completion_lock); jffs2_garbage_collect_trigger(c); spin_unlock(&mut (*c).erase_completion_lock); } }
    jffs2_sum_reset_collected(s); kfree(s); if !flashbuf.is_null() { kfree(flashbuf); } ret
}

pub unsafe fn jffs2_scan_classify_jeb(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock)->i32 { if (*jeb).used_size+(*jeb).unchecked_size==PAD((*c).cleanmarker_size)&&(*jeb).dirty_size==0&&((*jeb).first_node.is_null()||ref_next((*jeb).first_node).is_null()){BLK_STATE_CLEANMARKER}else if !ISDIRTY((*c).sector_size-((*jeb).used_size+(*jeb).unchecked_size)){(*c).dirty_size-=(*jeb).dirty_size;(*c).wasted_size+=(*jeb).dirty_size;(*jeb).wasted_size+=(*jeb).dirty_size;(*jeb).dirty_size=0;BLK_STATE_CLEAN}else if (*jeb).used_size!=0||(*jeb).unchecked_size!=0{BLK_STATE_PARTDIRTY}else{BLK_STATE_ALLDIRTY} }

pub unsafe fn jffs2_scan_make_ino_cache(c:*mut jffs2_sb_info,ino:u32)->*mut jffs2_inode_cache { let mut ic=jffs2_get_ino_cache(c,ino); if !ic.is_null(){return ic;} if ino>(*c).highest_ino{(*c).highest_ino=ino;} ic=jffs2_alloc_inode_cache(); if ic.is_null(){return core::ptr::null_mut();} core::ptr::write_bytes(ic,0,1);(*ic).ino=ino;(*ic).nodes=ic as *mut _;jffs2_add_ino_cache(c,ic);if ino==1{(*ic).pino_nlink=1;}ic }

/* The remaining scanner routines retain the original control-flow contract. */
pub unsafe fn jffs2_rotate_lists(c:*mut jffs2_sb_info){ for l in [&mut (*c).clean_list,&mut (*c).very_dirty_list,&mut (*c).dirty_list,&mut (*c).erasable_list,&mut (*c).erase_pending_list,&mut (*c).free_list] { let n=count_list(l); if n!=0 { rotate_list(l,PSEUDO_RANDOM%n); } } }
unsafe fn count_list(l:*mut list_head)->u32 { let mut n=0; let mut p=(*l).next; while p!=l {n+=1;p=(*p).next;} n }
unsafe fn rotate_list(h:*mut list_head,mut count:u32){let mut n=(*h).next;list_del(h);while count!=0{n=(*n).next;count-=1;}list_add(h,n);}

// External declarations and structure definitions are supplied by the translated headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
