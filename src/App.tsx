import { Button, Slider } from '@nextui-org/react';
import { Channel } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { SlPower } from 'react-icons/sl';
import { commands, Device, DiscoverEvent } from './bindings';
import './global.css';
import { Logo } from './Logo';

function App() {
  async function setBrightness(socketAddr: string, brightness: number) {
    const result = await commands.setBrightness(socketAddr, brightness);
    if (result.status !== 'ok') {
      setError(JSON.stringify(result.error, null, 2));
    } else {
      setDevices(devices =>
        devices.map(d => (d.addr === socketAddr ? { ...d, brightness } : d))
      );
    }
  }

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [devices, setDevices] = useState<Device[]>([]);

  const testDiscover = async () => {
    const channel = new Channel<DiscoverEvent>();
    channel.onmessage = event => {
      console.log('event', event);
    };
    commands.discover(channel).then(result => {
      console.log('result', result);
    });
  };

  const fetchDevices = async () => {
    try {
      setLoading(true);
      const result = await commands.getDevices();
      if (result.status === 'ok') {
        setDevices(
          result.data.sort((d1, d2) => d1.name.localeCompare(d2.name))
        );
      } else {
        setError(JSON.stringify(result.error, null, 2));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch devices');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDevices();
  }, []);

  return (
    <main className="flex flex-col gap-6 p-4 items-start ">
      <h1 className="mb-0 leading-none text-[3rem] flex flex-row gap-2 items-center ">
        <Logo className="h-16 text-primary-500" />
        overhead
      </h1>

      {error && (
        <pre className="text-red-500">{JSON.stringify(error, null, 2)}</pre>
      )}

      <div className="flex flex-row justify-between self-stretch items-center py-2">
        <button onClick={fetchDevices}>Refresh</button>
        <button onClick={testDiscover}>Discover</button>
      </div>

      {loading && <p>Loading devices...</p>}

      {devices.map(device => (
        <div
          key={device.addr}
          className="flex flex-col gap-4 w-full bg-white-alpha-50 p-4 rounded-xl"
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className="font-bold font-lg m-0">
              {device.name} {device.brightness}
            </h4>
            <Button
              isIconOnly
              aria-label="Toggle power"
              variant="faded"
              className=" px-2 py-1 text-sm  font-bold rounded-xl"
              color="primary"
              onPress={() => commands.toggle(device.addr)}
            >
              <SlPower size={48} />
            </Button>
          </div>
          {typeof device.brightness === 'number' && (
            <Slider
              classNames={{
                track: 'border-r-[transparent] !border-l-secondary-50',
                filler: 'bg-gradient-to-r from-secondary-50 to-primary-500',
              }}
              aria-label="Brightness"
              defaultValue={device.brightness ?? 0}
              maxValue={100}
              minValue={0}
              onChangeEnd={e => setBrightness(device.addr, +e)}
            />
          )}
        </div>
      ))}
    </main>
  );
}

export default App;
