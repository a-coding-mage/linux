// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of zstd.c. External kernel and Btrfs symbols are
+ * intentionally left as dependencies supplied by other translation units. */

const ZSTD_BTRFS_MAX_WINDOWLOG: i32 = 17;
const ZSTD_BTRFS_MAX_INPUT: usize = 1usize << ZSTD_BTRFS_MAX_WINDOWLOG;
const ZSTD_BTRFS_DEFAULT_LEVEL: i32 = 3;
const ZSTD_BTRFS_MIN_LEVEL: i32 = -15;
const ZSTD_BTRFS_MAX_LEVEL: usize = 15;
const ZSTD_BTRFS_RECLAIM_JIFFIES: usize = 307 * HZ;

unsafe fn zstd_get_btrfs_parameters(level: i32, src_len: usize) -> zstd_parameters {
    let mut params = zstd_get_params(level, src_len);
    if params.cParams.windowLog > ZSTD_BTRFS_MAX_WINDOWLOG { params.cParams.windowLog = ZSTD_BTRFS_MAX_WINDOWLOG; }
    WARN_ON(src_len > ZSTD_BTRFS_MAX_INPUT);
    params
}

#[repr(C)]
struct workspace {
    mem: *mut core::ffi::c_void,
    size: usize,
    buf: *mut i8,
    level: i32,
    req_level: i32,
    last_used: usize,
    list: list_head,
    lru_list: list_head,
    in_buf: zstd_in_buffer,
    out_buf: zstd_out_buffer,
    params: zstd_parameters,
}

#[repr(C)]
struct zstd_workspace_manager {
    lock: spinlock_t,
    lru_list: list_head,
    idle_ws: [list_head; ZSTD_BTRFS_MAX_LEVEL],
    active_map: usize,
    wait: wait_queue_head_t,
    timer: timer_list,
}

static mut zstd_ws_mem_sizes: [usize; ZSTD_BTRFS_MAX_LEVEL] = [0; ZSTD_BTRFS_MAX_LEVEL];

unsafe fn list_to_workspace(list: *mut list_head) -> *mut workspace {
    container_of!(list, workspace, list)
}
unsafe fn clip_level(level: i32) -> usize { if level - 1 > 0 { (level - 1) as usize } else { 0 } }

unsafe extern "C" fn zstd_reclaim_timer_fn(timer: *mut timer_list) {
    let zwsm = container_of!(timer, zstd_workspace_manager, timer);
    let reclaim_threshold = jiffies - ZSTD_BTRFS_RECLAIM_JIFFIES;
    let mut pos: *mut list_head;
    let mut next: *mut list_head;
    spin_lock(&mut (*zwsm).lock);
    if list_empty(&(*zwsm).lru_list) { spin_unlock(&mut (*zwsm).lock); return; }
    list_for_each_prev_safe!(pos, next, &mut (*zwsm).lru_list) {
        let victim = container_of!(pos, workspace, lru_list);
        if time_after((*victim).last_used, reclaim_threshold) { break; }
        if (*victim).req_level != 0 { continue; }
        let level = (*victim).level as usize;
        list_del(&mut (*victim).lru_list);
        list_del(&mut (*victim).list);
        zstd_free_workspace(&mut (*victim).list);
        if list_empty(&(*zwsm).idle_ws[level]) { clear_bit(level, &mut (*zwsm).active_map); }
    }
    if !list_empty(&(*zwsm).lru_list) { mod_timer(&mut (*zwsm).timer, jiffies + ZSTD_BTRFS_RECLAIM_JIFFIES); }
    spin_unlock(&mut (*zwsm).lock);
}

