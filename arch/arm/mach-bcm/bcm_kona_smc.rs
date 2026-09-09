// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2013 Broadcom Corporation

// External kernel symbols and types are supplied by the surrounding Rust translation.

static mut BCM_SMC_BUFFER_PHYS: u32 = 0; // physical address
static mut BCM_SMC_BUFFER: *mut core::ffi::c_void = core::ptr::null_mut(); // virtual address

#[repr(C)]
struct BcmKonaSmcData {
    service_id: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    result: u32,
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

static BCM_KONA_SMC_IDS: &[OfDeviceId] = &[
    OfDeviceId { compatible: c"brcm,kona-smc".as_ptr() },
    OfDeviceId { compatible: c"bcm,kona-smc".as_ptr() }, // deprecated name
    OfDeviceId { compatible: core::ptr::null() },
];

// Map in the args buffer area
unsafe fn bcm_kona_smc_init() -> i32 {
    let mut node: *mut core::ffi::c_void;
    let mut res = Resource { start: 0, end: 0 };
    let ret: i32;

    // Read buffer addr and size from the device tree node
    node = of_find_matching_node(core::ptr::null_mut(), BCM_KONA_SMC_IDS.as_ptr());
    if node.is_null() {
        return -ENODEV;
    }

    ret = of_address_to_resource(node, 0, &mut res);
    of_node_put(node);
    if ret != 0 {
        return -EINVAL;
    }

    BCM_SMC_BUFFER = ioremap(res.start, resource_size(&res));
    if BCM_SMC_BUFFER.is_null() {
        return -ENOMEM;
    }
    BCM_SMC_BUFFER_PHYS = res.start as u32;

    pr_info!("Kona Secure API initialized\n");

    0
}

/*
 * int bcm_kona_do_smc(u32 service_id, u32 buffer_addr)
 *
 * Only core 0 can run the secure monitor code. If an "smc" request is
 * initiated on a different core it must be redirected to core 0 for
 * execution. We rely on the caller to handle this.
 *
 * Each "smc" request supplies a service id and the address of a buffer
 * containing parameters related to the service to be performed. A flags
 * value defines the behavior of the level 2 cache and interrupt handling
 * while the secure monitor executes.
 *
 * Parameters to the "smc" request are passed in r4-r6 as follows:
 *     r4 service id
 *     r5 flags (SEC_ROM_*)
 *     r6 physical address of buffer with other parameters
 *
 * Execution of an "smc" request produces two distinct results.
 *
 * First, the secure monitor call itself (regardless of the specific service
 * request) can succeed, or can produce an error. When an "smc" request
 * completes this value is found in r12; it should always be SEC_EXIT_NORMAL.
 *
 * In addition, the particular service performed produces a result. The values
 * that should be expected depend on the service. We therefore return this
 * value to the caller, so it can handle the request result appropriately.
 * This result value is found in r0 when the "smc" request completes.
 */
unsafe fn bcm_kona_do_smc(service_id: u32, buffer_phys: u32) -> i32 {
    let r4 = service_id;
    let r5 = 0x3u32; // Keep IRQ and FIQ off in SM
    let r6 = buffer_phys;
    let (ip, r0): (u32, u32);

    // The ARM secure-monitor inline assembly is architecture/toolchain-specific.
    core::arch::asm!(
        ".arch_extension sec",
        "smc #0",
        inlateout("r4") r4 => _,
        inlateout("r5") r5 => _,
        inlateout("r6") r6 => _,
        lateout("r12") ip,
        lateout("r0") r0,
        clobber_abi("C"),
    );

    BUG_ON(ip != SEC_EXIT_NORMAL);
    r0 as i32
}

/* __bcm_kona_smc() should only run on CPU 0, with pre-emption disabled */
unsafe extern "C" fn __bcm_kona_smc(info: *mut core::ffi::c_void) {
    let data = &mut *(info as *mut BcmKonaSmcData);
    let mut args = BCM_SMC_BUFFER as *mut u32;

    BUG_ON(smp_processor_id() != 0);
    BUG_ON(args.is_null());

    // Copy the four 32 bit argument values into the bounce area
    writel_relaxed(data.arg0, args);
    args = args.add(1);
    writel_relaxed(data.arg1, args);
    args = args.add(1);
    writel_relaxed(data.arg2, args);
    args = args.add(1);
    writel(data.arg3, args);

    // Flush caches for input data passed to Secure Monitor
    flush_cache_all();

    // Trap into Secure Monitor and record the request result
    data.result = bcm_kona_do_smc(data.service_id, BCM_SMC_BUFFER_PHYS) as u32;
}

unsafe fn bcm_kona_smc(
    service_id: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
) -> u32 {
    let mut data = BcmKonaSmcData {
        service_id,
        arg0,
        arg1,
        arg2,
        arg3,
        result: 0,
    };

    /*
     * Due to a limitation of the secure monitor, we must use the SMP
     * infrastructure to forward all secure monitor calls to Core 0.
     */
    smp_call_function_single(0, __bcm_kona_smc, &mut data as *mut _ as *mut core::ffi::c_void, 1);

    data.result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
