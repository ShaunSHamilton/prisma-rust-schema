#[test]
fn cli_works() {
    build_cli();
    generate();
}

fn build_cli() {
    let output = std::process::Command::new("cargo")
        .arg("build")
        .output()
        .expect("Failed to run cli");

    println!(
        "CLI Build Output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    eprintln!(
        "CLI Build Error:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success());
}

fn generate() {
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("./target/debug:{}", current_path);

    let output = std::process::Command::new("npx")
        .args(&["prisma", "generate", "--no-engine"])
        .env("PATH", new_path) // Set the PATH for this command
        .output()
        .expect("Failed to run prisma generate");

    println!(
        "Prisma Generate Output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    eprintln!(
        "Prisma Generate Error:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success());
    // Add assertions based on the expected output of your generator
}
