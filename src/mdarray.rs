pub struct Array3D<T> {
    contents: Vec<T>,
    size: (usize, usize, usize)
}

pub struct Array3DIterator<T> {
    iteree: Array3D<T>,
    x: usize,
    y: usize,
    z: usize
}

impl<T> Iterator for Array3DIterator<T> where T:Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.x += 1;
        if self.x >= self.iteree.size.0 {
            self.x = 0;
            self.y += 1;
            if self.y >= self.iteree.size.1 {
                self.y = 0;
                self.z += 1;                   
            }
        }

        Some (*(self.iteree.get(self.x, self.y, self.z)?))
    }
}


impl<T> Array3D<T> {

    pub fn new(v: T, width: usize, height: usize, depth: usize) -> Array3D<T> where T: Copy {
        Array3D {
            contents: vec![v ; width * height * depth],
            size: (width,height,depth)
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&T> {
        let (width,height,depth) = self.size;
        if x < width && y < height && z < depth {
            Some (&self.contents[x + (y + z * depth) * width])
        }
        else {
            None
        }
    }
    
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut T> {
        let (width,height,depth) = self.size;
        if x < width && y < height && z < depth {
            Some (&mut self.contents[x + (y + z * depth) * width])
        }
        else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        if let Some(dest) = self.get_mut(x, y, z) {
            *dest = value;
        }
    }

    pub fn iter(self) -> Array3DIterator<T> {
        Array3DIterator {
            iteree: self, x: 0, y: 0, z: 0
        }
    }
}

