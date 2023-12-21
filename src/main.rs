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

    // Exercise
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
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

fn matrix_uses(){
    // contains the same type and never changes the lenght cant change
    let matrix: [&str; 4] = ["uno", "dos", "tres", "a"];
    let matrix2: [i32; 3] = [2; 3]; // Esto indica que el array de lenght 3 y su valor en cada caso es 2
    println!("matrix: {}", matrix[1]);

    println!("other matrix: {}", matrix2[0]);
}

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

    // Exercise
    exercise_two();

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

    // Exercise
    exercise_three();
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
