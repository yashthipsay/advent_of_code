let config_max = Some(3u8);

// match config_max {
//     Some(max) => println!("The maximum is configured to be {}", max),
//     _ => (),
// }

let config_value = Some(3u8);
if let Some(value) = config_value {
    println!("The value is configured to be {}", value);
}



