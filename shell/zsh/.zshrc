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

# Zsh history substring search must be sourced after syntax highlighting.
source "$ZDOTDIR/plugins/zsh-history-substring-search/zsh-history-substring-search.zsh"
bindkey "^[[A" history-substring-search-up
bindkey "^[[B" history-substring-search-down

# Always source exit at the end to avoid overriding.
source "$ZDOTDIR/exit.zsh"
