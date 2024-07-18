fn print_repeat(message:String, times: usize){
    let message = message.repeat(times);
    println!("{}", message);

}

pub(crate) fn print_header(){
    print_repeat(String::from("::"), 15)
}

pub(crate) fn print_footer(){
    print_repeat(String::from("::"), 15)
}