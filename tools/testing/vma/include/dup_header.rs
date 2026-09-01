/* SPDX-License-Identifier: GPL-2.0+ */

/* Forward declarations to avoid header cycle. */
pub enum vm_area_struct {}
pub unsafe fn vma_start_write(_vma: *mut vm_area_struct) {}

unsafe extern "C" {
    pub static vma_dummy_vm_ops: vm_operations_struct;
    pub static mut stack_guard_gap: c_ulong;
    pub fn rlimit(limit: c_uint) -> c_ulong;
    pub fn get_current() -> *mut task_struct;
}

pub const MMF_HAS_MDWE: c_int = 28;
pub unsafe fn current() -> *mut task_struct {
    unsafe { get_current() }
}

/*
 * Define the task command name length as enum, then it can be visible to
 * BPF programs.
 */
pub const TASK_COMM_LEN: usize = 16;

/* PARTIALLY implemented types. */
#[repr(C)]
pub union mm_struct_def_flags {
    pub def_flags: vm_flags_t,
    pub def_vma_flags: vma_flags_t,
}

#[repr(C)]
pub struct mm_struct {
    pub mm_mt: maple_tree,
    pub map_count: c_int,        /* number of VMAs */
    pub total_vm: c_ulong,       /* Total pages mapped */
    pub locked_vm: c_ulong,      /* Pages that have PG_mlocked set */
    pub data_vm: c_ulong,        /* VM_WRITE & ~VM_SHARED & ~VM_STACK */
    pub exec_vm: c_ulong,        /* VM_EXEC & ~VM_WRITE & ~VM_STACK */
    pub stack_vm: c_ulong,       /* VM_STACK */
    pub u: mm_struct_def_flags,
    pub flags: mm_flags_t,       /* Must use mm_flags_* helpers to access */
}

#[repr(C)]
pub struct address_space {
    pub i_mmap: rb_root_cached,
    pub flags: c_ulong,
    pub i_mmap_writable: atomic_t,
}

#[repr(C)]
pub struct file_operations {
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub mmap_prepare: Option<unsafe extern "C" fn(*mut vm_area_desc) -> c_int>,
}

#[repr(C)]
pub struct file {
    pub f_mapping: *mut address_space,
    pub f_op: *const file_operations,
}

#[repr(C)]
pub struct anon_vma_chain {
    pub anon_vma: *mut anon_vma,
    pub same_vma: list_head,
}

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub mm: *mut mm_struct,

    /* Used for emulating ABI behavior of previous Linux versions: */
    pub personality: c_uint,
}

#[repr(C)]
pub struct kref {
    pub refcount: refcount_t,
}

#[repr(C)]
pub struct anon_vma_name {
    pub kref: kref,
    /* The name needs to be at the end because it is dynamically sized. */
    pub name: [c_char; 0],
}

/*
 * Contains declarations that are DUPLICATED from kernel source in order to
 * faciliate userland VMA testing.
 *
 * These must be kept in sync with kernel source.
 */

pub const VMA_LOCK_OFFSET: c_ulong = 0x40000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freeptr_t {
    pub v: c_ulong,
}

pub const VM_NONE: vm_flags_t = 0x00000000;

pub type vma_flag_t = c_int;

pub const VMA_READ_BIT: vma_flag_t = 0;
pub const VMA_WRITE_BIT: vma_flag_t = 1;
pub const VMA_EXEC_BIT: vma_flag_t = 2;
pub const VMA_SHARED_BIT: vma_flag_t = 3;
/* mprotect() hardcodes VM_MAYREAD >> 4 == VM_READ, and so for r/w/x bits. */
pub const VMA_MAYREAD_BIT: vma_flag_t = 4;     /* limits for mprotect() etc. */
pub const VMA_MAYWRITE_BIT: vma_flag_t = 5;
pub const VMA_MAYEXEC_BIT: vma_flag_t = 6;
pub const VMA_MAYSHARE_BIT: vma_flag_t = 7;
pub const VMA_GROWSDOWN_BIT: vma_flag_t = 8;   /* general info on the segment */
/* CONFIG_MMU: missing pages tracking; !CONFIG_MMU: MAYOVERLAY reuses bit 9. */
pub const VMA_UFFD_MISSING_BIT: vma_flag_t = 9;
pub const VMA_MAYOVERLAY_BIT: vma_flag_t = 9;
/* Page-ranges managed without "struct page", just pure PFN */
pub const VMA_PFNMAP_BIT: vma_flag_t = 10;
pub const VMA_MAYBE_GUARD_BIT: vma_flag_t = 11;
pub const VMA_UFFD_WP_BIT: vma_flag_t = 12;    /* wrprotect pages tracking */
pub const VMA_LOCKED_BIT: vma_flag_t = 13;
pub const VMA_IO_BIT: vma_flag_t = 14;         /* Memory mapped I/O or similar */
pub const VMA_SEQ_READ_BIT: vma_flag_t = 15;   /* App will access data sequentially */
pub const VMA_RAND_READ_BIT: vma_flag_t = 16;  /* App will not benefit from clustered reads */
pub const VMA_DONTCOPY_BIT: vma_flag_t = 17;   /* Do not copy this vma on fork */
pub const VMA_DONTEXPAND_BIT: vma_flag_t = 18; /* Cannot expand with mremap() */
pub const VMA_LOCKONFAULT_BIT: vma_flag_t = 19;/* Lock pages covered when faulted in */
pub const VMA_ACCOUNT_BIT: vma_flag_t = 20;    /* Is a VM accounted object */
pub const VMA_NORESERVE_BIT: vma_flag_t = 21;  /* should the VM suppress accounting */
pub const VMA_HUGETLB_BIT: vma_flag_t = 22;    /* Huge TLB Page VM */
pub const VMA_SYNC_BIT: vma_flag_t = 23;       /* Synchronous page faults */
pub const VMA_ARCH_1_BIT: vma_flag_t = 24;     /* Architecture-specific flag */
pub const VMA_WIPEONFORK_BIT: vma_flag_t = 25; /* Wipe VMA contents in child. */
pub const VMA_DONTDUMP_BIT: vma_flag_t = 26;   /* Do not include in the core dump */
pub const VMA_SOFTDIRTY_BIT: vma_flag_t = 27;  /* NOT soft dirty clean area */
pub const VMA_MIXEDMAP_BIT: vma_flag_t = 28;   /* Can contain struct page and pure PFN pages */
pub const VMA_HUGEPAGE_BIT: vma_flag_t = 29;   /* MADV_HUGEPAGE marked this vma */
pub const VMA_NOHUGEPAGE_BIT: vma_flag_t = 30; /* MADV_NOHUGEPAGE marked this vma */
pub const VMA_MERGEABLE_BIT: vma_flag_t = 31;  /* KSM may merge identical pages */
/* These bits are reused, we define specific uses below. */
pub const VMA_HIGH_ARCH_0_BIT: vma_flag_t = 32;
pub const VMA_HIGH_ARCH_1_BIT: vma_flag_t = 33;
pub const VMA_HIGH_ARCH_2_BIT: vma_flag_t = 34;
pub const VMA_HIGH_ARCH_3_BIT: vma_flag_t = 35;
pub const VMA_HIGH_ARCH_4_BIT: vma_flag_t = 36;
pub const VMA_HIGH_ARCH_5_BIT: vma_flag_t = 37;
pub const VMA_HIGH_ARCH_6_BIT: vma_flag_t = 38;
/*
 * This flag is used to connect VFIO to arch specific KVM code. It
 * indicates that the memory under this VMA is safe for use with any
 * non-cachable memory type inside KVM. Some VFIO devices, on some
 * platforms, are thought to be unsafe and can cause machine crashes
 * if KVM does not lock down the memory type.
 */
