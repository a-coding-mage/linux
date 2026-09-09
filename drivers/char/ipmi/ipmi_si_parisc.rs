// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parisc_hpa {
    pub start: c_ulong,
}

#[repr(C)]
pub struct parisc_device {
    pub hpa: parisc_hpa,
    pub dev: device,
}

#[repr(C)]
pub struct si_sm_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct si_sm_io {
    pub si_info: *const si_sm_info,
    pub addr_source: c_int,
    pub addr_space: c_int,
    pub addr_data: c_ulong,
    pub regsize: c_int,
    pub regspacing: c_int,
    pub regshift: c_int,
    pub irq: c_int,
    pub irq_setup: *mut c_void,
    pub dev: *mut device,
}

#[repr(C)]
pub struct parisc_device_id {
    pub hw_type: c_int,
    pub hversion: c_int,
    pub hversion_rev: c_int,
    pub sversion: c_int,
}

#[repr(C)]
pub struct parisc_driver {
    pub name: *const c_char,
    pub id_table: *const parisc_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut parisc_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut parisc_device)>,
}

extern "C" {
    static ipmi_kcs_si_info: si_sm_info;
    fn ipmi_si_add_smi(io: *mut si_sm_io) -> c_int;
    fn ipmi_si_remove_by_dev(dev: *mut device);
    fn register_parisc_driver(driver: *mut parisc_driver);
    fn unregister_parisc_driver(driver: *mut parisc_driver);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const SI_DEVICETREE: c_int = 0;
const IPMI_MEM_ADDR_SPACE: c_int = 0;
const HPHW_MC: c_int = 0;
const HVERSION_REV_ANY_ID: c_int = 0;

static mut parisc_registered: bool = false;

unsafe extern "C" fn ipmi_parisc_probe(dev: *mut parisc_device) -> c_int {
    let mut io: si_sm_io = core::mem::zeroed();

    io.si_info = &ipmi_kcs_si_info;
    io.addr_source = SI_DEVICETREE;
    io.addr_space = IPMI_MEM_ADDR_SPACE;
    io.addr_data = (*dev).hpa.start;
    io.regsize = 1;
    io.regspacing = 1;
    io.regshift = 0;
    io.irq = 0; // no interrupt
    io.irq_setup = core::ptr::null_mut();
    io.dev = &mut (*dev).dev;

    dev_dbg(&mut (*dev).dev, c"addr 0x%lx\n", io.addr_data);

    ipmi_si_add_smi(&mut io)
}

unsafe extern "C" fn ipmi_parisc_remove(dev: *mut parisc_device) {
    ipmi_si_remove_by_dev(&mut (*dev).dev);
}

static ipmi_parisc_tbl: [parisc_device_id; 2] = [
    parisc_device_id {
        hw_type: HPHW_MC,
        hversion: HVERSION_REV_ANY_ID,
        hversion_rev: 0x004,
        sversion: 0xC0,
    },
    parisc_device_id {
        hw_type: 0,
        hversion: 0,
        hversion_rev: 0,
        sversion: 0,
    },
];

static mut ipmi_parisc_driver: parisc_driver = parisc_driver {
    name: c"ipmi".as_ptr(),
    id_table: ipmi_parisc_tbl.as_ptr(),
    probe: Some(ipmi_parisc_probe),
    remove: Some(ipmi_parisc_remove),
};

#[no_mangle]
pub unsafe extern "C" fn ipmi_si_parisc_init() {
    register_parisc_driver(&mut ipmi_parisc_driver);
    parisc_registered = true;
}

#[no_mangle]
pub unsafe extern "C" fn ipmi_si_parisc_shutdown() {
    if parisc_registered {
        unregister_parisc_driver(&mut ipmi_parisc_driver);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
