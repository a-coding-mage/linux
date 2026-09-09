/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust source-level translation of arm64/include/asm/kvm_host.h. */

// C header dependencies are intentionally left to the surrounding kernel translation.

pub const KVM_HALT_POLL_NS_DEFAULT: u64 = 500000;
pub const KVM_MAX_VCPUS: usize = VGIC_V3_MAX_CPUS;
pub const KVM_VCPU_MAX_FEATURES: usize = 10;
pub const KVM_VCPU_VALID_FEATURES: usize = (1usize << KVM_VCPU_MAX_FEATURES) - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KvmMode { KVM_MODE_DEFAULT, KVM_MODE_PROTECTED, KVM_MODE_NV, KVM_MODE_NONE }
#[cfg(feature = "CONFIG_KVM")]
extern "C" { pub fn kvm_get_mode() -> KvmMode; }
#[cfg(not(feature = "CONFIG_KVM"))]
#[inline] pub fn kvm_get_mode() -> KvmMode { KvmMode::KVM_MODE_NONE }

extern "C" {
    pub static mut kvm_sve_max_vl: u32;
    pub static mut kvm_host_sve_max_vl: u32;
    pub fn kvm_arm_init_sve() -> i32;
    pub fn kvm_target_cpu() -> u32;
    pub fn kvm_reset_vcpu(vcpu: *mut kvm_vcpu);
    pub fn kvm_arm_vcpu_destroy(vcpu: *mut kvm_vcpu);
}

#[repr(C)] pub struct kvm_hyp_memcache { pub head: phys_addr_t, pub nr_pages: c_ulong, pub mapping: *mut pkvm_mapping, pub flags: c_ulong }
pub const HYP_MEMCACHE_ACCOUNT_STAGE2: c_ulong = 1 << 1;

#[inline] pub unsafe fn push_hyp_memcache(mc: *mut kvm_hyp_memcache, p: *mut phys_addr_t, to_pa: unsafe extern "C" fn(*mut c_void)->phys_addr_t) { *p = (*mc).head; (*mc).head = to_pa(p as *mut c_void); (*mc).nr_pages += 1; }
#[inline] pub unsafe fn pop_hyp_memcache(mc: *mut kvm_hyp_memcache, to_va: unsafe extern "C" fn(phys_addr_t)->*mut c_void) -> *mut c_void { let p = to_va((*mc).head & PAGE_MASK); if (*mc).nr_pages == 0 { return core::ptr::null_mut(); } (*mc).head = *(p as *mut phys_addr_t); (*mc).nr_pages -= 1; p }
#[inline] pub unsafe fn __topup_hyp_memcache(mc:*mut kvm_hyp_memcache,min_pages:c_ulong,alloc_fn:unsafe extern "C" fn(*mut c_void)->*mut c_void,to_pa:unsafe extern "C" fn(*mut c_void)->phys_addr_t,arg:*mut c_void)->c_int { while (*mc).nr_pages < min_pages { let p=alloc_fn(arg); if p.is_null(){return -ENOMEM;} push_hyp_memcache(mc,p as *mut phys_addr_t,to_pa); } 0 }
#[inline] pub unsafe fn __free_hyp_memcache(mc:*mut kvm_hyp_memcache,free_fn:unsafe extern "C" fn(*mut c_void,*mut c_void),to_va:unsafe extern "C" fn(phys_addr_t)->*mut c_void,arg:*mut c_void){while (*mc).nr_pages!=0{free_fn(pop_hyp_memcache(mc,to_va),arg);}}

#[repr(C)] pub struct kvm_vmid { pub id: atomic64_t }
#[repr(C)] pub struct kvm_s2_mmu { pub vmid:kvm_vmid, pub pgd_phys:phys_addr_t, pub pgt:*mut kvm_pgtable, pub vtcr:u64, pub last_vcpu_ran:*mut c_int, pub split_page_cache:kvm_mmu_memory_cache, pub split_page_chunk_size:u64, pub arch:*mut kvm_arch, pub tlb_vttbr:u64, pub tlb_vtcr:u64, pub nested_stage2_enabled:bool, #[cfg(feature="CONFIG_PTDUMP_STAGE2_DEBUGFS")] pub shadow_pt_debugfs_dentry:*mut dentry, pub pending_unmap:bool, pub refcnt:atomic_t }
#[repr(C)] pub struct kvm_arch_memory_slot;
#[repr(C)] pub struct kvm_smccc_features { pub std_bmap:c_ulong, pub std_hyp_bmap:c_ulong, pub vendor_hyp_bmap:c_ulong, pub vendor_hyp_bmap_2:c_ulong }
pub type pkvm_handle_t=u16;
#[repr(C)] pub struct kvm_protected_vm { pub handle:pkvm_handle_t, pub teardown_mc:kvm_hyp_memcache, pub stage2_teardown_mc:kvm_hyp_memcache, pub is_protected:bool, pub is_created:bool, pub is_dying:bool }
#[repr(C)] pub struct kvm_mpidr_data { pub mpidr_mask:u64, pub cmpidr_to_idx:[u16;0] }
#[inline] pub unsafe fn kvm_mpidr_index(data:*mut kvm_mpidr_data,mpidr:u64)->c_ulong { let mut index=0; let mut aff=mpidr & MPIDR_HWID_BITMASK; bitmap_gather(&mut index,&mut aff,&(*data).mpidr_mask,fls((*data).mpidr_mask)); index }

