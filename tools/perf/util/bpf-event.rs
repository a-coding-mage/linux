// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/bpf-event.c.
// C include dependencies are intentionally represented as external symbols/types
// expected to be supplied by the surrounding translated repository.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type u8 = __u8;
type u32 = __u32;
type u64 = __u64;
type bool_ = bool;

unsafe fn snprintf_hex(buf: *mut c_char, size: size_t, data: *mut c_uchar, len: size_t) -> c_int {
    let mut ret: c_int = 0;
    let mut i: size_t = 0;

    while i < len {
        ret += scnprintf(
            buf.add(ret as usize),
            size.wrapping_sub(ret as usize),
            c_str!("%02x"),
            *data.add(i) as c_int,
        );
        i += 1;
    }
    ret
}

unsafe fn machine__process_bpf_event_load(
    machine: *mut machine,
    event: *mut perf_event,
    sample: *mut perf_sample,
) -> c_int {
    let mut info_node: *mut bpf_prog_info_node;
    let env: *mut perf_env = (*machine).env;
    let mut info_linear: *mut perf_bpil;
    let id: c_int = (*event).bpf.id as c_int;
    let mut i: c_uint;

    /* perf-record, no need to handle bpf-event */
    if env.is_null() {
        return 0;
    }

    info_node = perf_env__find_bpf_prog_info(env, id);
    if info_node.is_null() {
        return 0;
    }
    info_linear = (*info_node).info_linear;

    /* jited_ksyms is only valid if bpil_offs_to_addr() converted it */
    if ((*info_linear).arrays & (1u64 << PERF_BPIL_JITED_KSYMS)) == 0 {
        return 0;
    }

    i = 0;
    while i < (*info_linear).info.nr_jited_ksyms {
        let addrs: *mut u64 = (*info_linear).info.jited_ksyms as uintptr_t as *mut u64;
        let addr: u64 = *addrs.add(i as usize);
        let map: *mut map = maps__find(machine__kernel_maps(machine), addr);

        if !map.is_null() {
            let dso: *mut dso = map__dso(map);

            dso__set_binary_type(dso, DSO_BINARY_TYPE__BPF_PROG_INFO);
            (*dso__bpf_prog(dso)).id = id;
            (*dso__bpf_prog(dso)).sub_id = i;
            (*dso__bpf_prog(dso)).env = env;
            map__put(map);
        }
        i += 1;
    }
    0
}

pub unsafe fn machine__process_bpf(
    machine: *mut machine,
    event: *mut perf_event,
    sample: *mut perf_sample,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_bpf(event, stdout);
    }

    match (*event).bpf.type_ {
        PERF_BPF_EVENT_PROG_LOAD => {
            return machine__process_bpf_event_load(machine, event, sample);
        }

        PERF_BPF_EVENT_PROG_UNLOAD => {
            /*
             * Do not free bpf_prog_info and btf of the program here,
             * as annotation still need them. They will be freed at
             * the end of the session.
             */
        }
        _ => {
            pr_debug(c_str!("unexpected bpf event type of %d\n"), (*event).bpf.type_);
        }
    }
    0
}

unsafe fn perf_env__fetch_btf(env: *mut perf_env, btf_id: u32, btf: *mut btf) -> c_int {
    let mut node: *mut btf_node;
    let mut data_size: u32 = 0;
    let data: *const c_void;

    data = btf__raw_data(btf, &mut data_size);

    node = malloc(data_size as size_t + size_of::<btf_node>()) as *mut btf_node;
    if node.is_null() {
        return -1;
    }

    (*node).id = btf_id;
    (*node).data_size = data_size;
    memcpy((*node).data.as_mut_ptr() as *mut c_void, data, data_size as size_t);

    if !perf_env__insert_btf(env, node) {
        /* Insertion failed because of a duplicate. */
        free(node as *mut c_void);
        return -1;
    }
    0
}

unsafe fn synthesize_bpf_prog_name(
    buf: *mut c_char,
    size: c_int,
    info: *mut bpf_prog_info,
    btf: *mut btf,
    sub_id: u32,
) -> c_int {
    let prog_tags: *mut [u8; BPF_TAG_SIZE] = (*info).prog_tags as uintptr_t as *mut [u8; BPF_TAG_SIZE];
    let func_infos: *mut c_void = (*info).func_info as uintptr_t as *mut c_void;
    let sub_prog_cnt: u32 = (*info).nr_jited_ksyms;
    let mut finfo: *const bpf_func_info;
    let mut short_name: *const c_char = ptr::null();
    let mut t: *const btf_type;
    let mut name_len: c_int;

    name_len = scnprintf(buf, size as size_t, c_str!("bpf_prog_"));
    name_len += snprintf_hex(
        buf.add(name_len as usize),
        (size - name_len) as size_t,
        (*prog_tags.add(sub_id as usize)).as_mut_ptr(),
        BPF_TAG_SIZE,
    );
    if !btf.is_null()
        && (*info).func_info_rec_size as size_t >= size_of::<bpf_func_info>()
        && sub_id < (*info).nr_func_info
    {
        finfo = (func_infos as *mut u8).add(sub_id as usize * (*info).func_info_rec_size as usize)
            as *const bpf_func_info;
        t = btf__type_by_id(btf, (*finfo).type_id);
        if !t.is_null() {
            short_name = btf__name_by_offset(btf, (*t).name_off);
        }
    } else if sub_id == 0 && sub_prog_cnt == 1 {
        /* no subprog */
        if (*info).name[0] != 0 {
            short_name = (*info).name.as_ptr();
        }
    } else {
        short_name = c_str!("F");
    }
    if !short_name.is_null() {
        name_len += scnprintf(
            buf.add(name_len as usize),
            (size - name_len) as size_t,
            c_str!("_%s"),
            short_name,
        );
    }
    name_len
}

