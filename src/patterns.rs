//! # Lesson 6: Design patterns
//!
//! A design pattern is a reusable shape for solving a common software-design
//! problem. Patterns are ideas, not strict rules. Rust's ownership, enums, traits,
//! and generics often make classic object-oriented patterns smaller and safer.
//!
//! This lesson demonstrates four patterns:
//!
//! - **Builder** constructs a value through clear, chainable steps.
//! - **Strategy** swaps one calculation for another behind a shared trait.
//! - **Newtype** wraps a basic value to give it a safer, more meaningful type.
//! - **State** uses an enum to restrict a value to known states and transitions.

/// Runs and prints examples of each design pattern.
pub fn run() {
    println!("6. DESIGN PATTERNS");

    // Builder methods consume and return the builder, so they can be chained from
    // left to right like a sentence. `expect` extracts the finished Report because
    // this hard-coded valid title should never fail.
    let report = Report::builder("Weekly Rust Notes")
        .author("Ferris")
        .section("Ownership makes resource lifetimes explicit.")
        .section("Traits share behavior without class inheritance.")
        .build()
        .expect("the title was supplied");
    println!(
        "   Builder: {} by {} ({} sections)",
        report.title,
        report.author,
        report.sections.len()
    );

    // Both Checkout values use the same generic type but select different concrete
    // strategies. The compiler checks that each strategy implements the trait.
    let standard = Checkout::new(StandardShipping);
    let express = Checkout::new(ExpressShipping);
    println!(
        "   Strategy: standard={}, express={}",
        standard.total(100),
        express.total(100)
    );

    // UserId holds a u64 but cannot be confused with another newtype such as BookId.
    let user_id = UserId::new(42);
    println!("   Newtype: user id is {}", user_id.get());

    // TrafficLight::next consumes one small Copy value and returns the next state.
    let light = TrafficLight::Red;
    println!("   Enum state: {:?} -> {:?}\n", light, light.next());
}

/// A finished report produced by `ReportBuilder`.
///
/// These fields are public to keep the example easy to inspect. In larger programs,
/// private fields plus accessor methods may protect important rules more carefully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub title: String,
    pub author: String,
    pub sections: Vec<String>,
}

impl Report {
    /// Starts a Builder with a required title and no optional information yet.
    pub fn builder(title: impl Into<String>) -> ReportBuilder {
        ReportBuilder {
            title: title.into(),
            author: None,
            sections: Vec::new(),
        }
    }
}

/// Holds a Report's unfinished construction state.
///
/// `Option<String>` naturally represents an author that may or may not have been
/// supplied. Keeping this type separate prevents half-built Reports from escaping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportBuilder {
    title: String,
    author: Option<String>,
    sections: Vec<String>,
}

impl ReportBuilder {
    /// Records an author and returns the updated Builder for method chaining.
    ///
    /// `mut self` means the method owns the Builder and may change it.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Appends one section and returns the updated Builder.
    pub fn section(mut self, text: impl Into<String>) -> Self {
        self.sections.push(text.into());
        self
    }

    /// Validates the unfinished data and creates a Report on success.
    pub fn build(self) -> Result<Report, &'static str> {
        // `trim` ignores surrounding whitespace, so a title containing only spaces
        // is rejected just like an empty title.
        if self.title.trim().is_empty() {
            return Err("report title cannot be empty");
        }

        // `unwrap_or_else` uses the real author when present and lazily creates the
        // default String only when author is None.
        Ok(Report {
            title: self.title,
            author: self.author.unwrap_or_else(|| "Anonymous".to_owned()),
            sections: self.sections,
        })
    }
}

/// A Strategy contract for calculating a shipping cost.
///
/// Checkout does not need to know the rules used by a strategy. It only knows that
/// every ShippingStrategy can answer the same question.
pub trait ShippingStrategy {
    fn shipping_cost(&self, subtotal: u32) -> u32;
}

/// Standard delivery, represented by a unit struct because it needs no stored data.
#[derive(Debug, Clone, Copy)]
pub struct StandardShipping;

impl ShippingStrategy for StandardShipping {
    /// Gives free standard delivery for orders of at least 50 units of currency.
    fn shipping_cost(&self, subtotal: u32) -> u32 {
        if subtotal >= 50 { 0 } else { 5 }
    }
}

/// Faster delivery with one fixed price.
#[derive(Debug, Clone, Copy)]
pub struct ExpressShipping;

impl ShippingStrategy for ExpressShipping {
    fn shipping_cost(&self, _subtotal: u32) -> u32 {
        15
    }
}

/// Combines an order calculation with a selectable shipping Strategy.
///
/// `S` may be any type, while the impl block below requires it to implement
/// ShippingStrategy before the Checkout methods become available.
pub struct Checkout<S> {
    shipping: S,
}

impl<S: ShippingStrategy> Checkout<S> {
    /// Stores a chosen Strategy inside a new Checkout.
    pub fn new(shipping: S) -> Self {
        Self { shipping }
    }

    /// Adds the Strategy's shipping cost to the original subtotal.
    pub fn total(&self, subtotal: u32) -> u32 {
        subtotal + self.shipping.shipping_cost(subtotal)
    }
}

/// A meaningful user identifier instead of an unlabeled integer.
///
/// This is the Newtype pattern: a one-field tuple struct creates a distinct type.
/// Its private inner field ensures callers use our constructor and accessor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

impl UserId {
    /// Wraps a raw integer as a UserId.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Copies the inner integer out of this small Copy type.
    pub fn get(self) -> u64 {
        self.0
    }
}

/// Every valid state of a traffic light.
///
/// An enum often replaces a class-based State pattern in Rust. It is impossible to
/// construct an unknown state, and `match` checks that every state has a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    /// Performs one valid transition and returns the new state.
    pub fn next(self) -> Self {
        match self {
            Self::Red => Self::Green,
            Self::Green => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_supplies_defaults_and_validates() {
        // Test both paths through build: the default author and invalid title.
        let report = Report::builder("Status").build().unwrap();
        assert_eq!(report.author, "Anonymous");
        assert!(Report::builder("  ").build().is_err());
    }

    #[test]
    fn strategy_changes_checkout_behavior() {
        // The same Checkout logic now produces totals based on its chosen Strategy.
        assert_eq!(Checkout::new(StandardShipping).total(20), 25);
        assert_eq!(Checkout::new(StandardShipping).total(50), 50);
        assert_eq!(Checkout::new(ExpressShipping).total(50), 65);
    }

    #[test]
    fn enum_models_a_closed_state_machine() {
        assert_eq!(TrafficLight::Red.next().next().next(), TrafficLight::Red);
    }
}
