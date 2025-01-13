import { Button, Progress, Slider } from '@nextui-org/react';
import { PiEmpty } from 'react-icons/pi';
import { cx } from 'class-variance-authority';
import { useEffect, useState } from 'react';
import { AiOutlinePoweroff } from 'react-icons/ai';
import { commands, Device } from './bindings';
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

  async function toggle(socketAddr: string) {
    const result = await commands.toggle(socketAddr);
    if (result.status !== 'ok') {
      setError(JSON.stringify(result.error, null, 2));
    } else {
      setDevices(devices =>
        devices.map(d => (d.addr === socketAddr ? { ...d, isOn: !d.isOn } : d))
      );
    }
  }

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [devices, setDevices] = useState<Device[]>([]);

  const discoverDevices = async () => {
    try {
      setLoading(true);
      const result = await commands.discover();
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
    setInterval(discoverDevices, 10000);
  }, []);

  return (
    <main className="flex flex-col gap-6 px-4 py-8 items-start">
      {loading && (
        <Progress
          aria-label="refreshing devices"
          classNames={{
            base: 'fixed top-[-1px] left-0 right-0',
            indicator: 'bg-gradient-to-r from-primary-800 to-primary-500',
          }}
          isIndeterminate
          radius="none"
        />
      )}

      <h1
        className="leading-none text-[3rem] flex flex-row gap-0 self-center select-none"
        onDoubleClick={discoverDevices}
      >
        <Logo className="h-16 text-primary-500 mt-[7px]" />
        <span className="relative ml-[-8px]">overhead</span>
      </h1>

      {error && (
        <pre className="text-red-500">{JSON.stringify(error, null, 2)}</pre>
      )}

      {devices.length === 0 && (
        <div className="flex flex-row h-96 items-center justify-center w-full">
          <div className="flex flex-col gap-2 items-center w-full">
            <PiEmpty size={32} />
            <h2 className="text-2xl">No Devices</h2>
          </div>
        </div>
      )}

      {devices.map(device => (
        <div
          key={device.addr}
          className={cx(
            'flex flex-col gap-4 w-full p-4 rounded-xl transition-background',
            !device.isOn ? 'bg-alpha-50' : 'bg-alpha-300'
          )}
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className="font-bold font-lg m-0">{device.name}</h4>
            <Button
              isIconOnly
              aria-label="Toggle power"
              className=" px-2 py-1 text-sm  font-bold rounded-xl"
              color="primary"
              onPress={() => toggle(device.addr)}
              variant={device.isOn ? 'solid' : 'faded'}
            >
              <AiOutlinePoweroff size={48} />
            </Button>
          </div>
          {typeof device.brightness === 'number' && (
            <Slider
              aria-label="Brightness"
              className={cx(!device.isOn && '!opacity-10')}
              classNames={{ track: '!border-r-[transparent]' }}
              defaultValue={device.brightness ?? 0}
              isDisabled={!device.isOn}
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
