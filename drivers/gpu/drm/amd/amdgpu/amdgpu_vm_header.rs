/* Translated from amdgpu_vm.h. External kernel types and functions are supplied by dependencies. */

pub const AMDGPU_VM_MAX_UPDATE_SIZE: u64 = 0x3ffff;
pub const AMDGPU_PTE_VALID: u64 = 1u64 << 0;
pub const AMDGPU_PTE_SYSTEM: u64 = 1u64 << 1;
pub const AMDGPU_PTE_SNOOPED: u64 = 1u64 << 2;
pub const AMDGPU_PTE_TMZ: u64 = 1u64 << 3;
pub const AMDGPU_PTE_EXECUTABLE: u64 = 1u64 << 4;
pub const AMDGPU_PTE_READABLE: u64 = 1u64 << 5;
pub const AMDGPU_PTE_WRITEABLE: u64 = 1u64 << 6;
pub const AMDGPU_PTE_PRT: u64 = 1u64 << 51;
pub const AMDGPU_PDE_PTE: u64 = 1u64 << 54;
pub const AMDGPU_PTE_LOG: u64 = 1u64 << 55;
pub const AMDGPU_PTE_TF: u64 = 1u64 << 56;
pub const AMDGPU_PTE_NOALLOC: u64 = 1u64 << 58;
pub const AMDGPU_VM_NORETRY_FLAGS: u64 = AMDGPU_PTE_EXECUTABLE | AMDGPU_PDE_PTE | AMDGPU_PTE_TF;
pub const AMDGPU_VM_NORETRY_FLAGS_TF: u64 = AMDGPU_PTE_VALID | AMDGPU_PTE_SYSTEM | AMDGPU_PTE_PRT;
pub const AMDGPU_MTYPE_NC: u64 = 0;
pub const AMDGPU_MTYPE_CC: u64 = 2;
pub const AMDGPU_PTE_PRT_GFX12: u64 = 1u64 << 56;
pub const AMDGPU_PTE_DCC: u64 = 1u64 << 58;
pub const AMDGPU_PTE_BUS_ATOMICS: u64 = 1u64 << 59;
pub const AMDGPU_PTE_IS_PTE: u64 = 1u64 << 63;
pub const AMDGPU_PDE_PTE_GFX12: u64 = 1u64 << 63;
pub const AMDGPU_VM_FAULT_STOP_NEVER: i32 = 0;
pub const AMDGPU_VM_FAULT_STOP_FIRST: i32 = 1;
pub const AMDGPU_VM_FAULT_STOP_ALWAYS: i32 = 2;
pub const AMDGPU_VM_RESERVED_VRAM: u64 = 8u64 << 20;
pub const AMDGPU_MAX_VMHUBS: usize = 13;
pub const AMDGPU_GFXHUB_START: usize = 0;
pub const AMDGPU_MMHUB0_START: usize = 8;
pub const AMDGPU_MMHUB1_START: usize = 12;
pub const AMDGPU_VA_RESERVED_CSA_SIZE: u64 = 2u64 << 20;
pub const AMDGPU_VA_RESERVED_SEQ64_SIZE: u64 = 2u64 << 20;
pub const AMDGPU_VA_RESERVED_TRAP_SIZE: u64 = 1u64 << 16;
pub const AMDGPU_VA_RESERVED_BOTTOM: u64 = 1u64 << 16;
pub const AMDGPU_VA_RESERVED_TOP: u64 = AMDGPU_VA_RESERVED_TRAP_SIZE + AMDGPU_VA_RESERVED_SEQ64_SIZE + AMDGPU_VA_RESERVED_CSA_SIZE;
pub const AMDGPU_VM_USE_CPU_FOR_GFX: i32 = 1 << 0;
pub const AMDGPU_VM_USE_CPU_FOR_COMPUTE: i32 = 1 << 1;