pub const VMA_ALLOW_ANY_UNCACHED_BIT: vma_flag_t = 39;
/* CONFIG_PPC32 aliases DROPPABLE to ARCH_1; otherwise it is bit 40. */
pub const VMA_DROPPABLE_BIT: vma_flag_t = 40;
pub const VMA_UFFD_MINOR_BIT: vma_flag_t = 41;
pub const VMA_SEALED_BIT: vma_flag_t = 42;
/* Flags that reuse flags above. */
pub const VMA_PKEY_BIT0_BIT: vma_flag_t = VMA_HIGH_ARCH_0_BIT;
pub const VMA_PKEY_BIT1_BIT: vma_flag_t = VMA_HIGH_ARCH_1_BIT;
pub const VMA_PKEY_BIT2_BIT: vma_flag_t = VMA_HIGH_ARCH_2_BIT;
pub const VMA_PKEY_BIT3_BIT: vma_flag_t = VMA_HIGH_ARCH_3_BIT;
pub const VMA_PKEY_BIT4_BIT: vma_flag_t = VMA_HIGH_ARCH_4_BIT;
/* CONFIG_X86_USER_SHADOW_STACK aliases SHADOW_STACK to HIGH_ARCH_5;
 * CONFIG_ARM64_GCS aliases SHADOW_STACK to HIGH_ARCH_6. */
pub const VMA_SHADOW_STACK_BIT: vma_flag_t = VMA_HIGH_ARCH_5_BIT;
pub const VMA_SAO_BIT: vma_flag_t = VMA_ARCH_1_BIT;          /* Strong Access Ordering (powerpc) */
pub const VMA_GROWSUP_BIT: vma_flag_t = VMA_ARCH_1_BIT;      /* parisc */
pub const VMA_SPARC_ADI_BIT: vma_flag_t = VMA_ARCH_1_BIT;    /* sparc64 */
pub const VMA_ARM64_BTI_BIT: vma_flag_t = VMA_ARCH_1_BIT;    /* arm64 */
pub const VMA_ARCH_CLEAR_BIT: vma_flag_t = VMA_ARCH_1_BIT;   /* sparc64, arm64 */
pub const VMA_MAPPED_COPY_BIT: vma_flag_t = VMA_ARCH_1_BIT;  /* !CONFIG_MMU */
pub const VMA_MTE_BIT: vma_flag_t = VMA_HIGH_ARCH_4_BIT;     /* arm64 */
pub const VMA_MTE_ALLOWED_BIT: vma_flag_t = VMA_HIGH_ARCH_5_BIT; /* arm64 */
/* CONFIG_STACK_GROWSUP aliases STACK to GROWSUP and STACK_EARLY to GROWSDOWN. */
pub const VMA_STACK_BIT: vma_flag_t = VMA_GROWSDOWN_BIT;
pub const VMA_STACK_EARLY_BIT: vma_flag_t = VMA_GROWSDOWN_BIT;

pub const fn bit(nr: c_int) -> c_ulong {
    1u64.wrapping_shl(nr as u32) as c_ulong
}

pub const fn init_vm_flag(bitnum: vma_flag_t) -> vm_flags_t {
    bit(bitnum)
}

