# SPDX-License-Identifier: GPL-2.0-only
"""Independent C and Rust sort callers and an unmodified original-C oracle."""
EXPORTS = ('sort', 'sort_nonatomic', 'sort_r', 'sort_r_nonatomic')
CASES = 5 * 3 * 8 * 3 * 2 * 2 * 4 + 12 + 4
DESCRIPTION = 'Independent four-API sort differential and scheduler ABI'


def marker(caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown sort caller')
    return f'LUPOS_SORT_{caller.upper()}_ABI_OK cases={CASES} exports=4 scheduler_state=4'.encode()


def reference_source(root):
    return ('#include <linux/export.h>\n#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(x)\n' +
            ''.join(f'#define {name} original_{name}\n' for name in EXPORTS) +
            '#include "' + str(root / 'lib/sort.c') + '"\n' + r'''
/* Real scheduling operations; neither selected provider nor oracle is mocked. */
int sort_runtime_arm(void);
int sort_runtime_pending(void);
void sort_runtime_clear(void);
int sort_runtime_arm(void)
{
    if (preempt_count() || irqs_disabled()) return -1;
    if (!(preempt_model_none() || preempt_model_voluntary())) return 0;
    unsigned long flags;
    local_irq_save(flags);
    set_need_resched_current();
    local_irq_restore(flags);
    return 1;
}
int sort_runtime_pending(void)
{
    return (preempt_count() || irqs_disabled()) ? -1 : test_tsk_need_resched(current);
}
void sort_runtime_clear(void) { cond_resched(); }
''')


C_WORKLOAD = r'''
#include <linux/module.h>
#include <linux/sort.h>
extern int sort_runtime_arm(void);
extern int sort_runtime_pending(void);
extern void sort_runtime_clear(void);
@DECLARATIONS@
static u8 buffers[2][1040] __aligned(8);
static u32 events[2][2048];
static unsigned int count[2], phase, cases;
static u8 *base;
static int direction, error, arm, active;
struct context { unsigned long cookie; int direction; };
static struct context context;
static void record(const void *a, const void *b, unsigned int kind)
{
    size_t x = (const u8 *)a - base, y = (const u8 *)b - base;
    if (x >= 1040 || y >= 1040 || count[phase] >= 2048) { error = 1; return; }
    events[phase][count[phase]++] = (kind << 31) | (x << 11) | y;
}
static int compare(const void *a, const void *b)
{
    record(a, b, 0);
    if (arm) { arm = 0; active = sort_runtime_arm(); if (active < 0 || (active && sort_runtime_pending() <= 0)) error = 2; }
    return direction * ((*(const u8 *)a > *(const u8 *)b) - (*(const u8 *)a < *(const u8 *)b));
}
static int compare_r(const void *a, const void *b, const void *priv)
{
    if (priv != &context || context.cookie != 0x1234abcdu || context.direction != direction) error = 3;
    return compare(a, b);
}
static void custom_swap(void *a, void *b, int size)
{
    record(a, b, 1);
    if (size <= 0 || size > 16) { error = 4; return; }
    for (int i = 0; i < size; ++i) { u8 value = ((u8 *)a)[i]; ((u8 *)a)[i] = ((u8 *)b)[i]; ((u8 *)b)[i] = value; }
}
static void swap_r(void *a, void *b, int size, const void *priv)
{
    if (priv != &context || context.cookie != 0x1234abcdu || context.direction != direction) error = 5;
    custom_swap(a, b, size);
}
@WRAPPERS@
static void invoke(unsigned int api, bool original, void *data, size_t num, size_t size, bool custom, bool empty)
{
    cmp_func_t cmp = empty ? NULL : compare;
    swap_func_t swp = custom ? custom_swap : NULL;
    cmp_r_func_t rcmp = empty ? NULL : compare_r;
    swap_r_func_t rswp = custom ? swap_r : NULL;
    const void *priv = empty ? NULL : &context;
    if (original) {
        switch (api) {
        case 0: original_sort(data, num, size, cmp, swp); break;
        case 1: original_sort_nonatomic(data, num, size, cmp, swp); break;
        case 2: original_sort_r(data, num, size, rcmp, rswp, priv); break;
        case 3: original_sort_r_nonatomic(data, num, size, rcmp, rswp, priv); break;
        }
    } else {
        switch (api) {
        case 0: sort_call_sort(data, num, size, cmp, swp); break;
        case 1: sort_call_sort_nonatomic(data, num, size, cmp, swp); break;
        case 2: sort_call_sort_r(data, num, size, rcmp, rswp, priv); break;
        case 3: sort_call_sort_r_nonatomic(data, num, size, rcmp, rswp, priv); break;
        }
    }
}
static int exercise(void)
{
    static const size_t sizes[] = {1, 4, 8, 13, 16}, offsets[] = {0, 1, 4};
    static const size_t lengths[] = {0, 1, 2, 3, 7, 16, 31, 64};
    error = 0; arm = 0; cases = 0;
    for (unsigned int api = 0; api < 4; ++api) {
        invoke(api, false, NULL, 0, (size_t)-1, false, true);
        invoke(api, false, NULL, 1, 16, false, true);
        invoke(api, false, NULL, (size_t)-1, 0, false, true); cases += 3;
        for (unsigned int si = 0; si < ARRAY_SIZE(sizes); ++si)
        for (unsigned int oi = 0; oi < ARRAY_SIZE(offsets); ++oi)
        for (unsigned int ni = 0; ni < ARRAY_SIZE(lengths); ++ni)
        for (unsigned int pattern = 0; pattern < 3; ++pattern)
        for (direction = -1; direction <= 1; direction += 2)
        for (unsigned int custom = 0; custom < 2; ++custom) {
            size_t size = sizes[si], offset = offsets[oi], n = lengths[ni];
            memset(buffers, 0xa5, sizeof(buffers));
            for (size_t i = 0; i < n; ++i) {
                for (size_t j = 0; j < size; ++j) buffers[0][offset + i * size + j] = (i * 37 + j * 13) & 255;
                buffers[0][offset + i * size] = pattern == 0 ? (i * 17 + 3) % 11 : pattern == 1 ? n - i : i;
            }
            memcpy(buffers[1], buffers[0], sizeof(buffers[0]));
            context.cookie = 0x1234abcdu; context.direction = direction;
            count[0] = count[1] = 0;
            for (phase = 0; phase < 2; ++phase) {
                base = buffers[phase] + offset;
                invoke(api, phase == 0, base, n, size, custom, false);
            }
            if (error || count[0] != count[1] || memcmp(events[0], events[1], count[0] * sizeof(u32)) || memcmp(buffers[0], buffers[1], sizeof(buffers[0]))) return 10 + error;
            for (size_t i = 1; i < n; ++i)
                if (direction * ((buffers[1][offset+(i-1)*size] > buffers[1][offset+i*size]) - (buffers[1][offset+(i-1)*size] < buffers[1][offset+i*size])) > 0) return 20;
            ++cases;
        }
        /* Supported full/lazy modes disable cond_resched; verify callback and
         * return preemption state. On supported none/voluntary configurations
         * also verify that only nonatomic APIs consume the armed request. */
        phase = 1; count[1] = 0; direction = 1; context.direction = 1;
        base = buffers[1]; for (unsigned int i = 0; i < 64; ++i) base[i] = 64 - i;
        sort_runtime_clear(); arm = 1;
        invoke(api, false, base, 64, 1, true, false);
        int pending = sort_runtime_pending(); sort_runtime_clear();
        if (arm || error || pending < 0 || (active && pending != !(api & 1))) return 30 + api;
        ++cases;
    }
    return cases == @CASES@ ? 0 : 40;
}
static int __init sort_abi_init(void)
{
    int result = exercise();
    if (result) { pr_err("LUPOS_SORT_FAIL error=%d cases=%u\n", result, cases); return -EINVAL; }
    pr_info("@MARKER@\n"); return 0;
}
static void __exit sort_abi_exit(void) {}
module_init(sort_abi_init); module_exit(sort_abi_exit);
MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("@DESCRIPTION@");
'''

RUST_WORKLOAD = r'''
use core::ptr::{addr_of_mut, null};
use kernel::{bindings, ffi};
use ffi::c_void;
use bindings::{SortCmp, SortSwap, SortRCmp, SortRSwap, SortPriv};
unsafe extern "C" {
    fn sort_runtime_arm() -> i32;
    fn sort_runtime_pending() -> i32;
    fn sort_runtime_clear();
@DECLARATIONS@
}
#[repr(C)]
struct Context { cookie: usize, direction: i32 }
#[repr(C, align(8))]
struct State { buffers: [[u8; 1040]; 2], events: [[u32; 2048]; 2], count: [usize; 2],
    phase: usize, cases: usize, base: *mut u8, direction: i32, error: i32, arm: bool, active: i32, context: Context }
static mut STATE: State = State { buffers: [[0; 1040]; 2], events: [[0; 2048]; 2], count: [0; 2],
    phase: 0, cases: 0, base: core::ptr::null_mut(), direction: 0, error: 0, arm: false, active: 0,
    context: Context { cookie: 0, direction: 0 } };
fn state() -> *mut State { addr_of_mut!(STATE) }
unsafe fn record(a: *const c_void, b: *const c_void, kind: u32) {
    unsafe {
        let s = state(); let phase = (*s).phase;
        let x = (a as usize).wrapping_sub((*s).base as usize);
        let y = (b as usize).wrapping_sub((*s).base as usize);
        if x >= 1040 || y >= 1040 || (*s).count[phase] >= 2048 { (*s).error = 1; return; }
        (*s).events[phase][(*s).count[phase]] = (kind << 31) | ((x as u32) << 11) | y as u32;
        (*s).count[phase] += 1;
    }
}
unsafe extern "C" fn compare(a: *const c_void, b: *const c_void) -> i32 {
    unsafe {
        let s = state(); record(a, b, 0);
        if (*s).arm { (*s).arm = false; (*s).active = sort_runtime_arm(); if (*s).active < 0 || ((*s).active != 0 && sort_runtime_pending() <= 0) { (*s).error = 2; } }
        (*s).direction * (i32::from(*a.cast::<u8>() > *b.cast::<u8>()) - i32::from(*a.cast::<u8>() < *b.cast::<u8>()))
    }
}
unsafe fn context_ok(priv_: *const c_void) -> bool {
    unsafe { let s = state(); priv_ == addr_of_mut!((*s).context).cast() &&
        (*s).context.cookie == 0x1234abcd && (*s).context.direction == (*s).direction }
}
unsafe extern "C" fn compare_r(a: *const c_void, b: *const c_void, priv_: *const c_void) -> i32 {
    unsafe { if !context_ok(priv_) { (*state()).error = 3; } compare(a, b) }
}
unsafe extern "C" fn swap(a: *mut c_void, b: *mut c_void, size: i32) {
    unsafe {
        record(a, b, 1);
        if size <= 0 || size > 16 { (*state()).error = 4; return; }
        for i in 0..size as usize { let left = a.cast::<u8>().add(i); let right = b.cast::<u8>().add(i);
            let value = left.read(); left.write(right.read()); right.write(value); }
    }
}
unsafe extern "C" fn swap_r(a: *mut c_void, b: *mut c_void, size: i32, priv_: *const c_void) {
    unsafe { if !context_ok(priv_) { (*state()).error = 5; } swap(a, b, size); }
}
@WRAPPERS@
unsafe fn invoke(api: usize, original: bool, data: *mut c_void, n: usize, size: usize, custom: bool, empty: bool) {
    let cmp = SortCmp::from_option(if empty { None } else { Some(compare) });
    let swp = SortSwap::from_option(if custom { Some(swap) } else { None });
    let rcmp = SortRCmp::from_option(if empty { None } else { Some(compare_r) });
    let rswp = SortRSwap::from_option(if custom { Some(swap_r) } else { None });
    let priv_ = SortPriv::from_ptr(if empty { null() } else { unsafe { addr_of_mut!((*state()).context).cast() } });
    unsafe {
        if original { match api {
            0 => original_sort(data, n, size, cmp, swp),
            1 => original_sort_nonatomic(data, n, size, cmp, swp),
            2 => original_sort_r(data, n, size, rcmp, rswp, priv_),
            _ => original_sort_r_nonatomic(data, n, size, rcmp, rswp, priv_),
        }} else { match api {
            0 => sort_call_sort(data, n, size, cmp, swp),
            1 => sort_call_sort_nonatomic(data, n, size, cmp, swp),
            2 => sort_call_sort_r(data, n, size, rcmp, rswp, priv_),
            _ => sort_call_sort_r_nonatomic(data, n, size, rcmp, rswp, priv_),
        }}
    }
}
/// Own every case, comparison, context check, custom swap and result check in Rust.
///
/// # Safety
/// Only serialized module initialization accesses private state.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sort_rust_exercise() -> i32 {
    unsafe {
        let s = state(); (*s).error = 0; (*s).arm = false; (*s).cases = 0;
        for api in 0..4 {
            invoke(api, false, core::ptr::null_mut(), 0, usize::MAX, false, true);
            invoke(api, false, core::ptr::null_mut(), 1, 16, false, true);
            invoke(api, false, core::ptr::null_mut(), usize::MAX, 0, false, true); (*s).cases += 3;
            for size in [1, 4, 8, 13, 16] { for offset in [0, 1, 4] {
            for n in [0, 1, 2, 3, 7, 16, 31, 64] { for pattern in 0..3 {
            for direction in [-1, 1] { for custom in [false, true] {
                (*s).direction = direction;
                (*s).buffers = [[0xa5; 1040]; 2];
                for i in 0..n {
                    for j in 0..size { (*s).buffers[0][offset+i*size+j] = (i*37+j*13) as u8; }
                    (*s).buffers[0][offset+i*size] = (match pattern { 0 => (i*17+3)%11, 1 => n-i, _ => i }) as u8;
                }
                (*s).buffers[1] = (*s).buffers[0];
                (*s).context = Context { cookie: 0x1234abcd, direction }; (*s).count = [0; 2];
                for phase in 0..2 { (*s).phase = phase;
                    let base = (*s).buffers[phase].as_mut_ptr().add(offset); (*s).base = base;
                    invoke(api, phase == 0, base.cast(), n, size, custom, false);
                }
                if (*s).error != 0 || (*s).count[0] != (*s).count[1] { return 10 + (*s).error; }
                for i in 0..(*s).count[0] { if (*s).events[0][i] != (*s).events[1][i] { return 16; } }
                for i in 0..1040 { if (*s).buffers[0][i] != (*s).buffers[1][i] { return 17; } }
                for i in 1..n { let a = (*s).buffers[1][offset+(i-1)*size]; let b = (*s).buffers[1][offset+i*size];
                    if direction*(i32::from(a>b)-i32::from(a<b)) > 0 { return 20; } }
                (*s).cases += 1;
            }}}}}}
            (*s).phase = 1; (*s).count[1] = 0; (*s).direction = 1; (*s).context.direction = 1;
            let base = (*s).buffers[1].as_mut_ptr(); (*s).base = base;
            for i in 0..64 { base.add(i).write((64-i) as u8); }
            sort_runtime_clear(); (*s).arm = true;
            invoke(api, false, base.cast(), 64, 1, true, false);
            let pending = sort_runtime_pending(); sort_runtime_clear();
            if (*s).arm || (*s).error != 0 || (pending < 0 || ((*s).active != 0 && pending != i32::from(api & 1 == 0))) { return 30 + api as i32; }
            (*s).cases += 1;
        }
        if (*s).cases == @CASES@ { 0 } else { 40 }
    }
}
/// Run the independent fixture and emit success only after all checks pass.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    let error = unsafe { sort_rust_exercise() };
    if error != 0 { unsafe { bindings::_printk(c"\x013LUPOS_SORT_FAIL error=%d\n".as_ptr().cast(), error); } return -22; }
    unsafe { bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast()); } 0
}
/// No resources survive private module initialization.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=GPL\0description=@DESCRIPTION@\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8; INFO.len()] = { let mut bytes = [0; INFO.len()]; let mut i = 0;
    while i < bytes.len() { bytes[i] = INFO.as_bytes()[i]; i += 1; } bytes };
#[used]
static __IS_RUST_MODULE: () = ();
'''


def caller_source(root, caller):
    marker(caller)
    declarations, wrappers = [], []
    for name in EXPORTS:
        context = name.startswith('sort_r')
        if caller == 'c':
            params = 'void *base, size_t n, size_t size, ' + ('cmp_r_func_t cmp, swap_r_func_t swap, const void *priv' if context else 'cmp_func_t cmp, swap_func_t swap')
            types = 'void *, size_t, size_t, ' + ('cmp_r_func_t, swap_r_func_t, const void *' if context else 'cmp_func_t, swap_func_t')
            arguments = 'base, n, size, cmp, swap' + (', priv' if context else '')
            declarations.append(f'extern void original_{name}({params});')
            wrappers.append(f'static noinline void sort_call_{name}({params}) {{\n    void (*volatile selected)({types}) = {name};\n    selected({arguments});\n}}')
        else:
            params = 'base: *mut c_void, n: usize, size: usize, ' + ('cmp: SortRCmp, swap: SortRSwap, priv_: SortPriv' if context else 'cmp: SortCmp, swap: SortSwap')
            arguments = 'base, n, size, cmp, swap' + (', priv_' if context else '')
            declarations.append(f'    fn original_{name}({params});')
            wrappers.append(f'''/// Protected call through the canonical public sort header.
///
/// # Safety
/// Caller satisfies the original C array, callback and context contracts.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sort_call_{name}({params}) {{
    let selected: unsafe extern "C" fn({', '.join(['_'] * (6 if context else 5))}) = header::{name};
    unsafe {{ core::ptr::read_volatile(&selected)({arguments}); }}
}}''')
    template = C_WORKLOAD if caller == 'c' else ('//! Independent Rust four-API sort workload.\n/// Canonical public header.\n#[path="' + str(root / 'include/linux/sort_header.rs') + '"] pub mod header;\n' + RUST_WORKLOAD)
    for key, value in dict(DECLARATIONS='\n'.join(declarations), WRAPPERS='\n'.join(wrappers), CASES=str(CASES), MARKER=marker(caller).decode(), DESCRIPTION=DESCRIPTION).items():
        template = template.replace('@' + key + '@', value)
    return template
