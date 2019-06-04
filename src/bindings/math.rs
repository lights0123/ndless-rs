//! Bindings to libm, as no_std Rust does not support many float operations

use core::intrinsics;

use ndless_sys as cmath;

/// Copy and pasted from Rust std
pub trait Float: Sized {
	/// Returns the largest integer less than or equal to a number.
	///
	/// # Examples
	///
	/// ```
	/// let f = 3.99_f64;
	/// let g = 3.0_f64;
	///
	/// assert_eq!(f.floor(), 3.0);
	/// assert_eq!(g.floor(), 3.0);
	/// ```
	fn floor(self) -> Self;

	/// Returns the smallest integer greater than or equal to a number.
	///
	/// # Examples
	///
	/// ```
	/// let f = 3.01_f64;
	/// let g = 4.0_f64;
	///
	/// assert_eq!(f.ceil(), 4.0);
	/// assert_eq!(g.ceil(), 4.0);
	/// ```
	fn ceil(self) -> Self;

	/// Returns the nearest integer to a number. Round half-way cases away from
	/// `0.0`.
	///
	/// # Examples
	///
	/// ```
	/// let f = 3.3_f64;
	/// let g = -3.3_f64;
	///
	/// assert_eq!(f.round(), 3.0);
	/// assert_eq!(g.round(), -3.0);
	/// ```
	fn round(self) -> Self;

	/// Returns the integer part of a number.
	///
	/// # Examples
	///
	/// ```
	/// let f = 3.3_f64;
	/// let g = -3.7_f64;
	///
	/// assert_eq!(f.trunc(), 3.0);
	/// assert_eq!(g.trunc(), -3.0);
	/// ```
	fn trunc(self) -> Self;

	/// Returns the fractional part of a number.
	///
	/// # Examples
	///
	/// ```
	/// let x = 3.5_f64;
	/// let y = -3.5_f64;
	/// let abs_difference_x = (x.fract() - 0.5).abs();
	/// let abs_difference_y = (y.fract() - (-0.5)).abs();
	///
	/// assert!(abs_difference_x < 1e-10);
	/// assert!(abs_difference_y < 1e-10);
	/// ```
	fn fract(self) -> Self;

	/// Computes the absolute value of `self`. Returns `NAN` if the
	/// number is `NAN`.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = 3.5_f64;
	/// let y = -3.5_f64;
	///
	/// let abs_difference_x = (x.abs() - x).abs();
	/// let abs_difference_y = (y.abs() - (-y)).abs();
	///
	/// assert!(abs_difference_x < 1e-10);
	/// assert!(abs_difference_y < 1e-10);
	///
	/// assert!(f64::NAN.abs().is_nan());
	/// ```
	fn abs(self) -> Self;

	/// Returns a number that represents the sign of `self`.
	///
	/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
	/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
	/// - `NAN` if the number is `NAN`
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let f = 3.5_f64;
	///
	/// assert_eq!(f.signum(), 1.0);
	/// assert_eq!(f64::NEG_INFINITY.signum(), -1.0);
	///
	/// assert!(f64::NAN.signum().is_nan());
	/// ```
	fn signum(self) -> Self;

	/// Returns a number composed of the magnitude of `self` and the sign of
	/// `y`.
	///
	/// Equal to `self` if the sign of `self` and `y` are the same, otherwise
	/// equal to `-self`. If `self` is a `NAN`, then a `NAN` with the sign of
	/// `y` is returned.
	///
	/// # Examples
	///
	/// ```
	/// #![feature(copysign)]
	/// use std::f64;
	///
	/// let f = 3.5_f64;
	///
	/// assert_eq!(f.copysign(0.42), 3.5_f64);
	/// assert_eq!(f.copysign(-0.42), -3.5_f64);
	/// assert_eq!((-f).copysign(0.42), 3.5_f64);
	/// assert_eq!((-f).copysign(-0.42), -3.5_f64);
	///
	/// assert!(f64::NAN.copysign(1.0).is_nan());
	/// ```
	#[must_use]
	fn copysign(self, y: Self) -> Self;