// HAVE_LIBBPF_STRINGS_SUPPORT conditional block from C source.
const BPF_METADATA_PREFIX: *const c_char = c_str!("bpf_metadata_");
const BPF_METADATA_PREFIX_LEN: size_t = 13;

unsafe fn name_has_bpf_metadata_prefix(s: *mut *const c_char) -> bool {
    if strncmp(*s, BPF_METADATA_PREFIX, BPF_METADATA_PREFIX_LEN) != 0 {
        return false;
    }
    *s = (*s).add(BPF_METADATA_PREFIX_LEN);
    true
}

#[repr(C)]
struct bpf_metadata_map {
    btf: *mut btf,
    datasec: *const btf_type,
    rodata: *mut c_void,
    rodata_size: size_t,
    num_vars: c_uint,
}

unsafe fn bpf_metadata_read_map_data(map_id: __u32, map: *mut bpf_metadata_map) -> c_int {
    let map_fd: c_int;
    let mut map_info: bpf_map_info = core::mem::zeroed();
    let mut map_info_len: __u32;
    let mut key: c_int;
    let mut btf: *mut btf;
    let mut datasec: *const btf_type;
    let mut vsi: *mut btf_var_secinfo;
    let vlen: c_uint;
    let mut vars: c_uint;
    let rodata: *mut c_void;

    map_fd = bpf_map_get_fd_by_id(map_id);
    if map_fd < 0 {
        return -1;
    }

    memset(&mut map_info as *mut _ as *mut c_void, 0, size_of::<bpf_map_info>());
    map_info_len = size_of::<bpf_map_info>() as __u32;
    if bpf_obj_get_info_by_fd(
        map_fd,
        &mut map_info as *mut _ as *mut c_void,
        &mut map_info_len,
    ) < 0
    {
        close(map_fd);
        return -1;
    }

    /* If it's not an .rodata map, don't bother. */
    if map_info.type_ != BPF_MAP_TYPE_ARRAY
        || map_info.key_size as size_t != size_of::<c_int>()
        || map_info.max_entries != 1
        || map_info.btf_value_type_id == 0
        || strstr(map_info.name.as_ptr(), c_str!(".rodata")).is_null()
    {
        close(map_fd);
        return -1;
    }

    btf = btf__load_from_kernel_by_id(map_info.btf_id);
    if btf.is_null() {
        close(map_fd);
        return -1;
    }
    datasec = btf__type_by_id(btf, map_info.btf_value_type_id);
    if !btf_is_datasec(datasec) {
        btf__free(btf);
        close(map_fd);
        return -1;
    }

    /*
     * If there aren't any variables with the "bpf_metadata_" prefix,
     * don't bother.
     */
    vlen = btf_vlen(datasec);
    vsi = btf_var_secinfos(datasec);
    vars = 0;
    let mut i: c_uint = 0;
    while i < vlen {
        let t_var: *const btf_type = btf__type_by_id(btf, (*vsi).type_);
        let mut name: *const c_char = btf__name_by_offset(btf, (*t_var).name_off);

        if name_has_bpf_metadata_prefix(&mut name) {
            vars += 1;
        }
        i += 1;
        vsi = vsi.add(1);
    }
    if vars == 0 {
        btf__free(btf);
        close(map_fd);
        return -1;
    }

    rodata = zalloc(map_info.value_size as size_t);
    if rodata.is_null() {
        btf__free(btf);
        close(map_fd);
        return -1;
    }
    key = 0;
    if bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const c_void, rodata) != 0 {
        free(rodata);
        btf__free(btf);
        close(map_fd);
        return -1;
    }
    close(map_fd);

    (*map).btf = btf;
    (*map).datasec = datasec;
    (*map).rodata = rodata;
    (*map).rodata_size = map_info.value_size as size_t;
    (*map).num_vars = vars;
    0
}

#[repr(C)]
struct format_btf_ctx {
    buf: *mut c_char,
    buf_size: size_t,
    buf_idx: size_t,
}

unsafe extern "C" fn format_btf_cb(arg: *mut c_void, fmt: *const c_char, ap: va_list) {
    let n: c_int;
    let ctx: *mut format_btf_ctx = arg as *mut format_btf_ctx;

    n = vsnprintf(
        (*ctx).buf.add((*ctx).buf_idx),
        (*ctx).buf_size - (*ctx).buf_idx,
        fmt,
        ap,
    );
    (*ctx).buf_idx += n as size_t;
    if (*ctx).buf_idx >= (*ctx).buf_size {
        (*ctx).buf_idx = (*ctx).buf_size;
    }
}