#[repr(C)] pub enum fgt_group_id { __NO_FGT_GROUP__, HFGRTR_GROUP, HDFGRTR_GROUP, HFGITR_GROUP, HAFGRTR_GROUP, HFGRTR2_GROUP, HDFGRTR2_GROUP, HFGITR2_GROUP, ICH_HFGRTR_GROUP, ICH_HFGITR_GROUP, __NR_FGT_GROUP_IDS__ }
#[repr(C)] pub struct kvm_arch { pub mmu:kvm_s2_mmu, pub fgu:[u64;__NR_FGT_GROUP_IDS__ as usize], pub nested_mmus:*mut kvm_s2_mmu, pub nested_mmus_size:usize, pub nested_mmus_next:c_int, pub vgic:vgic_dist, pub timer_data:arch_timer_vm_data, pub psci_version:u32, pub config_lock:mutex, pub flags:c_ulong, pub vcpu_features:[c_ulong;0], pub mpidr_data:*mut kvm_mpidr_data, pub pmu_filter:*mut c_ulong, pub arm_pmu:*mut arm_pmu, pub supported_cpus:cpumask_var_t, pub nr_pmu_counters:u8, pub pmmir_slots:u8, pub smccc_feat:kvm_smccc_features, pub smccc_filter:maple_tree, pub id_regs:[u64;KVM_ARM_ID_REG_NUM], pub midr_el1:u64, pub revidr_el1:u64, pub aidr_el1:u64, pub ctr_el0:u64, pub sysreg_masks:*mut kvm_sysreg_masks, pub vncr_tlb_count:atomic_t, pub pkvm:kvm_protected_vm }
pub const KVM_ARCH_FLAG_RETURN_NISV_IO_ABORT_TO_USER:u32=0; pub const KVM_ARCH_FLAG_MTE_ENABLED:u32=1; pub const KVM_ARCH_FLAG_HAS_RAN_ONCE:u32=2; pub const KVM_ARCH_FLAG_VCPU_FEATURES_CONFIGURED:u32=3; pub const KVM_ARCH_FLAG_SYSTEM_SUSPEND_ENABLED:u32=4; pub const KVM_ARCH_FLAG_VM_COUNTER_OFFSET:u32=5; pub const KVM_ARCH_FLAG_TIMER_PPIS_IMMUTABLE:u32=6; pub const KVM_ARCH_FLAG_ID_REGS_INITIALIZED:u32=7; pub const KVM_ARCH_FLAG_FGU_INITIALIZED:u32=8; pub const KVM_ARCH_FLAG_GUEST_HAS_SVE:u32=9; pub const KVM_ARCH_FLAG_WRITABLE_IMP_ID_REGS:u32=10; pub const KVM_ARCH_FLAG_EXIT_SEA:u32=11;

