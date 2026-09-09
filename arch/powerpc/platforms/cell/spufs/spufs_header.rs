/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SPU file system */

/* Linux dependencies and build-time conditions are supplied by the surrounding translation. */

pub const SPUFS_PS_MAP_SIZE: usize = 0x20000;
pub const SPUFS_MFC_MAP_SIZE: usize = 0x1000;
pub const SPUFS_CNTL_MAP_SIZE: usize = 0x1000;
pub const SPUFS_SIGNAL_MAP_SIZE: usize = PAGE_SIZE;
pub const SPUFS_MSS_MAP_SIZE: usize = 0x1000;

pub const SPUFS_MAGIC: u32 = 0x23c9b64e;

pub struct spu_context_ops;
pub struct spu_gang;

pub const SPU_SCHED_NOTIFY_ACTIVE: u32 = 0;
pub const SPU_SCHED_WAS_ACTIVE: u32 = 1;
pub const SPU_SCHED_SPU_RUN: u32 = 2;

pub const SWITCH_LOG_BUFSIZE: usize = 4096;
pub const SWITCH_LOG_START: u32 = 0;
pub const SWITCH_LOG_STOP: u32 = 1;
pub const SWITCH_LOG_EXIT: u32 = 2;

#[repr(C)]
pub struct switch_log_entry {
    pub tstamp: timespec64,
    pub spu_id: i32,
    pub type_: u32,
    pub val: u32,
    pub timebase: u64,
}

#[repr(C)]
pub struct switch_log {
    pub wait: wait_queue_head_t,
    pub head: c_ulong,
    pub tail: c_ulong,
    pub log: [switch_log_entry; 0],
}

#[repr(C)]
pub struct spu_context {
    pub spu: *mut spu,
    pub csa: spu_state,
    pub mmio_lock: spinlock_t,
    pub local_store: *mut address_space,
    pub mfc: *mut address_space,
    pub cntl: *mut address_space,
    pub signal1: *mut address_space,
    pub signal2: *mut address_space,
    pub mss: *mut address_space,
    pub psmap: *mut address_space,
    pub mapping_lock: mutex,
    pub object_id: u64,
    pub state: u32,
    pub state_mutex: mutex,
    pub run_mutex: mutex,
    pub owner: *mut mm_struct,
    pub kref: kref,
    pub ibox_wq: wait_queue_head_t,
    pub wbox_wq: wait_queue_head_t,
    pub stop_wq: wait_queue_head_t,
    pub mfc_wq: wait_queue_head_t,
    pub run_wq: wait_queue_head_t,
    pub tagwait: u32,
    pub ops: *mut spu_context_ops,
    pub reap_work: work_struct,
    pub flags: c_ulong,
    pub event_return: c_ulong,
    pub gang_list: list_head,
    pub gang: *mut spu_gang,
    pub prof_priv_kref: *mut kref,
    pub prof_priv_release: Option<unsafe extern "C" fn(*mut kref)>,
    pub tid: pid_t,
    pub rq: list_head,
    pub time_slice: c_uint,
    pub sched_flags: c_ulong,
    pub cpus_allowed: cpumask_t,
    pub policy: c_int,
    pub prio: c_int,
    pub last_ran: c_int,
    pub stats: spu_context_stats,
    pub switch_log: *mut switch_log,
    pub aff_list: list_head,
    pub aff_head: c_int,
    pub aff_offset: c_int,
}

#[repr(C)]
pub struct spu_context_stats {
    pub util_state: spu_utilization_state,
    pub tstamp: u64,
    pub times: [u64; SPU_UTIL_MAX],
    pub vol_ctx_switch: u64,
    pub invol_ctx_switch: u64,
    pub min_flt: u64,
    pub maj_flt: u64,
    pub hash_flt: u64,
    pub slb_flt: u64,
    pub slb_flt_base: u64,
    pub class2_intr: u64,
    pub class2_intr_base: u64,
    pub libassist: u64,
}

#[repr(C)]
pub struct spu_gang {
    pub list: list_head,
    pub mutex: mutex,
    pub kref: kref,
    pub contexts: c_int,
    pub aff_ref_ctx: *mut spu_context,
    pub aff_list_head: list_head,
    pub aff_mutex: mutex,
    pub aff_flags: c_int,
    pub aff_ref_spu: *mut spu,
    pub aff_sched_count: atomic_t,
    pub alive: c_int,
}

