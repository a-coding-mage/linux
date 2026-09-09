// SPDX-License-Identifier: GPL-2.0
/* Marvell CN10K RVU Hardware Random Number Generator.
 *
 * Copyright (C) 2021 Marvell.
 *
 */

// Required Linux kernel dependencies are supplied by other files.

/* CSRs */
const RNM_CTL_STATUS: usize = 0x000;
const RNM_ENTROPY_STATUS: usize = 0x008;
const RNM_CONST: usize = 0x030;
const RNM_EBG_ENT: usize = 0x048;
const RNM_PF_EBG_HEALTH: usize = 0x050;
const RNM_PF_RANDOM: usize = 0x400;
const RNM_TRNG_RESULT: usize = 0x408;

/* Extended TRNG Read and Status Registers */
const RNM_PF_TRNG_DAT: usize = 0x1000;
const RNM_PF_TRNG_RES: usize = 0x1008;

#[repr(C)]
struct Cn10kRng {
    reg_base: *mut core::ffi::c_void,
    ops: hwrng,
    pdev: *mut pci_dev,
    /* Octeon CN10K-A A0/A1, CNF10K-A A0/A1 and CNF10K-B A0/B0
     * does not support extended TRNG registers
     */
    extended_trng_regs: bool,
}

const PLAT_OCTEONTX_RESET_RNG_EBG_HEALTH_STATE: u64 = 0xc2000b0f;

const PCI_SUBSYS_DEVID_CN10K_A_RNG: u16 = 0xB900;
const PCI_SUBSYS_DEVID_CNF10K_A_RNG: u16 = 0xBA00;
const PCI_SUBSYS_DEVID_CNF10K_B_RNG: u16 = 0xBC00;

unsafe fn cn10k_is_extended_trng_regs_supported(pdev: *mut pci_dev) -> bool {
    /* CN10K-A A0/A1 */
    if ((*pdev).subsystem_device == PCI_SUBSYS_DEVID_CN10K_A_RNG)
        && ((*pdev).revision == 0
            || ((*pdev).revision & 0xff) == 0x50
            || ((*pdev).revision & 0xff) == 0x51)
    {
        return false;
    }

    /* CNF10K-A A0 */
    if ((*pdev).subsystem_device == PCI_SUBSYS_DEVID_CNF10K_A_RNG)
        && ((*pdev).revision == 0
            || ((*pdev).revision & 0xff) == 0x60
            || ((*pdev).revision & 0xff) == 0x61)
    {
        return false;
    }

    /* CNF10K-B A0/B0 */
    if ((*pdev).subsystem_device == PCI_SUBSYS_DEVID_CNF10K_B_RNG)
        && ((*pdev).revision == 0
            || ((*pdev).revision & 0xff) == 0x70
            || ((*pdev).revision & 0xff) == 0x74)
    {
        return false;
    }

    true
}

unsafe fn reset_rng_health_state(rng: *mut Cn10kRng) -> c_ulong {
    let mut res: arm_smccc_res = core::mem::zeroed();

    /* Send SMC service call to reset EBG health state */
    arm_smccc_smc(
        PLAT_OCTEONTX_RESET_RNG_EBG_HEALTH_STATE,
        0, 0, 0, 0, 0, 0, 0, &mut res,
    );
    res.a0
}

unsafe fn check_rng_health(rng: *mut Cn10kRng) -> c_int {
    let status: u64;
    let err: c_ulong;

    /* Skip checking health */
    if (*rng).reg_base.is_null() {
        return -ENODEV;
    }

    status = readq((*rng).reg_base.add(RNM_PF_EBG_HEALTH));
    if status & (1u64 << 20) != 0 {
        err = reset_rng_health_state(rng);
        if err != 0 {
            dev_err(&(*(*rng).pdev).dev, "HWRNG: Health test failed (status=%llx)\n", status);
            dev_err(&(*(*rng).pdev).dev, "HWRNG: error during reset (error=%lx)\n", err);
            return -EIO;
        }
    }
    0
}

