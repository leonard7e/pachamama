// Non-differentiable random generators.
mod cellnoise;
mod hash;

/*
A NoiseFactory generates a noise which is differentiable.
*/
pub trait NoiseFactory <Input,Output>{
    fn gen_noise(self) -> Box<dyn Fn (Input) -> Output>;
}