#[inline] pub const fn AMDGPU_PTE_FRAG(x: u64) -> u64 { (x & 0x1f) << 7 }
#[inline] pub const fn AMDGPU_PDE_BFS(a: u64) -> u64 { a << 59 }
#[inline] pub const fn AMDGPU_PTE_MTYPE_VG10_SHIFT(mtype: u64) -> u64 { mtype << 57 }
pub const AMDGPU_PTE_MTYPE_VG10_MASK: u64 = AMDGPU_PTE_MTYPE_VG10_SHIFT(3);
#[inline] pub const fn AMDGPU_PTE_MTYPE_VG10(flags: u64, mtype: u64) -> u64 { (flags & !AMDGPU_PTE_MTYPE_VG10_MASK) | AMDGPU_PTE_MTYPE_VG10_SHIFT(mtype) }
pub const AMDGPU_PTE_DEFAULT_ATC: u64 = AMDGPU_PTE_SYSTEM | AMDGPU_PTE_SNOOPED | AMDGPU_PTE_EXECUTABLE | AMDGPU_PTE_READABLE | AMDGPU_PTE_WRITEABLE | AMDGPU_PTE_MTYPE_VG10(AMDGPU_MTYPE_CC, AMDGPU_MTYPE_CC);
#[inline] pub const fn AMDGPU_PTE_MTYPE_NV10_SHIFT(mtype: u64) -> u64 { mtype << 48 }
pub const AMDGPU_PTE_MTYPE_NV10_MASK: u64 = AMDGPU_PTE_MTYPE_NV10_SHIFT(7);
#[inline] pub const fn AMDGPU_PTE_MTYPE_NV10(flags: u64, mtype: u64) -> u64 { (flags & !AMDGPU_PTE_MTYPE_NV10_MASK) | AMDGPU_PTE_MTYPE_NV10_SHIFT(mtype) }
#[inline] pub const fn AMDGPU_PTE_MTYPE_GFX12_SHIFT(mtype: u64) -> u64 { mtype << 54 }
pub const AMDGPU_PTE_MTYPE_GFX12_MASK: u64 = AMDGPU_PTE_MTYPE_GFX12_SHIFT(3);
#[inline] pub const fn AMDGPU_PTE_MTYPE_GFX12(flags: u64, mtype: u64) -> u64 { (flags & !AMDGPU_PTE_MTYPE_GFX12_MASK) | AMDGPU_PTE_MTYPE_GFX12_SHIFT(mtype) }
#[inline] pub const fn AMDGPU_PDE_BFS_GFX12(a: u64) -> u64 { (a & 0x1f) << 58 }

pub const fn AMDGPU_GFXHUB(x: usize) -> usize { AMDGPU_GFXHUB_START + x }
pub const fn AMDGPU_MMHUB0(x: usize) -> usize { AMDGPU_MMHUB0_START + x }
pub const fn AMDGPU_MMHUB1(x: usize) -> usize { AMDGPU_MMHUB1_START + x }
pub const fn AMDGPU_IS_GFXHUB(x: usize) -> bool { x >= AMDGPU_GFXHUB_START && x < AMDGPU_MMHUB0_START }
pub const fn AMDGPU_IS_MMHUB0(x: usize) -> bool { x >= AMDGPU_MMHUB0_START && x < AMDGPU_MMHUB1_START }
pub const fn AMDGPU_IS_MMHUB1(x: usize) -> bool { x >= AMDGPU_MMHUB1_START && x < AMDGPU_MAX_VMHUBS }

#[repr(C)] pub enum amdgpu_vm_level { AMDGPU_VM_PDB3, AMDGPU_VM_PDB2, AMDGPU_VM_PDB1, AMDGPU_VM_PDB0, AMDGPU_VM_PTB }

