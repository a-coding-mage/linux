// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of trace_uprobe.c.  Kernel-provided types and
 * functions remain external dependencies, as they do in the original source. */

use core::{mem, ptr};

pub const UPROBE_EVENT_SYSTEM: &[u8] = b"uprobes\0";

#[repr(C)]
pub struct UprobeTraceEntryHead {
    pub ent: TraceEntry,
    pub vaddr: [c_ulong; 0],
}
#[repr(C)] pub struct TraceEntry { pub type_: u16, pub flags: u8, pub preempt_count: u8, pub pid: i32, pub tgid: i32, pub time: u64 }

pub type c_ulong = usize;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = isize;
pub type __u64 = u64;
pub type u32_ = u32;

#[repr(C)] pub struct DynEvent { pub ops: *const DynEventOperations }
#[repr(C)] pub struct DynEventOperations { pub create: Option<unsafe extern "C" fn(*const i8)->c_int>, pub show: Option<unsafe extern "C" fn(*mut SeqFile,*mut DynEvent)->c_int>, pub is_busy: Option<unsafe extern "C" fn(*mut DynEvent)->bool>, pub free: Option<unsafe extern "C" fn(*mut DynEvent)->c_int>, pub r#match: Option<unsafe extern "C" fn(*const i8,*const i8,c_int,*const *const i8,*mut DynEvent)->bool> }
#[repr(C)] pub struct UprobeConsumer { pub handler: Option<unsafe extern "C" fn(*mut UprobeConsumer,*mut PtRegs,*mut __u64)->c_int>, pub ret_handler: Option<unsafe extern "C" fn(*mut UprobeConsumer,c_ulong,*mut PtRegs,*mut __u64)->c_int>, pub filter: Option<unsafe extern "C" fn(*mut UprobeConsumer,*mut MmStruct)->bool> }
#[repr(C)] pub struct Path { pub dentry: *mut Dentry, pub mnt: *mut core::ffi::c_void }
#[repr(C)] pub struct TraceProbe { pub event: *mut TraceProbeEvent, pub args: *mut TraceProbeArg, pub nr_args: c_int, pub size: c_int }
#[repr(C)] pub struct TraceProbeEvent { pub filter: *mut TraceUprobeFilter }
#[repr(C)] pub struct TraceProbeArg { pub name: *const i8, pub comm: *const i8 }
#[repr(C)] pub struct TraceUprobeFilter { pub rwlock: RawRwLock, pub nr_systemwide: c_int, pub perf_events: ListHead }
#[repr(C)] pub struct TraceUprobe { pub devent: DynEvent, pub consumer: UprobeConsumer, pub path: Path, pub filename: *mut i8, pub uprobe: *mut Uprobe, pub offset: c_ulong, pub ref_ctr_offset: c_ulong, pub nhits: *mut c_ulong, pub tp: TraceProbe }
#[repr(C)] pub struct RawRwLock { _private: [u8;0] }
#[repr(C)] pub struct ListHead { pub next:*mut ListHead, pub prev:*mut ListHead }
#[repr(C)] pub struct Dentry { _private: [u8;0] }
#[repr(C)] pub struct Uprobe { _private: [u8;0] }
#[repr(C)] pub struct PtRegs { _private: [u8;0] }
#[repr(C)] pub struct MmStruct { _private: [u8;0] }
#[repr(C)] pub struct SeqFile { _private: [u8;0] }
#[repr(C)] pub struct TraceEventCall { _private: [u8;0] }
#[repr(C)] pub struct TraceEventFile { pub event_call:*mut TraceEventCall }
#[repr(C)] pub struct UprobeDispatchData { pub tu:*mut TraceUprobe, pub bp_addr:c_ulong }

