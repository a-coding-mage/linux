/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright(c) 2004 - 2009 Intel Corporation. All rights reserved. */

pub const IOAT_MMIO_BAR: u32 = 0;

pub const PCI_DEVICE_ID_INTEL_IOAT_IVB0: u32 = 0x0e20;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB1: u32 = 0x0e21;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB2: u32 = 0x0e22;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB3: u32 = 0x0e23;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB4: u32 = 0x0e24;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB5: u32 = 0x0e25;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB6: u32 = 0x0e26;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB7: u32 = 0x0e27;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB8: u32 = 0x0e2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB9: u32 = 0x0e2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW0: u32 = 0x2f20;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW1: u32 = 0x2f21;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW2: u32 = 0x2f22;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW3: u32 = 0x2f23;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW4: u32 = 0x2f24;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW5: u32 = 0x2f25;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW6: u32 = 0x2f26;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW7: u32 = 0x2f27;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW8: u32 = 0x2f2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW9: u32 = 0x2f2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD0: u32 = 0x0c50;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD1: u32 = 0x0c51;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD2: u32 = 0x0c52;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD3: u32 = 0x0c53;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE0: u32 = 0x6f50;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE1: u32 = 0x6f51;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE2: u32 = 0x6f52;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE3: u32 = 0x6f53;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX0: u32 = 0x6f20;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX1: u32 = 0x6f21;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX2: u32 = 0x6f22;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX3: u32 = 0x6f23;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX4: u32 = 0x6f24;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX5: u32 = 0x6f25;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX6: u32 = 0x6f26;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX7: u32 = 0x6f27;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX8: u32 = 0x6f2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX9: u32 = 0x6f2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_SKX: u32 = 0x2021;
pub const PCI_DEVICE_ID_INTEL_IOAT_ICX: u32 = 0x0b00;
pub const IOAT_VER_1_2: u32 = 0x12;
pub const IOAT_VER_2_0: u32 = 0x20;
pub const IOAT_VER_3_0: u32 = 0x30;
pub const IOAT_VER_3_2: u32 = 0x32;
pub const IOAT_VER_3_3: u32 = 0x33;
pub const IOAT_VER_3_4: u32 = 0x34;
pub const IOAT_DESC_SZ: usize = 64;

// C bit-field members are represented by their containing 32-bit word.
#[repr(C)] pub union ioat_dma_descriptor_ctl { pub ctl: u32, pub ctl_f: u32 }
#[repr(C)] pub union ioat_xor_descriptor_ctl { pub ctl: u32, pub ctl_f: u32 }
#[repr(C)] pub union ioat_pq_descriptor_dwbes { pub size: u32, pub dwbes: u32, pub dwbes_f: u32 }
#[repr(C)] pub union ioat_pq_descriptor_ctl { pub ctl: u32, pub ctl_f: u32 }
#[repr(C)] pub union ioat_pq_update_descriptor_ctl { pub ctl: u32, pub ctl_f: u32 }

pub const IOAT_OP_COPY: u32 = 0x00;
pub const IOAT_OP_XOR: u32 = 0x87;
pub const IOAT_OP_XOR_VAL: u32 = 0x88;
pub const IOAT_OP_PQ: u32 = 0x89;
pub const IOAT_OP_PQ_VAL: u32 = 0x8a;
pub const IOAT_OP_PQ_16S: u32 = 0xa0;
pub const IOAT_OP_PQ_VAL_16S: u32 = 0xa1;
pub const IOAT_OP_PQ_UP: u32 = 0x8b;

#[repr(C)] pub struct ioat_dma_descriptor { pub size: u32, pub ctl: ioat_dma_descriptor_ctl, pub src_addr: u64, pub dst_addr: u64, pub next: u64, pub rsv1: u64, pub rsv2: u64, pub user1: u64, pub user2: u64 }
#[repr(C)] pub struct ioat_xor_descriptor { pub size: u32, pub ctl: ioat_xor_descriptor_ctl, pub src_addr: u64, pub dst_addr: u64, pub next: u64, pub src_addr2: u64, pub src_addr3: u64, pub src_addr4: u64, pub src_addr5: u64 }
#[repr(C)] pub struct ioat_xor_ext_descriptor { pub src_addr6: u64, pub src_addr7: u64, pub src_addr8: u64, pub next: u64, pub rsvd: [u64; 4] }
#[repr(C)] pub struct ioat_pq_descriptor { pub dwbes: ioat_pq_descriptor_dwbes, pub ctl: ioat_pq_descriptor_ctl, pub src_addr: u64, pub p_addr: u64, pub next: u64, pub src_addr2: u64, pub src_addr3: u64, pub coef: [u8; 8], pub q_addr: u64 }
#[repr(C)] pub struct ioat_pq_ext_descriptor { pub src_addr4: u64, pub src_addr5: u64, pub src_addr6: u64, pub next: u64, pub src_addr7: u64, pub src_addr8: u64, pub rsvd: [u64; 2] }
#[repr(C)] pub struct ioat_pq_update_descriptor { pub size: u32, pub ctl: ioat_pq_update_descriptor_ctl, pub src_addr: u64, pub p_addr: u64, pub next: u64, pub src_addr2: u64, pub p_src: u64, pub q_src: u64, pub q_addr: u64 }
#[repr(C)] pub struct ioat_raw_descriptor { pub field: [u64; 8] }
#[repr(C)] pub struct ioat_pq16a_descriptor { pub coef: [u8; 8], pub src_addr3: u64, pub src_addr4: u64, pub src_addr5: u64, pub src_addr6: u64, pub src_addr7: u64, pub src_addr8: u64, pub src_addr9: u64 }
#[repr(C)] pub struct ioat_pq16b_descriptor { pub src_addr10: u64, pub src_addr11: u64, pub src_addr12: u64, pub src_addr13: u64, pub src_addr14: u64, pub src_addr15: u64, pub src_addr16: u64, pub rsvd: u64 }
#[repr(C)] pub union ioat_sed_pq_descriptor { pub a: ioat_pq16a_descriptor, pub b: ioat_pq16b_descriptor }
pub const SED_SIZE: usize = 64;
#[repr(C)] pub struct ioat_sed_raw_descriptor { pub a: [u64; 8], pub b: [u64; 8], pub c: [u64; 8] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