pub const VM_READ: vm_flags_t = init_vm_flag(VMA_READ_BIT);
pub const VM_WRITE: vm_flags_t = init_vm_flag(VMA_WRITE_BIT);
pub const VM_EXEC: vm_flags_t = init_vm_flag(VMA_EXEC_BIT);
pub const VM_SHARED: vm_flags_t = init_vm_flag(VMA_SHARED_BIT);
pub const VM_MAYREAD: vm_flags_t = init_vm_flag(VMA_MAYREAD_BIT);
pub const VM_MAYWRITE: vm_flags_t = init_vm_flag(VMA_MAYWRITE_BIT);
pub const VM_MAYEXEC: vm_flags_t = init_vm_flag(VMA_MAYEXEC_BIT);
pub const VM_MAYSHARE: vm_flags_t = init_vm_flag(VMA_MAYSHARE_BIT);
pub const VM_GROWSDOWN: vm_flags_t = init_vm_flag(VMA_GROWSDOWN_BIT);
/* CONFIG_MMU selects INIT_VM_FLAG(UFFD_MISSING); !CONFIG_MMU selects VM_NONE and defines VM_MAYOVERLAY. */
pub const VM_UFFD_MISSING: vm_flags_t = init_vm_flag(VMA_UFFD_MISSING_BIT);
pub const VM_MAYOVERLAY: vm_flags_t = init_vm_flag(VMA_MAYOVERLAY_BIT);
pub const VM_PFNMAP: vm_flags_t = init_vm_flag(VMA_PFNMAP_BIT);
pub const VM_MAYBE_GUARD: vm_flags_t = init_vm_flag(VMA_MAYBE_GUARD_BIT);
pub const VM_UFFD_WP: vm_flags_t = init_vm_flag(VMA_UFFD_WP_BIT);
pub const VM_LOCKED: vm_flags_t = init_vm_flag(VMA_LOCKED_BIT);
pub const VM_IO: vm_flags_t = init_vm_flag(VMA_IO_BIT);
pub const VM_SEQ_READ: vm_flags_t = init_vm_flag(VMA_SEQ_READ_BIT);
pub const VM_RAND_READ: vm_flags_t = init_vm_flag(VMA_RAND_READ_BIT);
pub const VM_DONTCOPY: vm_flags_t = init_vm_flag(VMA_DONTCOPY_BIT);
pub const VM_DONTEXPAND: vm_flags_t = init_vm_flag(VMA_DONTEXPAND_BIT);
pub const VM_LOCKONFAULT: vm_flags_t = init_vm_flag(VMA_LOCKONFAULT_BIT);
pub const VM_ACCOUNT: vm_flags_t = init_vm_flag(VMA_ACCOUNT_BIT);
pub const VM_NORESERVE: vm_flags_t = init_vm_flag(VMA_NORESERVE_BIT);
pub const VM_HUGETLB: vm_flags_t = init_vm_flag(VMA_HUGETLB_BIT);
pub const VM_SYNC: vm_flags_t = init_vm_flag(VMA_SYNC_BIT);
pub const VM_ARCH_1: vm_flags_t = init_vm_flag(VMA_ARCH_1_BIT);
pub const VM_WIPEONFORK: vm_flags_t = init_vm_flag(VMA_WIPEONFORK_BIT);
pub const VM_DONTDUMP: vm_flags_t = init_vm_flag(VMA_DONTDUMP_BIT);
/* CONFIG_MEM_SOFT_DIRTY selects INIT_VM_FLAG(SOFTDIRTY); otherwise VM_NONE. */
pub const VM_SOFTDIRTY: vm_flags_t = init_vm_flag(VMA_SOFTDIRTY_BIT);
pub const VM_MIXEDMAP: vm_flags_t = init_vm_flag(VMA_MIXEDMAP_BIT);
pub const VM_HUGEPAGE: vm_flags_t = init_vm_flag(VMA_HUGEPAGE_BIT);
pub const VM_NOHUGEPAGE: vm_flags_t = init_vm_flag(VMA_NOHUGEPAGE_BIT);
pub const VM_MERGEABLE: vm_flags_t = init_vm_flag(VMA_MERGEABLE_BIT);
pub const VM_STACK: vm_flags_t = init_vm_flag(VMA_STACK_BIT);
/* CONFIG_STACK_GROWSUP selects INIT_VM_FLAG(STACK_EARLY); otherwise VM_NONE. */
pub const VM_STACK_EARLY: vm_flags_t = VM_NONE;
pub const VM_PKEY_SHIFT: c_int = VMA_HIGH_ARCH_0_BIT;
pub const VM_PKEY_BIT0: vm_flags_t = init_vm_flag(VMA_PKEY_BIT0_BIT);
pub const VM_PKEY_BIT1: vm_flags_t = init_vm_flag(VMA_PKEY_BIT1_BIT);
pub const VM_PKEY_BIT2: vm_flags_t = init_vm_flag(VMA_PKEY_BIT2_BIT);
pub const VM_PKEY_BIT3: vm_flags_t = init_vm_flag(VMA_PKEY_BIT3_BIT);
pub const VM_PKEY_BIT4: vm_flags_t = init_vm_flag(VMA_PKEY_BIT4_BIT);
/* CONFIG_X86_USER_SHADOW_STACK || CONFIG_ARM64_GCS selects INIT_VM_FLAG(SHADOW_STACK); otherwise VM_NONE. */
pub const VM_SHADOW_STACK: vm_flags_t = VM_NONE;
pub const VM_SAO: vm_flags_t = init_vm_flag(VMA_SAO_BIT);
pub const VM_GROWSUP: vm_flags_t = VM_NONE;
pub const VM_SPARC_ADI: vm_flags_t = init_vm_flag(VMA_SPARC_ADI_BIT);
pub const VM_ARCH_CLEAR: vm_flags_t = init_vm_flag(VMA_ARCH_CLEAR_BIT);
pub const VM_ARM64_BTI: vm_flags_t = init_vm_flag(VMA_ARM64_BTI_BIT);
pub const VM_MAPPED_COPY: vm_flags_t = init_vm_flag(VMA_MAPPED_COPY_BIT);
pub const VM_MTE: vm_flags_t = VM_NONE;
pub const VM_MTE_ALLOWED: vm_flags_t = VM_NONE;
pub const VM_UFFD_MINOR: vm_flags_t = VM_NONE;
pub const VM_ALLOW_ANY_UNCACHED: vm_flags_t = VM_NONE;
pub const VM_SEALED: vm_flags_t = VM_NONE;
pub const VM_DROPPABLE: vm_flags_t = VM_NONE;

/* Bits set in the VMA until the stack is in its final location */
pub const VM_STACK_INCOMPLETE_SETUP: vm_flags_t = VM_RAND_READ | VM_SEQ_READ | VM_STACK_EARLY;
pub const VM_STARTGAP_FLAGS: vm_flags_t = VM_GROWSDOWN | VM_SHADOW_STACK;
pub const VM_ACCESS_FLAGS: vm_flags_t = VM_READ | VM_WRITE | VM_EXEC;
/*
 * Special vmas that are non-mergable, non-mlock()able.
 */
pub const VM_SPECIAL: vm_flags_t = VM_IO | VM_DONTEXPAND | VM_PFNMAP | VM_MIXEDMAP;

pub const DEFAULT_MAP_WINDOW: c_ulong = (1u64 << 47) as c_ulong - PAGE_SIZE;
pub const TASK_SIZE_LOW: c_ulong = DEFAULT_MAP_WINDOW;
pub const TASK_SIZE_MAX: c_ulong = DEFAULT_MAP_WINDOW;
pub const STACK_TOP: c_ulong = TASK_SIZE_LOW;
pub const STACK_TOP_MAX: c_ulong = TASK_SIZE_MAX;

/* This mask represents all the VMA flag bits used by mlock */
pub const VM_LOCKED_MASK: vm_flags_t = VM_LOCKED | VM_LOCKONFAULT;

pub const RLIMIT_STACK: c_uint = 3;     /* max stack size */
pub const RLIMIT_MEMLOCK: c_uint = 8;   /* max locked-in-memory address space */

pub const CAP_IPC_LOCK: c_int = 14;
pub const VM_COPY_ON_FORK: vm_flags_t = VM_PFNMAP | VM_MIXEDMAP | VM_UFFD_WP | VM_MAYBE_GUARD;
pub const AS_MM_ALL_LOCKS: c_int = 2;

/*
 * Flags for bug emulation.
 *
 * These occupy the top three bytes.
 */
pub const READ_IMPLIES_EXEC: c_uint = 0x0400000;

#[repr(C)]
pub struct vma_iterator {
    pub mas: ma_state,
}

pub const MAPCOUNT_ELF_CORE_MARGIN: c_int = 5;
pub const DEFAULT_MAX_MAP_COUNT: c_int = USHRT_MAX - MAPCOUNT_ELF_CORE_MARGIN;

#[inline]
pub unsafe fn vma_flags_empty(flags: *const vma_flags_t) -> bool {
    let bitmap = unsafe { (*flags).__vma_flags.as_ptr() };
    unsafe { bitmap_empty(bitmap, NUM_VMA_FLAG_BITS) }
}

