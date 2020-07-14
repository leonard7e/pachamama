// NoiseFactory2D is a noise generator

mod cellnoise;
mod hash;

/*
A noise function needs to be differentiable.
*/
pub trait NoiseFactory <Input,Output>{
    fn gen_noise(self) -> Box<dyn Fn (Input) -> Output>;
}
