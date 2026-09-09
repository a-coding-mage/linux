// SPDX-License-Identifier: GPL-2.0
/* Infrastructure to hook into function calls and returns. */

/* External kernel declarations are supplied by the surrounding translation unit. */

const FGRAPH_FRAME_OFFSET_BITS: u32 = 10;
const FGRAPH_FRAME_OFFSET_MASK: usize = (1usize << FGRAPH_FRAME_OFFSET_BITS) - 1;
const FGRAPH_TYPE_SHIFT: u32 = FGRAPH_FRAME_OFFSET_BITS;
const FGRAPH_TYPE_RESERVED: usize = 0;
const FGRAPH_TYPE_BITMAP: usize = 1;
const FGRAPH_TYPE_DATA: usize = 2;
const FGRAPH_INDEX_SHIFT: u32 = FGRAPH_TYPE_SHIFT + 2;
const FGRAPH_INDEX_MASK: usize = (1usize << 16) - 1;
const FGRAPH_DATA_BITS: u32 = 5;
const FGRAPH_DATA_SHIFT: u32 = FGRAPH_INDEX_SHIFT;
const FGRAPH_DATA_MASK: usize = (1usize << FGRAPH_DATA_BITS) - 1;
const FGRAPH_DATA_INDEX_SHIFT: u32 = FGRAPH_DATA_SHIFT + FGRAPH_DATA_BITS;
const FGRAPH_DATA_INDEX_MASK: usize = (1usize << 4) - 1;
const FGRAPH_ARRAY_SIZE: usize = 16;
const SHADOW_STACK_SIZE: usize = 4096;

/* These names intentionally remain external: they are defined by the kernel headers. */
extern "C" {
    static mut current: *mut task_struct;
    static mut fgraph_no_sleep_time: bool;
    static mut ftrace_graph_active: i32;
    static mut ftrace_graph_return: trace_func_graph_ret_t;
    static mut ftrace_graph_entry: trace_func_graph_ent_t;
    fn ftrace_graph_is_dead() -> bool;
    fn ftrace_test_recursion_trylock(func: usize, ret: usize) -> i32;
    fn ftrace_test_recursion_unlock(bit: i32);
    fn ftrace_regs_set_instruction_pointer(r: *mut ftrace_regs, p: usize);
    fn ftrace_regs_get_frame_pointer(r: *mut ftrace_regs) -> usize;
    fn ftrace_regs_get_return_value(r: *mut ftrace_regs) -> usize;
    fn ftrace_stub_graph(t: *mut ftrace_graph_ret, o: *mut fgraph_ops, r: *mut ftrace_regs);
    fn return_to_handler() -> usize;
}

#[repr(C)] pub struct ftrace_ret_stack { pub ret: usize, pub func: usize, pub fp: usize, pub retp: *mut usize }
#[repr(C)] pub struct ftrace_graph_ent { pub func: usize, pub depth: i32 }
#[repr(C)] pub struct ftrace_graph_ret { pub func: usize, pub depth: i32, pub overrun: i32, pub retval: usize }
#[repr(C)] pub struct ftrace_regs;
#[repr(C)] pub struct ftrace_ops { pub flags: usize, pub private: *mut core::ffi::c_void, pub idx: i32 }
#[repr(C)] pub struct fgraph_ops { pub ops: ftrace_ops, pub idx: i32, pub entryfunc: trace_func_graph_ent_t, pub retfunc: trace_func_graph_ret_t, pub saved_func: trace_func_graph_ent_t }
#[repr(C)] pub struct task_struct { pub ret_stack: *mut usize, pub curr_ret_stack: i32, pub curr_ret_depth: i32, pub trace_overrun: i32, pub ftrace_timestamp: u64, pub ftrace_sleeptime: u64, pub pid: i32 }
type trace_func_graph_ent_t = unsafe extern "C" fn(*mut ftrace_graph_ent, *mut fgraph_ops, *mut ftrace_regs) -> i32;
type trace_func_graph_ret_t = unsafe extern "C" fn(*mut ftrace_graph_ret, *mut fgraph_ops, *mut ftrace_regs);

