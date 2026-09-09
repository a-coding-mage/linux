/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the s390 KVM UAPI header. */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

pub const KVM_S390_CMMA_PEEK: u32 = 1 << 0;
#[repr(C)] pub struct kvm_s390_skeys { pub start_gfn: __u64, pub count: __u64, pub skeydata_addr: __u64, pub flags: __u32, pub reserved: [__u32; 9] }
#[repr(C)] pub union kvm_s390_cmma_log__u { pub remaining: __u64, pub mask: __u64 }
#[repr(C)] pub struct kvm_s390_cmma_log { pub start_gfn: __u64, pub count: __u32, pub flags: __u32, pub u: kvm_s390_cmma_log__u, pub values: __u64 }

pub const KVM_S390_RESET_POR: u32 = 1; pub const KVM_S390_RESET_CLEAR: u32 = 2; pub const KVM_S390_RESET_SUBSYSTEM: u32 = 4; pub const KVM_S390_RESET_CPU_INIT: u32 = 8; pub const KVM_S390_RESET_IPL: u32 = 16;
#[repr(C)] pub union kvm_s390_mem_op__u { pub access: kvm_s390_mem_op_access, pub sida_offset: __u32, pub reserved: [__u8; 32] }
#[repr(C)] pub struct kvm_s390_mem_op_access { pub ar: __u8, pub key: __u8, pub pad1: [__u8; 6], pub old_addr: __u64 }
#[repr(C)] pub struct kvm_s390_mem_op { pub gaddr: __u64, pub flags: __u64, pub size: __u32, pub op: __u32, pub buf: __u64, pub u: kvm_s390_mem_op__u }
pub const KVM_S390_MEMOP_LOGICAL_READ: u32 = 0; pub const KVM_S390_MEMOP_LOGICAL_WRITE: u32 = 1; pub const KVM_S390_MEMOP_SIDA_READ: u32 = 2; pub const KVM_S390_MEMOP_SIDA_WRITE: u32 = 3; pub const KVM_S390_MEMOP_ABSOLUTE_READ: u32 = 4; pub const KVM_S390_MEMOP_ABSOLUTE_WRITE: u32 = 5; pub const KVM_S390_MEMOP_ABSOLUTE_CMPXCHG: u32 = 6;
pub const KVM_S390_MEMOP_F_CHECK_ONLY: __u64 = 1 << 0; pub const KVM_S390_MEMOP_F_INJECT_EXCEPTION: __u64 = 1 << 1; pub const KVM_S390_MEMOP_F_SKEY_PROTECTION: __u64 = 1 << 2;
pub const KVM_S390_MEMOP_EXTENSION_CAP_BASE: u32 = 1 << 0; pub const KVM_S390_MEMOP_EXTENSION_CAP_CMPXCHG: u32 = 1 << 1;
#[repr(C)] pub struct kvm_s390_psw { pub mask: __u64, pub addr: __u64 }
pub const KVM_S390_SIGP_STOP: u32 = 0xfffe0000; pub const KVM_S390_PROGRAM_INT: u32 = 0xfffe0001; pub const KVM_S390_SIGP_SET_PREFIX: u32 = 0xfffe0002; pub const KVM_S390_RESTART: u32 = 0xfffe0003; pub const KVM_S390_INT_PFAULT_INIT: u32 = 0xfffe0004; pub const KVM_S390_INT_PFAULT_DONE: u32 = 0xfffe0005; pub const KVM_S390_MCHK: u32 = 0xfffe1000; pub const KVM_S390_INT_CLOCK_COMP: u32 = 0xffff1004; pub const KVM_S390_INT_CPU_TIMER: u32 = 0xffff1005; pub const KVM_S390_INT_VIRTIO: u32 = 0xffff2603; pub const KVM_S390_INT_SERVICE: u32 = 0xffff2401; pub const KVM_S390_INT_EMERGENCY: u32 = 0xffff1201; pub const KVM_S390_INT_EXTERNAL_CALL: u32 = 0xffff1202;
#[inline] pub const fn KVM_S390_INT_IO(ai: u32, cssid: u32, ssid: u32, schid: u32) -> u32 { schid | (ssid << 16) | (cssid << 18) | (ai << 26) }
pub const KVM_S390_INT_IO_MIN: u32 = 0; pub const KVM_S390_INT_IO_MAX: u32 = 0xfffdffff; pub const KVM_S390_INT_IO_AI_MASK: u32 = 0x04000000;
#[repr(C)] pub struct kvm_s390_interrupt { pub type_: __u32, pub parm: __u32, pub parm64: __u64 }
#[repr(C)] pub struct kvm_s390_io_info { pub subchannel_id: __u16, pub subchannel_nr: __u16, pub io_int_parm: __u32, pub io_int_word: __u32 }
#[repr(C)] pub struct kvm_s390_ext_info { pub ext_params: __u32, pub pad: __u32, pub ext_params2: __u64 }
#[repr(C)] pub struct kvm_s390_pgm_info { pub trans_exc_code: __u64, pub mon_code: __u64, pub per_address: __u64, pub data_exc_code: __u32, pub code: __u16, pub mon_class_nr: __u16, pub per_code: __u8, pub per_atmid: __u8, pub exc_access_id: __u8, pub per_access_id: __u8, pub op_access_id: __u8, pub flags: __u8, pub pad: [__u8; 2] }
pub const KVM_S390_PGM_FLAGS_ILC_VALID: u8 = 1; pub const KVM_S390_PGM_FLAGS_ILC_0: u8 = 2; pub const KVM_S390_PGM_FLAGS_ILC_1: u8 = 4; pub const KVM_S390_PGM_FLAGS_ILC_MASK: u8 = 6; pub const KVM_S390_PGM_FLAGS_NO_REWIND: u8 = 8;
#[repr(C)] pub struct kvm_s390_prefix_info { pub address: __u32 } #[repr(C)] pub struct kvm_s390_extcall_info { pub code: __u16 } #[repr(C)] pub struct kvm_s390_emerg_info { pub code: __u16 }
#[repr(C)] pub struct kvm_s390_stop_info { pub flags: __u32 } pub const KVM_S390_STOP_FLAG_STORE_STATUS: u32 = 1;
#[repr(C)] pub struct kvm_s390_mchk_info { pub cr14: __u64, pub mcic: __u64, pub failing_storage_address: __u64, pub ext_damage_code: __u32, pub pad: __u32, pub fixed_logout: [__u8; 16] }
#[repr(C)] pub union kvm_s390_irq__u { pub io: kvm_s390_io_info, pub ext: kvm_s390_ext_info, pub pgm: kvm_s390_pgm_info, pub emerg: kvm_s390_emerg_info, pub extcall: kvm_s390_extcall_info, pub prefix: kvm_s390_prefix_info, pub stop: kvm_s390_stop_info, pub mchk: kvm_s390_mchk_info, pub reserved: [u8; 64] }
#[repr(C)] pub struct kvm_s390_irq { pub type_: __u64, pub u: kvm_s390_irq__u } #[repr(C)] pub struct kvm_s390_irq_state { pub buf: __u64, pub flags: __u32, pub len: __u32, pub reserved: [__u32; 4] }
#[repr(C)] pub struct kvm_s390_ucas_mapping { pub user_addr: __u64, pub vcpu_addr: __u64, pub length: __u64 } #[repr(C)] pub struct kvm_s390_pv_sec_parm { pub origin: __u64, pub length: __u64 } #[repr(C)] pub struct kvm_s390_pv_unp { pub addr: __u64, pub size: __u64, pub tweak: __u64 }
#[repr(C)] pub struct kvm_s390_pv_dmp { pub subcmd: __u64, pub buff_addr: __u64, pub buff_len: __u64, pub gaddr: __u64, pub reserved: [__u64; 4] }
pub const KVM_PV_DUMP_INIT: u32 = 0; pub const KVM_PV_DUMP_CONFIG_STOR_STATE: u32 = 1; pub const KVM_PV_DUMP_COMPLETE: u32 = 2; pub const KVM_PV_DUMP_CPU: u32 = 3;
pub const KVM_PV_INFO_VM: u32 = 0; pub const KVM_PV_INFO_DUMP: u32 = 1;
#[repr(C)] pub struct kvm_s390_pv_info_dump { pub dump_cpu_buffer_len: __u64, pub dump_config_mem_buffer_per_1m: __u64, pub dump_config_finalize_len: __u64 }
#[repr(C)] pub struct kvm_s390_pv_info_vm { pub inst_calls_list: [__u64; 4], pub max_cpus: __u64, pub max_guests: __u64, pub max_guest_addr: __u64, pub feature_indication: __u64 }
#[repr(C)] pub struct kvm_s390_pv_info_header { pub id: __u32, pub len_max: __u32, pub len_written: __u32, pub reserved: __u32 }
#[repr(C)] pub union kvm_s390_pv_info__u { pub dump: kvm_s390_pv_info_dump, pub vm: kvm_s390_pv_info_vm } #[repr(C)] pub struct kvm_s390_pv_info { pub header: kvm_s390_pv_info_header, pub u: kvm_s390_pv_info__u }
pub const KVM_PV_ENABLE: u32=0; pub const KVM_PV_DISABLE:u32=1; pub const KVM_PV_SET_SEC_PARMS:u32=2; pub const KVM_PV_UNPACK:u32=3; pub const KVM_PV_VERIFY:u32=4; pub const KVM_PV_PREP_RESET:u32=5; pub const KVM_PV_UNSHARE_ALL:u32=6; pub const KVM_PV_INFO:u32=7; pub const KVM_PV_DUMP:u32=8; pub const KVM_PV_ASYNC_CLEANUP_PREPARE:u32=9; pub const KVM_PV_ASYNC_CLEANUP_PERFORM:u32=10;
#[repr(C)] pub struct kvm_pv_cmd { pub cmd: __u32, pub rc: __u16, pub rrc: __u16, pub data: __u64, pub flags: __u32, pub reserved: [__u32; 3] }
#[repr(C)] pub struct kvm_s390_zpci_reg_aen { pub ibv: __u64, pub sb: __u64, pub flags: __u32, pub noi: __u32, pub isc: __u8, pub sbo: __u8, pub pad: __u16 }
#[repr(C)] pub union kvm_s390_zpci_op__u { pub reg_aen: kvm_s390_zpci_reg_aen, pub reserved: [__u64; 8] }
#[repr(C)] pub struct kvm_s390_zpci_op { pub fh: __u32, pub op: __u8, pub pad: [__u8; 3], pub u: kvm_s390_zpci_op__u }
pub const KVM_S390_ZPCIOP_REG_AEN:u8=0; pub const KVM_S390_ZPCIOP_DEREG_AEN:u8=1; pub const KVM_S390_ZPCIOP_REGAEN_HOST:u32=1;
pub const KVM_DEV_FLIC_GET_ALL_IRQS:u32=1; pub const KVM_DEV_FLIC_ENQUEUE:u32=2; pub const KVM_DEV_FLIC_CLEAR_IRQS:u32=3; pub const KVM_DEV_FLIC_APF_ENABLE:u32=4; pub const KVM_DEV_FLIC_APF_DISABLE_WAIT:u32=5; pub const KVM_DEV_FLIC_ADAPTER_REGISTER:u32=6; pub const KVM_DEV_FLIC_ADAPTER_MODIFY:u32=7; pub const KVM_DEV_FLIC_CLEAR_IO_IRQ:u32=8; pub const KVM_DEV_FLIC_AISM:u32=9; pub const KVM_DEV_FLIC_AIRQ_INJECT:u32=10; pub const KVM_DEV_FLIC_AISM_ALL:u32=11; pub const KVM_S390_MAX_FLOAT_IRQS:u32=266250; pub const KVM_S390_FLIC_MAX_BUFFER:u32=0x2000000;
#[repr(C)] pub struct kvm_s390_io_adapter { pub id:__u32,pub isc:__u8,pub maskable:__u8,pub swap:__u8,pub flags:__u8 } pub const KVM_S390_ADAPTER_SUPPRESSIBLE:u8=1;
#[repr(C)] pub struct kvm_s390_ais_req { pub isc:__u8,pub mode:__u16 } #[repr(C)] pub struct kvm_s390_ais_all { pub simm:__u8,pub nimm:__u8 }
pub const KVM_S390_IO_ADAPTER_MASK:u32=1; pub const KVM_S390_IO_ADAPTER_MAP:u32=2; pub const KVM_S390_IO_ADAPTER_UNMAP:u32=3;
#[repr(C)] pub struct kvm_s390_io_adapter_req { pub id:__u32,pub type_:__u8,pub mask:__u8,pub pad0:__u16,pub addr:__u64 }
pub const KVM_S390_VM_MEM_CTRL:u32=0; pub const KVM_S390_VM_TOD:u32=1; pub const KVM_S390_VM_CRYPTO:u32=2; pub const KVM_S390_VM_CPU_MODEL:u32=3; pub const KVM_S390_VM_MIGRATION:u32=4; pub const KVM_S390_VM_CPU_TOPOLOGY:u32=5;
pub const KVM_S390_VM_MEM_ENABLE_CMMA:u32=0; pub const KVM_S390_VM_MEM_CLR_CMMA:u32=1; pub const KVM_S390_VM_MEM_LIMIT_SIZE:u32=2; pub const KVM_S390_NO_MEM_LIMIT:__u64=__u64::MAX;
pub const KVM_S390_VM_TOD_LOW:u32=0; pub const KVM_S390_VM_TOD_HIGH:u32=1; pub const KVM_S390_VM_TOD_EXT:u32=2;
#[repr(C)] pub struct kvm_s390_vm_tod_clock { pub epoch_idx:__u8,pub tod:__u64 } pub const KVM_S390_VM_CPU_PROCESSOR:u32=0;
#[repr(C)] pub struct kvm_s390_vm_cpu_processor { pub cpuid:__u64,pub ibc:__u16,pub pad:[__u8;6],pub fac_list:[__u64;256] }
pub const KVM_S390_VM_CPU_MACHINE:u32=1; #[repr(C)] pub struct kvm_s390_vm_cpu_machine { pub cpuid:__u64,pub ibc:__u32,pub pad:[__u8;4],pub fac_mask:[__u64;256],pub fac_list:[__u64;256] }
pub const KVM_S390_VM_CPU_PROCESSOR_FEAT:u32=2; pub const KVM_S390_VM_CPU_MACHINE_FEAT:u32=3; pub const KVM_S390_VM_CPU_FEAT_NR_BITS:u32=1024;
pub const KVM_S390_VM_CPU_FEAT_ESOP:u32=0; pub const KVM_S390_VM_CPU_FEAT_SIEF2:u32=1; pub const KVM_S390_VM_CPU_FEAT_64BSCAO:u32=2; pub const KVM_S390_VM_CPU_FEAT_SIIF:u32=3; pub const KVM_S390_VM_CPU_FEAT_GPERE:u32=4; pub const KVM_S390_VM_CPU_FEAT_GSLS:u32=5; pub const KVM_S390_VM_CPU_FEAT_IB:u32=6; pub const KVM_S390_VM_CPU_FEAT_CEI:u32=7; pub const KVM_S390_VM_CPU_FEAT_IBS:u32=8; pub const KVM_S390_VM_CPU_FEAT_SKEY:u32=9; pub const KVM_S390_VM_CPU_FEAT_CMMA:u32=10; pub const KVM_S390_VM_CPU_FEAT_PFMFI:u32=11; pub const KVM_S390_VM_CPU_FEAT_SIGPIF:u32=12; pub const KVM_S390_VM_CPU_FEAT_KSS:u32=13; pub const KVM_S390_VM_CPU_FEAT_ASTFLEIE2:u32=14;
#[repr(C)] pub struct kvm_s390_vm_cpu_feat { pub feat:[__u64;16] } pub const KVM_S390_VM_CPU_PROCESSOR_SUBFUNC:u32=4; pub const KVM_S390_VM_CPU_MACHINE_SUBFUNC:u32=5;
#[repr(C)] pub struct kvm_s390_vm_cpu_subfunc { pub plo:[__u8;32],pub ptff:[__u8;16],pub kmac:[__u8;16],pub kmc:[__u8;16],pub km:[__u8;16],pub kimd:[__u8;16],pub klmd:[__u8;16],pub pckmo:[__u8;16],pub kmctr:[__u8;16],pub kmf:[__u8;16],pub kmo:[__u8;16],pub pcc:[__u8;16],pub ppno:[__u8;16],pub kma:[__u8;16],pub kdsa:[__u8;16],pub sortl:[__u8;32],pub dfltcc:[__u8;32],pub pfcr:[__u8;16],pub reserved:[__u8;1712] }
pub const KVM_S390_VM_CPU_PROCESSOR_UV_FEAT_GUEST:u32=6; pub const KVM_S390_VM_CPU_MACHINE_UV_FEAT_GUEST:u32=7; pub const KVM_S390_VM_CPU_UV_FEAT_NR_BITS:u32=64;
#[repr(C)] pub union kvm_s390_vm_cpu_uv_feat__u { pub feat:__u64 } #[repr(C)] pub struct kvm_s390_vm_cpu_uv_feat { pub u:kvm_s390_vm_cpu_uv_feat__u }
pub const KVM_S390_VM_CRYPTO_ENABLE_AES_KW:u32=0; pub const KVM_S390_VM_CRYPTO_ENABLE_DEA_KW:u32=1; pub const KVM_S390_VM_CRYPTO_DISABLE_AES_KW:u32=2; pub const KVM_S390_VM_CRYPTO_DISABLE_DEA_KW:u32=3; pub const KVM_S390_VM_CRYPTO_ENABLE_APIE:u32=4; pub const KVM_S390_VM_CRYPTO_DISABLE_APIE:u32=5;
pub const KVM_S390_VM_MIGRATION_STOP:u32=0; pub const KVM_S390_VM_MIGRATION_START:u32=1; pub const KVM_S390_VM_MIGRATION_STATUS:u32=2;
#[repr(C)] pub struct kvm_regs { pub gprs:[__u64;16] } #[repr(C)] pub struct kvm_sregs { pub acrs:[__u32;16],pub crs:[__u64;16] } #[repr(C)] pub struct kvm_fpu { pub fpc:__u32,pub fprs:[__u64;16] }
pub const KVM_GUESTDBG_USE_HW_BP:u32=0x00010000; pub const KVM_HW_BP:u32=1; pub const KVM_HW_WP_WRITE:u32=2; pub const KVM_SINGLESTEP:u32=4;
#[repr(C)] pub struct kvm_debug_exit_arch { pub addr:__u64,pub type_:__u8,pub pad:[__u8;7] } #[repr(C)] pub struct kvm_hw_breakpoint { pub addr:__u64,pub phys_addr:__u64,pub len:__u64,pub type_:__u8,pub pad:[__u8;7] }
#[repr(C)] pub struct kvm_guest_debug_arch { pub nr_hw_bp:__u32,pub pad:__u32,pub hw_bp:*mut kvm_hw_breakpoint }
pub const KVM_S390_PFAULT_TOKEN_INVALID:__u64=0xffffffffffffffff; pub const KVM_SYNC_PREFIX:__u64=1<<0; pub const KVM_SYNC_GPRS:__u64=1<<1; pub const KVM_SYNC_ACRS:__u64=1<<2; pub const KVM_SYNC_CRS:__u64=1<<3; pub const KVM_SYNC_ARCH0:__u64=1<<4; pub const KVM_SYNC_PFAULT:__u64=1<<5; pub const KVM_SYNC_VRS:__u64=1<<6; pub const KVM_SYNC_RICCB:__u64=1<<7; pub const KVM_SYNC_FPRS:__u64=1<<8; pub const KVM_SYNC_GSCB:__u64=1<<9; pub const KVM_SYNC_BPBC:__u64=1<<10; pub const KVM_SYNC_ETOKEN:__u64=1<<11; pub const KVM_SYNC_DIAG318:__u64=1<<12;
pub const KVM_SYNC_S390_VALID_FIELDS:__u64=(1<<13)-1; pub const SDNXC:u32=8; pub const SDNXL:usize=1<<SDNXC;
#[repr(C)] pub union kvm_sync_regs__u { pub vrs:[[__u64;2];32],pub fprs:[__u64;16] } #[repr(C)] pub union kvm_sync_regs__u2 { pub sdnx:[__u8;SDNXL],pub fields:kvm_sync_regs_sdnx }
#[repr(C)] pub struct kvm_sync_regs_sdnx { pub reserved1:[__u64;2],pub gscb:[__u64;4],pub etoken:__u64,pub etoken_extension:__u64 }
#[repr(C)] pub struct kvm_sync_regs { pub prefix:__u64,pub gprs:[__u64;16],pub acrs:[__u32;16],pub crs:[__u64;16],pub todpr:__u64,pub cputm:__u64,pub ckc:__u64,pub pp:__u64,pub gbea:__u64,pub pft:__u64,pub pfs:__u64,pub pfc:__u64,pub u:kvm_sync_regs__u,pub reserved:[__u8;512],pub fpc:__u32,pub bpbc:__u8,pub reserved2:__u8,pub padding1:[__u8;51],pub riccb:[__u8;64],pub diag318:__u64,pub padding2:[__u8;184],pub u2:kvm_sync_regs__u2 }
pub const KVM_REG_S390_TODPR:__u64=(KVM_REG_S390|KVM_REG_SIZE_U32|0x1); pub const KVM_REG_S390_EPOCHDIFF:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x2); pub const KVM_REG_S390_CPU_TIMER:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x3); pub const KVM_REG_S390_CLOCK_COMP:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x4); pub const KVM_REG_S390_PFTOKEN:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x5); pub const KVM_REG_S390_PFCOMPARE:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x6); pub const KVM_REG_S390_PFSELECT:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x7); pub const KVM_REG_S390_PP:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x8); pub const KVM_REG_S390_GBEA:__u64=(KVM_REG_S390|KVM_REG_SIZE_U64|0x9);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