/* What action should be taken after an .mmap_prepare call is complete? */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mmap_action_type {
    MMAP_NOTHING,        /* Mapping is complete, no further action. */
    MMAP_REMAP_PFN,      /* Remap PFN range. */
    MMAP_IO_REMAP_PFN,   /* I/O remap PFN range. */
    MMAP_SIMPLE_IO_REMAP,/* I/O remap with guardrails. */
    MMAP_MAP_KERNEL_PAGES,/* Map kernel page range from an array. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_action_remap {
    pub start: c_ulong,
    pub start_pfn: c_ulong,
    pub size: c_ulong,
    pub pgprot: pgprot_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_action_simple_ioremap {
    pub start_phys_addr: phys_addr_t,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_action_map_kernel {
    pub start: c_ulong,
    pub pages: *mut *mut page,
    pub nr_pages: c_ulong,
    pub pgoff: pgoff_t,
}

#[repr(C)]
pub union mmap_action_u {
    pub remap: mmap_action_remap,
    pub simple_ioremap: mmap_action_simple_ioremap,
    pub map_kernel: mmap_action_map_kernel,
}

/*
 * Describes an action an mmap_prepare hook can instruct to be taken to complete
 * the mapping of a VMA. Specified in vm_area_desc.
 */
#[repr(C)]
pub struct mmap_action {
    pub u: mmap_action_u,
    pub type_: mmap_action_type,
    /*
     * If non-zero, replace errors that arise from mmap actions with this
     * value instead. Only valid error codes may be specified.
     */
    pub error_override: c_int,
    /*
     * This should be set in rare instances where the operation required
     * that the rmap should not be able to access the VMA until
     * completely set up.
     */
    pub hide_from_rmap_until_complete: bool,
}

/* Operations which modify VMAs. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum vma_operation {
    VMA_OP_SPLIT,
    VMA_OP_MERGE_UNFAULTED,
    VMA_OP_REMAP,
    VMA_OP_FORK,
}

/*
 * Describes a VMA that is about to be mmap()'ed. Drivers may choose to
 * manipulate mutable fields which will cause those fields to be updated in the
 * resultant VMA.
 *
 * Helper functions are not required for manipulating any field.
 */
#[repr(C)]
pub struct vm_area_desc {
    /* Immutable state. */
    pub mm: *mut mm_struct,
    pub file: *mut file, /* May vary from vm_file in stacked callers. */
    pub start: c_ulong,
    pub end: c_ulong,

    /* Mutable fields. Populated with initial state. */
    pub pgoff: pgoff_t,
    pub vm_file: *mut file,
    pub vma_flags: vma_flags_t,
    pub page_prot: pgprot_t,

    /* Write-only fields. */
    pub vm_ops: *const vm_operations_struct,
    pub private_data: *mut c_void,

    /* Take further action? */
    pub action: mmap_action,
}

#[repr(C)]
pub struct vm_operations_struct {
    pub open: Option<unsafe extern "C" fn(*mut vm_area_struct)>,
    pub close: Option<unsafe extern "C" fn(*mut vm_area_struct)>,
    pub mapped: Option<unsafe extern "C" fn(c_ulong, c_ulong, pgoff_t, *const file, *mut *mut c_void) -> c_int>,
    pub may_split: Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong) -> c_int>,
    pub mremap: Option<unsafe extern "C" fn(*mut vm_area_struct) -> c_int>,
    pub mprotect: Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, c_ulong, c_ulong) -> c_int>,
    pub fault: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub huge_fault: Option<unsafe extern "C" fn(*mut vm_fault, c_uint) -> vm_fault_t>,
    pub map_pages: Option<unsafe extern "C" fn(*mut vm_fault, pgoff_t, pgoff_t) -> vm_fault_t>,
    pub pagesize: Option<unsafe extern "C" fn(*mut vm_area_struct) -> c_ulong>,
    pub page_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub pfn_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub access: Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, *mut c_void, c_int, c_int) -> c_int>,
    pub name: Option<unsafe extern "C" fn(*mut vm_area_struct) -> *const c_char>,
    /* CONFIG_NUMA adds set_policy and get_policy callbacks.
     * CONFIG_FIND_NORMAL_PAGE adds find_normal_page callback. */
}

#[repr(C)]
pub struct vm_unmapped_area_info {
    pub flags: c_ulong,
    pub length: c_ulong,
    pub low_limit: c_ulong,
    pub high_limit: c_ulong,
    pub align_mask: c_ulong,
    pub align_offset: c_ulong,
    pub start_gap: c_ulong,
}
pub const VM_UNMAPPED_AREA_TOPDOWN: c_ulong = 1;

#[repr(C)]
pub struct pagetable_move_control {
    pub old: *mut vm_area_struct, /* Source VMA. */
    pub new: *mut vm_area_struct, /* Destination VMA. */
    pub old_addr: c_ulong,        /* Address from which the move begins. */
    pub old_end: c_ulong,         /* Exclusive address at which old range ends. */
    pub new_addr: c_ulong,        /* Address to move page tables to. */
    pub len_in: c_ulong,          /* Bytes to remap specified by user. */
    pub need_rmap_locks: bool,    /* Do rmap locks need to be taken? */
    pub for_stack: bool,          /* Is this an early temp stack being moved? */
}

#[inline]
pub unsafe fn vma_iter_invalidate(vmi: *mut vma_iterator) {
    unsafe { mas_pause(&mut (*vmi).mas) };
}

#[inline]
pub fn pgprot_val(x: pgprot_t) -> c_ulong {
    x.pgprot
}

#[inline]
pub fn __pgprot(x: c_ulong) -> pgprot_t {
    pgprot_t { pgprot: x }
}

#[inline]
pub fn pgprot_modify(oldprot: pgprot_t, newprot: pgprot_t) -> pgprot_t {
    __pgprot(pgprot_val(oldprot) | pgprot_val(newprot))
}

#[inline]
pub fn vm_get_page_prot(vm_flags: vm_flags_t) -> pgprot_t {
    __pgprot(vm_flags)
}

#[inline]
pub unsafe fn mm_flags_test(flag: c_int, mm: *const mm_struct) -> bool {
    unsafe { test_bit(flag, (*mm).flags.__mm_flags.as_ptr()) }
}

/*
 * Copy value to the first system word of VMA flags, non-atomically.
 *
 * IMPORTANT: This does not overwrite bytes past the first system word. The
 * caller must account for this.
 */
#[inline]
pub unsafe fn vma_flags_overwrite_word(flags: *mut vma_flags_t, value: c_ulong) {
    unsafe { (*flags).__vma_flags[0] = value };
}

/*
 * Copy value to the first system word of VMA flags ONCE, non-atomically.
 *
 * IMPORTANT: This does not overwrite bytes past the first system word. The
 * caller must account for this.
 */
#[inline]
pub unsafe fn vma_flags_overwrite_word_once(flags: *mut vma_flags_t, value: c_ulong) {
    unsafe { WRITE_ONCE(&mut (*flags).__vma_flags[0], value) };
}

/* Update the first system word of VMA flags setting bits, non-atomically. */
#[inline]
pub unsafe fn vma_flags_set_word(flags: *mut vma_flags_t, value: c_ulong) {
    unsafe { (*flags).__vma_flags[0] |= value };
}

/* Update the first system word of VMA flags clearing bits, non-atomically. */
#[inline]
pub unsafe fn vma_flags_clear_word(flags: *mut vma_flags_t, value: c_ulong) {
    unsafe { (*flags).__vma_flags[0] &= !value };
}

#[inline]
pub unsafe fn vma_flags_clear_all(flags: *mut vma_flags_t) {
    unsafe { bitmap_zero((*flags).__vma_flags.as_mut_ptr(), NUM_VMA_FLAG_BITS) };
}

/*
 * Helper function which converts a vma_flags_t value to a legacy vm_flags_t
 * value. This is only valid if the input flags value can be expressed in a
 * system word.
 *
 * Will be removed once the conversion to VMA flags is complete.
 */
