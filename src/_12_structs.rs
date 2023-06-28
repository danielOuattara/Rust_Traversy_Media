// Structs : use to create custom data types

/*  Traditional Struct : 1
-------------------------- */
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/* tuple Struct :  2
---------------------- */
struct ColorTuple(u8, u8, u8);

/* struct with function implementation : 3
------------------------------------------- */

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct a person
    fn create_person(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get person full name
    fn get_full_name(&self) {
        let full_name = format!("The full name is {} {}", self.first_name, self.last_name);
        println!("{}", full_name);
    }

    // update person first name
    fn update_first_name(&mut self, new_first_name: &str) {
        self.first_name = new_first_name.to_string()
    }

    // update person last name
    fn update_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string()
    }

    // name as a tuple

    fn name_as_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

//-----------------------------------------------------------------------

pub fn run() {
    /* --- 1 */
    let mut color_1 = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!(
        "color_1 is rgb({}, {}, {})",
        color_1.red, color_1.green, color_1.blue
    );

    color_1.red = 200;

    println!(
        "color_1 is rgb({}, {}, {})",
        color_1.red, color_1.green, color_1.blue
    );

    println!("------------------------------------------------");

    /* --- 2 */
    let mut color_tuple_1 = ColorTuple(0, 255, 0);

    println!(
        "color_tuple_1 is rgb({}, {}, {})",
        color_tuple_1.0, color_tuple_1.1, color_tuple_1.2
    );

    color_tuple_1.0 = 200;

    println!(
        "color_tuple_1 is rgb({}, {}, {})",
        color_tuple_1.0, color_tuple_1.1, color_tuple_1.2
    );

    println!("-----------------------------------------------");

    /* --- 3 */
    let person_1 = Person {
        first_name: "Jana".to_string(),
        last_name: String::from("Doe"),
    };

    println!("person_1 is {} {}", person_1.first_name, person_1.last_name);

    let mut person_2 = Person::create_person("John", "Doe");
    println!("person_2 is {} {}", person_2.first_name, person_2.last_name);

    person_2.get_full_name(); // John Doe

    person_2.update_first_name("Simon");
    person_2.get_full_name(); // John Simon

    person_2.update_last_name("Django");
    person_2.get_full_name(); //  Simon Django

    println!("{:?}", person_2.name_as_tuple());
}
