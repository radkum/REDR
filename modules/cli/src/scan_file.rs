pub(super) fn scan_file(file_path: &str, signatures_path: &str) -> anyhow::Result<()> {
    println!("{}", file_path);

    let signatures = signatures::get_signatures(signatures_path)?;
    scanner::run_scanner(file_path, signatures)?;
    sandbox::run_sandbox();
    Ok(())
}
