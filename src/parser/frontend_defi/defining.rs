pub trait FrontendT {
    fn dtype(&self) -> &Type;
    fn name(&self) -> &str;
    fn value(&self) -> &Value;
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionCall(FunctionCall),
    ArthematicOperator(ArthematicOperator),
}
#[derive(Debug)]
pub enum Type {
    String,
    Int,
    Float,
}
#[derive(Debug)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub dtype: Type,
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Value>,
}

#[derive(Debug)]
pub struct ArthematicOperator {
    pub operator: String,
    pub lhs: Type,
    pub rhs: Type,
}

impl FrontendT for VariableDeclaration {
    fn dtype(&self) -> &Type {
        &self.dtype
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> &Value {
        &self.value
    }
}
