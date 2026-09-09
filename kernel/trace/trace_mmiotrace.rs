// SPDX-License-Identifier: GPL-2.0
/*
 * Memory mapped I/O tracing
 *
 * Copyright (C) 2008 Pekka Paalanen <pq@iki.fi>
 */

// Linux kernel and local trace dependencies are supplied by other files.

#[repr(C)]
struct header_iter {
    dev: *mut pci_dev,
}

static mut mmio_trace_array: *mut trace_array = core::ptr::null_mut();
static mut overrun_detected: bool = false;
static mut prev_overruns: c_ulong = 0;
static mut dropped_count: atomic_t = atomic_t { counter: 0 };

unsafe fn mmio_reset_data(tr: *mut trace_array) {
    overrun_detected = false;
    prev_overruns = 0;
    atomic_set(&mut dropped_count, 0);

    tracing_reset_online_cpus(&mut (*tr).array_buffer);
}

unsafe fn mmio_trace_init(tr: *mut trace_array) -> c_int {
    pr_debug!("in {}\n", "mmio_trace_init");
    mmio_trace_array = tr;

    mmio_reset_data(tr);
    enable_mmiotrace();
    0
}

unsafe fn mmio_trace_reset(tr: *mut trace_array) {
    pr_debug!("in {}\n", "mmio_trace_reset");

    disable_mmiotrace();
    mmio_reset_data(tr);
    mmio_trace_array = core::ptr::null_mut();
}

unsafe fn mmio_trace_start(tr: *mut trace_array) {
    pr_debug!("in {}\n", "mmio_trace_start");
    mmio_reset_data(tr);
}

unsafe fn mmio_print_pcidev(s: *mut trace_seq, dev: *const pci_dev) {
    let mut i: c_int;
    let mut start: resource_size_t;
    let mut end: resource_size_t;
    let drv = pci_dev_driver(dev);

    trace_seq_printf(s, "PCIDEV %02x%02x %04x%04x %x", (*(*dev).bus).number,
        (*dev).devfn, (*dev).vendor, (*dev).device, (*dev).irq);
    i = 0;
    while i < 7 {
        start = (*dev).resource[i as usize].start;
        trace_seq_printf(s, " %llx", (start | ((*dev).resource[i as usize].flags & PCI_REGION_FLAG_MASK)) as c_ulonglong);
        i += 1;
    }
    i = 0;
    while i < 7 {
        start = (*dev).resource[i as usize].start;
        end = (*dev).resource[i as usize].end;
        trace_seq_printf(s, " %llx", if start < end { (end - start + 1) as c_ulonglong } else { 0 });
        i += 1;
    }
    if !drv.is_null() {
        trace_seq_printf(s, " %s\n", (*drv).name);
    } else {
        trace_seq_puts(s, " \n");
    }
}

unsafe fn destroy_header_iter(hiter: *mut header_iter) {
    if hiter.is_null() { return; }
    pci_dev_put((*hiter).dev);
    kfree(hiter as *mut c_void);
}

unsafe fn mmio_pipe_open(iter: *mut trace_iterator) {
    let s = &mut (*iter).seq;
    trace_seq_puts(s, "VERSION 20070824\n");

    let hiter = kzalloc::<header_iter>();
    if hiter.is_null() { return; }

    (*hiter).dev = pci_get_device(PCI_ANY_ID, PCI_ANY_ID, core::ptr::null_mut());
    (*iter).private = hiter as *mut c_void;
}

unsafe fn mmio_close(iter: *mut trace_iterator) {
    let hiter = (*iter).private as *mut header_iter;
    destroy_header_iter(hiter);
    (*iter).private = core::ptr::null_mut();
}

unsafe fn count_overruns(iter: *mut trace_iterator) -> c_ulong {
    let cnt = atomic_xchg(&mut dropped_count, 0) as c_ulong;
    let over = ring_buffer_overruns((*iter).array_buffer.buffer);
    let mut result = cnt;
    if over > prev_overruns { result += over - prev_overruns; }
    prev_overruns = over;
    result
}

unsafe fn mmio_read(iter: *mut trace_iterator, _filp: *mut file, ubuf: *mut c_char,
                    cnt: usize, _ppos: *mut loff_t) -> isize {
    let hiter = (*iter).private as *mut header_iter;
    let s = &mut (*iter).seq;
    let n = count_overruns(iter);
    if n != 0 {
        trace_seq_printf(s, "MARK 0.000000 Lost %lu events.\n", n);
        if !overrun_detected { pr_warn!("mmiotrace has lost events\n"); }
        overrun_detected = true;
        let ret = trace_seq_to_user(s, ubuf, cnt);
        return if ret == -EBUSY { 0 } else { ret };
    }
    if hiter.is_null() || (*hiter).dev.is_null() { return 0; }
    mmio_print_pcidev(s, (*hiter).dev);
    (*hiter).dev = pci_get_device(PCI_ANY_ID, PCI_ANY_ID, (*hiter).dev);
    if (*hiter).dev.is_null() { destroy_header_iter(hiter); (*iter).private = core::ptr::null_mut(); }
    let ret = trace_seq_to_user(s, ubuf, cnt);
    if ret == -EBUSY { 0 } else { ret }
}

