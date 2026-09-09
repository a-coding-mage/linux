// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of trace_output.c. External kernel types/functions
 * are intentionally referenced as dependencies supplied by other files. */

const EVENT_HASH_BITS: usize = 7;

extern "C" {
    static mut trace_event_sem: RwSemaphore;
    static mut event_hash: HashTable;
    static mut trace_event_ida: Ida;
}

pub unsafe fn trace_print_bputs_msg_only(iter: *mut trace_iterator) -> print_line_t {
    let s = &mut (*iter).seq;
    let entry = (*iter).ent;
    let field: *mut bputs_entry = entry as *mut bputs_entry;
    trace_seq_puts(s, (*field).str_);
    trace_handle_return(s)
}

pub unsafe fn trace_print_bprintk_msg_only(iter: *mut trace_iterator) -> print_line_t {
    let s = &mut (*iter).seq;
    let field = (*iter).ent as *mut bprint_entry;
    trace_seq_bprintf(s, (*field).fmt, (*field).buf);
    trace_handle_return(s)
}

pub unsafe fn trace_print_printk_msg_only(iter: *mut trace_iterator) -> print_line_t {
    let s = &mut (*iter).seq;
    let field = (*iter).ent as *mut print_entry;
    trace_seq_puts(s, (*field).buf);
    trace_handle_return(s)
}

pub unsafe fn trace_print_flags_seq(p: *mut trace_seq, delim: *const c_char,
    mut flags: c_ulong, array: *const trace_print_flags, size: usize) -> *const c_char {
    let ret = trace_seq_buffer_ptr(p); let mut first = true;
    for i in 0..size {
        if flags == 0 { break; }
        let f = &*array.add(i);
        if flags & f.mask != f.mask { continue; }
        flags &= !f.mask;
        if !first && !delim.is_null() { trace_seq_puts(p, delim); } else { first = false; }
        trace_seq_puts(p, f.name);
    }
    if flags != 0 { if !first && !delim.is_null() { trace_seq_puts(p, delim); } trace_seq_printf(p, cstr!("0x%lx"), flags); }
    trace_seq_putc(p, 0); ret
}

