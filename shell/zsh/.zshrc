source "$ZDOTDIR/entry.zsh"

# Rye
source "$RYE_HOME/env"

# Direnv
eval "$(direnv hook zsh)"

# Starship
eval "$(starship init zsh)"

# Always source exit at the end to avoid overriding
source "$ZDOTDIR/exit.zsh"
