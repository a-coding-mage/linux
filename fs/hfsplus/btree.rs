// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/hfsplus/btree.c. External kernel/HFS+ symbols are
 * supplied by the surrounding translation unit. */

const CLUMP_ENTRIES: usize = 15;

static mut clumptbl: [i16; CLUMP_ENTRIES * 3] = [
    4, 4, 4, 6, 6, 4, 8, 8, 4, 11, 11, 5,
    64, 32, 5, 84, 49, 6, 111, 74, 7, 147, 111, 8,
    194, 169, 9, 256, 256, 11, 294, 294, 14, 338, 338, 16,
    388, 388, 20, 446, 446, 25, 512, 512, 32,
];

pub unsafe fn hfsplus_calc_btree_clump_size(mut block_size: u32, node_size: u32,
                                             mut sectors: u64, file_id: i32) -> u32 {
    let mod_: u32 = if node_size > block_size { node_size } else { block_size };
    let mut clump_size: u32;
    let column: usize = match file_id {
        HFSPLUS_ATTR_CNID => 0,
        HFSPLUS_CAT_CNID => 1,
        _ => 2,
    };
    if sectors < 0x200000 {
        clump_size = (sectors << 2) as u32;
        if clump_size < 8 * node_size { clump_size = 8 * node_size; }
    } else {
        let mut i = 0usize;
        sectors >>= 22;
        while sectors != 0 && i < CLUMP_ENTRIES - 1 { i += 1; sectors >>= 1; }
        clump_size = clumptbl[column + i * 3] as u32 * 1024 * 1024;
    }
    clump_size /= mod_;
    clump_size *= mod_;
    if clump_size == 0 { clump_size = mod_; }
    clump_size
}

#[repr(C)]
struct hfs_bmap_ctx { page_idx: u32, off: u32, len: u16 }

unsafe fn hfs_bmap_get_map_page(node: *mut hfs_bnode, ctx: *mut hfs_bmap_ctx,
                                byte_offset: u32) -> *mut page {
    let rec_idx: u16;
    let mut off16: u16 = 0;
    if (*node).this == HFSPLUS_TREE_HEAD {
        if (*node).type_ != HFS_NODE_HEADER { pr_err!("hfsplus: invalid btree header node\n"); return ERR_PTR(-EIO); }
        rec_idx = HFSPLUS_BTREE_HDR_MAP_REC_INDEX;
    } else {
        if (*node).type_ != HFS_NODE_MAP { pr_err!("hfsplus: invalid btree map node\n"); return ERR_PTR(-EIO); }
        rec_idx = HFSPLUS_BTREE_MAP_NODE_REC_INDEX;
    }
    (*ctx).len = hfs_brec_lenoff(node, rec_idx, &mut off16);
    if hfs_brec_len_invalid(node, (*ctx).len) { return ERR_PTR(-EINVAL); }
    if !is_bnode_offset_valid(node, off16) { return ERR_PTR(-EIO); }
    (*ctx).len = check_and_correct_requested_length(node, off16, (*ctx).len);
    if byte_offset >= (*ctx).len as u32 { return ERR_PTR(-EINVAL); }
    let page_off = off16 as u32 + (*node).page_offset + byte_offset;
    (*ctx).page_idx = page_off >> PAGE_SHIFT;
    (*ctx).off = page_off & !PAGE_MASK;
    (*node).page[(*ctx).page_idx as usize]
}

unsafe fn hfs_bmap_test_bit(node: *mut hfs_bnode, node_bit_idx: u32) -> bool {
    let mut ctx = hfs_bmap_ctx { page_idx: 0, off: 0, len: 0 };
    let page = hfs_bmap_get_map_page(node, &mut ctx, node_bit_idx / BITS_PER_BYTE);
    if IS_ERR(page) { return false; }
    let bmap = kmap_local_page(page);
    let byte = *bmap.add(ctx.off as usize);
    kunmap_local(bmap);
    let mask: u8 = 1u8 << (7 - node_bit_idx % BITS_PER_BYTE);
    (byte & mask) != 0
}

