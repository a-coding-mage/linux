// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_int;

type PhysAddr = u64;
type U64 = u64;

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct EfiUnacceptedMemory {
    pub unit_size: U64,
    pub phys_base: PhysAddr,
    pub size: usize,
    pub bitmap: *mut usize,
}

#[repr(C)]
pub struct VmcoreCb {
    pub pfn_is_ram: Option<unsafe extern "C" fn(*mut VmcoreCb, usize) -> bool>,
}

extern "C" {
    fn efi_get_unaccepted_table() -> *mut EfiUnacceptedMemory;
    fn arch_accept_memory(start: usize, end: usize);
    fn pfn_is_unaccepted_memory(pfn: usize) -> bool;
    fn register_vmcore_cb(cb: *mut VmcoreCb);
    fn touch_softlockup_watchdog();
}

extern "C" {
    static mut unaccepted_memory_lock: usize;
}

#[repr(C)]
struct AcceptRange {
    list: ListHead,
    start: usize,
    end: usize,
}

static mut ACCEPTING_LIST: ListHead = ListHead {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

const PAGE_SIZE: u64 = 4096;
const BITS_PER_BYTE: usize = 8;

#[inline]
fn page_align(value: PhysAddr) -> PhysAddr {
    (value + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

#[inline]
fn page_align_down(value: PhysAddr) -> PhysAddr {
    value & !(PAGE_SIZE - 1)
}

#[inline]
fn div_round_up(value: u64, divisor: u64) -> usize {
    ((value + divisor - 1) / divisor) as usize
}

/*
 * accept_memory() -- Consult bitmap and accept the memory if needed.
 *
 * Only memory that is explicitly marked as unaccepted in the bitmap requires
 * an action. All the remaining memory is implicitly accepted and doesn't need
 * acceptance.
 *
 * No need to accept:
 *  - anything if the system has no unaccepted table;
 *  - memory that is below phys_base;
 *  - memory that is above the memory that addressable by the bitmap;
 */
pub unsafe extern "C" fn accept_memory(mut start: PhysAddr, size: usize) {
    let unaccepted = efi_get_unaccepted_table();
    if unaccepted.is_null() {
        return;
    }

    let mut end = page_align(start + size as u64);
    start = page_align_down(start);
    let unit_size = (*unaccepted).unit_size;

    if start < (*unaccepted).phys_base {
        start = (*unaccepted).phys_base;
    }
    if end < (*unaccepted).phys_base {
        return;
    }

    start -= (*unaccepted).phys_base;
    end -= (*unaccepted).phys_base;

    if end % unit_size == 0 {
        end += unit_size;
    }

    let bitmap_size = (*unaccepted).size as u64 * unit_size * BITS_PER_BYTE as u64;
    if end > bitmap_size {
        end = bitmap_size;
    }

    let range_start = (start / unit_size) as usize;
    let range_end = div_round_up(end, unit_size);
    let range = AcceptRange {
        list: ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
        start: range_start,
        end: range_end,
    };

    loop {
        // spin_lock_irqsave(&unaccepted_memory_lock, flags)
        let mut entry = (*core::ptr::addr_of_mut!(ACCEPTING_LIST)).next as *mut AcceptRange;
        let mut overlap = false;
        while !entry.is_null() && entry as *mut ListHead != core::ptr::addr_of_mut!(ACCEPTING_LIST) {
            if (*entry).end > range.start && (*entry).start < range.end {
                overlap = true;
                break;
            }
            entry = (*entry).list.next as *mut AcceptRange;
        }
        if overlap {
            // spin_unlock_irqrestore(&unaccepted_memory_lock, flags)
            continue;
        }
        break;
    }

    // list_add(&range.list, &accepting_list)
    let mut range = range;
    range.list.next = (*core::ptr::addr_of_mut!(ACCEPTING_LIST)).next;
    range.list.prev = core::ptr::addr_of_mut!(ACCEPTING_LIST);
    (*core::ptr::addr_of_mut!(ACCEPTING_LIST)).next = &mut range.list;

    let mut current = range.start;
    while current < range.end {
        let mut next = current + 1;
        while next < range.end && ((*unaccepted).bitmap.add(next / (usize::BITS as usize)) as *mut usize).read_volatile() & (1usize << (next % usize::BITS as usize)) == 0 {
            next += 1;
        }
        let len = next - current;
        if len != 0 {
            let phys_start = current as u64 * unit_size + (*unaccepted).phys_base;
            let phys_end = next as u64 * unit_size + (*unaccepted).phys_base;
            arch_accept_memory(phys_start as usize, phys_end as usize);
            for bit in current..next {
                let word = (*unaccepted).bitmap.add(bit / (usize::BITS as usize));
                let mask = 1usize << (bit % usize::BITS as usize);
                word.write_volatile(word.read_volatile() & !mask);
            }
        }
        current = next;
    }

    touch_softlockup_watchdog();
    // list_del(&range.list); spin_unlock_irqrestore(&unaccepted_memory_lock, flags)
}

pub unsafe extern "C" fn range_contains_unaccepted_memory(mut start: PhysAddr, size: usize) -> bool {
    let unaccepted = efi_get_unaccepted_table();
    if unaccepted.is_null() {
        return false;
    }

    let mut end = page_align(start + size as u64);
    start = page_align_down(start);
    let unit_size = (*unaccepted).unit_size;
    if start < (*unaccepted).phys_base {
        start = (*unaccepted).phys_base;
    }
    if end < (*unaccepted).phys_base {
        return false;
    }
    start -= (*unaccepted).phys_base;
    end -= (*unaccepted).phys_base;
    if end % unit_size == 0 {
        end += unit_size;
    }
    let bitmap_size = (*unaccepted).size as u64 * unit_size * BITS_PER_BYTE as u64;
    if end > bitmap_size {
        end = bitmap_size;
    }

    while start < end {
        let bit = (start / unit_size) as usize;
        let word = (*unaccepted).bitmap.add(bit / usize::BITS as usize).read_volatile();
        if word & (1usize << (bit % usize::BITS as usize)) != 0 {
            return true;
        }
        start += unit_size;
    }
    false
}

#[cfg(feature = "CONFIG_PROC_VMCORE")]
unsafe extern "C" fn unaccepted_memory_vmcore_pfn_is_ram(_cb: *mut VmcoreCb, pfn: usize) -> bool {
    !pfn_is_unaccepted_memory(pfn)
}

#[cfg(feature = "CONFIG_PROC_VMCORE")]
static mut VMCORE_CB: VmcoreCb = VmcoreCb {
    pfn_is_ram: Some(unaccepted_memory_vmcore_pfn_is_ram),
};

#[cfg(feature = "CONFIG_PROC_VMCORE")]
unsafe extern "C" fn unaccepted_memory_init_kdump() -> c_int {
    register_vmcore_cb(core::ptr::addr_of_mut!(VMCORE_CB));
    0
}

#[cfg(feature = "CONFIG_PROC_VMCORE")]
#[used]
static UNACCEPTED_MEMORY_INIT_KDUMP: unsafe extern "C" fn() -> c_int = unaccepted_memory_init_kdump;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
