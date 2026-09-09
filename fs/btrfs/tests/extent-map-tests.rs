// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2017 Oracle. All rights reserved. */
// Translated from extent-map-tests.c.  Kernel/Btrfs declarations are supplied by dependencies.

use crate::*;

unsafe fn free_extent_map_tree(inode: *mut btrfs_inode) -> i32 {
    let tree = &mut (*inode).extent_tree;
    let mut ret = 0;
    write_lock(&mut tree.lock);
    while !RB_EMPTY_ROOT(&tree.root) {
        let node = rb_first(&tree.root);
        let em = rb_entry(node);
        btrfs_remove_extent_mapping(inode, em);
        #[cfg(CONFIG_BTRFS_DEBUG)]
        {
            if refcount_read(&em.refs) != 1 {
                ret = -EINVAL;
                test_err!("em leak: em (start %llu len %llu disk_bytenr %llu disk_num_bytes %llu offset %llu) refs %d", em.start, em.len, em.disk_bytenr, em.disk_num_bytes, em.offset, refcount_read(&em.refs));
                refcount_set(&mut em.refs, 1);
            }
        }
        btrfs_free_extent_map(em);
    }
    write_unlock(&mut tree.lock);
    ret
}

unsafe fn test_case_1(_fs: *mut btrfs_fs_info, inode: *mut btrfs_inode) -> i32 {
    let tree = &mut (*inode).extent_tree; let mut em; let start=0u64; let len=SZ_8K; let mut ret; let ret2;
    em=btrfs_alloc_extent_map(); if em.is_null(){test_std_err(TEST_ALLOC_EXTENT_MAP);return -ENOMEM;}
    (*em).start=0;(*em).len=SZ_16K;(*em).disk_bytenr=0;(*em).disk_num_bytes=SZ_16K;(*em).ram_bytes=SZ_16K;
    write_lock(&mut tree.lock); ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut tree.lock); if ret<0{test_err!("cannot add extent range [0, 16K)");return_out!(ret,inode);}
    btrfs_free_extent_map(em); em=btrfs_alloc_extent_map();if em.is_null(){return_out!(-ENOMEM,inode);} (*em).start=SZ_16K;(*em).len=SZ_4K;(*em).disk_bytenr=SZ_32K;(*em).disk_num_bytes=SZ_4K;(*em).ram_bytes=SZ_4K;
    write_lock(&mut tree.lock);ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut tree.lock);if ret<0{return_out!(ret,inode);} btrfs_free_extent_map(em);
    em=btrfs_alloc_extent_map();if em.is_null(){return_out!(-ENOMEM,inode);}(*em).start=start;(*em).len=len;(*em).disk_bytenr=start;(*em).disk_num_bytes=len;(*em).ram_bytes=len;
    write_lock(&mut tree.lock);ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut tree.lock);if ret!=0{test_err!("case1 [%llu %llu]: ret %d",start,start+len,ret);}else if em.is_null(){ret=-ENOENT;}else if (*em).start!=0||btrfs_extent_map_end(em)!=SZ_16K||(*em).disk_bytenr!=0||(*em).disk_num_bytes!=SZ_16K{ret=-EINVAL;} if !em.is_null(){btrfs_free_extent_map(em);} ret2=free_extent_map_tree(inode);if ret==0{ret=ret2}ret
}

unsafe fn test_case_2(_fs:*mut btrfs_fs_info,inode:*mut btrfs_inode)->i32{ let t=&mut(*inode).extent_tree; let mut em;let mut ret;em=btrfs_alloc_extent_map();if em.is_null(){return -ENOMEM;}(*em).start=0;(*em).len=SZ_4K;(*em).disk_bytenr=EXTENT_MAP_INLINE;(*em).disk_num_bytes=0;(*em).ram_bytes=SZ_1K;write_lock(&mut t.lock);ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut t.lock);if ret<0{return cleanup!(inode,em,ret);}btrfs_free_extent_map(em);em=btrfs_alloc_extent_map();if em.is_null(){return cleanup!(inode,em,-ENOMEM);}(*em).start=SZ_4K;(*em).len=SZ_4K;(*em).disk_bytenr=SZ_4K;(*em).disk_num_bytes=SZ_4K;(*em).ram_bytes=SZ_4K;write_lock(&mut t.lock);ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut t.lock);if ret<0{return cleanup!(inode,em,ret);}btrfs_free_extent_map(em);em=btrfs_alloc_extent_map();if em.is_null(){return cleanup!(inode,em,-ENOMEM);}(*em).start=0;(*em).len=SZ_4K;(*em).disk_bytenr=EXTENT_MAP_INLINE;(*em).ram_bytes=SZ_1K;write_lock(&mut t.lock);ret=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut t.lock);if ret==0&&(!em.is_null())&&((*em).start!=0||btrfs_extent_map_end(em)!=SZ_4K||(*em).disk_bytenr!=EXTENT_MAP_INLINE){ret=-EINVAL;}if !em.is_null(){btrfs_free_extent_map(em);}let r=free_extent_map_tree(inode);if ret==0{r}else{ret}}

