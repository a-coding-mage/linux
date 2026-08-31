use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const ENOMEM: c_int = 12;

#[repr(C)]
pub struct rb_node {
    pub rb_parent_color: c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct memory_node {
    pub set: *mut c_ulong,
    pub size: u64,
    pub node: u64,
}

#[repr(C)]
pub struct perf_env {
    pub memory_nodes: *mut memory_node,
    pub memory_bsize: u64,
    pub nr_memory_nodes: c_int,
}

#[repr(C)]
pub struct mem2node {
    pub root: rb_root,
    pub entries: *mut phys_entry,
}

#[repr(C)]
pub struct phys_entry {
    pub rb_node: rb_node,
    pub start: u64,
    pub end: u64,
    pub node: u64,
}

unsafe extern "C" {
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn bitmap_weight(src: *const c_ulong, nbits: c_uint) -> c_uint;
    fn test_bit(nr: c_ulong, addr: *const c_ulong) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut phys_entry);
    fn pr_debug(fmt: *const c_char, ...);
}

type c_uint = u32;

#[inline]
unsafe fn rb_entry_phys_entry(ptr: *mut rb_node) -> *mut phys_entry {
    (ptr as *mut u8).sub(offset_of!(phys_entry, rb_node)) as *mut phys_entry
}

#[inline]
unsafe fn rb_clear_node(node: *mut rb_node) {
    (*node).rb_parent_color = node as c_ulong;
}

unsafe fn warn_once_no_memory_nodes(condition: bool) -> bool {
    static mut WARNED: bool = false;

    if condition {
        let already_warned = WARNED;
        if !already_warned {
            WARNED = true;
            pr_debug(c"No memory nodes, is CONFIG_MEMORY_HOTPLUG enabled?\n".as_ptr());
        }
        true
    } else {
        false
    }
}

unsafe fn phys_entry__insert(entry: *mut phys_entry, root: *mut rb_root) {
    let mut p: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut e: *mut phys_entry;

    while !(*p).is_null() {
        parent = *p;
        e = rb_entry_phys_entry(parent);

        if (*entry).start < (*e).start {
            p = &mut (*(*p)).rb_left;
        } else {
            p = &mut (*(*p)).rb_right;
        }
    }

    rb_link_node(&mut (*entry).rb_node, parent, p);
    rb_insert_color(&mut (*entry).rb_node, root);
}

unsafe fn phys_entry__init(entry: *mut phys_entry, start: u64, bsize: u64, node: u64) {
    (*entry).start = start;
    (*entry).end = start.wrapping_add(bsize);
    (*entry).node = node;
    rb_clear_node(&mut (*entry).rb_node);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem2node__init(map: *mut mem2node, env: *mut perf_env) -> c_int {
    let nodes: *mut memory_node = (*env).memory_nodes;
    let mut n: *mut memory_node;
    let mut entries: *mut phys_entry;
    let mut tmp_entries: *mut phys_entry;
    let bsize: u64 = (*env).memory_bsize;
    let mut i: c_int;
    let mut j: c_int = 0;
    let mut max: c_int = 0;

    memset(map as *mut c_void, 0x0, size_of::<mem2node>());
    (*map).root = rb_root {
        rb_node: ptr::null_mut(),
    };

    i = 0;
    while i < (*env).nr_memory_nodes {
        n = nodes.add(i as usize);
        max += bitmap_weight((*n).set, (*n).size as c_uint) as c_int;
        i += 1;
    }

    entries = calloc(max as usize, size_of::<phys_entry>()) as *mut phys_entry;
    if entries.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*env).nr_memory_nodes {
        let mut bit: u64;

        n = nodes.add(i as usize);

        bit = 0;
        while bit < (*n).size {
            let start: u64;

            if test_bit(bit as c_ulong, (*n).set) == 0 {
                bit += 1;
                continue;
            }

            start = bit.wrapping_mul(bsize);

            /*
             * Merge nearby areas, we walk in order
             * through the bitmap, so no need to sort.
             */
            if j > 0 {
                let prev: *mut phys_entry = entries.add((j - 1) as usize);

                if (*prev).end == start && (*prev).node == (*n).node {
                    (*prev).end = (*prev).end.wrapping_add(bsize);
                    bit += 1;
                    continue;
                }
            }

            phys_entry__init(entries.add(j as usize), start, bsize, (*n).node);
            j += 1;
            bit += 1;
        }

        i += 1;
    }

    /* Cut unused entries, due to merging. */
    tmp_entries = realloc(
        entries as *mut c_void,
        size_of::<phys_entry>().wrapping_mul(j as usize),
    ) as *mut phys_entry;
    if !tmp_entries.is_null() || warn_once_no_memory_nodes(j == 0) {
        entries = tmp_entries;
    }

    i = 0;
    while i < j {
        pr_debug(
            c"mem2node %03llu [0x%016llx-0x%016llx]\n".as_ptr(),
            (*entries.add(i as usize)).node,
            (*entries.add(i as usize)).start,
            (*entries.add(i as usize)).end,
        );

        phys_entry__insert(entries.add(i as usize), &mut (*map).root);
        i += 1;
    }

    (*map).entries = entries;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem2node__exit(map: *mut mem2node) {
    zfree(&mut (*map).entries);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem2node__node(map: *mut mem2node, addr: u64) -> c_int {
    let mut p: *mut *mut rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut entry: *mut phys_entry;

    p = &mut (*map).root.rb_node;
    while !(*p).is_null() {
        parent = *p;
        entry = rb_entry_phys_entry(parent);
        if addr < (*entry).start {
            p = &mut (*(*p)).rb_left;
        } else if addr >= (*entry).end {
            p = &mut (*(*p)).rb_right;
        } else {
            return (*entry).node as c_int;
        }
    }

    -1
}