unsafe fn hfs_bmap_clear_bit(node: *mut hfs_bnode, node_bit_idx: u32) -> i32 {
    let mut ctx = hfs_bmap_ctx { page_idx: 0, off: 0, len: 0 };
    let page = hfs_bmap_get_map_page(node, &mut ctx, node_bit_idx / BITS_PER_BYTE);
    if IS_ERR(page) { return PTR_ERR(page); }
    let bmap = kmap_local_page(page);
    let mask: u8 = 1u8 << (7 - node_bit_idx % BITS_PER_BYTE);
    if (*bmap.add(ctx.off as usize) & mask) == 0 { kunmap_local(bmap); return -EINVAL; }
    *bmap.add(ctx.off as usize) &= !mask;
    set_page_dirty(page); kunmap_local(bmap); 0
}

const HFS_EXTENT_TREE_NAME: &[u8] = b"Extents Overflow File\0";
const HFS_CATALOG_TREE_NAME: &[u8] = b"Catalog File\0";
const HFS_ATTR_TREE_NAME: &[u8] = b"Attributes File\0";
const HFS_UNKNOWN_TREE_NAME: &[u8] = b"Unknown B-tree\0";

unsafe fn hfs_btree_name(cnid: u32) -> *const u8 { match cnid {
    HFSPLUS_EXT_CNID => HFS_EXTENT_TREE_NAME.as_ptr(), HFSPLUS_CAT_CNID => HFS_CATALOG_TREE_NAME.as_ptr(),
    HFSPLUS_ATTR_CNID => HFS_ATTR_TREE_NAME.as_ptr(), _ => HFS_UNKNOWN_TREE_NAME.as_ptr(),
} }

/* The remaining routines preserve the C implementation's kernel object and
 * error-pointer operations; their declarations use the surrounding HFS+ ABI. */

pub unsafe fn hfs_btree_open(sb: *mut super_block, id: u32) -> *mut hfs_btree {
    let tree = kzalloc_obj::<hfs_btree>(); if tree.is_null() { return core::ptr::null_mut(); }
    mutex_init(&mut (*tree).tree_lock); spin_lock_init(&mut (*tree).hash_lock);
    (*tree).sb = sb; (*tree).cnid = id;
    let inode = hfsplus_iget(sb, id); if IS_ERR(inode) { kfree(tree); return core::ptr::null_mut(); }
    (*tree).inode = inode;
    if !(*HFSPLUS_I(inode)).first_blocks { pr_err!("invalid btree extent records (0 size)\n"); iput(inode); kfree(tree); return core::ptr::null_mut(); }
    let page = read_mapping_page((*inode).i_mapping, 0, core::ptr::null_mut());
    if IS_ERR(page) { iput(inode); kfree(tree); return core::ptr::null_mut(); }
    let head = (kmap_local_page(page) as *mut u8).add(core::mem::size_of::<hfs_bnode_desc>()) as *mut hfs_btree_header_rec;
    (*tree).root = be32_to_cpu((*head).root); (*tree).leaf_count = be32_to_cpu((*head).leaf_count);
    (*tree).leaf_head = be32_to_cpu((*head).leaf_head); (*tree).leaf_tail = be32_to_cpu((*head).leaf_tail);
    (*tree).node_count = be32_to_cpu((*head).node_count); (*tree).free_nodes = be32_to_cpu((*head).free_nodes);
    (*tree).attributes = be32_to_cpu((*head).attributes); (*tree).node_size = be16_to_cpu((*head).node_size);
    (*tree).max_key_len = be16_to_cpu((*head).max_key_len); (*tree).depth = be16_to_cpu((*head).depth);
    let valid = match id { HFSPLUS_EXT_CNID => { if (*tree).max_key_len != HFSPLUS_EXT_KEYLEN - 2 || ((*tree).attributes & HFS_TREE_VARIDXKEYS) != 0 { false } else { (*tree).keycmp = Some(hfsplus_ext_cmp_key); true } },
        HFSPLUS_CAT_CNID => { if (*tree).max_key_len != HFSPLUS_CAT_KEYLEN - 2 || ((*tree).attributes & HFS_TREE_VARIDXKEYS) == 0 { false } else { (*tree).keycmp = Some(if test_bit(HFSPLUS_SB_HFSX, &(*HFSPLUS_SB(sb)).flags) && (*head).key_type == HFSPLUS_KEY_BINARY { hfsplus_cat_bin_cmp_key } else { set_bit(HFSPLUS_SB_CASEFOLD, &mut (*HFSPLUS_SB(sb)).flags); hfsplus_cat_case_cmp_key }); true } },
        HFSPLUS_ATTR_CNID => { if (*tree).max_key_len != HFSPLUS_ATTR_KEYLEN - 2 { false } else { (*tree).keycmp = Some(hfsplus_attr_bin_cmp_key); true } }, _ => false };
    if !valid || ((*tree).attributes & HFS_TREE_BIGKEYS) == 0 || (*tree).node_size < HFSPLUS_NODE_MINSZ || (*tree).node_size > HFSPLUS_NODE_MXSZ || !is_power_of_2((*tree).node_size) || (*tree).node_count == 0 { kunmap_local(head); put_page(page); iput(inode); kfree(tree); return core::ptr::null_mut(); }
    (*tree).node_size_shift = ffs((*tree).node_size) - 1; (*tree).pages_per_bnode = ((*tree).node_size + PAGE_SIZE - 1) >> PAGE_SHIFT;
    kunmap_local(head); put_page(page); let node = hfs_bnode_find(tree, HFSPLUS_TREE_HEAD); if IS_ERR(node) { iput(inode); kfree(tree); return core::ptr::null_mut(); }
    if !hfs_bmap_test_bit(node, 0) { pr_warn!("btree map record invalid or bitmap corruption detected\n"); (*sb).s_flags |= SB_RDONLY; }
    hfs_bnode_put(node); tree
}

