fn main() {
    // println!("Hello, world!");
    // let port: u16 = 445;
    // let key: u8 = 0xAB;
    // let is_open: bool = true;
    // let host: &str = "10.0.0.1";
    // let owned: String = String::from("10.0.0.1");
    // let buf: Vec<u8> = vec![0u8; 4096];
    // let shellcode: [u8; 3] = [0x90, 0x90, 0x90];

    // println!("Port: {}", port);
    // println!("Key: 0x{:X}", key);
    // println!("Is Open: {}", is_open);
    // println!("Host: {}", host);
    // println!("Owned: {}", owned);
    // println!("Buffer Length: {}", buf.len());
    // println!("Shellcode: {:X?}", shellcode);

    // ------------------- Ownership Rules --------------------- //

    // 1. Every value has one owner.
    // let s1 = String::from("secret");

    // // 2. Assignment moves ownership for non-Copy types.
    // let s2 = s1; // s1 is invalid after this line
    //              // println!("{s1}");            // compile error
    // println!("{s2}");

    // // 3. References borrow without moving.
    // let s = String::from("creds");
    // let r: &str = &s; // immutable borrow
    // println!("{r}");
    // println!("{s}"); // still valid

    // // 4. Only one mutable borrow at a time.
    // let mut v = vec![1, 2, 3];
    // let m: &mut Vec<i32> = &mut v;
    // m.push(4);
    // // let m2 = &mut v;             // compile error while m is alive

    // // ------------------- Copy vs Move -------------------------
    // let n: u32 = 42;
    // let m = n; // Copy: both valid
    // println!("{n} {m}");

    // let s = String::from("abc");
    // let t = s.clone(); // explicit deep copy
    // println!("{s} {t}");


    // ------------------- Strings --------------------- //

    // Construction
    let a: &str = "static literal";          // &'static str
    let b: String = String::from("owned");
    let c: String = "x".to_string();
    let d: String = format!("{}:{}", host, port);

    // Conversion
    let owned: String = "abc".to_owned();
    let view: &str = &owned;                  // borrow as &str

    // Methods
    let s = "  ERROR: connection refused  ";
    s.trim();                                // "ERROR: connection refused"
    s.trim_start();
    s.trim_end();
    s.to_uppercase();
    s.to_lowercase();
    s.contains("ERROR");                     // true
    s.starts_with("ERROR");                  // true (after trim)
    s.ends_with("refused");                  // true
    s.find(":");                             // Option<usize>
    s.replace("ERROR", "WARN");
    let parts: Vec<&str> = s.split(',').collect();
    let line = "host: 10.0.0.1 port: 445";
    let tokens: Vec<&str> = line
        .split(|c: char| c == ':' || c == ' ')
        .filter(|t| !t.is_empty())
        .collect();

    // Empty / blank checks
    s.is_empty();
    s.trim().is_empty();                     // blank check

    // Case-insensitive compare
    "FOO".eq_ignore_ascii_case("foo");       // true

    // Formatting
    let port: u16 = 445;
    println!("{host}:{port}");
    println!("{port:X}");                    // hex: 1BD
    println!("{port:05}");                   // zero-padded: 00445
    println!("{:>10}", host);                // right-align in 10 cols



}
