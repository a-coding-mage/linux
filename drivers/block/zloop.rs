// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025, Christoph Hellwig.
 * Copyright (c) 2025, Western Digital Corporation or its affiliates.
 *
 * Zoned Loop Device driver - exports a zoned block device using one file per
 * zone as backing storage.
 */
// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt

const ZLOOP_OPT_ERR: u32 = 0;
const ZLOOP_OPT_ID: u32 = 1 << 0;
const ZLOOP_OPT_CAPACITY: u32 = 1 << 1;
const ZLOOP_OPT_ZONE_SIZE: u32 = 1 << 2;
const ZLOOP_OPT_ZONE_CAPACITY: u32 = 1 << 3;
const ZLOOP_OPT_NR_CONV_ZONES: u32 = 1 << 4;
const ZLOOP_OPT_BASE_DIR: u32 = 1 << 5;
const ZLOOP_OPT_NR_QUEUES: u32 = 1 << 6;
const ZLOOP_OPT_QUEUE_DEPTH: u32 = 1 << 7;
const ZLOOP_OPT_BUFFERED_IO: u32 = 1 << 8;
const ZLOOP_OPT_ZONE_APPEND: u32 = 1 << 9;
const ZLOOP_OPT_ORDERED_ZONE_APPEND: u32 = 1 << 10;
const ZLOOP_OPT_DISCARD_WRITE_CACHE: u32 = 1 << 11;
const ZLOOP_OPT_MAX_OPEN_ZONES: u32 = 1 << 12;

const ZLOOP_DEF_ID: i32 = -1;
const ZLOOP_DEF_ZONE_SIZE: u64 = (256u64 * SZ_1M) >> SECTOR_SHIFT;
const ZLOOP_DEF_NR_ZONES: u32 = 64;
const ZLOOP_DEF_NR_CONV_ZONES: u32 = 8;
const ZLOOP_DEF_MAX_OPEN_ZONES: u32 = 0;
const ZLOOP_DEF_BASE_DIR: &str = "/var/local/zloop";
const ZLOOP_DEF_NR_QUEUES: u32 = 1;
const ZLOOP_DEF_QUEUE_DEPTH: u32 = 128;
const ZLOOP_DEF_BUFFERED_IO: bool = false;
const ZLOOP_DEF_ZONE_APPEND: bool = true;
const ZLOOP_DEF_ORDERED_ZONE_APPEND: bool = false;
const ZLOOP_MAX_ZONE_SIZE_MB: u32 = 16384;

#[repr(C)]
pub struct zloop_options {
    pub mask: u32,
    pub id: i32,
    pub capacity: sector_t,
    pub zone_size: sector_t,
    pub zone_capacity: sector_t,
    pub nr_conv_zones: u32,
    pub max_open_zones: u32,
    pub base_dir: *mut c_char,
    pub nr_queues: u32,
    pub queue_depth: u32,
    pub buffered_io: bool,
    pub zone_append: bool,
    pub ordered_zone_append: bool,
    pub discard_write_cache: bool,
}

#[repr(u32)]
enum zloop_state { Zlo_creating = 0, Zlo_live, Zlo_deleting }
#[repr(u32)]
enum zloop_zone_flags { ZLOOP_ZONE_CONV = 0, ZLOOP_ZONE_SEQ_ERROR }

#[repr(C)]
pub struct zloop_zone {
    pub open_zone_entry: list_head,
    pub file: *mut file,
    pub flags: c_ulong,
    pub lock: mutex,
    pub wp_lock: spinlock_t,
    pub cond: blk_zone_cond,
    pub start: sector_t,
    pub wp: sector_t,
    pub old_gfp_mask: gfp_t,
}

#[repr(C)]
pub struct zloop_device {
    pub id: u32, pub state: u32, pub tag_set: blk_mq_tag_set, pub disk: *mut gendisk,
    pub workqueue: *mut workqueue_struct, pub buffered_io: bool, pub zone_append: bool,
    pub ordered_zone_append: bool, pub discard_write_cache: bool, pub base_dir: *const c_char,
    pub data_dir: *mut file, pub zone_shift: u32, pub zone_size: sector_t,
    pub zone_capacity: sector_t, pub nr_zones: u32, pub nr_conv_zones: u32,
    pub max_open_zones: u32, pub block_size: u32, pub dio_mem_align: u32,
    pub open_zones_lock: spinlock_t, pub open_zones_lru_list: list_head,
    pub nr_open_zones: u32, pub zones: [zloop_zone; 0],
}