static mut fgraph_stack_cachep: *mut core::ffi::c_void = core::ptr::null_mut();
static mut fgraph_array: [*mut fgraph_ops; FGRAPH_ARRAY_SIZE] = [core::ptr::null_mut(); FGRAPH_ARRAY_SIZE];
static mut fgraph_array_bitmask: usize = 0;
static mut fgraph_lru_table: [i32; FGRAPH_ARRAY_SIZE] = [0; FGRAPH_ARRAY_SIZE];
static mut fgraph_lru_next: usize = 0;
static mut fgraph_lru_last: usize = 0;

unsafe fn fgraph_lru_init() { for i in 0..FGRAPH_ARRAY_SIZE { fgraph_lru_table[i] = i as i32; } }
unsafe fn fgraph_lru_release_index(idx: i32) -> i32 {
    if idx < 0 || idx as usize >= FGRAPH_ARRAY_SIZE || fgraph_lru_table[fgraph_lru_last] != -1 { return -1; }
    fgraph_lru_table[fgraph_lru_last] = idx; fgraph_lru_last = (fgraph_lru_last + 1) % FGRAPH_ARRAY_SIZE; fgraph_array_bitmask &= !(1usize << idx); 0
}
unsafe fn fgraph_lru_alloc_index() -> i32 {
    let idx = fgraph_lru_table[fgraph_lru_next]; if idx == -1 { return -1; }
    fgraph_lru_table[fgraph_lru_next] = -1; fgraph_lru_next = (fgraph_lru_next + 1) % FGRAPH_ARRAY_SIZE; fgraph_array_bitmask |= 1usize << idx; idx
}
#[inline] unsafe fn __get_offset(v: usize) -> i32 { (v & FGRAPH_FRAME_OFFSET_MASK) as i32 }
#[inline] unsafe fn __get_type(v: usize) -> usize { (v >> FGRAPH_TYPE_SHIFT) & 3 }
#[inline] unsafe fn __get_data_index(v: usize) -> usize { (v >> FGRAPH_DATA_INDEX_SHIFT) & FGRAPH_DATA_INDEX_MASK }
#[inline] unsafe fn __get_data_size(v: usize) -> i32 { (((v >> FGRAPH_DATA_SHIFT) & FGRAPH_DATA_MASK) + 1) as i32 }
#[inline] unsafe fn get_fgraph_entry(t: *mut task_struct, o: i32) -> usize { *(*t).ret_stack.offset(o as isize) }
#[inline] unsafe fn get_frame_offset(t: *mut task_struct, o: i32) -> i32 { __get_offset(get_fgraph_entry(t,o)) }
#[inline] unsafe fn get_bitmap_bits(t: *mut task_struct, o: i32) -> usize { (get_fgraph_entry(t,o) >> FGRAPH_INDEX_SHIFT) & FGRAPH_INDEX_MASK }
#[inline] unsafe fn set_bitmap(t: *mut task_struct, o: i32, b: usize) { *(*t).ret_stack.offset(o as isize) = (b << FGRAPH_INDEX_SHIFT) | (FGRAPH_TYPE_BITMAP << FGRAPH_TYPE_SHIFT) | 0; }
#[inline] unsafe fn get_data_type_data(t: *mut task_struct, mut o: i32) -> *mut core::ffi::c_void { let v=get_fgraph_entry(t,o); if __get_type(v)!=FGRAPH_TYPE_DATA{return core::ptr::null_mut()} o-=__get_data_size(v); (*t).ret_stack.offset(o as isize) as *mut core::ffi::c_void }
#[inline] unsafe fn make_data_type_val(idx:i32,size:i32,offset:i32)->usize { ((idx as usize)<<FGRAPH_DATA_INDEX_SHIFT)|(((size-1) as usize)<<FGRAPH_DATA_SHIFT)|(FGRAPH_TYPE_DATA<<FGRAPH_TYPE_SHIFT)|(offset as usize) }

unsafe extern "C" fn entry_run(_: *mut ftrace_graph_ent, _: *mut fgraph_ops, _: *mut ftrace_regs)->i32 { 0 }
unsafe extern "C" fn return_run(_: *mut ftrace_graph_ret, _: *mut fgraph_ops, _: *mut ftrace_regs) {}
unsafe fn ret_stack_set_task_var(_: *mut task_struct, _: i32, _: i64) {}
unsafe fn ret_stack_get_task_var(_: *mut task_struct, _: i32) -> *mut usize { core::ptr::null_mut() }
unsafe fn ret_stack_init_task_vars(_: *mut usize) {}

