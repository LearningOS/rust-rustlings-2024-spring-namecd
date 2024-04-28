// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res : i32= 42;
    let option = Some(12);
    while let Some(x) = option {
        if let Some(res_new) = res.checked_add(x) {
            res = res_new;
        }
        else{
            break;
        }
    }
    println!("{}", res);
}
