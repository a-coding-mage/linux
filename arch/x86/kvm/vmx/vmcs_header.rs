/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translation unit. */

extern "C" {
    pub static mut current_vmcs: *mut vmcs;
}

/*
 * Indexing into the vmcs12 uses the VMCS encoding rotated left by 6 as a very
 * rudimentary compression of the range of indices.  The compression ratio is
 * good enough to allow KVM to use a (very sparsely populated) array without
 * wasting too much memory, while the "algorithm" is fast enough to be used to
 * lookup vmcs12 fields on-demand, e.g. for emulation.
 */
#[inline(always)]
pub const fn rol16(val: u16, n: u32) -> u16 {
    val.rotate_left(n)
}

#[inline(always)]
pub const fn vmcs12_idx_to_enc(idx: u16) -> u16 {
    rol16(idx, 10)
}

#[inline(always)]
pub const fn enc_to_vmcs12_idx(enc: u16) -> u16 {
    rol16(enc, 6)
}

/*
 * vmcs_host_state tracks registers that are loaded from the VMCS on VMEXIT
 * and whose values change infrequently, but are not constant.  I.e. this is
 * used as a write-through cache of the corresponding VMCS fields.
 */
#[repr(C)]
pub struct vmcs_host_state {
    pub cr3: c_ulong, /* May not match real cr3 */
    pub cr4: c_ulong, /* May not match real cr4 */
    pub gs_base: c_ulong,
    pub fs_base: c_ulong,
    pub rsp: c_ulong,

    pub fs_sel: u16,
    pub gs_sel: u16,
    pub ldt_sel: u16,
    #[cfg(CONFIG_X86_64)]
    pub ds_sel: u16,
    #[cfg(CONFIG_X86_64)]
    pub es_sel: u16,
}

#[repr(C)]
pub struct vmcs_controls_shadow {
    pub vm_entry: u32,
    pub vm_exit: u32,
    pub pin: u32,
    pub exec: u32,
    pub secondary_exec: u32,
    pub tertiary_exec: u64,
}

/*
 * Track a VMCS that may be loaded on a certain CPU. If it is (cpu!=-1), also
 * remember whether it was VMLAUNCHed, and maintain a linked list of all VMCSs
 * loaded on this CPU (so we can clear them if the CPU goes down).
 */
#[repr(C)]
pub struct loaded_vmcs {
    pub vmcs: *mut vmcs,
    pub shadow_vmcs: *mut vmcs,
    pub cpu: i32,
    pub launched: bool,
    pub nmi_known_unmasked: bool,
    pub hv_timer_soft_disabled: bool,
    /* Support for vnmi-less CPUs */
    pub soft_vnmi_blocked: i32,
    pub entry_time: ktime_t,
    pub vnmi_blocked_time: i64,
    pub msr_bitmap: *mut c_ulong,
    pub loaded_vmcss_on_cpu_link: list_head,
    pub host_state: vmcs_host_state,
    pub controls_shadow: vmcs_controls_shadow,
}

#[inline(always)]
pub fn is_intr_type(intr_info: u32, type_: u32) -> bool {
    let mask: u32 = INTR_INFO_VALID_MASK | INTR_INFO_INTR_TYPE_MASK;
    (intr_info & mask) == (INTR_INFO_VALID_MASK | type_)
}

#[inline]
pub fn is_intr_type_n(intr_info: u32, type_: u32, vector: u8) -> bool {
    let mask: u32 = INTR_INFO_VALID_MASK | INTR_INFO_INTR_TYPE_MASK |
        INTR_INFO_VECTOR_MASK;
    (intr_info & mask) == (INTR_INFO_VALID_MASK | type_ | vector as u32)
}

#[inline]
pub fn is_exception_n(intr_info: u32, vector: u8) -> bool {
    is_intr_type_n(intr_info, INTR_TYPE_HARD_EXCEPTION, vector)
}

#[inline]
pub fn is_debug(intr_info: u32) -> bool { is_exception_n(intr_info, DB_VECTOR) }

#[inline]
pub fn is_breakpoint(intr_info: u32) -> bool { is_exception_n(intr_info, BP_VECTOR) }

#[inline]
pub fn is_double_fault(intr_info: u32) -> bool { is_exception_n(intr_info, DF_VECTOR) }

#[inline]
pub fn is_page_fault(intr_info: u32) -> bool { is_exception_n(intr_info, PF_VECTOR) }

#[inline]
pub fn is_invalid_opcode(intr_info: u32) -> bool { is_exception_n(intr_info, UD_VECTOR) }

#[inline]
pub fn is_gp_fault(intr_info: u32) -> bool { is_exception_n(intr_info, GP_VECTOR) }

#[inline]
pub fn is_alignment_check(intr_info: u32) -> bool { is_exception_n(intr_info, AC_VECTOR) }

#[inline]
pub fn is_machine_check(intr_info: u32) -> bool { is_exception_n(intr_info, MC_VECTOR) }

#[inline]
pub fn is_nm_fault(intr_info: u32) -> bool { is_exception_n(intr_info, NM_VECTOR) }

#[inline]
pub fn is_ve_fault(intr_info: u32) -> bool { is_exception_n(intr_info, VE_VECTOR) }

/* Undocumented: icebp/int1 */
#[inline]
pub fn is_icebp(intr_info: u32) -> bool {
    is_intr_type(intr_info, INTR_TYPE_PRIV_SW_EXCEPTION)
}

#[inline(always)]
pub fn is_nmi(intr_info: u32) -> bool {
    is_intr_type(intr_info, INTR_TYPE_NMI_INTR)
}

#[inline]
pub fn is_external_intr(intr_info: u32) -> bool {
    is_intr_type(intr_info, INTR_TYPE_EXT_INTR)
}

#[inline]
pub fn is_exception_with_error_code(intr_info: u32) -> bool {
    let mask: u32 = INTR_INFO_VALID_MASK | INTR_INFO_DELIVER_CODE_MASK;
    (intr_info & mask) == mask
}

#[repr(C)]
pub enum vmcs_field_width {
    VMCS_FIELD_WIDTH_U16 = 0,
    VMCS_FIELD_WIDTH_U64 = 1,
    VMCS_FIELD_WIDTH_U32 = 2,
    VMCS_FIELD_WIDTH_NATURAL_WIDTH = 3,
}

#[inline]
pub fn vmcs_field_width(field: c_ulong) -> i32 {
    if (0x1 & field) != 0 { /* the *_HIGH fields are all 32 bit */
        return VMCS_FIELD_WIDTH_U32 as i32;
    }
    ((field >> 13) & 0x3) as i32
}

#[inline]
pub fn vmcs_field_readonly(field: c_ulong) -> i32 {
    (((field >> 10) & 0x3) == 1) as i32
}

pub const VMCS_FIELD_INDEX_SHIFT: c_ulong = 1;
pub const VMCS_FIELD_INDEX_MASK: c_ulong = GENMASK(9, 1);

#[inline]
pub fn vmcs_field_index(field: c_ulong) -> c_uint {
    ((field & VMCS_FIELD_INDEX_MASK) >> VMCS_FIELD_INDEX_SHIFT) as c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