pub unsafe fn hfs_btree_close(tree: *mut hfs_btree) { if tree.is_null() { return; } for i in 0..NODE_HASH_SIZE { while !(*tree).node_hash[i].is_null() { let node=(*tree).node_hash[i]; (*tree).node_hash[i]=(*node).next_hash; hfs_bnode_free(node); (*tree).node_hash_cnt-=1; } } iput((*tree).inode); kfree(tree); }

pub unsafe fn hfs_btree_write(tree: *mut hfs_btree) -> i32 { let node=hfs_bnode_find(tree,0); if IS_ERR(node) { return -EIO; } let page=(*node).page[0]; let head=(kmap_local_page(page) as *mut u8).add(core::mem::size_of::<hfs_bnode_desc>()) as *mut hfs_btree_header_rec; (*head).root=cpu_to_be32((*tree).root); (*head).leaf_count=cpu_to_be32((*tree).leaf_count); (*head).leaf_head=cpu_to_be32((*tree).leaf_head); (*head).leaf_tail=cpu_to_be32((*tree).leaf_tail); (*head).node_count=cpu_to_be32((*tree).node_count); (*head).free_nodes=cpu_to_be32((*tree).free_nodes); (*head).attributes=cpu_to_be32((*tree).attributes); (*head).depth=cpu_to_be16((*tree).depth); kunmap_local(head); set_page_dirty(page); hfs_bnode_put(node); 0 }

unsafe fn hfs_bmap_new_bmap(prev: *mut hfs_bnode, idx: u32) -> *mut hfs_bnode {
    let tree=(*prev).tree; let node=hfs_bnode_create(tree,idx); if IS_ERR(node) { return node; }
    (*tree).free_nodes-=1; (*prev).next=idx; let cnid=cpu_to_be32(idx); hfs_bnode_write(prev,&cnid as *const _ as *const core::ffi::c_void,core::mem::offset_of!(hfs_bnode_desc,next),4);
    (*node).type_=HFS_NODE_MAP; (*node).num_recs=1; hfs_bnode_clear(node,0,(*tree).node_size);
    let mut desc: hfs_bnode_desc=core::mem::zeroed(); desc.type_=HFS_NODE_MAP; desc.num_recs=cpu_to_be16(1);
    hfs_bnode_write(node,&desc as *const _ as *const core::ffi::c_void,0,core::mem::size_of::<hfs_bnode_desc>());
    hfs_bnode_write_u16(node,14,0x8000); hfs_bnode_write_u16(node,(*tree).node_size-2,14); hfs_bnode_write_u16(node,(*tree).node_size-4,(*tree).node_size-6); node
}

