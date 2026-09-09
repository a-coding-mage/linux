// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// The Linux and architecture headers included by the C source provide the
// types, constants, macros, and external functions referenced below.

extern "C" {
    static mut active_asids: atomic64_t;
    static mut reserved_asids: u64;
    static mut asid_info: asid_info;

    fn asid_check_context(
        info: *mut asid_info,
        asid: *mut _,
        cpu: u32,
        mm: *mut mm_struct,
    );
    fn local_tlb_invalid_all();
    fn asid_allocator_init(
        info: *mut asid_info,
        bits: u32,
        first_asid: u32,
        flush: unsafe extern "C" fn(),
    ) -> i32;
    fn kernel_bug_on(condition: bool);
    fn kernel_panic(message: *const u8, ...);
    fn kernel_pr_info(message: *const u8, ...);
    fn num_possible_cpus() -> usize;
    fn num_asids(info: *const asid_info) -> usize;
    fn num_ctxt_asids(info: *const asid_info) -> usize;
}

// These declarations are supplied by the included Linux headers.
// DEFINE_PER_CPU(atomic64_t, active_asids);
// DEFINE_PER_CPU(u64, reserved_asids);
type atomic64_t = core::ffi::c_longlong;

#[repr(C)]
pub struct asid_info {
    pub active: *mut atomic64_t,
    pub reserved: *mut u64,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}

#[repr(C)]
pub struct mm_context {
    pub asid: u64,
}

const CONFIG_CPU_ASID_BITS: u32 = 0;

pub unsafe extern "C" fn check_and_switch_context(mm: *mut mm_struct, cpu: u32) {
    asid_check_context(&mut asid_info, &mut (*mm).context.asid, cpu, mm);
}

unsafe extern "C" fn asid_flush_cpu_ctxt() {
    local_tlb_invalid_all();
}

unsafe extern "C" fn asids_init() -> i32 {
    let asid_limit = (1u32 << CONFIG_CPU_ASID_BITS).wrapping_sub(1);
    kernel_bug_on(asid_limit <= num_possible_cpus() as u32);

    if asid_allocator_init(
        &mut asid_info,
        CONFIG_CPU_ASID_BITS,
        1,
        asid_flush_cpu_ctxt,
    ) != 0
    {
        // panic("Unable to initialize ASID allocator for %lu ASIDs\n",
        //       NUM_ASIDS(&asid_info));
        kernel_panic(
            b"Unable to initialize ASID allocator for %lu ASIDs\n\0".as_ptr(),
            num_asids(&asid_info),
        );
    }

    asid_info.active = &mut active_asids;
    asid_info.reserved = &mut reserved_asids;

    // pr_info("ASID allocator initialised with %lu entries\n",
    //         NUM_CTXT_ASIDS(&asid_info));
    kernel_pr_info(
        b"ASID allocator initialised with %lu entries\n\0".as_ptr(),
        num_ctxt_asids(&asid_info),
    );

    0
}

// early_initcall(asids_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
