// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/wrapper.c
 *
 * Handling of HFS wrappers around HFS+ volumes
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct hfsplus_wd {
    pub ablk_size: u32,
    pub ablk_start: u16,
    pub embed_start: u16,
    pub embed_count: u16,
}

extern "C" {
    fn hfsplus_min_io_size(sb: *mut super_block) -> u64;
    fn bdev_rw_virt(bdev: *mut block_device, sector: u64, buf: *mut core::ffi::c_void,
                    io_size: u64, opf: u32) -> i32;
    fn sb_min_blocksize(sb: *mut super_block, size: u32) -> u32;
    fn kmalloc(size: u64, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn hfs_part_find(sb: *mut super_block, start: *mut u64, size: *mut u64) -> i32;
    fn set_bit(bit: u32, addr: *mut u64);
    fn ilog2(value: u32) -> u32;
    fn sb_set_blocksize(sb: *mut super_block, size: u32) -> u32;
}

#[repr(C)] pub struct block_device { pub bd_disk: *mut core::ffi::c_void }
#[repr(C)] pub struct super_block {
    pub s_bdev: *mut block_device,
    pub s_blocksize_bits: u32,
    pub s_fs_info: *mut hfsplus_sb_info,
}
#[repr(C)] pub struct hfsplus_sb_info {
    pub session: i32,
    pub min_io_size: u32,
    pub s_vhdr_buf: *mut core::ffi::c_void,
    pub s_backup_vhdr_buf: *mut core::ffi::c_void,
    pub s_vhdr: *mut hfsplus_vol_header,
    pub s_backup_vhdr: *mut hfsplus_vol_header,
    pub flags: u64,
    pub alloc_blksz: u32,
    pub alloc_blksz_shift: u32,
    pub blockoffset: u64,
    pub part_start: u64,
    pub sect_count: u64,
    pub fs_shift: u32,
}
#[repr(C)] pub struct hfsplus_vol_header { pub signature: u16, pub blocksize: u32 }

const HFSPLUS_SECTOR_SHIFT: u32 = 9;
const HFSPLUS_SECTOR_SIZE: u32 = 1 << HFSPLUS_SECTOR_SHIFT;
const HFSPLUS_VOLHEAD_SIG: u16 = 0x482b;
const HFSPLUS_VOLHEAD_SIGX: u16 = 0x4858;
const HFSP_WRAP_MAGIC: u16 = 0x4244;
const HFSPLUS_SB_HFSX: u32 = 0;
const REQ_OP_MASK: u32 = 0xff;
const REQ_OP_WRITE: u32 = 1;
const REQ_OP_READ: u32 = 0;
const GFP_KERNEL: u32 = 0;
const PAGE_SIZE: u32 = 4096;
const HFSP_WRAPOFF_EMBEDSIG: usize = 0;
const HFSP_WRAPOFF_ATTRIB: usize = 4;
const HFSP_WRAPOFF_ABLKSIZE: usize = 20;
const HFSP_WRAPOFF_ABLKSTART: usize = 28;
const HFSP_WRAPOFF_EMBEDEXT: usize = 16;
const HFSP_WRAP_ATTRIB_SLOCK: u16 = 0x8000;
const HFSP_WRAP_ATTRIB_SPARED: u16 = 0x4000;
const HFSPLUS_VOLHEAD_SECTOR: u64 = 2;

#[inline]
unsafe fn be16(p: *const u8) -> u16 { u16::from_be((p as *const u16).read_unaligned()) }
#[inline]
unsafe fn be32(p: *const u8) -> u32 { u32::from_be((p as *const u32).read_unaligned()) }

