# from simple_websocket_server import WebSocketServer, WebSocket

# # True == on
# # False == off
phone_alarm_state: bool = False

# class SimpleEcho(WebSocket):
#     def handle(self):
#         # echo message back to client
#         self.send_message(self.data)

#     def connected(self):
#         print(self.address, 'connected')
#         self.send_message({'phone_alarm_state': phone_alarm_state})

#     def handle_close(self):
#         print(self.address, 'closed')


# server = WebSocketServer('0.0.0.0', 8000, SimpleEcho)
# server.serve_forever()


import asyncio
import websockets

async def echo(websocket, path):
    async for message in websocket:
        # Echo back the received message
        await websocket.send(message)

start_server = websockets.serve(
    echo, "localhost", 8000, 
    # Allow cross-origin requests from any origin
    origins=[None]
)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()