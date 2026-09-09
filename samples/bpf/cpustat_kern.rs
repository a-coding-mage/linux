// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the eBPF build environment:
// linux/version.h, linux/ptrace.h, uapi/linux/bpf.h, bpf/bpf_helpers.h

const MAX_CPU: u32 = 8;
const MAX_PSTATE_ENTRIES: u32 = 5;
const MAX_CSTATE_ENTRIES: u32 = 3;

static mut CPU_OPPS: [i32; 5] = [208000, 432000, 729000, 960000, 1200000];

const MAP_OFF_CSTATE_TIME: u32 = 0;
const MAP_OFF_CSTATE_IDX: u32 = 1;
const MAP_OFF_PSTATE_TIME: u32 = 2;
const MAP_OFF_PSTATE_IDX: u32 = 3;
const MAP_OFF_NUM: u32 = 4;

#[repr(C)]
pub struct BpfMap {
    _private: [u8; 0],
}

// BPF_MAP_TYPE_ARRAY maps, with max_entries as in the original declarations.
#[link_section = ".maps"]
#[no_mangle]
pub static mut my_map: BpfMap = BpfMap { _private: [] };

#[link_section = ".maps"]
#[no_mangle]
pub static mut cstate_duration: BpfMap = BpfMap { _private: [] };

#[link_section = ".maps"]
#[no_mangle]
pub static mut pstate_duration: BpfMap = BpfMap { _private: [] };

#[repr(C)]
pub struct cpu_args {
    pub pad: u64,
    pub state: u32,
    pub cpu_id: u32,
}

extern "C" {
    fn bpf_map_lookup_elem(map: *mut BpfMap, key: *const u32) -> *mut u64;
    fn bpf_ktime_get_ns() -> u64;
}

unsafe fn find_cpu_pstate_idx(frequency: u32) -> u32 {
    let mut i: u32 = 0;

    while i < (core::mem::size_of::<[i32; 5]>() / core::mem::size_of::<u32>()) as u32 {
        if frequency == CPU_OPPS[i as usize] as u32 {
            return i;
        }
        i += 1;
    }

    i
}

#[no_mangle]
#[link_section = "tracepoint/power/cpu_idle"]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut cpu_args) -> i32 {
    let (mut cts, mut pts, mut cstate, mut pstate): (*mut u64, *mut u64, *mut u64, *mut u64);
    let (mut prev_state, mut cur_ts, mut delta): (u64, u64, u64);
    let (mut key, cpu, mut pstate_idx): (u32, u32, u32);
    let mut val: *mut u64;

    if (*ctx).cpu_id > MAX_CPU { return 0; }
    cpu = (*ctx).cpu_id;

    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_TIME;
    cts = bpf_map_lookup_elem(&mut my_map, &key);
    if cts.is_null() { return 0; }
    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_IDX;
    cstate = bpf_map_lookup_elem(&mut my_map, &key);
    if cstate.is_null() { return 0; }
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_TIME;
    pts = bpf_map_lookup_elem(&mut my_map, &key);
    if pts.is_null() { return 0; }
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_IDX;
    pstate = bpf_map_lookup_elem(&mut my_map, &key);
    if pstate.is_null() { return 0; }

    prev_state = *cstate;
    *cstate = (*ctx).state as u64;
    if *cts == 0 { *cts = bpf_ktime_get_ns(); return 0; }

    cur_ts = bpf_ktime_get_ns();
    delta = cur_ts.wrapping_sub(*cts);
    *cts = cur_ts;

    if (*ctx).state != u32::MAX {
        if *pts == 0 { return 0; }
        delta = cur_ts.wrapping_sub(*pts);
        pstate_idx = find_cpu_pstate_idx(*pstate as u32);
        if pstate_idx >= MAX_PSTATE_ENTRIES { return 0; }
        key = cpu * MAX_PSTATE_ENTRIES + pstate_idx;
        val = bpf_map_lookup_elem(&mut pstate_duration, &key);
        if !val.is_null() { *val = (*val).wrapping_add(delta); }
    } else {
        key = cpu * MAX_CSTATE_ENTRIES + prev_state as u32;
        val = bpf_map_lookup_elem(&mut cstate_duration, &key);
        if !val.is_null() { *val = (*val).wrapping_add(delta); }
    }

    if *pts != 0 { *pts = cur_ts; }
    0
}

#[no_mangle]
#[link_section = "tracepoint/power/cpu_frequency"]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut cpu_args) -> i32 {
    let (mut pts, mut cstate, mut pstate, mut cur_ts, mut delta): (*mut u64, *mut u64, *mut u64, u64, u64);
    let (mut key, cpu, mut pstate_idx): (u32, u32, u32);
    let mut val: *mut u64;

    cpu = (*ctx).cpu_id;
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_TIME;
    pts = bpf_map_lookup_elem(&mut my_map, &key);
    if pts.is_null() { return 0; }
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_IDX;
    pstate = bpf_map_lookup_elem(&mut my_map, &key);
    if pstate.is_null() { return 0; }
    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_IDX;
    cstate = bpf_map_lookup_elem(&mut my_map, &key);
    if cstate.is_null() { return 0; }

    *pstate = (*ctx).state as u64;
    if *pts == 0 { *pts = bpf_ktime_get_ns(); return 0; }
    cur_ts = bpf_ktime_get_ns();
    delta = cur_ts.wrapping_sub(*pts);
    *pts = cur_ts;
    if *cstate != u32::MAX as u64 { return 0; }

    pstate_idx = find_cpu_pstate_idx(*pstate as u32);
    if pstate_idx >= MAX_PSTATE_ENTRIES { return 0; }
    key = cpu * MAX_PSTATE_ENTRIES + pstate_idx;
    val = bpf_map_lookup_elem(&mut pstate_duration, &key);
    if !val.is_null() { *val = (*val).wrapping_add(delta); }
    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "version"]
#[no_mangle]
pub static mut _version: u32 = 0; // LINUX_VERSION_CODE supplied by linux/version.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
