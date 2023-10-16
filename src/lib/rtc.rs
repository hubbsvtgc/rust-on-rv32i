
//! # real time clock abstraction

#[path = "fe310/rtc.rs"] mod fe310_rtc;

const TICKS_PER_SEC: u16 = 32768;

const TICKS_PER_MIN: u32 = TICKS_PER_SEC as u32 * 60;
const TICKS_PER_HR: u32 = TICKS_PER_MIN as u32 * 60;
const TICKS_PER_DAY: u32 = TICKS_PER_HR as u32 * 24;

#[derive(Clone, Copy)]
pub enum RtcMonths{
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Clone, Copy)]
pub struct RtcDate{
    pub year: u16,
    pub month: RtcMonths,
    pub week: u8,
    pub day: u8,
}

pub struct RtcTime{
    pub hours: u8,
    pub mins: u8,
    pub secs: u8,
}

pub struct Rtc{
    date: RtcDate,
    time: RtcTime,
    ticks: u64,
    ready: bool,
}

impl Rtc {

    pub fn init() -> Self{

        fe310_rtc::reset_rtc_counter();
        fe310_rtc::clksel_32768k();
        fe310_rtc::set_scale(15);

        return  Self{
            date: { RtcDate {year: 1970, month: RtcMonths::January, week: 1, day: 1}},
            time: { RtcTime {hours: 0, mins: 0, secs: 0}},
            ticks: 0,
            ready: false,
        };
    }

    pub fn enable(&self){
        fe310_rtc::enable();
    }

    pub fn secs_now(&self) -> u32{
        return fe310_rtc::get_rtcs();
    }

    pub fn wait_in_secs(& mut self, s: u32){
        self.ticks = fe310_rtc::get_rtc_counter();
        let secs_now = fe310_rtc::get_rtcs();
        fe310_rtc::set_rtc_cmp( secs_now  + s);

        while(!fe310_rtc::is_cmp_reached()){

        }
    }

    pub fn set_time(& mut self, time: RtcTime){

        self.ticks = fe310_rtc::get_rtc_counter();
        self.time = time;

    }

    pub fn set_date(& mut self, date: RtcDate){

        self.ticks = fe310_rtc::get_rtc_counter();
        self.date = date;

    }

    pub fn date(& mut self) -> RtcDate{

        self.date
    }

    pub fn time(){

    }

    pub fn ticks(){

    }

}