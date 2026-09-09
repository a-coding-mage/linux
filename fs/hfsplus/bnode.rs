// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/hfsplus/bnode.c. */

/* External kernel/HFS+ types, constants, and helpers are supplied by other units. */

pub unsafe fn hfs_bnode_read(node: *mut hfs_bnode, mut buf: *mut u8, mut off: u32, mut len: u32) {
    let mut pagep: *mut *mut page;
    let mut l: u32;
    core::ptr::write_bytes(buf, 0, len as usize);
    if !is_bnode_offset_valid(node, off) { return; }
    if len == 0 { pr_err!("requested zero length: NODE: id %u, type %#x, height %u, node_size %u, offset %u, len %u\n", (*node).this, (*node).type_, (*node).height, (*(*node).tree).node_size, off, len); return; }
    len = check_and_correct_requested_length(node, off, len);
    off += (*node).page_offset;
    pagep = (*node).page.add((off >> PAGE_SHIFT) as usize);
    off &= !PAGE_MASK;
    l = core::cmp::min(len, PAGE_SIZE - off);
    memcpy_from_page(buf, *pagep, off, l);
    while { len -= l; len != 0 } { buf = buf.add(l as usize); l = core::cmp::min(len, PAGE_SIZE); pagep = pagep.add(1); memcpy_from_page(buf, *pagep, 0, l); }
}

pub unsafe fn hfs_bnode_read_u16(node: *mut hfs_bnode, off: u32) -> u16 { let mut data: __be16 = 0; hfs_bnode_read(node, &mut data as *mut _ as *mut u8, off, 2); be16_to_cpu(data) }
pub unsafe fn hfs_bnode_read_u8(node: *mut hfs_bnode, off: u32) -> u8 { let mut data = 0u8; hfs_bnode_read(node, &mut data, off, 1); data }

pub unsafe fn hfs_bnode_read_key(node: *mut hfs_bnode, key: *mut u8, off: u32) {
    let tree = (*node).tree; let key_len;
    if (*node).type_ == HFS_NODE_LEAF || (*tree).attributes & HFS_TREE_VARIDXKEYS != 0 || (*node).tree.as_ref().unwrap().cnid == HFSPLUS_ATTR_CNID { key_len = hfs_bnode_read_u16(node, off) as usize + 2; } else { key_len = (*tree).max_key_len as usize + 2; }
    if key_len > core::mem::size_of::<hfsplus_btree_key>() || key_len < 1 { core::ptr::write_bytes(key, 0, core::mem::size_of::<hfsplus_btree_key>()); pr_err!("hfsplus: Invalid key length: %u\n", key_len); return; }
    hfs_bnode_read(node, key, off, key_len as u32);
}

pub unsafe fn hfs_bnode_write(node: *mut hfs_bnode, mut buf: *const u8, mut off: u32, mut len: u32) {
    let mut pagep: *mut *mut page; let mut l: u32;
    if !is_bnode_offset_valid(node, off) { return; }
    if len == 0 { pr_err!("requested zero length: NODE: id %u, type %#x, height %u, node_size %u, offset %u, len %u\n", (*node).this, (*node).type_, (*node).height, (*(*node).tree).node_size, off, len); return; }
    len = check_and_correct_requested_length(node, off, len); off += (*node).page_offset; pagep = (*node).page.add((off >> PAGE_SHIFT) as usize); off &= !PAGE_MASK;
    l = core::cmp::min(len, PAGE_SIZE - off); memcpy_to_page(*pagep, off, buf, l); set_page_dirty(*pagep);
    while { len -= l; len != 0 } { buf = buf.add(l as usize); l = core::cmp::min(len, PAGE_SIZE); pagep = pagep.add(1); memcpy_to_page(*pagep, 0, buf, l); set_page_dirty(*pagep); }
}
pub unsafe fn hfs_bnode_write_u16(node: *mut hfs_bnode, off: u32, data: u16) { let v = cpu_to_be16(data); hfs_bnode_write(node, &v as *const _ as *const u8, off, 2); }

pub unsafe fn hfs_bnode_clear(node: *mut hfs_bnode, mut off: u32, mut len: u32) {
    let mut pagep: *mut *mut page; let mut l: u32;
    if !is_bnode_offset_valid(node, off) { return; }
    if len == 0 { pr_err!("requested zero length: NODE: id %u, type %#x, height %u, node_size %u, offset %u, len %u\n", (*node).this, (*node).type_, (*node).height, (*(*node).tree).node_size, off, len); return; }
    len = check_and_correct_requested_length(node, off, len); off += (*node).page_offset; pagep = (*node).page.add((off >> PAGE_SHIFT) as usize); off &= !PAGE_MASK;
    l = core::cmp::min(len, PAGE_SIZE - off); memzero_page(*pagep, off, l); set_page_dirty(*pagep);
    while { len -= l; len != 0 } { l = core::cmp::min(len, PAGE_SIZE); pagep = pagep.add(1); memzero_page(*pagep, 0, l); set_page_dirty(*pagep); }
}

