use std::fs;

fn has_docker_env_file() -> bool {
    match fs::metadata("/.dockerenv") {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn has_docker_in_cgroup() -> bool {
    match fs::read_to_string("/proc/self/cgroup") {
        Ok(file_contents) => file_contents.contains("docker"),
        Err(_error) => false,
    }
}

pub fn is_docker() -> bool {
    let is_docker = has_docker_env_file() || has_docker_in_cgroup();

    is_docker
}
