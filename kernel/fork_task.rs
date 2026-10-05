// SPDX-License-Identifier: GPL-2.0-only
// kernel/fork.c: task, stack and boot-time cache ownership.
#[no_mangle]
pub static mut total_forks: c_ulong = 0;
#[no_mangle]
pub static mut nr_threads: c_int = 0;
#[link_section = ".data..read_mostly"]
static mut max_threads: c_int = 0;
static mut task_struct_cachep: *mut kmem_cache = null_mut();
static mut signal_cachep: *mut kmem_cache = null_mut();
#[no_mangle]
pub static mut sighand_cachep: *mut kmem_cache = null_mut();
#[no_mangle]
pub static mut files_cachep: *mut kmem_cache = null_mut();
#[no_mangle]
pub static mut fs_cachep: *mut kmem_cache = null_mut();
static mut mm_cachep: *mut kmem_cache = null_mut();
#[cfg(CONFIG_ARCH_WANTS_DYNAMIC_TASK_STRUCT)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut arch_task_struct_size: c_int = 0;
const MIN_THREADS: u64 = 20;
const NR_CACHED_STACKS: usize = 2;

#[inline]
unsafe fn current_task() -> *mut task_struct {
    rust_fork_current()
}
#[inline]
fn err_ptr<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}
#[inline]
unsafe fn is_err<T>(p: *const T) -> bool {
    rust_fork_is_err(p.cast())
}
#[inline]
fn ptr_err<T>(p: *const T) -> c_int {
    p as isize as c_int
}
#[inline]
unsafe fn init_list(p: *mut list_head) {
    (*p).next = p;
    (*p).prev = p;
}
#[inline]
unsafe fn init_hlist(p: *mut hlist_node) {
    (*p).next = null_mut();
    (*p).pprev = null_mut();
}

#[cfg(CONFIG_PROVE_RCU)]
#[no_mangle]
pub unsafe extern "C" fn lockdep_tasklist_lock_is_held() -> c_int {
    rust_fork_tasklist_held()
}
#[no_mangle]
pub unsafe extern "C" fn nr_processes() -> c_int {
    let mut cpu = rust_fork_next_possible_cpu(-1);
    let mut total: c_int = 0;
    while cpu < rust_fork_nr_cpu_ids() as c_int {
        total = total.wrapping_add(rust_fork_process_count(cpu) as c_int);
        cpu = rust_fork_next_possible_cpu(cpu);
    }
    total
}
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn arch_release_task_struct(_tsk: *mut task_struct) {}
#[inline]
unsafe fn alloc_task_struct_node(node: c_int) -> *mut task_struct {
    rust_fork_kmem_cache_alloc_node(task_struct_cachep, RUST_FORK_GFP_KERNEL, node).cast()
}
#[inline]
unsafe fn free_task_struct(tsk: *mut task_struct) {
    kmem_cache_free(task_struct_cachep, tsk.cast());
}

