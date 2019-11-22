use num_complex::Complex;

mod avx;

pub trait ComplexVector {
    type Float;
    const WIDTH: usize;

    unsafe fn zero() -> Self;
    unsafe fn broadcast(value: &Complex<Self::Float>) -> Self;

    unsafe fn add(&self, rhs: &Self) -> Self;
    unsafe fn sub(&self, rhs: &Self) -> Self;
    unsafe fn mul(&self, rhs: &Self) -> Self;
    unsafe fn rotate(&self, positive: bool) -> Self;

    unsafe fn load(from: *const Complex<Self::Float>) -> Self;
    unsafe fn store(&self, to: *mut Complex<Self::Float>);

    unsafe fn partial_load(from: *const Complex<Self::Float>, count: usize) -> Self;
    unsafe fn partial_store(&self, to: *mut Complex<Self::Float>, count: usize);
}