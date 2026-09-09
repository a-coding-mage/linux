/*
 * Definitions for the SGI CRIME (CPU, Rendering, Interconnect and Memory
 * Engine)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000 Harald Koerfgen
 */

// Address map
pub const CRIME_BASE: usize = 0x14000000;

#[repr(C)]
pub struct SgiCrime {
    pub id: usize,
    pub control: usize,
    pub istat: usize,
    pub imask: usize,
    pub soft_int: usize,
    pub hard_int: usize,
    pub watchdog: usize,
    pub timer: usize,
    pub cpu_error_addr: usize,
    pub cpu_error_stat: usize,
    pub _pad0: [usize; 54],
    pub mc_ctrl: usize,
    pub bank_ctrl: [usize; 8],
    pub mem_ref_counter: usize,
    pub mem_error_stat: usize,
    pub mem_error_addr: usize,
    pub mem_ecc_syn: usize,
    pub mem_ecc_chk: usize,
    pub mem_ecc_repl: usize,
}

pub const CRIME_ID_MASK: usize = 0xff;
pub const CRIME_ID_IDBITS: usize = 0xf0;
pub const CRIME_ID_IDVALUE: usize = 0xa0;
pub const CRIME_ID_REV: usize = 0x0f;
pub const CRIME_REV_PETTY: usize = 0x00;
pub const CRIME_REV_11: usize = 0x11;
pub const CRIME_REV_13: usize = 0x13;
pub const CRIME_REV_14: usize = 0x14;

pub const CRIME_CONTROL_MASK: usize = 0x3fff;
pub const CRIME_CONTROL_TRITON_SYSADC: usize = 0x2000;
pub const CRIME_CONTROL_CRIME_SYSADC: usize = 0x1000;
pub const CRIME_CONTROL_HARD_RESET: usize = 0x0800;
pub const CRIME_CONTROL_SOFT_RESET: usize = 0x0400;
pub const CRIME_CONTROL_DOG_ENA: usize = 0x0200;
pub const CRIME_CONTROL_ENDIANESS: usize = 0x0100;
pub const CRIME_CONTROL_ENDIAN_BIG: usize = 0x0100;
pub const CRIME_CONTROL_ENDIAN_LITTLE: usize = 0x0000;
pub const CRIME_CONTROL_CQUEUE_HWM: usize = 0x000f;
pub const CRIME_CONTROL_CQUEUE_SHFT: usize = 0;
pub const CRIME_CONTROL_WBUF_HWM: usize = 0x00f0;
pub const CRIME_CONTROL_WBUF_SHFT: usize = 8;

// BIT(n) is supplied by the surrounding kernel environment.
pub const MACE_VID_IN1_INT: usize = 1 << 0;
pub const MACE_VID_IN2_INT: usize = 1 << 1;
pub const MACE_VID_OUT_INT: usize = 1 << 2;
pub const MACE_ETHERNET_INT: usize = 1 << 3;
pub const MACE_SUPERIO_INT: usize = 1 << 4;
pub const MACE_MISC_INT: usize = 1 << 5;
pub const MACE_AUDIO_INT: usize = 1 << 6;
pub const MACE_PCI_BRIDGE_INT: usize = 1 << 7;
pub const MACEPCI_SCSI0_INT: usize = 1 << 8;
pub const MACEPCI_SCSI1_INT: usize = 1 << 9;
pub const MACEPCI_SLOT0_INT: usize = 1 << 10;
pub const MACEPCI_SLOT1_INT: usize = 1 << 11;
pub const MACEPCI_SLOT2_INT: usize = 1 << 12;
pub const MACEPCI_SHARED0_INT: usize = 1 << 13;
pub const MACEPCI_SHARED1_INT: usize = 1 << 14;
pub const MACEPCI_SHARED2_INT: usize = 1 << 15;
pub const CRIME_GBE0_INT: usize = 1 << 16;
pub const CRIME_GBE1_INT: usize = 1 << 17;
pub const CRIME_GBE2_INT: usize = 1 << 18;
pub const CRIME_GBE3_INT: usize = 1 << 19;
pub const CRIME_CPUERR_INT: usize = 1 << 20;
pub const CRIME_MEMERR_INT: usize = 1 << 21;
pub const CRIME_RE_EMPTY_E_INT: usize = 1 << 22;
pub const CRIME_RE_FULL_E_INT: usize = 1 << 23;
pub const CRIME_RE_IDLE_E_INT: usize = 1 << 24;
pub const CRIME_RE_EMPTY_L_INT: usize = 1 << 25;
pub const CRIME_RE_FULL_L_INT: usize = 1 << 26;
pub const CRIME_RE_IDLE_L_INT: usize = 1 << 27;
pub const CRIME_SOFT0_INT: usize = 1 << 28;
pub const CRIME_SOFT1_INT: usize = 1 << 29;
pub const CRIME_SOFT2_INT: usize = 1 << 30;
pub const CRIME_SYSCORERR_INT: usize = CRIME_SOFT2_INT;
pub const CRIME_VICE_INT: usize = 1 << 31;

