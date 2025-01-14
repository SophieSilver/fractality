#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parameter {
    Value(f64),
    PixelX,
    PixelY,
}

impl Parameter {
    pub fn variant_str(self) -> &'static str {
        match self {
            Parameter::Value(_) => "Constant",
            Parameter::PixelX => "X coordinate",
            Parameter::PixelY => "Y coordinate",
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self::Value(0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ComplexParameter {
    pub real: Parameter,
    pub imaginary: Parameter,
}
