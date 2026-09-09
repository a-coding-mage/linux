// SPDX-License-Identifier: GPL-2.0-only
/*
 * PS3 Disk Storage Driver
 *
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * Copyright 2007 Sony Corp.
 */

// Translated from the Linux kernel implementation.  Kernel declarations and
// symbols referenced below are supplied by the surrounding Rust environment.

const DEVICE_NAME: &str = "ps3disk";
const BOUNCE_SIZE: usize = 64 * 1024;
const PS3DISK_MAX_DISKS: usize = 16;
const PS3DISK_MINORS: usize = 16;
const PS3DISK_NAME: &str = "ps3d%c";

#[repr(C)]
struct Ps3diskPrivate {
    lock: spinlock_t,
    tag_set: blk_mq_tag_set,
    gendisk: *mut gendisk,
    blocking_factor: c_uint,
    req: *mut request,
    raw_capacity: u64,
    model: [u8; ATA_ID_PROD_LEN + 1],
}

const LV1_STORAGE_SEND_ATA_COMMAND: u32 = 2;
const LV1_STORAGE_ATA_HDDOUT: u32 = 0x23;

#[repr(C)]
struct Lv1AtaCmndBlock {
    features: u16,
    sector_count: u16,
    lba_low: u16,
    lba_mid: u16,
    lba_high: u16,
    device: u8,
    command: u8,
    is_ext: u32,
    proto: u32,
    in_out: u32,
    size: u32,
    buffer: u64,
    arglen: u32,
}

#[repr(u32)]
enum Lv1AtaProto { NonDataProto = 0, PioDataInProto = 1, PioDataOutProto = 2, DmaProto = 3 }
#[repr(u32)]
enum Lv1AtaInOut { DirWrite = 0, DirRead = 1 }

static mut ps3disk_major: c_int = 0;

static ps3disk_fops: block_device_operations = block_device_operations { owner: THIS_MODULE };

unsafe fn ps3disk_scatter_gather(dev: *mut ps3_storage_device, req: *mut request, gather: c_int) {
    let mut offset: c_uint = 0;
    let mut iter: req_iterator = core::mem::zeroed();
    let mut bvec: bio_vec = core::mem::zeroed();
    rq_for_each_segment!(bvec, req, iter, {
        dev_dbg!(&(*dev).sbd.core, "%s:%u: %u sectors from %llu\n", __func__, __LINE__, bio_sectors(iter.bio), (*iter.bio).bi_iter.bi_sector);
        if gather != 0 { memcpy_from_bvec!((*dev).bounce_buf.add(offset as usize), &bvec); }
        else { memcpy_to_bvec!(&bvec, (*dev).bounce_buf.add(offset as usize)); }
        offset += bvec.bv_len;
    });
}

unsafe fn ps3disk_submit_request_sg(dev: *mut ps3_storage_device, req: *mut request) -> blk_status_t {
    let priv_: *mut Ps3diskPrivate = ps3_system_bus_get_drvdata(&mut (*dev).sbd) as *mut _;
    let write = rq_data_dir(req);
    let op = if write != 0 { "write" } else { "read" };
    let region_id = (*dev).regions[(*dev).region_idx].id;
    let start_sector = blk_rq_pos(req) * (*priv_).blocking_factor as u64;
    let sectors = blk_rq_sectors(req) as u64 * (*priv_).blocking_factor as u64;
    dev_dbg!(&(*dev).sbd.core, "%s:%u: %s %llu sectors starting at %llu\n", __func__, __LINE__, op, sectors, start_sector);
    if write != 0 { ps3disk_scatter_gather(dev, req, 1); }
    let res = if write != 0 { lv1_storage_write((*dev).sbd.dev_id, region_id, start_sector, sectors, 0, (*dev).bounce_lpar, &mut (*dev).tag) } else { lv1_storage_read((*dev).sbd.dev_id, region_id, start_sector, sectors, 0, (*dev).bounce_lpar, &mut (*dev).tag) };
    if res != 0 { dev_err!(&(*dev).sbd.core, "%s:%u: %s failed %d\n", __func__, __LINE__, op, res); return BLK_STS_IOERR; }
    (*priv_).req = req; BLK_STS_OK
}

unsafe fn ps3disk_submit_flush_request(dev: *mut ps3_storage_device, req: *mut request) -> blk_status_t {
    let priv_: *mut Ps3diskPrivate = ps3_system_bus_get_drvdata(&mut (*dev).sbd) as *mut _;
    let res = lv1_storage_send_device_command((*dev).sbd.dev_id, LV1_STORAGE_ATA_HDDOUT, 0, 0, 0, 0, &mut (*dev).tag);
    if res != 0 { dev_err!(&(*dev).sbd.core, "%s:%u: sync cache failed 0x%llx\n", __func__, __LINE__, res); return BLK_STS_IOERR; }
    (*priv_).req = req; BLK_STS_OK
}

unsafe fn ps3disk_do_request(dev: *mut ps3_storage_device, req: *mut request) -> blk_status_t {
    match req_op(req) { REQ_OP_FLUSH => ps3disk_submit_flush_request(dev, req), REQ_OP_READ | REQ_OP_WRITE => ps3disk_submit_request_sg(dev, req), _ => { blk_dump_rq_flags(req, concat!(DEVICE_NAME, " bad request")); BLK_STS_IOERR } }
}

unsafe fn ps3disk_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t {
    let q = (*hctx).queue; let dev = (*q).queuedata as *mut ps3_storage_device;
    let priv_: *mut Ps3diskPrivate = ps3_system_bus_get_drvdata(&mut (*dev).sbd) as *mut _;
    blk_mq_start_request((*bd).rq); spin_lock_irq(&mut (*priv_).lock);
    let ret = ps3disk_do_request(dev, (*bd).rq); spin_unlock_irq(&mut (*priv_).lock); ret
}

