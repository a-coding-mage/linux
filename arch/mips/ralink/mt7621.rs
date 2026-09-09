// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Nikolay Martynov <mar.kolya@gmail.com>
 * Copyright (C) 2015 John Crispin <john@phrozen.org>
 */

// Kernel and architecture headers from the C translation unit provide the
// external types, constants, and functions referenced below.

type U32 = u32;
type PhysAddr = usize;
type ResourceSize = u64;

#[repr(C)]
pub struct PciHostBridge {
    pub windows: ResourceList,
}
#[repr(C)]
pub struct ResourceList;
#[repr(C)]
pub struct ResourceEntry {
    pub res: *mut Resource,
}
#[repr(C)]
pub struct Resource {
    pub start: ResourceSize,
    pub end: ResourceSize,
}
#[repr(C)]
pub struct RalinkSocInfo {
    pub compatible: *const core::ffi::c_char,
    pub sys_type: [core::ffi::c_char; RAMIPS_SYS_TYPE_LEN],
    pub mem_detect: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct SocDevice;
#[repr(C)]
pub struct SocDeviceAttribute {
    pub soc_id: *const core::ffi::c_char,
    pub family: *const core::ffi::c_char,
    pub revision: *const core::ffi::c_char,
    pub data: *mut RalinkSocInfo,
}

extern "C" {
    static mut ralink_soc: U32;
    fn resource_list_first_type(windows: *mut ResourceList, flags: U32) -> *mut ResourceEntry;
    fn mips_cps_numiocu(cpu: U32) -> U32;
    fn write_gcr_reg1_base(value: ResourceSize);
    fn write_gcr_reg1_mask(value: ResourceSize);
    fn read_gcr_reg1_base() -> ResourceSize;
    fn read_gcr_reg1_mask() -> ResourceSize;
    fn panic(message: *const core::ffi::c_char) -> !;
    fn memblock_add(base: PhysAddr, size: PhysAddr);
    fn soc_device_register(attr: *mut SocDeviceAttribute) -> *mut SocDevice;
    fn kfree(ptr: *mut SocDeviceAttribute);
    fn ptr_err(ptr: *mut SocDevice) -> i32;
    fn mips_cm_probe();
    fn mips_cpc_probe();
    fn write_gcr_reg0_base(value: PhysAddr);
    fn write_gcr_reg0_mask(value: PhysAddr);
    fn register_cps_smp_ops() -> i32;
    fn register_vsmp_smp_ops() -> i32;
}

const MT7621_MEM_TEST_PATTERN: U32 = 0xaa5555aa;

static mut detect_magic: U32 = 0;
static mut soc_info_ptr: *mut RalinkSocInfo = core::ptr::null_mut();

pub unsafe extern "C" fn pcibios_root_bridge_prepare(bridge: *mut PciHostBridge) -> i32 {
    let entry = resource_list_first_type(&mut (*bridge).windows, IORESOURCE_MEM);
    if entry.is_null() {
        return -EINVAL;
    }

    if mips_cps_numiocu(0) != 0 {
        /* Hardware doesn't accept mask values with 1s after 0s (e.g. 0xffef). */
        let mask = !((*(*entry).res).end - (*(*entry).res).start) & CM_GCR_REGN_MASK_ADDRMASK;
        write_gcr_reg1_base((*(*entry).res).start);
        write_gcr_reg1_mask(mask | CM_GCR_REGN_MASK_CMTGT_IOCU0);
    }

    0
}

pub unsafe extern "C" fn mips_cpc_default_phys_base() -> PhysAddr {
    panic(b"Cannot detect cpc address\0".as_ptr() as *const core::ffi::c_char)
}

unsafe fn mt7621_addr_wraparound_test(size: PhysAddr) -> bool {
    let dm = KSEG1ADDR((&raw mut detect_magic) as *mut U32) as *mut U32;
    if CPHYSADDR(dm.add(size / core::mem::size_of::<U32>())) >= MT7621_LOWMEM_MAX_SIZE {
        return true;
    }
    core::ptr::write_volatile(dm, MT7621_MEM_TEST_PATTERN);
    if core::ptr::read_volatile(dm) != core::ptr::read_volatile(dm.add(size / 4)) {
        return false;
    }
    core::ptr::write_volatile(dm, !MT7621_MEM_TEST_PATTERN);
    core::ptr::read_volatile(dm) == core::ptr::read_volatile(dm.add(size / 4))
}

unsafe extern "C" fn mt7621_memory_detect() {
    let mut size: PhysAddr = 32 * SZ_1M;
    while size <= 256 * SZ_1M {
        if mt7621_addr_wraparound_test(size) {
            memblock_add(MT7621_LOWMEM_BASE, size);
            return;
        }
        size <<= 1;
    }
    memblock_add(MT7621_LOWMEM_BASE, MT7621_LOWMEM_MAX_SIZE);
    memblock_add(MT7621_HIGHMEM_BASE, MT7621_HIGHMEM_SIZE);
}

unsafe fn mt7621_get_soc_name0() -> U32 { __raw_readl(MT7621_SYSC_BASE + SYSC_REG_CHIP_NAME0) }
unsafe fn mt7621_get_soc_name1() -> U32 { __raw_readl(MT7621_SYSC_BASE + SYSC_REG_CHIP_NAME1) }
unsafe fn mt7621_soc_valid() -> bool {
    mt7621_get_soc_name0() == MT7621_CHIP_NAME0 && mt7621_get_soc_name1() == MT7621_CHIP_NAME1
}
unsafe fn mt7621_get_soc_id() -> &'static str { if mt7621_soc_valid() { "MT7621" } else { "invalid" } }
unsafe fn mt7621_get_soc_rev() -> U32 { __raw_readl(MT7621_SYSC_BASE + SYSC_REG_CHIP_REV) }
unsafe fn mt7621_get_soc_ver() -> U32 { (mt7621_get_soc_rev() >> CHIP_REV_VER_SHIFT) & CHIP_REV_VER_MASK }
unsafe fn mt7621_get_soc_eco() -> U32 { mt7621_get_soc_rev() & CHIP_REV_ECO_MASK }
unsafe fn mt7621_get_soc_revision() -> &'static str {
    if mt7621_get_soc_rev() == 1 && mt7621_get_soc_eco() == 1 { "E2" } else { "E1" }
}

