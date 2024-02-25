use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let project_dir = env::current_dir().unwrap();
    let env_dev_file = project_dir.join(".env.dev");
    let env_prod_file = project_dir.join(".env.prod");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    let target_env_dev_file = out_path.join(".env.dev");
    let target_env_prod_file = out_path.join(".env.prod");

    fs::copy(&env_dev_file, &target_env_dev_file).unwrap_or_else(|err| {
        eprintln!("Failed to copy .env.dev file: {}", err);
        print!("Source: {:?}", &env_dev_file.as_os_str());
        print!("Target: {:?}", &target_env_dev_file.as_os_str());
        std::process::exit(1);
    });
    fs::copy(&env_prod_file, &target_env_prod_file).unwrap_or_else(|err| {
        eprintln!("Failed to copy .env.prod file: {}", err);
        print!("Source: {:?}", &env_prod_file.as_os_str());
        print!("Target: {:?}", &target_env_prod_file.as_os_str());
        std::process::exit(1);
    });
}
