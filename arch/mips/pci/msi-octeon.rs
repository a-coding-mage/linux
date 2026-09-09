/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005-2009, 2010 Cavium Networks
 */

// Linux and Octeon dependencies supplied by other translation units.

static mut MSI_FREE_IRQ_BITMASK: [u64; 4] = [0; 4];
static mut MSI_MULTIPLE_IRQ_BITMASK: [u64; 4] = [0; 4];
static mut MSI_IRQ_SIZE: i32 = 0;

/* External kernel/device declarations. */
extern "C" {
    static mut octeon_dma_bar_type: i32;
    static mut msi_free_irq_bitmask_lock: usize;
    static mut octeon_irq_msi_lock: usize;
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16);
    fn pci_write_config_word(dev: *mut pci_dev, where_: u32, val: u16);
    fn irq_set_msi_desc(irq: i32, desc: *mut msi_desc);
    fn pci_write_msi_msg(irq: i32, msg: *mut msi_msg);
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_write_csr(address: u64, value: u64);
    fn do_IRQ(irq: i32);
    fn fls64(value: u64) -> i32;
    fn raw_spin_lock_irqsave(lock: *mut usize, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut usize, flags: usize);
    fn spin_lock(lock: *mut usize);
    fn spin_unlock(lock: *mut usize);
    fn panic(message: *const u8, ...);
    fn pr_err(message: *const u8, ...);
    fn irq_set_chip_and_handler(irq: i32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, dev_id: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t) -> i32;
    fn octeon_has_feature(feature: i32) -> bool;
    fn octeon_is_pci_host() -> bool;
}

#[repr(C)] pub struct pci_dev { pub msi_cap: u32 }
#[repr(C)] pub struct msi_desc { pub pci: msi_desc_pci }
#[repr(C)] pub struct msi_desc_pci { pub msi_attrib: msi_attrib }
#[repr(C)] pub struct msi_attrib { pub is_msix: bool }
#[repr(C)] pub struct msi_msg { pub address_lo: u32, pub address_hi: u32, pub data: u32 }
#[repr(C)] pub struct irq_data { pub irq: i32 }
#[repr(C)] pub struct irq_chip { pub name: *const u8, pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)> }
pub type irqreturn_t = i32;

const EINVAL: i32 = 22;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const PCI_MSI_FLAGS: u32 = 2;
const PCI_MSI_FLAGS_QSIZE: u16 = 0x0070;
const PCI_MSI_FLAGS_QMASK: u16 = 0x000e;
const OCTEON_DMA_BAR_TYPE_SMALL: i32 = 0;
const OCTEON_DMA_BAR_TYPE_BIG: i32 = 1;
const OCTEON_DMA_BAR_TYPE_PCIE: i32 = 2;
const OCTEON_DMA_BAR_TYPE_PCIE2: i32 = 3;
const OCTEON_DMA_BAR_TYPE_INVALID: i32 = -1;

extern "C" {
    static OCTEON_IRQ_MSI_BIT0: i32;
    static OCTEON_IRQ_MSI_LAST: i32;
    static OCTEON_IRQ_PCI_MSI0: i32;
    static OCTEON_IRQ_PCI_MSI1: i32;
    static OCTEON_IRQ_PCI_MSI2: i32;
    static OCTEON_IRQ_PCI_MSI3: i32;
    static CVMX_PCI_MSI_RCV: u64;
    static CVMX_NPEI_PCIE_MSI_RCV: u64;
    static CVMX_SLI_PCIE_MSI_RCV: u64;
    static CVMX_PEXP_NPEI_MSI_RCV0: u64; static CVMX_PEXP_NPEI_MSI_RCV1: u64;
    static CVMX_PEXP_NPEI_MSI_RCV2: u64; static CVMX_PEXP_NPEI_MSI_RCV3: u64;
    static CVMX_PEXP_NPEI_MSI_ENB0: u64; static CVMX_PEXP_NPEI_MSI_ENB1: u64;
    static CVMX_PEXP_NPEI_MSI_ENB2: u64; static CVMX_PEXP_NPEI_MSI_ENB3: u64;
    static CVMX_NPI_NPI_MSI_RCV: u64;
}

