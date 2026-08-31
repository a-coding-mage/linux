// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type pid_t = i32;
type off_t = i64;
type ssize_t = isize;
type size_t = usize;

// Dependencies from the original C includes are provided by surrounding perf code.

const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const ULLONG_MAX: u64 = u64::MAX;

extern "C" {
    static page_size: size_t;
    static mut errno: c_int;
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pkey: *mut c_void,
    pvalue: *mut c_void,
}

#[repr(C)]
pub struct machine {
    pid: pid_t,
    env: *mut perf_env,
    priv_: *mut c_void,
    rb_node: rb_node,
}

#[repr(C)]
pub struct machines {
    host: machine,
    guests: rb_root_cached,
}

#[repr(C)]
pub struct maps {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct map {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct dso {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct thread {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct perf_data {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    data: *mut perf_data,
}

#[repr(C)]
pub struct perf_pmu {
    name: *const c_char,
}

#[repr(C)]
pub struct perf_event_attr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period_or_freq: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
    wakeup_events_or_watermark: u32,
    bp_type: u32,
    bp_addr: u64,
    bp_len: u64,
    branch_sample_type: u64,
    sample_regs_user: u64,
    sample_stack_user: u32,
    clockid: c_int,
    sample_regs_intr: u64,
    aux_watermark: u32,
    sample_max_stack: u16,
    __reserved_2: u16,
    aux_sample_size: u32,
    __reserved_3: u32,
    sig_data: u64,
    config1: u64,
    config2: u64,
}

#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    core: evsel_core,
    sample_size: c_int,
}

#[repr(C)]
pub struct evlist {
    __opaque: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    type_: u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
pub struct perf_record_mmap {
    header: perf_event_header,
    pid: u32,
    tid: u32,
    start: u64,
    len: u64,
    pgoff: u64,
    filename: [c_char; 0],
}

#[repr(C)]
pub struct perf_record_mmap2 {
    header: perf_event_header,
    pid: u32,
    tid: u32,
    start: u64,
    len: u64,
    pgoff: u64,
    maj: u32,
    min: u32,
    ino: u64,
    ino_generation: u64,
    prot: u32,
    flags: u32,
    filename: [c_char; 0],
}

#[repr(C)]
pub struct perf_record_ksymbol {
    header: perf_event_header,
    addr: u64,
    len: u32,
    ksym_type: u16,
    flags: u16,
    name: [c_char; 0],
}

#[repr(C)]
pub struct perf_record_auxtrace {
    header: perf_event_header,
    size: u64,
}

#[repr(C)]
pub struct perf_record_attr {
    header: perf_event_header,
    attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_record_sample {
    header: perf_event_header,
    array: [u64; 0],
}

#[repr(C)]
pub union perf_event {
    header: perf_event_header,
    mmap: perf_record_mmap,
    mmap2: perf_record_mmap2,
    ksymbol: perf_record_ksymbol,
    auxtrace: perf_record_auxtrace,
    attr: perf_record_attr,
    sample: perf_record_sample,
}

#[repr(C)]
pub struct perf_sample {
    evsel: *mut evsel,
    cpumode: u8,
    pid: u32,
    tid: u32,
    ip: u64,
    addr: u64,
    raw_size: u32,
}

#[repr(C)]
pub struct addr_location {
    map: *mut map,
}

#[repr(C)]
pub struct perf_tool {
    ordered_events: bool,
    sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    mmap: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    mmap2: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    comm: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    fork: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    exit: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    ksymbol: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    text_poke: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    auxtrace: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> s64>,
    auxtrace_info: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    auxtrace_error: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
}

#[repr(C)]
pub struct delegate_tool {
    tool: perf_tool,
    delegate: *mut perf_tool,
}

#[repr(C)]
struct remap_addresses_key {
    machine: *mut machine,
    dso: *mut dso,
    invariant: u64,
    pid: pid_t,
}

#[repr(C)]
struct aslr_mapping {
    node: list_head,
    orig_start: u64,
    len: u64,
    remap_start: u64,
}

#[repr(C)]
struct aslr_evsel_priv {
    orig_sample_type: u64,
    orig_sample_regs_user: u64,
    orig_sample_regs_intr: u64,
    orig_sample_size: c_int,
}

#[repr(C)]
struct process_top_address {
    remapped_max: u64,
}

#[repr(C, align(8))]
struct AlignedEventCopy([c_char; PERF_SAMPLE_MAX_SIZE]);

#[repr(C)]
struct aslr_tool {
    /// @tool: The tool implemented here and a pointer to a delegate to process the data.
    tool: delegate_tool,
    /// @machines: The machines with the input, not remapped, virtual address layout.
    machines: machines,
    /// @event_copy: Buffer used to create an event to pass to the delegate.
    event_copy: AlignedEventCopy,
    /// @remap_addresses: mapping from remap_addresses_key to remapped address.
    remap_addresses: hashmap,
    /// @top_addresses: mapping from process to max remapped address.
    top_addresses: hashmap,
    /// @evsel_orig_attrs: mapping from evsel pointer to its original
    ///                    unstripped sample_type and registers bitmasks.
    evsel_orig_attrs: hashmap,
}

#[repr(C)]
struct top_addresses_key {
    machine: *mut machine,
    pid: pid_t,
}

#[repr(C)]
struct aslr_machine_priv {
    kernel_maps_loaded: bool,
}

const kernel_pid: pid_t = -1;
const user_space_start: u64 = 0x200000;
const kernel_space_start_64: u64 = 0xffff800010000000;
const kernel_space_start_32: u64 = 0x80000000;

const PERF_SAMPLE_MAX_SIZE: usize = 65536;
const PERF_RECORD_MISC_CPUMODE_MASK: u16 = 7;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_USER: u8 = 2;
const PERF_RECORD_MISC_HYPERVISOR: u8 = 3;
const PERF_RECORD_MISC_GUEST_KERNEL: u8 = 4;
const PERF_RECORD_MISC_GUEST_USER: u8 = 5;
const PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER: u16 = 1;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_ID: u64 = 1 << 6;
const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_READ: u64 = 1 << 4;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_RAW: u64 = 1 << 10;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_WEIGHT_TYPE: u64 = 1 << 14;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;
const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
const PERF_SAMPLE_CGROUP: u64 = 1 << 21;
const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 1 << 22;
const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 1 << 23;
const PERF_SAMPLE_AUX: u64 = 1 << 24;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_LOST: u64 = 1 << 4;
const PERF_SAMPLE_BRANCH_HW_INDEX: u64 = 1 << 17;
const PERF_SAMPLE_BRANCH_COUNTERS: u64 = 1 << 23;
const PERF_SAMPLE_REGS_ABI_NONE: u64 = 0;
const PERF_CONTEXT_MAX: u64 = !4095u64;
const PERF_CONTEXT_HV: u64 = !32u64;
const PERF_CONTEXT_KERNEL: u64 = !128u64;
const PERF_CONTEXT_USER: u64 = !512u64;
const PERF_CONTEXT_GUEST: u64 = !2048u64;
const PERF_CONTEXT_GUEST_KERNEL: u64 = !2176u64;
const PERF_CONTEXT_GUEST_USER: u64 = !2304u64;
const PERF_CONTEXT_USER_DEFERRED: u64 = !2400u64;
const PERF_ATTR_SIZE_VER0: u32 = 64;
const PERF_TYPE_BREAKPOINT: u32 = 5;
const PERF_TYPE_MAX: u32 = 6;
const ASLR_SUPPORTED_SAMPLE_TYPE: u64 = !(PERF_SAMPLE_REGS_USER
    | PERF_SAMPLE_REGS_INTR
    | PERF_SAMPLE_RAW
    | PERF_SAMPLE_STACK_USER
    | PERF_SAMPLE_PHYS_ADDR
    | PERF_SAMPLE_AUX);
const HASHMAP_ADD: c_int = 0;

extern "C" {
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> bool;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__put(thread: *mut thread);
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn maps__load_maps(maps: *mut maps) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__size(map: *mut map) -> size_t;
    fn map__pgoff(map: *mut map) -> u64;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn is_anon_memory(name: *const c_char) -> bool;
    fn is_no_dso_memory(name: *const c_char) -> bool;
    fn is_kernel_module(name: *const c_char, cpumode: u8) -> bool;
    fn hashmap__init(map: *mut hashmap, hash: unsafe extern "C" fn(c_long, *mut c_void) -> size_t, equal: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool, ctx: *mut c_void);
    fn hashmap__find(map: *const hashmap, key: *const c_void, value: *mut *mut c_void) -> bool;
    fn hashmap__insert(map: *mut hashmap, key: *mut c_void, value: *mut c_void, action: c_int, old_key: *mut *mut c_void, old_value: *mut *mut c_void) -> c_int;
    fn hashmap__add(map: *mut hashmap, key: *mut c_void, value: *mut c_void) -> c_int;
    fn hashmap__clear(map: *mut hashmap);
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn machines__init(machines: *mut machines) -> c_int;
    fn machines__findnew(machines: *mut machines, pid: pid_t) -> *mut machine;
    fn machines__destroy_kernel_maps(machines: *mut machines);
    fn machines__exit(machines: *mut machines);
    fn machine__delete(machine: *mut machine);
    fn machine__findnew_thread(machine: *mut machine, pid: u32, tid: u32) -> *mut thread;
    fn perf_event__process_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_ksymbol(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> bool;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_pmus__find_by_type(type_: u32) -> *mut perf_pmu;
    fn evsel__is_dummy_event(evsel: *mut evsel) -> bool;
    fn evsel__is_offcpu_event(evsel: *mut evsel) -> bool;
    fn __evsel__sample_size(sample_type: u64) -> c_int;
    fn __evsel__parse_sample(evsel: *mut evsel, event: *mut perf_event, sample: *mut perf_sample, needs_swap: bool) -> c_int;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evsel__calc_id_pos(evsel: *mut evsel);
    fn hweight64(w: u64) -> u64;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
}

unsafe fn container_of<T, U>(ptr: *const U, offset: usize) -> *mut T {
    (ptr as *const u8).sub(offset) as *mut T
}

unsafe fn zfree(ptrp: *mut *mut c_void) {
    if !(*ptrp).is_null() {
        free(*ptrp);
        *ptrp = ptr::null_mut();
    }
}

unsafe fn rb_entry_machine(node: *mut rb_node) -> *mut machine {
    container_of::<machine, rb_node>(node, offset_of!(machine, rb_node))
}

unsafe extern "C" fn evsel_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn evsel_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    key1 == key2
}

unsafe extern "C" fn remap_addresses__hash(_key: c_long, _ctx: *mut c_void) -> size_t {
    let key = _key as *mut remap_addresses_key;
    let dso_ptr = if !(*key).dso.is_null() {
        (*key).dso as *mut c_void
    } else {
        ptr::null_mut()
    };

    ((*key).machine as size_t) ^ (dso_ptr as size_t) ^ ((*key).invariant as size_t) ^ ((*key).pid as size_t)
}

unsafe extern "C" fn remap_addresses__equal(_key1: c_long, _key2: c_long, _ctx: *mut c_void) -> bool {
    let key1 = _key1 as *mut remap_addresses_key;
    let key2 = _key2 as *mut remap_addresses_key;

    (*key1).machine == (*key2).machine
        && (*key1).dso == (*key2).dso
        && (*key1).invariant == (*key2).invariant
        && (*key1).pid == (*key2).pid
}

unsafe extern "C" fn top_addresses__hash(_key: c_long, _ctx: *mut c_void) -> size_t {
    let key = _key as *mut top_addresses_key;
    ((*key).machine as size_t) ^ ((*key).pid as size_t)
}

unsafe extern "C" fn top_addresses__equal(_key1: c_long, _key2: c_long, _ctx: *mut c_void) -> bool {
    let key1 = _key1 as *mut top_addresses_key;
    let key2 = _key2 as *mut top_addresses_key;

    (*key1).machine == (*key2).machine && (*key1).pid == (*key2).pid
}

unsafe fn round_up_to_page_size(addr: u64) -> u64 {
    (addr + page_size as u64 - 1) & !((page_size as u64) - 1)
}

unsafe fn aslr_tool__remap_address(aslr: *mut aslr_tool, aslr_thread: *mut thread, cpumode: u8, addr: u64) -> u64 {
    let mut al: addr_location = core::mem::zeroed();
    let mut key: remap_addresses_key = core::mem::zeroed();
    let mut remapped_invariant_ptr: *mut c_void = ptr::null_mut();
    let mut remap_addr: u64 = 0;
    let mut effective_cpumode = cpumode;
    let dso: *mut dso;
    let dso_name: *const c_char;

    if aslr_thread.is_null() {
        return 0;
    }

    addr_location__init(&mut al);
    if !thread__find_map(aslr_thread, cpumode, addr, &mut al) {
        if cpumode == PERF_RECORD_MISC_KERNEL {
            effective_cpumode = PERF_RECORD_MISC_USER;
        } else if cpumode == PERF_RECORD_MISC_USER {
            effective_cpumode = PERF_RECORD_MISC_KERNEL;
        } else if cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
            effective_cpumode = PERF_RECORD_MISC_GUEST_USER;
        } else if cpumode == PERF_RECORD_MISC_GUEST_USER {
            effective_cpumode = PERF_RECORD_MISC_GUEST_KERNEL;
        }

        if !thread__find_map(aslr_thread, effective_cpumode, addr, &mut al) {
            addr_location__exit(&mut al);
            return 0;
        }
    }

