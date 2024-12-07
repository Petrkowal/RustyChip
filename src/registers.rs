use crate::datatypes::datatypes::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Register<T, V> {
    index: usize,
    value: V,
    _marker: std::marker::PhantomData<T>,
}

pub trait Loadable {
    type Value;

    fn load(&mut self, value: Self::Value) -> &mut Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VRegisterMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IRegisterMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PCRegisterMarker;

pub type VRegister = Register<VRegisterMarker, Byte>;
pub type IRegister = Register<IRegisterMarker, Address>;
pub type PCRegister = Register<PCRegisterMarker, Address>;
pub type VRegisterNumber = usize;
pub type IRegisterNumber = usize;
pub type PCRegisterNumber = usize;

impl<T, V: Default> Register<T, V> {
    pub fn new(index: usize) -> Self {
        Register {
            index,
            value: V::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl Register<VRegisterMarker, Byte> {
    pub fn increment(&mut self) {
        self.value.0 = self.value.0.wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.value.0 = self.value.0.wrapping_sub(1);
    }
}

impl Register<PCRegisterMarker, Address> {
    pub fn increment(&mut self) {
        self.value.0 = self.value.0.wrapping_add(2);
    }

    pub fn jump(&mut self, address: Address) {
        self.value = address;
    }

    pub fn as_usize(&self) -> usize {
        self.value.0 as usize
    }
}

impl Register<IRegisterMarker, Address> {
    pub fn increment(&mut self) {
        self.value.0 = self.value.0.wrapping_add(1);
    }
    
    pub fn load(&mut self, address: Address) {
        self.value = address;
    }
}

impl Loadable for VRegister {
    type Value = Byte;

    fn load(&mut self, value: Byte) -> &mut Self {
        self.value = value;
        self
    }
}