#[cfg(CONFIG_VMAP_STACK)]
unsafe fn alloc_thread_stack_node_from_cache(
    _tsk: *mut task_struct,
    node: c_int,
) -> *mut vm_struct {
    rust_fork_preempt_disable();
    let mut result = null_mut();
    if node == NUMA_NO_NODE || rust_fork_numa_node_id() == node {
        for i in 0..NR_CACHED_STACKS {
            let vm = rust_fork_cached_stack_xchg(i as c_uint);
            if !vm.is_null() {
                result = vm;
                break;
            }
        }
    }
    rust_fork_preempt_enable();
    result
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe fn try_release_thread_stack_to_cache(vm: *mut vm_struct) -> bool {
    rust_fork_preempt_disable();
    let nid = rust_fork_numa_node_id();
    let mut local = true;
    if rust_fork_node_has_memory(nid) {
        for i in 0..(*vm).nr_pages as usize {
            if rust_fork_page_to_nid(*(*vm).pages.add(i)) != nid {
                local = false;
                break;
            }
        }
    }
    let mut released = false;
    if local {
        for i in 0..NR_CACHED_STACKS {
            if rust_fork_cached_stack_cmpxchg(i as c_uint, vm) {
                released = true;
                break;
            }
        }
    }
    rust_fork_preempt_enable();
    released
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe extern "C" fn thread_stack_free_rcu(rh: *mut rcu_head) {
    let stack = rh
        .byte_sub(offset_of!(rust_fork_vm_stack, rcu))
        .cast::<rust_fork_vm_stack>();
    let vm = (*stack).stack_vm_area;
    if !try_release_thread_stack_to_cache(vm) {
        vfree((*vm).addr);
    }
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe fn thread_stack_delayed_free(tsk: *mut task_struct) {
    let stack = (*tsk).stack.cast::<rust_fork_vm_stack>();
    (*stack).stack_vm_area = (*tsk).stack_vm_area;
    call_rcu(addr_of_mut!((*stack).rcu), Some(thread_stack_free_rcu));
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe extern "C" fn free_vm_stack_cache(cpu: c_uint) -> c_int {
    for i in 0..NR_CACHED_STACKS {
        let slot = rust_fork_cached_stack_slot(cpu, i as c_uint);
        let vm = *slot;
        if !vm.is_null() {
            vfree((*vm).addr);
            *slot = null_mut();
        }
    }
    0
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe fn memcg_charge_kernel_stack(vm: *mut vm_struct) -> c_int {
    rust_fork_bug_on(
        (*vm).nr_pages as usize != RUST_FORK_THREAD_SIZE as usize / RUST_FORK_PAGE_SIZE as usize,
    );
    for i in 0..RUST_FORK_THREAD_SIZE as usize / RUST_FORK_PAGE_SIZE as usize {
        let ret = rust_fork_memcg_charge_page(*(*vm).pages.add(i), RUST_FORK_GFP_KERNEL, 0);
        if ret != 0 {
            for j in 0..i {
                rust_fork_memcg_uncharge_page(*(*vm).pages.add(j), 0);
            }
            return ret;
        }
    }
    0
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe fn alloc_thread_stack_node(tsk: *mut task_struct, node: c_int) -> c_int {
    let mut vm = alloc_thread_stack_node_from_cache(tsk, node);
    if !vm.is_null() {
        if memcg_charge_kernel_stack(vm) != 0 {
            vfree((*vm).addr);
            return -(ENOMEM as c_int);
        }
        if !rust_fork_kasan_hw_tags_enabled() {
            rust_fork_kasan_unpoison_range((*vm).addr, RUST_FORK_THREAD_SIZE as usize);
        }
        let stack = rust_fork_kasan_reset_tag((*vm).addr);
        rust_fork_clear_pages((*vm).addr, (*vm).nr_pages);
        (*tsk).stack_vm_area = vm;
        (*tsk).stack = stack;
        return 0;
    }
    let stack = rust_fork_vmalloc_node(
        RUST_FORK_THREAD_SIZE as c_ulong,
        RUST_FORK_THREAD_ALIGN as c_ulong,
        RUST_FORK_GFP_VMAP_STACK,
        node,
        rust_fork_return_address(),
    );
    if stack.is_null() {
        return -(ENOMEM as c_int);
    }
    vm = find_vm_area(stack);
    if memcg_charge_kernel_stack(vm) != 0 {
        vfree(stack);
        return -(ENOMEM as c_int);
    }
    (*tsk).stack_vm_area = vm;
    (*tsk).stack = rust_fork_kasan_reset_tag(stack);
    0
}
#[cfg(CONFIG_VMAP_STACK)]
unsafe fn free_thread_stack(tsk: *mut task_struct) {
    if !try_release_thread_stack_to_cache((*tsk).stack_vm_area) {
        thread_stack_delayed_free(tsk);
    }
    (*tsk).stack = null_mut();
    (*tsk).stack_vm_area = null_mut();
}
#[cfg(not(CONFIG_VMAP_STACK))]
static mut thread_stack_cache: *mut kmem_cache = null_mut();
#[cfg(not(CONFIG_VMAP_STACK))]
unsafe extern "C" fn thread_stack_free_rcu(rh: *mut rcu_head) {
    if RUST_FORK_THREAD_SIZE >= RUST_FORK_PAGE_SIZE {
        __free_pages(
            rust_fork_virt_to_page(rh.cast()),
            RUST_FORK_THREAD_SIZE_ORDER,
        );
    } else {
        kmem_cache_free(thread_stack_cache, rh.cast());
    }
}
#[cfg(not(CONFIG_VMAP_STACK))]
unsafe fn thread_stack_delayed_free(tsk: *mut task_struct) {
    call_rcu((*tsk).stack.cast(), Some(thread_stack_free_rcu));
}
#[cfg(not(CONFIG_VMAP_STACK))]
unsafe fn alloc_thread_stack_node(tsk: *mut task_struct, node: c_int) -> c_int {
    if RUST_FORK_THREAD_SIZE >= RUST_FORK_PAGE_SIZE {
        let page =
            rust_fork_alloc_pages_node(node, RUST_FORK_THREADINFO_GFP, RUST_FORK_THREAD_SIZE_ORDER);
        if page.is_null() {
            return -(ENOMEM as c_int);
        }
        (*tsk).stack = rust_fork_kasan_reset_tag(rust_fork_page_address(page));
        0
    } else {
        let stack =
            rust_fork_kmem_cache_alloc_node(thread_stack_cache, RUST_FORK_THREADINFO_GFP, node);
        (*tsk).stack = rust_fork_kasan_reset_tag(stack);
        if (*tsk).stack.is_null() {
            -(ENOMEM as c_int)
        } else {
            0
        }
    }
}
#[cfg(not(CONFIG_VMAP_STACK))]
unsafe fn free_thread_stack(tsk: *mut task_struct) {
    thread_stack_delayed_free(tsk);
    (*tsk).stack = null_mut();
}
#[cfg(not(CONFIG_VMAP_STACK))]
#[no_mangle]
pub unsafe extern "C" fn rust_fork_thread_stack_cache_init() {
    thread_stack_cache = rust_fork_kmem_cache_create_usercopy(
        c"thread_stack".as_ptr().cast(),
        RUST_FORK_THREAD_SIZE as usize,
        RUST_FORK_THREAD_SIZE as usize,
        0,
        0,
        RUST_FORK_THREAD_SIZE as usize,
        None,
    );
    rust_fork_bug_on(thread_stack_cache.is_null());
}
unsafe fn account_kernel_stack(tsk: *mut task_struct, account: c_int) {
    #[cfg(CONFIG_VMAP_STACK)]
    {
        let vm = rust_fork_task_stack_vm_area(tsk);
        for i in 0..RUST_FORK_THREAD_SIZE as usize / RUST_FORK_PAGE_SIZE as usize {
            rust_fork_mod_lruvec_page_state(
                *(*vm).pages.add(i),
                NR_KERNEL_STACK_KB,
                account as c_long * (RUST_FORK_PAGE_SIZE as c_long / 1024),
            );
        }
    }
    #[cfg(not(CONFIG_VMAP_STACK))]
    rust_fork_mod_lruvec_kmem_state(
        rust_fork_task_stack_page(tsk),
        NR_KERNEL_STACK_KB,
        account as c_long * (RUST_FORK_THREAD_SIZE as c_long / 1024),
    );
}
#[no_mangle]
pub unsafe extern "C" fn exit_task_stack_account(tsk: *mut task_struct) {
    account_kernel_stack(tsk, -1);
    #[cfg(CONFIG_VMAP_STACK)]
    {
        let vm = rust_fork_task_stack_vm_area(tsk);
        for i in 0..RUST_FORK_THREAD_SIZE as usize / RUST_FORK_PAGE_SIZE as usize {
            rust_fork_memcg_uncharge_page(*(*vm).pages.add(i), 0);
        }
    }
}
unsafe fn release_task_stack(tsk: *mut task_struct) {
    if rust_fork_warn_on(read_volatile(addr_of!((*tsk).__state)) != TASK_DEAD) {
        return;
    }
    free_thread_stack(tsk);
}
#[cfg(CONFIG_THREAD_INFO_IN_TASK)]
#[no_mangle]
pub unsafe extern "C" fn put_task_stack(tsk: *mut task_struct) {
    if rust_fork_refcount_dec_and_test(addr_of_mut!((*tsk).stack_refcount)) {
        release_task_stack(tsk);
    }
}
#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
unsafe fn put_task_stack(_tsk: *mut task_struct) { /* original task_stack.h has no reference */
}
#[no_mangle]
pub unsafe extern "C" fn free_task(tsk: *mut task_struct) {
    #[cfg(CONFIG_SECCOMP)]
    rust_fork_warn_seccomp_once(!(*tsk).seccomp.filter.is_null());
    release_user_cpus_ptr(tsk);
    rust_fork_scs_release(tsk);
    rust_fork_smp_task_ipi_mask_free(tsk);
    #[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
    release_task_stack(tsk);
    #[cfg(CONFIG_THREAD_INFO_IN_TASK)]
    rust_fork_warn_stack_once(rust_fork_refcount_read(addr_of!((*tsk).stack_refcount)) != 0);
    rust_fork_rt_mutex_debug_task_free(tsk);
    rust_fork_ftrace_graph_exit_task(tsk);
    arch_release_task_struct(tsk);
    if (*tsk).flags & PF_KTHREAD != 0 {
        free_kthread_struct(tsk);
    }
    rust_fork_bpf_task_storage_free(tsk);
    rust_fork_put_task_exec_state(read_volatile(addr_of!((*tsk).exec_state)));
    free_task_struct(tsk);
}
#[no_mangle]
#[linkage = "weak"]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_task_cache_init() {}
#[link_section = ".init.text"]
unsafe fn set_max_threads(suggested: c_uint) {
    let pages = memblock_estimated_nr_free_pages() as u64;
    let bits = if pages == 0 {
        0
    } else {
        64 - pages.leading_zeros()
    };
    let page_bits = 64 - (RUST_FORK_PAGE_SIZE as u64).leading_zeros();
    let mut threads = if bits + page_bits > 64 {
        FUTEX_TID_MASK as u64
    } else {
        pages * RUST_FORK_PAGE_SIZE as u64 / (RUST_FORK_THREAD_SIZE as u64 * 8)
    };
    threads = threads.min(suggested as u64);
    max_threads = threads.clamp(MIN_THREADS, FUTEX_TID_MASK as u64) as c_int;
}
#[link_section = ".init.text"]
unsafe fn task_struct_whitelist(offset: *mut c_ulong, size: *mut c_ulong) {
    rust_fork_arch_thread_struct_whitelist(offset, size);
    if *size == 0 {
        *offset = 0;
    } else {
        *offset += offset_of!(task_struct, thread) as c_ulong;
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn fork_init() {
    let align = (RUST_FORK_L1_CACHE_BYTES as usize).max(RUST_FORK_ARCH_MIN_TASKALIGN as usize);
    let (mut offset, mut size) = (0, 0);
    task_struct_whitelist(&mut offset, &mut size);
    task_struct_cachep = rust_fork_kmem_cache_create_usercopy(
        c"task_struct".as_ptr().cast(),
        rust_fork_arch_task_struct_size(),
        align,
        RUST_FORK_SLAB_PANIC | RUST_FORK_SLAB_ACCOUNT,
        offset as usize,
        size as usize,
        None,
    );
    arch_task_cache_init();
    set_max_threads(FUTEX_TID_MASK);
    let sig = (*addr_of_mut!(init_task)).signal;
    (*sig).rlim[RLIMIT_NPROC as usize].rlim_cur = (max_threads / 2) as c_ulong;
    (*sig).rlim[RLIMIT_NPROC as usize].rlim_max = (max_threads / 2) as c_ulong;
    (*sig).rlim[RLIMIT_SIGPENDING as usize] = (*sig).rlim[RLIMIT_NPROC as usize];
    for i in 0..RUST_FORK_UCOUNT_COUNTS as usize {
        (*addr_of_mut!(init_user_ns)).ucount_max[i] = (max_threads / 2) as c_long;
    }
    for resource in [
        UCOUNT_RLIMIT_NPROC,
        UCOUNT_RLIMIT_MSGQUEUE,
        UCOUNT_RLIMIT_SIGPENDING,
        UCOUNT_RLIMIT_MEMLOCK,
    ] {
        rust_fork_set_userns_rlimit_max(
            addr_of_mut!(init_user_ns),
            resource,
            RUST_FORK_RLIM_INFINITY,
        );
    }
    #[cfg(CONFIG_VMAP_STACK)]
    rust_fork_cpuhp_stack_cache(Some(free_vm_stack_cache));
    rust_fork_scs_init();
    rust_fork_lockdep_init_task(addr_of_mut!(init_task));
    rust_fork_uprobes_init();
}
#[no_mangle]
#[linkage = "weak"]
/// Copy the native task representation into an unpublished child.
///
/// # Safety
/// dst is exclusive allocated child storage and src is the live original task
/// under the original fork protocol. Exclusive dst does not imply exclusive
/// src: concurrent scheduler/BPF field access remains a native-runtime
/// qualification obligation. The weak architecture override is unchanged.
pub unsafe extern "C" fn arch_dup_task_struct(
    dst: *mut task_struct,
    src: *mut task_struct,
) -> c_int {
    // The native aggregate assignment preserves fork.c:903. Rust must not
    // independently snapshot overlapping live scheduler fields here.
    rust_fork_task_struct_copy(dst, src);
    0
}
#[no_mangle]
pub unsafe extern "C" fn set_task_stack_end_magic(tsk: *mut task_struct) {
    *rust_fork_end_of_stack(tsk) = STACK_END_MAGIC as c_ulong;
}
unsafe fn dup_task_struct(orig: *mut task_struct, mut node: c_int) -> *mut task_struct {
    if node == NUMA_NO_NODE {
        node = rust_fork_tsk_fork_get_node(orig);
    }
    let tsk = alloc_task_struct_node(node);
    if tsk.is_null() {
        return tsk;
    }
    if arch_dup_task_struct(tsk, orig) != 0 || alloc_thread_stack_node(tsk, node) != 0 {
        free_task_struct(tsk);
        return null_mut();
    }
    #[cfg(CONFIG_THREAD_INFO_IN_TASK)]
    rust_fork_refcount_set(addr_of_mut!((*tsk).stack_refcount), 1);
    account_kernel_stack(tsk, 1);
    if rust_fork_smp_task_ipi_mask_alloc(tsk) != 0 {
        exit_task_stack_account(tsk);
        free_thread_stack(tsk);
        free_task_struct(tsk);
        return null_mut();
    }
    if rust_fork_scs_prepare(tsk, node) != 0 {
        rust_fork_smp_task_ipi_mask_free(tsk);
        exit_task_stack_account(tsk);
        free_thread_stack(tsk);
        free_task_struct(tsk);
        return null_mut();
    }
    #[cfg(CONFIG_SECCOMP)]
    {
        (*tsk).seccomp.filter = null_mut();
    }
    write_volatile(addr_of_mut!((*tsk).exec_state), null_mut());
    rust_fork_setup_thread_stack(tsk, orig);
    rust_fork_clear_user_return_notifier(tsk);
    rust_fork_clear_tsk_need_resched(tsk);
    set_task_stack_end_magic(tsk);
    rust_fork_clear_syscall_user_dispatch(tsk);
    #[cfg(CONFIG_STACKPROTECTOR)]
    {
        (*tsk).stack_canary = rust_fork_get_random_canary();
    }
    if (*orig).cpus_ptr == addr_of!((*orig).cpus_mask) {
        (*tsk).cpus_ptr = addr_of!((*tsk).cpus_mask);
    }
    dup_user_cpus_ptr(tsk, orig, node);
    rust_fork_refcount_set(addr_of_mut!((*tsk).rcu_users), 2);
    rust_fork_refcount_set(addr_of_mut!((*tsk).usage), 1);
    #[cfg(CONFIG_BLK_DEV_IO_TRACE)]
    {
        (*tsk).btrace_seq = 0;
    }
    (*tsk).splice_pipe = null_mut();
    (*tsk).task_frag.page = null_mut();
    (*tsk).wake_q.next = null_mut();
    (*tsk).worker_private = null_mut();
    rust_fork_kcov_task_init(tsk);
    rust_fork_kmsan_task_create(tsk);
    rust_fork_kmap_local_fork(tsk);
    #[cfg(CONFIG_FAULT_INJECTION)]
    {
        (*tsk).fail_nth = 0;
    }
    #[cfg(CONFIG_BLK_CGROUP)]
    {
        (*tsk).throttle_disk = null_mut();
        (*tsk).set_use_memdelay(0);
    }
    #[cfg(CONFIG_ARCH_HAS_CPU_PASID)]
    {
        (*tsk).set_pasid_activated(0);
    }
    #[cfg(CONFIG_MEMCG)]
    {
        (*tsk).active_memcg = null_mut();
    }
    #[cfg(CONFIG_X86_BUS_LOCK_DETECT)]
    {
        (*tsk).set_reported_split_lock(0);
    }
    #[cfg(CONFIG_SCHED_MM_CID)]
    {
        (*tsk).mm_cid.cid = RUST_FORK_MM_CID_UNSET;
        (*tsk).mm_cid.active = 0;
        init_hlist(addr_of_mut!((*tsk).mm_cid.node));
    }
    #[cfg(CONFIG_BPF_SYSCALL)]
    {
        write_volatile(addr_of_mut!((*tsk).bpf_storage), null_mut());
        (*tsk).bpf_ctx = null_mut();
    }
    tsk
}
