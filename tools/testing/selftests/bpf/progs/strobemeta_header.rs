// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// Translated from testing/selftests/bpf/progs/strobemeta.h.
// C includes and BPF section/map-definition helper macros are represented by
// Rust declarations/comments where they require external build support.

pub type pid_t = u32;

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

pub const TASK_COMM_LEN: usize = 16;
pub const PERF_MAX_STACK_DEPTH: usize = 127;

pub const STROBE_TYPE_INVALID: i32 = 0;
pub const STROBE_TYPE_INT: i32 = 1;
pub const STROBE_TYPE_STR: i32 = 2;
pub const STROBE_TYPE_MAP: i32 = 3;

pub const STACK_TABLE_EPOCH_SHIFT: u32 = 20;
pub const STROBE_MAX_STR_LEN: usize = 1;
pub const STROBE_MAX_CFGS: usize = 32;
pub const READ_MAP_VAR_PAYLOAD_CAP: usize =
    (1 + STROBE_MAX_MAP_ENTRIES * 2) * STROBE_MAX_STR_LEN;
pub const STROBE_MAX_PAYLOAD: usize =
    STROBE_MAX_STRS * STROBE_MAX_STR_LEN + STROBE_MAX_MAPS * READ_MAP_VAR_PAYLOAD_CAP;

#[repr(C)]
pub struct strobe_value_header {
    /*
     * meaning depends on type:
     * 1. int: 0, if value not set, 1 otherwise
     * 2. str: 1 always, whether value is set or not is determined by ptr
     * 3. map: 1 always, pointer points to additional struct with number
     *    of entries (up to STROBE_MAX_MAP_ENTRIES)
     */
    pub len: u16,
    /*
     * _reserved might be used for some future fields/flags, but we always
     * want to keep strobe_value_header to be 8 bytes, so BPF can read 16
     * bytes in one go and get both header and value
     */
    pub _reserved: [u8; 6],
}

/*
 * strobe_value_generic is used from BPF probe only, but needs to be a union
 * of strobe_value_int/strobe_value_str/strobe_value_map
 */
#[repr(C)]
pub struct strobe_value_generic {
    pub header: strobe_value_header,
    pub u: strobe_value_generic_union,
}

