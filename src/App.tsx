import { createResource, Show, Suspense } from 'solid-js';
// import logo from './assets/logo.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [config] = createResource<object>(() => invoke('get_config'));

  const [devices, { refetch }] = createResource<Array<object>>(
    // config.latest,
    () => invoke('get_devices')
  );

  return (
    <main class="flex flex-col gap-4">
      <h1>Overhead</h1>
      <Suspense fallback={<p>Loading config...</p>}>
        <h2>Config</h2>
        <pre>{JSON.stringify(config(), null, 2)}</pre>
        <h2>Devices</h2>
        <pre>{JSON.stringify(devices(), null, 2)}</pre>
        <button onClick={() => refetch()}>Refresh</button>
      </Suspense>
    </main>
  );
}

export default App;
