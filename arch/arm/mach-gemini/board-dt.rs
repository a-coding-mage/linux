// SPDX-License-Identifier: GPL-2.0
/*
 * Gemini Device Tree boot support
 */

// The following names and constants are supplied by the kernel headers and
// build configuration in the surrounding translation unit.

#[cfg(CONFIG_DEBUG_GEMINI)]
#[repr(C)]
struct map_desc {
    virtual_: usize,
    pfn: usize,
    length: usize,
    type_: usize,
}

#[cfg(CONFIG_DEBUG_GEMINI)]
extern "C" {
    fn iotable_init(io_desc: *mut map_desc, size: usize);
    fn __phys_to_pfn(phys: usize) -> usize;
}

extern "C" {
    static mut arm_pm_idle: Option<unsafe extern "C" fn()>;
    fn raw_local_irq_enable();
    fn raw_local_irq_disable();
    fn cpu_do_idle();
}

#[cfg(CONFIG_DEBUG_GEMINI)]
static mut gemini_io_desc: [map_desc; 1] = [map_desc {
    virtual_: CONFIG_DEBUG_UART_VIRT,
    pfn: 0, // Initialized below to preserve __phys_to_pfn(CONFIG_DEBUG_UART_PHYS).
    length: SZ_4K,
    type_: MT_DEVICE,
}];

#[cfg(CONFIG_DEBUG_GEMINI)]
unsafe fn gemini_map_io() {
    gemini_io_desc[0].pfn = __phys_to_pfn(CONFIG_DEBUG_UART_PHYS);
    iotable_init(gemini_io_desc.as_mut_ptr(), gemini_io_desc.len());
}

#[cfg(not(CONFIG_DEBUG_GEMINI))]
const gemini_map_io: Option<unsafe fn()> = None;

unsafe extern "C" fn gemini_idle() {
    /*
     * Because of broken hardware we have to enable interrupts or the CPU
     * will never wakeup... Actually it is not very good to enable
     * interrupts first since scheduler can miss a tick, but there is
     * no other way around this. Platforms that needs it for power saving
     * should enable it in init code, since by default it is
     * disabled.
     */

    /* FIXME: Enabling interrupts here is racy! */
    raw_local_irq_enable();
    cpu_do_idle();
    raw_local_irq_disable();
}

unsafe extern "C" fn gemini_init_machine() {
    arm_pm_idle = Some(gemini_idle);
}

static mut gemini_board_compat: [*const core::ffi::c_char; 2] = [
    b"cortina,gemini\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(GEMINI_DT, "Gemini (Device Tree)")
//     .map_io       = gemini_map_io,
//     .init_machine = gemini_init_machine,
//     .dt_compat    = gemini_board_compat,
// MACHINE_END
#[repr(C)]
struct GeminiDtMachine {
    map_io: Option<unsafe fn()>,
    init_machine: unsafe extern "C" fn(),
    dt_compat: *mut *const core::ffi::c_char,
}

#[cfg(CONFIG_DEBUG_GEMINI)]
static GEMINI_DT: GeminiDtMachine = GeminiDtMachine {
    map_io: Some(gemini_map_io),
    init_machine: gemini_init_machine,
    dt_compat: unsafe { gemini_board_compat.as_mut_ptr() },
};

#[cfg(not(CONFIG_DEBUG_GEMINI))]
static GEMINI_DT: GeminiDtMachine = GeminiDtMachine {
    map_io: gemini_map_io,
    init_machine: gemini_init_machine,
    dt_compat: unsafe { gemini_board_compat.as_mut_ptr() },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