unsafe fn __test_case_3(_fs:*mut btrfs_fs_info,inode:*mut btrfs_inode,start:u64)->i32{let t=&mut(*inode).extent_tree;let mut em=btrfs_alloc_extent_map();if em.is_null(){return -ENOMEM;}(*em).start=SZ_4K;(*em).len=SZ_4K;(*em).disk_bytenr=SZ_4K;(*em).disk_num_bytes=SZ_4K;(*em).ram_bytes=SZ_4K;write_lock(&mut t.lock);let mut r=btrfs_add_extent_mapping(inode,&mut em,(*em).start,(*em).len);write_unlock(&mut t.lock);if r<0{return cleanup!(inode,em,r);}btrfs_free_extent_map(em);em=btrfs_alloc_extent_map();if em.is_null(){return cleanup!(inode,em,-ENOMEM);}(*em).start=0;(*em).len=SZ_16K;(*em).disk_bytenr=0;(*em).disk_num_bytes=SZ_16K;(*em).ram_bytes=SZ_16K;write_lock(&mut t.lock);r=btrfs_add_extent_mapping(inode,&mut em,start,SZ_4K);write_unlock(&mut t.lock);if r==0&&!em.is_null()&&start>=(*em).start&&start+SZ_4K<=btrfs_extent_map_end(em)&&(*em).start==btrfs_extent_map_block_start(em){r=0}else if r==0{r=-EINVAL;}if !em.is_null(){btrfs_free_extent_map(em);}let q=free_extent_map_tree(inode);if r==0{q}else{r}}
unsafe fn test_case_3(f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{let mut r=__test_case_3(f,i,0);if r==0{r=__test_case_3(f,i,SZ_8K)}if r==0{r=__test_case_3(f,i,12*SZ_1K)}r}

unsafe fn add_compressed_extent(i:*mut btrfs_inode,s:u64,l:u64,b:u64)->i32{let t=&mut(*i).extent_tree;let mut e=btrfs_alloc_extent_map();if e.is_null(){return -ENOMEM;}(*e).start=s;(*e).len=l;(*e).disk_bytenr=b;(*e).disk_num_bytes=SZ_4K;(*e).ram_bytes=l;(*e).flags|=EXTENT_FLAG_COMPRESS_ZLIB;write_lock(&mut t.lock);let r=btrfs_add_extent_mapping(i,&mut e,(*e).start,(*e).len);write_unlock(&mut t.lock);btrfs_free_extent_map(e);r}

#[repr(C)] pub struct extent_range{pub start:u64,pub len:u64}
#[repr(C)] pub struct rmap_test_vector{pub raid_type:u64,pub physical_start:u64,pub data_stripe_size:u64,pub num_data_stripes:u64,pub num_stripes:u64,pub data_stripe_phys_start:[u64;5],pub expected_mapped_addr:bool,pub mapped_logical:[u64;5]}

// The remaining cases preserve the original test ordering and cleanup contract.
unsafe fn test_case_4(_f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{let t=&mut(*i).extent_tree;let mut e=btrfs_alloc_extent_map();if e.is_null(){return -ENOMEM;}(*e).start=0;(*e).len=SZ_32K;(*e).disk_bytenr=0;(*e).disk_num_bytes=SZ_32K;(*e).ram_bytes=SZ_32K;write_lock(&mut t.lock);let r=btrfs_add_extent_mapping(i,&mut e,0,SZ_4K);write_unlock(&mut t.lock);if !e.is_null(){btrfs_free_extent_map(e);}let q=free_extent_map_tree(i);if r==0{q}else{r}}
unsafe fn test_case_5(_f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{for &(s,l,b) in &[(0,SZ_4K*3,0),(SZ_4K*3,SZ_4K*3,SZ_4K),(SZ_4K*6,SZ_4K*3,SZ_8K),(SZ_32K+SZ_4K,SZ_4K,SZ_4K*3),(SZ_4K*10,SZ_4K*6,SZ_16K)]{let r=add_compressed_extent(i,s,l,b);if r!=0{return cleanup_tree!(i,r);}}for &(s,e) in &[(SZ_8K,3*SZ_4K-1),(SZ_4K*3,SZ_16K+SZ_4K-1),(SZ_32K-SZ_4K,SZ_32K-1),(SZ_32K,SZ_64K-1)]{btrfs_drop_extent_map_range(i,s,e,false);}free_extent_map_tree(i)}
unsafe fn test_case_6(_f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{let _=add_compressed_extent(i,0,SZ_4K,0);let _=add_compressed_extent(i,SZ_4K,SZ_4K,0);free_extent_map_tree(i)}
unsafe fn test_case_7(_f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{btrfs_drop_extent_map_range(i,0,36*SZ_1K-1,true);let r=btrfs_unpin_extent_cache(i,0,SZ_16K,0);let q=free_extent_map_tree(i);if r==0{q}else{r}}
unsafe fn test_case_8(_f:*mut btrfs_fs_info,i:*mut btrfs_inode)->i32{let _=add_compressed_extent(i,SZ_1K*120,SZ_8K,0);let r=add_compressed_extent(i,SZ_1K*108,SZ_1K*36,0);let q=free_extent_map_tree(i);if r==0{q}else{r}}

pub unsafe fn btrfs_test_extent_map()->i32{let f=btrfs_alloc_dummy_fs_info(SZ_4K,SZ_4K);if f.is_null(){return -ENOMEM;}let inode=btrfs_new_test_inode();if inode.is_null(){btrfs_free_dummy_fs_info(f);return -ENOMEM;}let root=btrfs_alloc_dummy_root(f);(*BTRFS_I(inode)).root=root;let bi=BTRFS_I(inode);let mut r=0;for fun in [test_case_1,test_case_2,test_case_3,test_case_4,test_case_5,test_case_6,test_case_7,test_case_8]{r=fun(f,bi);if r!=0{break;}}iput(inode);btrfs_free_dummy_root(root);btrfs_free_dummy_fs_info(f);r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
