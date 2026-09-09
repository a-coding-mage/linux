// SPDX-License-Identifier: GPL-2.0

// External kernel declarations and constants are supplied by the surrounding
// PowerPC KVM translation unit.

static KVM_PPC_GSE_IDEN_LEN: [u16; __KVMPPC_GSE_TYPE_MAX as usize] = [
    [KVMPPC_GSE_BE32 as usize]: core::mem::size_of::<__be32>() as u16,
    [KVMPPC_GSE_BE64 as usize]: core::mem::size_of::<__be64>() as u16,
    [KVMPPC_GSE_VEC128 as usize]: core::mem::size_of::<vector128>() as u16,
    [KVMPPC_GSE_PARTITION_TABLE as usize]: core::mem::size_of::<kvmppc_gs_part_table>() as u16,
    [KVMPPC_GSE_PROCESS_TABLE as usize]: core::mem::size_of::<kvmppc_gs_proc_table>() as u16,
    [KVMPPC_GSE_BUFFER as usize]: core::mem::size_of::<kvmppc_gs_buff_info>() as u16,
];

/// kvmppc_gsb_new() - create a new guest state buffer
pub unsafe fn kvmppc_gsb_new(size: usize, guest_id: c_ulong, vcpu_id: c_ulong, flags: gfp_t) -> *mut kvmppc_gs_buff {
    let mut gsb = kzalloc_obj::<kvmppc_gs_buff>(flags);
    if gsb.is_null() { return core::ptr::null_mut(); }

    let size = roundup_pow_of_two(size);
    (*gsb).hdr = kzalloc(size, GFP_KERNEL);
    if (*gsb).hdr.is_null() {
        kfree(gsb as *mut c_void);
        return core::ptr::null_mut();
    }
    (*gsb).capacity = size;
    (*gsb).len = core::mem::size_of::<kvmppc_gs_header>();
    (*gsb).vcpu_id = vcpu_id;
    (*gsb).guest_id = guest_id;
    (*(*gsb).hdr).nelems = cpu_to_be32(0);
    gsb
}

pub unsafe fn kvmppc_gsb_free(gsb: *mut kvmppc_gs_buff) {
    kfree((*gsb).hdr as *mut c_void);
    kfree(gsb as *mut c_void);
}

pub unsafe fn kvmppc_gsb_put(gsb: *mut kvmppc_gs_buff, size: usize) -> *mut c_void {
    let nelems = kvmppc_gsb_nelems(gsb);
    let p = (kvmppc_gsb_header(gsb) as *mut u8).add(kvmppc_gsb_len(gsb)) as *mut c_void;
    (*gsb).len += size;
    (*kvmppc_gsb_header(gsb)).nelems = cpu_to_be32(nelems + 1);
    p
}

unsafe fn kvmppc_gsid_class(iden: u16) -> c_int {
    if iden >= KVMPPC_GSE_GUESTWIDE_START && iden <= KVMPPC_GSE_GUESTWIDE_END { return KVMPPC_GS_CLASS_GUESTWIDE; }
    if iden >= KVMPPC_GSE_HOSTWIDE_START && iden <= KVMPPC_GSE_HOSTWIDE_END { return KVMPPC_GS_CLASS_HOSTWIDE; }
    if iden >= KVMPPC_GSE_META_START && iden <= KVMPPC_GSE_META_END { return KVMPPC_GS_CLASS_META; }
    if iden >= KVMPPC_GSE_DW_REGS_START && iden <= KVMPPC_GSE_DW_REGS_END { return KVMPPC_GS_CLASS_DWORD_REG; }
    if iden >= KVMPPC_GSE_W_REGS_START && iden <= KVMPPC_GSE_W_REGS_END { return KVMPPC_GS_CLASS_WORD_REG; }
    if iden >= KVMPPC_GSE_VSRS_START && iden <= KVMPPC_GSE_VSRS_END { return KVMPPC_GS_CLASS_VECTOR; }
    if iden >= KVMPPC_GSE_INTR_REGS_START && iden <= KVMPPC_GSE_INTR_REGS_END { return KVMPPC_GS_CLASS_INTR; }
    -1
}