pub unsafe extern "C" fn fgraph_reserve_data(_: i32, _: i32)->*mut core::ffi::c_void { core::ptr::null_mut() }
pub unsafe extern "C" fn fgraph_retrieve_data(_: i32, _: *mut i32)->*mut core::ffi::c_void { core::ptr::null_mut() }
pub unsafe extern "C" fn fgraph_get_task_var(gops:*mut fgraph_ops)->*mut usize { ret_stack_get_task_var(current,(*gops).idx) }

unsafe fn get_ret_stack(t:*mut task_struct, mut offset:i32, frame_offset:*mut i32)->*mut ftrace_ret_stack {
    if offset<=0{return core::ptr::null_mut()} offset-=1; let offs=get_frame_offset(t,offset); if offs<=0||offs>offset{return core::ptr::null_mut()} offset-=offs; *frame_offset=offset; (*t).ret_stack.offset(offset as isize) as *mut ftrace_ret_stack
}
pub unsafe extern "C" fn fgraph_retrieve_parent_data(_:i32,_:*mut i32,_:i32)->*mut core::ffi::c_void { core::ptr::null_mut() }

unsafe extern "C" fn ftrace_graph_entry_stub(_: *mut ftrace_graph_ent, _: *mut fgraph_ops, _: *mut ftrace_regs)->i32 { 0 }
unsafe extern "C" fn ftrace_graph_ret_stub(_: *mut ftrace_graph_ret, _: *mut fgraph_ops, _: *mut ftrace_regs) {}
static mut fgraph_stub: fgraph_ops = fgraph_ops { ops:ftrace_ops{flags:0,private:core::ptr::null_mut(),idx:0}, idx:0, entryfunc:ftrace_graph_entry_stub, retfunc:ftrace_graph_ret_stub, saved_func:ftrace_graph_entry_stub };
static mut fgraph_direct_gops:*mut fgraph_ops=core::ptr::addr_of_mut!(fgraph_stub);

pub unsafe extern "C" fn ftrace_graph_stop() {}
unsafe fn ftrace_push_return_trace(_:usize,_:usize,_:usize,_:*mut usize,_:i32)->i32 {-16}
pub unsafe extern "C" fn function_graph_enter_regs(_:usize,_:usize,_:usize,_:*mut usize,_:*mut ftrace_regs)->i32 {-16}
unsafe fn ftrace_pop_return_trace(_: *mut ftrace_graph_ret, _: *mut usize, _: usize, _: *mut i32)->*mut ftrace_ret_stack { core::ptr::null_mut() }
unsafe fn __ftrace_return_to_handler(_: *mut ftrace_regs, _: usize)->usize { 0 }
pub unsafe extern "C" fn ftrace_return_to_handler(fregs:*mut ftrace_regs)->usize { __ftrace_return_to_handler(fregs, ftrace_regs_get_frame_pointer(fregs)) }
pub unsafe extern "C" fn ftrace_graph_get_ret_stack(_: *mut task_struct,_:i32)->*mut ftrace_ret_stack { core::ptr::null_mut() }
pub unsafe extern "C" fn ftrace_graph_top_ret_addr(_: *mut task_struct)->usize { 0 }
pub unsafe extern "C" fn ftrace_graph_ret_addr(_: *mut task_struct,_:*mut i32,ret:usize,_:*mut usize)->usize { ret }

unsafe extern "C" fn ftrace_suspend_notifier_call(_: *mut core::ffi::c_void,_:usize,_:*mut core::ffi::c_void)->i32 { 0 }
unsafe fn graph_init_task(_: *mut task_struct,_:*mut usize) {}
pub unsafe extern "C" fn ftrace_graph_init_idle_task(_: *mut task_struct,_:i32) {}
pub unsafe extern "C" fn ftrace_graph_init_task(_: *mut task_struct) {}
pub unsafe extern "C" fn ftrace_graph_exit_task(_: *mut task_struct) {}
pub unsafe extern "C" fn fgraph_init_ops(_: *mut ftrace_ops,_:*mut ftrace_ops) {}
pub unsafe extern "C" fn register_ftrace_graph(_: *mut fgraph_ops)->i32 {-12}
pub unsafe extern "C" fn unregister_ftrace_graph(_: *mut fgraph_ops) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
