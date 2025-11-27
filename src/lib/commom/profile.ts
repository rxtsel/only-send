import { invoke } from "@tauri-apps/api/core";
import type { Profile } from "../types";


export async function saveProfile(profile: Profile): Promise<void> {
  await invoke("save_profile_command", profile);
}

export async function getProfile(): Promise<Profile | null> {
  return await invoke<Profile | null>("get_profile");
}
