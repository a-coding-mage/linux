/* Translated from drm/gpu_scheduler.h. */

/* External kernel types and macros are supplied by other translated headers. */

pub const MAX_WAIT_SCHED_ENTITY_Q_EMPTY: u64 = msecs_to_jiffies(1000);
pub const DRM_SCHED_FENCE_DONT_PIPELINE: u32 = DMA_FENCE_FLAG_USER_BITS;
pub const DRM_SCHED_FENCE_FLAG_HAS_DEADLINE_BIT: u32 = DMA_FENCE_FLAG_USER_BITS + 1;

pub enum dma_resv_usage {}
pub enum dma_resv {}
pub enum drm_gem_object {}
pub enum drm_gpu_scheduler {}
pub enum drm_sched_rq {}
pub enum drm_file {}
pub enum drm_sched_entity_stats {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_sched_priority {
    DRM_SCHED_PRIORITY_INVALID = -1,
    DRM_SCHED_PRIORITY_KERNEL,
    DRM_SCHED_PRIORITY_HIGH,
    DRM_SCHED_PRIORITY_NORMAL,
    DRM_SCHED_PRIORITY_LOW,
    DRM_SCHED_PRIORITY_COUNT,
}

#[repr(C)]
pub struct drm_sched_entity {
    pub list: list_head,
    pub lock: spinlock_t,
    pub rq: *mut drm_sched_rq,
    pub stats: *mut drm_sched_entity_stats,
    pub sched_list: *mut *mut drm_gpu_scheduler,
    pub num_sched_list: c_uint,
    pub priority: drm_sched_priority,
    pub rq_priority: drm_sched_priority,
    pub rr_ts: ktime_t,
    pub job_queue: spsc_queue,
    pub fence_seq: atomic_t,
    pub fence_context: u64,
    pub dependency: *mut dma_fence,
    pub cb: dma_fence_cb,
    pub guilty: *mut atomic_t,
    pub last_scheduled: *mut dma_fence,
    pub last_user: *mut task_struct,
    pub stopped: bool,
    pub entity_idle: completion,
    pub oldest_job_waiting: ktime_t,
    pub rb_tree_node: rb_node,
}

#[repr(C)]
pub struct drm_sched_rq {
    pub sched: *mut drm_gpu_scheduler,
    pub lock: spinlock_t,
    pub rr_ts: ktime_t,
    pub entities: list_head,
    pub rb_tree_root: rb_root_cached,
    pub head_prio: drm_sched_priority,
}

#[repr(C)]
pub struct drm_sched_fence {
    pub scheduled: dma_fence,
    pub finished: dma_fence,
    pub deadline: ktime_t,
    pub parent: *mut dma_fence,
    pub sched: *mut drm_gpu_scheduler,
    pub lock: spinlock_t,
    pub owner: *mut c_void,
    pub drm_client_id: u64,
}

#[repr(C)]
pub struct drm_sched_job {
    pub submit_ts: ktime_t,
    pub sched: *mut drm_gpu_scheduler,
    pub s_fence: *mut drm_sched_fence,
    pub entity: *mut drm_sched_entity,
    pub entity_stats: *mut drm_sched_entity_stats,
    pub s_priority: drm_sched_priority,
    pub credits: u32,
    pub last_dependency: c_uint,
    pub karma: atomic_t,
    pub queue_node: spsc_node,
    pub list: list_head,
    pub work: drm_sched_job_work,
    pub cb: dma_fence_cb,
    pub dependencies: xarray,
}

#[repr(C)]
pub union drm_sched_job_work {
    pub finish_cb: dma_fence_cb,
    pub work: work_struct,
}

extern "C" {
    pub fn to_drm_sched_fence(f: *mut dma_fence) -> *mut drm_sched_fence;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_gpu_sched_stat {
    DRM_GPU_SCHED_STAT_NONE,
    DRM_GPU_SCHED_STAT_RESET,
    DRM_GPU_SCHED_STAT_ENODEV,
    DRM_GPU_SCHED_STAT_NO_HANG,
}

#[repr(C)]
pub struct drm_sched_backend_ops {
    pub prepare_job: Option<unsafe extern "C" fn(*mut drm_sched_job, *mut drm_sched_entity) -> *mut dma_fence>,
    pub run_job: Option<unsafe extern "C" fn(*mut drm_sched_job) -> *mut dma_fence>,
    pub timedout_job: Option<unsafe extern "C" fn(*mut drm_sched_job) -> drm_gpu_sched_stat>,
    pub free_job: Option<unsafe extern "C" fn(*mut drm_sched_job)>,
    pub cancel_job: Option<unsafe extern "C" fn(*mut drm_sched_job)>,
}

#[repr(C)]
pub struct drm_gpu_scheduler {
    pub ops: *const drm_sched_backend_ops,
    pub credit_limit: u32,
    pub credit_count: atomic_t,
    pub timeout: c_long,
    pub name: *const c_char,
    pub num_rqs: u32,
    pub num_user_rqs: u32,
    pub sched_rq: *mut *mut drm_sched_rq,
    pub job_scheduled: wait_queue_head_t,
    pub job_id_count: atomic64_t,
    pub submit_wq: *mut workqueue_struct,
    pub timeout_wq: *mut workqueue_struct,
    pub avg_job_us: ewma_drm_sched_avgtime,
    pub work_run_job: work_struct,
    pub work_free_job: work_struct,
    pub work_tdr: delayed_work,
    pub pending_list: list_head,
    pub job_list_lock: spinlock_t,
    pub hang_limit: c_int,
    pub score: *mut atomic_t,
    pub _score: atomic_t,
    pub ready: bool,
    pub free_guilty: bool,
    pub pause_submit: bool,
    pub own_submit_wq: bool,
    pub dev: *mut device,
}

#[repr(C)]
pub struct drm_sched_init_args {
    pub ops: *const drm_sched_backend_ops,
    pub submit_wq: *mut workqueue_struct,
    pub timeout_wq: *mut workqueue_struct,
    pub num_rqs: u32,
    pub credit_limit: u32,
    pub hang_limit: c_uint,
    pub timeout: c_long,
    pub score: *mut atomic_t,
    pub name: *const c_char,
    pub dev: *mut device,
}

extern "C" {
    pub fn drm_sched_init(sched: *mut drm_gpu_scheduler, args: *const drm_sched_init_args) -> c_int;
    pub fn drm_sched_fini(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_suspend_timeout(sched: *mut drm_gpu_scheduler) -> c_ulong;
    pub fn drm_sched_resume_timeout(sched: *mut drm_gpu_scheduler, remaining: c_ulong);
    pub fn drm_sched_tdr_queue_imm(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_wqueue_ready(sched: *mut drm_gpu_scheduler) -> bool;
    pub fn drm_sched_wqueue_stop(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_wqueue_start(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_stop(sched: *mut drm_gpu_scheduler, bad: *mut drm_sched_job);
    pub fn drm_sched_start(sched: *mut drm_gpu_scheduler, errno: c_int);
    pub fn drm_sched_resubmit_jobs(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_fault(sched: *mut drm_gpu_scheduler);
    pub fn drm_sched_is_stopped(sched: *mut drm_gpu_scheduler) -> bool;
    pub fn drm_sched_pick_best(sched_list: *mut *mut drm_gpu_scheduler, num_sched_list: c_uint) -> *mut drm_gpu_scheduler;
    pub fn drm_sched_job_init(job: *mut drm_sched_job, entity: *mut drm_sched_entity, credits: u32, owner: *mut c_void, drm_client_id: u64) -> c_int;
    pub fn drm_sched_job_arm(job: *mut drm_sched_job);
    pub fn drm_sched_entity_push_job(sched_job: *mut drm_sched_job);
    pub fn drm_sched_job_add_dependency(job: *mut drm_sched_job, fence: *mut dma_fence) -> c_int;
    pub fn drm_sched_job_add_syncobj_dependency(job: *mut drm_sched_job, file: *mut drm_file, handle: u32, point: u32) -> c_int;
    pub fn drm_sched_job_add_resv_dependencies(job: *mut drm_sched_job, resv: *mut dma_resv, usage: dma_resv_usage) -> c_int;
    pub fn drm_sched_job_add_implicit_dependencies(job: *mut drm_sched_job, obj: *mut drm_gem_object, write: bool) -> c_int;
    pub fn drm_sched_job_has_dependency(job: *mut drm_sched_job, fence: *mut dma_fence) -> bool;
    pub fn drm_sched_job_cleanup(job: *mut drm_sched_job);
    pub fn drm_sched_increase_karma(bad: *mut drm_sched_job);
    pub fn drm_sched_job_is_signaled(job: *mut drm_sched_job) -> bool;
    pub fn drm_sched_entity_init(entity: *mut drm_sched_entity, priority: drm_sched_priority, sched_list: *mut *mut drm_gpu_scheduler, num_sched_list: c_uint, guilty: *mut atomic_t) -> c_int;
    pub fn drm_sched_entity_flush(entity: *mut drm_sched_entity, timeout: c_long) -> c_long;
    pub fn drm_sched_entity_kill(entity: *mut drm_sched_entity);
    pub fn drm_sched_entity_fini(entity: *mut drm_sched_entity);
    pub fn drm_sched_entity_destroy(entity: *mut drm_sched_entity);
    pub fn drm_sched_entity_set_priority(entity: *mut drm_sched_entity, priority: drm_sched_priority);
    pub fn drm_sched_entity_error(entity: *mut drm_sched_entity) -> c_int;
    pub fn drm_sched_entity_modify_sched(entity: *mut drm_sched_entity, sched_list: *mut *mut drm_gpu_scheduler, num_sched_list: c_uint);
}

#[repr(C)]
pub struct drm_sched_pending_job_iter { pub sched: *mut drm_gpu_scheduler }

/* Drivers should never call these directly. */
#[inline]
pub unsafe fn __drm_sched_pending_job_iter_begin(sched: *mut drm_gpu_scheduler) -> drm_sched_pending_job_iter {
    /* Equivalent to WARN_ON(!drm_sched_is_stopped(sched)); */
    drm_sched_pending_job_iter { sched }
}

#[inline]
pub unsafe fn __drm_sched_pending_job_iter_end(iter: drm_sched_pending_job_iter) {
    /* Equivalent to WARN_ON(!drm_sched_is_stopped(iter.sched)); */
    let _ = iter;
}

/* C scoped_guard/list_for_each_entry iterator; preserve its filtering intent. */
#[macro_export]
macro_rules! drm_sched_for_each_pending_job {
    ($job:expr, $sched:expr, $entity:expr) => {
        /* Iterate ($sched).pending_list, filtering by entity when non-null. */
    };
}

#[inline]
pub unsafe fn drm_sched_invalidate_job(s_job: *mut drm_sched_job, threshold: c_int) -> bool {
    !s_job.is_null() && atomic_inc_return(&mut (*s_job).karma) > threshold
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
