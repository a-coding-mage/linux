// SPDX-License-Identifier: LGPL-2.0+
/* Generic MTRR (Memory Type Range Register) driver. */

// Kernel includes and the associated external definitions are supplied by the
// surrounding translation unit.

const MTRR_TO_PHYS_WC_OFFSET: i32 = 1000;

pub static mut num_var_ranges: u32 = 0;
pub static mut mtrr_usage_table: [u32; MTRR_MAX_VAR_RANGES as usize] = [0; MTRR_MAX_VAR_RANGES as usize];
pub static mut mtrr_if: *const mtrr_ops = core::ptr::null();
pub static mut changed_by_mtrr_cleanup: i32 = 0;

extern "C" {
    static mut mtrr_mutex: mutex;
    static mut boot_cpu_data: cpuinfo_x86;
    static mut phys_hi_rsvd: u32;
    static mut mtrr_state: mtrr_state_t;
    static mut generic_mtrr_ops: mtrr_ops;
    static mut memory_caching_control: u32;
    static cpu_online_mask: cpumask;

    fn pci_get_class(class: u32, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pr_info(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn mtrr_enabled() -> bool;
    fn generic_rebuild_map();
    fn stop_machine_cpuslocked(func: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                               data: *mut core::ffi::c_void, mask: *const cpumask);
    fn mtrr_attrib_to_str(ty: mtrr_type) -> *const u8;
    fn mtrr_check(base: usize, size: usize) -> i32;
    fn mtrr_cleanup() -> i32;
    fn mtrr_build_map();
    fn mtrr_copy_map();
    fn mtrr_set_if();
    fn get_mtrr_state() -> i32;
    fn mtrr_save_fixed_ranges(info: *mut core::ffi::c_void);
    fn smp_call_function_single(cpu: i32, func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);
    fn mtrr_state_warn();
    fn mtrr_register_syscore();
    fn pat_enabled() -> bool;
    fn WARN_ON(condition: bool);
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub vendor: u16, pub device: u16, pub revision: u8 }
#[repr(C)] pub struct cpuinfo_x86 { pub x86_phys_bits: u8 }
#[repr(C)] pub struct mtrr_state_t { pub enabled: bool, pub have_fixed: bool }
#[repr(C)] pub struct mtrr_ops { pub have_wrcomb: Option<unsafe extern "C" fn() -> i32>, pub var_regs: usize, pub set: unsafe extern "C" fn(u32, usize, usize, mtrr_type), pub get: unsafe extern "C" fn(i32, *mut usize, *mut usize, *mut mtrr_type), pub validate_add_page: unsafe extern "C" fn(usize, usize, u32) -> i32, pub get_free_region: unsafe extern "C" fn(usize, usize, i32) -> i32 }
pub type mtrr_type = u32;

const MTRR_TYPE_UNCACHABLE: mtrr_type = 0;
const MTRR_TYPE_WRCOMB: mtrr_type = 1;
const MTRR_TYPE_WRTHROUGH: mtrr_type = 4;
const MTRR_TYPE_WRBACK: mtrr_type = 6;
const MTRR_NUM_TYPES: u32 = 7;
const MTRR_MAX_VAR_RANGES: u32 = 256;
const MTRR_CAP_VCNT: usize = 0xff;
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const ENXIO: i32 = 6;
const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const ENOSYS: i32 = 38;
const CACHE_MTRR: u32 = 1;

#[repr(C)] struct set_mtrr_data { smp_base: usize, smp_size: usize, smp_reg: u32, smp_type: mtrr_type }

unsafe extern "C" fn mtrr_rendezvous_handler(info: *mut core::ffi::c_void) -> i32 {
    let data = &*(info as *mut set_mtrr_data);
    ((*mtrr_if).set)(data.smp_reg, data.smp_base, data.smp_size, data.smp_type);
    0
}

unsafe fn have_wrcomb() -> i32 {
    let dev = pci_get_class((PCI_CLASS_BRIDGE_HOST << 8) as u32, core::ptr::null_mut());
    if !dev.is_null() {
        if (*dev).vendor == PCI_VENDOR_ID_SERVERWORKS && (*dev).device == PCI_DEVICE_ID_SERVERWORKS_LE && (*dev).revision <= 5 { pci_dev_put(dev); return 0; }
        if (*dev).vendor == PCI_VENDOR_ID_INTEL && (*dev).device == PCI_DEVICE_ID_INTEL_82451NX { pci_dev_put(dev); return 0; }
        pci_dev_put(dev);
    }
    (*mtrr_if).have_wrcomb.map(|f| f()).unwrap_or(0)
}

unsafe fn init_table() { for i in 0..num_var_ranges as usize { mtrr_usage_table[i] = 1; } }

unsafe fn types_compatible(type1: mtrr_type, type2: mtrr_type) -> bool { type1 == MTRR_TYPE_UNCACHABLE || type2 == MTRR_TYPE_UNCACHABLE || (type1 == MTRR_TYPE_WRTHROUGH && type2 == MTRR_TYPE_WRBACK) || (type1 == MTRR_TYPE_WRBACK && type2 == MTRR_TYPE_WRTHROUGH) }

unsafe fn set_mtrr(reg: u32, base: usize, size: usize, ty: mtrr_type) {
    let mut data = set_mtrr_data { smp_reg: reg, smp_base: base, smp_size: size, smp_type: ty };
    stop_machine_cpuslocked(mtrr_rendezvous_handler, &mut data as *mut _ as *mut _, &cpu_online_mask);
    generic_rebuild_map();
}

pub unsafe fn mtrr_add_page(base: usize, size: usize, ty: u32, increment: bool) -> i32 {
    if !mtrr_enabled() { return -ENXIO; }
    let error = ((*mtrr_if).validate_add_page)(base, size, ty); if error != 0 { return error; }
    if ty >= MTRR_NUM_TYPES || size == 0 || ((base | (base + size - 1)) >> (boot_cpu_data.x86_phys_bits as usize - PAGE_SHIFT)) != 0 { return -EINVAL; }
    let mut lbase = 0usize; let mut lsize = 0usize; let mut ltype = 0; let mut replace = -1; let mut error = -EINVAL;
    for i in 0..num_var_ranges as i32 { ((*mtrr_if).get)(i, &mut lbase, &mut lsize, &mut ltype); if lsize == 0 || base > lbase + lsize - 1 || base + size - 1 < lbase { continue; } if base < lbase || base + size - 1 > lbase + lsize - 1 { if base <= lbase && base + size - 1 >= lbase + lsize - 1 && ty == ltype { replace = if replace == -1 { i } else { -2 }; continue; } if !types_compatible(ty, ltype) { break; } continue; } if ltype != ty { if types_compatible(ty, ltype) { continue; } break; } if increment { mtrr_usage_table[i as usize] += 1; } return i; }
    let i = ((*mtrr_if).get_free_region)(base, size, replace); if i >= 0 { set_mtrr(i as u32, base, size, ty); mtrr_usage_table[i as usize] = if replace < 0 { 1 } else { mtrr_usage_table[replace as usize] + increment as u32 }; if replace >= 0 && replace != i { set_mtrr(replace as u32, 0, 0, 0); mtrr_usage_table[replace as usize] = 0; } } error = i; error
}

pub unsafe fn mtrr_add(base: usize, size: usize, ty: u32, increment: bool) -> i32 { if !mtrr_enabled() { return -ENODEV; } if (base & (PAGE_SIZE - 1)) != 0 || (size & (PAGE_SIZE - 1)) != 0 { return -EINVAL; } mtrr_add_page(base >> PAGE_SHIFT, size >> PAGE_SHIFT, ty, increment) }

pub unsafe fn mtrr_del_page(mut reg: i32, base: usize, size: usize) -> i32 { if !mtrr_enabled() { return -ENODEV; } let max = num_var_ranges as i32; let mut lbase=0; let mut lsize=0; let mut ltype=0; if reg < 0 { for i in 0..max { ((*mtrr_if).get)(i,&mut lbase,&mut lsize,&mut ltype); if lbase==base && lsize==size { reg=i; break; } } } if reg < 0 || reg >= max { return -EINVAL; } ((*mtrr_if).get)(reg,&mut lbase,&mut lsize,&mut ltype); if lsize < 1 || mtrr_usage_table[reg as usize] < 1 { return -EINVAL; } mtrr_usage_table[reg as usize] -= 1; if mtrr_usage_table[reg as usize] < 1 { set_mtrr(reg as u32,0,0,0); } reg }
pub unsafe fn mtrr_del(reg: i32, base: usize, size: usize) -> i32 { if !mtrr_enabled() { return -ENODEV; } if (base & (PAGE_SIZE-1)) != 0 || (size & (PAGE_SIZE-1)) != 0 { return -EINVAL; } mtrr_del_page(reg,base>>PAGE_SHIFT,size>>PAGE_SHIFT) }
pub unsafe fn arch_phys_wc_add(base: usize, size: usize) -> i32 { if pat_enabled() || !mtrr_enabled() { return 0; } let ret=mtrr_add(base,size,MTRR_TYPE_WRCOMB,true); if ret<0 { return ret; } ret + MTRR_TO_PHYS_WC_OFFSET }
pub unsafe fn arch_phys_wc_del(handle: i32) { if handle >= 1 { WARN_ON(handle < MTRR_TO_PHYS_WC_OFFSET); mtrr_del(handle-MTRR_TO_PHYS_WC_OFFSET,0,0); } }
pub fn arch_phys_wc_index(handle: i32) -> i32 { if handle < MTRR_TO_PHYS_WC_OFFSET {-1} else {handle-MTRR_TO_PHYS_WC_OFFSET} }

extern "C" { static PCI_CLASS_BRIDGE_HOST: u32; static PCI_VENDOR_ID_SERVERWORKS: u16; static PCI_DEVICE_ID_SERVERWORKS_LE: u16; static PCI_VENDOR_ID_INTEL: u16; static PCI_DEVICE_ID_INTEL_82451NX: u16; }

pub unsafe fn mtrr_bp_init() {
    let generic_mtrrs = true; // cpu_feature_enabled(X86_FEATURE_MTRR)
    let mut why = "(not available)";
    phys_hi_rsvd = if boot_cpu_data.x86_phys_bits >= 32 { u32::MAX >> (64 - boot_cpu_data.x86_phys_bits as u32) } else { 0 };
    if !generic_mtrrs && mtrr_state.enabled { init_table(); mtrr_build_map(); return; }
    if generic_mtrrs { mtrr_if = &raw mut generic_mtrr_ops; } else { mtrr_set_if(); }
    if mtrr_enabled() {
        num_var_ranges = if mtrr_if == (&raw mut generic_mtrr_ops) { MTRR_CAP_VCNT as u32 } else { (*mtrr_if).var_regs as u32 };
        init_table();
        if mtrr_if == (&raw mut generic_mtrr_ops) {
            if get_mtrr_state() != 0 { memory_caching_control |= CACHE_MTRR; changed_by_mtrr_cleanup = mtrr_cleanup(); mtrr_build_map(); }
            else { mtrr_if = core::ptr::null(); why = "by BIOS"; }
        }
    }
    if !mtrr_enabled() { let _ = why; }
}

pub unsafe fn mtrr_save_state() { if !mtrr_enabled() || !mtrr_state.have_fixed { return; } smp_call_function_single(0, mtrr_save_fixed_ranges, core::ptr::null_mut(), 1); }

pub unsafe fn mtrr_init_finalize() -> i32 {
    mtrr_copy_map();
    if !mtrr_enabled() { return 0; }
    if memory_caching_control & CACHE_MTRR != 0 { if changed_by_mtrr_cleanup == 0 { mtrr_state_warn(); } return 0; }
    mtrr_register_syscore();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
