import { invoke } from "@tauri-apps/api/core";

export async function hasApiKey(): Promise<boolean> {
  return await invoke<boolean>("has_api_key");
}

export async function saveApiKey(apiKey: string): Promise<void> {
  await invoke("save_api_key", { apiKey });
}

export async function getApiKey(): Promise<string | null> {
  return await invoke<string | null>("get_api_key");
}

export async function deleteApiKey(): Promise<void> {
  await invoke("delete_api_key");
}
