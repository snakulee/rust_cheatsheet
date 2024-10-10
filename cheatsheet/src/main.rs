/* Rust cheatsheet from https://letsgetrusty.com/ */

use core::error;

fn main() {
    println!("Hello, world!");

    /* ---------------------------------------------------------------------- */
    /* ------------------    Basic Types & Variables    --------------------- */
    /* ---------------------------------------------------------------------- */
    // Boolean
    //      bool
    // Unsigned integers
    //      u8, u16, u32, u64, u128
    // Signed integers
    //      i8, i16, i32, i64, i128
    // Floating point numbers
    //      f32, f64
    // Platform specific integers
    //      usize - Unsigned integer. Same number of bits
    //              as the platform's pointer type.
    //      isize - Signed integer. Same number of bits
    //              as the platform's pointer type.
    // Unicode scalar value
    //      char
    // String slice
    //      &str
    // Owned string
    //      String

    /* -----------------------------   Tuple   ------------------------------ */
    {
        let _coordinates = (82, 64);
        let _score = ("Team A", 12);
    }

    /* -------------------------   Array & Slice   -------------------------- */
    {
        // Arrays must have a known length and all elements must be initialized
        let array = [1, 2, 3, 4, 5];
        let _array2 = [0; 3]; // [0, 0, 0]

        // Unlike arrays the length of a slice is determined at runtime
        let _slice = &array[1..3];
    }

    /* ---------------------------   HashMap   ------------------------------ */
    {
        use std::collections::HashMap;
        let mut subs = HashMap::new();
        subs.insert(String::from("LGR"), 100000);

        // Insert key if it doesn't have a value
        subs.entry("Let‘s Get Rusty".to_owned()).or_insert(3);
    }

    /* ---------------------------   Struct    ------------------------------ */
    {
        // Definition
        struct User {
            _username: String,
            _active: bool,
        }

        // Instantiation
        let _user1 = User {
            _username: String::from("bogdan"),
            _active: true,
        };

        // Tuple struct
        struct Color(i32, i32, i32);
        let _black = Color(0, 0, 0);
    }

    /* -----------------------------   Enum    ------------------------------ */
    {
        // Definition
        enum Command {
            Quit,
            Move { _x: i32, _y: i32 },
            Speak(String),
            ChangeBGColor(i32, i32, i32),
        }

        // Instantiation
        let _msg1 = Command::Quit;
        let _msg2 = Command::Move { _x: 1, _y: 2 };
        let _msg3 = Command::Speak("Hi".to_owned());
        let _msg4 = Command::ChangeBGColor(0, 0, 0);
    }

    /* -------------------------   Constant    ------------------------------ */
    {
        const _MAX_POINTS: u32 = 100_000;
    }

    /* -----------------------   Static Variable  --------------------------- */
    {
        // Unlike constants static variables are stored in a dedicated memory
        // location and can be mutated.
        static _MAJOR_VERSION: u32 = 1;
        static mut _COUNTER: u32 = 0;
    }

    /* ------------------------     Mutability    --------------------------- */
    {
        let mut _x = 5;
        _x = 6;
    }

    /* --------------------------   Shadowing  ------------------------------ */
    {
        let _x = 5;
        let _x = _x * 2;
    }

    /* -------------------------   Type Alias  ------------------------------ */
    {
        // `_NanoSecond` is a new name for `u64`.
        type _NanoSecond = u64;
    }

    /* ---------------------------------------------------------------------- */
    /* ----------------------     Control Flow       ------------------------ */
    /* ---------------------------------------------------------------------- */

    /* -------------------------   If & If Let  ----------------------------- */
    {
        let num = Some(22);
        if num.is_some() {
            println!("number is: {}", num.unwrap());
        }

        // match pattern and assign variable
        if let Some(i) = num {
            println!("if let variable 'i' is: {i}");
        }
    }

    /* ---------------------------    Loop    ------------------------------- */
    {
        let mut count = 0;
        loop {
            count += 1;
            if count == 5 {
                println!("loop count is: {count}");
                break; // Exit loop
            }
        }
    }

    /* ---------------------   Returning From Loops    ---------------------- */
    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter;
            }
        };
        println!("loop return result is {result}")
    }

    /* ---------------------   Nested Loops & Labels   ---------------------- */
    {
        'outer: loop {
            'inner: loop {
                // This breaks the outer loop
                break 'outer;

                // This breaks the inner loop
                break;
            }
        }
    }

    /* -----------------------   While & While Let   ------------------------ */
    {
        let mut n = 0;
        while n < 101 {
            n += 1;
        }

        let mut my_picks: Vec<u8> = vec![3, 33, 44];
        while let Some(num) = my_picks.pop() {
            println!("while let Some num is {}", num);
        }
    }

    /* ---------------------------   for loop   ----------------------------- */
    {
        print!("for loop n is");
        for n in 1..10 {
            print!(" {}", n);
        }
        println!("");

        let names = vec!["Bogdan", "Wallace", "Snaku"];
        print!("for loop name.iter is");
        for name in names.iter() {
            print!(" {}", name);
        }
        println!("");
    }

    /* ----------------------------    Match    ----------------------------- */
    {
        let optional = Some(0);
        match optional {
            Some(i) => println!("match Some(0) is {}", i),
            None => println!("No value."),
        }
    }

    /* ---------------------------------------------------------------------- */
    /* -----------------    Ownership  &  Borrowing     --------------------- */
    /* ---------------------------------------------------------------------- */

    // Ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Borrowing rules
    // 1. At any given time, you can have either one mutable reference or
    //    any number ofimmutable references.
    // 2. References must always be valid

    /* --------------------    Creating References   ------------------------ */
    {
        let s1 = String::from("hello world!");
        let _s1_ref = &s1; // immutable reference
        let mut s2 = String::from("hello");
        let s2_ref = &mut s2; // mutable reference
        s2_ref.push_str(" world!");
        println!("\"hello\".push(\" world!\") is {s2_ref}")
    }

    /* --------------------    Copy, Move & Clone    ------------------------ */
    {
        // Simple values which implement the Copy trait are copied by value
        let x = 5;
        let _y = x;
        // println!("{}", x); // x is still valid

        // The string is moved to s2 and s1 is invalidated
        let s1 = String::from("Let's Get Rusty!");
        let _s2 = s1; // Shallow copy a.k.a move
                      // println!("{}", s1); // Error: s1 is invalid

        let s1 = String::from("Let's Get Rusty!");
        let _s2 = s1.clone(); // Deep copy
        println!("{}", s1); // Valid because s1 isn't moved
    }

    /* -------------------    Ownership & functions    ---------------------- */
    {
        let x = 5;
        takes_copy(x); // x is copied by value

        let s = String::from("Let’s Get Rusty!");
        takes_ownership(s); // s is moved into the function

        let _s1 = gives_ownership(); // return value is moved into s1

        let s2 = String::from("LGR");
        let _s3 = takes_and_gives_back(s2);
    }

    fn takes_copy(some_integer: i32) {
        println!("takes_copy a i32: {}", some_integer);
    }

    fn takes_ownership(some_string: String) {
        println!("takes_ownership a String: {}", some_string);
    }

    // some_string goes out of scope and dropis called. The backing memory is freed.
    fn gives_ownership() -> String {
        let some_string = String::from("LGR");
        some_string
    }

    fn takes_and_gives_back(some_string: String) -> String {
        some_string
    }

    /* ---------------------------------------------------------------------- */
    /* ---------------------      Pattern Match      ------------------------ */
    /* ---------------------------------------------------------------------- */

    /* ----------------------------    Basic    ----------------------------- */
    {
        let x = 5;

        match x {
            1 => println!("one"),                      // matching literals
            2 | 3 => println!("two or three"),         // matching multiple patterns
            4..=9 => println!("match x within range"), // matching ranges
            x => println!("match x is {}", x),         // matching named variables
            _ => println!("default Case"),             // default case (ignores value)
        }
    }

    /* -----------------------    Destructuring    -------------------------- */
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => {
                println!("p is a Point with y = 0 {}", x);
            }
            Point { x, y } => {
                println!("p is a Point x:{} y:{}", x, y);
            }
        }

        #[derive(Debug)]
        enum Shape {
            Rectangle { width: i32, height: i32 },
            Circle(i32),
        }

        let shape1 = Shape::Circle(10);
        match shape1 {
            Shape::Rectangle {
                width: _x,
                height: _y,
            } => {
                println!("shape1 is an {:?}", shape1);
            }
            Shape::Circle(_radius) => {
                println!("shape1 is a {:?}", shape1);
            }
        }

        let shape2 = Shape::Rectangle {
            width: 70,
            height: 50,
        };
        match shape2 {
            Shape::Rectangle {
                width: _x,
                height: _y,
            } => {
                println!("shape2 is an {:?}", shape2);
            }
            Shape::Circle(_radius) => {
                println!("shape2 is a {:?}", shape2);
            }
        }
    }

    /* ---------------------------------------------------------------------- */
    /* ------------------------      Iterators     -------------------------- */
    /* ---------------------------------------------------------------------- */

    /* ----------------------------    Usage    ----------------------------- */
    {
        // Methods that consume iterators
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        println!("vec.iter().sum(): {:?}", total);

        // Methods that produce new iterators
        let v1: Vec<i32> = vec![1, 2, 3];
        let iter = v1.iter().map(|x| x + 1);
        println!("vec.iter().map(): {:?}", iter);

        // Turning iterators into a collection
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("vec.iter().map().collect(): {:?}", v2);
    }

    /* ----------------  Implementing the Iterator trait  ------------------- */
    {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
    }

    /* ---------------------------------------------------------------------- */
    /* ----------------------     ERROR HANDLING     ------------------------ */
    /* ---------------------------------------------------------------------- */

    /* -------------------------   Option enum  ----------------------------- */
    {
        struct DataBase{
            name: String,
            id: u32
        }
    
        impl DataBase{
            fn user_exists(&self, name: &str) -> bool{
                if self.name == name{
                    return true
                }
                false
            }
    
            fn get_id(&self, name: &str) -> u32{
                if self.user_exists(name){
                    return self.id
                }
                0
            }
        }

        fn get_user_id(name: &str) -> Option<u32> {  
            let database = DataBase{name:"Snaku".to_string(), id:23};  
            if database.user_exists(name) {
               return Some(database.get_id(name))
            } 
            None
        }

        match get_user_id("Snaku"){
            Some(id) => println!("Snaku's id in database is {id}"),
            None => println!("user not exist"),
        }
    }

    /* -------------------------   Result enum  ----------------------------- */
    {
        struct Error{
            msg: String
        }
    
        struct User{
            id: u32
        }

        fn is_logged_in_as(id:u32)-> bool{
            if id != 0{
                return true
            }
            false
        }

        fn get_user_object(id:u32) -> User{
            let mut user = User{id:0};
            user.id = id;
            user
        }

        fn get_user(id: u32) -> Result<User, Error> {
            if is_logged_in_as(id) {
                return Ok(get_user_object(id))
            } 
            Err(Error { msg: "not logged in".to_string() })
        }

        match get_user(0){
            Ok(u) => println!("get user id is {}", u.id),
            Err(e) => println!("get user error: {}",e.msg),
        }

        match get_user(23){
            Ok(u) => println!("get user id is {}", u.id),
            Err(e) => println!("get user error: {}",e.msg),
        }
        
    }

    /* ----------------------   '?'  Operator    ---------------------------- */
    {
        struct User{
            id: i32,
            name: String,
            job: Job,
        }

        struct Job{
            _name: String,
            salary: u32,
        }

        impl User{
            fn get_job(&self) -> Option<&Job>{
                Some(&self.job)
            }
        }
        
        #[derive(Debug)]
        enum Connection{
            _Http,
            Ssh,
            _Telnet,
            _Restful
        }

        struct Database{
            user: User,
            conn: Connection,
        }

        struct Error{
            msg: String
        }

        impl Database{
            fn get_user(&self, id:i32) -> Option<&User>{
                if id == 23{
                    Some(&self.user)
                }
                else{
                    None
                }
            }

            fn get_connect(self) -> Result<Connection, Error>{
                Ok(self.conn)
            }
        }

        let work = Job{_name: "FW".to_string(), salary: 100000};
        let user1 = User{id:23, name:"Snaku".to_string(), job:work};
        let db = Database{user: user1, conn:Connection::Ssh};

        fn get_salary(db: &Database, id: i32) -> Option<u32> {
            Some(db.get_user(id)?.get_job()?.salary)
        }

        fn connect(db: Database) -> Result<Connection, Error> {
            let conn = db.get_connect()?;
            Ok(conn)
        }

        match get_salary(&db, 23){
            Some(s) => println!("user 23's salary is {}", s),
            None => println!("find user 23 error"),
        }

        match connect(db){
            Ok(conn) => println!("db conn is {:?}", conn),
            Err(e) => println!("{}", e.msg),
        }
    }
    

}
