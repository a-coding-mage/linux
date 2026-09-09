// SPDX-License-Identifier: GPL-2.0
/*
 * Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
 *                  Volker Sameske <sameske@de.ibm.com>
 * Bugreports.to..: <Linux390@de.ibm.com>
 * Copyright IBM Corp. 1999, 2012
 */

// Linux and architecture headers provide the referenced types, constants,
// globals, and functions in the surrounding translation unit.

#[repr(C)]
pub union label_t {
    pub vol: vtoc_volume_label_cdl,
    pub lnx: vtoc_volume_label_ldl,
    pub cms: vtoc_cms_label,
}

/* compute the block number from a cyl-cyl-head-head structure */
unsafe fn cchh2blk(ptr: *const vtoc_cchh, geo: *const hd_geometry) -> sector_t {
    let mut cyl: sector_t = (*ptr).hh & 0xFFF0;
    cyl <<= 12;
    cyl |= (*ptr).cc;
    let head: u16 = (*ptr).hh & 0x000F;
    cyl * (*geo).heads as sector_t * (*geo).sectors as sector_t
        + head as sector_t * (*geo).sectors as sector_t
}

/* compute the block number from a cyl-cyl-head-head-block structure */
unsafe fn cchhb2blk(ptr: *const vtoc_cchhb, geo: *const hd_geometry) -> sector_t {
    let mut cyl: sector_t = (*ptr).hh & 0xFFF0;
    cyl <<= 12;
    cyl |= (*ptr).cc;
    let head: u16 = (*ptr).hh & 0x000F;
    cyl * (*geo).heads as sector_t * (*geo).sectors as sector_t
        + head as sector_t * (*geo).sectors as sector_t
        + (*ptr).b as sector_t
}

pub const DASD_VOL_TYPE_LEN: usize = 4;
pub const DASD_VOL_ID_LEN: usize = 6;
pub const DASD_VOLLBL_TYPE_VOL1: i32 = 0;
pub const DASD_VOLLBL_TYPE_LNX1: i32 = 1;
pub const DASD_VOLLBL_TYPE_CMS1: i32 = 2;

#[repr(C)]
struct dasd_vollabel {
    type_: *const u8,
    idx: i32,
}

static DASD_VOLLBLS: [dasd_vollabel; 3] = [
    dasd_vollabel { type_: b"VOL1\0".as_ptr(), idx: DASD_VOLLBL_TYPE_VOL1 },
    dasd_vollabel { type_: b"LNX1\0".as_ptr(), idx: DASD_VOLLBL_TYPE_LNX1 },
    dasd_vollabel { type_: b"CMS1\0".as_ptr(), idx: DASD_VOLLBL_TYPE_CMS1 },
];

unsafe fn get_label_by_type(type_: *const u8) -> i32 {
    for label in DASD_VOLLBLS.iter() {
        if memcmp(type_, label.type_, DASD_VOL_TYPE_LEN) == 0 {
            return label.idx;
        }
    }
    -1
}