    dso = map__dso(al.map);
    dso_name = if !dso.is_null() { dso__long_name(dso) } else { ptr::null() };

    key.machine = maps__machine(thread__maps(aslr_thread));
    key.dso = dso;
    if !dso.is_null() && !is_anon_memory(dso_name) && !is_no_dso_memory(dso_name) {
        key.invariant = map__start(al.map) - map__pgoff(al.map);
    } else {
        key.invariant = map__start(al.map);
    }
    key.pid = if effective_cpumode == PERF_RECORD_MISC_KERNEL || effective_cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
        kernel_pid
    } else {
        thread__pid(aslr_thread)
    };

    if hashmap__find(&(*aslr).remap_addresses, &key as *const _ as *const c_void, &mut remapped_invariant_ptr) {
        remap_addr = *(remapped_invariant_ptr as *mut u64) + map__pgoff(al.map) + (addr - map__start(al.map));
    } else {
        pr_debug(c"Cannot find a remapped entry for address %lx in mapping %lx(%zu) for pid=%d\n".as_ptr(), addr, map__start(al.map), map__size(al.map), key.pid);
    }

    addr_location__exit(&mut al);
    remap_addr
}

unsafe fn aslr_tool__preload_kernel_maps(machine: *mut machine) -> c_int {
    let mut mpriv = (*machine).priv_ as *mut aslr_machine_priv;

    if mpriv.is_null() {
        mpriv = zalloc(size_of::<aslr_machine_priv>()) as *mut aslr_machine_priv;
        if mpriv.is_null() {
            return -ENOMEM;
        }
        (*machine).priv_ = mpriv as *mut c_void;
    }

    if !(*mpriv).kernel_maps_loaded {
        let kmaps = machine__kernel_maps(machine);

        if !kmaps.is_null() {
            let err = maps__load_maps(kmaps);

            if err < 0 {
                pr_err(c"ASLR: Failed to preload kernel maps for machine pid %d\n".as_ptr(), (*machine).pid);
                return err;
            }
        }
        (*mpriv).kernel_maps_loaded = true;
    }
    0
}

unsafe fn aslr_tool__free_machine_priv(machine: *mut machine) {
    free((*machine).priv_);
    (*machine).priv_ = ptr::null_mut();
}

unsafe fn aslr_tool__destroy_machines_priv(machines: *mut machines) {
    let mut nd: *mut rb_node;

    aslr_tool__free_machine_priv(&mut (*machines).host);
    nd = rb_first_cached(&(*machines).guests);
    while !nd.is_null() {
        let machine = rb_entry_machine(nd);

        aslr_tool__free_machine_priv(machine);
        nd = rb_next(nd);
    }
}

unsafe fn aslr_tool__findnew_mapping(
    aslr: *mut aslr_tool,
    session_machine: *mut machine,
    aslr_thread: *mut thread,
    cpumode: u8,
    start: u64,
    len: u64,
    pgoff: u64,
) -> u64 {
    let mut al: addr_location = core::mem::zeroed();
    let mut remap_key: remap_addresses_key = core::mem::zeroed();
    let mut remapped_invariant_ptr: *mut c_void = ptr::null_mut();
    let mut top_addr_key: top_addresses_key = core::mem::zeroed();
    let mut top: *mut c_void = ptr::null_mut();
    let mut remap_addr: u64;
    let mut new_remap_key: *mut remap_addresses_key = ptr::null_mut();
    let mut new_remap_val: *mut u64 = ptr::null_mut();
    let mut err: c_int = 0;

    if aslr_thread.is_null() {
        return 0;
    }

    addr_location__init(&mut al);
    remap_key.machine = maps__machine(thread__maps(aslr_thread));
    remap_key.pid = if cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
        kernel_pid
    } else {
        thread__pid(aslr_thread)
    };
    if thread__find_map(aslr_thread, cpumode, start, &mut al) {
        let dso = map__dso(al.map);
        let dso_name = if !dso.is_null() { dso__long_name(dso) } else { ptr::null() };

        remap_key.dso = dso;
        if !dso.is_null() && !is_anon_memory(dso_name) && !is_no_dso_memory(dso_name) {
            remap_key.invariant = map__start(al.map) - map__pgoff(al.map);
        } else {
            remap_key.invariant = map__start(al.map);
        }
    } else {
        remap_key.dso = ptr::null_mut();
        remap_key.invariant = start;
    }

