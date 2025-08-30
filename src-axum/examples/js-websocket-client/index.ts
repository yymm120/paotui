let websocket = new WebSocket("wss://localhost:4000/ws");

websocket.onopen = function() {
  console.log("Web Socket open");
}

websocket.onclose = function() {
  console.log("Web Socket closed");
}

websocket.onerror = function(err) {
  console.log("Web Socket error " + err);
}

websocket.onmessage = function(msg) {
  console.log("Web Socket message " + msg);
}
console.log("11111111111111111111111");