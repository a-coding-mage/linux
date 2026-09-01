/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * KVM s390 specific structures and definitions
 *
 * Copyright IBM Corp. 2008, 2018
 *
 *    Author(s): Carsten Otte <cotte@de.ibm.com>
 *               Christian Borntraeger <borntraeger@de.ibm.com>
 */
/* C source included <linux/types.h>. */

pub const __KVM_S390: bool = true;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_skeys {
    pub start_gfn: __u64,
    pub count: __u64,
    pub skeydata_addr: __u64,
    pub flags: __u32,
    pub reserved: [__u32; 9],
}

pub const KVM_S390_CMMA_PEEK: __u32 = 1 << 0;

/**
 * kvm_s390_cmma_log - Used for CMMA migration.
 *
 * Used both for input and output.
 *
 * @start_gfn: Guest page number to start from.
 * @count: Size of the result buffer.
 * @flags: Control operation mode via KVM_S390_CMMA_* flags
 * @remaining: Used with KVM_S390_GET_CMMA_BITS. Indicates how many dirty
 *             pages are still remaining.
 * @mask: Used with KVM_S390_SET_CMMA_BITS. Bitmap of bits to actually set
 *        in the PGSTE.
 * @values: Pointer to the values buffer.
 *
 * Used in KVM_S390_{G,S}ET_CMMA_BITS ioctls.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_cmma_log {
    pub start_gfn: __u64,
    pub count: __u32,
    pub flags: __u32,
    pub u: kvm_s390_cmma_log__bindgen_ty_1,
    pub values: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_cmma_log__bindgen_ty_1 {
    pub remaining: __u64,
    pub mask: __u64,
}

pub const KVM_S390_RESET_POR: __u32 = 1;
pub const KVM_S390_RESET_CLEAR: __u32 = 2;
pub const KVM_S390_RESET_SUBSYSTEM: __u32 = 4;
pub const KVM_S390_RESET_CPU_INIT: __u32 = 8;
pub const KVM_S390_RESET_IPL: __u32 = 16;

/* for KVM_S390_MEM_OP */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mem_op {
    /* in */
    pub gaddr: __u64, /* the guest address */
    pub flags: __u64, /* flags */
    pub size: __u32, /* amount of bytes */
    pub op: __u32, /* type of operation */
    pub buf: __u64, /* buffer in userspace */
    pub u: kvm_s390_mem_op__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_mem_op__bindgen_ty_1 {
    pub logical: kvm_s390_mem_op__bindgen_ty_1__bindgen_ty_1,
    pub sida_offset: __u32, /* offset into the sida */
    pub reserved: [__u8; 32], /* ignored */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mem_op__bindgen_ty_1__bindgen_ty_1 {
    pub ar: __u8, /* the access register number */
    pub key: __u8, /* access key, ignored if flag unset */
    pub pad1: [__u8; 6], /* ignored */
    pub old_addr: __u64, /* ignored if cmpxchg flag unset */
}

/* types for kvm_s390_mem_op->op */
pub const KVM_S390_MEMOP_LOGICAL_READ: __u32 = 0;
pub const KVM_S390_MEMOP_LOGICAL_WRITE: __u32 = 1;
pub const KVM_S390_MEMOP_SIDA_READ: __u32 = 2;
pub const KVM_S390_MEMOP_SIDA_WRITE: __u32 = 3;
pub const KVM_S390_MEMOP_ABSOLUTE_READ: __u32 = 4;
pub const KVM_S390_MEMOP_ABSOLUTE_WRITE: __u32 = 5;
pub const KVM_S390_MEMOP_ABSOLUTE_CMPXCHG: __u32 = 6;

/* flags for kvm_s390_mem_op->flags */
pub const KVM_S390_MEMOP_F_CHECK_ONLY: __u64 = 1u64 << 0;
pub const KVM_S390_MEMOP_F_INJECT_EXCEPTION: __u64 = 1u64 << 1;
pub const KVM_S390_MEMOP_F_SKEY_PROTECTION: __u64 = 1u64 << 2;

/* flags specifying extension support via KVM_CAP_S390_MEM_OP_EXTENSION */
pub const KVM_S390_MEMOP_EXTENSION_CAP_BASE: __u32 = 1 << 0;
pub const KVM_S390_MEMOP_EXTENSION_CAP_CMPXCHG: __u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_psw {
    pub mask: __u64,
    pub addr: __u64,
}

/* valid values for type in kvm_s390_interrupt */
pub const KVM_S390_SIGP_STOP: __u32 = 0xfffe0000u32;
pub const KVM_S390_PROGRAM_INT: __u32 = 0xfffe0001u32;
pub const KVM_S390_SIGP_SET_PREFIX: __u32 = 0xfffe0002u32;
pub const KVM_S390_RESTART: __u32 = 0xfffe0003u32;
pub const KVM_S390_INT_PFAULT_INIT: __u32 = 0xfffe0004u32;
pub const KVM_S390_INT_PFAULT_DONE: __u32 = 0xfffe0005u32;
pub const KVM_S390_MCHK: __u32 = 0xfffe1000u32;
pub const KVM_S390_INT_CLOCK_COMP: __u32 = 0xffff1004u32;
pub const KVM_S390_INT_CPU_TIMER: __u32 = 0xffff1005u32;
pub const KVM_S390_INT_VIRTIO: __u32 = 0xffff2603u32;
pub const KVM_S390_INT_SERVICE: __u32 = 0xffff2401u32;
pub const KVM_S390_INT_EMERGENCY: __u32 = 0xffff1201u32;
pub const KVM_S390_INT_EXTERNAL_CALL: __u32 = 0xffff1202u32;
/* Anything below 0xfffe0000u is taken by INT_IO */
pub const fn KVM_S390_INT_IO(ai: __u32, cssid: __u32, ssid: __u32, schid: __u32) -> __u32 {
    (schid) | ((ssid) << 16) | ((cssid) << 18) | ((ai) << 26)
}
pub const KVM_S390_INT_IO_MIN: __u32 = 0x00000000u32;
pub const KVM_S390_INT_IO_MAX: __u32 = 0xfffdffffu32;
pub const KVM_S390_INT_IO_AI_MASK: __u32 = 0x04000000u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_interrupt {
    pub type_: __u32,
    pub parm: __u32,
    pub parm64: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_info {
    pub subchannel_id: __u16,
    pub subchannel_nr: __u16,
    pub io_int_parm: __u32,
    pub io_int_word: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ext_info {
    pub ext_params: __u32,
    pub pad: __u32,
    pub ext_params2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pgm_info {
    pub trans_exc_code: __u64,
    pub mon_code: __u64,
    pub per_address: __u64,
    pub data_exc_code: __u32,
    pub code: __u16,
    pub mon_class_nr: __u16,
    pub per_code: __u8,
    pub per_atmid: __u8,
    pub exc_access_id: __u8,
    pub per_access_id: __u8,
    pub op_access_id: __u8,
    pub flags: __u8,
    pub pad: [__u8; 2],
}

pub const KVM_S390_PGM_FLAGS_ILC_VALID: __u8 = 0x01;
pub const KVM_S390_PGM_FLAGS_ILC_0: __u8 = 0x02;
pub const KVM_S390_PGM_FLAGS_ILC_1: __u8 = 0x04;
pub const KVM_S390_PGM_FLAGS_ILC_MASK: __u8 = 0x06;
pub const KVM_S390_PGM_FLAGS_NO_REWIND: __u8 = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_prefix_info {
    pub address: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_extcall_info {
    pub code: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_emerg_info {
    pub code: __u16,
}

pub const KVM_S390_STOP_FLAG_STORE_STATUS: __u32 = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_stop_info {
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mchk_info {
    pub cr14: __u64,
    pub mcic: __u64,
    pub failing_storage_address: __u64,
    pub ext_damage_code: __u32,
    pub pad: __u32,
    pub fixed_logout: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_irq {
    pub type_: __u64,
    pub u: kvm_s390_irq__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_irq__bindgen_ty_1 {
    pub io: kvm_s390_io_info,
    pub ext: kvm_s390_ext_info,
    pub pgm: kvm_s390_pgm_info,
    pub emerg: kvm_s390_emerg_info,
    pub extcall: kvm_s390_extcall_info,
    pub prefix: kvm_s390_prefix_info,
    pub stop: kvm_s390_stop_info,
    pub mchk: kvm_s390_mchk_info,
    pub reserved: [i8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_irq_state {
    pub buf: __u64,
    pub flags: __u32, /* will stay unused for compatibility reasons */
    pub len: __u32,
    pub reserved: [__u32; 4], /* will stay unused for compatibility reasons */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ucas_mapping {
    pub user_addr: __u64,
    pub vcpu_addr: __u64,
    pub length: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_sec_parm {
    pub origin: __u64,
    pub length: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_unp {
    pub addr: __u64,
    pub size: __u64,
    pub tweak: __u64,
}

pub const KVM_PV_DUMP_INIT: __u32 = 0;
pub const KVM_PV_DUMP_CONFIG_STOR_STATE: __u32 = 1;
pub const KVM_PV_DUMP_COMPLETE: __u32 = 2;
pub const KVM_PV_DUMP_CPU: __u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_dmp {
    pub subcmd: __u64,
    pub buff_addr: __u64,
    pub buff_len: __u64,
    pub gaddr: __u64, /* For dump storage state */
    pub reserved: [__u64; 4],
}

pub const KVM_PV_INFO_VM: __u32 = 0;
pub const KVM_PV_INFO_DUMP: __u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_dump {
    pub dump_cpu_buffer_len: __u64,
    pub dump_config_mem_buffer_per_1m: __u64,
    pub dump_config_finalize_len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_vm {
    pub inst_calls_list: [__u64; 4],
    pub max_cpus: __u64,
    pub max_guests: __u64,
    pub max_guest_addr: __u64,
    pub feature_indication: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_header {
    pub id: __u32,
    pub len_max: __u32,
    pub len_written: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info {
    pub header: kvm_s390_pv_info_header,
    pub u: kvm_s390_pv_info__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_pv_info__bindgen_ty_1 {
    pub dump: kvm_s390_pv_info_dump,
    pub vm: kvm_s390_pv_info_vm,
}

pub const KVM_PV_ENABLE: __u32 = 0;
pub const KVM_PV_DISABLE: __u32 = 1;
pub const KVM_PV_SET_SEC_PARMS: __u32 = 2;
pub const KVM_PV_UNPACK: __u32 = 3;
pub const KVM_PV_VERIFY: __u32 = 4;
pub const KVM_PV_PREP_RESET: __u32 = 5;
pub const KVM_PV_UNSHARE_ALL: __u32 = 6;
pub const KVM_PV_INFO: __u32 = 7;
pub const KVM_PV_DUMP: __u32 = 8;
pub const KVM_PV_ASYNC_CLEANUP_PREPARE: __u32 = 9;
pub const KVM_PV_ASYNC_CLEANUP_PERFORM: __u32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pv_cmd {
    pub cmd: __u32, /* Command to be executed */
    pub rc: __u16, /* Ultravisor return code */
    pub rrc: __u16, /* Ultravisor return reason code */
    pub data: __u64, /* Data or address */
    pub flags: __u32, /* flags for future extensions. Must be 0 for now */
    pub reserved: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_zpci_op {
    /* in */
    pub fh: __u32, /* target device */
    pub op: __u8, /* operation to perform */
    pub pad: [__u8; 3],
    pub u: kvm_s390_zpci_op__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_zpci_op__bindgen_ty_1 {
    pub reg_aen: kvm_s390_zpci_op__bindgen_ty_1__bindgen_ty_1,
    pub reserved: [__u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_zpci_op__bindgen_ty_1__bindgen_ty_1 {
    pub ibv: __u64, /* Guest addr of interrupt bit vector */
    pub sb: __u64, /* Guest addr of summary bit */
    pub flags: __u32,
    pub noi: __u32, /* Number of interrupts */
    pub isc: __u8, /* Guest interrupt subclass */
    pub sbo: __u8, /* Offset of guest summary bit vector */
    pub pad: __u16,
}

/* types for kvm_s390_zpci_op->op */
pub const KVM_S390_ZPCIOP_REG_AEN: __u32 = 0;
pub const KVM_S390_ZPCIOP_DEREG_AEN: __u32 = 1;

/* flags for kvm_s390_zpci_op->u.reg_aen.flags */
pub const KVM_S390_ZPCIOP_REGAEN_HOST: __u32 = 1 << 0;

/* Device control API: s390-specific devices */
pub const KVM_DEV_FLIC_GET_ALL_IRQS: __u32 = 1;
pub const KVM_DEV_FLIC_ENQUEUE: __u32 = 2;
pub const KVM_DEV_FLIC_CLEAR_IRQS: __u32 = 3;
pub const KVM_DEV_FLIC_APF_ENABLE: __u32 = 4;
pub const KVM_DEV_FLIC_APF_DISABLE_WAIT: __u32 = 5;
pub const KVM_DEV_FLIC_ADAPTER_REGISTER: __u32 = 6;
pub const KVM_DEV_FLIC_ADAPTER_MODIFY: __u32 = 7;
pub const KVM_DEV_FLIC_CLEAR_IO_IRQ: __u32 = 8;
pub const KVM_DEV_FLIC_AISM: __u32 = 9;
pub const KVM_DEV_FLIC_AIRQ_INJECT: __u32 = 10;
pub const KVM_DEV_FLIC_AISM_ALL: __u32 = 11;
/*
 * We can have up to 4*64k pending subchannels + 8 adapter interrupts,
 * as well as up  to ASYNC_PF_PER_VCPU*KVM_MAX_VCPUS pfault done interrupts.
 * There are also sclp and machine checks. This gives us
 * sizeof(kvm_s390_irq)*(4*65536+8+64*64+1+1) = 72 * 266250 = 19170000
 * Lets round up to 8192 pages.
 */
pub const KVM_S390_MAX_FLOAT_IRQS: __u32 = 266250;
pub const KVM_S390_FLIC_MAX_BUFFER: __u32 = 0x2000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_adapter {
    pub id: __u32,
    pub isc: __u8,
    pub maskable: __u8,
    pub swap: __u8,
    pub flags: __u8,
}

pub const KVM_S390_ADAPTER_SUPPRESSIBLE: __u32 = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ais_req {
    pub isc: __u8,
    pub mode: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ais_all {
    pub simm: __u8,
    pub nimm: __u8,
}

pub const KVM_S390_IO_ADAPTER_MASK: __u32 = 1;
pub const KVM_S390_IO_ADAPTER_MAP: __u32 = 2;
pub const KVM_S390_IO_ADAPTER_UNMAP: __u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_adapter_req {
    pub id: __u32,
    pub type_: __u8,
    pub mask: __u8,
    pub pad0: __u16,
    pub addr: __u64,
}

/* kvm attr_group  on vm fd */
pub const KVM_S390_VM_MEM_CTRL: __u32 = 0;
pub const KVM_S390_VM_TOD: __u32 = 1;
pub const KVM_S390_VM_CRYPTO: __u32 = 2;
pub const KVM_S390_VM_CPU_MODEL: __u32 = 3;
pub const KVM_S390_VM_MIGRATION: __u32 = 4;
pub const KVM_S390_VM_CPU_TOPOLOGY: __u32 = 5;

/* kvm attributes for mem_ctrl */
pub const KVM_S390_VM_MEM_ENABLE_CMMA: __u32 = 0;
pub const KVM_S390_VM_MEM_CLR_CMMA: __u32 = 1;
pub const KVM_S390_VM_MEM_LIMIT_SIZE: __u32 = 2;

pub const KVM_S390_NO_MEM_LIMIT: __u64 = U64_MAX;

/* kvm attributes for KVM_S390_VM_TOD */
pub const KVM_S390_VM_TOD_LOW: __u32 = 0;
pub const KVM_S390_VM_TOD_HIGH: __u32 = 1;
pub const KVM_S390_VM_TOD_EXT: __u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_tod_clock {
    pub epoch_idx: __u8,
    pub tod: __u64,
}

/* kvm attributes for KVM_S390_VM_CPU_MODEL */
/* processor related attributes are r/w */
pub const KVM_S390_VM_CPU_PROCESSOR: __u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_processor {
    pub cpuid: __u64,
    pub ibc: __u16,
    pub pad: [__u8; 6],
    pub fac_list: [__u64; 256],
}

/* machine related attributes are r/o */
pub const KVM_S390_VM_CPU_MACHINE: __u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_machine {
    pub cpuid: __u64,
    pub ibc: __u32,
    pub pad: [__u8; 4],
    pub fac_mask: [__u64; 256],
    pub fac_list: [__u64; 256],
}

pub const KVM_S390_VM_CPU_PROCESSOR_FEAT: __u32 = 2;
pub const KVM_S390_VM_CPU_MACHINE_FEAT: __u32 = 3;

pub const KVM_S390_VM_CPU_FEAT_NR_BITS: __u32 = 1024;
pub const KVM_S390_VM_CPU_FEAT_ESOP: __u32 = 0;
pub const KVM_S390_VM_CPU_FEAT_SIEF2: __u32 = 1;
pub const KVM_S390_VM_CPU_FEAT_64BSCAO: __u32 = 2;
pub const KVM_S390_VM_CPU_FEAT_SIIF: __u32 = 3;
pub const KVM_S390_VM_CPU_FEAT_GPERE: __u32 = 4;
pub const KVM_S390_VM_CPU_FEAT_GSLS: __u32 = 5;
pub const KVM_S390_VM_CPU_FEAT_IB: __u32 = 6;
pub const KVM_S390_VM_CPU_FEAT_CEI: __u32 = 7;
pub const KVM_S390_VM_CPU_FEAT_IBS: __u32 = 8;
pub const KVM_S390_VM_CPU_FEAT_SKEY: __u32 = 9;
pub const KVM_S390_VM_CPU_FEAT_CMMA: __u32 = 10;
pub const KVM_S390_VM_CPU_FEAT_PFMFI: __u32 = 11;
pub const KVM_S390_VM_CPU_FEAT_SIGPIF: __u32 = 12;
pub const KVM_S390_VM_CPU_FEAT_KSS: __u32 = 13;
pub const KVM_S390_VM_CPU_FEAT_ASTFLEIE2: __u32 = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_feat {
    pub feat: [__u64; 16],
}

pub const KVM_S390_VM_CPU_PROCESSOR_SUBFUNC: __u32 = 4;
pub const KVM_S390_VM_CPU_MACHINE_SUBFUNC: __u32 = 5;
/* for "test bit" instructions MSB 0 bit ordering, for "query" raw blocks */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_subfunc {
    pub plo: [__u8; 32], /* always */
    pub ptff: [__u8; 16], /* with TOD-clock steering */
    pub kmac: [__u8; 16], /* with MSA */
    pub kmc: [__u8; 16], /* with MSA */
    pub km: [__u8; 16], /* with MSA */
    pub kimd: [__u8; 16], /* with MSA */
    pub klmd: [__u8; 16], /* with MSA */
    pub pckmo: [__u8; 16], /* with MSA3 */
    pub kmctr: [__u8; 16], /* with MSA4 */
    pub kmf: [__u8; 16], /* with MSA4 */
    pub kmo: [__u8; 16], /* with MSA4 */
    pub pcc: [__u8; 16], /* with MSA4 */
    pub ppno: [__u8; 16], /* with MSA5 */
    pub kma: [__u8; 16], /* with MSA8 */
    pub kdsa: [__u8; 16], /* with MSA9 */
    pub sortl: [__u8; 32], /* with STFLE.150 */
    pub dfltcc: [__u8; 32], /* with STFLE.151 */
    pub pfcr: [__u8; 16], /* with STFLE.201 */
    pub reserved: [__u8; 1712],
}

pub const KVM_S390_VM_CPU_PROCESSOR_UV_FEAT_GUEST: __u32 = 6;
pub const KVM_S390_VM_CPU_MACHINE_UV_FEAT_GUEST: __u32 = 7;

pub const KVM_S390_VM_CPU_UV_FEAT_NR_BITS: __u32 = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_uv_feat_bits {
    /* C bitfields in one __u64 storage unit:
     * anonymous reserved: 4, ap: 1 (bit 4), ap_intr: 1 (bit 5),
     * anonymous reserved: 58.
     */
    pub _bitfield_1: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_vm_cpu_uv_feat {
    pub bits: kvm_s390_vm_cpu_uv_feat_bits,
    pub feat: __u64,
}

/* kvm attributes for crypto */
pub const KVM_S390_VM_CRYPTO_ENABLE_AES_KW: __u32 = 0;
pub const KVM_S390_VM_CRYPTO_ENABLE_DEA_KW: __u32 = 1;
pub const KVM_S390_VM_CRYPTO_DISABLE_AES_KW: __u32 = 2;
pub const KVM_S390_VM_CRYPTO_DISABLE_DEA_KW: __u32 = 3;
pub const KVM_S390_VM_CRYPTO_ENABLE_APIE: __u32 = 4;
pub const KVM_S390_VM_CRYPTO_DISABLE_APIE: __u32 = 5;

/* kvm attributes for migration mode */
pub const KVM_S390_VM_MIGRATION_STOP: __u32 = 0;
pub const KVM_S390_VM_MIGRATION_START: __u32 = 1;
pub const KVM_S390_VM_MIGRATION_STATUS: __u32 = 2;

/* for KVM_GET_REGS and KVM_SET_REGS */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
    /* general purpose regs for s390 */
    pub gprs: [__u64; 16],
}

/* for KVM_GET_SREGS and KVM_SET_SREGS */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
    pub acrs: [__u32; 16],
    pub crs: [__u64; 16],
}

/* for KVM_GET_FPU and KVM_SET_FPU */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fpc: __u32,
    pub fprs: [__u64; 16],
}

pub const KVM_GUESTDBG_USE_HW_BP: __u32 = 0x00010000;

pub const KVM_HW_BP: __u32 = 1;
pub const KVM_HW_WP_WRITE: __u32 = 2;
pub const KVM_SINGLESTEP: __u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub addr: __u64,
    pub type_: __u8,
    pub pad: [__u8; 7], /* Should be set to 0 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hw_breakpoint {
    pub addr: __u64,
    pub phys_addr: __u64,
    pub len: __u64,
    pub type_: __u8,
    pub pad: [__u8; 7], /* Should be set to 0 */
}

/* for KVM_SET_GUEST_DEBUG */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
    pub nr_hw_bp: __u32,
    pub pad: __u32, /* Should be set to 0 */
    pub hw_bp: *mut kvm_hw_breakpoint,
}

/* for KVM_SYNC_PFAULT and KVM_REG_S390_PFTOKEN */
pub const KVM_S390_PFAULT_TOKEN_INVALID: __u64 = 0xffffffffffffffffu64;

pub const KVM_SYNC_PREFIX: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 0;
pub const KVM_SYNC_GPRS: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 1;
pub const KVM_SYNC_ACRS: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 2;
pub const KVM_SYNC_CRS: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 3;
pub const KVM_SYNC_ARCH0: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 4;
pub const KVM_SYNC_PFAULT: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 5;
pub const KVM_SYNC_VRS: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 6;
pub const KVM_SYNC_RICCB: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 7;
pub const KVM_SYNC_FPRS: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 8;
pub const KVM_SYNC_GSCB: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 9;
pub const KVM_SYNC_BPBC: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 10;
pub const KVM_SYNC_ETOKEN: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 11;
pub const KVM_SYNC_DIAG318: ::core::ffi::c_ulong = 1usize as ::core::ffi::c_ulong << 12;

pub const KVM_SYNC_S390_VALID_FIELDS: ::core::ffi::c_ulong = KVM_SYNC_PREFIX
    | KVM_SYNC_GPRS
    | KVM_SYNC_ACRS
    | KVM_SYNC_CRS
    | KVM_SYNC_ARCH0
    | KVM_SYNC_PFAULT
    | KVM_SYNC_VRS
    | KVM_SYNC_RICCB
    | KVM_SYNC_FPRS
    | KVM_SYNC_GSCB
    | KVM_SYNC_BPBC
    | KVM_SYNC_ETOKEN
    | KVM_SYNC_DIAG318;

/* length and alignment of the sdnx as a power of two */
pub const SDNXC: usize = 8;
pub const SDNXL: usize = 1usize << SDNXC;

/* definition of registers in kvm_run */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
    pub prefix: __u64, /* prefix register */
    pub gprs: [__u64; 16], /* general purpose registers */
    pub acrs: [__u32; 16], /* access registers */
    pub crs: [__u64; 16], /* control registers */
    pub todpr: __u64, /* tod programmable register [ARCH0] */
    pub cputm: __u64, /* cpu timer [ARCH0] */
    pub ckc: __u64, /* clock comparator [ARCH0] */
    pub pp: __u64, /* program parameter [ARCH0] */
    pub gbea: __u64, /* guest breaking-event address [ARCH0] */
    pub pft: __u64, /* pfault token [PFAULT] */
    pub pfs: __u64, /* pfault select [PFAULT] */
    pub pfc: __u64, /* pfault compare [PFAULT] */
    pub vrs_fprs: kvm_sync_regs__bindgen_ty_1,
    pub reserved: [__u8; 512], /* for future vector expansion */
    pub fpc: __u32, /* valid on KVM_SYNC_VRS or KVM_SYNC_FPRS */
    /* C bitfields in one __u8 storage unit: bpbc: 1, reserved2: 7. */
    pub _bitfield_1: __u8,
    pub padding1: [__u8; 51], /* riccb needs to be 64byte aligned */
    pub riccb: [__u8; 64], /* runtime instrumentation controls block */
    pub diag318: __u64, /* diagnose 0x318 info */
    pub padding2: [__u8; 184], /* sdnx needs to be 256byte aligned */
    pub u: kvm_sync_regs__bindgen_ty_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_sync_regs__bindgen_ty_1 {
    pub vrs: [[__u64; 2]; 32], /* vector registers (KVM_SYNC_VRS) */
    pub fprs: [__u64; 16], /* fp registers (KVM_SYNC_FPRS) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_sync_regs__bindgen_ty_2 {
    pub sdnx: [__u8; SDNXL], /* state description annex */
    pub gscb: kvm_sync_regs__bindgen_ty_2__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs__bindgen_ty_2__bindgen_ty_1 {
    pub reserved1: [__u64; 2],
    pub gscb: [__u64; 4],
    pub etoken: __u64,
    pub etoken_extension: __u64,
}

pub const KVM_REG_S390_TODPR: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U32 | 0x1;
pub const KVM_REG_S390_EPOCHDIFF: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x2;
pub const KVM_REG_S390_CPU_TIMER: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x3;
pub const KVM_REG_S390_CLOCK_COMP: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x4;
pub const KVM_REG_S390_PFTOKEN: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x5;
pub const KVM_REG_S390_PFCOMPARE: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x6;
pub const KVM_REG_S390_PFSELECT: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x7;
pub const KVM_REG_S390_PP: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x8;
pub const KVM_REG_S390_GBEA: __u64 = KVM_REG_S390 | KVM_REG_SIZE_U64 | 0x9;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
