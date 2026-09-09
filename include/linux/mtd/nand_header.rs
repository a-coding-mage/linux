/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/mtd/nand.h. */

use core::ffi::c_void;

pub struct nand_device;
pub struct mtd_info;
pub struct mtd_ooblayout_ops;
pub struct device;
pub struct list_head;
pub struct module;
pub struct device_node;
pub struct mtd_oob_ops;
pub struct erase_info;

#[repr(C)]
pub struct nand_memory_organization { pub bits_per_cell: u32, pub pagesize: u32, pub oobsize: u32, pub pages_per_eraseblock: u32, pub eraseblocks_per_lun: u32, pub max_bad_eraseblocks_per_lun: u32, pub planes_per_lun: u32, pub luns_per_target: u32, pub ntargets: u32 }

#[macro_export]
macro_rules! NAND_MEMORG { ($bpc:expr, $ps:expr, $os:expr, $ppe:expr, $epl:expr, $mbb:expr, $ppl:expr, $lpt:expr, $nt:expr) => { nand_memory_organization { bits_per_cell: $bpc, pagesize: $ps, oobsize: $os, pages_per_eraseblock: $ppe, eraseblocks_per_lun: $epl, max_bad_eraseblocks_per_lun: $mbb, planes_per_lun: $ppl, luns_per_target: $lpt, ntargets: $nt } }; }

#[repr(C)] pub struct nand_row_converter { pub lun_addr_shift: u32, pub eraseblock_addr_shift: u32 }
#[repr(C)] pub struct nand_pos { pub target: u32, pub lun: u32, pub plane: u32, pub eraseblock: u32, pub page: u32 }

#[repr(C)] #[derive(Copy, Clone)] pub enum nand_page_io_req_type { NAND_PAGE_READ = 0, NAND_PAGE_WRITE }
#[repr(C)] pub union nand_page_io_buf { pub out: *const c_void, pub r#in: *mut c_void }
#[repr(C)] pub struct nand_page_io_req { pub r#type: nand_page_io_req_type, pub pos: nand_pos, pub dataoffs: u32, pub datalen: u32, pub databuf: nand_page_io_buf, pub ooboffs: u32, pub ooblen: u32, pub oobbuf: nand_page_io_buf, pub mode: i32, pub continuous: bool }

extern "C" { pub fn nand_get_small_page_ooblayout() -> *const mtd_ooblayout_ops; pub fn nand_get_large_page_ooblayout() -> *const mtd_ooblayout_ops; pub fn nand_get_large_page_hamming_ooblayout() -> *const mtd_ooblayout_ops; }

#[repr(C)] pub enum nand_ecc_engine_type { NAND_ECC_ENGINE_TYPE_INVALID, NAND_ECC_ENGINE_TYPE_NONE, NAND_ECC_ENGINE_TYPE_SOFT, NAND_ECC_ENGINE_TYPE_ON_HOST, NAND_ECC_ENGINE_TYPE_ON_DIE }
#[repr(C)] pub enum nand_ecc_placement { NAND_ECC_PLACEMENT_UNKNOWN, NAND_ECC_PLACEMENT_OOB, NAND_ECC_PLACEMENT_INTERLEAVED }
#[repr(C)] pub enum nand_ecc_algo { NAND_ECC_ALGO_UNKNOWN, NAND_ECC_ALGO_HAMMING, NAND_ECC_ALGO_BCH, NAND_ECC_ALGO_RS }
#[repr(C)] pub struct nand_ecc_props { pub engine_type: nand_ecc_engine_type, pub placement: nand_ecc_placement, pub algo: nand_ecc_algo, pub strength: u32, pub step_size: u32, pub flags: u32 }
#[macro_export] macro_rules! NAND_ECCREQ { ($str_:expr, $stp:expr) => { nand_ecc_props { strength: $str_, step_size: $stp, ..unsafe { core::mem::zeroed() } } }; }
pub const NAND_ECC_MAXIMIZE_STRENGTH: u32 = 1 << 0;