	/// Fused multiply-add. Computes `(self * a) + b` with only one rounding
	/// error, yielding a more accurate result than an unfused multiply-add.
	///
	/// Using `mul_add` can be more performant than an unfused multiply-add if
	/// the target architecture has a dedicated `fma` CPU instruction.
	///
	/// # Examples
	///
	/// ```
	/// let m = 10.0_f64;
	/// let x = 4.0_f64;
	/// let b = 60.0_f64;
	///
	/// // 100.0
	/// let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn mul_add(self, a: Self, b: Self) -> Self;

	/// Calculates Euclidean division, the matching method for `mod_euc`.
	///
	/// This computes the integer `n` such that
	/// `self = n * rhs + self.mod_euc(rhs)`.
	/// In other words, the result is `self / rhs` rounded to the integer `n`
	/// such that `self >= n * rhs`.
	///
	/// # Examples
	///
	/// ```
	/// #![feature(euclidean_division)]
	/// let a: f64 = 7.0;
	/// let b = 4.0;
	/// assert_eq!(a.div_euc(b), 1.0); // 7.0 > 4.0 * 1.0
	/// assert_eq!((-a).div_euc(b), -2.0); // -7.0 >= 4.0 * -2.0
	/// assert_eq!(a.div_euc(-b), -1.0); // 7.0 >= -4.0 * -1.0
	/// assert_eq!((-a).div_euc(-b), 2.0); // -7.0 >= -4.0 * 2.0
	/// ```
	fn div_euc(self, rhs: Self) -> Self;

	/// Calculates the Euclidean modulo (self mod rhs), which is never negative.
	///
	/// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
	/// most cases.  However, due to a floating point round-off error it can
	/// result in `r == rhs.abs()`, violating the mathematical definition, if
	/// `self` is much smaller than `rhs.abs()` in magnitude and `self < 0.0`.
	/// This result is not an element of the function's codomain, but it is the
	/// closest floating point number in the real numbers and thus fulfills the
	/// property `self == self.div_euc(rhs) * rhs + self.mod_euc(rhs)`
	/// approximatively.
	///
	/// # Examples
	///
	/// ```
	/// #![feature(euclidean_division)]
	/// let a: f64 = 7.0;
	/// let b = 4.0;
	/// assert_eq!(a.mod_euc(b), 3.0);
	/// assert_eq!((-a).mod_euc(b), 1.0);
	/// assert_eq!(a.mod_euc(-b), 3.0);
	/// assert_eq!((-a).mod_euc(-b), 1.0);
	/// // limitation due to round-off error
	/// assert_ne!((-std::f64::EPSILON).mod_euc(3.0), 0.0);
	/// ```
	fn mod_euc(self, rhs: Self) -> Self;

	/// Raises a number to an integer power.
	///
	/// Using this function is generally faster than using `powf`
	///
	/// # Examples
	///
	/// ```
	/// let x = 2.0_f64;
	/// let abs_difference = (x.powi(2) - x*x).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn powi(self, n: i32) -> Self;

	/// Raises a number to a floating point power.
	///
	/// # Examples
	///
	/// ```
	/// let x = 2.0_f64;
	/// let abs_difference = (x.powf(2.0) - x*x).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn powf(self, n: Self) -> Self;

	/// Takes the square root of a number.
	///
	/// Returns NaN if `self` is a negative number.
	///
	/// # Examples
	///
	/// ```
	/// let positive = 4.0_f64;
	/// let negative = -4.0_f64;
	///
	/// let abs_difference = (positive.sqrt() - 2.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// assert!(negative.sqrt().is_nan());
	/// ```
	fn sqrt(self) -> Self;

	/// Returns `e^(self)`, (the exponential function).
	///
	/// # Examples
	///
	/// ```
	/// let one = 1.0_f64;
	/// // e^1
	/// let e = one.exp();
	///
	/// // ln(e) - 1 == 0
	/// let abs_difference = (e.ln() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn exp(self) -> Self;

