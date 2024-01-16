import { HumidityRandomGenerator } from './infra/humidity_random_generator'
import { LoggerInitializer } from './infra/logger'
import { Messaging } from './infra/messaging'
import { TempRandomGenerator } from './infra/temp_random_generator'
import { HumidityGeneratorService } from './services/humidity'
import { type IMessaging, type IRandomGenerator } from './services/interfaces'
import { TempGeneratorService } from './services/temp'
import dotenv from 'dotenv'

function main (): void {
  const { logger, tempRandom, humidityRandom, messaging } = setup()

  logger.info('application running')

  startup(tempRandom, humidityRandom, messaging)
}

function setup () {
  dotenv.config()

  const logger = LoggerInitializer.init()

  logger.info('stating application...')

  const messaging = new Messaging(logger)
  messaging.connect()

  const tempRandom = new TempRandomGenerator(logger)
  const humidityRandom = new HumidityRandomGenerator(logger)

  return {
    logger,
    messaging,
    tempRandom,
    humidityRandom
  }
}

function startup (tempRandom: IRandomGenerator, humidityRandom: IRandomGenerator, messaging: IMessaging) {
  const tempGeneratorService = new TempGeneratorService(tempRandom, messaging)
  const humidlyRandomService = new HumidityGeneratorService(humidityRandom, messaging)

  tempGeneratorService.do()
  humidlyRandomService.do()
}

main()
