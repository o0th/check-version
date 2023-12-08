use std::{env, process::ExitCode};
use toml::Table;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Usage: {} <repository> <from> <to> <file>", args[0]);
        return ExitCode::FAILURE;
    }

    let repository = args[1].split("/").collect::<Vec<&str>>();
    if repository.len() != 2 {
        eprintln!("Error: Invalid repository");
        return ExitCode::FAILURE;
    }

    let from = &args[2];
    let to = &args[3];

    let from_file = match get_file(repository[0], repository[1], from, &args[4]).await {
        Ok(file) => file,
        Err(e) => return e,
    };

    let to_file = match get_file(repository[0], repository[1], to, &args[4]).await {
        Ok(file) => file,
        Err(e) => return e,
    };

    let from_version = match get_version(&from_file) {
        Ok(version) => version,
        Err(e) => return e,
    };

    let to_version = match get_version(&to_file) {
        Ok(version) => version,
        Err(e) => return e,
    };

    println!("{:?}", from_version);
    println!("{:?}", to_version);

    return ExitCode::SUCCESS;
}

async fn get_file(
    username: &str,
    repository: &str,
    refs: &str,
    file: &str,
) -> Result<String, ExitCode> {
    let request = octocrab::instance()
        .repos(username, repository)
        .get_content()
        .r#ref(refs)
        .path(file)
        .send()
        .await;

    let response = match request {
        Ok(request) => request,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(ExitCode::FAILURE);
        }
    };

    if response.items.len() == 0 {
        eprintln!("Error: No item found");
        return Err(ExitCode::FAILURE);
    }

    let file = match response.items[0].decoded_content() {
        Some(file) => file,
        None => {
            eprintln!("Error: No content");
            return Err(ExitCode::FAILURE);
        }
    };

    return Ok(file);
}

fn get_version(file: &str) -> Result<String, ExitCode> {
    let table = match file.parse::<Table>() {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(ExitCode::FAILURE);
        }
    };

    let package = match table.get("package") {
        Some(package) => package,
        None => {
            eprintln!("Error: No package");
            return Err(ExitCode::FAILURE);
        }
    };

    let version = match package.get("version") {
        Some(version) => version,
        None => {
            eprintln!("Error: No version");
            return Err(ExitCode::FAILURE);
        }
    };

    return Ok(version.to_string().replace('"', ""));
}
