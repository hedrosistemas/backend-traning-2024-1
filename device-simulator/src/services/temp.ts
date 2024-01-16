import { type IRandomGenerator, type IMessaging, type DeviceData } from './interfaces'

export class TempGeneratorService {
  constructor (
    private readonly randomGenerator: IRandomGenerator,
    private readonly messaging: IMessaging
  ) {
  }

  public do () {
    const random = this.randomGenerator.generate()

    const data: DeviceData = {
      device: 'random',
      type: 'TEMP',
      value: String(random)
    }

    this.messaging.pub(data)
  }
}