#[inline]
pub fn vma_flags_to_legacy(flags: vma_flags_t) -> vm_flags_t {
    flags.__vma_flags[0] as vm_flags_t
}

/*
 * Helper function which converts a legacy vm_flags_t value to a vma_flags_t
 * value.
 *
 * Will be removed once the conversion to VMA flags is complete.
 */
#[inline]
pub unsafe fn legacy_to_vma_flags(flags: vm_flags_t) -> vma_flags_t {
    let mut ret = EMPTY_VMA_FLAGS;
    unsafe { vma_flags_overwrite_word(&mut ret, flags) };
    ret
}

#[inline]
pub unsafe fn vma_flags_set_flag(flags: *mut vma_flags_t, bit: vma_flag_t) {
    unsafe { __set_bit(bit as c_int, (*flags).__vma_flags.as_mut_ptr()) };
}

/* Use when VMA is not part of the VMA tree and needs no locking */
#[inline]
pub unsafe fn vm_flags_init(vma: *mut vm_area_struct, flags: vm_flags_t) {
    unsafe {
        vma_flags_clear_all(&mut (*vma).flags);
        vma_flags_overwrite_word(&mut (*vma).flags, flags);
    }
}

#[inline]
pub unsafe fn vm_flags_reset(vma: *mut vm_area_struct, flags: vm_flags_t) {
    unsafe {
        vma_assert_write_locked(vma);
        vm_flags_init(vma, flags);
    }
}

#[inline]
pub unsafe fn vma_flags_reset_once(vma: *mut vm_area_struct, flags: *mut vma_flags_t) {
    let word = unsafe { (*flags).__vma_flags[0] };
    unsafe {
        vma_flags_overwrite_word_once(&mut (*vma).flags, word);
        if NUM_VMA_FLAG_BITS > BITS_PER_LONG {
            let dst = (*vma).flags.__vma_flags.as_mut_ptr().add(1);
            let src = (*flags).__vma_flags.as_ptr().add(1);
            bitmap_copy(dst, src, NUM_VMA_FLAG_BITS - BITS_PER_LONG);
        }
    }
}

#[inline]
pub unsafe fn vm_flags_set(vma: *mut vm_area_struct, flags: vm_flags_t) {
    unsafe {
        vma_start_write(vma);
        vma_flags_set_word(&mut (*vma).flags, flags);
    }
}

#[inline]
pub unsafe fn vm_flags_clear(vma: *mut vm_area_struct, flags: vm_flags_t) {
    unsafe {
        vma_start_write(vma);
        vma_flags_clear_word(&mut (*vma).flags, flags);
    }
}

#[inline]
pub unsafe fn __mk_vma_flags(mut flags: vma_flags_t, count: size_t, bits: *const vma_flag_t) -> vma_flags_t {
    let mut i: c_int = 0;
    while (i as size_t) < count {
        unsafe { vma_flags_set_flag(&mut flags, *bits.add(i as usize)) };
        i += 1;
    }
    flags
}

#[inline]
pub unsafe fn vma_flags_count(flags: *const vma_flags_t) -> c_int {
    unsafe { bitmap_weight((*flags).__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) }
}

#[inline]
pub unsafe fn vma_flags_test(flags: *const vma_flags_t, bit: vma_flag_t) -> bool {
    unsafe { test_bit(bit as c_int, (*flags).__vma_flags.as_ptr()) }
}

#[inline]
pub unsafe fn vma_flags_and_mask(flags: *const vma_flags_t, to_and: vma_flags_t) -> vma_flags_t {
    let mut dst = EMPTY_VMA_FLAGS;
    unsafe {
        bitmap_and(dst.__vma_flags.as_mut_ptr(), (*flags).__vma_flags.as_ptr(), to_and.__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS);
    }
    dst
}

#[inline]
pub unsafe fn vma_flags_test_any_mask(flags: *const vma_flags_t, to_test: vma_flags_t) -> bool {
    unsafe { bitmap_intersects(to_test.__vma_flags.as_ptr(), (*flags).__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) }
}

#[inline]
pub unsafe fn vma_flags_test_all_mask(flags: *const vma_flags_t, to_test: vma_flags_t) -> bool {
    unsafe { bitmap_subset(to_test.__vma_flags.as_ptr(), (*flags).__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) }
}

#[inline]
pub unsafe fn vma_flags_test_single_mask(flags: *const vma_flags_t, flagmask: vma_flags_t) -> bool {
    unsafe {
        VM_WARN_ON_ONCE(vma_flags_count(&flagmask) > 1);
        vma_flags_test_any_mask(flags, flagmask)
    }
}

#[inline]
pub unsafe fn vma_flags_set_mask(flags: *mut vma_flags_t, to_set: vma_flags_t) {
    unsafe { bitmap_or((*flags).__vma_flags.as_mut_ptr(), (*flags).__vma_flags.as_ptr(), to_set.__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) };
}

#[inline]
pub unsafe fn vma_flags_clear_mask(flags: *mut vma_flags_t, to_clear: vma_flags_t) {
    unsafe { bitmap_andnot((*flags).__vma_flags.as_mut_ptr(), (*flags).__vma_flags.as_ptr(), to_clear.__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) };
}

#[inline]
pub unsafe fn vma_flags_diff_pair(flags: *const vma_flags_t, flags_other: *const vma_flags_t) -> vma_flags_t {
    let mut dst = EMPTY_VMA_FLAGS;
    unsafe { bitmap_xor(dst.__vma_flags.as_mut_ptr(), (*flags).__vma_flags.as_ptr(), (*flags_other).__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) };
    dst
}

#[inline]
pub unsafe fn vma_flags_same_pair(flags: *const vma_flags_t, flags_other: *const vma_flags_t) -> bool {
    unsafe { bitmap_equal((*flags).__vma_flags.as_ptr(), (*flags_other).__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) }
}

#[inline]
pub unsafe fn vma_flags_same_mask(flags: *const vma_flags_t, flags_other: vma_flags_t) -> bool {
    unsafe { bitmap_equal((*flags).__vma_flags.as_ptr(), flags_other.__vma_flags.as_ptr(), NUM_VMA_FLAG_BITS) }
}

#[inline]
pub unsafe fn vma_test(vma: *const vm_area_struct, bit: vma_flag_t) -> bool {
    unsafe { vma_flags_test(&(*vma).flags, bit) }
}

#[inline]
pub unsafe fn vma_test_any_mask(vma: *const vm_area_struct, flags: vma_flags_t) -> bool {
    unsafe { vma_flags_test_any_mask(&(*vma).flags, flags) }
}

#[inline]
pub unsafe fn vma_test_all_mask(vma: *const vm_area_struct, flags: vma_flags_t) -> bool {
    unsafe { vma_flags_test_all_mask(&(*vma).flags, flags) }
}

#[inline]
pub unsafe fn vma_test_single_mask(vma: *const vm_area_struct, flagmask: vma_flags_t) -> bool {
    unsafe { vma_flags_test_single_mask(&(*vma).flags, flagmask) }
}

