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
  const pluginIds = ["processes", "apps", "files", "youtube", "google"]
  const plugins: Plugin[] = []

  for (const id of pluginIds) {
    try {
      const pluginInfo = await invoke<Plugin>("get_plugin_info", { pluginId: id })
      plugins.push({
        ...pluginInfo,
        search: async (query: string) => {
          return await invoke<PluginResult[]>("search_plugin", { pluginId: id, query })
        }
      })
    } catch (error) {
      console.warn(`Failed to load plugin ${id}:`, error)
    }
  }

  return plugins
}

export async function executePluginAction(pluginId: string, resultId: string, actionId: string): Promise<string> {
  return await invoke<string>("execute_plugin_action", { pluginId, resultId, actionId })
}