pub unsafe fn hfsplus_submit_bio(sb: *mut super_block, mut sector: u64,
    buf: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void, opf: u32) -> i32 {
    let io_size = hfsplus_min_io_size(sb);
    let start = (sector << HFSPLUS_SECTOR_SHIFT) as i64;
    let offset = (start as u64 & (io_size - 1)) as usize;
    if (opf & REQ_OP_MASK) != REQ_OP_WRITE && !data.is_null() {
        *data = (buf as *mut u8).add(offset) as *mut core::ffi::c_void;
    }
    sector &= !((io_size >> HFSPLUS_SECTOR_SHIFT) - 1);
    bdev_rw_virt((*sb).s_bdev, sector, buf, io_size, opf)
}

unsafe fn hfsplus_get_last_session(sb: *mut super_block, start: *mut u64, size: *mut u64) -> i32 {
    *start = 0;
    *size = bdev_nr_sectors((*sb).s_bdev);
    // CD-ROM session handling is supplied by the kernel's cdrom interfaces.
    if (*(*sb).s_fs_info).session >= 0 {
        let cdi = disk_to_cdi((*sb).s_bdev);
        if cdi.is_null() { return -22; }
        let mut te = cdrom_tocentry { cdte_track: (*(*sb).s_fs_info).session as u8,
            cdte_format: CDROM_LBA, cdte_ctrl: 0, cdte_addr_lba: 0 };
        if cdrom_read_tocentry(cdi, &mut te) != 0 || te.cdte_ctrl & CDROM_DATA_TRACK != 4 { return -22; }
        *start = te.cdte_addr_lba as u64 << 2;
    } else {
        let cdi = disk_to_cdi((*sb).s_bdev);
        if !cdi.is_null() {
            let mut ms = cdrom_multisession { addr_format: CDROM_LBA, xa_flag: 0, addr_lba: 0 };
            if cdrom_multisession(cdi, &mut ms) == 0 && ms.xa_flag != 0 { *start = ms.addr_lba as u64 << 2; }
        }
    }
    0
}

extern "C" {
    fn bdev_nr_sectors(bdev: *mut block_device) -> u64;
    fn disk_to_cdi(bdev: *mut block_device) -> *mut cdrom_device_info;
    fn cdrom_read_tocentry(cdi: *mut cdrom_device_info, te: *mut cdrom_tocentry) -> i32;
    fn cdrom_multisession(cdi: *mut cdrom_device_info, ms: *mut cdrom_multisession) -> i32;
}
#[repr(C)] pub struct cdrom_device_info;
#[repr(C)] pub struct cdrom_tocentry { cdte_track: u8, cdte_format: u8, cdte_ctrl: u8, cdte_addr_lba: u32 }
#[repr(C)] pub struct cdrom_multisession { addr_format: u8, xa_flag: u8, addr_lba: u32 }
const CDROM_LBA: u8 = 1;
const CDROM_DATA_TRACK: u8 = 4;

unsafe fn hfsplus_read_mdb(bufptr: *mut u8, wd: *mut hfsplus_wd) -> i32 {
    let sig = be16(bufptr.add(HFSP_WRAPOFF_EMBEDSIG));
    if sig != HFSPLUS_VOLHEAD_SIG && sig != HFSPLUS_VOLHEAD_SIGX { return 0; }
    let attrib = be16(bufptr.add(HFSP_WRAPOFF_ATTRIB));
    if attrib & HFSP_WRAP_ATTRIB_SLOCK == 0 || attrib & HFSP_WRAP_ATTRIB_SPARED == 0 { return 0; }
    (*wd).ablk_size = be32(bufptr.add(HFSP_WRAPOFF_ABLKSIZE));
    if (*wd).ablk_size < HFSPLUS_SECTOR_SIZE || (*wd).ablk_size % HFSPLUS_SECTOR_SIZE != 0 { return 0; }
    (*wd).ablk_start = be16(bufptr.add(HFSP_WRAPOFF_ABLKSTART));
    let extent = be32(bufptr.add(HFSP_WRAPOFF_EMBEDEXT));
    (*wd).embed_start = (extent >> 16) as u16;
    (*wd).embed_count = extent as u16;
    1
}

