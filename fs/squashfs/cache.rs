// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct translation of cache.c. Kernel and Squashfs declarations are supplied by dependencies. */

use core::ffi::c_void;

#[repr(C)]
pub struct squashfs_cache_wait {
    pub wait: wait_queue_entry_t,
    pub block: u64,
    pub capacity_wake: bool,
}

unsafe fn squashfs_cache_wake_function(wait: *mut wait_queue_entry_t, mode: u32, sync: i32, key: *mut c_void) -> i32 {
    let cache_wait = container_of!(wait, squashfs_cache_wait, wait);
    let block = key as *mut u64;
    if !block.is_null() && (*cache_wait).block != *block { return 0; }
    write_once!((*cache_wait).capacity_wake, block.is_null());
    default_wake_function(wait, mode, sync, key);
    list_del_init_careful!(&mut (*wait).entry);
    1
}

unsafe fn squashfs_cache_wake_block(cache: *mut squashfs_cache, block: u64) {
    __wake_up!(&mut (*cache).wait_queue, TASK_NORMAL, 0, &block as *const u64 as *mut c_void);
}

pub unsafe fn squashfs_cache_get(sb: *mut super_block, cache: *mut squashfs_cache, block: u64, length: i32) -> *mut squashfs_cache_entry {
    let mut i: i32;
    let mut n: i32;
    let mut entry: *mut squashfs_cache_entry = core::ptr::null_mut();
    let mut capacity_wake = false;
    spin_lock!(&mut (*cache).lock);
    loop {
        let mut pending: bool;
        let mut wake_next: bool;
        let mut wake_block: bool;
        i = (*cache).curr_blk;
        n = 0;
        while n < (*cache).entries {
            if (*(*cache).entry.add(i as usize)).block == block { (*cache).curr_blk = i; break; }
            i = (i + 1) % (*cache).entries; n += 1;
        }
        if n == (*cache).entries {
            if (*cache).unused == 0 {
                let mut wait: squashfs_cache_wait = core::mem::zeroed();
                wait.block = block;
                wait.capacity_wake = false;
                init_wait_func!(&mut wait.wait, squashfs_cache_wake_function);
                (*cache).num_waiters += 1;
                prepare_to_wait_exclusive!(&mut (*cache).wait_queue, &mut wait.wait, TASK_UNINTERRUPTIBLE);
                spin_unlock!(&mut (*cache).lock);
                schedule!();
                finish_wait!(&mut (*cache).wait_queue, &mut wait.wait);
                capacity_wake = read_once!(wait.capacity_wake);
                spin_lock!(&mut (*cache).lock);
                (*cache).num_waiters -= 1;
                continue;
            }
            i = (*cache).next_blk;
            n = 0;
            while n < (*cache).entries {
                if (*(*cache).entry.add(i as usize)).refcount == 0 { break; }
                i = (i + 1) % (*cache).entries; n += 1;
            }
            (*cache).next_blk = (i + 1) % (*cache).entries;
            entry = (*cache).entry.add(i as usize);
            (*cache).unused -= 1;
            (*entry).block = block; (*entry).refcount = 1; (*entry).pending = 1;
            (*entry).num_waiters = 0; (*entry).error = 0;
            wake_block = (*cache).num_waiters > 0;
            spin_unlock!(&mut (*cache).lock);
            if wake_block { squashfs_cache_wake_block(cache, block); }
            (*entry).length = squashfs_read_data(sb, block, length, &mut (*entry).next_index, (*entry).actor);
            spin_lock!(&mut (*cache).lock);
            if (*entry).length < 0 { (*entry).error = (*entry).length; }
            (*entry).pending = 0;
            if (*entry).num_waiters != 0 { spin_unlock!(&mut (*cache).lock); wake_up_all!(&mut (*entry).wait_queue); }
            else { spin_unlock!(&mut (*cache).lock); }
            break;
        }
        entry = (*cache).entry.add(i as usize);
        if (*entry).refcount == 0 { (*cache).unused -= 1; capacity_wake = false; }
        (*entry).refcount += 1;
        pending = (*entry).pending;
        if pending { (*entry).num_waiters += 1; }
        wake_next = capacity_wake && (*cache).unused != 0 && (*cache).num_waiters != 0;
        spin_unlock!(&mut (*cache).lock);
        if wake_next { wake_up!(&mut (*cache).wait_queue); }
        if pending { wait_event!(&mut (*entry).wait_queue, !(*entry).pending); }
        break;
    }
    trace!("Got %s %d, start block %lld, refcount %d, error %d\n", (*cache).name, i, (*entry).block, (*entry).refcount, (*entry).error);
    if (*entry).error != 0 { error!("Unable to read %s cache entry [%llx]\n", (*cache).name, block); }
    entry
}

pub unsafe fn squashfs_cache_put(entry: *mut squashfs_cache_entry) {
    let cache = (*entry).cache;
    spin_lock!(&mut (*cache).lock);
    (*entry).refcount -= 1;
    if (*entry).refcount == 0 { (*cache).unused += 1; if (*cache).num_waiters != 0 { spin_unlock!(&mut (*cache).lock); wake_up!(&mut (*cache).wait_queue); return; } }
    spin_unlock!(&mut (*cache).lock);
}

pub unsafe fn squashfs_cache_delete(cache: *mut squashfs_cache) {
    if is_err!(cache) || cache.is_null() { return; }
    for i in 0..(*cache).entries { let entry = cache.as_ref().unwrap().entry.add(i as usize); if !(*entry).data.is_null() { for j in 0..(*cache).pages { kfree!(*(*entry).data.add(j as usize)); } kfree!((*entry).data); } kfree!((*entry).actor); }
    kfree!((*cache).entry); kfree!(cache);
}

