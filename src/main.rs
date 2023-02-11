fn main() {
    println!("{}",2);
    let mut index :u128 = 3;
    loop {
        let mut previous :u128 = 1;
        let mut current :u128 = 3;
        for _ in 2..index{
            (previous,current) = (current,(previous+current)%index);
        }
        if current==1 {
            println!("{index}");
        }
        index += 2
    }
}
            
