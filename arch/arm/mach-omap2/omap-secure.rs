// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP Secure API infrastructure.
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 * Copyright (C) 2012 Ivaylo Dimitrov <freemangordon@abv.bg>
 * Copyright (C) 2013 Pali Rohár <pali@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut omap_secure_memblock_base: phys_addr_t = 0;

pub static mut optee_available: bool = false;

// OMAP_SIP_SMC_STD_CALL_VAL(func_num) =
// ARM_SMCCC_CALL_VAL(ARM_SMCCC_STD_CALL, ARM_SMCCC_SMC_32,
//                    ARM_SMCCC_OWNER_SIP, (func_num))

unsafe fn omap_optee_init_check() {
    let np: *mut device_node;

    /*
     * We only check that the OP-TEE node is present and available. The
     * OP-TEE kernel driver is not needed for the type of interaction made
     * with OP-TEE here so the driver's status is not checked.
     */
    np = of_find_node_by_path("/firmware/optee\0".as_ptr() as *const i8);
    if !np.is_null() && of_device_is_available(np) {
        optee_available = true;
    }
    of_node_put(np);
}

/**
 * omap_secure_dispatcher - Routine to dispatch low power secure
 * service routines
 * @idx: The HAL API index
 * @flag: The flag indicating criticality of operation
 * @nargs: Number of valid arguments out of four.
 * @arg1, arg2, arg3 args4: Parameters passed to secure API
 *
 * Return the non-zero error value on failure.
 */
pub unsafe fn omap_secure_dispatcher(
    idx: u32, flag: u32, nargs: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32,
) -> u32 {
    static mut BUF: [[u32; 5]; NR_CPUS] = [[0; 5]; NR_CPUS];
    let cpu = get_cpu();
    let param = BUF[cpu as usize].as_mut_ptr();

    *param.add(0) = nargs;
    *param.add(1) = arg1;
    *param.add(2) = arg2;
    *param.add(3) = arg3;
    *param.add(4) = arg4;

    /* Secure API needs physical address pointer for the parameters. */
    flush_cache_all();
    outer_clean_range(__pa(param), __pa(param.add(5)));
    let ret = omap_smc2(idx, flag, __pa(param));

    put_cpu();
    ret
}

pub unsafe fn omap_smccc_smc(fn_: u32, arg: u32) {
    let mut res: arm_smccc_res = core::mem::zeroed();
    arm_smccc_smc(
        ARM_SMCCC_CALL_VAL(ARM_SMCCC_STD_CALL, ARM_SMCCC_SMC_32,
                           ARM_SMCCC_OWNER_SIP, fn_),
        arg, 0, 0, 0, 0, 0, 0, &mut res,
    );
    WARN(res.a0, "Secure function call 0x%08x failed\n", fn_);
}

pub unsafe fn omap_smc1(fn_: u32, arg: u32) {
    /* If this platform has OP-TEE installed use ARM SMC calls; otherwise use OMAP ROM calls. */
    if optee_available {
        omap_smccc_smc(fn_, arg);
    } else {
        _omap_smc1(fn_, arg);
    }
}

/* Allocate the memory to save secure ram */
pub unsafe fn omap_secure_ram_reserve_memblock() -> i32 {
    let mut size: u32 = OMAP_SECURE_RAM_STORAGE;
    size = ALIGN(size, SECTION_SIZE);
    omap_secure_memblock_base = arm_memblock_steal(size, SECTION_SIZE);
    0
}

#[cfg(all(CONFIG_ARCH_OMAP3, CONFIG_PM))]
pub unsafe fn omap3_save_secure_ram(addr: *mut core::ffi::c_void, size: i32) -> u32 {
    static mut PARAM: [u32; 5] = [0; 5];
    if size != OMAP3_SAVE_SECURE_RAM_SZ {
        return OMAP3_SAVE_SECURE_RAM_SZ;
    }
    PARAM[0] = 4;
    PARAM[1] = __pa(addr);
    PARAM[2] = 0;
    PARAM[3] = 1;
    PARAM[4] = 1;
    save_secure_ram_context(__pa(PARAM.as_mut_ptr()))
}

/** rx51_secure_dispatcher: Routine to dispatch secure PPA API calls */
unsafe fn rx51_secure_dispatcher(idx: u32, process: u32, flag: u32, nargs: u32,
                                 arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
    static mut PARAM: [u32; 5] = [0; 5];
    PARAM[0] = nargs.wrapping_add(1);
    PARAM[1] = arg1;
    PARAM[2] = arg2;
    PARAM[3] = arg3;
    PARAM[4] = arg4;
    local_irq_disable();
    local_fiq_disable();
    flush_cache_all();
    outer_clean_range(__pa(PARAM.as_mut_ptr()), __pa(PARAM.as_mut_ptr().add(5)));
    let ret = omap_smc3(idx, process, flag, __pa(PARAM.as_mut_ptr()));
    flush_cache_all();
    local_fiq_enable();
    local_irq_enable();
    ret
}

/** rx51_secure_update_aux_cr: Routine to modify the contents of Auxiliary Control Register */
pub unsafe fn rx51_secure_update_aux_cr(set_bits: u32, clear_bits: u32) -> u32 {
    let mut acr: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) acr);
    acr &= !clear_bits;
    acr |= set_bits;
    rx51_secure_dispatcher(RX51_PPA_WRITE_ACR, 0, FLAG_START_CRITICAL, 1, acr, 0, 0, 0)
}

/** rx51_secure_rng_call: Routine for HW random generator */
pub unsafe fn rx51_secure_rng_call(ptr: u32, count: u32, flag: u32) -> u32 {
    rx51_secure_dispatcher(RX51_PPA_HWRNG, 0, NO_FLAG, 3, ptr, count, flag, 0)
}

pub unsafe fn omap_secure_init() {
    omap_optee_init_check();
}

/* Dummy dispatcher call after core OSWR and MPU off. Updates the ROM return address after MMU re-enable. */
unsafe fn cpu_notifier(_nb: *mut notifier_block, cmd: u32, _v: *mut core::ffi::c_void) -> i32 {
    match cmd {
        CPU_CLUSTER_PM_EXIT => {
            omap_secure_dispatcher(OMAP4_PPA_SERVICE_0, FLAG_START_CRITICAL, 0, 0, 0, 0, 0);
        }
        _ => {}
    }
    NOTIFY_OK
}

static mut secure_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(cpu_notifier),
};

unsafe fn secure_pm_init() -> i32 {
    if omap_type() == OMAP2_DEVICE_TYPE_GP || !soc_is_omap44xx() {
        return 0;
    }
    cpu_pm_register_notifier(&mut secure_notifier_block);
    0
}

omap_arch_initcall!(secure_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
