// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the C includes are supplied by other files.

use core::ffi::c_char;

extern "C" {
    fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> usize;
    fn create_dev(name: *const c_char, dev: usize);
    fn rd_load_image() -> i32;
    fn init_unlink(name: *const c_char);
}

// Root_RAM0 is supplied by the kernel headers.
extern "C" {
    static Root_RAM0: usize;
}

#[no_mangle]
pub static mut initrd_start: usize = 0;
#[no_mangle]
pub static mut initrd_end: usize = 0;
#[no_mangle]
pub static mut initrd_below_start_ok: i32 = 0;
static mut mount_initrd: i32 = 1;

#[no_mangle]
pub static mut phys_initrd_start: usize = 0;
#[no_mangle]
pub static mut phys_initrd_size: usize = 0;

unsafe fn no_initrd(_str: *mut c_char) -> i32 {
    // pr_warn("noinitrd option is deprecated and will be removed soon\\n");
    mount_initrd = 0;
    1
}

// __setup("noinitrd", no_initrd);

unsafe fn early_initrdmem(p: *mut c_char) -> i32 {
    let mut endp: *mut c_char = core::ptr::null_mut();

    let start = memparse(p, &mut endp);
    if !endp.is_null() && *endp == b',' as c_char {
        let size = memparse(endp.add(1), core::ptr::null_mut());

        phys_initrd_start = start;
        phys_initrd_size = size;
    }
    0
}

// early_param("initrdmem", early_initrdmem);

unsafe fn early_initrd(p: *mut c_char) -> i32 {
    early_initrdmem(p)
}

// early_param("initrd", early_initrd);

#[no_mangle]
pub unsafe fn initrd_load() {
    if mount_initrd != 0 {
        create_dev(b"/dev/ram\0".as_ptr() as *const c_char, Root_RAM0);
        /*
         * Load the initrd data into /dev/ram0.
         */
        if rd_load_image() != 0 {
            // pr_warn!("using deprecated initrd support, will be removed in January 2027; \
            // use initramfs instead or (as a last resort) /sys/firmware/initrd; \
            // see section \"Workaround\" in \
            // https://lore.kernel.org/lkml/20251010094047.3111495-1-safinaskar@gmail.com\\n");
        }
    }
    init_unlink(b"/initrd.image\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
