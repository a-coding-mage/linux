/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Firmware-Assisted Dump support on POWER platform (OPAL).
 *
 * Copyright 2019, Hari Bathini, IBM Corporation.
 */

// Dependency: asm/reg.h supplies SPRN_CTR, SPRN_LR, SPRN_XER, SPRN_DAR, and
// SPRN_DSISR. The C header guard is intentionally omitted.

/*
 * With kernel & initrd loaded at 512MB (with 256MB size), enforce a minimum
 * boot memory size of 768MB to ensure f/w loading kernel and initrd doesn't
 * mess with crash'ed kernel's memory during MPIPL.
 */
pub const OPAL_FADUMP_MIN_BOOT_MEM: ::core::ffi::c_ulong = 0x30000000;

/* OPAL FADump metadata structure format version */
pub const OPAL_FADUMP_VERSION: u8 = 0x1;

/*
 * OPAL FAdump kernel metadata
 *
 * The address of this structure will be registered with f/w for retrieving
 * in the capture kernel to process the crash dump.
 */
#[repr(C, packed)]
pub struct opal_fadump_mem_struct {
    pub version: u8,
    pub reserved: [u8; 3],
    pub region_cnt: __be16,
    pub registered_regions: __be16,
    pub fadumphdr_addr: __be64,
    pub rgn: [opal_mpipl_region; FADUMP_MAX_MEM_REGS],
}

/* CPU state data */
pub const HDAT_FADUMP_CPU_DATA_VER: u8 = 1;
pub const HDAT_FADUMP_CORE_INACTIVE: u8 = 0x0F;

/* HDAT thread header for register entries */
#[repr(C, packed)]
pub struct hdat_fadump_thread_hdr {
    pub pir: __be32,
    /* 0x00 - 0x0F - The corresponding stop state of the core */
    pub core_state: u8,
    pub reserved: [u8; 3],
    pub offset: __be32,
    pub ecnt: __be32,
    pub esize: __be32,
    pub eactsz: __be32,
}

/* Register types populated by f/w */
pub const HDAT_FADUMP_REG_TYPE_GPR: u32 = 0x01;
pub const HDAT_FADUMP_REG_TYPE_SPR: u32 = 0x02;

/* ID numbers used by f/w while populating certain registers */
pub const HDAT_FADUMP_REG_ID_NIP: u32 = 0x7D0;
pub const HDAT_FADUMP_REG_ID_MSR: u32 = 0x7D1;
pub const HDAT_FADUMP_REG_ID_CCR: u32 = 0x7D2;

/* HDAT register entry. */
#[repr(C, packed)]
pub struct hdat_fadump_reg_entry {
    pub reg_type: __be32,
    pub reg_num: __be32,
    pub reg_val: __be64,
}

pub unsafe fn opal_fadump_set_regval_regnum(
    regs: *mut pt_regs,
    reg_type: u32,
    reg_num: u32,
    reg_val: u64,
) {
    if reg_type == HDAT_FADUMP_REG_TYPE_GPR {
        if reg_num < 32 {
            (*regs).gpr[reg_num as usize] = reg_val;
        }
        return;
    }

    match reg_num {
        SPRN_CTR => (*regs).ctr = reg_val,
        SPRN_LR => (*regs).link = reg_val,
        SPRN_XER => (*regs).xer = reg_val,
        SPRN_DAR => (*regs).dar = reg_val,
        SPRN_DSISR => (*regs).dsisr = reg_val,
        HDAT_FADUMP_REG_ID_NIP => (*regs).nip = reg_val,
        HDAT_FADUMP_REG_ID_MSR => (*regs).msr = reg_val,
        HDAT_FADUMP_REG_ID_CCR => (*regs).ccr = reg_val,
        _ => {}
    }
}

pub unsafe fn opal_fadump_read_regs(
    mut bufp: *mut ::core::ffi::c_char,
    regs_cnt: ::core::ffi::c_uint,
    reg_entry_size: ::core::ffi::c_uint,
    cpu_endian: bool,
    regs: *mut pt_regs,
) {
    ::core::ptr::write_bytes(regs as *mut u8, 0, ::core::mem::size_of::<pt_regs>());

    for _i in 0..regs_cnt {
        let reg_entry = bufp as *const hdat_fadump_reg_entry;
        let val = if cpu_endian {
            be64_to_cpu((*reg_entry).reg_val)
        } else {
            (*reg_entry).reg_val as u64
        };
        opal_fadump_set_regval_regnum(
            regs,
            be32_to_cpu((*reg_entry).reg_type),
            be32_to_cpu((*reg_entry).reg_num),
            val,
        );
        bufp = bufp.add(reg_entry_size as usize);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
