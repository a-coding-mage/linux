/* SPDX-License-Identifier: GPL-2.0 */
// Dependency supplied by pcmcia/ss.h is intentionally not implemented here.

pub struct module;
pub struct cpufreq_freqs;

#[repr(C)]
pub struct soc_pcmcia_regulator {
    pub reg: *mut regulator,
    pub on: bool,
}

#[repr(C)]
pub struct pcmcia_state {
    // C bitfields, each one bit wide, packed into an unsigned int.
    pub bits: u32,
}

/*
 * This structure encapsulates per-socket state which we might need to
 * use when responding to a Card Services query of some kind.
 */
#[repr(C)]
pub struct soc_pcmcia_socket {
    pub socket: pcmcia_socket,

    /*
     * Info from low level handler
     */
    pub nr: u32,
    pub clk: *mut clk,

    /*
     * Core PCMCIA state
     */
    pub ops: *const pcmcia_low_level,

    pub status: u32,
    pub cs_state: socket_state_t,

    pub spd_io: [u16; MAX_IO_WIN],
    pub spd_mem: [u16; MAX_WIN],
    pub spd_attr: [u16; MAX_WIN],

    pub res_skt: resource,
    pub res_io: resource,
    pub res_io_io: resource,
    pub res_mem: resource,
    pub res_attr: resource,

    pub stat: [soc_pcmcia_socket_stat; 6],

    pub gpio_reset: *mut gpio_desc,
    pub gpio_bus_enable: *mut gpio_desc,
    pub vcc: soc_pcmcia_regulator,
    pub vpp: soc_pcmcia_regulator,

    pub irq_state: u32,

    // Conditional on CONFIG_CPU_FREQ in the C build.
    #[cfg(CONFIG_CPU_FREQ)]
    pub cpufreq_nb: notifier_block,
    pub poll_timer: timer_list,
    pub node: list_head,
    pub driver_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct soc_pcmcia_socket_stat {
    pub gpio: i32,
    pub desc: *mut gpio_desc,
    pub irq: u32,
    pub name: *const core::ffi::c_char,
}

pub const SOC_STAT_CD: i32 = 0; // Card detect
pub const SOC_STAT_BVD1: i32 = 1; // BATDEAD / IOSTSCHG
pub const SOC_STAT_BVD2: i32 = 2; // BATWARN / IOSPKR
pub const SOC_STAT_RDY: i32 = 3; // Ready / Interrupt
pub const SOC_STAT_VS1: i32 = 4; // Voltage sense 1
pub const SOC_STAT_VS2: i32 = 5; // Voltage sense 2

#[repr(C)]
pub struct pcmcia_low_level {
    pub owner: *mut module,

    /* first socket in system */
    pub first: i32,
    /* nr of sockets */
    pub nr: i32,

    pub hw_init: Option<unsafe extern "C" fn(*mut soc_pcmcia_socket) -> i32>,
    pub hw_shutdown: Option<unsafe extern "C" fn(*mut soc_pcmcia_socket)>,

    pub socket_state:
        Option<unsafe extern "C" fn(*mut soc_pcmcia_socket, *mut pcmcia_state)>,
    pub configure_socket:
        Option<unsafe extern "C" fn(*mut soc_pcmcia_socket, *const socket_state_t) -> i32>,

    /*
     * Enable card status IRQs on (re-)initialisation.  This can
     * be called at initialisation, power management event, or
     * pcmcia event.
     */
    pub socket_init: Option<unsafe extern "C" fn(*mut soc_pcmcia_socket)>,

    /*
     * Disable card status IRQs and PCMCIA bus on suspend.
     */
    pub socket_suspend: Option<unsafe extern "C" fn(*mut soc_pcmcia_socket)>,

    /*
     * Hardware specific timing routines.
     * If provided, the get_timing routine overrides the SOC default.
     */
    pub get_timing:
        Option<unsafe extern "C" fn(*mut soc_pcmcia_socket, u32, u32) -> u32>,
    pub set_timing: Option<unsafe extern "C" fn(*mut soc_pcmcia_socket) -> i32>,
    pub show_timing:
        Option<unsafe extern "C" fn(*mut soc_pcmcia_socket, *mut core::ffi::c_char) -> i32>,

    // Conditional on CONFIG_CPU_FREQ in the C build.
    /*
     * CPUFREQ support.
     */
    #[cfg(CONFIG_CPU_FREQ)]
    pub frequency_change:
        Option<unsafe extern "C" fn(*mut soc_pcmcia_socket, u64, *mut cpufreq_freqs) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
