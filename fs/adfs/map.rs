// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/adfs/map.c
 *
 *  Copyright (C) 1997-2002 Russell King
 */

/* C headers and "adfs.h" provide the external kernel types and functions. */

static mut ADFS_MAP_LOCK: RwLock = RwLock::new();

#[inline]
unsafe fn get_frag_id(map: *const u8, start: u32, idmask: u32) -> u32 {
    let m = map.add((start >> 3) as usize);
    let mut frag = get_unaligned_le32(m);
    frag >>= start & 7;
    frag & idmask
}

unsafe fn lookup_zone(dm: *const adfs_discmap, idlen: u32, frag_id: u32,
                      offset: *mut u32) -> i32 {
    let endbit = (*dm).dm_endbit;
    let idmask = (1u32 << idlen) - 1;
    let map = (*(*dm).dm_bh).b_data;
    let mut start = (*dm).dm_startbit;
    let mut freelink;
    let mut fragend;
    let mut frag = get_frag_id(map, 8, idmask & 0x7fff);
    freelink = if frag != 0 { 8 + frag } else { 0 };

    loop {
        frag = get_frag_id(map, start, idmask);
        fragend = find_next_bit_le(map, endbit, start + idlen);
        if fragend >= endbit {
            printk(KERN_ERR, "adfs: oversized fragment 0x%x at 0x%x-0x%x\n", frag, start, fragend);
            return -1;
        }
        if start == freelink {
            freelink += frag & 0x7fff;
        } else if frag == frag_id {
            let length = fragend + 1 - start;
            if *offset < length { return (start + *offset) as i32; }
            *offset -= length;
        }
        start = fragend + 1;
        if start >= endbit { return -1; }
    }
}

unsafe fn scan_free_map(asb: *mut adfs_sb_info, dm: *mut adfs_discmap) -> u32 {
    let endbit = (*dm).dm_endbit;
    let idlen = (*asb).s_idlen;
    let frag_idlen = if idlen <= 15 { idlen } else { 15 };
    let idmask = (1u32 << frag_idlen) - 1;
    let map = (*(*dm).dm_bh).b_data;
    let mut start = 8;
    let mut frag = get_frag_id(map, start, idmask);
    let mut total: u32 = 0;
    if frag == 0 { return 0; }
    loop {
        start += frag;
        frag = get_frag_id(map, start, idmask);
        let fragend = find_next_bit_le(map, endbit, start + idlen);
        if fragend >= endbit {
            printk(KERN_ERR, "adfs: oversized free fragment\n");
            return 0;
        }
        total += fragend + 1 - start;
        if frag < idlen + 1 { break; }
    }
    if frag != 0 { printk(KERN_ERR, "adfs: undersized free fragment\n"); }
    total
}

unsafe fn scan_map(asb: *mut adfs_sb_info, mut zone: u32, frag_id: u32, mut mapoff: u32) -> i32 {
    let idlen = (*asb).s_idlen;
    let mut dm = (*asb).s_map.add(zone as usize);
    let dm_end = (*asb).s_map.add((*asb).s_map_size as usize);
    loop {
        let result = lookup_zone(dm, idlen, frag_id, &mut mapoff);
        if result != -1 { return result - (*dm).dm_startbit as i32 + (*dm).dm_startblk as i32; }
        dm = dm.add(1);
        if dm == dm_end { dm = (*asb).s_map; }
        zone -= 1;
        if zone == 0 { return -1; }
    }
}

pub unsafe fn adfs_map_statfs(sb: *mut super_block, buf: *mut kstatfs) {
    let asb = ADFS_SB(sb);
    let dr = adfs_map_discrecord((*asb).s_map);
    let mut dm = (*asb).s_map;
    let mut zone = (*asb).s_map_size;
    let mut total = 0;
    loop { total += scan_free_map(asb, dm); dm = dm.add(1); zone -= 1; if zone == 0 { break; } }
    (*buf).f_blocks = adfs_disc_size(dr) >> (*sb).s_blocksize_bits;
    (*buf).f_files = (*asb).s_ids_per_zone * (*asb).s_map_size;
    (*buf).f_bavail = signed_asl(total, (*asb).s_map2blk);
    (*buf).f_bfree = (*buf).f_bavail;
}

pub unsafe fn adfs_map_lookup(sb: *mut super_block, frag_id: u32, offset: u32) -> i32 {
    let asb = ADFS_SB(sb);
    let zone = if frag_id == ADFS_ROOT_FRAG { (*asb).s_map_size >> 1 } else { frag_id / (*asb).s_ids_per_zone };
    if zone >= (*asb).s_map_size { adfs_error(sb, "invalid fragment 0x%04x (zone = %d, max = %d)", frag_id, zone, (*asb).s_map_size); return 0; }
    let mapoff = signed_asl(offset, -(*asb).s_map2blk);
    read_lock(&ADFS_MAP_LOCK);
    let result = scan_map(asb, zone, frag_id, mapoff);
    read_unlock(&ADFS_MAP_LOCK);
    if result > 0 { return offset - signed_asl(mapoff, (*asb).s_map2blk) + signed_asl(result as u32, (*asb).s_map2blk); }
    adfs_error(sb, "fragment 0x%04x at offset %d not found in map", frag_id, offset);
    0
}

