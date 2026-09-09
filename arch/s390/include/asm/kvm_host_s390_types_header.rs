/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const KVM_S390_BSCA_CPU_SLOTS: usize = 64;
pub const KVM_S390_ESCA_CPU_SLOTS: usize = 248;
pub const SCB_ALIGNMENT_SHIFT: u32 = 9;
pub const SIGP_CTRL_C: u8 = 0x80;
pub const SIGP_CTRL_SCN_MASK: u8 = 0x3f;

#[repr(C)] pub union bsca_sigp_ctrl { pub value: u8, pub bits: bsca_sigp_ctrl_bits }
#[repr(C)] pub struct bsca_sigp_ctrl_bits { pub c: u8, pub r: u8, pub scn: u8 }
#[repr(C)] pub union esca_sigp_ctrl { pub value: u16, pub bits: esca_sigp_ctrl_bits }
#[repr(C)] pub struct esca_sigp_ctrl_bits { pub c: u8, pub reserved: u8, pub scn: u8 }

#[repr(C)] pub struct esca_entry { pub sigp_ctrl: esca_sigp_ctrl, pub reserved1: [u16; 3], pub sda: u64, pub reserved2: [u64; 6] }
#[repr(C)] pub struct bsca_entry { pub reserved0: u8, pub sigp_ctrl: bsca_sigp_ctrl, pub reserved: [u16; 3], pub sda: u64, pub reserved2: [u64; 2] }

#[repr(C)] pub union ipte_control { pub val: usize, pub bits: ipte_control_bits }
#[repr(C)] pub struct ipte_control_bits { pub k: usize, pub kh: usize, pub kg: usize }
#[repr(C)] pub union sca_utility { pub val: u32, pub bits: sca_utility_bits }
#[repr(C)] pub struct sca_utility_bits { pub mtcr: u32, pub reserved: u32 }

#[repr(C)] pub struct bsca_block { pub ipte_control: ipte_control, pub reserved: [u64; 5], pub mcn: u64, pub utility: sca_utility, pub reserved2: [u8; 4], pub cpu: [bsca_entry; KVM_S390_BSCA_CPU_SLOTS] }
#[repr(C)] pub struct esca_block { pub ipte_control: ipte_control, pub reserved1: [u64; 6], pub utility: sca_utility, pub reserved2: [u8; 4], pub mcn: [u64; 4], pub reserved3: [u64; 20], pub cpu: [esca_entry; KVM_S390_ESCA_CPU_SLOTS] }

#[repr(C)] pub struct mcck_volatile_info { pub mcic: u64, pub failing_storage_address: u64, pub ext_damage_code: u32, pub reserved: u32 }

pub const SIDAD_SIZE_MASK: u64 = 0xff;
/* sida_addr and sida_size retain the original phys_to_virt/PAGE_SIZE dependencies. */
#[inline] pub unsafe fn sida_addr(sie_block: *const kvm_s390_sie_block) -> *mut core::ffi::c_void { phys_to_virt((*sie_block).sidad & !(PAGE_SIZE as u64 - 1)) }
#[inline] pub unsafe fn sida_size(sie_block: *const kvm_s390_sie_block) -> usize { (((*sie_block).sidad & SIDAD_SIZE_MASK) + 1) as usize * PAGE_SIZE }

pub const CPUSTAT_STOPPED: u32 = 0x80000000; pub const CPUSTAT_WAIT: u32 = 0x10000000; pub const CPUSTAT_ECALL_PEND: u32 = 0x08000000; pub const CPUSTAT_STOP_INT: u32 = 0x04000000; pub const CPUSTAT_IO_INT: u32 = 0x02000000; pub const CPUSTAT_EXT_INT: u32 = 0x01000000; pub const CPUSTAT_RUNNING: u32 = 0x00800000; pub const CPUSTAT_RETAINED: u32 = 0x00400000; pub const CPUSTAT_TIMING_SUB: u32 = 0x00020000; pub const CPUSTAT_SIE_SUB: u32 = 0x00010000; pub const CPUSTAT_RRF: u32 = 0x00008000; pub const CPUSTAT_SLSV: u32 = 0x00004000; pub const CPUSTAT_SLSR: u32 = 0x00002000; pub const CPUSTAT_ZARCH: u32 = 0x00000800; pub const CPUSTAT_MCDS: u32 = 0x00000100; pub const CPUSTAT_KSS: u32 = 0x00000200; pub const CPUSTAT_SM: u32 = 0x80; pub const CPUSTAT_IBS: u32 = 0x40; pub const CPUSTAT_GED2: u32 = 0x10; pub const CPUSTAT_G: u32 = 8; pub const CPUSTAT_GED: u32 = 4; pub const CPUSTAT_J: u32 = 2; pub const CPUSTAT_P: u32 = 1;