#[repr(C)]
pub union strobe_value_generic_union {
    pub val: i64,
    pub ptr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct strobe_value_int {
    pub header: strobe_value_header,
    pub value: i64,
}

#[repr(C)]
pub struct strobe_value_str {
    pub header: strobe_value_header,
    pub value: *const core::ffi::c_char,
}

#[repr(C)]
pub struct strobe_value_map {
    pub header: strobe_value_header,
    pub value: *const strobe_map_raw,
}

#[repr(C)]
pub struct strobe_map_entry {
    pub key: *const core::ffi::c_char,
    pub val: *const core::ffi::c_char,
}

/*
 * Map of C-string key/value pairs with fixed maximum capacity. Each map has
 * corresponding int64 ID, which application can use (or ignore) in whatever
 * way appropriate. Map is "write-only", there is no way to get data out of
 * map. Map is intended to be used to provide metadata for profilers and is
 * not to be used for internal in-app communication. All methods are
 * thread-safe.
 */
#[repr(C)]
pub struct strobe_map_raw {
    /*
     * general purpose unique ID that's up to application to decide
     * whether and how to use; for request metadata use case id is unique
     * request ID that's used to match metadata with stack traces on
     * Strobelight backend side
     */
    pub id: i64,
    /* number of used entries in map */
    pub cnt: i64,
    /*
     * having volatile doesn't change anything on BPF side, but clang
     * emits warnings for passing `volatile const char *` into
     * bpf_probe_read_user_str that expects just `const char *`
     */
    pub tag: *const core::ffi::c_char,
    /*
     * key/value entries, each consisting of 2 pointers to key and value
     * C strings
     */
    pub entries: [strobe_map_entry; STROBE_MAX_MAP_ENTRIES],
}

/* Following values define supported values of TLS mode */
pub const TLS_NOT_SET: i64 = -1;
pub const TLS_LOCAL_EXEC: i64 = 0;
pub const TLS_IMM_EXEC: i64 = 1;
pub const TLS_GENERAL_DYN: i64 = 2;

/*
 * structure that universally represents TLS location (both for static
 * executables and shared libraries)
 */
#[repr(C)]
pub struct strobe_value_loc {
    /*
     * tls_mode defines what TLS mode was used for particular metavariable:
     * - -1 (TLS_NOT_SET) - no metavariable;
     * - 0 (TLS_LOCAL_EXEC) - Local Executable mode;
     * - 1 (TLS_IMM_EXEC) - Immediate Executable mode;
     * - 2 (TLS_GENERAL_DYN) - General Dynamic mode;
     * Local Dynamic mode is not yet supported, because never seen in
     * practice.  Mode defines how offset field is interpreted. See
     * calc_location() in below for details.
     */
    pub tls_mode: i64,
    /*
     * TLS_LOCAL_EXEC: offset from thread pointer (fs:0 for x86-64,
     * tpidr_el0 for aarch64).
     * TLS_IMM_EXEC: absolute address of GOT entry containing offset
     * from thread pointer;
     * TLS_GENERAL_DYN: absolute address of double GOT entry
     * containing tls_index_t struct;
     */
    pub offset: i64,
}

#[repr(C)]
pub struct strobemeta_cfg {
    pub req_meta_idx: i64,
    pub int_locs: [strobe_value_loc; STROBE_MAX_INTS],
    pub str_locs: [strobe_value_loc; STROBE_MAX_STRS],
    pub map_locs: [strobe_value_loc; STROBE_MAX_MAPS],
}

#[repr(C)]
pub struct strobe_map_descr {
    pub id: u64,
    pub tag_len: i16,
    /*
     * cnt <0 - map value isn't set;
     * 0 - map has id set, but no key/value entries
     */
    pub cnt: i16,
    /*
     * both key_lens[i] and val_lens[i] should be >0 for present key/value
     * entry
     */
    pub key_lens: [u16; STROBE_MAX_MAP_ENTRIES],
    pub val_lens: [u16; STROBE_MAX_MAP_ENTRIES],
}

#[repr(C)]
pub struct strobemeta_payload {
    /* req_id has valid request ID, if req_meta_valid == 1 */
    pub req_id: i64,
    pub req_meta_valid: u8,
    /*
     * mask has Nth bit set to 1, if Nth metavar was present and
     * successfully read
     */
    pub int_vals_set_mask: u64,
    pub int_vals: [i64; STROBE_MAX_INTS],
    /* len is >0 for present values */
    pub str_lens: [u16; STROBE_MAX_STRS],
    /* if map_descrs[i].cnt == -1, metavar is not present/set */
    pub map_descrs: [strobe_map_descr; STROBE_MAX_MAPS],
    /*
     * payload has compactly packed values of str and map variables in the
     * form: strval1\0strval2\0map1key1\0map1val1\0map2key1\0map2val1\0
     * (and so on); str_lens[i], key_lens[i] and val_lens[i] determines
     * value length
     */
    pub payload: [core::ffi::c_char; STROBE_MAX_PAYLOAD],
}

#[repr(C)]
pub struct strobelight_bpf_sample {
    pub ktime: u64,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub user_stack_id: i32,
    pub kernel_stack_id: i32,
    pub has_meta: i32,
    pub metadata: strobemeta_payload,
    /*
     * makes it possible to pass (<real payload size> + 1) as data size to
     * perf_submit() to avoid perf_submit's paranoia about passing zero as
     * size, as it deduces that <real payload size> might be
     * **theoretically** zero
     */
    pub dummy_safeguard: core::ffi::c_char,
}

// BPF map definitions from C:
// samples: PERF_EVENT_ARRAY, max_entries 32, key_size sizeof(int), value_size sizeof(int), SEC(".maps")
// stacks_0: STACK_TRACE, max_entries 16, key_size sizeof(uint32_t),
//           value_size sizeof(uint64_t) * PERF_MAX_STACK_DEPTH, SEC(".maps")
// stacks_1: STACK_TRACE, max_entries 16, key_size sizeof(uint32_t),
//           value_size sizeof(uint64_t) * PERF_MAX_STACK_DEPTH, SEC(".maps")
// sample_heap: PERCPU_ARRAY, max_entries 1, key uint32_t, value strobelight_bpf_sample, SEC(".maps")
// strobemeta_cfgs: PERCPU_ARRAY, max_entries STROBE_MAX_CFGS, key pid_t, value strobemeta_cfg, SEC(".maps")
extern "C" {
    pub static mut samples: core::ffi::c_void;
    pub static mut stacks_0: core::ffi::c_void;
    pub static mut stacks_1: core::ffi::c_void;
    pub static mut sample_heap: core::ffi::c_void;
    pub static mut strobemeta_cfgs: core::ffi::c_void;
}

/* Type for the dtv.  */
/* https://github.com/lattera/glibc/blob/master/nptl/sysdeps/x86_64/tls.h#L34 */
#[repr(C)]
pub union dtv {
    pub counter: usize,
    pub pointer: dtv_pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_pointer {
    pub val: *mut core::ffi::c_void,
    pub is_static: bool,
}

pub type dtv_t = dtv;

/* Partial definition for tcbhead_t */
/* https://github.com/bminor/glibc/blob/master/sysdeps/x86_64/nptl/tls.h#L42 */
#[repr(C)]
pub struct tcbhead {
    pub tcb: *mut core::ffi::c_void,
    pub dtv: *mut dtv_t,
}

/*
 * TLS module/offset information for shared library case.
 * For x86-64, this is mapped onto two entries in GOT.
 * For aarch64, this is pointed to by second GOT entry.
 */
#[repr(C)]
pub struct tls_index {
    pub module: u64,
    pub offset: u64,
}

extern "C" {
    pub fn bpf_probe_read_user(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    pub fn bpf_probe_read_user_str(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    pub fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(u64, *mut read_var_ctx) -> i32,
        callback_ctx: *mut read_var_ctx,
        flags: u64,
    ) -> i32;
    pub fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size_of_buf: u32) -> i64;
    pub fn bpf_ktime_get_ns() -> u64;
    pub fn bpf_get_current_task() -> *mut core::ffi::c_void;
    pub fn bpf_get_stackid(
        ctx: *mut core::ffi::c_void,
        map: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_perf_event_output(
        ctx: *mut core::ffi::c_void,
        map: *mut core::ffi::c_void,
        flags: u64,
        data: *mut core::ffi::c_void,
        size: u64,
    ) -> i64;
}

unsafe fn calc_location(
    loc: *mut strobe_value_loc,
    tls_base: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    /*
     * tls_mode value is:
     * - -1 (TLS_NOT_SET), if no metavar is present;
     * - 0 (TLS_LOCAL_EXEC), if metavar uses Local Executable mode of TLS
     * (offset from fs:0 for x86-64 or tpidr_el0 for aarch64);
     * - 1 (TLS_IMM_EXEC), if metavar uses Immediate Executable mode of TLS;
     * - 2 (TLS_GENERAL_DYN), if metavar uses General Dynamic mode of TLS;
     * This schema allows to use something like:
     * (tls_mode + 1) * (tls_base + offset)
     * to get NULL for "no metavar" location, or correct pointer for local
     * executable mode without doing extra ifs.
     */
    if (*loc).tls_mode <= TLS_LOCAL_EXEC {
        /* static executable is simple, we just have offset from tls_base */
        let addr = (tls_base as *mut u8).offset((*loc).offset as isize) as *mut core::ffi::c_void;
        /* multiply by (tls_mode + 1) to get NULL, if we have no metavar in this slot */
        return (((*loc).tls_mode + 1).wrapping_mul(addr as i64)) as *mut core::ffi::c_void;
    }
    /*
     * Other modes are more complicated, we need to jump through few hoops.
     *
     * For immediate executable mode (currently supported only for aarch64):
     *  - loc->offset is pointing to a GOT entry containing fixed offset
     *  relative to tls_base;
     *
     * For general dynamic mode:
     *  - loc->offset is pointing to a beginning of double GOT entries;
     *  - (for aarch64 only) second entry points to tls_index_t struct;
     *  - (for x86-64 only) two GOT entries are already tls_index_t;
     *  - tls_index_t->module is used to find start of TLS section in
     *  which variable resides;
     *  - tls_index_t->offset provides offset within that TLS section,
     *  pointing to value of variable.
     */
    let mut tls_index: tls_index = core::mem::zeroed();
    let mut dtv: *mut dtv_t;
    let mut tls_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

    bpf_probe_read_user(
        &mut tls_index as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<tls_index>(),
        (*loc).offset as *const core::ffi::c_void,
    );
    /* valid module index is always positive */
    if tls_index.module > 0 {
        /* dtv = ((struct tcbhead *)tls_base)->dtv[tls_index.module] */
        bpf_probe_read_user(
            &mut dtv as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&dtv),
            &(*(tls_base as *mut tcbhead)).dtv as *const _ as *const core::ffi::c_void,
        );
        dtv = dtv.add(tls_index.module as usize);
    } else {
        dtv = core::ptr::null_mut();
    }
    bpf_probe_read_user(
        &mut tls_ptr as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<*mut core::ffi::c_void>(),
        dtv as *const core::ffi::c_void,
    );
    /* if pointer has (void *)-1 value, then TLS wasn't initialized yet */
    if tls_ptr.is_null() || tls_ptr == (-1isize as *mut core::ffi::c_void) {
        return core::ptr::null_mut();
    }
    (tls_ptr as *mut u8).add(tls_index.offset as usize) as *mut core::ffi::c_void
}

unsafe fn read_int_var(
    cfg: *mut strobemeta_cfg,
    idx: usize,
    tls_base: *mut core::ffi::c_void,
    value: *mut strobe_value_generic,
    data: *mut strobemeta_payload,
) {
    let location = calc_location(&mut (*cfg).int_locs[idx], tls_base);
    if location.is_null() {
        return;
    }

    bpf_probe_read_user(
        value as *mut core::ffi::c_void,
        core::mem::size_of::<strobe_value_generic>(),
        location,
    );
    (*data).int_vals[idx] = (*value).u.val;
    if (*value).header.len != 0 {
        (*data).int_vals_set_mask |= 1u64 << idx;
    }
}

unsafe fn read_str_var(
    cfg: *mut strobemeta_cfg,
    idx: usize,
    tls_base: *mut core::ffi::c_void,
    value: *mut strobe_value_generic,
    data: *mut strobemeta_payload,
    off: usize,
) -> u64 {
    let location: *mut core::ffi::c_void;
    let mut len: u64;

    (*data).str_lens[idx] = 0;
    location = calc_location(&mut (*cfg).str_locs[idx], tls_base);
    if location.is_null() {
        return 0;
    }

    bpf_probe_read_user(
        value as *mut core::ffi::c_void,
        core::mem::size_of::<strobe_value_generic>(),
        location,
    );
    len = bpf_probe_read_user_str(
        &mut (*data).payload[off] as *mut _ as *mut core::ffi::c_void,
        STROBE_MAX_STR_LEN,
        (*value).u.ptr as *const core::ffi::c_void,
    ) as u64;
    /*
     * if bpf_probe_read_user_str returns error (<0), due to casting to
     * unsigned int, it will become big number, so next check is
     * sufficient to check for errors AND prove to BPF verifier, that
     * bpf_probe_read_user_str won't return anything bigger than
     * STROBE_MAX_STR_LEN
     */
    if len > STROBE_MAX_STR_LEN as u64 {
        return 0;
    }

    (*data).str_lens[idx] = len as u16;
    off as u64 + len
}

unsafe fn read_map_var(
    cfg: *mut strobemeta_cfg,
    idx: usize,
    tls_base: *mut core::ffi::c_void,
    value: *mut strobe_value_generic,
    data: *mut strobemeta_payload,
    mut off: usize,
) -> u64 {
    let descr = &mut (*data).map_descrs[idx] as *mut strobe_map_descr;
    let mut map: strobe_map_raw = core::mem::zeroed();
    let location: *mut core::ffi::c_void;
    let mut len: u64;

    (*descr).tag_len = 0; /* presume no tag is set */
    (*descr).cnt = -1; /* presume no value is set */

    location = calc_location(&mut (*cfg).map_locs[idx], tls_base);
    if location.is_null() {
        return off as u64;
    }

    bpf_probe_read_user(
        value as *mut core::ffi::c_void,
        core::mem::size_of::<strobe_value_generic>(),
        location,
    );
    if bpf_probe_read_user(
        &mut map as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<strobe_map_raw>(),
        (*value).u.ptr as *const core::ffi::c_void,
    ) != 0
    {
        return off as u64;
    }

    (*descr).id = map.id as u64;
    (*descr).cnt = map.cnt as i16;
    if (*cfg).req_meta_idx == idx as i64 {
        (*data).req_id = map.id;
        (*data).req_meta_valid = 1;
    }

    len = bpf_probe_read_user_str(
        &mut (*data).payload[off] as *mut _ as *mut core::ffi::c_void,
        STROBE_MAX_STR_LEN,
        map.tag as *const core::ffi::c_void,
    ) as u64;
    if len <= STROBE_MAX_STR_LEN as u64 {
        (*descr).tag_len = len as i16;
        off += len as usize;
    }

    // C uses __pragma_loop_no_unroll or __pragma_loop_unroll depending on NO_UNROLL.
    for i in 0..STROBE_MAX_MAP_ENTRIES {
        if i as i64 >= map.cnt {
            break;
        }

        (*descr).key_lens[i] = 0;
        len = bpf_probe_read_user_str(
            &mut (*data).payload[off] as *mut _ as *mut core::ffi::c_void,
            STROBE_MAX_STR_LEN,
            map.entries[i].key as *const core::ffi::c_void,
        ) as u64;
        if len <= STROBE_MAX_STR_LEN as u64 {
            (*descr).key_lens[i] = len as u16;
            off += len as usize;
        }
        (*descr).val_lens[i] = 0;
        len = bpf_probe_read_user_str(
            &mut (*data).payload[off] as *mut _ as *mut core::ffi::c_void,
            STROBE_MAX_STR_LEN,
            map.entries[i].val as *const core::ffi::c_void,
        ) as u64;
        if len <= STROBE_MAX_STR_LEN as u64 {
            (*descr).val_lens[i] = len as u16;
            off += len as usize;
        }
    }

    off as u64
}

// Present when USE_BPF_LOOP is defined in C.
#[repr(C)]
pub enum read_type {
    READ_INT_VAR,
    READ_MAP_VAR,
    READ_STR_VAR,
}

#[repr(C)]
pub struct read_var_ctx {
    pub data: *mut strobemeta_payload,
    pub tls_base: *mut core::ffi::c_void,
    pub cfg: *mut strobemeta_cfg,
    pub payload_off: usize,
    /* value gets mutated */
    pub value: *mut strobe_value_generic,
    pub type_: read_type,
}

unsafe extern "C" fn read_var_callback(index: u64, ctx: *mut read_var_ctx) -> i32 {
    /*
     * lose precision info for ctx->payload_off, verifier won't track
     * double xor, barrier_var() is needed to force clang keep both xors.
     */
    (*ctx).payload_off ^= index as usize;
    barrier_var(&mut (*ctx).payload_off);
    (*ctx).payload_off ^= index as usize;
    match (*ctx).type_ {
        read_type::READ_INT_VAR => {
            if index >= STROBE_MAX_INTS as u64 {
                return 1;
            }
            read_int_var(
                (*ctx).cfg,
                index as usize,
                (*ctx).tls_base,
                (*ctx).value,
                (*ctx).data,
            );
        }
        read_type::READ_MAP_VAR => {
            if index >= STROBE_MAX_MAPS as u64 {
                return 1;
            }
            if (*ctx).payload_off
                > core::mem::size_of_val(&(*(*ctx).data).payload) - READ_MAP_VAR_PAYLOAD_CAP
            {
                return 1;
            }
            (*ctx).payload_off = read_map_var(
                (*ctx).cfg,
                index as usize,
                (*ctx).tls_base,
                (*ctx).value,
                (*ctx).data,
                (*ctx).payload_off,
            ) as usize;
        }
        read_type::READ_STR_VAR => {
            if index >= STROBE_MAX_STRS as u64 {
                return 1;
            }
            if (*ctx).payload_off
                > core::mem::size_of_val(&(*(*ctx).data).payload) - STROBE_MAX_STR_LEN
            {
                return 1;
            }
            (*ctx).payload_off = read_str_var(
                (*ctx).cfg,
                index as usize,
                (*ctx).tls_base,
                (*ctx).value,
                (*ctx).data,
                (*ctx).payload_off,
            ) as usize;
        }
    }
    0
}

extern "C" {
    pub fn barrier_var(var: *mut usize);
}

/*
 * read_strobe_meta returns NULL, if no metadata was read; otherwise returns
 * pointer to *right after* payload ends
 */
unsafe fn read_strobe_meta(
    task: *mut task_struct,
    data: *mut strobemeta_payload,
) -> *mut core::ffi::c_void {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;
    let mut value: strobe_value_generic = core::mem::zeroed();
    let cfg: *mut strobemeta_cfg;
    let mut payload_off: usize;
    let tls_base: *mut core::ffi::c_void;

    cfg = bpf_map_lookup_elem(
        &mut strobemeta_cfgs as *mut _ as *mut core::ffi::c_void,
        &pid as *const _ as *const core::ffi::c_void,
    ) as *mut strobemeta_cfg;
    if cfg.is_null() {
        return core::ptr::null_mut();
    }

    (*data).int_vals_set_mask = 0;
    (*data).req_meta_valid = 0;
    payload_off = 0;
    /*
     * we don't have struct task_struct definition, it should be:
     * tls_base = (void *)task->thread.fsbase;
     */
    tls_base = task as *mut core::ffi::c_void;

    // C conditionally uses bpf_loop when USE_BPF_LOOP is defined. The direct
    // loop form below translates the #else path; read_var_ctx/read_var_callback
    // above preserve the USE_BPF_LOOP branch bodies.
    // C uses __pragma_loop_no_unroll or __pragma_loop_unroll depending on NO_UNROLL.
    for i in 0..STROBE_MAX_INTS {
        read_int_var(cfg, i, tls_base, &mut value, data);
    }
    // C uses __pragma_loop_no_unroll or __pragma_loop_unroll depending on NO_UNROLL.
    for i in 0..STROBE_MAX_STRS {
        payload_off = read_str_var(cfg, i, tls_base, &mut value, data, payload_off) as usize;
    }
    // C uses __pragma_loop_no_unroll or __pragma_loop_unroll depending on NO_UNROLL.
    for i in 0..STROBE_MAX_MAPS {
        payload_off = read_map_var(cfg, i, tls_base, &mut value, data, payload_off) as usize;
    }

    /*
     * return pointer right after end of payload, so it's possible to
     * calculate exact amount of useful data that needs to be sent
     */
    &mut (*data).payload[payload_off] as *mut _ as *mut core::ffi::c_void
}

// SEC("raw_tracepoint/kfree_skb")
pub unsafe extern "C" fn on_event(ctx: *mut core::ffi::c_void) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;
    let sample: *mut strobelight_bpf_sample;
    let task: *mut task_struct;
    let zero: u32 = 0;
    let ktime_ns: u64;
    let mut sample_end: *mut core::ffi::c_void;

