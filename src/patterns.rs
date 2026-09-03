//! Common Rust-friendly design patterns.

pub fn run() {
    println!("6. DESIGN PATTERNS");

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

    let standard = Checkout::new(StandardShipping);
    let express = Checkout::new(ExpressShipping);
    println!(
        "   Strategy: standard={}, express={}",
        standard.total(100),
        express.total(100)
    );

    let user_id = UserId::new(42);
    println!("   Newtype: user id is {}", user_id.get());

    let light = TrafficLight::Red;
    println!("   Enum state: {:?} -> {:?}\n", light, light.next());
}

/// Builder pattern: readable construction when a value has optional parts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub title: String,
    pub author: String,
    pub sections: Vec<String>,
}

impl Report {
    pub fn builder(title: impl Into<String>) -> ReportBuilder {
        ReportBuilder {
            title: title.into(),
            author: None,
            sections: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportBuilder {
    title: String,
    author: Option<String>,
    sections: Vec<String>,
}

impl ReportBuilder {
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn section(mut self, text: impl Into<String>) -> Self {
        self.sections.push(text.into());
        self
    }

    pub fn build(self) -> Result<Report, &'static str> {
        if self.title.trim().is_empty() {
            return Err("report title cannot be empty");
        }

        Ok(Report {
            title: self.title,
            author: self.author.unwrap_or_else(|| "Anonymous".to_owned()),
            sections: self.sections,
        })
    }
}

/// Strategy pattern: swap an algorithm behind a trait.
pub trait ShippingStrategy {
    fn shipping_cost(&self, subtotal: u32) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct StandardShipping;

impl ShippingStrategy for StandardShipping {
    fn shipping_cost(&self, subtotal: u32) -> u32 {
        if subtotal >= 50 { 0 } else { 5 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExpressShipping;

impl ShippingStrategy for ExpressShipping {
    fn shipping_cost(&self, _subtotal: u32) -> u32 {
        15
    }
}

pub struct Checkout<S> {
    shipping: S,
}

impl<S: ShippingStrategy> Checkout<S> {
    pub fn new(shipping: S) -> Self {
        Self { shipping }
    }

    pub fn total(&self, subtotal: u32) -> u32 {
        subtotal + self.shipping.shipping_cost(subtotal)
    }
}

/// Newtype pattern: a distinct type prevents mixing unrelated integer IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

impl UserId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Enums often replace a class-based State pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
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
        let report = Report::builder("Status").build().unwrap();
        assert_eq!(report.author, "Anonymous");
        assert!(Report::builder("  ").build().is_err());
    }

    #[test]
    fn strategy_changes_checkout_behavior() {
        assert_eq!(Checkout::new(StandardShipping).total(20), 25);
        assert_eq!(Checkout::new(StandardShipping).total(50), 50);
        assert_eq!(Checkout::new(ExpressShipping).total(50), 65);
    }

    #[test]
    fn enum_models_a_closed_state_machine() {
        assert_eq!(TrafficLight::Red.next().next().next(), TrafficLight::Red);
    }
}
