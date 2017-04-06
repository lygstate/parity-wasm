use super::invoke::{Invoke, Identity};
use elements;

pub enum Signature {
    TypeReference(u32),
    Inline(elements::FunctionType),
}

pub struct SignatureBuilder<F=Identity> {
    callback: F,
    signature: elements::FunctionType,
}

impl<F> SignatureBuilder<F> where F: Invoke<elements::FunctionType> {
    pub fn with_callback(callback: F) -> Self {
        SignatureBuilder { 
            callback: callback, 
            signature: elements::FunctionType::default(),
        }
    }

    pub fn param(mut self, value_type: elements::ValueType) -> Self {
        self.signature.params_mut().push(value_type);

        self
    }

    pub fn return_type(mut self, value_type: elements::ValueType) -> Self {
        *self.signature.return_type_mut() = Some(value_type);                
        self
    }

    pub fn build(self) -> F::Result {
        self.callback.invoke(self.signature)
    }
}

pub struct TypeRefBuilder<F=Identity> {
    callback: F,
    type_ref: u32,
}

impl<F> TypeRefBuilder<F> where F: Invoke<u32> {
    pub fn with_callback(callback: F) -> Self {
        TypeRefBuilder { 
            callback: callback, 
            type_ref: 0
        }
    }

    pub fn val(mut self, val: u32) -> Self {
        self.type_ref = val;
        self
    }

    pub fn build(self) -> F::Result { self.callback.invoke(self.type_ref) }
}

pub struct FunctionsBuilder<F=Identity> {
    callback: F,
    section: Vec<Signature>,
}

impl FunctionsBuilder {
    /// New empty functions section builder
    pub fn new() -> Self {
        FunctionsBuilder::with_callback(Identity)
    }
}

impl<F> FunctionsBuilder<F> {
    pub fn with_callback(callback: F) -> Self {
        FunctionsBuilder {
            callback: callback,
            section: Vec::new(),
        }
    }

    pub fn with_signature(mut self, signature: Signature) -> Self {
        self.section.push(signature);
        self
    }

    pub fn type_ref(self) -> TypeRefBuilder<Self> {
        TypeRefBuilder::with_callback(self)
    }    
}

impl<F> FunctionsBuilder<F> where F: Invoke<SignatureBindings> {
    pub fn signature(self) -> SignatureBuilder<Self> {
        SignatureBuilder::with_callback(self)
    }
}

impl<F> Invoke<elements::FunctionType> for FunctionsBuilder<F> {
	type Result = Self;

	fn invoke(self, signature: elements::FunctionType) -> Self {
		self.with_signature(Signature::Inline(signature))
    }    
}

impl<F> Invoke<u32> for FunctionsBuilder<F> {
	type Result = Self;

	fn invoke(self, type_ref: u32) -> Self {
		self.with_signature(Signature::TypeReference(type_ref))
    }    
}

impl<F> FunctionsBuilder<F> where F: Invoke<elements::FunctionsSection> {
    pub fn build(self) -> F::Result {
        let mut result = elements::FunctionsSection::new();
        for f in self.section.into_iter() {
            if let Signature::TypeReference(type_ref) = f {
                result.entries_mut().push(elements::Func::new(type_ref));
            } else {
                unreachable!(); // never possible with current generics impl-s
            }
        }
        self.callback.invoke(result)
    }
}

pub type SignatureBindings = Vec<Signature>;

impl<F> FunctionsBuilder<F> where F: Invoke<SignatureBindings> {
    pub fn bind(self) -> F::Result {
        self.callback.invoke(self.section)
    }
}

/// New function builder.
pub fn function() -> FunctionsBuilder {
    FunctionsBuilder::new()
}

#[cfg(test)]
mod tests {

    use super::function;

    #[test]
    fn example() {
        let result = function()
            .type_ref().val(1).build()
            .build();

        assert_eq!(result.entries().len(), 1);

        let result = function()
            .signature()
                .param(::elements::ValueType::I32)
                .param(::elements::ValueType::I32)
                .return_type(::elements::ValueType::I64)
                .build()
            .bind();      

        assert_eq!(result.len(), 1);              
    }
}