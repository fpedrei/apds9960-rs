use embedded_hal::i2c::I2c;
use {
    register::{Enable, GConfig1, GConfig2, GConfig4, GPulse},
    Apds9960, BitFlags, GestureDataThreshold, GestureGain, GestureLDrive, GestureWait, GesturePulseLength, Register, DEV_ADDR
};

/// Gesture engine configuration.
impl<I2C> Apds9960<I2C>
where
    I2C: I2c,
{
    /// Enable gesture detection
    pub fn enable_gesture(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_enable(Enable::GEN, true)
    }

    /// Disable gesture detection
    pub fn disable_gesture(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_enable(Enable::GEN, false)
    }

    /// Enable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn enable_gesture_mode(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_gconfig4(GConfig4::GMODE, true)
    }

    /// Disable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn disable_gesture_mode(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_gconfig4(GConfig4::GMODE, false)
    }

    /// Enable gesture interrupt generation
    pub fn enable_gesture_interrupts(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_gconfig4(GConfig4::GIEN, true)
    }

    /// Disable gesture interrupt generation
    pub fn disable_gesture_interrupts(&mut self) -> Result<(), I2C::Error> {
        self.set_flag_gconfig4(GConfig4::GIEN, false)
    }

    /// Set the gain of gesture readings
    pub fn set_gesture_gain_level(
        &mut self,
        gain: GestureGain,
    ) -> Result<(), I2C::Error> {
        use GestureGain as GAIN;
        let flags = match gain {
            GAIN::Level0 => (false, false),
            GAIN::Level1 => (false, true),
            GAIN::Level2 => (true, false),
            GAIN::Level3 => (true, true),
        };
        let new = self
            .gconfig2
            .with(GConfig2::GGAIN1, flags.0)
            .with(GConfig2::GGAIN2, flags.1);
        self.config_register(&new)?;
        self.gconfig2 = new;
        Ok(())
    }

    /// Set the gesture LED drive strength
    pub fn set_gesture_led_drive_strength(
        &mut self,
        strength: GestureLDrive
    ) -> Result<(), I2C::Error> {
        use GestureLDrive as DRIVE;
        let flags = match strength {
            DRIVE::Strength0 => (false, false),
            DRIVE::Strength1 => (false, true),
            DRIVE::Strength2 => (true, false),
            DRIVE::Strength3 => (true, true),
        };
        let new = self
            .gconfig2
            .with(GConfig2::GDRIVE1, flags.0)
            .with(GConfig2::GDRIVE2, flags.1);
        self.config_register(&new)?;
        self.gconfig2 = new;
        Ok(())
    }

    /// Set the gesture wait time
    pub fn set_gesture_wait_time(
        &mut self,
        time: GestureWait
    ) -> Result<(), I2C::Error> {
        use GestureWait as WAIT;
        let flags = match time {
            WAIT::Time0 => (false, false, false),
            WAIT::Time1 => (false, false, true),
            WAIT::Time2 => (false, true, false),
            WAIT::Time3 => (false, true, true),
            WAIT::Time4 => (true, false, false),
            WAIT::Time5 => (true, false, true),
            WAIT::Time6 => (true, true, false),
            WAIT::Time7 => (true, true, true),
        };
        let new = self
            .gconfig2
            .with(GConfig2::GWAIT1, flags.0)
            .with(GConfig2::GWAIT2, flags.1)
            .with(GConfig2::GWAIT3, flags.2);
        self.config_register(&new)?;
        self.gconfig2 = new;
        Ok(())
    }

    /// Set the gesture pulse length
    pub fn set_gesture_pulse_length(
        &mut self,
        length: GesturePulseLength
    ) -> Result<(), I2C::Error> {
        use GesturePulseLength as PULSE;
        let flags = match length {
            PULSE::Length0 => (false, false),
            PULSE::Length1 => (false, true),
            PULSE::Length2 => (true, false),
            PULSE::Length3 => (true, true),
        };
        let new = self
            .gpulse
            .with(GPulse::GPLEN1, flags.0)
            .with(GPulse::GPLEN2, flags.1);
        self.config_register(&new)?;
        self.gpulse = new;
        Ok(())
    }

    /// Set the number of gesture pulses
    pub fn set_gesture_pulse_number(
        &mut self,
        number: u8,
    ) -> Result<(), I2C::Error> {
        let new = self
            .gpulse
            .with(GPulse::GPULSE1, GPulse::GPULSE1 & number == GPulse::GPULSE1)
            .with(GPulse::GPULSE2, GPulse::GPULSE2 & number == GPulse::GPULSE2)
            .with(GPulse::GPULSE3, GPulse::GPULSE3 & number == GPulse::GPULSE3)
            .with(GPulse::GPULSE4, GPulse::GPULSE4 & number == GPulse::GPULSE4)
            .with(GPulse::GPULSE5, GPulse::GPULSE5 & number == GPulse::GPULSE5)
            .with(GPulse::GPULSE6, GPulse::GPULSE6 & number == GPulse::GPULSE6);
        self.config_register(&new)?;
        self.gpulse = new;
        Ok(())
    }

    /// Set the threshold of amount of available data in the gesture FIFO registers.
    pub fn set_gesture_data_level_threshold(
        &mut self,
        threshold: GestureDataThreshold,
    ) -> Result<(), I2C::Error> {
        use GestureDataThreshold as GDTH;
        let flags = match threshold {
            GDTH::Th1 => (false, false),
            GDTH::Th4 => (false, true),
            GDTH::Th8 => (true, false),
            GDTH::Th16 => (true, true),
        };
        let new = self
            .gconfig1
            .with(GConfig1::GFIFOTH1, flags.0)
            .with(GConfig1::GFIFOTH0, flags.1);
        self.config_register(&new)?;
        self.gconfig1 = new;
        Ok(())
    }

    /// Set the gesture proximity entry threshold.
    pub fn set_gesture_proximity_entry_threshold(&mut self, threshold: u8) -> Result<(), I2C::Error> {
        self.write_register(Register::GPENTH, threshold)
    }

    /// Set the gesture proximity exit threshold.
    pub fn set_gesture_proximity_exit_threshold(&mut self, threshold: u8) -> Result<(), I2C::Error> {
        self.write_register(Register::GPEXTH, threshold)
    }

    /// Set the gesture up offset.
    pub fn set_gesture_up_offset(&mut self, offset: i8) -> Result<(), I2C::Error> {
        self.write_register(Register::GOFFSET_U, offset as u8)
    }

    /// Set the gesture down offset.
    pub fn set_gesture_down_offset(&mut self, offset: i8) -> Result<(), I2C::Error> {
        self.write_register(Register::GOFFSET_D, offset as u8)
    }

    /// Set the gesture left offset.
    pub fn set_gesture_left_offset(&mut self, offset: i8) -> Result<(), I2C::Error> {
        self.write_register(Register::GOFFSET_L, offset as u8)
    }

    /// Set the gesture right offset.
    pub fn set_gesture_right_offset(&mut self, offset: i8) -> Result<(), I2C::Error> {
        self.write_register(Register::GOFFSET_R, offset as u8)
    }

    /// Set the gesture up, down, left and right offsets.
    pub fn set_gesture_offsets(
        &mut self,
        offset_up: i8,
        offset_down: i8,
        offset_left: i8,
        offset_right: i8,
    ) -> Result<(), I2C::Error> {
        self.i2c
            .write(
                DEV_ADDR,
                &[
                    Register::GOFFSET_U,
                    offset_up as u8,
                    offset_down as u8,
                    offset_left as u8,
                    offset_right as u8,
                ],
            )
    }
}