	/// Returns `2^(self)`.
	///
	/// # Examples
	///
	/// ```
	/// let f = 2.0_f64;
	///
	/// // 2^2 - 4 == 0
	/// let abs_difference = (f.exp2() - 4.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn exp2(self) -> Self;

	/// Returns the natural logarithm of the number.
	///
	/// # Examples
	///
	/// ```
	/// let one = 1.0_f64;
	/// // e^1
	/// let e = one.exp();
	///
	/// // ln(e) - 1 == 0
	/// let abs_difference = (e.ln() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn ln(self) -> Self;

	/// Returns the logarithm of the number with respect to an arbitrary base.
	///
	/// The result may not be correctly rounded owing to implementation details;
	/// `self.log2()` can produce more accurate results for base 2, and
	/// `self.log10()` can produce more accurate results for base 10.
	///
	/// # Examples
	///
	/// ```
	/// let five = 5.0_f64;
	///
	/// // log5(5) - 1 == 0
	/// let abs_difference = (five.log(5.0) - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn log(self, base: Self) -> Self;

	/// Returns the base 2 logarithm of the number.
	///
	/// # Examples
	///
	/// ```
	/// let two = 2.0_f64;
	///
	/// // log2(2) - 1 == 0
	/// let abs_difference = (two.log2() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn log2(self) -> Self;

	/// Returns the base 10 logarithm of the number.
	///
	/// # Examples
	///
	/// ```
	/// let ten = 10.0_f64;
	///
	/// // log10(10) - 1 == 0
	/// let abs_difference = (ten.log10() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn log10(self) -> Self;

	/// The positive difference of two numbers.
	///
	/// * If `self <= other`: `0:0`
	/// * Else: `self - other`
	///
	/// # Examples
	///
	/// ```
	/// let x = 3.0_f64;
	/// let y = -3.0_f64;
	///
	/// let abs_difference_x = (x.abs_sub(1.0) - 2.0).abs();
	/// let abs_difference_y = (y.abs_sub(1.0) - 0.0).abs();
	///
	/// assert!(abs_difference_x < 1e-10);
	/// assert!(abs_difference_y < 1e-10);
	/// ```
	#[deprecated(note = "you probably meant `(self - other).abs()`: \
	                     this operation is `(self - other).max(0.0)` (also \
	                     known as `fdim` in C). If you truly need the positive \
	                     difference, consider using that expression or the C function \
	                     `fdim`, depending on how you wish to handle NaN (please consider \
	                     filing an issue describing your use-case too).")]
	fn abs_sub(self, other: Self) -> Self;

	/// Takes the cubic root of a number.
	///
	/// # Examples
	///
	/// ```
	/// let x = 8.0_f64;
	///
	/// // x^(1/3) - 2 == 0
	/// let abs_difference = (x.cbrt() - 2.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn cbrt(self) -> Self;

	/// Calculates the length of the hypotenuse of a right-angle triangle given
	/// legs of length `x` and `y`.
	///
	/// # Examples
	///
	/// ```
	/// let x = 2.0_f64;
	/// let y = 3.0_f64;
	///
	/// // sqrt(x^2 + y^2)
	/// let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn hypot(self, other: Self) -> Self;

	/// Computes the sine of a number (in radians).
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = f64::consts::PI/2.0;
	///
	/// let abs_difference = (x.sin() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn sin(self) -> Self;

	/// Computes the cosine of a number (in radians).
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = 2.0*f64::consts::PI;
	///
	/// let abs_difference = (x.cos() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn cos(self) -> Self;

	/// Computes the tangent of a number (in radians).
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = f64::consts::PI/4.0;
	/// let abs_difference = (x.tan() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-14);
	/// ```
	fn tan(self) -> Self;

