use crate::app::gui::Dongle;
use anyhow::{Ok, Result};
use rtlsdr;

pub struct RtlSdr {}

impl Dongle for RtlSdr {
    fn open() -> Result<()> {
        let _sdr_index = rtlsdr::get_devices();
        println!("{_sdr_index:?}");
        // let _sdr = rtlsdr::open(_sdr_index);

        Ok(())
    }
    fn close() -> Result<()> {
        Ok(())
    }
}
