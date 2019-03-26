#![no_std]

pub use generic_array::{typenum, GenericArray};
pub use rgb::*;

pub type RGBW<ComponentType> = RGBA<ComponentType>;

pub type RGBW8 = RGBW<u8>;
pub type RGBW16 = RGBW<u16>;

pub struct GenericPixel<ColorCount, ComponentType>
where
    ColorCount: generic_array::ArrayLength<ComponentType>,
{
    pub data: GenericArray<ComponentType, ColorCount>,
}

impl<ColorCount, ComponentType> GenericPixel<ColorCount, ComponentType>
where
    ColorCount: generic_array::ArrayLength<ComponentType>,
{
    pub fn iter(&self) -> core::slice::Iter<ComponentType> {
        self.data.iter()
    }
}

impl<ComponentType> From<RGB8> for GenericPixel<typenum::U3, ComponentType>
where
    ComponentType: From<u8>,
    GenericArray<ComponentType, typenum::U3>: core::iter::FromIterator<u8>,
{
    fn from(color: RGB8) -> GenericPixel<typenum::U3, ComponentType> {
        GenericPixel {
            data: color.iter().collect(),
        }
    }
}

impl<ComponentType> From<RGBW8> for GenericPixel<typenum::U4, ComponentType>
where
    ComponentType: From<u8>,
    GenericArray<ComponentType, typenum::U4>: core::iter::FromIterator<u8>,
{
    fn from(color: RGBW8) -> GenericPixel<typenum::U4, ComponentType> {
        GenericPixel {
            data: color.iter().collect(),
        }
    }
}

pub trait Pixel {
    type ColorCount;
    type ComponentType;
}

impl<ComponentType> Pixel for RGB<ComponentType> {
    type ColorCount = typenum::U3;
    type ComponentType = ComponentType;
}

impl<ComponentType> Pixel for RGBW<ComponentType> {
    type ColorCount = typenum::U4;
    type ComponentType = ComponentType;
}

impl<ColorCount, ComponentType> Pixel for GenericPixel<ColorCount, ComponentType>
where
    ColorCount: generic_array::ArrayLength<ComponentType>,
{
    type ColorCount = ColorCount;
    type ComponentType = ComponentType;
}

pub trait SmartLedsWrite {
    type Pixel;
    type Error;

    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        Self::Pixel: Pixel,
        T: Iterator<Item = I>,
        I: Into<Self::Pixel>;
}