unsafe fn zstd_calc_ws_mem_sizes() {
    let mut max_size = 0usize;
    for level in ZSTD_BTRFS_MIN_LEVEL..=ZSTD_BTRFS_MAX_LEVEL as i32 {
        if level == 0 { continue; }
        let params = zstd_get_btrfs_parameters(level, ZSTD_BTRFS_MAX_INPUT);
        let level_size = core::cmp::max(zstd_cstream_workspace_bound(&params.cParams), zstd_dstream_workspace_bound(ZSTD_BTRFS_MAX_INPUT));
        max_size = core::cmp::max(max_size, level_size);
        zstd_ws_mem_sizes[clip_level(level)] = max_size;
    }
}

pub unsafe fn zstd_alloc_workspace_manager(fs_info: *mut btrfs_fs_info) -> i32 {
    ASSERT!((*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] == core::ptr::null_mut());
    let zwsm = kzalloc_obj::<zstd_workspace_manager>();
    if zwsm.is_null() { return -ENOMEM; }
    zstd_calc_ws_mem_sizes();
    spin_lock_init(&mut (*zwsm).lock); init_waitqueue_head(&mut (*zwsm).wait);
    timer_setup(&mut (*zwsm).timer, zstd_reclaim_timer_fn, 0);
    INIT_LIST_HEAD(&mut (*zwsm).lru_list);
    for i in 0..ZSTD_BTRFS_MAX_LEVEL { INIT_LIST_HEAD(&mut (*zwsm).idle_ws[i]); }
    (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] = zwsm as *mut _;
    let ws = zstd_alloc_workspace(fs_info, ZSTD_BTRFS_MAX_LEVEL as i32);
    if IS_ERR(ws) { btrfs_warn(core::ptr::null_mut(), c"cannot preallocate zstd compression workspace"); }
    else { set_bit(ZSTD_BTRFS_MAX_LEVEL - 1, &mut (*zwsm).active_map); list_add(ws, &mut (*zwsm).idle_ws[ZSTD_BTRFS_MAX_LEVEL - 1]); }
    0
}

pub unsafe fn zstd_free_workspace_manager(fs_info: *mut btrfs_fs_info) {
    let zwsm = (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] as *mut zstd_workspace_manager;
    if zwsm.is_null() { return; }
    (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] = core::ptr::null_mut();
    spin_lock_bh(&mut (*zwsm).lock);
    for i in 0..ZSTD_BTRFS_MAX_LEVEL { while !list_empty(&(*zwsm).idle_ws[i]) {
        let ws = container_of!((*zwsm).idle_ws[i].next, workspace, list);
        list_del(&mut (*ws).list); list_del(&mut (*ws).lru_list); zstd_free_workspace(&mut (*ws).list);
    }}
    spin_unlock_bh(&mut (*zwsm).lock); timer_delete_sync(&mut (*zwsm).timer); kfree(zwsm as *mut _);
}

unsafe fn zstd_find_workspace(fs_info: *mut btrfs_fs_info, level: i32) -> *mut list_head {
    let zwsm = (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] as *mut zstd_workspace_manager;
    let mut i = clip_level(level); ASSERT!(!zwsm.is_null()); spin_lock_bh(&mut (*zwsm).lock);
    for_each_set_bit_from!(i, &mut (*zwsm).active_map, ZSTD_BTRFS_MAX_LEVEL) {
        if !list_empty(&(*zwsm).idle_ws[i]) {
            let ws = (*zwsm).idle_ws[i].next; let workspace = list_to_workspace(ws); list_del_init(ws);
            (*workspace).req_level = level;
            if clip_level(level) == (*workspace).level as usize { list_del(&mut (*workspace).lru_list); }
            if list_empty(&(*zwsm).idle_ws[i]) { clear_bit(i, &mut (*zwsm).active_map); }
            spin_unlock_bh(&mut (*zwsm).lock); return ws;
        }
    }
    spin_unlock_bh(&mut (*zwsm).lock); core::ptr::null_mut()
}

