# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

# To publish on Github Pages

### Option 1: publish from branch

Inside `Dioxus.toml`, make sure that `out_dir = "docs"` and maybe `default_platform = "web"` are under `[application]`, and that `base_path = "publich"` is under `[web.app]`

In the Github pages settings, set to publish from the appropriate branch, from the "docs" directory.

When you want to publish, run "dx bundle", and then git push everything. After Github deploys the site, it should be in the  "magenroy.github.io/public" URL

### Option 2:
