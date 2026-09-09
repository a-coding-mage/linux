// SPDX-License-Identifier: GPL-2.0
/*
 * Cortex A72 EDAC L1 and L2 cache error detection
 *
 * Copyright (c) 2020 Pengutronix, Sascha Hauer <s.hauer@pengutronix.de>
 * Copyright (c) 2025 Microsoft Corporation, <vijayb@linux.microsoft.com>
 *
 * Based on Code from:
 * Copyright (c) 2018, NXP Semiconductor
 * Author: York Sun <york.sun@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const DRVNAME: &str = "a72-edac";

const SYS_CPUMERRSR_EL1: u32 = sys_reg(3, 1, 15, 2, 2);
const SYS_L2MERRSR_EL1: u32 = sys_reg(3, 1, 15, 2, 3);

const CPUMERRSR_EL1_RAMID: u64 = genmask(30, 24);
const L2MERRSR_EL1_CPUID_WAY: u64 = genmask(21, 18);

const CPUMERRSR_EL1_VALID: u64 = 1u64 << 31;
const CPUMERRSR_EL1_FATAL: u64 = 1u64 << 63;
const L2MERRSR_EL1_VALID: u64 = 1u64 << 31;
const L2MERRSR_EL1_FATAL: u64 = 1u64 << 63;

const L1_I_TAG_RAM: u64 = 0x00;
const L1_I_DATA_RAM: u64 = 0x01;
const L1_D_TAG_RAM: u64 = 0x08;
const L1_D_DATA_RAM: u64 = 0x09;
const TLB_RAM: u64 = 0x18;

const MESSAGE_SIZE: usize = 64;

#[repr(C)]
struct mem_err_synd_reg {
    cpu_mesr: u64,
    l2_mesr: u64,
}

static mut compat_mask: cpumask = cpumask::default();

unsafe fn report_errors(edac_ctl: *mut edac_device_ctl_info, cpu: i32, mesr: *mut mem_err_synd_reg) {
    let cpu_mesr = (*mesr).cpu_mesr;
    let l2_mesr = (*mesr).l2_mesr;
    let mut msg = [0u8; MESSAGE_SIZE];

    if cpu_mesr & CPUMERRSR_EL1_VALID != 0 {
        let str_: &str;
        let fatal = cpu_mesr & CPUMERRSR_EL1_FATAL != 0;

        str_ = match (cpu_mesr & CPUMERRSR_EL1_RAMID) >> 24 {
            L1_I_TAG_RAM => "L1-I Tag RAM",
            L1_I_DATA_RAM => "L1-I Data RAM",
            L1_D_TAG_RAM => "L1-D Tag RAM",
            L1_D_DATA_RAM => "L1-D Data RAM",
            TLB_RAM => "TLB RAM",
            _ => "Unspecified",
        };

        snprintf(&mut msg, MESSAGE_SIZE, "%s %s error(s) on CPU %d", str_, if fatal { "fatal" } else { "correctable" }, cpu);

        if fatal {
            edac_device_handle_ue(edac_ctl, cpu, 0, msg.as_ptr());
        } else {
            edac_device_handle_ce(edac_ctl, cpu, 0, msg.as_ptr());
        }
    }

    if l2_mesr & L2MERRSR_EL1_VALID != 0 {
        let fatal = l2_mesr & L2MERRSR_EL1_FATAL != 0;

        snprintf(&mut msg, MESSAGE_SIZE, "L2 %s error(s) on CPU %d CPUID/WAY 0x%lx", if fatal { "fatal" } else { "correctable" }, cpu, (l2_mesr & L2MERRSR_EL1_CPUID_WAY) >> 18);
        if fatal {
            edac_device_handle_ue(edac_ctl, cpu, 1, msg.as_ptr());
        } else {
            edac_device_handle_ce(edac_ctl, cpu, 1, msg.as_ptr());
        }
    }
}

unsafe fn read_errors(data: *mut core::ffi::c_void) {
    let mesr = data as *mut mem_err_synd_reg;
    (*mesr).cpu_mesr = read_sysreg_s(SYS_CPUMERRSR_EL1);
    if (*mesr).cpu_mesr & CPUMERRSR_EL1_VALID != 0 {
        write_sysreg_s(0, SYS_CPUMERRSR_EL1);
        isb();
    }
    (*mesr).l2_mesr = read_sysreg_s(SYS_L2MERRSR_EL1);
    if (*mesr).l2_mesr & L2MERRSR_EL1_VALID != 0 {
        write_sysreg_s(0, SYS_L2MERRSR_EL1);
        isb();
    }
}

unsafe fn a72_edac_check(edac_ctl: *mut edac_device_ctl_info) {
    let mut mesr = mem_err_synd_reg { cpu_mesr: 0, l2_mesr: 0 };
    cpus_read_lock();
    for_each_cpu_and(|cpu| {
        smp_call_function_single(cpu, read_errors, &mut mesr as *mut _ as *mut core::ffi::c_void, true);
        report_errors(edac_ctl, cpu, &mut mesr);
    }, cpu_online_mask, &compat_mask);
    cpus_read_unlock();
}

unsafe fn a72_edac_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let edac_ctl = edac_device_alloc_ctl_info(0, "cpu", num_possible_cpus(), "L", 2, 1, edac_device_alloc_index());
    if edac_ctl.is_null() {
        return -12;
    }
    (*edac_ctl).edac_check = Some(a72_edac_check);
    (*edac_ctl).dev = dev;
    (*edac_ctl).mod_name = dev_name(dev);
    (*edac_ctl).dev_name = dev_name(dev);
    (*edac_ctl).ctl_name = DRVNAME;
    dev_set_drvdata(dev, edac_ctl);
    let rc = edac_device_add_device(edac_ctl);
    if rc != 0 {
        edac_device_free_ctl_info(edac_ctl);
        return rc;
    }
    0
}

unsafe fn a72_edac_remove(pdev: *mut platform_device) {
    let edac_ctl = dev_get_drvdata(&mut (*pdev).dev);
    edac_device_del_device((*edac_ctl).dev);
    edac_device_free_ctl_info(edac_ctl);
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

static cortex_arm64_edac_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"arm,cortex-a72".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe fn(*mut platform_device) -> i32>,
    remove: Option<unsafe fn(*mut platform_device)>,
    driver: driver,
}

#[repr(C)]
struct driver {
    name: &'static str,
}

static mut a72_edac_driver: platform_driver = platform_driver {
    probe: Some(a72_edac_probe),
    remove: Some(a72_edac_remove),
    driver: driver { name: DRVNAME },
};

static mut a72_pdev: *mut platform_device = core::ptr::null_mut();

unsafe fn a72_edac_driver_init() -> i32 {
    for_each_possible_cpu(|cpu| {
        let np = of_cpu_device_node_get(cpu);
        if !np.is_null() {
            if !of_match_node(cortex_arm64_edac_of_match.as_ptr(), np).is_null()
                && of_property_read_bool(np, "edac-enabled") {
                cpumask_set_cpu(cpu, &mut compat_mask);
            }
        } else {
            pr_warn("failed to find device node for CPU %d\n", cpu);
        }
    });
    if cpumask_empty(&compat_mask) {
        return 0;
    }
    a72_pdev = platform_device_register_simple(DRVNAME, -1, core::ptr::null_mut(), 0);
    if is_err(a72_pdev) {
        pr_err("failed to register A72 EDAC device\n");
        return ptr_err(a72_pdev);
    }
    platform_driver_register(&mut a72_edac_driver)
}

unsafe fn a72_edac_driver_exit() {
    platform_device_unregister(a72_pdev);
    platform_driver_unregister(&mut a72_edac_driver);
}

// module_init(a72_edac_driver_init);
// module_exit(a72_edac_driver_exit);
// MODULE_DEVICE_TABLE(of, cortex_arm64_edac_of_match);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
// MODULE_DESCRIPTION("Cortex A72 L1 and L2 cache EDAC driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
