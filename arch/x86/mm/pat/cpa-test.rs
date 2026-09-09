// SPDX-License-Identifier: GPL-2.0
/*
 * self test for change_page_attr.
 *
 * Clears the a test pte bit on random pages in the direct mapping,
 * then reverts and compares page tables forwards and afterwards.
 */

/* Kernel dependencies supplied by the surrounding build. */

/* Only print the results of the first pass: */
static mut PRINT: i32 = 1;

const NTEST: usize = 3 * 100;
const NPAGES: usize = 100;
// LPS is selected by CONFIG_X86_64, CONFIG_X86_PAE, or the 32-bit fallback.
const LPS: usize = 1 << PMD_SHIFT;
const GPS: usize = 1 << 30;

// #define PAGE_CPA_TEST __pgprot(_PAGE_CPA_TEST)

#[repr(C)]
struct SplitState {
    lpg: i64,
    gpg: i64,
    spg: i64,
    exec: i64,
    min_exec: i64,
    max_exec: i64,
}

static mut ADDR: [usize; NTEST] = [0; NTEST];
static mut LEN: [u32; NTEST] = [0; NTEST];
static mut PAGES: [*mut Page; NPAGES] = [core::ptr::null_mut(); NPAGES];
static mut ADDRS: [usize; NPAGES] = [0; NPAGES];

unsafe fn pte_testbit(pte: PteT) -> i32 {
    pte_flags(pte) & _PAGE_SOFTW1
}

unsafe fn print_split(s: *mut SplitState) -> i32 {
    let mut i: usize = 0;
    let mut missed: i64 = 0;
    let mut err = 0;

    (*s).lpg = 0;
    (*s).gpg = 0;
    (*s).spg = 0;
    (*s).exec = 0;
    (*s).min_exec = !0i64;
    (*s).max_exec = 0;
    while i < max_pfn_mapped {
        let addr = __va(i << PAGE_SHIFT) as usize;
        let mut level: u32 = 0;
        let pte = lookup_address(addr, &mut level);
        if pte.is_null() {
            missed += 1;
            i += 1;
            continue;
        }

        if level == PG_LEVEL_1G && core::mem::size_of::<i64>() == 8 {
            (*s).gpg += 1;
            i += GPS / PAGE_SIZE;
        } else if level == PG_LEVEL_2M {
            if (pte_val(*pte) & _PAGE_PRESENT) != 0 && (pte_val(*pte) & _PAGE_PSE) == 0 {
                printk(KERN_ERR, "%lx level %d but not PSE %Lx\n", addr, level, pte_val(*pte) as u64);
                err = 1;
            }
            (*s).lpg += 1;
            i += LPS / PAGE_SIZE;
        } else {
            (*s).spg += 1;
            i += 1;
        }
        if (pte_val(*pte) & _PAGE_NX) == 0 {
            (*s).exec += 1;
            if (addr as i64 < (*s).min_exec) { (*s).min_exec = addr as i64; }
            if (addr as i64 > (*s).max_exec) { (*s).max_exec = addr as i64; }
        }
    }
    if PRINT != 0 {
        printk(KERN_INFO, " 4k %lu large %lu gb %lu x %lu[%lx-%lx] miss %lu\n",
            (*s).spg, (*s).lpg, (*s).gpg, (*s).exec,
            if (*s).min_exec != !0i64 { (*s).min_exec } else { 0 },
            (*s).max_exec, missed);
    }

    let expected = (((*s).gpg * GPS as i64 + (*s).lpg * LPS as i64) / PAGE_SIZE as i64)
        + (*s).spg + missed;
    if expected != i as i64 {
        printk(KERN_ERR, "CPA max_pfn_mapped %lu but expected %lu\n", max_pfn_mapped, expected);
        return 1;
    }
    err
}

