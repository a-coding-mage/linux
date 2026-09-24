.. SPDX-License-Identifier: GPL-2.0-only

C to Rust translation integration
================================

The migration objective is to replace the translated C sources throughout the
kernel, build tools, architecture support, and tests with working Rust, with
equivalent build, runtime, and test behavior. This objective is not yet complete.
The presence of an adjacent ``.rs`` file does not mean that its definitions are
complete or that Kbuild uses it.

Translated files retain their original ``SOURCE-COMMIT`` comments so the C
revision used for translation remains identifiable. Refactoring or integrating
a translation must not delete that provenance marker. Update it only when the
translation has actually been reconciled with a newer C source revision.
The provenance regression test records each migrated file's exact expected
revision in ``scripts/tests/translated_sources.txt``. An intentional revision
update must update that manifest as well; removing or replacing a marker
accidentally fails the test.

Current conservative inventory (2026-09-24): 55/64 canonical C-origin host tools
(85.94%, nine remain) and 38/526 translated lib units (7.22%, 488 remain).
Three lib host generators overlap the two inventories. These are integrated,
tested units within the documented configuration coverage, not a whole-kernel
completion percentage or proof of every exceptional path.

Host tools
----------

Migrated host tools default to Rust. Both implementations remain selectable::

    make O=/tmp/lupos-build HOST_TOOLS_LANG=rust defconfig
    make O=/tmp/lupos-build HOST_TOOLS_LANG=rust -j8

    make O=/tmp/lupos-build HOST_TOOLS_LANG=c defconfig
    make O=/tmp/lupos-build HOST_TOOLS_LANG=c -j8

Pass the same choice to each build invocation. Kbuild records the selected
compiler command and rebuilds affected tools when switching languages, so a
separate output directory is optional. The C branches retain their original
source files, object lists, flags and generated-header dependencies. This
selection applies to migrated host tools, not the target kernel's
``CONFIG_RUST`` setting or tools which were originally Rust-only.
``rust-host-tools`` and ``rust-host-tests`` require ``HOST_TOOLS_LANG=rust``;
the retained C implementations are available through normal Kbuild targets.

The following normal Kbuild targets have Rust implementations:

