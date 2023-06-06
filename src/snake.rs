#[derive(Debug)]
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