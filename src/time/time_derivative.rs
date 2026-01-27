//! Traits for time-based calculus relationships.
//!
//! These traits define the relationships between quantities and their
//! time derivatives/integrals.
//!
//! For example:
//! - Length is the TimeIntegral of Velocity (Length / Time = Velocity)
//! - Velocity is the TimeDerivative of Length (Velocity * Time = Length)
//! - Velocity is also the TimeIntegral of Acceleration
//! - Acceleration is the TimeDerivative of Velocity

use super::Time;
use crate::core::Quantity;

/// A quantity that is the time derivative of another quantity.
///
/// The time derivative represents a rate of change over time.
/// For example, Velocity is the time derivative of Length.
///
/// # Type Parameters
///
/// - `Integral`: The quantity type that this is the derivative of.
///
/// # Example
///
/// ```ignore
/// // Velocity is TimeDerivative<Length>
/// // velocity * time = length
/// let velocity = Velocity::meters_per_second(10.0);
/// let time = Time::seconds(5.0);
/// let distance = velocity.integrate_over(time);  // 50 meters
/// ```
pub trait TimeDerivative<Integral: Quantity>: Quantity {
    /// Returns the integrated quantity over the given time.
    ///
    /// This computes `self * time`.
    fn integrate_over(&self, time: Time) -> Integral;

    /// Returns the time integral quantity for a unit of time.
    ///
    /// This is the "per unit" value used for integration calculations.
    fn time_integrated(&self) -> Integral;

    /// Returns the time unit used for this derivative (typically 1 second).
    fn derivative_time(&self) -> Time;
}

/// A quantity that is the time integral of another quantity.
///
/// The time integral represents an accumulated quantity over time.
/// For example, Length is the time integral of Velocity.
///
/// # Type Parameters
///
/// - `Derivative`: The quantity type that is the derivative of this.
///
/// # Example
///
/// ```ignore
/// // Length is TimeIntegral<Velocity>
/// // length / time = velocity
/// let distance = Length::meters(100.0);
/// let time = Time::seconds(10.0);
/// let velocity = distance.per(time);  // 10 m/s
/// ```
pub trait TimeIntegral<Derivative: Quantity>: Quantity {
    /// Returns the time derivative of this quantity over the given time.
    ///
    /// This computes `self / time`.
    fn per(&self, time: Time) -> Derivative;

    /// Alias for `per`.
    fn divide_by_time(&self, time: Time) -> Derivative {
        self.per(time)
    }

    /// Returns the derived quantity for a unit of time.
    fn time_derived(&self) -> Derivative;

    /// Returns the time unit used for this integral (typically 1 second).
    fn integral_time(&self) -> Time;
}

/// A quantity that is the second time derivative of another quantity.
///
/// For example, Acceleration is the second time derivative of Length
/// (and the first time derivative of Velocity).
///
/// # Type Parameters
///
/// - `SecondIntegral`: The quantity type that this is the second derivative of.
pub trait SecondTimeDerivative<SecondIntegral: Quantity>: Quantity {
    /// Returns the second integral quantity over the given time squared.
    fn integrate_over_squared(&self, time_squared: f64, time_unit: Time) -> SecondIntegral;
}

/// A quantity that is the second time integral of another quantity.
///
/// For example, Length is the second time integral of Acceleration
/// (and the first time integral of Velocity).
///
/// # Type Parameters
///
/// - `SecondDerivative`: The quantity type that is the second derivative of this.
pub trait SecondTimeIntegral<SecondDerivative: Quantity>: Quantity {
    /// Returns the second time derivative of this quantity.
    fn per_time_squared(&self, time_squared: f64, time_unit: Time) -> SecondDerivative;
}

#[cfg(test)]
mod tests {
    // Tests will be added when we implement concrete quantities
    // that use these traits (e.g., Length, Velocity, Acceleration)
}
