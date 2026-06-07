use crate::status::{OnboardingDisplay, Status, StatusSummary};
use embedded_graphics::{
    mono_font::{
        MonoTextStyle, ascii::{FONT_5X7}
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use ssd1306::mode::BufferedGraphicsMode;
use linux_embedded_hal::I2cdev;
use anyhow::*;


pub struct I2cStatus {
    display: Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>
}

impl I2cStatus {
    pub fn new(path: &str) -> Result<I2cStatus> {
        let dev = I2cdev::new(path)?;
        let mut display= Ssd1306::new(
            I2CDisplayInterface::new(dev),
            DisplaySize128x32,
            DisplayRotation::Rotate0
        ).into_buffered_graphics_mode();

        display.init().map_err(|e| anyhow::anyhow!("Unable to init: {:?}", e))?;

        Ok(Self { display })
    }
}

impl Status for I2cStatus {

    fn show(&mut self, status: &StatusSummary) -> Result<()> {
        let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

        // --- First row: time range header ---
        let from_str = status.from.format("%H:%M").to_string();
        let till_str = status.till.format("%H:%M").to_string();
        let header = format!("{} till {}", from_str, till_str);

        Text::with_alignment(&header, Point::new(64, 8), style, Alignment::Center)
            .draw(&mut self.display).map_err(|e| anyhow::anyhow!("Unable to draw: {:?}", e))?;

        // --- Sensor rows ---
        let lines = [
            format!("T {:.0} C  | S {:.0} pF", status.temperature, status.soil_moisture),
            format!("H {:.0} RH | L {:.0} lx", status.humidity, status.light)
        ];

        let mut y = 20;
        for line in lines {
            Text::new(&line, Point::new(0, y), style)
                .draw(&mut self.display)
                .map_err(|e| anyhow::anyhow!("Unable to draw: {:?}", e))?;

            y += 10; // Line spacing
        }

        self.display.flush().map_err(|e| anyhow::anyhow!("Unable to flush display: {:?}", e))?;

        Ok(())
    }

    fn show_onboarding(&mut self, display: &OnboardingDisplay) -> Result<()> {
        let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

        self.display
            .clear(BinaryColor::Off)
            .map_err(|e| anyhow::anyhow!("Unable to clear display: {:?}", e))?;

        Text::with_alignment(&display.line1, Point::new(64, 10), style, Alignment::Center)
            .draw(&mut self.display)
            .map_err(|e| anyhow::anyhow!("Unable to draw: {:?}", e))?;

        if let Some(line2) = &display.line2 {
            Text::with_alignment(line2, Point::new(64, 22), style, Alignment::Center)
                .draw(&mut self.display)
                .map_err(|e| anyhow::anyhow!("Unable to draw: {:?}", e))?;
        }

        self.display
            .flush()
            .map_err(|e| anyhow::anyhow!("Unable to flush display: {:?}", e))?;

        Ok(())
    }
}