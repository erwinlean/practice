fn main (){

    // Check all types of types...
    /*
    8-bit	i8	    u8	 	 
    16-bit	i16	    u16	 	 
    32-bit	i32  	u32	 	 
    64-bit	i64	    u64	 	 
    128-bit	i128	u128	
            f32 
            f64 
            char 
            String
            &str	 
            bool
    architecture-dependent	isize	usize
    */

    // simple variables
    let mut a_number: i8 = -1  ; // use mut for mutate variable, otherway cant change the variable
    println!("hello worldd! {}", a_number);
    a_number = 5;
    println!("hello worldd! {}", a_number+a_number);
    // shadow variable can mutate, "sobre escribiendo valores"
    let shadow_num: u8 = 4;
    println!("{}", shadow_num);
    let shadow_num: u32 = 34; // the value change, but the old values get destroyed
    println!("{}", shadow_num);

    // tuplas
    let new_tupla: (&str, u8, f32, char) = ( "asd", 5, 4.2 , '2');
    println!("la tupla: {}", new_tupla.0);

    // struct must strat with struct and capitalized (there are 3 types of struct)
    // One
    struct TestStruct { age: u8, idk : String, letter: char, numbers: u64}
    let test_one = TestStruct {
        age: u8::from(5),
        idk: String::from("testeando"),
        letter : char::from('2'),
        numbers: 2423456546452342 as u64
    };
    println!("clasic struct: {} {} {} {}", test_one.age, test_one.idk, test_one.letter, test_one.numbers);
    // Two
    struct TestStruct2(u8, char, String);
    let test_two = TestStruct2(
        8,
        'a',
        String::from("adsad asdas dasd asd da sdas342423 asd")
    );
    println!("new struct: {}", test_two.0);
    // three
    // latter on

    // Enum  
    #[derive(Debug)]
    struct KeyPress(String, char);
    let key_press = KeyPress(
        String::from("test"),
        'a'
    );

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }
    let mouse_click = MouseClick{
        x: i64::from(2),
        y: i64::from(3)
    };
    println!("{}{}", mouse_click.x, mouse_click.y);

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
    let we_load = WebEvent::WELoad(true);
    let we_keys = WebEvent::WEKeys(key_press);
    let we_click = WebEvent::WEClick(mouse_click);

    println!("\nWebEvent enum structure: \n {:#?} \n {:#?} \n {:#?}", we_load, we_click, we_keys);

    // Functions
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));

    // Exercise one
    #[derive(Debug)]
    struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
    }
    #[derive(Debug)]
    enum Transmission {
        Manual(bool),
        // SemiAuto(bool),
        // Automatic(bool)
    }
    let manual_car = Car {
        color: String::from("gris"),
        transmission: Transmission::Manual(false),
        convertible: bool::from(false),
        mileage: 50 as u32
    };
    println!("properties: {}{}{}{:#?}", manual_car.color,manual_car.convertible,manual_car.mileage,manual_car.transmission);

    // Matrix(array) uses
    matrix_uses();

    // Vectors
    vectors();

    // Exercise
    exercise_two();

    // Exercise
    exercise_three();

    // Hash map
    hash_map();

    // Exercise
    exercise_four_and_five();

    // Bucles
    for_while_loop();
    
    // Errors/Warnings
    errors();

    // Match
    errors_and_match();

    // unwrap expect, assert
    unwrap_expect_asserts_usage();

    // Exercise six
    options_ausence();
}

// Funcion create example
fn divide_by_5(num: u32) -> u32 {
    num / 5
}

// Matrix usage
fn matrix_uses(){
    // contains the same type and never changes the lenght cant change
    let matrix: [&str; 4] = ["uno", "dos", "tres", "a"];
    let matrix2: [i32; 3] = [2; 3]; // Esto indica que el array de lenght 3 y su valor en cada caso es 2
    println!("matrix: {}", matrix[1]);

    println!("other matrix: {}", matrix2[0]);
}

