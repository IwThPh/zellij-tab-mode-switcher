
# Zellij - Tab Mode Switcher

Hacky Zellij plugin to change mode to "Locked" when switching to the first tab.

The goal of this plugin is to allow for easy switching to Nvim.
The plugin will then lock the pane to prevent accidental keybinds meant for Nvim being caught by Zellij.

Hopefully this should be a temporary solution until Zellij has better support for either "pane-less" plugins or an easier way to pass through keybinds to the underlying program under certain conditions.

## Example usage

With the following configuration, Zellij will switch to "Locked" mode when switching to the first tab.

The plugin is required to be loaded as a "pane". I have set this up using a size 1 borderless pane.
Then on loading the layout I change the editor to fullscreen to hide the plugin.

`~/.congif/zellij/layouts/default.kdl`
```kdl
{
  tab name="nvim" focus=true {
    pane focus=true edit="."
    pane size=1 borderless=true {
      plugin location="file:~/.config/zellij/plugins/tab-mode-switcher.wasm"
    }
  }

  tab name=" " split_direction="vertical" {
    pane 
    pane
  }
}
```

