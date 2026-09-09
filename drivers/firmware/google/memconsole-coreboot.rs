// SPDX-License-Identifier: GPL-2.0-only
/*
 * memconsole-coreboot.c
 *
 * Memory based BIOS console accessed through coreboot table.
 *
 * Copyright 2017 Google Inc.
 */

// Dependencies supplied by the kernel and the surrounding driver sources are
// intentionally left as external Rust declarations.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const CB_TAG_CBMEM_CONSOLE: u32 = 0x17;

/* CBMEM firmware console log descriptor. */
#[repr(C, packed)]
struct CbmemCons {
    size_dont_access_after_boot: u32,
    cursor: u32,
    body: [u8; 0],
}

const CURSOR_MASK: u32 = (1u32 << 28) - 1;
const OVERFLOW: u32 = 1u32 << 31;

static mut CBMEM_CONSOLE: *mut CbmemCons = ptr::null_mut();
static mut CBMEM_CONSOLE_SIZE: u32 = 0;

/* External declarations corresponding to symbols supplied by other files. */
#[repr(C)]
pub struct CorebootDevice {
    pub dev: Device,
    pub cbmem_ref: CbmemRef,
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CbmemRef {
    pub cbmem_addr: usize,
}

#[repr(C)]
pub struct CorebootDeviceId {
    pub tag: u32,
}

#[repr(C)]
pub struct CorebootDriver {
    pub probe: Option<unsafe extern "C" fn(*mut CorebootDevice) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut CorebootDevice)>,
    pub drv: Driver,
    pub id_table: *const CorebootDeviceId,
}

#[repr(C)]
pub struct Driver {
    pub name: *const c_char,
}

extern "C" {
    fn memremap(addr: usize, size: usize, flags: c_int) -> *mut c_void;
    fn devm_memremap(dev: *mut Device, addr: usize, size: usize, flags: c_int) -> *mut CbmemCons;
    fn memunmap(addr: *mut CbmemCons);
    fn is_err(ptr: *const c_void) -> bool;
    fn ptr_err(ptr: *const c_void) -> c_int;
    fn memory_read_from_buffer(
        to: *mut c_char,
        count: usize,
        ppos: *mut i64,
        from: *const u8,
        available: usize,
    ) -> usize;
    fn memconsole_setup(read: Option<unsafe extern "C" fn(*mut c_char, i64, usize) -> isize>);
    fn memconsole_sysfs_init() -> c_int;
    fn memconsole_exit();
    fn module_coreboot_driver(driver: *mut CorebootDriver);
}

const MEMREMAP_WB: c_int = 1;

/*
 * The cbmem_console structure is read again on every access because it may
 * change at any time if runtime firmware logs new messages. This may rarely
 * lead to race conditions where the firmware overwrites the beginning of the
 * ring buffer with more lines after we have already read |cursor|. It should
 * be rare and harmless enough that we don't spend extra effort working around it.
 */
unsafe extern "C" fn memconsole_coreboot_read(
    buf: *mut c_char,
    mut pos: i64,
    count: usize,
) -> isize {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Seg {
        /* describes ring buffer segments in logical order */
        phys: u32, /* physical offset from start of mem buffer */
        len: u32,  /* length of segment */
    }

    let cursor_value = (*CBMEM_CONSOLE).cursor;
    let mut cursor = cursor_value & CURSOR_MASK;
    let flags = cursor_value & !CURSOR_MASK;
    let size = CBMEM_CONSOLE_SIZE;
    let mut seg = [Seg { phys: 0, len: 0 }; 2];
    let mut done: usize = 0;

    if flags & OVERFLOW != 0 {
        if cursor > size {
            /* Shouldn't really happen, but... */
            cursor = 0;
        }
        seg[0] = Seg { phys: cursor, len: size - cursor };
        seg[1] = Seg { phys: 0, len: cursor };
    } else {
        seg[0] = Seg { phys: 0, len: core::cmp::min(cursor, size) };
    }

    let mut i = 0usize;
    while i < seg.len() && count > done {
        let body = (*CBMEM_CONSOLE).body.as_ptr().add(seg[i].phys as usize);
        done += memory_read_from_buffer(
            buf.add(done),
            count - done,
            &mut pos,
            body,
            seg[i].len as usize,
        );
        pos -= seg[i].len as i64;
        i += 1;
    }
    done as isize
}

unsafe extern "C" fn memconsole_probe(dev: *mut CorebootDevice) -> c_int {
    let tmp_cbmc = memremap(
        (*dev).cbmem_ref.cbmem_addr,
        core::mem::size_of::<CbmemCons>(),
        MEMREMAP_WB,
    ) as *mut CbmemCons;

    if tmp_cbmc.is_null() {
        return -12; // -ENOMEM
    }

    /* Read size only once to prevent overrun attack through /dev/mem. */
    CBMEM_CONSOLE_SIZE = (*tmp_cbmc).size_dont_access_after_boot;
    CBMEM_CONSOLE = devm_memremap(
        &mut (*dev).dev,
        (*dev).cbmem_ref.cbmem_addr,
        CBMEM_CONSOLE_SIZE as usize + core::mem::size_of::<CbmemCons>(),
        MEMREMAP_WB,
    );
    memunmap(tmp_cbmc);

    if is_err(CBMEM_CONSOLE as *const c_void) {
        return ptr_err(CBMEM_CONSOLE as *const c_void);
    }

    memconsole_setup(Some(memconsole_coreboot_read));
    memconsole_sysfs_init()
}

unsafe extern "C" fn memconsole_remove(_dev: *mut CorebootDevice) {
    memconsole_exit();
}

static MEMCONSOLE_IDS: [CorebootDeviceId; 2] = [
    CorebootDeviceId { tag: CB_TAG_CBMEM_CONSOLE },
    CorebootDeviceId { tag: 0 }, /* sentinel */
];

static mut MEMCONSOLE_DRIVER: CorebootDriver = CorebootDriver {
    probe: Some(memconsole_probe),
    remove: Some(memconsole_remove),
    drv: Driver {
        name: b"memconsole\0".as_ptr() as *const c_char,
    },
    id_table: MEMCONSOLE_IDS.as_ptr(),
};

// MODULE_DEVICE_TABLE(coreboot, memconsole_ids);
// module_coreboot_driver(memconsole_driver);
// MODULE_AUTHOR("Google, Inc.");
// MODULE_DESCRIPTION("Memory based BIOS console accessed through coreboot table");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