pub const AFF_OFFSETS_SET: c_int = 1;
pub const AFF_MERGED: c_int = 2;

#[repr(C)]
pub struct mfc_dma_command {
    pub pad: i32,
    pub lsa: u32,
    pub ea: u64,
    pub size: u16,
    pub tag: u16,
    pub class_: u16,
    pub cmd: u16,
}

#[repr(C)]
pub struct spu_context_ops {
    pub mbox_read: Option<unsafe extern "C" fn(*mut spu_context, *mut u32) -> c_int>,
    pub mbox_stat_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub mbox_stat_poll: Option<unsafe extern "C" fn(*mut spu_context, __poll_t) -> __poll_t>,
    pub ibox_read: Option<unsafe extern "C" fn(*mut spu_context, *mut u32) -> c_int>,
    pub wbox_write: Option<unsafe extern "C" fn(*mut spu_context, u32) -> c_int>,
    pub signal1_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub signal1_write: Option<unsafe extern "C" fn(*mut spu_context, u32)>,
    pub signal2_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub signal2_write: Option<unsafe extern "C" fn(*mut spu_context, u32)>,
    pub signal1_type_set: Option<unsafe extern "C" fn(*mut spu_context, u64)>,
    pub signal1_type_get: Option<unsafe extern "C" fn(*mut spu_context) -> u64>,
    pub signal2_type_set: Option<unsafe extern "C" fn(*mut spu_context, u64)>,
    pub signal2_type_get: Option<unsafe extern "C" fn(*mut spu_context) -> u64>,
    pub npc_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub npc_write: Option<unsafe extern "C" fn(*mut spu_context, u32)>,
    pub status_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub get_ls: Option<unsafe extern "C" fn(*mut spu_context) -> *mut c_char>,
    pub privcntl_write: Option<unsafe extern "C" fn(*mut spu_context, u64)>,
    pub runcntl_read: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub runcntl_write: Option<unsafe extern "C" fn(*mut spu_context, u32)>,
    pub runcntl_stop: Option<unsafe extern "C" fn(*mut spu_context)>,
    pub master_start: Option<unsafe extern "C" fn(*mut spu_context)>,
    pub master_stop: Option<unsafe extern "C" fn(*mut spu_context)>,
    pub set_mfc_query: Option<unsafe extern "C" fn(*mut spu_context, u32, u32) -> c_int>,
    pub read_mfc_tagstatus: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub get_mfc_free_elements: Option<unsafe extern "C" fn(*mut spu_context) -> u32>,
    pub send_mfc_command: Option<unsafe extern "C" fn(*mut spu_context, *mut mfc_dma_command) -> c_int>,
    pub dma_info_read: Option<unsafe extern "C" fn(*mut spu_context, *mut spu_dma_info)>,
    pub proxydma_info_read: Option<unsafe extern "C" fn(*mut spu_context, *mut spu_proxydma_info)>,
    pub restart_dma: Option<unsafe extern "C" fn(*mut spu_context)>,
}

extern "C" {
    pub static mut spu_hw_ops: spu_context_ops;
    pub static mut spu_backing_ops: spu_context_ops;
}

#[repr(C)]
pub struct spufs_inode_info {
    pub i_ctx: *mut spu_context,
    pub i_gang: *mut spu_gang,
    pub vfs_inode: inode,
    pub i_openers: c_int,
}

#[macro_export]
macro_rules! SPUFS_I {
    ($inode:expr) => {
        unsafe { &mut *((($inode as *mut u8).sub(offset_of!(spufs_inode_info, vfs_inode))) as *mut spufs_inode_info) }
    };
}

#[repr(C)]
pub struct spufs_tree_descr {
    pub name: *const c_char,
    pub ops: *const file_operations,
    pub mode: umode_t,
    pub size: usize,
}

extern "C" {
    pub static spufs_dir_contents: [spufs_tree_descr; 0];
    pub static spufs_dir_nosched_contents: [spufs_tree_descr; 0];
    pub static spufs_dir_debug_contents: [spufs_tree_descr; 0];
    pub static mut spufs_calls: spufs_calls;
    pub fn spufs_run_spu(ctx: *mut spu_context, npc: *mut u32, status: *mut u32) -> c_long;
    pub fn spufs_create(nd: *const path, dentry: *mut dentry, flags: c_uint, mode: umode_t, filp: *mut file) -> c_long;
    pub fn spufs_coredump_extra_notes_size() -> c_int;
}

pub struct coredump_params;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