* ``scripts/basic/fixdep``
* ``scripts/kallsyms``
* ``scripts/tracepoint-update``
* ``scripts/gen_packed_field_checks``
* ``scripts/asn1_compiler``
* ``scripts/unifdef``
* ``scripts/sorttable``
* ``scripts/recordmcount``
* ``scripts/insert-sys-cert``
* ``scripts/sign-file``
* ``certs/extract-cert``
* ``scripts/mod/mk_elfconfig``
* ``scripts/mod/modpost``
* ``scripts/selinux/mdp/mdp``
* ``security/selinux/genheaders``
* ``scripts/kconfig/conf``
* ``scripts/genksyms/genksyms``
* ``scripts/gendwarfksyms/gendwarfksyms``
* ``scripts/dtc/dtc``
* ``scripts/dtc/fdtoverlay``
* ``scripts/dtc/fdtget`` and ``scripts/dtc/fdtput`` (explicit host targets)
* ``scripts/ipe/polgen/polgen``
* ``arch/x86/tools/vdso2c``
* ``arch/x86/tools/relocs``
* ``arch/x86/tools/insn_decoder_test`` and ``arch/x86/tools/insn_sanity``
* ``arch/x86/boot/compressed/mkpiggy``
* ``arch/x86/boot/mkcpustr``
* ``arch/powerpc/boot/addnote``
* ``arch/powerpc/boot/hack-coff``
* ``arch/powerpc/boot/mktree``
* ``arch/arm/vdso/vdsomunge`` (also used by ARM64's 32-bit compatibility vDSO)
* ``arch/arm64/kernel/pi/relacheck``
* ``arch/arm64/kvm/hyp/nvhe/gen-hyprel``
* ``arch/alpha/boot/tools/mkbb``
* ``arch/alpha/boot/tools/objstrip``
* ``arch/mips/tools/elf-entry``
* ``arch/mips/tools/loongson3-llsc-check``
* ``arch/mips/boot/elf2ecoff`` (also used by compressed boot)
* ``arch/mips/boot/tools/relocs``
* ``arch/mips/boot/compressed/calc_vmlinuz_load_addr``
* ``arch/mips/vdso/genvdso``
* ``arch/s390/tools/gen_facilities``
* ``arch/s390/tools/gen_opcode_table``
* ``arch/s390/tools/relocs``
* ``arch/sparc/boot/piggyback``
* ``arch/sparc/vdso/vdso2c``
* ``usr/gen_init_cpio``
* ``lib/raid/raid6/mktables``
* ``lib/crc/gen_crc32table`` and ``lib/crc/gen_crc64table``
* ``fs/unicode/mkutf8data``

These tools use the Rust standard library without third-party Rust crates or
unresolved declarations for functions that were C macros or inline helpers.
The signing and certificate-extraction tools additionally link the existing
OpenSSL ``libcrypto`` host library through an audited, opaque-handle ownership
boundary. Their common Rust module replaces the translated C stdio and macro
declarations. A small Python helper probes the installed OpenSSL headers using
the host C compiler to select the correct library ABI; it compiles no C shim.
The host OpenSSL development headers/library and Python 3 are therefore needed
when building these tools, including the combined ``rust-host-tools`` target.
The DWARF symbol-version generator links the existing elfutils ``libdw``
library, which also requires libelf and zlib. The combined host-tools target
therefore needs these host libraries as well. Its C-reference tests additionally
need the elfutils development headers; no C shim or generated Rust binding is
compiled for the Rust implementation.
The tracepoint checker and table sorter import ``scripts/elf-parse.rs`` as a
checked Rust module. Kbuild tracks that module through rustc's dependency output.
The ASN.1 compiler imports translated tag definitions, and the SELinux policy
generator imports shared immutable class, permission, SID, and capability
tables. Its filesystem configuration flags preserve the distinction between
built-in and modular features.

The module postprocessor uses owned module/export records and the shared checked
ELF reader. Its source-version digest, all 57 device-table alias families,
namespaces, symbol versions, export trimming, section-reference diagnostics,
and generated files are tested against C, including ELF32/ELF64 and both byte
orders. Normal Kbuild embeds the target compiler's generated device-table
offsets; the target C structure definitions and their layout emitter remain
part of this mixed build. The no-config ``rust-host-tools`` executable does not
embed target-specific offsets. Set ``MODPOST_DEVICETABLE_OFFSETS`` to an existing
generated ``scripts/mod/devicetable-offsets.h`` when using that executable on
device tables; missing metadata is an error, not silently omitted aliases.
Command-line paths, symbol dumps, and ELF text metadata currently require UTF-8;
source-version dependency filenames are byte-preserving. Malformed ELF extents,
links, symbol indices, and device-table data are checked before access.
To compare complete generated files against C using an existing kernel and
its modules, including saved CRCs when ``CONFIG_MODVERSIONS=y``::

    MODPOST_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_modpost.py

The IPE boot-policy generator preserves every input byte and the original C
string escaping, including binary policy data and non-UTF-8 pathnames. Tests
compile the generated C source and verify its embedded bytes. Buffered write
failures are reported instead of silently producing incomplete policy source.

The Alpha boot helpers decode ELF and ECOFF files and construct SRM boot blocks
using explicit target-endian fields. They preserve disk labels, checksums,
entry-point offsets, BSS padding, primary-loader block counts, and non-UTF-8
filenames. Differential tests cover the original C output and diagnostics,
including sparse primary images larger than 32 bits. Malformed header extents
and arithmetic are checked, and partial writes advance by the bytes written.
These checks do not substitute for an Alpha target-kernel build or boot.

The RAID6 table generator preserves the original generated ``tables.c`` byte
for byte, including all six finite-field tables. The table calculations are
safe Rust; a narrow C-stdio boundary and C-compatible entry point retain the
original ignored-output-error and inherited-SIGPIPE behavior. The existing C
generator remains selectable. Eight focused tests pass with minimum Rust 1.85
and strict warnings against both GCC and Clang references, covering all 65,536
multiplication entries, ignored arguments, failed output streams, cold parallel
C/Rust/C and Rust/C/Rust builds, source dependencies, no-ops and isolation from
an enclosing make environment. The expanded 49-tool aggregate build and its
dependency checks pass, as do all three language/provenance invariants. This
adds a verified host generator; the native RAID6 algorithms are unchanged.

The CRC32/CRC32C and CRC64 table generators are also selected through
``HOST_TOOLS_LANG``. They construct local Rust arrays, import the translated
CRC32 polynomial header and emit byte-identical headers, including CRC64 row
commas missing from the initial translation. Their narrow stdio boundary
preserves ignored arguments and output errors and inherited SIGPIPE behavior.
Four focused test groups pass against both GCC and Clang using minimum Rust
1.85 and the actual strict host lints. They cover original generated bytes,
compile-success negative controls for output/argv regressions, cold parallel
C/Rust/C and Rust/C/Rust builds, source/header dependencies, no-ops and a real
parallel parent-make environment. The combined 51-tool build/dependency tests
and all language/provenance invariants pass. Native CRC algorithms and the
original C generators are unchanged.

The Unicode generator now implements the complete UCD parsing, normalization,
trie construction and verification pipeline in Rust; it does not substitute
the shipped table. With the seven official UCD 12.1 inputs, its 331,286-byte
output matches both original C and the shipped table, and all 18,820 original
normalization cases pass. The existing source provenance and SGI license are
retained. Original C remains selectable, including normal shipped-table builds
and explicit regeneration. Narrow libc lexical/stdio calls preserve raw-byte
input, signed-age diagnostics, nonseekable-input behavior and stream errors.

Nine integrated groups pass without skips using minimum Rust 1.85 and actual
strict host flags; independent final GCC and Clang runs also pass all nine.
They cover exact output/diagnostics, malformed-input and normalization negative
controls, original stream failures, cold parallel C/Rust/C and Rust/C/Rust
builds, all imported dependencies and no-ops. ``UNICODE_UCD_DIR`` supplies the
official corpus and its checked provenance; an explicitly invalid corpus is
an error, not a skip. The expanded 52-tool aggregate build/dependency gate
passes. This validates the host generator, not a Rust Unicode target driver
or execution of the complete generator on a 32-bit host.

The SELinux header generator reuses the translated class/permission and initial
SID tables. Original ordering, permission bits and SID holes are unchanged.
Its fallible uppercase allocation and narrow libc boundary preserve original
allocation-failure status, raw paths, inherited signals, each output-format
call and close-error behavior. In particular, the finite-file-size regression
at 65,535 bytes detects a fragmented writer that otherwise emits identical
normal output. Sixteen integrated groups pass without skips; independent GCC
and Clang runs also pass all sixteen, including allocation-failure controls,
210 output-size-limit combinations and actual C/Rust build switching and
dependencies. The expanded 53-tool aggregate and three language/provenance
invariants pass. The maintained ASCII-source/ASCII-compatible-encoding boundary
and untested libc/startup-exhaustion cases are documented in
``scripts/tests/selinux_genheaders.md``; arbitrary frontend encodings are not
claimed equivalent.

The Speakup ``makemapdata`` and ``genmap`` translations now share the repaired
translated utility module and generate the real keyboard maps through Kbuild.
The selected host C frontend validates the retained originals and produces
initializer data only: original translation-unit preprocessing is preserved,
function bodies are removed before code generation, and an independent ELF
guard rejects executable content. No original C algorithm is linked or run
inside either Rust tool. Unsupported declaration syntax, object/relocation
formats, LTO and non-UTF-8 frontend encodings explicitly require the retained
C selection; this is not an arbitrary-C parser or cross-host execution claim.

Ten integrated groups pass with GCC and Clang, without skips, including exact
maps/diagnostics, raw strings/paths, output failures and inherited SIGPIPE,
macro expansion context, attributed declarations, executable-leakage controls,
strict minimum Rust 1.85 and real C/Rust switching/dependencies/no-ops. The
expanded 55-tool aggregate passes, including imported utility/frontend/input
dependencies and unchanged generated maps on a no-op build.

Build the migrated tools without creating a kernel configuration::

    make O=/tmp/lupos-build rust-host-tools

Build and run the regression suite::

    make O=/tmp/lupos-build rust-host-tests

The suite compiles the unchanged C implementations as test references and
compares their output, diagnostics, and exit codes against Rust. It also checks
Kbuild source selection, incremental rebuilds, and imported-module
dependencies. This requires a host C compiler/linker, ``HOSTRUSTC``, and
Python 3. Kconfig, genksyms, and DTC differential tests additionally require Flex
and Bison to build the original C references; the Rust tools themselves need
neither. Tests use temporary directories. Normal builds use the Rust tools;
the C implementations remain selectable build implementations and behavioral
references.

CORDIC, reciprocal and decoder native-ABI tests also use ``bindgen`` to generate
the actual structure and enum types from the original C headers. Their KCFI
comparisons require Clang. With a non-default installation, pass
``BINDGEN=/path/to/bindgen`` as a ``make`` command-line argument; a shell-only
environment assignment is overridden by Kbuild's normal tool defaults.

External module builds conservatively rebuild fixdep-tracked targets when the
kernel's ``include/config/auto.conf`` changes. External builds run outside the
kernel output directory, so their saved relative configuration wildcards alone
cannot detect those changes. The shared Kbuild dependency also repairs existing
``.cmd`` files with either C or Rust host tools, preventing stale module version
magic after configuration switches. Unchanged builds remain no-ops, and the
configuration file is not added to archive or module-order inputs. Focused
regressions are in ``test_external_module_config.py``.

The device-tree compiler uses owned tree nodes, ordered properties and labels,
byte-valued data with reference markers, and checked blob input/output. Its full
source grammar includes overlays, includes, binary includes, deletion, labels,
and unsigned integer expressions. All registered checks, phandle allocation,
symbol/fixup generation, filesystem input, source annotations, blob versions
1/2/3/16/17, DTS, and assembly output are tested against C. As with the existing
Kbuild configuration, YAML output is not enabled. Source parsing and tree
serialization use explicit stacks, including deep nodes and expressions.
The parser no longer inherits Bison's fixed parser-stack allocation limit;
buffered output failures and malformed blob bounds are reported safely.
Differential fixtures include real ARM, ARM64, RISC-V, and PowerPC boards and
the kernel's OF unit-test overlays.

The device-tree overlay, query, and editing tools share safe byte-slice libfdt
modules. Header/tag traversal, old-format property alignment, aliases, phandles,
reservation maps, string lists, in-place mutation, sequential construction,
packing/reordering, and overlay fixups and symbol propagation are compared
against C. Tests compare entire buffers, including both damaged blobs after
an overlay failure, and apply the kernel's OF overlay fixtures and chained
compiled DTS overlays. Names, values, and filesystem paths retain their bytes.
Overlay traversal uses explicit stacks. Malformed local-fixup offsets and
sequential-write headers are bounds checked, and truncated input is detected
using actual bytes read rather than allocation capacity. The legacy unbuilt
query/edit applets had undefined behavior for 16-bit writes, invalid integers,
and raw formatting: Rust writes complete big-endian halfwords, diagnoses invalid
numbers, and implements raw byte values without invalid format strings.

The tracepoint checker additionally rejects truncated ELF data and checks
multiple files without retaining section pointers from a previous file.
Its cross-target cases cover ELF32, ELF64, both byte orders, and extended
section numbering.

The table sorter supports all exception-table architectures from the C tool,
both ELF classes and byte orders, extended symbol indices, x86 and LoongArch
ORC layouts, and address- or RELA-based mcount tables. Kbuild passes the ORC
and mcount configuration to rustc. Differential tests compare the complete
modified ELF files, including function-list filtering and relative fixups.
The Rust sorter validates all edits before writing, processes the final symbol
table entry, and keeps architecture state separate for each input file.

The ftrace call-site recorder implements the original architecture-specific
relocation filters, section-base selection, MIPS call pairs, and x86, ARM,
Thumb, and AArch64 instruction replacements. Tests compare output files for
every supported architecture, both ELF classes and byte orders, extended
indices, and real ``-pg`` compiler objects that are linked after rewriting.
It supports byte-preserving paths and section names. ELF accesses are checked,
failed edits are not committed, and architecture state is reset for each file.
Big-endian extended indices are decoded correctly instead of following the
original unchecked host-endian reads. New-section alignment bytes are zeroed;
the C implementation leaves those padding bytes uninitialized. Mixed REL/RELA
call-site tables are rejected instead of emitting inconsistent entry sizes.
An optional check recompiles a saved kernel compilation unit with profiling
into a temporary directory, leaving the kernel output tree unchanged::

    RECORDMCOUNT_KERNEL_BUILD=/tmp/lupos-build \
    RECORDMCOUNT_KERNEL_OBJECT=lib/test_hexdump.o \
        python3 -m unittest discover -s scripts/tests -p test_recordmcount.py

The x86 vDSO image generator uses the same checked ELF reader for both 32-bit
and 64-bit inputs. Its generated C image data and metadata match the original
tool; malformed input is rejected before opening the output, and write errors
are checked. Generated target-kernel C image data remains part of the mixed
build until those consumers are migrated.

The x86 relocation extractor shares the checked ELF reader and preserves
kernel and real-mode symbol classifications, relocation filtering, duplicate
offsets, sorted binary tables, assembler output, and the absolute-symbol and
relocation-inspection modes. Tests cover both ELF classes, extended indices,
all relocation types, and over 100,000 randomized relocations per class.
Additional comparisons against retained kernel relocations can be run with::

    RELOCS_TEST_VMLINUX=/tmp/lupos-build/vmlinux.unstripped \
    RELOCS_TEST_REALMODE=/tmp/lupos-build/arch/x86/realmode/rm/realmode.elf \
        python3 -m unittest discover -s scripts/tests -p test_relocs.py

Malformed section links, string references, and table strides are rejected
with checked errors rather than accessing memory outside the ELF image.

The compressed x86 image wrapper is also generated by Rust, preserving the
embedded file name, compressed length, and little-endian uncompressed-size
footer. Its tests assemble the wrapper and check the embedded bytes, including
non-UTF-8 file names and sparse input files larger than 4 GiB.

The extra-certificate insertion tool shares the checked ELF reader and supports
both symbol tables and a supplied ``System.map``. It preserves native-class
and native-endian restrictions, original inode and permissions, replacement
accounting, reserve zeroing, and no-op behavior. Tests compare complete modified
images with C, including real linked ELF input, and check that malformed images
are rejected before writing. Binary certificate comparison and exact map-symbol
matching intentionally fix the C tool's NUL-terminated and substring comparisons.
An optional comparison modifies only temporary copies of a real kernel and
its stripped image (with ``CONFIG_SYSTEM_EXTRA_CERTIFICATE=y``)::

    INSERT_CERT_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_insert_sys_cert.py

Certificate extraction preserves PEM bundle ordering, DER output, verbose
subject descriptions, empty key lists, and byte-preserving paths. Tests compare
outputs and errors against C and check partial-output behavior after a malformed
later certificate. Explicit BIO flushing reports buffered output failures that
the original ignored. Signing tests compare deterministic RSA CMS signatures
and module trailers, independently verify RSA and EC signatures, and cover
encrypted keys, raw signatures, detached signatures, and certificate formats.
The shared wrapper implements provider and legacy-engine PKCS#11 paths; real
hardware-token success and older OpenSSL library versions still require
environment-specific validation. The installed OpenSSL 3 library and missing
provider failures are covered by the automated tests.

The x86 CPU-string generator imports the translated feature-bit definitions
and immutable name table from ``cpufeatures_header.rs``. Tests verify every
translated feature bit, compare the generated header against C for both
kernel widths, and compile the header under multiple required-feature masks.

The x86 instruction-test tools share one allocation-free, core-only Rust decoder
and immutable attribute tables with the translated architecture sources. Their
tools-side modules import that canonical implementation. Byte slices and checked
cursors replace instruction pointers; C union aliases share one stored field.
Tests compare every intermediate field and failure state with C, including
legacy, REX/REX2, VEX/EVEX/XOP, addressing and immediate forms, truncation, retries,
and manually selected operand widths. Real kernel disassembly and seeded random
streams also match. Target-kernel selection is independent: C remains the
default, with the opt-in ``CONFIG_RUST_X86_INSN`` integration described below.

``arch/x86/tools/gen_inat_tables.py`` emits an ``apply_patch`` patch for updating
the checked-in Rust tables from the original opcode map. Its ``--check`` mode
verifies reproducibility. A compile-time map fingerprint rejects stale tables,
and rustc tracks the map as a dependency; normal Rust builds run no C table
generator. The original AWK/C path remains available with ``HOST_TOOLS_LANG=c``.
CLI tests preserve option parsing, field dumps, raw-byte input, and glibc's
seeded random sequence. Excess disassembly bytes are rejected instead of
overflowing a stack buffer, the first short sanity-input tail is initialized
instead of reading uninitialized bytes, and buffered output errors are reported.
After a configured x86 kernel build, the normal decoder post-test can be enabled
with ``CONFIG_X86_DECODER_SELFTEST=y``. An optional C/Rust comparison of its
complete disassembly stream is available with::

    INSN_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_insn_decoder.py

The PowerPC Open Firmware helpers preserve ELF32/ELF64 notes in both byte
orders, XCOFF optional-header section indices, and tree-boot image headers,
checksums, and sector padding. Differential tests compare complete modified
images and errors with C, including non-UTF-8 paths and 65,535 XCOFF sections.
The existing ELF32 RPA note-size field, host-endian tree-image checksum, and
retained final-sector bytes are deliberately preserved. Malformed overflowing
ELF offsets and oversized XCOFF optional headers are rejected with checked
errors instead of accessing memory outside their buffers. These host tools
are validated through Kbuild; a PowerPC target-kernel boot is not yet validated.

The ARM vDSO helper preserves EABI validation, soft-float flag clearing,
output permissions, and failed-output cleanup. The ARM64 early-relocation
checker uses the shared checked ELF reader, preserves data-section filtering,
converts opted-in ABS64 entries to PREL64, and removes rejected outputs.
Tests compare complete images, diagnostics, and link behavior with C, including
100,001 randomized relocations per byte order. When Clang and LLD are available,
the suite also builds real ARM and AArch64 fixtures in both byte orders.
Truncated headers, invalid encodings, and out-of-bounds section references are
rejected safely; extended ELF section numbering is supported. Kbuild tests
exercise both the ARM helper rule and ARM64's borrowed helper path. ARM and
ARM64 target-kernel boots are not yet validated.

The ARM64 hypervisor relocation generator uses the same checked ELF reader,
with byte order selected from the target kernel configuration rather than the
host. It preserves generated assembly, section filtering, relocation ordering,
and partial output on errors. Tests cover every accepted relocation type,
rejected types, both byte orders, raw-byte paths and section names, and real
AArch64 assembler objects. Malformed extents and indices are rejected safely;
extended section numbering is supported. Assertion diagnostics retain the
original decoded values and source locations but use portable expressions
instead of host-libc macro expansions. Delayed output errors are reported.
Set ``HYPREL_KERNEL_BUILD`` to an ARM64 build containing
``arch/arm64/kvm/hyp/nvhe/kvm_nvhe.tmp.o`` to compare an actual hypervisor object
against the unchanged C tool and Kbuild's saved ``hyp-reloc.S`` output.
The normal little-endian ARM64 hypervisor-object build is verified. This tree
marks big-endian ARM64 kernels ``BROKEN``; big-endian coverage is limited to
tool fixtures and explicit host-rule configuration, not a target-kernel build.

The MIPS boot helpers preserve ELF32/ELF64 entry-address sign extension in both
byte orders, compressed-load address rounding and hexadecimal argument parsing,
and complete ELF32-to-ECOFF conversion. ECOFF coverage includes all header fields,
the optional six-section form, segment ordering and gaps, PROM padding, retained
existing-file tails and permissions, and partial output on errors. Tests compare
randomized images and real big- and little-endian MIPS compiler/linker output.
Malformed ELF magic, class, and encoding are rejected before opening an ECOFF
output instead of interpreting arbitrary bytes as an ELF32 header. Raw-byte
paths and checked output errors are supported. Normal and compressed-boot Kbuild
host rules are verified; a MIPS target-kernel boot is not yet validated.

The Loongson LL/SC checker preserves executable-section filtering, instruction
classification, synchronization checks, branch displacements, the original
loop-boundary rule, and diagnostic ordering. Its byte-oriented ELF access also
handles empty sections and extended numbering safely and checks branch targets
before reading. Tests compare all opcode families, randomized streams, large
sections, and actual MIPS64 little-endian assembler objects with C; malformed
headers and output failures are reported without unchecked memory access.

The MIPS vDSO generator repairs ABI sections in both debug and stripped ELF
images and emits byte-identical C image descriptions for O32, N32, and N64 in
both byte orders. It preserves in-place inode/permission behavior, ordered
partial repairs on semantic errors, and required-symbol output cleanup.
Checked shared ELF access replaces native-layout casts and unresolved helper
declarations. Extended section numbering and large symbol tables are supported;
input/output inode aliases and mismatched image formats are rejected before
modification, and output failures are reported. Tests compile and execute the
generated C to compare its embedded bytes. Normal 32-bit little-endian Malta
and 64-bit big-endian Malta vDSO builds, including both compatibility ABIs,
are verified through target compilation. To compare their raw inputs, repaired
images and generated source against C, set ``MIPS_VDSO_KERNEL_BUILD`` to the
corresponding build directory when running ``test_mips_genvdso.py``.

The MIPS relocation processor uses checked ELF32/ELF64 access in both byte
orders, including MIPS64's mixed-width relocation records. It preserves
relocation order, symbol and section filtering, offset encoding, all output
modes, and the original in-place section-size updates, including their effect
with ``--text`` and ``--bin``. Differential tests compare complete modified
images, all relocation types and flag combinations, partial table records,
large relocation streams, and real compiler/linker output. Invalid extents,
links, indices and overlapping writes are rejected safely. A relocation table
overflow is diagnosed before modifying the file, unlike C's destructive
write-before-check behavior; capacity uses an unsigned byte count instead of
C's signed ``int`` truncation, and output failures are also reported. The
original C implementation remains available with ``HOST_TOOLS_LANG=c``.
The normal 32-bit little-endian and 64-bit big-endian Malta kernel builds are
verified through final linking and relocation insertion. All 16 command-line
flag combinations also match C on copies of both linked kernels. Set
``MIPS_RELOCS_KERNEL_BUILD`` to a
build directory retaining ``.tmp_vmlinux2`` (or ``MIPS_RELOCS_VMLINUX`` to an
unprocessed linked image) to run that comparison. MIPS kernel boot validation
is still outstanding.

The s390 facility and disassembler generators preserve all three facility masks,
configuration-dependent architecture bits, instruction formats, long names,
opcode grouping, and table ordering. Differential tests cover all 64 feature
combinations and the complete checked-in opcode table, and compile the generated
disassembler header as C. Normal ``kapi`` rules generate byte-identical headers,
rebuild on configuration changes, and preserve unchanged header timestamps.
Opcode input is byte-preserving, including ASCII case conversion and whitespace.
Overlong fields and incomplete multi-byte opcode fragments are rejected instead
of overflowing C's fixed arrays or reading uninitialized data; input/output
errors are reported. An s390 target-kernel boot is not yet validated.

The s390 relocation extractor uses checked ELF64 big-endian access, including
extended section and symbol indices. It preserves allocated-section filtering,
all supported relocation types, KCFI absolute-symbol exceptions, sorted 32-bit
offsets, duplicates, ignored entry-size fields, and original diagnostics.
Differential tests cover over 100,000 relocations, more than 65,536 sections,
randomized images, and a real s390 compiler/linker executable. Invalid table
links, symbol indices, and unterminated strings are rejected safely; delayed
output failures are reported instead of being ignored. The retained C tool
remains selectable with ``HOST_TOOLS_LANG=c``.

The SPARC boot helper preserves a.out header updates, 32-bit and 64-bit page
alignment, fixed-column System.map parsing, and in-place ramdisk appending,
including existing trailing bytes and permissions. It rejects input inode
aliases that would endlessly append a file to itself and reports ramdisk read
errors. The SPARC vDSO converter preserves big-endian ELF32/ELF64 load and dynamic
table validation and byte-identical raw or generated-C outputs. Checked offsets
replace unchecked pointers; invalid input preserves an existing destination,
and buffered output failures are reported. Tests cover randomized images, raw
byte filenames, a real linked SPARC64 vDSO, and compilation and execution of
generated C to verify embedded bytes and padding. Both host rules pass C/Rust/C
selection tests. A full SPARC target build and boot are not yet validated: the
available Clang rejects the tree's existing ``-Wa,--undeclared-regs`` option
before compiling the kernel vDSO inputs.

The line-oriented Kconfig front end uses owned Rust modules for preprocessing,
lexing, parsing, expression evaluation, menus, symbols, and configuration I/O.
It implements the existing configuration modes, preserves choice priority,
tristate dependencies, select/imply behavior, configuration backups, and
generated-header and dependency-file ordering. Full-tree x86 tests compare
complete output files and diagnostics with the original C implementation.
The existing Kconfig tests can also be run with::

    make O=/tmp/lupos-build testconfig

That target requires Python's ``pytest`` package. The interactive graphical
and terminal front ends (``menuconfig``, ``nconfig``, ``xconfig``, and
``gconfig``) still use the C Kconfig core and are not yet migrated.
The Rust Kconfig model currently requires UTF-8 source and configuration
strings. Non-UTF-8 configuration input is rejected without replacing the
existing file; byte-preserving support remains outstanding. Output write
errors and failed configuration-backup renames are also reported instead of
being silently ignored.

The symbol-version generator uses the complete declaration grammar with
checked-in LR tables, stable token-arena indices, and owned symbol definitions.
It preserves typedef and compound expansion, enum values, CRCs, source-file
boundaries, reference modversions, type dumps, and parser/lexer diagnostics.
Tests compare real exported kernel declarations, generated declarations,
malformed-input recovery, deep type graphs, and binary input bytes with C.
The table-maintenance helper ``scripts/genksyms/gen_parser_tables.py`` emits
an ``apply_patch`` patch; it is not needed during normal builds. Grammar changes
require regenerating the tables and reviewing the corresponding Rust actions.
Malformed reference-file read failures are reported instead of reproducing
the C implementation's nonterminating read-error loop.
After completing a kernel build with ``CONFIG_MODVERSIONS=y``, the optional
integration check compares saved compilation-unit CRCs and ``Module.symvers``
against the source-built C reference::

    GENKSYMS_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_genksyms.py

The DWARF symbol-version generator uses owned symbol and type records,
byte-preserving names, checked ELF metadata, and explicit traversal stacks.
Its type expansion, CRC calculation, symbol aliases, export-pointer fallback,
symtypes output, and kABI stability rules are compared with the original C
implementation. Fixtures include GCC, Clang and Rust debugging information,
DWARF versions 2 through 5, both ELF classes and byte orders, compressed and
separate debug information, and linked compilation units. Anonymous type
cycles and malformed ELF metadata are rejected without unchecked accesses.
Buffered output failures are reported instead of silently succeeding.

Only the public elfutils DWARF APIs cross an unsafe boundary. Library handles,
file descriptors, DIEs, and borrowed strings have checked ownership and
lifetimes. Regression tests verify the C ABI layouts and function signatures
against installed headers, reject escaping or cross-thread handles at compile
time, and check descriptor cleanup. These checks do not replace the external
library's own input validation. Run this family's focused tests with::

    python3 -m unittest discover -s scripts/tests -p 'test_gendwarf*.py'

For development headers extracted outside the system, set ``GENDWARF_PREFIX``
to their ``usr`` directory. ``GENDWARF_CFLAGS`` and ``GENDWARF_LIBS`` can override
the differential tests' compiler and linker flags. Tests do not download or
install dependencies, and explicitly skip C comparisons when the required
development headers are absent.

An x86-64 kernel with ``CONFIG_GENDWARFKSYMS=y``, ``CONFIG_MODVERSIONS=y``
and DWARF 5 debugging information is verified through the normal build,
emulated boot, and module loading. Its saved compilation-unit CRCs, symtypes,
``Module.symvers``, linked kernel images, and an external module are compared
against C. For a completed build made with ``KBUILD_SYMTYPES=1``, run the
read-only integration audit with::

    python3 scripts/tests/check_gendwarf_kernel.py /tmp/lupos-build \
        --module /tmp/lupos-external-module

The optional ``--module`` argument may be repeated or omitted. The checker
writes its reference tools and comparison outputs only in temporary directories.
Native Rust compilation units use their actual Kbuild export ordering and
filtering, including the C-built Rust helpers. A native Rust kernel with the
translated hexadecimal helpers has exact per-unit C/Rust/Kbuild parity for
423 compilation units and 6,664 CRC records, including their symtypes files.
The complete ``vmlinux.o`` and final ``vmlinux`` also match C for all 6,664
exports, diagnostics and symtypes.

The combined native build with Rust integer math, BCD, character classification,
hexadecimal helpers and x86 instruction decoding, with the four libraries'
export metadata emitted directly from Rust, passes the same read-only audit:
438 compilation units and 6,834 CRC records match C/Rust/Kbuild, including
every symtypes file. Both complete kernel images match the C version tool for
all 6,834 exported symbols, diagnostics and symtypes.

Adding the Rust GCD/LCM owner and enabling jump labels retains this parity:
438 compilation units and 6,854 CRC records, including every symtypes file,
match between C/Rust version tools and Kbuild. The complete ``vmlinux.o`` and
final ``vmlinux`` also match for all 6,854 exported symbols.

With rational approximation and its original KUnit suite added, plus common
clock support and module unloading enabled for the built-in/module test matrix,
the final native build has exact per-unit parity for 453 compilation units and
7,019 CRC records and symtypes. The complete ``vmlinux.o`` and final ``vmlinux``
also match for all 7,019 exports. Its Rust metadata bridge matches the original
C bridge for all 3,156 records.

With reciprocal division added, all 453 compilation units, 7,023 CRC records
and symtypes files match between C/Rust version tools and Kbuild. The complete
``vmlinux.o`` and final ``vmlinux`` also match for all 7,023 exports, and the
Rust metadata bridge matches all 3,160 original C records.

With integer logarithms and their original KUnit suite added, all 453
compilation units, 7,026 CRC records and symtypes files match between C/Rust
version tools and Kbuild. Both complete kernel images match all 7,026 exports;
the Rust metadata bridge matches all 3,163 original C records.

With generic wide division and its translated self-test modules added, the
native x86-64 build matches for all 453 compilation units, 7,065 CRC records
and symtypes files, and all 7,065 exports in both complete kernel images.
Its Rust metadata bridge matches 3,202 original C records. The little-endian
ARM64 build independently matches for 385 compilation units, 6,742 CRC/symtypes
records, all 6,742 exports in both images and 3,083 bridge records. ARM64's
bridge comparison uses an explicit private reference compiled from unchanged
``rust/exports.c`` with that completed build's actual export lists and C flags;
automatic reference compilation in the bridge checker remains x86-only.

With CORDIC added, x86-64 matches for all 454 compilation units, 7,069
CRC/symtypes records, 7,069 exports in each complete kernel image and 3,205
original C bridge records. ARM64 matches for 386 units, 6,746 CRC/symtypes
records, 6,746 exports in each image and 3,086 bridge records. Its original
C reference is freshly compiled from the updated generated lists. These are
read-only audits of the completed native build outputs.

The separate strict normalized-KCFI configurations used for the seven native
signature corrections also pass these audits. On x86-64, 454 units and 7,064
CRC/symtypes records match C/Rust/Kbuild; both linked images match all 7,064
exports, and the bridge matches 3,200 original C records. ARM64 matches 388
units and 6,743 per-unit records, 6,742 exports in each linked image and 3,081
bridge records, using a fresh original reference with the final native flags.

Native Rust library export bridge
---------------------------------

``CONFIG_RUST_NATIVE_EXPORTS=y`` selects ``rust/exports_rust.o`` in place of
``rust/exports.o``. It requires ``CONFIG_RUST``, defaults to disabled, and is
independent of ``HOST_TOOLS_LANG``. The original ``rust/exports.c`` and its
generated C-header build path remain selectable without source changes.

The native bridge imports the translated export definitions and generates
metadata directly from the same ordered ``nm``/AWK symbol lists as C. All
exports remain GPL-only, including the existing helper and optional build-assert
guards. Mangled linker names are referenced without fictitious ``extern``
types: the defining objects still provide the real function/data kinds, DWARF
and version CRCs. The bridge deliberately generates no second set of versions.
The optional ``rust_build_error`` retains its existing unversioned policy.

In the initial combined native x86-64 build, all 3,152 bridge records match the
original C bridge exactly, and the complete ``Module.symvers`` is byte-identical.
Switching this metadata-only option therefore does not require rebuilding
modules. A previously built independent Rust consumer loads and passes its
8,194-input ctype checks after switching to the native bridge; a new consumer
built with that bridge also loads unchanged after restoring the C bridge.
With ``CONFIG_RUST_BUILD_ASSERT_ALLOW=y``, both choices produce the same 3,153
records and identical versions, including the original build-error warning
and zero CRC. Repeating the final Rust build leaves its outputs unchanged.

For a completed kernel, perform a read-only audit with::

    python3 scripts/tests/check_rust_exports_bridge.py /tmp/lupos-build

The checker verifies linked selection, current generated lists and dependencies,
actual export records, all final kernel exports and defining-object version
provenance. It ignores stale unlinked objects but rejects mixed bridge selection
and stale images. Original C comparison files are compiled only in a temporary
directory; no native build files are modified. Automatic comparison supports
x86 ELF32/ELF64; other targets require an original bridge object supplied with
``--reference``. Final ``vmlinux`` freshness is checked on every target; x86 also
requires a current ``bzImage``. Inspection is independent of the invoking
locale; DWARF parsing covers both GCC and Clang representations. Focused tests
are in ``test_rust_exports_bridge*.py``.

This removes the C metadata bridge when enabled; it does not translate the
remaining C helper implementations or the rest of the target kernel.

Translated target-kernel code
-----------------------------

The integer-math, integer-logarithm, generic wide-division, CORDIC, GCD/LCM,
rational-approximation, reciprocal-division, BCD, character-classification and
hexadecimal-helper Rust owners emit their own native export records using the
repaired translation
``include/linux/export_header.rs``, imported through ``rust/ffi_export.rs``.
No C export-glue source or object is selected for these implementations.
Kbuild reads the actual Rust object's DWARF to generate module versions;
export licenses, namespaces and target pointer widths are retained. Pure
Rust API consumers do not emit these records a second time.
This interface is for native implementation owners, not a general emulation of
C preprocessor export macros. Namespaces are explicit printable ASCII strings
excluding quotes, backslashes and braces; ordinary identifiers and
``module:name,name*`` namespaces are supported. These migrated owners retain
their original empty namespaces.

With ``CONFIG_MODVERSIONS=y``, rebuild modules when switching between the C
and Rust implementations. Native Rust debug types differ from C typedefs and
debug types, even when the machine-level C ABI is identical. The version tools
must describe the real implementation, not force a C checksum onto Rust types.
The existing Rust Kconfig dependency requires ``CONFIG_GENDWARFKSYMS`` for
module versioning; the original C configuration still supports genksyms.

At the strict-KCFI repair milestone, the combined host regression suite passed
all 1,170 tests with minimum Rust 1.85 and no skips, including genuine i686
execution and export-object checks.
Native x86-64 QEMU checks pass both C and independent Rust callers for integer
math, integer logarithms, GCD/LCM, rational approximation, reciprocal division,
BCD and ctype,
all seven hexdump exports,
and the original 1,184 hexdump tests. CORDIC, the wide-division provider and its
translated self-tests additionally pass the x86-64/ARM64 selection matrices
described below. A Rust-to-C-to-Rust build cycle restores
each choice's symbol versions;
the retained C kernel also loads its previously built ctype test module.
At the CORDIC milestone, a repeated final Rust build left all 32 monitored
active implementation, self-test, inspection, metadata, archive, version and
kernel-image artifacts unchanged in each of the x86-64 and ARM64 output trees.
Their hashes and timestamps also remained unchanged across that full host
suite, read-only audits and VM regressions.
For the subsequent strict-KCFI configurations, 13 x86-64 and 12 ARM64 active
configuration, implementation, binding/metadata, archive, version and image
artifacts remained byte- and timestamp-identical across the final no-op builds
and the 1,170-test suite, including the new polynomial arithmetic tests.

Differential tests generate C reference wrappers and ABI callers only in
temporary or out-of-tree build directories. These are test programs or
disposable QEMU modules, not new production implementations. The unchanged
Kbuild module machinery also generates ``*.mod.c`` metadata for both C and
Rust modules. Original C sources remain selectable, and translated sources
retain their ``SOURCE-COMMIT`` markers.

KCFI compatibility is an additional requirement, not implied by matching
structure layout or passing ordinary direct-call tests. A normalized-KCFI
code-generation audit identified and corrected type-identity mismatches in
the two reciprocal constructors, ``bin2hex``, ``hex2bin``, ``hex_dump_to_buffer``,
``print_hex_dump`` and ``insn_decode``. Native boundaries now retain the actual
C structure tags, the kernel's unsigned ``char`` type, and the decoder's C enum
identity, respectively. Compiler regressions compare the original C type IDs
and execute protected indirect calls; deliberately wrong types still produce
correct ordinary ABI results but trap with KCFI enabled.

Targeted strict-KCFI runs, with ``CONFIG_CFI=y`` and
``CONFIG_CFI_PERMISSIVE`` disabled, pass the four hexdump indirect-call paths
with unload/reload on x86-64 and ARM64. Both also pass the original 1,184-case
hexdump module twice. Both architectures pass both reciprocal callers with
reload; x86-64 passes 108 indirect decoder calls plus all seven original kprobe
cases. The corresponding host regressions are included in the full-suite
result above; these checks do not imply that the overall translation is complete.

Both strict configurations also pass the retained C providers with the same
callers and reload checks, including the original hexdump module and x86
decoder cases. Restoring the Rust selections reproduces each saved complete
configuration and ``Module.symvers`` byte-for-byte.

``CONFIG_RUST_INT_MATH=y`` selects translated integer exponentiation and square
roots. It requires native Rust support, defaults to disabled and is independent
of ``HOST_TOOLS_LANG``. The Makefile retains ``int_pow.o`` and ``int_sqrt.o`` for
the C choice, or selects the single ``int_math_rust.o`` for Rust.
``int_pow`` remains GPL-only; the square-root exports retain their original
license classification. As in C, ``int_sqrt64`` is an exported symbol only on
32-bit kernels; 64-bit C callers retain the original inline helper.

Independent Rust consumers import ``kernel::math``. Its pure, safe ``const``
functions share the translated algorithms without defining duplicate C exports.
``int_pow`` preserves wrapping 64-bit multiplication, including ``0**0 == 1``.
``int_sqrt`` uses the native word width, ``int_sqrt32`` offers an explicit 32-bit
input, and ``int_sqrt64`` consumes all 64 input bits on every architecture.
Core integer operations replace the translation's fictitious external bit-scan
functions. These functions need no allocation or foreign calls.

Only the integer-function boundary of ``include/linux/math_header.rs`` has been
repaired to reuse these algorithms. Its other translated macros and fraction
types remain pending work and are not exposed through ``kernel::math``.

For a completed x86-64 build with ``CONFIG_INT_POW_KUNIT_TEST=y`` and
``CONFIG_INT_SQRT_KUNIT_TEST=y``, run::

    python3 scripts/tests/check_int_math_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_int_math_kernel.py /tmp/lupos-build --caller rust

Each runner verifies all 30 original KUnit cases and compares 82,113 square-root
inputs plus 4,400 power pairs with the unchanged C implementation. The Rust
caller imports ``kernel::math`` independently. ``--allow-c-baseline`` selects
the retained C implementation; the usual isolated-tool arguments are supported.
Modules are loaded only inside QEMU. Focused host tests are in
``test_int_math*.py``. ``INT_MATH_I686_SYSROOT`` optionally supplies a matching
Rust i686 sysroot for genuine ELF32 differential execution; this is not a
simulation of pointer width through conditional-compilation overrides.

``CONFIG_RUST_GCD_LCM=y`` selects the translated greatest-common-divisor and
least-common-multiple implementations. It requires ``CONFIG_RUST``, defaults
to disabled and is independent of ``HOST_TOOLS_LANG`` and ``RUST_INT_MATH``.
The original ``gcd.o`` and ``lcm.o`` remain the C choice; Rust selects the single
``gcd_lcm_rust.o`` owner with the same three GPL-only C exports.

The canonical translations and their header modules provide safe, allocation-free
``const`` functions, also available as ``kernel::math::{gcd, lcm, lcm_not_zero}``.
They retain both original GCD algorithms, zero handling and division-before-
wrapping-multiplication semantics. Pure Rust imports define no C symbols and
need no fictitious foreign bit-scan functions or static-key layouts.

The native owner retains the real, nonexported ``efficient_ffs_key`` used by
architecture setup code, including RISC-V's existing Zbb-dependent disable.
Rust-owned storage uses ``Opaque`` over the actual generated C binding, not a
replacement layout. The typed static-branch macro requires the actual key's
permanent storage and rejects reference or dereference-wrapper substitutions.
Native calls consult the mutable key; pure constant-evaluable helpers select
the configured algorithm without kernel state. Both produce the same results.

For a completed x86-64 build with ``CONFIG_GCD_KUNIT_TEST=y``, run::

    python3 scripts/tests/check_gcd_lcm_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_gcd_lcm_kernel.py /tmp/lupos-build --caller rust

Each runner checks 211,056 input pairs across real key transitions 1-to-0-to-1
and all 11 unchanged GCD KUnit cases. ``--allow-c-baseline`` checks the retained
C implementation. The private test fixture resolves the exact image's key
address and boots only that image with ``nokaslr`` inside QEMU; no production
key export is added and no module is loaded on the host. The key is restored
before module initialization returns, including normal error paths.

Both callers pass with both C and Rust selected, with jump labels enabled and
disabled. The Rust-to-C-to-Rust cycle restores each implementation's version
CRCs and restores the complete Rust ``Module.symvers`` byte-for-byte. The native
key's actual header/binding layout and initializer bytes match C in both
configurations. The existing false-key macro is also checked at a nonzero
field offset. Focused tests are in ``test_gcd_lcm*.py`` and
``test_rust_static_key.py``; pure-algorithm differentials execute genuine
32-bit and 64-bit code with Rust 1.85 at O0/O2/Os. Native boot validation here
is x86-64; this is not a claim that other architectures have been boot-tested.

``CONFIG_RUST_RATIONAL=y`` selects translated rational approximation when
``CONFIG_RATIONAL`` is enabled. It requires ``CONFIG_RUST``, defaults to disabled
and is independent of the host-tool and other math selectors. The original
``rational.c`` remains selectable. The Rust composite uses ``rational_rust.o``
and preserves both the built-in choice and the module name ``rational.ko``.
The library retains its unrestricted export, ``GPL v2`` module license and
description, without introducing initialization state or init/exit functions.
Its export record and module metadata are emitted directly in Rust.

``kernel::math::rational_best_approximation`` and the translated rational header
share a safe, allocation-free ``const`` function returning a numerator and
denominator tuple. Native-word wrapping is explicit, including continued-
fraction limit calculations and tie-breaking products. Zero denominators and
zero bounds retain the original C behavior. The native six-argument C wrapper
writes its two raw output pointers sequentially, preserving identical-pointer
aliasing without creating aliased mutable references.

For a completed x86-64 build with ``CONFIG_RATIONAL`` and
``CONFIG_RATIONAL_KUNIT_TEST`` enabled, run::

    python3 scripts/tests/check_rational_kernel.py /tmp/lupos-build --reload-modules
    python3 scripts/tests/check_rational_kernel.py /tmp/lupos-build --caller rust --reload-modules

Each caller compares 12,296 input tuples against the unchanged C algorithm,
with normal, reversed and identical output pointers. The Rust caller also
checks the pure tuple API; the independent non-GPL C caller verifies that the
export remains unrestricted. All eight original rational KUnit cases run.
The runner supports built-in and modular providers and KUnit tests, loading
dependencies in order. Optional reload requires ``CONFIG_MODULE_UNLOAD=y``;
it unloads in reverse dependency order and repeats both caller checks and
modular KUnit tests. Modules are loaded only inside QEMU. Use
``--allow-c-baseline`` to test the retained C provider.

Both C and Rust providers pass with both callers in all three valid library/
KUnit configurations: built-in/built-in, built-in/module and module/module.
Return builds restore the captured C built-in and Rust built-in/module
``Module.symvers`` byte-for-byte, without removing intermediate objects.

Focused tests are in ``test_rational*.py``. They exercise actual 32-bit and
64-bit code with Rust 1.85 at O0/O2/Os, constant evaluation, direct pointer ABI
and aliasing, exact C module metadata, native DWARF versioning, source selection,
dependency rebuilding and no-op builds. Composite module versions are read
from the real constituent owner's saved command, not fabricated on the linked
aggregate. ``INT_MATH_I686_SYSROOT`` supplies optional genuine i686 support;
``NATIVE_RATIONAL_KERNEL_BUILD`` enables read-only checks of a completed kernel.
Both original ``SOURCE-COMMIT`` markers remain unchanged.

``CONFIG_RUST_RECIPROCAL_DIV=y`` selects the translated reciprocal constructors.
It requires ``CONFIG_RUST``, defaults to disabled and is independent of the
host-tool and other math choices. Kbuild selects ``reciprocal_div_rust.o`` or
the unchanged original ``reciprocal_div.o``, never both. The Rust owner emits
both unrestricted exports directly, retaining their C structure-return ABI.
Generated Rust bindings import the actual original reciprocal header.
The native owner converts fields into ``bindings::reciprocal_value`` and
``bindings::reciprocal_value_adv``. Returning the pure Rust structures directly
would preserve the machine layout but lose the C struct tags required by KCFI.
The safe API retains its independent Rust structures.

The canonical translation and thin translated header expose safe, allocation-
free ``const`` functions through ``kernel::math``. Both reciprocal structures
are ``repr(C)``. The basic constructor accepts every nonzero ``u32`` divisor.
The advanced constructor preserves the complete defined C domain: divisors
``1..=2^31`` and precision up to ``32 + ceil(log2(divisor))``, including zero
precision and wrapping of the original 64-bit addition. Invalid inputs return
``None`` in the safe API; the unsafe native C boundary retains the original
preconditions, without claiming C equivalence for undefined input arithmetic.
Manually supplied basic multipliers retain wrapping arithmetic when both
shifts are below 32. The 32-bit implementation uses native high-word division
and low-word restoring division without a ``__udivdi3`` runtime dependency.

For a completed little-endian x86-64 or ARM64 kernel with ``CONFIG_SLUB=y``,
``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and ``CONFIG_MULTIUSER=y``, run::

    python3 scripts/tests/check_reciprocal_kernel.py /tmp/lupos-build --reload-modules
    python3 scripts/tests/check_reciprocal_kernel.py /tmp/lupos-build --caller rust --reload-modules

The original x86-64 validation passes both C and Rust providers with both
callers, including unload and reload.
Each load checks 69,728 basic constructors, 557,824 quotient applications and
16,811 advanced constructors against unchanged C. The Rust caller also checks
the independent safe API. The non-GPL C caller verifies unrestricted exports
through volatile, correctly typed constructor pointers. Its SLUB workload runs
in a separate GPL allocator preload, keeping unrelated GPL-only allocator
dependencies out of the proprietary arithmetic module. Each run exercises
24 actual SLUB caches across non-power-of-two object sizes and three alignments,
with 3,456 allocations, alternating frees and
replacement allocations, and full-byte checks of live objects. The checker
requires the actual SLUB object to reference the selected constructor.
Modules are loaded only inside QEMU; fixtures stay outside the source tree.
Before the final-metadata repair described below, strict x86 builds could leave four unused
page/memory-helper names in the final symbol table after their addressable
sections are discarded. The private proprietary fixture removes only these
allowlisted undefined names after proving that no relocation or version record
uses them. It verifies that all remaining symbols, section contents, relocation
targets, license metadata and protected calls are unchanged. This does not
modify production modules or suppress GPL checks on either constructor.
The checker selects the architecture-specific image and emulator from the
completed configuration; ``--qemu`` and ``--qemu-data`` override the emulator.
Use ``--allow-c-baseline`` for the original provider. Optional reload requires
``CONFIG_MODULE_UNLOAD=y``; the Rust caller requires ``CONFIG_RUST=y``.
Forced module signatures must be disabled for these unsigned fixtures.
Rebuild modules when switching providers because
their native DWARF version CRCs differ, despite the compatible machine ABI.
That x86-64 provider cycle restores each complete configuration and
``Module.symvers`` byte-for-byte; only the two constructor versions differ
between the provider choices. The Rust provider also passes both callers and
reload with ``CONFIG_BUG=y``, verbose warning metadata and Rust debug assertions
enabled. The standard test configuration is restored afterwards.
The corrected native structure identities additionally pass both callers and
reload on ARM64 and x86-64 with strict normalized KCFI, as recorded in the
strict-boundary checkpoint above.

Focused regressions are in ``test_reciprocal*.py``. The pure translation is
tested with Rust 1.85 at O0/O2/Os on genuine 32-bit and 64-bit targets, including
independent imports, constant evaluation, original header layouts, all valid
manual shift pairs and invalid-input rejection. Tests compare structure fields,
not indeterminate C padding. The native O0 ABI tests link the real core library
and a fail-fast panic handler, with a forced-panic control; they do not invent
panic runtime symbols.
``INT_MATH_I686_SYSROOT`` supplies a matching optional i686 core; an explicitly
requested toolchain must work, not skip.
``NATIVE_RECIPROCAL_KERNEL_BUILD`` enables read-only native build inspection.
Both original ``SOURCE-COMMIT`` markers remain unchanged.

``CONFIG_RUST_INT_LOG=y`` selects the translated fixed-point logarithms.
It requires ``CONFIG_RUST``, defaults to disabled and is independent of
``HOST_TOOLS_LANG`` and the other math selectors. Kbuild selects either
``int_log_rust.o`` or the unchanged original ``int_log.o``. Both functions keep
their unsigned 32-bit C interfaces and unrestricted exports, without module
initialization state or C export glue.

The translation retains all 256 table entries, Q24 interpolation, the final
table-bucket correction and the exact base-conversion multiplier. It does not
substitute a differently rounded mathematical logarithm: ``intlog10(10)``
still returns ``16777225``, not exactly ``1 << 24``. Both original LGPL notices,
copyright notices and ``SOURCE-COMMIT`` markers remain unchanged.

The safe ``const`` helpers ``kernel::math::intlog2`` and ``intlog10``, also
available through the thin translated header, return ``None`` for zero without
logging. Nonzero input ``1`` correctly returns ``Some(0)``. The native C
interface instead preserves the original zero-input behavior: one non-once
kernel warning per invocation when ``CONFIG_BUG=y``, then a zero return value.
With ``CONFIG_BUG=n``, it returns zero without a warning. The base-ten wrapper
does not emit a second warning through the base-two calculation.

For a completed x86-64 build with ``CONFIG_INT_LOG_KUNIT_TEST=y`` or ``m``, run::

    python3 scripts/tests/check_int_log_kernel.py /tmp/lupos-build --reload-modules
    python3 scripts/tests/check_int_log_kernel.py /tmp/lupos-build --caller rust --reload-modules

The checker compares 71,272 positive inputs per caller against unchanged C,
covering all positive 16-bit values, table boundaries, powers and full-width
samples. Each load additionally calls each actual export with zero three times,
checks its zero return and counts warnings between exact invocation markers.
Warnings must identify the selected implementation and function. This catches
once-only suppression and accidental double warnings independently of KUnit.
The original 17 KUnit cases all run; modular reload repeats all 17 cases.
The Rust caller additionally exercises the independent safe API, and the
non-GPL C caller verifies that both exports remain unrestricted.

Both callers pass with both providers, including reload, with warnings disabled
and built-in KUnit, and with warnings enabled and either built-in or modular
KUnit. Warning provenance checks require ``CONFIG_KALLSYMS=y`` and
``CONFIG_DEBUG_BUGVERBOSE=y`` when ``CONFIG_BUG=y``. Other prerequisites are
``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and ``CONFIG_MULTIUSER=y``; optional
reload requires ``CONFIG_MODULE_UNLOAD=y``. Modules are loaded only inside QEMU.
Use ``--allow-c-baseline`` for the retained C provider and rebuild modules when
switching providers, because their native DWARF versions differ.

Focused regressions are in ``test_int_log*.py``. They cover original-C parity
with Rust 1.85 at O0/O2/Os on genuine 32-bit and 64-bit targets, constant
evaluation, imports, table/notice preservation, exports, versions and actual
Kbuild selection and dependency handling. Checked O0 builds link the real core
library and a fail-fast panic handler, including a deliberate panic control;
no panic or arithmetic runtime symbols are fabricated. Older math-family
consumer fixtures still import and type-check the complete production module,
but expose only their tested family so dependency checks stay scoped to those
functions. Actual whole-kernel consumers are separately built and boot-tested.
``INT_MATH_I686_SYSROOT`` supplies an optional matching i686 core, and
``NATIVE_INT_LOG_KERNEL_BUILD`` enables read-only completed-kernel inspection.

``CONFIG_RUST_DIV64=y`` selects the translated generic wide-division library.
It requires ``CONFIG_RUST``, defaults to disabled and is independent of
``HOST_TOOLS_LANG`` and the other math selectors. Kbuild selects either
``div64_rust.o`` or the unchanged original ``div64.o``. The native export set
still follows the architecture: on x86-64 this replaces only
``iter_div_u64_rem``; on ARM64 it also replaces
``mul_u64_add_u64_div_u64``. Existing architecture-specific assembly and inline
implementations remain in use. This is not a migration of every architecture's
division backend.

The safe, allocation-free ``kernel::math`` API shares the canonical algorithms
through the translated Linux, vDSO and generic division headers. Division
returns ``None`` for zero; signed division also rejects the minimum signed
value divided by minus one. Remainders use tuples or safe mutable references.
The native C interface instead retains its nonzero-divisor and pointer-validity
preconditions, including the original generic ILP32 signed-overflow result:
minimum divided by minus one wraps back to minimum. The shared wide
multiply/add/divide algorithm saturates an oversized quotient to ``u64::MAX``;
it does not reproduce the retained x86-64 inline assembly's overflow trap.
``mul_u64_u32_div`` deliberately retains the generic header's truncating
quotient rather than adopting saturation. Rounding and shift helpers preserve
their original wrapping and configuration-specific shift domains. These safe
helpers support constant evaluation on Rust 1.85 except for the iterative
division loop, whose real optimization barrier remains a runtime operation.

``CONFIG_RUST_DIV64_TESTS=y`` independently selects the two existing translated
self-test modules. It does not require the Rust native provider. The original
``CONFIG_TEST_DIV64`` and ``CONFIG_TEST_MULDIV64`` choices still control
built-in (``y``), module (``m``) or disabled selection, preserving
``test_div64.ko`` and ``test_mul_u64_u64_div_u64.ko``. Explicit rules select
the Rust source for ``.o``, ``.s`` and ``.ll`` targets; disabling the selector
restores the corresponding C rules. The ordinary ``.rsi`` target remains a
Rust-source expansion under either selection.

The division test retains all 144 cases in both constant-divisor and runtime
paths, repeated 1,024 times. The wide test retains all 28 original vectors,
including their exact full-width divisors, and checks both ordinary and
rounded-up results: 56 checks per variant. On LP64 there are three variants:
the public helper, the generic implementation and its smaller-digit algorithm;
the third variant is omitted on ILP32 as in C. Module lifecycle, original
diagnostics, real elapsed timing and the original init-return policies are
preserved. These supplementary self-tests do not alone establish that a
selected native Rust export was called.

For a completed x86-64 or little-endian ARM64 build with both original test
options set to ``m``, run::

    python3 scripts/tests/check_div64_kernel.py /tmp/lupos-build --reload-modules
    python3 scripts/tests/check_div64_kernel.py /tmp/lupos-build --caller rust --reload-modules

Each caller checks 4,222 iterative inputs through the selected native export.
ARM64 additionally checks 17,462 wide inputs through its selected wide export.
The independent Rust caller checks the safe generic wide API on all 17,462
inputs on either architecture. The x86-64 C caller does not claim a wide Rust
export exists. Comparisons use unchanged C algorithms compiled only into
private external test fixtures, not new production C glue. The checker verifies
the actual linked owner, export records, saved source/dependency commands,
freshness and self-test module identity before loading the two supplementary
test modules and the caller inside QEMU. Optional reload repeats the sequence;
it never loads modules into the host kernel.

The checker requires ``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and
``CONFIG_MULTIUSER=y``. The Rust caller or Rust self-tests require
``CONFIG_RUST=y``; reload requires ``CONFIG_MODULE_UNLOAD=y``. Forced module
signatures must be disabled for these unsigned fixtures. Use
``--allow-c-baseline`` for the original C provider. Rebuild native-export
consumers when changing providers under DWARF module versioning, because C
and Rust debug types produce different CRCs despite retaining the C ABI.

Focused validation includes 15 native-owner/build groups, 10 complete-header
groups and 11 canonical-arithmetic groups, including genuine 32-bit and 64-bit
coverage with Rust 1.85. The native Rust self-test modules and their assembly
and LLVM-IR inspection targets have also compiled on x86-64 and ARM64 with
that compiler. All 16 combinations of architecture, C/Rust provider, C/Rust
self-tests and C/Rust caller pass, with complete unload/reload in each run.
Restoring C restores each architecture's exact reference configuration and
symbol versions; restoring Rust restores its versions too. Rust self-tests
also pass as built-ins on both architectures through ``make rust-boot-test``,
with archive/source selection checked separately from their console results.
``NATIVE_DIV64_KERNEL_BUILD`` enables a read-only completed-kernel audit.
``INT_MATH_I686_SYSROOT`` supplies an optional matching 32-bit core;
``DIV64_LLVM_PREFIX`` supplies LLVM 19 for the inline-helper bitcode pipeline
test. The PowerPC object gate uses the compiler's matching
``powerpc-unknown-linux-gnu`` core. PowerPC coverage is an actual
object/link proof, not a kernel boot: the original strong assembly definition
wins over the generic weak definition in both link orders. Kbuild grants
``linkage`` only to the native division owner, adding
``asm_experimental_arch`` for its PowerPC export-record assembly on Rust 1.85;
the shared arithmetic and independent Rust consumers need neither feature.
Original copyright notices and all existing ``SOURCE-COMMIT`` markers remain
unchanged. Architecture-specific inline backends and unrelated translated
math tests remain outside this integration.

``CONFIG_RUST_CORDIC=y`` selects the translated fixed-point CORDIC library.
It requires native Rust, defaults to disabled and is independent of
``HOST_TOOLS_LANG``. The original ``CONFIG_CORDIC`` tristate still controls
built-in, module or disabled selection. The Rust choice uses
``cordic_rust.o`` as the implementation of the original ``cordic`` composite;
the C choice keeps ``cordic.c``. Both retain ``cordic.ko``, the original module
description, author and license, and the unrestricted ``cordic_calc_iq`` export.
No initialization, allocation or new production C glue is introduced.

The independent ``kernel::math`` API exposes ``CordicIq``, ``cordic_calc_iq``,
``cordic_fixed`` and ``cordic_float`` through the translated header and one
canonical implementation. All three helpers support constant evaluation on
Rust 1.85. The original eighteen-entry arctangent table remains immutable
static storage. Conversion to Q16 precedes normalization, preserving discarded
high input bits, signed remainder and wrapping arithmetic. Rounding also
retains the original minimum-signed-input exception: ``cordic_float(i32::MIN)``
returns positive 32768. Existing b43 callers' double fixed-point conversion is
tested as written, not silently changed to different angle units.

At the native boundary, the result is converted fieldwise to the actual
``bindings::cordic_iq`` type. Identical structure layout alone is insufficient
for KCFI: the pure Rust type's different nominal name produces a different
protected-call identifier. Tests compare the original Clang C and Rust 1.85
identifiers, execute protected indirect C calls at O0/O2, and show that the
old-name negative control traps despite passing unprotected ABI comparisons.
Both actual kernel and bindings metadata are recorded build dependencies.

For a completed x86-64 or little-endian ARM64 kernel, run::

    python3 scripts/tests/check_cordic_kernel.py /tmp/lupos-build --reload-modules
    python3 scripts/tests/check_cordic_kernel.py /tmp/lupos-build --caller rust --reload-modules

Each load compares 69,841 angles, 69,841 fixed-point conversions and 896
b43-style double-conversion calls with the unchanged C reference. The Rust
caller checks both the selected native binding and the independent safe API.
The checker validates linked objects, actual source and library dependencies,
export records, module identity, freshness and import/version provenance
before loading anything inside QEMU. It requires ``MODULES``, ``PRINTK`` and
``MULTIUSER``; reload additionally requires ``MODULE_UNLOAD``. Forced module
signatures must be disabled for the unsigned private fixtures. Use
``--allow-c-baseline`` for the retained C provider, and rebuild consumers when
switching implementation under module versioning.

All sixteen combinations of architecture, C/Rust provider, built-in/module
selection and C/Rust caller pass with unload/reload. ARM64 additionally passes
both callers and both provider placements with strict normalized KCFI, without
permissive mode. Disabled selections on both architectures have no native
CORDIC export or linked owner while retaining the independent Rust API.
With sources held fixed, the C-to-Rust round trip restores each selection's
exact configuration and symbol-version file. Native Rust assembly and LLVM-IR
inspection targets also build on both architectures.

The 41 focused regression groups cover genuine 32/64-bit execution, all
65,536 fixed-angle patterns, all Q16 integer parts around rounding boundaries,
266,240 aggregate-return inputs, metadata, real Kbuild switching and negative
controls. ``INT_MATH_I686_SYSROOT`` supplies a matching 32-bit core, ``BINDGEN``
selects the original-header binding generator, and
``NATIVE_CORDIC_KERNEL_BUILD`` enables read-only completed-kernel inspection.
Original notices and both existing ``SOURCE-COMMIT`` markers are preserved.

``CONFIG_RUST_BCD=y`` selects the translated binary-coded decimal conversions.
It requires native Rust support, defaults to disabled and is independent of
``HOST_TOOLS_LANG``. Leaving it disabled retains ``bcd.o``; the Rust choice
uses only ``bcd_rust.o``. The implementation is pure,
allocation-free and retains the two exported C functions, including unsigned
overflow and invalid-digit behavior.

Independent Rust consumers import ``kernel::bcd``. The shared safe helpers do
not introduce duplicate C exports or foreign calls and work with either build
selection. Rust cannot reproduce C's ``__builtin_constant_p`` dispatch, so the
API names distinguish its two arithmetic paths explicitly: ``bcd2bin(u8)``
and ``bin2bcd(u32)`` match the runtime functions, while ``const_bcd2bin(u32)``
and ``const_bin2bcd(u32)`` match the full-width unsigned constant expressions.
For example, ``bin2bcd(1024)`` returns ``0xfa``, whereas
``const_bin2bcd(1024)`` returns ``0x664`` before any caller-requested narrowing.
All helpers support Rust constant evaluation without changing these semantics.
``bcd_is_valid`` and ``const_bcd_is_valid`` check the entire unsigned argument;
conversion itself does not reject invalid digits.

Rebuild modules after changing this option when using DWARF symbol versioning.
The Rust choice versions the real Rust definitions, including their parameter
names and native types. The C selection restores the original C metadata.

After a completed x86-64 build, check the selected implementation in QEMU::

    python3 scripts/tests/check_bcd_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_bcd_kernel.py /tmp/lupos-build --caller rust

The first command checks both C exports and the header's constant/runtime
dispatch. The second builds an independent native ``kernel::bcd`` consumer.
Each compares 69,646 inputs, including the complete 16-bit range, against the
unchanged C source and header. Use ``--allow-c-baseline`` for the original C
selection and ``--make-arg``, ``--qemu`` or ``--qemu-data`` for isolated tools.
The runner requires modules, printk and multiuser support, plus native Rust
for the Rust caller, and rejects forced module signatures for its unsigned
fixtures. It validates linked objects and image freshness before booting and
never loads modules into the host. Artifacts remain under ``rust-bcd-test/``
and ``rust-boot-test/``. Focused host tests are in ``test_bcd*.py``.

Native x86-64 builds with Rust 1.85 and extended DWARF module versions pass
both callers with C and Rust implementations. Switching C to Rust and back to
C in one output tree restores the original C CRCs and permits the original
C test module to load unchanged. The independent Rust API consumer also
loads unchanged when switching from C to Rust because it shares pure
helpers rather than depending on either C export's version.

``CONFIG_RUST_CTYPE=y`` selects the translated, immutable 256-byte character
classification table. It requires native Rust support, defaults to disabled,
and is independent of ``HOST_TOOLS_LANG``. The original ``ctype.o`` remains
available; the Rust selection uses only ``ctype_rust.o``.
Boot, firmware and host-tool copies are unchanged. The table preserves all
historical Latin-1 entries, including NBSP whitespace and punctuation at
``0xd7`` and ``0xf7``; this is neither ASCII-only nor Unicode classification.

Native Rust consumers import ``kernel::ctype``. That API reuses the translated
safe helper definitions with a private, bounded lookup into the selected C or
Rust table. It introduces no second table, allocation or initialization work.
Only the table-owning crate includes ``lib/ctype.rs``; independent modules must
not include that source again. Table-independent helpers remain usable in
constant expressions. Calls through ``kernel::ctype`` that read the foreign
table require runtime access, because its contents are resolved at link time.

The helpers retain unsigned-byte truncation where C performs it, the full
integer comparison used by ``isdigit``, and the original case-conversion
quirks. Differential tests compare every table byte and helper with the
unchanged C sources under signed/unsigned ``char``. Independent-crate tests
check that consumers share exactly one immutable table. The native Rust array
definition supplies its complete debug type, without a C redeclaration. Its
DWARF checksum differs from C's ``const unsigned char[256]`` debug type, so
versioned modules must be rebuilt when changing the selected implementation.

After a completed x86-64 build, test C callers inside QEMU with::

    python3 scripts/tests/check_ctype_kernel.py /tmp/lupos-build

Use ``--caller rust`` to test a separately compiled native Rust module using
``kernel::ctype``, and ``--allow-c-baseline`` to exercise the original C table.
These checks require ``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and
``CONFIG_MULTIUSER=y``; the Rust caller also requires ``CONFIG_RUST=y``.
Forced module signatures must be disabled for the unsigned fixtures. The
runner checks actual archive membership and image freshness before booting,
and never loads a module into the host. ``--make-arg``, ``--qemu`` and
``--qemu-data`` support isolated toolchains and emulators. Test artifacts stay
under ``rust-ctype-test/`` and ``rust-boot-test/`` in the output directory.

Native x86-64 builds with Rust 1.85 and extended DWARF module versions pass
all 256 table bytes and 8,194 inputs per caller with both table selections.
These callers must be rebuilt against the selected table's symbol version;
the former C metadata shim's cross-selection checksum compatibility no longer
applies. Host differential, build-selection and runtime-runner checks are
available through ``test_ctype*.py``.

``CONFIG_RUST_HEXDUMP=y`` selects the translated ``lib/hexdump.rs`` for the
kernel's hexadecimal conversion and dump functions. This option requires
native ``CONFIG_RUST`` support and is disabled by default; leaving it disabled
keeps the original C implementation. It is independent of ``HOST_TOOLS_LANG``.
``lib/hexdump_rust.rs`` supplies the kernel crate boundary without changing the
adjacent C source's build rule. It emits the export records directly, including
the ``CONFIG_PRINTK``-conditional export; no C export-only object is needed.

The translation needs no allocation. It preserves forward overlapping-buffer
operations, partial writes, native-endian groups, empty-buffer behavior, the
branch-free hexadecimal-digit conversion, and the kernel's printk formatting
and pointer policy. ``CONFIG_PRINTK_INDEX`` emits three records using the real
packed kernel binding and Rust call-site locations. Signed and unsigned host
``char`` bindings are tested separately. Native char pointers use
``kernel::ffi::c_char``, matching the kernel's ``-funsigned-char`` setting;
standalone imports retain ``core::ffi::c_char``. Their machine pointer ABI is
the same, but normalized KCFI distinguishes signed and unsigned pointees.
Tests match the original C IDs and protected calls at O0/O2/Os, with
wrong-signedness controls that work unchecked but trap under KCFI.

Rebuild modules when changing this option. In DWARF module-versioning mode,
the native Rust definitions have different debug types and CRCs from C despite
preserving the machine-level C ABI. The C selection retains its original
metadata; the Rust selection and modules built against it use matching
generated versions.

The normal x86-64 build with Rust 1.85, extended module versions and printk
indexing is verified through QEMU. The unchanged C ``test_hexdump`` module
passes all 1,184 tests against the Rust implementation. An additional native
C-caller module checks all seven exported symbols, including conversion errors,
overlap, null/zero-length calls, and all printk prefix modes. After a completed
little-endian x86-64 or ARM64 build with ``CONFIG_RUST_HEXDUMP=y``,
``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and ``CONFIG_MULTIUSER=y``, run it with::

    python3 scripts/tests/check_hexdump_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_hexdump_kernel.py /tmp/lupos-build --reload-modules

This runner builds a temporary test module and loads it only inside QEMU,
never into the host kernel. It checks the linked implementation and rejects
stale architecture-specific images before booting. The four pointer-signature
functions are called through volatile pointers with their actual C types.
Optional reload requires ``CONFIG_MODULE_UNLOAD=y`` and repeats the complete
fixture; validation requires both byte-exact print sequences and correctly
ordered load, unload, reload and final boot markers. Strict-KCFI kernels pass
this fixture with reload on both architectures. Read-only inspection of those
actual module objects confirms the unsigned-char/normalized-KCFI compiler
flags and all 13 protected call sites: x86 type checks and trap records, and
ARM64 type comparisons with conditional traps before indirect branches.
Use ``--allow-c-baseline`` to run the same checks against the retained C
selection, ``--make-arg`` for nonstandard toolchain
assignments, and ``--qemu``/``--qemu-data`` for an isolated emulator installation.
Forced module signatures must be disabled for this unsigned test fixture.
Artifacts remain in ``rust-hexdump-test/`` and ``rust-boot-test/`` under the
output directory. Host differential, guarded-memory, ABI and build-selection
regressions are available through ``test_hexdump*.py``.

``CONFIG_RUST_X86_INSN=y`` selects the translated x86 instruction decoder
and attribute tables for the running kernel. It requires ``CONFIG_RUST`` and
``CONFIG_INSTRUCTION_DECODER`` and defaults to disabled. The original C
``insn.o`` and ``inat.o`` remain selected when it is disabled; the C instruction
evaluator, compressed-boot decoder and objtool decoder are unchanged.

The native boundary uses the actual generated ``asm/insn.h`` bindings and
provides all ten decoder and six attribute-table C entry points. Field-wise
updates preserve union aliases, padding, noncanonical cached flags, manually
selected operand/address widths, and partial state after errors. Bounded reads
visit only requested bytes; cached stages need not access instruction memory.
Rebased cursors and input overlapping the state structure are covered. The
safe, allocation-free decoding core remains shared with the host tools.
The native ``insn_mode`` binding is a transparent integer newtype with the
original C enum's ``cfi_encoding``. This preserves KCFI type identity without
using a Rust enum that would make out-of-enumeration integer values invalid.
Its constants come from the original header; the standalone safe decoder API
is unchanged. ``test_x86_decoder_kcfi.py`` compares original C type IDs and
protected calls, including an integer-alias negative control.

Host regressions in ``test_x86_decoder_abi.py`` compare native layouts and
complete state against the original architecture C headers and implementation,
including guarded pages and self-aliasing input. Build-selection and runtime
checker regressions are in ``test_x86_decoder_build.py`` and
``test_x86_decoder_runtime.py``. To run the unchanged kernel kprobe/kretprobe
KUnit suite inside QEMU, build with ``CONFIG_KPROBES=y``, ``CONFIG_KUNIT=y``,
``CONFIG_KPROBES_SANITY_TEST=y``, ``CONFIG_PRINTK=y``, ``CONFIG_MULTIUSER=y``,
and a reliable unwinder
such as ``CONFIG_UNWINDER_ORC=y``, then run::

    python3 scripts/tests/check_x86_decoder_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_x86_decoder_kernel.py /tmp/lupos-build --indirect-abi --reload-modules

The checker verifies linked decoder selection and a current boot image, then
requires every configured kprobe case to pass without skips. Use
``--allow-c-baseline`` to run the same tests with the retained C decoder.
``--qemu`` and ``--qemu-data`` select an isolated emulator installation.
``--indirect-abi`` additionally builds a private C module using the exact linked
``insn_decode`` address from the completed image and its original enum-bearing
function-pointer type. The symbol is not made into a module export: this is a
checked-address fixture for the runner's ``nokaslr`` guest only. It requires
``CONFIG_MODULES=y`` and disabled forced module signatures; reload also requires
``CONFIG_MODULE_UNLOAD=y``. Each load executes 54 instruction/mode combinations,
including truncation and out-of-enumeration mode values. With strict normalized
KCFI, x86-64 passes all 108 calls across load/reload and the seven original
kprobe/kretprobe cases. The checker verifies the actual fixture's indirect call
and KCFI trap records when ``CONFIG_CFI=y``.
The minimal ``UNWINDER_GUESS`` configuration fails the original nested-kretprobe
stacktrace test with both implementations; it is not a decoder regression.
With ORC, all seven original kprobe/kretprobe cases pass on both C and Rust
x86-64 kernels built with Rust 1.85. Switching C to Rust and back to C in the
same output directory also passes all seven cases after each rebuild; stale
objects from the other selection remain harmless and are not linked.

Kernel pipeline validation
--------------------------

To exercise the migrated generators through an x86-64 kernel build, header
installation, and an emulated boot::

    make O=/tmp/lupos-build defconfig
    make O=/tmp/lupos-build -j8 bzImage
    make O=/tmp/lupos-build headers_install INSTALL_HDR_PATH=/tmp/lupos-headers
    make O=/tmp/lupos-build rust-boot-test

The boot test supports x86-64 and little-endian ARM64. With ``ARCH=arm64``,
``make rust-boot-test`` builds ``Image`` and selects QEMU's ARM64 ``virt``
machine; the default x86 path uses ``bzImage``. ARM64's static Rust PID 1
requires the matching ``aarch64-unknown-linux-musl`` standard library for
``HOSTRUSTC`` and uses that toolchain's bundled linker and musl startup files.
The direct runner accepts ``--arch aarch64`` as well. Set ``QEMU`` to an
alternate emulator command and ``QEMU_DATA`` to an alternate firmware
directory if necessary. The test uses software emulation without networking or
root privileges. It builds a static Rust PID 1, packs an initramfs with the
migrated generator, and boots the image. PID 1 verifies binary file contents,
hardlinks, symlinks, ownership, and permissions before printing a success
marker. The runner then stops QEMU. Artifacts and the console log are kept in
``rust-boot-test/`` under the kernel output directory.

The ownership checks require ``CONFIG_MULTIUSER=y``. ``CONFIG_PRINTK=y`` is
also required: PID 1 emits each success marker as one ``/dev/kmsg`` record,
serialized with module output. The isolated guest disables userspace log
rate limiting and enables informational console messages. A short record write
fails instead of retrying a partial marker; strict checks still reject missing,
duplicated, corrupted or reordered events, with or without printk timestamps.
The runner also accepts
``--module /path/to/module.ko`` to load a module inside the isolated VM and
repeatable ``--reject-module /path/to/unsigned.ko`` arguments to check signature
enforcement. Rejection tests require ``CONFIG_MODULE_SIG_FORCE=y`` and expect
``EKEYREJECTED`` before attempting the valid load. For a signing-pipeline test,
enable ``CONFIG_TEST_HEXDUMP=m``, ``CONFIG_MODULE_SIG_ALL=y`` and signature
enforcement, install modules into a temporary ``INSTALL_MOD_PATH``, pass the
installed signed ``test_hexdump.ko`` as ``--module``, and pass the original
unsigned build artifact as ``--reject-module``. This exercises generated symbol
versions, certificate extraction, signing, kernel verification, and the loaded
module's own hexdump tests without loading anything into the host kernel.

This currently validates a mixed C/Rust kernel build; it does not establish
that the target-kernel translation is complete.

Polynomial arithmetic
---------------------

``lib/math/polynomial.rs`` and its translated header now provide a checked,
allocation-free, constant-evaluable API. ``Polynomial`` borrows a bounded term
slice instead of representing the C inline flexible array as a pointer field.
The algorithm retains unsigned degrees, native-long wrapping, the original
quotient/remainder redistribution order and truncation toward zero. It stops
at the first constant term, ignores that term's unused step divider and any
trailing terms, and treats a zero total divider as one. Evaluated invalid
divisions or a missing terminating term return ``None``.

The focused differential tests use Rust 1.85, genuine 32/64-bit execution
and both GCC and Clang reference builds. They cover
all 16 original KUnit vectors, the four original polynomial descriptors from
three drivers, wrapping/rounding edges, checked errors, constant evaluation,
and independent consumers of the canonical source, translated header and
actual public ``kernel::math`` API, including incremental term evaluation.
The original C source, public header and ``mult_frac`` macro supply the oracle.
``test_polynomial_translation.py`` contains these checks.

``CONFIG_RUST_POLYNOMIAL=y`` now selects the Rust native owner independently
of ``HOST_TOOLS_LANG``. The original ``CONFIG_POLYNOMIAL=n/y/m`` selection,
``polynomial.ko`` identity, GPL export and stateless module metadata remain
intact. The default-off selector retains the original C implementation.
``lib/math/polynomial_rust.rs`` uses the original header's generated binding
types and reads the inline flexible array field by field. It neither creates
an unbounded Rust slice nor reads a constant term's unused divider, padding,
or trailing terms. The safe Rust API remains available independently of the
native provider selection. Existing ``SOURCE-COMMIT`` markers are unchanged.

``test_polynomial_build.py`` checks genuine 32/64-bit native ABI behavior,
actual binding types, original metadata and GPL export records, KCFI type
identity and rejection of a wrong nominal pointer type, DWARF versions,
and parallel/no-op C/Rust build selection. Native version records belong to
the defining Rust leaf object, not its aggregate ``polynomial.o`` wrapper.
Set ``NATIVE_POLYNOMIAL_KERNEL_BUILD`` to a completed native build to enable
the optional selected-owner/archive/version/freshness check.

``check_polynomial_kernel.py BUILD --caller c|rust --reload-modules`` validates
and boots the actual selected implementation. For the original C provider,
also pass ``--allow-c-baseline``. The runner accepts repeated ``--make-arg``
options and the same ``--qemu``/``--qemu-data`` overrides as the other native
math checks. Its fixtures live only in the output directory and compare
against unchanged original C; no production C implementation is added.

All 16 native combinations pass with Rust 1.85 on x86-64 and ARM64: C/Rust
providers, built-in/modular selection and C/Rust callers, each with module
unload/reload. Each load checks 6,084 cases, including all 16 original KUnit
vectors and all 1,024 inputs for each of the four original driver descriptors.
The unchanged original KUnit suite also passes: 16 cases for built-in tests,
or 32 across module load/reload. These kernels enable symbol versioning and
strict integer-normalized KCFI, with permissive mode disabled. Actual C
caller instructions are checked for protected indirect calls on both ISAs.
Returning to the saved C built-in configuration reproduces its exact
configuration and symbol-version table. With the provider disabled, both
values of the Rust selector build successfully on both architectures, with
no selected native provider, export or module and a fresh safe math API.
Restoring the final Rust built-in configurations likewise reproduces their
saved configuration and symbol-version tables; both callers still pass
afterward on both architectures.

The three unchanged C consumers, ``lan966x-hwmon``, ``eic7700-pvt`` and
``mxl-gpy``, also build against either provider with real polynomial call
relocations and matching module dependencies. Both sets of modules register,
unload and reload in an x86-64 VM. The compile-test configuration enables the
required regmap and reset-controller support so the EIC probe does not become
an unconditional error path that optimizes away its polynomial callbacks.
This configuration necessarily disables ``MODVERSIONS``; versioned parity
is established by the separate native caller matrix above. These driver
checks do not claim sensor measurements or hardware probing in the VM.

All 42 polynomial build, runtime-checker and arithmetic tests pass with both
GCC and Clang references, the native-build gate enabled and genuine 32-bit
execution, without skips.
At the native-library milestone, strict-kernel DWARF audits match original C
tooling for 455 compilation
units/7,069 unit CRCs on x86-64 and 405 units/6,901 unit CRCs on ARM64. The
linked-image export counts are 7,069 and 6,900 respectively; both unstripped
and final images agree. Fresh C references also match all 3,204 x86-64 and
3,159 ARM64 Rust export-bridge records.

At the native-library integration milestone, the minimum-Rust-1.85 host regression suite
passes all 1,200 tests without skips, including the new polynomial native
gate and genuine i686 execution. All 28 monitored active configuration,
implementation, binding/metadata, archive, version and image artifacts across
the final strict x86-64/ARM64 builds retain their exact hashes and timestamps
across the suite and subsequent no-op builds.

Parameterized polynomial KUnit suite
-----------------------------------

``CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=y`` selects the repaired translation of
``lib/math/tests/polynomial_kunit.rs`` independently of the native polynomial
provider and ``HOST_TOOLS_LANG``. The option defaults off, retaining the C
suite. The original test tristate and ``polynomial_kunit.ko`` name are
unchanged; explicit object, assembly and LLVM-IR rules select the Rust source
despite its shared basename with C.

The translation uses generated native binding types, including the original
polynomial flexible-array layout, and calls the selected provider's real ABI.
It retains all nine descriptors, 16 parameter values/descriptions, original
suite/case names, module metadata and exact ``SOURCE-COMMIT``. It now registers
an actual KUnit suite instead of relying on incomplete private structure
definitions or discarded expectation results.

Shared helpers in ``rust/kernel/kunit.rs`` supply native parameter registration,
the real 128-byte description bound, original default attributes and nonfatal
integer equality expectations. Expectations evaluate operands once, compare
their original types, record the source location and use the C framework's
binary diagnostic formatter without aborting Rust frames. Literal suite names
preserve punctuation in original names; the existing identifier-based macro
retains its previous defaults. Call-site expansion supports a modular KUnit
framework without introducing built-in references to its module symbols.

``test_kunit_parameters.py`` checks the shared implementation against original
C declarations and macros with Rust 1.85, GCC/Clang references and genuine
32/64-bit execution. ``test_polynomial_kunit.py`` exercises actual translated
callbacks and generators, including intentionally incorrect provider results,
as well as metadata, provenance and real Kbuild language selection. Its
read-only native gate uses ``NATIVE_POLYNOMIAL_KERNEL_BUILD``.

The translated polynomial suite has also run in x86-64 and ARM64 kernels,
built-in and modular, including module unload/reload and ``KUNIT=m``. The
native checker verifies the independently selected suite source and preloads
the framework before provider/test modules when necessary. Each suite run
retains the original 16 parameter invocations.

``check_kunit_expectations.py BUILD --reload-modules`` adds a private Rust-only
negative-control module. It requires two intentional expectation failures with
the exact signed/unsigned diagnostics and source locations, continuation after
each failure, and a passing following case. Both built-in and modular framework
controls pass on both architectures, including unload/reload. Unrelated failures and
kernel faults are rejected. The original C failure formatter itself executes
an internal assertion while registering its temporary resources, so it may
overwrite ``last_seen``; the control checks that field after a successful
expectation and checks the failure call sites through the actual diagnostics.

Integer-power and integer-root KUnit suites
------------------------------------------

``CONFIG_RUST_INT_MATH_KUNIT_TESTS=y`` independently selects the repaired
``int_pow_kunit.rs`` and ``int_sqrt_kunit.rs`` translations. Each original
test tristate, module name and C fallback remain available. The option defaults
off and does not select the native provider language or depend on
``HOST_TOOLS_LANG``. Their explicit object, assembly and LLVM-IR rules retain
the original basenames.

Both suites now use the shared native-binding KUnit helpers above, with all
nine power and 21 root vectors, parameter descriptions, suite/case names,
module metadata and exact provenance markers preserved. Expectations remain
nonfatal and retain the original expected/actual operand order. Power vectors
retain full-width ``u64`` values; root vectors call the actual native-word
``int_sqrt`` binding, not the separate ``int_sqrt64`` helper.

``test_int_math_kunit.py`` runs original C and translated Rust callbacks and
parameter generators with genuine 32/64-bit targets, including intentionally
corrupted provider results. It checks metadata, provenance, independent
selection and real C/Rust Kbuild object/assembly/IR switching and dependencies.
``check_int_math_kernel.py`` now verifies the actual selected suite sources,
registration, native imports, module versions and artifact freshness before
booting. It accepts ``--reload-modules`` and detects x86-64 versus ARM64 from
the native configuration.

The initial built-in Rust suites pass on both architectures with strict KCFI
and symbol versioning. All 30 original parameter cases run, while each C or
Rust caller module load additionally checks 82,113 root and 4,400 power inputs;
both callers also pass unload/reload. Both modular suites pass with built-in
or modular KUnit; the two mixed built-in/module arrangements also pass.
With modular KUnit, all four C/Rust provider/suite-language combinations pass
on both architectures, including 60 parameter invocations across module
load/reload. With built-in tests, all four language combinations likewise
pass. Returning to the saved all-Rust built-in configurations reproduces their
exact configuration and symbol-version tables, and both callers pass again
for the integer-math and polynomial suites.

With these three translated KUnit suites integrated, the complete minimum-
Rust-1.85 regression run passes all 1,261 tests without skips, including genuine
i686 execution and the configured native-build gates. The focused KUnit tests
also pass with GCC and Clang references. The 38 monitored active configuration,
version, implementation, test-object, binding/metadata, archive and image
artifacts retain their exact contents and timestamps across the full suite and
subsequent no-op strict-kernel builds.

Fresh strict-kernel DWARF audits match original C tooling for 455 compilation
units/7,070 unit CRCs on x86-64 and 404 units/6,902 unit CRCs on ARM64. Both
intermediate and final linked images agree on 7,070 and 6,901 exported symbols,
respectively. Original C export-bridge references match all 3,205 x86-64 and
3,160 ARM64 records. The normal GCC x86 and LLVM ARM64 outputs retain their
previous symbol-version tables; repeated builds preserve sizes and timestamps
across all 6,611 and 4,205 regular output files, respectively.

GCD and rational-approximation KUnit suites
------------------------------------------

``CONFIG_RUST_GCD_KUNIT_TEST=y`` and
``CONFIG_RUST_RATIONAL_KUNIT_TEST=y`` independently select the repaired
``gcd_kunit.rs`` and ``rational_kunit.rs`` translations. Both language options
default off and preserve the original C suite, test tristate, module name and
provider selection. Explicit object, assembly and LLVM-IR rules select Rust
without changing the original object order.

The suites use actual generated KUnit and arithmetic bindings, with the native
declarations supplied by ``linux/gcd.h`` and ``linux/rational.h``. All 11 GCD
and eight rational vectors, their ordered descriptions (including duplicate
GCD descriptions), native unsigned-long widths, original module metadata and
exact provenance markers are preserved. GCD calls the native provider rather
than replacing its runtime static-key selection with a pure Rust calculation.
Rational retains two independent, nonfatal numerator/denominator expectations.

``test_gcd_rational_kunit.py`` compares the original C and actual translated
callbacks/generators on genuine 32/64-bit targets, including corrupted provider
outputs. All 11 behavior, metadata and build-selection groups pass with minimum
Rust 1.85, GCC/Clang references and genuine i686 execution. The build checks
cover independent C/Rust selections, original tristates/module order, cold
parallel object/assembly/IR builds, no-op rebuilds and transitive dependencies.
Both translated suites also build and pass their original
19 parameter cases in x86-64 and ARM64 kernels with CFI and symbol versioning,
using built-in Rust providers and a built-in KUnit framework. With modular
KUnit and both suites as modules, framework-first load and reverse-order
unload/reload pass on both architectures, repeating all 19 parameter cases.
Both rational native callers additionally pass 12,296 tuples per load on each
architecture; the checker validates native source selection, imports, versions
and ordered suite/module completion. The proprietary C caller's private build
uses ``__DISABLE_EXPORTS`` only to suppress unused header address-reference
metadata. It exports nothing, retains its original non-GPL license and keeps
KCFI enabled, verified in the saved compiler flags and indirect-call machine
code. This does not change kernel compilation flags or remove real imports.

Both GCD/LCM callers pass 422,112 pairs across load/reload on x86-64, including
real static-key transitions ``1 -> 0 -> 1`` and 22 original KUnit parameter
invocations. The Rust caller checks both the actual native exports and the
pure Rust API. The address-bound static-key fixture remains explicitly
x86-64-only; the ARM64 suite boot/reload checks do not claim to validate that
fixture. All four native provider/suite-language combinations pass with modular
KUnit on both architectures, including framework-first load and suite reload.
The x86-64 GCD fixture checks the real static-key transitions in each combination.
An additional x86-64 configuration genuinely selects ``RATIONAL=m``; all four
provider/suite-language combinations pass with framework, provider and suite
modules loaded in order and unloaded/reloaded. ARM64 selects ``COMMON_CLK``,
which forces ``RATIONAL=y``; its tests do not claim modular-provider coverage.
Restoring the saved all-Rust built-in configurations reproduces both original
symbol-version tables; the only configuration addition is the disabled new
integer-log suite selector. The restored native rational callers pass again on
both architectures, as do the x86-64 GCD static-key fixture and both built-in
GCD/rational suites. The separate modular-provider configuration likewise
restores its symbol-version table and passes again.

The normal GCC x86 and LLVM ARM64 outputs keep both new suite-language
options disabled. After refreshing their generated bindings, repeated normal
builds preserve sizes and timestamps across all 6,611 and 4,205 regular output
files, respectively.

At this GCD/rational integration checkpoint, the complete minimum-Rust-1.85
regression run passes all 1,309 tests without skips. All 80 monitored active
configuration, provider, suite, framework, binding, module-version and image
artifacts across the strict modular-framework builds keep their exact contents
and timestamps across the full suite and subsequent no-op builds. The normal
outputs' 10,816 regular files also retain their sizes and timestamps.

Fresh modular-framework DWARF audits match the original tooling for 455
compilation units/7,002 unit CRCs on x86-64 and 403 units/6,832 unit CRCs on
ARM64. Both intermediate and final linked images agree on 6,937 and 6,764
exported symbols, respectively; original C export-bridge references match all
3,137 x86-64 and 3,090 ARM64 records.

Integer-logarithm KUnit suite
----------------------------

``CONFIG_RUST_INT_LOG_KUNIT_TEST=y`` independently selects the repaired
``int_log_kunit.rs``. The default-off choice retains the original C suite,
test tristate, module identity and object order, with explicit Rust object,
assembly and LLVM-IR rules. All nine binary-log and eight decimal-log vectors,
ordered descriptions, expected-first nonfatal assertions, module metadata and
the exact provenance marker are preserved. The suite calls actual native
bindings, including both zero inputs and their provider warnings, rather than
substituting the pure Rust checked API's ``None`` result.

All ten groups in ``test_int_log_kunit.py`` pass with minimum Rust 1.85,
GCC/Clang references and genuine 32/64-bit execution. These cover original
callbacks/generators, corrupted provider results, scoped zero-warning counts,
metadata, independent Kconfig choices and real C/Rust Kbuild switching,
parallel object/assembly/IR builds, no-op rebuilds and dependencies. Negative
warning controls must compile successfully and fail at execution with the
expected code. The combined translation, ABI, Kbuild, KUnit and runtime-checker
regression passes all 67 tests without skips, including a read-only native
artifact/version audit.

The selected Rust suite builds and passes all 17 original parameters in both
x86-64 and ARM64 kernels with CFI, module versioning, built-in KUnit and
``BUG=y``. Both C and Rust caller modules additionally pass 71,272 positive
inputs and six zero probes per load, including unload/reload. The checker
requires the two original KUnit zero warnings and exactly one warning inside
each caller's zero-invocation boundaries, from the selected native function.
Unrelated warnings and kernel faults remain errors. On x86-64 with modular
KUnit, all four provider/suite-language combinations pass framework-first
load and reverse-order unload/reload with ``BUG=n``, including 34 original
parameter invocations and no zero warnings. The all-Rust module configuration
is restored afterward and passes again without changing its symbol-version
table. All four built-in provider/suite-language combinations also pass on
both architectures with ``BUG=y``, including exact warning provenance and
counts for the selected C or Rust provider. Returning to the saved all-Rust
built-in selections reproduces both exact configurations and symbol-version
tables, and the native Rust callers pass again.

ARM64 exposed legitimate ``nbcon`` console takeover: a partial record followed
by the kernel's explicit replay marker and the complete record. The checker
normalizes only this exact marker and an adjacent, nonempty, byte-matching
prefix of the newline-terminated replay; it does not discard unmarked
duplicates or guess mismatched/multiline replays. Tests retain raw-console
fatal-diagnostic checks, exact warning counts and ordered suite/module events.

The all-Rust modular-framework/suite configuration additionally passes with
``BUG=y`` on both x86-64 and ARM64, for both proprietary C and Rust callers.
Each run repeats the 17 suite parameters after unload/reload, checks 71,272
positive inputs and six zero probes per caller load, and requires the exact
16 combined suite/caller warnings.

At this integer-log checkpoint, the complete minimum-Rust-1.85 regression
passes all 1,339 tests without skips. All 76 monitored active strict-build
configuration, provider, suite, binding, version and image artifacts retain
their exact contents and timestamps through the suite and true no-op builds.
Fresh DWARF audits cover 456 compilation units/7,074 unit CRCs on x86-64 and
403 units/6,903 unit CRCs on ARM64. Intermediate and final linked images agree
on all 7,074 and 6,902 exports, respectively. Independent original-C bridge
references match all 3,207 x86-64 and 3,160 ARM64 records.

The normal GCC x86 and LLVM ARM64 builds keep the new suite-language option
disabled and retain their symbol-version tables. After the one-time Kconfig
refresh, repeated builds preserve sizes and timestamps across all 6,611 and
4,205 regular output files, respectively.

Prime-number cache and KUnit integration
---------------------------------------

``CONFIG_RUST_PRIME_NUMBERS=y`` selects the repaired existing translation
without changing ``PRIME_NUMBERS=n/y/m`` or the ``prime_numbers.ko`` identity.
Explicit rules cover object, assembly and LLVM-IR targets; disabling the option
retains the original C source. The implementation uses actual native bindings,
flexible-array offsets, Kmalloc, a statically initialized native mutex, LKMM
atomics and RCU read guards. It preserves allocation before locking, the writer
recheck, immutable bitmap publication, deferred retirement, exit-only cleanup,
unrestricted exports and the original descending trial-division fallback.
The original ``slow_is_prime_number(1)`` quirk is preserved separately from the
public predicate, which excludes one. Checked rounding/layout overflow falls
back instead of reproducing the C wrap-to-zero allocation overrun.

The repaired public header exposes ``kernel::primes`` with either provider
language selected, including modular providers. Its inline facade does not
duplicate the cache or introduce provider imports into the built-in kernel
object. Inclusive iterators preserve the initial ``from`` value unchanged,
including zero, one and composites. The otherwise nonterminating ULONG_MAX
upper-bound case yields its sentinel once. All original source markers remain.

``CONFIG_RUST_PRIME_NUMBERS_KUNIT_TEST`` independently selects the translated
suite, consuming the repaired private header. It retains the complete original
65,534-input sweep, 6,542 next-prime checks, fatal binary assertions and their
printf arguments, RCU-protected suite-exit bitmap dump, module metadata and
optional packed printk-index records. Records remain present with
``PRINTK_INDEX=y, PRINTK=n`` as in the original macro.

Both the original C suite and the translated Rust suite have built and passed
with the Rust provider, modular KUnit and module unload/reload on x86-64 and
ARM64, using minimum Rust 1.85, CFI and symbol versioning. A separate x86-64
SMP/lockdep/debug-mutex configuration also passes the original suite. Six
provider groups and thirteen suite groups pass with GCC/Clang references and
genuine 32/64-bit execution. Their deterministic host allocation/RCU transport
checks algorithmic and lifetime boundaries; it is not a native concurrency
proof. Five real Kconfig/Kbuild provider groups also check source switching,
archive/module membership, parallel inspection targets, no-op rebuilds and
transitive dependencies.

The boot runner's optional ``--prepare-failslab`` mounts debugfs only inside
the isolated VM, writes and reads back scoped controls before any module load,
and rejects a missing, malformed or misplaced setup marker. With two CPUs
actually online, the debug x86-64 kernel passes two fresh-cache runs across
unload/reload: four real NULL allocations, two successful retries, a cached
nonallocating control and an old RCU reader held across publication and eight
same-size allocator churn allocations in each run. The original C suite also
passes after each stress run. This proves held-reader safety for that protocol,
not complete batched deferred reclamation. A previous ACPI-disabled guest
discovered only one CPU and was correctly rejected before creating workers.

All four C/Rust provider/suite combinations pass proprietary C and Rust
public-ABI callers on x86-64 and ARM64, both built-in and modular: 32 native
caller configurations, including caller unload/reload and complete dependency
reload for modular providers. Each caller load checks
65,536 primality values, 65,537 next-prime calls and twelve iterator cases.
Actual typed indirect calls retain native KCFI guards. The fixture preserves
the original slow fallback's next(0)=1 quirk independently from the public
cached next(0)=2 result; both complete caller bodies also execute against the
unchanged original C cache in the host regression. Built-in prime matrix boots
use the existing KUnit default suite filter. Separate final boots restore the
unfiltered configuration on both architectures: the prime suite, all 30
power/root parameters, 17 integer-logarithm parameters, 11 GCD parameters,
eight rational parameters and 16 polynomial parameters pass. The Rust
integer-logarithm ABI caller also passes unload/reload, with the exact expected
zero-input warnings distinguished from unrelated kernel diagnostics.

The original C suite also passes twice with the Rust provider under
PREEMPT_RT, SMP, lockdep and module unload/reload. Its static mutex therefore
uses the actual RT initializer and backend in native execution as well as the
non-RT paths above.

A native x86 load exposed four unversioned, relocation-free imports introduced
by C module metadata after modpost, not by the Rust owner. Final metadata
compilation now disables incidental header export/KCFI-addressability emission;
real owner compilation and generated KSYMTAB/CRC records are unchanged.
Private native-command comparisons on x86-64 and ARM64 preserve allocated
payload sections and normalized relocations, excluding ``.note.gnu.build-id``:
executable bytes, exports, CRCs, module information and owner KCFI symbols remain
unchanged. Seven focused metadata/configuration groups pass with GCC and Clang
and minimum Rust 1.85. Final ELF validation requires matching CRCs for every
resolved import and ``module_layout``; unreferenced orphan symbols receive no
exception. A fresh RT boot has no missing-version diagnostic.

The expanded stress protocol passes with both the original C and Rust
providers in the non-RT debug kernel and the Rust provider under PREEMPT_RT,
using the original C suite, two online CPUs and two fresh-cache runs each.
The non-RT configuration includes debug mutex checks; the RT configuration
uses its available RT lockdep/spinlock checks, not ``DEBUG_MUTEXES``.
The protocol also forces two
writers to allocate before publication. Writer A publishes first; writer B
resumes only after a guarded snapshot identifies A's allocation. Exactly one
native free cancels B's distinct allocation, with no matching A free during
the observation window; a final guarded snapshot still identifies A. Workers
are joined and tracepoints unregistered before unload. Registration and
unregistration stay outside the CPU-hotplug read lock, avoiding the lockdep
inversion caught by the first native run. This observes cancellation and
held-reader safety, not complete batched deferred reclamation.

Both languages also pass both ABI callers with the prime suite disabled on
both architectures, without conditional test exports. Disabling the provider
removes its archive members, module-order entries and public/private exports
and definitions from each native build. Both private modular configurations
have been restored to Rust provider and suite, with fresh proprietary Rust
ABI load/unload/reload checks passing.

Fresh strict-build DWARF audits agree between original C, translated Rust and
saved Kbuild output across 457 x86 compilation units with 7,079 CRC records,
and 404 ARM64 units with 6,908 CRC records. Relocatable and final kernel images
agree on all 7,079 x86 and 6,907 ARM64 exports. Fresh original-C bridge
references also match all 3,208 x86 and 3,161 ARM64 Rust bridge records.

The normal GCC x86 and LLVM ARM64 builds retain the default-off prime choices
and pass after configuration refresh. Subsequent true no-op builds preserve
every regular file's path, size, nanosecond timestamp and SHA-256 across
6,611 x86 and 4,205 ARM64 files.

Before the subsequent common-metadata integration, the expanded regression
passes all 1,419 tests without skips in 609.515
seconds with minimum Rust 1.85, ``-Dwarnings``, genuine i686 execution and
every configured native-artifact gate enabled. The non-default bindgen path
is passed as a make command-line assignment. An initial invocation supplied
it only in the environment and failed because Kbuild selected an unavailable
``bindgen``; the corrected complete run passes, with no source workaround.
Across the full run and subsequent true no-op builds, all 33,611 regular
files in the six strict, modular, debug and RT output trees retain their
paths, sizes, nanosecond timestamps and SHA-256 hashes. This checkpoint proves
the migrated components and their regression coverage, not the outstanding
whole-kernel translation.

A subsequent callback-ABI audit found that Rust 1.85 encodes the optional
callback argument of the private ``with_primes`` export differently for
KCFI. Its previous tests used direct calls; the protected public ``is`` and
``next`` calls are unaffected. The provider and generated-binding interface
now consistently use the original non-null callback contract. The actual
x86 native object's type ID matches C's ``0x5bdf0b2f``. All 25 focused provider,
suite and callback groups pass without skips, including real i686 execution,
protected LP64 C/Rust indirect callers and a wrongly nullable caller that
traps against both providers. A wrong inner callback likewise preserves the
ordinary ABI but traps under KCFI with either provider. Rust 1.85 does not
support i686 KCFI; its ELF32
coverage is ordinary indirect ABI execution, not a simulated KCFI target.
The full-suite checkpoint above predates this correction and the new groups.

The native stress fixture now routes every private snapshot call through a
typed volatile function pointer. Its compiled object and final module must
contain the actual provider's KCFI type check, conditional trap and indirect
call; legitimate split trap sections are individually validated. The updated
debug and PREEMPT_RT VMs both pass two fresh-cache runs, their original C
KUnit suite and unload/reload with this guard active, retaining every
allocation, held-reader and competing-writer assertion above. All 85 focused
prime provider, suite, build, runtime and stress groups pass without skips
with genuine i686 execution where supported.

Common module metadata
----------------------

``CONFIG_RUST_MODULE_COMMON`` selects the repaired existing
``scripts/module-common.rs`` independently of the module owner language.
The default remains the original C implementation. A Rust host generator
decodes preprocessed authoritative target headers into inline data; the
selected C frontend validates the original declarations with
``-fsyntax-only`` but emits no common metadata object. Rust emits the real
version magic, build-salt/LTO notes, optional retpoline record and ORC hash,
without owner exports, runtime imports or a module initializer.

The generator checks the target frontend's execution character set and
rejects unsupported non-UTF-8 execution strings explicitly, preserving the C
choice for those configurations. A non-UTF-8 input file converted by GCC to
UTF-8 execution strings still matches the original C object.

Ten differential groups pass with GCC and Clang references, minimum Rust
1.85 and genuine i686, x86-64, ARM64 and big-endian PowerPC objects. Fresh
x86-64 and ARM64 native modular builds select Rust metadata successfully;
their actual common objects match privately compiled original C records,
section flags and alignment, including the x86 GNU property note. Both native
VMs pass the proprietary Rust public-ABI caller, two selected KUnit runs and
dependency unload/reload. Eight new metadata-verifier groups and 113 combined
runtime-checker tests pass without skips; Rust metadata gets its own strict
source, dependency and ELF-data validation, not an exemption from the C
fixture's checks. Nine actual Kbuild groups additionally pass C-to-Rust-to-C
switching, independent C/Rust module ownership and host tools, separate
external working directories, merged syntax-validation/preprocessor
dependencies, source/core/configuration rebuilds, true no-op builds,
preservation of a previous good object after an error, and cleanup after the
selector is disabled. All 27 focused metadata groups pass without skips with
both GCC and Clang references. Native original-C object comparisons and both
VM unload/reload checks also pass after the execution-charset guard.
This option does not migrate per-module ``*.mod.c`` or final-vmlinux exports;
the latter now have the separate selection described below.

Before the subsequent final-vmlinux export integration, with the common
metadata integration and private callback correction included,
the complete regression passes all 1,456 tests without skips in 620.018
seconds. This run uses minimum Rust 1.85, ``-Dwarnings``, genuine i686 execution,
the explicit bindgen make argument and all configured native-artifact gates.
The eight strict, modular, debug, RT and normal compatibility builds also pass;
the current common-metadata VMs and guarded debug/RT stress runs are described
above. Across the full suite and true no-op builds, all 44,441 regular files
in these eight output trees retain their exact paths, sizes, nanosecond
timestamps and SHA-256 hashes. This is a component integration checkpoint,
not a completed Rust kernel.

Final-vmlinux export metadata
----------------------------

``CONFIG_RUST_VMLINUX_EXPORT`` independently selects Rust construction of the
final export tables and built-in aliases. It requires Rust support and defaults
off; the original C path remains selectable. A dedicated Rust modpost helper
also works with ``HOST_TOOLS_LANG=c``, without changing the ordinary host-tool
selection. Its explicit, distinct crate name prevents temporary-object
collisions when both Rust modpost executables compile concurrently.

The selected path generates ``.vmlinux.export.h`` for original target-frontend
syntax validation and field preprocessing, then ``.vmlinux.export.rs``. The
frontend emits no C object. The repaired ``export-internal_header.rs`` constructs
the actual PREL32 or native-width tables, symbol labels, CRCs and flags; Rust
also emits the normalized alias bytes. The original macro expansion and
stringification remain authoritative, including compiler/header macro
collisions, adjacent literals, counters and source-filename builtins. This is
not a passthrough of C-expanded table assembly. No module-owner marker,
initializer, export declarations or new version calculations are introduced.

The field parser requires UTF-8 input and execution strings and UTF-8 decoded
export names, references and namespaces; alias byte strings retain the
frontend's accepted escapes. Unsupported filename-prefix mappings fail with
retained-C guidance. GNU-as-only unquoted Unicode identifiers are not rewritten
to overcome LLVM assembler limitations and still require the C selection.
Strict direct export APIs remain distinct from trusted preprocessed fields.

Twelve export-header groups and 34 emitter groups pass with minimum Rust 1.85,
including genuine i686, x86-64, ARM64 and big-endian PowerPC object comparisons
and the existing-kernel-object gate. Ten actual Kbuild groups pass with both
GCC and Clang references, including C/Rust/C switching, missing-output
regeneration, transitive dependencies, external-module C routing, cleanup,
cold parallel Rust/Rust host-tool compilation and exact subsequent no-ops.
The minimal fixtures isolate inherited parent-kernel flags while retaining
explicit per-test flag and response-file probes. A poisoned-parent regression
executes the original top-level test recipe and the real nested C/Rust/C and
charset cases, covering the environment leak found by full-suite execution.

Native x86-64 and ARM64 builds select the Rust path successfully with strict
warnings and KCFI retained on their owners. Fresh private original-C modpost
and target-compiler references match all 7,079 and 6,907 exports respectively:
section contents, flags, alignment, symbol labels, relocations and all 8/41
built-in aliases. Full ``Module.symvers`` contents are unchanged by selection.
Both VMs boot and pass their unfiltered KUnit suites and the proprietary Rust
integer-logarithm ABI caller, including 71,272 positive inputs and six zero
probes per load, expected warnings and module unload/reload. The other six
modular, debug, RT and normal compatibility builds also pass with the selector
disabled. This does not establish native boot coverage for every Rust-enabled
architecture. Per-module metadata was outside this checkpoint; its new
independent selection is described below.

After the parent-environment fixture correction, the expanded regression passes
all 1,512 tests without skips in 773.920 seconds, using minimum Rust 1.85,
strict warnings, genuine i686 execution and all configured native-artifact
gates. Across both full-suite attempts and the eight no-op builds, all 44,453
regular files in the eight native output trees retain their exact paths,
sizes, nanosecond timestamps and SHA-256 hashes. This verifies the integrated
components, not the remaining translated kernel sources.

Per-module loader metadata
--------------------------

``CONFIG_RUST_MODULE_METADATA`` independently selects typed Rust ``*.mod.o``
objects and defaults off. Original C remains selectable, independently of
``HOST_TOOLS_LANG``, common metadata and final-vmlinux exports. The original
target frontend validates generated ``*.mod.h`` declarations and preprocesses
their fields; it emits no metadata object. Rust uses the actual generated
``bindings::module`` and ``modversion_info`` layouts, preserving lifecycle
relocations, export records, basic/extended symbol versions and module info.
This does not introduce another module owner or recalculate owner CRCs.

The 35 integrated fidelity/build/cleanup groups pass without skips using
minimum Rust 1.85, genuine i686 and native x86-64/ARM64 inputs; object checks
also cover big-endian PowerPC and little-endian PowerPC64. Six additional
protocol groups cover required guards, malformed records and output failure.
The data-only Rust statics have distinct mangled global symbols, unlike C's
local statics; tests check exact permitted families and collision-free
parallel linking, not whole-symbol-table identity. Natural layout/alignment
is checked rather than optional C array over-alignment. Nonempty
``MODULE_ARCH_INIT`` and unsupported execution encodings explicitly require
the retained C selection.

Strict native x86-64 and ARM64 modular builds pass with common and per-module
metadata selected. Two further native builds pass with final-vmlinux and
per-module metadata selected, leaving common metadata in C. The shared runtime
checker validates the selected typed metadata rather than stale ``*.mod.c``
artifacts. Twenty-one integrated groups include real native artifacts and
semantic negative controls. Lifecycle fields are compared independently to
the actual implementation definitions, so consistently omitting cleanup from
both generated Rust and linked ELF cannot satisfy the checker. Unchanged
metadata may legitimately predate a rebuilt owner because modpost uses
write-if-changed; timestamps alone do not establish semantic staleness.
Unrelated updates to ``auto.conf`` likewise do not imply staleness: the checker
uses the actual recorded header and fine-grained CONFIG dependencies. Separate
controls accept unrelated configuration changes and reject changed recorded
dependencies. All twenty-one groups pass after the list-sort selection matrix
and refreshed external Rust prime callers on both architectures.
The combined metadata build/fidelity/cleanup/runtime and downstream consumer
regression subsequently passes 285 tests without skips in one process.

Both architectures pass proprietary C and Rust prime callers, ordered loading
of provider/framework/suite modules, and unload/reload with selected Rust
per-module/common metadata. Each caller load checks 65,536 primality inputs,
65,537 next-prime inputs and 12 iterator cases; each run executes the selected
KUnit suite twice. Both main kernels also pass Rust integer-log callers with
final-vmlinux/per-module Rust metadata: 71,272 positive inputs, six zero probes
and exact expected warning checks per load, plus unload/reload. These checks
do not replace the separate allocation-failure/RCU stress configurations.

Linked-list sorting
-------------------

``CONFIG_RUST_LIST_SORT`` selects the repaired existing ``lib/list_sort.rs``
through a distinct native owner, using real ``bindings::list_head`` and the
original explicitly nonnull C comparator contract. Both provenance markers
remain unchanged. The default is C, and either selection retains the original
archive position between ``scatterlist.o`` and ``uuid.o``. Native Rust DWARF
changes the symbol-version CRC: dependent modules must be rebuilt when
switching implementations.

Eight integrated groups pass without skips, including genuine ELF32/ELF64
execution, original-C callback/link traces, stable ties, KCFI positive and
trapping negative controls, actual Makefile selection/dependencies/no-ops,
and read-only audits of both selected native kernels. Native x86-64 and ARM64
builds and boots pass with the original C KUnit suite calling the Rust
provider. The complete boot logs contain 97 and 90 passing KUnit cases
respectively, without failures or skips. Subsequent external proprietary C and
Rust callers both pass 6,435 cases per load on both architectures, including
unload/reload, exact comparator/link traces, stable ties and bidirectional
list traversal. Twenty-one integrated runtime checker groups pass.

``CONFIG_RUST_LIST_SORT_KUNIT_TEST`` independently selects the repaired
existing Rust suite while retaining ``CONFIG_TEST_LIST_SORT`` and the original
``test_list_sort.o/.ko`` identity. Original assertions, diagnostic operands,
allocation flags/order, random-byte rejection sampling and KUnit ownership are
preserved. Five integrated groups pass without skips, including genuine ELF32
execution and minimum-Rust native compilation/KCFI checks on x86-64 and ARM64.
The deterministic failure transport is not native FAILSLAB coverage. Native
modular validation now passes all four independent provider/suite selections
(C/C, C/Rust, Rust/C and Rust/Rust) on x86-64 and ARM64: eight image/module
builds and sixteen VM runs with both C and Rust external callers, 6,435 cases
per load and ordered framework/suite unload/reload. Both outputs were restored
to Rust/Rust. Built-in execution of the translated suite is still unproven;
the earlier built-in VM evidence used the original C suite.

Generic software Hamming weight
-------------------------------

``CONFIG_RUST_HWEIGHT`` selects the repaired existing ``lib/hweight.rs`` through
a sole native export owner, in the original generic object's archive slot.
The default remains C. All four unsigned interfaces and unrestricted exports
are retained, including full unsigned-int inputs to the 8/16-bit algorithms
and a native unsigned-long result for the 64-bit input. Both configured
multiplier paths preserve the original wrapping arithmetic.

Nine integrated groups pass without skips: original-C differential matrices
at O0/O2/Os with checked/unchecked arithmetic on genuine ELF32/ELF64, real
native x86-64/ARM64 bindings/KCFI/export/DWARF checks, semantic and ABI negative
controls, and actual Kbuild switching/archive/dependency/no-op tests. Native
compiler flags are taken from an always-C object; a regression repeats the
proof with the unselected C objects' saved commands absent.

Architecture-specific assembly remains under its existing selection. In
particular, native x86's stronger register-preservation convention is not
replaced by an ordinary Rust C-ABI function; generic UML is a separate case.
An ARM64 image and modules now build with the generic Rust owner selected;
the linked archive has only ``hweight_rust.o`` and the image retains all four
unrestricted exports with genuine Rust-derived versions. That kernel boots
and passes the Rust list-sort consumer/KUnit load-reload smoke test.
Fifteen dedicated runtime-checker groups pass without skips. Independent
native C and Rust proprietary callers then each pass 593,408 input values
and 2,373,632 protected export comparisons per load on ARM64, including
unload/reload. The full C/Rust-provider by C/Rust-caller VM matrix passes and
the Rust provider is restored afterward. Retained native x86 assembly also
passes its separate artifact/defining-type/version audit; that audit is not
misrepresented as a generic four-export x86 VM run.

Base64 and UUID/GUID
-------------------

``CONFIG_RUST_BASE64`` and ``CONFIG_RUST_UUID`` independently select the
repaired existing translations through sole native owners, retaining the
original archive slots, C defaults, exported/static symbols and export
licenses. Existing translated headers now distinguish actual native binding
interfaces from safe Rust helpers. All original provenance markers remain.

Base64 retains all three alphabets, padding/trailing-bit rules, overlapping
raw-buffer behavior and partial writes. Defined negative-length paths match
C instead of being replaced with undefined behavior. Its nominal enum uses
the existing bindgen ``cfi_encoding`` mechanism: a plain integer alias has
the same ordinary ABI but the wrong KCFI type. Sixteen integrated groups pass
without skips, including genuine ELF32/ELF64, protected C/Rust consumers,
actual-header/native flags, real Kbuild/dependencies and an original-C corpus
of 964,152 records per full differential executable. Independent overlap
review checks a further 18,568 cases per executable.

UUID/GUID uses genuine binding structs, original early validation and
overlapping parse order, and the actual kernel RNG with original mask order.
Initializer macros preserve C integer promotions and repeated evaluation for
narrow, signed and wide operands. Eleven integrated groups pass without
skips, including genuine ELF32/ELF64, 9,216 position/byte mutations, 61 overlap
offsets, 36 guard-page prefix cases, 1,024 deterministic RNG-fixture cases,
native KCFI/export/DWARF and real Kbuild/dependency/no-op checks. These
deterministic RNG fixtures do not replace native kernel RNG execution.

Both components' native tests also work when the unselected original C
object and saved command are absent. Explicit invalid inputs fail rather
than skip. Strict native x86-64 and ARM64 images/modules build with both Rust
providers and their original C KUnit suites selected as modules.

Dedicated Base64 and UUID runtime checkers are integrated: respectively 21
groups (5.433 seconds) and 22 groups (8.827 seconds) pass without skips. Both
languages own their complete native consumer loops, with unchanged original C
used only as the algorithm oracle (and Base64 case data). Compiled UUID
constant-result and C-dispatch-delegation controls are rejected. Actual module
metadata, defining/imported versions, protected call sites, licenses, exact
KUnit results and unload/reload ordering are checked. The full C/Rust-provider
by C/Rust-caller matrix passes on both architectures: four kernel builds and
16 VM runs across both components, with their original C suites loaded twice
per VM. Rust providers are restored afterward. Each Base64 load checks 154,328
cases; each UUID load checks 9,217 parser inputs across 61 overlap offsets,
1,024 real RNG calls and both exported null values. This completes the two
providers' dedicated native gates, not the new full regression or every
configuration. The conservative lib inventory becomes 28 native units plus
three host generators (31/526); these overlap the separate host-tool inventory.

The translated UUID suite is independently selectable with
``CONFIG_RUST_UUID_KUNIT_TEST``, default off, preserving ``UUID_KUNIT_TEST`` y/m
and the original ``uuid_kunit`` object/module identity. All eight cases, 82
nonfatal expectations, 12 parse calls and 32 generation calls are retained,
using actual KUnit assertions/formatters and nominal UUID/GUID bindings.
Eight integrated groups pass without skips (10.289 seconds), including actual
ELF32/ELF64 traces, fault injection, protected callbacks, native x86/ARM64
compilation, module/builtin metadata and real Kbuild selection/dependencies.
The translated suite now also passes four native builds and eight VM runs,
covering both C/Rust providers and C/Rust callers on x86-64 and ARM64, with
all eight cases run twice per VM through module unload/reload. Together with
the original-suite matrix, UUID covers all provider/suite/caller language
combinations on both architectures. Rust provider and suite are restored.
Builtin execution of the translated suite subsequently passed on both
architectures with C and Rust consumers, including consumer unload/reload.
The new full regression remains outstanding. Adding this natively executed
test unit makes the conservative
lib inventory 29 native units plus three host generators (32/526); the
separate host-tool inventory overlaps these counts.

The translated Base64 suite is now independently selectable through
``CONFIG_RUST_BASE64_KUNIT``, default off, retaining ``BASE64_KUNIT`` y/m and
the original ``base64_kunit`` object/module identity. All four cases, vector
order, assertions and the two 1,000-iteration timing workloads are retained.
Allocation uses the existing kernel Rust allocator path; fatal allocation
assertions preserve the original three allocation attempts and abort behavior.
Exact original-C traces, real nominal bindings, protected callback dispatch,
printk-index records, ELF32/ELF64 execution and actual Kbuild switching are
covered by eleven integrated groups. Both GCC and Clang reference runs pass
without skips (72.737 and 76.549 seconds). The translated modular suite then
passed the C/Rust-provider by C/Rust-consumer matrix on x86-64 and ARM64:
four native builds and eight VM runs, with suite unload/reload. A further
builtin checkpoint covering Base64, UUID and list_sort passed two native
builds and twelve VM runs with Rust providers/suites and both caller languages.
These are retained pre-reboot results, not new executions after restoring logs.

The strengthened Base64 checker subsequently passed 33 groups without skips
(13.726 seconds). Its callback KCFI identity comes from the selected original
KUnit framework object and header prototype; a common nonzero identity alone
is insufficient. Actual C/Rust builtin/module objects, wrong nominal callback
types, disabled CFI, metadata/common dependencies and private compiler-output
isolation are covered. The strengthened builtin selected-artifact audit passed
on both architectures. The earlier modular VM matrix predates the strengthened
metadata/common guard, so that guard must be rerun on rebuilt modular outputs.
That rerun now passes after the reboot: rebuilt strict Rust 1.85 x86-64/ARM64
outputs pass all eight C/Rust-provider by C/Rust-caller VMs, with the translated
four-case suite loaded twice per VM under modular KUnit. The same fresh matrix
passes for UUID's translated eight-case suite. Two shared metadata-checker
defects were corrected: response paths resolve from the compiler's working
directory, and freshness follows fixdep's actual configuration dependencies
instead of unrelated changes to global auto.conf. Fourteen focused metadata
groups retain wrong-path and genuinely stale-input rejection. Both builds are
restored to Rust and their selected-artifact audits pass. The Base64 suite adds
one validated unit (33/526); the new complete regression is still outstanding.

Parser, command-line, memory-weight, min/max and glob candidates
--------------------------------------------------------------

``CONFIG_RUST_PARSER``, ``CONFIG_RUST_CMDLINE``, ``CONFIG_RUST_MEMWEIGHT`` and
``CONFIG_RUST_WIN_MINMAX`` independently select repaired translations while
retaining the original C selections and archive positions. Parser and min/max
types are generated from their unchanged C headers. Original SOURCE-COMMIT
markers and export licenses are preserved; modules must be rebuilt after a
language change because genuine Rust DWARF symbol versions can differ.

The parser retains all nine APIs, conditional/partial capture writes, numeric
errors and real kernel allocation. Command-line helpers preserve pointer/write
order and integer wrapping, including count-only ranges. Memory weight keeps
the leading-byte scan before the original BUG threshold and the big-endian
tail contract. Windowed min/max retains wrapping timestamps, exact comparison
edges and the C-defined partial-initialization domain without creating a Rust
reference to an incompletely initialized tracker.

``CONFIG_RUST_GLOB`` selects the allocation-free, lazy matcher when the original
GLOB bool is enabled. Immediate mismatch and trailing-star completion do not
scan the remaining input. Selected object, assembly and LLVM-IR targets retain
the Rust source and dependency tracking; C remains selectable.

Integrated tests pass without skips for parser (eight groups), cmdline (seven),
memweight (six), min/max (seven) and glob (five), including genuine ELF32
execution. Parser/minmax additionally audit the final selected x86/ARM64
kernel objects, archives, bindings and symbol versions. The first full native
build exposed a parser type/function name collision hidden by a type-only
fixture; a type alias preserves the real nominal binding, and a compiled
negative control now rejects the old duplicate import.

Both architectures now build with all five Rust providers selected. Original
C cmdline and glob KUnit modules pass in two preflight VMs, each loaded twice:
eight cmdline cases and 64 glob parameters per load, with exact successful
case/order/summary and unload/reload checks. Dedicated independent C/Rust
consumer matrices and the refreshed complete regression remain outstanding.
The parser's dedicated checker has since been integrated: nineteen focused
groups pass, including actual native compiler controls and both selected Rust
artifact audits. Its full C/Rust-provider by C/Rust-caller matrix now passes on
x86-64 and ARM64, eight fresh VMs with consumer unload/reload. Each load executes
1,495 groups across all nine public APIs, with independent caller loops, original
C as oracle, real kernel allocation, selected symbol versions and protected
indirect calls. Rust providers are restored and final audits pass. Adding this
provider makes the conservative lib inventory 31 native units plus three host
generators: 34/526 (6.46%), leaving 492 units. The separate host inventory remains
55/64 (85.94%), leaving nine; its three lib generators overlap these counts.
There is no validated whole-kernel percentage. At that parser milestone the
remaining four providers' dedicated native matrices were still pending, along
with the refreshed complete regression.

The integrated memweight runtime checker subsequently passed fourteen groups,
and its noncrashing provider/caller matrix passed eight fresh x86-64/ARM64 VMs.
Each independent caller performs 12,651 protected original-C comparisons per
load, including real 128 MiB allocation and input-integrity checks; unload/reload
runs the workload twice. Both outputs are restored to Rust and audited. Its
older differential/build suite also passes seven groups without skips with
isolated compiler scratch space and official Rust 1.85 i686 libraries, including
ELF32 O0/O2/Os execution. A focused combined regression passes 243 tests without
skips. Independent boundary review found no implementation discrepancy: the
original alignment scan, threshold comparison and real BUG-helper binding are
preserved. Host tests cover sixteen original-C cases immediately below the
threshold and reject early-guard, off-by-one and signed-narrowing mutations.
Actual exceptional BUG dispatch in a target VM remains untested; this is
additional assurance, not a discovered functional defect or an original
self-test requirement. With the ordinary native matrix and restored-provider
audit complete, memweight adds one validated unit: 35/526 (6.65%), 491 remain.
This count does not claim exhaustive path/configuration coverage.

Windowed min/max and glob now also pass their complete C/Rust-provider by
C/Rust-caller matrices on both architectures: eight fresh VMs per component,
each unloading/reloading the consumer. The min/max caller executes 136,840
checks per load, including wrapping timestamps and valid partially initialized
states. The glob caller executes 95,537 comparisons per load; its unchanged C
KUnit module runs all 64 original parameters twice per VM. Glob additionally
passes a real proprietary-consumer modpost check. Both Rust providers are
restored and their final selected-artifact audits pass. Integrated checker
groups pass 17/17 for min/max and 21/21 for glob, without skips, with genuine
ABI/CFI controls and concurrent compiler-output isolation checks. These two
providers bring the conservative lib inventory to 34 native units plus three
host generators: 37/526. The new complete migration regression remains pending.

Command-line now also passes all eight C/Rust-provider by C/Rust-caller VMs on
x86-64 and ARM64, with 33,411 original-C comparisons per consumer load and
unload/reload. Its unchanged eight-case KUnit module runs twice per VM. Both
outputs are restored to Rust and the final linkage/bindings audits pass.
Nineteen integrated checker groups pass without skips, including independent
coverage of the non-exported fifth helper, actual framework-derived callback
types, strict compiler controls and compiler-output isolation. External-module
saved paths are checked relative to their actual M= compiler working directory;
in-tree binding dependencies remain relative to O=. This adds one validated
unit: 35 native units plus three host generators, or 38/526 (7.22%), with 488
remaining. The resumed runtime cycle totals 56 successful VMs across seven
components, not proof of every architecture/configuration or full regression.

Remaining integration
---------------------

Most of the migration is still outstanding. In particular:

* The ``nconf``, ``mconf``, ``gconf`` and ``qconf`` Kconfig front ends still
  select their original C/C++ objects. Remaining host translations need
  per-file implementation and build-selection audits; the already selectable
  command-line host tools do not prove all host code has been migrated.
* Outside the opt-in integrations described above,
  target-kernel C objects still take precedence over adjacent Rust files.
  Their Rust definitions, shared types, configuration handling, exported
  symbols, and module boundaries must be repaired before selecting them.
* Some translated files omit core behavior. For example,
  ``init/main.rs::start_kernel`` currently panics instead of initializing
  and starting the kernel.
* Architecture build files and translated selftests need integration and
  behavioral verification.
  The polynomial, integer-power, integer-root, GCD, rational,
  integer-logarithm and prime-number suites cover the current
  ``lib/math/tests/Makefile`` entries; tests elsewhere still need their own
  translation, binding and native lifecycle audits. The Rust division
  self-tests described above are a separate, already selectable integration.
* Per-module loader metadata now has an independent Rust selection and native
  module load/reload checks on x86-64 and ARM64. Unsupported
  architecture initializers/encodings retain C; object-level cross-target
  checks do not establish boot coverage on every Rust-enabled architecture.
* A successful mixed C/Rust build validates the migrated components only.
  Full Rust kernel linking, booting, and the applicable kernel/selftest
  suites remain required before declaring the translation complete.
