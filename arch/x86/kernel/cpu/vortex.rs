// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel headers:
//   linux/kernel.h, asm/processor.h, and cpu.h

/*
 * No special init required for Vortex processors.
 */

#[allow(non_camel_case_types, non_upper_case_globals)]
static vortex_cpu_dev: cpu_dev = cpu_dev {
    c_vendor: "Vortex",
    c_ident: ["Vortex86 SoC"],
    legacy_models: [
        cpu_model_info {
            family: 5,
            model_names: [
                /* [0] */ None,
                /* [1] */ None,
                /* [2] */ Some("Vortex86DX"),
                /* [3] */ None,
                /* [4] */ None,
                /* [5] */ None,
                /* [6] */ None,
                /* [7] */ None,
                /* [8] */ Some("Vortex86MX"),
            ],
        },
        cpu_model_info {
            family: 6,
            model_names: [
                /*
                 * Both the Vortex86EX and the Vortex86EX2
                 * have the same family and model id.
                 *
                 * However, the -EX2 supports the product name
                 * CPUID call, so this name will only be used
                 * for the -EX, which does not.
                 */
                /* [0] */ Some("Vortex86EX"),
            ],
        },
    ],
    c_x86_vendor: X86_VENDOR_VORTEX,
};

// Equivalent to the cpu_dev_register(vortex_cpu_dev) registration macro.
unsafe extern "C" {
    fn cpu_dev_register(dev: cpu_dev);
}

#[allow(non_snake_case)]
unsafe fn register_vortex_cpu_dev() {
    cpu_dev_register(vortex_cpu_dev);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
