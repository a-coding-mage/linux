#!/usr/bin/perl
# SPDX-License-Identifier: GPL-2.0-only
# Historical boot-smoke-v1/verify.pl checks, with CI path/identity plumbing.
# The caller seals boot-context.json BEFORE launch and boot-serial.sha256 AFTER
# guard completion, before this verifier. No process supervision is added here.
use strict;
use warnings;
use File::Find qw(find);
use Fcntl qw(:mode);
use JSON::PP;
use Digest::SHA qw(sha256_hex);
my ($serial_file,$guard_file,$source_file,$release_file,$input_file)=@ARGV;
die "usage: verify-boot.pl SERIAL GUARD SOURCE RELEASE INPUTS\n" unless @ARGV==5;
my $run='/work/evidence';
my $work='/work/O/boot-inputs';
die 'Unexpected CI input paths' unless
    $serial_file eq "$run/boot.log" && $guard_file eq "$run/boot.status" &&
    $source_file eq "$run/source.commit" && $release_file eq '/work/O/include/config/kernel.release' &&
    $input_file eq "$run/boot-inputs.sha256";
sub read_text {open my $f,'<',$_[0] or die "$_[0]: $!"; binmode $f; local $/; return <$f>;}
sub hash_file {open my $f,'<',$_[0] or die "$_[0]: $!"; binmode $f; return Digest::SHA->new(256)->addfile($f)->hexdigest;}
sub manifest {
    my ($path)=@_;
    my %rows;
    my $data=read_text($path);
    die "Empty or unterminated manifest: $path" unless length($data) && $data=~/\n\z/;
    for my $line (split /\n/,$data) {
        $line=~/\A([0-9a-f]{64})  (\/[!-~]+)\z/ or die "Malformed manifest: $path";
        my ($digest,$name)=($1,$2);
        die "Unsafe manifest name: $name" if $name=~m{(?:\A|/)\.\.?(/|\z)|\\|//};
        die "Duplicate manifest entry: $name" if exists $rows{$name};
        die "Unexpected manifest root: $name" unless $name=~m{\A/(?:work/(?:O|evidence)/|pilot/|usr/)};
        die "Manifest target not regular: $name" unless -f $name;
        die "Input changed: $name" unless hash_file($name) eq $digest;
        $rows{$name}=$digest;
    }
    return \%rows;
}
my $source=read_text($source_file);
my $release=read_text($release_file);
$source=~/\A([0-9a-f]{40})\n\z/ or die 'Malformed full source SHA'; $source=$1;
$release=~/\A([A-Za-z0-9._+-]+)\n\z/ or die 'Malformed kernel release'; $release=$1;
my $context=decode_json(read_text("$run/boot-context.json"));
die 'Malformed preboot context' unless ref($context) eq 'HASH' &&
    join(',',sort keys %$context) eq 'immutable_inputs_sha256,kernel_release,source_sha' &&
    ($context->{source_sha}//'')=~/\A[0-9a-f]{40}\z/ &&
    ($context->{kernel_release}//'')=~/\A[A-Za-z0-9._+-]+\z/ &&
    ($context->{immutable_inputs_sha256}//'')=~/\A[0-9a-f]{64}\z/;
die 'Immutable ledger differs from preboot context' unless
    hash_file($input_file) eq $context->{immutable_inputs_sha256};
my $inputs=manifest($input_file);
for my $required ($source_file,$release_file,'/work/O/.config','/work/O/arch/x86/boot/bzImage',
    '/work/O/usr/gen_init_cpio','/pilot/boot-init','/usr/bin/qemu-system-x86_64',"$work/metadata/fixture.sha256") {
    die "Missing immutable input: $required" unless exists $inputs->{$required};
}
for my $nested ("$work/metadata/fixture.sha256","$work/metadata/payload-files.sha256","$work/metadata/firmware.sha256") {
    die "Unsealed nested ledger: $nested" unless exists $inputs->{$nested};
    my $rows=manifest($nested);
    for my $name (keys %$rows) {
        die "Input absent from complete ledger: $name" unless exists($inputs->{$name}) && $inputs->{$name} eq $rows->{$name};
    }
}
die 'Historical PID1 bytes changed' unless
    ($inputs->{'/pilot/boot-init'}//'') eq 'a80b17c42c08706a15528d23a58cecd36829599be2ac3c874bb0a2d0325d25c2' &&
    ($inputs->{"$work/root/init"}//'') eq $inputs->{'/pilot/boot-init'};
die 'Pinned package closure changed' unless
    ($inputs->{"$work/metadata/packages.tsv"}//'') eq '03d8ea3e7b8eb73c36a0825f91eed8c9cc0767447b41654a58e0a70bcf842860';

# Ensure the sealed payload inventory is complete and its physical modes and
# symlinks still equal the preboot frozen layout, not merely a list of hashes.
my (@layout,@regular);
find({no_chdir=>1,wanted=>sub {
    my $name=$File::Find::name;
    my @st=lstat($name); die "lstat $name: $!" unless @st;
    my $kind=S_ISDIR($st[2])?'d':S_ISREG($st[2])?'f':S_ISLNK($st[2])?'l':die "Unexpected payload type: $name";
    my $relative=$name eq "$work/root"?'':substr($name,length("$work/root/"));
    my $target=$kind eq 'l'?readlink($name):'';
    die 'Unrepresentable payload name' if $relative=~/[\s\\]/ || $target=~/[\s\\]/;
    push @layout,join("\t",$kind,sprintf('%o',$st[2]&07777),$relative,$target)."\n";
    push @regular,$name if $kind eq 'f';
}},"$work/root");
die 'Payload mode/symlink inventory changed' unless
    join('',sort @layout) eq read_text("$work/metadata/payload-host-layout.tsv");
my $payload=manifest("$work/metadata/payload-files.sha256");
die 'Payload file inventory incomplete' unless join("\n",sort @regular) eq join("\n",sort keys %$payload);
my $firmware_links=read_text("$work/metadata/firmware-links.tsv");
my %firmware_link_paths;
for my $line (split /\n/,$firmware_links) {
    $line=~m{\A(\Q$work\E/qemu-data/[^\s\\]+)\t(/usr/share/(?:qemu|seabios)/[^\s\\]+)\z} or die 'Malformed firmware link';
    my ($name,$target)=($1,$2);
    die 'Duplicate firmware link' if $firmware_link_paths{$name}++;
    die "Firmware symlink changed: $name" unless -l $name && readlink($name) eq $target;
}
my $firmware=manifest("$work/metadata/firmware.sha256");
die 'Firmware inventory incomplete' unless keys(%$firmware)>0 &&
    join("\n",sort keys %firmware_link_paths) eq join("\n",sort keys %$firmware) &&
    exists $firmware->{"$work/qemu-data/bios-256k.bin"};
my @actual_firmware_links;
find({no_chdir=>1,wanted=>sub {
    my $name=$File::Find::name;
    if (-l $name) {push @actual_firmware_links,$name;}
    elsif (!-d $name) {die "Unexpected firmware search entry: $name";}
}},"$work/qemu-data");
die 'Firmware symlink inventory changed' unless
    join("\n",sort @actual_firmware_links) eq join("\n",sort keys %firmware_link_paths);

my $guard=read_text($guard_file);
$guard=~/\Astage=boot native_exit=([0-9]+) supervisor_exit=([0-9]+) cleanup_exit=([0-9]+) interrupted=([0-9]+) telemetry_exit=([0-9]+) timeout_seconds=360\n\z/
    or die 'Malformed or wrong-stage terminal guard status';
my ($native,$supervisor,$cleanup,$interrupted,$telemetry)=($1,$2,$3,$4,$5);
my $forced=$supervisor!=0 || $cleanup!=0 || $interrupted!=0 || $telemetry!=0 || $native==124 || $native>=128;
my $serial_receipt=read_text("$run/boot-serial.sha256");
$serial_receipt=~/\A([0-9a-f]{64})  \Q$serial_file\E\n\z/ or die 'Malformed independent serial receipt';
my $serial_expected=$1;
my $status={state=>'terminal',qemu_exit_status=>$forced?undef:0+$native,
    termination_reason=>$forced?'ci_guard_or_native_timeout':undef,
    timeout_wrapper_signal=>$native>=128?$native-128:undef,
    serial_sha256=>$serial_expected,source_commit=>$context->{source_sha},kernel_release=>$context->{kernel_release},
    known_open_gate=>'Original-C suite acceptance and wider runtime-family coverage require separate evidence.'};
die 'Preboot release differs from immutable kernel release' unless $status->{kernel_release} eq $release;
my $raw=read_text($serial_file);
my $text=$raw; $text=~s/\r//g;
my %checks;
$checks{terminal}=($status->{state}//'') eq 'terminal';
$checks{exit_zero}=defined($status->{qemu_exit_status}) && $status->{qemu_exit_status}==0;
$checks{not_forced}=!defined($status->{termination_reason}) && !defined($status->{timeout_wrapper_signal});
$checks{serial_hash_matches}=sha256_hex($raw) eq ($status->{serial_sha256}//'');
$checks{correct_source_commit}=($status->{source_commit}//'') eq $source;
$checks{pid1_begin_exactly_once}=(()=$text=~/^LUPOS_BOOT_SMOKE_BEGIN v1 pid=1$/mg)==1;
$checks{correct_guest_release}=index($text,"LUPOS_KERNEL_RELEASE=$status->{kernel_release}\n")>=0;
$checks{success_exactly_once}=(()=$text=~/^LUPOS_BOOT_SMOKE_SUCCESS v1$/mg)==1;
$checks{poweroff_marker_exactly_once}=(()=$text=~/^LUPOS_BOOT_SMOKE_POWEROFF v1$/mg)==1;
$checks{kernel_powered_down}=index($text,'reboot: Power down')>=0;
$checks{boot_only_scope}=index($text,'LUPOS_BOOT_SMOKE_SCOPE=boot_only_no_workloads_no_original_C_acceptance')>=0;
$checks{no_faults}=$text!~/Kernel panic|Oops:|BUG:|general protection fault|LUPOS_BOOT_SMOKE_FAILURE|segfault at|invalid opcode:/i;
$checks{bash_posix_shell}=index($text,'LUPOS_SH_BASH_VERSION=')>=0 && $text=~/^posix\s+on$/m;
for my $mount (['proc','/proc'],['sysfs','/sys'],['devtmpfs','/dev'],['tmpfs','/run'],['tmpfs','/tmp']) {
    $checks{"mount_$mount->[1]"}=index($text,"LUPOS_MOUNT_READY type=$mount->[0] target=$mount->[1]")>=0;
}
die 'Historical assertion count changed' unless keys(%checks)==18;
my $pass=1;
for my $name (keys %checks) {$pass=0 unless $checks{$name}; $checks{$name}=$checks{$name}?JSON::PP::true:JSON::PP::false;}
my $result={result=>$pass?'PASS_BOOT_SMOKE_ONLY':'FAIL',checks=>\%checks,
    scope=>'Startup, PID1, mount, tool-identification and shutdown only. No original-C acceptance or parity claim.',
    known_open_gate=>$status->{known_open_gate},
    identity=>{source_sha=>$status->{source_commit},kernel_release=>$status->{kernel_release},
        serial_sha256=>sha256_hex($raw),immutable_inputs_sha256=>$context->{immutable_inputs_sha256},
        initramfs_sha256=>$inputs->{"$work/initramfs.cpio.gz"},initramfs_bytes=>0+(-s "$work/initramfs.cpio.gz")}};
my $encoded=JSON::PP->new->canonical->pretty->encode($result);
open my $out,'>',"$run/boot-verification.json" or die $!; print {$out} $encoded; close $out or die $!;
print $encoded;
exit($pass?0:1);
