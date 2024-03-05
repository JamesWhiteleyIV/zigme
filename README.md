# zigme
Everything you should need for a zigbee2mqtt home alarm system.


# TODO: 
----------------------
- Documentation
- Cleanup environment var setup
- Tests
- Refactor 
- Remove jaegar tracing in favor of stdout; I want all info spans to print to stdout and dont really care for full-blown jaegar setup
  - https://stackoverflow.com/questions/70013172/how-to-use-the-tracing-library

Requirements
----------------------
npm and node; I used the following environment:
- nvm v0.39.7
- node v21.6.1


WIP: How to develop
----------------------
Install dependencies
```
cd zigme-ui
npm install
```
Run dev server
```
npm run dev -- --open
```

Svelte Components
----------------------
https://www.shadcn-svelte.com/docs/components/card