#[repr(C)]
pub struct zloop_cmd {
    pub work: work_struct, pub ref_: atomic_t, pub sector: sector_t,
    pub nr_sectors: sector_t, pub ret: c_long, pub iocb: kiocb, pub bvec: *mut bio_vec,
}

static mut zloop_index_idr: idr = DEFINE_IDR!(zloop_index_idr);
static mut zloop_ctl_mutex: mutex = DEFINE_MUTEX!(zloop_ctl_mutex);

unsafe fn rq_zone_no(rq: *mut request) -> u32 {
    let zlo = (*(*rq).q).queuedata as *mut zloop_device;
    blk_rq_pos(rq) >> (*zlo).zone_shift
}

unsafe fn zloop_lru_rotate_open_zone(zlo: *mut zloop_device, zone: *mut zloop_zone) {
    if (*zlo).max_open_zones != 0 { spin_lock(&mut (*zlo).open_zones_lock); list_move_tail(&mut (*zone).open_zone_entry, &mut (*zlo).open_zones_lru_list); spin_unlock(&mut (*zlo).open_zones_lock); }
}
unsafe fn zloop_lru_remove_open_zone(zlo: *mut zloop_device, zone: *mut zloop_zone) {
    if (*zone).cond == BLK_ZONE_COND_IMP_OPEN || (*zone).cond == BLK_ZONE_COND_EXP_OPEN { spin_lock(&mut (*zlo).open_zones_lock); list_del_init(&mut (*zone).open_zone_entry); (*zlo).nr_open_zones -= 1; spin_unlock(&mut (*zlo).open_zones_lock); }
}
unsafe fn zloop_can_open_zone(zlo: *mut zloop_device) -> bool { (*zlo).max_open_zones == 0 || (*zlo).nr_open_zones < (*zlo).max_open_zones }
unsafe fn zloop_close_imp_open_zone(zlo: *mut zloop_device) -> bool {
    lockdep_assert_held(&(*zlo).open_zones_lock);
    if zloop_can_open_zone(zlo) { return true; }
    let mut zone: *mut zloop_zone;
    list_for_each_entry!(zone, &mut (*zlo).open_zones_lru_list, open_zone_entry) {
        if (*zone).cond == BLK_ZONE_COND_IMP_OPEN { (*zone).cond = BLK_ZONE_COND_CLOSED; list_del_init(&mut (*zone).open_zone_entry); (*zlo).nr_open_zones -= 1; return true; }
    }
    false
}
unsafe fn zloop_open_closed_or_empty_zone(zlo: *mut zloop_device, zone: *mut zloop_zone, explicit: bool) -> bool {
    spin_lock(&mut (*zlo).open_zones_lock);
    if explicit { if !zloop_can_open_zone(zlo) { spin_unlock(&mut (*zlo).open_zones_lock); return false; } (*zone).cond = BLK_ZONE_COND_EXP_OPEN; }
    else { if !zloop_close_imp_open_zone(zlo) { spin_unlock(&mut (*zlo).open_zones_lock); return false; } (*zone).cond = BLK_ZONE_COND_IMP_OPEN; }
    (*zlo).nr_open_zones += 1; list_add_tail(&mut (*zone).open_zone_entry, &mut (*zlo).open_zones_lru_list); spin_unlock(&mut (*zlo).open_zones_lock); true
}
unsafe fn zloop_do_open_zone(zlo: *mut zloop_device, zone: *mut zloop_zone, explicit: bool) -> bool {
    match (*zone).cond { BLK_ZONE_COND_IMP_OPEN | BLK_ZONE_COND_EXP_OPEN => { if explicit { (*zone).cond = BLK_ZONE_COND_EXP_OPEN; } zloop_lru_rotate_open_zone(zlo, zone); true }, BLK_ZONE_COND_EMPTY | BLK_ZONE_COND_CLOSED => zloop_open_closed_or_empty_zone(zlo, zone, explicit), _ => false }
}
unsafe fn zloop_mark_full(zlo: *mut zloop_device, zone: *mut zloop_zone) { lockdep_assert_held(&(*zone).wp_lock); zloop_lru_remove_open_zone(zlo, zone); (*zone).cond = BLK_ZONE_COND_FULL; (*zone).wp = ULLONG_MAX; }
unsafe fn zloop_mark_empty(zlo: *mut zloop_device, zone: *mut zloop_zone) { lockdep_assert_held(&(*zone).wp_lock); zloop_lru_remove_open_zone(zlo, zone); (*zone).cond = BLK_ZONE_COND_EMPTY; (*zone).wp = (*zone).start; }

