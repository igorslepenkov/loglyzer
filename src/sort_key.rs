use std::{cmp::Ordering, sync::Arc};

use jmespath::Variable;

use crate::AppError;

pub enum SortKey {
    Num(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Eq for SortKey {}

impl PartialEq for SortKey {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        if let Ordering::Equal = self.cmp(other) {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for SortKey {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortKey {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        match self {
            SortKey::Null => match other {
                SortKey::Null => Ordering::Equal,
                SortKey::Num(_)
                | SortKey::Bool(_)
                | SortKey::String(_) => Ordering::Less,
            },
            SortKey::Bool(val) => match other {
                SortKey::Null => Ordering::Greater,
                SortKey::Bool(val2) => val.cmp(val2),
                SortKey::Num(_) | SortKey::String(_) => {
                    Ordering::Less
                }
            },
            SortKey::Num(val) => match other {
                SortKey::Null => Ordering::Greater,
                SortKey::Bool(_) => Ordering::Greater,
                SortKey::Num(val2) => val.total_cmp(val2),
                SortKey::String(_) => Ordering::Less,
            },
            SortKey::String(val) => match other {
                SortKey::Null => Ordering::Greater,
                SortKey::Bool(_) => Ordering::Greater,
                SortKey::Num(_) => Ordering::Greater,
                SortKey::String(val2) => val.cmp(val2),
            },
        }
    }
}

impl TryFrom<&Variable> for SortKey {
    type Error = AppError;
    fn try_from(
        value: &Variable
    ) -> Result<Self, Self::Error> {
        match *value {
            Variable::Null => Ok(Self::Null),
            Variable::Bool(bool) => Ok(Self::Bool(bool)),
            Variable::Number(ref num) => {
                let as_f64 = num.as_f64();
                match as_f64 {
                    Some(num) => Ok(Self::Num(num)),
                    None => {
                        Err(AppError::InvalidSortingKey(
                            format!(
                                "received number {}, that cannot be converted to f64",
                                num
                            ),
                        ))
                    }
                }
            }
            Variable::Array(ref arr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "arrays are not valid for sorting. Received: {:?}",
                    arr
                )))
            }
            Variable::Object(ref obj) => {
                Err(AppError::InvalidSortingKey(format!(
                    "objects are not valid for sorting. Received: {:?}",
                    obj
                )))
            }
            Variable::String(ref str) => {
                Ok(Self::String(str.to_owned()))
            }
            Variable::Expref(ref expr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "expressions are not valid for sorting. Received: {:?}",
                    expr
                )))
            }
        }
    }
}

impl TryFrom<Arc<Variable>> for SortKey {
    type Error = AppError;
    fn try_from(
        value: Arc<Variable>
    ) -> Result<Self, Self::Error> {
        match *value {
            Variable::Null => Ok(Self::Null),
            Variable::Bool(bool) => Ok(Self::Bool(bool)),
            Variable::Number(ref num) => {
                let as_f64 = num.as_f64();
                match as_f64 {
                    Some(num) => Ok(Self::Num(num)),
                    None => {
                        Err(AppError::InvalidSortingKey(
                            format!(
                                "received number {}, that cannot be converted to f64",
                                num
                            ),
                        ))
                    }
                }
            }
            Variable::Array(ref arr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "arrays are not valid for sorting. Received: {:?}",
                    arr
                )))
            }
            Variable::Object(ref obj) => {
                Err(AppError::InvalidSortingKey(format!(
                    "objects are not valid for sorting. Received: {:?}",
                    obj
                )))
            }
            Variable::String(ref str) => {
                Ok(Self::String(str.to_owned()))
            }
            Variable::Expref(ref expr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "expressions are not valid for sorting. Received: {:?}",
                    expr
                )))
            }
        }
    }
}

impl TryFrom<Variable> for SortKey {
    type Error = AppError;
    fn try_from(
        value: Variable
    ) -> Result<Self, Self::Error> {
        match value {
            Variable::Null => Ok(Self::Null),
            Variable::Bool(bool) => Ok(Self::Bool(bool)),
            Variable::Number(ref num) => {
                let as_f64 = num.as_f64();
                match as_f64 {
                    Some(num) => Ok(Self::Num(num)),
                    None => {
                        Err(AppError::InvalidSortingKey(
                            format!(
                                "received number {}, that cannot be converted to f64",
                                num
                            ),
                        ))
                    }
                }
            }
            Variable::Array(ref arr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "arrays are not valid for sorting. Received: {:?}",
                    arr
                )))
            }
            Variable::Object(ref obj) => {
                Err(AppError::InvalidSortingKey(format!(
                    "objects are not valid for sorting. Received: {:?}",
                    obj
                )))
            }
            Variable::String(ref str) => {
                Ok(Self::String(str.to_owned()))
            }
            Variable::Expref(ref expr) => {
                Err(AppError::InvalidSortingKey(format!(
                    "expressions are not valid for sorting. Received: {:?}",
                    expr
                )))
            }
        }
    }
}