pub unsafe fn hfsplus_read_wrapper(sb: *mut super_block) -> i32 {
    let sbi = (*sb).s_fs_info;
    let mut wd = hfsplus_wd { ablk_size: 0, ablk_start: 0, embed_start: 0, embed_count: 0 };
    let (mut part_start, mut part_size): (u64, u64) = (0, 0);
    let mut blocksize: u32;
    let mut error: i32;
    error = -22;
    blocksize = sb_min_blocksize(sb, HFSPLUS_SECTOR_SIZE);
    if blocksize == 0 { return error; }
    (*sbi).min_io_size = blocksize;
    if hfsplus_get_last_session(sb, &mut part_start, &mut part_size) != 0 { return error; }
    error = -12;
    (*sbi).s_vhdr_buf = kmalloc(hfsplus_min_io_size(sb), GFP_KERNEL);
    if (*sbi).s_vhdr_buf.is_null() { return error; }
    (*sbi).s_backup_vhdr_buf = kmalloc(hfsplus_min_io_size(sb), GFP_KERNEL);
    if (*sbi).s_backup_vhdr_buf.is_null() { kfree((*sbi).s_vhdr_buf); return error; }
    loop {
        error = hfsplus_submit_bio(sb, part_start + HFSPLUS_VOLHEAD_SECTOR, (*sbi).s_vhdr_buf,
            &mut (*sbi).s_vhdr as *mut _ as *mut _, REQ_OP_READ);
        if error != 0 { break; }
        error = -22;
        match (*sbi).s_vhdr.as_ref().unwrap().signature {
            HFSPLUS_VOLHEAD_SIGX => { set_bit(HFSPLUS_SB_HFSX, &mut (*sbi).flags); }
            HFSPLUS_VOLHEAD_SIG => {}
            HFSP_WRAP_MAGIC => {
                if hfsplus_read_mdb((*sbi).s_vhdr as *mut _ as *mut u8, &mut wd) == 0 { break; }
                wd.ablk_size >>= HFSPLUS_SECTOR_SHIFT;
                part_start += wd.ablk_start as u64 + wd.embed_start as u64 * wd.ablk_size as u64;
                part_size = wd.embed_count as u64 * wd.ablk_size as u64;
                continue;
            }
            _ => { if hfs_part_find(sb, &mut part_start, &mut part_size) != 0 { break; } continue; }
        }
        error = hfsplus_submit_bio(sb, part_start + part_size - 2, (*sbi).s_backup_vhdr_buf,
            &mut (*sbi).s_backup_vhdr as *mut _ as *mut _, REQ_OP_READ);
        if error != 0 { break; }
        error = -22;
        if (*sbi).s_backup_vhdr.as_ref().unwrap().signature != (*sbi).s_vhdr.as_ref().unwrap().signature { break; }
        blocksize = u32::from_be((*sbi).s_vhdr.as_ref().unwrap().blocksize);
        if blocksize < HFSPLUS_SECTOR_SIZE || ((blocksize - 1) & blocksize) != 0 { break; }
        (*sbi).alloc_blksz = blocksize;
        (*sbi).alloc_blksz_shift = ilog2(blocksize);
        blocksize = core::cmp::min((*sbi).alloc_blksz, PAGE_SIZE);
        while part_start & ((blocksize >> HFSPLUS_SECTOR_SHIFT) as u64 - 1) != 0 { blocksize >>= 1; }
        if sb_set_blocksize(sb, blocksize) != blocksize { break; }
        (*sbi).blockoffset = part_start >> ((*sb).s_blocksize_bits - HFSPLUS_SECTOR_SHIFT);
        (*sbi).part_start = part_start; (*sbi).sect_count = part_size;
        (*sbi).fs_shift = (*sbi).alloc_blksz_shift - (*sb).s_blocksize_bits;
        return 0;
    }
    kfree((*sbi).s_backup_vhdr_buf); kfree((*sbi).s_vhdr_buf); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
