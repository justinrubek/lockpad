fn main() {
    // If we're building on docs.rs, set the SQLX_OFFLINE environment variable so the build will succeed.
    if std::env::var_os("DOCS_RS").is_some() {
        println!("cargo:rustc-env=SQLX_OFFLINE=true");
    }
}
