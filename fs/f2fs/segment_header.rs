/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of fs/f2fs/segment.h. External kernel and f2fs
 * types/functions are intentionally left as dependencies of this header. */

pub const NULL_SEGNO: u32 = !0;
pub const NULL_SECNO: u32 = !0;
pub const DEF_RECLAIM_PREFREE_SEGMENTS: u32 = 5;
pub const DEF_MAX_RECLAIM_PREFREE_SEGMENTS: u32 = 4096;
pub const F2FS_MIN_SEGMENTS: u32 = 9;
pub const F2FS_MIN_META_SEGMENTS: u32 = 8;
pub const INVALID_MTIME: u64 = u64::MAX;
pub const MAX_SKIP_GC_COUNT: u32 = 16;
pub const DEF_MIN_IPU_UTIL: u32 = 70;
pub const DEF_MIN_FSYNC_BLOCKS: u32 = 8;
pub const DEF_MIN_HOT_BLOCKS: u32 = 16;
pub const SMALL_VOLUME_SEGMENTS: u32 = 16 * 512;
pub const F2FS_IPU_DISABLE: u32 = 0;

#[repr(C)]
pub struct victim_sel_policy { pub alloc_mode: i32, pub gc_mode: i32, pub dirty_bitmap: *mut c_ulong, pub max_search: u32, pub offset: u32, pub ofs_unit: u32, pub min_cost: u32, pub oldest_age: u64, pub min_segno: u32, pub age: u64, pub age_threshold: u64, pub one_time_gc: bool }
#[repr(C)]
pub struct seg_entry { pub type_: u32, pub valid_blocks: u32, pub ckpt_valid_blocks: u32, pub padding: u32, pub cur_valid_map: *mut u8, pub ckpt_valid_map: *mut u8, pub discard_map: *mut u8, pub mtime: u64 }
#[repr(C)]
pub struct sec_entry { pub valid_blocks: u32, pub ckpt_valid_blocks: u32 }
#[repr(C)]
pub struct revoke_entry { pub list: list_head, pub old_addr: block_t, pub index: pgoff_t }
#[repr(C)]
pub struct sit_info {
 pub sit_base_addr: block_t, pub sit_blocks: block_t, pub written_valid_blocks: block_t,
 pub bitmap: *mut c_char, pub sit_bitmap: *mut c_char,
 #[cfg(feature="CONFIG_F2FS_CHECK_FS")] pub invalid_segmap: *mut c_ulong,
 pub bitmap_size: u32, pub tmp_map: *mut c_ulong, pub dirty_sentries_bitmap: *mut c_ulong,
 pub dirty_sentries: u32, pub sents_per_block: u32, pub sentry_lock: rw_semaphore,
 pub sentries: *mut seg_entry, pub sec_entries: *mut sec_entry, pub elapsed_time: u64,
 pub mounted_time: u64, pub min_mtime: u64, pub max_mtime: u64, pub dirty_min_mtime: u64,
 pub dirty_max_mtime: u64, pub last_victim: [u32; MAX_GC_POLICY as usize]
}
#[repr(C)]
pub struct free_segmap_info { pub start_segno: u32, pub free_segments: u32, pub free_sections: u32, pub segmap_lock: spinlock_t, pub free_segmap: *mut c_ulong, pub free_secmap: *mut c_ulong }
#[repr(C)]
pub struct dirty_seglist_info { pub dirty_segmap: [*mut c_ulong; NR_DIRTY_TYPE as usize], pub dirty_secmap: *mut c_ulong, pub seglist_lock: mutex, pub nr_dirty: [i32; NR_DIRTY_TYPE as usize], pub victim_secmap: *mut c_ulong, pub pinned_secmap: *mut c_ulong, pub pinned_secmap_cnt: u32, pub enable_pin_section: bool }
#[repr(C)]
pub struct curseg_info { pub curseg_mutex: mutex, pub sum_blk: *mut f2fs_summary_block, pub journal_rwsem: rw_semaphore, pub journal: *mut f2fs_journal, pub alloc_type: u8, pub seg_type: u16, pub segno: u32, pub next_blkoff: u16, pub zone: u32, pub next_segno: u32, pub fragment_remained_chunk: i32, pub inited: bool }
#[repr(C)]
pub struct sit_entry_set { pub set_list: list_head, pub start_segno: u32, pub entry_cnt: u32 }

pub const LFS: i32 = 0; pub const SSR: i32 = 1; pub const AT_SSR: i32 = 2;
pub const GC_CB: i32 = 0; pub const GC_GREEDY: i32 = 1; pub const GC_AT: i32 = 2; pub const ALLOC_NEXT: i32 = 3; pub const FLUSH_DEVICE: i32 = 4; pub const MAX_GC_POLICY: i32 = 5;
pub const BG_GC: i32 = 0; pub const FG_GC: i32 = 1;
pub const DIRTY_HOT_DATA: usize = 0; pub const DIRTY_WARM_DATA: usize = 1; pub const DIRTY_COLD_DATA: usize = 2; pub const DIRTY_HOT_NODE: usize = 3; pub const DIRTY_WARM_NODE: usize = 4; pub const DIRTY_COLD_NODE: usize = 5; pub const DIRTY: usize = 6; pub const PRE: usize = 7; pub const NR_DIRTY_TYPE: usize = 8;
pub const F2FS_IPU_FORCE: u32 = 0; pub const F2FS_IPU_SSR: u32 = 1; pub const F2FS_IPU_UTIL: u32 = 2; pub const F2FS_IPU_SSR_UTIL: u32 = 3; pub const F2FS_IPU_FSYNC: u32 = 4; pub const F2FS_IPU_ASYNC: u32 = 5; pub const F2FS_IPU_NOCACHE: u32 = 6; pub const F2FS_IPU_HONOR_OPU_WRITE: u32 = 7; pub const F2FS_IPU_MAX: u32 = 8;

