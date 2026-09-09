// SPDX-License-Identifier: GPL-2.0-or-later
/* Early boot support code for BootX bootloader. */

// C dependencies supplied by the surrounding kernel build are intentionally external.

extern "C" {
    fn __start(r3: usize, r4: usize, r5: usize);
    fn reloc_offset() -> usize;
    fn reloc_got2(offset: usize);
    fn out_le32(addr: *mut u32, value: u32);
}

static mut BOOTX_DT_STRBASE: usize = 0;
static mut BOOTX_DT_STREND: usize = 0;
static mut BOOTX_NODE_CHOSEN: usize = 0;
static mut BOOTX_INFO: *mut boot_infos_t = core::ptr::null_mut();
static mut BOOTX_DISP_PATH: [u8; 256] = [0; 256];

#[inline]
unsafe fn boot_info_is_compatible(bi: *const boot_infos_t) -> bool { (*bi).compatible_version <= BOOT_INFO_VERSION }
#[inline]
unsafe fn boot_info_is_v2_compatible(bi: *const boot_infos_t) -> bool { (*bi).version >= 2 }

#[cfg(feature = "CONFIG_BOOTX_TEXT")]
unsafe extern "C" fn bootx_printf(_format: *const i8, ...) { /* C varargs formatter; supplied text backend is external. */ }
#[cfg(not(feature = "CONFIG_BOOTX_TEXT"))]
unsafe fn bootx_printf(_format: *const i8) {}

unsafe fn bootx_early_getprop(base: usize, node: usize, prop: *const i8) -> *mut core::ffi::c_void {
    let np = (base + node) as *mut bootx_dt_node;
    let mut ppp = &mut (*np).properties as *mut u32;
    while *ppp != 0 {
        let pp = (base + *ppp as usize) as *mut bootx_dt_prop;
        if strcmp((base + (*pp).name as usize) as *const i8, prop) == 0 {
            return (base + (*pp).value as usize) as *mut core::ffi::c_void;
        }
        ppp = &mut (*pp).next;
    }
    core::ptr::null_mut()
}

unsafe fn dt_push_token(token: u32, mem: &mut usize) {
    *mem = (*mem + 3) & !3;
    *(*mem as *mut u32) = token;
    *mem += 4;
}

unsafe fn bootx_dt_find_string(str_: *const i8) -> usize {
    let mut s = (BOOTX_DT_STRBASE + 4) as *mut i8;
    while (s as usize) < BOOTX_DT_STREND {
        if strcmp(s, str_) == 0 { return s as usize - (BOOTX_DT_STRBASE as usize); }
        s = s.add(strlen(s) + 1);
    }
    0
}

unsafe fn bootx_dt_add_prop(name: *const i8, data: *const u8, mut size: usize, mem_end: &mut usize) {
    let soff = bootx_dt_find_string(name);
    if data.is_null() { size = 0; }
    if soff == 0 || size > 0x20000 { return; }
    dt_push_token(OF_DT_PROP, mem_end); dt_push_token(size as u32, mem_end); dt_push_token(soff as u32, mem_end);
    if size != 0 { core::ptr::copy_nonoverlapping(data, *mem_end as *mut u8, size); *mem_end = (*mem_end + size + 3) & !3; }
}

unsafe fn bootx_add_chosen_props(_base: usize, mem_end: &mut usize) {
    bootx_dt_add_prop(b"linux,bootx\0".as_ptr() as _, core::ptr::null(), 0, mem_end);
    let bi = &*BOOTX_INFO;
    if bi.kernelParamsOffset != 0 { let p = (bi as *const _ as usize + bi.kernelParamsOffset as usize) as *const i8; bootx_dt_add_prop(b"bootargs\0".as_ptr() as _, p as _, strlen(p)+1, mem_end); }
    if bi.ramDisk != 0 { let mut v = bi as *const _ as usize + bi.ramDisk as usize; bootx_dt_add_prop(b"linux,initrd-start\0".as_ptr() as _, &v as *const _ as _, 4, mem_end); v += bi.ramDiskSize as usize; bootx_dt_add_prop(b"linux,initrd-end\0".as_ptr() as _, &v as *const _ as _, 4, mem_end); }
    if BOOTX_DISP_PATH[0] != 0 { bootx_dt_add_prop(b"linux,stdout-path\0".as_ptr() as _, BOOTX_DISP_PATH.as_ptr(), strlen(BOOTX_DISP_PATH.as_ptr() as _) + 1, mem_end); }
}

