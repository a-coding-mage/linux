// SPDX-License-Identifier: GPL-2.0
/* Translated from linux/fs/hfs/btree.c. */

#[repr(C)]
struct hfs_bmap_ctx {
    page_idx: c_uint,
    off: c_uint,
    len: u16,
}

unsafe fn hfs_bmap_get_map_page(node: *mut hfs_bnode, ctx: *mut hfs_bmap_ctx,
                                byte_offset: u32) -> *mut page {
    let mut rec_idx: u16;
    let mut off16: u16 = 0;
    let page_off: c_uint;
    if (*node).this == HFS_TREE_HEAD {
        if (*node).type_ != HFS_NODE_HEADER { pr_err!("hfs: invalid btree header node\n"); return ERR_PTR(-EIO); }
        rec_idx = HFS_BTREE_HDR_MAP_REC_INDEX;
    } else {
        if (*node).type_ != HFS_NODE_MAP { pr_err!("hfs: invalid btree map node\n"); return ERR_PTR(-EIO); }
        rec_idx = HFS_BTREE_MAP_NODE_REC_INDEX;
    }
    (*ctx).len = hfs_brec_lenoff(node, rec_idx, &mut off16);
    if (*ctx).len == 0 { return ERR_PTR(-ENOENT); }
    if !is_bnode_offset_valid(node, off16) { return ERR_PTR(-EIO); }
    (*ctx).len = check_and_correct_requested_length(node, off16, (*ctx).len);
    if byte_offset >= (*ctx).len as u32 { return ERR_PTR(-EINVAL); }
    page_off = off16 as u32 + (*node).page_offset + byte_offset;
    (*ctx).page_idx = page_off >> PAGE_SHIFT;
    (*ctx).off = page_off & !PAGE_MASK;
    *(*node).page.add((*ctx).page_idx as usize)
}

unsafe fn hfs_bmap_test_bit(node: *mut hfs_bnode, node_bit_idx: u32) -> bool {
    let mut ctx = hfs_bmap_ctx { page_idx: 0, off: 0, len: 0 };
    let page = hfs_bmap_get_map_page(node, &mut ctx, node_bit_idx / BITS_PER_BYTE);
    if IS_ERR(page) { return false; }
    let bmap = kmap_local_page(page);
    let byte = *bmap.add(ctx.off as usize);
    kunmap_local(bmap);
    let mask: u8 = 1u8 << (7 - (node_bit_idx % BITS_PER_BYTE));
    byte & mask != 0
}

unsafe fn hfs_bmap_clear_bit(node: *mut hfs_bnode, node_bit_idx: u32) -> c_int {
    let mut ctx = hfs_bmap_ctx { page_idx: 0, off: 0, len: 0 };
    let page = hfs_bmap_get_map_page(node, &mut ctx, node_bit_idx / BITS_PER_BYTE);
    if IS_ERR(page) { return PTR_ERR(page); }
    let bmap = kmap_local_page(page);
    let mask: u8 = 1u8 << (7 - (node_bit_idx % BITS_PER_BYTE));
    if *bmap.add(ctx.off as usize) & mask == 0 { kunmap_local(bmap); return -EINVAL; }
    *bmap.add(ctx.off as usize) &= !mask;
    set_page_dirty(page); kunmap_local(bmap); 0
}

