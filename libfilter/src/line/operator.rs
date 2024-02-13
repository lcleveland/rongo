//! Operator
use std::fmt;

#[derive(Debug)]
/// Operator
pub enum Operator {
    /// >
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::GreaterThan;
    /// # assert_eq!(operator.to_string(), ">");
    /// ```
    GreaterThan,

    /// >=
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::GreaterThanOrEqual;
    /// # assert_eq!(operator.to_string(), ">=");
    /// ```
    GreaterThanOrEqual,

    /// <
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::LessThan;
    /// # assert_eq!(operator.to_string(), "<");
    /// ```
    LessThan,

    /// <=
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::LessThanOrEqual;
    /// # assert_eq!(operator.to_string(), "<=");
    /// ```
    LessThanOrEqual,

    /// ==
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::Equal;
    /// # assert_eq!(operator.to_string(), "==");
    /// ```
    Equal,

    /// !=
    ///
    /// Example:
    /// ```
    /// # use libfilter::Operator;
    /// let operator = Operator::NotEqual;
    /// # assert_eq!(operator.to_string(), "!=");
    /// ```
    NotEqual,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
        }
    }
}