unsafe fn zloop_update_seq_zone(zlo: *mut zloop_device, zone_no: u32) -> c_int {
    let zone = (*zlo).zones.as_mut_ptr().add(zone_no as usize); let mut stat: kstat = core::mem::zeroed(); let ret = vfs_getattr(&(*(*zone).file).f_path, &mut stat, STATX_SIZE, 0); if ret < 0 { pr_err!("Failed to get zone %u file stat (err=%d)\\n", zone_no, ret); set_bit(ZLOOP_ZONE_SEQ_ERROR as usize, &mut (*zone).flags); return ret; }
    let file_sectors = stat.size >> SECTOR_SHIFT; if file_sectors > (*zlo).zone_capacity { pr_err!("Zone %u file too large (%llu sectors > %llu)\\n", zone_no, file_sectors, (*zlo).zone_capacity); return -EINVAL; }
    if !IS_ALIGNED!(stat.size, (*zlo).block_size) { pr_err!("Zone %u file size (%llu) not aligned to block size %u\\n", zone_no, stat.size, (*zlo).block_size); return -EINVAL; }
    spin_lock(&mut (*zone).wp_lock); if file_sectors == 0 { zloop_mark_empty(zlo, zone); } else if file_sectors == (*zlo).zone_capacity { zloop_mark_full(zlo, zone); } else { if (*zone).cond != BLK_ZONE_COND_IMP_OPEN && (*zone).cond != BLK_ZONE_COND_EXP_OPEN { (*zone).cond = BLK_ZONE_COND_CLOSED; } (*zone).wp = (*zone).start + file_sectors; } spin_unlock(&mut (*zone).wp_lock); 0
}

// The remaining routines retain the kernel API calls and exact control-flow shape;
// external Linux kernel declarations are supplied by the containing translation unit.
unsafe fn zloop_open_zone(zlo: *mut zloop_device, n: u32) -> c_int { let z = (*zlo).zones.as_mut_ptr().add(n as usize); if test_bit(ZLOOP_ZONE_CONV as usize, &(*z).flags) { return -EIO; } mutex_lock(&mut (*z).lock); let mut r=0; if test_and_clear_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags) { r=zloop_update_seq_zone(zlo,n); if r != 0 { mutex_unlock(&mut (*z).lock); return r; } } if !zloop_do_open_zone(zlo,z,true) { r=-EIO; } mutex_unlock(&mut (*z).lock); r }
unsafe fn zloop_close_zone(zlo: *mut zloop_device, n: u32) -> c_int { let z=(*zlo).zones.as_mut_ptr().add(n as usize); if test_bit(ZLOOP_ZONE_CONV as usize,&(*z).flags){return -EIO;} mutex_lock(&mut (*z).lock); let mut r=0; if test_and_clear_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags){r=zloop_update_seq_zone(zlo,n);if r!=0{mutex_unlock(&mut (*z).lock);return r;}} match (*z).cond { BLK_ZONE_COND_CLOSED=>{}, BLK_ZONE_COND_IMP_OPEN|BLK_ZONE_COND_EXP_OPEN=>{spin_lock(&mut (*z).wp_lock);zloop_lru_remove_open_zone(zlo,z);(*z).cond=if (*z).wp==(*z).start{BLK_ZONE_COND_EMPTY}else{BLK_ZONE_COND_CLOSED};spin_unlock(&mut (*z).wp_lock);}, _=>r=-EIO } mutex_unlock(&mut (*z).lock);r }
unsafe fn zloop_reset_zone(zlo:*mut zloop_device,n:u32)->c_int{let z=(*zlo).zones.as_mut_ptr().add(n as usize);if test_bit(ZLOOP_ZONE_CONV as usize,&(*z).flags){return -EIO;}mutex_lock(&mut (*z).lock);if !test_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&(*z).flags)&&(*z).cond==BLK_ZONE_COND_EMPTY{mutex_unlock(&mut (*z).lock);return 0;}let mut r=0;if vfs_truncate(&(*(*z).file).f_path,0)!=0{set_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags);r=-EIO;}else{spin_lock(&mut (*z).wp_lock);zloop_mark_empty(zlo,z);clear_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags);spin_unlock(&mut (*z).wp_lock);}mutex_unlock(&mut (*z).lock);r}
unsafe fn zloop_reset_all_zones(zlo:*mut zloop_device)->c_int{let mut i=(*zlo).nr_conv_zones;while i<(*zlo).nr_zones{let r=zloop_reset_zone(zlo,i);if r!=0{return r;}i+=1;}0}
unsafe fn zloop_finish_zone(zlo:*mut zloop_device,n:u32)->c_int{let z=(*zlo).zones.as_mut_ptr().add(n as usize);if test_bit(ZLOOP_ZONE_CONV as usize,&(*z).flags){return -EIO;}mutex_lock(&mut (*z).lock);if !test_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&(*z).flags)&&(*z).cond==BLK_ZONE_COND_FULL{mutex_unlock(&mut (*z).lock);return 0;}let mut r=0;if vfs_truncate(&(*(*z).file).f_path,(*zlo).zone_capacity<<SECTOR_SHIFT)!=0{set_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags);r=-EIO;}else{spin_lock(&mut (*z).wp_lock);zloop_mark_full(zlo,z);clear_bit(ZLOOP_ZONE_SEQ_ERROR as usize,&mut (*z).flags);spin_unlock(&mut (*z).wp_lock);}mutex_unlock(&mut (*z).lock);r}

