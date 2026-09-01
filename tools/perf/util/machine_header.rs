/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/machine.h. C include dependencies:
 * <sys/types.h>, <linux/rbtree.h>, maps.h, dsos.h, rwsem.h, threads.h.
 */

pub type pid_t = i32;
pub type u8 = u8;
pub type u16 = u16;
pub type u64 = u64;
pub type size_t = usize;

/* Native host kernel uses -1 as pid index in machine */
pub const HOST_KERNEL_ID: pid_t = -1;
pub const DEFAULT_GUEST_KERNEL_ID: pid_t = 0;

unsafe extern "C" {
    pub static ref_reloc_sym_names: [*const ::core::ffi::c_char; 0];
}

#[repr(C)]
pub struct addr_location;
#[repr(C)]
pub struct branch_stack;
#[repr(C)]
pub struct dso;
#[repr(C)]
pub struct dso_id;
#[repr(C)]
pub struct evsel;
#[repr(C)]
pub struct perf_sample;
#[repr(C)]
pub struct symbol;
#[repr(C)]
pub struct target;
#[repr(C)]
pub struct thread;
#[repr(C)]
pub union perf_event {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct vdso_info;
#[repr(C)]
pub struct rb_node;
#[repr(C)]
pub struct rb_root_cached;
#[repr(C)]
pub struct threads;
#[repr(C)]
pub struct perf_env;
#[repr(C)]
pub struct dsos;
#[repr(C)]
pub struct maps;
#[repr(C)]
pub struct map;
#[repr(C)]
pub struct comm;
#[repr(C)]
pub struct branch_info;
#[repr(C)]
pub struct mem_info;
#[repr(C)]
pub struct callchain_cursor;
#[repr(C)]
pub struct FILE;
#[repr(C)]
pub struct list_head;

pub const KMAP_NAME_LEN: usize = crate::KMAP_NAME_LEN;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct machine_addr_range {
    pub text_start: u64,
    pub text_end: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union machine_tool_area {
    pub priv_: *mut ::core::ffi::c_void,
    pub db_id: u64,
}

#[repr(C)]
pub struct machine {
    pub rb_node: rb_node,
    pub pid: pid_t,
    pub id_hdr_size: u16,
    pub comm_exec: bool,
    pub kptr_restrict_warned: bool,
    pub single_address_space: bool,
    pub root_dir: *mut ::core::ffi::c_char,
    pub mmap_name: *mut ::core::ffi::c_char,
    pub kallsyms_filename: *mut ::core::ffi::c_char,
    pub threads: threads,
    pub vdso_info: *mut vdso_info,
    pub env: *mut perf_env,
    pub dsos: dsos,
    pub kmaps: *mut maps,
    pub vmlinux_map: *mut map,
    pub kernel_start: u64,
    pub sched: machine_addr_range,
    pub lock: machine_addr_range,
    pub traceiter: machine_addr_range,
    pub trace: machine_addr_range,
    /*
     * The current parallelism level (number of threads that run on CPUs).
     * This value can be less than 1, or larger than the total number
     * of CPUs, if events are poorly ordered.
     */
    pub parallelism: ::core::ffi::c_int,
    pub current_tid: *mut pid_t,
    pub current_tid_sz: size_t,
    /* Tool specific area */
    pub tool_area: machine_tool_area,
    pub machines: *mut machines,
    pub trampolines_mapped: bool,
}

/*
 * The main kernel (vmlinux) map
 */
pub unsafe fn machine__kernel_map(machine: *mut machine) -> *mut map {
    unsafe { (*machine).vmlinux_map }
}

/*
 * kernel (the one returned by machine__kernel_map()) plus kernel modules maps
 */
pub unsafe fn machine__kernel_maps(machine: *mut machine) -> *mut maps {
    unsafe { (*machine).kmaps }
}

unsafe extern "C" {
    pub fn machine__get_kernel_start(machine: *mut machine) -> ::core::ffi::c_int;
}

pub unsafe fn machine__kernel_start(machine: *mut machine) -> u64 {
    unsafe {
        if (*machine).kernel_start == 0 {
            machine__get_kernel_start(machine);
        }
        (*machine).kernel_start
    }
}

pub unsafe fn machine__kernel_ip(machine: *mut machine, ip: u64) -> bool {
    let kernel_start = unsafe { machine__kernel_start(machine) };

    ip >= kernel_start
}

unsafe extern "C" {
    pub fn machine__addr_cpumode(machine: *mut machine, cpumode: u8, addr: u64) -> u8;

    pub fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    pub fn machine__idle_thread(machine: *mut machine) -> *mut thread;
    pub fn machine__thread_exec_comm(machine: *mut machine, thread: *mut thread) -> *mut comm;

    pub fn machine__process_comm_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_exit_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_fork_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_lost_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_lost_samples_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_aux_event(
        machine: *mut machine,
        event: *mut perf_event,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_itrace_start_event(
        machine: *mut machine,
        event: *mut perf_event,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_aux_output_hw_id_event(
        machine: *mut machine,
        event: *mut perf_event,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_switch_event(
        machine: *mut machine,
        event: *mut perf_event,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_namespaces_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_cgroup_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_mmap_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_mmap2_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_ksymbol(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_text_poke(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
    pub fn machine__process_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;
}

pub type machine__process_t =
    Option<unsafe extern "C" fn(machine: *mut machine, data: *mut ::core::ffi::c_void)>;

#[repr(C)]
pub struct machines {
    pub host: machine,
    pub guests: rb_root_cached,
}

unsafe extern "C" {
    pub fn machines__init(machines: *mut machines) -> ::core::ffi::c_int;
    pub fn machines__exit(machines: *mut machines);

    pub fn machines__process_guests(
        machines: *mut machines,
        process: machine__process_t,
        data: *mut ::core::ffi::c_void,
    );

    pub fn machines__add(
        machines: *mut machines,
        pid: pid_t,
        root_dir: *const ::core::ffi::c_char,
    ) -> *mut machine;
    pub fn machines__find(machines: *mut machines, pid: pid_t) -> *mut machine;
    pub fn machines__findnew(machines: *mut machines, pid: pid_t) -> *mut machine;
    pub fn machines__find_guest(machines: *mut machines, pid: pid_t) -> *mut machine;
    pub fn machines__findnew_guest_code(machines: *mut machines, pid: pid_t) -> *mut thread;
    pub fn machine__findnew_guest_code(machine: *mut machine, pid: pid_t) -> *mut thread;

    pub fn machines__set_id_hdr_size(machines: *mut machines, id_hdr_size: u16);
    pub fn machines__set_comm_exec(machines: *mut machines, comm_exec: bool);

    pub fn machine__new_host(host_env: *mut perf_env) -> *mut machine;
    pub fn machine__new_kallsyms(host_env: *mut perf_env) -> *mut machine;
    pub fn machine__new_live(
        host_env: *mut perf_env,
        kernel_maps: bool,
        pid: pid_t,
    ) -> *mut machine;
    pub fn machine__init(
        machine: *mut machine,
        root_dir: *const ::core::ffi::c_char,
        pid: pid_t,
    ) -> ::core::ffi::c_int;
    pub fn machine__exit(machine: *mut machine);
    pub fn machine__delete_threads(machine: *mut machine);
    pub fn machine__delete(machine: *mut machine);
    pub fn machine__remove_thread(machine: *mut machine, th: *mut thread);

    pub fn sample__resolve_bstack(
        sample: *mut perf_sample,
        al: *mut addr_location,
    ) -> *mut branch_info;
    pub fn sample__resolve_mem(
        sample: *mut perf_sample,
        al: *mut addr_location,
    ) -> *mut mem_info;

    pub fn __thread__resolve_callchain(
        thread: *mut thread,
        cursor: *mut callchain_cursor,
        sample: *mut perf_sample,
        parent: *mut *mut symbol,
        root_al: *mut addr_location,
        max_stack: ::core::ffi::c_int,
        symbols: bool,
    ) -> ::core::ffi::c_int;
}

pub unsafe fn thread__resolve_callchain(
    thread: *mut thread,
    cursor: *mut callchain_cursor,
    sample: *mut perf_sample,
    parent: *mut *mut symbol,
    root_al: *mut addr_location,
    max_stack: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        __thread__resolve_callchain(
            thread,
            cursor,
            sample,
            parent,
            root_al,
            max_stack,
            true, /*symbols=*/
        )
    }
}

/*
 * Default guest kernel is defined by parameter --guestkallsyms
 * and --guestmodules
 */
pub unsafe fn machine__is_default_guest(machine: *mut machine) -> bool {
    unsafe {
        if !machine.is_null() {
            (*machine).pid == DEFAULT_GUEST_KERNEL_ID
        } else {
            false
        }
    }
}

pub unsafe fn machine__is_host(machine: *mut machine) -> bool {
    unsafe {
        if !machine.is_null() {
            (*machine).pid == HOST_KERNEL_ID
        } else {
            false
        }
    }
}

unsafe extern "C" {
    pub fn machine__is_lock_function(machine: *mut machine, addr: u64) -> bool;
    pub fn machine__nr_cpus_avail(machine: *mut machine) -> ::core::ffi::c_int;

    pub fn machine__findnew_thread(
        machine: *mut machine,
        pid: pid_t,
        tid: pid_t,
    ) -> *mut thread;

    pub fn machine__findnew_dso_id(
        machine: *mut machine,
        filename: *const ::core::ffi::c_char,
        id: *const dso_id,
    ) -> *mut dso;
    pub fn machine__findnew_dso(
        machine: *mut machine,
        filename: *const ::core::ffi::c_char,
    ) -> *mut dso;

    pub fn machine__fprintf(machine: *mut machine, fp: *mut FILE) -> size_t;
}

pub unsafe fn machine__find_kernel_symbol(
    machine: *mut machine,
    addr: u64,
    mapp: *mut *mut map,
) -> *mut symbol {
    unsafe { maps__find_symbol((*machine).kmaps, addr, mapp) }
}

pub unsafe fn machine__find_kernel_symbol_by_name(
    machine: *mut machine,
    name: *const ::core::ffi::c_char,
    mapp: *mut *mut map,
) -> *mut symbol {
    unsafe { maps__find_symbol_by_name((*machine).kmaps, name, mapp) }
}

unsafe extern "C" {
    pub fn maps__find_symbol(kmaps: *mut maps, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    pub fn maps__find_symbol_by_name(
        kmaps: *mut maps,
        name: *const ::core::ffi::c_char,
        mapp: *mut *mut map,
    ) -> *mut symbol;

    pub fn arch__fix_module_text_start(
        start: *mut u64,
        size: *mut u64,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn machine__load_kallsyms(
        machine: *mut machine,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn machine__load_vmlinux_path(machine: *mut machine) -> ::core::ffi::c_int;
}

pub type machine__fprintf_dsos_buildid_skip_t =
    Option<unsafe extern "C" fn(dso: *mut dso, parm: ::core::ffi::c_int) -> bool>;

unsafe extern "C" {
    pub fn machine__fprintf_dsos_buildid(
        machine: *mut machine,
        fp: *mut FILE,
        skip: machine__fprintf_dsos_buildid_skip_t,
        parm: ::core::ffi::c_int,
    ) -> size_t;
    pub fn machines__fprintf_dsos(machines: *mut machines, fp: *mut FILE) -> size_t;
    pub fn machines__fprintf_dsos_buildid(
        machines: *mut machines,
        fp: *mut FILE,
        skip: machine__fprintf_dsos_buildid_skip_t,
        parm: ::core::ffi::c_int,
    ) -> size_t;

    pub fn machine__destroy_kernel_maps(machine: *mut machine);
    pub fn machine__create_kernel_maps(machine: *mut machine) -> ::core::ffi::c_int;

    pub fn machines__create_kernel_maps(
        machines: *mut machines,
        pid: pid_t,
    ) -> ::core::ffi::c_int;
    pub fn machines__create_guest_kernel_maps(machines: *mut machines) -> ::core::ffi::c_int;
    pub fn machines__destroy_kernel_maps(machines: *mut machines);
}

pub type machine__dso_t = Option<
    unsafe extern "C" fn(
        dso: *mut dso,
        machine: *mut machine,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

unsafe extern "C" {
    pub fn machine__for_each_dso(
        machine: *mut machine,
        fn_: machine__dso_t,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

pub type machine__map_t =
    Option<unsafe extern "C" fn(map: *mut map, priv_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

unsafe extern "C" {
    pub fn machine__for_each_kernel_map(
        machine: *mut machine,
        fn_: machine__map_t,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

pub type machine__thread_fn_t = Option<
    unsafe extern "C" fn(thread: *mut thread, p: *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;

unsafe extern "C" {
    pub fn machine__for_each_thread(
        machine: *mut machine,
        fn_: machine__thread_fn_t,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn machines__for_each_thread(
        machines: *mut machines,
        fn_: machine__thread_fn_t,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct thread_list {
    pub list: list_head,
    pub thread: *mut thread,
}

/* Make a list of struct thread_list based on threads in the machine. */
unsafe extern "C" {
    pub fn machine__thread_list(machine: *mut machine, list: *mut list_head) -> ::core::ffi::c_int;
}
/* Free up the nodes within the thread_list list. */
unsafe extern "C" {
    pub fn thread_list__delete(list: *mut list_head);

    pub fn machine__get_current_tid(machine: *mut machine, cpu: ::core::ffi::c_int) -> pid_t;
    pub fn machine__set_current_tid(
        machine: *mut machine,
        cpu: ::core::ffi::c_int,
        pid: pid_t,
        tid: pid_t,
    ) -> ::core::ffi::c_int;
}

/*
 * For use with libtraceevent's tep_set_function_resolver()
 */
unsafe extern "C" {
    pub fn machine__resolve_kernel_addr(
        vmachine: *mut ::core::ffi::c_void,
        addrp: *mut ::core::ffi::c_ulonglong,
        modp: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;

    pub fn machine__get_kallsyms_filename(
        machine: *mut machine,
        buf: *mut ::core::ffi::c_char,
        bufsz: size_t,
    );

    pub fn machine__create_extra_kernel_maps(
        machine: *mut machine,
        kernel: *mut dso,
    ) -> ::core::ffi::c_int;
}

/* Kernel-space maps for symbols that are outside the main kernel map and module maps */
#[repr(C)]
pub struct extra_kernel_map {
    pub start: u64,
    pub end: u64,
    pub pgoff: u64,
    pub name: [::core::ffi::c_char; KMAP_NAME_LEN],
}

unsafe extern "C" {
    pub fn machine__create_extra_kernel_map(
        machine: *mut machine,
        kernel: *mut dso,
        xm: *mut extra_kernel_map,
    ) -> ::core::ffi::c_int;

    pub fn machine__map_x86_64_entry_trampolines(
        machine: *mut machine,
        kernel: *mut dso,
    ) -> ::core::ffi::c_int;

    pub fn machine__resolve(
        machine: *mut machine,
        al: *mut addr_location,
        sample: *mut perf_sample,
    ) -> ::core::ffi::c_int;

    pub fn machine__hit_all_dsos(machine: *mut machine) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