pub unsafe fn hfs_bmap_reserve(tree:*mut hfs_btree,rsvd_nodes:u32)->i32 { if rsvd_nodes==0{return 0;} let inode=(*tree).inode; let hip=HFSPLUS_I(inode); while (*tree).free_nodes<rsvd_nodes { let res=hfsplus_file_extend(inode,hfs_bnode_need_zeroout(tree)); if res!=0{return res;} (*hip).phys_size=(*inode).i_size=((*hip).alloc_blocks as i64)<<(*HFSPLUS_SB((*tree).sb)).alloc_blksz_shift; (*hip).fs_blocks=(*hip).alloc_blocks<<(*HFSPLUS_SB((*tree).sb)).fs_shift; inode_set_bytes(inode,(*inode).i_size); let count=((*inode).i_size as u64>>(*tree).node_size_shift) as u32; (*tree).free_nodes+=count-(*tree).node_count; (*tree).node_count=count;} 0 }

pub unsafe fn hfs_bmap_alloc(tree:*mut hfs_btree)->*mut hfs_bnode { if hfs_bmap_reserve(tree,1)!=0{return ERR_PTR(-EIO);} let mut node=hfs_bnode_find(tree,0); if IS_ERR(node){return node;} let mut ctx=core::mem::zeroed::<hfs_bmap_ctx>(); let mut page=hfs_bmap_get_map_page(node,&mut ctx,0); if IS_ERR(page){hfs_bnode_put(node);return page as *mut hfs_bnode;} let mut data=kmap_local_page(page); let mut idx=0u32; loop { while ctx.len!=0 { let byte=*data.add(ctx.off as usize); if byte!=0xff { for i in 0..8 { let m=0x80u8>>i; if byte&m==0 {*data.add(ctx.off as usize)|=m;set_page_dirty(page);kunmap_local(data);(*tree).free_nodes-=1;hfs_btree_write(tree);mark_inode_dirty((*tree).inode);hfs_bnode_put(node);return hfs_bnode_create(tree,idx+i);}}} ctx.off+=1;if ctx.off>=PAGE_SIZE as u32{kunmap_local(data);ctx.page_idx+=1;page=(*node).page[ctx.page_idx as usize];data=kmap_local_page(page);ctx.off=0;}idx+=8;ctx.len-=1;}kunmap_local(data);let next=if (*node).next==0{let n=hfs_bmap_new_bmap(node,idx);hfs_btree_write(tree);n}else{hfs_bnode_find(tree,(*node).next)};hfs_bnode_put(node);if IS_ERR(next){return next;}node=next;page=hfs_bmap_get_map_page(node,&mut ctx,0);if IS_ERR(page){hfs_bnode_put(node);return page as *mut hfs_bnode;}data=kmap_local_page(page);}}

pub unsafe fn hfs_bmap_free(mut node:*mut hfs_bnode){BUG_ON((*node).this==0);let tree=(*node).tree;let mut nidx=(*node).this;node=hfs_bnode_find(tree,0);if IS_ERR(node){return;}let mut off=0u16;let mut len=hfs_brec_lenoff(node,2,&mut off);if hfs_brec_len_invalid(node,len){hfs_bnode_put(node);return;}while nidx>=(len as u32)*8{nidx-=(len as u32)*8;let i=(*node).next;if i==0{hfs_bnode_put(node);return;}hfs_bnode_put(node);node=hfs_bnode_find(tree,i);if IS_ERR(node){return;}len=hfs_brec_lenoff(node,0,&mut off);if hfs_brec_len_invalid(node,len){hfs_bnode_put(node);return;}}let res=hfs_bmap_clear_bit(node,nidx);if res==0{(*tree).free_nodes+=1;hfs_btree_write(tree);mark_inode_dirty((*tree).inode);}hfs_bnode_put(node);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
