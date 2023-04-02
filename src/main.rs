// fn main() {

//     fn plus_one(x: Option<i32.) -> Option<i32> {
//         match x {
//             None => {
//                 println!("No value");
//                 None;
//             }
//             Some(i)) => {
//                 println!("Initial value: {}", i);
//                 Some(i + 1);
//         }
//     }


//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None::<T>);

//     // println!{"{}", five.to_string()};
//     // println!{"{}", six.to_string()};
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);