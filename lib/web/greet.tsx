
import { invoke } from '@tauri-apps/api/tauri'

export function GreetApi(name: string) {
  invoke<string>('greet', { name: name })
    .catch(console.error)

}

