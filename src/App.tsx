import { useState, useEffect, Suspense } from 'react';
import { commands } from './bindings';
import './global.css';
import { Logo } from './Logo';

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
    <main className="prose flex flex-col gap-8 p-4 items-start text-white">
      <h1 className="mb-0 leading-none text-[3rem] flex flex-row gap-2 items-center text-primary">
        <Logo className="h-16" />
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
        <div key={socketAddr} className="flex flex-col gap-2 w-full">
          <div className="flex flex-row justify-between items-center w-full">
            <div className="flex flex-col gap-1 grow">
              <h4 className="text-white">{device.system.get_sysinfo.alias}</h4>
            </div>
            <button
              className="bg-primary text-black px-2 py-1"
              onClick={() => commands.deviceCommand(socketAddr, device)}
            >
              Toggle
            </button>
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
