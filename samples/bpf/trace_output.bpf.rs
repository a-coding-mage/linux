// Translated from trace_output.bpf.c.
// Dependency declarations and section placement are supplied by the eBPF build environment.

#[repr(C)]
pub struct MyMap {
    _private: [u8; 0],
}

// __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
// __uint(key_size, sizeof(int));
// __uint(value_size, sizeof(u32));
// __uint(max_entries, 2);
// SEC(".maps")
#[no_mangle]
pub static mut my_map: MyMap = MyMap { _private: [] };

#[repr(C)]
pub struct S {
    pub pid: u64,
    pub cookie: u64,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_perf_event_output(
        ctx: *mut pt_regs,
        map: *mut MyMap,
        flags: u64,
        data: *const core::ffi::c_void,
        size: u64,
    ) -> i64;
}

// SEC("ksyscall/write")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    let mut data: S;

    data = S {
        pid: bpf_get_current_pid_tgid(),
        cookie: 0x1234_5678,
    };

    bpf_perf_event_output(
        ctx,
        &raw mut my_map,
        0,
        (&data as *const S).cast::<core::ffi::c_void>(),
        core::mem::size_of::<S>() as u64,
    );

    0
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC("version")
// LINUX_VERSION_CODE is supplied by the Linux/eBPF build environment.
#[no_mangle]
pub static _version: u32 = LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
