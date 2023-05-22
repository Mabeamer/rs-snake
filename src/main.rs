//following guide https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html
//we're going to error out up here for some reason?
//unresolved import 'crate::direction::Direction;

use crate::direction::Direction;
use crate::point::Point;

fn main() {
    //use crate::direction::Direction;
    //use crate::point::Point;
    pub enum Direction {
        Up,
        Right,
        Down,
        Left
    }

    pub enum Command{
        Quit,
        Turn(Direction),
    }
    //strut declares a custom type holding unsigned 16 bit integers (0-65535)
    pub struct Point{
        pub x: u16,
        pub y: u16,
    }

    //"use an impl to add methods and functions to the struct"
    impl Point{
        //creating new function object? returning self.
        pub fn new(x: u16, y: u16) -> Self{
            Self {x, y}
        }

        pub fn transform(&self, direction: Direction, times: u16) -> Self{
            let times = times as i16;
            let transformation = match direction {
                Direction::Up => (0, -times),
                Direction::Right => (times, 0),
                Direction::Down => (0, times),
                Direction::Left => (-times, 0),
            };
            //?????
            //Update: this is referencing the constuctor created on line 24
            self::new(
                Self::transform_value(self.x, transformation.0),
                Self::transform_value(self.y, transformation.1),
            )
        }   

        fn transform_value(value: u16, by: i16) -> u16 {
            if by.is_negative() && by.abs() as u16 > value {
                panic!("Transforming vaue {} by {} would result in a negative number", value, by);
            } else{
                //????????? what is going on in this language man
                (value as i16 + by) as u16
            }
        }

    }
    //error out here this could be an issue

    let point = Point { x: 1, y: 2};

    //return Point { x: x, y: y }; 


    let a = Point::new(1,2);
    //figure out what tranform does
    //Update: Direction::(input), times
    let b = a.transform(Direction::Up, 1);

    //go bakc and read ownership and borrowing you did not grasp that at all.

    pub struct Snake {
        //body is holding a vector over the point ype. List that is holding values of "Point"
        body: Vec<Point>,
        //Representing the direction the snake is currently facing using the direction enum
        direction: Direction,
        //T/F representation to check if the snake has eaten food.
        digesting: bool,
    }

    //err here
    //read out what's going on in here
    impl Snake {
        //creating a new instance of the snake struct
        pub fn new(start: Point, length: u16, direction: Direction) -> Self{

            //calling the opposite method, this returns the opposite value of the current one (I.E LEFT = RIGHT)
            let opposite = direction.opposite();

            //(0..length) creteas std::ops::Range, representing 0 - length
            // going over the full vector of body. So the range of the list?
            let body: Vec<Point> = (0..length)
            //.into allows us to iterate over the range
                .into_iter()
                //.map creates std::iter::Map, allows us to apply a transformation to the u16 value.
                //makes the 
                .map(|i| start.transform(opposite, i ))
                .collect();


            Self { body, direction, digesting: false}    
        }
    }




}
