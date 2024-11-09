import { createSignal } from 'solid-js';
import logo from './assets/logo.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [greetMsg, setGreetMsg] = createSignal('');
  const [name, setName] = createSignal('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke('greet', { name: name() }));
  }

  return (
    <main class="bg-slate-400">
      <h1>Overhead</h1>

      <form
        class="row"
        onSubmit={e => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={e => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg()}</p>
    </main>
  );
}

export default App;
