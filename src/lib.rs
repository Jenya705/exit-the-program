#![feature(never_type)]


pub fn exit() -> ! {
    // Safety. No safety, hooray
    unsafe {
        let bytes = [0u8; std::mem::size_of::<usize>()];
        let ptr: *const i32 = std::mem::transmute(bytes);
        let value = *ptr;
        println!("If this is printing in console then exit is failed! {}", value)
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::exit;

    #[test]
    fn exit_test() {
        exit()
    }

}