/*
 * linux/fs/befs/btree.c
 *
 * Direct Rust translation of the original implementation.
 * External kernel and BeFS types/functions are supplied by other files.
 */

#[repr(C)]
pub struct befs_btree_node {
    pub head: befs_host_btree_nodehead,
    pub bh: *mut buffer_head,
    pub od_node: *mut befs_btree_nodehead,
}

const BEFS_BT_INVAL: befs_off_t = 0xffffffffffffffffu64;

unsafe fn befs_bt_read_super(sb: *mut super_block, ds: *const befs_data_stream,
                             sup: *mut befs_btree_super) -> i32 {
    let bh = befs_read_datastream(sb, ds, 0, core::ptr::null_mut());
    if bh.is_null() { befs_error(sb, "Couldn't read index header."); return BEFS_ERR; }
    let od_sup = (*bh).b_data as *mut befs_disk_btree_super;
    befs_dump_index_entry(sb, od_sup);
    (*sup).magic = fs32_to_cpu(sb, (*od_sup).magic);
    (*sup).node_size = fs32_to_cpu(sb, (*od_sup).node_size);
    (*sup).max_depth = fs32_to_cpu(sb, (*od_sup).max_depth);
    (*sup).data_type = fs32_to_cpu(sb, (*od_sup).data_type);
    (*sup).root_node_ptr = fs64_to_cpu(sb, (*od_sup).root_node_ptr);
    brelse(bh);
    if (*sup).magic != BEFS_BTREE_MAGIC { befs_error(sb, "Index header has bad magic."); return BEFS_ERR; }
    BEFS_OK
}

unsafe fn befs_bt_read_node(sb: *mut super_block, ds: *const befs_data_stream,
                            node: *mut befs_btree_node, node_off: befs_off_t) -> i32 {
    let mut off: u32 = 0;
    if !(*node).bh.is_null() { brelse((*node).bh); }
    (*node).bh = befs_read_datastream(sb, ds, node_off, &mut off);
    if (*node).bh.is_null() { befs_error(sb, "%s failed to read node at %llu", "befs_bt_read_node", node_off); return BEFS_ERR; }
    (*node).od_node = ((*node).bh).as_ref().unwrap().b_data.add(off as usize) as *mut befs_btree_nodehead;
    befs_dump_index_node(sb, (*node).od_node);
    (*node).head.left = fs64_to_cpu(sb, (*(*node).od_node).left);
    (*node).head.right = fs64_to_cpu(sb, (*(*node).od_node).right);
    (*node).head.overflow = fs64_to_cpu(sb, (*(*node).od_node).overflow);
    (*node).head.all_key_count = fs16_to_cpu(sb, (*(*node).od_node).all_key_count);
    (*node).head.all_key_length = fs16_to_cpu(sb, (*(*node).od_node).all_key_length);
    BEFS_OK
}

unsafe fn befs_leafnode(node: *mut befs_btree_node) -> i32 {
    if (*node).head.overflow == BEFS_BT_INVAL { 1 } else { 0 }
}

unsafe fn befs_bt_keylen_index(node: *mut befs_btree_node) -> *mut fs16 {
    let align = 8usize;
    let mut off = core::mem::size_of::<befs_btree_nodehead>() + (*node).head.all_key_length as usize;
    let rem = off % align;
    if rem != 0 { off += align - rem; }
    (*node).od_node.cast::<u8>().add(off).cast()
}

unsafe fn befs_bt_valarray(node: *mut befs_btree_node) -> *mut fs64 {
    befs_bt_keylen_index(node).cast::<u8>().add((*node).head.all_key_count as usize * core::mem::size_of::<fs16>()).cast()
}

unsafe fn befs_bt_keydata(node: *mut befs_btree_node) -> *mut i8 {
    (*node).od_node.cast::<u8>().add(core::mem::size_of::<befs_btree_nodehead>()).cast()
}

unsafe fn befs_compare_strings(key1: *const core::ffi::c_void, keylen1: i32,
                               key2: *const core::ffi::c_void, keylen2: i32) -> i32 {
    let len = core::cmp::min(keylen1, keylen2) as usize;
    let a = core::slice::from_raw_parts(key1.cast::<u8>(), len);
    let b = core::slice::from_raw_parts(key2.cast::<u8>(), len);
    match a.cmp(b) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Equal => keylen1 - keylen2 }
}

unsafe fn befs_bt_get_key(sb: *mut super_block, node: *mut befs_btree_node,
                          index: i32, keylen: *mut u16) -> *mut i8 {
    if index < 0 || index > (*node).head.all_key_count as i32 { *keylen = 0; return core::ptr::null_mut(); }
    let start = befs_bt_keydata(node);
    let ix = befs_bt_keylen_index(node);
    let prev = if index == 0 { 0 } else { fs16_to_cpu(sb, *ix.add((index - 1) as usize)) as i32 };
    *keylen = fs16_to_cpu(sb, *ix.add(index as usize)) - prev as u16;
    start.add(prev as usize)
}

