import { createResource } from 'solid-js';
// import logo from './assets/logo.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [config, { refetch }] = createResource(() => invoke('get_config'));

  return (
    <main class="flex flex-col gap-4">
      <h1>Overhead</h1>

      <h2>Config</h2>
      <pre>{JSON.stringify(config(), null, 2)}</pre>
      <button onClick={() => refetch()}>Refresh</button>
    </main>
  );
}

export default App;
