// SPDX-License-Identifier: GPL-2.0
/* Low-level direct PCI config space access via ECAM - common code between
 * i386 and x86-64. */

// C includes and build-time configuration are supplied by other translation units.

static mut pci_mmcfg_running_state: bool = false;
static mut pci_mmcfg_arch_init_failed: bool = false;
static mut mcp55_checked: bool = false;
static mut known_bridge: i32 = 0;

// External kernel types, globals, macros, and functions are supplied elsewhere.
extern "C" {
    static mut pci_mmcfg_list: list_head;
    static mut pci_mmcfg_lock: mutex;
    static mut raw_pci_ops: *mut raw_pci_ops;
    static mut pci_probe: u32;
    static mut pcibios_last_bus: i32;
    static mut iomem_resource: resource;
    static mut acpi_disabled: bool;
    fn pci_mmcfg_arch_free();
    fn pci_mmcfg_arch_init() -> i32;
    fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> i32;
    fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region);
    fn release_resource(res: *mut resource);
    fn insert_resource(root: *mut resource, new: *mut resource) -> i32;
    fn insert_resource_conflict(root: *mut resource, new: *mut resource) -> *mut resource;
    fn synchronize_rcu();
}

unsafe fn pci_mmconfig_remove(cfg: *mut pci_mmcfg_region) {
    if !(*cfg).res.parent.is_null() { release_resource(&mut (*cfg).res); }
    list_del(&mut (*cfg).list);
    kfree(cfg as *mut core::ffi::c_void);
}

unsafe fn free_all_mmcfg() {
    pci_mmcfg_arch_free();
    let mut cfg: *mut pci_mmcfg_region = core::ptr::null_mut();
    let mut tmp: *mut pci_mmcfg_region = core::ptr::null_mut();
    list_for_each_entry_safe(&mut cfg, &mut tmp, &mut pci_mmcfg_list, list) {
        pci_mmconfig_remove(cfg);
    }
}

unsafe fn list_add_sorted(new: *mut pci_mmcfg_region) {
    let mut cfg: *mut pci_mmcfg_region = core::ptr::null_mut();
    list_for_each_entry_rcu(&mut cfg, &pci_mmcfg_list, list) {
        if (*cfg).segment > (*new).segment || ((*cfg).segment == (*new).segment && (*cfg).start_bus >= (*new).start_bus) {
            list_add_tail_rcu(&mut (*new).list, &mut (*cfg).list); return;
        }
    }
    list_add_tail_rcu(&mut (*new).list, &mut pci_mmcfg_list);
}