unsafe fn hfs_btree_open(sb: *mut super_block, id: u32, keycmp: btree_keycmp) -> *mut hfs_btree {
    let tree = kzalloc_obj::<hfs_btree>();
    if tree.is_null() { return core::ptr::null_mut(); }
    mutex_init(&mut (*tree).tree_lock); spin_lock_init(&mut (*tree).hash_lock);
    (*tree).sb = sb; (*tree).cnid = id; (*tree).keycmp = keycmp;
    (*tree).inode = iget_locked(sb, id);
    if (*tree).inode.is_null() { goto_free_tree(tree); }
    BUG_ON(!(inode_state_read_once((*tree).inode) & I_NEW));
    let mdb = HFS_SB(sb).mdb; HFS_I((*tree).inode).flags = 0;
    mutex_init(&mut HFS_I((*tree).inode).extents_lock);
    match id {
        HFS_EXT_CNID => { hfs_inode_read_fork((*tree).inode, mdb.drXTExtRec, mdb.drXTFlSize, mdb.drXTFlSize, be32_to_cpu(mdb.drXTClpSiz)); if HFS_I((*tree).inode).alloc_blocks > HFS_I((*tree).inode).first_blocks { pr_err!("invalid btree extent records\n"); unlock_new_inode((*tree).inode); goto_free_inode(tree); } (*tree).inode.i_mapping.a_ops = &hfs_btree_aops; },
        HFS_CAT_CNID => { hfs_inode_read_fork((*tree).inode, mdb.drCTExtRec, mdb.drCTFlSize, mdb.drCTFlSize, be32_to_cpu(mdb.drCTClpSiz)); if HFS_I((*tree).inode).first_blocks == 0 { pr_err!("invalid btree extent records (0 size)\n"); unlock_new_inode((*tree).inode); goto_free_inode(tree); } (*tree).inode.i_mapping.a_ops = &hfs_btree_aops; },
        _ => BUG(),
    }
    unlock_new_inode((*tree).inode);
    let mapping = (*tree).inode.i_mapping; let folio = filemap_grab_folio(mapping, 0); if IS_ERR(folio) { goto_free_inode(tree); }
    folio_zero_range(folio, 0, folio_size(folio));
    let dblock = hfs_ext_find_block(HFS_I((*tree).inode).first_extents, 0);
    let mut start_block = HFS_SB(sb).fs_start + dblock as u64 * HFS_SB(sb).fs_div as u64;
    let mut size = folio_size(folio); let mut offset: loff_t = 0;
    while size > 0 { let bh = sb_bread(sb, start_block); if bh.is_null() { pr_err!("unable to read tree header\n"); goto_put_folio(tree, folio); } let len = core::cmp::min(folio_size(folio), sb.s_blocksize as usize); memcpy_to_folio(folio, offset, (*bh).b_data, sb.s_blocksize as usize); brelse(bh); start_block += 1; offset += len as loff_t; size -= len; }
    folio_mark_uptodate(folio);
    let head = (kmap_local_folio(folio, 0).add(core::mem::size_of::<hfs_bnode_desc>())) as *mut hfs_btree_header_rec;
    (*tree).root=be32_to_cpu((*head).root); (*tree).leaf_count=be32_to_cpu((*head).leaf_count); (*tree).leaf_head=be32_to_cpu((*head).leaf_head); (*tree).leaf_tail=be32_to_cpu((*head).leaf_tail); (*tree).node_count=be32_to_cpu((*head).node_count); (*tree).free_nodes=be32_to_cpu((*head).free_nodes); (*tree).attributes=be32_to_cpu((*head).attributes); (*tree).node_size=be16_to_cpu((*head).node_size); (*tree).max_key_len=be16_to_cpu((*head).max_key_len); (*tree).depth=be16_to_cpu((*head).depth);
    size = (*tree).node_size as usize; if !is_power_of_2(size) || (*tree).node_count == 0 { kunmap_local(head); goto_put_folio(tree, folio); }
    let expected = if id == HFS_EXT_CNID { HFS_MAX_EXT_KEYLEN } else if id == HFS_CAT_CNID { HFS_MAX_CAT_KEYLEN } else { BUG(); };
    if (*tree).max_key_len != expected { pr_err!("invalid max_key_len\n"); kunmap_local(head); goto_put_folio(tree, folio); }
    (*tree).node_size_shift = ffs(size as u32) - 1; (*tree).pages_per_bnode = ((*tree).node_size as usize + PAGE_SIZE - 1) >> PAGE_SHIFT;
    kunmap_local(head); folio_unlock(folio); folio_put(folio);
    let node = hfs_bnode_find(tree, HFS_TREE_HEAD); if IS_ERR(node) { goto_free_inode(tree); }
    if !hfs_bmap_test_bit(node, HFS_TREE_HEAD) { pr_warn!("(%s): bitmap corrupted, forcing rdonly\n", sb.s_id); sb.s_flags |= SB_RDONLY; }
    hfs_bnode_put(node); tree
}

unsafe fn goto_free_tree(tree: *mut hfs_btree) -> ! { kfree(tree); panic!("translation control flow") }
unsafe fn goto_free_inode(tree: *mut hfs_btree) -> ! { iput((*tree).inode); kfree(tree); panic!("translation control flow") }
unsafe fn goto_put_folio(tree: *mut hfs_btree, folio: *mut folio) -> ! { folio_unlock(folio); folio_put(folio); goto_free_inode(tree) }

unsafe fn hfs_btree_close(tree: *mut hfs_btree) { if tree.is_null(){return;} for i in 0..NODE_HASH_SIZE { while !(*tree).node_hash[i].is_null() { let node=(*tree).node_hash[i]; (*tree).node_hash[i]=(*node).next_hash; hfs_bnode_free(node); (*tree).node_hash_cnt-=1; } } iput((*tree).inode); kfree(tree); }

unsafe fn hfs_btree_write(tree: *mut hfs_btree) { let node=hfs_bnode_find(tree,0); if IS_ERR(node){return;} let page=(*node).page[0]; let head=(kmap_local_page(page).add(core::mem::size_of::<hfs_bnode_desc>())) as *mut hfs_btree_header_rec; (*head).root=cpu_to_be32((*tree).root); (*head).leaf_count=cpu_to_be32((*tree).leaf_count); (*head).leaf_head=cpu_to_be32((*tree).leaf_head); (*head).leaf_tail=cpu_to_be32((*tree).leaf_tail); (*head).node_count=cpu_to_be32((*tree).node_count); (*head).free_nodes=cpu_to_be32((*tree).free_nodes); (*head).attributes=cpu_to_be32((*tree).attributes); (*head).depth=cpu_to_be16((*tree).depth); kunmap_local(head); set_page_dirty(page); hfs_bnode_put(node); }

