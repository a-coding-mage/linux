/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of powerpc/include/asm/kvm_host.h.  External kernel types are
 * intentionally left as dependencies supplied by the surrounding translation. */

pub const KVM_MAX_VCPUS: usize = NR_CPUS;
pub const KVM_MAX_VCORES: usize = NR_CPUS;
#[cfg(feature = "kvm_book3s_hv_possible")]
pub const KVM_MAX_VCPU_IDS: usize = MAX_SMT_THREADS * KVM_MAX_VCORES;
#[cfg(not(feature = "kvm_book3s_hv_possible"))]
pub const KVM_MAX_VCPU_IDS: usize = KVM_MAX_VCPUS;
#[cfg(feature = "kvm_book3s_hv_possible")]
pub const KVM_MAX_NESTED_GUESTS_SHIFT: u32 = 12;
pub const KVM_HALT_POLL_NS_DEFAULT: u32 = 10000;
pub const KVM_NR_IRQCHIPS: u32 = 1;
pub const KVM_IRQCHIP_NUM_PINS: u32 = 256;
pub const HPTEG_CACHE_NUM: usize = 1 << 15;
pub const HPTEG_HASH_BITS_PTE: u32 = 13;
pub const HPTEG_HASH_BITS_PTE_LONG: u32 = 12;
pub const HPTEG_HASH_BITS_VPTE: u32 = 13;
pub const HPTEG_HASH_BITS_VPTE_LONG: u32 = 5;
pub const HPTEG_HASH_BITS_VPTE_64K: u32 = 11;
pub const HPTEG_HASH_NUM_PTE: usize = 1 << HPTEG_HASH_BITS_PTE;
pub const HPTEG_HASH_NUM_PTE_LONG: usize = 1 << HPTEG_HASH_BITS_PTE_LONG;
pub const HPTEG_HASH_NUM_VPTE: usize = 1 << HPTEG_HASH_BITS_VPTE;
pub const HPTEG_HASH_NUM_VPTE_LONG: usize = 1 << HPTEG_HASH_BITS_VPTE_LONG;
pub const HPTEG_HASH_NUM_VPTE_64K: usize = 1 << HPTEG_HASH_BITS_VPTE_64K;
pub const KVM_PAM: u64 = 0x0fff_ffff_ffff_ffff;

pub const KVMPPC_RMAP_TYPE_MASK: u64 = 0xff00_0000_0000_0000;
pub const KVMPPC_RMAP_NESTED: u64 = 0xc000_0000_0000_0000;
pub const KVMPPC_RMAP_HPT: u64 = 0x0100_0000_0000_0000;
pub const KVMPPC_RMAP_LOCK_BIT: u32 = 43;
pub const KVMPPC_RMAP_RC_SHIFT: u32 = 32;
pub const KVMPPC_RMAP_REFERENCED: u64 = (HPTE_R_R as u64) << KVMPPC_RMAP_RC_SHIFT;
pub const KVMPPC_RMAP_PRESENT: usize = 0x1000_0000_0;
pub const KVMPPC_RMAP_INDEX: usize = 0xffff_ffff;

