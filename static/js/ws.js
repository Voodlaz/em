const socket = new WebSocket("ws://127.0.0.1:8000/ws/")
const button = document.querySelector("#send");

socket.onopen = (event) => {
    console.log("WebSocket is open now.");
};

socket.onclose = (event) => {
    console.log("WebSocket is closed now.");
};

socket.onerror = (event) => {
    console.error("WebSocket error observed:", event);
};

socket.onmessage = (event) => {
  alert(event.data)
};

socket.onmessage = (event) => {
  // append received message from the server to the DOM element
  const chat = document.querySelector("#chat");
  chat.innerHTML += event.data;
};

socket.onopen = (event) => {
  socket.send("sometext")
};


button.addEventListener("click", () => {
  const message = document.querySelector("#message");
  const data = `<p>${message.value}</p>`;

  // Send composed message to the server
  socket.send(data);

  // clear input fields
  name.value = "";
  message.value = "";
});