unsafe fn befs_find_key(sb: *mut super_block, node: *mut befs_btree_node,
                        findkey: *const i8, value: *mut befs_off_t) -> i32 {
    let findlen = strlen(findkey) as i32;
    let mut first = 0i32; let mut last = (*node).head.all_key_count as i32 - 1;
    let mut keylen = 0u16;
    let key = befs_bt_get_key(sb, node, last, &mut keylen);
    if befs_compare_strings(key.cast(), keylen as i32, findkey.cast(), findlen) < 0 { return BEFS_BT_OVERFLOW; }
    let vals = befs_bt_valarray(node); let mut mid = 0i32; let mut eq = 0i32;
    while last >= first { mid = (last + first) / 2; let k = befs_bt_get_key(sb, node, mid, &mut keylen);
        eq = befs_compare_strings(k.cast(), keylen as i32, findkey.cast(), findlen);
        if eq == 0 { *value = fs64_to_cpu(sb, *vals.add(mid as usize)); return BEFS_BT_MATCH; }
        if eq > 0 { last = mid - 1; } else { first = mid + 1; }
    }
    *value = fs64_to_cpu(sb, *vals.add(if eq < 0 { (mid + 1) as usize } else { mid as usize }));
    BEFS_BT_NOT_FOUND
}

unsafe fn befs_btree_seekleaf(sb: *mut super_block, ds: *const befs_data_stream,
                              node: *mut befs_btree_node, node_off: *mut befs_off_t) -> i32 {
    if befs_bt_read_node(sb, ds, node, *node_off) != BEFS_OK { return BEFS_ERR; }
    if (*node).head.all_key_count == 0 && befs_leafnode(node) != 0 { return BEFS_BT_EMPTY; }
    while befs_leafnode(node) == 0 {
        *node_off = if (*node).head.all_key_count == 0 { (*node).head.overflow } else { fs64_to_cpu(sb, *befs_bt_valarray(node)) };
        if befs_bt_read_node(sb, ds, node, *node_off) != BEFS_OK { return BEFS_ERR; }
    }
    BEFS_OK
}

pub unsafe fn befs_btree_find(sb: *mut super_block, ds: *const befs_data_stream,
                              key: *const i8, value: *mut befs_off_t) -> i32 {
    let mut sup = core::mem::zeroed::<befs_btree_super>();
    if befs_bt_read_super(sb, ds, &mut sup) != BEFS_OK { *value = 0; return BEFS_ERR; }
    let node = kmalloc_obj::<befs_btree_node>(GFP_NOFS);
    if node.is_null() { *value = 0; return BEFS_ERR; } (*node).bh = core::ptr::null_mut();
    let mut off = sup.root_node_ptr;
    if befs_bt_read_node(sb, ds, node, off) != BEFS_OK { kfree(node); *value = 0; return BEFS_ERR; }
    while befs_leafnode(node) == 0 { if befs_find_key(sb, node, key, &mut off) == BEFS_BT_OVERFLOW { off = (*node).head.overflow; }
        if befs_bt_read_node(sb, ds, node, off) != BEFS_OK { kfree(node); *value = 0; return BEFS_ERR; } }
    let r = befs_find_key(sb, node, key, value); brelse((*node).bh); kfree(node);
    if r == BEFS_BT_MATCH { BEFS_OK } else { *value = 0; BEFS_BT_NOT_FOUND }
}

pub unsafe fn befs_btree_read(sb: *mut super_block, ds: *const befs_data_stream,
                              key_no: i64, bufsize: usize, keybuf: *mut i8,
                              keysize: *mut usize, value: *mut befs_off_t) -> i32 {
    let mut sup = core::mem::zeroed::<befs_btree_super>();
    if befs_bt_read_super(sb, ds, &mut sup) != BEFS_OK { *keysize = 0; *value = 0; return BEFS_ERR; }
    let node = kmalloc_obj::<befs_btree_node>(GFP_NOFS); if node.is_null() { *keysize=0; *value=0; return BEFS_ERR; }
    (*node).bh = core::ptr::null_mut(); let mut off = sup.root_node_ptr;
    let r = befs_btree_seekleaf(sb, ds, node, &mut off); if r != BEFS_OK { brelse((*node).bh); kfree(node); *keysize=0; *value=0; return r; }
    let mut sum = 0i64; while sum + (*node).head.all_key_count as i64 <= key_no {
        if (*node).head.right == BEFS_BT_INVAL { brelse((*node).bh); kfree(node); *keysize=0; *value=0; return BEFS_BT_END; }
        sum += (*node).head.all_key_count as i64; off = (*node).head.right;
        if befs_bt_read_node(sb, ds, node, off) != BEFS_OK { kfree(node); *keysize=0; *value=0; return BEFS_ERR; }
    }
    let idx = (key_no - sum) as i32; let mut len=0u16; let k=befs_bt_get_key(sb,node,idx,&mut len);
    if bufsize < len as usize + 1 { brelse((*node).bh); kfree(node); *keysize=0; *value=0; return BEFS_ERR; }
    core::ptr::copy_nonoverlapping(k.cast::<u8>(), keybuf.cast::<u8>(), len as usize); *keybuf.add(len as usize)=0;
    *value=fs64_to_cpu(sb,*befs_bt_valarray(node).add(idx as usize)); *keysize=len as usize; brelse((*node).bh); kfree(node); BEFS_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