#[inline]
pub unsafe fn vma_set_flags_mask(vma: *mut vm_area_struct, flags: vma_flags_t) {
    unsafe { vma_flags_set_mask(&mut (*vma).flags, flags) };
}

#[inline]
pub unsafe fn vma_clear_flags_mask(vma: *mut vm_area_struct, flags: vma_flags_t) {
    unsafe { vma_flags_clear_mask(&mut (*vma).flags, flags) };
}

#[inline]
pub unsafe fn vma_desc_test(desc: *const vm_area_desc, bit: vma_flag_t) -> bool {
    unsafe { vma_flags_test(&(*desc).vma_flags, bit) }
}

#[inline]
pub unsafe fn vma_desc_test_any_mask(desc: *const vm_area_desc, flags: vma_flags_t) -> bool {
    unsafe { vma_flags_test_any_mask(&(*desc).vma_flags, flags) }
}

#[inline]
pub unsafe fn vma_desc_test_all_mask(desc: *const vm_area_desc, flags: vma_flags_t) -> bool {
    unsafe { vma_flags_test_all_mask(&(*desc).vma_flags, flags) }
}

#[inline]
pub unsafe fn vma_desc_set_flags_mask(desc: *mut vm_area_desc, flags: vma_flags_t) {
    unsafe { vma_flags_set_mask(&mut (*desc).vma_flags, flags) };
}

#[inline]
pub unsafe fn vma_desc_clear_flags_mask(desc: *mut vm_area_desc, flags: vma_flags_t) {
    unsafe { vma_flags_clear_mask(&mut (*desc).vma_flags, flags) };
}

#[inline]
pub unsafe fn is_shared_maywrite(flags: *const vma_flags_t) -> bool {
    let bits = [VMA_SHARED_BIT, VMA_MAYWRITE_BIT];
    let mask = unsafe { __mk_vma_flags(EMPTY_VMA_FLAGS, 2, bits.as_ptr()) };
    unsafe { vma_flags_test_all_mask(flags, mask) }
}

#[inline]
pub unsafe fn vma_is_shared_maywrite(vma: *mut vm_area_struct) -> bool {
    unsafe { is_shared_maywrite(&(*vma).flags) }
}

#[inline]
pub unsafe fn vma_flags_is_cow_mapping(flags: *const vma_flags_t) -> bool {
    unsafe { vma_flags_test(flags, VMA_MAYWRITE_BIT) && !vma_flags_test(flags, VMA_SHARED_BIT) }
}

#[inline]
pub unsafe fn vma_is_cow_mapping(vma: *const vm_area_struct) -> bool {
    unsafe { vma_flags_is_cow_mapping(&(*vma).flags) }
}

#[inline]
pub unsafe fn vma_next(vmi: *mut vma_iterator) -> *mut vm_area_struct {
    /*
     * Uses mas_find() to get the first VMA when the iterator starts.
     * Calling mas_next() could skip the first entry.
     */
    unsafe { mas_find(&mut (*vmi).mas, ULONG_MAX) as *mut vm_area_struct }
}

#[inline]
pub unsafe fn vma_is_attached(vma: *mut vm_area_struct) -> bool {
    unsafe { refcount_read(&(*vma).vm_refcnt) != 0 }
}

#[inline]
pub unsafe fn vma_assert_attached(vma: *mut vm_area_struct) {
    unsafe { WARN_ON_ONCE(!vma_is_attached(vma)); }
}

#[inline]
pub unsafe fn vma_assert_detached(vma: *mut vm_area_struct) {
    unsafe { WARN_ON_ONCE(vma_is_attached(vma)); }
}

pub unsafe fn vma_assert_write_locked(_vma: *mut vm_area_struct);

#[inline]
pub unsafe fn vma_mark_attached(vma: *mut vm_area_struct) {
    unsafe {
        vma_assert_write_locked(vma);
        vma_assert_detached(vma);
        refcount_set_release(&mut (*vma).vm_refcnt, 1);
    }
}

#[inline]
pub unsafe fn vma_mark_detached(vma: *mut vm_area_struct) {
    unsafe {
        vma_assert_write_locked(vma);
        vma_assert_attached(vma);
        /* We are the only writer, so no need to use vma_refcount_put(). */
        if unlikely(!refcount_dec_and_test(&mut (*vma).vm_refcnt)) {
            /*
             * Reader must have temporarily raised vm_refcnt but it will
             * drop it without using the vma since vma is write-locked.
             */
        }
    }
}

#[inline]
pub unsafe fn vma_init(vma: *mut vm_area_struct, mm: *mut mm_struct) {
    unsafe {
        memset(vma as *mut c_void, 0, core::mem::size_of_val(&*vma));
        (*vma).vm_mm = mm;
        (*vma).vm_ops = &vma_dummy_vm_ops;
        INIT_LIST_HEAD(&mut (*vma).anon_vma_chain);
        (*vma).vm_lock_seq = UINT_MAX;
    }
}

#[inline]
pub fn is_exec_mapping(flags: vm_flags_t) -> bool {
    (flags & (VM_EXEC | VM_WRITE | VM_STACK)) == VM_EXEC
}

#[inline]
pub fn is_stack_mapping(flags: vm_flags_t) -> bool {
    ((flags & VM_STACK) == VM_STACK) || ((flags & VM_SHADOW_STACK) != 0)
}

#[inline]
pub fn is_data_mapping(flags: vm_flags_t) -> bool {
    (flags & (VM_WRITE | VM_SHARED | VM_STACK)) == VM_WRITE
}

#[inline]
pub unsafe fn vm_stat_account(mm: *mut mm_struct, flags: vm_flags_t, npages: c_long) {
    unsafe {
        WRITE_ONCE(&mut (*mm).total_vm, READ_ONCE(&(*mm).total_vm).wrapping_add(npages as c_ulong));
        if is_exec_mapping(flags) {
            (*mm).exec_vm = (*mm).exec_vm.wrapping_add(npages as c_ulong);
        } else if is_stack_mapping(flags) {
            (*mm).stack_vm = (*mm).stack_vm.wrapping_add(npages as c_ulong);
        } else if is_data_mapping(flags) {
            (*mm).data_vm = (*mm).data_vm.wrapping_add(npages as c_ulong);
        }
    }
}

#[inline]
pub unsafe fn vm_unacct_memory(pages: c_long) {
    unsafe { vm_acct_memory(-pages) };
}

#[inline]
pub unsafe fn mapping_allow_writable(mapping: *mut address_space) {
    unsafe { atomic_inc(&mut (*mapping).i_mmap_writable) };
}

#[inline]
pub unsafe fn vma_find(vmi: *mut vma_iterator, max: c_ulong) -> *mut vm_area_struct {
    unsafe { mas_find(&mut (*vmi).mas, max.wrapping_sub(1)) as *mut vm_area_struct }
}

