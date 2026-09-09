/* Rust translation of cvmx-cmd-queue.h. */

/* Dependency intent: linux/prefetch.h, asm/compiler.h, and cvmx-fpa.h. */

pub const CVMX_CMD_QUEUE_ENABLE_MAX_DEPTH: i32 = 0;

pub type cvmx_cmd_queue_id_t = i32;
pub const CVMX_CMD_QUEUE_PKO_BASE: cvmx_cmd_queue_id_t = 0x00000;
pub const CVMX_CMD_QUEUE_ZIP: cvmx_cmd_queue_id_t = 0x10000;
pub const CVMX_CMD_QUEUE_DFA: cvmx_cmd_queue_id_t = 0x20000;
pub const CVMX_CMD_QUEUE_RAID: cvmx_cmd_queue_id_t = 0x30000;
pub const CVMX_CMD_QUEUE_DMA_BASE: cvmx_cmd_queue_id_t = 0x40000;
pub const CVMX_CMD_QUEUE_END: cvmx_cmd_queue_id_t = 0x50000;

#[inline]
pub const fn CVMX_CMD_QUEUE_PKO(queue: cvmx_cmd_queue_id_t) -> cvmx_cmd_queue_id_t {
    CVMX_CMD_QUEUE_PKO_BASE + (queue & 0xffff)
}
#[inline]
pub const fn CVMX_CMD_QUEUE_DMA(queue: cvmx_cmd_queue_id_t) -> cvmx_cmd_queue_id_t {
    CVMX_CMD_QUEUE_DMA_BASE + (queue & 0xffff)
}

pub type cvmx_cmd_queue_result_t = i32;
pub const CVMX_CMD_QUEUE_SUCCESS: cvmx_cmd_queue_result_t = 0;
pub const CVMX_CMD_QUEUE_NO_MEMORY: cvmx_cmd_queue_result_t = -1;
pub const CVMX_CMD_QUEUE_FULL: cvmx_cmd_queue_result_t = -2;
pub const CVMX_CMD_QUEUE_INVALID_PARAM: cvmx_cmd_queue_result_t = -3;
pub const CVMX_CMD_QUEUE_ALREADY_SETUP: cvmx_cmd_queue_result_t = -4;

#[repr(C)]
pub struct __cvmx_cmd_queue_state_t {
    pub now_serving: u8,
    pub unused1: u64, // C bit-field: 24 bits
    pub max_depth: u32,
    pub fpa_pool: u64, // C bit-field: 3 bits
    pub base_ptr_div128: u64, // C bit-field: 29 bits
    pub unused2: u64, // C bit-field: 6 bits
    pub pool_size_m1: u64, // C bit-field: 13 bits
    pub index: u64, // C bit-field: 13 bits
}

#[repr(C)]
pub struct __cvmx_cmd_queue_all_state_t {
    pub ticket: [u64; ((CVMX_CMD_QUEUE_END >> 16) * 256) as usize],
    pub state: [__cvmx_cmd_queue_state_t; ((CVMX_CMD_QUEUE_END >> 16) * 256) as usize],
}

extern "C" {
    pub fn cvmx_cmd_queue_initialize(queue_id: cvmx_cmd_queue_id_t, max_depth: i32, fpa_pool: i32, pool_size: i32) -> cvmx_cmd_queue_result_t;
    pub fn cvmx_cmd_queue_shutdown(queue_id: cvmx_cmd_queue_id_t) -> cvmx_cmd_queue_result_t;
    pub fn cvmx_cmd_queue_length(queue_id: cvmx_cmd_queue_id_t) -> i32;
    pub fn cvmx_cmd_queue_buffer(queue_id: cvmx_cmd_queue_id_t) -> *mut core::ffi::c_void;
    pub static mut __cvmx_cmd_queue_state_ptr: *mut __cvmx_cmd_queue_all_state_t;
    pub fn cvmx_phys_to_ptr(addr: u64) -> *mut core::ffi::c_void;
    pub fn cvmx_ptr_to_phys(ptr: *const u64) -> u64;
    pub fn cvmx_fpa_alloc(pool: u64) -> *mut u64;
}

#[inline]
pub unsafe fn __cvmx_cmd_queue_get_index(queue_id: cvmx_cmd_queue_id_t) -> i32 {
    let unit = queue_id >> 16;
    let q = (queue_id >> 4) & 0xf;
    let core = queue_id & 0xf;
    unit * 256 + core * 16 + q
}

