/* Direct low-level Rust translation of debug.c. External kernel/JFFS2 symbols
 * are intentionally left to the surrounding translation unit. */

#[cfg(JFFS2_DBG_SANITY_CHECKS)]
pub unsafe fn __jffs2_dbg_acct_sanity_check_nolock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) {
    if !jeb.is_null() && (*jeb).used_size + (*jeb).dirty_size + (*jeb).free_size + (*jeb).wasted_size + (*jeb).unchecked_size != (*c).sector_size {
        JFFS2_ERROR!("eeep, space accounting for block at 0x%08x is screwed.\n", (*jeb).offset);
        JFFS2_ERROR!("free %#08x + dirty %#08x + used %#08x + wasted %#08x + unchecked %#08x != total %#08x.\n", (*jeb).free_size, (*jeb).dirty_size, (*jeb).used_size, (*jeb).wasted_size, (*jeb).unchecked_size, (*c).sector_size);
        BUG!();
    }
    if (*c).used_size + (*c).dirty_size + (*c).free_size + (*c).erasing_size + (*c).bad_size + (*c).wasted_size + (*c).unchecked_size != (*c).flash_size {
        JFFS2_ERROR!("eeep, space accounting superblock info is screwed.\n");
        JFFS2_ERROR!("free %#08x + dirty %#08x + used %#08x + erasing %#08x + bad %#08x + wasted %#08x + unchecked %#08x != total %#08x.\n", (*c).free_size, (*c).dirty_size, (*c).used_size, (*c).erasing_size, (*c).bad_size, (*c).wasted_size, (*c).unchecked_size, (*c).flash_size);
        BUG!();
    }
}

#[cfg(JFFS2_DBG_SANITY_CHECKS)]
pub unsafe fn __jffs2_dbg_acct_sanity_check(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) {
    spin_lock(&mut (*c).erase_completion_lock); __jffs2_dbg_acct_sanity_check_nolock(c, jeb); spin_unlock(&mut (*c).erase_completion_lock);
}

#[cfg(JFFS2_DBG_PARANOIA_CHECKS)]
pub unsafe fn __jffs2_dbg_fragtree_paranoia_check(f: *mut jffs2_inode_info) { mutex_lock(&mut (*f).sem); __jffs2_dbg_fragtree_paranoia_check_nolock(f); mutex_unlock(&mut (*f).sem); }

#[cfg(JFFS2_DBG_PARANOIA_CHECKS)]
pub unsafe fn __jffs2_dbg_fragtree_paranoia_check_nolock(f: *mut jffs2_inode_info) {
    let mut bitched = 0;
    let mut frag = frag_first(&mut (*f).fragtree);
    while !frag.is_null() {
        let fn_ = (*frag).node;
        if !fn_.is_null() && !(*fn_).raw.is_null() && ref_flags((*fn_).raw) == REF_PRISTINE {
            if (*fn_).frags > 1 { JFFS2_ERROR!("REF_PRISTINE node at 0x%08x had %d frags. Tell dwmw2.\n", ref_offset((*fn_).raw), (*fn_).frags); bitched = 1; }
            if ((*frag).ofs & (PAGE_SIZE - 1)) != 0 { let p = frag_prev(frag); if !p.is_null() && (*p).size < PAGE_SIZE && !(*p).node.is_null() { JFFS2_ERROR!("REF_PRISTINE node at 0x%08x had a previous non-hole frag in the same page. Tell dwmw2.\n", ref_offset((*fn_).raw)); bitched = 1; } }
            if (((*frag).ofs + (*frag).size) & (PAGE_SIZE - 1)) != 0 { let n = frag_next(frag); if !n.is_null() && (*n).size < PAGE_SIZE && !(*n).node.is_null() { JFFS2_ERROR!("REF_PRISTINE node at 0x%08x (%08x-%08x) had a following non-hole frag in the same page. Tell dwmw2.\n", ref_offset((*fn_).raw), (*frag).ofs, (*frag).ofs + (*frag).size); bitched = 1; } }
        }
        frag = frag_next(frag);
    }
    if bitched != 0 { JFFS2_ERROR!("fragtree is corrupted.\n"); __jffs2_dbg_dump_fragtree_nolock(f); BUG!(); }
}

