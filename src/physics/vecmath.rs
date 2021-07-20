#[derive(Clone, PartialEq, Debug)]
pub struct PhysVec{pub x: f32, pub y: f32}

impl PhysVec {
    /*
        a = [x, y]
    */
    // create new "vector"
    pub fn new(x: f32, y: f32) -> Self {PhysVec{x, y}}
    // return tuple of x and y
    pub fn raw(&self) -> (f32, f32) {(self.x, self.y)}
    // negate values
    pub fn invert(&self) -> PhysVec{
        PhysVec{x: -self.x, y: -self.y}
    }
    /*
        A change in position as a vector is represented by:
        a = d*n
        d is the straight line distance of the change, aka the magnitude
        magnitude = d = |a| = sqrt(x^2+y^2) = pythagorean theorem
    */
    pub fn magnitude(&self) -> f32 {
        let x = self.x; let y =self.y;
        (x*x+y*y).sqrt()
    }
    pub fn magnitude_sq(&self) -> f32 {
        let x = self.x; let y =self.y;
        x*x+y*y
    }
    /*
        continuing off the equation a = d*n, n is the direction of the 
        change whose straight line distance is always 1. It is also known as the unit-length
        direction of a.
        a = a/|a| = a/d
    */
    pub fn normalize(&self) -> PhysVec{
        let length = self.magnitude();
        if length > 0f32 {
            self.dot_product(1f32/length)
        }
        else {PhysVec::new(0.0,0.0)}
    }
    // replace values
    pub fn replace(&mut self, other: &PhysVec) {
        self.x = other.x;
        self.y = other.y;
    }
    // multiply by a scalar value
    pub fn dot_product(&self, scalar: f32) -> PhysVec {
        let (x, y) = self.raw();
        PhysVec{x: x*scalar, y: y*scalar}
    }
    // perform magnitude product and place values into self
    pub fn dot_replace(&mut self, scalar: f32) {
        self.replace(&self.dot_product(scalar));
    }
    // multiply components
    pub fn component_product(&self, other: &PhysVec) -> PhysVec {
        let (x, y) = self.raw();
        let (xp, yp) = other.raw();
        PhysVec{x: x*xp, y: y*yp}    
    }
    // perform component product and place values into self
    pub fn component_replace(&mut self, other: &PhysVec) {
        self.replace(&self.component_product(&other));
    }
    // perform vector addition
    pub fn add(&self, other: &PhysVec) -> PhysVec {
        let (x, y) = self.raw();
        let (xp, yp) = other.raw();
        PhysVec{x: x+xp, y: y+yp}
    }
    // perform vector subtraction
    pub fn sub(&self, other: &PhysVec) -> PhysVec {
        self.add(&other.invert())
    }
    // calculate scalar product
    pub fn scalar_product(&self, other: &PhysVec) -> f32 {
        let (x, y) = self.raw();
        let (xp, yp) = other.raw();
        x*xp+y*yp
    }
    // calculate scalar product, add product to self, store
    pub fn add_scaled_product(&mut self, other: &PhysVec, scale: f32) {
        let mut mutCopy = other.clone();
        mutCopy.dot_replace(scale); // other*scale
        self.add_vec(&mutCopy); // self = self+(other*scale)
    }
    // add vector to self and store
    pub fn add_vec(&mut self, other: &PhysVec) {
        self.replace(&self.add(&other));
    }
    // add vector to self and store
    pub fn add_scalar(&mut self, scalar: f32) {
        self.replace(&self.add(&PhysVec::new(scalar, scalar)));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn testInit() {
        let vec = PhysVec::new(0f32, 0f32);

        assert_eq!(vec.x, 0f32);
        assert_eq!(vec.y, 0f32);
    }

    #[test]
    pub fn testRaw() {
        let vec = PhysVec::new(0f32, 0f32);

        assert_eq!(vec.raw(), (0f32, 0f32));
    }

    #[test]
    pub fn testAdd() {
        let mut vec = PhysVec::new(0f32, 0f32);
        let add = vec.add(&PhysVec::new(4.3, 7.987));
        assert_eq!(add.raw(), (4.3, 7.987));
    }

    #[test]
    pub fn testSub() {
        let mut vec = PhysVec::new(0f32, 0f32);
        let sub = vec.sub(&PhysVec::new(4.3, 7.987));
        assert_eq!(sub.raw(), (-4.3, -7.987));
    }

    #[test]
    pub fn testMagnitude() {
        let mut vec = PhysVec::new(4f32, 0f32);
        assert_eq!(vec.magnitude(), 4f32);
    }

    #[test]
    pub fn testAddScalar() {
        let mut vec = PhysVec::new(4f32, 0f32);
        vec.add_scalar(5.5);
        assert_eq!(vec.raw(), (9.5, 5.5));
    }

    #[test]
    pub fn testScalarProduct() {
        let mut vec = PhysVec::new(4f32, 0f32);
        let res = vec.scalar_product(&PhysVec::new(3f32,3f32));
        assert_eq!(res, 12f32);
    }

    #[test]
    pub fn testAddScalarProduct() {
        let mut vec = PhysVec::new(4f32, 0f32);
        vec.add_scaled_product(&PhysVec::new(3f32,3f32), 2f32);
        assert_eq!(vec.raw(), (10f32, 6f32));
    }
}