/* Rust cheatsheet from https://letsgetrusty.com/ */
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
        struct _Counter {
            count: u32,
        }

        impl _Counter {
            fn _new() -> _Counter {
                _Counter { count: 0 }
            }
        }

        impl Iterator for _Counter {
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
        struct DataBase {
            name: String,
            id: u32,
        }

        impl DataBase {
            fn user_exists(&self, name: &str) -> bool {
                if self.name == name {
                    return true;
                }
                false
            }

            fn get_id(&self, name: &str) -> u32 {
                if self.user_exists(name) {
                    return self.id;
                }
                0
            }
        }

        fn get_user_id(name: &str) -> Option<u32> {
            let database = DataBase {
                name: "Snaku".to_string(),
                id: 23,
            };
            if database.user_exists(name) {
                return Some(database.get_id(name));
            }
            None
        }

        match get_user_id("Snaku") {
            Some(id) => println!("Snaku's id in database is {id}"),
            None => println!("user not exist"),
        }
    }

    /* -------------------------   Result enum  ----------------------------- */
    {
        struct Error {
            msg: String,
        }

        struct User {
            id: u32,
        }

        fn is_logged_in_as(id: u32) -> bool {
            if id != 0 {
                return true;
            }
            false
        }

        fn get_user_object(id: u32) -> User {
            let mut user = User { id: 0 };
            user.id = id;
            user
        }

        fn get_user(id: u32) -> Result<User, Error> {
            if is_logged_in_as(id) {
                return Ok(get_user_object(id));
            }
            Err(Error {
                msg: "not logged in".to_string(),
            })
        }

        match get_user(0) {
            Ok(u) => println!("get user id is {}", u.id),
            Err(e) => println!("get user error: {}", e.msg),
        }

        match get_user(23) {
            Ok(u) => println!("get user id is {}", u.id),
            Err(e) => println!("get user error: {}", e.msg),
        }
    }

    /* ----------------------   '?'  Operator    ---------------------------- */
    {
        struct User {
            _id: i32,
            _name: String,
            job: Job,
        }

        struct Job {
            _name: String,
            salary: u32,
        }

        impl User {
            fn get_job(&self) -> Option<&Job> {
                Some(&self.job)
            }
        }

        #[derive(Debug)]
        enum Connection {
            _Http,
            Ssh,
            _Telnet,
            _Restful,
        }

        struct Database {
            user: User,
            conn: Connection,
        }

        struct Error {
            msg: String,
        }

        impl Database {
            fn get_user(&self, id: i32) -> Option<&User> {
                if id == 23 {
                    Some(&self.user)
                } else {
                    None
                }
            }

            fn get_connect(self) -> Result<Connection, Error> {
                Ok(self.conn)
            }
        }

        let work = Job {
            _name: "FW".to_string(),
            salary: 100000,
        };
        let user1 = User {
            _id: 23,
            _name: "Snaku".to_string(),
            job: work,
        };
        let db = Database {
            user: user1,
            conn: Connection::Ssh,
        };

        fn get_salary(db: &Database, id: i32) -> Option<u32> {
            Some(db.get_user(id)?.get_job()?.salary)
        }

        fn connect(db: Database) -> Result<Connection, Error> {
            let conn = db.get_connect()?;
            Ok(conn)
        }

        match get_salary(&db, 23) {
            Some(s) => println!("user 23's salary is {}", s),
            None => println!("find user 23 error"),
        }

        match connect(db) {
            Ok(conn) => println!("db conn is {:?}", conn),
            Err(e) => println!("{}", e.msg),
        }
    }

    /* ---------------------------------------------------------------------- */
    /* ------------------------     COMBINATORS     ------------------------- */
    /* ---------------------------------------------------------------------- */

    /* -----------------------------   .map  -------------------------------- */
    {
        let some_string = Some("LGR".to_owned());
        let some_len = some_string.map(|s| s.len());
        println!("some_len is {:?}", some_len);

        #[derive(Debug)]
        struct Error {
            _msg: String,
        }
        #[derive(Debug)]
        struct User {
            _name: String,
        }
        let string_result: Result<String, Error> = Ok("Bogdan".to_owned());
        let user_result: Result<User, Error> = string_result.map(|_name| User { _name });
        println!("user_result is {:?}", user_result);
    }

    /* --------------------------  .and_then    ----------------------------- */
    {
        let vec = Some(vec![1, 2, 3]);
        let first_element = vec.and_then(|vec| vec.into_iter().next());
        println!("first_element is {:?}", first_element);

        // let vec = Some(vec![1, 2, 3]);
        // let mut iterates = vec.unwrap().into_iter();
        // _ = iterates.next();
        // let second_element = iterates.next();
        // println!("second_element is {:?}", second_element);

        let string_result: Result<&'static str, _> = Ok("5");
        let number_result = string_result.and_then(|s| s.parse::<u32>());
        println!("number_result is {:?}", number_result);
    }

    /* ---------------------------------------------------------------------- */
    /* -------------------    MULTIPLE ERROR TYPES    ----------------------- */
    /* ---------------------------------------------------------------------- */

    /* ------------------   Define custom error type   ---------------------- */
    {
        use std::fmt;

        type _Result<T> = std::result::Result<T, _CustomError>;
        
        #[derive(Debug, Clone)]
        struct _CustomError;
        impl fmt::Display for _CustomError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "custom error message")
            }
        }
    }

    /* --------------------     Boxing errors    ---------------------------- */
    {
        use std::error::Error;
        type _Result<T> = std::result::Result<T, Box<dyn Error>>;
    }

    /* ---------------------------------------------------------------------- */
    /* -------------------    ITERATING OVER ERRORS    ----------------------- */
    /* ---------------------------------------------------------------------- */

    /* -----------    Ignore failed items with filter_map()   --------------- */
    {
        let strings = vec!["LGR", "22", "7"];
        let numbers: Vec<_> = strings
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        println!("filter_map numbers are {:?}", numbers);
    }

    /* ----------   Fail the entire operation with collect()   -------------- */
    {
        let strings = vec!["LGR", "22", "7"];

        let numbers: Result<Vec<_>, _> = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();

        println!("map without filter, Error msg: {:?}", numbers);
    }


    /* ------   Collect all valid values & failures with partition()  ------- */
    {
        let strings = vec!["LGR", "22", "7"];

        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        println!("numbers: {:?}, errors: {:?}", numbers, errors);
        
        let numbers: Vec<_> = numbers
            .into_iter()
            .map(Result::unwrap)
            .collect();
        println!("numbers: {:?}", numbers);
        
        let errors: Vec<_> = errors
            .into_iter()
            .map(Result::unwrap_err)
            .collect();
        println!("errors: {:?}", errors);
    }


    /* ---------------------------------------------------------------------- */
    /* ---------------    GENERICS, TRAITS & LIFETIMES    ------------------- */
    /* ---------------------------------------------------------------------- */

    /* ----------------------    Using Generics   --------------------------- */
    {
        struct _Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> _Point<T, U> {
            fn _mixup<V, W>(self, other: _Point<V, W>) -> _Point<T, W> {
                _Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }



    /* ---------------------    Defining Traits    -------------------------- */
    trait _Animal {
        fn new(name: &'static str) -> Self;
        fn noise(&self) -> &'static str { "" }
    }
    
    struct _Dog { name: &'static str }
    
    impl _Dog {
        fn _fetch() {   }
    }
    
    impl _Animal for _Dog {
        fn new(name: &'static str) -> _Dog {
            _Dog { name: name }
        }
        fn noise(&self) -> &'static str {
            "woof!"
        }
    }

    /* ----------    Default implementations with Derive     ---------------- */
    {
        // A tuple struct that can be printed
        #[derive(Debug)]
        struct _Inches(i32);
    }

    /* ---------------------       Trait Bounds       ----------------------- */
    {
        fn _largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
    }

    /* ----------------------       impl trait       ------------------------ */
    {
        fn _make_adder_function(y: i32) -> impl Fn(i32)->i32 {
            let closure = move |x: i32| { x + y };
            closure
        }
    }
    
    /* ----------------------     Trait Objects     ------------------------- */
    {
        pub trait _Draw {
            fn draw(&self);
        }

        pub struct _Screen {
            pub components: Vec<Box<dyn _Draw>>,
        }
    }



    /* -------------------     Operator Overloading     --------------------- */
    {
        use std::ops::Add;
        
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        } 
        
        impl Add for Point {
            type Output = Point;
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
    }


    /* -----------------------      Supertraits     ------------------------- */
    {
        use std::fmt;

        trait _Log: fmt::Display {
            fn log(&self) {
                let output = self.to_string();
                println!("Logging: {}", output);
            }
        }
    }

    /* ----------      Lifetimes in function signatures     ----------------- */
    {
        fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
    
    /* -----------      Lifetimes in struct definitions      ---------------- */
    {
        struct _User<'a> {
            full_name: &'a str,
        }
    }
    
    /* --------------------      Static lifetimes     ----------------------- */
    {
        let _s: &'static str = "Let’s Get Rusty!";
    }


    /* ---------------------------------------------------------------------- */
    /* ---------------    FUNCTION POINTERS & CLOSURES    ------------------- */
    /* ---------------------------------------------------------------------- */

    /* ---------------    Associated Functions and Methods   ---------------- */
    {
        struct _Point { x: i32, y: i32, }

        impl _Point {
            // Associated function
            fn _new(x: i32, y: i32) -> _Point {
                _Point { x: x, y: y }
            } 
            
            // Method (have "&self" parameter )
            fn _get_x(&self) -> i32 { self.x }
        }
    }


    /* ---------------------    Function Pointers     ----------------------- */
    {
        fn _do_twice(f: fn(i32)->i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
    }
        
    /* --------------------     Creating Closures     ----------------------- */
    {
        let _add_one = |num: u32| -> u32 {
            num + 1
        };
    }
    
    /* -------------------     Returning Closures     ----------------------- */
    {
        fn _add_one() -> impl Fn(i32)->i32 {
            |x| x + 1
        }
        
        fn _add_or_subtract(x: i32) -> Box<dyn Fn(i32)->i32> {
            if x > 10 {
                Box::new(move |y| y + x)
            } else {
                Box::new(move |y| y - x)
            }
        }
    }

    /* --------------------     Closure Traits     -------------------------- */
    //
    // • FnOnce - consumes the variables it captures from its enclosing scope.
    //
    // • FnMut - mutably borrows values from its enclosing scope.
    //
    // • Fn - immutably borrows values from its enclosing scope.
    //
        
    /* -----------------     Store Closure in Struct     -------------------- */
    {
        struct _Cacher<T>
            where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }
    }

    /* ------    Function that Accepts Closure or Function Pointer    ------- */
    {
        fn _do_twice<T>(f: T, x: i32) -> i32
            where T: Fn(i32)->i32
        {
            f(x) + f(x)
        }
    }


    /* ---------------------------------------------------------------------- */
    /* -----------------------      POINTERS      --------------------------- */
    /* ---------------------------------------------------------------------- */

    /* -----------------------     References     --------------------------- */
    {
        let mut num = 5;

        let _r1 = &num; // immutable reference
        
        let _r2 = &mut num; // mutable reference
    }
        
    /* ---------------------      Raw Pointers     -------------------------- */
    {
        let mut num = 5;
        
        let _r1 = &num as *const i32; // immutable raw pointer
        
        let _r2 = &mut num as *mut i32; // mutable raw pointer
    }

    /* ---------------------------------------------------------------------- */
    /* ----------------------      Smart Points      ------------------------ */
    /* ---------------------------------------------------------------------- */

    /* -----------   Box<T> - for allocating values on the heap    ---------- */
    {
        let _b = Box::new(5);   
    }
    
    /* --------    Rc<T> - multiple ownership with reference counting    ---- */
    {
        use std::rc::Rc;

        let a = Rc::new(5);
        let _b = Rc::clone(&a);
    }
        
    /* -----------      Ref<T>, RefMut<T>, and RefCell<T>      -------------- */
    {
        // enforce borrowing rules at runtime instead of compile time.
        use std::cell::RefCell;

        let r1 = RefCell::new(5);
        
        let _r2 = r1.borrow();  // Ref - immutable borrow

        let _r3 = r1.borrow();  // Ref - immutable borrow
        
        //let _r3 = r1.borrow_mut();   // RefMut - mutable borrow, !! Panic at this line !!
        
        //let _r4 = r1.borrow_mut();   // RefMut - second mutable borrow
    }

    /* -------------     Multiple owners of mutable data    ----------------- */
    {
        use std::rc::Rc;
        use std::cell::RefCell;

        let _x = Rc::new(RefCell::new(5));
    }


    /* ---------------------------------------------------------------------- */
    /* ----------------      Packages, Crates, Modules     ------------------ */
    /* ---------------------------------------------------------------------- */

    /* ----------------------      Definitions      ------------------------- */
    //
    // • Packages - A Cargo feature that lets you build, test, and share crates.
    // 
    // • Crates - A tree of modules that produces a library or executable.
    //
    // • Modules and use - Let you control the organization, scope, and privacy of paths.
    //
    // • Paths - A way of naming an item, such as a struct, function, or module.
    //

    /* ---------------------    Creating Crates     ------------------------- */
    //
    // $ cargo new my-project           // to create a new package with a binary crate
    //
    // $ cargo new my-project --lib     // to create a new package with a library crate


    /* ----------------      Defining & using Modules     ------------------- */
    // fn some_function() {}
    //
    // mod outer_module { // private module
    //     pub mod inner_module { // public module
    //         pub fn inner_public_function() {
    //             super::super::some_function();
    //         } 
    //         fn inner_private_function() {}
    //     }
    // }
    //
    // fn main() {
    //     // absolute path
    //     crate::outer_module::inner_module::inner_public_function();
    //
    //     // relative path path
    //     outer_module::inner_module::inner_public_function();
    //
    //     // bringing path into scope
    //     use outer_module::inner_module;
    //     inner_module::inner_public_function();
    // }
    
    /* --------------     Re-exporting with 'pub use'     ------------------- */
    //
    // pub use crate::outer_module::inner_module;
    //

    /* ----------------      Renaming with as Keyword     ------------------- */
    {
        use std::fmt::Result;
        use std::io::Result as IoResult;
    }


    /* -----------    Defining modules in separate files     ---------------- */
    // -- src/lib.rs --
    // mod my_module;
    // pub fn some_function() {
    //      my_module::my_function();
    // }
    // --
    // 
    // -- src/my_module.rs --
    // pub fn my_function() {}
    // --

}
