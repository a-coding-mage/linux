// SPDX-License-Identifier: GPL-2.0
/*
 * AMD Family 10h mmconfig enablement
 */

// C includes are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct pci_hostbridge_probe {
    pub bus: u32,
    pub slot: u32,
    pub vendor: u32,
    pub device: u32,
}

static mut fam10h_pci_mmconf_base: u64 = 0;

static mut pci_probes: [pci_hostbridge_probe; 2] = [
    pci_hostbridge_probe { bus: 0, slot: 0x18, vendor: PCI_VENDOR_ID_AMD, device: 0x1200 },
    pci_hostbridge_probe { bus: 0xff, slot: 0, vendor: PCI_VENDOR_ID_AMD, device: 0x1200 },
];

unsafe extern "C" {
    static mut pci_probe: u32;
    static mut acpi_pci_disabled: bool;
    fn early_pci_allowed() -> bool;
    fn read_pci_config(bus: u32, slot: u32, fn_: u32, reg: u32) -> u32;
    fn rdmsrq(address: u32, val: *mut u64);
    fn wrmsrq(address: u32, val: u64);
    fn printk(fmt: *const u8, ...);
    fn sort(base: *mut core::ffi::c_void, num: usize, size: usize,
            cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
            swap: *mut core::ffi::c_void);
    fn dmi_check_system(table: *const dmi_system_id) -> i32;
}

#[repr(C)]
pub struct range { pub start: u64, pub end: u64 }

unsafe extern "C" fn cmp_range(x1: *const core::ffi::c_void, x2: *const core::ffi::c_void) -> i32 {
    let r1 = &*(x1 as *const range);
    let r2 = &*(x2 as *const range);
    let start1 = (r1.start >> 32) as i32;
    let start2 = (r2.start >> 32) as i32;
    start1 - start2
}

const MMCONF_UNIT: u64 = 1u64 << FAM10H_MMIO_CONF_BASE_SHIFT;
const MMCONF_MASK: u64 = !(MMCONF_UNIT - 1);
const MMCONF_SIZE: u64 = MMCONF_UNIT << 8;
/* need to avoid (0xfd<<32), (0xfe<<32), and (0xff<<32), ht used space */
const FAM10H_PCI_MMCONF_BASE: u64 = 0xfcu64 << 32;

#[inline]
fn base_valid(b: u64) -> bool {
    b.wrapping_add(MMCONF_SIZE) <= (0xfdu64 << 32) || b >= (1u64 << 40)
}

unsafe fn get_fam10h_pci_mmconf_base() {
    let mut val: u64 = 0;
    let mut address: u32;
    let mut tom2: u64;
    let mut base: u64 = FAM10H_PCI_MMCONF_BASE;
    let mut range = [range { start: 0, end: 0 }; 8];

    /* only try to get setting from BSP */
    if fam10h_pci_mmconf_base != 0 { return; }
    if !early_pci_allowed() { return; }

    let mut bus = 0u32;
    let mut slot = 0u32;
    let mut found = false;
    for i in 0..pci_probes.len() {
        let probe = &pci_probes[i];
        bus = probe.bus;
        slot = probe.slot;
        let id = read_pci_config(bus, slot, 0, PCI_VENDOR_ID);
        let vendor = (id & 0xffff) as u16;
        let device = ((id >> 16) & 0xffff) as u16;
        if probe.vendor as u16 == vendor && probe.device as u16 == device {
            found = true;
            break;
        }
    }
    if !found { return; }

    /* SYS_CFG */
    address = MSR_AMD64_SYSCFG;
    rdmsrq(address, &mut val);
    /* TOP_MEM2 is not enabled? */
    if val & (1u64 << 21) == 0 {
        tom2 = 1u64 << 32;
    } else {
        /* TOP_MEM2 */
        address = MSR_K8_TOP_MEM2;
        rdmsrq(address, &mut val);
        tom2 = core::cmp::max(val & 0xffffff800000u64, 1u64 << 32);
    }
    if base <= tom2 { base = (tom2 + 2 * MMCONF_UNIT - 1) & MMCONF_MASK; }

    /* need to check if the range is in the high mmio range that is above 4G */
    let mut hi_mmio_num = 0usize;
    for i in 0..8u32 {
        let mut reg = read_pci_config(bus, slot, 1, 0x80 + (i << 3));
        if reg & 3 == 0 { continue; }
        let start = ((reg & 0xffffff00) as u64) << 8;
        reg = read_pci_config(bus, slot, 1, 0x84 + (i << 3));
        let end = (((reg & 0xffffff00) as u64) << 8) | 0xffff;
        if end < tom2 { continue; }
        range[hi_mmio_num] = range { start, end };
        hi_mmio_num += 1;
    }
    if hi_mmio_num == 0 { return; }

    /* sort the range */
    sort(range.as_mut_ptr() as *mut core::ffi::c_void, hi_mmio_num,
         core::mem::size_of::<range>(), cmp_range, core::ptr::null_mut());
    if range[hi_mmio_num - 1].end < base { return; }
    if range[0].start > base + MMCONF_SIZE { return; }

    /* need to find one window */
    base = (range[0].start & MMCONF_MASK).wrapping_sub(MMCONF_UNIT);
    if base > tom2 && base_valid(base) { fam10h_pci_mmconf_base = base; return; }
    base = (range[hi_mmio_num - 1].end + MMCONF_UNIT) & MMCONF_MASK;
    if base_valid(base) { fam10h_pci_mmconf_base = base; return; }
    /* need to find window between ranges */
    for i in 1..hi_mmio_num {
        base = (range[i - 1].end + MMCONF_UNIT) & MMCONF_MASK;
        val = range[i].start & MMCONF_MASK;
        if val >= base + MMCONF_SIZE && base_valid(base) {
            fam10h_pci_mmconf_base = base;
            return;
        }
    }
}

