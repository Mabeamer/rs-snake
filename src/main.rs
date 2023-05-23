//following guide https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html
//we're going to error out up here for some reason?
//unresolved import 'crate::direction::Direction;
use crate::snake::Snake;
use crate::point::Point;
use crate::direction::Direction;




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

        //need a better understanding of this function
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

            //let this sink in

            //(0..length) creteas std::ops::Range, representing 0 - length
            // going over the full vector of body. So the range of the list?
            let body: Vec<Point> = (0..length)
            //.into allows us to iterate over the range
                .into_iter()
                //.map creates std::iter::Map, allows us to apply a transformation to the u16 value.
                //makes the iterator mutatable (?)
                //uses start transform to return a new point for every I, 
                .map(|i| start.transform(opposite, i ))
                //collect executes the operation and brings the resulte of vec of type Point representing the body of the snake
                .collect();

            //passing in the new direction and setting the base digestion of false
            Self { body, direction, digesting: false}    
        }

        //what is going on here man
        //update: are we grabbing the direct values above?
        //udate no2: getters and setters for building our function, need to learn how to think about this on my own.

        //returns the first Point of the body field, this is the head of the snake.
        pub fn get_head_point(&self) -> Point {
            //"the firss method returns an Option<&Point> || Represents either Some or None" ?????????
            //rust does not have a concept of null so it uses Option to represent null
            //this is referencing a value within the vector
            self.body.first().unwrap().clone()

        }

        //returns a clone of the first point of the body field that represents the head of the snake
        pub fn get_body_points(&self) -> Vec<Point>{
            self.body.clone()   
        }
        //returns a clone of the direction field
        pub fn get_direction(&self) -> Direction {
            self.direction.clone()
        }
        //this is the check to see if the snakes body part contains a point
        pub fn contains_point(&self, point: &Point) -> bool {
            self.body.contains(point)
        }



        //using mut to access and change the snake itself
        pub fn slither(&mut self){
            //this looks to be the important statement to learn
            self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1));
            //if it is digesting it removes one from its head
            if !self.digesting{
                self.body.remove(self.body.len()-1);
            }else{
                self.digesting = false;
            }

        }
        //updates the direction field with the current direction
        pub fn set_direction(&mut self, direction: Direction){
            self.direction = direction;
        }
        //setting digesting to true
        pub fn grow(&mut self){
            self.digesting = true;
        }

    }

    //??
    impl Direction{
        pub fn opposite(&self) -> Self {
            match self{
                Self::Up => Self::Down,
                Self::Right => Self::Left,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
            }
        }
    }

    

}
