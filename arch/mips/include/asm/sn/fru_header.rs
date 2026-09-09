/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Derived from IRIX <sys/SN/SN0/sn0_fru.h>
 *
 * Copyright (C) 1992 - 1997, 1999 Silcon Graphics, Inc.
 * Copyright (C) 1999, 2006 Ralf Baechle (ralf@linux-mips)
 */

pub const MAX_DIMMS: usize = 8; /* max # of dimm banks */
pub const MAX_PCIDEV: usize = 8; /* max # of pci devices on a pci bus */

pub type confidence_t = u8;

#[repr(C)]
pub struct kf_mem_t {
    pub km_confidence: confidence_t, /* confidence level that the memory is bad
                                      * is this necessary ?
                                      */
    pub km_dimm: [confidence_t; MAX_DIMMS],
    /* confidence level that dimm[i] is bad
     *I think this is the right number
     */
}

#[repr(C)]
pub struct kf_cpu_t {
    pub kc_confidence: confidence_t, /* confidence level that cpu is bad */
    pub kc_icache: confidence_t, /* confidence level that instr. cache is bad */
    pub kc_dcache: confidence_t, /* confidence level that data   cache is bad */
    pub kc_scache: confidence_t, /* confidence level that sec.   cache is bad */
    pub kc_sysbus: confidence_t, /* confidence level that sysad/cmd/state bus is bad */
}

#[repr(C)]
pub struct kf_pci_bus_t {
    pub kpb_belief: confidence_t, /* confidence level  that the  pci bus is bad */
    pub kpb_pcidev_belief: [confidence_t; MAX_PCIDEV],
    /* confidence level that the pci dev is bad */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
