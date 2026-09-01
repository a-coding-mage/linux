/* Translated from testing/selftests/bpf/bpf_experimental.h. */

/* Dependencies from the C header:
 * vmlinux.h, bpf/bpf_tracing.h, bpf/bpf_helpers.h, bpf/bpf_core_read.h,
 * and bpf_may_goto.h.
 */

/* C macro:
 * #define __contains(name, node) __attribute__((btf_decl_tag("contains:" #name ":" #node)))
 */

/* Convenience macro to wrap over bpf_obj_new:
 * #define bpf_obj_new(type) ((type *)bpf_obj_new(bpf_core_type_id_local(type)))
 */

/* Convenience macro to wrap over bpf_percpu_obj_new:
 * #define bpf_percpu_obj_new(type) ((type __percpu_kptr *)bpf_percpu_obj_new(bpf_core_type_id_local(type)))
 */

#[repr(C)]
pub struct bpf_iter_task_vma {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_task_vma_new(
        it: *mut bpf_iter_task_vma,
        task: *mut task_struct,
        addr: __u64,
    ) -> ::core::ffi::c_int;
    pub fn bpf_iter_task_vma_next(it: *mut bpf_iter_task_vma) -> *mut vm_area_struct;
    pub fn bpf_iter_task_vma_destroy(it: *mut bpf_iter_task_vma);
}

/* Description
 *  Throw a BPF exception from the program, immediately terminating its
 *  execution and unwinding the stack. The supplied 'cookie' parameter
 *  will be the return value of the program when an exception is thrown,
 *  and the default exception callback is used. Otherwise, if an exception
 *  callback is set using the '__exception_cb(callback)' declaration tag
 *  on the main program, the 'cookie' parameter will be the callback's only
 *  input argument.
 *
 *  Thus, in case of default exception callback, 'cookie' is subjected to
 *  constraints on the program's return value (as with R0 on exit).
 *  Otherwise, the return value of the marked exception callback will be
 *  subjected to the same checks.
 *
 *  Note that throwing an exception with lingering resources (locks,
 *  references, etc.) will lead to a verification error.
 *
 *  Note that callbacks *cannot* call this helper.
 * Returns
 *  Never.
 * Throws
 *  An exception with the specified 'cookie' value.
 */
extern "C" {
    pub fn bpf_throw(cookie: u64) -> !;
}

/* Description
 *  Acquire a reference on the exe_file member field belonging to the
 *  mm_struct that is nested within the supplied task_struct. The supplied
 *  task_struct must be trusted/referenced.
 * Returns
 *  A referenced file pointer pointing to the exe_file member field of the
 *  mm_struct nested in the supplied task_struct, or NULL.
 */
extern "C" {
    pub fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file;
}

/* Description
 *  Release a reference on the supplied file. The supplied file must be
 *  acquired.
 */
extern "C" {
    pub fn bpf_put_file(file: *mut file);
}

/* Description
 *  Resolve a pathname for the supplied path and store it in the supplied
 *  buffer. The supplied path must be trusted/referenced.
 * Returns
 *  A positive integer corresponding to the length of the resolved pathname,
 *  including the NULL termination character, stored in the supplied
 *  buffer. On error, a negative integer is returned.
 */
extern "C" {
    pub fn bpf_path_d_path(
        path: *const path,
        buf: *mut ::core::ffi::c_char,
        buf__sz: size_t,
    ) -> ::core::ffi::c_int;
}

/* This macro must be used to mark the exception callback corresponding to the
 * main program. For example:
 *
 * int exception_cb(u64 cookie) {
 *  return cookie;
 * }
 *
 * SEC("tc")
 * __exception_cb(exception_cb)
 * int main_prog(struct __sk_buff *ctx) {
 *  ...
 *  return TC_ACT_OK;
 * }
 *
 * Here, exception callback for the main program will be 'exception_cb'. Note
 * that this attribute can only be used once, and multiple exception callbacks
 * specified for the main program will lead to verification error.
 *
 * C macro:
 * #define __exception_cb(name) __attribute__((btf_decl_tag("exception_callback:" #name)))
 */