unsafe fn format_btf_variable(
    btf: *mut btf,
    buf: *mut c_char,
    buf_size: size_t,
    t: *const btf_type,
    btf_data: *const c_void,
) {
    let mut ctx = format_btf_ctx {
        buf,
        buf_idx: 0,
        buf_size,
    };
    let opts = btf_dump_type_data_opts {
        sz: size_of::<btf_dump_type_data_opts>(),
        skip_names: 1,
        compact: 1,
        emit_strings: 1,
    };
    let d: *mut btf_dump;
    let btf_size: size_t;

    d = btf_dump__new(btf, Some(format_btf_cb), &mut ctx as *mut _ as *mut c_void, ptr::null());
    btf_size = btf__resolve_size(btf, (*t).type_);
    btf_dump__dump_type_data(d, (*t).type_, btf_data, btf_size, &opts);
    btf_dump__free(d);
}

unsafe fn bpf_metadata_fill_event(
    map: *mut bpf_metadata_map,
    bpf_metadata_event: *mut perf_record_bpf_metadata,
) {
    let mut vsi: *mut btf_var_secinfo;
    let mut i: c_uint;
    let vlen: c_uint;

    memset(
        (*bpf_metadata_event).prog_name.as_mut_ptr() as *mut c_void,
        0,
        BPF_PROG_NAME_LEN,
    );
    vlen = btf_vlen((*map).datasec);
    vsi = btf_var_secinfos((*map).datasec);

    i = 0;
    while i < vlen {
        let t_var: *const btf_type = btf__type_by_id((*map).btf, (*vsi).type_);
        let mut name: *const c_char = btf__name_by_offset((*map).btf, (*t_var).name_off);
        let nr_entries: __u64 = (*bpf_metadata_event).nr_entries;
        let entry: *mut perf_record_bpf_metadata_entry;

        if !name_has_bpf_metadata_prefix(&mut name) {
            i += 1;
            vsi = vsi.add(1);
            continue;
        }

        if nr_entries >= (*map).num_vars as __u64 {
            break;
        }

        entry = (*bpf_metadata_event).entries.as_mut_ptr().add(nr_entries as usize);
        memset(entry as *mut c_void, 0, size_of::<perf_record_bpf_metadata_entry>());
        snprintf((*entry).key.as_mut_ptr(), BPF_METADATA_KEY_LEN, c_str!("%s"), name);
        format_btf_variable(
            (*map).btf,
            (*entry).value.as_mut_ptr(),
            BPF_METADATA_VALUE_LEN,
            t_var,
            ((*map).rodata as *mut u8).add((*vsi).offset as usize) as *const c_void,
        );
        (*bpf_metadata_event).nr_entries += 1;
        i += 1;
        vsi = vsi.add(1);
    }
}

unsafe fn bpf_metadata_free_map_data(map: *mut bpf_metadata_map) {
    btf__free((*map).btf);
    free((*map).rodata);
}

unsafe fn bpf_metadata_alloc(nr_prog_tags: __u32, nr_variables: __u32) -> *mut bpf_metadata {
    let metadata: *mut bpf_metadata;
    let event_size: size_t;

    metadata = zalloc(size_of::<bpf_metadata>()) as *mut bpf_metadata;
    if metadata.is_null() {
        return ptr::null_mut();
    }

    (*metadata).prog_names = calloc(nr_prog_tags as size_t, size_of::<*mut c_char>()) as *mut *mut c_char;
    if (*metadata).prog_names.is_null() {
        bpf_metadata_free(metadata);
        return ptr::null_mut();
    }
    let mut prog_index: __u32 = 0;
    while prog_index < nr_prog_tags {
        *(*metadata).prog_names.add(prog_index as usize) = zalloc(BPF_PROG_NAME_LEN) as *mut c_char;
        if (*(*metadata).prog_names.add(prog_index as usize)).is_null() {
            bpf_metadata_free(metadata);
            return ptr::null_mut();
        }
        (*metadata).nr_prog_names += 1;
        prog_index += 1;
    }

    event_size = size_of::<perf_record_bpf_metadata>()
        + nr_variables as size_t * size_of::<perf_record_bpf_metadata_entry>();
    /*
     * header.size is __u16.  synthesize_perf_record_bpf_metadata()
     * adds machine->id_hdr_size (up to ~64 bytes) after this, so
     * leave headroom to prevent the final size from wrapping.
     */
    if event_size > UINT16_MAX as size_t - 256 {
        bpf_metadata_free(metadata);
        return ptr::null_mut();
    }
    (*metadata).event = zalloc(event_size) as *mut perf_event;
    if (*metadata).event.is_null() {
        bpf_metadata_free(metadata);
        return ptr::null_mut();
    }
    (*(*metadata).event).bpf_metadata.header.type_ = PERF_RECORD_BPF_METADATA;
    (*(*metadata).event).bpf_metadata.header.size = event_size as __u16;
    (*(*metadata).event).bpf_metadata.nr_entries = 0;

    metadata
}