unsafe fn find_label(
    state: *mut parsed_partitions, info: *mut dasd_information2_t,
    geo: *mut hd_geometry, blocksize: i32, labelsect: *mut sector_t,
    name: *mut u8, type_: *mut u8, label: *mut label_t,
) -> i32 {
    let mut testsect = [0 as sector_t; 3];
    let testcount: i32;
    if !info.is_null() {
        if ((*info).cu_type == 0x6310 && (*info).dev_type == 0x9336)
            || ((*info).cu_type == 0x3880 && (*info).dev_type == 0x3370) {
            testsect[0] = (*info).label_block;
        } else {
            testsect[0] = (*info).label_block * (blocksize >> 9) as sector_t;
        }
        testcount = 1;
    } else {
        testsect = [1, (blocksize >> 9) as sector_t, 2 * (blocksize >> 9) as sector_t];
        testcount = 3;
    }
    for i in 0..testcount as usize {
        let mut sect: Sector = core::mem::zeroed();
        let data = read_part_sector(state, testsect[i], &mut sect);
        if data.is_null() { continue; }
        memcpy(label as *mut u8, data as *const u8, core::mem::size_of::<label_t>());
        memcpy(type_, data as *const u8, DASD_VOL_TYPE_LEN);
        EBCASC(type_, DASD_VOL_TYPE_LEN);
        put_dev_sector(sect);
        match get_label_by_type(type_) {
            DASD_VOLLBL_TYPE_VOL1 => {
                memcpy(name, (*label).vol.volid.as_ptr(), DASD_VOL_ID_LEN);
                EBCASC(name, DASD_VOL_ID_LEN); *labelsect = testsect[i]; return 1;
            }
            DASD_VOLLBL_TYPE_LNX1 | DASD_VOLLBL_TYPE_CMS1 => {
                memcpy(name, (*label).lnx.volid.as_ptr(), DASD_VOL_ID_LEN);
                EBCASC(name, DASD_VOL_ID_LEN); *labelsect = testsect[i]; return 1;
            }
            _ => {}
        }
    }
    0
}

// The remaining partition-format routines retain the original C control flow
// and call the surrounding kernel interfaces.
unsafe fn find_vol1_partitions(state: *mut parsed_partitions, geo: *mut hd_geometry, blocksize: i32, name: *const u8, label: *mut label_t) -> i32 {
    let secperblk = blocksize >> 9;
    seq_buf_printf(&mut (*state).pp_buf, b"VOL1/%8s:\0".as_ptr(), name);
    let mut blk = cchhb2blk(&(*label).vol.vtoc, geo) + 1;
    let mut counter = 0;
    let mut sect: Sector = core::mem::zeroed();
    let mut data = read_part_sector(state, blk * secperblk as sector_t, &mut sect);
    while !data.is_null() {
        let mut f1: vtoc_format1_label = core::mem::zeroed();
        memcpy(&mut f1 as *mut _ as *mut u8, data as *const u8, core::mem::size_of::<vtoc_format1_label>());
        put_dev_sector(sect);
        if f1.DS1FMTID == _ascebc['4' as usize] || f1.DS1FMTID == _ascebc['5' as usize] || f1.DS1FMTID == _ascebc['7' as usize] || f1.DS1FMTID == _ascebc['9' as usize] { blk += 1; data = read_part_sector(state, blk * secperblk as sector_t, &mut sect); continue; }
        if f1.DS1FMTID != _ascebc['1' as usize] && f1.DS1FMTID != _ascebc['8' as usize] { break; }
        let offset = cchh2blk(&f1.DS1EXT1.llimit, geo);
        let size = cchh2blk(&f1.DS1EXT1.ulimit, geo) - offset + (*geo).sectors as sector_t;
        if counter >= (*state).limit { break; }
        put_partition(state, counter + 1, offset * secperblk as sector_t, size * secperblk as sector_t);
        counter += 1; blk += 1; data = read_part_sector(state, blk * secperblk as sector_t, &mut sect);
    }
    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr());
    if data.is_null() { -1 } else { 1 }
}

unsafe fn find_lnx1_partitions(state: *mut parsed_partitions, geo: *mut hd_geometry, blocksize: i32, name: *const u8, label: *mut label_t, labelsect: sector_t, nr_sectors: sector_t, info: *mut dasd_information2_t) -> i32 {
    let secperblk = blocksize >> 9;
    seq_buf_printf(&mut (*state).pp_buf, b"LNX1/%8s:\0".as_ptr(), name);
    let mut size: loff_t;
    if (*label).lnx.ldl_version == 0xf2 { size = (*label).lnx.formatted_blocks * secperblk as _; }
    else {
        let geo_size = (*geo).cylinders as loff_t * (*geo).heads as loff_t * (*geo).sectors as loff_t * secperblk as loff_t;
        size = nr_sectors as loff_t;
        if size != geo_size {
            if info.is_null() { seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr()); return 1; }
            if strcmp((*info).type.as_ptr(), b"ECKD\0".as_ptr()) == 0 && geo_size < size { size = geo_size; }
        }
    }
    let offset = labelsect + secperblk as sector_t;
    put_partition(state, 1, offset, size as sector_t - offset);
    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr()); 1
}

