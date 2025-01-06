import { useEffect, useState } from 'react';
import { SlPower } from 'react-icons/sl';
import { commands } from './bindings';
import './global.css';
import { Logo } from './Logo';
import { Button, Slider } from '@nextui-org/react';

type GetDevicesResult = Awaited<ReturnType<typeof commands.getDevices>>;

function App() {
  const [error, setError] = useState<string | null>(null);
  const [devices, setDevices] = useState<GetDevicesResult>([]);
  const [loading, setLoading] = useState(true);

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
        overhead
      </h1>

      {error && (
        <pre className="text-red-500">{JSON.stringify(error, null, 2)}</pre>
      )}

      <div className="flex flex-row justify-between self-stretch items-center py-2">
        <button onClick={fetchDevices}>Refresh</button>
      </div>

      {loading && <p>Loading devices...</p>}

      {devices.map(([socketAddr, device]) => (
        <div
          key={socketAddr}
          className="flex flex-col gap-4 w-full bg-white-alpha-50 p-4 rounded-xl"
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className="font-bold font-lg m-0">
              {device.system.get_sysinfo.alias}
              {/* {' '}
              {device.system.get_sysinfo.brightness} */}
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
                // base: 'max-w-md gap-3',
                track: ' border-[transparent] ',
                filler: 'bg-gradient-to-r from-secondary-50 to-primary-500',
              }}
              aria-label="Brightness"
              maxValue={100}
              minValue={0}
              // size="lg"
              // step={5}
              onChangeEnd={e => {
                console.log('onChangeEnd', e);
                commands.setBrightness(socketAddr, device, +e).catch(err => {
                  console.error(err);
                  setError(
                    err instanceof Error
                      ? err.message
                      : 'Failed to set brightness'
                  );
                });
              }}
              renderThumb={props => (
                <div {...props} className={`${props.className} !h-4 !w-4`}>
                  {/* <span className="transition-transform bg-gradient-to-br shadow-small from-secondary-100 to-secondary-500 rounded-full w-5 h-5 block group-data-[dragging=true]:scale-80" /> */}
                </div>
              )}
              value={device.system.get_sysinfo.brightness ?? 0}
            />
          )}
        </div>
      ))}
    </main>
  );
}

export default App;
