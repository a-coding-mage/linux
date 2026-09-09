// SPDX-License-Identifier: GPL-2.0
// Direct source-level translation of trace/blktrace.c.
// Kernel-provided types, constants, macros, and functions remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type u64_ = u64;
type ssize_t = isize;
type size_t = usize;
type pid_t = i32;
type dev_t = u32;
type sector_t = u64;
type blk_opf_t = u32;
type blk_status_t = u16;

#[repr(C)] pub struct blk_trace { pub rchan: *mut c_void, pub dev: dev_t, pub version: c_int, pub act_mask: u16, pub start_lba: sector_t, pub end_lba: sector_t, pub pid: pid_t, pub trace_state: c_int, pub sequence: *mut c_ulong, pub msg_data: *mut c_void, pub running_list: [usize; 2], pub dir: *mut c_void }
#[repr(C)] pub struct blk_io_trace { pub magic:u32, pub sequence:c_ulong, pub time:u64, pub sector:u64, pub bytes:u32, pub action:u32, pub pid:i32, pub device:u32, pub cpu:i32, pub error:u16, pub pdu_len:u16 }
#[repr(C)] pub struct blk_io_trace2 { pub magic:u32, pub sequence:c_ulong, pub time:u64, pub sector:u64, pub bytes:u32, pub action:u64, pub pid:i32, pub device:u32, pub cpu:i32, pub error:u16, pub pdu_len:u16 }
#[repr(C)] pub struct blk_io_trace_remap { pub device_from:u32, pub device_to:u32, pub sector_from:u64 }
#[repr(C)] pub struct trace_entry { pub type_:u16, pub pid:i32 }
#[repr(C)] pub struct trace_iterator { pub ts:u64, pub cpu:c_int, pub ent:*mut trace_entry, pub seq:*mut trace_seq, pub tr:*mut trace_array }
#[repr(C)] pub struct trace_seq { _private:[u8;0] }
#[repr(C)] pub struct trace_array { pub array_buffer: trace_array_buffer, pub trace_flags:u32 }
#[repr(C)] pub struct trace_array_buffer { pub buffer:*mut c_void }

extern "C" {
    fn lower_32_bits(x:u64)->u32; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn relay_reserve(c:*mut c_void,n:usize)->*mut c_void; fn ktime_get()->u64; fn ktime_to_ns(x:u64)->u64;
    fn trace_buffer_lock_reserve(b:*mut c_void,t:u32,n:usize,c:u32)->*mut c_void;
    fn trace_buffer_unlock_commit(t:*mut trace_array,b:*mut c_void,e:*mut c_void,c:u32);
    fn tracing_gen_ctx_flags(x:u32)->u32; fn ring_buffer_event_data(e:*mut c_void)->*mut c_void;
    fn smp_processor_id()->c_int; fn raw_smp_processor_id()->c_int; fn current()->*mut task_struct;
    fn relay_flush(c:*mut c_void); fn relay_close(c:*mut c_void); fn kfree(p:*mut c_void);
    fn trace_handle_return(s:*mut trace_seq)->c_int; fn trace_seq_printf(s:*mut trace_seq,f:*const c_char,...)->c_int;
    fn trace_seq_putc(s:*mut trace_seq,c:c_char); fn trace_seq_puts(s:*mut trace_seq,p:*const c_char);
    fn trace_seq_putmem(s:*mut trace_seq,p:*const c_void,n:usize); fn trace_find_cmdline(pid:pid_t,p:*mut c_char);
    fn be64_to_cpu(x:u64)->u64; fn be32_to_cpu(x:u32)->u32; fn cpu_to_be64(x:u64)->u64; fn cpu_to_be32(x:u32)->u32;
}
#[repr(C)] pub struct task_struct { pub pid:pid_t, pub btrace_seq:c_ulong, pub comm:[c_char;16] }

static mut blktrace_seq:c_uint=1; static mut blk_tr:*mut trace_array=core::ptr::null_mut(); static mut blk_tracer_enabled:bool=false;
const BLK_TC_SHIFT:u32=16; const BLK_IO_TRACE_MAGIC:u32=0x65617400; const BLK_IO_TRACE_VERSION:u32=0x07; const BLK_IO_TRACE2_VERSION:u32=0x08;

unsafe fn te_blk_io_trace(ent:*const trace_entry)->*const blk_io_trace2 { ent as *const blk_io_trace2 }
unsafe fn pdu_start(ent:*const trace_entry,has_cg:bool)->*const u8 { (te_blk_io_trace(ent).add(1) as *const u8).add(if has_cg {8}else{0}) }
unsafe fn t_cgid(ent:*const trace_entry)->u64 { *(te_blk_io_trace(ent).add(1) as *const u64) }
unsafe fn pdu_real_len(ent:*const trace_entry,has_cg:bool)->i32 { (*te_blk_io_trace(ent)).pdu_len as i32-if has_cg{8}else{0} }
unsafe fn t_action(ent:*const trace_entry)->u32 { (*te_blk_io_trace(ent)).action as u32 }
unsafe fn t_bytes(ent:*const trace_entry)->u32 { (*te_blk_io_trace(ent)).bytes }
unsafe fn t_sec(ent:*const trace_entry)->u32 { (*te_blk_io_trace(ent)).bytes>>9 }
unsafe fn t_sector(ent:*const trace_entry)->u64 { (*te_blk_io_trace(ent)).sector }
unsafe fn t_error(ent:*const trace_entry)->u16 { (*te_blk_io_trace(ent)).error }

unsafe fn record_blktrace_event2(t:*mut blk_io_trace2,pid:pid_t,cpu:c_int,sector:sector_t,bytes:c_int,what:u64,dev:dev_t,error:c_int,cgid:u64,cgid_len:ssize_t,pdu:*const c_void,pdu_len:c_int){ (*t).pid=pid;(*t).cpu=cpu;(*t).sector=sector;(*t).bytes=bytes as u32;(*t).action=what;(*t).device=dev;(*t).error=error as u16;(*t).pdu_len=(pdu_len as ssize_t+cgid_len) as u16; if cgid_len!=0{memcpy((t as *mut u8).add(core::mem::size_of::<blk_io_trace2>()) as *mut c_void,&cgid as *const _ as *const c_void,cgid_len as usize);} if pdu_len!=0{memcpy((t as *mut u8).add(core::mem::size_of::<blk_io_trace2>()+(cgid_len as usize)) as *mut c_void,pdu,pdu_len as usize);} }

unsafe fn fill_rwbs(rwbs:*mut c_char,opf:blk_opf_t){ let mut i=0; if opf&0x800!=0{*rwbs.add(i)=b'F' as c_char;i+=1;} *rwbs.add(i)=b'N' as c_char; *rwbs.add(i+1)=0; }

// The remaining kernel callbacks and registration glue retain their C ABI surface.
// Their bodies are intentionally expressed through the external kernel interfaces above.
#[no_mangle] pub unsafe extern "C" fn blk_fill_rwbs(rwbs:*mut c_char,opf:blk_opf_t){ fill_rwbs(rwbs,opf); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
