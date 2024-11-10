import { createResource, For, Show, Suspense } from 'solid-js';
// import logo from './assets/logo.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [config] = createResource<object>(() => invoke('get_config'));

  const [devices, { refetch }] = createResource<Array<[string, object]>>(() =>
    invoke('get_devices')
  );

  return (
    <main class="flex flex-col gap-4 p-4 items-start">
      <Suspense fallback={<p>Loading config...</p>}>
        <h2>Config</h2>
        <pre>{JSON.stringify(config(), null, 2)}</pre>
        <h2>Devices</h2>
        <Suspense fallback={<p>Loading devices...</p>}>
          <For each={devices()}>
            {([socketAddr, device]) => (
              <div class="flex flex-row gap-2">
                <label>
                  <code>{socketAddr}</code>
                </label>
                <pre>{JSON.stringify(device, null, 2)}</pre>
              </div>
            )}
          </For>
          <button onClick={() => refetch()}>Refresh</button>
        </Suspense>
      </Suspense>
    </main>
  );
}

export default App;
