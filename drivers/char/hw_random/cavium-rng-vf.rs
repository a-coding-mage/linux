// SPDX-License-Identifier: GPL-2.0
/*
 * Hardware Random Number Generator support.
 * Cavium Thunder, Marvell OcteonTx/Tx2 processor families.
 *
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependencies are supplied by the surrounding kernel Rust bindings.

const PCI_DEVID_CAVIUM_RNG_PF: u16 = 0xA018;
const PCI_DEVID_CAVIUM_RNG_VF: u16 = 0xA033;
const HEALTH_STATUS_REG: usize = 0x38;
const PCI_DEVICE_ID_RST_OTX2: u16 = 0xA085;
const RST_BOOT_REG: u64 = 0x1600;
const CLOCK_BASE_RATE: u64 = 50_000_000;

#[inline]
const fn msec_to_nsec(x: u64) -> u64 { x * 1_000_000 }

#[repr(C)]
struct CaviumRng {
    ops: hwrng,
    result: *mut core::ffi::c_void,
    pf_regbase: *mut core::ffi::c_void,
    pdev: *mut pci_dev,
    clock_rate: u64,
    prev_error: u64,
    prev_time: u64,
}

#[inline]
unsafe fn is_octeontx(pdev: *mut pci_dev) -> bool {
    let _ = pdev;
    if midr_is_cpu_model_range(read_cpuid_id(), MIDR_THUNDERX_83XX,
                               MIDR_CPU_VAR_REV(0, 0), MIDR_CPU_VAR_REV(3, 0))
        || midr_is_cpu_model_range(read_cpuid_id(), MIDR_THUNDERX_81XX,
                                   MIDR_CPU_VAR_REV(0, 0), MIDR_CPU_VAR_REV(3, 0))
        || midr_is_cpu_model_range(read_cpuid_id(), MIDR_THUNDERX,
                                   MIDR_CPU_VAR_REV(0, 0), MIDR_CPU_VAR_REV(3, 0))
    { true } else { false }
}

unsafe fn rng_get_coprocessor_clkrate() -> u64 {
    let mut ret = CLOCK_BASE_RATE * 16;
    let pdev = pci_get_device(PCI_VENDOR_ID_CAVIUM, PCI_DEVICE_ID_RST_OTX2, core::ptr::null_mut());
    if pdev.is_null() { return ret; }
    let base = pci_ioremap_bar(pdev, 0);
    if base.is_null() { pci_dev_put(pdev); return ret; }
    ret = CLOCK_BASE_RATE * ((readq((base as *mut u8).add(RST_BOOT_REG as usize) as _) >> 33) & 0x3f);
    iounmap(base);
    pci_dev_put(pdev);
    ret
}

unsafe fn check_rng_health(rng: *mut CaviumRng) -> i32 {
    if (*rng).pf_regbase.is_null() { return 0; }
    let status = readq(((*rng).pf_regbase as *mut u8).add(HEALTH_STATUS_REG) as _);
    if status & (1u64 << 0) != 0 {
        dev_err(&mut (*(*rng).pdev).dev, "HWRNG: Startup health test failed\n");
        return -EIO;
    }
    let mut cycles = status >> 1;
    if cycles == 0 { return 0; }
    let cur_time = arch_timer_read_counter();
    cycles /= 2;
    let cur_err = (cycles * 1_000_000_000) / (*rng).clock_rate;
    if cur_err > msec_to_nsec(10) {
        (*rng).prev_error = 0;
        (*rng).prev_time = 0;
        return 0;
    }
    if (*rng).prev_error != 0 {
        let mut time_elapsed = (cur_time - (*rng).prev_time) * 10;
        time_elapsed += (*rng).prev_error;
        if cur_err < time_elapsed {
            dev_err(&mut (*(*rng).pdev).dev, "HWRNG failure detected\n");
            (*rng).prev_error = cur_err;
            (*rng).prev_time = cur_time;
            return -EIO;
        }
    }
    (*rng).prev_error = cur_err;
    (*rng).prev_time = cur_time;
    0
}

unsafe extern "C" fn cavium_rng_read(rng: *mut hwrng, dat: *mut core::ffi::c_void,
                                      max: usize, _wait: bool) -> i32 {
    let p = container_of!(rng, CaviumRng, ops);
    let err = check_rng_health(p);
    if err != 0 { return err; }
    let mut size = max;
    let mut ptr = dat as *mut u8;
    while size >= 8 {
        *(ptr as *mut u64) = readq((*p).result);
        size -= 8; ptr = ptr.add(8);
    }
    while size > 0 {
        *ptr = readb((*p).result);
        size -= 1; ptr = ptr.add(1);
    }
    max as i32
}

unsafe fn cavium_map_pf_regs(rng: *mut CaviumRng) -> i32 {
    if is_octeontx((*rng).pdev) { (*rng).pf_regbase = core::ptr::null_mut(); return 0; }
    let pdev = pci_get_device(PCI_VENDOR_ID_CAVIUM, PCI_DEVID_CAVIUM_RNG_PF, core::ptr::null_mut());
    if pdev.is_null() { pr_err!("Cannot find RNG PF device\n"); return -EIO; }
    (*rng).pf_regbase = ioremap(pci_resource_start(pdev, 0), pci_resource_len(pdev, 0));
    if (*rng).pf_regbase.is_null() {
        dev_err(&mut (*pdev).dev, "Failed to map PF CSR region\n");
        pci_dev_put(pdev); return -ENOMEM;
    }
    pci_dev_put(pdev);
    (*rng).clock_rate = rng_get_coprocessor_clkrate();
    0
}

unsafe extern "C" fn cavium_rng_probe_vf(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let rng = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<CaviumRng>(), GFP_KERNEL) as *mut CaviumRng;
    if rng.is_null() { return -ENOMEM; }
    (*rng).pdev = pdev;
    (*rng).result = pcim_iomap(pdev, 0, 0);
    if (*rng).result.is_null() { dev_err(&mut (*pdev).dev, "Error iomap failed retrieving result.\n"); return -ENOMEM; }
    (*rng).ops.name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, "cavium-rng-%s", dev_name(&(*pdev).dev));
    if (*rng).ops.name.is_null() { return -ENOMEM; }
    (*rng).ops.read = Some(cavium_rng_read);
    pci_set_drvdata(pdev, rng as _);
    let ret = cavium_map_pf_regs(rng);
    if ret != 0 { return ret; }
    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*rng).ops);
    if ret != 0 { dev_err(&mut (*pdev).dev, "Error registering device as HWRNG.\n"); return ret; }
    0
}

unsafe extern "C" fn cavium_rng_remove_vf(pdev: *mut pci_dev) {
    let rng = pci_get_drvdata(pdev) as *mut CaviumRng;
    iounmap((*rng).pf_regbase);
}

static mut CAVIUM_RNG_VF_ID_TABLE: [pci_device_id; 2] = [
    PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, PCI_DEVID_CAVIUM_RNG_VF),
    pci_device_id { _bindgen_opaque_blob: [0; core::mem::size_of::<pci_device_id>()] },
];

static mut CAVIUM_RNG_VF_DRIVER: pci_driver = pci_driver {
    name: c"cavium_rng_vf".as_ptr(),
    id_table: CAVIUM_RNG_VF_ID_TABLE.as_ptr(),
    probe: Some(cavium_rng_probe_vf),
    remove: Some(cavium_rng_remove_vf),
};

module_pci_driver!(CAVIUM_RNG_VF_DRIVER);
module_author!("Omer Khaliq <okhaliq@caviumnetworks.com>");
module_description!("Cavium ThunderX Random Number Generator VF support");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