unsafe fn kvmppc_gsid_type(iden: u16) -> c_int {
    let mut ty = -1;
    match kvmppc_gsid_class(iden) {
        KVMPPC_GS_CLASS_HOSTWIDE => match iden {
            KVMPPC_GSID_L0_GUEST_HEAP | KVMPPC_GSID_L0_GUEST_HEAP_MAX | KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE | KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX | KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM => ty = KVMPPC_GSE_BE64,
            _ => {}
        },
        KVMPPC_GS_CLASS_GUESTWIDE => match iden {
            KVMPPC_GSID_HOST_STATE_SIZE | KVMPPC_GSID_RUN_OUTPUT_MIN_SIZE | KVMPPC_GSID_TB_OFFSET => ty = KVMPPC_GSE_BE64,
            KVMPPC_GSID_PARTITION_TABLE => ty = KVMPPC_GSE_PARTITION_TABLE,
            KVMPPC_GSID_PROCESS_TABLE => ty = KVMPPC_GSE_PROCESS_TABLE,
            KVMPPC_GSID_LOGICAL_PVR => ty = KVMPPC_GSE_BE32,
            _ => {}
        },
        KVMPPC_GS_CLASS_META => match iden {
            KVMPPC_GSID_RUN_INPUT | KVMPPC_GSID_RUN_OUTPUT => ty = KVMPPC_GSE_BUFFER,
            KVMPPC_GSID_VPA => ty = KVMPPC_GSE_BE64,
            _ => {}
        },
        KVMPPC_GS_CLASS_DWORD_REG => ty = KVMPPC_GSE_BE64,
        KVMPPC_GS_CLASS_WORD_REG => ty = KVMPPC_GSE_BE32,
        KVMPPC_GS_CLASS_VECTOR => ty = KVMPPC_GSE_VEC128,
        KVMPPC_GS_CLASS_INTR => match iden {
            KVMPPC_GSID_HDAR | KVMPPC_GSID_ASDR | KVMPPC_GSID_HEIR => ty = KVMPPC_GSE_BE64,
            KVMPPC_GSID_HDSISR => ty = KVMPPC_GSE_BE32,
            _ => {}
        },
        _ => {}
    }
    ty
}

pub unsafe fn kvmppc_gsid_flags(iden: u16) -> c_ulong {
    match kvmppc_gsid_class(iden) {
        KVMPPC_GS_CLASS_GUESTWIDE => KVMPPC_GS_FLAGS_WIDE,
        KVMPPC_GS_CLASS_HOSTWIDE => KVMPPC_GS_FLAGS_HOST_WIDE,
        _ => 0,
    }
}

pub unsafe fn kvmppc_gsid_size(iden: u16) -> u16 {
    let ty = kvmppc_gsid_type(iden);
    if ty == -1 || ty >= __KVMPPC_GSE_TYPE_MAX { return 0; }
    KVM_PPC_GSE_IDEN_LEN[ty as usize]
}

pub unsafe fn kvmppc_gsid_mask(iden: u16) -> u64 {
    match iden {
        KVMPPC_GSID_LPCR => LPCR_DPFD | LPCR_ILE | LPCR_AIL | LPCR_LD | LPCR_MER | LPCR_GTSE,
        KVMPPC_GSID_MSR => !(MSR_HV | MSR_S | MSR_ME),
        _ => !0u64,
    }
}

pub unsafe fn __kvmppc_gse_put(gsb: *mut kvmppc_gs_buff, iden: u16, size: u16, data: *const c_void) -> c_int {
    let total_size = core::mem::size_of::<kvmppc_gs_elem>() + size as usize;
    if total_size + kvmppc_gsb_len(gsb) > kvmppc_gsb_capacity(gsb) { return -ENOMEM; }
    if kvmppc_gsid_size(iden) != size { return -EINVAL; }
    let gse = kvmppc_gsb_put(gsb, total_size) as *mut kvmppc_gs_elem;
    (*gse).iden = cpu_to_be16(iden);
    (*gse).len = cpu_to_be16(size);
    memcpy((*gse).data.as_mut_ptr() as *mut c_void, data, size as usize);
    0
}

