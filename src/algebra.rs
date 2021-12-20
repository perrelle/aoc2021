
#[derive(Debug,PartialEq,Eq,Clone,Hash,PartialOrd,Ord)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0, y: 0, z: 0 };
    pub const X: Vector = Vector { x: 1, y: 0, z: 0 };
    pub const Y: Vector = Vector { x: 0, y: 1, z: 0 };
    pub const Z: Vector = Vector { x: 0, y: 0, z: 1 };
    pub const AXES : [Vector ; 6] = [
        Vector::X, Vector::Y, Vector::Z,
        Vector::neg(&Vector::X),
        Vector::neg(&Vector::Y),
        Vector::neg(&Vector::Z) ];

    pub fn is_zero(self: &Vector) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    pub fn norm1(self: &Vector) -> i32 {
        [self.x, self.y, self.z].into_iter().map(i32::abs).sum()
    }

    pub fn norm_inf(self: &Vector) -> i32 {
        *[self.x, -self.x, self.y, -self.y, self.z, -self.z]
            .iter().max().unwrap()
    }

    pub fn add(self: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z
        }
    }

    pub fn sub(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z
        }
    }

    pub const fn neg(v: &Vector) -> Vector {
        Vector {
            x: -v.x,
            y: -v.y,
            z: -v.z
        }
    }

    pub fn inner_product(v1: &Vector, v2: &Vector) -> i32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
     }

    pub fn outer_product(v1: &Vector, v2: &Vector) -> Vector {
       Vector { 
           x: v1.y * v2.z - v1.z * v2.y,
           y: v1.z * v2.x - v1.x * v2.z,
           z: v1.x * v2.y - v1.y * v2.x
       } 
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

use std::ops::*;

macro_rules! forward_ref_binop {
    (impl $imp:ident, $m:ident, $base:ident, $method:ident for $t:ty, $u:ty, $r:ty) => {
        impl $imp<$u> for $t {
            type Output = $r;

            #[inline]
            fn $m(self, other: $u) -> <$t as $imp<$u>>::Output {
                $base::$method(&self, &other)
            }
        }

        impl<'a> $imp<$u> for &'a $t {
            type Output = $r;

            #[inline]
            fn $m(self, other: $u) -> <$t as $imp<$u>>::Output {
                $base::$method(self, &other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = $r;

            #[inline]
            fn $m(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $base::$method(&self, other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = $r;

            #[inline]
            fn $m(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $base::$method(self, other)
            }
        }
    }
}

forward_ref_binop! { impl Add, add, Vector, add for Vector, Vector, Vector }
forward_ref_binop! { impl Sub, sub, Vector, sub for Vector, Vector, Vector }
forward_ref_binop! { impl BitXor, bitxor, Vector, outer_product for Vector, Vector, Vector }
forward_ref_binop! { impl Mul, mul, Vector, inner_product for Vector, Vector, i32 }

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self: Vector) -> Vector {
        Self::neg(&self)
    }
}

#[derive(Debug,Clone)]
pub struct LinearMap {
    pub x: Vector,
    pub y: Vector,
    pub z: Vector
}

impl LinearMap {
    pub const ID: LinearMap = LinearMap {
        x: Vector {x: 1, y: 0, z: 0},
        y: Vector {x: 0, y: 1, z: 0},
        z: Vector {x: 0, y: 0, z: 1}
    };

    pub fn apply(self: &LinearMap, v: &Vector) -> Vector {
        Vector {
            x: &self.x * v,
            y: &self.y * v,
            z: &self.z * v
        }
    }

    pub fn transpose(self: &LinearMap) -> LinearMap {
        LinearMap {
            x: Vector{ x: self.x.x, y: self.y.x, z: self.z.x },
            y: Vector{ x: self.x.y, y: self.y.y, z: self.z.y },
            z: Vector{ x: self.x.z, y: self.y.z, z: self.z.z }
        }
    }

    pub fn invert(self: &LinearMap) -> LinearMap {
        // Incorrect if det != 1
        LinearMap {
            x: &self.y ^ &self.z,
            y: &self.z ^ &self.x,
            z: &self.x ^ &self.y
        }.transpose()
    }

    pub fn compose(l1: &LinearMap, l2: &LinearMap) -> LinearMap {
        let l2t = l2.transpose();
        LinearMap {
            x: l1.apply(&l2t.x),
            y: l1.apply(&l2t.y),
            z: l1.apply(&l2t.z)
        }.transpose()
    }
}

forward_ref_binop! { impl Mul, mul, LinearMap, compose for LinearMap, LinearMap, LinearMap }

#[derive(Debug,Clone)]
pub struct AffineMap {
    pub linear: LinearMap,
    pub translation: Vector
}

impl AffineMap {
    pub const ID: AffineMap = AffineMap {
        linear: LinearMap::ID,
        translation: Vector::ZERO
    };

    pub fn apply(self: &AffineMap, v: &Vector) -> Vector {
        self.linear.apply(v) + &self.translation
    }

    pub fn compose(a1: &AffineMap, a2: &AffineMap) -> AffineMap {
        AffineMap {
            linear: &a1.linear * &a2.linear,
            translation: a1.linear.apply(&a2.translation) + &a1.translation
        }
    }

    pub fn affine_invert(a: &AffineMap) -> AffineMap {
        let linear = a.linear.invert();
        let translation = -linear.apply(&a.translation);
        AffineMap { linear, translation }
    }
}

forward_ref_binop! { impl Mul, mul, AffineMap, compose for AffineMap, AffineMap, AffineMap }
