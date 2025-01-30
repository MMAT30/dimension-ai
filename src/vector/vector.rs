

// 
trait Dot {
    fn dot(&self, other: &Self) -> f64;
}


impl Dot for Vec<f64> {
    fn dot(&self, other: &Self) -> f64 {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
    }
}