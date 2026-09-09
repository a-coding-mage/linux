// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the corresponding kernel headers are referenced by
// name below; their declarations are provided elsewhere.

pub static mut virt_bi_data: virt_booter_data = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn sprintf(str_: *mut core::ffi::c_char, format: *const core::ffi::c_char, ...);
    fn do_kernel_restart(cmd: *const core::ffi::c_char);
    fn be16_to_cpu(value: u16) -> u16;
    fn be32_to_cpup(value: *const core::ffi::c_void) -> u32;
    fn goldfish_timer_init(irq: u32, base: *mut core::ffi::c_void);
    fn snprintf(
        str_: *mut core::ffi::c_char,
        size: usize,
        format: *const core::ffi::c_char,
        ...,
    ) -> i32;
    fn setup_earlycon(name: *const core::ffi::c_char);
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    fn virt_init_IRQ();
}

unsafe fn virt_get_model(str_: *mut core::ffi::c_char) {
    /* str is 80 characters long */
    sprintf(
        str_,
        c"QEMU Virtual M68K Machine (%u.%u.%u)".as_ptr(),
        (virt_bi_data.qemu_version >> 24) as u8,
        (virt_bi_data.qemu_version >> 16) as u8,
        (virt_bi_data.qemu_version >> 8) as u8,
    );
}

unsafe fn virt_reset() {
    do_kernel_restart(core::ptr::null());
}

/*
 * Parse a virtual-m68k-specific record in the bootinfo
 */
pub unsafe extern "C" fn virt_parse_bootinfo(record: *const bi_record) -> i32 {
    let mut unknown: i32 = 0;
    let mut data = (*record).data as *const u8;

    match be16_to_cpu((*record).tag) {
        BI_VIRT_QEMU_VERSION => {
            virt_bi_data.qemu_version = be32_to_cpup(data as *const core::ffi::c_void);
        }
        BI_VIRT_GF_PIC_BASE => {
            virt_bi_data.pic.mmio = be32_to_cpup(data as *const core::ffi::c_void);
            data = data.add(4);
            virt_bi_data.pic.irq = be32_to_cpup(data as *const core::ffi::c_void);
        }
        BI_VIRT_GF_RTC_BASE => {
            virt_bi_data.rtc.mmio = be32_to_cpup(data as *const core::ffi::c_void);
            data = data.add(4);
            virt_bi_data.rtc.irq = be32_to_cpup(data as *const core::ffi::c_void);
        }
        BI_VIRT_GF_TTY_BASE => {
            virt_bi_data.tty.mmio = be32_to_cpup(data as *const core::ffi::c_void);
            data = data.add(4);
            virt_bi_data.tty.irq = be32_to_cpup(data as *const core::ffi::c_void);
        }
        BI_VIRT_CTRL_BASE => {
            virt_bi_data.ctrl.mmio = be32_to_cpup(data as *const core::ffi::c_void);
            data = data.add(4);
            virt_bi_data.ctrl.irq = be32_to_cpup(data as *const core::ffi::c_void);
        }
        BI_VIRT_VIRTIO_BASE => {
            virt_bi_data.virtio.mmio = be32_to_cpup(data as *const core::ffi::c_void);
            data = data.add(4);
            virt_bi_data.virtio.irq = be32_to_cpup(data as *const core::ffi::c_void);
        }
        _ => {
            unknown = 1;
        }
    }
    unknown
}

unsafe fn virt_sched_init() {
    goldfish_timer_init(
        virt_bi_data.rtc.irq,
        virt_bi_data.rtc.mmio as *mut core::ffi::c_void,
    );
}

pub unsafe extern "C" fn config_virt() {
    let mut earlycon = [0 as core::ffi::c_char; 24];

    snprintf(
        earlycon.as_mut_ptr(),
        core::mem::size_of_val(&earlycon),
        c"early_gf_tty,0x%08x".as_ptr(),
        virt_bi_data.tty.mmio,
    );
    setup_earlycon(earlycon.as_ptr());

    mach_init_IRQ = Some(virt_init_IRQ);
    mach_sched_init = Some(virt_sched_init);
    mach_get_model = Some(virt_get_model);
    mach_reset = Some(virt_reset);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
