use puppet_forge_updates::puppet_module;
use std::env;

// main takes an argument with a path to a Puppetfile
// and reads its modules into a vector of puppet_module::PuppetModule
// afterwards it will print the update status of each module
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        // Hilfe anzeigen, exit 0
        println!(
            "USAGE: {} <Puppetfile>",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("puppet_forge_updates")
        );
        println!("Checks for available updates of modules listed in the Puppetfile.");
        std::process::exit(0);
    }

    if args.len() < 2 {
        // Zu wenige Argumente: Usage auf stdout, exit 1
        println!(
            "USAGE: {} <Puppetfile>",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("puppet_forge_updates")
        );
        println!("Checks for available updates of modules listed in the Puppetfile.");
        std::process::exit(1);
    }

    let path = &args[1];
    let modules = puppet_module::read_puppetfile(path);

    for module in modules.iter() {
        if module.determine_update().is_some() {
            println!("{module}");
        }
    }
}