    sample = bpf_map_lookup_elem(
        &mut sample_heap as *mut _ as *mut core::ffi::c_void,
        &zero as *const _ as *const core::ffi::c_void,
    ) as *mut strobelight_bpf_sample;
    if sample.is_null() {
        return 0; /* this will never happen */
    }

    (*sample).pid = pid;
    bpf_get_current_comm(
        &mut (*sample).comm as *mut _ as *mut core::ffi::c_void,
        TASK_COMM_LEN as u32,
    );
    ktime_ns = bpf_ktime_get_ns();
    (*sample).ktime = ktime_ns;

    task = bpf_get_current_task() as *mut task_struct;
    sample_end = read_strobe_meta(task, &mut (*sample).metadata);
    (*sample).has_meta = (!sample_end.is_null()) as i32;
    if sample_end.is_null() {
        sample_end = &mut (*sample).metadata as *mut _ as *mut core::ffi::c_void;
    }

    if ((ktime_ns >> STACK_TABLE_EPOCH_SHIFT) & 1) != 0 {
        (*sample).kernel_stack_id =
            bpf_get_stackid(ctx, &mut stacks_1 as *mut _ as *mut core::ffi::c_void, 0) as i32;
        (*sample).user_stack_id = bpf_get_stackid(
            ctx,
            &mut stacks_1 as *mut _ as *mut core::ffi::c_void,
            BPF_F_USER_STACK as u64,
        ) as i32;
    } else {
        (*sample).kernel_stack_id =
            bpf_get_stackid(ctx, &mut stacks_0 as *mut _ as *mut core::ffi::c_void, 0) as i32;
        (*sample).user_stack_id = bpf_get_stackid(
            ctx,
            &mut stacks_0 as *mut _ as *mut core::ffi::c_void,
            BPF_F_USER_STACK as u64,
        ) as i32;
    }

    let sample_size: u64 = (sample_end as usize).wrapping_sub(sample as usize) as u64;
    /* should always be true */
    if sample_size < core::mem::size_of::<strobelight_bpf_sample>() as u64 {
        bpf_perf_event_output(
            ctx,
            &mut samples as *mut _ as *mut core::ffi::c_void,
            0,
            sample as *mut core::ffi::c_void,
            1 + sample_size,
        );
    }
    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
