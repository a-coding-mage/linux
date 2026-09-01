// SPDX-License-Identifier: GPL-2.0

// Depends on Linux eBPF definitions from <linux/bpf.h> and
// <bpf/bpf_helpers.h>, including SEC map/program placement metadata.

use core::ffi::c_void;

/* Permit pretty deep stack traces */
const MAX_STACK_RAWTP: usize = 100;

#[repr(C)]
pub struct stack_trace_t {
    pub pid: i32,
    pub kern_stack_size: i32,
    pub user_stack_size: i32,
    pub user_stack_buildid_size: i32,
    pub kern_stack: [u64; MAX_STACK_RAWTP],
    pub user_stack: [u64; MAX_STACK_RAWTP],
    pub user_stack_buildid: [bpf_stack_build_id; MAX_STACK_RAWTP],
}

// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
// __uint(max_entries, 2);
// __uint(key_size, sizeof(int));
// __uint(value_size, sizeof(__u32));
#[repr(C)]
pub struct perfmap {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut perfmap: perfmap = perfmap { _private: [] };

// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(max_entries, 1);
// __type(key, __u32);
// __type(value, struct stack_trace_t);
#[repr(C)]
pub struct stackdata_map {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut stackdata_map: stackdata_map = stackdata_map { _private: [] };

/* Allocate per-cpu space twice the needed. For the code below
 *   usize = bpf_get_stack(ctx, raw_data, max_len, BPF_F_USER_STACK);
 *   if (usize < 0)
 *     return 0;
 *   ksize = bpf_get_stack(ctx, raw_data + usize, max_len - usize, 0);
 *
 * If we have value_size = MAX_STACK_RAWTP * sizeof(__u64),
 * verifier will complain that access "raw_data + usize"
 * with size "max_len - usize" may be out of bound.
 * The maximum "raw_data + usize" is "raw_data + max_len"
 * and the maximum "max_len - usize" is "max_len", verifier
 * concludes that the maximum buffer access range is
 * "raw_data[0...max_len * 2 - 1]" and hence reject the program.
 *
 * Doubling the to-be-used max buffer size can fix this verifier
 * issue and avoid complicated C programming massaging.
 * This is an acceptable workaround since there is one entry here.
 */
// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(max_entries, 1);
// __type(key, __u32);
// __type(value, __u64[2 * MAX_STACK_RAWTP]);
#[repr(C)]
pub struct rawdata_map {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut rawdata_map: rawdata_map = rawdata_map { _private: [] };

extern "C" {
    pub type bpf_stack_build_id;

    static BPF_F_USER_STACK: u64;
    static BPF_F_USER_BUILD_ID: u64;

    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_stack(ctx: *mut c_void, buf: *mut c_void, size: u32, flags: u64) -> i64;
    fn bpf_perf_event_output(
        ctx: *mut c_void,
        map: *mut c_void,
        flags: u64,
        data: *mut c_void,
        size: u64,
    ) -> i64;
}

// SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut c_void) -> i32 {
    let max_len: i32;
    let max_buildid_len: i32;
    let total_size: i32;
    let data: *mut stack_trace_t;
    let usize: i64;
    let ksize: i64;
    let raw_data: *mut c_void;
    let key: u32 = 0;

    data = bpf_map_lookup_elem(
        &mut stackdata_map as *mut stackdata_map as *mut c_void,
        &key as *const u32 as *const c_void,
    ) as *mut stack_trace_t;
    if data.is_null() {
        return 0;
    }

    max_len = (MAX_STACK_RAWTP * core::mem::size_of::<u64>()) as i32;
    max_buildid_len =
        (MAX_STACK_RAWTP * core::mem::size_of::<bpf_stack_build_id>()) as i32;
    (*data).pid = bpf_get_current_pid_tgid() as i32;
    (*data).kern_stack_size = bpf_get_stack(
        ctx,
        (*data).kern_stack.as_mut_ptr() as *mut c_void,
        max_len as u32,
        0,
    ) as i32;
    (*data).user_stack_size = bpf_get_stack(
        ctx,
        (*data).user_stack.as_mut_ptr() as *mut c_void,
        max_len as u32,
        BPF_F_USER_STACK,
    ) as i32;
    (*data).user_stack_buildid_size = bpf_get_stack(
        ctx,
        (*data).user_stack_buildid.as_mut_ptr() as *mut c_void,
        max_buildid_len as u32,
        BPF_F_USER_STACK | BPF_F_USER_BUILD_ID,
    ) as i32;
    bpf_perf_event_output(
        ctx,
        &mut perfmap as *mut perfmap as *mut c_void,
        0,
        data as *mut c_void,
        core::mem::size_of_val(&*data) as u64,
    );

    /* write both kernel and user stacks to the same buffer */
    raw_data = bpf_map_lookup_elem(
        &mut rawdata_map as *mut rawdata_map as *mut c_void,
        &key as *const u32 as *const c_void,
    );
    if raw_data.is_null() {
        return 0;
    }

    usize = bpf_get_stack(ctx, raw_data, max_len as u32, BPF_F_USER_STACK);
    if usize < 0 {
        return 0;
    }

    ksize = bpf_get_stack(
        ctx,
        (raw_data as *mut u8).add(usize as usize) as *mut c_void,
        (max_len as i64 - usize) as u32,
        0,
    );
    if ksize < 0 {
        return 0;
    }

    total_size = (usize + ksize) as i32;
    if total_size > 0 && total_size <= max_len {
        bpf_perf_event_output(
            ctx,
            &mut perfmap as *mut perfmap as *mut c_void,
            0,
            raw_data,
            total_size as u64,
        );
    }

    return 0;
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
