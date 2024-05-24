source "$ZDOTDIR/entry.zsh"

# Plugins
source "$ZDOTDIR/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh"

# Rye
source "$RYE_HOME/env"

# Starship
eval "$(starship init zsh)"

# Direnv
eval "$(direnv hook zsh)"

# Syntax highlight must be sourced at the nearly end.
source "$ZDOTDIR/plugins/config/zsh-syntax-highlighting.zsh"
source "$ZDOTDIR/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

# Always source exit at the end to avoid overriding.
source "$ZDOTDIR/exit.zsh"
