# zigme
Everything you should need for a zigbee2mqtt home alarm system.

# How this system works
1. Sensor sends message over zigbee protocol.
2. Dongle picks up message.
3. zigbee2mqtt converts serial message to mqtt message and pushes through mosquito broker.
4. mosquitto sends mqtt message to subscribers.
5. zigme-eventhandler receives mqtt message and checks to see if its an event we care about (e.g. vibration==True or contact==False) and if so sends request to zigme-api.
6. zigme-api receives request to trigger alarm, and routes to the proper alarm trigger based on the current alarm state.
7. zigme-ui allows users to update alarm state by interacting with zigme-api.

# Required Software - mosquitto and zigbee2mqtt
(See docker-compose.yml) These are both simply pulled from docker.io and are the basis for the other apps. Mosquitto is an mqtt broker, which handles routing mqtt messages for apps to publish/subscibe to various mqtt topics. zigbee2mqtt handles converting zigbee sensors to a zigbee device into mqtt messages which can be sent through the mosquitto broker. I use aqara door/window sensors and aqara vibration sensors. If you use something different you will need to play with the code in zigme-eventhandler to route payloads accordingly.

# Required Hardware - zigbee dongle
In order for zigbee2mqtt to work, you will need a dongle to convert zigbee radio signals to something your computer can use. I use the SONOFF [ZBDongle-P](https://www.amazon.com/SONOFF-Gateway-Universal-Assistant-Wireless/dp/B09KXTCMSC/ref=sr_1_6?dib=eyJ2IjoiMSJ9.xfFEitihTiGU5eql5n0JRaiyH8Qi1VdW79xhXrnRDnm1ipELTPiwLomv8dyo8eQE0KuDIUbsSMW0000sKCBlk3d6K6xwghSgVEQorgtuEkOuAQvCtqqPSbAE455PvQujwJ3e-i47WpziGhn0WEt1RO0Tz6HvkHQOMNtGU0ezMpMWGdJ6tKfx-FYAcwtfROjLSHXOFj1EXOj3jgdgAzf002Q5ZASzzNrB79eQrldWmYei_QAr5MQslBO-sPedn79iRDxaQuTVAJEXmHQ68R7s8inK46yX-qUgdFKGffKYrKM.a2bxwBBR8V1JYnyF9PajdQsb6b5swU4JGxZhem-Yjbk&dib_tag=se&keywords=Zigbee2mqtt&qid=1710736347&sr=8-6&th=1)

Look at the zigbee2mqtt guides on there site to get a run-down on how this works (and look at the docker-compose.yml).

# zigme-eventhandler
Receives MQTT messages and routes matching topic events to zigme-api for further handling.

# zigme-api
Rust axum rest API for setting/getting alarm state, reading/storing alarm trigger events. Uses Redis DB for storing persistent states and events.

# zigme-ui
A sveltekit user interface for setting alarm mode (phone alarm, phone notifications, local alarm, off, etc.).


# How to dev locally
The easiest way to test is to just run `docker compose up --build` to rebuild your containers and integrate test everything.
If you are only working on a single part, for example lets say I am working on zigme-api, I can instead run `docker compose up --build zigme-api` to not rebuild everything.

# How to run on your production server  
1. Pull this repo to your server (I am using an old ubuntu dell precision 5530 laptop, you can probably use a raspberry pi but it was taking FOREVER to build this on it, and redis wasn't working properly)
2. Run `docker compose up -d`

# How to check service health
The following command will show the health status of each service:
```
sudo docker compose ps
```
To inspect what health checks are failing you can check the service individually with:
```
sudo docker inspect <service-name>
```
e.g.
```
sudo docker inspect zigme-api
```

### TODO --- everything below here ---

### Requirements
npm and node; I used the following environment:
- nvm v0.39.7
- node v21.6.1

### How to Run
Install dependencies
```
cd zigme-ui
npm install
```
Run dev server
```
npm run dev -- --open
```

### Svelte Components
https://www.shadcn-svelte.com/docs/components/card

