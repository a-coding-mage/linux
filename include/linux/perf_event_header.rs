/* Rust translation of linux/perf_event.h. Included kernel dependencies are
 * intentionally left as external symbols/types supplied by other units. */

use core::ffi::{c_char, c_int, c_long, c_void};

pub type u8 = core::primitive::u8; pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32; pub type u64 = core::primitive::u64;
pub type ulong = usize; pub type ssize_t = isize;

#[repr(C)] pub struct perf_callchain_entry { pub nr: u64, pub ip: [u64; 0] }
#[repr(C)] pub struct perf_callchain_entry_ctx { pub entry: *mut perf_callchain_entry, pub max_stack:u32, pub nr:u32, pub contexts:i16, pub contexts_maxed:bool }
pub type perf_copy_f = unsafe extern "C" fn(*mut c_void,*const c_void,usize,usize)->usize;
#[repr(C,packed)] pub union perf_raw_frag_next { pub next:*mut perf_raw_frag, pub pad:usize }
#[repr(C,packed)] pub struct perf_raw_frag { pub link:perf_raw_frag_next, pub copy:perf_copy_f, pub data:*mut c_void, pub size:u32 }
#[repr(C)] pub struct perf_raw_record { pub frag:perf_raw_frag, pub size:u32 }
#[inline] pub unsafe fn perf_raw_frag_last(frag:*const perf_raw_frag)->bool { (*frag).link.pad < core::mem::size_of::<u64>() }

#[repr(C)] pub struct perf_branch_stack { pub nr:u64, pub hw_idx:u64, pub entries:[perf_branch_entry;0] }
#[repr(C)] pub struct hw_perf_event_extra { pub config:u64, pub reg:u32, pub alloc:c_int, pub idx:c_int }
pub const PERF_EVENT_FLAG_ARCH:u32=0x0fffffff; pub const PERF_EVENT_FLAG_USER_READ_CNT:u32=0x80000000;

#[repr(C)] pub struct hw_perf_event {
    pub config:u64,pub config1:u64,pub last_tag:u64,pub dyn_constraint:u64,pub config_base:usize,pub event_base:usize,
    pub event_base_rdpmc:c_int,pub idx:c_int,pub last_cpu:c_int,pub flags:c_int,pub extra_reg:hw_perf_event_extra,pub branch_reg:hw_perf_event_extra,
    pub target:*mut task_struct,pub addr_filters:*mut c_void,pub addr_filters_gen:usize,pub state:c_int,pub prev_count:local64_t,pub sample_period:u64,
    pub last_period:u64,pub period_left:local64_t,pub saved_metric:u64,pub saved_slots:u64,pub interrupts_seq:u64,pub interrupts:u64,pub freq_time_stamp:u64,pub freq_count_stamp:u64 }