unsafe fn bpf_metadata_create(info: *mut bpf_prog_info) -> *mut bpf_metadata {
    let mut metadata: *mut bpf_metadata;
    let map_ids: *const __u32 = (*info).map_ids as uintptr_t as *const __u32;

    let mut map_index: __u32 = 0;
    while map_index < (*info).nr_map_ids {
        let mut map: bpf_metadata_map = core::mem::zeroed();

        if bpf_metadata_read_map_data(*map_ids.add(map_index as usize), &mut map) != 0 {
            map_index += 1;
            continue;
        }

        metadata = bpf_metadata_alloc((*info).nr_prog_tags, map.num_vars);
        if metadata.is_null() {
            bpf_metadata_free_map_data(&mut map);
            map_index += 1;
            continue;
        }

        bpf_metadata_fill_event(&mut map, &mut (*(*metadata).event).bpf_metadata);

        let mut index: __u32 = 0;
        while index < (*info).nr_prog_tags {
            synthesize_bpf_prog_name(
                *(*metadata).prog_names.add(index as usize),
                BPF_PROG_NAME_LEN as c_int,
                info,
                map.btf,
                index,
            );
            index += 1;
        }

        bpf_metadata_free_map_data(&mut map);

        return metadata;
    }

    ptr::null_mut()
}

unsafe fn synthesize_perf_record_bpf_metadata(
    metadata: *const bpf_metadata,
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event_size: size_t = (*(*metadata).event).header.size as size_t;
    let event: *mut perf_event;
    let mut err: c_int = 0;

    event = zalloc(event_size + (*machine).id_hdr_size as size_t) as *mut perf_event;
    if event.is_null() {
        return -1;
    }
    memcpy(event as *mut c_void, (*metadata).event as *const c_void, event_size);
    memset(
        (event as *mut u8).add((*event).header.size as usize) as *mut c_void,
        0,
        (*machine).id_hdr_size as size_t,
    );
    (*event).header.size += (*machine).id_hdr_size as __u16;
    let mut index: __u32 = 0;
    while index < (*metadata).nr_prog_names {
        memcpy(
            (*event).bpf_metadata.prog_name.as_mut_ptr() as *mut c_void,
            *(*metadata).prog_names.add(index as usize) as *const c_void,
            BPF_PROG_NAME_LEN,
        );
        err = perf_tool__process_synth_event(tool, event, machine, process);
        if err != 0 {
            break;
        }
        index += 1;
    }

    free(event as *mut c_void);
    err
}

pub unsafe fn bpf_metadata_free(metadata: *mut bpf_metadata) {
    if metadata.is_null() {
        return;
    }
    let mut index: __u32 = 0;
    while index < (*metadata).nr_prog_names {
        free(*(*metadata).prog_names.add(index as usize) as *mut c_void);
        index += 1;
    }
    free((*metadata).prog_names as *mut c_void);
    free((*metadata).event as *mut c_void);
    free(metadata as *mut c_void);
}

// C #else fallback for !HAVE_LIBBPF_STRINGS_SUPPORT:
// bpf_metadata_create() returns NULL, synthesize_perf_record_bpf_metadata() returns 0,
// and bpf_metadata_free() is a no-op in builds without libbpf string support.

#[repr(C)]
struct bpf_metadata_final_ctx {
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
}

unsafe extern "C" fn synthesize_final_bpf_metadata_cb(
    node: *mut bpf_prog_info_node,
    data: *mut c_void,
) {
    let ctx: *mut bpf_metadata_final_ctx = data as *mut bpf_metadata_final_ctx;
    let metadata: *mut bpf_metadata = (*node).metadata;
    let err: c_int;

    if metadata.is_null() {
        return;
    }
    err = synthesize_perf_record_bpf_metadata(metadata, (*ctx).tool, (*ctx).process, (*ctx).machine);
    if err != 0 {
        let prog_name: *const c_char = *(*metadata).prog_names.add(0);

        if !prog_name.is_null() {
            pr_warning(c_str!("Couldn't synthesize final BPF metadata for %s.\n"), prog_name);
        } else {
            pr_warning(c_str!("Couldn't synthesize final BPF metadata.\n"));
        }
    }
    bpf_metadata_free(metadata);
    (*node).metadata = ptr::null_mut();
}

pub unsafe fn perf_event__synthesize_final_bpf_metadata(
    session: *mut perf_session,
    process: perf_event__handler_t,
) {
    let env: *mut perf_env = &mut (*session).header.env;
    let mut ctx = bpf_metadata_final_ctx {
        tool: (*session).tool,
        process,
        machine: &mut (*session).machines.host,
    };

    perf_env__iterate_bpf_prog_info(
        env,
        Some(synthesize_final_bpf_metadata_cb),
        &mut ctx as *mut _ as *mut c_void,
    );
}

