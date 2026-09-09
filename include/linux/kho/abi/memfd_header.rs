/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 *
 * Copyright (C) 2025 Amazon.com Inc. or its affiliates.
 * Pratyush Yadav <ptyadav@amazon.de>
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h and linux/kho/abi/kexec_handover.h

/**
 * DOC: memfd Live Update ABI
 *
 * memfd uses the ABI defined below for preserving its state across a kexec
 * reboot using the LUO.
 *
 * The state is serialized into a packed structure `struct memfd_luo_ser`
 * which is handed over to the next kernel via the KHO mechanism.
 *
 * This interface is a contract. Any modification to the structure layout
 * constitutes a breaking change. Such changes require incrementing the
 * version number in the MEMFD_LUO_FH_COMPATIBLE string.
 */

/// The folio is dirty.
pub const MEMFD_LUO_FOLIO_DIRTY: u64 = 1u64 << 0;

/// The folio is up-to-date.
pub const MEMFD_LUO_FOLIO_UPTODATE: u64 = 1u64 << 1;

/**
 * Serialized state of a single folio.
 *
 * The C bitfields `pfn:52` and `flags:12` occupy the first packed u64 in
 * declaration order. The low 52 bits contain pfn and the high 12 bits flags.
 */
#[repr(C, packed)]
pub struct memfd_luo_folio_ser {
    pub pfn_flags: u64,
    pub index: u64,
}

impl memfd_luo_folio_ser {
    pub const PFN_MASK: u64 = (1u64 << 52) - 1;
    pub const FLAGS_MASK: u64 = (1u64 << 12) - 1;

    #[inline]
    pub const fn pfn(&self) -> u64 {
        self.pfn_flags & Self::PFN_MASK
    }

    #[inline]
    pub const fn flags(&self) -> u64 {
        (self.pfn_flags >> 52) & Self::FLAGS_MASK
    }
}

/*
 * The set of seals this version supports preserving. If support for any new
 * seals is needed, add it here and bump version.
 */
pub const MEMFD_LUO_ALL_SEALS: u32 = F_SEAL_SEAL
    | F_SEAL_SHRINK
    | F_SEAL_GROW
    | F_SEAL_WRITE
    | F_SEAL_FUTURE_WRITE
    | F_SEAL_EXEC;

/**
 * Main serialization structure for a memfd.
 */
#[repr(C, packed)]
pub struct memfd_luo_ser {
    pub pos: u64,
    pub size: u64,
    pub seals: u32,
    pub flags: u32,
    pub nr_folios: u64,
    pub folios: kho_vmalloc,
}

/* The compatibility string for memfd file handler */
pub const MEMFD_LUO_FH_COMPATIBLE: &[u8] = b"memfd-v2\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