    top_addr_key.machine = remap_key.machine;
    top_addr_key.pid = remap_key.pid;

    if hashmap__find(&(*aslr).remap_addresses, &remap_key as *const _ as *const c_void, &mut remapped_invariant_ptr) {
        let calculated_max: u64;

        if !al.map.is_null() {
            remap_addr = *(remapped_invariant_ptr as *mut u64) + map__pgoff(al.map) + (start - map__start(al.map));
        } else {
            remap_addr = *(remapped_invariant_ptr as *mut u64) + pgoff;
        }

        calculated_max = remap_addr + len;

        if hashmap__find(&(*aslr).top_addresses, &top_addr_key as *const _ as *const c_void, &mut top) {
            let top = top as *mut process_top_address;
            if calculated_max > (*top).remapped_max {
                (*top).remapped_max = calculated_max;
            }
        }
        addr_location__exit(&mut al);
        return remap_addr;
    }

    if hashmap__find(&(*aslr).top_addresses, &top_addr_key as *const _ as *const c_void, &mut top) {
        let mut prev_al: addr_location = core::mem::zeroed();
        let mut is_contiguous = false;
        let top = top as *mut process_top_address;

        remap_addr = (*top).remapped_max;

        addr_location__init(&mut prev_al);
        if thread__find_map(aslr_thread, cpumode, start - 1, &mut prev_al) {
            if map__end(prev_al.map) == start {
                is_contiguous = true;
            }
        }
        addr_location__exit(&mut prev_al);

        if is_contiguous {
            remap_addr = round_up_to_page_size(remap_addr);
        } else {
            remap_addr = round_up_to_page_size(remap_addr);
            remap_addr += page_size as u64;
        }
        if remap_addr + len > (*top).remapped_max {
            (*top).remapped_max = remap_addr + len;
        }
    } else {
        let tk: *mut top_addresses_key;
        let top_val: *mut process_top_address;
        let env = if !session_machine.is_null() { (*session_machine).env } else { ptr::null_mut() };
        let is_64 = if !env.is_null() { perf_env__kernel_is_64_bit(env) } else { size_of::<*mut c_void>() == 8 };
        let kernel_start_addr = if is_64 { kernel_space_start_64 } else { kernel_space_start_32 };

        remap_addr = if cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
            kernel_start_addr
        } else {
            user_space_start
        };
        remap_addr = round_up_to_page_size(remap_addr);