unsafe fn adfs_calczonecheck(sb: *mut super_block, map: *const u8) -> u8 {
    let mut v0=0u32; let mut v1=0u32; let mut v2=0u32; let mut v3=0u32;
    let mut i = (*sb).s_blocksize - 4;
    while i != 0 { v0 += *map.add(i as usize) as u32 + (v3 >> 8); v3 &= 0xff; v1 += *map.add((i+1) as usize) as u32 + (v0 >> 8); v0 &= 0xff; v2 += *map.add((i+2) as usize) as u32 + (v1 >> 8); v1 &= 0xff; v3 += *map.add((i+3) as usize) as u32 + (v2 >> 8); v2 &= 0xff; i -= 4; }
    v0 += v3 >> 8; v1 += *map.add(1) as u32 + (v0 >> 8); v2 += *map.add(2) as u32 + (v1 >> 8); v3 += *map.add(3) as u32 + (v2 >> 8); (v0 ^ v1 ^ v2 ^ v3) as u8
}

unsafe fn adfs_checkmap(sb: *mut super_block, dm: *mut adfs_discmap) -> bool {
    let mut crosscheck=0u8; let mut zonecheck=true;
    for i in 0..(*ADFS_SB(sb)).s_map_size { let map=(*dm.add(i as usize)).dm_bh.as_ref().unwrap().b_data; if adfs_calczonecheck(sb,map) != *map { adfs_error(sb,"zone %d fails zonecheck",i); zonecheck=false; } crosscheck ^= *map.add(3); }
    if crosscheck != 0xff { adfs_error(sb,"crosscheck != 0xff"); } crosscheck == 0xff && zonecheck
}

unsafe fn adfs_map_layout(dm:*mut adfs_discmap,nzones:u32,dr:*mut adfs_discrecord) { let zone_size=(8<<(*dr).log2secsize)-le16_to_cpu((*dr).zone_spare); (*dm).dm_bh=None; (*dm).dm_startblk=0; (*dm).dm_startbit=32+ADFS_DR_SIZE_BITS; (*dm).dm_endbit=32+zone_size; for zone in 1..nzones { let d=&mut *dm.add(zone as usize); d.dm_bh=None; d.dm_startblk=zone*zone_size-ADFS_DR_SIZE_BITS; d.dm_startbit=32; d.dm_endbit=32+zone_size; } let size=adfs_disc_size(dr)>>(*dr).log2bpmb; (*dm.add((nzones-1) as usize)).dm_endbit=32+size-(nzones-1)*zone_size+ADFS_DR_SIZE_BITS; }

unsafe fn adfs_map_read(dm:*mut adfs_discmap,sb:*mut super_block,map_addr:u32,nzones:u32)->i32 { for zone in 0..nzones { (*dm.add(zone as usize)).dm_bh=sb_bread(sb,map_addr+zone); if (*dm.add(zone as usize)).dm_bh.is_none(){return -EIO;} } 0 }
unsafe fn adfs_map_relse(dm:*mut adfs_discmap,nzones:u32){for zone in 0..nzones{brelse((*dm.add(zone as usize)).dm_bh);}}

pub unsafe fn adfs_read_map(sb:*mut super_block,dr:*mut adfs_discrecord)->*mut adfs_discmap { let asb=ADFS_SB(sb); let nzones=(*dr).nzones|((*dr).nzones_high<<8); let zone_size=(8<<(*dr).log2secsize)-le16_to_cpu((*dr).zone_spare); (*asb).s_idlen=(*dr).idlen; (*asb).s_map_size=nzones; (*asb).s_map2blk=(*dr).log2bpmb-(*dr).log2secsize; (*asb).s_log2sharesize=(*dr).log2sharesize; (*asb).s_ids_per_zone=zone_size/((*asb).s_idlen+1); let mut map_addr=(nzones>>1)*zone_size-if nzones>1{ADFS_DR_SIZE_BITS}else{0}; map_addr=signed_asl(map_addr,(*asb).s_map2blk); let dm=kmalloc_objs::<adfs_discmap>(nzones); if dm.is_null(){adfs_error(sb,"not enough memory");return ERR_PTR(-ENOMEM);} adfs_map_layout(dm,nzones,dr); if adfs_map_read(dm,sb,map_addr,nzones)!=0 {adfs_error(sb,"unable to read map");adfs_map_relse(dm,nzones);kfree(dm);return ERR_PTR(-ENOMEM);} if adfs_checkmap(sb,dm){return dm;} adfs_error(sb,"map corrupted"); adfs_map_relse(dm,nzones); kfree(dm); ERR_PTR(-EIO) }

pub unsafe fn adfs_free_map(sb:*mut super_block){let asb=ADFS_SB(sb);adfs_map_relse((*asb).s_map,(*asb).s_map_size);kfree((*asb).s_map);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