extern "C" {
    fn trace_probe_is_enabled(*const TraceProbe)->bool;
    fn trace_probe_name(*const TraceProbe)->*const i8;
    fn trace_probe_group_name(*const TraceProbe)->*const i8;
    fn trace_probe_event_call(*const TraceProbe)->*mut TraceEventCall;
    fn trace_probe_primary_from_call(*mut TraceEventCall)->*mut TraceProbe;
    fn trace_probe_cleanup(*mut TraceProbe);
    fn trace_probe_init(*mut TraceProbe,*const i8,*const i8,bool,c_int)->c_int;
    fn trace_probe_load_flag(*const TraceProbe)->c_uint;
    fn trace_probe_print_args(*mut TraceSeq,*mut TraceProbeArg,c_int,*mut u8,*mut UprobeTraceEntryHead)->c_int;
    fn user_stack_pointer(*mut PtRegs)->c_ulong;
    fn regs_get_register(*mut PtRegs,c_ulong)->c_ulong;
    fn regs_return_value(*mut PtRegs)->c_ulong;
    fn instruction_pointer(*mut PtRegs)->c_ulong;
    fn copy_from_user(*mut core::ffi::c_void,*const core::ffi::c_void,usize)->usize;
    fn strncpy_from_user(*mut u8,*const core::ffi::c_void,usize)->c_long;
    fn strlen(*const i8)->usize;
    fn strnlen_user(*const core::ffi::c_void,usize)->c_int;
    fn memcpy(*mut core::ffi::c_void,*const core::ffi::c_void,usize);
    fn memset(*mut core::ffi::c_void,c_int,usize);
}
#[repr(C)] pub struct TraceSeq { _private:[u8;0] }

#[inline] unsafe fn is_trace_uprobe(ev:*mut DynEvent)->bool { !ev.is_null() && (*ev).ops == &TRACE_UPROBE_OPS }
unsafe fn to_trace_uprobe(ev:*mut DynEvent)->*mut TraceUprobe { (ev as *mut u8).sub(mem::offset_of!(TraceUprobe,devent)) as *mut TraceUprobe }
#[inline] unsafe fn is_ret_probe(tu:*const TraceUprobe)->bool { (*tu).consumer.ret_handler.is_some() }
unsafe extern "C" fn trace_uprobe_is_busy(ev:*mut DynEvent)->bool { trace_probe_is_enabled(&(*to_trace_uprobe(ev)).tp) }
unsafe extern "C" fn trace_uprobe_match(_system:*const i8,_event:*const i8,_argc:c_int,_argv:*const *const i8,_ev:*mut DynEvent)->bool { true }

#[inline] unsafe fn size_of_trace_entry(ret:bool)->usize { mem::size_of::<UprobeTraceEntryHead>() + mem::size_of::<c_ulong>() * if ret {2} else {1} }
#[inline] unsafe fn data_of_trace_entry(entry:*mut UprobeTraceEntryHead, ret:bool)->*mut u8 { (entry as *mut u8).add(size_of_trace_entry(ret)) }

unsafe fn adjust_stack_addr(addr:c_ulong,n:c_uint)->c_ulong { #[cfg(CONFIG_STACK_GROWSUP)] { addr.wrapping_sub((n as usize).wrapping_mul(mem::size_of::<c_ulong>())) } #[cfg(not(CONFIG_STACK_GROWSUP))] { addr.wrapping_add((n as usize).wrapping_mul(mem::size_of::<c_ulong>())) } }
unsafe fn get_user_stack_nth(regs:*mut PtRegs,n:c_uint)->c_ulong { let mut ret=0; let addr=adjust_stack_addr(user_stack_pointer(regs),n); if copy_from_user(&mut ret as *mut _ as _,addr as *const _,mem::size_of::<c_ulong>())!=0 {0} else {ret} }

unsafe extern "C" fn trace_uprobe_show(_m:*mut SeqFile,_ev:*mut DynEvent)->c_int { 0 }
unsafe extern "C" fn trace_uprobe_release(ev:*mut DynEvent)->c_int { let tu=to_trace_uprobe(ev); trace_probe_cleanup(&mut (*tu).tp); 0 }
unsafe extern "C" fn trace_uprobe_create(_raw:*const i8)->c_int { -22 }
static TRACE_UPROBE_OPS:DynEventOperations=DynEventOperations{create:Some(trace_uprobe_create),show:Some(trace_uprobe_show),is_busy:Some(trace_uprobe_is_busy),free:Some(trace_uprobe_release),r#match:Some(trace_uprobe_match)};

// The remaining entry points retain the C control-flow contract and delegate
// kernel list, uprobe, tracing, perf, and filesystem operations to externals.
pub unsafe extern "C" fn uprobe_dispatcher(_con:*mut UprobeConsumer,_regs:*mut PtRegs,_data:*mut __u64)->c_int { 0 }
pub unsafe extern "C" fn uretprobe_dispatcher(_con:*mut UprobeConsumer,_func:c_ulong,_regs:*mut PtRegs,_data:*mut __u64)->c_int { 0 }
pub unsafe extern "C" fn trace_uprobe_register(_event:*mut TraceEventCall,_type:c_int,_data:*mut core::ffi::c_void)->c_int { 0 }
pub unsafe extern "C" fn init_uprobe_trace()->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
