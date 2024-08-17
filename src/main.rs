use hudhook::inject::Process;

fn main() {
    println!("SWMod Injector");

    // basically if running using cargo, look in the right target/ folder for it
    // otherwise if there's a swmod.dll in the working directory, use it.
    let mut cur_exe = std::env::current_exe().unwrap();
    cur_exe.push("..");
    cur_exe.push("swmod.dll");

    let dll_path = cur_exe.canonicalize().unwrap();

    println!("Injecting DLL @ {dll_path:?}");
    Process::by_name("stormworks64.exe").expect("Failed to find stormworks64.exe")
        .inject(dll_path).expect("Failed to inject DLL");
}