#[repr(C)] pub struct amdgpu_vm_bo_base { pub vm: *mut amdgpu_vm, pub bo: *mut amdgpu_bo, pub next: *mut amdgpu_vm_bo_base, pub vm_status: list_head, pub shared: bool, pub moved: bool }
#[repr(C)] pub struct amdgpu_vm_bo_status { pub evicted: list_head, pub needs_update: list_head, pub idle: list_head }
#[repr(C)] pub struct amdgpu_vm_pte_funcs {
    pub copy_pte_num_dw: c_uint,
    pub copy_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib, u64, u64, c_uint)>,
    pub write_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib, u64, u64, c_uint, u32)>,
    pub set_pte_pde: Option<unsafe extern "C" fn(*mut amdgpu_ib, u64, u64, c_uint, u32, u64)>,
}
#[repr(C)] pub struct amdgpu_task_info { pub task: drm_wedge_task_info, pub process_name: [c_char; TASK_COMM_LEN], pub tgid: pid_t, pub refcount: kref }
#[repr(C)] pub struct amdgpu_vm_update_params { pub adev: *mut amdgpu_device, pub vm: *mut amdgpu_vm, pub immediate: bool, pub unlocked: bool, pub pages_addr: *mut dma_addr_t, pub job: *mut amdgpu_job, pub num_dw_left: c_uint, pub needs_flush: bool, pub override_pte: bool, pub tlb_flush_waitlist: list_head }
#[repr(C)] pub struct amdgpu_vm_update_funcs { pub map_table: Option<unsafe extern "C" fn(*mut amdgpu_bo_vm) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut amdgpu_vm_update_params, *mut amdgpu_sync, u64) -> c_int>, pub update: Option<unsafe extern "C" fn(*mut amdgpu_vm_update_params, *mut amdgpu_bo_vm, u64, u64, c_uint, u32, u64) -> c_int>, pub commit: Option<unsafe extern "C" fn(*mut amdgpu_vm_update_params, *mut *mut dma_fence) -> c_int> }
#[repr(C)] pub struct amdgpu_vm_fault_info { pub addr: u64, pub status: u32, pub vmhub: c_uint }
#[repr(C)] pub struct amdgpu_mem_stats { pub drm: drm_memory_stats, pub evicted: u64 }

#[repr(C)] pub struct amdgpu_vm { pub va: rb_root_cached, pub eviction_lock: mutex, pub evicting: bool, pub saved_flags: c_uint, pub stats_lock: spinlock_t, pub stats: [amdgpu_mem_stats; __AMDGPU_PL_NUM], pub kernel: amdgpu_vm_bo_status, pub always_valid: amdgpu_vm_bo_status, pub individual_lock: spinlock_t, pub individual: amdgpu_vm_bo_status, pub freed: list_head, pub root: amdgpu_vm_bo_base, pub last_update: *mut dma_fence, pub immediate: drm_sched_entity, pub delayed: drm_sched_entity, pub tlb_seq: atomic64_t, pub last_tlb_flush: *mut dma_fence, pub kfd_last_flushed_seq: atomic64_t, pub tlb_fence_context: u64, pub generation: u64, pub last_unlocked: *mut dma_fence, pub pasid: c_uint, pub reserved_vmid: [*mut amdgpu_vmid; AMDGPU_MAX_VMHUBS], pub use_cpu_for_update: bool, pub update_funcs: *const amdgpu_vm_update_funcs, pub faults: kfifo, pub process_info: *mut amdkfd_process_info, pub vm_list_node: list_head, pub pd_phys_addr: u64, pub task_info: *mut amdgpu_task_info, pub lru_bulk_move: ttm_lru_bulk_move, pub is_compute_context: bool, pub need_tlb_fence: bool, pub mem_id: i8, pub fault_info: amdgpu_vm_fault_info }
#[repr(C)] pub struct amdgpu_vm_manager { pub id_mgr: [amdgpu_vmid_mgr; AMDGPU_MAX_VMHUBS], pub first_kfd_vmid: c_uint, pub concurrent_flush: bool, pub max_pfn: u64, pub max_level: u32, pub num_level: u32, pub block_size: u32, pub fragment_size: u32, pub root_level: amdgpu_vm_level, pub vram_base_offset: u64, pub vm_pte_funcs: *const amdgpu_vm_pte_funcs, pub vm_pte_scheds: [*mut drm_gpu_scheduler; AMDGPU_MAX_RINGS], pub vm_pte_num_scheds: c_uint, pub page_fault: *mut amdgpu_ring, pub prt_lock: spinlock_t, pub num_prt_users: atomic_t, pub vm_update_mode: c_int, pub fault_info: amdgpu_vm_fault_info }

