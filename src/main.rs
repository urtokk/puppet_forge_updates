use puppet_forge_updates::puppet_module;
use std::env;

// main takes an argument with a path to a Puppetfile
// and reads its modules into a vector of puppet_module::PuppetModule
// afterwards it will print the update status of each module
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let modules = puppet_module::read_puppetfile(path);

    for module in modules.iter() {
        if module.determine_update().is_some() {
            println!("{module}");
        }
    }
}
