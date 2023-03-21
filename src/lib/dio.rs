
//! # Digital IO Abstraction

#[path = "fe310/gpio.rs"] mod gpio;

pub type DioInstance =u8;
pub type DioPort =u8;
pub type DioPinNum = u8;

pub struct DioPin {
    pub instance: DioInstance,
    pub port: DioPort,
    pub pin_num: DioPinNum,
}

pub enum DioLogic{
    H,
    L,
}

enum DioDriveStrength{
    I10ma,
    I20ma,
}

pub enum DioPullUp{
    Enabled,
    Disabled,
}

pub enum DioFuncMode{
    Gpio,
    Mux,
}

pub enum DioMuxFunctions{
    Spi,
    Pwm,
    I2c,
    Uart,
}

pub struct DioConfig{
    pin: DioPin,
    mode: DioFuncMode,
    func: DioMuxFunctions,
    pup: DioPullUp,
    ds: DioDriveStrength,
}

impl DioPin {
    pub fn setup_pin(&self) {}
    pub fn configure_pin(&self, cfg: DioConfig){}

    pub fn enable_pin_inlet(&self) {
        gpio::enable_inlet((*self).pin_num);
    }

    pub fn enable_pin_inlet_pullup(&self) {}

    pub fn enable_pin_outlet(&self)  {
        gpio::enable_outlet((*self).pin_num);
    }
    pub fn read_pin_inlet_state(&self) -> DioLogic{
        let p = DioLogic::H;
        return p;
    }
    pub fn read_pin_outlet_state(&self) -> DioLogic {
        let p = DioLogic::H;
        return p;
    }
    pub fn write_pin_outlet_state(&self, v: DioLogic) {

    }
    pub fn toggle_pin_outlet_state(&self) {}

    pub fn set_pin_outlet_high(&self) {
        gpio::set_high((*self).pin_num);
    }

    pub fn set_pin_outlet_low(&self) {
        gpio::set_low((*self).pin_num);
    }

    pub fn set_pin_dir_as_in(&self) {}
    pub fn set_pin_dir_as_out(&self) {}
    pub fn get_pin_func_mode(&self) -> DioFuncMode {
        let p = DioFuncMode::Gpio;
        return p;
    }

    pub fn set_pin_func_mode(&self, mode: &DioFuncMode){
        match mode {
            DioFuncMode::Gpio => gpio::set_as_dio((*self).pin_num),
            DioFuncMode::Mux => gpio::set_as_iof((*self).pin_num),
        }
    }
    pub fn select_pin_iof_func(&self, s: bool){
        gpio::select_iof_func((*self).pin_num, s);
    }
}