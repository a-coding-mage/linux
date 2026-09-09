// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation:
// asm/vendor_extensions/sifive.h
// asm/vendor_extensions/sifive_hwprobe.h
// asm/vendor_extensions/vendor_hwprobe.h
// linux/cpumask.h
// linux/types.h
// uapi/asm/hwprobe.h
// uapi/asm/vendor/sifive.h

pub unsafe fn hwprobe_isa_vendor_ext_sifive_0(
    pair: *mut riscv_hwprobe,
    cpus: *const cpumask,
) {
    vendor_extension_supported!(
        pair,
        cpus,
        riscv_isa_vendor_ext_list_sifive.per_hart_isa_bitmap,
        {
            vendor_ext_key!(XSFVQMACCDOD);
            vendor_ext_key!(XSFVQMACCQOQ);
            vendor_ext_key!(XSFVFNRCLIPXFQF);
            vendor_ext_key!(XSFVFWMACCQQQ);
        }
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