/*
 * Synthesize PERF_RECORD_KSYMBOL and PERF_RECORD_BPF_EVENT for one bpf
 * program. One PERF_RECORD_BPF_EVENT is generated for the program. And
 * one PERF_RECORD_KSYMBOL is generated for each sub program.
 *
 * Returns:
 *    0 for success;
 *   -1 for failures;
 *   -2 for lack of kernel support.
 */
unsafe fn perf_event__synthesize_one_bpf_prog(
    session: *mut perf_session,
    process: perf_event__handler_t,
    machine: *mut machine,
    fd: c_int,
    event: *mut perf_event,
    opts: *mut record_opts,
) -> c_int {
    let ksymbol_event: *mut perf_record_ksymbol = &mut (*event).ksymbol;
    let bpf_event: *mut perf_record_bpf_event = &mut (*event).bpf;
    let tool: *const perf_tool = (*session).tool;
    let mut info_node: *mut bpf_prog_info_node;
    let mut info_linear: *mut perf_bpil;
    let mut metadata: *mut bpf_metadata;
    let info: *mut bpf_prog_info;
    let mut btf: *mut btf = ptr::null_mut();
    let env: *mut perf_env;
    let mut sub_prog_cnt: u32;
    let mut i: u32;
    let mut err: c_int = 0;
    let mut arrays: u64;

    /*
     * for perf-record and perf-report use header.env;
     * otherwise, use global perf_env.
     */
    env = perf_session__env(session);

    arrays = 1u64 << PERF_BPIL_JITED_KSYMS;
    arrays |= 1u64 << PERF_BPIL_JITED_FUNC_LENS;
    arrays |= 1u64 << PERF_BPIL_FUNC_INFO;
    arrays |= 1u64 << PERF_BPIL_PROG_TAGS;
    arrays |= 1u64 << PERF_BPIL_JITED_INSNS;
    arrays |= 1u64 << PERF_BPIL_LINE_INFO;
    arrays |= 1u64 << PERF_BPIL_JITED_LINE_INFO;
    arrays |= 1u64 << PERF_BPIL_MAP_IDS;

    info_linear = get_bpf_prog_info_linear(fd, arrays);
    if IS_ERR_OR_NULL(info_linear as *const c_void) {
        info_linear = ptr::null_mut();
        pr_debug(c_str!("%s: failed to get BPF program info. aborting\n"), c_str!("perf_event__synthesize_one_bpf_prog"));
        return -1;
    }

    if (*info_linear).info_len < offset_of!(bpf_prog_info, prog_tags) {
        free(info_linear as *mut c_void);
        pr_debug(c_str!("%s: the kernel is too old, aborting\n"), c_str!("perf_event__synthesize_one_bpf_prog"));
        return -2;
    }

    info = &mut (*info_linear).info;
    if (*info).jited_ksyms == 0 {
        free(info_linear as *mut c_void);
        return -1;
    }

    /* number of ksyms, func_lengths, and tags should match */
    sub_prog_cnt = (*info).nr_jited_ksyms;
    if sub_prog_cnt != (*info).nr_prog_tags || sub_prog_cnt != (*info).nr_jited_func_lens {
        free(info_linear as *mut c_void);
        return -1;
    }

    /* check BTF func info support */
    if (*info).btf_id != 0 && (*info).nr_func_info != 0 && (*info).func_info_rec_size != 0 {
        /* btf func info number should be same as sub_prog_cnt */
        if sub_prog_cnt != (*info).nr_func_info {
            pr_debug(c_str!("%s: mismatch in BPF sub program count and BTF function info count, aborting\n"), c_str!("perf_event__synthesize_one_bpf_prog"));
            free(info_linear as *mut c_void);
            return -1;
        }
        btf = btf__load_from_kernel_by_id((*info).btf_id);
        if libbpf_get_error(btf as *const c_void) != 0 {
            pr_debug(c_str!("%s: failed to get BTF of id %u, aborting\n"), c_str!("perf_event__synthesize_one_bpf_prog"), (*info).btf_id);
            err = -1;
            free(info_linear as *mut c_void);
            btf__free(btf);
            return if err != 0 { -1 } else { 0 };
        }
        perf_env__fetch_btf(env, (*info).btf_id, btf);
    }

    /* Synthesize PERF_RECORD_KSYMBOL */
    i = 0;
    while i < sub_prog_cnt {
        let prog_lens: *mut __u32 = (*info).jited_func_lens as uintptr_t as *mut __u32;
        let prog_addrs: *mut __u64 = (*info).jited_ksyms as uintptr_t as *mut __u64;
        let name_len: c_int;

        ptr::write(ksymbol_event, core::mem::zeroed());
        (*ksymbol_event).header.type_ = PERF_RECORD_KSYMBOL;
        (*ksymbol_event).header.size = offset_of!(perf_record_ksymbol, name) as __u16;
        (*ksymbol_event).addr = *prog_addrs.add(i as usize);
        (*ksymbol_event).len = *prog_lens.add(i as usize) as __u64;
        (*ksymbol_event).ksym_type = PERF_RECORD_KSYMBOL_TYPE_BPF;
        (*ksymbol_event).flags = 0;

        name_len = synthesize_bpf_prog_name((*ksymbol_event).name.as_mut_ptr(), KSYM_NAME_LEN as c_int, info, btf, i);
        (*ksymbol_event).header.size += PERF_ALIGN(name_len as size_t + 1, size_of::<u64>()) as __u16;

        memset(
            (event as *mut u8).add((*event).header.size as usize) as *mut c_void,
            0,
            (*machine).id_hdr_size as size_t,
        );
        (*event).header.size += (*machine).id_hdr_size as __u16;
        err = perf_tool__process_synth_event(tool, event, machine, process);
        i += 1;
    }

    if !(*opts).no_bpf_event {
        /* Synthesize PERF_RECORD_BPF_EVENT */
        ptr::write(bpf_event, core::mem::zeroed());
        (*bpf_event).header.type_ = PERF_RECORD_BPF_EVENT;
        (*bpf_event).header.size = size_of::<perf_record_bpf_event>() as __u16;
        (*bpf_event).type_ = PERF_BPF_EVENT_PROG_LOAD;
        (*bpf_event).flags = 0;
        (*bpf_event).id = (*info).id;
        memcpy((*bpf_event).tag.as_mut_ptr() as *mut c_void, (*info).tag.as_ptr() as *const c_void, BPF_TAG_SIZE);
        memset(
            (event as *mut u8).add((*event).header.size as usize) as *mut c_void,
            0,
            (*machine).id_hdr_size as size_t,
        );
        (*event).header.size += (*machine).id_hdr_size as __u16;

        /* save bpf_prog_info to env */
        info_node = malloc(size_of::<bpf_prog_info_node>()) as *mut bpf_prog_info_node;
        if info_node.is_null() {
            err = -1;
            free(info_linear as *mut c_void);
            btf__free(btf);
            return if err != 0 { -1 } else { 0 };
        }

        (*info_node).info_linear = info_linear;
        (*info_node).metadata = ptr::null_mut();
        if !perf_env__insert_bpf_prog_info(env, info_node) {
            /*
             * Insert failed, likely because of a duplicate event
             * made by the sideband thread. Ignore synthesizing the
             * metadata.
             */
            free(info_node as *mut c_void);
            btf__free(btf);
            return if err != 0 { -1 } else { 0 };
        }
        /* info_linear is now owned by info_node and shouldn't be freed below. */
        info_linear = ptr::null_mut();

        /*
         * process after saving bpf_prog_info to env, so that
         * required information is ready for look up
         */
        err = perf_tool__process_synth_event(tool, event, machine, process);

        /* Synthesize PERF_RECORD_BPF_METADATA */
        metadata = bpf_metadata_create(info);
        if !metadata.is_null() {
            err = synthesize_perf_record_bpf_metadata(metadata, tool, process, machine);
            bpf_metadata_free(metadata);
        }
    }

    free(info_linear as *mut c_void);
    btf__free(btf);
    if err != 0 { -1 } else { 0 }
}