        tk = malloc(size_of::<top_addresses_key>()) as *mut top_addresses_key;
        top_val = malloc(size_of::<process_top_address>()) as *mut process_top_address;
        if tk.is_null() || top_val.is_null() {
            err = -ENOMEM;
        } else {
            *tk = top_addr_key;
            (*top_val).remapped_max = remap_addr + len;
            err = hashmap__insert(&mut (*aslr).top_addresses, tk as *mut c_void, top_val as *mut c_void, HASHMAP_ADD, ptr::null_mut(), ptr::null_mut());
        }
        if err != 0 {
            errno = -err;
            pr_err(c"Failure to add ASLR process top address %m\n".as_ptr());
            free(tk as *mut c_void);
            free(top_val as *mut c_void);
            addr_location__exit(&mut al);
            return 0;
        }
    }

    new_remap_key = malloc(size_of::<remap_addresses_key>()) as *mut remap_addresses_key;
    new_remap_val = malloc(size_of::<u64>()) as *mut u64;
    if new_remap_key.is_null() || new_remap_val.is_null() {
        err = -ENOMEM;
    } else {
        *new_remap_key = remap_key;
        (*new_remap_key).dso = dso__get(remap_key.dso);
        if cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
            if !al.map.is_null() {
                *new_remap_val = remap_addr - (start - map__start(al.map)) - map__pgoff(al.map);
            } else {
                *new_remap_val = remap_addr - pgoff;
            }
        } else {
            *new_remap_val = remap_addr
                - if !al.map.is_null() {
                    (start - map__start(al.map)) + map__pgoff(al.map)
                } else {
                    pgoff
                };
        }
        err = hashmap__add(&mut (*aslr).remap_addresses, new_remap_key as *mut c_void, new_remap_val as *mut c_void);
        if err != 0 {
            dso__put((*new_remap_key).dso);
        }
    }
    if err != 0 {
        errno = -err;
        pr_err(c"Failure to add ASLR remapping %m\n".as_ptr());
        free(new_remap_key as *mut c_void);
        free(new_remap_val as *mut c_void);
        addr_location__exit(&mut al);
        return 0;
    }
    addr_location__exit(&mut al);
    remap_addr
}

unsafe extern "C" fn aslr_tool__process_mmap(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let new_event = (*aslr).event_copy.0.as_mut_ptr() as *mut perf_event;
    let cpumode = ((*event).header.misc & PERF_RECORD_MISC_CPUMODE_MASK) as u8;
    let aslr_machine: *mut machine;
    let thread: *mut thread;
    let mut err: c_int;

    aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }

    err = perf_event__process_mmap(tool, event, sample, aslr_machine);
    if err != 0 {
        return err;
    }

    thread = machine__findnew_thread(aslr_machine, (*event).mmap.pid, (*event).mmap.tid);
    if thread.is_null() {
        return -ENOMEM;
    }
    memcpy(&mut (*new_event).mmap as *mut _ as *mut c_void, &(*event).mmap as *const _ as *const c_void, (*event).mmap.header.size as size_t);
    (*new_event).mmap.start = aslr_tool__findnew_mapping(aslr, machine, thread, cpumode, (*event).mmap.start, (*event).mmap.len, (*event).mmap.pgoff);
    if is_anon_memory((*event).mmap.filename.as_ptr())
        || is_no_dso_memory((*event).mmap.filename.as_ptr())
        || ((cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL)
            && !is_kernel_module((*event).mmap.filename.as_ptr(), cpumode))
    {
        (*new_event).mmap.pgoff = (*new_event).mmap.start;
    }
    err = ((*delegate).mmap.unwrap())(delegate, new_event, sample, machine);
    thread__put(thread);
    err
}

unsafe extern "C" fn aslr_tool__process_mmap2(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let new_event = (*aslr).event_copy.0.as_mut_ptr() as *mut perf_event;
    let cpumode = ((*event).header.misc & PERF_RECORD_MISC_CPUMODE_MASK) as u8;
    let aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    let thread: *mut thread;
    let mut err: c_int;

    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }

    err = perf_event__process_mmap2(tool, event, sample, aslr_machine);
    if err != 0 {
        return err;
    }

    thread = machine__findnew_thread(aslr_machine, (*event).mmap2.pid, (*event).mmap2.tid);
    if thread.is_null() {
        return -ENOMEM;
    }
    memcpy(&mut (*new_event).mmap2 as *mut _ as *mut c_void, &(*event).mmap2 as *const _ as *const c_void, (*event).mmap2.header.size as size_t);
    (*new_event).mmap2.start = aslr_tool__findnew_mapping(aslr, machine, thread, cpumode, (*event).mmap2.start, (*event).mmap2.len, (*event).mmap2.pgoff);
    if is_anon_memory((*event).mmap2.filename.as_ptr())
        || is_no_dso_memory((*event).mmap2.filename.as_ptr())
        || ((cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL)
            && !is_kernel_module((*event).mmap2.filename.as_ptr(), cpumode))
    {
        (*new_event).mmap2.pgoff = (*new_event).mmap2.start;
    }
    err = ((*delegate).mmap2.unwrap())(delegate, new_event, sample, machine);
    thread__put(thread);
    err
}

unsafe extern "C" fn aslr_tool__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    let err: c_int;

    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }
    err = perf_event__process_comm(tool, event, sample, aslr_machine);
    if err != 0 {
        return err;
    }
    ((*delegate).comm.unwrap())(delegate, event, sample, machine)
}

unsafe extern "C" fn aslr_tool__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    let err: c_int;

    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }
    err = perf_event__process_fork(tool, event, sample, aslr_machine);
    if err != 0 {
        return err;
    }
    ((*delegate).fork.unwrap())(delegate, event, sample, machine)
}

unsafe extern "C" fn aslr_tool__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    let err: c_int;

    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }
    err = perf_event__process_exit(tool, event, sample, aslr_machine);
    if err != 0 {
        return err;
    }
    ((*delegate).exit.unwrap())(delegate, event, sample, machine)
}

unsafe extern "C" fn aslr_tool__process_text_poke(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    /* Drop in case the instruction encodes an ASLR revealing address. */
    0
}

unsafe extern "C" fn aslr_tool__process_ksymbol(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let new_event = (*aslr).event_copy.0.as_mut_ptr() as *mut perf_event;
    let aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    let thread: *mut thread;
    let is_unregister: bool;
    let mut err: c_int;

    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }

    thread = machine__findnew_thread(aslr_machine, kernel_pid as u32, 0);
    if thread.is_null() {
        return -ENOMEM;
    }

    is_unregister = ((*event).ksymbol.flags & PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER) != 0;
    memcpy(&mut (*new_event).ksymbol as *mut _ as *mut c_void, &(*event).ksymbol as *const _ as *const c_void, (*event).ksymbol.header.size as size_t);

    if is_unregister {
        (*new_event).ksymbol.addr = aslr_tool__findnew_mapping(aslr, machine, thread, PERF_RECORD_MISC_KERNEL, (*event).ksymbol.addr, (*event).ksymbol.len as u64, 0);
        err = perf_event__process_ksymbol(tool, event, sample, aslr_machine);
    } else {
        err = perf_event__process_ksymbol(tool, event, sample, aslr_machine);
        (*new_event).ksymbol.addr = aslr_tool__findnew_mapping(aslr, machine, thread, PERF_RECORD_MISC_KERNEL, (*event).ksymbol.addr, (*event).ksymbol.len as u64, 0);
    }
    if err != 0 {
        thread__put(thread);
        return err;
    }

    err = ((*delegate).ksymbol.unwrap())(delegate, new_event, sample, machine);
    thread__put(thread);
    err
}