/* The __bpf_assert_* and bpf_cmp_* C macros rely on _Generic, _Static_assert,
 * statement expressions, labels-as-values, and BPF inline assembly constraints.
 * Their source-level intent is preserved here; callers need target-specific
 * Rust/LLVM BPF assembly support to map them exactly.
 */

#[inline(always)]
pub unsafe fn bpf_assert(cond: bool) {
    if !cond {
        bpf_throw(0);
    }
}

#[inline(always)]
pub unsafe fn bpf_assert_with(cond: bool, value: u64) {
    if !cond {
        bpf_throw(value);
    }
}

/* bpf_assert_range(LHS, BEG, END) and bpf_assert_range_with(LHS, BEG, END, value)
 * assert that LHS is in [BEG, END], update verifier bounds through BPF assembly,
 * and throw 0 or value on failure.
 */

/* bpf_nop_mov(var) emits a BPF no-op register move:
 * asm volatile("%[reg]=%[reg]"::[reg]"r"((short)var))
 */

/* emit instruction:
 * rX = rX .off = BPF_ADDR_SPACE_CAST .imm32 = (dst_as << 16) | src_as
 *
 * bpf_addr_space_cast(var, dst_as, src_as) is implemented in C with explicit
 * .byte/.short/.long inline assembly and the BPF_ADDR_SPACE_CAST dependency.
 */

extern "C" {
    pub fn bpf_preempt_disable();
    pub fn bpf_preempt_enable();
}

#[repr(C)]
pub struct __bpf_preempt_t {
    _unused: [u8; 0],
}

#[inline(always)]
pub unsafe fn __bpf_preempt_constructor() -> __bpf_preempt_t {
    let ret = __bpf_preempt_t { _unused: [] };

    bpf_preempt_disable();
    ret
}

#[inline(always)]
pub unsafe fn __bpf_preempt_destructor(_t: *mut __bpf_preempt_t) {
    bpf_preempt_enable();
}

/* C macro bpf_guard_preempt() creates a unique __bpf_preempt_t automatic
 * variable with a cleanup destructor and initializes it with
 * __bpf_preempt_constructor().
 */

