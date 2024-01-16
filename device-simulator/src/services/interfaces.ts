interface IRandomGenerator {
  generate: () => number
}

export interface DeviceData {
  device: string
  value: string
  type: 'TEMP' | 'HUMIDITY'
}

interface IMessaging {
  pub: (data: DeviceData) => void
}

export type { IRandomGenerator, IMessaging }
