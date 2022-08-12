pub mod git;

use std::{process::{Command, Stdio}, fs};

use git::git_root;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Testing Solutions...");

    let mut problems = git_root()?;
    problems.push("exercism");
    problems.push("rust");
    let problems = problems;

    for entry in problems.read_dir()? {
        let problem = entry?.path();
        if !problem.is_dir() { continue; }

        let name = problem.display();
        // caching
        let snt = problem.join("done.snt");
        if snt.exists() {
            println!("🏅 {name}");
            continue;
        }

        match Command::new("cargo")
            .arg("test")
            .current_dir(&problem)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status() {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("✅ {name}");
                        fs::File::create(&snt)?;
                    }
                    else {
                        println!("🗻 {name}");
                    }
                },
                Err(_) => {
                    println!("🤔 failed to test {name}?!");
                },
            }
    }
    Ok(())
}