#[repr(C)]
pub struct bpf_iter_css_task {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct cgroup_subsys_state {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_css_task_new(
        it: *mut bpf_iter_css_task,
        css: *mut cgroup_subsys_state,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn bpf_iter_css_task_next(it: *mut bpf_iter_css_task) -> *mut task_struct;
    pub fn bpf_iter_css_task_destroy(it: *mut bpf_iter_css_task);
}

#[repr(C)]
pub struct bpf_iter_task {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_task_new(
        it: *mut bpf_iter_task,
        task: *mut task_struct,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn bpf_iter_task_next(it: *mut bpf_iter_task) -> *mut task_struct;
    pub fn bpf_iter_task_destroy(it: *mut bpf_iter_task);
}

#[repr(C)]
pub struct bpf_iter_css {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_css_new(
        it: *mut bpf_iter_css,
        start: *mut cgroup_subsys_state,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn bpf_iter_css_next(it: *mut bpf_iter_css) -> *mut cgroup_subsys_state;
    pub fn bpf_iter_css_destroy(it: *mut bpf_iter_css);
}

extern "C" {
    pub fn bpf_wq_init(
        wq: *mut bpf_wq,
        p__map: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn bpf_wq_start(wq: *mut bpf_wq, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct bpf_iter_kmem_cache {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_kmem_cache_new(it: *mut bpf_iter_kmem_cache) -> ::core::ffi::c_int;
    pub fn bpf_iter_kmem_cache_next(it: *mut bpf_iter_kmem_cache) -> *mut kmem_cache;
    pub fn bpf_iter_kmem_cache_destroy(it: *mut bpf_iter_kmem_cache);
}

#[repr(C)]
pub struct bpf_iter_dmabuf {
    _unused: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_dmabuf_new(it: *mut bpf_iter_dmabuf) -> ::core::ffi::c_int;
    pub fn bpf_iter_dmabuf_next(it: *mut bpf_iter_dmabuf) -> *mut dma_buf;
    pub fn bpf_iter_dmabuf_destroy(it: *mut bpf_iter_dmabuf);
}

extern "C" {
    pub fn bpf_cgroup_read_xattr(
        cgroup: *mut cgroup,
        name__str: *const ::core::ffi::c_char,
        value_p: *mut bpf_dynptr,
    ) -> ::core::ffi::c_int;

    pub fn bpf_sock_read_xattr(
        sock: *mut socket,
        name__str: *const ::core::ffi::c_char,
        value_p: *mut bpf_dynptr,
    ) -> ::core::ffi::c_int;
}

pub const PREEMPT_BITS: u64 = 8;
pub const SOFTIRQ_BITS: u64 = 8;
pub const HARDIRQ_DISABLE_BITS: u64 = 8;
pub const HARDIRQ_BITS: u64 = 4;
pub const NMI_BITS: u64 = 1;

pub const PREEMPT_SHIFT: u64 = 0;
pub const SOFTIRQ_SHIFT: u64 = PREEMPT_SHIFT + PREEMPT_BITS;
pub const HARDIRQ_DISABLE_SHIFT: u64 = SOFTIRQ_SHIFT + SOFTIRQ_BITS;
pub const HARDIRQ_SHIFT: u64 = HARDIRQ_DISABLE_SHIFT + HARDIRQ_DISABLE_BITS;
pub const NMI_SHIFT: u64 = HARDIRQ_SHIFT + HARDIRQ_BITS;

pub const fn __IRQ_MASK(x: u64) -> u64 {
    (1_u64 << x).wrapping_sub(1)
}

pub const SOFTIRQ_MASK: u64 = __IRQ_MASK(SOFTIRQ_BITS) << SOFTIRQ_SHIFT;
pub const HARDIRQ_DISABLE_MASK: u64 =
    __IRQ_MASK(HARDIRQ_DISABLE_BITS) << HARDIRQ_DISABLE_SHIFT;
pub const HARDIRQ_MASK: u64 = __IRQ_MASK(HARDIRQ_BITS) << HARDIRQ_SHIFT;
pub const NMI_MASK: u64 = __IRQ_MASK(NMI_BITS) << NMI_SHIFT;

pub const SOFTIRQ_OFFSET: u64 = 1_u64 << SOFTIRQ_SHIFT;

extern "C" {
    pub static CONFIG_PREEMPT_RT: bool;
}

/* bpf_target_x86 */
extern "C" {
    pub static __preempt_count: ::core::ffi::c_int;
}

#[repr(C)]
pub struct pcpu_hot___local {
    pub preempt_count: ::core::ffi::c_int,
}

extern "C" {
    pub static mut pcpu_hot: pcpu_hot___local;
}

#[repr(C)]
pub struct task_struct___preempt_rt {
    pub softirq_disable_cnt: ::core::ffi::c_int,
}

/* bpf_target_s390 */
extern "C" {
    pub fn bpf_get_lowcore() -> *mut lowcore;
}

#[inline(always)]
pub unsafe fn get_preempt_count() -> ::core::ffi::c_int {
    /* C conditional branches:
     * - bpf_target_x86 reads per-CPU __preempt_count if present, otherwise
     *   pcpu_hot.preempt_count when the CO-RE field exists.
     * - bpf_target_arm64 reads current_task->thread_info.preempt.count.
     * - bpf_target_powerpc, loongarch, and riscv read
     *   current_task->thread_info.preempt_count.
     * - bpf_target_s390 reads bpf_get_lowcore()->preempt_count.
     *
     * Those target-specific paths depend on BPF CO-RE helpers and architecture
     * fields supplied by the surrounding build. With no matching C preprocessor
     * target in this isolated file, the common fallthrough is preserved.
     */
    0
}

/* Description
 *  Report whether it is in interrupt context. Only works on the following archs:
 *  * x86
 *  * arm64
 *  * powerpc64
 *  * s390x
 *  * loongarch
 *  * riscv
 */
#[inline(always)]
pub unsafe fn bpf_in_interrupt() -> ::core::ffi::c_int {
    let tsk: *mut task_struct___preempt_rt;
    let pcnt: ::core::ffi::c_int;

    pcnt = get_preempt_count();
    if !CONFIG_PREEMPT_RT {
        return pcnt & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_MASK) as ::core::ffi::c_int;
    }

    tsk = bpf_get_current_task_btf() as *mut task_struct___preempt_rt;
    (pcnt & (NMI_MASK | HARDIRQ_MASK) as ::core::ffi::c_int)
        | ((*tsk).softirq_disable_cnt & SOFTIRQ_MASK as ::core::ffi::c_int)
}

/* Description
 *  Report whether it is in NMI context. Only works on the following archs:
 *  * x86
 *  * arm64
 *  * powerpc64
 *  * s390x
 *  * loongarch
 *  * riscv
 */
#[inline(always)]
pub unsafe fn bpf_in_nmi() -> ::core::ffi::c_int {
    get_preempt_count() & NMI_MASK as ::core::ffi::c_int
}

/* Description
 *  Report whether it is in hard IRQ context. Only works on the following archs:
 *  * x86
 *  * arm64
 *  * powerpc64
 *  * s390x
 *  * loongarch
 *  * riscv
 */
#[inline(always)]
pub unsafe fn bpf_in_hardirq() -> ::core::ffi::c_int {
    get_preempt_count() & HARDIRQ_MASK as ::core::ffi::c_int
}

/* Description
 *  Report whether it is in softirq context. Only works on the following archs:
 *  * x86
 *  * arm64
 *  * powerpc64
 *  * s390x
 *  * loongarch
 *  * riscv
 */
#[inline(always)]
pub unsafe fn bpf_in_serving_softirq() -> ::core::ffi::c_int {
    let tsk: *mut task_struct___preempt_rt;
    let pcnt: ::core::ffi::c_int;

    pcnt = get_preempt_count();
    if !CONFIG_PREEMPT_RT {
        return (pcnt & SOFTIRQ_MASK as ::core::ffi::c_int)
            & SOFTIRQ_OFFSET as ::core::ffi::c_int;
    }

    tsk = bpf_get_current_task_btf() as *mut task_struct___preempt_rt;
    ((*tsk).softirq_disable_cnt & SOFTIRQ_MASK as ::core::ffi::c_int)
        & SOFTIRQ_OFFSET as ::core::ffi::c_int
}

/* Description
 *  Report whether it is in task context. Only works on the following archs:
 *  * x86
 *  * arm64
 *  * powerpc64
 *  * s390x
 *  * loongarch
 *  * riscv
 */
#[inline(always)]
pub unsafe fn bpf_in_task() -> ::core::ffi::c_int {
    let tsk: *mut task_struct___preempt_rt;
    let pcnt: ::core::ffi::c_int;

    pcnt = get_preempt_count();
    if !CONFIG_PREEMPT_RT {
        return if (pcnt & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_OFFSET) as ::core::ffi::c_int) == 0 {
            1
        } else {
            0
        };
    }

    tsk = bpf_get_current_task_btf() as *mut task_struct___preempt_rt;
    if ((pcnt & (NMI_MASK | HARDIRQ_MASK) as ::core::ffi::c_int)
        | (((*tsk).softirq_disable_cnt & SOFTIRQ_MASK as ::core::ffi::c_int)
            & SOFTIRQ_OFFSET as ::core::ffi::c_int))
        == 0
    {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
