import {Channel, invoke, isTauri} from "@tauri-apps/api/core";


export async function fragment({headless = false, close = false}: { headless?: boolean, close?: boolean} = {}): Promise<{ status?: boolean, message?: string} | null> {
  return await invoke<{ status?: boolean, message?: string}>("plugin:txmap|fragment", {
    payload: {
      headless ,
      close,
    },
  }).then((r) => {
    console.log(r)
    return r
  });
}

export type TxMapTasks = Map<string, Task>
export type TxMapOptions = {
  tasks: TxMapTasks
}

export type WatchOptions = {
  timeout?: number,
  rate?: number,
}

export type GetLocationOptions = {
  timeout?: number,
  age?: number,
}

export async function startTxMapWithChannel(txMapOptions: TxMapOptions, cb: (res: any, error?: string) => void): Promise<number> {
  if (!isTauri()) {
    return -1
  }
  const channel = new Channel<{res: any, error?: string}>()
  channel.onmessage = (message) => {
      cb(message)
  }
  await invoke('plugin:txmap|start_tx_map_with_channel', {
    options: txMapOptions,
    channel
  })
  return channel.id
}

export type Task = {
  id: string
  status: string,
  start_position: {
    lat: number,
    lng: number,
  },
  end_position: {
    lat: number,
    lng: number
  },
  send_info: { avatar?: string },
  receive_info: { avatar?: string },
}



export async function addTaskPreview(task: Task) {
  if (!isTauri()) {
    return
  }
  let res = await invoke<{status: boolean, error?: string}>('plugin:txmap|add_task', {
    payload: { task: task, preview: true }
  })
  return res
}

export async function addTaskReal(task: Task) {
  if (!isTauri()) {
    return
  }
  let res = await invoke<{status: boolean, error?: string}>('plugin:txmap|add_task', { task: task, preview: false })
  return res
}

export async function clearTask(id: string) {
  if (!isTauri()) {
    return
  }
  let res = await invoke<{status: boolean, error?: string}>('plugin:txmap|clear_task', { id })
  return res
}

export async function clearChannel(id: string) {
  if (!isTauri()) {
    return
  }
  await invoke('plugin:txmap|clear_channel', {
    id
  })
}

export async function watchLocation(watchOptions: WatchOptions, cb: (res: any, error?: string) => void): Promise<number> {
  if (!isTauri()) {
    return -1
  }
  const channel = new Channel<{res: any, error?: string}>()
  channel.onmessage = (message) => {
    cb(message)
  }
  await invoke('plugin:txmap|watch_location', {
    options: {...watchOptions, watch: true},
    channel
  })
  return channel.id
}

export async function clearWatchLocation(id: number) {
  await invoke('plugin:txmap|clear_watch_location', {
    id: id
  });
}

export async function clearWatchDirection(id: number) {
  await invoke('plugin:txmap|clear_watch_direction', {
    id: id
  });
}


export async function getLocation(options: GetLocationOptions, cb: (res: any, error?: string) => void): Promise<number> {
  if (!isTauri()) {
    return -1
  }
  const channel = new Channel<{res: any, error?: string}>()
  channel.onmessage = (message) => {
    cb(message)
    clearWatchLocation(channel.id)
  }
  await invoke('plugin:txmap|watch_location', {
    options: { timeout: options.timeout , rate: options.age, watch: false},
    channel
  })
  return channel.id
}


export async function watchDirection(watchOptions: WatchOptions, cb: (res: any, error?: string) => void): Promise<number> {
  if (!isTauri()) {
    return -1
  }
  const channel = new Channel<{res: any, error?: string}>()
  channel.onmessage = (message) => {
    cb(message)
  }
  await invoke('plugin:txmap|watch_direction', {
    options: {...watchOptions, watch: true},
    channel
  })
  return channel.id
}