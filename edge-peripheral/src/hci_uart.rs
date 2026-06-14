use alloc::boxed::Box;

use bt_hci::transport::{Error as HciTransportError, Transport, WithIndicator};
use bt_hci::{ControllerToHostPacket, HostToControllerPacket, ReadHci, WriteHci};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use esp_hal::uart::{Config, IoError, Uart};
use esp_hal::Async;
use trouble_host::prelude::ExternalController;

use crate::utils::anyhow::ResultAny;

pub const HCI_BAUD: u32 = 115_200;

pub struct EspUartTransport<'d> {
    uart: &'d Mutex<NoopRawMutex, Uart<'d, Async>>,
}

impl embedded_io::ErrorType for EspUartTransport<'_> {
    type Error = HciTransportError<IoError>;
}

impl Transport for EspUartTransport<'_> {
    async fn read<'a>(
        &self,
        rx: &'a mut [u8],
    ) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        let mut uart = self.uart.lock().await;
        ControllerToHostPacket::read_hci_async(&mut *uart, rx)
            .await
            .map_err(HciTransportError::Read)
    }

    async fn write<T: HostToControllerPacket>(&self, tx: &T) -> Result<(), Self::Error> {
        let mut uart = self.uart.lock().await;
        WithIndicator::new(tx)
            .write_hci_async(&mut *uart)
            .await
            .map_err(HciTransportError::Write)
    }
}

pub fn init_hci_uart(
    uart: esp_hal::peripherals::UART1<'static>,
    tx: esp_hal::peripherals::GPIO17<'static>,
    rx: esp_hal::peripherals::GPIO16<'static>,
) -> anyhow::Result<ExternalController<EspUartTransport<'static>, 20>> {
    let config = Config::default().with_baudrate(HCI_BAUD);
    let uart = Uart::new(uart, config)
        .with_anyhow("Failed to init HCI UART")?
        .with_tx(tx)
        .with_rx(rx)
        .into_async();

    let mutex: &'static Mutex<NoopRawMutex, Uart<'static, Async>> =
        Box::leak(Box::new(Mutex::new(uart)));

    Ok(ExternalController::new(EspUartTransport { uart: mutex }))
}