unsafe fn mmio_print_rw(iter: *mut trace_iterator) -> print_line_t {
    let entry = (*iter).ent;
    let field = entry as *mut trace_mmiotrace_rw;
    let rw = &(*field).rw;
    let s = &mut (*iter).seq;
    let mut t = ns2usecs((*iter).ts);
    let usec_rem = t % USEC_PER_SEC;
    t /= USEC_PER_SEC;
    let secs = t as c_ulong;
    match rw.opcode {
        MMIO_READ | MMIO_WRITE => {
            let p = if rw.opcode == MMIO_READ { "R" } else { "W" };
            trace_seq_printf(s, "{} {} {}.{:06} {} 0x{:x} 0x{:x} 0x{:x} 0\n", p, rw.width, secs, usec_rem, rw.map_id, rw.phys, rw.value, rw.pc);
        },
        MMIO_UNKNOWN_OP => trace_seq_printf(s, "UNKNOWN {}.{:06} {} 0x{:x} {:02x},{:02x},{:02x} 0x{:x} 0\n", secs, usec_rem, rw.map_id, rw.phys, (rw.value >> 16) & 0xff, (rw.value >> 8) & 0xff, rw.value & 0xff, rw.pc),
        _ => trace_seq_puts(s, "rw what?\n"),
    }
    trace_handle_return(s)
}

unsafe fn mmio_print_map(iter: *mut trace_iterator) -> print_line_t {
    let m = &(*( (*iter).ent as *mut trace_mmiotrace_map)).map;
    let s = &mut (*iter).seq;
    let mut t = ns2usecs((*iter).ts);
    let usec_rem = t % USEC_PER_SEC; t /= USEC_PER_SEC;
    match m.opcode {
        MMIO_PROBE => trace_seq_printf(s, "MAP {}.{:06} {} 0x{:x} 0x{:x} 0x{:x} 0x0 0\n", t, usec_rem, m.map_id, m.phys, m.virt, m.len),
        MMIO_UNPROBE => trace_seq_printf(s, "UNMAP {}.{:06} {} 0x0 0\n", t, usec_rem, m.map_id),
        _ => trace_seq_puts(s, "map what?\n"),
    }
    trace_handle_return(s)
}

unsafe fn mmio_print_mark(iter: *mut trace_iterator) -> print_line_t {
    let print = (*iter).ent as *mut print_entry;
    let mut t = ns2usecs((*iter).ts);
    let usec_rem = t % USEC_PER_SEC; t /= USEC_PER_SEC;
    trace_seq_printf(&mut (*iter).seq, "MARK {}.{:06} {}", t, usec_rem, (*print).buf);
    trace_handle_return(&mut (*iter).seq)
}

unsafe fn mmio_print_line(iter: *mut trace_iterator) -> print_line_t {
    match (*(*iter).ent).type_ {
        TRACE_MMIO_RW => mmio_print_rw(iter),
        TRACE_MMIO_MAP => mmio_print_map(iter),
        TRACE_PRINT => mmio_print_mark(iter),
        _ => TRACE_TYPE_HANDLED,
    }
}

static mut mmio_tracer: tracer = tracer {
    name: "mmiotrace", init: Some(mmio_trace_init), reset: Some(mmio_trace_reset),
    start: Some(mmio_trace_start), pipe_open: Some(mmio_pipe_open), close: Some(mmio_close),
    pipe_close: Some(mmio_close), read: Some(mmio_read), print_line: Some(mmio_print_line), noboot: true,
};

unsafe fn init_mmio_trace() -> c_int { register_tracer(&mut mmio_tracer) }

unsafe fn __trace_mmiotrace_rw(tr: *mut trace_array, rw: *mut mmiotrace_rw) {
    if tr.is_null() { return; }
    let buffer = (*tr).array_buffer.buffer;
    let trace_ctx = tracing_gen_ctx_flags(0);
    let event = trace_buffer_lock_reserve(buffer, TRACE_MMIO_RW, core::mem::size_of::<trace_mmiotrace_rw>(), trace_ctx);
    if event.is_null() { atomic_inc(&mut dropped_count); return; }
    (*(ring_buffer_event_data(event) as *mut trace_mmiotrace_rw)).rw = *rw;
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
}

unsafe fn mmio_trace_rw(rw: *mut mmiotrace_rw) { __trace_mmiotrace_rw(mmio_trace_array, rw); }

unsafe fn __trace_mmiotrace_map(tr: *mut trace_array, map: *mut mmiotrace_map) {
    if tr.is_null() { return; }
    let buffer = (*tr).array_buffer.buffer;
    let trace_ctx = tracing_gen_ctx_flags(0);
    let event = trace_buffer_lock_reserve(buffer, TRACE_MMIO_MAP, core::mem::size_of::<trace_mmiotrace_map>(), trace_ctx);
    if event.is_null() { atomic_inc(&mut dropped_count); return; }
    (*(ring_buffer_event_data(event) as *mut trace_mmiotrace_map)).map = *map;
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
}

unsafe fn mmio_trace_mapping(map: *mut mmiotrace_map) { __trace_mmiotrace_map(mmio_trace_array, map); }

unsafe fn mmio_trace_printk(fmt: *const c_char, args: va_list) -> c_int { trace_vprintk(0, fmt, args) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