unsafe fn hfs_bmap_new_bmap(prev: *mut hfs_bnode, idx: u32) -> *mut hfs_bnode { let tree=(*prev).tree; let node=hfs_bnode_create(tree,idx); if IS_ERR(node){return node;} if (*tree).free_nodes==0 {panic!("FIXME!!!");} (*tree).free_nodes-=1; (*prev).next=idx; let cnid=cpu_to_be32(idx); hfs_bnode_write(prev,&cnid,core::mem::offset_of!(hfs_bnode_desc,next),4); (*node).type_=HFS_NODE_MAP; (*node).num_recs=1; hfs_bnode_clear(node,0,(*tree).node_size); let mut desc=hfs_bnode_desc::default(); desc.type_=HFS_NODE_MAP; desc.num_recs=cpu_to_be16(1); hfs_bnode_write(node,&desc,0,core::mem::size_of_val(&desc)); hfs_bnode_write_u16(node,14,0x8000); hfs_bnode_write_u16(node,(*tree).node_size-2,14); hfs_bnode_write_u16(node,(*tree).node_size-4,(*tree).node_size-6); node }

unsafe fn hfs_bmap_reserve(tree:*mut hfs_btree,rsvd_nodes:u32)->c_int { while (*tree).free_nodes<rsvd_nodes { let inode=(*tree).inode; let res=hfs_extend_file(inode); if res!=0{return res;} HFS_I(inode).phys_size=inode.i_size=HFS_I(inode).alloc_blocks as i64*HFS_SB((*tree).sb).alloc_blksz as i64; HFS_I(inode).fs_blocks=inode.i_size>>(*tree).sb.s_blocksize_bits; inode_set_bytes(inode,inode.i_size); let count=(inode.i_size as u64>>(*tree).node_size_shift) as u32; (*tree).free_nodes+=count-(*tree).node_count; (*tree).node_count=count;} 0 }

unsafe fn hfs_bmap_alloc(tree:*mut hfs_btree)->*mut hfs_bnode { let mut node=hfs_bnode_find(tree,0); if IS_ERR(node){return node;} let mut ctx=hfs_bmap_ctx{page_idx:0,off:0,len:0}; let mut page=hfs_bmap_get_map_page(node,&mut ctx,0); if IS_ERR(page){hfs_bnode_put(node);return page;} let mut data=kmap_local_page(page); let mut idx=0u32; loop { while ctx.len!=0 { let byte=*data.add(ctx.off as usize); if byte!=0xff { for i in 0..8 {let m=0x80u8>>i;if byte&m==0 {*data.add(ctx.off as usize)|=m;set_page_dirty(page);kunmap_local(data);(*tree).free_nodes-=1;mark_inode_dirty((*tree).inode);hfs_bnode_put(node);return hfs_bnode_create(tree,idx+i);}}} ctx.off+=1;if ctx.off>=PAGE_SIZE as u32 {kunmap_local(data);ctx.page_idx+=1;page=(*node).page[ctx.page_idx as usize];data=kmap_local_page(page);ctx.off=0;}idx+=8;ctx.len-=1;}kunmap_local(data);let next=if (*node).next==0{hfs_bmap_new_bmap(node,idx)}else{hfs_bnode_find(tree,(*node).next)};hfs_bnode_put(node);if IS_ERR(next){return next;}node=next;page=hfs_bmap_get_map_page(node,&mut ctx,0);if IS_ERR(page){hfs_bnode_put(node);return page;}data=kmap_local_page(page);} }

unsafe fn hfs_bmap_free(mut node:*mut hfs_bnode){let tree=(*node).tree;let nidx0=(*node).this;node=hfs_bnode_find(tree,0);if IS_ERR(node){return;}let mut off=0;let mut len=hfs_brec_lenoff(node,2,&mut off);let mut nidx=nidx0;while nidx>=len as u32*8{nidx-=len as u32*8;let i=(*node).next;if i==0{hfs_bnode_put(node);return;}hfs_bnode_put(node);node=hfs_bnode_find(tree,i);if IS_ERR(node){return;}if (*node).type_!=HFS_NODE_MAP{hfs_bnode_put(node);return;}len=hfs_brec_lenoff(node,0,&mut off);}let res=hfs_bmap_clear_bit(node,nidx);if res==0{(*tree).free_nodes+=1;mark_inode_dirty((*tree).inode);}hfs_bnode_put(node);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
