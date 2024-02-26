# zigme
Everything you should need for a zigbee2mqtt home alarm system.

I still need to document usage, but in a nut-shell, update all the individual package .env.example files and rename to .env, then from the root directory run `docker compose up -d` to start all the docker processes.

# TODO: better documentation
# TODO: cleanup environment vars

## Requirements
npm and node; I used the following environment:
- nvm v0.39.7
- node v21.6.1


## How to develop
Install dependencies
```
cd zigme-ui
npm install
```
Run dev server
```
npm run dev -- --open
```

# components
https://www.shadcn-svelte.com/docs/components/card