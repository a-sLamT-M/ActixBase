use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let project_dir = env::current_dir().unwrap();
    let env_file = project_dir.join(".env");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    let target_env_dev_file = out_path.join(".env.dev");
    let target_env_prod_file = out_path.join(".env.prod");

    fs::copy(&env_file, target_env_dev_file).expect("Failed to copy .env.dev file");
    fs::copy(&env_file, target_env_prod_file).expect("Failed to copy .env.prod file");
}