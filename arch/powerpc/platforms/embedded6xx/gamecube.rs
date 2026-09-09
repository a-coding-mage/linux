// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/embedded6xx/gamecube.c
 *
 * Nintendo GameCube board-specific support
 * Copyright (C) 2004-2009 The GameCube Linux Team
 * Copyright (C) 2007,2008,2009 Albert Herranz
 */

// Kernel and architecture dependencies supplied by the surrounding repository.

extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;

    fn cpu_relax();
    fn local_irq_disable();
    fn flipper_platform_reset();
    fn ug_udbg_init();
    fn flipper_quiesce();
    fn flipper_pic_probe();
    fn flipper_pic_get_irq() -> i32;
    fn udbg_progress();
    fn of_platform_bus_probe(
        root: *mut core::ffi::c_void,
        matches: *const OfDeviceId,
        parent: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
pub struct OfDeviceId {
    pub name: *const core::ffi::c_char,
    pub type_: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

unsafe extern "C" fn gamecube_spin() -> ! {
    /* spin until power button pressed */
    loop {
        cpu_relax();
    }
}

unsafe extern "C" fn gamecube_restart(_cmd: *mut core::ffi::c_char) -> ! {
    local_irq_disable();
    flipper_platform_reset();
    gamecube_spin();
}

unsafe extern "C" fn gamecube_power_off() {
    local_irq_disable();
    gamecube_spin();
}

unsafe extern "C" fn gamecube_halt() -> ! {
    gamecube_restart(core::ptr::null_mut());
}

unsafe extern "C" fn gamecube_probe() -> i32 {
    pm_power_off = Some(gamecube_power_off);

    ug_udbg_init();

    1
}

unsafe extern "C" fn gamecube_shutdown() {
    flipper_quiesce();
}

// The define_machine macro supplies the architecture-specific machine descriptor.
define_machine!(gamecube {
    name: "gamecube",
    compatible: "nintendo,gamecube",
    probe: gamecube_probe,
    restart: gamecube_restart,
    halt: gamecube_halt,
    init_IRQ: flipper_pic_probe,
    get_irq: flipper_pic_get_irq,
    progress: udbg_progress,
    machine_shutdown: gamecube_shutdown,
});

static GAMECUBE_OF_BUS: [OfDeviceId; 2] = [
    OfDeviceId {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"nintendo,flipper".as_ptr(),
        data: core::ptr::null(),
    },
    OfDeviceId {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe extern "C" fn gamecube_device_probe() -> i32 {
    of_platform_bus_probe(
        core::ptr::null_mut(),
        GAMECUBE_OF_BUS.as_ptr(),
        core::ptr::null_mut(),
    );
    0
}

// Registers gamecube_device_probe through the architecture initcall mechanism.
machine_device_initcall!(gamecube, gamecube_device_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
