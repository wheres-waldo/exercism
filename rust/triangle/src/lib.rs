use itertools::Itertools;
use num_traits::Num;

pub struct Triangle<T> {
    sides: Vec<T>,
}

impl<T> Triangle<T>
where
    T: Copy + Num + PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().all(|x| *x != T::zero())
            && !sides
                .iter()
                .permutations(3)
                .any(|sides| *sides[0] > *sides[1] + *sides[2])
        {
            Some(Triangle {
                sides: sides.to_vec(),
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.windows(2).all(|xs| xs[0] == xs[1])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.iter().combinations(2).all(|xs| xs[0] != xs[1])
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.iter().combinations(2).any(|xs| xs[0] == xs[1])
    }
}
