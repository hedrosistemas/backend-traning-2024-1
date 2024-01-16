import { type Logger } from 'pino'
import { type IRandomGenerator } from '../services/interfaces'

export class TempRandomGenerator implements IRandomGenerator {
  private readonly TEMP_MIN = 10
  private readonly TEMP_MAX = 45

  constructor (private readonly logger: Logger) {}

  generate (): number {
    this.logger.info('generated random data')
    return 0
  }
}
