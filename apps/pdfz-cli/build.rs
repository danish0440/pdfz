//! Windows only: embed version info (VERSIONINFO) into `pdfz-cli.exe`, so Explorer's
//! Details tab, `(Get-Item pdfz-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `pdfz.exe` (`apps/pdfz/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFZ_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

/// The in-box DLLs only printing uses (Windows.Data.Pdf draws the job, prntvpt reads the
/// driver's print capabilities). Trimmed Windows installations (NTLite) can lack them, and a
/// load-time import of a missing DLL stops the program before `main` (#2630). Delay-loaded, they
/// load on the first print; `pdfz_print::spool` checks they exist before calling in.
/// Same list in `apps/pdfz{,-cli}/build.rs`, `crates/print/build.rs` and `crates/print/src/spool.rs`.
const DELAY_LOADED: [&str; 2] = ["windows.data.pdf.dll", "prntvpt.dll"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PDFZ_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        for dll in DELAY_LOADED {
            println!("cargo:rustc-link-arg-bins=/DELAYLOAD:{dll}");
        }
        println!("cargo:rustc-link-arg-bins=delayimp.lib");
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "PdfZ")
        .set("FileDescription", "PdfZ command-line tool")
        .set("LegalCopyright", "Copyright (c) the PdfCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pdfz-cli.exe")
        .set("InternalName", "pdfz-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFZ_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=pdfz-cli.exe built without version resources: {e}");
    }
}
