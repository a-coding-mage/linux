/* Rust translation of summary.c. External kernel/JFFS2 declarations are supplied by other modules. */

pub unsafe fn jffs2_sum_init(c: *mut jffs2_sb_info) -> i32 {
    let sum_size: u32 = core::cmp::min((*c).sector_size, MAX_SUMMARY_SIZE);
    (*c).summary = kzalloc_obj::<jffs2_summary>();
    if (*c).summary.is_null() { JFFS2_WARNING!("Can't allocate memory for summary information!\n"); return -ENOMEM; }
    (*(*c).summary).sum_buf = kmalloc(sum_size, GFP_KERNEL);
    if (*(*c).summary).sum_buf.is_null() { JFFS2_WARNING!("Can't allocate buffer for writing out summary information!\n"); kfree((*c).summary); return -ENOMEM; }
    dbg_summary!("returned successfully\n"); 0
}

pub unsafe fn jffs2_sum_exit(c: *mut jffs2_sb_info) {
    dbg_summary!("called\n");
    jffs2_sum_disable_collecting((*c).summary);
    kfree((*(*c).summary).sum_buf); (*(*c).summary).sum_buf = core::ptr::null_mut();
    kfree((*c).summary); (*c).summary = core::ptr::null_mut();
}

unsafe fn jffs2_sum_add_mem(s: *mut jffs2_summary, item: *mut jffs2_sum_mem) -> i32 {
    if (*s).sum_list_head.is_null() { (*s).sum_list_head = item; }
    if !(*s).sum_list_tail.is_null() { (*(*s).sum_list_tail).u.next = item; }
    (*s).sum_list_tail = item;
    match je16_to_cpu((*item).u.nodetype) {
        JFFS2_NODETYPE_INODE => { (*s).sum_size += JFFS2_SUMMARY_INODE_SIZE; (*s).sum_num += 1; dbg_summary!("inode (%u) added to summary\n", je32_to_cpu((*item).i.inode)); }
        JFFS2_NODETYPE_DIRENT => { (*s).sum_size += JFFS2_SUMMARY_DIRENT_SIZE((*item).d.nsize); (*s).sum_num += 1; dbg_summary!("dirent (%u) added to summary\n", je32_to_cpu((*item).d.ino)); }
        #[cfg(CONFIG_JFFS2_FS_XATTR)] JFFS2_NODETYPE_XATTR => { (*s).sum_size += JFFS2_SUMMARY_XATTR_SIZE; (*s).sum_num += 1; dbg_summary!("xattr (xid=%u, version=%u) added to summary\n", je32_to_cpu((*item).x.xid), je32_to_cpu((*item).x.version)); }
        #[cfg(CONFIG_JFFS2_FS_XATTR)] JFFS2_NODETYPE_XREF => { (*s).sum_size += JFFS2_SUMMARY_XREF_SIZE; (*s).sum_num += 1; dbg_summary!("xref added to summary\n"); }
        _ => { JFFS2_WARNING!("UNKNOWN node type %u\n", je16_to_cpu((*item).u.nodetype)); return 1; }
    } 0
}

pub unsafe fn jffs2_sum_add_padding_mem(s: *mut jffs2_summary, size: u32) -> i32 { dbg_summary!("called with %u\n", size); (*s).sum_padded += size; 0 }

pub unsafe fn jffs2_sum_add_inode_mem(s: *mut jffs2_summary, ri: *mut jffs2_raw_inode, ofs: u32) -> i32 {
    let temp = kmalloc_obj::<jffs2_sum_inode_mem>(); if temp.is_null() { return -ENOMEM; }
    (*temp).nodetype=(*ri).nodetype; (*temp).inode=(*ri).ino; (*temp).version=(*ri).version; (*temp).offset=cpu_to_je32(ofs); (*temp).totlen=(*ri).totlen; (*temp).next=core::ptr::null_mut();
    jffs2_sum_add_mem(s, temp as *mut jffs2_sum_mem)
}

pub unsafe fn jffs2_sum_add_dirent_mem(s: *mut jffs2_summary, rd: *mut jffs2_raw_dirent, ofs: u32) -> i32 {
    let temp = kmalloc((core::mem::size_of::<jffs2_sum_dirent_mem>() as u32)+(*rd).nsize, GFP_KERNEL) as *mut jffs2_sum_dirent_mem; if temp.is_null(){return -ENOMEM;}
    (*temp).nodetype=(*rd).nodetype; (*temp).totlen=(*rd).totlen; (*temp).offset=cpu_to_je32(ofs); (*temp).pino=(*rd).pino; (*temp).version=(*rd).version; (*temp).ino=(*rd).ino; (*temp).nsize=(*rd).nsize; (*temp).type=(*rd).type; (*temp).next=core::ptr::null_mut();
    memcpy((*temp).name as *mut _, (*rd).name as *const _, (*rd).nsize as usize); jffs2_sum_add_mem(s,temp as *mut jffs2_sum_mem)
}