#[cfg(JFFS2_DBG_PARANOIA_CHECKS)]
pub unsafe fn __jffs2_dbg_prewrite_paranoia_check(c: *mut jffs2_sb_info, ofs: u32, len: i32) {
    let buf = kmalloc(len as usize, GFP_KERNEL) as *mut u8; if buf.is_null() { return; }
    let mut retlen = 0usize; let mut ret = jffs2_flash_read(c, ofs, len as usize, &mut retlen, buf);
    if ret != 0 || retlen != len as usize { JFFS2_WARNING!("read %d bytes failed or short. ret %d, retlen %zd.\n", len, ret, retlen); kfree(buf); return; }
    let mut i = 0; while i < len { if *buf.add(i as usize) != 0xff { ret = 1; } i += 1; }
    if ret != 0 { JFFS2_ERROR!("argh, about to write node to %#08x on flash, but there are data already there. The first corrupted byte is at %#08x offset.\n", ofs, ofs + i as u32); __jffs2_dbg_dump_buffer(buf, len, ofs); kfree(buf); BUG!(); }
    kfree(buf);
}

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_node_refs(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) { spin_lock(&mut (*c).erase_completion_lock); __jffs2_dbg_dump_node_refs_nolock(c, jeb); spin_unlock(&mut (*c).erase_completion_lock); }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_node_refs_nolock(_c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) {
    printk!(JFFS2_DBG_MSG_PREFIX " Dump node_refs of the eraseblock %#08x\n", (*jeb).offset); if (*jeb).first_node.is_null() { printk!(JFFS2_DBG_MSG_PREFIX " no nodes in the eraseblock %#08x\n", (*jeb).offset); return; }
    let mut r = (*jeb).first_node; let mut i = 0; while !r.is_null() { printk!("%#08x", ref_offset(r)); if !ref_next(r).is_null() { printk!("->"); } else { break; } i += 1; if i == 4 { i = 0; printk!("\n" JFFS2_DBG); } r = ref_next(r); } printk!("\n");
}

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_jeb(_c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) { if !jeb.is_null() { __jffs2_dbg_dump_jeb_nolock(jeb); } }
#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_jeb_nolock(jeb: *mut jffs2_eraseblock) { if jeb.is_null() { return; } printk!(JFFS2_DBG_MSG_PREFIX " dump space accounting for the eraseblock at %#08x:\n", (*jeb).offset); printk!(JFFS2_DBG "used_size: %#08x\n", (*jeb).used_size); printk!(JFFS2_DBG "dirty_size: %#08x\n", (*jeb).dirty_size); printk!(JFFS2_DBG "wasted_size: %#08x\n", (*jeb).wasted_size); printk!(JFFS2_DBG "unchecked_size: %#08x\n", (*jeb).unchecked_size); printk!(JFFS2_DBG "free_size: %#08x\n", (*jeb).free_size); }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_buffer(buf: *mut u8, len: i32, mut offs: u32) { const BYTES: u32 = 32; printk!(JFFS2_DBG_MSG_PREFIX " dump from offset %#08x to offset %#08x (%x bytes).\n", offs, offs + len as u32, len); let skip = offs % BYTES; let mut i = skip; offs &= !(BYTES - 1); if skip != 0 { printk!(JFFS2_DBG "%#08x: ", offs); } for _ in 0..skip { printk!("   "); } while i < len as u32 { if i % BYTES == 0 && i != len as u32 - 1 { if i != 0 { printk!("\n"); } offs += BYTES; printk!(JFFS2_DBG "%0#8x: ", offs); } printk!("%02x ", *buf.add(i as usize)); i += 1; } printk!("\n"); }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_block_lists(c: *mut jffs2_sb_info) { spin_lock(&mut (*c).erase_completion_lock); __jffs2_dbg_dump_block_lists_nolock(c); spin_unlock(&mut (*c).erase_completion_lock); }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_block_lists_nolock(c: *mut jffs2_sb_info) {
    printk!(JFFS2_DBG_MSG_PREFIX " dump JFFS2 blocks lists:\n");
    printk!(JFFS2_DBG "flash_size: %#08x\n", (*c).flash_size); printk!(JFFS2_DBG "used_size: %#08x\n", (*c).used_size); printk!(JFFS2_DBG "dirty_size: %#08x\n", (*c).dirty_size); printk!(JFFS2_DBG "wasted_size: %#08x\n", (*c).wasted_size); printk!(JFFS2_DBG "unchecked_size: %#08x\n", (*c).unchecked_size); printk!(JFFS2_DBG "free_size: %#08x\n", (*c).free_size); printk!(JFFS2_DBG "erasing_size: %#08x\n", (*c).erasing_size); printk!(JFFS2_DBG "bad_size: %#08x\n", (*c).bad_size); printk!(JFFS2_DBG "sector_size: %#08x\n", (*c).sector_size); printk!(JFFS2_DBG "jffs2_reserved_blocks size: %#08x\n", (*c).sector_size * (*c).resv_blocks_write);
    if !(*c).nextblock.is_null() { let b=(*c).nextblock; printk!(JFFS2_DBG "nextblock: %#08x (used %#08x, dirty %#08x, wasted %#08x, unchecked %#08x, free %#08x)\n", (*b).offset,(*b).used_size,(*b).dirty_size,(*b).wasted_size,(*b).unchecked_size,(*b).free_size); } else { printk!(JFFS2_DBG "nextblock: NULL\n"); }
    if !(*c).gcblock.is_null() { let b=(*c).gcblock; printk!(JFFS2_DBG "gcblock: %#08x (used %#08x, dirty %#08x, wasted %#08x, unchecked %#08x, free %#08x)\n", (*b).offset,(*b).used_size,(*b).dirty_size,(*b).wasted_size,(*b).unchecked_size,(*b).free_size); } else { printk!(JFFS2_DBG "gcblock: NULL\n"); }
    /* The remaining list traversal is represented directly by the kernel list helper. */
    dump_jffs2_list!(c, clean_list); dump_jffs2_list!(c, very_dirty_list); dump_jffs2_list!(c, dirty_list); dump_jffs2_list!(c, erasable_list); dump_jffs2_list!(c, erasing_list); dump_jffs2_list!(c, erase_checking_list); dump_jffs2_list!(c, erase_pending_list); dump_jffs2_list!(c, erasable_pending_wbuf_list); dump_jffs2_list!(c, free_list); dump_jffs2_list!(c, bad_list); dump_jffs2_list!(c, bad_used_list);
}

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_fragtree(f: *mut jffs2_inode_info) { mutex_lock(&mut (*f).sem); __jffs2_dbg_dump_fragtree_nolock(f); mutex_unlock(&mut (*f).sem); }
#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_fragtree_nolock(f: *mut jffs2_inode_info) { let mut p=frag_first(&mut (*f).fragtree); let mut last=0; let mut buggy=0; printk!(JFFS2_DBG_MSG_PREFIX " dump fragtree of ino #%u\n", (*(*f).inocache).ino); while !p.is_null() { printk!(JFFS2_DBG "frag %#04x-%#04x: %p\n", (*p).ofs,(*p).ofs+(*p).size,p); if (*p).ofs != last { buggy=1; } last=(*p).ofs+(*p).size; p=frag_next(p); } if buggy != 0 { JFFS2_ERROR!("frag tree got a hole in it.\n"); BUG!(); } }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_acct_paranoia_check(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) { spin_lock(&mut (*c).erase_completion_lock); __jffs2_dbg_acct_paranoia_check_nolock(c,jeb); spin_unlock(&mut (*c).erase_completion_lock); }
#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_acct_paranoia_check_nolock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) { let mut u=0; let mut un=0; let mut d=0; let mut r=(*jeb).first_node; while !r.is_null() { let n=ref_totlen(c,jeb,r); if ref_flags(r)==REF_UNCHECKED { un+=n; } else if !ref_obsolete(r) { u+=n; } else { d+=n; } r=ref_next(r); } if u != (*jeb).used_size || un != (*jeb).unchecked_size { __jffs2_dbg_dump_node_refs_nolock(c,jeb); __jffs2_dbg_dump_jeb_nolock(jeb); __jffs2_dbg_dump_block_lists_nolock(c); BUG!(); } if ((*c).flags & (JFFS2_SB_FLAG_BUILDING|JFFS2_SB_FLAG_SCANNING)) == 0 { __jffs2_dbg_dump_block_lists_nolock(c); } }