extern "C" { pub static amdgpu_vm_cpu_funcs: amdgpu_vm_update_funcs; pub static amdgpu_vm_sdma_funcs: amdgpu_vm_update_funcs; }

extern "C" {
    pub fn amdgpu_vm_manager_init(adev: *mut amdgpu_device); pub fn amdgpu_vm_manager_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_vm_wait_idle(vm: *mut amdgpu_vm, timeout: c_long) -> c_long; pub fn amdgpu_vm_init(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, xcp_id: i32) -> c_int; pub fn amdgpu_vm_make_compute(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> c_int; pub fn amdgpu_vm_fini(adev: *mut amdgpu_device, vm: *mut amdgpu_vm);
    pub fn amdgpu_vm_lock_pd(vm: *mut amdgpu_vm, exec: *mut drm_exec, num_fences: c_uint) -> c_int; pub fn amdgpu_vm_lock_individual(vm: *mut amdgpu_vm, exec: *mut drm_exec, num_fences: c_uint) -> c_int; pub fn amdgpu_vm_ready(vm: *mut amdgpu_vm) -> bool; pub fn amdgpu_vm_generation(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> u64;
    pub fn amdgpu_vm_bo_base_init(base: *mut amdgpu_vm_bo_base, vm: *mut amdgpu_vm, bo: *mut amdgpu_bo); pub fn amdgpu_vm_bo_update(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va, clear: bool) -> c_int; pub fn amdgpu_vm_evictable(bo: *mut amdgpu_bo) -> bool; pub fn amdgpu_vm_bo_invalidate(bo: *mut amdgpu_bo, evicted: bool); pub fn amdgpu_vm_map_gart(pages_addr: *const dma_addr_t, addr: u64) -> u64;
}

// The remaining declarations are intentionally retained as external dependencies.
extern "C" {
    pub fn amdgpu_vm_flush(ring: *mut amdgpu_ring, job: *mut amdgpu_job, need_pipe_sync: *mut bool, emit_spm_needed: *mut bool, emit_gds_needed: *mut bool);
    pub fn amdgpu_vm_update_pdes(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, immediate: bool) -> c_int;
    pub fn amdgpu_vm_clear_freed(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, fence: *mut *mut dma_fence) -> c_int;
    pub fn amdgpu_vm_handle_moved(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, ticket: *mut ww_acquire_ctx) -> c_int;
    pub fn amdgpu_vm_flush_compute_tlb(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, flush_type: u32, xcc_mask: u32) -> c_int;
    pub fn amdgpu_vm_update_range(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, immediate: bool, unlocked: bool, flush_tlb: bool, allow_override: bool, sync: *mut amdgpu_sync, start: u64, last: u64, flags: u64, offset: u64, vram_base: u64, res: *mut ttm_resource, pages_addr: *mut dma_addr_t, fence: *mut *mut dma_fence) -> c_int;
    pub fn amdgpu_vm_bo_find(vm: *mut amdgpu_vm, bo: *mut amdgpu_bo) -> *mut amdgpu_bo_va; pub fn amdgpu_vm_bo_add(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, bo: *mut amdgpu_bo) -> *mut amdgpu_bo_va;
    pub fn amdgpu_vm_bo_map(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va, addr: u64, offset: u64, size: u64, flags: u32) -> c_int; pub fn amdgpu_vm_bo_unmap(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va, addr: u64) -> c_int; pub fn amdgpu_vm_bo_del(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va);
    pub fn amdgpu_vm_adjust_size(adev: *mut amdgpu_device, min_vm_size: u32, fragment_size_default: u32, max_level: c_uint, max_bits: c_uint); pub fn amdgpu_vm_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int; pub fn amdgpu_vm_check_compute_bug(adev: *mut amdgpu_device);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