	/// Computes the arcsine of a number. Return value is in radians in
	/// the range [-pi/2, pi/2] or NaN if the number is outside the range
	/// [-1, 1].
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let f = f64::consts::PI / 2.0;
	///
	/// // asin(sin(pi/2))
	/// let abs_difference = (f.sin().asin() - f64::consts::PI / 2.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn asin(self) -> Self;

	/// Computes the arccosine of a number. Return value is in radians in
	/// the range [0, pi] or NaN if the number is outside the range
	/// [-1, 1].
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let f = f64::consts::PI / 4.0;
	///
	/// // acos(cos(pi/4))
	/// let abs_difference = (f.cos().acos() - f64::consts::PI / 4.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn acos(self) -> Self;

	/// Computes the arctangent of a number. Return value is in radians in the
	/// range [-pi/2, pi/2];
	///
	/// # Examples
	///
	/// ```
	/// let f = 1.0_f64;
	///
	/// // atan(tan(1))
	/// let abs_difference = (f.tan().atan() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn atan(self) -> Self;

	/// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
	///
	/// * `x = 0`, `y = 0`: `0`
	/// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
	/// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
	/// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let pi = f64::consts::PI;
	/// // Positive angles measured counter-clockwise
	/// // from positive x axis
	/// // -pi/4 radians (45 deg clockwise)
	/// let x1 = 3.0_f64;
	/// let y1 = -3.0_f64;
	///
	/// // 3pi/4 radians (135 deg counter-clockwise)
	/// let x2 = -3.0_f64;
	/// let y2 = 3.0_f64;
	///
	/// let abs_difference_1 = (y1.atan2(x1) - (-pi/4.0)).abs();
	/// let abs_difference_2 = (y2.atan2(x2) - 3.0*pi/4.0).abs();
	///
	/// assert!(abs_difference_1 < 1e-10);
	/// assert!(abs_difference_2 < 1e-10);
	/// ```
	fn atan2(self, other: Self) -> Self;

	/// Simultaneously computes the sine and cosine of the number, `x`. Returns
	/// `(sin(x), cos(x))`.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = f64::consts::PI/4.0;
	/// let f = x.sin_cos();
	///
	/// let abs_difference_0 = (f.0 - x.sin()).abs();
	/// let abs_difference_1 = (f.1 - x.cos()).abs();
	///
	/// assert!(abs_difference_0 < 1e-10);
	/// assert!(abs_difference_1 < 1e-10);
	/// ```
	fn sin_cos(self) -> (Self, Self)
	where
		Self: Sized;

	/// Returns `e^(self) - 1` in a way that is accurate even if the
	/// number is close to zero.
	///
	/// # Examples
	///
	/// ```
	/// let x = 7.0_f64;
	///
	/// // e^(ln(7)) - 1
	/// let abs_difference = (x.ln().exp_m1() - 6.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn exp_m1(self) -> Self;

	/// Returns `ln(1+n)` (natural logarithm) more accurately than if
	/// the operations were performed separately.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let x = f64::consts::E - 1.0;
	///
	/// // ln(1 + (e - 1)) == ln(e) == 1
	/// let abs_difference = (x.ln_1p() - 1.0).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn ln_1p(self) -> Self;

	/// Hyperbolic sine function.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let e = f64::consts::E;
	/// let x = 1.0_f64;
	///
	/// let f = x.sinh();
	/// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
	/// let g = (e*e - 1.0)/(2.0*e);
	/// let abs_difference = (f - g).abs();
	///
	/// assert!(abs_difference < 1e-10);
	/// ```
	fn sinh(self) -> Self;

	/// Hyperbolic cosine function.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let e = f64::consts::E;
	/// let x = 1.0_f64;
	/// let f = x.cosh();
	/// // Solving cosh() at 1 gives this result
	/// let g = (e*e + 1.0)/(2.0*e);
	/// let abs_difference = (f - g).abs();
	///
	/// // Same result
	/// assert!(abs_difference < 1.0e-10);
	/// ```
	fn cosh(self) -> Self;