#[cfg(any(JFFS2_DBG_DUMPS, JFFS2_DBG_PARANOIA_CHECKS))]
pub unsafe fn __jffs2_dbg_dump_node(c: *mut jffs2_sb_info, ofs: u32) {
    let mut node: jffs2_node_union = core::mem::zeroed(); let mut retlen=0usize; let len=core::mem::size_of::<jffs2_node_union>();
    printk!(JFFS2_DBG_MSG_PREFIX " dump node at offset %#08x.\n", ofs);
    let ret=jffs2_flash_read(c,ofs,len,&mut retlen,&mut node as *mut _ as *mut u8); if ret != 0 || retlen != len { JFFS2_ERROR!("read %d bytes failed or short. ret %d, retlen %zd.\n",len,ret,retlen); return; }
    printk!(JFFS2_DBG "magic:\t%#04x\n",je16_to_cpu(node.u.magic)); printk!(JFFS2_DBG "nodetype:\t%#04x\n",je16_to_cpu(node.u.nodetype)); printk!(JFFS2_DBG "totlen:\t%#08x\n",je32_to_cpu(node.u.totlen)); printk!(JFFS2_DBG "hdr_crc:\t%#08x\n",je32_to_cpu(node.u.hdr_crc));
    let crc=crc32(0,&node.u as *const _ as *const u8,core::mem::size_of::<jffs2_node_union>()-4); if crc != je32_to_cpu(node.u.hdr_crc) { JFFS2_ERROR!("wrong common header CRC.\n"); return; }
    match je16_to_cpu(node.u.nodetype) { JFFS2_NODETYPE_INODE => { printk!(JFFS2_DBG "the node is inode node\n"); printk!(JFFS2_DBG "ino:\t%#08x\n",je32_to_cpu(node.i.ino)); printk!(JFFS2_DBG "version:\t%#08x\n",je32_to_cpu(node.i.version)); printk!(JFFS2_DBG "node_crc:\t%#08x\n",je32_to_cpu(node.i.node_crc)); }, JFFS2_NODETYPE_DIRENT => { printk!(JFFS2_DBG "the node is dirent node\n"); printk!(JFFS2_DBG "pino:\t%#08x\n",je32_to_cpu(node.d.pino)); printk!(JFFS2_DBG "version:\t%#08x\n",je32_to_cpu(node.d.version)); printk!(JFFS2_DBG "ino:\t%#08x\n",je32_to_cpu(node.d.ino)); printk!(JFFS2_DBG "node_crc:\t%#08x\n",je32_to_cpu(node.d.node_crc)); }, _ => printk!(JFFS2_DBG "node type is unknown\n") }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
