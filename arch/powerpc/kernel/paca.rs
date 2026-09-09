// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * c 2001 PPC 64 Team, IBM Corp
 */

// Linux/architecture dependencies supplied externally.

#[cfg(not(feature = "CONFIG_SMP"))]
const boot_cpuid: i32 = 0;

unsafe fn alloc_paca_data(size: usize, align: usize, limit: usize, cpu: i32) -> *mut core::ffi::c_void {
    let nid: i32;
    if cpu == boot_cpuid {
        nid = NUMA_NO_NODE;
        memblock_set_bottom_up(true);
    } else {
        nid = early_cpu_to_node(cpu);
    }

    let ptr = memblock_alloc_try_nid(size, align, MEMBLOCK_LOW_LIMIT, limit, nid);
    if ptr.is_null() {
        panic!("cannot allocate paca data");
    }

    if cpu == boot_cpuid {
        memblock_set_bottom_up(false);
    }
    ptr
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
const LPPACA_SIZE: usize = 0x400;

#[cfg(feature = "CONFIG_PPC_PSERIES")]
unsafe fn alloc_shared_lppaca(size: usize, limit: usize, _cpu: i32) -> *mut core::ffi::c_void {
    let shared_lppaca_total_size = page_align(nr_cpu_ids * LPPACA_SIZE);
    static mut SHARED_LPPACA_SIZE: usize = 0;
    static mut SHARED_LPPACA: *mut core::ffi::c_void = core::ptr::null_mut();

    if SHARED_LPPACA.is_null() {
        memblock_set_bottom_up(true);
        SHARED_LPPACA = memblock_alloc_try_nid(
            shared_lppaca_total_size, PAGE_SIZE, MEMBLOCK_LOW_LIMIT,
            limit, NUMA_NO_NODE,
        );
        if SHARED_LPPACA.is_null() {
            panic!("cannot allocate shared data");
        }
        memblock_set_bottom_up(false);
        uv_share_page(
            phys_pfn(__pa(SHARED_LPPACA)),
            shared_lppaca_total_size >> PAGE_SHIFT,
        );
    }

    let ptr = (SHARED_LPPACA as *mut u8).add(SHARED_LPPACA_SIZE) as *mut core::ffi::c_void;
    SHARED_LPPACA_SIZE += size;
    debug_assert!(SHARED_LPPACA_SIZE <= shared_lppaca_total_size);
    ptr
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
unsafe fn init_lppaca(lppaca: *mut lppaca) {
    *lppaca = lppaca {
        desc: cpu_to_be32(0xd397d781),
        size: cpu_to_be16(LPPACA_SIZE as u16),
        fpregs_in_use: 1,
        slb_count: cpu_to_be16(64),
        vmxregs_in_use: 0,
        page_ins: 0,
    };
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
unsafe fn new_lppaca(cpu: i32, limit: usize) -> *mut lppaca {
    if early_cpu_has_feature(CPU_FTR_HVMODE) {
        return core::ptr::null_mut();
    }
    let lp = if is_secure_guest() {
        alloc_shared_lppaca(LPPACA_SIZE, limit, cpu) as *mut lppaca
    } else {
        alloc_paca_data(LPPACA_SIZE, 0x400, limit, cpu) as *mut lppaca
    };
    init_lppaca(lp);
    lp
}

#[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
unsafe fn new_slb_shadow(cpu: i32, limit: usize) -> *mut slb_shadow {
    if cpu != boot_cpuid && early_radix_enabled() {
        return core::ptr::null_mut();
    }
    let s = alloc_paca_data(core::mem::size_of::<slb_shadow>(), L1_CACHE_BYTES, limit, cpu)
        as *mut slb_shadow;
    (*s).persistent = cpu_to_be32(SLB_NUM_BOLTED);
    (*s).buffer_length = cpu_to_be32(core::mem::size_of::<slb_shadow>() as u32);
    s
}

pub static mut paca_ptrs: *mut *mut paca_struct = core::ptr::null_mut();

pub unsafe fn initialise_paca(new_paca: *mut paca_struct, cpu: i32) {
    #[cfg(feature = "CONFIG_PPC_PSERIES")]
    { (*new_paca).lppaca_ptr = core::ptr::null_mut(); }
    #[cfg(feature = "CONFIG_PPC_BOOK3E_64")]
    { (*new_paca).kernel_pgd = swapper_pg_dir; }
    (*new_paca).lock_token = 0x8000;
    (*new_paca).paca_index = cpu;
    #[cfg(not(feature = "CONFIG_PPC_KERNEL_PCREL"))]
    { (*new_paca).kernel_toc = kernel_toc_addr(); }
    (*new_paca).kernelbase = _stext as usize;
    (*new_paca).kernel_msr = MSR_KERNEL & !(MSR_IR | MSR_DR);
    (*new_paca).hw_cpu_id = 0xffff;
    (*new_paca).kexec_state = KEXEC_STATE_NONE;
    (*new_paca).__current = &mut init_task;
    (*new_paca).data_offset = 0xfeeeeeeeeeeeeeeeu64;
    #[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
    { (*new_paca).slb_shadow_ptr = core::ptr::null_mut(); }
    #[cfg(feature = "CONFIG_PPC_BOOK3E_64")]
    { (*new_paca).tcd_ptr = &mut (*new_paca).tcd; }
}

pub unsafe fn setup_paca(new_paca: *mut paca_struct) {
    local_paca = new_paca;
    #[cfg(feature = "CONFIG_PPC_BOOK3E_64")]
    { mtspr(SPRN_SPRG_TLB_EXFRAME, (*local_paca).extlb); }
    #[cfg(not(feature = "CONFIG_PPC_BOOK3E_64"))]
    {
        if mfmsr() & MSR_HV != 0 { mtspr(SPRN_SPRG_HPACA, local_paca); }
    }
    mtspr(SPRN_SPRG_PACA, local_paca);
}

static mut paca_nr_cpu_ids: i32 = 0;
static mut paca_ptrs_size: usize = 0;
static mut paca_struct_size: usize = 0;

pub unsafe fn allocate_paca_ptrs() {
    paca_nr_cpu_ids = nr_cpu_ids as i32;
    paca_ptrs_size = core::mem::size_of::<*mut paca_struct>() * nr_cpu_ids;
    paca_ptrs = memblock_alloc_raw(paca_ptrs_size, SMP_CACHE_BYTES) as *mut *mut paca_struct;
    if paca_ptrs.is_null() { panic!("Failed to allocate {} bytes for paca pointers\n", paca_ptrs_size); }
    core::ptr::write_bytes(paca_ptrs as *mut u8, 0x88, paca_ptrs_size);
}

pub unsafe fn allocate_paca(cpu: i32) {
    assert!((cpu as usize) < paca_nr_cpu_ids as usize);
    let limit = {
        #[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
        { core::cmp::min(ppc64_bolted_size(), ppc64_rma_size) }
        #[cfg(not(feature = "CONFIG_PPC_BOOK3S_64"))]
        { ppc64_rma_size }
    };
    let paca = alloc_paca_data(core::mem::size_of::<paca_struct>(), L1_CACHE_BYTES, limit, cpu)
        as *mut paca_struct;
    *paca_ptrs.add(cpu as usize) = paca;
    initialise_paca(paca, cpu);
    #[cfg(feature = "CONFIG_PPC_PSERIES")]
    { (*paca).lppaca_ptr = new_lppaca(cpu, limit); }
    #[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
    { (*paca).slb_shadow_ptr = new_slb_shadow(cpu, limit); }
    paca_struct_size += core::mem::size_of::<paca_struct>();
}

pub unsafe fn free_unused_pacas() {
    let new_ptrs_size = core::mem::size_of::<*mut paca_struct>() * nr_cpu_ids;
    if new_ptrs_size < paca_ptrs_size {
        memblock_phys_free(__pa(paca_ptrs) + new_ptrs_size, paca_ptrs_size - new_ptrs_size);
    }
    paca_nr_cpu_ids = nr_cpu_ids as i32;
    paca_ptrs_size = new_ptrs_size;
    #[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
    if early_radix_enabled() {
        memblock_phys_free(__pa((*paca_ptrs.add(boot_cpuid as usize)).slb_shadow_ptr), core::mem::size_of::<slb_shadow>());
        (*paca_ptrs.add(boot_cpuid as usize)).slb_shadow_ptr = core::ptr::null_mut();
    }
    printk(KERN_DEBUG, "Allocated %u bytes for %u pacas\n", paca_ptrs_size + paca_struct_size, nr_cpu_ids);
}

#[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
pub unsafe fn copy_mm_to_paca(mm: *mut mm_struct) {
    let context = &mut (*mm).context;
    memcpy(&mut (*get_paca()).mm_ctx_low_slices_psize, mm_ctx_low_slices(context), LOW_SLICE_ARRAY_SZ);
    memcpy(&mut (*get_paca()).mm_ctx_high_slices_psize, mm_ctx_high_slices(context), TASK_SLICE_ARRAY_SZ(context));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
