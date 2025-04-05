import {
  Alert,
  Button,
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  Progress,
  Slider,
  useDisclosure,
} from '@heroui/react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { cx } from 'class-variance-authority';
import { useEffect, useState } from 'react';
import { AiOutlinePoweroff } from 'react-icons/ai';
import { PiEmpty } from 'react-icons/pi';
import { commands, Device } from './bindings';
import './global.css';
import { Logo } from './Logo';

export default function App() {
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

  // const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [devices, setDevices] = useState<Device[]>([]);
  const discoverDevices = async () => {
    try {
      setLoading(true);
      const result = await commands.discover();
      if (result.status === 'ok') {
        setDevices(ds =>
          Object.values({
            ...Object.fromEntries(ds.map(d => [d.addr, d])), // known devices
            ...Object.fromEntries(result.data.map(d => [d.addr, d])), // discovered/updated devices
          })
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
    discoverDevices();

    /** Rediscover devices on window focus */
    const unlisten = getCurrentWindow().onFocusChanged(isFocused => {
      console.log('focus changed', isFocused);
      if (isFocused) {
        discoverDevices();
      }
    });

    /** Rediscover devices every 30 seconds but only if the window is focused */
    const interval = setInterval(async () => {
      if (!(await getCurrentWindow().isFocused())) {
        console.log('not focused');
        return;
      }
      discoverDevices();
    }, 30000);

    return () => {
      clearInterval(interval);
      unlisten.then(fn => fn());
    };
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
        <Alert className="w-full" hideIcon color="danger">
          <pre className="text-red-500">{error}</pre>
        </Alert>
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
            'flex flex-col gap-4 w-full max-w-xl p-4 rounded-xl transition-background select-none cursor-pointer',
            !device.isOn ? 'bg-alpha-50' : 'bg-alpha-300'
          )}
          onClick={() => setSelectedDevice(device)}
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className="font-bold font-lg m-0 select-none">{device.name}</h4>
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
              className={cx(
                !device.isOn && '!opacity-10',
                'pointer-events-auto'
              )}
              classNames={{ track: '!border-r-[transparent]' }}
              defaultValue={device.brightness ?? 0}
              isDisabled={!device.isOn}
              maxValue={100}
              minValue={0}
              onClick={e => e.preventDefault()}
              onChangeEnd={e => setBrightness(device.addr, +e)}
            />
          )}
        </div>
      ))}

      <Drawer
        backdrop="blur"
        isOpen={!!selectedDevice}
        onOpenChange={isOpen =>
          setSelectedDevice(isOpen ? selectedDevice : null)
        }
        size="sm"
      >
        <DrawerContent>
          {onClose =>
            selectedDevice && (
              <>
                <DrawerHeader className="flex flex-col gap-1">
                  {selectedDevice.name}
                </DrawerHeader>
                <DrawerBody className="flex flex-col gap-4">
                  {typeof selectedDevice.brightness === 'number' && (
                    <Slider
                      aria-label="Brightness"
                      className={cx(
                        !selectedDevice.isOn && '!opacity-10',
                        'pointer-events-auto'
                      )}
                      classNames={{ track: '!border-r-[transparent]' }}
                      defaultValue={selectedDevice.brightness ?? 0}
                      isDisabled={!selectedDevice.isOn}
                      maxValue={100}
                      minValue={0}
                      onChangeEnd={e => setBrightness(selectedDevice.addr, +e)}
                    />
                  )}

                  <pre>{JSON.stringify(selectedDevice, null, 2)}</pre>
                </DrawerBody>
                <DrawerFooter>
                  <Button color="danger" variant="light" onPress={onClose}>
                    Close
                  </Button>
                </DrawerFooter>
              </>
            )
          }
        </DrawerContent>
      </Drawer>
    </main>
  );
}
