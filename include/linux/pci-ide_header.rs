/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common helpers for drivers (e.g. low-level PCI/TSM drivers) implementing the
 * IDE key management protocol (IDE_KM) as defined by:
 * PCIe r7.0 section 6.33 Integrity & Data Encryption (IDE)
 *
 * Copyright(c) 2024-2025 Intel Corporation. All rights reserved.
 */

#[repr(C)]
pub enum pci_ide_partner_select {
    PCI_IDE_EP,
    PCI_IDE_RP,
    PCI_IDE_PARTNER_MAX,
    /*
     * In addition to the resources in each partner port the
     * platform / host-bridge additionally has a Stream ID pool that
     * it shares across root ports. Let pci_ide_stream_alloc() use
     * the alloc_stream_index() helper as endpoints and root ports.
     */
    PCI_IDE_HB = PCI_IDE_PARTNER_MAX as isize,
}

/**
 * struct pci_ide_partner - Per port pair Selective IDE Stream settings
 * @rid_start: Partner Port Requester ID range start
 * @rid_end: Partner Port Requester ID range end (inclusive)
 * @stream_index: Selective IDE Stream Register Block selection
 * @mem_assoc: PCI bus memory address association for targeting peer partner
 * @pref_assoc: PCI bus prefetchable memory address association for
 *	 targeting peer partner
 * @default_stream: Endpoint uses this stream for all upstream TLPs regardless of
 *			 address and RID association registers
 * @setup: flag to track whether to run pci_ide_stream_teardown() for this
 *	   partner slot
 * @enable: flag whether to run pci_ide_stream_disable() for this partner slot
 *
 * By default, pci_ide_stream_alloc() initializes @mem_assoc and @pref_assoc
 * with the immediate ancestor downstream port memory ranges (i.e. Type 1
 * Configuration Space Header values). Caller may zero size ({0, -1}) the range
 * to drop it from consideration at pci_ide_stream_setup() time.
 */
#[repr(C)]
pub struct pci_ide_partner {
    pub rid_start: u16,
    pub rid_end: u16,
    pub stream_index: u8,
    pub mem_assoc: pci_bus_region,
    pub pref_assoc: pci_bus_region,
    /* C bit-fields are represented in their containing unsigned-int storage. */
    pub default_stream: u32,
    pub setup: u32,
    pub enable: u32,
}

#[repr(C)]
pub struct pci_ide_regs_addr {
    pub assoc1: u32,
    pub assoc2: u32,
    pub assoc3: u32,
}

/**
 * struct pci_ide_regs - Hardware register association settings for Selective
 *			 IDE Streams
 * @rid1: IDE RID Association Register 1
 * @rid2: IDE RID Association Register 2
 * @addr: Up to two address association blocks (IDE Address Association Register
 *	 1 through 3) for MMIO and prefetchable MMIO
 * @nr_addr: Number of address association blocks initialized
 *
 * See pci_ide_stream_to_regs()
 */
#[repr(C)]
pub struct pci_ide_regs {
    pub rid1: u32,
    pub rid2: u32,
    pub addr: [pci_ide_regs_addr; 2],
    pub nr_addr: i32,
}

/**
 * struct pci_ide - PCIe Selective IDE Stream descriptor
 * @pdev: PCIe Endpoint in the pci_ide_partner pair
 * @partner: per-partner settings
 * @host_bridge_stream: allocated from host bridge @ide_stream_ida pool
 * @stream_id: unique Stream ID (within Partner Port pairing)
 * @name: name of the established Selective IDE Stream in sysfs
 *
 * Negative @stream_id values indicate "uninitialized" on the
 * expectation that with TSM established IDE the TSM owns the stream_id
 * allocation.
 */
#[repr(C)]
pub struct pci_ide {
    pub pdev: *mut pci_dev,
    pub partner: [pci_ide_partner; PCI_IDE_PARTNER_MAX as usize],
    pub host_bridge_stream: u8,
    pub stream_id: i32,
    pub name: *const core::ffi::c_char,
}

/*
 * Some devices need help with aliased stream-ids even for idle streams. Use
 * this id as the "never enabled" place holder.
 */
pub const PCI_IDE_RESERVED_STREAM_ID: u8 = 255;

extern "C" {
    pub fn pci_ide_set_nr_streams(hb: *mut pci_host_bridge, nr: u16);
    pub fn pci_ide_to_settings(
        pdev: *mut pci_dev,
        ide: *mut pci_ide,
    ) -> *mut pci_ide_partner;
    pub fn pci_ide_stream_alloc(pdev: *mut pci_dev) -> *mut pci_ide;
    pub fn pci_ide_stream_free(ide: *mut pci_ide);
    pub fn pci_ide_stream_register(ide: *mut pci_ide) -> i32;
    pub fn pci_ide_stream_unregister(ide: *mut pci_ide);
    pub fn pci_ide_stream_setup(pdev: *mut pci_dev, ide: *mut pci_ide);
    pub fn pci_ide_stream_teardown(pdev: *mut pci_dev, ide: *mut pci_ide);
    pub fn pci_ide_stream_enable(pdev: *mut pci_dev, ide: *mut pci_ide) -> i32;
    pub fn pci_ide_stream_disable(pdev: *mut pci_dev, ide: *mut pci_ide);
    pub fn pci_ide_stream_release(ide: *mut pci_ide);
}

/* DEFINE_FREE(pci_ide_stream_release, struct pci_ide *,
 *             if (_T) pci_ide_stream_release(_T)) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
