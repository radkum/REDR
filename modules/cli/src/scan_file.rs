pub(super) fn scan_file(file_path: &str) -> anyhow::Result<()> {
    println!("{}", file_path);

    scanner::run_scanner(file_path)?;
    sandbox::run_sandbox();
    Ok(())
}