	/// Hyperbolic tangent function.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let e = f64::consts::E;
	/// let x = 1.0_f64;
	///
	/// let f = x.tanh();
	/// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
	/// let g = (1.0 - e.powi(-2))/(1.0 + e.powi(-2));
	/// let abs_difference = (f - g).abs();
	///
	/// assert!(abs_difference < 1.0e-10);
	/// ```
	fn tanh(self) -> Self;

	/// Inverse hyperbolic sine function.
	///
	/// # Examples
	///
	/// ```
	/// let x = 1.0_f64;
	/// let f = x.sinh().asinh();
	///
	/// let abs_difference = (f - x).abs();
	///
	/// assert!(abs_difference < 1.0e-10);
	/// ```
	fn asinh(self) -> Self;

	/// Inverse hyperbolic cosine function.
	///
	/// # Examples
	///
	/// ```
	/// let x = 1.0_f64;
	/// let f = x.cosh().acosh();
	///
	/// let abs_difference = (f - x).abs();
	///
	/// assert!(abs_difference < 1.0e-10);
	/// ```
	fn acosh(self) -> Self;

	/// Inverse hyperbolic tangent function.
	///
	/// # Examples
	///
	/// ```
	/// use std::f64;
	///
	/// let e = f64::consts::E;
	/// let f = e.tanh().atanh();
	///
	/// let abs_difference = (f - e).abs();
	///
	/// assert!(abs_difference < 1.0e-10);
	/// ```
	fn atanh(self) -> Self;
}