#[repr(C)] pub struct perf_pmu_scope_tag { _private:[u8;0] }
pub const PERF_HES_STOPPED:i32=1; pub const PERF_HES_UPTODATE:i32=2; pub const PERF_HES_ARCH:i32=4;
pub const PERF_PMU_TXN_ADD:u32=1; pub const PERF_PMU_TXN_READ:u32=2;
pub const PERF_PMU_CAP_NO_INTERRUPT:i32=1; pub const PERF_PMU_CAP_NO_NMI:i32=2; pub const PERF_PMU_CAP_AUX_NO_SG:i32=4; pub const PERF_PMU_CAP_EXTENDED_REGS:i32=8; pub const PERF_PMU_CAP_EXCLUSIVE:i32=16; pub const PERF_PMU_CAP_ITRACE:i32=32; pub const PERF_PMU_CAP_NO_EXCLUDE:i32=64; pub const PERF_PMU_CAP_AUX_OUTPUT:i32=128; pub const PERF_PMU_CAP_EXTENDED_HW_TYPE:i32=256; pub const PERF_PMU_CAP_AUX_PAUSE:i32=512; pub const PERF_PMU_CAP_AUX_PREFER_LARGE:i32=1024; pub const PERF_PMU_CAP_MEDIATED_VPMU:i32=2048;
#[repr(C)] pub struct pmu { pub entry:list_head,pub events_lock:spinlock_t,pub events:list_head,pub module:*mut module,pub dev:*mut device,pub parent:*mut device,pub attr_groups:*const *const attribute_group,pub attr_update:*const *const attribute_group,pub name:*const c_char,pub type_:c_int,pub capabilities:c_int,pub scope:u32,pub cpu_pmu_context:*mut *mut perf_cpu_pmu_context,pub exclusive_cnt:atomic_t,pub task_ctx_nr:c_int,pub hrtimer_interval_ms:c_int,pub nr_addr_filters:u32,pub pmu_enable:Option<unsafe extern "C" fn(*mut pmu)>,pub pmu_disable:Option<unsafe extern "C" fn(*mut pmu)>,pub event_init:Option<unsafe extern "C" fn(*mut perf_event)->c_int>,pub event_mapped:Option<unsafe extern "C" fn(*mut perf_event,*mut mm_struct)>,pub event_unmapped:Option<unsafe extern "C" fn(*mut perf_event,*mut mm_struct)>,pub add:Option<unsafe extern "C" fn(*mut perf_event,c_int)->c_int>,pub del:Option<unsafe extern "C" fn(*mut perf_event,c_int)>,pub start:Option<unsafe extern "C" fn(*mut perf_event,c_int)>,pub stop:Option<unsafe extern "C" fn(*mut perf_event,c_int)>,pub read:Option<unsafe extern "C" fn(*mut perf_event)>,pub start_txn:Option<unsafe extern "C" fn(*mut pmu,u32)>,pub commit_txn:Option<unsafe extern "C" fn(*mut pmu)->c_int>,pub cancel_txn:Option<unsafe extern "C" fn(*mut pmu)>,pub event_idx:Option<unsafe extern "C" fn(*mut perf_event)->c_int>,pub sched_task:Option<unsafe extern "C" fn(*mut perf_event_pmu_context,*mut task_struct,bool)>,pub task_ctx_cache:*mut kmem_cache>,pub setup_aux:Option<unsafe extern "C" fn(*mut perf_event,*mut *mut c_void,c_int,bool)->*mut c_void>,pub free_aux:Option<unsafe extern "C" fn(*mut c_void)>,pub snapshot_aux:Option<unsafe extern "C" fn(*mut perf_event,*mut perf_output_handle,usize)->c_long>,pub addr_filters_validate:Option<unsafe extern "C" fn(*mut list_head)->c_int>,pub addr_filters_sync:Option<unsafe extern "C" fn(*mut perf_event)>,pub aux_output_match:Option<unsafe extern "C" fn(*mut perf_event)->c_int>,pub filter:Option<unsafe extern "C" fn(*mut pmu,c_int)->bool>,pub check_period:Option<unsafe extern "C" fn(*mut perf_event,u64)->c_int> }
#[repr(C)] pub struct perf_addr_filter { pub entry:list_head,pub path:path,pub offset:usize,pub size:usize,pub action:perf_addr_filter_action_t }
#[repr(C)] pub struct perf_addr_filters_head { pub list:list_head,pub lock:raw_spinlock_t,pub nr_file_filters:u32 }
#[repr(C)] pub struct perf_addr_filter_range { pub start:usize,pub size:usize }
#[repr(i32)] pub enum perf_addr_filter_action_t { STOP=0, START, FILTER }
#[repr(i32)] pub enum perf_event_state { DEAD=-5, REVOKED=-4, EXIT=-3, ERROR=-2, OFF=-1, INACTIVE=0, ACTIVE=1 }
pub const PERF_EV_CAP_SOFTWARE:i32=1; pub const PERF_EV_CAP_READ_ACTIVE_PKG:i32=2; pub const PERF_EV_CAP_SIBLING:i32=4; pub const PERF_EV_CAP_READ_SCOPE:i32=8;
pub const PERF_ATTACH_CONTEXT:i32=1; pub const PERF_ATTACH_GROUP:i32=2; pub const PERF_ATTACH_TASK:i32=4; pub const PERF_ATTACH_TASK_DATA:i32=8; pub const PERF_ATTACH_GLOBAL_DATA:i32=16; pub const PERF_ATTACH_SCHED_CB:i32=32; pub const PERF_ATTACH_CHILD:i32=64; pub const PERF_ATTACH_EXCLUSIVE:i32=128; pub const PERF_ATTACH_CALLCHAIN:i32=256; pub const PERF_ATTACH_ITRACE:i32=512;
#[repr(C)] pub struct perf_event_groups { pub tree:rb_root,pub index:u64 }
#[repr(C)] pub struct perf_time_ctx { pub time:u64,pub stamp:u64,pub offset:u64 }
#[repr(C)] pub struct perf_event_pmu_context { pub pmu:*mut pmu,pub ctx:*mut perf_event_context,pub pmu_ctx_entry:list_head,pub pinned_active:list_head,pub flexible_active:list_head,pub embedded:u32,pub nr_events:u32,pub nr_cgroups:u32,pub nr_freq:u32,pub refcount:atomic_t,pub rcu_head:rcu_head,pub rotate_necessary:c_int }
#[repr(C)] pub struct perf_event_context { pub lock:raw_spinlock_t,pub mutex:mutex,pub pmu_ctx_list:list_head,pub pinned_groups:perf_event_groups,pub flexible_groups:perf_event_groups,pub event_list:list_head,pub nr_events:c_int,pub nr_user:c_int,pub is_active:c_int,pub nr_stat:c_int,pub nr_freq:c_int,pub rotate_disable:c_int,pub refcount:refcount_t,pub task:*mut task_struct,pub time:perf_time_ctx,pub timeguest:perf_time_ctx,pub parent_ctx:*mut perf_event_context,pub parent_gen:u64,pub generation:u64,pub pin_count:c_int,pub rcu_head:rcu_head,pub nr_no_switch_fast:local_t }
#[inline] pub unsafe fn perf_pmu_ctx_is_active(e:*mut perf_event_pmu_context)->bool { !list_empty(&(*e).flexible_active) || !list_empty(&(*e).pinned_active) }