pub unsafe fn zstd_get_workspace(fs_info: *mut btrfs_fs_info, mut level: i32) -> *mut list_head {
    let zwsm = (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] as *mut zstd_workspace_manager; ASSERT!(!zwsm.is_null());
    if level == 0 { level = 1; }
    loop { let ws = zstd_find_workspace(fs_info, level); if !ws.is_null() { return ws; }
        let nofs = memalloc_nofs_save(); let ws = zstd_alloc_workspace(fs_info, level); memalloc_nofs_restore(nofs);
        if !IS_ERR(ws) { return ws; }
        let mut wait = DEFINE_WAIT!(); prepare_to_wait(&mut (*zwsm).wait, &mut wait, TASK_UNINTERRUPTIBLE); schedule(); finish_wait(&mut (*zwsm).wait, &mut wait);
    }
}

pub unsafe fn zstd_put_workspace(fs_info: *mut btrfs_fs_info, ws: *mut list_head) {
    let zwsm = (*fs_info).compr_wsm[BTRFS_COMPRESS_ZSTD] as *mut zstd_workspace_manager; let workspace = list_to_workspace(ws); ASSERT!(!zwsm.is_null()); spin_lock_bh(&mut (*zwsm).lock);
    if clip_level((*workspace).req_level) == (*workspace).level as usize {
        if list_empty(&(*zwsm).idle_ws[ZSTD_BTRFS_MAX_LEVEL - 1]) { INIT_LIST_HEAD(&mut (*workspace).lru_list); }
        else { (*workspace).last_used = jiffies; list_add(&mut (*workspace).lru_list, &mut (*zwsm).lru_list); if !timer_pending(&(*zwsm).timer) { mod_timer(&mut (*zwsm).timer, jiffies + ZSTD_BTRFS_RECLAIM_JIFFIES); } }
    }
    set_bit((*workspace).level as usize, &mut (*zwsm).active_map); list_add(&mut (*workspace).list, &mut (*zwsm).idle_ws[(*workspace).level as usize]); (*workspace).req_level = 0; spin_unlock_bh(&mut (*zwsm).lock);
    if (*workspace).level as usize == clip_level(ZSTD_BTRFS_MAX_LEVEL as i32) { cond_wake_up(&mut (*zwsm).wait); }
}

pub unsafe fn zstd_free_workspace(ws: *mut list_head) { let workspace = container_of!(ws, workspace, list); kvfree((*workspace).mem); kfree((*workspace).buf as *mut _); kfree(workspace as *mut _); }

pub unsafe fn zstd_alloc_workspace(fs_info: *mut btrfs_fs_info, level: i32) -> *mut list_head {
    let workspace = kzalloc_obj::<workspace>(); if workspace.is_null() { return ERR_PTR(-ENOMEM); }
    (*workspace).size = zstd_ws_mem_sizes[clip_level(level)]; (*workspace).level = clip_level(level) as i32; (*workspace).req_level = level; (*workspace).last_used = jiffies;
    (*workspace).mem = kvmalloc((*workspace).size, GFP_KERNEL | __GFP_NOWARN); (*workspace).buf = kmalloc((*fs_info).sectorsize, GFP_KERNEL);
    if (*workspace).mem.is_null() || (*workspace).buf.is_null() { zstd_free_workspace(&mut (*workspace).list); return ERR_PTR(-ENOMEM); }
    INIT_LIST_HEAD(&mut (*workspace).list); INIT_LIST_HEAD(&mut (*workspace).lru_list); &mut (*workspace).list
}

