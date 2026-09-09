/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SNI specific PCI support for RM200/RM300.
 *
 * Copyright (C) 1997 - 2000, 2003, 04 Ralf Baechle (ralf@linux-mips.org)
 */

/* PCIMT Shortcuts ... */
const SCSI: i8 = PCIMT_IRQ_SCSI as i8;
const ETH: i8 = PCIMT_IRQ_ETHERNET as i8;
const INTA: i8 = PCIMT_IRQ_INTA as i8;
const INTB: i8 = PCIMT_IRQ_INTB as i8;
const INTC: i8 = PCIMT_IRQ_INTC as i8;
const INTD: i8 = PCIMT_IRQ_INTD as i8;

static mut IRQ_TAB_RM200: [[i8; 5]; 8] = [
    [0, 0, 0, 0, 0],
    [SCSI, SCSI, SCSI, SCSI, SCSI],
    [ETH, ETH, ETH, ETH, ETH],
    [INTB, INTB, INTB, INTB, INTB],
    [0, 0, 0, 0, 0],
    [0, INTB, INTC, INTD, INTA],
    [0, INTC, INTD, INTA, INTB],
    [0, INTD, INTA, INTB, INTC],
];

static mut IRQ_TAB_RM300D: [[i8; 5]; 8] = [
    [0, 0, 0, 0, 0],
    [SCSI, SCSI, SCSI, SCSI, SCSI],
    [0, INTC, INTD, INTA, INTB],
    [INTB, INTB, INTB, INTB, INTB],
    [0, 0, 0, 0, 0],
    [0, INTB, INTC, INTD, INTA],
    [0, INTC, INTD, INTA, INTB],
    [0, INTD, INTA, INTB, INTC],
];

static mut IRQ_TAB_RM300E: [[i8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [SCSI, SCSI, SCSI, SCSI, SCSI],
    [0, INTC, INTD, INTA, INTB],
    [0, INTD, INTA, INTB, INTC],
    [0, INTA, INTB, INTC, INTD],
];

/* PCIT Shortcuts ... */
const SCSI0: i8 = PCIT_IRQ_SCSI0 as i8;
const SCSI1: i8 = PCIT_IRQ_SCSI1 as i8;
const ETH_PCI: i8 = PCIT_IRQ_ETHERNET as i8;
const INTA_PCI: i8 = PCIT_IRQ_INTA as i8;
const INTB_PCI: i8 = PCIT_IRQ_INTB as i8;
const INTC_PCI: i8 = PCIT_IRQ_INTC as i8;
const INTD_PCI: i8 = PCIT_IRQ_INTD as i8;

static mut IRQ_TAB_PCIT: [[i8; 5]; 13] = [
    [0, 0, 0, 0, 0],
    [SCSI0, SCSI0, SCSI0, SCSI0, SCSI0],
    [SCSI1, SCSI1, SCSI1, SCSI1, SCSI1],
    [ETH_PCI, ETH_PCI, ETH_PCI, ETH_PCI, ETH_PCI],
    [0, INTA_PCI, INTB_PCI, INTC_PCI, INTD_PCI],
    [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0],
    [0, INTA_PCI, INTB_PCI, INTC_PCI, INTD_PCI],
    [0, INTB_PCI, INTC_PCI, INTD_PCI, INTA_PCI],
    [0, INTC_PCI, INTD_PCI, INTA_PCI, INTB_PCI],
    [0, INTD_PCI, INTA_PCI, INTB_PCI, INTC_PCI],
    [0, INTA_PCI, INTB_PCI, INTC_PCI, INTD_PCI],
];

static mut IRQ_TAB_PCIT_CPLUS: [[i8; 5]; 13] = [
    [0, 0, 0, 0, 0], [0, INTB_PCI, INTC_PCI, INTD_PCI, INTA_PCI],
    [0, 0, 0, 0, 0], [0, 0, 0, 0, 0],
    [0, INTA_PCI, INTB_PCI, INTC_PCI, INTD_PCI],
    [0, INTB_PCI, INTC_PCI, INTD_PCI, INTA_PCI],
    [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0],
];

#[inline]
unsafe fn is_rm300_revd() -> bool {
    let csmsr = core::ptr::read_volatile(PCIMT_CSMSR as *const u8);
    (csmsr & 0xa0) == 0x20
}

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, mut slot: u8, pin: u8) -> i32 {
    match sni_brd_type {
        SNI_BRD_PCI_TOWER_CPLUS => {
            if slot == 4 {
                let mut current = dev;
                while !current.is_null() && (*(*current).bus).number != 1 {
                    current = (*(*current).bus).self_;
                }
                if !current.is_null() && (*current).devfn >= PCI_DEVFN(4, 0) {
                    slot = 5;
                }
            }
            IRQ_TAB_PCIT_CPLUS[slot as usize][pin as usize] as i32
        }
        SNI_BRD_PCI_TOWER => IRQ_TAB_PCIT[slot as usize][pin as usize] as i32,
        SNI_BRD_PCI_MTOWER => {
            if is_rm300_revd() { IRQ_TAB_RM300D[slot as usize][pin as usize] as i32 }
            else { IRQ_TAB_RM200[slot as usize][pin as usize] as i32 }
        }
        SNI_BRD_PCI_DESKTOP => IRQ_TAB_RM200[slot as usize][pin as usize] as i32,
        SNI_BRD_PCI_MTOWER_CPLUS => IRQ_TAB_RM300E[slot as usize][pin as usize] as i32,
        _ => 0,
    }
}

pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
