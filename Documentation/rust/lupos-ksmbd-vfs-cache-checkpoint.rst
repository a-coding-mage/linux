Lupos ksmbd file-cache source checkpoint
======================================

This is a reviewed, UNBUILT source reconstruction of fs/smb/server/vfs_cache.c.
It is not native ABI validation, runtime parity, or deployment acceptance.
CONFIG_RUST_KSMBD_VFS_CACHE is default-off; the original C provider and all
original tests are retained.  Selection preserves vfs_cache.o in ksmbd.o.

Source provenance
-----------------

The complete source was reconstructed against commit
2099a6c20c54c99ae1689eb59f5240c54cc2774e.  The original vfs_cache.c SHA-256 is
bb68967cc60898ea73252eec62d0d60a295edf5e99c999e5915d1710b27aaa63.
The complete reviewed source-archive SHA-256 is
3083735bbd35808a0f88544da7ac384f472032210ed19eed3397fe5090828480.
At the first checkpoint, the nine unit source/integration files matched that
archive byte-for-byte. The follow-up below changes the root Rust file and its
binding header; the archived original checkpoint remains unchanged.
The separately reviewed rust/Makefile include is composed onto the existing
parent file without replacing its other provider includes.

All 82 distinct function names defined by the retained C unit have Rust
implementations, including procfs, inode hashing, delete and stream state,
ID lookup/removal, full close, durable scavenging and owner/reconnect paths.
The implementation uses configured bindings from the actual kernel and ksmbd
headers.  The C companion supplies lock initializers and thin macro/inline
boundaries; none of the cache algorithms falls back to the original C unit.

Source review checked inode and handle lifetimes, persistent-ID invalidation
before reuse, FP_NEW opener ownership during teardown, session unpublication
before durable preservation, strong connection references, scavenger disposal,
stream-specific deletion, and one-at-a-time notify cancellation ownership.
Separate original RCU-list diagnostic sites retain separate once-state.
Independent review found and repaired an inline-only seq_puts boundary.
Generated recipes propagate producer/formatter failures before fixdep.

Validation status and original C test route
------------------------------------------

At the first checkpoint, only textual function coverage, source checksums,
independent source review and patch applicability had been checked; configured
bindgen, compiler, make and native/link checks had not yet run. The later native
attempt and its failures are recorded below. No server, guest, authentication
operation or runtime test has run. Source checks do not establish parity.

The retained Linux server Makefile has no vfs_cache-specific KUnit/selftest
target.  Samba's maintained C smbtorture SMB2 tests are the protocol-test
route.  Pin a Samba revision and retain its original tests and build system:
source4/torture/smb2/wscript_build registers its C test files, and the original
top-level Makefile supports make bin/smbtorture through the upstream Waf build.
Enumerate the built binary's exact suite names before selecting tests.
The generic Samba make test target normally provisions Samba environments;
it is not evidence that the tested server was the ksmbd provider under review.

Run unchanged durable_open.c and durable_v2_open.c test families against the
approved isolated ksmbd server, together with delete-on-close.c, streams.c,
sharemode.c, lock.c, notify.c, notify_disabled.c, oplock.c, lease.c, replay.c
and session.c as supported by the pinned suite.  Compare matched retained-C
and Rust kernels, record kernel/provider and test-binary identities, and keep
all original assertions, skips and failures visible.

Upstream test and build references:

* https://github.com/samba-team/samba/blob/master/source4/torture/smb2/wscript_build
* https://github.com/samba-team/samba/blob/master/source4/torture/smb2/smb2.c
* https://github.com/samba-team/samba/blob/master/Makefile

Before acceptance, verify configured layouts and callbacks/KCFI, built-in
and module linking, provider-off behavior, and actual runtime ownership.
Protocol tests alone do not prove deterministic FP_NEW teardown races,
persistent-ID reuse, scavenger-versus-inode lookup, every notify CANCEL/close
interleaving, allocation and IDR failures, credential restoration on xattr
errors, timeout wrap, procfs output, or architecture/configuration coverage.
Those gaps remain open; no source-only result should be counted as a pass.

This checkpoint changes no CI file, request or workflow and does not request
another CI execution.  The previously approved CI subtree is inherited
unchanged from its parent.  Original C and maintained test bytes are unmodified.

First native module-object attempt and pending correction
--------------------------------------------------------

At source 47b0c36cddd7754a08b4ee64d7fc5638c6d280ea, the original C cache provider,
five original C callers and native ksmbd.o module composite compiled/linked;
their identical-command repeats preserved whole-output hashes and artifact
timestamps. The common fixture used MODULES=y, SMB_SERVER=m and RUST=y.
The C-to-Rust configuration difference was only RUST_KSMBD_VFS_CACHE=n to y.

Configured bindgen succeeded, but Rust provider compilation failed with 51
errors. They identify three source assumptions: platform core::ffi aliases
instead of the kernel's configured aliases, an ungenerated KSMBD_NO_FID macro,
and access to dentry.d_name without its generated anonymous-union layer.
This follow-up uses kernel::ffi, casts C string pointers to the configured char
pointer type, exports the actual C constant expression through RVC_NO_FID,
and uses the emitted dentry union field. Algorithms and original C are unchanged.

The correction is not yet compiled or runtime-qualified. Rust helper/caller/
composite ownership audits and Rust no-op checks were not reached. The earlier
Rust object command used canonical RUST_MODFILE=fs/smb/server/vfs_cache with
MODULE; a proposed observer's ksmbd value was an unrun assumption, not a kernel
ABI finding. Native validation must resume from a coherently reviewed new pin;
no successful Rust build, module load or runtime parity is claimed here.