unsafe fn pageattr_test() -> i32 {
    let mut sa = SplitState { lpg: 0, gpg: 0, spg: 0, exec: 0, min_exec: 0, max_exec: 0 };
    let mut sb = SplitState { lpg: 0, gpg: 0, spg: 0, exec: 0, min_exec: 0, max_exec: 0 };
    let mut sc = SplitState { lpg: 0, gpg: 0, spg: 0, exec: 0, min_exec: 0, max_exec: 0 };
    let bm = vzalloc((max_pfn_mapped + 7) / 8) as *mut usize;
    let mut failed = 0;
    if PRINT != 0 { printk(KERN_INFO, "CPA self-test:\n"); }
    if bm.is_null() { printk(KERN_ERR, "CPA Cannot vmalloc bitmap\n"); return -ENOMEM; }

    failed += print_split(&mut sa);
    for i in 0..NTEST {
        let pfn = get_random_u32_below(max_pfn_mapped) as usize;
        ADDR[i] = __va(pfn << PAGE_SHIFT) as usize;
        LEN[i] = get_random_u32_below(NPAGES as u32);
        LEN[i] = core::cmp::min(LEN[i] as usize, max_pfn_mapped - pfn - 1) as u32;
        if LEN[i] == 0 { LEN[i] = 1; }
        let mut pte: *mut PteT = core::ptr::null_mut();
        let mut pte0 = pfn_pte(0, __pgprot(0));
        let mut level = 0;
        let mut k = 0;
        while k < LEN[i] as usize {
            pte = lookup_address(ADDR[i] + k * PAGE_SIZE, &mut level);
            if pte.is_null() || pgprot_val(pte_pgprot(*pte)) == 0 || (pte_val(*pte) & _PAGE_PRESENT) == 0 {
                ADDR[i] = 0; break;
            }
            if k == 0 { pte0 = *pte; }
            else if pgprot_val(pte_pgprot(*pte)) != pgprot_val(pte_pgprot(pte0)) { LEN[i] = k as u32; break; }
            if test_bit(pfn + k, bm) != 0 { LEN[i] = k as u32; break; }
            __set_bit(pfn + k, bm);
            ADDRS[k] = ADDR[i] + k * PAGE_SIZE;
            PAGES[k] = pfn_to_page(pfn + k);
            k += 1;
        }
        if ADDR[i] == 0 || pte.is_null() || k == 0 { ADDR[i] = 0; continue; }
        let err = match i % 3 {
            0 => change_page_attr_set(&mut ADDR[i], LEN[i] as usize, __pgprot(_PAGE_CPA_TEST), 0),
            1 => change_page_attr_set(ADDRS.as_mut_ptr(), LEN[i] as usize, __pgprot(_PAGE_CPA_TEST), 1),
            _ => cpa_set_pages_array(PAGES.as_mut_ptr(), LEN[i] as usize, __pgprot(_PAGE_CPA_TEST)),
        };
        if err < 0 { printk(KERN_ERR, "CPA %d failed %d\n", i, err); failed += 1; }
        pte = lookup_address(ADDR[i], &mut level);
        if pte.is_null() || pte_testbit(*pte) == 0 || pte_huge(*pte) != 0 { printk(KERN_ERR, "CPA %lx: bad pte\n", ADDR[i]); failed += 1; }
        if level != PG_LEVEL_4K { printk(KERN_ERR, "CPA %lx: unexpected level %d\n", ADDR[i], level); failed += 1; }
    }
    vfree(bm as *mut core::ffi::c_void);
    failed += print_split(&mut sb);
    for i in 0..NTEST {
        if ADDR[i] == 0 { continue; }
        let mut level = 0;
        let mut pte = lookup_address(ADDR[i], &mut level);
        if pte.is_null() { printk(KERN_ERR, "CPA lookup of %lx failed\n", ADDR[i]); failed += 1; continue; }
        let err = change_page_attr_clear(&mut ADDR[i], LEN[i] as usize, __pgprot(_PAGE_CPA_TEST), 0);
        if err < 0 { printk(KERN_ERR, "CPA reverting failed: %d\n", err); failed += 1; }
        pte = lookup_address(ADDR[i], &mut level);
        if pte.is_null() || pte_testbit(*pte) != 0 { printk(KERN_ERR, "CPA %lx: bad pte after revert\n", ADDR[i]); failed += 1; }
    }
    failed += print_split(&mut sc);
    if failed != 0 { WARN(1, KERN_ERR "NOT PASSED. Please report.\n"); -EINVAL }
    else { if PRINT != 0 { printk(KERN_INFO, "ok.\n"); } 0 }
}

unsafe fn do_pageattr_test(_unused: *mut core::ffi::c_void) -> i32 {
    while kthread_should_stop() == 0 {
        schedule_timeout_interruptible(HZ * 30);
        if pageattr_test() < 0 { break; }
        if PRINT != 0 { PRINT -= 1; }
    }
    0
}

unsafe fn start_pageattr_test() -> i32 {
    let p = kthread_create(do_pageattr_test, core::ptr::null_mut(), "pageattr-test");
    if !IS_ERR(p) { wake_up_process(p); } else { WARN_ON(1); }
    0
}

// device_initcall(start_pageattr_test);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