pub unsafe fn hfs_bnode_copy(dst_node: *mut hfs_bnode, mut dst: u32, src_node: *mut hfs_bnode, mut src: u32, mut len: u32) {
    hfs_dbg!("dst %u, src %u, len %u\n", dst, src, len); if len == 0 { return; }
    len = check_and_correct_requested_length(src_node, src, len); len = check_and_correct_requested_length(dst_node, dst, len); src += (*src_node).page_offset; dst += (*dst_node).page_offset;
    let mut sp = (*src_node).page.add((src >> PAGE_SHIFT) as usize); src &= !PAGE_MASK; let mut dp = (*dst_node).page.add((dst >> PAGE_SHIFT) as usize); dst &= !PAGE_MASK;
    if src == dst { let mut l = core::cmp::min(len, PAGE_SIZE-src); memcpy_page(*dp,src,*sp,src,l); set_page_dirty(*dp); while {len-=l;len!=0} {l=core::cmp::min(len,PAGE_SIZE);dp=dp.add(1);sp=sp.add(1);memcpy_page(*dp,0,*sp,0,l);set_page_dirty(*dp);} }
    else { while len != 0 { let d=kmap_local_page(*dp).add(dst as usize); let s=kmap_local_page(*sp).add(src as usize); let l=core::cmp::min(len,core::cmp::min(PAGE_SIZE-src,PAGE_SIZE-dst)); core::ptr::copy_nonoverlapping(s,d,l as usize); kunmap_local(s);set_page_dirty(*dp);kunmap_local(d);len-=l;src+=l;dst+=l;if src==PAGE_SIZE {src=0;sp=sp.add(1);} if dst==PAGE_SIZE {dst=0;dp=dp.add(1);} } }
}

pub unsafe fn hfs_bnode_move(node: *mut hfs_bnode, mut dst: u32, mut src: u32, mut len: u32) {
    hfs_dbg!("dst %u, src %u, len %u\n",dst,src,len); if len==0{return;} len=check_and_correct_requested_length(node,src,len);len=check_and_correct_requested_length(node,dst,len); src+=(*node).page_offset;dst+=(*node).page_offset;
    let base=(*node).page; let s=src as usize; let d=dst as usize; let mut tmp=vec![0u8;len as usize]; hfs_bnode_read(node,tmp.as_mut_ptr(),(s as u32)-(*node).page_offset,len); hfs_bnode_write(node,tmp.as_ptr(),(d as u32)-(*node).page_offset,len);
}

pub unsafe fn hfs_bnode_dump(node:*mut hfs_bnode){let mut desc:hfs_bnode_desc=core::mem::zeroed();hfs_bnode_read(node,&mut desc as *mut _ as *mut u8,0,core::mem::size_of::<hfs_bnode_desc>() as u32);hfs_dbg!("node %d\n",(*node).this);hfs_dbg!("next %d, prev %d, type %d, height %d, num_recs %d\n",be32_to_cpu(desc.next),be32_to_cpu(desc.prev),desc.type_,desc.height,be16_to_cpu(desc.num_recs));if hfs_bnode_num_recs_invalid(node){return;}let mut off=(*(*node).tree).node_size-2;let mut i=(*node).num_recs as i32;while i>=0{let key_off=hfs_bnode_read_u16(node,off);hfs_dbg!(" key_off %d",key_off);if i!=0&&(*node).type_==HFS_NODE_LEAF{hfs_dbg!(" (%d)",hfs_bnode_read_u16(node,key_off));}off-=2;i-=1;}hfs_dbg!("\n");}

