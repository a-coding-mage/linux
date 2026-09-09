/* SPDX-License-Identifier: GPL-2.0 */

// Translation of trace/events/fs_dax.h.  The tracepoint declaration helpers
// and the kernel types referenced below are supplied by the surrounding
// kernel/tracepoint environment.

// C: DECLARE_EVENT_CLASS(dax_pmd_fault_class, ...)
pub const DAX_PMD_FAULT_CLASS: &str = "dax_pmd_fault_class";
pub const DAX_PMD_FAULT_CLASS_PROTO:
    &str = "struct inode *inode, struct vm_fault *vmf, pgoff_t max_pgoff, int result";

// TP_STRUCT__entry:
//   u64 ino; unsigned long vm_start, vm_end, address;
//   vm_flags_t vm_flags; pgoff_t pgoff, max_pgoff; dev_t dev;
//   unsigned int flags; int result;
// TP_fast_assign copies inode->i_sb->s_dev, inode->i_ino, vmf->vma fields,
// vmf->address, vmf->flags, vmf->pgoff, max_pgoff, and result in that order.
// TP_printk formats the device, inode, sharing mode, fault flags, address,
// VMA bounds, page offsets, and VM fault result as in the C declaration.

macro_rules! define_pmd_fault_event {
    ($name:ident) => {
        pub const $name: &str = stringify!($name);
    };
}

define_pmd_fault_event!(dax_pmd_fault);
define_pmd_fault_event!(dax_pmd_fault_done);

// C: DECLARE_EVENT_CLASS(dax_pmd_load_hole_class, ...)
pub const DAX_PMD_LOAD_HOLE_CLASS: &str = "dax_pmd_load_hole_class";
pub const DAX_PMD_LOAD_HOLE_CLASS_PROTO:
    &str = "struct inode *inode, struct vm_fault *vmf, struct folio *zero_folio, void *radix_entry";
// Entry fields are: u64 ino, vm_flags_t vm_flags, unsigned long address,
// struct folio *zero_folio, void *radix_entry, and dev_t dev.
// Assignment and printk formatting are preserved by the tracepoint backend.

macro_rules! define_pmd_load_hole_event {
    ($name:ident) => {
        pub const $name: &str = stringify!($name);
    };
}

define_pmd_load_hole_event!(dax_pmd_load_hole);
define_pmd_load_hole_event!(dax_pmd_load_hole_fallback);

// C: DECLARE_EVENT_CLASS(dax_pte_fault_class, ...)
pub const DAX_PTE_FAULT_CLASS: &str = "dax_pte_fault_class";
pub const DAX_PTE_FAULT_CLASS_PROTO: &str =
    "struct inode *inode, struct vm_fault *vmf, int result";
// Entry fields are: u64 ino, vm_flags_t vm_flags, unsigned long address,
// pgoff_t pgoff, dev_t dev, unsigned int flags, and int result.
// Assignment and printk formatting are preserved by the tracepoint backend.

macro_rules! define_pte_fault_event {
    ($name:ident) => {
        pub const $name: &str = stringify!($name);
    };
}

define_pte_fault_event!(dax_pte_fault);
define_pte_fault_event!(dax_pte_fault_done);
define_pte_fault_event!(dax_load_hole);
define_pte_fault_event!(dax_insert_pfn_mkwrite_no_entry);
define_pte_fault_event!(dax_insert_pfn_mkwrite);

// C: DECLARE_EVENT_CLASS(dax_writeback_range_class, ...)
pub const DAX_WRITEBACK_RANGE_CLASS: &str = "dax_writeback_range_class";
pub const DAX_WRITEBACK_RANGE_CLASS_PROTO: &str =
    "struct inode *inode, pgoff_t start_index, pgoff_t end_index";
// Entry fields are: u64 ino, pgoff_t start_index, pgoff_t end_index, dev_t dev.
// Assignment and printk formatting are preserved by the tracepoint backend.

macro_rules! define_writeback_range_event {
    ($name:ident) => {
        pub const $name: &str = stringify!($name);
    };
}

define_writeback_range_event!(dax_writeback_range);
define_writeback_range_event!(dax_writeback_range_done);

// TRACE_EVENT(dax_writeback_one, ...)
pub const DAX_WRITEBACK_ONE: &str = "dax_writeback_one";
pub const DAX_WRITEBACK_ONE_PROTO: &str =
    "struct inode *inode, pgoff_t pgoff, pgoff_t pglen";
// Entry fields are: u64 ino, pgoff_t pgoff, pgoff_t pglen, dev_t dev.
// TP_fast_assign copies inode->i_sb->s_dev, inode->i_ino, pgoff, and pglen;
// TP_printk formats the device, inode, page offset, and page length.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
