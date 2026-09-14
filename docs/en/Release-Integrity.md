# Release Integrity

## Source of truth

The tagged GitHub repository revision is the source of truth for a release. Obtain releases only from the project GitHub Releases page, PyPI, or npm package registries linked by the project.

## GitHub release verification

Releases generated from v0.3.15 onward contain:

- Native archives for supported operating systems.
- A WebAssembly archive.
- `SHA256SUMS.txt` containing SHA-256 digests for the archives.
- `security-core-ffi-cargo-metadata.json`, a machine-readable Cargo dependency manifest.

Verify a downloaded archive on Linux or macOS:

```bash
sha256sum -c SHA256SUMS.txt
```

On PowerShell:

```powershell
Get-FileHash .\native-windows-x86_64.tar.gz -Algorithm SHA256
```

Compare the returned digest to the matching value in `SHA256SUMS.txt`.

## Registry verification

Pin exact versions in Python and JavaScript dependency manifests. Registry packages provide convenient installation; the GitHub release remains the appropriate place to obtain native archives and their checksum manifest.

## Limits

Checksums establish file integrity only when obtained from a trusted release page. They do not replace application review, dependency review, signed provenance policy, or an independent cryptographic audit.