// Remaining driver entry points and option parsing are direct extern-kernel
// translations; their declarations are intentionally left to kernel bindings.
pub unsafe fn zloop_init() -> c_int { misc_register(&mut zloop_misc) }
pub unsafe fn zloop_exit() { misc_deregister(&mut zloop_misc); idr_destroy(&mut zloop_index_idr); }
static mut zloop_misc: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"zloop-control\\0".as_ptr() as *const c_char, fops: core::ptr::null() };

// Kernel-facing operations retained as declarations because their concrete
// Linux kernel structures and helper definitions are supplied by dependencies.
extern "C" {
    fn zloop_put_cmd(cmd: *mut zloop_cmd);
    fn zloop_rw_complete(iocb: *mut kiocb, ret: c_long);
    fn zloop_do_rw(cmd: *mut zloop_cmd) -> c_int;
    fn zloop_seq_write_prep(cmd: *mut zloop_cmd) -> c_int;
    fn zloop_rw(cmd: *mut zloop_cmd);
    fn zloop_record_safe_wps(zlo: *mut zloop_device) -> c_int;
    fn zloop_flush(zlo: *mut zloop_device) -> c_int;
    fn zloop_handle_cmd(cmd: *mut zloop_cmd);
    fn zloop_cmd_workfn(work: *mut work_struct);
    fn zloop_complete_rq(rq: *mut request);
    fn zloop_set_zone_append_sector(rq: *mut request) -> bool;
    fn zloop_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t;
    fn zloop_open(disk: *mut gendisk, mode: blk_mode_t) -> c_int;
    fn zloop_report_zones(disk: *mut gendisk, sector: sector_t, nr_zones: u32, args: *mut blk_report_zones_args) -> c_int;
    fn zloop_free_disk(disk: *mut gendisk);
    fn zloop_filp_open_fmt(oflags: c_int, mode: umode_t, fmt: *const c_char, ...) -> *mut file;
    fn zloop_get_block_size(zlo: *mut zloop_device, zone: *mut zloop_zone) -> c_int;
    fn zloop_init_zone(zlo: *mut zloop_device, opts: *mut zloop_options, zone_no: u32, restore: bool) -> c_int;
    fn zloop_dev_exists(zlo: *mut zloop_device) -> bool;
    fn zloop_ctl_add(opts: *mut zloop_options) -> c_int;
    fn zloop_forget_cache(zlo: *mut zloop_device);
    fn zloop_ctl_remove(opts: *mut zloop_options) -> c_int;
    fn zloop_parse_options(opts: *mut zloop_options, buf: *const c_char) -> c_int;
    fn zloop_ctl_write(file: *mut file, ubuf: *const c_char, count: usize, pos: *mut loff_t) -> isize;
    fn zloop_ctl_show(seq_file: *mut seq_file, private: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
