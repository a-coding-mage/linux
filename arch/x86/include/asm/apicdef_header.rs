/* SPDX-License-Identifier: GPL-2.0 */

/* Constants for various Intel APICs (local APIC, IOAPIC, etc.). */

pub const IO_APIC_DEFAULT_PHYS_BASE: u32 = 0xfec00000;
pub const APIC_DEFAULT_PHYS_BASE: u32 = 0xfee00000;
pub const IO_APIC_SLOT_SIZE: u32 = 1024;

pub const APIC_DELIVERY_MODE_FIXED: u32 = 0;
pub const APIC_DELIVERY_MODE_LOWESTPRIO: u32 = 1;
pub const APIC_DELIVERY_MODE_SMI: u32 = 2;
pub const APIC_DELIVERY_MODE_NMI: u32 = 4;
pub const APIC_DELIVERY_MODE_INIT: u32 = 5;
pub const APIC_DELIVERY_MODE_EXTINT: u32 = 7;

pub const APIC_ID: u32 = 0x20;
pub const APIC_LVR: u32 = 0x30;
pub const APIC_LVR_MASK: u32 = 0xFF00FF;
pub const APIC_LVR_DIRECTED_EOI: u32 = 1 << 24;
macro_rules! GET_APIC_VERSION { ($x:expr) => (($x) & 0xFFu32); }
macro_rules! GET_APIC_MAXLVT { ($x:expr) => ((($x) >> 16) & 0xFFu32); }
#[cfg(CONFIG_X86_32)]
macro_rules! APIC_INTEGRATED { ($x:expr) => (($x) & 0xF0u32); }
#[cfg(not(CONFIG_X86_32))]
macro_rules! APIC_INTEGRATED { ($x:expr) => (1); }
macro_rules! APIC_XAPIC { ($x:expr) => (($x) >= 0x14); }
macro_rules! APIC_EXT_SPACE { ($x:expr) => (($x) & 0x80000000); }
pub const APIC_TASKPRI: u32 = 0x80;
pub const APIC_TPRI_MASK: u32 = 0xFF;
pub const APIC_ARBPRI: u32 = 0x90;
pub const APIC_ARBPRI_MASK: u32 = 0xFF;
pub const APIC_PROCPRI: u32 = 0xA0;
pub const APIC_EOI: u32 = 0xB0;
pub const APIC_EOI_ACK: u32 = 0x0;
pub const APIC_RRR: u32 = 0xC0;
pub const APIC_LDR: u32 = 0xD0;
pub const APIC_LDR_MASK: u32 = 0xFF << 24;
macro_rules! GET_APIC_LOGICAL_ID { ($x:expr) => ((($x) >> 24) & 0xFFu32); }
macro_rules! SET_APIC_LOGICAL_ID { ($x:expr) => (($x) << 24); }
pub const APIC_ALL_CPUS: u32 = 0xFF;
pub const APIC_DFR: u32 = 0xE0;
pub const APIC_DFR_CLUSTER: u32 = 0x0FFFFFFF;
pub const APIC_DFR_FLAT: u32 = 0xFFFFFFFF;
pub const APIC_SPIV: u32 = 0xF0;
pub const APIC_SPIV_DIRECTED_EOI: u32 = 1 << 12;
pub const APIC_SPIV_FOCUS_DISABLED: u32 = 1 << 9;
pub const APIC_SPIV_APIC_ENABLED: u32 = 1 << 8;
pub const APIC_ISR: u32 = 0x100;
pub const APIC_ISR_NR: u32 = 0x8;
pub const APIC_TMR: u32 = 0x180;
pub const APIC_IRR: u32 = 0x200;
pub const APIC_ESR: u32 = 0x280;
pub const APIC_ESR_SEND_CS: u32 = 0x00001;
pub const APIC_ESR_RECV_CS: u32 = 0x00002;
pub const APIC_ESR_SEND_ACC: u32 = 0x00004;
pub const APIC_ESR_RECV_ACC: u32 = 0x00008;
pub const APIC_ESR_SENDILL: u32 = 0x00020;
pub const APIC_ESR_RECVILL: u32 = 0x00040;
pub const APIC_ESR_ILLREGA: u32 = 0x00080;
pub const APIC_LVTCMCI: u32 = 0x2f0;
pub const APIC_ICR: u32 = 0x300;
pub const APIC_DEST_SELF: u32 = 0x40000;
pub const APIC_DEST_ALLINC: u32 = 0x80000;
pub const APIC_DEST_ALLBUT: u32 = 0xC0000;
pub const APIC_ICR_RR_MASK: u32 = 0x30000;
pub const APIC_ICR_RR_INVALID: u32 = 0x00000;
pub const APIC_ICR_RR_INPROG: u32 = 0x10000;
pub const APIC_ICR_RR_VALID: u32 = 0x20000;
pub const APIC_INT_LEVELTRIG: u32 = 0x08000;
pub const APIC_INT_ASSERT: u32 = 0x04000;
pub const APIC_ICR_BUSY: u32 = 0x01000;
pub const APIC_DEST_LOGICAL: u32 = 0x00800;
pub const APIC_DEST_PHYSICAL: u32 = 0x00000;
pub const APIC_DM_FIXED: u32 = 0x00000;
pub const APIC_DM_FIXED_MASK: u32 = 0x00700;
pub const APIC_DM_LOWEST: u32 = 0x00100;
pub const APIC_DM_SMI: u32 = 0x00200;
pub const APIC_DM_REMRD: u32 = 0x00300;
pub const APIC_DM_NMI: u32 = 0x00400;
pub const APIC_DM_INIT: u32 = 0x00500;
pub const APIC_DM_STARTUP: u32 = 0x00600;
pub const APIC_DM_EXTINT: u32 = 0x00700;
pub const APIC_VECTOR_MASK: u32 = 0x000FF;
pub const APIC_ICR2: u32 = 0x310;
macro_rules! GET_XAPIC_DEST_FIELD { ($x:expr) => ((($x) >> 24) & 0xFF); }
macro_rules! SET_XAPIC_DEST_FIELD { ($x:expr) => (($x) << 24); }
pub const APIC_LVTT: u32 = 0x320;
pub const APIC_LVTTHMR: u32 = 0x330;
pub const APIC_LVTPC: u32 = 0x340;
pub const APIC_LVT0: u32 = 0x350;
pub const APIC_LVT_TIMER_ONESHOT: u32 = 0 << 17;
pub const APIC_LVT_TIMER_PERIODIC: u32 = 1 << 17;
pub const APIC_LVT_TIMER_TSCDEADLINE: u32 = 2 << 17;
pub const APIC_LVT_MASKED: u32 = 1 << 16;
pub const APIC_LVT_LEVEL_TRIGGER: u32 = 1 << 15;
pub const APIC_LVT_REMOTE_IRR: u32 = 1 << 14;
pub const APIC_INPUT_POLARITY: u32 = 1 << 13;
pub const APIC_SEND_PENDING: u32 = 1 << 12;
pub const APIC_MODE_MASK: u32 = 0x700;
macro_rules! GET_APIC_DELIVERY_MODE { ($x:expr) => ((($x) >> 8) & 0x7); }
macro_rules! SET_APIC_DELIVERY_MODE { ($x:expr, $y:expr) => ((($x) & !0x700) | (($y) << 8)); }
pub const APIC_MODE_FIXED: u32 = 0x0;
pub const APIC_MODE_NMI: u32 = 0x4;
pub const APIC_MODE_EXTINT: u32 = 0x7;
pub const APIC_LVT1: u32 = 0x360;
pub const APIC_LVTERR: u32 = 0x370;
pub const APIC_TMICT: u32 = 0x380;
pub const APIC_TMCCT: u32 = 0x390;
pub const APIC_TDCR: u32 = 0x3E0;
pub const APIC_SELF_IPI: u32 = 0x3F0;
pub const APIC_TDR_DIV_TMBASE: u32 = 1 << 2;
pub const APIC_TDR_DIV_1: u32 = 0xB;
pub const APIC_TDR_DIV_2: u32 = 0x0;
pub const APIC_TDR_DIV_4: u32 = 0x1;
pub const APIC_TDR_DIV_8: u32 = 0x2;
pub const APIC_TDR_DIV_16: u32 = 0x3;
pub const APIC_TDR_DIV_32: u32 = 0x8;
pub const APIC_TDR_DIV_64: u32 = 0x9;
pub const APIC_TDR_DIV_128: u32 = 0xA;
pub const APIC_EFEAT: u32 = 0x400;
pub const APIC_ECTRL: u32 = 0x410;
pub const APIC_SEOI: u32 = 0x420;
pub const APIC_IER: u32 = 0x480;
macro_rules! APIC_EILVTn { ($n:expr) => (0x500 + 0x10 * ($n)); }
pub const APIC_EILVT_NR_AMD_10H: u32 = 4;
pub const APIC_EILVT_NR_MAX: u32 = APIC_EILVT_NR_AMD_10H;

