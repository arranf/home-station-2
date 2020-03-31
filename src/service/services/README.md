# Services

Services follow a general pattern of using mpsc channels for their communication.

To start use `server::spawn` and pass the `Sender` it returns into `client::new`.

`spawn()` spins up a `Server` in a separate thread creating its own channel to communicate with the new thread and forward messages on, the `Sender` half of this channel is returned.
Requests can then be passed from the `Client` to the `Server` and back. The 'and back' is handeled by channels that are created on a request by request basis.

@todo Optimise this so that channels are long lived (?)
@todo Have `Server`s created `Client`s directly rather than returning a `Sender` that's passed into the `Client::new`?
