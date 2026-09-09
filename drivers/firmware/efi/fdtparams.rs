// SPDX-License-Identifier: GPL-2.0-only

// #define pr_fmt(fmt) "efi: " fmt
// C dependencies supplied by the surrounding kernel translation unit are
// intentionally referenced here without local implementations.

#[repr(usize)]
enum Param {
    Systab,
    Mmbase,
    Mmsize,
    Dcsize,
    Dcvers,
    Paramcount,
}

static NAME: [&str; Param::Paramcount as usize] = [
    "System Table         ",
    "MemMap Address       ",
    "MemMap Size          ",
    "MemMap Desc. Size    ",
    "MemMap Desc. Version ",
];

struct DtParams {
    path: &'static str,
    paravirt: u8,
    params: [&'static str; Param::Paramcount as usize],
}

static DT_PARAMS: &[DtParams] = &[
    #[cfg(feature = "CONFIG_XEN")]
    DtParams {
        path: "/hypervisor/uefi",
        paravirt: 1,
        params: [
            "xen,uefi-system-table",
            "xen,uefi-mmap-start",
            "xen,uefi-mmap-size",
            "xen,uefi-mmap-desc-size",
            "xen,uefi-mmap-desc-ver",
        ],
    },
    DtParams {
        path: "/chosen",
        paravirt: 0,
        params: [
            "linux,uefi-system-table",
            "linux,uefi-mmap-start",
            "linux,uefi-mmap-size",
            "linux,uefi-mmap-desc-size",
            "linux,uefi-mmap-desc-ver",
        ],
    },
];

extern "C" {
    fn fdt_getprop(fdt: *const core::ffi::c_void, node: i32, name: *const i8,
                   lenp: *mut i32) -> *const core::ffi::c_void;
    fn be32_to_cpup(p: *const core::ffi::c_void) -> u32;
    fn get_unaligned_be64(p: *const core::ffi::c_void) -> u64;
    fn efi_enabled(feature: u32) -> bool;
    fn fdt_path_offset(fdt: *const core::ffi::c_void, path: *const i8) -> i32;
    fn set_bit(nr: u32, addr: *mut u64);
}

#[repr(C)]
pub struct EfiMemoryMapData {
    pub phys_map: u64,
    pub size: usize,
    pub desc_size: usize,
    pub desc_version: u32,
}

extern "C" {
    static initial_boot_params: *const core::ffi::c_void;
    static mut efi: EfiState;
}

#[repr(C)]
struct EfiState {
    pub flags: u64,
}

const EFI_DBG: u32 = 0;
const EFI_PARAVIRT: u32 = 0;
const U32_MAX: u64 = 0xffff_ffff;

unsafe fn efi_get_fdt_prop(
    fdt: *const core::ffi::c_void,
    node: i32,
    pname: *const i8,
    _rname: *const i8,
    var: *mut core::ffi::c_void,
    size: i32,
) -> i32 {
    let mut len = 0i32;
    let prop = fdt_getprop(fdt, node, pname, &mut len);
    if prop.is_null() {
        return 1;
    }

    let val = if len == 4 {
        be32_to_cpup(prop) as u64
    } else {
        get_unaligned_be64(prop)
    };

    if size == 8 {
        *(var as *mut u64) = val;
    } else {
        *(var as *mut u32) = if val < U32_MAX { val as u32 } else { U32_MAX as u32 };
    }

    // if (efi_enabled(EFI_DBG)) pr_info("  %s: 0x%0*llx\\n", rname, size * 2, val);
    0
}

pub unsafe fn efi_get_fdt_params(mm: *mut EfiMemoryMapData) -> u64 {
    let fdt = initial_boot_params;
    let mut systab: usize = 0;
    let target: [(*mut core::ffi::c_void, i32); Param::Paramcount as usize] = [
        (&mut systab as *mut usize).cast(),
        (&mut (*mm).phys_map as *mut u64).cast(),
        (&mut (*mm).size as *mut usize).cast(),
        (&mut (*mm).desc_size as *mut usize).cast(),
        (&mut (*mm).desc_version as *mut u32).cast(),
    ];

    if fdt.is_null() {
        return 0;
    }

    for dt in DT_PARAMS {
        let node = fdt_path_offset(fdt, dt.path.as_ptr() as *const i8);
        if node < 0 {
            continue;
        }

        // if (efi_enabled(EFI_DBG)) pr_info("Getting UEFI parameters from %s in DT:\\n", dt.path);
        let mut notfound = false;
        for j in 0..target.len() {
            let pname = dt.params[j];
            if efi_get_fdt_prop(fdt, node, pname.as_ptr() as *const i8,
                                NAME[j].as_ptr() as *const i8,
                                target[j].0, target[j].1) == 0 {
                continue;
            }
            if j == 0 {
                // goto notfound
                notfound = true;
                break;
            }
            // pr_err("Can't find property '%s' in DT!\\n", pname);
            return 0;
        }
        if notfound {
            break;
        }
        if dt.paravirt != 0 {
            set_bit(EFI_PARAVIRT, &mut efi.flags);
        }
        return systab as u64;
    }

    // notfound: pr_info("UEFI not found.\\n");
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