macro_rules! APIC_BASE { () => (fix_to_virt(FIX_APIC_BASE)); }
pub const APIC_BASE_MSR: u32 = 0x800;
pub const APIC_X2APIC_ID_MSR: u32 = 0x802;
macro_rules! XAPIC_ENABLE { () => (BIT(11)); }
macro_rules! X2APIC_ENABLE { () => (BIT(10)); }

#[cfg(CONFIG_X86_32)]
pub const MAX_IO_APICS: u32 = 64;
#[cfg(CONFIG_X86_32)]
pub const MAX_LOCAL_APIC: u32 = 256;
#[cfg(not(CONFIG_X86_32))]
pub const MAX_IO_APICS: u32 = 128;
#[cfg(not(CONFIG_X86_32))]
pub const MAX_LOCAL_APIC: u32 = 32768;

pub const XAPIC_DEST_CPUS_SHIFT: u32 = 4;
pub const XAPIC_DEST_CPUS_MASK: u32 = (1u32 << XAPIC_DEST_CPUS_SHIFT) - 1;
pub const XAPIC_DEST_CLUSTER_MASK: u32 = XAPIC_DEST_CPUS_MASK << XAPIC_DEST_CPUS_SHIFT;
macro_rules! APIC_CLUSTER { ($apicid:expr) => (($apicid) & XAPIC_DEST_CLUSTER_MASK); }
macro_rules! APIC_CLUSTERID { ($apicid:expr) => (APIC_CLUSTER!($apicid) >> XAPIC_DEST_CPUS_SHIFT); }
macro_rules! APIC_CPUID { ($apicid:expr) => (($apicid) & XAPIC_DEST_CPUS_MASK); }
macro_rules! NUM_APIC_CLUSTERS { () => ((BAD_APICID + 1) >> XAPIC_DEST_CPUS_SHIFT); }

#[cfg(CONFIG_X86_32)]
pub const BAD_APICID: u32 = 0xFF;
#[cfg(not(CONFIG_X86_32))]
pub const BAD_APICID: u32 = 0xFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