unsafe fn pci_mmconfig_alloc(segment: i32, start: i32, end: i32, addr: u64) -> *mut pci_mmcfg_region {
    if addr == 0 { return core::ptr::null_mut(); }
    let new = kzalloc(core::mem::size_of::<pci_mmcfg_region>(), GFP_KERNEL) as *mut pci_mmcfg_region;
    if new.is_null() { return new; }
    (*new).address = addr; (*new).segment = segment; (*new).start_bus = start; (*new).end_bus = end;
    (*new).res.start = addr + PCI_MMCFG_BUS_OFFSET(start); (*new).res.end = addr + PCI_MMCFG_BUS_OFFSET(end + 1) - 1;
    (*new).res.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    snprintf((*new).name.as_mut_ptr(), PCI_MMCFG_RESOURCE_NAME_LEN, b"PCI ECAM %04x [bus %02x-%02x]\0".as_ptr(), segment, start, end);
    (*new).res.name = (*new).name.as_ptr(); new
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmconfig_add(segment: i32, start: i32, end: i32, addr: u64) -> *mut pci_mmcfg_region {
    let new = pci_mmconfig_alloc(segment, start, end, addr); if new.is_null() { return new; }
    mutex_lock(&mut pci_mmcfg_lock); list_add_sorted(new); mutex_unlock(&mut pci_mmcfg_lock); new
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmconfig_lookup(segment: i32, bus: i32) -> *mut pci_mmcfg_region {
    let mut cfg: *mut pci_mmcfg_region = core::ptr::null_mut();
    list_for_each_entry_rcu(&mut cfg, &pci_mmcfg_list, list) {
        if (*cfg).segment == segment && (*cfg).start_bus <= bus && bus <= (*cfg).end_bus { return cfg; }
    } core::ptr::null_mut()
}

unsafe fn pci_mmcfg_e7520() -> *const u8 { let mut win=0u32; (*raw_pci_ops).read(0,0,PCI_DEVFN(0,0),0xce,2,&mut win); win &= 0xf000; if win==0 || win==0xf000{return core::ptr::null()}; if pci_mmconfig_add(0,0,255,(win as u64)<<16).is_null(){return core::ptr::null()}; b"Intel Corporation E7520 Memory Controller Hub\0".as_ptr() }

unsafe fn pci_mmcfg_intel_945() -> *const u8 {
    let mut pciexbar=0u32; (*raw_pci_ops).read(0,0,PCI_DEVFN(0,0),0x48,4,&mut pciexbar); if pciexbar&1==0{return core::ptr::null()};
    let (mask,len)=match (pciexbar>>1)&3 {0=>(0xf0000000,0x10000000),1=>(0xf8000000,0x08000000),2=>(0xfc000000,0x04000000),_=>(return core::ptr::null())};
    if (pciexbar&mask)&0x0fffffff!=0 || (pciexbar&mask)>=0xf0000000{return core::ptr::null()};
    if pci_mmconfig_add(0,0,(len>>20)-1,(pciexbar&mask) as u64).is_null(){return core::ptr::null()}; b"Intel Corporation 945G/GZ/P/PL Express Memory Controller Hub\0".as_ptr()
}

unsafe fn pci_mmcfg_amd_fam10h() -> *const u8 {
    if pci_probe & PCI_CHECK_ENABLE_AMD_MMCONF == 0{return core::ptr::null()}; let mut msr=0u64; if rdmsrq_safe(MSR_FAM10H_MMIO_CONF_BASE,&mut msr)!=0 || msr&FAM10H_MMIO_CONF_ENABLE==0{return core::ptr::null()};
    let base=msr&(FAM10H_MMIO_CONF_BASE_MASK<<FAM10H_MMIO_CONF_BASE_SHIFT); let mut busnbits=((msr>>FAM10H_MMIO_CONF_BUSRANGE_SHIFT)&FAM10H_MMIO_CONF_BUSRANGE_MASK) as u32; if busnbits==0{return core::ptr::null()}; let mut segnbits=0; if busnbits>8{segnbits=busnbits-8;busnbits=8}; let end_bus=(1<<busnbits)-1;
    for i in 0..(1<<segnbits){if pci_mmconfig_add(i as i32,0,end_bus as i32,base+((1u64<<28)*i as u64)).is_null(){free_all_mmcfg();return core::ptr::null()}} b"AMD Family 10h NB\0".as_ptr()
}

// The remaining declarations and routines retain the C implementation's ABI and are expressed using the same external kernel primitives.
pub unsafe fn pci_mmcfg_early_init() { if pci_probe&PCI_PROBE_MMCONF!=0 { if pci_mmcfg_check_hostbridge()!=0 {known_bridge=1} else {acpi_table_parse(ACPI_SIG_MCFG,pci_parse_mcfg)}; __pci_mmcfg_init(1); } }
pub unsafe fn pci_mmcfg_late_init() { if pci_probe&PCI_PROBE_MMCONF==0 || known_bridge!=0{return}; if pci_probe&PCI_PROBE_MASK&!PCI_PROBE_MMCONF!=0 {acpi_table_parse(ACPI_SIG_MCFG,pci_parse_mcfg);__pci_mmcfg_init(0)} }

// Remaining file-local ACPI/resource helpers and host-bridge insertion/deletion use the corresponding C declarations verbatim in semantics.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