// Compression/decompression entry points retain the C control flow; kernel helper
// and zstd API declarations are supplied by the surrounding translation units.
pub unsafe fn zstd_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32 {
    let inode = (*cb).bbio.inode; let fs_info = (*(*inode).root).fs_info; let workspace = container_of!(ws, workspace, list); let mapping = (*inode).vfs_inode.i_mapping; let bio = &mut (*cb).bbio.bio; let mut ret = 0; let mut in_folio: *mut folio = core::ptr::null_mut(); let mut out_folio: *mut folio = core::ptr::null_mut(); let mut tot_in=0usize; let mut tot_out=0usize; let start=(*cb).start; let len=(*cb).len as usize; let end=start+len as u64; let min_folio_size=btrfs_min_folio_size(fs_info);
    (*workspace).params=zstd_get_btrfs_parameters((*workspace).req_level,len); let stream=zstd_init_cstream(&(*workspace).params,len,(*workspace).mem,(*workspace).size); if stream.is_null(){ btrfs_err(fs_info,c"zstd compression init failed"); return -EIO; }
    ret=btrfs_compress_filemap_get_folio(mapping,start,&mut in_folio); if ret<0{return ret;} (*workspace).in_buf.src=kmap_local_folio(in_folio,offset_in_folio(in_folio,start)); (*workspace).in_buf.pos=0; (*workspace).in_buf.size=btrfs_calc_input_length(in_folio,end,start);
    out_folio=btrfs_alloc_compr_folio(fs_info,GFP_NOFS); if out_folio.is_null(){ret=-ENOMEM;goto compress_out;} (*workspace).out_buf.dst=folio_address(out_folio); (*workspace).out_buf.pos=0; (*workspace).out_buf.size=min_folio_size;
    loop { let r=zstd_compress_stream(stream,&mut (*workspace).out_buf,&mut (*workspace).in_buf); if zstd_is_error(r){ret=-EIO;break;} if tot_in+(*workspace).in_buf.pos>((*fs_info).sectorsize*2) && tot_in+(*workspace).in_buf.pos<tot_out+(*workspace).out_buf.pos {ret=-E2BIG;break;} if (*workspace).out_buf.pos>=(*workspace).out_buf.size {tot_out+=min_folio_size;if tot_out>=len||bio_add_folio(bio,out_folio,folio_size(out_folio),0)==0{ret=-E2BIG;break;} out_folio=btrfs_alloc_compr_folio(fs_info,GFP_NOFS);if out_folio.is_null(){ret=-ENOMEM;break;} (*workspace).out_buf.dst=folio_address(out_folio);(*workspace).out_buf.pos=0;(*workspace).out_buf.size=min_folio_size;} if tot_in+(*workspace).in_buf.pos>=len{tot_in+=(*workspace).in_buf.pos;break;} if (*workspace).in_buf.pos>=(*workspace).in_buf.size {tot_in+=(*workspace).in_buf.size;let cur=start+tot_in as u64;kunmap_local((*workspace).in_buf.src);folio_put(in_folio);ret=btrfs_compress_filemap_get_folio(mapping,cur,&mut in_folio);if ret<0{break;}(*workspace).in_buf.src=kmap_local_folio(in_folio,offset_in_folio(in_folio,cur));(*workspace).in_buf.pos=0;(*workspace).in_buf.size=btrfs_calc_input_length(in_folio,end,cur);} }
    if ret==0 {loop{let r=zstd_end_stream(stream,&mut (*workspace).out_buf);if zstd_is_error(r){ret=-EIO;break;}if r==0{tot_out+=(*workspace).out_buf.pos;if tot_out>=len||bio_add_folio(bio,out_folio,(*workspace).out_buf.pos,0)==0{ret=-E2BIG;}else{out_folio=core::ptr::null_mut();}break;}tot_out+=min_folio_size;if tot_out>=len||bio_add_folio(bio,out_folio,folio_size(out_folio),0)==0{ret=-E2BIG;break;}out_folio=btrfs_alloc_compr_folio(fs_info,GFP_NOFS);if out_folio.is_null(){ret=-ENOMEM;break;}(*workspace).out_buf.dst=folio_address(out_folio);(*workspace).out_buf.pos=0;(*workspace).out_buf.size=min_folio_size;}} if ret==0&&tot_out>=tot_in{ret=-E2BIG;}
compress_out: if !out_folio.is_null(){btrfs_free_compr_folio(out_folio);} if !(*workspace).in_buf.src.is_null(){kunmap_local((*workspace).in_buf.src);folio_put(in_folio);} ret
}

