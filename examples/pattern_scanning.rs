use toy_arms::{detect_keydown, Memory, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Memory::from_module_name("client.dll");
    unsafe {
            //let pattern = "46 83 ? 04 ? 44 ? 14 ? ? 45 FF";
        let pattern = "55 8B ? 83 ? F8 ? EC ? ? 00 00";
        println!("pattern found: {:p}", memory.signature_scan(pattern, 0, 0).unwrap());
    }
    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}