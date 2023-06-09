// Global variables
var canvas;
var canvasContext;

var serverAddress;
var serverName;

var sessionToken;
var nick;

// Throwing a fatal error to the user
function throwError(message) {
    alert("Error: "+message);
    location.reload(true); // Delete the "true" in prod
}

// Keeping the session alive
function keepAliveLoop() {
    $.post(serverAddress + "/keep_alive", nick+","+sessionToken);
}

// Main loop
function mainLoop() {
    canvasContext.fillStyle = "#ff0000";
    canvasContext.fillRect(5, 5, 35, 35);
}

// On page loaded
$(document).ready(async function() {
    // Getting the server's ip
    do {
        serverAddress=prompt("Enter the IP of the server you want to connect to.", "193.107.8.49:8888");
    } while (serverAddress==null);

    // Getting the server name
    document.getElementById("bottom_info").innerHTML = "CONNECTING...";
    
    await $.get("http://"+serverAddress+"/server_name", function(data, statusText, xhr) {
        serverName=data;
        document.getElementById("bottom_info").innerHTML = "Connected to: "+serverAddress+" | "+serverName;
    }).fail(function(data, statusText, xhr) {
        throwError("Couldn't connect to server.");
    });

    // Simplify the usage of serverAddress
    serverAddress="http://"+serverAddress; // It's going to be https in prod

    // Getting the player's nick and registration
    let nickError;
    do {
        if (nickError !== undefined) {
            alert(nickError);
        }

        do {
            nick=prompt("Enter your nick.");
        } while (nick===null);

        await $.post(serverAddress + "/register", nick, function(data, statusText, xhr) {
            sessionToken=data;
            nickError=null;
        }).fail(function(data, statusText, xhr) {
            let splittedData=String(data).split(" ");
            if (splittedData[0]=="0") {
                nickError="The nick's length has to be over 0 and below 21.";
            } else if (splittedData[0]=="1") {
                nickError="The nick can contain only english alphabet upper and lower case letters, the space, a dash (or a minus sign), and a floor character.";
            } else if (splittedData[0]=="2") {
                nickError="This nick is already taken."
            }
        });
    } while (nickError !== null);

    // Canvas init
    canvas = document.getElementById("canvas");
    canvas.classList.add("canvas_loaded");
    canvasContext = canvas.getContext("2d");

    // Calling the loop functions periodically
    setInterval(mainLoop, 32);
    setInterval(keepAliveLoop, 15000);
});
