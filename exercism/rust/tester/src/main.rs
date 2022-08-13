pub mod git;

use std::{process::{Command, Stdio}, fs, path::{PathBuf, Path}};

use git::git_root;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum ProblemState {
    Done,
    DonePreviously,
    Todo,
}

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    name: String,
    path: PathBuf,
    state: ProblemState,
}

impl Ord for Problem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.cmp(&other.state)
    }
}

impl PartialOrd for Problem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.state.partial_cmp(&other.state) {
            Some(core::cmp::Ordering::Equal) => {
                match self.name.partial_cmp(&other.name) {
                    Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Equal),
                    ord => ord,
                }
            }
            ord => ord,
        }
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self.state {
            ProblemState::Done => "✅",
            ProblemState::DonePreviously => "🏅",
            ProblemState::Todo => "❌",
        };
        write!(f, "{emoji} {}", self.name)
    }
}

impl Problem {
    fn test(name: String, path: PathBuf) -> Result<Problem, Box<dyn std::error::Error>>  {
        // caching
        let snt = path.join("done.snt");
        if snt.exists() {
            return Ok(Problem {
                name,
                path,
                state: ProblemState::DonePreviously,
            });
        }

        let exit_status = Command::new("cargo")
            .arg("test")
            .current_dir(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        let state = if exit_status.success() {
            fs::File::create(&snt)?;
            ProblemState::Done
        }
        else {
            ProblemState::Todo
        };

        Ok(Problem { name, path, state })
    }
}

fn problems(root: &Path) -> Result<Vec<(String, PathBuf)>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for entry in root.read_dir()? {
        let problem = entry?.path();
        if !problem.is_dir() { continue; }

        let name = problem
            .file_name()
            .ok_or("🤔 no problem?")?
            .to_str()
            .ok_or("🤔 no problem?")?;

        result.push((String::from(name), problem ));
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let root = git_root()?.join("exercism").join("rust");

    let mut results: Vec<Problem> = Vec::new();
    for (name, path) in problems(&root)? {
        match Problem::test(name.clone(), path) {
            Ok(problem) => {
                print!(".");
                results.push(problem);
            },
            Err(e) => println!("⚠️ {name} {e}"),
        }
    }
    println!();

    results.sort();

    for r in results {
        println!("{r}");
    }

    Ok(())
}
