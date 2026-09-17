use embassy_stm32::adc::{
    Adc, AdcChannel, AdcConfig, AnyAdcChannel, Averaging, CkModePclk, Clock, Resolution, SampleTime,
};
use embassy_stm32::pac::adc::Adc as AdcRegs;
use embassy_stm32::{Peri, adc, dma, interrupt};

use qpoint_common::measurement::{MeasurementResult, U3V3_1LSB, U5V_1LSB};

/// Driver for measuring the base voltage and collector current.
pub struct AdcControl<'d, T, D>
where
    T: adc::Instance<Regs = AdcRegs>,
    D: adc::RxDma<T>,
{
    adc: Adc<'d, T>,
    dma: Peri<'d, D>,
    base: AnyAdcChannel<'d, T>,
    collector: AnyAdcChannel<'d, T>,
}

impl<'d, T, D> AdcControl<'d, T, D>
where
    T: adc::Instance<Regs = AdcRegs>,
    D: adc::RxDma<T>,
{
    /// Configure the ADC and construct the measurement driver.
    pub fn new<C1, C2>(adc: Peri<'d, T>, dma: Peri<'d, D>, base: C1, collector: C2) -> Self
    where
        C1: AdcChannel<T> + 'd,
        C2: AdcChannel<T> + 'd,
    {
        let config = AdcConfig {
            clock: Some(Clock::Sync {
                div: CkModePclk::DIV2,
            }),
            resolution: Some(Resolution::BITS12),
            averaging: Some(Averaging::Samples64), // no "extra" bits, just eliminate noise
            ..AdcConfig::default()
        };

        let adc = Adc::new_with_config(adc, config);

        Self {
            adc,
            dma,
            base: base.degrade_adc(),
            collector: collector.degrade_adc(),
        }
    }

    /// Perform the measurements.
    pub async fn measure<I>(&mut self, interrupts: I) -> MeasurementResult
    where
        I: interrupt::typelevel::Binding<D::Interrupt, dma::InterruptHandler<D>>,
    {
        let mut buf = [0; 2];

        self.adc
            .read(
                self.dma.reborrow(),
                interrupts,
                [
                    (&mut self.base, SampleTime::CYCLES160_5),
                    (&mut self.collector, SampleTime::CYCLES160_5),
                ]
                .into_iter(),
                &mut buf,
            )
            .await;

        // Ub [mV]
        let base_voltage = (buf[0] as f32) * U5V_1LSB;
        // Ic [mA] = -(Ui [mV] - 1650) / (100 * 0,3)
        // the `-` sign is due to the wiring of the in-amp
        let collector_current = -(((buf[1] as f32) * U3V3_1LSB) - 1650.0) / 30.0;

        MeasurementResult {
            base_voltage,
            collector_current,
        }
    }
}
