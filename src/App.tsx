import { Button, Slider } from '@nextui-org/react';
import { Channel } from '@tauri-apps/api/core';
import { useEffect, useOptimistic, useState, useTransition } from 'react';
import { SlPower } from 'react-icons/sl';
import { commands, DiscoverEvent } from './bindings';
import './global.css';
import { Logo } from './Logo';

type DeviceResult =
  Awaited<ReturnType<typeof commands.getDevices>> extends Array<infer T>
    ? T
    : never;

function App() {
  async function setBrightness(
    [socketAddr, device]: DeviceResult,
    brightness: number
  ) {
    setOptimisticDevices([
      socketAddr,
      {
        ...device,
        system: {
          ...device.system,
          get_sysinfo: { ...device.system.get_sysinfo, brightness },
        },
      },
    ]);
    commands.setBrightness(socketAddr, device, brightness).then(
      udpatedSysInfo => {
        if (udpatedSysInfo.status !== 'ok') {
          setError(JSON.stringify(udpatedSysInfo.error, null, 2));
        } else {
          setDevices(devices =>
            devices.map(d =>
              d[0] === socketAddr
                ? [
                    socketAddr,
                    {
                      ...d[1],
                      system: {
                        get_sysinfo: {
                          ...d[1].system.get_sysinfo,
                          ...udpatedSysInfo.data,
                        },
                      },
                    },
                  ]
                : d
            )
          );
        }
      },
      err => {
        console.error(err);
        setError(
          err instanceof Error ? err.message : 'Failed to set brightness'
        );
      }
    );
  }

  const [isPending, startTransition] = useTransition();
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [devices, setDevices] = useState<DeviceResult[]>([]);
  const [optimisticDevices, setOptimisticDevices] = useOptimistic<
    DeviceResult[],
    DeviceResult
  >(devices, (state, [addr, update]) => {
    return state.map(([socketAddr, device]) =>
      socketAddr === addr ? [socketAddr, update] : [socketAddr, device]
    );
  });

  const testDiscover = async () => {
    const channel = new Channel<DiscoverEvent>();
    channel.onmessage = event => {
      console.log('event', event);
    };
    commands.discover(channel).then(
      result => {
        console.log('result', result);
      },
      err => {
        console.error(err);
      }
    );
  };

  const fetchDevices = async () => {
    setLoading(true);
    try {
      const result = await commands.getDevices();
      setDevices(
        result.sort(([, d1], [, d2]) =>
          d1.system.get_sysinfo.alias.localeCompare(d2.system.get_sysinfo.alias)
        )
      );
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
        overhead {isPending ? '...' : ''}
      </h1>

      {error && (
        <pre className="text-red-500">{JSON.stringify(error, null, 2)}</pre>
      )}

      <div className="flex flex-row justify-between self-stretch items-center py-2">
        <button onClick={fetchDevices}>Refresh</button>
        <button onClick={testDiscover}>Discover</button>
      </div>

      {loading && <p>Loading devices...</p>}

      {optimisticDevices.map(([socketAddr, device]) => (
        <div
          key={socketAddr}
          className="flex flex-col gap-4 w-full bg-white-alpha-50 p-4 rounded-xl"
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className="font-bold font-lg m-0">
              {device.system.get_sysinfo.alias}{' '}
              {device.system.get_sysinfo.brightness}
            </h4>
            {/* <div className="flex flex-col gap-1 grow">
            </div> */}
            <Button
              isIconOnly
              aria-label="Toggle power"
              variant="faded"
              className=" px-2 py-1 text-sm  font-bold rounded-xl"
              color="primary"
              onPress={() => commands.deviceCommand(socketAddr, device)}
            >
              <SlPower size={48} />
            </Button>
          </div>
          {typeof device.system.get_sysinfo.brightness === 'number' && (
            <Slider
              classNames={{
                track: 'border-r-[transparent] border-l-secondary-50',
                filler: 'bg-gradient-to-r from-secondary-50 to-primary-500',
              }}
              aria-label="Brightness"
              maxValue={100}
              minValue={0}
              onChangeEnd={e => {
                startTransition(async () =>
                  setBrightness([socketAddr, device], +e)
                );
              }}
              // renderThumb={props => (
              //   <div {...props} className={`${props.className} !h-4 !w-4`} />
              // )}
              value={device.system.get_sysinfo.brightness ?? 0}
            />
          )}
        </div>
      ))}
    </main>
  );
}

export default App;