#[repr(C)] pub struct kvm_vm_stat { pub generic: kvm_vm_stat_generic, pub num_2M_pages: u64, pub num_1G_pages: u64 }
#[repr(C)] pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic, pub sum_exits:u64, pub mmio_exits:u64, pub signal_exits:u64,
    pub light_exits:u64, pub itlb_real_miss_exits:u64, pub itlb_virt_miss_exits:u64,
    pub dtlb_real_miss_exits:u64, pub dtlb_virt_miss_exits:u64, pub syscall_exits:u64,
    pub isi_exits:u64, pub dsi_exits:u64, pub emulated_inst_exits:u64, pub dec_exits:u64,
    pub ext_intr_exits:u64, pub halt_successful_wait:u64, pub dbell_exits:u64, pub gdbell_exits:u64,
    pub ld:u64, pub st:u64, pub pthru_all:u64, pub pthru_host:u64, pub pthru_bad_aff:u64,
}
#[repr(u32)] pub enum kvm_exit_types { MMIO_EXITS, SIGNAL_EXITS, ITLB_REAL_MISS_EXITS, ITLB_VIRT_MISS_EXITS, DTLB_REAL_MISS_EXITS, DTLB_VIRT_MISS_EXITS, SYSCALL_EXITS, ISI_EXITS, DSI_EXITS, EMULATED_INST_EXITS, EMULATED_MTMSRWE_EXITS, EMULATED_WRTEE_EXITS, EMULATED_MTSPR_EXITS, EMULATED_MFSPR_EXITS, EMULATED_MTMSR_EXITS, EMULATED_MFMSR_EXITS, EMULATED_TLBSX_EXITS, EMULATED_TLBWE_EXITS, EMULATED_RFI_EXITS, EMULATED_RFCI_EXITS, EMULATED_RFDI_EXITS, DEC_EXITS, EXT_INTR_EXITS, HALT_WAKEUP, USR_PR_INST, FP_UNAVAIL, DEBUG_EXITS, TIMEINGUEST, DBELL_EXITS, GDBELL_EXITS, __NUMBER_OF_KVM_EXIT_TYPES }

