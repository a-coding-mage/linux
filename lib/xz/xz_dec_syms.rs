// SPDX-License-Identifier: 0BSD

/*
 * XZ decoder module information
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 */

// The following symbols are defined by the XZ decoder implementation and are
// exported through the Linux kernel module interface in the C source.
// EXPORT_SYMBOL(xz_dec_init);
// EXPORT_SYMBOL(xz_dec_reset);
// EXPORT_SYMBOL(xz_dec_run);
// EXPORT_SYMBOL(xz_dec_end);

// CONFIG_XZ_DEC_MICROLZMA is a build-time kernel configuration option.
// Preserve the conditional exports when that configuration is enabled.
#[cfg(feature = "CONFIG_XZ_DEC_MICROLZMA")]
mod microlzma_exports {
    // EXPORT_SYMBOL(xz_dec_microlzma_alloc);
    // EXPORT_SYMBOL(xz_dec_microlzma_reset);
    // EXPORT_SYMBOL(xz_dec_microlzma_run);
    // EXPORT_SYMBOL(xz_dec_microlzma_end);
}

// Linux kernel module metadata corresponding to the C MODULE_* declarations.
// MODULE_DESCRIPTION("XZ decompressor");
// MODULE_VERSION("1.2");
// MODULE_AUTHOR("Lasse Collin <lasse.collin@tukaani.org> and Igor Pavlov");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
