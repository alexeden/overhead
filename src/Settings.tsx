import { load } from '@tauri-apps/plugin-store';
export const settingsStore = await load('settings.json', { autoSave: false });

export function Settings() {
  return <div>Settings</div>;
}
