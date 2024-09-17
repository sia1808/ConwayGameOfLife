// Sia Sharma
// Collaborators: Om Italiya, Nikita Salkar

use std::ops::Neg;

// Defining the struct Point<T>
struct Point<T> {
    x: T,
    y: T,
}

// Clockwise and Counterclockwise Functions
impl<T:Copy + Neg<Output = T>> Point<T> {
    fn clockwise(&self) -> (T,T) {
        let mut x = self.x;
        let mut y = self.y;
        // Change the coordinate according to the rotation
        return (y, -x);
    }
    fn counterclockwise(&self) -> (T,T) {
        let mut x = self.x;
        let mut y = self.y;
        // Change the coordinate according to the rotation
        return (-y,x);
    }
}

// Main function for 2 examples of such points
fn main() {
    // Defining Point 1, which is of type f64
    let p1 = Point{x:3.25 as f64, y:2.75 as f64};
    //Defining Point 2, which is of type i32 (no floating point numbers)
    let p2 = Point{x:3 as i32, y:2 as i32};
    println!("Point 1 rotated clockwise is {:?}", p1.clockwise());
    println!("Point 1 rotated counterclockwise is {:?}", p1.counterclockwise());
    println!();
    println!("Point 2 rotated clockwise is {:?}", p2.clockwise());
    println!("Point 2 rotated counterclockwise is {:?}", p2.counterclockwise());

}

//Test Functions
#[test]
fn test1() {
    let testpoint = Point{x:2,y:3};
    assert_eq!(testpoint.clockwise(), (3,-2));
}

#[test]
fn test2() {
    let testpoint2 = Point{x:5,y:6};
    assert_eq!(testpoint2.counterclockwise(),(-6,5))
}