#[inline]
pub unsafe fn vma_iter_clear_gfp(vmi: *mut vma_iterator, start: c_ulong, end: c_ulong, gfp: gfp_t) -> c_int {
    unsafe {
        __mas_set_range(&mut (*vmi).mas, start, end.wrapping_sub(1));
        mas_store_gfp(&mut (*vmi).mas, core::ptr::null_mut(), gfp);
        if unlikely(mas_is_err(&mut (*vmi).mas)) {
            return -ENOMEM;
        }
    }
    0
}

#[inline]
pub unsafe fn vma_set_anonymous(vma: *mut vm_area_struct) {
    unsafe { (*vma).vm_ops = core::ptr::null() };
}

/* Declared in vma.h. */
pub unsafe fn compat_set_vma_from_desc(vma: *mut vm_area_struct, desc: *mut vm_area_desc);

#[inline]
pub unsafe fn compat_set_desc_from_vma(desc: *mut vm_area_desc, file: *const file, vma: *const vm_area_struct) {
    unsafe {
        memset(desc as *mut c_void, 0, core::mem::size_of_val(&*desc));
        (*desc).mm = (*vma).vm_mm;
        (*desc).file = file as *mut file;
        (*desc).start = (*vma).vm_start;
        (*desc).end = (*vma).vm_end;
        (*desc).pgoff = (*vma).vm_pgoff;
        (*desc).vm_file = (*vma).vm_file;
        (*desc).vma_flags = (*vma).flags;
        (*desc).page_prot = (*vma).vm_page_prot;
        (*desc).vm_ops = (*vma).vm_ops;
        /* Default. */
        (*desc).action.type_ = mmap_action_type::MMAP_NOTHING;
    }
}

#[inline]
pub unsafe fn vma_pages(vma: *const vm_area_struct) -> c_ulong {
    unsafe { ((*vma).vm_end.wrapping_sub((*vma).vm_start)) >> PAGE_SHIFT }
}

#[inline]
pub unsafe fn vma_start_pgoff(vma: *const vm_area_struct) -> pgoff_t {
    unsafe { (*vma).vm_pgoff }
}

#[inline]
pub unsafe fn vma_end_pgoff(vma: *const vm_area_struct) -> pgoff_t {
    unsafe { vma_start_pgoff(vma).wrapping_add(vma_pages(vma) as pgoff_t) }
}

#[inline]
pub unsafe fn vma_start_anon_pgoff(vma: *const vm_area_struct) -> pgoff_t {
    let mut pgoff: pgoff_t = 0;
    /* CONFIG_64BIT adds __vm_anon_pgoff_hi shifted by 32. */
    unsafe {
        pgoff = pgoff.wrapping_add((*vma).__vm_anon_pgoff_lo as pgoff_t);
    }
    pgoff
}

#[inline]
pub unsafe fn vma_end_anon_pgoff(vma: *const vm_area_struct) -> pgoff_t {
    unsafe { vma_start_anon_pgoff(vma).wrapping_add(vma_pages(vma) as pgoff_t) }
}

#[inline]
pub unsafe fn vma_last_anon_pgoff(vma: *const vm_area_struct) -> pgoff_t {
    unsafe { vma_end_anon_pgoff(vma).wrapping_sub(1) }
}

#[inline]
pub unsafe fn vfs_mmap_prepare(file: *mut file, desc: *mut vm_area_desc) -> c_int {
    unsafe { ((*(*file).f_op).mmap_prepare.unwrap())(desc) }
}

#[inline]
pub unsafe fn __compat_vma_mmap(desc: *mut vm_area_desc, vma: *mut vm_area_struct) -> c_int {
    let err: c_int;
    unsafe {
        /* Perform any preparatory tasks for mmap action. */
        err = mmap_action_prepare(desc);
        if err != 0 {
            return err;
        }
        /* Update the VMA from the descriptor. */
        compat_set_vma_from_desc(vma, desc);
        /* Complete any specified mmap actions. */
        mmap_action_complete(vma, &mut (*desc).action, true)
    }
}

#[inline]
pub unsafe fn compat_vma_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let mut desc: vm_area_desc = unsafe { core::mem::zeroed() };
    let err: c_int;
    unsafe {
        compat_set_desc_from_vma(&mut desc, file, vma);
        err = vfs_mmap_prepare(file, &mut desc);
        if err != 0 {
            return err;
        }
        /* being invoked from .mmmap means we don't have to enforce this. */
        desc.action.hide_from_rmap_until_complete = false;
        __compat_vma_mmap(&mut desc, vma)
    }
}

#[inline]
pub unsafe fn vma_iter_init(vmi: *mut vma_iterator, mm: *mut mm_struct, addr: c_ulong) {
    unsafe { mas_init(&mut (*vmi).mas, &mut (*mm).mm_mt, addr) };
}

pub unsafe fn mmap_assert_locked(mm: *mut mm_struct);

#[inline]
pub unsafe fn find_vma_intersection(mm: *mut mm_struct, start_addr: c_ulong, end_addr: c_ulong) -> *mut vm_area_struct {
    let mut index = start_addr;
    unsafe {
        mmap_assert_locked(mm);
        mt_find(&mut (*mm).mm_mt, &mut index, end_addr.wrapping_sub(1)) as *mut vm_area_struct
    }
}

