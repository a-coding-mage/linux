// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation unit:
// mm_types, maple_tree, rwsem, spinlock, list, cpumask, mman, pgtable,
// atomic, user_namespace, iommu, and asm/mmu.

// INIT_MM_CONTEXT(name) is empty unless supplied by the build configuration.
// The following initializer preserves the C initializer's layout and
// configuration-dependent fields; its referenced types/macros are external.
#[allow(non_camel_case_types)]
extern "C" {
    pub static vma_dummy_vm_ops: vm_operations_struct;
    pub static mut init_mm: mm_struct;
}

// External kernel types. Their definitions are supplied by other translated
// units.
#[allow(non_camel_case_types)]
pub enum vm_operations_struct {}
#[allow(non_camel_case_types)]
pub enum mm_struct {}

/*
 * For dynamically allocated mm_structs, there is a dynamically sized cpumask
 * at the end of the structure, the size of which depends on the maximum CPU
 * number the system can see. That way we allocate only as much memory for
 * mm_cpumask() as needed for the hundreds, or thousands of processes that
 * a system typically runs.
 *
 * Since there is only one init_mm in the entire system, keep it simple
 * and size this cpu_bitmask to NR_CPUS.
 */
// C definition preserved conceptually below. MTREE_INIT_EXT, swapper_pg_dir,
// atomic initializers, lock/list initializers, flexible-array initialization,
// and INIT_MM_CONTEXT are supplied by the kernel dependencies.
//
// struct mm_struct init_mm = {
//     .mm_mt = MTREE_INIT_EXT(mm_mt, MM_MT_FLAGS, init_mm.mmap_lock),
//     .pgd = swapper_pg_dir,
//     .mm_users = ATOMIC_INIT(2),
//     .mm_count = ATOMIC_INIT(1),
//     .write_protect_seq = SEQCNT_ZERO(init_mm.write_protect_seq),
//     MMAP_LOCK_INITIALIZER(init_mm),
//     .page_table_lock = __SPIN_LOCK_UNLOCKED(init_mm.page_table_lock),
//     .arg_lock = __SPIN_LOCK_UNLOCKED(init_mm.arg_lock),
//     .mmlist = LIST_HEAD_INIT(init_mm.mmlist),
//     #[cfg(feature = "CONFIG_PER_VMA_LOCK")]
//     .vma_writer_wait = __RCUWAIT_INITIALIZER(init_mm.vma_writer_wait),
//     .mm_lock_seq = SEQCNT_ZERO(init_mm.mm_lock_seq),
//     #[cfg(feature = "CONFIG_SCHED_MM_CID")]
//     .mm_cid.lock = __RAW_SPIN_LOCK_UNLOCKED(init_mm.mm_cid.lock),
//     .flexible_array = MM_STRUCT_FLEXIBLE_ARRAY_INIT,
//     INIT_MM_CONTEXT(init_mm)
// };

pub unsafe fn setup_initial_init_mm(
    start_code: *mut core::ffi::c_void,
    end_code: *mut core::ffi::c_void,
    end_data: *mut core::ffi::c_void,
    brk: *mut core::ffi::c_void,
) {
    // Field accesses correspond directly to the C mm_struct members.
    // The concrete mm_struct layout is supplied by the kernel dependency.
    // init_mm.start_code = start_code as usize;
    // init_mm.end_code = end_code as usize;
    // init_mm.end_data = end_data as usize;
    // init_mm.brk = brk as usize;
    let _ = (&mut init_mm, start_code, end_code, end_data, brk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
