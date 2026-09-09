// SPDX-License-Identifier: GPL-2.0-only
/*
 * The IRET instruction, when returning to a 16-bit segment, only restores
 * the bottom 16 bits of the user space stack pointer.  This file sets up
 * the espfix ministacks and related page tables.
 *
 * C headers and build-time configuration symbols are supplied by the
 * surrounding kernel translation unit.
 */

const ESPFIX_STACK_SIZE: usize = 8 * 8;
const ESPFIX_STACKS_PER_PAGE: usize = PAGE_SIZE / ESPFIX_STACK_SIZE;
const ESPFIX_PAGE_SPACE: usize = 1usize << (P4D_SHIFT - PAGE_SHIFT - 16);
const ESPFIX_MAX_CPUS: usize = ESPFIX_STACKS_PER_PAGE * ESPFIX_PAGE_SPACE;
const PGALLOC_GFP: gfp_t = GFP_KERNEL | __GFP_ZERO;
const PTE_STRIDE: usize = 65536 / PAGE_SIZE;
const ESPFIX_PTE_CLONES: usize = PTRS_PER_PTE / PTE_STRIDE;
const ESPFIX_PMD_CLONES: usize = PTRS_PER_PMD;
const ESPFIX_PUD_CLONES: usize = 65536 / (ESPFIX_PTE_CLONES * ESPFIX_PMD_CLONES);
const PGTABLE_PROT: pteval_t = (_KERNPG_TABLE & !_PAGE_RW) | _PAGE_NX;
const ESPFIX_MAX_PAGES: usize = div_round_up(CONFIG_NR_CPUS, ESPFIX_STACKS_PER_PAGE);

// DEFINE_PER_CPU_READ_MOSTLY(unsigned long, espfix_stack);
// DEFINE_PER_CPU_READ_MOSTLY(unsigned long, espfix_waddr);
static mut espfix_stack: [usize; CONFIG_NR_CPUS] = [0; CONFIG_NR_CPUS];
static mut espfix_waddr: [usize; CONFIG_NR_CPUS] = [0; CONFIG_NR_CPUS];

// static DEFINE_MUTEX(espfix_init_mutex);
static mut espfix_pages: [*mut core::ffi::c_void; ESPFIX_MAX_PAGES] =
    [core::ptr::null_mut(); ESPFIX_MAX_PAGES];

// __page_aligned_bss pud_t espfix_pud_page[PTRS_PER_PUD]
//     __aligned(PAGE_SIZE);
static mut espfix_pud_page: [pud_t; PTRS_PER_PUD] = [unsafe { core::mem::zeroed() }; PTRS_PER_PUD];

static mut page_random: u32 = 0;
static mut slot_random: u32 = 0;

#[inline]
unsafe fn espfix_base_addr(cpu: u32) -> usize {
    let page = (cpu as usize / ESPFIX_STACKS_PER_PAGE) ^ page_random as usize;
    let slot = (cpu as usize + slot_random as usize) % ESPFIX_STACKS_PER_PAGE;
    let mut addr = (page << PAGE_SHIFT) + slot * ESPFIX_STACK_SIZE;
    addr = (addr & 0xffff) | ((addr & !0xffff) << 16);
    addr + ESPFIX_BASE_ADDR
}

unsafe fn init_espfix_random() {
    let rand = get_random_long();
    slot_random = (rand % ESPFIX_STACKS_PER_PAGE as u64) as u32;
    page_random = ((rand / ESPFIX_STACKS_PER_PAGE as u64)
        & (ESPFIX_PAGE_SPACE as u64 - 1)) as u32;
}

pub unsafe fn init_espfix_bsp() {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;

    // FRED systems always restore the full value of %rsp.
    if cpu_feature_enabled(X86_FEATURE_FRED) {
        return;
    }

    pgd = &mut init_top_pgt[pgd_index(ESPFIX_BASE_ADDR)];
    p4d = p4d_alloc(&mut init_mm, pgd, ESPFIX_BASE_ADDR);
    p4d_populate(&mut init_mm, p4d, espfix_pud_page.as_mut_ptr());

    init_espfix_random();
    init_espfix_ap(0);
}

pub unsafe fn init_espfix_ap(cpu: i32) {
    let page: usize;
    let addr: usize;
    let mut pud: pud_t;
    let pud_p: *mut pud_t;
    let mut pmd: pmd_t;
    let mut pmd_p: *mut pmd_t;
    let mut pte: pte_t;
    let mut pte_p: *mut pte_t;
    let mut n: i32;
    let node: i32;
    let mut stack_page: *mut core::ffi::c_void;
    let ptemask: pteval_t;

    // FRED systems always restore the full value of %rsp.
    if cpu_feature_enabled(X86_FEATURE_FRED) {
        return;
    }
    if likely(per_cpu(&espfix_stack, cpu) != 0) {
        return;
    }

    addr = espfix_base_addr(cpu as u32);
    page = cpu as usize / ESPFIX_STACKS_PER_PAGE;
    stack_page = READ_ONCE(espfix_pages[page]);
    if likely(!stack_page.is_null()) {
        goto_done!();
    }

    mutex_lock(&mut espfix_init_mutex);
    stack_page = READ_ONCE(espfix_pages[page]);
    if !stack_page.is_null() {
        mutex_unlock(&mut espfix_init_mutex);
        goto_done!();
    }

    node = cpu_to_node(cpu);
    ptemask = __supported_pte_mask;
    pud_p = espfix_pud_page.as_mut_ptr().add(pud_index(addr));
    pud = *pud_p;
    if !pud_present(pud) {
        let page = alloc_pages_node(node, PGALLOC_GFP, 0);
        pmd_p = page_address(page) as *mut pmd_t;
        pud = __pud(__pa(pmd_p) | (PGTABLE_PROT & ptemask));
        paravirt_alloc_pmd(&mut init_mm, __pa(pmd_p) >> PAGE_SHIFT);
        for n in 0..ESPFIX_PUD_CLONES as i32 { set_pud(pud_p.add(n as usize), pud); }
    }

    pmd_p = pmd_offset(&mut pud, addr);
    pmd = *pmd_p;
    if !pmd_present(pmd) {
        let page = alloc_pages_node(node, PGALLOC_GFP, 0);
        pte_p = page_address(page) as *mut pte_t;
        pmd = __pmd(__pa(pte_p) | (PGTABLE_PROT & ptemask));
        paravirt_alloc_pte(&mut init_mm, __pa(pte_p) >> PAGE_SHIFT);
        for n in 0..ESPFIX_PMD_CLONES as i32 { set_pmd(pmd_p.add(n as usize), pmd); }
    }

    pte_p = pte_offset_kernel(&mut pmd, addr);
    stack_page = page_address(alloc_pages_node(node, GFP_KERNEL, 0));
    pte = __pte(__pa(stack_page) | ((__PAGE_KERNEL_RO | _PAGE_ENC) & ptemask));
    for n in 0..ESPFIX_PTE_CLONES as i32 {
        set_pte(pte_p.add(n as usize * PTE_STRIDE), pte);
    }
    WRITE_ONCE(espfix_pages[page], stack_page);
    mutex_unlock(&mut espfix_init_mutex);

    goto_done!();
    macro_rules! goto_done { () => {
        per_cpu_set(&mut espfix_stack, cpu, addr);
        per_cpu_set(&mut espfix_waddr, cpu, stack_page as usize + (addr & !PAGE_MASK));
    }}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
