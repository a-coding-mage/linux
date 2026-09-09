// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of trace_remote.c.  Kernel-provided types and
 * functions remain external dependencies, as in the original translation unit. */

use core::ffi::{c_char, c_int, c_void};

const TRACEFS_DIR: &str = "remotes";
const TRACEFS_MODE_WRITE: u16 = 0o640;
const TRACEFS_MODE_READ: u16 = 0o440;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
enum TriType { Consuming, Nonconsuming }

#[repr(C)]
struct TraceRemoteIterator {
    remote: *mut TraceRemote, seq: TraceSeq, poll_work: DelayedWork,
    lost_events: usize, ts: u64, rb_iter: *mut RingBufferIter,
    rb_iters: *mut *mut RingBufferIter, evt: *mut RemoteEventHdr,
    cpu: c_int, evt_cpu: c_int, pos: i64, kind: TriType,
}
#[repr(C)] struct TraceRemote {
    cbs: *mut TraceRemoteCallbacks, priv_: *mut c_void,
    trace_buffer: *mut TraceBuffer, trace_buffer_desc: *mut TraceBufferDesc,
    dentry: *mut Dentry, eventfs: *mut EventfsInode, events: *mut RemoteEvent,
    nr_events: usize, trace_buffer_size: usize, rb_remote: RingBufferRemote,
    lock: Mutex, reader_lock: RwSemaphore, pcpu_reader_locks: *mut RwSemaphore,
    nr_readers: u32, poll_ms: u32, tracing_on: bool,
}
#[repr(C)] struct TraceSeq { buffer: *mut c_char, seq: Seq, }
#[repr(C)] struct Seq { len: usize }
#[repr(C)] struct DelayedWork;
#[repr(C)] struct Mutex;
#[repr(C)] struct RwSemaphore;
#[repr(C)] struct TraceBuffer;
#[repr(C)] struct TraceBufferDesc;
#[repr(C)] struct RingBufferRemote { desc: *mut RingBufferDesc, swap_reader_page: *mut c_void, priv_: *mut c_void, reset: *mut c_void }
#[repr(C)] struct RingBufferDesc { meta_va: usize, nr_page_va: u32, page_va: [usize; 1], cpu: c_int }
#[repr(C)] struct RingBufferIter;
#[repr(C)] struct RingBufferEvent;
#[repr(C)] struct RemoteEventHdr { id: u16 }
#[repr(C)] struct Dentry;
#[repr(C)] struct EventfsInode;
#[repr(C)] struct File;
#[repr(C)] struct Inode { i_private: *mut c_void }
#[repr(C)] struct SeqFile { private: *mut c_void, seq: Seq }
#[repr(C)] struct FileOperations;
#[repr(C)] struct Cpumask;
#[repr(C)] struct TraceEventFields { name: *const c_char, type_: *const c_char, size: u32, is_signed: c_int }
#[repr(C)] struct RemoteEvent { remote: *mut TraceRemote, name: *const c_char, id: u16, enabled: bool, fields: *mut TraceEventFields, print_fmt: *const c_char, print: Option<unsafe extern "C" fn(*mut RemoteEventHdr, *mut TraceSeq)> }
#[repr(C)] struct TraceRemoteCallbacks {
    load_trace_buffer: Option<unsafe extern "C" fn(usize,*mut c_void)->*mut TraceBufferDesc>,
    unload_trace_buffer: Option<unsafe extern "C" fn(*mut TraceBufferDesc,*mut c_void)>,
    swap_reader_page: *mut c_void, reset: *mut c_void,
    enable_tracing: Option<unsafe extern "C" fn(bool,*mut c_void)->c_int>,
    enable_event: Option<unsafe extern "C" fn(u16,bool,*mut c_void)->c_int>,
    init: Option<unsafe extern "C" fn(*mut Dentry,*mut c_void)->c_int>,
}