#[repr(C)] pub struct nand_bbt { pub cache: *mut usize }
#[repr(C)] pub struct nand_ops { pub erase: Option<unsafe extern "C" fn(*mut nand_device, *const nand_pos) -> i32>, pub markbad: Option<unsafe extern "C" fn(*mut nand_device, *const nand_pos) -> i32>, pub isbad: Option<unsafe extern "C" fn(*mut nand_device, *const nand_pos) -> bool> }
#[repr(C)] pub struct nand_ecc_context { pub conf: nand_ecc_props, pub nsteps: u32, pub total: u32, pub priv_: *mut c_void }
#[repr(C)] pub struct nand_ecc_engine_ops { pub init_ctx: Option<unsafe extern "C" fn(*mut nand_device) -> i32>, pub cleanup_ctx: Option<unsafe extern "C" fn(*mut nand_device)>, pub prepare_io_req: Option<unsafe extern "C" fn(*mut nand_device, *mut nand_page_io_req) -> i32>, pub finish_io_req: Option<unsafe extern "C" fn(*mut nand_device, *mut nand_page_io_req) -> i32> }
#[repr(C)] pub enum nand_ecc_engine_integration { NAND_ECC_ENGINE_INTEGRATION_INVALID, NAND_ECC_ENGINE_INTEGRATION_PIPELINED, NAND_ECC_ENGINE_INTEGRATION_EXTERNAL }
#[repr(C)] pub struct nand_ecc_engine { pub dev: *mut device, pub node: list_head, pub ops: *const nand_ecc_engine_ops, pub integration: nand_ecc_engine_integration, pub priv_: *mut c_void }

extern "C" { pub fn of_get_nand_ecc_user_config(nand: *mut nand_device); pub fn nand_ecc_init_ctx(nand: *mut nand_device) -> i32; pub fn nand_ecc_cleanup_ctx(nand: *mut nand_device); pub fn nand_ecc_prepare_io_req(nand: *mut nand_device, req: *mut nand_page_io_req) -> i32; pub fn nand_ecc_finish_io_req(nand: *mut nand_device, req: *mut nand_page_io_req) -> i32; pub fn nand_ecc_is_strong_enough(nand: *mut nand_device) -> bool; pub fn nand_ecc_register_on_host_hw_engine(engine: *mut nand_ecc_engine) -> i32; pub fn nand_ecc_unregister_on_host_hw_engine(engine: *mut nand_ecc_engine) -> i32; pub fn nand_ecc_get_sw_engine(nand: *mut nand_device) -> *mut nand_ecc_engine; pub fn nand_ecc_get_on_die_hw_engine(nand: *mut nand_device) -> *mut nand_ecc_engine; pub fn nand_ecc_get_on_host_hw_engine(nand: *mut nand_device) -> *mut nand_ecc_engine; pub fn nand_ecc_put_on_host_hw_engine(nand: *mut nand_device); pub fn nand_ecc_get_engine_dev(host: *mut device) -> *mut device; pub fn nand_ecc_sw_hamming_get_engine() -> *mut nand_ecc_engine; pub fn nand_ecc_sw_bch_get_engine() -> *mut nand_ecc_engine; }

#[repr(C)] pub struct nand_ecc_req_tweak_ctx { pub orig_req: nand_page_io_req, pub nand: *mut nand_device, pub page_buffer_size: u32, pub oob_buffer_size: u32, pub spare_databuf: *mut c_void, pub spare_oobbuf: *mut c_void, pub bounce_data: bool, pub bounce_oob: bool }
extern "C" { pub fn nand_ecc_init_req_tweaking(ctx: *mut nand_ecc_req_tweak_ctx, nand: *mut nand_device) -> i32; pub fn nand_ecc_cleanup_req_tweaking(ctx: *mut nand_ecc_req_tweak_ctx); pub fn nand_ecc_tweak_req(ctx: *mut nand_ecc_req_tweak_ctx, req: *mut nand_page_io_req); pub fn nand_ecc_restore_req(ctx: *mut nand_ecc_req_tweak_ctx, req: *mut nand_page_io_req); }
#[repr(C)] pub struct nand_ecc { pub defaults: nand_ecc_props, pub requirements: nand_ecc_props, pub user_conf: nand_ecc_props, pub ctx: nand_ecc_context, pub ondie_engine: *mut nand_ecc_engine, pub engine: *mut nand_ecc_engine }
#[repr(C)] pub struct nand_device { pub mtd: mtd_info, pub memorg: nand_memory_organization, pub ecc: nand_ecc, pub rowconv: nand_row_converter, pub bbt: nand_bbt, pub ops: *const nand_ops }
#[repr(C)] pub struct nand_io_iter { pub req: nand_page_io_req, pub oobbytes_per_page: u32, pub dataleft: u32, pub oobleft: u32 }

