// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/arch/x86/util/event.c.
// C includes referenced:
// <linux/types.h>, <linux/string.h>, <linux/zalloc.h>, <stdlib.h>
// ../../../util/event.h, ../../../util/synthetic-events.h,
// ../../../util/machine.h, ../../../util/tool.h, ../../../util/map.h,
// ../../../util/debug.h, util/sample.h

#[cfg(target_arch = "x86_64")]
use core::ffi::{c_char, c_int, c_void};

#[cfg(target_arch = "x86_64")]
extern "C" {
    static PATH_MAX: usize;
    static PERF_RECORD_MMAP: u32;
    static PERF_RECORD_MISC_KERNEL: u16;
    static PERF_RECORD_MISC_GUEST_KERNEL: u16;

    fn strlen(s: *const c_char) -> usize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn __map__is_extra_kernel_map(map: *mut map) -> bool;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn map__start(map: *mut map) -> u64;
    fn map__size(map: *mut map) -> u64;
    fn map__pgoff(map: *mut map) -> u64;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn maps__for_each_map(
        maps: *mut maps,
        cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn perf_tool__process_synth_event(
        tool: *const perf_tool,
        event: *mut perf_event,
        machine: *mut machine,
        process: perf_event__handler_t,
    ) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct machine {
    pub id_hdr_size: usize,
    pub pid: u32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct kmap {
    pub name: *const c_char,
}

#[cfg(target_arch = "x86_64")]
pub type perf_event__handler_t = Option<unsafe extern "C" fn() -> c_int>;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_mmap {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: [c_char; 1],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub mmap: perf_record_mmap,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct perf_event__synthesize_extra_kmaps_cb_args {
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
    event: *mut perf_event,
}

#[cfg(target_arch = "x86_64")]
const fn perf_align(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn perf_event__synthesize_extra_kmaps_cb(
    map: *mut map,
    data: *mut c_void,
) -> c_int {
    let args = data as *mut perf_event__synthesize_extra_kmaps_cb_args;
    let event = (*args).event;
    let kmap: *mut kmap;
    let size: usize;

    if !__map__is_extra_kernel_map(map) {
        return 0;
    }

    kmap = map__kmap(map);

    size = core::mem::size_of::<perf_record_mmap>()
        - core::mem::size_of_val(&(*event).mmap.filename)
        + perf_align(strlen((*kmap).name) + 1, core::mem::size_of::<u64>())
        + (*(*args).machine).id_hdr_size;

    memset(event as *mut c_void, 0, size);

    (*event).mmap.header.type_ = PERF_RECORD_MMAP;

    /*
     * kernel uses 0 for user space maps, see kernel/perf_event.c
     * __perf_event_mmap
     */
    if machine__is_host((*args).machine) {
        (*event).header.misc = PERF_RECORD_MISC_KERNEL;
    } else {
        (*event).header.misc = PERF_RECORD_MISC_GUEST_KERNEL;
    }

    (*event).mmap.header.size = size as u16;

    (*event).mmap.start = map__start(map);
    (*event).mmap.len = map__size(map);
    (*event).mmap.pgoff = map__pgoff(map);
    (*event).mmap.pid = (*(*args).machine).pid;

    strlcpy((*event).mmap.filename.as_mut_ptr(), (*kmap).name, PATH_MAX);

    if perf_tool__process_synth_event((*args).tool, event, (*args).machine, (*args).process) != 0 {
        return -1;
    }

    0
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_extra_kmaps(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let rc: c_int;
    let kmaps = machine__kernel_maps(machine);
    let mut args = perf_event__synthesize_extra_kmaps_cb_args {
        tool,
        process,
        machine,
        event: zalloc(core::mem::size_of::<perf_record_mmap>() + (*machine).id_hdr_size)
            as *mut perf_event,
    };

    if args.event.is_null() {
        pr_debug(
            b"Not enough memory synthesizing mmap event for extra kernel maps\n\0".as_ptr()
                as *const c_char,
        );
        return -1;
    }

    rc = maps__for_each_map(
        kmaps,
        Some(perf_event__synthesize_extra_kmaps_cb),
        &mut args as *mut perf_event__synthesize_extra_kmaps_cb_args as *mut c_void,
    );

    free(args.event as *mut c_void);
    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