#[inline]
pub unsafe fn vma_lookup(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct {
    unsafe { mtree_load(&mut (*mm).mm_mt, addr) as *mut vm_area_struct }
}

#[inline]
pub unsafe fn vma_prev(vmi: *mut vma_iterator) -> *mut vm_area_struct {
    unsafe { mas_prev(&mut (*vmi).mas, 0) as *mut vm_area_struct }
}

#[inline]
pub unsafe fn vma_iter_set(vmi: *mut vma_iterator, addr: c_ulong) {
    unsafe { mas_set(&mut (*vmi).mas, addr) };
}

#[inline]
pub unsafe fn vma_is_anonymous(vma: *const vm_area_struct) -> bool {
    unsafe { (*vma).vm_ops.is_null() }
}

#[inline]
pub unsafe fn find_vma_prev(mm: *mut mm_struct, addr: c_ulong, pprev: *mut *mut vm_area_struct) -> *mut vm_area_struct {
    let mut vmi = vma_iterator {
        mas: ma_state {
            tree: unsafe { &mut (*mm).mm_mt },
            index: addr,
            node: core::ptr::null_mut(),
            status: ma_state_status::ma_start,
        },
    };
    unsafe {
        let mut vma = mas_walk(&mut vmi.mas) as *mut vm_area_struct;
        *pprev = vma_prev(&mut vmi);
        if vma.is_null() {
            vma = vma_next(&mut vmi);
        }
        vma
    }
}

#[inline]
pub unsafe fn vma_iter_free(vmi: *mut vma_iterator) {
    unsafe { mas_destroy(&mut (*vmi).mas) };
}

#[inline]
pub unsafe fn vma_iter_next_range(vmi: *mut vma_iterator) -> *mut vm_area_struct {
    unsafe { mas_next_range(&mut (*vmi).mas, ULONG_MAX) as *mut vm_area_struct }
}

unsafe extern "C" {
    pub fn vma_wants_writenotify(vma: *mut vm_area_struct, vm_page_prot: pgprot_t) -> bool;
}

/* Update vma->vm_page_prot to reflect vma->vm_flags. */
#[inline]
pub unsafe fn vma_set_page_prot(vma: *mut vm_area_struct) {
    unsafe {
        let mut vm_flags = (*vma).vm_flags;
        let mut vm_page_prot = pgprot_modify((*vma).vm_page_prot, vm_get_page_prot(vm_flags));
        if vma_wants_writenotify(vma, vm_page_prot) {
            vm_flags &= !VM_SHARED;
            vm_page_prot = pgprot_modify(vm_page_prot, vm_get_page_prot(vm_flags));
        }
        WRITE_ONCE(&mut (*vma).vm_page_prot, vm_page_prot);
    }
}

#[inline]
pub unsafe fn stack_guard_start_gap(vma: *mut vm_area_struct) -> c_ulong {
    unsafe {
        if ((*vma).vm_flags & VM_GROWSDOWN) != 0 {
            return stack_guard_gap;
        }
        /* See reasoning around the VM_SHADOW_STACK definition */
        if ((*vma).vm_flags & VM_SHADOW_STACK) != 0 {
            return PAGE_SIZE;
        }
        0
    }
}

#[inline]
pub unsafe fn vm_start_gap(vma: *mut vm_area_struct) -> c_ulong {
    unsafe {
        let gap = stack_guard_start_gap(vma);
        let mut vm_start = (*vma).vm_start;
        vm_start = vm_start.wrapping_sub(gap);
        if vm_start > (*vma).vm_start {
            vm_start = 0;
        }
        vm_start
    }
}

#[inline]
pub unsafe fn vm_end_gap(vma: *mut vm_area_struct) -> c_ulong {
    unsafe {
        let mut vm_end = (*vma).vm_end;
        if ((*vma).vm_flags & VM_GROWSUP) != 0 {
            vm_end = vm_end.wrapping_add(stack_guard_gap);
            if vm_end < (*vma).vm_end {
                vm_end = (-(PAGE_SIZE as c_long)) as c_ulong;
            }
        }
        vm_end
    }
}

#[inline]
pub unsafe fn vma_is_accessible(vma: *mut vm_area_struct) -> bool {
    unsafe { ((*vma).vm_flags & VM_ACCESS_FLAGS) != 0 }
}

#[inline]
pub unsafe fn mlock_future_ok(mm: *const mm_struct, vm_flags: vm_flags_t, bytes: c_ulong) -> bool {
    unsafe {
        let mut locked_pages: c_ulong;
        let mut limit_pages: c_ulong;
        if (vm_flags & VM_LOCKED) == 0 || capable(CAP_IPC_LOCK) {
            return true;
        }
        locked_pages = bytes >> PAGE_SHIFT;
        locked_pages = locked_pages.wrapping_add((*mm).locked_vm);
        limit_pages = rlimit(RLIMIT_MEMLOCK);
        limit_pages >>= PAGE_SHIFT;
        locked_pages <= limit_pages
    }
}

#[inline]
pub unsafe fn mapping_map_writable(mapping: *mut address_space) -> c_int {
    unsafe {
        if atomic_inc_unless_negative(&mut (*mapping).i_mmap_writable) {
            0
        } else {
            -EPERM
        }
    }
}

/* Did the driver provide valid mmap hook configuration? */
#[inline]
pub unsafe fn can_mmap_file(file: *mut file) -> bool {
    unsafe {
        let has_mmap = (*(*file).f_op).mmap.is_some();
        let has_mmap_prepare = (*(*file).f_op).mmap_prepare.is_some();
        /* Hooks are mutually exclusive. */
        if WARN_ON_ONCE(has_mmap && has_mmap_prepare) {
            return false;
        }
        if !has_mmap && !has_mmap_prepare {
            return false;
        }
        true
    }
}

#[inline]
pub unsafe fn vfs_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    unsafe {
        if (*(*file).f_op).mmap_prepare.is_some() {
            return compat_vma_mmap(file, vma);
        }
        ((*(*file).f_op).mmap.unwrap())(file, vma)
    }
}

#[inline]
pub unsafe fn vma_set_file(vma: *mut vm_area_struct, mut file: *mut file) {
    unsafe {
        /* Changing an anonymous vma with this is illegal */
        get_file(file);
        core::mem::swap(&mut (*vma).vm_file, &mut file);
        fput(file);
    }
}

unsafe extern "C" {
    pub static mut sysctl_max_map_count: c_int;
}

#[inline]
pub unsafe fn get_sysctl_max_map_count() -> c_int {
    unsafe { READ_ONCE(&sysctl_max_map_count) }
}

#[inline]
pub fn pgtable_supports_soft_dirty() -> bool {
    IS_ENABLED_CONFIG_MEM_SOFT_DIRTY
}

#[inline]
pub fn vma_flags_to_page_prot(vma_flags: vma_flags_t) -> pgprot_t {
    let vm_flags = vma_flags_to_legacy(vma_flags);
    vm_get_page_prot(vm_flags)
}

#[inline]
pub unsafe fn linear_page_delta(vma: *const vm_area_struct, address: c_ulong) -> pgoff_t {
    unsafe { ((address.wrapping_sub((*vma).vm_start)) >> PAGE_SHIFT) as pgoff_t }
}

#[inline]
pub unsafe fn linear_page_index(vma: *const vm_area_struct, address: c_ulong) -> pgoff_t {
    unsafe {
        let mut pgoff = linear_page_delta(vma, address);
        pgoff = pgoff.wrapping_add(vma_start_pgoff(vma));
        pgoff
    }
}

#[inline]
pub unsafe fn vma_assert_can_modify(vma: *mut vm_area_struct) {
    unsafe {
        if vma_is_attached(vma) {
            vma_assert_write_locked(vma);
        }
    }
}

#[inline]
pub unsafe fn vma_get_page_prot(vma: *const vm_area_struct) -> pgprot_t {
    unsafe { vma_flags_to_page_prot((*vma).flags) }
}

#[inline]
pub unsafe fn __linear_anon_page_index(vma: *const vm_area_struct, address: c_ulong) -> pgoff_t {
    unsafe {
        let mut pgoff = linear_page_delta(vma, address);
        pgoff = pgoff.wrapping_add(vma_start_anon_pgoff(vma));
        pgoff
    }
}

#[inline]
pub unsafe fn linear_anon_page_index(vma: *const vm_area_struct, address: c_ulong) -> pgoff_t {
    unsafe {
        let pgoff = __linear_anon_page_index(vma, address);
        VM_WARN_ON_ONCE(!vma_is_cow_mapping(vma));
        /* Account for MAP_PRIVATE-/dev/zero which is only semi-anonymous. */
        if vma_is_anonymous(vma) && (*vma).vm_file.is_null() {
            VM_WARN_ON_ONCE(pgoff != linear_page_index(vma, address));
        }
        pgoff
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
