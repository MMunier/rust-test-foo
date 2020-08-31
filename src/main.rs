use std::ops::{Add, Mul, Sub, Neg};
use std::time::{Duration, Instant};
use std::thread::sleep;
//use std::
// struct ODESystem<S,F>
//     where 
//         S: Add<Output=S>,
//         F: Fn(S) -> S, {
//     state: S,
//     diff: F,
// }
//trait Numeric = Add + Mul;

trait Zero {
    type Output;
    fn zero() -> Self::Output {
        unimplemented!()
    }
}

macro_rules! zero_impl {
    ($($t:ty)*) => { $(
        impl Zero for $t {
            type Output = $t;
            fn zero() -> $t {
                0 as _
            }
        }
    )*};
}

zero_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[derive(Clone, Debug)]
struct Complex<T: Add + Mul + Sub + Clone + Zero> {
    real: T,
    imag: T,
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Clone + Zero<Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, other: Complex<T>) -> Complex<T>{
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Clone + Zero<Output=T>> Mul for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, other: Complex<T>) -> Complex<T>{
        let self_copy = self.clone();
        let other_copy = other.clone();
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self_copy.real * other_copy.imag + self_copy.imag * other_copy.real
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Clone + Zero<Output=T>> Mul<T> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, other: T) -> Complex<T>{
        Complex {
            real: self.real * other.clone(),
            imag: self.imag * other,
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Clone + Zero<Output=T>> From<T> for Complex<T> {
    fn from(orig: T) -> Complex<T>{
        return Complex {
            real: orig,
            imag: T::zero(),
        }
    }
}

impl<T: Mul+Add+Sub+Clone+Zero> Complex<T> {
    fn new(real: T, imag:T) -> Complex<T> {
        Complex {
            real,
            imag,
        }
    }
}

impl<T: Neg<Output=T> + Clone + Zero + Add + Mul + Sub> Complex<T> {
    fn conjugate(mut self) -> Self {
        self.imag = -self.imag.clone();
        self
    }
}

impl<T: Neg<Output=T> + Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Clone + Zero<Output=T>> Complex<T> {
    fn abs(self) -> T {
        (self.clone() * self.conjugate()).real
    }
}

trait ODE {
    type State: Add<Output=Self::State> + Mul<f64, Output=Self::State> + Clone;
    fn diff(&self, S: &Self::State) -> Self::State {
        unimplemented!()
    }
}

struct RK4<S: ODE> {
    t: f64,
    delta_t: f64,
    system: S,
    state: S::State,
}


impl<S: ODE> Iterator for RK4<S> {
    type Item = (f64, S::State);

    fn next(&mut self) -> Option<Self::Item> {
        let df1 = self.system.diff(&self.state);        
        let df2 = self.system.diff(&(self.state.clone() + df1.clone() * (0.5 * self.delta_t)));
        let df3 = self.system.diff(&(self.state.clone() + df2.clone() * (0.5 * self.delta_t)));
        let df4 = self.system.diff(&(self.state.clone() + df3.clone() * (      self.delta_t)));

        let df = df1 * (1.0/6.0) + df2 * (2.0/6.0) + df3 * (2.0/6.0) + df4 * (1.0/6.0); 
        self.state = self.state.clone() + df * self.delta_t;
        self.t += self.delta_t;
        
        return Some((self.t, self.state.clone()))
    }
}


struct SimpleDGL {}
impl ODE for SimpleDGL {
    type State = Complex<f64>;

    fn diff(&self, state: &Self::State) -> Self::State {
        return state.clone() * Complex::new(0 as _, 1 as _);
    }
}

fn main() {
    println!("Hello, world!");

    let iter = RK4 {
        t: 0.0,
        delta_t: 0.01,
        system: SimpleDGL{},
        state: Complex::from(1.0),
    };

    let start_time = Instant::now();
    for i in iter {
        println!("{:3.3?} {:10.5?}, | {:5.5?} |",i.0, i.1, i.1.clone().abs());
        sleep(Duration::from_secs_f64(i.0) - start_time.elapsed());
    }
}

#[cfg(tests)]
mod test {
    #[test]
    fn name() {
        Duration::from_millis(1) - Duration::from_millis(2); 
    }
}
