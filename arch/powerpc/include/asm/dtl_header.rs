// Layout of entries in the hypervisor's dispatch trace log buffer.
#[repr(C)]
pub struct dtl_entry {
    pub dispatch_reason: u8,
    pub preempt_reason: u8,
    pub processor_id: __be16,
    pub enqueue_to_dispatch_time: __be32,
    pub ready_to_enqueue_time: __be32,
    pub waiting_to_ready_time: __be32,
    pub timebase: __be64,
    pub fault_addr: __be64,
    pub srr0: __be64,
    pub srr1: __be64,
}

pub const DISPATCH_LOG_BYTES: usize = 4096; // bytes per cpu
pub const N_DISPATCH_LOG: usize = DISPATCH_LOG_BYTES / core::mem::size_of::<dtl_entry>();

// Dispatch trace log event enable mask:
//   0x1: voluntary virtual processor waits
//   0x2: time-slice preempts
//   0x4: virtual partition memory page faults
pub const DTL_LOG_CEDE: u32 = 0x1;
pub const DTL_LOG_PREEMPT: u32 = 0x2;
pub const DTL_LOG_FAULT: u32 = 0x4;
pub const DTL_LOG_ALL: u32 = DTL_LOG_CEDE | DTL_LOG_PREEMPT | DTL_LOG_FAULT;

extern "C" {
    pub static mut dtl_cache: *mut kmem_cache;
    pub static mut dtl_access_lock: rw_semaphore;

    pub fn register_dtl_buffer(cpu: core::ffi::c_int);
    pub fn alloc_dtl_buffers(time_limit: *mut c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
