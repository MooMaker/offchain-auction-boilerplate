const socket = new WebSocket('ws://localhost:3000/ws');

socket.addEventListener('open', function (event) {
    console.log("WS Connection opened");
    socket.send('Hello Server!');
});

socket.addEventListener('message', function (event) {
    console.log('Received message from server:', event.data);
});
