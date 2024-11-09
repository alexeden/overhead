use log::*;
use std::{
    collections::HashSet,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread::JoinHandle,
    time::Duration,
};

use crate::tplink::{
    devices::Device,
    discover::{discover_devices, DiscoverConfig},
};

#[derive(Clone, Debug)]
pub enum DiscoveryEvent {
    DevicesFound(Vec<Device>),
}

pub struct Discovery {
    event_handler: Receiver<DiscoveryEvent>,
    handle: JoinHandle<()>,
    stop_sig_emitter: Sender<()>,
}

impl Discovery {
    pub fn start(config: DiscoverConfig) -> Self {
        Self::start_with_interval(config, Duration::from_secs(5))
    }

    pub fn start_with_interval(
        config: DiscoverConfig,
        poll_interval: Duration,
    ) -> Self {
        // Use a sync_channel instead of channel so that `event_emitter.send`
        // will block until the main threead is actually listening
        // This will allow dialumi to start discovery early on in startup
        // and continue on initializing other things without missing device
        // discovery events
        // This pattern is called a "rendezvous chanenl"
        let (event_emitter, event_handler) = std::sync::mpsc::sync_channel(0);
        let (stop_sig_emitter, stop_sig_receiver) =
            std::sync::mpsc::channel::<()>();

        let handle = std::thread::spawn(move || {
            let mut addrs_found = HashSet::new();

            loop {
                match stop_sig_receiver.try_recv() {
                    Ok(_) => break,
                    _ => (),
                }

                debug!("Running discovery cycle");
                let devices_data = match discover_devices(config) {
                    Ok(ds) => ds,
                    Err(err) => {
                        error!("discover_devices error {:?}", err);
                        vec![]
                    }
                };

                debug!("Discovery found {} devices", devices_data.len());

                let devices = devices_data
                    .into_iter()
                    .filter(|(addr, _)| addrs_found.insert(addr.clone()))
                    .filter_map(|(addr, data)| {
                        debug!(
                            "Creating device from data {:?}: {}",
                            addr, data.system.sysinfo.alias
                        );
                        Device::from_data(addr, &data)
                    })
                    .collect::<Vec<Device>>();

                if let Err(err) =
                    event_emitter.send(DiscoveryEvent::DevicesFound(devices))
                {
                    error!("DeviceFound event send error {:?}", err);
                }

                std::thread::sleep(poll_interval);
            }

            debug!("Discovery thread is returning");
        });

        Self {
            event_handler,
            handle,
            stop_sig_emitter,
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.handle.is_finished()
    }

    pub fn stop(&self) {
        if let Err(err) = self.stop_sig_emitter.send(()) {
            error!("Failed to stop discovery thread {:?}", err);
        }
    }

    pub fn try_recv(&self) -> Result<DiscoveryEvent, TryRecvError> {
        self.event_handler.try_recv()
    }
}