pub unsafe fn kvmppc_gse_parse(gsp: *mut kvmppc_gs_parser, gsb: *mut kvmppc_gs_buff) -> c_int {
    let mut i = 0;
    let mut curr: *mut kvmppc_gs_elem = core::ptr::null_mut();
    let mut rem = 0;
    kvmppc_gsb_for_each_elem!(i, curr, gsb, rem);
    if kvmppc_gsb_nelems(gsb) != i { return -EINVAL; }
    0
}

unsafe fn kvmppc_gse_flatten_iden(iden: u16) -> c_int {
    let class = kvmppc_gsid_class(iden);
    if class == KVMPPC_GS_CLASS_GUESTWIDE { return (iden - KVMPPC_GSE_GUESTWIDE_START) as c_int; }
    let mut bit = KVMPPC_GSE_GUESTWIDE_COUNT;
    if class == KVMPPC_GS_CLASS_HOSTWIDE { return bit + (iden - KVMPPC_GSE_HOSTWIDE_START) as c_int; }
    bit += KVMPPC_GSE_HOSTWIDE_COUNT;
    if class == KVMPPC_GS_CLASS_META { return bit + (iden - KVMPPC_GSE_META_START) as c_int; }
    bit += KVMPPC_GSE_META_COUNT;
    if class == KVMPPC_GS_CLASS_DWORD_REG { return bit + (iden - KVMPPC_GSE_DW_REGS_START) as c_int; }
    bit += KVMPPC_GSE_DW_REGS_COUNT;
    if class == KVMPPC_GS_CLASS_WORD_REG { return bit + (iden - KVMPPC_GSE_W_REGS_START) as c_int; }
    bit += KVMPPC_GSE_W_REGS_COUNT;
    if class == KVMPPC_GS_CLASS_VECTOR { return bit + (iden - KVMPPC_GSE_VSRS_START) as c_int; }
    bit += KVMPPC_GSE_VSRS_COUNT;
    if class == KVMPPC_GS_CLASS_INTR { return bit + (iden - KVMPPC_GSE_INTR_REGS_START) as c_int; }
    0
}

unsafe fn kvmppc_gse_unflatten_iden(mut bit: c_int) -> u16 {
    if bit < KVMPPC_GSE_GUESTWIDE_COUNT { return KVMPPC_GSE_GUESTWIDE_START + bit as u16; }
    bit -= KVMPPC_GSE_GUESTWIDE_COUNT;
    if bit < KVMPPC_GSE_HOSTWIDE_COUNT { return KVMPPC_GSE_HOSTWIDE_START + bit as u16; }
    bit -= KVMPPC_GSE_HOSTWIDE_COUNT;
    if bit < KVMPPC_GSE_META_COUNT { return KVMPPC_GSE_META_START + bit as u16; }
    bit -= KVMPPC_GSE_META_COUNT;
    if bit < KVMPPC_GSE_DW_REGS_COUNT { return KVMPPC_GSE_DW_REGS_START + bit as u16; }
    bit -= KVMPPC_GSE_DW_REGS_COUNT;
    if bit < KVMPPC_GSE_W_REGS_COUNT { return KVMPPC_GSE_W_REGS_START + bit as u16; }
    bit -= KVMPPC_GSE_W_REGS_COUNT;
    if bit < KVMPPC_GSE_VSRS_COUNT { return KVMPPC_GSE_VSRS_START + bit as u16; }
    bit -= KVMPPC_GSE_VSRS_COUNT;
    if bit < KVMPPC_GSE_IDEN_COUNT { return KVMPPC_GSE_INTR_REGS_START + bit as u16; }
    0
}

pub unsafe fn kvmppc_gsp_insert(gsp: *mut kvmppc_gs_parser, iden: u16, gse: *mut kvmppc_gs_elem) {
    let i = kvmppc_gse_flatten_iden(iden) as usize;
    kvmppc_gsbm_set(&mut (*gsp).iterator, iden);
    (*gsp).gses[i] = gse;
}

pub unsafe fn kvmppc_gsp_lookup(gsp: *mut kvmppc_gs_parser, iden: u16) -> *mut kvmppc_gs_elem {
    (*gsp).gses[kvmppc_gse_flatten_iden(iden) as usize]
}