// Vectors usage
fn vectors(){
    // Vector only contain one type, the lenght can change.
    // Declare vector, initialize with three values
    let three_nums: Vec<i32> = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes: Vec<i32> = vec![0; 5];
    println!("Zeroes: {:?}", zeroes); 

    // Mutable vectors
    let mut mutable_vector: Vec<String> = Vec::new();
    mutable_vector.push(String::from("asd"));
    println!("mutable vector: {}", mutable_vector[0]);

    // Declare vector, initialize with three values
    let mut index_vec: Vec<i32> = vec![15, 3, 46];
    index_vec.push(2);
    let three: i32 = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // if condition example
    let num: i32 = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("Range: {}", out_of_range);
}

// Exercise
fn exercise_two(){
    #[derive(PartialEq, Debug)]
    // Declare Car struct to describe vehicle with four named fields
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        mileage: Age
    }
    // let mileage_tupla: (u32, u32) = (450, 1992); > unused
    #[derive(PartialEq, Debug)]
    // Declare enum for Car transmission type
    enum Transmission { Automatic(bool) }
    #[derive(PartialEq, Debug)]
    enum Age { New(bool) /*, Usage(u8) */}
    let car = Car {
        color: String::from("green"),
        motor: Transmission::Automatic(true),
        roof: bool::from(false),
        mileage: Age::New(true)
    };
    println!("the car {:#?}", car)
}

// Exercise three
fn exercise_three() {
    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    #[derive(PartialEq, Debug)]
    enum Age {
        New,
        Used,
    }

    //////////////////////////////////////////////////

    fn car_quality(miles: u32) -> (Age, u32) {
        if miles > 0 {
            return (Age::Used, miles);
        }

        (Age::New, 0)
    }

    //////////////////////////////////////////////////

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        if car_quality(miles).0 == Age::Used {
            if roof {
                println!(
                    "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                    motor, color, miles
                );
            }
        }

        // Create a new "Car" instance as requested
        // - Bind first three fields to values of input arguments
        // - Bind "age" to tuple returned from car_quality(miles)
        Car {
            color,
            motor,
            roof,
            age: car_quality(miles),
        }
    }

    fn car_main() {
        // Car order #1: New, Manual, Hard top
        car_factory(String::from("Orange"), Transmission::Manual, true, 0);

        // Car order #2: Used, Semi-automatic, Convertible
        car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

        // Car order #3: Used, Automatic, Hard top
        car_factory(String::from("White"), Transmission::Automatic, true, 3000);
    }

    car_main();
}

// Mapas hash, can be mutate
use std::collections::HashMap; // call the library for hash maps
fn hash_map(){
    let mut reviews: HashMap<String, String> = HashMap::new(); // create the hashmap from the library  indicates is mutate

    // Inser value to the hash map
    // K: represents key, v: value.
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));
    reviews.insert(String::from("testing"), String::from("Test."));

    // Access to the hash map
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
    // test access
    println!("{:?}", reviews.get("testing")); // call hashmap and get, with the key for obtein the value

    // Deleting
    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);
    reviews.remove("testing");

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
    println!("{:?}", reviews.get("testing")); 
}

