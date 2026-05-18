## Experiment 2.1: Original Broadcast Chat


# How to run
1. open 4 terminal consist of 1 for server and 3 for the client
2. type a message in any client, then press enter

# what happens

![ss_1](./ss_1.png)

- the server listens on port 2000 via websocket
- each client connects and can send message
- when a client send a message, the server received it and broadcast it to all connected client
using tokio broadcast channel
- all clients can see every message typed by anyone

## Experiment 2.2: Modifying Port

the file that needs to be modifed is in the client and server file  
in server we change the TcpListener::bind("127.0.0.1:2000") to port 8888
in client we change the websocket URI from ws://127.0.0.1:2000 to ws://127.0.0.1:8888

## where the protocol is defined 

The ws:// prefix in the URI defines the WebSocket protocol. It is defined in 
the client using Uri::from_static("ws://127.0.0.1:8888"). The server side 
uses tokio-websockets's ServerBuilder which handles the WebSocket handshake 
automatically over the TCP connection so the protocol is implied on the 
server side by using that library.

## Experiment 2.3: Small changes, add IP and Port

![ss_2](./ss_2.png)

# what was changed
- Server : When a client sends a message, the server prefixes it with the 
  sender's IP and port (e.g. `127.0.0.1:49837: hi`) before broadcasting to all clients.
- Server : Added a welcome message sent to each new client on connection.
- Client : Display format updated to show `Name - From server: <message>`.

# WHY
without sender information, clients that receives the message cannot tell who sent which message 
by including the sender SocketAddr  (IP + port) in the broadcast message, every client can see the origin 
of each message.

