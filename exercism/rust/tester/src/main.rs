pub mod git;

use std::{process::{Command, Stdio}, fs, path::{PathBuf, Path}};

use git::git_root;

struct Problem {
    name: String,
    path: PathBuf,
}

fn problems(root: &Path) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for entry in root.read_dir()? {
        let problem = entry?.path();
        if !problem.is_dir() { continue; }

        let name = problem
            .file_name()
            .ok_or("🤔 no problem?")?
            .to_str()
            .ok_or("🤔 no problem?")?;

        result.push(Problem { name: String::from(name), path: problem });
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Testing Solutions...");

    let mut root = git_root()?;
    root.push("exercism");
    root.push("rust");
    let root = root;

    for problem in problems(&root)? {
        // caching
        let snt = problem.path.join("done.snt");
        if snt.exists() {
            println!("🏅 {}", problem.name);
            continue;
        }

        let name = problem.name;

        match Command::new("cargo")
            .arg("test")
            .current_dir(&problem.path)
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