extern "C" { pub fn perf_event_init(); pub fn perf_pmu_register(p:*mut pmu,name:*const c_char,type_:c_int)->c_int; pub fn perf_pmu_unregister(p:*mut pmu)->c_int; pub fn perf_event_enable(e:*mut perf_event); pub fn perf_event_disable(e:*mut perf_event); }

/* Opaque dependency-backed kernel structures and the remaining declarations. */
#[repr(C)] pub struct perf_event { _private:[u8;0] }
#[repr(C)] pub struct perf_output_handle { _private:[u8;0] }
#[repr(C)] pub struct perf_cpu_pmu_context { _private:[u8;0] }
#[repr(C)] pub struct perf_sample_data { _private:[u8;0] }
#[repr(C)] pub struct perf_branch_entry { pub from:u64,pub to:u64 }
#[repr(C)] pub struct task_struct { pub sched_migrated:u8 }
#[repr(C)] pub struct list_head { _private:[u8;0] } #[repr(C)] pub struct raw_spinlock_t{_private:[u8;0]} #[repr(C)] pub struct spinlock_t{_private:[u8;0]} #[repr(C)] pub struct mutex{_private:[u8;0]} #[repr(C)] pub struct local64_t{_private:[u8;0]} #[repr(C)] pub struct local_t{_private:[u8;0]} #[repr(C)] pub struct atomic_t{_private:[u8;0]} #[repr(C)] pub struct refcount_t{_private:[u8;0]} #[repr(C)] pub struct rcu_head{_private:[u8;0]} #[repr(C)] pub struct rb_root{_private:[u8;0]} #[repr(C)] pub struct module{_private:[u8;0]} #[repr(C)] pub struct device{_private:[u8;0]} #[repr(C)] pub struct attribute_group{_private:[u8;0]} #[repr(C)] pub struct mm_struct{_private:[u8;0]} #[repr(C)] pub struct kmem_cache{_private:[u8;0]} #[repr(C)] pub struct path{_private:[u8;0]}
extern "C" { fn list_empty(x:*const list_head)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
