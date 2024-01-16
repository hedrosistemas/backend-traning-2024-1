import { type Logger } from 'pino'
import { type IRandomGenerator } from '../services/interfaces'

export class HumidityRandomGenerator implements IRandomGenerator {
  private readonly HUMIDITY_MIN = 0.3
  private readonly HUMIDITY_MAX = 0.8

  constructor (private readonly logger: Logger) {}

  generate (): number {
    this.logger.info('generated random data')
    return 0
  }
}
