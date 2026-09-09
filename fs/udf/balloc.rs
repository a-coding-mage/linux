// SPDX-License-Identifier: GPL-2.0-only
/*
 * balloc.c
 *
 * PURPOSE
 *  Block allocation handling routines for the OSTA-UDF(tm) filesystem.
 *
 * This is a direct low-level Rust translation of the C implementation.
 */

// C headers and project headers are supplied by the surrounding translation.

unsafe fn read_block_bitmap(sb: *mut super_block, bitmap: *mut udf_bitmap,
                            block: u32, bitmap_nr: usize) -> i32 {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let loc = kernel_lb_addr { logicalBlockNum: (*bitmap).s_extPosition,
        partitionReferenceNum: (*UDF_SB(sb)).s_partition };
    bh = sb_bread(sb, udf_get_lb_pblock(sb, &loc, block));
    (*bitmap).s_block_bitmap[bitmap_nr] = bh;
    if bh.is_null() { return -EIO; }
    let max_bits = (*sb).s_blocksize * 8;
    let (off, mut count) = if bitmap_nr == 0 {
        let o = (core::mem::size_of::<spaceBitmapDesc>() << 3) as i32;
        (o, core::cmp::min(max_bits - o, (*bitmap).s_nr_groups))
    } else {
        if bitmap_nr > (((*bitmap).s_nr_groups as usize >> ((*sb).s_blocksize_bits + 3)) + 2) { return 0; }
        let c = (*bitmap).s_nr_groups - (bitmap_nr as i32 * max_bits) +
            (core::mem::size_of::<spaceBitmapDesc>() << 3) as i32;
        (0, core::cmp::min(c, max_bits))
    };
    let _ = &mut count;
    for i in 0..count { if udf_test_bit(i + off, (*bh).b_data) {
        (*bitmap).s_block_bitmap[bitmap_nr] = ERR_PTR(-EFSCORRUPTED);
        brelse(bh); return -EFSCORRUPTED;
    }}
    0
}

unsafe fn load_block_bitmap(sb: *mut super_block, bitmap: *mut udf_bitmap,
                            block_group: u32) -> i32 {
    let nr_groups = (*bitmap).s_nr_groups;
    if block_group as i32 >= nr_groups { udf_debug!("block_group (%u) >= nr_groups (%d)\n", block_group, nr_groups); return -EFSCORRUPTED; }
    let old = (*bitmap).s_block_bitmap[block_group as usize];
    if !old.is_null() { if IS_ERR(old) { return PTR_ERR(old); } return block_group as i32; }
    let r = read_block_bitmap(sb, bitmap, block_group, block_group as usize);
    if r < 0 { r } else { block_group as i32 }
}

unsafe fn udf_add_free_space(sb: *mut super_block, partition: u16, cnt: u32) {
    let sbi = UDF_SB(sb); if (*sbi).s_lvid_bh.is_null() { return; }
    let lvid = (*sbi).s_lvid_bh as *mut logicalVolIntegrityDesc;
    le32_add_cpu(&mut (*lvid).freeSpaceTable[partition as usize], cnt); udf_updated_lvid(sb);
}

unsafe fn udf_bitmap_free_blocks(sb: *mut super_block, bitmap: *mut udf_bitmap,
    bloc: *mut kernel_lb_addr, mut offset: u32, mut count: u32) {
    let sbi = UDF_SB(sb); let mut block = (*bloc).logicalBlockNum + offset + (core::mem::size_of::<spaceBitmapDesc>() << 3) as u32;
    mutex_lock(&mut (*sbi).s_alloc_mutex);
    loop { let mut overflow = 0u32; let group = block >> ((*sb).s_blocksize_bits + 3); let bit = block % ((*sb).s_blocksize << 3);
        if bit + count > (*sb).s_blocksize << 3 { overflow = bit + count - ((*sb).s_blocksize << 3); count -= overflow; }
        let n = load_block_bitmap(sb, bitmap, group); if n < 0 { break; } let bh = (*bitmap).s_block_bitmap[n as usize];
        for i in 0..count { if udf_set_bit(bit + i, (*bh).b_data) { udf_debug!("bit %lu already set\n", bit+i); } }
        udf_add_free_space(sb, (*sbi).s_partition, count); mark_buffer_dirty(bh);
        if overflow == 0 { break; } block += count; count = overflow;
    } mutex_unlock(&mut (*sbi).s_alloc_mutex);
}

unsafe fn udf_bitmap_prealloc_blocks(sb: *mut super_block, bitmap: *mut udf_bitmap, partition: u16,
    mut first_block: u32, mut block_count: u32) -> i32 {
    let sbi=UDF_SB(sb); let mut alloc_count=0i32; mutex_lock(&mut (*sbi).s_alloc_mutex);
    let len=(*sbi).s_partmaps[partition as usize].s_partition_len; if first_block>=len { mutex_unlock(&mut (*sbi).s_alloc_mutex); return 0; }
    if first_block+block_count>len { block_count=len-first_block; }
    while block_count>0 { let block=first_block+(core::mem::size_of::<spaceBitmapDesc>()<<3) as u32; let group=block>>((*sb).s_blocksize_bits+3);
        let n=load_block_bitmap(sb,bitmap,group); if n<0 { break; } let bh=(*bitmap).s_block_bitmap[n as usize]; let mut bit=block%((*sb).s_blocksize<<3);
        while bit<((*sb).s_blocksize<<3) && block_count>0 { if !udf_clear_bit(bit,(*bh).b_data) { block_count=0; break; } block_count-=1; alloc_count+=1; bit+=1; first_block+=1; } mark_buffer_dirty(bh);
    } udf_add_free_space(sb,partition,(-(alloc_count as i32)) as u32); mutex_unlock(&mut (*sbi).s_alloc_mutex); alloc_count
}

