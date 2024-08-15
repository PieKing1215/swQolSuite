use std::path::{Path, PathBuf};

use hudhook::inject::Process;

fn main() {
    println!("SWMod Injector");

    // basically if running using cargo, look in the right target/ folder for it
    // otherwise if there's a swmod.dll in the working directory, use it.
    let dll_path = option_env!("OUT_DIR").map_or_else(|| {
        let mut cur_exe = std::env::current_exe().unwrap();
        cur_exe.push("..");
        cur_exe.push("swmod.dll");

        cur_exe.canonicalize().unwrap()
    },
    |p| {
        let p = PathBuf::from(p);
        // eg navigate from
        // target/debug/build/swmod-xxxxxxxx/out/
        // to
        // target/debug/swmod.dll
        let mut p = p
            .parent()
            .and_then(Path::parent)
            .and_then(Path::parent)
            .map(|p| p.to_path_buf())
            .unwrap();
        
        p.push("swmod.dll");
        
        p
    });

    println!("Injecting DLL @ {dll_path:?}");
    Process::by_name("stormworks64.exe").expect("Failed to find stormworks64.exe")
        .inject(dll_path).expect("Failed to inject DLL");
}