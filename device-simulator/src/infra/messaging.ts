import { type Logger } from 'pino'
import { type DeviceData, type IMessaging } from '../services/interfaces'
import { type MqttClient, connect } from 'mqtt'

export class Messaging implements IMessaging {
  private conn: MqttClient
  private readonly MQTT_HOST: string
  private readonly MQTT_PROTOCOL: string

  constructor (private readonly logger: Logger) {
    const mqttHost = process.env.MQTT_HOST
    const mqttProtocol = process.env.MQTT_PROTOCOL

    // eslint-disable-next-line @typescript-eslint/prefer-optional-chain
    if (mqttHost === undefined || mqttHost === null || mqttProtocol === undefined || mqttProtocol === null) {
      throw new Error('invalid mqtt credentials')
    }

    this.MQTT_HOST = mqttHost
    this.MQTT_PROTOCOL = mqttProtocol
  }

  public pub (data: DeviceData): void {
    this.logger.debug(`publishing:  ${JSON.stringify(data)}`)
    this.conn.publish(`HedroTraining2024/${data.device}/${data.type}`, JSON.stringify(data))
    this.logger.debug('published')
  }

  public connect (): void {
    try {
      this.conn = connect(`${this.MQTT_PROTOCOL}://${this.MQTT_HOST}`)
    } catch (err) {
      this.logger.error({ msg: 'something went wrong', error: err })
      throw err
    }
  }
}
