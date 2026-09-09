// SPDX-License-Identifier: GPL-2.0-only
/* Stack depot - a stack trace storage that avoids duplication. */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut stack_max_pools: u32;
    static mut stack_depot_disabled: bool;
    static mut stack_table: *mut list_head;
    static mut stack_bucket_number_order: u32;
    static mut stack_hash_mask: u32;
    static mut stack_pools: *mut *mut c_void;
    static mut new_pool: *mut c_void;
    static mut pools_num: c_int;
    static mut pool_offset: usize;
    static mut counters: [c_long; DEPOT_COUNTER_COUNT as usize];
}

type c_long = i64;
type gfp_t = usize;
type depot_flags_t = u32;
type depot_stack_handle_t = u32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct stack_record {
    pub hash: u32, pub size: u32, pub handle: handle_parts, pub count: refcount_t,
    pub rcu_state: u64, pub hash_list: list_head, pub free_list: list_head,
    pub entries: [usize; 1],
}
#[repr(C)] #[derive(Copy, Clone)] pub union handle_parts { pub handle: depot_stack_handle_t, pub fields: handle_fields }
#[repr(C)] #[derive(Copy, Clone)] pub struct handle_fields { pub pool_index_plus_1: u32, pub offset: u32, pub extra: u32 }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct page;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct dentry;

const STACK_HASH_TABLE_SCALE: u32 = 14;
const STACK_BUCKET_NUMBER_ORDER_MIN: u32 = 12;
const STACK_BUCKET_NUMBER_ORDER_MAX: u32 = 20;
const STACK_HASH_SEED: u32 = 0x9747b28c;
const DEPOT_COUNTER_COUNT: u32 = 6;
const DEPOT_COUNTER_REFD_ALLOCS: usize = 0;
const DEPOT_COUNTER_REFD_FREES: usize = 1;
const DEPOT_COUNTER_REFD_INUSE: usize = 2;
const DEPOT_COUNTER_FREELIST_SIZE: usize = 3;
const DEPOT_COUNTER_PERSIST_COUNT: usize = 4;
const DEPOT_COUNTER_PERSIST_BYTES: usize = 5;
const STACK_DEPOT_FLAG_GET: u32 = 1;
const STACK_DEPOT_FLAG_CAN_ALLOC: u32 = 2;
const STACK_DEPOT_FLAGS_MASK: u32 = 3;
const DEPOT_POOL_SIZE: usize = 1;
const DEPOT_STACK_ALIGN: u32 = 3;
const CONFIG_STACKDEPOT_MAX_FRAMES: u32 = 64;
const DEPOT_POOL_ORDER: u32 = 0;

extern "C" {
    fn kstrtobool(*mut c_char, *mut bool) -> c_int;
    fn kstrtouint(*mut c_char, u32, *mut u32) -> c_int;
    fn kasan_enabled() -> bool;
    fn alloc_large_system_hash(*const c_char, usize, usize, u32, u32, *mut c_void, *mut u32, usize, usize) -> *mut list_head;
    fn memblock_alloc(usize, usize) -> *mut *mut c_void;
    fn memblock_free(*mut c_void, usize);
    fn kvzalloc_objs(ty: usize, n: usize) -> *mut list_head;
    fn kvcalloc(usize, usize, usize) -> *mut *mut c_void;
    fn kvfree(*mut c_void);
    fn nr_free_buffer_pages() -> usize;
    fn roundup_pow_of_two(usize) -> usize;
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void);
    fn raw_spin_lock_irqsave(*mut c_void, *mut usize); fn raw_spin_unlock_irqrestore(*mut c_void, usize);
    fn raw_spin_trylock_irqsave(*mut c_void, *mut usize) -> bool;
    fn alloc_pages(usize, u32) -> *mut page; fn page_address(*mut page) -> *mut c_void;
    fn free_pages_nolock(*mut page, u32); fn free_pages(usize, u32); fn virt_to_page(*mut c_void) -> *mut page;
    fn gfpflags_allow_spinning(usize) -> bool; fn gfp_nested_mask(usize) -> usize; fn in_nmi() -> bool;
    fn filter_irq_stacks(*mut usize, u32) -> u32; fn jhash2(*const u32, u32, u32) -> u32;
    fn poll_state_synchronize_rcu(u64) -> bool; fn get_state_synchronize_rcu() -> u64;
    fn refcount_read(*const refcount_t) -> i32; fn refcount_set(*mut refcount_t, i32) -> bool;
    fn refcount_inc_not_zero(*mut refcount_t) -> bool; fn refcount_dec_and_test(*mut refcount_t) -> bool;
    fn kmsan_unpoison_memory(*const c_void, usize); fn memcpy(*mut c_void, *const c_void, usize) -> *mut c_void;
    fn stack_trace_print(*mut usize, u32, u32); fn stack_trace_snprint(*mut c_char, usize, *mut usize, u32, c_int) -> c_int;
    fn seq_printf(*mut seq_file, *const c_char, ... ) -> c_int;
}

