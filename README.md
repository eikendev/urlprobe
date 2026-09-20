<div align="center">
	<h1>urlprobe</h1>
	<h4 align="center">
	    A toy project that will never serve a purpose.
	</h4>
	<p>urlprobe lets you probe URLs for their status code in high speed.</p>
</div>

<p align="center">
	<a href="https://github.com/eikendev/urlprobe/actions"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/eikendev/urlprobe/release.yaml?branch=main"/></a>&nbsp;
	<a href="https://github.com/eikendev/urlprobe/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/eikendev/urlprobe"/></a>&nbsp;
	<a href="https://crates.io/crates/urlprobe"><img alt="Version" src="https://img.shields.io/crates/v/urlprobe"/></a>&nbsp;
	<a href="https://crates.io/crates/urlprobe"><img alt="Downloads" src="https://img.shields.io/crates/d/urlprobe"/></a>&nbsp;
</p>

## 🚀&nbsp;Installation

Install a prebuilt binary with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall urlprobe
```

Or build from source:

```bash
cargo install urlprobe
```

Linux binaries for `x86_64` and `aarch64`, both glibc and static musl, are attached to every [release](https://github.com/eikendev/urlprobe/releases), each with a `.sha256` checksum and a verifiable build provenance:

```bash
gh attestation verify urlprobe-x86_64-unknown-linux-musl.tar.gz --repo eikendev/urlprobe
```

## 📄&nbsp;Usage

Simply feed it a list of URLs like so:
```bash
urlprobe < urls.txt
```