#[repr(C)]
struct kallsyms_parse {
    event: *mut perf_event,
    process: perf_event__handler_t,
    machine: *mut machine,
    tool: *const perf_tool,
}

unsafe fn process_bpf_image(name: *mut c_char, addr: u64, data: *mut kallsyms_parse) -> c_int {
    let machine: *mut machine = (*data).machine;
    let event: *mut perf_event = (*data).event;
    let ksymbol: *mut perf_record_ksymbol;
    let len: c_int;

    ksymbol = &mut (*event).ksymbol;

    ptr::write(ksymbol, core::mem::zeroed());
    (*ksymbol).header.type_ = PERF_RECORD_KSYMBOL;
    (*ksymbol).header.size = offset_of!(perf_record_ksymbol, name) as __u16;
    (*ksymbol).addr = addr;
    (*ksymbol).len = page_size;
    (*ksymbol).ksym_type = PERF_RECORD_KSYMBOL_TYPE_BPF;
    (*ksymbol).flags = 0;

    len = scnprintf((*ksymbol).name.as_mut_ptr(), KSYM_NAME_LEN, c_str!("%s"), name);
    (*ksymbol).header.size += PERF_ALIGN(len as size_t + 1, size_of::<u64>()) as __u16;
    memset(
        (event as *mut u8).add((*event).header.size as usize) as *mut c_void,
        0,
        (*machine).id_hdr_size as size_t,
    );
    (*event).header.size += (*machine).id_hdr_size as __u16;

    perf_tool__process_synth_event((*data).tool, event, machine, (*data).process)
}

unsafe extern "C" fn kallsyms_process_symbol(
    data: *mut c_void,
    _name: *const c_char,
    type_: c_char,
    start: u64,
) -> c_int {
    let mut disp: [c_char; KSYM_NAME_LEN] = [0; KSYM_NAME_LEN];
    let module: *const c_char;
    let name: *mut c_char;
    let mut id: c_ulong = 0;
    let mut err: c_int = 0;

    module = strchr(_name, '\t' as c_int);
    if module.is_null() {
        return 0;
    }

    /* We are going after [bpf] module ... */
    if strcmp(module.add(1), c_str!("[bpf]")) != 0 {
        return 0;
    }

    name = memdup(_name as *const c_void, module.offset_from(_name) as size_t + 1) as *mut c_char;
    if name.is_null() {
        return -ENOMEM;
    }

    *name.add(module.offset_from(_name) as usize) = 0;

    /* .. and only for trampolines and dispatchers */
    if sscanf(name, c_str!("bpf_trampoline_%lu"), &mut id) == 1
        || sscanf(name, c_str!("bpf_dispatcher_%s"), disp.as_mut_ptr()) == 1
    {
        err = process_bpf_image(name, start, data as *mut kallsyms_parse);
    }

    free(name as *mut c_void);
    err
}

