// SPDX-License-Identifier: GPL-2.0-only
/* PS3 Platform spu routines. */

// External kernel and platform types/functions are supplied by surrounding dependencies.

#[repr(C)]
pub enum spe_type { SPE_TYPE_LOGICAL = 0 }

#[repr(C)]
pub struct spe_shadow {
    pub padding_0140: [u8; 0x0140],
    pub int_status_class0_RW: u64,
    pub int_status_class1_RW: u64,
    pub int_status_class2_RW: u64,
    pub padding_0158: [u8; 0x0610 - 0x0158],
    pub mfc_dsisr_RW: u64,
    pub padding_0618: [u8; 0x0620 - 0x0618],
    pub mfc_dar_RW: u64,
    pub padding_0628: [u8; 0x0800 - 0x0628],
    pub mfc_dsipr_R: u64,
    pub padding_0808: [u8; 0x0810 - 0x0808],
    pub mfc_lscrr_R: u64,
    pub padding_0818: [u8; 0x0c00 - 0x0818],
    pub mfc_cer_R: u64,
    pub padding_0c08: [u8; 0x0f00 - 0x0c08],
    pub spe_execution_status: u64,
    pub padding_0f08: [u8; 0x1000 - 0x0f08],
}

#[repr(C)]
pub enum spe_ex_state {
    SPE_EX_STATE_UNEXECUTABLE = 0,
    SPE_EX_STATE_EXECUTABLE = 2,
    SPE_EX_STATE_EXECUTED = 3,
}

#[repr(C)]
pub struct priv1_cache {
    pub masks: [u64; 3],
    pub sr1: u64,
    pub tclass_id: u64,
}

#[repr(C)]
pub struct spu_pdata {
    pub spe_id: u64,
    pub resource_id: u64,
    pub priv2_addr: u64,
    pub shadow_addr: u64,
    pub shadow: *mut spe_shadow,
    pub cache: priv1_cache,
}

unsafe fn spu_pdata_of(spu: *mut spu) -> *mut spu_pdata {
    (*spu).pdata as *mut spu_pdata
}

pub unsafe extern "C" fn ps3_get_spe_id(arg: *mut core::ffi::c_void) -> u64 {
    (*(arg as *mut spu_pdata)).spe_id
}

unsafe fn get_vas_id() -> u64 {
    let mut id = 0u64;
    lv1_get_logical_ppe_id(&mut id);
    lv1_get_virtual_address_space_id_of_ppe(&mut id);
    id
}

unsafe fn construct_spu(spu: *mut spu) -> i32 {
    let mut unused = 0u64;
    let mut problem_phys = 0u64;
    let mut local_store_phys = 0u64;
    let pd = spu_pdata_of(spu);
    let result = lv1_construct_logical_spe(
        PAGE_SHIFT, PAGE_SHIFT, PAGE_SHIFT, PAGE_SHIFT, PAGE_SHIFT,
        get_vas_id(), spe_type::SPE_TYPE_LOGICAL as i32, &mut (*pd).priv2_addr,
        &mut problem_phys, &mut local_store_phys, &mut unused,
        &mut (*pd).shadow_addr, &mut (*pd).spe_id,
    );
    (*spu).problem_phys = problem_phys;
    (*spu).local_store_phys = local_store_phys;
    if result != 0 {
        pr_debug!("%s:%d: lv1_construct_logical_spe failed: %s\\n",
                  __func__, __LINE__, ps3_result(result));
    }
    result
}

unsafe fn spu_unmap(spu: *mut spu) {
    iounmap((*spu).priv2);
    iounmap((*spu).problem);
    iounmap((*spu).local_store as *mut u8);
    iounmap((*spu_pdata_of(spu)).shadow);
}

unsafe fn setup_areas(spu: *mut spu) -> i32 {
    let pd = spu_pdata_of(spu);
    (*pd).shadow = ioremap_prot(
        (*pd).shadow_addr, core::mem::size_of::<spe_shadow>(),
        pgprot_noncached_wc(PAGE_KERNEL_RO),
    );
    if (*pd).shadow.is_null() {
        pr_debug!("%s:%d: ioremap shadow failed\\n", __func__, __LINE__);
        spu_unmap(spu);
        return -ENOMEM;
    }
    (*spu).local_store = ioremap_wc((*spu).local_store_phys, LS_SIZE)
        as *mut core::ffi::c_void;
    if (*spu).local_store.is_null() {
        pr_debug!("%s:%d: ioremap local_store failed\\n", __func__, __LINE__);
        spu_unmap(spu);
        return -ENOMEM;
    }
    (*spu).problem = ioremap(
        (*spu).problem_phys, core::mem::size_of::<spu_problem>());
    if (*spu).problem.is_null() {
        pr_debug!("%s:%d: ioremap problem failed\\n", __func__, __LINE__);
        spu_unmap(spu);
        return -ENOMEM;
    }
    (*spu).priv2 = ioremap(
        (*pd).priv2_addr, core::mem::size_of::<spu_priv2>());
    if (*spu).priv2.is_null() {
        pr_debug!("%s:%d: ioremap priv2 failed\\n", __func__, __LINE__);
        spu_unmap(spu);
        return -ENOMEM;
    }
    0
}

