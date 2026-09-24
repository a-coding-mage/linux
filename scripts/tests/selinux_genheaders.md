# SELinux genheaders host-tool tests

Run from a source tree with `HOSTRUSTC` selecting the compiler to validate:

```sh
HOSTRUSTC=rustc python3 -m unittest discover -s scripts/tests \
    -p test_selinux_genheaders.py -v
```

The suite uses `ROOT = Path(__file__).resolve().parents[2]`, creates its own
temporary source/output trees, and never builds into the input source tree.
`TMPDIR` can restrict all temporary output to a private workspace.
`SELINUX_GENHEADERS_REFERENCE_ROOT` supplies unchanged files missing from a
private overlay. Omit it after integration. `SELINUX_GENHEADERS_TEST_LOG` names
an optional append-only command/result log, including raw diagnostic bytes and
SHA256 hashes of differential output files. No external Python packages are
required. `HOSTCC` and `HOSTRUSTC` support shell-style command arguments.

The suite evaluates the actual root Makefile's `rust_common_flags`,
`KBUILD_HOSTRUSTFLAGS`, and compiler-version-dependent flags in a private make
fragment, with `HOSTRUSTFLAGS=-Dwarnings`. It sets `RUSTC_BOOTSTRAP=1` for the
existing Kbuild `-Z` options on the supported stable compiler. This includes
`-Dunsafe_op_in_unsafe_fn`, not a reduced standalone flag set.

Differential tests compile the unchanged original C and candidate Rust. They
compare exit/signal status, stdout/stderr bytes, and both generated files. The
official tables are the policy ABI oracle. Private replacement table fixtures
exercise SID holes and array length, class/permission ordering, 32 permission
bits and the 33rd-permission diagnostic, empty names/tables, C NULs, socket
suffixes, long names and negative printf widths, and deterministic unusual
ASCII names. Two compiling mutations (permission order and SID placement) must
be rejected by the differential comparison.

CLI tests cover invalid UTF-8 argv[0] and output paths, missing/empty/directory/
overlong paths, ENOTDIR and permission denial, extra ignored arguments, flag-looking filenames, same output
path, truncation, output creation order, both `/dev/full` close failures, and
broken stdout pipes with both inherited default and ignored SIGPIPE. Device
tests explicitly skip if their required Linux device paths are absent.

The finite-file-size matrix tests 35 limits (0 through 107027, including 65535,
buffer boundaries, and both official output lengths), inherited default/ignored
SIGXFSZ, and both regular outputs together or each isolated with `/dev/null`:
210 combinations with exact status, diagnostics and partial-file comparison.
Another 32 cases combine permission overflow with file limits and both signal
dispositions. The 65535 ignored-SIGXFSZ regression explicitly requires status 0,
empty stderr, and 65535 permission bytes, matching C despite a failed write.
A compiling fragmented-writer mutation passes ordinary bytes but must fail
that regression. A Linux/glibc test-only allocation interposer fails each of
11 uppercase allocations across all generation phases, then tests success at
the next ordinal; raw program-name diagnostics, status 3 and partial bytes match.

Kbuild tests copy the actual `Makefile.build`, `Makefile.host`, supporting make
files, and component Makefile into temporary trees. A private original-C
`fixdep` is only a test prerequisite. Both cold `-j4` C/Rust/C and Rust/C/Rust
sequences build the tool and generated headers, compare against original-C
golden bytes, inspect recorded dependencies/flags, verify real header-change
rebuilds and no-op mtimes for the executable and generated headers, ignore
changes to unselected-language headers, and repeat with poisoned parent make variables.
Compiling language-specific markers prove which executable actually runs.
A fake compiler returning zero without an executable cannot pass the artifact
gate. These are actual component Kbuild executions, not dry-run predictions.
They do not claim that the shared root aggregate has already been integrated.

## Translation contract

The command line accepts two output paths and ignores further arguments, as C
does. There is no runtime table parser, identifier grammar, or option grammar.
Flags such as `--help` are ordinary filenames when two paths are supplied.
Paths and diagnostics stay as raw C strings, including invalid UTF-8.

The imported headers are maintained source translations: class order assigns
class numbers, permission order assigns bits, slices replace C sentinels, and
`Option` entries preserve every initial SID hole including slot zero. The
current maintained names are ASCII, as are the source fixtures. No extra
runtime validation or arbitrary length limit is imposed by the generator.
Padding counts bytes and implements the absolute value of C's signed dynamic
width for class names longer than 39. Names can be empty, contain punctuation,
or contain a C NUL prefix terminator; the tests distinguish these from valid
policy identifiers rather than silently rejecting them.

The original shared header's `append` helper already constrains common
permission groups to 32 at compile time. That existing maintained-source
invariant is not a runtime CLI rule. A private fixture bypasses the helper to
prove that the generator itself reports C's 33rd-permission error and flushes
the same partial file. Both existing translated headers remain unchanged.

The translation does not promise behavior for fictional tables with more than
`INT_MAX` entries/bytes, C undefined behavior (such as `toupper` of negative
non-EOF signed chars), or platform-dependent oversized C initializers. Those
are not inputs to this host tool. Raw non-UTF-8 *paths* remain fully supported.

Generation uses idiomatic Rust table iteration and one fallible malloc-backed
uppercase owner per original `stoupperx`, preserving allocation ordering and
status 3. Slice iteration fills the allocation, and Drop frees it at the same
scope boundary as C. No lossy UTF-8 conversion, transmute, static work buffer,
or third-party dependency is used.

Each output `fprintf` has the original format, argument types, and call boundary.
This is essential: fragmented writes can produce identical partial bytes but
different libc buffer state and `fclose` results after failed flushes. Individual
write results remain ignored; errors are never cleared, and both original close
checks remain. The overflow diagnostic uses one `fprintf` with precision-bounded
`%.*s` slices instead of `%s`, avoiding new allocations for unterminated Rust
table strings on that error path. Its bytes/status are tested, including limits.
A C ABI `main` preserves inherited SIGPIPE/SIGXFSZ, and libc exit flushes streams
even on permission overflow or uppercase-allocation failure.

Validated host scope is Linux x86-64 with the supplied Rust 1.85 toolchain and
system libc/C compiler. The allocation test depends on glibc's `__libc_malloc`
and excludes larger FILE/buffer allocations using the known small fixture sizes;
it is not exhaustive startup/libc exhaustion testing. Other operating systems/
libcs and whole-tree/native kernel/VM gates remain separate validation.
The source contract assumes ASCII-compatible C execution encoding. Arbitrary
C frontend flags (notably EBCDIC literals on an ASCII libc host) are not reproduced
by the maintained Rust translation. Neither tool calls `setlocale`; changing
locale environment names does not activate a non-C locale. Table translations
must be updated with future C header changes; Kbuild tracks selected-language
dependencies, not automatic semantic synchronization of the two sources.
