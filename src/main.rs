fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <version1> <version2>", args[0]);
        return std::process::ExitCode::FAILURE;
    }

    let v1 = &args[1];
    let v2 = &args[2];

    let v1 = match split_version(&v1) {
        Ok(version) => version,
        Err(exit_code) => return exit_code,
    };

    let v2 = match split_version(&v2) {
        Ok(version) => version,
        Err(exit_code) => return exit_code,
    };

    if v1.0 > v2.0 {
        return std::process::ExitCode::FAILURE;
    }

    if v1.0 == v2.0 && v1.1 > v2.1 {
        return std::process::ExitCode::FAILURE;
    }

    if v1.0 == v2.0 && v1.1 == v2.1 && v1.2 > v2.2 {
        return std::process::ExitCode::FAILURE;
    }

    if v1.0 == v2.0 && v1.1 == v2.1 && v1.2 == v2.2 {
        return std::process::ExitCode::FAILURE;
    }

    return std::process::ExitCode::SUCCESS;
}

fn split_version(version: &str) -> Result<(u32, u32, u32), std::process::ExitCode> {
    let version = version.split(".").collect::<Vec<&str>>();
    if version.len() != 3 {
        eprintln!("Error: Invalid version");
        return Err(std::process::ExitCode::FAILURE);
    }

    let major = match version[0].parse::<u32>() {
        Ok(major) => major,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let minor = match version[1].parse::<u32>() {
        Ok(minor) => minor,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    let patch = match version[2].parse::<u32>() {
        Ok(patch) => patch,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(std::process::ExitCode::FAILURE);
        }
    };

    return Ok((major, minor, patch));
}