#[repr(C)] pub struct kvm_vcpu_fault_info { pub esr_el2:u64,pub far_el2:u64,pub hpfar_el2:u64,pub disr_el1:u64 }
#[repr(C)] pub struct resx { pub res0:u64,pub res1:u64 }
#[repr(C)] pub struct kvm_sysreg_masks { pub mask:[resx;0] }
#[repr(C)] pub struct fgt_masks { pub str:*const c_char,pub mask:u64,pub nmask:u64,pub res0:u64,pub res1:u64 }
#[repr(C)] pub struct kvm_cpu_context { pub regs:user_pt_regs,pub spsr_abt:u64,pub spsr_und:u64,pub spsr_irq:u64,pub spsr_fiq:u64,pub fp_regs:user_fpsimd_state,pub sys_regs:[u64;NR_SYS_REGS],pub hyp_running_vcpu:*mut kvm_vcpu,pub vncr_array:*mut u64 }
#[repr(C)] pub struct vcpu_reset_state { pub pc:c_ulong,pub r0:c_ulong,pub be:bool,pub reset:bool }
#[repr(C)] pub struct kvm_vcpu_arch { pub ctxt:kvm_cpu_context,pub sve_state:*mut arm64_sve_state,pub fp_type:fp_type,pub sve_max_vl:c_uint,pub hw_mmu:*mut kvm_s2_mmu,pub hcr_el2:u64,pub hcrx_el2:u64,pub mdcr_el2:u64,pub fgt:[[u64;2];__NR_FGT_GROUP_IDS__ as usize],pub fault:kvm_vcpu_fault_info,pub cflags:u8,pub iflags:u8,pub sflags:u16,pub pause:bool,pub vcpu_debug_state:kvm_guest_debug_arch,pub external_debug_state:kvm_guest_debug_arch,pub external_mdscr_el1:u64,pub debug_owner:c_int,pub vgic_cpu:vgic_cpu,pub timer_cpu:arch_timer_cpu,pub pmu:kvm_pmu,pub mp_state:kvm_mp_state,pub mp_state_lock:spinlock_t,pub mmu_page_cache:kvm_mmu_memory_cache,pub pkvm_memcache:kvm_hyp_memcache,pub vsesr_el2:u64,pub reset_state:vcpu_reset_state,pub last_steal:u64,pub steal_base:gpa_t,pub ccsidr:*mut u32,pub vncr_tlb:*mut vncr_tlb,pub pid:pid_t }

extern "C" { pub fn free_hyp_memcache(*mut kvm_hyp_memcache); pub fn topup_hyp_memcache(*mut kvm_hyp_memcache,c_ulong)->c_int; pub fn vcpu_read_sys_reg(*const kvm_vcpu,vcpu_sysreg)->u64; pub fn vcpu_write_sys_reg(*mut kvm_vcpu,u64,vcpu_sysreg); pub fn kvm_vcpu_apply_reg_masks(*const kvm_vcpu,vcpu_sysreg,u64)->u64; pub fn kvm_arch_alloc_vm()->*mut kvm; pub fn kvm_arm_vcpu_finalize(*mut kvm_vcpu,c_int)->c_int; }
#[inline] pub unsafe fn kvm_arch_pmi_in_guest(vcpu:*mut kvm_vcpu)->bool { cfg!(feature="CONFIG_GUEST_PERF_EVENTS") && !vcpu.is_null() }
#[inline] pub unsafe fn kvm_arm_pvtime_vcpu_init(v:*mut kvm_vcpu_arch){(*v).steal_base=INVALID_GPA;}
#[inline] pub unsafe fn kvm_arm_is_pvtime_enabled(v:*mut kvm_vcpu_arch)->bool{(*v).steal_base!=INVALID_GPA}

// The remaining kernel macros are preserved as Rust macro interfaces; their referenced
// constants and helper symbols are supplied by the translated dependency headers.
macro_rules! __vcpu_single_flag { ($set:ident,$f:expr)=>{($set,$f,$f)} }
macro_rules! vcpu_get_flag { ($v:expr,$set:ident,$f:expr,$m:expr)=>{unsafe{(*$v).arch.$set & $m}} }
macro_rules! vcpu_set_flag { ($v:expr,$set:ident,$f:expr,$m:expr)=>{unsafe{(*$v).arch.$set |= $f}} }
macro_rules! vcpu_clear_flag { ($v:expr,$set:ident,$f:expr,$m:expr)=>{unsafe{(*$v).arch.$set &= !$m}} }
pub const VCPU_INITIALIZED:(u8,u8,u8)=(0,1,1); pub const VCPU_SVE_FINALIZED:(u8,u8,u8)=(0,2,2); pub const VCPU_PKVM_FINALIZED:(u8,u8,u8)=(0,4,4);
pub const PENDING_EXCEPTION:(u8,u8,u8)=(1,1,1); pub const INCREMENT_PC:(u8,u8,u8)=(1,2,2); pub const EXCEPT_MASK:(u8,u8,u8)=(1,14,14); pub const PKVM_HOST_STATE_DIRTY:(u8,u8,u8)=(1,16,16);
pub const ON_UNSUPPORTED_CPU:(u8,u16,u16)=(2,1,1); pub const IN_WFIT:(u8,u16,u16)=(2,2,2); pub const SYSREGS_ON_CPU:(u8,u16,u16)=(2,4,4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