pub unsafe fn squashfs_cache_init(name: *mut i8, entries: i32, block_size: i32) -> *mut squashfs_cache {
    if entries == 0 { return core::ptr::null_mut(); }
    let cache = kzalloc_obj!(squashfs_cache);
    if cache.is_null() { error!("Failed to allocate %s cache\n", name); return err_ptr!(-ENOMEM); }
    (*cache).entry = kzalloc_objs!(squashfs_cache_entry, entries);
    if (*cache).entry.is_null() { error!("Failed to allocate %s cache\n", name); squashfs_cache_delete(cache); return err_ptr!(-ENOMEM); }
    (*cache).curr_blk=0; (*cache).next_blk=0; (*cache).unused=entries; (*cache).entries=entries; (*cache).block_size=block_size; (*cache).pages=core::cmp::max(block_size >> PAGE_SHIFT, 1); (*cache).name=name; (*cache).num_waiters=0; spin_lock_init!(&mut (*cache).lock); init_waitqueue_head!(&mut (*cache).wait_queue);
    for i in 0..entries { let e=cache.as_ref().unwrap().entry.add(i as usize); init_waitqueue_head!(&mut (*e).wait_queue); (*e).cache=cache; (*e).block=SQUASHFS_INVALID_BLK; (*e).data=kcalloc!((*cache).pages, core::mem::size_of::<*mut c_void>(), GFP_KERNEL); if (*e).data.is_null() { squashfs_cache_delete(cache); return err_ptr!(-ENOMEM); } for j in 0..(*cache).pages { *(*e).data.add(j as usize)=kmalloc!(PAGE_SIZE, GFP_KERNEL); if (*(*e).data.add(j as usize)).is_null() { squashfs_cache_delete(cache); return err_ptr!(-ENOMEM); } } (*e).actor=squashfs_page_actor_init((*e).data, (*cache).pages, 0); if (*e).actor.is_null() { squashfs_cache_delete(cache); return err_ptr!(-ENOMEM); } }
    cache
}

pub unsafe fn squashfs_copy_data(mut buffer: *mut c_void, entry: *mut squashfs_cache_entry, mut offset: i32, length: i32) -> i32 {
    let mut remaining=length; if length==0 || offset<0 { return 0; } if buffer.is_null() { return core::cmp::min(length, (*entry).length-offset); }
    while offset < (*entry).length { let buff=(*(*entry).data.add((offset/PAGE_SIZE) as usize)).add((offset%PAGE_SIZE) as usize); let bytes=core::cmp::min((*entry).length-offset, PAGE_SIZE-(offset%PAGE_SIZE)); if bytes>=remaining { memcpy!(buffer,buff,remaining); remaining=0; break; } memcpy!(buffer,buff,bytes); buffer=buffer.add(bytes as usize); remaining-=bytes; offset+=bytes; } length-remaining
}

pub unsafe fn squashfs_read_metadata(sb:*mut super_block, mut buffer:*mut c_void, block:*mut u64, offset:*mut i32, mut length:i32)->i32 { let msblk=(*sb).s_fs_info as *mut squashfs_sb_info; let mut res=length; let mut entry:*mut squashfs_cache_entry; trace!("Entered squashfs_read_metadata [%llx:%x]\n",*block,*offset); if length<0 || *offset<0 || *offset>=SQUASHFS_METADATA_SIZE { return -EIO; } while length!=0 { entry=squashfs_cache_get(sb,(*msblk).block_cache,*block,0); if (*entry).error!=0 { res=(*entry).error; squashfs_cache_put(entry); return res; } if *offset>=(*entry).length { squashfs_cache_put(entry); return -EIO; } let bytes=squashfs_copy_data(buffer,entry,*offset,length); if !buffer.is_null(){buffer=buffer.add(bytes as usize);} length-=bytes;*offset+=bytes;if *offset==(*entry).length {*block=(*entry).next_index;*offset=0;} squashfs_cache_put(entry); } res }

pub unsafe fn squashfs_get_fragment(sb:*mut super_block,start_block:u64,length:i32)->*mut squashfs_cache_entry { squashfs_cache_get(sb,(*((*sb).s_fs_info as *mut squashfs_sb_info)).fragment_cache,start_block,length) }
pub unsafe fn squashfs_get_datablock(sb:*mut super_block,start_block:u64,length:i32)->*mut squashfs_cache_entry { squashfs_cache_get(sb,(*((*sb).s_fs_info as *mut squashfs_sb_info)).read_page,start_block,length) }

pub unsafe fn squashfs_read_table(sb:*mut super_block,block:u64,length:i32)->*mut c_void { let pages=(length+PAGE_SIZE-1)>>PAGE_SHIFT; let table=kmalloc!(length,GFP_KERNEL); if table.is_null(){return err_ptr!(-ENOMEM);} let data=kcalloc!(pages,core::mem::size_of::<*mut c_void>(),GFP_KERNEL); if data.is_null(){kfree!(table);return err_ptr!(-ENOMEM);} let actor=squashfs_page_actor_init(data,pages,length); if actor.is_null(){kfree!(data);kfree!(table);return err_ptr!(-ENOMEM);} for i in 0..pages {*data.add(i as usize)=table.add((i*PAGE_SIZE) as usize);} let res=squashfs_read_data(sb,block,length|SQUASHFS_COMPRESSED_BIT_BLOCK,core::ptr::null_mut(),actor); kfree!(data);kfree!(actor); if res<0 {kfree!(table);return err_ptr!(res);} table }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