pub unsafe fn perf_event__synthesize_bpf_events(
    session: *mut perf_session,
    process: perf_event__handler_t,
    machine: *mut machine,
    opts: *mut record_opts,
) -> c_int {
    let mut kallsyms_filename: *const c_char = c_str!("/proc/kallsyms");
    let mut arg: kallsyms_parse;
    let event: *mut perf_event;
    let mut id: __u32 = 0;
    let mut err: c_int;
    let mut fd: c_int;

    if (*opts).no_bpf_event {
        return 0;
    }

    event = malloc(size_of::<perf_record_bpf_event>() + KSYM_NAME_LEN + (*machine).id_hdr_size as size_t) as *mut perf_event;
    if event.is_null() {
        return -1;
    }

    /* Synthesize all the bpf programs in system. */
    loop {
        err = bpf_prog_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT {
                err = 0;
                break;
            }
            /* don't report error on old kernel or EPERM  */
            err = if errno == EINVAL || errno == EPERM { 0 } else { -1 };
            pr_debug(
                c_str!("%s: can't get next program: %m%s\n"),
                c_str!("perf_event__synthesize_bpf_events"),
                if errno == EINVAL { c_str!(" -- kernel too old?") } else { c_str!("") },
            );
            break;
        }
        fd = bpf_prog_get_fd_by_id(id);
        if fd < 0 {
            pr_debug(
                c_str!("%s: failed to get fd for prog_id %u\n"),
                c_str!("perf_event__synthesize_bpf_events"),
                id,
            );
            continue;
        }

        err = perf_event__synthesize_one_bpf_prog(session, process, machine, fd, event, opts);
        close(fd);
        if err != 0 {
            /* do not return error for old kernel */
            if err == -2 {
                err = 0;
            }
            break;
        }
    }

    /* Synthesize all the bpf images - trampolines/dispatchers. */
    if !symbol_conf.kallsyms_name.is_null() {
        kallsyms_filename = symbol_conf.kallsyms_name;
    }

    arg = kallsyms_parse {
        event,
        process,
        machine,
        tool: (*session).tool,
    };

    if kallsyms__parse(kallsyms_filename, &mut arg as *mut _ as *mut c_void, Some(kallsyms_process_symbol)) != 0 {
        pr_err(c_str!("%s: failed to synthesize bpf images: %m\n"), c_str!("perf_event__synthesize_bpf_events"));
    }

    free(event as *mut c_void);
    err
}

unsafe fn perf_env__add_bpf_info(env: *mut perf_env, id: u32) -> c_int {
    let mut info_node: *mut bpf_prog_info_node;
    let mut info_linear: *mut perf_bpil;
    let mut btf: *mut btf = ptr::null_mut();
    let mut arrays: u64;
    let btf_id: u32;
    let fd: c_int;
    let mut err: c_int = 0;

    fd = bpf_prog_get_fd_by_id(id);
    if fd < 0 {
        return -EINVAL;
    }

    arrays = 1u64 << PERF_BPIL_JITED_KSYMS;
    arrays |= 1u64 << PERF_BPIL_JITED_FUNC_LENS;
    arrays |= 1u64 << PERF_BPIL_FUNC_INFO;
    arrays |= 1u64 << PERF_BPIL_PROG_TAGS;
    arrays |= 1u64 << PERF_BPIL_JITED_INSNS;
    arrays |= 1u64 << PERF_BPIL_LINE_INFO;
    arrays |= 1u64 << PERF_BPIL_JITED_LINE_INFO;
    arrays |= 1u64 << PERF_BPIL_MAP_IDS;

    info_linear = get_bpf_prog_info_linear(fd, arrays);
    if IS_ERR_OR_NULL(info_linear as *const c_void) {
        pr_debug(c_str!("%s: failed to get BPF program info. aborting\n"), c_str!("perf_env__add_bpf_info"));
        err = PTR_ERR(info_linear as *const c_void) as c_int;
        btf__free(btf);
        close(fd);
        return err;
    }

    btf_id = (*info_linear).info.btf_id;

    info_node = malloc(size_of::<bpf_prog_info_node>()) as *mut bpf_prog_info_node;
    if !info_node.is_null() {
        (*info_node).info_linear = info_linear;
        (*info_node).metadata = bpf_metadata_create(&mut (*info_linear).info);
        if !perf_env__insert_bpf_prog_info(env, info_node) {
            pr_debug(
                c_str!("%s: duplicate add bpf info request for id %u\n"),
                c_str!("perf_env__add_bpf_info"),
                btf_id,
            );
            bpf_metadata_free((*info_node).metadata);
            free(info_linear as *mut c_void);
            free(info_node as *mut c_void);
            btf__free(btf);
            close(fd);
            return err;
        }
    } else {
        free(info_linear as *mut c_void);
        err = -ENOMEM;
        btf__free(btf);
        close(fd);
        return err;
    }

    if btf_id == 0 {
        btf__free(btf);
        close(fd);
        return err;
    }

    btf = btf__load_from_kernel_by_id(btf_id);
    if btf.is_null() {
        err = -errno;
        pr_debug(c_str!("%s: failed to get BTF of id %u %d\n"), c_str!("perf_env__add_bpf_info"), btf_id, err);
    } else {
        perf_env__fetch_btf(env, btf_id, btf);
    }

    btf__free(btf);
    close(fd);
    err
}