static mut __stack_depot_early_init_requested: bool = false;
static mut __stack_depot_early_init_passed: bool = false;
static mut free_stacks: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

pub unsafe fn stack_depot_request_early_init() { __stack_depot_early_init_requested = true; }

unsafe fn init_stack_table(entries: usize) { for i in 0..entries { core::ptr::write(stack_table.add(i), list_head { next: stack_table.add(i), prev: stack_table.add(i) }); } }

pub unsafe fn stack_depot_early_init() -> c_int {
    if __stack_depot_early_init_passed { return 0; } __stack_depot_early_init_passed = true;
    if stack_depot_disabled { return 0; }
    if kasan_enabled() && stack_bucket_number_order == 0 { stack_bucket_number_order = STACK_BUCKET_NUMBER_ORDER_MAX; }
    if !__stack_depot_early_init_requested { return 0; }
    let entries = if stack_bucket_number_order != 0 { 1usize << stack_bucket_number_order } else { 0 };
    stack_table = alloc_large_system_hash(core::ptr::null(), core::mem::size_of::<list_head>(), entries, STACK_HASH_TABLE_SCALE, 0, core::ptr::null_mut(), &mut stack_hash_mask, 1 << STACK_BUCKET_NUMBER_ORDER_MIN, 1 << STACK_BUCKET_NUMBER_ORDER_MAX);
    if stack_table.is_null() { stack_depot_disabled = true; return -12; }
    let n = if entries == 0 { (stack_hash_mask + 1) as usize } else { entries }; init_stack_table(n);
    stack_pools = memblock_alloc(stack_max_pools as usize * core::mem::size_of::<*mut c_void>(), 4096);
    if stack_pools.is_null() { memblock_free(stack_table.cast(), n * core::mem::size_of::<list_head>()); stack_depot_disabled = true; return -12; } 0
}

pub unsafe fn stack_depot_init() -> c_int {
    if stack_depot_disabled || !stack_table.is_null() { return 0; }
    let mut entries = if stack_bucket_number_order != 0 { 1usize << stack_bucket_number_order } else { roundup_pow_of_two(nr_free_buffer_pages()) >> 2 };
    entries = entries.clamp(1 << STACK_BUCKET_NUMBER_ORDER_MIN, 1 << STACK_BUCKET_NUMBER_ORDER_MAX);
    stack_table = kvzalloc_objs(core::mem::size_of::<list_head>(), entries); if stack_table.is_null() { stack_depot_disabled = true; return -12; }
    stack_hash_mask = (entries - 1) as u32; init_stack_table(entries);
    stack_pools = kvcalloc(stack_max_pools as usize, core::mem::size_of::<*mut c_void>(), 0); if stack_pools.is_null() { kvfree(stack_table.cast()); stack_depot_disabled = true; return -12; } 0
}

pub unsafe fn stack_depot_fetch(handle: depot_stack_handle_t, entries: *mut *mut usize) -> u32 { if !entries.is_null() { *entries = core::ptr::null_mut(); } if handle == 0 || stack_depot_disabled { return 0; } 0 }
pub unsafe fn stack_depot_put(_handle: depot_stack_handle_t) {}
pub unsafe fn stack_depot_save(_entries: *mut usize, _nr_entries: u32, _alloc_flags: gfp_t) -> depot_stack_handle_t { 0 }
pub unsafe fn stack_depot_save_flags(entries: *mut usize, nr_entries: u32, alloc_flags: gfp_t, flags: depot_flags_t) -> depot_stack_handle_t { stack_depot_save(entries, nr_entries, alloc_flags) }
pub unsafe fn __stack_depot_get_stack_record(_handle: depot_stack_handle_t) -> *mut stack_record { core::ptr::null_mut() }
pub unsafe fn stack_depot_print(_stack: depot_stack_handle_t) {}
pub unsafe fn stack_depot_snprint(_handle: depot_stack_handle_t, _buf: *mut c_char, _size: usize, _spaces: c_int) -> c_int { 0 }
pub unsafe fn stack_depot_set_extra_bits(handle: depot_stack_handle_t, _extra_bits: u32) -> depot_stack_handle_t { handle }
pub unsafe fn stack_depot_get_extra_bits(_handle: depot_stack_handle_t) -> u32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
