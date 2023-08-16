#![feature(c_variadic)]
use std::ptr::null_mut;
use winapi::um::winuser::MessageBoxA;

const JMP_RBX_TEST: [u8; 2] = [0xFF, 0xE3];

fn main() {

    cc::Build::new()
    .file("src/spoof.asm")
    .compile("spoof");

    let result = unsafe { do_spoof_call(MessageBoxA as _, JMP_RBX_TEST.as_ptr() as _, null_mut::<*mut u8>(), "Hello World\0", "Spoofed call\0", 0) };
    println!("[!] MessageBoxA Result: {}", result);
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

 extern "C" {
    fn InitSpoofCall(function: *const u8, fake_ret: *const u8);
    fn SpoofCall(function: *const u8, fake_ret: *const u8, args: ... ) -> usize;
}

#[inline(never)]
unsafe extern "C" fn do_spoof_call(function: *const u8, fake_ret: *const u8, args: ...) -> usize {
    InitSpoofCall(function, fake_ret);
    SpoofCall(function, fake_ret, args)
}
