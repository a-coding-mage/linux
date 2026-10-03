Rust x86 compressed-boot checkpoint
==================================

This batch fills production Rust gaps in the x86-64 gzip decompressor. The
original C implementations remain the reference; original C test sources and
assertions are unchanged. Remaining C production owners are unfinished migration
work, not a completed Rust kernel target.

Implemented source closure
--------------------------

``CONFIG_RUST_X86_COMPRESSED`` selects a freestanding Rust crate for ELF
extraction and relocation, boot-parameter sanitization, early console output,
allocation, physical/virtual KASLR and entropy, gzip/DEFLATE decoding, command-line
parsing, string/memory primitives and fatal diagnostics. The selected source
closure includes the original Rust siblings under ``arch/x86/boot`` and
``lib/zlib_inflate``; their missing or incorrect bodies were repaired against
the corresponding original C sources.

Native boot, ELF, firmware and zlib types/constants are generated from the real
configured headers. No abbreviated hand-written kernel structure substitutes
for an authoritative layout. The small runtime ``MAXMEM`` expression remains a
header boundary; it does not contain a KASLR implementation.

The gzip implementation preserves this Linux version's complete state machine,
Huffman tables, window and overlap behavior, unsigned widths, progress/error
returns and allocation/callback cleanup. Its optional gzip-header handling and
trailer behavior intentionally match the original implementation. All 544
fixed-table entries were compared with the original C definitions. DFLTCC is
explicitly unsupported in this Rust row. Non-PREBOOT library integration remains
a separate requirement; selecting the boot codec does not change the kernel's
zlib provider automatically.

Freestanding build requirements
-------------------------------

The decompressor cannot use the already-decompressed kernel's Rust runtime.
It builds its own core for the early-boot small-code-model, position-independent,
no-red-zone, no-SIMD ABI. Warning and checked-arithmetic policies are retained.
The selected target is x86-64/GZIP with Clang. Other architectures/codecs and
the GCC-to-bindgen build route are unfinished and are not enabled by this
selector. A required-owner configuration must reject a dropped selector rather
than accepting the original C provider as successful Rust ownership.

Private whole-program optimization is required to eliminate unused panic
location/format pointer metadata, which otherwise produces absolute data
relocations that the compressed image cannot process after moving. Overflow
and bounds checks still reach the same fatal Rust error/halt handler. This is
a boot-specific linkage requirement, not a change to kernel or test compiler
flags. The earlier non-LTO relocation failure is retained as negative evidence.
Final allocated relocations, public symbols, actual source ownership and the
absence of SIMD/unwind dependencies must be checked on the integrated output.

Validation and remaining gates
------------------------------

The preceding source commit ``8e8505218ff400546323d71109207c781769e54e`` built
``vmlinux``, modules, ``bzImage`` and ``usr_gen_init_cpio``, booted to Bash PID 1
and completed its declared original-C test profile. Its 15 requested KUnit
suites had 60 passing registered cases and four expected disabled benchmark
skips, with all 146 nested parameter invocations passing. The four legacy C
initcall test contracts also passed. Only the specifically mapped API calls
receive Rust-provider coverage; division tests bypassed Rust div64 and CRC32
remained C. That earlier result is not reassigned to this new boot batch.

This source checkpoint still requires the integrated kernel/image build,
unique final Rust ownership, an unchanged-command regression boot and a
separate default-KASLR-active boot with observed randomization. Original tests,
assertions and runtime bounds must stay unchanged. EFI/KHO/immovable-memory,
32-bit, alternative codec and failure-path coverage need their applicable
configurations and tests. A no-op build must preserve artifact bytes.

The original C crypto manager's deflate tests provide fixed decompression
vectors and compression round trips through single-entry scatterlists. They
can qualify the same decoder after genuine kernel-library Rust integration;
boot ownership alone does not enable that route. They are not evidence of
randomized fragmented-buffer/error coverage.

Remaining C boot owners include CPU feature discovery, platform/firmware and
page-table/startup support. Broader kernel, filesystem, networking and
configuration gaps remain tracked in the Lupos module issues, with device
drivers last. No whole-Linux or final userspace A/B completion is claimed.
