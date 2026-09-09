// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013, Michael Ellerman, IBM Corporation.
 */

// pr_fmt(fmt) = "powernv-rng: " fmt
// C header dependencies are supplied by the surrounding kernel translation.

const DARN_ERR: c_ulong = 0xFFFF_FFFF_FFFF_FFFF;

#[repr(C)]
struct pnv_rng {
    regs: *mut core::ffi::c_void,
    regs_real: *mut core::ffi::c_void,
    mask: c_ulong,
}

// DEFINE_PER_CPU(struct pnv_rng *, pnv_rng);
static mut pnv_rng: *mut pnv_rng = core::ptr::null_mut();

unsafe fn rng_whiten(rng: *mut pnv_rng, mut val: c_ulong) -> c_ulong {
    let parity: c_ulong;

    /* Calculate the parity of the value */
    parity = val.count_ones() as c_ulong;

    /* xor our value with the previous mask */
    val ^= (*rng).mask;

    /* update the mask based on the parity of this value */
    (*rng).mask = ((*rng).mask << 1) | (parity & 1);

    val
}

unsafe fn pnv_get_random_darn(v: *mut c_ulong) -> c_int {
    let val: c_ulong;

    /* Using DARN with L=1 - 64-bit conditioned random number */
    // PPC_DARN(val, 1)
    core::arch::asm!("darn {0}, 1", out(reg) val);

    if val == DARN_ERR {
        return 0;
    }

    *v = val;
    1
}

unsafe fn initialise_darn() -> c_int {
    let mut val: c_ulong = 0;
    let mut i = 0;

    if !cpu_has_feature(CPU_FTR_ARCH_300) {
        return -ENODEV;
    }

    while i < 10 {
        if pnv_get_random_darn(&mut val) != 0 {
            ppc_md.get_random_seed = Some(pnv_get_random_darn);
            return 0;
        }
        i += 1;
    }
    -EIO
}

pub unsafe fn pnv_get_random_long(v: *mut c_ulong) -> c_int {
    let rng: *mut pnv_rng;

    if mfmsr() & MSR_DR != 0 {
        rng = get_cpu_var(&raw mut pnv_rng);
        *v = rng_whiten(rng, in_be64((*rng).regs));
        put_cpu_var(rng);
    } else {
        rng = raw_cpu_read(&raw const pnv_rng);
        *v = rng_whiten(rng, __raw_rm_readq((*rng).regs_real));
    }
    1
}

// EXPORT_SYMBOL_GPL(pnv_get_random_long);

unsafe fn rng_init_per_cpu(rng: *mut pnv_rng, dn: *mut device_node) {
    let chip_id = of_get_ibm_chip_id(dn);
    if chip_id == -1 {
        pr_warn!("No ibm,chip-id found for %pOF.\n", dn);
    }

    // for_each_possible_cpu(cpu)
    for cpu in for_each_possible_cpu() {
        if per_cpu_pnv_rng(cpu).is_null() || cpu_to_chip_id(cpu) == chip_id {
            set_per_cpu_pnv_rng(cpu, rng);
        }
    }
}

unsafe fn rng_create(dn: *mut device_node) -> c_int {
    let rng = kzalloc_obj::<pnv_rng>();
    if rng.is_null() {
        return -ENOMEM;
    }

    let mut res = resource::default();
    if of_address_to_resource(dn, 0, &mut res) != 0 {
        kfree(rng);
        return -ENXIO;
    }

    (*rng).regs_real = res.start as *mut core::ffi::c_void;
    (*rng).regs = of_iomap(dn, 0);
    if (*rng).regs.is_null() {
        kfree(rng);
        return -ENXIO;
    }

    let val = in_be64((*rng).regs);
    (*rng).mask = val;
    rng_init_per_cpu(rng, dn);
    ppc_md.get_random_seed = Some(pnv_get_random_long);
    0
}

unsafe fn pnv_get_random_long_early(v: *mut c_ulong) -> c_int {
    let mut dn: *mut device_node;

    if !slab_is_available() {
        return 0;
    }

    if cmpxchg_get_random_seed(Some(pnv_get_random_long_early), None)
        != Some(pnv_get_random_long_early)
    {
        return 0;
    }

    // for_each_compatible_node(dn, NULL, "ibm,power-rng")
    for node in for_each_compatible_node("ibm,power-rng") {
        dn = node;
        rng_create(dn);
    }

    if ppc_md.get_random_seed.is_none() {
        return 0;
    }
    (ppc_md.get_random_seed.unwrap())(v)
}

pub unsafe fn pnv_rng_init() {
    let dn: *mut device_node;

    /* Prefer darn over the rest. */
    if initialise_darn() == 0 {
        return;
    }

    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), "ibm,power-rng");
    if !dn.is_null() {
        ppc_md.get_random_seed = Some(pnv_get_random_long_early);
    }
    of_node_put(dn);
}

unsafe fn pnv_rng_late_init() -> c_int {
    let mut v: c_ulong = 0;

    /* In case it wasn't called during init for some other reason. */
    if ppc_md.get_random_seed == Some(pnv_get_random_long_early) {
        pnv_get_random_long_early(&mut v);
    }

    if ppc_md.get_random_seed == Some(pnv_get_random_long) {
        // for_each_compatible_node(dn, NULL, "ibm,power-rng")
        for dn in for_each_compatible_node("ibm,power-rng") {
            of_platform_device_create(dn, core::ptr::null(), core::ptr::null_mut());
        }
    }
    0
}

// machine_subsys_initcall(powernv, pnv_rng_late_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
