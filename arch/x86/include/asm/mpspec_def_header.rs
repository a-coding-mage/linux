/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Structure definitions for SMP machines following the
 * Intel Multiprocessing Specification 1.1 and 1.4.
 */

/* This tag identifies where the SMP configuration information is. */
pub const SMP_MAGIC_IDENT: u32 = ((b'_' as u32) << 24)
    | ((b'P' as u32) << 16)
    | ((b'M' as u32) << 8)
    | (b'_' as u32);

#[cfg(target_pointer_width = "32")]
pub const MAX_MPC_ENTRY: usize = 1024;

/* Intel MP Floating Pointer Structure */
#[repr(C)]
pub struct mpf_intel {
    pub signature: [u8; 4],
    pub physptr: u32,
    pub length: u8,
    pub specification: u8,
    pub checksum: u8,
    pub feature1: u8,
    pub feature2: u8,
    pub feature3: u8,
    pub feature4: u8,
    pub feature5: u8,
}

pub const MPC_SIGNATURE: &[u8; 5] = b"PCMP\0";

#[repr(C)]
pub struct mpc_table {
    pub signature: [u8; 4],
    pub length: u16,
    pub spec: u8,
    pub checksum: u8,
    pub oem: [u8; 8],
    pub productid: [u8; 12],
    pub oemptr: u32,
    pub oemsize: u16,
    pub oemcount: u16,
    pub lapic: u32,
    pub reserved: u32,
}

pub const MP_PROCESSOR: u32 = 0;
pub const MP_BUS: u32 = 1;
pub const MP_IOAPIC: u32 = 2;
pub const MP_INTSRC: u32 = 3;
pub const MP_LINTSRC: u32 = 4;
pub const MP_TRANSLATION: u32 = 192;

pub const CPU_ENABLED: u32 = 1;
pub const CPU_BOOTPROCESSOR: u32 = 2;
pub const CPU_STEPPING_MASK: u32 = 0x000F;
pub const CPU_MODEL_MASK: u32 = 0x00F0;
pub const CPU_FAMILY_MASK: u32 = 0x0F00;

#[repr(C)]
pub struct mpc_cpu {
    pub r#type: u8,
    pub apicid: u8,
    pub apicver: u8,
    pub cpuflag: u8,
    pub cpufeature: u32,
    pub featureflag: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
pub struct mpc_bus {
    pub r#type: u8,
    pub busid: u8,
    pub bustype: [u8; 6],
}

pub const BUSTYPE_EISA: &[u8; 5] = b"EISA\0";
pub const BUSTYPE_ISA: &[u8; 4] = b"ISA\0";
pub const BUSTYPE_INTERN: &[u8; 7] = b"INTERN\0";
pub const BUSTYPE_MCA: &[u8; 4] = b"MCA\0";
pub const BUSTYPE_VL: &[u8; 3] = b"VL\0";
pub const BUSTYPE_PCI: &[u8; 4] = b"PCI\0";
pub const BUSTYPE_PCMCIA: &[u8; 7] = b"PCMCIA\0";
pub const BUSTYPE_CBUS: &[u8; 5] = b"CBUS\0";
pub const BUSTYPE_CBUSII: &[u8; 7] = b"CBUSII\0";
pub const BUSTYPE_FUTURE: &[u8; 7] = b"FUTURE\0";
pub const BUSTYPE_MBI: &[u8; 4] = b"MBI\0";
pub const BUSTYPE_MBII: &[u8; 5] = b"MBII\0";
pub const BUSTYPE_MPI: &[u8; 4] = b"MPI\0";
pub const BUSTYPE_MPSA: &[u8; 5] = b"MPSA\0";
pub const BUSTYPE_NUBUS: &[u8; 6] = b"NUBUS\0";
pub const BUSTYPE_TC: &[u8; 3] = b"TC\0";
pub const BUSTYPE_VME: &[u8; 4] = b"VME\0";
pub const BUSTYPE_XPRESS: &[u8; 7] = b"XPRESS\0";

pub const MPC_APIC_USABLE: u32 = 0x01;

#[repr(C)]
pub struct mpc_ioapic {
    pub r#type: u8,
    pub apicid: u8,
    pub apicver: u8,
    pub flags: u8,
    pub apicaddr: u32,
}

#[repr(C)]
pub struct mpc_intsrc {
    pub r#type: u8,
    pub irqtype: u8,
    pub irqflag: u16,
    pub srcbus: u8,
    pub srcbusirq: u8,
    pub dstapic: u8,
    pub dstirq: u8,
}

#[repr(C)]
pub enum mp_irq_source_types {
    mp_INT = 0,
    mp_NMI = 1,
    mp_SMI = 2,
    mp_ExtINT = 3,
}

pub const MP_IRQPOL_DEFAULT: u32 = 0x0;
pub const MP_IRQPOL_ACTIVE_HIGH: u32 = 0x1;
pub const MP_IRQPOL_RESERVED: u32 = 0x2;
pub const MP_IRQPOL_ACTIVE_LOW: u32 = 0x3;
pub const MP_IRQPOL_MASK: u32 = 0x3;
pub const MP_IRQTRIG_DEFAULT: u32 = 0x0;
pub const MP_IRQTRIG_EDGE: u32 = 0x4;
pub const MP_IRQTRIG_RESERVED: u32 = 0x8;
pub const MP_IRQTRIG_LEVEL: u32 = 0xc;
pub const MP_IRQTRIG_MASK: u32 = 0xc;
pub const MP_APIC_ALL: u32 = 0xFF;

#[repr(C)]
pub struct mpc_lintsrc {
    pub r#type: u8,
    pub irqtype: u8,
    pub irqflag: u16,
    pub srcbusid: u8,
    pub srcbusirq: u8,
    pub destapic: u8,
    pub destapiclint: u8,
}

pub const MPC_OEM_SIGNATURE: &[u8; 5] = b"_OEM\0";

#[repr(C)]
pub struct mpc_oemtable {
    pub signature: [u8; 4],
    pub length: u16,
    pub rev: u8,
    pub checksum: u8,
    pub mpc: [u8; 8],
}

#[repr(C)]
pub enum mp_bustype {
    MP_BUS_ISA = 1,
    MP_BUS_EISA,
    MP_BUS_PCI,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