/* The C bitfields below are represented by their containing words; masks preserve their intent. */
#[repr(C, packed, align(512))]
pub struct kvm_s390_sie_block {
 pub cpuflags: atomic_t, pub prefix_ibc: u32, pub reserved08: [u8;4], pub prog0c: u32,
 pub pv: sie_pv, pub prog20: atomic_t, pub reserved24: [u8;4], pub cputm:u64, pub ckc:u64, pub epoch:u64, pub svcc:u32, pub lctl:u16, pub icpua:i16, pub ictl:u32, pub eca:u32, pub icptcode:u8, pub icptstatus:u8, pub ihcpu:u16, pub reserved54:u8, pub iictl:u8, pub ipa:u16, pub ipb:u32, pub scaoh:u32, pub fpf:u8, pub ecb:u8, pub ecb2:u8, pub ecb3:u8, pub scaol:u32, pub sdf:u8, pub epdx:u8, pub cpnc:u8, pub reserved6b:u8, pub todpr:u32, pub gd:u32, pub reserved74:[u8;12], pub mso:u64, pub msl:u64, pub gpsw:psw_t, pub gg14:u64, pub gg15:u64, pub reservedb0:[u8;8], pub hpid:u8, pub reservedb9:[u8;7], pub eiparams:u32, pub extcpuaddr:u16, pub eic:u16, pub reservedc8:u32, pub pgmilc:u16, pub iprcc:u16, pub dxc:u32, pub mcn:u16, pub perc:u8, pub peratmid:u8, pub peraddr:u64, pub eai:u8, pub peraid:u8, pub oai:u8, pub armid:u8, pub reservede4:[u8;4], pub tecmc:u64, pub reservedf4:[u8;8], pub crycbd:u32, pub gcr:[u64;16], pub gbea_sidad:u64, pub reserved188:[u8;8], pub sdnxo:u64, pub reserved198:[u8;8], pub fac:u32, pub reserved1a4:[u8;20], pub cbrlo:u64, pub reserved1c0:[u8;8], pub ecd:u32, pub reserved1cc:[u8;18], pub pp:u64, pub reserved1e6:[u8;2], pub itdba:u64, pub riccbd:u64, pub gvrd:u64,
}
#[repr(C)] pub union sie_pv { pub reserved10:[u8;16], pub handles:sie_pv_handles }
#[repr(C)] pub struct sie_pv_handles { pub pv_handle_cpu:u64, pub pv_handle_config:u64 }

