import * as net from 'net';

const client = new net.Socket();

client.connect(5060, '127.0.0.1', () => {
    console.log('Connected');
    client.write("hhhh");
    // client.on('data', (data) => console.log(data));

});
