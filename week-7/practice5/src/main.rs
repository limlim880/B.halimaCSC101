fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is {}",num);
}

fn mutate_num_to_zero(mut paranum_num: i32) {
    paranum_num = paranum_num*0;
    println!("paranum_num value is :{}",paranum_num);
}