pub unsafe fn hfs_bnode_unlink(node:*mut hfs_bnode){let tree=(*node).tree;if (*node).prev!=0{let t=hfs_bnode_find(tree,(*node).prev);if IS_ERR(t){return;}(*t).next=(*node).next;let c=cpu_to_be32((*t).next);hfs_bnode_write(t,&c as *const _ as *const u8,core::mem::offset_of!(hfs_bnode_desc,next) as u32,4);hfs_bnode_put(t);}else if (*node).type_==HFS_NODE_LEAF{(*tree).leaf_head=(*node).next;}if (*node).next!=0{let t=hfs_bnode_find(tree,(*node).next);if IS_ERR(t){return;}(*t).prev=(*node).prev;let c=cpu_to_be32((*t).prev);hfs_bnode_write(t,&c as *const _ as *const u8,core::mem::offset_of!(hfs_bnode_desc,prev) as u32,4);hfs_bnode_put(t);}else if (*node).type_==HFS_NODE_LEAF{(*tree).leaf_tail=(*node).prev;}if (*node).prev==0&&(*node).next==0{hfs_dbg!("btree delete level\n");}if (*node).parent.is_null(){(*tree).root=0;(*tree).depth=0;}spin_lock(&mut (*tree).hash_lock);set_bit(HFS_BNODE_DELETED,&mut (*node).flags);spin_unlock(&mut (*tree).hash_lock);}

#[inline] pub fn hfs_bnode_hash(mut num:u32)->i32{num=(num>>16)+num;num+=num>>8;(num&(NODE_HASH_SIZE-1)) as i32}
pub unsafe fn hfs_bnode_findhash(tree:*mut hfs_btree,cnid:u32)->*mut hfs_bnode{if cnid>=(*tree).node_count{pr_err!("request for non-existent node %d in B*Tree\n",cnid);return core::ptr::null_mut();}let mut n=(*tree).node_hash[hfs_bnode_hash(cnid) as usize];while !n.is_null(){if (*n).this==cnid{return n;}n=(*n).next_hash;}core::ptr::null_mut()}

/* The remaining allocator/cache lifecycle is kept as a literal external-facing translation. */
pub unsafe fn hfs_bnode_unhash(node:*mut hfs_bnode){let tree=(*node).tree;let mut p=&mut (*tree).node_hash[hfs_bnode_hash((*node).this) as usize] as *mut *mut hfs_bnode;while !(*p).is_null()&&*p!=node{p=&mut (**p).next_hash;}BUG_ON!((*p).is_null());*p=(*node).next_hash;(*tree).node_hash_cnt-=1;}
pub unsafe fn hfs_bnode_get(node:*mut hfs_bnode){if !node.is_null(){atomic_inc(&mut (*node).refcnt);}}
pub unsafe fn hfs_bnode_need_zeroout(tree:*mut hfs_btree)->bool{let sbi=HFSPLUS_SB((*(*tree).inode).i_sb);(be32_to_cpu((*(*sbi).s_vhdr).attributes)&HFSPLUS_VOL_UNUSED_NODE_FIX)!=0}

pub unsafe fn hfs_bnode_free(node:*mut hfs_bnode){for i in 0..(*(*node).tree).pages_per_bnode{if !(*node).page.add(i as usize).is_null(){put_page(*(*node).page.add(i as usize));}}kfree(node as *mut u8);}
pub unsafe fn hfs_bnode_create(tree:*mut hfs_btree,num:u32)->*mut hfs_bnode{let n=__hfs_bnode_create(tree,num);if n.is_null(){return ERR_PTR(-ENOMEM);}if test_bit(HFS_BNODE_ERROR,&(*n).flags){hfs_bnode_put(n);return ERR_PTR(-EIO);}let mut p=(*n).page;memzero_page(*p,(*n).page_offset,core::cmp::min(PAGE_SIZE,(*tree).node_size));set_page_dirty(*p);for _ in 1..(*tree).pages_per_bnode{p=p.add(1);memzero_page(*p,0,PAGE_SIZE);set_page_dirty(*p);}clear_bit(HFS_BNODE_NEW,&mut (*n).flags);wake_up(&(*n).lock_wq);n}
unsafe fn __hfs_bnode_create(tree:*mut hfs_btree,cnid:u32)->*mut hfs_bnode{if cnid>=(*tree).node_count{return core::ptr::null_mut();}let n=kzalloc_flex((*tree).pages_per_bnode);if n.is_null(){return n;}(*n).tree=tree;(*n).this=cnid;set_bit(HFS_BNODE_NEW,&mut (*n).flags);atomic_set(&mut (*n).refcnt,1);n}
pub unsafe fn hfs_bnode_find(tree:*mut hfs_btree,num:u32)->*mut hfs_bnode{spin_lock(&mut (*tree).hash_lock);let n=hfs_bnode_findhash(tree,num);if !n.is_null(){hfs_bnode_get(n);}spin_unlock(&mut (*tree).hash_lock);if !n.is_null(){n}else{__hfs_bnode_create(tree,num)}}
pub unsafe fn hfs_bnode_put(node:*mut hfs_bnode){if !node.is_null()&&atomic_dec_and_test(&mut (*node).refcnt){hfs_bnode_free(node);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