impl Float for f64 {
	#[inline]
	fn floor(self) -> f64 {
		unsafe { intrinsics::floorf64(self) }
	}
	#[inline]
	fn ceil(self) -> f64 {
		unsafe { intrinsics::ceilf64(self) }
	}
	#[inline]
	fn round(self) -> f64 {
		unsafe { intrinsics::roundf64(self) }
	}
	#[inline]
	fn trunc(self) -> f64 {
		unsafe { intrinsics::truncf64(self) }
	}
	#[inline]
	fn fract(self) -> f64 {
		self - self.trunc()
	}
	#[inline]
	fn abs(self) -> f64 {
		unsafe { intrinsics::fabsf64(self) }
	}
	#[inline]
	fn signum(self) -> f64 {
		if self.is_nan() {
			core::f64::NAN
		} else {
			unsafe { intrinsics::copysignf64(1.0, self) }
		}
	}
	#[inline]
	#[must_use]
	fn copysign(self, y: f64) -> f64 {
		unsafe { intrinsics::copysignf64(self, y) }
	}
	#[inline]
	fn mul_add(self, a: f64, b: f64) -> f64 {
		unsafe { intrinsics::fmaf64(self, a, b) }
	}
	#[inline]
	fn div_euc(self, rhs: f64) -> f64 {
		let q = (self / rhs).trunc();
		if self % rhs < 0.0 {
			return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
		}
		q
	}
	#[inline]
	fn mod_euc(self, rhs: f64) -> f64 {
		let r = self % rhs;
		if r < 0.0 {
			r + rhs.abs()
		} else {
			r
		}
	}
	#[inline]
	fn powi(self, n: i32) -> f64 {
		unsafe { intrinsics::powif64(self, n) }
	}
	#[inline]
	fn powf(self, n: f64) -> f64 {
		unsafe { intrinsics::powf64(self, n) }
	}
	#[inline]
	fn sqrt(self) -> f64 {
		if self < 0.0 {
			core::f64::NAN
		} else {
			unsafe { intrinsics::sqrtf64(self) }
		}
	}
	#[inline]
	fn exp(self) -> f64 {
		unsafe { intrinsics::expf64(self) }
	}
	#[inline]
	fn exp2(self) -> f64 {
		unsafe { intrinsics::exp2f64(self) }
	}
	#[inline]
	fn ln(self) -> f64 {
		unsafe { intrinsics::logf64(self) }
	}
	#[inline]
	fn log(self, base: f64) -> f64 {
		self.ln() / base.ln()
	}
	#[inline]
	fn log2(self) -> f64 {
		unsafe { intrinsics::log2f64(self) }
	}
	#[inline]
	fn log10(self) -> f64 {
		unsafe { intrinsics::log10f64(self) }
	}
	#[inline]
	fn abs_sub(self, other: f64) -> f64 {
		unsafe { cmath::fdim(self, other) }
	}
	#[inline]
	fn cbrt(self) -> f64 {
		unsafe { cmath::cbrt(self) }
	}
	#[inline]
	fn hypot(self, other: f64) -> f64 {
		unsafe { cmath::hypot(self, other) }
	}
	#[inline]
	fn sin(self) -> f64 {
		unsafe { intrinsics::sinf64(self) }
	}
	#[inline]
	fn cos(self) -> f64 {
		unsafe { intrinsics::cosf64(self) }
	}
	#[inline]
	fn tan(self) -> f64 {
		unsafe { cmath::tan(self) }
	}
	#[inline]
	fn asin(self) -> f64 {
		unsafe { cmath::asin(self) }
	}
	#[inline]
	fn acos(self) -> f64 {
		unsafe { cmath::acos(self) }
	}
	#[inline]
	fn atan(self) -> f64 {
		unsafe { cmath::atan(self) }
	}
	#[inline]
	fn atan2(self, other: f64) -> f64 {
		unsafe { cmath::atan2(self, other) }
	}
	#[inline]
	fn sin_cos(self) -> (f64, f64) {
		(self.sin(), self.cos())
	}
	#[inline]
	fn exp_m1(self) -> f64 {
		unsafe { cmath::expm1(self) }
	}
	#[inline]
	fn ln_1p(self) -> f64 {
		unsafe { cmath::log1p(self) }
	}
	#[inline]
	fn sinh(self) -> f64 {
		unsafe { cmath::sinh(self) }
	}
	#[inline]
	fn cosh(self) -> f64 {
		unsafe { cmath::cosh(self) }
	}
	#[inline]
	fn tanh(self) -> f64 {
		unsafe { cmath::tanh(self) }
	}
	#[inline]
	fn asinh(self) -> f64 {
		if self == core::f64::NEG_INFINITY {
			core::f64::NEG_INFINITY
		} else {
			(self + ((self * self) + 1.0).sqrt()).ln()
		}
	}
	#[inline]
	fn acosh(self) -> f64 {
		match self {
			x if x < 1.0 => core::f64::NAN,
			x => (x + ((x * x) - 1.0).sqrt()).ln(),
		}
	}
	#[inline]
	fn atanh(self) -> f64 {
		0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
	}
}