extern "C" { pub fn nanddev_init(nand: *mut nand_device, ops: *const nand_ops, owner: *mut module) -> i32; pub fn nanddev_cleanup(nand: *mut nand_device); pub fn nanddev_isbad(nand: *mut nand_device, pos: *const nand_pos) -> bool; pub fn nanddev_isreserved(nand: *mut nand_device, pos: *const nand_pos) -> bool; pub fn nanddev_markbad(nand: *mut nand_device, pos: *const nand_pos) -> i32; pub fn nanddev_ecc_engine_init(nand: *mut nand_device) -> i32; pub fn nanddev_ecc_engine_cleanup(nand: *mut nand_device); pub fn nanddev_bbt_init(nand: *mut nand_device) -> i32; pub fn nanddev_bbt_cleanup(nand: *mut nand_device); pub fn nanddev_bbt_update(nand: *mut nand_device) -> i32; pub fn nanddev_bbt_get_block_status(nand: *const nand_device, entry: u32) -> i32; pub fn nanddev_bbt_set_block_status(nand: *mut nand_device, entry: u32, status: nand_bbt_block_status) -> i32; pub fn nanddev_bbt_markbad(nand: *mut nand_device, block: u32) -> i32; pub fn nanddev_mtd_erase(mtd: *mut mtd_info, einfo: *mut erase_info) -> i32; pub fn nanddev_mtd_max_bad_blocks(mtd: *mut mtd_info, offs: i64, len: usize) -> i32; pub fn nand_check_erased_ecc_chunk(data: *mut c_void, datalen: i32, ecc: *mut c_void, ecclen: i32, extraoob: *mut c_void, extraooblen: i32, threshold: i32) -> i32; }

#[inline] pub unsafe fn nanddev_bits_per_cell(n: *const nand_device) -> u32 { (*n).memorg.bits_per_cell }
#[inline] pub unsafe fn nanddev_page_size(n: *const nand_device) -> usize { (*n).memorg.pagesize as usize }
#[inline] pub unsafe fn nanddev_per_page_oobsize(n: *const nand_device) -> u32 { (*n).memorg.oobsize }
#[inline] pub unsafe fn nanddev_pages_per_eraseblock(n: *const nand_device) -> u32 { (*n).memorg.pages_per_eraseblock }
#[inline] pub unsafe fn nanddev_pages_per_target(n: *const nand_device) -> u32 { (*n).memorg.pages_per_eraseblock * (*n).memorg.eraseblocks_per_lun * (*n).memorg.luns_per_target }
#[inline] pub unsafe fn nanddev_eraseblock_size(n: *const nand_device) -> usize { (*n).memorg.pagesize as usize * (*n).memorg.pages_per_eraseblock as usize }
#[inline] pub unsafe fn nanddev_eraseblocks_per_lun(n: *const nand_device) -> u32 { (*n).memorg.eraseblocks_per_lun }
#[inline] pub unsafe fn nanddev_eraseblocks_per_target(n: *const nand_device) -> u32 { (*n).memorg.eraseblocks_per_lun * (*n).memorg.luns_per_target }
#[inline] pub unsafe fn nanddev_target_size(n: *const nand_device) -> u64 { (*n).memorg.luns_per_target as u64 * (*n).memorg.eraseblocks_per_lun as u64 * (*n).memorg.pages_per_eraseblock as u64 * (*n).memorg.pagesize as u64 }
#[inline] pub unsafe fn nanddev_ntargets(n: *const nand_device) -> u32 { (*n).memorg.ntargets }
#[inline] pub unsafe fn nanddev_neraseblocks(n: *const nand_device) -> u32 { (*n).memorg.ntargets * (*n).memorg.luns_per_target * (*n).memorg.eraseblocks_per_lun }
#[inline] pub unsafe fn nanddev_size(n: *const nand_device) -> u64 { nanddev_target_size(n) * nanddev_ntargets(n) as u64 }
#[inline] pub unsafe fn nanddev_get_memorg(n: *mut nand_device) -> *mut nand_memory_organization { &mut (*n).memorg }
#[inline] pub unsafe fn nanddev_get_ecc_conf(n: *mut nand_device) -> *const nand_ecc_props { &(*n).ecc.ctx.conf }
#[inline] pub unsafe fn nanddev_get_ecc_nsteps(n: *mut nand_device) -> u32 { (*n).ecc.ctx.nsteps }
#[inline] pub unsafe fn nanddev_get_ecc_bytes_per_step(n: *mut nand_device) -> u32 { (*n).ecc.ctx.total / (*n).ecc.ctx.nsteps }
#[inline] pub unsafe fn nanddev_get_ecc_requirements(n: *mut nand_device) -> *const nand_ecc_props { &(*n).ecc.requirements }
#[inline] pub unsafe fn nanddev_set_ecc_requirements(n: *mut nand_device, r: *const nand_ecc_props) { (*n).ecc.requirements = *r; }

#[inline] pub unsafe fn nanddev_pos_cmp(a: *const nand_pos, b: *const nand_pos) -> i32 { for (x,y) in [((*a).target,(*b).target),((*a).lun,(*b).lun),((*a).eraseblock,(*b).eraseblock),((*a).page,(*b).page)] { if x != y { return if x < y {-1} else {1}; } } 0 }
#[inline] pub unsafe fn nanddev_pos_to_row(n: *mut nand_device, p: *const nand_pos) -> u32 { ((*p).lun << (*n).rowconv.lun_addr_shift) | ((*p).eraseblock << (*n).rowconv.eraseblock_addr_shift) | (*p).page }
#[inline] pub unsafe fn nand_to_ecc_ctx(n: *mut nand_device) -> *mut c_void { (*n).ecc.ctx.priv_ }