pub unsafe extern "C" fn arch_setup_msi_irq(dev: *mut pci_dev, desc: *mut msi_desc) -> i32 {
    let mut msg: msi_msg = core::mem::zeroed();
    let mut control: u16 = 0;
    let mut configured_private_bits: i32;
    let mut request_private_bits: i32;
    let mut irq: i32 = 0;
    let mut irq_step: i32;
    let mut search_mask: u64;
    let mut index: i32;
    if (*desc).pci.msi_attrib.is_msix { return -EINVAL; }
    pci_read_config_word(dev, (*dev).msi_cap + PCI_MSI_FLAGS, &mut control);
    configured_private_bits = ((control & PCI_MSI_FLAGS_QSIZE) >> 4) as i32;
    if configured_private_bits == 0 { request_private_bits = ((control & PCI_MSI_FLAGS_QMASK) >> 1) as i32; }
    else { request_private_bits = configured_private_bits; }
    if request_private_bits > 5 { request_private_bits = 0; }
    'try_only_one: loop {
        irq_step = 1 << request_private_bits;
        search_mask = (1u64 << irq_step) - 1;
        spin_lock(&raw mut msi_free_irq_bitmask_lock);
        index = 0;
        'search: loop {
            if index >= MSI_IRQ_SIZE / 64 { break; }
            irq = 0;
            while irq < 64 {
                if (MSI_FREE_IRQ_BITMASK[index as usize] & (search_mask << irq)) == 0 {
                    MSI_FREE_IRQ_BITMASK[index as usize] |= search_mask << irq;
                    MSI_MULTIPLE_IRQ_BITMASK[index as usize] |= (search_mask >> 1) << irq;
                    break 'search;
                }
                irq += irq_step;
            }
            index += 1;
        }
        spin_unlock(&raw mut msi_free_irq_bitmask_lock);
        if irq >= 64 {
            if request_private_bits != 0 { pr_err(b"arch_setup_msi_irq: Unable to find free interrupts, trying just one\0".as_ptr()); request_private_bits = 0; continue 'try_only_one; }
            panic(b"arch_setup_msi_irq: Unable to find a free MSI interrupt\0".as_ptr());
        }
        break;
    }
    irq += index * 64 + OCTEON_IRQ_MSI_BIT0;
    let address: u64 = match octeon_dma_bar_type {
        OCTEON_DMA_BAR_TYPE_SMALL => (128u64 << 20) + CVMX_PCI_MSI_RCV,
        OCTEON_DMA_BAR_TYPE_BIG => CVMX_PCI_MSI_RCV,
        OCTEON_DMA_BAR_TYPE_PCIE => CVMX_NPEI_PCIE_MSI_RCV,
        OCTEON_DMA_BAR_TYPE_PCIE2 => CVMX_SLI_PCIE_MSI_RCV,
        _ => { panic(b"arch_setup_msi_irq: Invalid octeon_dma_bar_type\0".as_ptr()); 0 }
    };
    msg.address_lo = address as u32; msg.address_hi = (address >> 32) as u32;
    msg.data = (irq - OCTEON_IRQ_MSI_BIT0) as u32;
    control &= !PCI_MSI_FLAGS_QSIZE; control |= (request_private_bits as u16) << 4;
    pci_write_config_word(dev, (*dev).msi_cap + PCI_MSI_FLAGS, control);
    irq_set_msi_desc(irq, desc); pci_write_msi_msg(irq, &mut msg); 0
}

pub unsafe extern "C" fn arch_teardown_msi_irq(mut irq: u32) {
    if irq < OCTEON_IRQ_MSI_BIT0 as u32 || irq > (MSI_IRQ_SIZE + OCTEON_IRQ_MSI_BIT0) as u32 { panic(b"arch_teardown_msi_irq: Attempted to teardown illegal MSI interrupt\0".as_ptr()); }
    irq -= OCTEON_IRQ_MSI_BIT0 as u32;
    let index = irq / 64; let irq0 = irq % 64; let mut number_irqs = 0;
    while irq0 + number_irqs < 64 && (MSI_MULTIPLE_IRQ_BITMASK[index as usize] & (1u64 << (irq0 + number_irqs))) != 0 { number_irqs += 1; }
    number_irqs += 1; let bitmask = ((1u64 << number_irqs) - 1) << irq0;
    if (MSI_FREE_IRQ_BITMASK[index as usize] & bitmask) != bitmask { panic(b"arch_teardown_msi_irq: Attempted to teardown MSI interrupt not in use\0".as_ptr()); }
    spin_lock(&raw mut msi_free_irq_bitmask_lock); MSI_FREE_IRQ_BITMASK[index as usize] &= !bitmask; MSI_MULTIPLE_IRQ_BITMASK[index as usize] &= !bitmask; spin_unlock(&raw mut msi_free_irq_bitmask_lock);
}

static mut MSI_RCV_REG: [u64; 4] = [0; 4];
static mut MIS_ENA_REG: [u64; 4] = [0; 4];

unsafe extern "C" fn octeon_irq_msi_enable_pcie(data: *mut irq_data) { let n = (*data).irq - OCTEON_IRQ_MSI_BIT0; let i = (n >> 6) as usize; let b = n & 0x3f; let mut flags = 0; raw_spin_lock_irqsave(&raw mut octeon_irq_msi_lock, &mut flags); let mut en = cvmx_read_csr(MIS_ENA_REG[i]); en |= 1u64 << b; cvmx_write_csr(MIS_ENA_REG[i], en); cvmx_read_csr(MIS_ENA_REG[i]); raw_spin_unlock_irqrestore(&raw mut octeon_irq_msi_lock, flags); }
unsafe extern "C" fn octeon_irq_msi_disable_pcie(data: *mut irq_data) { let n = (*data).irq - OCTEON_IRQ_MSI_BIT0; let i = (n >> 6) as usize; let b = n & 0x3f; let mut flags = 0; raw_spin_lock_irqsave(&raw mut octeon_irq_msi_lock, &mut flags); let mut en = cvmx_read_csr(MIS_ENA_REG[i]); en &= !(1u64 << b); cvmx_write_csr(MIS_ENA_REG[i], en); cvmx_read_csr(MIS_ENA_REG[i]); raw_spin_unlock_irqrestore(&raw mut octeon_irq_msi_lock, flags); }
static mut OCTEON_IRQ_CHIP_MSI_PCIE: irq_chip = irq_chip { name: b"MSI\0".as_ptr(), irq_enable: Some(octeon_irq_msi_enable_pcie), irq_disable: Some(octeon_irq_msi_disable_pcie) };
unsafe extern "C" fn octeon_irq_msi_enable_pci(_data: *mut irq_data) {}
unsafe extern "C" fn octeon_irq_msi_disable_pci(_data: *mut irq_data) {}
static mut OCTEON_IRQ_CHIP_MSI_PCI: irq_chip = irq_chip { name: b"MSI\0".as_ptr(), irq_enable: Some(octeon_irq_msi_enable_pci), irq_disable: Some(octeon_irq_msi_disable_pci) };

