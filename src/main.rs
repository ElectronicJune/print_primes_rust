fn main() {
    println!("{}", 2);
    let mut index: u8 = 3;
    loop {
        if index <= 127 {
            let mut previous: u8 = 1;
            let mut current: u8 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % index);
            }
            if current == 1 {
                println!("{index}");
            }
        } else {
            let mut previous: u16 = 1;
            let mut current: u16 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % (index as u16));
            }
            if current == 1 {
                println!("{index}");
            }
        }
        if index == 253 {
            break;
        }
        index += 2
    }
    let mut index: u16 = index as u16;
    loop {
        if index <= 32767 {
            let mut previous: u16 = 1;
            let mut current: u16 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % index);
            }
            if current == 1 {
                println!("{index}");
            }
        } else {
            let mut previous: u32 = 1;
            let mut current: u32 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % (index as u32));
            }
            if current == 1 {
                println!("{index}");
            }
        }

        if index == 65533 {
            break;
        }
        index += 2
    }
    let mut index: u32 = index as u32;
    loop {
        if index <= 2147483647 {
            let mut previous: u32 = 1;
            let mut current: u32 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % index);
            }
            if current == 1 {
                println!("{index}");
            }
        } else {
            let mut previous: u64 = 1;
            let mut current: u64 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % (index as u64));
            }
            if current == 1 {
                println!("{index}");
            }
        }

        if index == 4294967293 {
            break;
        }
        index += 2
    }
    let mut index: u64 = index as u64;
    loop {
        if index <= 9223372036854775807 {
            let mut previous: u64 = 1;
            let mut current: u64 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % index);
            }
            if current == 1 {
                println!("{index}");
            }
        } else {
            let mut previous: u128 = 1;
            let mut current: u128 = 3;
            for _ in 2..index {
                (previous, current) = (current, (previous + current) % (index as u128));
            }
            if current == 1 {
                println!("{index}");
            }
        }

        if index == 18446744073709551613 {
            break;
        }
        index += 2
    }
}
