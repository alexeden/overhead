import { createResource, createSignal, For, Show, Suspense } from 'solid-js';
import { commands } from './bindings';
import './global.css';
import { Logo } from './Logo';

function App() {
  // const [config] = createResource(() => commands.getConfig());
  const [error] = createSignal<string | null>(null);
  const [devices, { refetch }] = createResource(() => commands.getDevices());

  return (
    <main class="prose flex flex-col gap-2 p-4 items-start text-white">
      <h1 class="mb-0 leading-none text-[3rem] flex flex-row gap-2 items-center text-primary">
        <Logo class="h-16" />
        overhead
      </h1>
      {/* <Suspense fallback={<p>Loading config...</p>}> */}
      {/* <h2>Config</h2>
        <pre>{JSON.stringify(config(), null, 2)}</pre> */}
      <Show when={error()}>
        <pre class="text-red-500">{JSON.stringify(error(), null, 2)}</pre>
      </Show>
      <div class="flex flex-row justify-between self-stretch items-center py-2">
        <button onClick={() => refetch()}>Refresh</button>
      </div>
      <Suspense fallback={<p>Loading devices...</p>}>
        <For each={devices()}>
          {([socketAddr, device]) => (
            <div class="flex flex-col gap-2 w-full">
              <div class="flex flex-row justify-between items-center w-full">
                <div class="flex flex-col gap-1 grow">
                  <label class="text-lg">
                    {device.system.get_sysinfo.alias}
                  </label>
                  {/* <small>{socketAddr}</small> */}
                </div>
                <button
                  class="bg-primary text-black px-2 py-1 prose-h5"
                  onClick={() => commands.deviceCommand(socketAddr, device)}
                >
                  Toggle
                </button>
              </div>
              {/* Create a slider compoent */}
              <Show
                when={typeof device.system.get_sysinfo.brightness === 'number'}
              >
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
              </Show>

              {/* <pre>{JSON.stringify(device, null, 2)}</pre> */}
            </div>
          )}
        </For>
      </Suspense>
      {/* </Suspense> */}
    </main>
  );
}

export default App;
