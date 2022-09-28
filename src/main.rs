fn main() {
    let hello_world_code: String = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+".to_string()
        + "[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    if hersenneuk::is_valid_brainfuck(&hello_world_code) {
        hersenneuk::run(&hello_world_code);
    } else {
        println!("Oh noes :'(");
    }
}