unsafe fn find_cms1_partitions(state: *mut parsed_partitions, _geo: *mut hd_geometry, _blocksize: i32, name: *const u8, label: *mut label_t, labelsect: sector_t) -> i32 {
    let blocksize = (*label).cms.block_size;
    let secperblk = blocksize >> 9;
    let (offset, size);
    if (*label).cms.disk_offset != 0 {
        seq_buf_printf(&mut (*state).pp_buf, b"CMS1/%8s(MDSK):\0".as_ptr(), name);
        offset = (*label).cms.disk_offset * secperblk as sector_t;
        size = ((*label).cms.block_count - 1) * secperblk as sector_t;
    } else {
        seq_buf_printf(&mut (*state).pp_buf, b"CMS1/%8s:\0".as_ptr(), name);
        offset = if labelsect == 1 { 2 * secperblk as sector_t } else { labelsect + secperblk as sector_t };
        size = (*label).cms.block_count * secperblk as sector_t;
    }
    put_partition(state, 1, offset, size - offset);
    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr()); 1
}

pub unsafe fn ibm_partition(state: *mut parsed_partitions) -> i32 {
    let disk = (*state).disk;
    let bdev = (*disk).part0;
    if (*(*disk).fops).getgeo.is_none() { return 0; }
    let blocksize = bdev_logical_block_size(bdev);
    if blocksize <= 0 { return 0; }
    let nr_sectors = bdev_nr_sectors(bdev);
    if nr_sectors == 0 { return 0; }
    let mut info: *mut dasd_information2_t = kmalloc_obj();
    let geo: *mut hd_geometry = kmalloc_obj();
    let label: *mut label_t = kmalloc_obj();
    if info.is_null() || geo.is_null() || label.is_null() { if !label.is_null(){kfree(label as *mut _);} if !geo.is_null(){kfree(geo as *mut _);} if !info.is_null(){kfree(info as *mut _);} return 0; }
    (*geo).start = get_start_sect(bdev);
    if ((*(*disk).fops).getgeo.unwrap())(disk, geo) != 0 { kfree(label as *mut _); kfree(geo as *mut _); kfree(info as *mut _); return 0; }
    let mut name = [0u8; DASD_VOL_ID_LEN + 1]; let mut type_ = [0u8; DASD_VOL_TYPE_LEN + 1]; let mut labelsect = 0;
    let mut res = 0;
    if find_label(state, info, geo, blocksize, &mut labelsect, name.as_mut_ptr(), type_.as_mut_ptr(), label) != 0 {
        match get_label_by_type(type_.as_ptr()) { 0 => res=find_vol1_partitions(state,geo,blocksize,name.as_ptr(),label), 1 => res=find_lnx1_partitions(state,geo,blocksize,name.as_ptr(),label,labelsect,nr_sectors,info), 2 => res=find_cms1_partitions(state,geo,blocksize,name.as_ptr(),label,labelsect), _=>{} }
    } else if !info.is_null() { res=1; if (*info).format == DASD_FORMAT_LDL { seq_buf_puts(&mut (*state).pp_buf,b"(nonl)\0".as_ptr()); let offset=((*info).label_block+1)*(blocksize>>9) as sector_t; put_partition(state,1,offset,nr_sectors-offset); seq_buf_puts(&mut (*state).pp_buf,b"\n\0".as_ptr()); } }
    kfree(label as *mut _); kfree(geo as *mut _); kfree(info as *mut _); res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
