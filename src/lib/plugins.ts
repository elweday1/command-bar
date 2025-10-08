import { invoke } from "@tauri-apps/api/core"

export interface Plugin {
  id: string
  name: string
  description: string
  prefix: string
  icon: string
  config?: PluginConfig
  search: (query: string) => Promise<PluginResult[]>
  onPrefixActivate?: () => void
}

export interface PluginConfig {
  [key: string]: any
}

export interface PluginResult {
  id: string
  title: string
  subtitle?: string
  icon?: string
  actions: PluginAction[]
}

export interface PluginAction {
  id: string
  label: string
  shortcut?: string
}

export async function loadPlugins(): Promise<Plugin[]> {
  try {
    const availablePlugins = await invoke<Plugin[]>("list_plugins")
    return availablePlugins.map(pluginInfo => ({
      ...pluginInfo,
      search: async (query: string) => {
        return await invoke<PluginResult[]>("search_plugin", { pluginId: pluginInfo.id, query })
      }
    }))
  } catch (error) {
    console.error("Failed to load plugins:", error)
    return []
  }
}

export async function executePluginAction(pluginId: string, resultId: string, actionId: string): Promise<string> {
  return await invoke<string>("execute_plugin_action", { pluginId, resultId, actionId })
}