// Masks for deciding who handles the interrupt
pub const CRIME_MACE_INT_MASK: usize = 0x8f;
pub const CRIME_MACEISA_INT_MASK: usize = 0x70;
pub const CRIME_MACEPCI_INT_MASK: usize = 0xff00;
pub const CRIME_CRIME_INT_MASK: usize = 0xffff0000;

pub const CRIME_DOG_POWER_ON_RESET: usize = 0x00010000;
pub const CRIME_DOG_WARM_RESET: usize = 0x00080000;
pub const CRIME_DOG_TIMEOUT: usize = CRIME_DOG_POWER_ON_RESET | CRIME_DOG_WARM_RESET;
pub const CRIME_DOG_VALUE: usize = 0x00007fff;
pub const CRIME_MASTER_FREQ: usize = 66666500;
pub const CRIME_NS_PER_TICK: usize = 15;
pub const CRIME_CPU_ERROR_ADDR_MASK: usize = 0x3ffffffff;
pub const CRIME_CPU_ERROR_MASK: usize = 0x7;
pub const CRIME_CPU_ERROR_CPU_ILL_ADDR: usize = 0x4;
pub const CRIME_CPU_ERROR_VICE_WRT_PRTY: usize = 0x2;
pub const CRIME_CPU_ERROR_CPU_WRT_PRTY: usize = 0x1;
pub const CRIME_MEM_BANK_CONTROL_MASK: usize = 0x11f;
pub const CRIME_MEM_BANK_CONTROL_ADDR: usize = 0x01f;
pub const CRIME_MEM_BANK_CONTROL_SDRAM_SIZE: usize = 0x100;
pub const CRIME_MAXBANKS: usize = 8;
pub const CRIME_MEM_REF_COUNTER_MASK: usize = 0x3ff;
pub const CRIME_MEM_ERROR_STAT_MASK: usize = 0x0ff7ffff;
pub const CRIME_MEM_ERROR_MACE_ID: usize = 0x0000007f;
pub const CRIME_MEM_ERROR_MACE_ACCESS: usize = 0x00000080;
pub const CRIME_MEM_ERROR_RE_ID: usize = 0x00007f00;
pub const CRIME_MEM_ERROR_RE_ACCESS: usize = 0x00008000;
pub const CRIME_MEM_ERROR_GBE_ACCESS: usize = 0x00010000;
pub const CRIME_MEM_ERROR_VICE_ACCESS: usize = 0x00020000;
pub const CRIME_MEM_ERROR_CPU_ACCESS: usize = 0x00040000;
pub const CRIME_MEM_ERROR_RESERVED: usize = 0x00080000;
pub const CRIME_MEM_ERROR_SOFT_ERR: usize = 0x00100000;
pub const CRIME_MEM_ERROR_HARD_ERR: usize = 0x00200000;
pub const CRIME_MEM_ERROR_MULTIPLE: usize = 0x00400000;
pub const CRIME_MEM_ERROR_ECC: usize = 0x01800000;
pub const CRIME_MEM_ERROR_MEM_ECC_RD: usize = 0x00800000;
pub const CRIME_MEM_ERROR_MEM_ECC_RMW: usize = 0x01000000;
pub const CRIME_MEM_ERROR_INV: usize = 0x0e000000;
pub const CRIME_MEM_ERROR_INV_MEM_ADDR_RD: usize = 0x02000000;
pub const CRIME_MEM_ERROR_INV_MEM_ADDR_WR: usize = 0x04000000;
pub const CRIME_MEM_ERROR_INV_MEM_ADDR_RMW: usize = 0x08000000;
pub const CRIME_MEM_ERROR_ADDR_MASK: usize = 0x3fffffff;
pub const CRIME_MEM_ERROR_ECC_SYN_MASK: usize = 0xffffffff;
pub const CRIME_MEM_ERROR_ECC_CHK_MASK: usize = 0xffffffff;
pub const CRIME_MEM_ERROR_ECC_REPL_MASK: usize = 0xffffffff;

unsafe extern "C" {
    pub static mut crime: *mut SgiCrime;
}

pub const CRIME_HI_MEM_BASE: usize = 0x40000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