pub unsafe fn fam10h_check_enable_mmcfg() {
    let mut val: u64 = 0;
    let address = MSR_FAM10H_MMIO_CONF_BASE;
    if pci_probe & PCI_CHECK_ENABLE_AMD_MMCONF == 0 { return; }
    rdmsrq(address, &mut val);
    /* try to make sure that AP's setting is identical to BSP setting */
    if val & FAM10H_MMIO_CONF_ENABLE != 0 {
        let busnbits = (val >> FAM10H_MMIO_CONF_BUSRANGE_SHIFT) & FAM10H_MMIO_CONF_BUSRANGE_MASK;
        /* only trust the one handle 256 buses, if acpi=off */
        if !acpi_pci_disabled || busnbits >= 8 {
            let base = val & MMCONF_MASK;
            if fam10h_pci_mmconf_base == 0 { fam10h_pci_mmconf_base = base; return; }
            else if fam10h_pci_mmconf_base == base { return; }
        }
    }
    /* if it is not enabled, try to enable it and assume only one segment
     * with 256 buses */
    get_fam10h_pci_mmconf_base();
    if fam10h_pci_mmconf_base == 0 {
        pci_probe &= !PCI_CHECK_ENABLE_AMD_MMCONF;
        return;
    }
    printk(b"Enable MMCONFIG on AMD Family 10h\0".as_ptr());
    val &= !((FAM10H_MMIO_CONF_BASE_MASK << FAM10H_MMIO_CONF_BASE_SHIFT) |
             (FAM10H_MMIO_CONF_BUSRANGE_MASK << FAM10H_MMIO_CONF_BUSRANGE_SHIFT));
    val |= fam10h_pci_mmconf_base | (8 << FAM10H_MMIO_CONF_BUSRANGE_SHIFT) |
           FAM10H_MMIO_CONF_ENABLE;
    wrmsrq(address, val);
}

unsafe fn set_check_enable_amd_mmconf(_d: *const dmi_system_id) -> i32 {
    pci_probe |= PCI_CHECK_ENABLE_AMD_MMCONF;
    0
}

#[repr(C)]
pub struct dmi_system_id { pub callback: Option<unsafe fn(*const dmi_system_id) -> i32>, pub ident: *const u8, pub matches: [u8; 1] }

static mmconf_dmi_table: [dmi_system_id; 2] = [
    dmi_system_id { callback: Some(set_check_enable_amd_mmconf), ident: b"Sun Microsystems Machine\0".as_ptr(), matches: [0] },
    dmi_system_id { callback: None, ident: core::ptr::null(), matches: [0] },
];

/* Called from a non __init function, but only on the BSP. */
pub unsafe fn check_enable_amd_mmconf_dmi() { dmi_check_system(mmconf_dmi_table.as_ptr()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
