import { useEffect, useState } from 'react';
import { commands } from './bindings';
import './global.css';
import { Logo } from './Logo';
import { Button } from '@nextui-org/react';

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
          className="flex flex-col gap-2 w-full bg-white-alpha-50 p-4 rounded-xl"
        >
          <div className="flex flex-row justify-between items-center w-full">
            <h4 className=" m-0">{device.system.get_sysinfo.alias}</h4>
            {/* <div className="flex flex-col gap-1 grow">
            </div> */}
            <Button
              className=" px-2 py-1 text-sm font-bold rounded-xl"
              color="primary"
              onPress={() => commands.deviceCommand(socketAddr, device)}
            >
              Toggle
            </Button>
          </div>

          {typeof device.system.get_sysinfo.brightness === 'number' && (
            <input
              type="range"
              min="0"
              step="5"
              max="100"
              onChange={e =>
                commands.setBrightness(socketAddr, device, +e.target.value)
              }
              value={device.system.get_sysinfo.brightness ?? 0}
            />
          )}
        </div>
      ))}
    </main>
  );
}

export default App;
