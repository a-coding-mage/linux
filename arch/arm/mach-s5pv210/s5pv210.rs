// SPDX-License-Identifier: GPL-2.0
//
// Samsung's S5PC110/S5PV210 flattened device tree enabled machine.
//
// Copyright (c) 2013-2014 Samsung Electronics Co., Ltd.
// Mateusz Krawczuk <m.krawczuk@partner.samsung.com>
// Tomasz Figa <t.figa@samsung.com>

// Dependencies supplied by the Linux kernel and by the corresponding
// architecture sources are intentionally left as external Rust items.

unsafe fn s5pv210_fdt_map_sys(
    node: libc::c_ulong,
    _uname: *const libc::c_char,
    _depth: libc::c_int,
    _data: *mut libc::c_void,
) -> libc::c_int {
    let mut iodesc: map_desc = core::mem::zeroed();
    let mut reg: *const __be32;
    let mut len: libc::c_int = 0;

    if of_flat_dt_is_compatible(node, c"samsung,s5pv210-clock".as_ptr()) == 0 {
        return 0;
    }

    reg = of_get_flat_dt_prop(node, c"reg".as_ptr(), &mut len);
    if reg.is_null() || len != (core::mem::size_of::<libc::c_ulong>() * 2) as libc::c_int {
        return 0;
    }

    iodesc.pfn = __phys_to_pfn(u32::from_be(core::ptr::read(reg)) as _);
    iodesc.length = u32::from_be(core::ptr::read(reg.add(1))) as _ - 1;
    iodesc.virtual_ = S3C_VA_SYS as libc::c_ulong;
    iodesc.type_ = MT_DEVICE;
    iotable_init(&mut iodesc, 1);

    1
}

unsafe fn s5pv210_dt_map_io() {
    debug_ll_io_init();
    of_scan_flat_dt(Some(s5pv210_fdt_map_sys), core::ptr::null_mut());
}

unsafe fn s5pv210_dt_restart(_mode: reboot_mode, _cmd: *const libc::c_char) {
    __raw_writel(0x1, S5P_SWRESET);
}

unsafe fn s5pv210_dt_init_late() {
    platform_device_register_simple(c"s5pv210-cpufreq".as_ptr(), -1, core::ptr::null(), 0);
    s5pv210_pm_init();
}

static S5PV210_DT_COMPAT: [*const libc::c_char; 3] = [
    c"samsung,s5pc110".as_ptr(),
    c"samsung,s5pv210".as_ptr(),
    core::ptr::null(),
];

// Equivalent to DT_MACHINE_START(S5PV210_DT,
// "Samsung S5PC110/S5PV210-based board") ... MACHINE_END.
#[no_mangle]
pub static mut S5PV210_DT: machine_desc = machine_desc {
    dt_compat: S5PV210_DT_COMPAT.as_ptr(),
    map_io: Some(s5pv210_dt_map_io),
    restart: Some(s5pv210_dt_restart),
    init_late: Some(s5pv210_dt_init_late),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