pub unsafe fn trace_print_symbols_seq(p: *mut trace_seq, val: c_ulong,
    array: *const trace_print_flags, size: usize) -> *const c_char {
    let ret = trace_seq_buffer_ptr(p); let mut found = false;
    for i in 0..size { let f = &*array.add(i); if val == f.mask { trace_seq_puts(p, f.name); found = true; break; } }
    if !found { trace_seq_printf(p, cstr!("0x%lx"), val); }
    trace_seq_putc(p, 0); ret
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn trace_print_flags_seq_u64(p: *mut trace_seq, delim: *const c_char,
    mut flags: c_ulonglong, array: *const trace_print_flags_u64, size: usize) -> *const c_char {
    let ret = trace_seq_buffer_ptr(p); let mut first = true;
    for i in 0..size { if flags == 0 { break; } let f=&*array.add(i); if flags & f.mask != f.mask {continue;} flags &= !f.mask; if !first && !delim.is_null(){trace_seq_puts(p,delim)} else {first=false;} trace_seq_puts(p,f.name); }
    if flags != 0 { if !first && !delim.is_null(){trace_seq_puts(p,delim)} trace_seq_printf(p,cstr!("0x%llx"),flags); } trace_seq_putc(p,0); ret
}

pub unsafe fn trace_print_bitmask_seq(iter: *mut trace_iterator, ptr: *mut c_void, size: c_uint) -> *const c_char {
    let p=&mut (*iter).tmp_seq; trace_seq_init(p); let ret=trace_seq_buffer_ptr(p);
    if (*(*iter).tr).trace_flags & TRACE_ITER_BITMASK_LIST != 0 { trace_seq_bitmask_list(p,ptr,size*8); } else { trace_seq_bitmask(p,ptr,size*8); }
    trace_seq_putc(p,0); ret
}

pub unsafe fn trace_print_hex_seq(p:*mut trace_seq, buf:*const u8, len:c_int, concatenate:bool)->*const c_char { let ret=trace_seq_buffer_ptr(p); let fmt=if concatenate {cstr!("%*phN")} else {cstr!("%*ph")}; let mut i=0; while i<len { if !concatenate&&i!=0 {trace_seq_putc(p,b' ' as c_int);} trace_seq_printf(p,fmt,core::cmp::min(len-i,16),buf.add(i as usize)); i+=16;} trace_seq_putc(p,0); ret }

pub unsafe fn trace_print_array_seq(p:*mut trace_seq, buf:*const c_void, count:c_int, mut el_size:usize)->*const c_char { let ret=trace_seq_buffer_ptr(p); let mut ptr=buf as *const u8; let end=ptr.add(count as usize*el_size); let mut prefix=cstr!(""); trace_seq_putc(p,b'{' as c_int); while ptr<end { match el_size {1=>trace_seq_printf(p,cstr!("%s0x%x"),prefix,*ptr as u32),2=>trace_seq_printf(p,cstr!("%s0x%x"),prefix,*(ptr as *const u16)),4=>trace_seq_printf(p,cstr!("%s0x%x"),prefix,*(ptr as *const u32)),8=>trace_seq_printf(p,cstr!("%s0x%llx"),prefix,*(ptr as *const u64)),_=>{trace_seq_printf(p,cstr!("BAD SIZE:%zu 0x%x"),el_size,*ptr as u32);el_size=1}} prefix=cstr!(","); ptr=ptr.add(el_size);} trace_seq_putc(p,b'}' as c_int); trace_seq_putc(p,0); ret }

pub unsafe fn trace_print_hex_dump_seq(p:*mut trace_seq,prefix:*const c_char,ptype:c_int,rowsize:c_int,groupsize:c_int,buf:*const c_void,len:usize,ascii:bool)->*const c_char { let ret=trace_seq_buffer_ptr(p); trace_seq_putc(p,b'\n' as c_int); trace_seq_hex_dump(p,prefix,ptype,rowsize,groupsize,buf,len,ascii); trace_seq_putc(p,0); ret }

pub unsafe fn trace_raw_output_prep(iter:*mut trace_iterator, ev:*mut trace_event)->c_int { let event=container_of_trace_event_call(ev); let s=&mut (*iter).seq; trace_seq_init(&mut (*iter).tmp_seq); if (*(*iter).ent).type_ != (*event).event.type_ { WARN_ON_ONCE(1); return TRACE_TYPE_UNHANDLED; } trace_seq_printf(s,cstr!("%s: "),trace_event_name(event)); trace_handle_return(s) }

pub unsafe fn trace_event_printf(iter:*mut trace_iterator, fmt:*const c_char, mut args:...){ if ignore_event(iter){return;} trace_seq_vprintf(&mut (*iter).seq,trace_event_format(iter,fmt),args); }

// The remaining declarations preserve the source-level interfaces and kernel control flow.
extern "C" {
    fn trace_seq_puts(_: *mut trace_seq, _: *const c_char); fn trace_seq_putc(_: *mut trace_seq, _: c_int); fn trace_seq_printf(_: *mut trace_seq, _: *const c_char, ...)->c_int;
    fn trace_seq_buffer_ptr(_: *mut trace_seq)->*const c_char; fn trace_seq_init(_: *mut trace_seq); fn trace_handle_return(_: *mut trace_seq)->print_line_t;
    fn trace_seq_bprintf(_: *mut trace_seq, _: *const c_char, _: *const c_uchar); fn trace_seq_bitmask(_: *mut trace_seq,*mut c_void,usize); fn trace_seq_bitmask_list(_: *mut trace_seq,*mut c_void,usize); fn trace_seq_hex_dump(_: *mut trace_seq,*const c_char,c_int,c_int,c_int,*const c_void,usize,bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
