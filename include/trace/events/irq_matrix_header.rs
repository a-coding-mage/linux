// Translation of trace/events/irq_matrix.h.
// C preprocessor tracepoint declarations are represented here as C-layout
// entry records and unsafe recording helpers.

#[repr(C)]
pub struct irq_matrix {
    pub online_maps: ::core::ffi::c_uint,
    pub global_available: ::core::ffi::c_uint,
    pub global_reserved: ::core::ffi::c_uint,
    pub total_allocated: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct cpumap {
    pub online: bool,
    pub available: ::core::ffi::c_uint,
    pub allocated: ::core::ffi::c_uint,
    pub managed: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct irq_matrix_global_entry {
    pub online_maps: ::core::ffi::c_uint,
    pub global_available: ::core::ffi::c_uint,
    pub global_reserved: ::core::ffi::c_uint,
    pub total_allocated: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct irq_matrix_global_update_entry {
    pub bit: ::core::ffi::c_int,
    pub online_maps: ::core::ffi::c_uint,
    pub global_available: ::core::ffi::c_uint,
    pub global_reserved: ::core::ffi::c_uint,
    pub total_allocated: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct irq_matrix_cpu_entry {
    pub bit: ::core::ffi::c_int,
    pub cpu: ::core::ffi::c_uint,
    pub online: bool,
    pub available: ::core::ffi::c_uint,
    pub allocated: ::core::ffi::c_uint,
    pub managed: ::core::ffi::c_uint,
    pub online_maps: ::core::ffi::c_uint,
    pub global_available: ::core::ffi::c_uint,
    pub global_reserved: ::core::ffi::c_uint,
    pub total_allocated: ::core::ffi::c_uint,
}

#[inline]
pub unsafe fn irq_matrix_global(matrix: *const irq_matrix) -> irq_matrix_global_entry {
    irq_matrix_global_entry {
        online_maps: (*matrix).online_maps,
        global_available: (*matrix).global_available,
        global_reserved: (*matrix).global_reserved,
        total_allocated: (*matrix).total_allocated,
    }
}

#[inline]
pub unsafe fn irq_matrix_global_update(
    bit: ::core::ffi::c_int,
    matrix: *const irq_matrix,
) -> irq_matrix_global_update_entry {
    irq_matrix_global_update_entry {
        bit,
        online_maps: (*matrix).online_maps,
        global_available: (*matrix).global_available,
        global_reserved: (*matrix).global_reserved,
        total_allocated: (*matrix).total_allocated,
    }
}

#[inline]
pub unsafe fn irq_matrix_cpu(
    bit: ::core::ffi::c_int,
    cpu: ::core::ffi::c_uint,
    matrix: *const irq_matrix,
    cmap: *const cpumap,
) -> irq_matrix_cpu_entry {
    irq_matrix_cpu_entry {
        bit,
        cpu,
        online: (*cmap).online,
        available: (*cmap).available,
        allocated: (*cmap).allocated,
        managed: (*cmap).managed,
        online_maps: (*matrix).online_maps,
        global_available: (*matrix).global_available,
        global_reserved: (*matrix).global_reserved,
        total_allocated: (*matrix).total_allocated,
    }
}

// DEFINE_EVENT instances for irq_matrix_global.
pub type irq_matrix_online = irq_matrix_global_entry;
pub type irq_matrix_offline = irq_matrix_global_entry;
pub type irq_matrix_reserve = irq_matrix_global_entry;
pub type irq_matrix_remove_reserved = irq_matrix_global_entry;

// DEFINE_EVENT instance for irq_matrix_global_update.
pub type irq_matrix_assign_system = irq_matrix_global_update_entry;

// DEFINE_EVENT instances for irq_matrix_cpu.
pub type irq_matrix_reserve_managed = irq_matrix_cpu_entry;
pub type irq_matrix_remove_managed = irq_matrix_cpu_entry;
pub type irq_matrix_alloc_managed = irq_matrix_cpu_entry;
pub type irq_matrix_assign = irq_matrix_cpu_entry;
pub type irq_matrix_alloc = irq_matrix_cpu_entry;
pub type irq_matrix_free = irq_matrix_cpu_entry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
