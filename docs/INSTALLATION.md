# Installation

In order to install Hestia permanently, you'll have to make some choices in how to install both the [server](#server) and [client](#client) parts of the application.

From now on we assume you've cloned the repository into `~/hestia` and the username is `hestia`. Replace this with your own values, if this isn't the case.

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [NodeJS](https://nodejs.org/en/download)
- OpenSSL dev lib (for example libssl-dev on Debian or openssl on Arch)

## Server

Before running the server you might want to consider [configurating](CONFIGURATION.md#server) it first.

### Running the server

There are several ways to run the server. Here I'll list 2 examples using: GNU Screen, systemd and using a Docker container.

#### GNU Screen

Using a screen to do this is the easiest way. You simply install GNU Screen if you haven't (this is already installed by default on many distributions).

```bash
$ sudo apt install screen
```

Then, you can start a new screen instance and run the program.

```bash
$ screen -S hestia-server
$ cd ~/hestia/server
$ cargo run --release
...
.. [Thread main] INFO - Binding to http://0.0.0.0:3000
```

#### systemd

Another way you might want to run this is through a systemd service. To start with, we'll want to compile the program and relocate the binary.

```bash
$ cd ~/hestia/server
$ cargo build --release
$ mv target/release/server .
```

Next we'll create your own service in order to run the program. In this example I will make it a user service. To make this service we create a new file.

~/.config/systemd/user/hestia-server.service
```ini
[Unit]
Description=Hestia Server
After=network.target

[Service]
Type=simple
Restart=always
RestartSec=5
WorkingDirectory=/home/hestia/hestia/server
ExecStart=/home/hestia/hestia/server/server

[Install]
WantedBy=default.target
```

Then we'll start our new service

```bash
$ systemctl --user start hestia-server
$ systemctl --user status hestia-server
● hestia-server.service - Hestia Server
     Loaded: loaded (/home/hestia/.config/systemd/user/hestia-server.service; disabled; preset: enabled)
     Active: active (running) since Sat 2024-03-23 20:34:20 CET; 39s ago
     ...
     .. [Thread main] INFO - Binding to http://0.0.0.0:3000
```

If everything went correctly it'll output something similar to the above. Optionally we can also enable it, so it'll run on system startup.

```bash
$ systemctl --user enable hestia-server
```

To turn off the server we'd run:
```bash
$ systemctl --user stop hestia-server
```

And to restart, for example after a configuration change we'd run:
```bash
$ systemctl --user restart hestia-server
```

For more information about how to configure the service further refer to the [systemd documentation](https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html) and for command usage refer to [systemctl documentation](https://www.freedesktop.org/software/systemd/man/latest/systemctl.html).

#### Docker

The final way here will be to run the server through a Docker container. To start we'd write a basic Dockerfile.

~/hestia/server/Dockerfile
```docker
FROM rust:latest

COPY . .

# Build hestia
RUN cargo build --release
RUN mv /target/release/server /

# Run the binary
CMD ["./server"]
```

We can then build the Docker container under tag hestia-server
```bash
$ cd ~/hestia/server
$ docker build -t hestia-server .
```

After building we can start it with the following command:
```bash
$ docker run -d --restart=always -p 3000:3000 hestia-server
```

### Proxy (optional)

In order to easily add SSL to the server you could proxy it through something like [nginx](https://docs.nginx.com/nginx/).

An example of this would be:

/etc/nginx/sites-available/bin.conf -> /etc/nginx/sites-enabled/bin.conf
```nginx
server {
    listen 80;
    listen [::]:80;
    server_name bin;

    location /api/ {
       proxy_pass_header Server;
       proxy_pass http://localhost:3000/;
    }
}
```

This would expose the server at port 80 at `http://bin.*domain*/api`

If you choose to make it like this (API on the same subdomain as client, but on a specific path), you'll have to follow [this guide](#double-requests-known-issue) at the end. If you don't care about this then you should host the API on a different subdomain, for example `http://bin-api.*domain*`

## Client

If you're installing the client on a platform like [Vercel](https://vercel.com/) or [Netlify](https://www.netlify.com/), you don't have to change anything since we use the auto adapter by default.

Before running the server you might want to consider [configurating](CONFIGURATION.md#client) it first.

In order to start the client on your own server, you'll have to pick a SvelteKit adapter. There are many of these available and you can find them [here](https://www.sveltesociety.dev/packages?category=sveltekit-adapters). In this installation guide we will talk about the [Node adapter](https://kit.svelte.dev/docs/adapter-node) and the [Bun adapter](https://github.com/gornostay25/svelte-adapter-bun). They work similarly, with Node having more proven security features and Bun being more performant.

### Changing adapter in code

Start by installing and changing the adapter in the `client/svelte.config.js` file.

```bash
$ bun i # For Bun
OR
$ npm i # For Node
```

```bash
$ bun add -d svelte-adapter-bun # For Bun
OR
$ npm i -D @sveltejs/adapter-node # For Node
```

```js
// Pick one of these
import adapter from 'svelte-adapter-bun'; // For Bun
import adapter from '@sveltejs/adapter-node'; // For Node

// import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
```

Now you'll want to build the frontend.

```bash
$ bun run build # For Bun
OR
$ npm run build # For Node
```

To run the frontend on any port (example: 5173), use:
```bash
$ PORT=5173 bun ./build # For Bun
OR
$ PORT=5173 node build # For Node
```

You can refer to the [server guide](#running-the-server) for ideas on how to daemonize running the frontend.


### Proxy (optional)

Just as the [optional proxy](#proxy-optional) step in the server we can modify this to safely expose the client as well.

/etc/nginx/sites-available/bin.conf -> /etc/nginx/sites-enabled/bin.conf
```diff
server {
    listen 80;
    listen [::]:80;
    server_name bin;

+    location / {
+       proxy_pass http://localhost:5173/;
+    }

    location /api/ {
       proxy_pass_header Server;
       proxy_pass http://localhost:3000/;
    }
}
```

## SSL (optional)

After doing all of this, you should probably go ahead and add SSL to be an acceptable modern service. You can do this through something like [certbot](https://certbot.eff.org/).

## Double requests (Known issue)

Due to the way SvelteKit (as of writing) handles fetching and by extension caching responses, it's not built for making requests to the same origin. It'll end up doubling up as it produces a cache miss. In order to fix this in **THIS SPECIFIC APPLICATION** if you're adamant about having the client and server on the same origin you can comment out a line in the SvelteKit source. You can find this line in `~/hestia/node_modules/@sveltejs/kit/src/runtime/client/client.js` by looking for

```javascript
if (resolved.origin === url.origin) {
    requested = resolved.href.slice(url.origin.length);
}
```

and commenting the whole block. This will fix the double requests (and view counting).

This issue could be fixed upstream at any point, and you should check if you actually have double view counting before doing anything.