unsafe fn check_bounds(i: size_t, j: size_t, required_i: size_t, required_j: size_t, max_i: u64, max_j: u64) -> bool {
    i as u64 + required_i as u64 > max_i || j as u64 + required_j as u64 > max_j
}

unsafe fn copy_u64(in_array: *mut u64, out_array: *mut u64, i: &mut size_t, j: &mut size_t, max_i: u64, max_j: u64) -> c_int {
    if check_bounds(*i, *j, 1, 1, max_i, max_j) {
        return -EFAULT;
    }
    *out_array.add(*j) = *in_array.add(*i);
    *j += 1;
    *i += 1;
    0
}

unsafe extern "C" fn aslr_tool__process_sample(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let evsel = (*sample).evsel;
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let delegate = (*aslr).tool.delegate;
    let mut ret: c_int;
    let orig_sample_size: c_int;
    let mut sample_type: u64;
    let thread: *mut thread;
    let aslr_machine: *mut machine;
    let max_i: u64;
    let max_j: u64;
    let new_event = (*aslr).event_copy.0.as_mut_ptr() as *mut perf_event;
    let mut new_sample: perf_sample = core::mem::zeroed();
    let in_array: *mut u64;
    let out_array: *mut u64;
    let mut cpumode: u8;
    let mut addr: u64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut priv_ptr: *mut c_void = ptr::null_mut();
    let orig_sample_type: u64;
    let orig_regs_user: u64;
    let orig_regs_intr: u64;

    if evsel__is_dummy_event(evsel) {
        return ((*delegate).sample.unwrap())(delegate, event, sample, machine);
    }

    ret = -EFAULT;

    if hashmap__find(&(*aslr).evsel_orig_attrs, evsel as *const c_void, &mut priv_ptr) {
        let priv_ = priv_ptr as *mut aslr_evsel_priv;
        orig_sample_type = (*priv_).orig_sample_type;
        orig_regs_user = (*priv_).orig_sample_regs_user;
        orig_regs_intr = (*priv_).orig_sample_regs_intr;
    } else {
        orig_sample_type = (*evsel).core.attr.sample_type;
        orig_regs_user = (*evsel).core.attr.sample_regs_user;
        orig_regs_intr = (*evsel).core.attr.sample_regs_intr;
    }

    orig_sample_size = (*evsel).sample_size;

    sample_type = orig_sample_type;
    sample_type &= !PERF_SAMPLE_REGS_USER;
    sample_type &= !PERF_SAMPLE_REGS_INTR;
    sample_type &= ASLR_SUPPORTED_SAMPLE_TYPE;

    max_i = ((*event).header.size as usize - size_of::<perf_event_header>()) as u64 / size_of::<u64>() as u64;
    max_j = (PERF_SAMPLE_MAX_SIZE - size_of::<perf_event_header>()) as u64 / size_of::<u64>() as u64;
    cpumode = (*sample).cpumode;

    aslr_machine = machines__findnew(&mut (*aslr).machines, (*machine).pid);
    if aslr_machine.is_null() {
        return -ENOMEM;
    }
    if aslr_tool__preload_kernel_maps(aslr_machine) < 0 {
        return -ENOMEM;
    }

    thread = machine__findnew_thread(aslr_machine, (*sample).pid, (*sample).tid);
    if thread.is_null() {
        return -ENOMEM;
    }

    if max_i > (PERF_SAMPLE_MAX_SIZE / size_of::<u64>()) as u64 {
        goto_out_put(thread, ret);
        return ret;
    }

    (*new_event).sample.header = (*event).sample.header;
    in_array = (*event).sample.array.as_mut_ptr();
    out_array = (*new_event).sample.array.as_mut_ptr();

    macro_rules! copy_or_out {
        () => {{
            ret = copy_u64(in_array, out_array, &mut i, &mut j, max_i, max_j);
            if ret != 0 {
                thread__put(thread);
                return ret;
            }
        }};
    }
    macro_rules! remap_or_out {
        ($addr_field:expr) => {{
            if check_bounds(i, j, 1, 1, max_i, max_j) {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            let remapped = aslr_tool__remap_address(aslr, thread, cpumode, $addr_field);
            *out_array.add(j) = remapped;
            j += 1;
            i += 1;
        }};
    }

    if (orig_sample_type & PERF_SAMPLE_IDENTIFIER) != 0 {
        copy_or_out!();
    }
    if (orig_sample_type & PERF_SAMPLE_IP) != 0 {
        remap_or_out!((*sample).ip);
    }
    if (orig_sample_type & PERF_SAMPLE_TID) != 0 {
        #[repr(C)]
        union U {
            val64: u64,
            val32: [u32; 2],
        }
        let mut u = U { val64: 0 };
        if check_bounds(i, j, 1, 1, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        u.val32[0] = (*sample).pid;
        u.val32[1] = (*sample).tid;
        *out_array.add(j) = u.val64;
        j += 1;
        i += 1;
    }
    if (orig_sample_type & PERF_SAMPLE_TIME) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_ADDR) != 0 { remap_or_out!((*sample).addr); }
    if (orig_sample_type & PERF_SAMPLE_ID) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_STREAM_ID) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_CPU) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_PERIOD) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_READ) != 0 {
        if ((*evsel).core.attr.read_format & PERF_FORMAT_GROUP) == 0 {
            copy_or_out!();
            if ((*evsel).core.attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED) != 0 { copy_or_out!(); }
            if ((*evsel).core.attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING) != 0 { copy_or_out!(); }
            if ((*evsel).core.attr.read_format & PERF_FORMAT_ID) != 0 { copy_or_out!(); }
            if ((*evsel).core.attr.read_format & PERF_FORMAT_LOST) != 0 { copy_or_out!(); }
        } else {
            let nr: u64;
            if check_bounds(i, j, 1, 1, max_i, max_j) {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            nr = *in_array.add(i);
            copy_or_out!();
            if ((*evsel).core.attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED) != 0 { copy_or_out!(); }
            if ((*evsel).core.attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING) != 0 { copy_or_out!(); }
            let mut cntr = 0;
            while cntr < nr {
                copy_or_out!();
                if ((*evsel).core.attr.read_format & PERF_FORMAT_ID) != 0 { copy_or_out!(); }
                if ((*evsel).core.attr.read_format & PERF_FORMAT_LOST) != 0 { copy_or_out!(); }
                cntr += 1;
            }
        }
    }
    if (orig_sample_type & PERF_SAMPLE_CALLCHAIN) != 0 {
        let nr: u64;

        if check_bounds(i, j, 1, 1, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        nr = *in_array.add(i);
        copy_or_out!();

        let mut cntr = 0;
        while cntr < nr {
            if check_bounds(i, j, 1, 1, max_i, max_j) {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            addr = *in_array.add(i);
            i += 1;
            if addr >= PERF_CONTEXT_MAX {
                *out_array.add(j) = addr;
                j += 1;
                match addr {
                    PERF_CONTEXT_HV => cpumode = PERF_RECORD_MISC_HYPERVISOR,
                    PERF_CONTEXT_KERNEL => cpumode = PERF_RECORD_MISC_KERNEL,
                    PERF_CONTEXT_USER => cpumode = PERF_RECORD_MISC_USER,
                    PERF_CONTEXT_GUEST => cpumode = PERF_RECORD_MISC_GUEST_KERNEL,
                    PERF_CONTEXT_GUEST_KERNEL => cpumode = PERF_RECORD_MISC_GUEST_KERNEL,
                    PERF_CONTEXT_GUEST_USER => cpumode = PERF_RECORD_MISC_GUEST_USER,
                    PERF_CONTEXT_USER_DEFERRED => {
                        if cntr + 1 >= nr {
                            pr_debug(c"Truncated callchain deferred cookie context\n".as_ptr());
                            ret = 0;
                            thread__put(thread);
                            return ret;
                        }
                        if check_bounds(i, j, 1, 1, max_i, max_j) {
                            ret = -EFAULT;
                            thread__put(thread);
                            return ret;
                        }
                        *out_array.add(j) = *in_array.add(i);
                        j += 1;
                        i += 1;
                        cntr += 1;
                        cpumode = PERF_RECORD_MISC_USER;
                    }
                    _ => {
                        pr_debug(c"invalid callchain context: %lx\n".as_ptr(), addr);
                        ret = 0;
                        thread__put(thread);
                        return ret;
                    }
                }
                cntr += 1;
                continue;
            }
            addr = aslr_tool__remap_address(aslr, thread, cpumode, addr);
            *out_array.add(j) = addr;
            j += 1;
            cntr += 1;
        }
    }
    if (orig_sample_type & PERF_SAMPLE_RAW) != 0 {
        let bytes = size_of::<u32>() + (*sample).raw_size as size_t;
        let u64_words = (bytes + 7) / 8;
        if i + u64_words > max_i as usize || j + u64_words > max_j as usize {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        memcpy(out_array.add(j) as *mut c_void, in_array.add(i) as *const c_void, bytes);
        pr_debug(c"Dropping raw samples as possible ASLR leak\n".as_ptr());
        ret = 0;
        thread__put(thread);
        return ret;
    }
    if (orig_sample_type & PERF_SAMPLE_BRANCH_STACK) != 0 {
        let nr: u64;
        if check_bounds(i, j, 1, 1, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        nr = *in_array.add(i);
        copy_or_out!();
        if ((*evsel).core.attr.branch_sample_type & PERF_SAMPLE_BRANCH_HW_INDEX) != 0 { copy_or_out!(); }
        if nr > ULLONG_MAX / 3 {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        if nr * 3 > max_i - i as u64 || nr * 3 > max_j - j as u64 {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        let mut cntr = 0;
        while cntr < nr {
            let mut from = *in_array.add(i);
            i += 1;
            let mut to = *in_array.add(i);
            i += 1;
            from = aslr_tool__remap_address(aslr, thread, (*sample).cpumode, from);
            to = aslr_tool__remap_address(aslr, thread, (*sample).cpumode, to);
            *out_array.add(j) = from;
            j += 1;
            *out_array.add(j) = to;
            j += 1;
            *out_array.add(j) = *in_array.add(i);
            j += 1;
            i += 1;
            cntr += 1;
        }
        if ((*evsel).core.attr.branch_sample_type & PERF_SAMPLE_BRANCH_COUNTERS) != 0 {
            if nr > max_i - i as u64 || nr > max_j - j as u64 {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            let mut cntr = 0;
            while cntr < nr {
                copy_or_out!();
                cntr += 1;
            }
        }
    }
    if (orig_sample_type & PERF_SAMPLE_REGS_USER) != 0 {
        let abi: u64;
        if check_bounds(i, j, 1, 0, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        abi = *in_array.add(i);
        i += 1;
        if abi != PERF_SAMPLE_REGS_ABI_NONE {
            let nr = hweight64(orig_regs_user);
            if nr > max_i - i as u64 {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            i += nr as usize;
        }
    }
    if (orig_sample_type & PERF_SAMPLE_STACK_USER) != 0 {
        let size: u64;
        if check_bounds(i, j, 1, 1, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        size = *in_array.add(i);
        copy_or_out!();
        if size > 0 {
            let u64_words = size as size_t / 8 + if size % 8 != 0 { 1 } else { 0 };
            if u64_words > max_i as usize - i || u64_words > max_j as usize - j {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            memcpy(out_array.add(j) as *mut c_void, in_array.add(i) as *const c_void, size as size_t);
            if size % 8 != 0 {
                let pad = 8 - (size % 8);
                memset((out_array.add(j) as *mut c_char).add(size as usize) as *mut c_void, 0, pad as size_t);
            }
        }
        pr_debug(c"Dropping stack user sample as possible ASLR leak\n".as_ptr());
        ret = 0;
        thread__put(thread);
        return ret;
    }
    if (orig_sample_type & PERF_SAMPLE_WEIGHT_TYPE) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_DATA_SRC) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_TRANSACTION) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_REGS_INTR) != 0 {
        let abi: u64;
        if check_bounds(i, j, 1, 0, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        abi = *in_array.add(i);
        i += 1;
        if abi != PERF_SAMPLE_REGS_ABI_NONE {
            let nr = hweight64(orig_regs_intr);
            if nr > max_i - i as u64 {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            i += nr as usize;
        }
    }
    if (orig_sample_type & PERF_SAMPLE_PHYS_ADDR) != 0 {
        copy_or_out!();
        pr_debug(c"Dropping physical address sample as possible ASLR leak\n".as_ptr());
        ret = 0;
        thread__put(thread);
        return ret;
    }
    if (orig_sample_type & PERF_SAMPLE_CGROUP) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_DATA_PAGE_SIZE) != 0 { copy_or_out!(); }
    if (orig_sample_type & PERF_SAMPLE_CODE_PAGE_SIZE) != 0 { copy_or_out!(); }

    if (orig_sample_type & PERF_SAMPLE_AUX) != 0 {
        let size: u64;
        if check_bounds(i, j, 1, 1, max_i, max_j) {
            ret = -EFAULT;
            thread__put(thread);
            return ret;
        }
        *out_array.add(j) = *in_array.add(i);
        size = *out_array.add(j);
        j += 1;
        i += 1;
        if size > 0 {
            let u64_words = size as size_t / 8 + if size % 8 != 0 { 1 } else { 0 };
            if u64_words > max_i as usize - i || u64_words > max_j as usize - j {
                ret = -EFAULT;
                thread__put(thread);
                return ret;
            }
            memcpy(out_array.add(j) as *mut c_void, in_array.add(i) as *const c_void, size as size_t);
            if size % 8 != 0 {
                let pad = 8 - (size % 8);
                memset((out_array.add(j) as *mut c_char).add(size as usize) as *mut c_void, 0, pad as size_t);
            }
        }
        pr_debug(c"Dropping aux sample as possible ASLR leak\n".as_ptr());
        ret = 0;
        thread__put(thread);
        return ret;
    }

    if evsel__is_offcpu_event(evsel) {
        pr_debug(c"Dropping off-CPU sample as possible ASLR leak\n".as_ptr());
        ret = 0;
        thread__put(thread);
        return ret;
    }

    (*new_event).sample.header.size = (size_of::<perf_event_header>() + j * size_of::<u64>()) as u16;
    (*evsel).sample_size = __evsel__sample_size(sample_type);
    (*evsel).core.attr.sample_type = sample_type;
    (*evsel).core.attr.sample_regs_user = 0;
    (*evsel).core.attr.sample_regs_intr = 0;
    perf_sample__init(&mut new_sample, true);
    ret = __evsel__parse_sample(evsel, new_event, &mut new_sample, false);

    if ret != 0 {
        (*evsel).sample_size = orig_sample_size;
        (*evsel).core.attr.sample_type = orig_sample_type;
        (*evsel).core.attr.sample_regs_user = orig_regs_user;
        (*evsel).core.attr.sample_regs_intr = orig_regs_intr;
        perf_sample__exit(&mut new_sample);
        thread__put(thread);
        return ret;
    }

    new_sample.evsel = evsel;
    ret = ((*delegate).sample.unwrap())(delegate, new_event, &mut new_sample, machine);
    perf_sample__exit(&mut new_sample);

    (*evsel).sample_size = orig_sample_size;
    (*evsel).core.attr.sample_type = orig_sample_type;
    (*evsel).core.attr.sample_regs_user = orig_regs_user;
    (*evsel).core.attr.sample_regs_intr = orig_regs_intr;

    thread__put(thread);
    ret
}

unsafe fn goto_out_put(thread: *mut thread, _ret: c_int) {
    thread__put(thread);
}

unsafe fn skipn(fd: c_int, mut n: off_t) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut ret: ssize_t;

    while n > 0 {
        let count = if n < size_of_val(&buf) as off_t { n } else { size_of_val(&buf) as off_t };
        ret = read(fd, buf.as_mut_ptr() as *mut c_void, count as size_t);
        if ret <= 0 {
            return ret as c_int;
        }
        n -= ret as off_t;
    }

    0
}

unsafe extern "C" fn aslr_tool__process_auxtrace(_tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> s64 {
    pr_warning_once(c"ASLR: Dropping auxtrace data as it cannot be obfuscated.\n".as_ptr());
    if perf_data__is_pipe((*session).data) {
        let err = skipn(perf_data__fd((*session).data), (*event).auxtrace.size as off_t);

        if err < 0 {
            return err as s64;
        }
    }
    (*event).auxtrace.size as s64
}

unsafe extern "C" fn aslr_tool__process_auxtrace_info(_tool: *const perf_tool, _session: *mut perf_session, _event: *mut perf_event) -> c_int {
    0
}

unsafe extern "C" fn aslr_tool__process_auxtrace_error(_tool: *const perf_tool, _session: *mut perf_session, _event: *mut perf_event) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__strip_attr_event(event: *mut perf_event, evlist: *mut evlist) {
    let attr_size: u32;

    if evlist.is_null() {
        return;
    }

    attr_size = if (*event).attr.attr.size != 0 { (*event).attr.attr.size } else { PERF_ATTR_SIZE_VER0 };

    if attr_size as usize >= offset_of!(perf_event_attr, sample_type) + size_of::<u64>() {
        (*event).attr.attr.sample_type &= ASLR_SUPPORTED_SAMPLE_TYPE;

        if attr_size as usize >= offset_of!(perf_event_attr, sample_regs_user) + size_of::<u64>() {
            (*event).attr.attr.sample_regs_user = 0;
        }
        if attr_size as usize >= offset_of!(perf_event_attr, sample_regs_intr) + size_of::<u64>() {
            (*event).attr.attr.sample_regs_intr = 0;
        }
    }

    if attr_size as usize >= offset_of!(perf_event_attr, type_) + size_of::<u32>() {
        let type_ = (*event).attr.attr.type_;

        if type_ == PERF_TYPE_BREAKPOINT
            && attr_size as usize >= offset_of!(perf_event_attr, bp_addr) + size_of::<u64>()
        {
            (*event).attr.attr.bp_addr = 0;
        } else if type_ >= PERF_TYPE_MAX {
            let pmu = perf_pmus__find_by_type(type_);
            if !pmu.is_null() && (strcmp((*pmu).name, c"kprobe".as_ptr()) == 0 || strcmp((*pmu).name, c"uprobe".as_ptr()) == 0) {
                if attr_size as usize >= offset_of!(perf_event_attr, config1) + size_of::<u64>() {
                    (*event).attr.attr.config1 = 0;
                }
                if attr_size as usize >= offset_of!(perf_event_attr, config2) + size_of::<u64>() {
                    (*event).attr.attr.config2 = 0;
                }
            }
        }
    }
}

unsafe fn aslr_tool__init(aslr: *mut aslr_tool, delegate: *mut perf_tool) -> c_int {
    delegate_tool__init(&mut (*aslr).tool, delegate);
    (*aslr).tool.tool.ordered_events = true;

    if machines__init(&mut (*aslr).machines) != 0 {
        return -ENOMEM;
    }

    hashmap__init(&mut (*aslr).remap_addresses, remap_addresses__hash, remap_addresses__equal, ptr::null_mut());
    hashmap__init(&mut (*aslr).top_addresses, top_addresses__hash, top_addresses__equal, ptr::null_mut());
    hashmap__init(&mut (*aslr).evsel_orig_attrs, evsel_hash, evsel_equal, ptr::null_mut());

    (*aslr).tool.tool.sample = Some(aslr_tool__process_sample);
    (*aslr).tool.tool.mmap = Some(aslr_tool__process_mmap);
    (*aslr).tool.tool.mmap2 = Some(aslr_tool__process_mmap2);
    (*aslr).tool.tool.comm = Some(aslr_tool__process_comm);
    (*aslr).tool.tool.fork = Some(aslr_tool__process_fork);
    (*aslr).tool.tool.exit = Some(aslr_tool__process_exit);
    (*aslr).tool.tool.ksymbol = Some(aslr_tool__process_ksymbol);
    (*aslr).tool.tool.text_poke = Some(aslr_tool__process_text_poke);
    (*aslr).tool.tool.auxtrace = Some(aslr_tool__process_auxtrace);
    (*aslr).tool.tool.auxtrace_info = Some(aslr_tool__process_auxtrace_info);
    (*aslr).tool.tool.auxtrace_error = Some(aslr_tool__process_auxtrace_error);

    0
}

extern "C" {
    fn delegate_tool__init(tool: *mut delegate_tool, delegate: *mut perf_tool);
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__new(delegate: *mut perf_tool) -> *mut perf_tool {
    let aslr = zalloc(size_of::<aslr_tool>()) as *mut aslr_tool;

    if aslr.is_null() {
        return ptr::null_mut();
    }

    if aslr_tool__init(aslr, delegate) != 0 {
        free(aslr as *mut c_void);
        return ptr::null_mut();
    }
    &mut (*aslr).tool.tool
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__delete(tool: *mut perf_tool) {
    let del_tool: *mut delegate_tool;
    let aslr: *mut aslr_tool;
    let mut cur: *mut hashmap_entry = ptr::null_mut();
    let mut bkt: size_t = 0;
    let mut nd: *mut rb_node;

    if tool.is_null() {
        return;
    }

    del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));

    hashmap_for_each_entry(&mut (*aslr).remap_addresses, &mut cur, &mut bkt, |cur| {
        let key = (*cur).pkey as *mut remap_addresses_key;
        if !key.is_null() {
            dso__put((*key).dso);
        }
        zfree(&mut (*cur).pkey);
        zfree(&mut (*cur).pvalue);
    });
    hashmap_for_each_entry(&mut (*aslr).top_addresses, &mut cur, &mut bkt, |cur| {
        zfree(&mut (*cur).pkey);
        zfree(&mut (*cur).pvalue);
    });
    hashmap_for_each_entry(&mut (*aslr).evsel_orig_attrs, &mut cur, &mut bkt, |cur| {
        zfree(&mut (*cur).pvalue);
    });

    hashmap__clear(&mut (*aslr).remap_addresses);
    hashmap__clear(&mut (*aslr).top_addresses);
    hashmap__clear(&mut (*aslr).evsel_orig_attrs);
    aslr_tool__destroy_machines_priv(&mut (*aslr).machines);
    machines__destroy_kernel_maps(&mut (*aslr).machines);

    nd = rb_first_cached(&(*aslr).machines.guests);
    while !nd.is_null() {
        let machine = rb_entry_machine(nd);
        rb_erase_cached(nd, &mut (*aslr).machines.guests);
        machine__delete(machine);
        nd = rb_first_cached(&(*aslr).machines.guests);
    }

    machines__exit(&mut (*aslr).machines);
    free(aslr as *mut c_void);
}

unsafe fn hashmap_for_each_entry<F>(_map: *mut hashmap, _cur: *mut *mut hashmap_entry, _bkt: *mut size_t, _f: F)
where
    F: FnMut(*mut hashmap_entry),
{
    // Translation placeholder for hashmap__for_each_entry macro supplied by perf's C helpers.
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__cache_orig_attrs(tool: *mut perf_tool, evsel: *mut evsel) -> c_int {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let priv_ = zalloc(size_of::<aslr_evsel_priv>()) as *mut aslr_evsel_priv;
    let err: c_int;

    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).orig_sample_type = (*evsel).core.attr.sample_type;
    (*priv_).orig_sample_regs_user = (*evsel).core.attr.sample_regs_user;
    (*priv_).orig_sample_regs_intr = (*evsel).core.attr.sample_regs_intr;
    (*priv_).orig_sample_size = (*evsel).sample_size;

    err = hashmap__add(&mut (*aslr).evsel_orig_attrs, evsel as *mut c_void, priv_ as *mut c_void);
    if err != 0 {
        free(priv_ as *mut c_void);
        return err;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__strip_evlist(_tool: *const perf_tool, evlist: *mut evlist) {
    let mut evsel: *mut evsel = ptr::null_mut();

    evlist_for_each_entry(evlist, &mut evsel, |evsel| {
        (*evsel).core.attr.sample_type &= ASLR_SUPPORTED_SAMPLE_TYPE;
        (*evsel).core.attr.sample_regs_user = 0;
        (*evsel).core.attr.sample_regs_intr = 0;
        (*evsel).sample_size = __evsel__sample_size((*evsel).core.attr.sample_type);
        evsel__calc_id_pos(evsel);

        if (*evsel).core.attr.type_ == PERF_TYPE_BREAKPOINT {
            (*evsel).core.attr.bp_addr = 0;
        } else if (*evsel).core.attr.type_ >= PERF_TYPE_MAX {
            let pmu = perf_pmus__find_by_type((*evsel).core.attr.type_);

            if !pmu.is_null() && (strcmp((*pmu).name, c"kprobe".as_ptr()) == 0 || strcmp((*pmu).name, c"uprobe".as_ptr()) == 0) {
                (*evsel).core.attr.config1 = 0;
                (*evsel).core.attr.config2 = 0;
            }
        }
    });
}

unsafe fn evlist_for_each_entry<F>(_evlist: *mut evlist, _evsel: *mut *mut evsel, _f: F)
where
    F: FnMut(*mut evsel),
{
    // Translation placeholder for evlist__for_each_entry macro supplied by perf's C helpers.
}

#[no_mangle]
pub unsafe extern "C" fn aslr_tool__restore_evlist(tool: *const perf_tool, evlist: *mut evlist) {
    let del_tool = container_of::<delegate_tool, perf_tool>(tool, offset_of!(delegate_tool, tool));
    let aslr = container_of::<aslr_tool, delegate_tool>(del_tool, offset_of!(aslr_tool, tool));
    let mut evsel: *mut evsel = ptr::null_mut();
    let mut priv_ptr: *mut c_void = ptr::null_mut();

    evlist_for_each_entry(evlist, &mut evsel, |evsel| {
        if hashmap__find(&(*aslr).evsel_orig_attrs, evsel as *const c_void, &mut priv_ptr) {
            let priv_ = priv_ptr as *mut aslr_evsel_priv;
            (*evsel).core.attr.sample_type = (*priv_).orig_sample_type;
            (*evsel).core.attr.sample_regs_user = (*priv_).orig_sample_regs_user;
            (*evsel).core.attr.sample_regs_intr = (*priv_).orig_sample_regs_intr;
            (*evsel).sample_size = (*priv_).orig_sample_size;
            evsel__calc_id_pos(evsel);
        }
    });
}
