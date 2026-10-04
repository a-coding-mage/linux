/* SPDX-License-Identifier: GPL-2.0-only */
/* Fixed-format compiler/varargs leaves retain native printk-index metadata.
 * All diagnostic selection, rate limiting, and table walking is Rust. */
RM_VOID(log_suppressed, (unsigned long n), pr_alert("BUG: Bad page map: %lu messages suppressed\n", n))
RM_VOID(log_pgd, (const char *a), pr_alert("pgd:%s\n", a))
RM_VOID(log_p4d, (const char *a, const char *b), pr_alert("pgd:%s p4d:%s\n", a, b))
RM_VOID(log_pud, (const char *a, const char *b, const char *c), pr_alert("pgd:%s p4d:%s pud:%s\n", a, b, c))
RM_VOID(log_pmd, (const char *a, const char *b, const char *c, const char *d), pr_alert("pgd:%s p4d:%s pud:%s pmd:%s\n", a, b, c, d))
RM_VOID(log_bad_map_header, (const char *comm, const char *level, const char *entry), pr_alert("BUG: Bad page map in process %s  %s:%s", comm, level, entry))
RM_VOID(log_bad_map_location, (void *a, unsigned long f, struct anon_vma *v, struct address_space *m), pr_alert("addr:%px vm_flags:%08lx anon_vma:%px mapping:%px", a, f, v, m))
RM_VOID(log_bad_map_index, (pgoff_t i), pr_cont(" index:%lx\n", i))
RM_VOID(log_bad_map_indices, (pgoff_t i, pgoff_t a), pr_cont(" index:%lx (file) %lx (anon)\n", i, a))
RM_VOID(log_bad_map_file, (struct file *f, const void *fault, const void *mmap, const void *prepare, const void *read), pr_alert("file:%pD fault:%ps mmap:%ps mmap_prepare: %ps read_folio:%ps\n", f, fault, mmap, prepare, read))
RM_VOID(log_unknown_swap, (unsigned long v), pr_alert("unrecognized swap entry 0x%lx\n", v))
