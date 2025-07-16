mod utils;
use utils::ec;

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");

    let register = std::env::args()
        .nth(2)
        .expect("No register provided")
        .parse::<u8>()
        .expect("Invalid register");

    match action.as_str() {
        "read" => {
            let mut ec = ec::EC::new().expect("Failed to initialize EC");
            let value = ec.read(register).expect("Failed to read from EC");
            println!("{}", value);
        }
        "write" => {
            let value = std::env::args()
                .nth(3)
                .expect("No value provided")
                .parse::<u8>()
                .expect("Invalid value");

            let mut ec = ec::EC::new().expect("Failed to initialize EC");
            ec.write(register, value).expect("Failed to write to EC");
        }
        _ => eprintln!("Unknown action: {}", action),
    }
}