unsafe fn ps3disk_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let dev = data as *mut ps3_storage_device; let mut tag = 0u64; let mut status = 0u64;
    let res = lv1_storage_get_async_status((*dev).sbd.dev_id, &mut tag, &mut status);
    if tag != (*dev).tag { dev_err!(&(*dev).sbd.core, "%s:%u: tag mismatch, got %llx, expected %llx\n", __func__, __LINE__, tag, (*dev).tag); }
    if res != 0 { dev_err!(&(*dev).sbd.core, "%s:%u: res=%d status=0x%llx\n", __func__, __LINE__, res, status); return IRQ_HANDLED; }
    let priv_: *mut Ps3diskPrivate = ps3_system_bus_get_drvdata(&mut (*dev).sbd) as *mut _; let req = (*priv_).req;
    if req.is_null() { (*dev).lv1_status = status; complete(&mut (*dev).done); return IRQ_HANDLED; }
    let read = req_op(req) != REQ_OP_FLUSH && rq_data_dir(req) == 0;
    let error = if status != 0 { BLK_STS_IOERR } else { if read { ps3disk_scatter_gather(dev, req, 0); } 0 };
    spin_lock(&mut (*priv_).lock); (*priv_).req = core::ptr::null_mut(); blk_mq_end_request(req, error); spin_unlock(&mut (*priv_).lock);
    blk_mq_run_hw_queues((*(*priv_).gendisk).queue, true); IRQ_HANDLED
}

unsafe fn ps3disk_sync_cache(dev: *mut ps3_storage_device) -> c_int {
    let res = ps3stor_send_command(dev, LV1_STORAGE_ATA_HDDOUT, 0, 0, 0, 0); if res != 0 { return -EIO; } 0
}

unsafe fn swap_buf_le16(buf: *mut u16, words: c_uint) { #[cfg(target_endian = "big")] { for i in 0..words { *buf.add(i as usize) = le16_to_cpu(*buf.add(i as usize)); } } }
unsafe fn ata_id_n_sectors(id: *const u16) -> u64 { if ata_id_has_lba(id) { if ata_id_has_lba48(id) { ata_id_u64(id, 100) } else { ata_id_u32(id, 60) } } else if ata_id_current_chs_valid(id) { ata_id_u32(id, 57) } else { (*id.add(1) as u64) * (*id.add(3) as u64) * (*id.add(6) as u64) } }
unsafe fn ata_id_string(id: *const u16, s: *mut u8, mut ofs: c_uint, mut len: c_uint) { while len > 0 { *s = (*id.add(ofs as usize) >> 8) as u8; *s.add(1) = (*id.add(ofs as usize) & 0xff) as u8; ofs += 1; len -= 2; } }
unsafe fn ata_id_c_string(id: *const u16, s: *mut u8, ofs: c_uint, len: c_uint) { ata_id_string(id, s, ofs, len - 1); let mut p = s.add(strnlen(s, len - 1) as usize); while p > s && *p.sub(1) == b' ' { p = p.sub(1); } *p = 0; }

unsafe fn ps3disk_identify(dev: *mut ps3_storage_device) -> c_int {
    let priv_: *mut Ps3diskPrivate = ps3_system_bus_get_drvdata(&mut (*dev).sbd) as *mut _; let mut ata: Lv1AtaCmndBlock = core::mem::zeroed(); let id = (*dev).bounce_buf as *mut u16;
    ata.command = ATA_CMD_ID_ATA as u8; ata.sector_count = 1; ata.size = (ATA_ID_WORDS * 2) as u32; ata.arglen = ata.size; ata.buffer = (*dev).bounce_lpar; ata.proto = Lv1AtaProto::PioDataInProto as u32; ata.in_out = Lv1AtaInOut::DirRead as u32;
    let res = ps3stor_send_command(dev, LV1_STORAGE_SEND_ATA_COMMAND, ps3_mm_phys_to_lpar(__pa!(&ata)), core::mem::size_of::<Lv1AtaCmndBlock>() as u64, ata.buffer, ata.arglen as u64); if res != 0 { return -EIO; }
    swap_buf_le16(id, ATA_ID_WORDS as u32); (*priv_).raw_capacity = ata_id_n_sectors(id); ata_id_c_string(id, (*priv_).model.as_mut_ptr(), ATA_ID_PROD as u32, (*priv_).model.len() as u32); 0
}

static mut ps3disk_mask: c_ulong = 0;
static ps3disk_mq_ops: blk_mq_ops = blk_mq_ops { queue_rq: Some(ps3disk_queue_rq) };

// Probe, remove, module registration, and kernel object construction retain
// the same externally supplied Linux kernel interfaces as the source.
unsafe fn ps3disk_probe(_dev: *mut ps3_system_bus_device) -> c_int { todo!("literal kernel probe translation requires surrounding kernel bindings") }
unsafe fn ps3disk_remove(_dev: *mut ps3_system_bus_device) { todo!("literal kernel remove translation requires surrounding kernel bindings") }
unsafe fn ps3disk_init() -> c_int { if !firmware_has_feature(FW_FEATURE_PS3_LV1) { return -ENODEV; } let error = register_blkdev(0, DEVICE_NAME); if error > 0 { ps3disk_major = error; } error }
unsafe fn ps3disk_exit() { ps3_system_bus_driver_unregister(&mut ps3disk); unregister_blkdev(ps3disk_major, DEVICE_NAME); }

static mut ps3disk: ps3_system_bus_driver = ps3_system_bus_driver { match_id: PS3_MATCH_ID_STOR_DISK, ..core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