pub unsafe fn kvmppc_gsbm_set(gsbm: *mut kvmppc_gs_bitmap, iden: u16) { set_bit(kvmppc_gse_flatten_iden(iden), (*gsbm).bitmap); }
pub unsafe fn kvmppc_gsbm_clear(gsbm: *mut kvmppc_gs_bitmap, iden: u16) { clear_bit(kvmppc_gse_flatten_iden(iden), (*gsbm).bitmap); }
pub unsafe fn kvmppc_gsbm_test(gsbm: *mut kvmppc_gs_bitmap, iden: u16) -> bool { test_bit(kvmppc_gse_flatten_iden(iden), (*gsbm).bitmap) }

pub unsafe fn kvmppc_gsbm_next(gsbm: *mut kvmppc_gs_bitmap, prev: u16) -> u16 {
    let pbit = if prev != 0 { kvmppc_gse_flatten_iden(prev) + 1 } else { 0 };
    let bit = find_next_bit((*gsbm).bitmap, KVMPPC_GSE_IDEN_COUNT, pbit);
    if bit < KVMPPC_GSE_IDEN_COUNT { kvmppc_gse_unflatten_iden(bit) } else { 0 }
}

pub unsafe fn kvmppc_gsm_init(gsm: *mut kvmppc_gs_msg, ops: *mut kvmppc_gs_msg_ops, data: *mut c_void, flags: c_ulong) -> c_int {
    memset(gsm as *mut c_void, 0, core::mem::size_of::<kvmppc_gs_msg>());
    (*gsm).ops = ops; (*gsm).data = data; (*gsm).flags = flags; 0
}

pub unsafe fn kvmppc_gsm_new(ops: *mut kvmppc_gs_msg_ops, data: *mut c_void, flags: c_ulong, gfp_flags: gfp_t) -> *mut kvmppc_gs_msg {
    let gsm = kzalloc_obj::<kvmppc_gs_msg>(gfp_flags);
    if gsm.is_null() { return core::ptr::null_mut(); }
    kvmppc_gsm_init(gsm, ops, data, flags); gsm
}

pub unsafe fn kvmppc_gsm_size(gsm: *mut kvmppc_gs_msg) -> usize { if (*(*gsm).ops).get_size.is_some() { ((*(*gsm).ops).get_size.unwrap())(gsm) } else { 0 } }
pub unsafe fn kvmppc_gsm_free(gsm: *mut kvmppc_gs_msg) { kfree(gsm as *mut c_void); }
pub unsafe fn kvmppc_gsm_fill_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> c_int { if (*(*gsm).ops).fill_info.is_none() { return -EINVAL; } ((*(*gsm).ops).fill_info.unwrap())(gsb, gsm) }
pub unsafe fn kvmppc_gsm_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> c_int { if (*(*gsm).ops).fill_info.is_none() { return -EINVAL; } ((*(*gsm).ops).refresh_info.unwrap())(gsm, gsb) }

pub unsafe fn kvmppc_gsb_send(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int {
    if kvmppc_gsb_nelems(gsb) == 0 { return 0; }
    let mut hflags = 0;
    if flags & KVMPPC_GS_FLAGS_WIDE != 0 { hflags |= H_GUEST_FLAGS_WIDE; }
    if flags & KVMPPC_GS_FLAGS_HOST_WIDE != 0 { hflags |= H_GUEST_FLAGS_HOST_WIDE; }
    let mut i = 0; plpar_guest_set_state(hflags, (*gsb).guest_id, (*gsb).vcpu_id, __pa((*gsb).hdr), (*gsb).capacity, &mut i)
}

pub unsafe fn kvmppc_gsb_recv(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int {
    let mut hflags = 0;
    if flags & KVMPPC_GS_FLAGS_WIDE != 0 { hflags |= H_GUEST_FLAGS_WIDE; }
    if flags & KVMPPC_GS_FLAGS_HOST_WIDE != 0 { hflags |= H_GUEST_FLAGS_HOST_WIDE; }
    let mut i = 0; plpar_guest_get_state(hflags, (*gsb).guest_id, (*gsb).vcpu_id, __pa((*gsb).hdr), (*gsb).capacity, &mut i)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