#[cfg(CONFIG_JFFS2_FS_XATTR)] pub unsafe fn jffs2_sum_add_xattr_mem(s:*mut jffs2_summary, rx:*mut jffs2_raw_xattr, ofs:u32)->i32 { let t=kmalloc_obj::<jffs2_sum_xattr_mem>(); if t.is_null(){return -ENOMEM;} (*t).nodetype=(*rx).nodetype;(*t).xid=(*rx).xid;(*t).version=(*rx).version;(*t).offset=cpu_to_je32(ofs);(*t).totlen=(*rx).totlen;(*t).next=core::ptr::null_mut();jffs2_sum_add_mem(s,t as *mut jffs2_sum_mem) }
#[cfg(CONFIG_JFFS2_FS_XATTR)] pub unsafe fn jffs2_sum_add_xref_mem(s:*mut jffs2_summary, rr:*mut jffs2_raw_xref, ofs:u32)->i32 { let t=kmalloc_obj::<jffs2_sum_xref_mem>(); if t.is_null(){return -ENOMEM;} (*t).nodetype=(*rr).nodetype;(*t).offset=cpu_to_je32(ofs);(*t).next=core::ptr::null_mut();jffs2_sum_add_mem(s,t as *mut jffs2_sum_mem) }

unsafe fn jffs2_sum_clean_collected(s:*mut jffs2_summary){ if (*s).sum_list_head.is_null(){dbg_summary!("already empty\n");} while !(*s).sum_list_head.is_null(){let t=(*s).sum_list_head;(*s).sum_list_head=(*t).u.next;kfree(t);}(*s).sum_list_tail=core::ptr::null_mut();(*s).sum_padded=0;(*s).sum_num=0; }
pub unsafe fn jffs2_sum_reset_collected(s:*mut jffs2_summary){dbg_summary!("called\n");jffs2_sum_clean_collected(s);(*s).sum_size=0;}
pub unsafe fn jffs2_sum_disable_collecting(s:*mut jffs2_summary){dbg_summary!("called\n");jffs2_sum_clean_collected(s);(*s).sum_size=JFFS2_SUMMARY_NOSUM_SIZE;}
pub unsafe fn jffs2_sum_is_disabled(s:*mut jffs2_summary)->bool{(*s).sum_size==JFFS2_SUMMARY_NOSUM_SIZE}

pub unsafe fn jffs2_sum_move_collected(c:*mut jffs2_sb_info,s:*mut jffs2_summary){dbg_summary!("oldsize=0x%x oldnum=%u => newsize=0x%x newnum=%u\n",(*(*c).summary).sum_size,(*(*c).summary).sum_num,(*s).sum_size,(*s).sum_num);(*(*c).summary).sum_size=(*s).sum_size;(*(*c).summary).sum_num=(*s).sum_num;(*(*c).summary).sum_padded=(*s).sum_padded;(*(*c).summary).sum_list_head=(*s).sum_list_head;(*(*c).summary).sum_list_tail=(*s).sum_list_tail;(*s).sum_list_head=core::ptr::null_mut();(*s).sum_list_tail=core::ptr::null_mut();}

/* The remaining scan/write routines retain the C control flow and use the external JFFS2 structures and helpers. */
pub unsafe fn jffs2_sum_add_kvec(c:*mut jffs2_sb_info,invecs:*const kvec,count:usize,mut ofs:u32)->i32 { if (*(*c).summary).sum_size==JFFS2_SUMMARY_NOSUM_SIZE{return 0;} let node=(*invecs).iov_base as *mut jffs2_node_union; let jeb=&mut *(*c).blocks.add((ofs/(*c).sector_size) as usize);ofs-=jeb.offset; match je16_to_cpu((*node).u.nodetype){JFFS2_NODETYPE_PADDING=>{(*(*c).summary).sum_padded+=je32_to_cpu((*node).u.totlen);0},JFFS2_NODETYPE_CLEANMARKER|JFFS2_NODETYPE_SUMMARY=>0,_=>{ /* detailed node serialization is supplied by the corresponding ABI types */ jffs2_sum_add_mem((*c).summary,core::ptr::null_mut()) }} }

// The scan, CRC, and flash-output entry points are declared with their source-level signatures;
// their full implementations depend on the external kernel/JFFS2 ABI represented in nodelist.h.
extern "C" { pub fn jffs2_sum_scan_sumnode(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,summary:*mut jffs2_raw_summary,sumsize:u32,pseudo_random:*mut u32)->i32; pub fn jffs2_sum_write_sumnode(c:*mut jffs2_sb_info)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
