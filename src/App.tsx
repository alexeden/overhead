import { createResource, createSignal, For, Show, Suspense } from 'solid-js';
import { commands } from './bindings';
import './global.css';

function App() {
  const [config] = createResource(() => commands.getConfig());
  const [error, setError] = createSignal<string | null>(null);
  const [loading] = createSignal<boolean>();
  const [devices, { refetch }] = createResource(() => commands.getDevices());

  return (
    <main class="prose flex flex-col gap-2 p-4 items-start">
      <h1 class="prose-2xl mb-0 leading-none text-[3rem] prose-h1">
        🔆 Overhead
      </h1>
      <Suspense fallback={<p>Loading config...</p>}>
        <h2>Config</h2>
        <pre>{JSON.stringify(config(), null, 2)}</pre>
        <Show when={error()}>
          <pre class="text-red-500">{JSON.stringify(error(), null, 2)}</pre>
        </Show>
        <Show when={loading()}>
          <div class="flex flex-row justify-between">
            <p>Doing something...</p>
          </div>
        </Show>
        <div class="flex flex-row justify-between self-stretch items-center py-2">
          <h2 class="leading-none p-0 m-0">Devices</h2>
          <button onClick={() => refetch()}>Refresh</button>
        </div>
        <Suspense fallback={<p>Loading devices...</p>}>
          <For each={devices()}>
            {([socketAddr, device]) => (
              <div class="flex flex-col gap-2">
                <div class="flex flex-row justify-between items-center">
                  <div class="flex flex-col gap-1">
                    <label>{device.system.get_sysinfo.alias}</label>
                    <small>{socketAddr}</small>
                  </div>
                  <button
                    class="bg-yellow-500 p-2 prose-h4"
                    onClick={() => commands.deviceCommand(socketAddr, device)}
                  >
                    Toggle
                  </button>
                </div>
                {/* Create a slider compoent */}
                <input
                  type="range"
                  min="0"
                  step="5"
                  max="100"
                  onChange={async e => {
                    console.log(e.target.value, typeof e.target.value);

                    await commands.setBrightness(
                      socketAddr,
                      device,
                      +e.target.value
                    );
                  }}
                  value={device.system.get_sysinfo.brightness ?? 0}
                />
                <pre>{JSON.stringify(device, null, 2)}</pre>
              </div>
            )}
          </For>
        </Suspense>
      </Suspense>
    </main>
  );
}

export default App;
