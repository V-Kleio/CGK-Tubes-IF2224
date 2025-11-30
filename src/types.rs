use std::fmt;

/// Represents the data types in Pascal-S
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Integer,
    Real,
    Boolean,
    Char,
    String,
    Array(usize), // Index to atab
    UserDefined(String),
    Void,    // For procedures
    Unknown, // For error recovery
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => write!(f, "integer"),
            DataType::Real => write!(f, "real"),
            DataType::Boolean => write!(f, "boolean"),
            DataType::Char => write!(f, "char"),
            DataType::String => write!(f, "string"),
            DataType::Array(idx) => write!(f, "array[{}]", idx),
            DataType::UserDefined(name) => write!(f, "{}", name),
            DataType::Void => write!(f, "void"),
            DataType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Represents the kind of object an identifier refers to
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectKind {
    Constant,
    Variable,
    Type,
    Procedure,
    Function,
    Parameter,
    Program,
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectKind::Constant => write!(f, "constant"),
            ObjectKind::Variable => write!(f, "variable"),
            ObjectKind::Type => write!(f, "type"),
            ObjectKind::Procedure => write!(f, "procedure"),
            ObjectKind::Function => write!(f, "function"),
            ObjectKind::Parameter => write!(f, "parameter"),
            ObjectKind::Program => write!(f, "program"),
        }
    }
}

impl DataType {
    /// Check if two types are compatible for operations
    pub fn is_compatible(&self, other: &DataType) -> bool {
        match (self, other) {
            (DataType::Integer, DataType::Integer) => true,
            (DataType::Real, DataType::Real) => true,
            (DataType::Boolean, DataType::Boolean) => true,
            (DataType::Char, DataType::Char) => true,
            (DataType::String, DataType::String) => true,
            // Integer can be promoted to Real
            (DataType::Integer, DataType::Real) | (DataType::Real, DataType::Integer) => true,
            _ => false,
        }
    }

    /// Check if a value of type `from` can be assigned to type `to`
    pub fn can_assign(to: &DataType, from: &DataType) -> bool {
        match (to, from) {
            (DataType::Integer, DataType::Integer) => true,
            (DataType::Real, DataType::Real) => true,
            (DataType::Real, DataType::Integer) => true, // Integer can be assigned to Real
            (DataType::Boolean, DataType::Boolean) => true,
            (DataType::Char, DataType::Char) => true,
            (DataType::String, DataType::String) => true,
            (DataType::UserDefined(a), DataType::UserDefined(b)) => a == b,
            _ => false,
        }
    }

    /// Get the result type of a binary arithmetic operation
    pub fn get_arithmetic_result_type(left: &DataType, right: &DataType) -> Result<DataType, String> {
        match (left, right) {
            (DataType::Integer, DataType::Integer) => Ok(DataType::Integer),
            (DataType::Real, DataType::Real) => Ok(DataType::Real),
            (DataType::Integer, DataType::Real) | (DataType::Real, DataType::Integer) => {
                Ok(DataType::Real)
            }
            _ => Err(format!(
                "Arithmetic operation not supported between {} and {}",
                left, right
            )),
        }
    }

    /// Get the result type of a relational operation (always boolean)
    pub fn get_relational_result_type(left: &DataType, right: &DataType) -> Result<DataType, String> {
        if left.is_compatible(right) {
            Ok(DataType::Boolean)
        } else {
            Err(format!(
                "Cannot compare incompatible types {} and {}",
                left, right
            ))
        }
    }

    /// Get the result type of a logical operation (must be boolean)
    pub fn get_logical_result_type(left: &DataType, right: &DataType) -> Result<DataType, String> {
        match (left, right) {
            (DataType::Boolean, DataType::Boolean) => Ok(DataType::Boolean),
            _ => Err(format!(
                "Logical operation requires boolean operands, got {} and {}",
                left, right
            )),
        }
    }

    /// Check if this is a numeric type
    #[allow(dead_code)]
    pub fn is_numeric(&self) -> bool {
        matches!(self, DataType::Integer | DataType::Real)
    }

    /// Check if this is an ordinal type (can be used in for loops, array indices)
    #[allow(dead_code)]
    pub fn is_ordinal(&self) -> bool {
        matches!(self, DataType::Integer | DataType::Char | DataType::Boolean)
    }
    
    /// Convert DataType to numeric code (for Pascal-S compatibility)
    /// Following standard Pascal-S type codes:
    /// 0 = Void, 1 = Integer, 2 = Real, 3 = Boolean, 4 = String, 5 = Char
    /// 6+ = Array/Record (ref to atab/btab)
    pub fn to_numeric(&self) -> String {
        match self {
            DataType::Void => "0".to_string(),
            DataType::Integer => "1".to_string(),
            DataType::Real => "2".to_string(),
            DataType::Boolean => "3".to_string(),
            DataType::String => "4".to_string(),
            DataType::Char => "5".to_string(),
            DataType::Array(idx) => format!("{}", idx),
            DataType::UserDefined(_) => "6".to_string(),
            DataType::Unknown => "-".to_string(),
        }
    }
}