pub unsafe fn curseg_i(sbi: *mut f2fs_sb_info, ty: i32) -> *mut curseg_info { (SM_I(sbi).curseg_array as *mut curseg_info).add(ty as usize) }
pub unsafe fn is_curseg(sbi: *mut f2fs_sb_info, segno: u32) -> bool { for i in CURSEG_HOT_DATA..NO_CHECK_TYPE { if (*curseg_i(sbi,i)).segno == segno { return true; } } false }
pub unsafe fn is_cursec(sbi: *mut f2fs_sb_info, secno: u32) -> bool { for i in CURSEG_HOT_DATA..NO_CHECK_TYPE { if GET_SEC_FROM_SEG(sbi,(*curseg_i(sbi,i)).segno) == secno { return true; } } false }
pub unsafe fn get_seg_entry(sbi: *mut f2fs_sb_info, segno: u32) -> *mut seg_entry { (*SIT_I(sbi)).sentries.add(segno as usize) }
pub unsafe fn get_sec_entry(sbi: *mut f2fs_sb_info, segno: u32) -> *mut sec_entry { (*SIT_I(sbi)).sec_entries.add(GET_SEC_FROM_SEG(sbi,segno) as usize) }
pub unsafe fn get_valid_blocks(sbi:*mut f2fs_sb_info, segno:u32, section:bool)->u32 { if section && __is_large_section(sbi) {(*get_sec_entry(sbi,segno)).valid_blocks} else {(*get_seg_entry(sbi,segno)).valid_blocks} }
pub unsafe fn get_ckpt_valid_blocks(sbi:*mut f2fs_sb_info, segno:u32, section:bool)->u32 { if section && __is_large_section(sbi) {(*get_sec_entry(sbi,segno)).ckpt_valid_blocks} else {(*get_seg_entry(sbi,segno)).ckpt_valid_blocks} }
pub unsafe fn set_ckpt_valid_blocks(sbi:*mut f2fs_sb_info, segno:u32) { let sec=GET_SEC_FROM_SEG(sbi,segno); let mut start=GET_SEG_FROM_SEC(sbi,sec); let mut blocks=0; for _ in 0..SEGS_PER_SEC(sbi) { blocks += (*get_seg_entry(sbi,start)).ckpt_valid_blocks; start+=1; } (*get_sec_entry(sbi,segno)).ckpt_valid_blocks=blocks; }
pub unsafe fn valid_main_segno(sbi:*mut f2fs_sb_info, segno:u32)->bool { segno <= MAIN_SEGS(sbi)-1 }
pub unsafe fn written_block_count(sbi:*mut f2fs_sb_info)->block_t { (*SIT_I(sbi)).written_valid_blocks }
pub unsafe fn free_segments(sbi:*mut f2fs_sb_info)->u32 { (*FREE_I(sbi)).free_segments }
pub unsafe fn reserved_segments(sbi:*mut f2fs_sb_info)->u32 { SM_I(sbi).reserved_segments }
pub unsafe fn free_sections(sbi:*mut f2fs_sb_info)->u32 { (*FREE_I(sbi)).free_sections }
pub unsafe fn prefree_segments(sbi:*mut f2fs_sb_info)->u32 { DIRTY_I(sbi).nr_dirty[PRE] as u32 }
pub unsafe fn dirty_segments(sbi:*mut f2fs_sb_info)->u32 { DIRTY_I(sbi).nr_dirty[..6].iter().map(|x|*x as u32).sum() }
pub unsafe fn overprovision_segments(sbi:*mut f2fs_sb_info)->i32 { SM_I(sbi).ovp_segments }
pub unsafe fn reserved_sections(sbi:*mut f2fs_sb_info)->i32 { GET_SEC_FROM_SEG(sbi,reserved_segments(sbi)) as i32 }
pub unsafe fn is_f2fs_ipu_disable(sbi:*mut f2fs_sb_info)->bool { SM_I(sbi).ipu_policy == F2FS_IPU_DISABLE }

/* The remaining expressions are retained as C-compatible macro interfaces. */
#[macro_export] macro_rules! GET_L2R_SEGNO { ($f:expr,$s:expr) => { ($s) - (*$f).start_segno }; }
#[macro_export] macro_rules! GET_R2L_SEGNO { ($f:expr,$s:expr) => { ($s) + (*$f).start_segno }; }
#[macro_export] macro_rules! GET_SEC_FROM_SEG { ($sbi:expr,$s:expr) => { if ($s)==u32::MAX {u32::MAX} else {($s)/SEGS_PER_SEC($sbi)} }; }
#[macro_export] macro_rules! GET_SEG_FROM_SEC { ($sbi:expr,$s:expr) => { ($s)*SEGS_PER_SEC($sbi) }; }
#[macro_export] macro_rules! GET_START_SEG_FROM_SEC { ($sbi:expr,$s:expr) => { (($s)/SEGS_PER_SEC($sbi))*SEGS_PER_SEC($sbi) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