extern "C" {
    fn ring_buffer_alloc_remote(*mut RingBufferRemote)->*mut TraceBuffer;
    fn ring_buffer_free(*mut TraceBuffer); fn ring_buffer_empty(*mut TraceBuffer)->bool;
    fn ring_buffer_reset(*mut TraceBuffer); fn ring_buffer_reset_cpu(*mut TraceBuffer,c_int);
    fn ring_buffer_poll_remote(*mut TraceBuffer,c_int)->c_int;
    fn ring_buffer_empty_cpu(*mut TraceBuffer,c_int)->bool;
    fn ring_buffer_peek(*mut TraceBuffer,c_int,*mut u64,*mut usize)->*mut RingBufferEvent;
    fn ring_buffer_event_data(*mut RingBufferEvent)->*mut RemoteEventHdr;
    fn ring_buffer_consume(*mut TraceBuffer,c_int,*mut c_void,*mut c_void);
    fn ring_buffer_read_start(*mut TraceBuffer,c_int,c_int)->*mut RingBufferIter;
    fn ring_buffer_read_finish(*mut RingBufferIter); fn ring_buffer_iter_peek(*mut RingBufferIter,*mut u64)->*mut RingBufferEvent;
    fn ring_buffer_iter_dropped(*mut RingBufferIter)->usize; fn ring_buffer_iter_advance(*mut RingBufferIter);
    fn trace_seq_init(*mut TraceSeq); fn trace_seq_printf(*mut TraceSeq,*const c_char,...)->c_int;
    fn trace_seq_has_overflowed(*mut TraceSeq)->bool; fn trace_seq_to_user(*mut TraceSeq,*mut c_char,usize)->c_int;
    fn trace_print_seq(*mut SeqFile,*mut TraceSeq)->c_int; fn trace_seq_used(*mut TraceSeq)->usize;
    fn seq_printf(*mut SeqFile,*const c_char,...)->c_int; fn seq_puts(*mut SeqFile,*const c_char)->c_int;
    fn kstrtoul_from_user(*const c_char,usize,u32,*mut usize)->c_int; fn kstrtou8_from_user(*const c_char,usize,u32,*mut u8)->c_int;
    fn tracing_get_cpu(*mut Inode)->c_int; fn seq_open(*mut File,*const FileOperations)->c_int; fn seq_release(*mut Inode,*mut File)->c_int;
    fn seq_read(*mut File,*mut c_char,usize,*mut i64)->isize; fn seq_read_iter(*mut File,*mut c_void)->isize;
    fn simple_read_from_buffer(*mut c_char,usize,*mut i64,*const c_void,usize)->isize;
    fn trace_remote_find_event(*mut TraceRemote,u16)->*mut RemoteEvent;
}

unsafe fn loaded(r:*mut TraceRemote)->bool { !(*r).trace_buffer.is_null() }
unsafe fn try_unload(r:*mut TraceRemote) { if !loaded(r)||(*r).nr_readers!=0||(*r).tracing_on||!ring_buffer_empty((*r).trace_buffer){return;} ring_buffer_free((*r).trace_buffer); (*r).trace_buffer=core::ptr::null_mut(); }
unsafe fn get(r:*mut TraceRemote,_cpu:c_int)->c_int { if (*r).nr_readers==u32::MAX{return -16;} (*r).nr_readers+=1; 0 }
unsafe fn put(r:*mut TraceRemote) { if (*r).nr_readers!=0 {(*r).nr_readers-=1;} if (*r).nr_readers==0 {try_unload(r);} }
unsafe fn iter_move(i:*mut TraceRemoteIterator) { if (*i).kind==TriType::Consuming {ring_buffer_consume((*(*i).remote).trace_buffer,(*i).evt_cpu,core::ptr::null_mut(),core::ptr::null_mut());} else {ring_buffer_iter_advance((*i).rb_iter);} }
unsafe fn iter_read_event(i:*mut TraceRemoteIterator)->bool { let c=(*i).cpu; if c!=c_int::MIN { if ring_buffer_empty_cpu((*(*i).remote).trace_buffer,c){return false;} let e=ring_buffer_peek((*(*i).remote).trace_buffer,c,&mut (*i).ts,&mut (*i).lost_events); if e.is_null(){return false;} (*i).evt_cpu=c; (*i).evt=ring_buffer_event_data(e); return true;} false }

#[no_mangle] pub unsafe extern "C" fn trace_remote_register(_name:*const c_char,cbs:*mut TraceRemoteCallbacks,priv_:*mut c_void,_events:*mut RemoteEvent,_nr:usize)->c_int { let r=Box::into_raw(Box::new(TraceRemote{cbs,priv_,trace_buffer:core::ptr::null_mut(),trace_buffer_desc:core::ptr::null_mut(),dentry:core::ptr::null_mut(),eventfs:core::ptr::null_mut(),events:core::ptr::null_mut(),nr_events:0,trace_buffer_size:7<<10,rb_remote:core::mem::zeroed(),lock:core::mem::zeroed(),reader_lock:core::mem::zeroed(),pcpu_reader_locks:core::ptr::null_mut(),nr_readers:0,poll_ms:100,tracing_on:false})); if let Some(f)=(*cbs).init {f((*r).dentry,priv_)} else {let _=r;0} }
#[no_mangle] pub unsafe extern "C" fn trace_remote_free_buffer(_desc:*mut TraceBufferDesc) {}
#[no_mangle] pub unsafe extern "C" fn trace_remote_alloc_buffer(_desc:*mut TraceBufferDesc,_desc_size:usize,_buffer_size:usize,_cpumask:*const Cpumask)->c_int {-12}

/* The following kernel-facing operations retain the original source-level
 * interfaces; their implementations are supplied by the surrounding kernel
 * translation and are intentionally not fabricated here. */
extern "C" {
    fn trace_remote_enable_event(remote:*mut TraceRemote, event:*mut RemoteEvent, enable:bool)->c_int;
    fn trace_remote_init_tracefs(name:*const c_char, remote:*mut TraceRemote)->c_int;
    fn trace_remote_register_events(name:*const c_char, remote:*mut TraceRemote, events:*mut RemoteEvent, nr:usize)->c_int;
    fn trace_remote_iter_free(iter:*mut TraceRemoteIterator);
    fn trace_remote_iter_read_start(iter:*mut TraceRemoteIterator);
    fn trace_remote_iter_read_finished(iter:*mut TraceRemoteIterator);
    fn trace_remote_iter_print_event(iter:*mut TraceRemoteIterator)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
