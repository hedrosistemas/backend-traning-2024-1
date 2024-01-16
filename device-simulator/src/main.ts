//SOLID
//12 factories
//Log
//Variáveis ambientes
//Code Linter
//MQTT
//Container

const TEMP_MIN = 10;
const TEMP_MAX = 45;

const HUMIDITY_MIN = 0.3;
const HUMIDITY_MAX = 0.8;

const INTERVAL_TIMEOUT = 1000;

function random(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min)) + min
}

type DeviceData = {
  device: string,
  value: string,
  type: "TEMP" | "HUMIDITY"
}

function tempGenerator(): DeviceData {
  const value = random(TEMP_MIN, TEMP_MAX)
  
  return {
    device: "device",
    value: String(value),
    type: "TEMP"
  }
}

function humidityGenerator(): DeviceData {
  const value = random(HUMIDITY_MIN, HUMIDITY_MAX)
  
  return {
    device: "device",
    value: String(value),
    type: "HUMIDITY"
  }
}

function main() {
  setInterval(() => {
    console.table(tempGenerator())
    console.table(humidityGenerator())
  }, INTERVAL_TIMEOUT)

}

main()