// Exercise four & five
fn exercise_four_and_five(){
    #[derive(PartialEq, Debug)]
    struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

    #[derive(PartialEq, Debug)]
    enum Transmission { Manual, SemiAuto, Automatic }

    #[derive(PartialEq, Debug)]
    enum Age { New, Used }

    // Get the car quality by testing the value of the input argument
    // - miles (u32)
    // Return tuple with car age ("New" or "Used") and mileage
    fn car_quality (miles: u32) -> (Age, u32) {

        // Check if car has accumulated miles
        // Return tuple early for Used car
        if miles > 0 {
            return (Age::Used, miles);
        }

        // Return tuple for New car, no need for "return" keyword or semicolon
        (Age::New, miles)
    }

    // Build "Car" using input arguments
    fn car_factory(order: i32, miles: u32) -> Car {
        let mut colors = vec!["Blue", "Green", "Red", "Silver"];

        // Prevent panic: Check color index for colors array, reset as needed
        // Valid color = 1, 2, 3, or 4
        // If color > 4, reduce color to valid index
        {
            let mut color = order as usize;
            colors.push("caca");
            if color >= colors.len(){
                color = color % colors.len();
                print!("color: {}", color);
            }
        }
        // Add variety to orders for motor type and roof type
        let mut motor = Transmission::Manual;
        let mut roof: bool = true;
        if order % 3 == 0 {          // 3, 6, 9
            motor = Transmission::Automatic;
        } else if order % 2 == 0 {   // 2, 4, 8, 10
            motor = Transmission::SemiAuto;
            roof = false;
        }                            // 1, 5, 7, 11

        // Return requested "Car"
        let color_index = (order % colors.len() as i32).abs() as usize;
        Car {
            color: String::from(colors[color_index]),
            motor: motor,
            roof: roof,
            age: car_quality(miles)
        }
    }

    fn main_car() {
        let mut orders: HashMap<i32, Car > = HashMap::new();

        // Exercise four deleted
        // Initialize counter variable
        let mut miles = 0;
        let mut car: Car;
        
        for order in 1..=5{ // Hardcodeado numero de iteraciones?
            // Notes: for can "create" the variable in this case order himself doesnt need to create
    
            car = car_factory(order, miles);
            orders.insert(order, car);

            // Reset miles for order variety
            if miles == 2100 {
                miles = 0;
            } else {
                miles = miles + 700;
            }
        }

        for (order, car) in orders.iter() {
            println!("Car order {}: {:?}", order, car);
        }

    }

    //reviews.insert(String::from("testing"), String::from("Test."));
    main_car()
}

// Iteraccions examples
fn for_while_loop(){

    // Loop until "break;"
    loop {
        println!("We loop forever!");
        break;
    }
    let mut counter :i32 = 1;
    let stop_loop: i32 = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break {}.", stop_loop);

    // While
    let mut counter_1: i32 = 0;
    while counter_1 < 5 {
        println!("We loop a while...");
        counter_1 = counter_1 + 1;
    }

    // For
    let big_birds: [&str; 3] = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
}

// Errors
fn errors(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let first = fruits.get(0);
    println!("{:?}", first);
    let third = fruits.get(2);
    println!("{:?}", third);
    
    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
}

// Match
fn errors_and_match(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) { // match serch for the index we call (this case with error for the 99)
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    // En este caso, nos gustaría ignorar la variante None y todos los valores de Some<u8> que no coincidan con Some(7)
    let a_number: Option<u8> = Some(7);
    match a_number { // like an if?
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }
    // same as
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }
}

// Unwrap, expect and assert_eq
// assert_eq means: Asserts that two expressions are equal to each other (using PartialEq)
fn unwrap_expect_asserts_usage(){
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");
    
    //let empty_gift: Option<&str> = None;
    //assert_eq!(empty_gift.unwrap(), "candy");

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}

// Exercise six > todo ! domani
fn options_ausence(){
    struct Person {
        first: String,
        middle: Option<String>,
        last: String,
    }
    
    fn build_full_name(person: &Person) -> String {
        let mut full_name = String::new();
        full_name.push_str(&person.first);
        full_name.push_str(" ");
    
        // TODO: Implement the part of this function that handles the person's middle name.
    
        full_name.push_str(&person.last);
        full_name
    }
    
    fn main() {
        let john = Person {
            first: String::from("James"),
            middle: Some(String::from("Oliver")),
            last: String::from("Smith"),
        };
        assert_eq!(build_full_name(&john), "James Oliver Smith");
    
        let alice = Person {
            first: String::from("Alice"),
            middle: None,
            last: String::from("Stevens"),
        };
        assert_eq!(build_full_name(&alice), "Alice Stevens");
    
        let bob = Person {
            first: String::from("Robert"),
            middle: Some(String::from("Murdock")),
            last: String::from("Jones"),
        };
        assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
    }
}