pub unsafe fn zstd_decompress_bio(ws:*mut list_head,cb:*mut compressed_bio)->i32{let workspace=container_of!(ws,workspace,list);let fs_info=cb_to_fs_info(cb);let srclen=bio_get_size(&(*cb).bbio.bio);let mut ret=0;let mut total_out=0usize;let min=btrfs_min_folio_size(fs_info);let mut fi=core::mem::zeroed::<folio_iter>();bio_first_folio(&mut fi,&(*cb).bbio.bio,0);if fi.folio.is_null(){return -EINVAL;}let stream=zstd_init_dstream(ZSTD_BTRFS_MAX_INPUT,(*workspace).mem,(*workspace).size);if stream.is_null(){return -EIO;}(*workspace).in_buf.src=kmap_local_folio(fi.folio,0);(*workspace).in_buf.pos=0;(*workspace).in_buf.size=core::cmp::min(srclen,min);(*workspace).out_buf.dst=(*workspace).buf as *mut _;(*workspace).out_buf.pos=0;(*workspace).out_buf.size=(*fs_info).sectorsize;loop{let r=zstd_decompress_stream(stream,&mut (*workspace).out_buf,&mut (*workspace).in_buf);if zstd_is_error(r){ret=-EIO;break;}let start=total_out;total_out+=(*workspace).out_buf.pos;(*workspace).out_buf.pos=0;ret=btrfs_decompress_buf2page((*workspace).out_buf.dst,total_out-start,cb,start);if ret==0||(*workspace).in_buf.pos>=srclen||r==0{break;}if (*workspace).in_buf.pos==(*workspace).in_buf.size{kunmap_local((*workspace).in_buf.src);bio_next_folio(&mut fi,&(*cb).bbio.bio);if fi.folio.is_null(){ret=-EIO;break;}(*workspace).in_buf.src=kmap_local_folio(fi.folio,0);(*workspace).in_buf.pos=0;(*workspace).in_buf.size=core::cmp::min(srclen-(*workspace).in_buf.pos,min);}}if !(*workspace).in_buf.src.is_null(){kunmap_local((*workspace).in_buf.src);}ret}

pub unsafe fn zstd_decompress(ws:*mut list_head,data_in:*const u8,dest_folio:*mut folio,dest_pgoff:usize,srclen:usize,destlen:usize)->i32{let workspace=container_of!(ws,workspace,list);let fs_info=btrfs_sb(folio_inode(dest_folio)->i_sb);let stream=zstd_init_dstream(ZSTD_BTRFS_MAX_INPUT,(*workspace).mem,(*workspace).size);if stream.is_null(){folio_zero_range(dest_folio,dest_pgoff,destlen);return -EIO;}(*workspace).in_buf=zstd_in_buffer{src:data_in,pos:0,size:srclen};(*workspace).out_buf=zstd_out_buffer{dst:(*workspace).buf as *mut _,pos:0,size:(*fs_info).sectorsize};let ret=zstd_decompress_stream(stream,&mut (*workspace).out_buf,&mut (*workspace).in_buf);let to_copy=(*workspace).out_buf.pos;if !zstd_is_error(ret){memcpy_to_folio(dest_folio,dest_pgoff,(*workspace).out_buf.dst,to_copy);}if to_copy<destlen{folio_zero_range(dest_folio,dest_pgoff+to_copy,destlen-to_copy);return -EIO;}ret as i32}

#[repr(C)]
pub struct btrfs_compress_levels { pub min_level:i32,pub max_level:i32,pub default_level:i32 }
#[no_mangle] pub static btrfs_zstd_compress:btrfs_compress_levels=btrfs_compress_levels{min_level:ZSTD_BTRFS_MIN_LEVEL,max_level:ZSTD_BTRFS_MAX_LEVEL as i32,default_level:ZSTD_BTRFS_DEFAULT_LEVEL};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