unsafe extern "C" fn mt7621_soc_dev_init() -> i32 {
    let attr = Box::into_raw(Box::new(SocDeviceAttribute {
        soc_id: b"mt7621\0".as_ptr() as *const _, family: b"Ralink\0".as_ptr() as *const _,
        revision: mt7621_get_soc_revision().as_ptr() as *const _, data: soc_info_ptr,
    }));
    let soc_dev = soc_device_register(attr);
    if (soc_dev as isize) < 0 && (soc_dev as isize) >= -4095 { kfree(attr); return ptr_err(soc_dev); }
    0
}

pub unsafe extern "C" fn prom_soc_init(soc_info: *mut RalinkSocInfo) {
    mips_cm_probe(); mips_cpc_probe();
    if mips_cps_numiocu(0) != 0 {
        write_gcr_reg0_base(MT7621_PALMBUS_BASE);
        write_gcr_reg0_mask(!MT7621_PALMBUS_SIZE | CM_GCR_REGN_MASK_CMTGT_IOCU0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    if mt7621_soc_valid() { (*soc_info).compatible = b"mediatek,mt7621-soc\0".as_ptr() as *const _; }
    else { panic(b"mt7621: unknown SoC\0".as_ptr() as *const _); }
    ralink_soc = MT762X_SOC_MT7621AT;
    (*soc_info).mem_detect = Some(mt7621_memory_detect);
    soc_info_ptr = soc_info;
    if register_cps_smp_ops() == 0 { return; }
    if register_vsmp_smp_ops() == 0 { return; }
}

// device_initcall(mt7621_soc_dev_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
