/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Copyright (C) 2023 Alexander Graf <graf@amazon.com>
 * Copyright (C) 2025 Microsoft Corporation, Mike Rapoport <rppt@kernel.org>
 * Copyright (C) 2025 Google LLC, Changyuan Lyu <changyuanl@google.com>
 * Copyright (C) 2025 Google LLC, Jason Miu <jasonmiu@google.com>
 */

/* Kexec Handover ABI. The ABI uses Flattened Device Tree (FDT) format. */

/* The compatible string for the KHO FDT root node. */
pub const KHO_FDT_COMPATIBLE: &str = "kho-v4";

/* The FDT property for the preserved memory map. */
pub const KHO_FDT_MEMORY_MAP_PROP_NAME: &str = "preserved-memory-map";

/* The FDT property for preserved data blobs. */
pub const KHO_SUB_TREE_PROP_NAME: &str = "preserved-data";

/* The FDT property for the size of preserved data blobs. */
pub const KHO_SUB_TREE_SIZE_PROP_NAME: &str = "blob-size";

/*
 * Helper macro to define a union for a serializable pointer.
 *
 * `phys` stores the physical address and `ptr` stores the corresponding
 * pointer representation. This is the Rust equivalent of DECLARE_KHOSER_PTR.
 */
#[repr(C)]
pub union KhoSerPtr<T> {
    pub phys: u64,
    pub ptr: *mut T,
}

/* The C macros KHOSER_STORE_PTR and KHOSER_LOAD_PTR require the external
 * virt_to_phys and phys_to_virt conversion helpers and are represented by
 * these direct low-level operations at their call sites. */

/*
 * This header is embedded at the beginning of each `kho_vmalloc_chunk`
 * and contains a pointer to the next chunk in the linked list,
 * stored as a physical address for handover.
 */
#[repr(C)]
pub struct kho_vmalloc_hdr {
    pub next: KhoSerPtr<kho_vmalloc_chunk>,
}

pub const KHO_VMALLOC_SIZE: usize =
    (PAGE_SIZE - core::mem::size_of::<kho_vmalloc_hdr>()) / core::mem::size_of::<u64>();

/*
 * Each chunk is a single page and is part of a linked list that describes
 * a preserved vmalloc area. It contains the header with the link to the next
 * chunk and a zero terminated array of physical addresses of the pages that
 * make up the preserved vmalloc area.
 */
#[repr(C)]
pub struct kho_vmalloc_chunk {
    pub hdr: kho_vmalloc_hdr,
    pub phys: [u64; KHO_VMALLOC_SIZE],
}

const _: () = assert!(core::mem::size_of::<kho_vmalloc_chunk>() == PAGE_SIZE);

/*
 * Describes a preserved vmalloc memory area, including the
 * total number of pages, allocation flags, page order, and a pointer to the
 * first chunk of physical page addresses.
 */
#[repr(C)]
pub struct kho_vmalloc {
    pub first: KhoSerPtr<kho_vmalloc_chunk>,
    pub total_pages: core::ffi::c_uint,
    pub flags: core::ffi::c_ushort,
    pub order: core::ffi::c_ushort,
}

/* KHO radix tree constants. */
pub const KHO_RADIX_KEY_WIDTH: usize = 64 - PAGE_SHIFT + 1;
pub const KHO_TABLE_SIZE_LOG2: usize = const_ilog2(PAGE_SIZE / core::mem::size_of::<phys_addr_t>());
pub const KHO_BITMAP_SIZE_LOG2: usize = PAGE_SHIFT + const_ilog2(BITS_PER_BYTE);
pub const KHO_TREE_MAX_DEPTH: usize =
    (KHO_RADIX_KEY_WIDTH - KHO_BITMAP_SIZE_LOG2 + KHO_TABLE_SIZE_LOG2 - 1)
        / KHO_TABLE_SIZE_LOG2
        + 1;

#[repr(C)]
pub struct kho_radix_node {
    pub table: [u64; 1usize << KHO_TABLE_SIZE_LOG2],
}

#[repr(C)]
pub struct kho_radix_leaf {
    pub bitmap: [u8; (1usize << KHO_BITMAP_SIZE_LOG2) / BITS_PER_BYTE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
