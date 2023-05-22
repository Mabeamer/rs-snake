fn main() {
    pub enum Direction {
        Up,
        Right,
        Down,
        Left
    }

    use crate::direction::Direction;

    pub enum Command{
        Quit,
        Turn(Direction),
    }
    //strut declares a custom type holding unsigned 16 bit integers (0-65535)
    pub struct Point{
        pub x: u16,
        pub y: u16,
    }


    impl Point{
        //creating new function object? returning self.
        pub fn new(x: u16, y: u16) -> Self{
            Self {x, y}
        }

        pub fn transform(&self, direction: Direction, times: u16) -> self{
            let times = times as i16;
            let transformation = match direction {
                Direction::Up => (0, -times),
                Direction::Right => (times, 0),
                Direction::Down => (0, times),
                Direction::Left => (-times, 0),
            };
            //?????
            self::new(
                Self::transform_value(self.x, transformation.0),
                Self::transform_value(self.y, transformation.1),
            )
        }   

        fn transform_value(value: u16, by: i16) -> u16 {
            if by.is_negative() && by.abs() as u16 > value {
                panic!("Transforming vaue {} by {} would result in a negative number", value, by);
            } else{
                //????????? what the hell is going on in this language man
                (value as i16 + by) as u16
            }
        }

    }

}