unsafe fn setup_interrupts(spu: *mut spu) -> i32 {
    let mut result = ps3_spe_irq_setup(
        PS3_BINDING_CPU_ANY, (*spu_pdata_of(spu)).spe_id, 0, &mut (*spu).irqs[0]);
    if result != 0 {
        (*spu).irqs = [0; 3];
        return result;
    }
    result = ps3_spe_irq_setup(
        PS3_BINDING_CPU_ANY, (*spu_pdata_of(spu)).spe_id, 1, &mut (*spu).irqs[1]);
    if result != 0 {
        ps3_spe_irq_destroy((*spu).irqs[0]);
        (*spu).irqs = [0; 3];
        return result;
    }
    result = ps3_spe_irq_setup(
        PS3_BINDING_CPU_ANY, (*spu_pdata_of(spu)).spe_id, 2, &mut (*spu).irqs[2]);
    if result != 0 {
        ps3_spe_irq_destroy((*spu).irqs[1]);
        ps3_spe_irq_destroy((*spu).irqs[0]);
        (*spu).irqs = [0; 3];
    }
    result
}

unsafe fn int_mask_and(spu: *mut spu, class: i32, mask: u64) {
    int_mask_set(spu, class, int_mask_get(spu, class) & mask);
}
unsafe fn int_mask_or(spu: *mut spu, class: i32, mask: u64) {
    int_mask_set(spu, class, int_mask_get(spu, class) | mask);
}
unsafe fn int_mask_set(spu: *mut spu, class: i32, mask: u64) {
    (*spu_pdata_of(spu)).cache.masks[class as usize] = mask;
    lv1_set_spe_interrupt_mask((*spu_pdata_of(spu)).spe_id, class, mask);
}
unsafe fn int_mask_get(spu: *mut spu, class: i32) -> u64 {
    (*spu_pdata_of(spu)).cache.masks[class as usize]
}
unsafe fn int_stat_clear(spu: *mut spu, class: i32, stat: u64) {
    lv1_clear_spe_interrupt_status((*spu_pdata_of(spu)).spe_id, class, stat, 0);
}
unsafe fn int_stat_get(spu: *mut spu, class: i32) -> u64 {
    let mut stat = 0;
    lv1_get_spe_interrupt_status((*spu_pdata_of(spu)).spe_id, class, &mut stat);
    stat
}
unsafe fn cpu_affinity_set(_: *mut spu, _: i32) {}
unsafe fn mfc_dar_get(spu: *mut spu) -> u64 {
    in_be64(&(*(*spu_pdata_of(spu)).shadow).mfc_dar_RW)
}
unsafe fn mfc_dsisr_set(_: *mut spu, _: u64) {}
unsafe fn mfc_dsisr_get(spu: *mut spu) -> u64 {
    in_be64(&(*(*spu_pdata_of(spu)).shadow).mfc_dsisr_RW)
}
unsafe fn mfc_sdr_setup(_: *mut spu) {}
unsafe fn mfc_sr1_set(spu: *mut spu, sr1: u64) {
    let allowed = !(MFC_STATE1_LOCAL_STORAGE_DECODE_MASK | MFC_STATE1_PROBLEM_STATE_MASK);
    BUG_ON((sr1 & allowed) != ((*spu_pdata_of(spu)).cache.sr1 & allowed));
    (*spu_pdata_of(spu)).cache.sr1 = sr1;
    lv1_set_spe_privilege_state_area_1_register(
        (*spu_pdata_of(spu)).spe_id, core::mem::offset_of!(spu_priv1, mfc_sr1_RW), sr1);
}
unsafe fn mfc_sr1_get(spu: *mut spu) -> u64 { (*spu_pdata_of(spu)).cache.sr1 }
unsafe fn mfc_tclass_id_set(spu: *mut spu, id: u64) {
    (*spu_pdata_of(spu)).cache.tclass_id = id;
    lv1_set_spe_privilege_state_area_1_register(
        (*spu_pdata_of(spu)).spe_id, core::mem::offset_of!(spu_priv1, mfc_tclass_id_RW), id);
}
unsafe fn mfc_tclass_id_get(spu: *mut spu) -> u64 { (*spu_pdata_of(spu)).cache.tclass_id }
unsafe fn tlb_invalidate(_: *mut spu) {}
unsafe fn resource_allocation_groupID_set(_: *mut spu, _: u64) {}
unsafe fn resource_allocation_groupID_get(_: *mut spu) -> u64 { 0 }
unsafe fn resource_allocation_enable_set(_: *mut spu, _: u64) {}
unsafe fn resource_allocation_enable_get(_: *mut spu) -> u64 { 0 }