#[repr(C)] pub enum nand_bbt_block_status { NAND_BBT_BLOCK_STATUS_UNKNOWN, NAND_BBT_BLOCK_GOOD, NAND_BBT_BLOCK_WORN, NAND_BBT_BLOCK_RESERVED, NAND_BBT_BLOCK_FACTORY_BAD, NAND_BBT_BLOCK_NUM_STATUS }
#[inline] pub unsafe fn nanddev_bbt_is_initialized(n: *mut nand_device) -> bool { !(*n).bbt.cache.is_null() }

extern "C" { pub fn mtd_device_register(mtd: *mut mtd_info, parts: *const c_void, count: i32) -> i32; pub fn mtd_device_unregister(mtd: *mut mtd_info) -> i32; pub fn mtd_set_of_node(mtd: *mut mtd_info, np: *mut device_node); pub fn mtd_get_of_node(mtd: *mut mtd_info) -> *mut device_node; pub fn mtd_oobavail(mtd: *mut mtd_info, req: *mut mtd_oob_ops) -> u32; }
#[inline] pub unsafe fn nanddev_register(n: *mut nand_device) -> i32 { mtd_device_register(&mut (*n).mtd, core::ptr::null(), 0) }
#[inline] pub unsafe fn nanddev_unregister(n: *mut nand_device) -> i32 { mtd_device_unregister(&mut (*n).mtd) }
#[inline] pub unsafe fn nanddev_set_of_node(n: *mut nand_device, np: *mut device_node) { mtd_set_of_node(&mut (*n).mtd, np) }
#[inline] pub unsafe fn nanddev_get_of_node(n: *mut nand_device) -> *mut device_node { mtd_get_of_node(&mut (*n).mtd) }

#[inline] pub unsafe fn nanddev_offs_to_pos(n: *mut nand_device, mut offs: i64, p: *mut nand_pos) -> u32 { let pageoffs = (offs as u64 % (*n).memorg.pagesize as u64) as u32; offs /= (*n).memorg.pagesize as i64; (*p).page = (offs as u64 % (*n).memorg.pages_per_eraseblock as u64) as u32; offs /= (*n).memorg.pages_per_eraseblock as i64; (*p).eraseblock = (offs as u64 % (*n).memorg.eraseblocks_per_lun as u64) as u32; (*p).plane = (*p).eraseblock % (*n).memorg.planes_per_lun; offs /= (*n).memorg.eraseblocks_per_lun as i64; (*p).lun = (offs as u64 % (*n).memorg.luns_per_target as u64) as u32; (*p).target = offs as u32; pageoffs }
#[inline] pub unsafe fn nanddev_pos_to_offs(n: *mut nand_device, p: *const nand_pos) -> i64 { let npages = (*p).page + (((*p).eraseblock + ((*p).lun + (*p).target * (*n).memorg.luns_per_target) * (*n).memorg.eraseblocks_per_lun) * (*n).memorg.pages_per_eraseblock); npages as i64 * (*n).memorg.pagesize as i64 }
#[inline] pub unsafe fn nanddev_pos_next_target(_n: *mut nand_device, p: *mut nand_pos) { (*p).page=0; (*p).plane=0; (*p).eraseblock=0; (*p).lun=0; (*p).target+=1; }
#[inline] pub unsafe fn nanddev_pos_next_lun(n: *mut nand_device, p: *mut nand_pos) { if (*p).lun >= (*n).memorg.luns_per_target - 1 { return nanddev_pos_next_target(n,p) } (*p).lun+=1; (*p).page=0; (*p).plane=0; (*p).eraseblock=0; }
#[inline] pub unsafe fn nanddev_pos_next_eraseblock(n: *mut nand_device, p: *mut nand_pos) { if (*p).eraseblock >= (*n).memorg.eraseblocks_per_lun - 1 { return nanddev_pos_next_lun(n,p) } (*p).eraseblock+=1; (*p).page=0; (*p).plane=(*p).eraseblock % (*n).memorg.planes_per_lun; }
#[inline] pub unsafe fn nanddev_pos_next_page(n: *mut nand_device, p: *mut nand_pos) { if (*p).page >= (*n).memorg.pages_per_eraseblock - 1 { return nanddev_pos_next_eraseblock(n,p) } (*p).page+=1; }

#[inline] pub unsafe fn nanddev_bbt_pos_to_entry(n: *mut nand_device, p: *const nand_pos) -> u32 { (*p).eraseblock + ((*p).lun + (*p).target * (*n).memorg.luns_per_target) * (*n).memorg.eraseblocks_per_lun }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
