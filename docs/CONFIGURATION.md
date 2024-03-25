# Configuration

From now on we assume you've cloned the repository into `~/hestia` and the username is `hestia`. Replace this with your own values, if this isn't the case.

## Server
There are a few things you should configure before running the server. All configuration will be done in the `~/hestia/server/Config.toml` file.

```toml
[api]
bind = "0.0.0.0:3000" # The address the API will bind to.

[database]
type = "MEMORY" # Type of database to use, valid choices: ["MEMORY", "POSTGRES", "SQLITE"].
database_uri = "" # URI of the database you're trying to connect to.
```

The options here are self explanatory. I recommend changing the binding to some less common port that isn't in use and changing to a proper persistent database. The datamanager expects the URI to include the database too when working on SQL based databases.

## Client

### API Base Path

In the client the configuration is a bit more spread out. To begin with you'll have to change the base API url to that of where you'll be accessing the server. You can do this at `~/hestia/client/src/lib/api.ts` by changing the `BASE_PATH` constant.

### Theme
In order to change the theme you'll have to change 2 separate files. One of which is responsible for the main theming of the application which you can find at `~/hestia/client/static/global.css`. By changing the global colour variables you can change the way the application looks.

```css
...
:root {
    --primary-colour: rgb(167, 167, 167);
    --secondary-colour: rgb(105, 255, 105);
    --tertiary-colour: white;

    --primary-bg-colour: rgb(37, 37, 44);
    --secondary-bg-colour: #191a1c;
    --tertiary-bg-colour: #282a36;
}
...
```

The other file you'll have to change is responsible for the code editor, which will be changing how both the syntax highlighting and the editor theme look. The editor theme is configured to use the variables in the `global.css` file, but the highlighting is not due to the many different colours. You can also take this moment to decouple the editor theme from the main theme if you so like by changing those variables. Changing these things can be done at `~/hestia/client/src/components/editors/codemirrortheme.ts`.

Changing other things currently isn't done through config, but through code directly. You can for example change the name of the application shown by changing a variable in `~/hestia/client/src/routes/+layout.svelte`.

```svelte
<script>
    import Logo from "../components/Logo.svelte";
    import Footer from "../components/Footer.svelte";
</script>


<div>
    <Logo name="*CHANGE YOUR NAME HERE*" />
    <slot />
    <Footer />
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }
</style>
```

For other more detailed changes you'll have to explore the codebase :).