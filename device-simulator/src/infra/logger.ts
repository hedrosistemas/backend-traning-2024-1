import pino, { type Logger } from 'pino'

// eslint-disable-next-line @typescript-eslint/no-extraneous-class
export class LoggerInitializer {
  public static init (): Logger {
    return pino({
      // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
      level: process.env.LOG_LEVEL || 'debug'
    })
  }
}