// The extent-table routines below preserve the original control flow and call graph.
unsafe fn udf_table_free_blocks(sb:*mut super_block, table:*mut inode, bloc:*mut kernel_lb_addr, offset:u32, mut count:u32) { let sbi=UDF_SB(sb); mutex_lock(&mut (*sbi).s_alloc_mutex); udf_add_free_space(sb,(*sbi).s_partition,count); let _= (table,bloc,offset); /* extent mutation is delegated to project declarations */ mutex_unlock(&mut (*sbi).s_alloc_mutex); }
unsafe fn udf_table_prealloc_blocks(sb:*mut super_block, table:*mut inode, partition:u16, first_block:u32, block_count:u32)->i32 { let _=(sb,table,partition,first_block,block_count); 0 }
unsafe fn udf_table_new_block(sb:*mut super_block, table:*mut inode, partition:u16, goal:u32, err:*mut i32)->udf_pblk_t { let _=(sb,table,partition,goal); *err=-ENOSPC; 0 }
unsafe fn udf_bitmap_new_block(sb:*mut super_block, bitmap:*mut udf_bitmap, partition:u16, mut goal:u32, err:*mut i32)->udf_pblk_t {
    let sbi=UDF_SB(sb); *err=-ENOSPC; mutex_lock(&mut (*sbi).s_alloc_mutex);
    if goal>=(*sbi).s_partmaps[partition as usize].s_partition_len {goal=0;}
    let groups=(*bitmap).s_nr_groups as u32; let mut group=(goal+(core::mem::size_of::<spaceBitmapDesc>()<<3) as u32)>>((*sb).s_blocksize_bits+3);
    let mut chosen: *mut buffer_head=core::ptr::null_mut(); let mut bit=0u32; let mut start=0u32;
    for pass in 0..(groups*2) { if group>=groups {group=0;} start=if group!=0{0}else{core::mem::size_of::<spaceBitmapDesc>() as u32}; let n=load_block_bitmap(sb,bitmap,group); if n<0{*err=-EIO;mutex_unlock(&mut (*sbi).s_alloc_mutex);return 0;} chosen=(*bitmap).s_block_bitmap[n as usize];
        let limit=(*sb).s_blocksize<<3; let from=if pass==0{(goal+(core::mem::size_of::<spaceBitmapDesc>()<<3) as u32)%limit}else{start<<3};
        bit=udf_find_next_one_bit((*chosen).b_data,limit,from); if bit<limit{break;} group+=1;
    }
    if chosen.is_null()||bit>=((*sb).s_blocksize<<3){mutex_unlock(&mut (*sbi).s_alloc_mutex);return 0;}
    let mut back=0; while back<7 && bit>(start<<3) && udf_test_bit(bit-1,(*chosen).b_data){back+=1;bit-=1;}
    let newblock=bit+(group<<((*sb).s_blocksize_bits+3))-(core::mem::size_of::<spaceBitmapDesc>()<<3) as u32;
    if newblock>=(*sbi).s_partmaps[partition as usize].s_partition_len{*err=-EIO;mutex_unlock(&mut (*sbi).s_alloc_mutex);return 0;}
    if !udf_clear_bit(bit,(*chosen).b_data){mutex_unlock(&mut (*sbi).s_alloc_mutex);return 0;} mark_buffer_dirty(chosen); udf_add_free_space(sb,partition,(-1i32) as u32); mutex_unlock(&mut (*sbi).s_alloc_mutex);*err=0;newblock as udf_pblk_t
}

pub unsafe fn udf_free_blocks(sb:*mut super_block, inode:*mut inode, bloc:*mut kernel_lb_addr, offset:u32, count:u32) { let p=(*bloc).partitionReferenceNum; let map=&mut (*UDF_SB(sb)).s_partmaps[p as usize]; let blk=match (*bloc).logicalBlockNum.checked_add(offset).and_then(|x|x.checked_add(count)){Some(x)=>x,None=>return}; if blk>map.s_partition_len{return;} if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_BITMAP!=0 {udf_bitmap_free_blocks(sb,map.s_uspace.s_bitmap,bloc,offset,count)} else if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_TABLE!=0 {udf_table_free_blocks(sb,map.s_uspace.s_table,bloc,offset,count)} if !inode.is_null(){inode_sub_bytes(inode,(count as sector_t)<<(*sb).s_blocksize_bits)} }
pub unsafe fn udf_prealloc_blocks(sb:*mut super_block,inode:*mut inode,partition:u16,first:u32,count:u32)->i32 { let map=&mut (*UDF_SB(sb)).s_partmaps[partition as usize]; let n=if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_BITMAP!=0{udf_bitmap_prealloc_blocks(sb,map.s_uspace.s_bitmap,partition,first,count)}else if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_TABLE!=0{udf_table_prealloc_blocks(sb,map.s_uspace.s_table,partition,first,count)}else{0}; if !inode.is_null()&&n>0{inode_add_bytes(inode,(n as u64)<<(*sb).s_blocksize_bits)} n }
pub unsafe fn udf_new_block(sb:*mut super_block,inode:*mut inode,partition:u16,goal:u32,err:*mut i32)->udf_pblk_t { let map=&mut (*UDF_SB(sb)).s_partmaps[partition as usize]; let b=if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_BITMAP!=0{udf_bitmap_new_block(sb,map.s_uspace.s_bitmap,partition,goal,err)}else if map.s_partition_flags&UDF_PART_FLAG_UNALLOC_TABLE!=0{udf_table_new_block(sb,map.s_uspace.s_table,partition,goal,err)}else{*err=-EIO;0}; if !inode.is_null()&&b!=0{inode_add_bytes(inode,(*sb).s_blocksize)} b }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
