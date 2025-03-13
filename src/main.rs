fn main() {
    // println!("Hello, world!");

    // let brand_1 = String::from("FORD");
    //
    // let car_1 = Car::from(brand_1);
    //
    // println!("car_1: {:?}", car_1); // car_1: Car { Brand: "FORD" }
    //
    // let brand_from_car: String = car_1.into();
    // println!("brand_from_car: {:?}", brand_from_car);
    //
    // let brand_car_2 = String::from("FORD");
    // let car_2 = Car::try_from(brand_car_2).unwrap();

    let brand_1 = String::from("FORD");
    let car_1 = Car::try_from(brand_1).unwrap();
    println!("car_1: {:?}", car_1); // Car { brand: "FORD" }

    let car_2 = Car::try_from(String::from("")).unwrap_err();
    println!("car_2: {:?}", car_2); // "Invalid brand name"

    let brand_1: Result<String, &'static str> = car_1.try_into();
    println!("brand_1: {:?}", brand_1.unwrap()); // "FORD"

    let car_3 = Car::try_from(String::from("KIA")).unwrap();
    let brand_2: Result<String, &'static str> = car_3.try_into();
    println!("brand_2: {:?}", brand_2.unwrap_err()); // "Not allowed!"
}

#[derive(Debug)]
struct Car {
    brand: String,
}

impl From<String> for Car {
    fn from(brand: String) -> Self {
        Car { brand }
    }
}

impl From<Car> for String {
    fn from(value: Car) -> Self {
        String::from(value.brand)
    }
}

impl TryFrom<String> for Car {
    type Error = &'static str;

    fn try_from(brand: String) -> Result<Self, Self::Error> {
        if brand.is_empty() {
            Err("Invalid band name")
        } else {
            Ok(Car { brand })
        }
    }
}

impl TryFrom<Car> for String {
    type Error = &'static str;

    fn try_from(car: Car) -> Result<Self, Self::Error> {
        if car.brand == "KIA" || car.brand == "BMW" {
            Err("Not allowed!")
        } else {
            Ok(String::from(car.brand))
        }
    }
}
