source "$ZDOTDIR/entry.zsh"

# Plugins
source "$ZDOTDIR/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh"

# Rye
RYE_ENV="$RYE_HOME/env" && test -f $RYE_ENV && source $RYE_ENV

# Starship
eval "$(starship init zsh)"

# Direnv
(("$+commands[direnv]")) && eval "$(direnv hook zsh)"

# Zoxide
eval "$(zoxide init zsh)"

# Syntax highlight must be sourced at the nearly end.
source "$ZDOTDIR/plugins/config/zsh-syntax-highlighting.zsh"
source "$ZDOTDIR/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

# Zsh history substring search must be sourced after syntax highlighting.
source "$ZDOTDIR/plugins/zsh-history-substring-search/zsh-history-substring-search.zsh"
bindkey "^[[A" history-substring-search-up
bindkey "^[[B" history-substring-search-down

# Always source exit at the end to avoid overriding.
source "$ZDOTDIR/exit.zsh"