unsafe extern "C" fn bpf_event__sb_cb(event: *mut perf_event, data: *mut c_void) -> c_int {
    let env: *mut perf_env = data as *mut perf_env;
    let mut ret: c_int = 0;

    if (*event).header.type_ != PERF_RECORD_BPF_EVENT {
        return -1;
    }

    match (*event).bpf.type_ {
        PERF_BPF_EVENT_PROG_LOAD => {
            ret = perf_env__add_bpf_info(env, (*event).bpf.id);
        }

        PERF_BPF_EVENT_PROG_UNLOAD => {
            /*
             * Do not free bpf_prog_info and btf of the program here,
             * as annotation still need them. They will be freed at
             * the end of the session.
             */
        }
        _ => {
            pr_debug(c_str!("unexpected bpf event type of %d\n"), (*event).bpf.type_);
        }
    }

    ret
}

pub unsafe fn evlist__add_bpf_sb_event(evlist: *mut evlist, env: *mut perf_env) -> c_int {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        config: PERF_COUNT_SW_DUMMY,
        sample_id_all: 1,
        watermark: 1,
        bpf_event: 1,
        size: size_of::<perf_event_attr>() as __u32, /* to capture ABI version */
        ..core::mem::zeroed()
    };

    /*
     * Older gcc versions don't support designated initializers, like above,
     * for unnamed union members, such as the following:
     */
    attr.wakeup_watermark = 1;

    evlist__add_sb_event(evlist, &mut attr, Some(bpf_event__sb_cb), env as *mut c_void)
}

pub unsafe fn __bpf_event__print_bpf_prog_info(
    info_linear: *mut perf_bpil,
    env: *mut perf_env,
    fp: *mut FILE,
) {
    let info: *mut bpf_prog_info = &mut (*info_linear).info;
    let required_arrays: __u64 = (1u64 << PERF_BPIL_JITED_KSYMS)
        | (1u64 << PERF_BPIL_JITED_FUNC_LENS)
        | (1u64 << PERF_BPIL_PROG_TAGS);
    let prog_lens: *mut __u32;
    let prog_addrs: *mut __u64;
    let mut name: [c_char; KSYM_NAME_LEN] = [0; KSYM_NAME_LEN];
    let mut btf: *mut btf = ptr::null_mut();
    let sub_prog_cnt: u32;
    let mut i: u32;

    sub_prog_cnt = (*info).nr_jited_ksyms;
    if sub_prog_cnt != (*info).nr_prog_tags || sub_prog_cnt != (*info).nr_jited_func_lens {
        return;
    }

    /* Ensure the arrays were present and converted by bpil_offs_to_addr() */
    if ((*info_linear).arrays & required_arrays) != required_arrays {
        return;
    }

    prog_lens = (*info).jited_func_lens as uintptr_t as *mut __u32;
    prog_addrs = (*info).jited_ksyms as uintptr_t as *mut __u64;

    if (*info).btf_id != 0 {
        let node: *mut btf_node;

        node = __perf_env__find_btf(env, (*info).btf_id);
        if !node.is_null() {
            btf = btf__new((*node).data.as_mut_ptr() as *mut __u8, (*node).data_size);
        }
    }

    if sub_prog_cnt == 1 {
        synthesize_bpf_prog_name(name.as_mut_ptr(), KSYM_NAME_LEN as c_int, info, btf, 0);
        fprintf(
            fp,
            c_str!("# bpf_prog_info %u: %s addr 0x%llx size %u\n"),
            (*info).id,
            name.as_ptr(),
            *prog_addrs.add(0),
            *prog_lens.add(0),
        );
        btf__free(btf);
        return;
    }

    fprintf(fp, c_str!("# bpf_prog_info %u:\n"), (*info).id);
    i = 0;
    while i < sub_prog_cnt {
        synthesize_bpf_prog_name(name.as_mut_ptr(), KSYM_NAME_LEN as c_int, info, btf, i);

        fprintf(
            fp,
            c_str!("# \tsub_prog %u: %s addr 0x%llx size %u\n"),
            i,
            name.as_ptr(),
            *prog_addrs.add(i as usize),
            *prog_lens.add(i as usize),
        );
        i += 1;
    }
    btf__free(btf);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