impl Float for f32 {
	#[inline]
	fn floor(self) -> f32 {
		unsafe { intrinsics::floorf32(self) }
	}
	#[inline]
	fn ceil(self) -> f32 {
		unsafe { intrinsics::ceilf32(self) }
	}
	#[inline]
	fn round(self) -> f32 {
		unsafe { intrinsics::roundf32(self) }
	}
	#[inline]
	fn trunc(self) -> f32 {
		unsafe { intrinsics::truncf32(self) }
	}
	#[inline]
	fn fract(self) -> f32 {
		self - self.trunc()
	}
	#[inline]
	fn abs(self) -> f32 {
		unsafe { intrinsics::fabsf32(self) }
	}
	#[inline]
	fn signum(self) -> f32 {
		if self.is_nan() {
			core::f32::NAN
		} else {
			unsafe { intrinsics::copysignf32(1.0, self) }
		}
	}
	#[inline]
	#[must_use]
	fn copysign(self, y: f32) -> f32 {
		unsafe { intrinsics::copysignf32(self, y) }
	}
	#[inline]
	fn mul_add(self, a: f32, b: f32) -> f32 {
		unsafe { intrinsics::fmaf32(self, a, b) }
	}
	#[inline]
	fn div_euc(self, rhs: f32) -> f32 {
		let q = (self / rhs).trunc();
		if self % rhs < 0.0 {
			return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
		}
		q
	}
	#[inline]
	fn mod_euc(self, rhs: f32) -> f32 {
		let r = self % rhs;
		if r < 0.0 {
			r + rhs.abs()
		} else {
			r
		}
	}
	#[inline]
	fn powi(self, n: i32) -> f32 {
		unsafe { intrinsics::powif32(self, n) }
	}
	#[inline]
	fn powf(self, n: f32) -> f32 {
		unsafe { intrinsics::powf32(self, n) }
	}
	#[inline]
	fn sqrt(self) -> f32 {
		if self < 0.0 {
			core::f32::NAN
		} else {
			unsafe { intrinsics::sqrtf32(self) }
		}
	}
	#[inline]
	fn exp(self) -> f32 {
		unsafe { intrinsics::expf32(self) }
	}
	#[inline]
	fn exp2(self) -> f32 {
		unsafe { intrinsics::exp2f32(self) }
	}
	#[inline]
	fn ln(self) -> f32 {
		unsafe { intrinsics::logf32(self) }
	}
	#[inline]
	fn log(self, base: f32) -> f32 {
		self.ln() / base.ln()
	}
	#[inline]
	fn log2(self) -> f32 {
		unsafe { intrinsics::log2f32(self) }
	}
	#[inline]
	fn log10(self) -> f32 {
		unsafe { intrinsics::log10f32(self) }
	}
	#[inline]
	fn abs_sub(self, other: f32) -> f32 {
		unsafe { cmath::fdimf(self, other) }
	}
	#[inline]
	fn cbrt(self) -> f32 {
		unsafe { cmath::cbrtf(self) }
	}
	#[inline]
	fn hypot(self, other: f32) -> f32 {
		unsafe { cmath::hypotf(self, other) }
	}
	#[inline]
	fn sin(self) -> f32 {
		unsafe { intrinsics::sinf32(self) }
	}
	#[inline]
	fn cos(self) -> f32 {
		unsafe { intrinsics::cosf32(self) }
	}
	#[inline]
	fn tan(self) -> f32 {
		unsafe { cmath::tanf(self) }
	}
	#[inline]
	fn asin(self) -> f32 {
		unsafe { cmath::asinf(self) }
	}
	#[inline]
	fn acos(self) -> f32 {
		unsafe { cmath::acosf(self) }
	}
	#[inline]
	fn atan(self) -> f32 {
		unsafe { cmath::atanf(self) }
	}
	#[inline]
	fn atan2(self, other: f32) -> f32 {
		unsafe { cmath::atan2f(self, other) }
	}
	#[inline]
	fn sin_cos(self) -> (f32, f32) {
		(self.sin(), self.cos())
	}
	#[inline]
	fn exp_m1(self) -> f32 {
		unsafe { cmath::expm1f(self) }
	}
	#[inline]
	fn ln_1p(self) -> f32 {
		unsafe { cmath::log1pf(self) }
	}
	#[inline]
	fn sinh(self) -> f32 {
		unsafe { cmath::sinhf(self) }
	}
	#[inline]
	fn cosh(self) -> f32 {
		unsafe { cmath::coshf(self) }
	}
	#[inline]
	fn tanh(self) -> f32 {
		unsafe { cmath::tanhf(self) }
	}
	#[inline]
	fn asinh(self) -> f32 {
		if self == core::f32::NEG_INFINITY {
			core::f32::NEG_INFINITY
		} else {
			(self + ((self * self) + 1.0).sqrt()).ln()
		}
	}
	#[inline]
	fn acosh(self) -> f32 {
		match self {
			x if x < 1.0 => core::f32::NAN,
			x => (x + ((x * x) - 1.0).sqrt()).ln(),
		}
	}
	#[inline]
	fn atanh(self) -> f32 {
		0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
	}
}