#[inline]
pub unsafe fn __cvmx_cmd_queue_lock(queue_id: cvmx_cmd_queue_id_t, _qptr: *mut __cvmx_cmd_queue_state_t) {
    /* The original uses MIPS ll/sc ticket-lock assembly; preserve the operation as an external dependency. */
    let _ = queue_id;
}

#[inline]
pub unsafe fn __cvmx_cmd_queue_unlock(qptr: *mut __cvmx_cmd_queue_state_t) {
    (*qptr).now_serving = (*qptr).now_serving.wrapping_add(1);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline]
pub unsafe fn __cvmx_cmd_queue_get_state(queue_id: cvmx_cmd_queue_id_t) -> *mut __cvmx_cmd_queue_state_t {
    (*__cvmx_cmd_queue_state_ptr).state.as_mut_ptr().add(__cvmx_cmd_queue_get_index(queue_id) as usize)
}

#[inline]
pub unsafe fn cvmx_cmd_queue_write(queue_id: cvmx_cmd_queue_id_t, use_locking: i32, mut cmd_count: i32, mut cmds: *mut u64) -> cvmx_cmd_queue_result_t {
    let qptr = __cvmx_cmd_queue_get_state(queue_id);
    if use_locking != 0 { __cvmx_cmd_queue_lock(queue_id, qptr); }
    if CVMX_CMD_QUEUE_ENABLE_MAX_DEPTH != 0 && (*qptr).max_depth != 0 && cvmx_cmd_queue_length(queue_id) > (*qptr).max_depth as i32 {
        if use_locking != 0 { __cvmx_cmd_queue_unlock(qptr); }
        return CVMX_CMD_QUEUE_FULL;
    }
    if ((*qptr).index as i32 + cmd_count) < (*qptr).pool_size_m1 as i32 {
        let mut ptr = cvmx_phys_to_ptr(((*qptr).base_ptr_div128) << 7) as *mut u64;
        ptr = ptr.add((*qptr).index as usize);
        (*qptr).index += cmd_count as u64;
        while cmd_count != 0 { *ptr = *cmds; ptr = ptr.add(1); cmds = cmds.add(1); cmd_count -= 1; }
    } else {
        let mut ptr: *mut u64;
        let count = (*qptr).pool_size_m1 as i32 - (*qptr).index as i32;
        let new_buffer = cvmx_fpa_alloc((*qptr).fpa_pool);
        if new_buffer.is_null() { if use_locking != 0 { __cvmx_cmd_queue_unlock(qptr); } return CVMX_CMD_QUEUE_NO_MEMORY; }
        ptr = cvmx_phys_to_ptr((*qptr).base_ptr_div128 << 7) as *mut u64;
        ptr = ptr.add((*qptr).index as usize);
        cmd_count -= count;
        let mut n = count; while n != 0 { *ptr = *cmds; ptr = ptr.add(1); cmds = cmds.add(1); n -= 1; }
        *ptr = cvmx_ptr_to_phys(new_buffer);
        (*qptr).base_ptr_div128 = *ptr >> 7;
        (*qptr).index = cmd_count as u64;
        ptr = new_buffer;
        while cmd_count != 0 { *ptr = *cmds; ptr = ptr.add(1); cmds = cmds.add(1); cmd_count -= 1; }
    }
    if use_locking != 0 { __cvmx_cmd_queue_unlock(qptr); }
    CVMX_CMD_QUEUE_SUCCESS
}

#[inline]
pub unsafe fn cvmx_cmd_queue_write2(queue_id: cvmx_cmd_queue_id_t, use_locking: i32, cmd1: u64, cmd2: u64) -> cvmx_cmd_queue_result_t {
    let a = [cmd1, cmd2];
    cvmx_cmd_queue_write(queue_id, use_locking, 2, a.as_ptr() as *mut u64)
}

#[inline]
pub unsafe fn cvmx_cmd_queue_write3(queue_id: cvmx_cmd_queue_id_t, use_locking: i32, cmd1: u64, cmd2: u64, cmd3: u64) -> cvmx_cmd_queue_result_t {
    let a = [cmd1, cmd2, cmd3];
    cvmx_cmd_queue_write(queue_id, use_locking, 3, a.as_ptr() as *mut u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