#[repr(C)] pub union kvmppc_exit_timing { pub tv64:u64, pub tv32:kvmppc_exit_timing_32 }
#[repr(C)] pub struct kvmppc_exit_timing_32 { pub tbu:u32, pub tbl:u32 }
#[repr(C)] pub struct kvmppc_pginfo { pub pfn: c_ulong, pub refcnt: atomic_t }
#[repr(C)] pub struct kvmppc_spapr_tce_iommu_table { pub rcu:rcu_head, pub next:list_head, pub tbl:*mut iommu_table, pub kref:kref }
pub const TCES_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<u64>();
#[repr(C)] pub struct kvmppc_spapr_tce_table { pub list:list_head, pub kvm:*mut kvm, pub liobn:u64, pub rcu:rcu_head, pub page_shift:u32, pub offset:u64, pub size:u64, pub iommu_tables:list_head, pub alloc_lock:mutex, pub pages:[*mut page; 0] }
extern "C" { pub static mut kvm_xics_ops:kvm_device_ops; pub static mut kvm_xive_ops:kvm_device_ops; pub static mut kvm_xive_native_ops:kvm_device_ops; }
#[repr(C)] pub struct revmap_entry { pub guest_rpte:c_ulong, pub forw:u32, pub back:u32 }
#[repr(C)] pub struct kvm_arch_memory_slot { #[cfg(feature="kvm_book3s_hv_possible")] pub rmap:*mut c_ulong }
#[repr(C)] pub struct kvm_hpt_info { pub virt:c_ulong, pub rev:*mut revmap_entry, pub order:u32, pub cma:c_int }
#[repr(C)] pub struct kvm_arch { pub lpid:u64, pub smt_mode:c_uint, pub emul_smt_mode:c_uint, pub kvm_ops:*mut kvmppc_ops }
pub const VCORE_EXIT_REQ:u32=0x10000;
pub const VCORE_INACTIVE:u32=0; pub const VCORE_PREEMPT:u32=1; pub const VCORE_PIGGYBACK:u32=2; pub const VCORE_SLEEPING:u32=3; pub const VCORE_RUNNING:u32=4; pub const VCORE_EXITING:u32=5; pub const VCORE_POLLING:u32=6;
#[repr(C)] pub struct kvmppc_vpa { pub gpa:c_ulong, pub pinned_addr:*mut c_void, pub pinned_end:*mut c_void, pub next_gpa:c_ulong, pub len:c_ulong, pub update_pending:u8, pub dirty:bool }
#[repr(C)] pub struct kvmppc_pte { pub eaddr:ulong, pub vpage:u64, pub raddr:ulong, pub may_read:bool, pub may_write:bool, pub may_execute:bool, pub wimg:ulong, pub rc:ulong, pub page_size:u8, pub page_shift:u8 }
#[repr(C)] pub struct kvmppc_slb { pub esid:u64, pub vsid:u64, pub orige:u64, pub origv:u64, pub valid:bool, pub Ks:bool, pub Kp:bool, pub nx:bool, pub large:bool, pub tb:bool, pub class:bool, pub base_page_size:u8 }
#[repr(C)] pub struct kvmhv_tb_accumulator { pub seqcount:u64, pub tb_total:u64, pub tb_min:u64, pub tb_max:u64 }
pub const KVMPPC_BOOKE_IAC_NUM:u32=4; pub const KVMPPC_BOOKE_DAC_NUM:u32=2; pub const KVMPPC_BOOKE_MAX_IAC:u32=4; pub const KVMPPC_BOOKE_MAX_DAC:u32=2;
pub const KVMPPC_EPR_NONE:u32=0; pub const KVMPPC_EPR_USER:u32=1; pub const KVMPPC_EPR_KERNEL:u32=2;
pub const KVMPPC_IRQ_DEFAULT:u32=0; pub const KVMPPC_IRQ_MPIC:u32=1; pub const KVMPPC_IRQ_XICS:u32=2; pub const KVMPPC_IRQ_XIVE:u32=3;
pub const MMIO_HPTE_CACHE_SIZE:usize=4;
#[repr(C)] pub struct mmio_hpte_cache_entry { pub hpte_v:c_ulong, pub hpte_r:c_ulong, pub rpte:c_ulong, pub pte_index:c_ulong, pub eaddr:c_ulong, pub slb_v:c_ulong, pub mmio_update:c_long, pub slb_base_pshift:c_uint }
#[repr(C)] pub struct mmio_hpte_cache { pub entry:[mmio_hpte_cache_entry;MMIO_HPTE_CACHE_SIZE], pub index:c_uint }
pub const KVMPPC_VSX_COPY_NONE:u8=0; pub const KVMPPC_VSX_COPY_WORD:u8=1; pub const KVMPPC_VSX_COPY_DWORD:u8=2; pub const KVMPPC_VSX_COPY_DWORD_LOAD_DUMP:u8=3; pub const KVMPPC_VSX_COPY_WORD_LOAD_DUMP:u8=4;
pub const KVMPPC_VMX_COPY_BYTE:u8=8; pub const KVMPPC_VMX_COPY_HWORD:u8=9; pub const KVMPPC_VMX_COPY_WORD:u8=10; pub const KVMPPC_VMX_COPY_DWORD:u8=11;
#[repr(C)] pub union xive_tma_w01 { pub fields:xive_tma_w01_fields, pub w01:__be64 }
#[repr(C)] pub struct xive_tma_w01_fields { pub nsr:u8,pub cppr:u8,pub ipb:u8,pub lsmfb:u8,pub ack:u8,pub inc:u8,pub age:u8,pub pipr:u8 }
#[repr(C)] pub struct kvmhv_nestedv2_config { pub vcpu_run_output_cfg:kvmppc_gs_buff_info, pub vcpu_run_input_cfg:kvmppc_gs_buff_info, pub vcpu_run_output_size:u64 }
#[repr(C)] pub struct kvmhv_nestedv2_io { pub cfg:kvmhv_nestedv2_config, pub vcpu_run_output:*mut kvmppc_gs_buff, pub vcpu_run_input:*mut kvmppc_gs_buff, pub vcpu_message:*mut kvmppc_gs_msg, pub vcore_message:*mut kvmppc_gs_msg, pub valids:kvmppc_gs_bitmap }

