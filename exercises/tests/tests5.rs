// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

/// # Safety  
///  
/// `address` 必须包含指向有效的 `u32` 值的可变引用。  
/// 调用者必须确保该地址指向一个唯一且有效的  
/// 存储位置，在该位置存放的是一个 `u32`。  
unsafe fn modify_by_address(address: usize) {  
    // SAFETY: 我们假设该地址是有效的，并且指向一个  
    // 可变的 `u32`。调用者对该操作的安全性负责。  
    let ptr = address as *mut u32; // 将地址转换为可变指针  
    *ptr = 0xAABBCCDD;              // 修改给定地址处的值  
}  

#[cfg(test)]  
mod tests {  
    use super::*;  

    #[test]  
    fn test_success() {  
        let mut t: u32 = 0x12345678;  
        // SAFETY: 该地址被保证是有效的，并且包含  
        // 指向一个 `u32` 本地变量的唯一引用。  
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };  
        assert!(t == 0xAABBCCDD);  
    }  
}