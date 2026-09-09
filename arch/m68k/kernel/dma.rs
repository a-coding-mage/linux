/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn cache_push(addr: phys_addr_t, size: usize);
    fn cache_clear(addr: phys_addr_t, size: usize);
    fn page_to_phys(page: *mut page) -> phys_addr_t;
    fn pgprot_val(prot: *mut pgprot_t) -> *mut usize;
    fn pr_err_ratelimited(fmt: *const core::ffi::c_char, ...);
}

// Types and constants supplied by the surrounding kernel translation.
type phys_addr_t = usize;
type pgprot_t = usize;
struct page;
type dma_data_direction = u32;

const DMA_BIDIRECTIONAL: dma_data_direction = 0;
const DMA_TO_DEVICE: dma_data_direction = 1;
const DMA_FROM_DEVICE: dma_data_direction = 2;

const _PAGE_CACHE040: usize = 0;
const _PAGE_GLOBAL040: usize = 0;
const _PAGE_NOCACHE_S: usize = 0;
const _PAGE_NOCACHE030: usize = 0;

// The CONFIG_COLDFIRE build condition is supplied by the build configuration.
#[cfg(not(CONFIG_COLDFIRE))]
pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    cache_push(page_to_phys(page), size);
}

#[cfg(not(CONFIG_COLDFIRE))]
pub unsafe fn pgprot_dmacoherent(mut prot: pgprot_t) -> pgprot_t {
    // CPU_IS_040_OR_060 is a platform-provided build/runtime condition.
    if CPU_IS_040_OR_060 {
        *pgprot_val(&mut prot) &= !_PAGE_CACHE040;
        *pgprot_val(&mut prot) |= _PAGE_GLOBAL040 | _PAGE_NOCACHE_S;
    } else {
        *pgprot_val(&mut prot) |= _PAGE_NOCACHE030;
    }
    prot
}

extern "C" {
    static CPU_IS_040_OR_060: bool;
}

pub unsafe fn arch_sync_dma_for_device(
    handle: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    match dir {
        DMA_BIDIRECTIONAL | DMA_TO_DEVICE => {
            cache_push(handle, size);
        }
        DMA_FROM_DEVICE => {
            cache_clear(handle, size);
        }
        _ => {
            pr_err_ratelimited(
                b"dma_sync_single_for_device: unsupported dir %u\n\0".as_ptr()
                    as *const core::ffi::c_char,
                dir,
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