pub unsafe fn ps3_spu_set_platform() {
    spu_priv1_ops = &spu_priv1_ps3_ops;
    spu_management_ops = &spu_management_ps3_ops;
}

unsafe fn enable_spu(spu: *mut spu) -> i32 {
    let mut result = lv1_enable_logical_spe((*spu_pdata_of(spu)).spe_id, (*spu_pdata_of(spu)).resource_id);
    if result == 0 { result = setup_areas(spu); }
    if result == 0 { result = setup_interrupts(spu); }
    if result != 0 {
        spu_unmap(spu);
        lv1_disable_logical_spe((*spu_pdata_of(spu)).spe_id, 0);
    }
    result
}
unsafe fn ps3_destroy_spu(spu: *mut spu) -> i32 {
    let mut result = lv1_disable_logical_spe((*spu_pdata_of(spu)).spe_id, 0);
    BUG_ON(result);
    ps3_spe_irq_destroy((*spu).irqs[2]);
    ps3_spe_irq_destroy((*spu).irqs[1]);
    ps3_spe_irq_destroy((*spu).irqs[0]);
    (*spu).irqs = [0; 3];
    spu_unmap(spu);
    result = lv1_destruct_logical_spe((*spu_pdata_of(spu)).spe_id);
    BUG_ON(result);
    kfree((*spu).pdata);
    (*spu).pdata = core::ptr::null_mut();
    0
}
unsafe fn ps3_create_spu(spu: *mut spu, data: *mut core::ffi::c_void) -> i32 {
    (*spu).pdata = kzalloc_obj::<spu_pdata>();
    if (*spu).pdata.is_null() { return -ENOMEM; }
    (*spu_pdata_of(spu)).resource_id = data as usize as u64;
    (*spu_pdata_of(spu)).cache.sr1 = 0x33;
    let mut result = construct_spu(spu);
    if result == 0 { result = enable_spu(spu); }
    if result != 0 { ps3_destroy_spu(spu); return result; }
    while in_be64(&(*(*spu_pdata_of(spu)).shadow).spe_execution_status)
        != spe_ex_state::SPE_EX_STATE_EXECUTED as u64 { cpu_relax(); }
    result
}
unsafe fn ps3_enumerate_spus(fn_ptr: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32) -> i32 {
    let mut num = 0u32;
    let mut result = ps3_repository_read_num_spu_resource_id(&mut num);
    let mut i = 0;
    while result == 0 && i < num {
        let mut resource_type = 0;
        let mut resource_id = 0u32;
        result = ps3_repository_read_spu_resource_id(i, &mut resource_type, &mut resource_id);
        if result == 0 && resource_type == PS3_SPU_RESOURCE_TYPE_EXCLUSIVE {
            result = fn_ptr(resource_id as usize as *mut core::ffi::c_void);
        }
        i += 1;
    }
    if result != 0 { return result; }
    num as i32
}
unsafe fn ps3_init_affinity() -> i32 { 0 }
unsafe fn ps3_enable_spu(_: *mut spu_context) {}
unsafe fn ps3_disable_spu(ctx: *mut spu_context) { ((*(*ctx).ops).runcntl_stop)(ctx); }

static mut spu_management_ps3_ops: spu_management_ops = spu_management_ops {
    enumerate_spus: ps3_enumerate_spus, create_spu: ps3_create_spu,
    destroy_spu: ps3_destroy_spu, enable_spu: ps3_enable_spu,
    disable_spu: ps3_disable_spu, init_affinity: ps3_init_affinity,
};
static mut spu_priv1_ps3_ops: spu_priv1_ops = spu_priv1_ops {
    int_mask_and, int_mask_or, int_mask_set, int_mask_get, int_stat_clear,
    int_stat_get, cpu_affinity_set, mfc_dar_get, mfc_dsisr_set, mfc_dsisr_get,
    mfc_sdr_setup, mfc_sr1_set, mfc_sr1_get, mfc_tclass_id_set, mfc_tclass_id_get,
    tlb_invalidate, resource_allocation_groupID_set, resource_allocation_groupID_get,
    resource_allocation_enable_set, resource_allocation_enable_get,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