/* The large architecture state is represented field-for-field in the kernel ABI. */
#[repr(C)] pub struct kvm_vcpu_arch {
 pub host_stack:ulong,pub host_pid:u32, pub regs:pt_regs, pub fp:thread_fp_state, pub vrsave:u32,pub mmucr:u32,pub shadow_msr:ulong,
 pub csrr0:ulong,pub csrr1:ulong,pub dsrr0:ulong,pub dsrr1:ulong,pub mcsrr0:ulong,pub mcsrr1:ulong,pub mcsr:ulong,pub dec:ulong,
 pub entry_tb:u64,pub entry_vtb:u64,pub entry_ic:u64,pub tcr:u32,pub tsr:ulong,pub ivor:[u32;64],pub ivpr:ulong,pub pvr:u32,
 pub shadow_pid:u32,pub shadow_pid1:u32,pub pid:u32,pub swap_pid:u32,pub ccr0:u32,pub ccr1:u32,pub dbsr:u32,
 pub mmcr:[u64;4],pub mmcra:u64,pub mmcrs:u64,pub pmc:[u32;8],pub spmc:[u32;2],pub siar:u64,pub sdar:u64,pub sier:[u64;3],
 pub paddr_accessed:gpa_t,pub vaddr_accessed:gva_t,pub pgdir:*mut pgd_t,pub io_gpr:u16,pub mmio_host_swabbed:u8,pub mmio_sign_extend:u8,
 pub mmio_sp64_extend:u8,pub mmio_vsx_copy_nums:u8,pub mmio_vsx_offset:u8,pub mmio_vmx_copy_nums:u8,pub mmio_vmx_offset:u8,pub mmio_copy_type:u8,
 pub osi_needed:u8,pub osi_enabled:u8,pub papr_enabled:u8,pub watchdog_enabled:u8,pub sane:u8,pub cpu_type:u8,pub hcall_needed:u8,pub epr_flags:u8,pub epr_needed:u8,pub external_oneshot:u8,
 pub cpr0_cfgaddr:u32,pub dec_timer:hrtimer,pub dec_jiffies:u64,pub dec_expires:u64,pub pending_exceptions:ulong,pub ceded:u8,pub prodded:u8,pub doorbell_request:u8,pub irq_pending:u8,pub last_inst:ulong,
 pub wait:rcuwait,pub waitp:*mut rcuwait,pub vcore:*mut kvmppc_vcore,pub ret:c_int,pub trap:c_int,pub state:c_int,pub ptid:c_int,pub thread_cpu:c_int,pub prev_cpu:c_int,pub timer_running:bool,pub cpu_run:wait_queue_head_t,
 pub shared:*mut kvm_vcpu_arch_shared,pub magic_page_pa:ulong,pub magic_page_ea:ulong,pub disable_kernel_nx:bool,pub irq_type:c_int,pub irq_cpu_id:c_int,pub mpic:*mut openpic,
}
pub const KVMPPC_VCPU_NOTREADY:u32=0; pub const KVMPPC_VCPU_RUNNABLE:u32=1; pub const KVMPPC_VCPU_BUSY_IN_HOST:u32=2;
pub const KVM_MMIO_REG_MASK:u32=0x003f; pub const KVM_MMIO_REG_EXT_MASK:u32=0xffc0; pub const KVM_MMIO_REG_GPR:u32=0; pub const KVM_MMIO_REG_FPR:u32=0x40; pub const KVM_MMIO_REG_QPR:u32=0x80; pub const KVM_MMIO_REG_FQPR:u32=0xc0; pub const KVM_MMIO_REG_VSX:u32=0x100; pub const KVM_MMIO_REG_VMX:u32=0x180; pub const KVM_MMIO_REG_NESTED_GPR:u32=0xffc0;
pub const __KVM_HAVE_ARCH_VCPU_DEBUGFS:bool=true; pub const __KVM_HAVE_ARCH_INTC_INITIALIZED:bool=true; pub const __KVM_HAVE_ARCH_WQP:bool=true; pub const __KVM_HAVE_CREATE_DEVICE:bool=true;
pub unsafe fn kvm_arch_memslots_updated(_kvm:*mut kvm,_gen:u64){} pub unsafe fn kvm_arch_flush_shadow_all(_kvm:*mut kvm){} pub unsafe fn kvm_arch_vcpu_blocking(_vcpu:*mut kvm_vcpu){} pub unsafe fn kvm_arch_vcpu_unblocking(_vcpu:*mut kvm_vcpu){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