#[repr(C)] pub struct kvm_s390_itdb { pub data:[u8;256] }
#[repr(C)] pub struct sie_page { pub sie_block:kvm_s390_sie_block, pub mcck_info:mcck_volatile_info, pub reserved218:[u8;360], pub pv_grregs:[u64;16], pub reserved400:[u8;512], pub itdb:kvm_s390_itdb, pub reserved700:[u8;2304] }
pub const PROG_IN_SIE:u32=1<<0; pub const PROG_BLOCK_SIE:u32=1<<0; pub const PROG_REQUEST:u32=1<<1;
pub const LCTL_CR0:u16=0x8000; pub const LCTL_CR6:u16=0x0200; pub const LCTL_CR9:u16=0x0040; pub const LCTL_CR10:u16=0x0020; pub const LCTL_CR11:u16=0x0010; pub const LCTL_CR14:u16=0x0002;
pub const ICTL_OPEREXC:u32=0x80000000; pub const ICTL_PINT:u32=0x20000000; pub const ICTL_LPSW:u32=0x00400000; pub const ICTL_STCTL:u32=0x00040000; pub const ICTL_ISKE:u32=0x00004000; pub const ICTL_SSKE:u32=0x00002000; pub const ICTL_RRBE:u32=0x00001000; pub const ICTL_TPROT:u32=0x00000200;
pub const ECA_CEI:u32=0x80000000; pub const ECA_IB:u32=0x40000000; pub const ECA_SIGPI:u32=0x10000000; pub const ECA_MVPGI:u32=0x01000000; pub const ECA_AIV:u32=0x00200000; pub const ECA_VX:u32=0x00020000; pub const ECA_PROTEXCI:u32=0x00002000; pub const ECA_APIE:u32=8; pub const ECA_SII:u32=1;
pub const ICPT_INST:u8=4; pub const ICPT_PROGI:u8=8; pub const ICPT_INSTPROGI:u8=0x0c; pub const ICPT_EXTREQ:u8=0x10; pub const ICPT_EXTINT:u8=0x14; pub const ICPT_IOREQ:u8=0x18; pub const ICPT_WAIT:u8=0x1c; pub const ICPT_VALIDITY:u8=0x20; pub const ICPT_STOP:u8=0x28; pub const ICPT_OPEREXC:u8=0x2c; pub const ICPT_PARTEXEC:u8=0x38; pub const ICPT_IOINST:u8=0x40; pub const ICPT_KSS:u8=0x5c; pub const ICPT_MCHKREQ:u8=0x60; pub const ICPT_INT_ENABLE:u8=0x64; pub const ICPT_PV_INSTR:u8=0x68; pub const ICPT_PV_NOTIFY:u8=0x6c; pub const ICPT_PV_PREF:u8=0x70;
pub const IICTL_CODE_NONE:u8=0; pub const IICTL_CODE_MCHK:u8=1; pub const IICTL_CODE_EXT:u8=2; pub const IICTL_CODE_IO:u8=3; pub const IICTL_CODE_RESTART:u8=4; pub const IICTL_CODE_SPECIFICATION:u8=0x10; pub const IICTL_CODE_OPERAND:u8=0x11;
pub const FPF_BPBC:u8=0x20; pub const ECB_GS:u8=0x40; pub const ECB_TE:u8=0x10; pub const ECB_SPECI:u8=8; pub const ECB_SRSI:u8=4; pub const ECB_HOSTPROTINT:u8=2; pub const ECB_PTF:u8=1; pub const ECB2_CMMA:u8=0x80; pub const ECB2_IEP:u8=0x20; pub const ECB2_PFMFI:u8=8; pub const ECB2_ESCA:u8=4; pub const ECB2_ZPCI_LSI:u8=2; pub const ECB3_AISI:u8=0x20; pub const ECB3_AISII:u8=0x10; pub const ECB3_DEA:u8=8; pub const ECB3_AES:u8=4; pub const ECB3_RI:u8=1;
pub const ESCA_SCAOL_MASK:u32=!0x3f; pub const GISA_FORMAT1:u32=1; pub const HPID_KVM:u8=4; pub const HPID_VSIE:u8=5; pub const CRYCB_FORMAT_MASK:u32=3; pub const CRYCB_FORMAT0:u32=0; pub const CRYCB_FORMAT1:u32=1; pub const CRYCB_FORMAT2:u32=3; pub const ECD_HOSTREGMGMT:u32=0x20000000; pub const ECD_MEF:u32=0x08000000; pub const ECD_ETOKENF:u32=0x02000000; pub const ECD_ECC:u32=0x00200000; pub const ECD_HMAC:u32=0x00004000;
pub const CR0_INITIAL_MASK: u64 = CR0_UNUSED_56 | CR0_INTERRUPT_KEY_SUBMASK | CR0_MEASUREMENT_ALERT_SUBMASK;
pub const CR14_INITIAL_MASK: u64 = CR14_UNUSED_32 | CR14_UNUSED_33 | CR14_EXTERNAL_DAMAGE_SUBMASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