unsafe fn octeon_msi_do_interrupt(index: i32, msi_bits: u64) -> irqreturn_t { let mut bit = fls64(msi_bits); if bit != 0 { bit -= 1; cvmx_write_csr(MSI_RCV_REG[index as usize], 1u64 << bit); do_IRQ(bit + OCTEON_IRQ_MSI_BIT0 + 64 * index); return IRQ_HANDLED; } IRQ_NONE }
macro_rules! msi_handler { ($name:ident, $x:expr) => { pub unsafe extern "C" fn $name(_cpl: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t { octeon_msi_do_interrupt($x, cvmx_read_csr(MSI_RCV_REG[$x])) } }; }
msi_handler!(octeon_msi_interrupt0, 0); msi_handler!(octeon_msi_interrupt1, 1); msi_handler!(octeon_msi_interrupt2, 2); msi_handler!(octeon_msi_interrupt3, 3);

pub unsafe extern "C" fn octeon_msi_initialize() -> i32 {
    let msi: *mut irq_chip;
    if octeon_dma_bar_type == OCTEON_DMA_BAR_TYPE_INVALID { return 0; }
    if octeon_dma_bar_type == OCTEON_DMA_BAR_TYPE_PCIE { MSI_RCV_REG = [CVMX_PEXP_NPEI_MSI_RCV0, CVMX_PEXP_NPEI_MSI_RCV1, CVMX_PEXP_NPEI_MSI_RCV2, CVMX_PEXP_NPEI_MSI_RCV3]; MIS_ENA_REG = [CVMX_PEXP_NPEI_MSI_ENB0, CVMX_PEXP_NPEI_MSI_ENB1, CVMX_PEXP_NPEI_MSI_ENB2, CVMX_PEXP_NPEI_MSI_ENB3]; msi = &raw mut OCTEON_IRQ_CHIP_MSI_PCIE; }
    else { MSI_RCV_REG = [CVMX_NPI_NPI_MSI_RCV, 0x8700000000000000, 0x8700000000000000, 0x8700000000000000]; MIS_ENA_REG = [0x8700000000000000; 4]; msi = &raw mut OCTEON_IRQ_CHIP_MSI_PCI; }
    let mut irq = OCTEON_IRQ_MSI_BIT0; while irq <= OCTEON_IRQ_MSI_LAST { irq_set_chip_and_handler(irq, msi, handle_simple_irq); irq += 1; }
    if octeon_has_feature(0) { request_irq(OCTEON_IRQ_PCI_MSI0, octeon_msi_interrupt0, 0, b"MSI[0:63]\0".as_ptr(), octeon_msi_interrupt0); request_irq(OCTEON_IRQ_PCI_MSI1, octeon_msi_interrupt1, 0, b"MSI[64:127]\0".as_ptr(), octeon_msi_interrupt1); request_irq(OCTEON_IRQ_PCI_MSI2, octeon_msi_interrupt2, 0, b"MSI[127:191]\0".as_ptr(), octeon_msi_interrupt2); request_irq(OCTEON_IRQ_PCI_MSI3, octeon_msi_interrupt3, 0, b"MSI[192:255]\0".as_ptr(), octeon_msi_interrupt3); MSI_IRQ_SIZE = 256; }
    else if octeon_is_pci_host() { request_irq(OCTEON_IRQ_PCI_MSI0, octeon_msi_interrupt0, 0, b"MSI[0:15]\0".as_ptr(), octeon_msi_interrupt0); request_irq(OCTEON_IRQ_PCI_MSI1, octeon_msi_interrupt0, 0, b"MSI[16:31]\0".as_ptr(), octeon_msi_interrupt0); request_irq(OCTEON_IRQ_PCI_MSI2, octeon_msi_interrupt0, 0, b"MSI[32:47]\0".as_ptr(), octeon_msi_interrupt0); request_irq(OCTEON_IRQ_PCI_MSI3, octeon_msi_interrupt0, 0, b"MSI[48:63]\0".as_ptr(), octeon_msi_interrupt0); MSI_IRQ_SIZE = 64; }
    0
}

extern "C" { fn handle_simple_irq(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
