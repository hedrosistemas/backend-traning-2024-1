import { type IRandomGenerator, type IMessaging, type DeviceData } from './interfaces'

export class HumidityGeneratorService {
  constructor (
    private readonly randomGenerator: IRandomGenerator,
    private readonly messaging: IMessaging
  ) {
  }

  public do () {
    const random = this.randomGenerator.generate()

    const data: DeviceData = {
      device: 'random',
      type: 'HUMIDITY',
      value: String(random)
    }

    this.messaging.pub(data)
  }
}