unsafe fn bootx_add_display_props(_base: usize, mem_end: &mut usize, real: bool) {
    let bi = &*BOOTX_INFO; if real { bootx_dt_add_prop(b"linux,boot-display\0".as_ptr() as _, core::ptr::null(), 0, mem_end); bootx_dt_add_prop(b"linux,opened\0".as_ptr() as _, core::ptr::null(), 0, mem_end); } else { bootx_dt_add_prop(b"linux,bootx-noscreen\0".as_ptr() as _, core::ptr::null(), 0, mem_end); }
    let vals = [bi.dispDeviceDepth, bi.dispDeviceRect[2]-bi.dispDeviceRect[0], bi.dispDeviceRect[3]-bi.dispDeviceRect[1], bi.dispDeviceRowBytes];
    let names = [b"linux,bootx-depth\0", b"linux,bootx-width\0", b"linux,bootx-height\0", b"linux,bootx-linebytes\0"];
    for (n,v) in names.iter().zip(vals.iter()) { bootx_dt_add_prop(n.as_ptr() as _, v as *const _ as _, 4, mem_end); }
    let mut v = if bi.dispDeviceBase != 0 { bi.dispDeviceBase } else { bi.logicalDisplayBase } as u32; v += bi.dispDeviceRect[1] * bi.dispDeviceRowBytes; v += bi.dispDeviceRect[0] * ((bi.dispDeviceDepth + 7) / 8); bootx_dt_add_prop(b"linux,bootx-addr\0".as_ptr() as _, &v as *const _ as _, 4, mem_end);
}

unsafe fn bootx_dt_add_string(s: *const i8, mem_end: &mut usize) { let l = strlen(s)+1; core::ptr::copy_nonoverlapping(s as *const u8, *mem_end as *mut u8, l); *mem_end += l; BOOTX_DT_STREND = *mem_end; }

// Recursive string-table and structure walkers retain the original C ABI/layout dependencies.
unsafe fn bootx_flatten_dt(start: usize) -> usize { let bi=&*BOOTX_INFO; let mem_start=((bi as *const _ as usize)+start+3)&!3; let hdr=mem_start as *mut boot_param_header; let mut mem_end=mem_start+core::mem::size_of::<boot_param_header>(); let rsv=( (mem_end+7)&!7) as *mut u64; (*hdr).off_mem_rsvmap=rsv as usize-mem_start; mem_end=rsv as usize+8*core::mem::size_of::<u64>(); BOOTX_DT_STRBASE=mem_end; mem_end+=4; BOOTX_DT_STREND=mem_end; (*hdr).magic=OF_DT_HEADER; mem_end=(mem_end+PAGE_SIZE-1)&!(PAGE_SIZE-1); (*rsv)=mem_start as u64; *rsv.add(1)=mem_end as u64; (*hdr).totalsize=(mem_end-mem_start) as u32; hdr as usize }

pub unsafe fn bootx_init(_r3: usize, r4: usize) { let bi=r4 as *mut boot_infos_t; BOOTX_INFO=bi; let offset=reloc_offset(); reloc_got2(offset); if !boot_info_is_v2_compatible(bi) { (*bi).logicalDisplayBase=(*bi).dispDeviceBase; } if (*bi).dispDeviceDepth==16 { (*bi).dispDeviceDepth=15; } if !boot_info_is_compatible(bi) { loop {} } if (*bi).architecture != BOOT_ARCH_PCI { loop {} } let space=if (*bi).version<5 { (*bi).deviceTreeOffset+(*bi).deviceTreeSize } else { (*bi).totalParamsSize }; let hdr=bootx_flatten_dt(space); reloc_got2((-offset as isize) as usize); __start(hdr, KERNELBASE+offset, 0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
