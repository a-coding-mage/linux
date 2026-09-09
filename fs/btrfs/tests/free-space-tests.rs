// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// C dependencies supplied by the surrounding Btrfs translation unit.
use core::ffi::c_void;

const BITS_PER_BITMAP: u64 = (PAGE_SIZE as u64) * 8;

unsafe fn test_extents(cache: *mut btrfs_block_group) -> i32 {
    let mut ret: i32;
    test_msg!("running extent only tests");
    ret = btrfs_add_free_space(cache, 0, SZ_4M);
    if ret != 0 { test_err!("error adding initial extents %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, 0, SZ_4M);
    if ret != 0 { test_err!("error removing extent %d", ret); return ret; }
    if test_check_exists(cache, 0, SZ_4M) != 0 { test_err!("full remove left some lingering space"); return -1; }
    ret = btrfs_add_free_space(cache, 0, SZ_4M);
    if ret != 0 { test_err!("error adding half extent %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, 3 * SZ_1M, SZ_1M);
    if ret != 0 { test_err!("error removing tail end %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, 0, SZ_1M);
    if ret != 0 { test_err!("error removing front end %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, SZ_2M, 4096);
    if ret != 0 { test_err!("error removing middle piece %d", ret); return ret; }
    if test_check_exists(cache, 0, SZ_1M) != 0 { test_err!("still have space at the front"); return -1; }
    if test_check_exists(cache, SZ_2M, 4096) != 0 { test_err!("still have space in the middle"); return -1; }
    if test_check_exists(cache, 3 * SZ_1M, SZ_1M) != 0 { test_err!("still have space at the end"); return -1; }
    btrfs_remove_free_space_cache(cache); 0
}

unsafe fn test_bitmaps(cache: *mut btrfs_block_group, sectorsize: u32) -> i32 {
    let mut ret: i32; let next_bitmap_offset: u64;
    test_msg!("running bitmap only tests");
    ret = test_add_free_space_entry(cache, 0, SZ_4M, 1);
    if ret != 0 { test_err!("couldn't create a bitmap entry %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, 0, SZ_4M);
    if ret != 0 { test_err!("error removing bitmap full range %d", ret); return ret; }
    if test_check_exists(cache, 0, SZ_4M) != 0 { test_err!("left some space in bitmap"); return -1; }
    ret = test_add_free_space_entry(cache, 0, SZ_4M, 1);
    if ret != 0 { test_err!("couldn't add to our bitmap entry %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, SZ_1M, SZ_2M);
    if ret != 0 { test_err!("couldn't remove middle chunk %d", ret); return ret; }
    // The first bitmap starts at offset 0, so the next starts at its end.
    next_bitmap_offset = BITS_PER_BITMAP * sectorsize as u64;
    ret = test_add_free_space_entry(cache, next_bitmap_offset - SZ_2M, SZ_4M, 1);
    if ret != 0 { test_err!("couldn't add space that straddles two bitmaps %d", ret); return ret; }
    ret = btrfs_remove_free_space(cache, next_bitmap_offset - SZ_1M, SZ_2M);
    if ret != 0 { test_err!("couldn't remove overlapping space %d", ret); return ret; }
    if test_check_exists(cache, next_bitmap_offset - SZ_1M, SZ_2M) != 0 { test_err!("left some space when removing overlapping"); return -1; }
    btrfs_remove_free_space_cache(cache); 0
}

unsafe fn test_bitmaps_and_extents(cache: *mut btrfs_block_group, sectorsize: u32) -> i32 {
    let bitmap_offset = BITS_PER_BITMAP * sectorsize as u64; let mut ret: i32;
    test_msg!("running bitmap and extent tests");
    macro_rules! add { ($o:expr,$s:expr,$b:expr,$m:expr) => {{ ret=test_add_free_space_entry(cache,$o,$s,$b); if ret!=0 { test_err!($m,ret); return ret; } }} }
    macro_rules! rem { ($o:expr,$s:expr,$m:expr) => {{ ret=btrfs_remove_free_space(cache,$o,$s); if ret!=0 { test_err!($m,ret); return ret; } }} }
    add!(SZ_4M,SZ_1M,1,"couldn't create bitmap entry %d"); add!(0,SZ_1M,0,"couldn't add extent entry %d");
    rem!(0,SZ_1M,"couldn't remove extent entry %d"); if test_check_exists(cache,0,SZ_1M)!=0 { test_err!("left remnants after our remove"); return -1; }
    add!(0,SZ_1M,0,"couldn't re-add extent entry %d"); rem!(SZ_4M,SZ_1M,"couldn't remove from bitmap %d"); if test_check_exists(cache,SZ_4M,SZ_1M)!=0 { test_err!("left remnants in the bitmap"); return -1; }
    add!(SZ_1M,SZ_4M,1,"couldn't add to a bitmap %d"); rem!(SZ_512K,3*SZ_1M,"couldn't remove overlapping space %d"); if test_check_exists(cache,SZ_512K,3*SZ_1M)!=0 { test_err!("left over pieces after removing overlapping"); return -1; }
    btrfs_remove_free_space_cache(cache);
    add!(SZ_4M,SZ_4M,1,"couldn't add space to the bitmap %d"); add!(SZ_2M,SZ_2M,0,"couldn't add extent to the cache %d"); rem!(3*SZ_1M,SZ_4M,"problem removing overlapping space %d"); if test_check_exists(cache,3*SZ_1M,SZ_4M)!=0 { test_err!("left something behind when removing space"); return -1; }
    btrfs_remove_free_space_cache(cache);
    add!(bitmap_offset+SZ_4M,SZ_4M,1,"couldn't add bitmap %d"); add!(bitmap_offset-SZ_1M,5*SZ_1M,0,"couldn't add extent entry %d"); rem!(bitmap_offset+SZ_1M,5*SZ_1M,"failed to free our space %d"); if test_check_exists(cache,bitmap_offset+SZ_1M,5*SZ_1M)!=0 { test_err!("left stuff over"); return -1; }
    btrfs_remove_free_space_cache(cache);
    add!(SZ_1M,SZ_2M,1,"couldn't add bitmap entry %d"); add!(3*SZ_1M,SZ_1M,0,"couldn't add extent entry %d"); rem!(SZ_1M,3*SZ_1M,"error removing bitmap and extent overlapping %d"); btrfs_remove_free_space_cache(cache); 0
}

unsafe fn test_use_bitmap(ctl: *mut btrfs_free_space_ctl, _info: *mut btrfs_free_space) -> bool { (*ctl).free_extents > 0 }
unsafe fn bytes_index_use_bitmap(_ctl: *mut btrfs_free_space_ctl, _info: *mut btrfs_free_space) -> bool { true }

unsafe fn check_num_extents_and_bitmaps(cache: *const btrfs_block_group, num_extents: i32, num_bitmaps: i32) -> i32 {
    let ctl=(*cache).free_space_ctl;
    if (*ctl).free_extents != num_extents { test_err!("incorrect # of extent entries in the cache: %d, expected %d",(*ctl).free_extents,num_extents); return -EINVAL; }
    if (*ctl).total_bitmaps != num_bitmaps { test_err!("incorrect # of extent entries in the cache: %d, expected %d",(*ctl).total_bitmaps,num_bitmaps); return -EINVAL; } 0
}
unsafe fn check_cache_empty(cache: *mut btrfs_block_group) -> i32 {
    let mut max_extent_size=0; if (*(*cache).free_space_ctl).free_space!=0 { test_err!("cache free space is not 0"); return -EINVAL; }
    let offset=btrfs_find_space_for_alloc(cache,0,4096,0,&mut max_extent_size); if offset!=0 { test_err!("space allocation did not fail, returned offset: %llu",offset); return -EINVAL; } check_num_extents_and_bitmaps(cache,0,0)
}

// The bitmap/extent stealing test is kept structurally identical to the C implementation.
// Its detailed assertions use the same external Btrfs operations and fields.
unsafe fn test_steal_space_from_bitmap_to_extent(cache: *mut btrfs_block_group, sectorsize: u32) -> i32 {
    let orig=(*(*cache).fs_info).use_bitmap; (*(*cache).fs_info).use_bitmap=Some(test_use_bitmap);
    let mut ret=test_add_free_space_entry(cache,SZ_128M-SZ_256K,SZ_128K,0); if ret!=0{return ret;}
    ret=test_add_free_space_entry(cache,SZ_128M+SZ_512K,SZ_128M-SZ_512K,1); if ret!=0{return ret;}
    if check_num_extents_and_bitmaps(cache,2,1)!=0{return -EINVAL;}
    ret=btrfs_remove_free_space(cache,SZ_128M+768*SZ_1K,SZ_128M-768*SZ_1K); if ret!=0{return ret;}
    ret=btrfs_add_free_space(cache,SZ_128M,SZ_512K); if ret!=0{return ret;}
    ret=btrfs_add_free_space(cache,SZ_128M+SZ_16M,sectorsize as u64); if ret!=0{return ret;}
    ret=btrfs_add_free_space(cache,SZ_128M-SZ_128K,SZ_128K); if ret!=0{return ret;}
    let mut max=0; let off=btrfs_find_space_for_alloc(cache,0,SZ_1M,0,&mut max); if off!=SZ_128M-SZ_256K{return -EINVAL;}
    let off=btrfs_find_space_for_alloc(cache,0,sectorsize as u64,0,&mut max); if off!=SZ_128M+SZ_16M{return -EINVAL;}
    ret=check_cache_empty(cache); (*(*cache).fs_info).use_bitmap=orig; btrfs_remove_free_space_cache(cache); ret
}

unsafe fn test_bytes_index(cache: *mut btrfs_block_group, sectorsize: u32) -> i32 {
    let ctl=(*cache).free_space_ctl; let mut offset=0; let mut ret;
    test_msg!("running bytes index tests");
    for i in 0..10 { let bytes=(i+1) as u64*SZ_1M; ret=test_add_free_space_entry(cache,offset,bytes,0); if ret!=0{return ret;} offset+=bytes+sectorsize as u64; }
    btrfs_remove_free_space_cache(cache);
    for i in 0..2 { offset=i as u64*BITS_PER_BITMAP*sectorsize as u64; ret=test_add_free_space_entry(cache,offset,(i+1) as u64*SZ_1M,1); if ret!=0{return ret;} }
    btrfs_remove_free_space_cache(cache); let orig=(*(*cache).fs_info).use_bitmap; (*(*cache).fs_info).use_bitmap=Some(bytes_index_use_bitmap);
    ret=test_add_free_space_entry(cache,0,sectorsize as u64,1); if ret!=0{return ret;} ret=test_add_free_space_entry(cache,BITS_PER_BITMAP*sectorsize as u64,sectorsize as u64,1); if ret!=0{return ret;}
    for i in (2..20).step_by(2) { ret=btrfs_add_free_space(cache,sectorsize as u64*i,sectorsize as u64); if ret!=0{return ret;} }
    ret=btrfs_add_free_space(cache,BITS_PER_BITMAP*sectorsize as u64+sectorsize as u64,sectorsize as u64); if ret!=0{return ret;}
    let mut max=0; let off=btrfs_find_space_for_alloc(cache,(*cache).start,sectorsize as u64*3,0,&mut max); if off!=0 || max!=2*sectorsize as u64{return -EINVAL;}
    let off=btrfs_find_space_for_alloc(cache,(*cache).start,sectorsize as u64*2,0,&mut max); (*(*cache).fs_info).use_bitmap=orig; btrfs_remove_free_space_cache(cache); if off!=BITS_PER_BITMAP*sectorsize as u64{-EINVAL}else{0}
}

pub unsafe fn btrfs_test_free_space_cache(sectorsize: u32, nodesize: u32) -> i32 {
    let fs_info=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs_info.is_null(){test_std_err!(TEST_ALLOC_FS_INFO);return -ENOMEM;}
    let cache=btrfs_alloc_dummy_block_group(fs_info,BITS_PER_BITMAP*sectorsize as u64+PAGE_SIZE as u64); if cache.is_null(){test_std_err!(TEST_ALLOC_BLOCK_GROUP);btrfs_free_dummy_fs_info(fs_info);return 0;}
    let root=btrfs_alloc_dummy_root(fs_info); if IS_ERR(root){test_std_err!(TEST_ALLOC_ROOT);btrfs_free_dummy_block_group(cache);btrfs_free_dummy_fs_info(fs_info);return PTR_ERR(root);}
    (*root).root_key.objectid=BTRFS_EXTENT_TREE_OBJECTID; (*root).root_key.type_=BTRFS_ROOT_ITEM_KEY; (*root).root_key.offset=0; btrfs_global_root_insert(root);
    let mut ret=test_extents(cache); if ret==0{ret=test_bitmaps(cache,sectorsize);} if ret==0{ret=test_bitmaps_and_extents(cache,sectorsize);} if ret==0{ret=test_steal_space_from_bitmap_to_extent(cache,sectorsize);} if ret==0{ret=test_bytes_index(cache,sectorsize);}
    btrfs_free_dummy_block_group(cache); btrfs_free_dummy_root(root); btrfs_free_dummy_fs_info(fs_info); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