/* Returns true when valid data available otherwise return false */
unsafe fn cn10k_read_trng(rng: *mut Cn10kRng, value: *mut u64) -> bool {
    let mut retry_count: u16 = 0;
    let upper: u64;
    let lower: u64;
    let mut status: u64;

    if (*rng).extended_trng_regs {
        loop {
            *value = readq((*rng).reg_base.add(RNM_PF_TRNG_DAT));
            if *value != 0 { return true; }
            status = readq((*rng).reg_base.add(RNM_PF_TRNG_RES));
            retry_count = retry_count.wrapping_add(1);
            if status == 0 && retry_count > 0x1000 { return false; }
            if status != 0 { break; }
        }
    }

    *value = readq((*rng).reg_base.add(RNM_PF_RANDOM));

    /* HW can run out of entropy if large amount random data is read in
     * quick succession. Zeros may not be real random data from HW.
     */
    if *value == 0 {
        let mut u = readq((*rng).reg_base.add(RNM_PF_RANDOM));
        let mut l = readq((*rng).reg_base.add(RNM_PF_RANDOM));
        while u & 0x00000000FFFFFFFFu64 == 0 { u = readq((*rng).reg_base.add(RNM_PF_RANDOM)); }
        while l & 0xFFFFFFFF00000000u64 == 0 { l = readq((*rng).reg_base.add(RNM_PF_RANDOM)); }
        *value = (u & 0xFFFFFFFF00000000u64) | (l & 0xFFFFFFFFu64);
    }
    true
}

unsafe fn cn10k_rng_read(hwrng: *mut hwrng, data: *mut c_void, max: usize, _wait: bool) -> c_int {
    let rng = (*hwrng).priv_data as *mut Cn10kRng;
    let mut size = max;
    let mut pos = data as *mut u8;
    let mut value: u64 = 0;

    let err = check_rng_health(rng);
    if err != 0 { return err; }

    while size >= 8 {
        if !cn10k_read_trng(rng, &mut value) { break; }
        *(pos as *mut u64) = value;
        size -= 8;
        pos = pos.add(8);
    }
    if size > 0 && cn10k_read_trng(rng, &mut value) {
        while size > 0 {
            *pos = value as u8;
            value >>= 8;
            size -= 1;
            pos = pos.add(1);
        }
    }
    (max - size) as c_int
}

unsafe fn cn10k_rng_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    let rng = devm_kzalloc(&(*pdev).dev, core::mem::size_of::<Cn10kRng>(), GFP_KERNEL)
        as *mut Cn10kRng;
    if rng.is_null() { return -ENOMEM; }

    (*rng).pdev = pdev;
    pci_set_drvdata(pdev, rng as *mut c_void);

    (*rng).reg_base = pcim_iomap(pdev, 0, 0);
    if (*rng).reg_base.is_null() { return -ENOMEM; }

    (*rng).ops.name = devm_kasprintf(&(*pdev).dev, GFP_KERNEL, "cn10k-rng-%s", dev_name(&(*pdev).dev));
    if (*rng).ops.name.is_null() { return -ENOMEM; }

    (*rng).ops.read = Some(cn10k_rng_read);
    (*rng).ops.priv_data = rng as c_ulong;
    (*rng).extended_trng_regs = cn10k_is_extended_trng_regs_supported(pdev);

    reset_rng_health_state(rng);

    let err = devm_hwrng_register(&(*pdev).dev, &mut (*rng).ops);
    if err != 0 { return dev_err_probe(&(*pdev).dev, err, "Could not register hwrng device.\n"); }
    0
}

static CN10K_RNG_ID_TABLE: [pci_device_id; 2] = [
    PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xA098), /* RNG PF */
    pci_device_id { _private: 0 },
];

static mut cn10k_rng_driver: pci_driver = pci_driver {
    name: "cn10k_rng",
    id_table: CN10K_RNG_ID_TABLE.as_ptr(),
    probe: Some(cn10k_rng_probe),
};

module_pci_driver!(cn10k_rng_driver);
module_device_table!(pci, CN10K_RNG_ID_TABLE);
module_author!("Sunil Goutham <sgoutham@marvell.com>");
module_description!("Marvell CN10K HW RNG Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
